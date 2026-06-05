[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\compare-zone\26.1.0"

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot)
    if (-not $full.StartsWith($root, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside repository: $full"
    }
}

function Remove-RepoDirectory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (Test-Path -LiteralPath $Path) {
        Assert-RepoSubPath -Path $Path
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
}

function New-Directory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null
    }
}

function Invoke-External {
    param(
        [Parameter(Mandatory = $true)][string]$FilePath,
        [Parameter(Mandatory = $true)][string[]]$Arguments
    )
    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Command failed ($LASTEXITCODE): $FilePath $($Arguments -join ' ')"
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch [regex]::Escape($Pattern)) {
        Write-Host $Text
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

$energyPlus = Join-Path $OracleRoot "energyplus.exe"
$converter = Join-Path $OracleRoot "ConvertInputFormat.exe"
$weather = Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
foreach ($path in @($energyPlus, $converter, $weather)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required oracle file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot
New-Directory -Path $OutputRoot

$fixtureIdf = Join-Path $RepoRoot "data\conformance_cases\zone_temperature_diagnostic_001\zone_temperature.idf"
if (-not (Test-Path -LiteralPath $fixtureIdf -PathType Leaf)) {
    throw "Missing zone temperature fixture: $fixtureIdf"
}
$idf = Join-Path $OutputRoot "zone-temperature.idf"
Copy-Item -LiteralPath $fixtureIdf -Destination $idf -Force

Write-Host "Running EnergyPlus zone temperature comparison oracle case."
Invoke-External -FilePath $energyPlus -Arguments @("-w", $weather, "-d", $OutputRoot, $idf)

$eso = Join-Path $OutputRoot "eplusout.eso"
if (-not (Test-Path -LiteralPath $eso -PathType Leaf)) {
    throw "EnergyPlus did not produce eplusout.eso"
}

Push-Location $OutputRoot
try {
    Invoke-External -FilePath $converter -Arguments @("zone-temperature.idf")
}
finally {
    Pop-Location
}

$epjson = Join-Path $OutputRoot "zone-temperature.epJSON"
if (-not (Test-Path -LiteralPath $epjson -PathType Leaf)) {
    throw "ConvertInputFormat did not produce zone-temperature.epJSON"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Comparing Rust heat-balance state trace with EnergyPlus ESO zone temperatures."
$reportDir = Join-Path $OutputRoot "compare"
$output = & $cargo.Source run -p ep_cli --quiet -- compare zone-temperature $epjson $weather $eso --report-dir $reportDir 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Zone temperature comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Zone Temperature Diagnostic" -Description "comparison header"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "conformance boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: none" -Description "tolerance boundary"
Assert-Contains -Text $text -Pattern "runtime_class: heat-balance-state-shell" -Description "runtime class"
Assert-Contains -Text $text -Pattern "zone: ZONE ONE" -Description "zone name"
Assert-Contains -Text $text -Pattern "heat_balance_timesteps: 96" -Description "heat-balance timestep count"
Assert-Contains -Text $text -Pattern "zone_count: 1" -Description "zone count"
Assert-Contains -Text $text -Pattern "surface_count: 6" -Description "surface count"
Assert-Contains -Text $text -Pattern "samples: 24" -Description "sample count"
Assert-Contains -Text $text -Pattern "length_match: true" -Description "length match"
Assert-Contains -Text $text -Pattern "max_abs_delta:" -Description "delta summary"
Assert-Contains -Text $text -Pattern "first_delta_sample:" -Description "first delta sample"
Assert-Contains -Text $text -Pattern "max_delta_sample:" -Description "max delta sample"
Assert-Contains -Text $text -Pattern "exact_match: not_available" -Description "exact-match boundary"
Assert-Contains -Text $text -Pattern "exit_code_semantics: extraction-only" -Description "exit-code boundary"
Assert-Contains -Text $text -Pattern "report_dir:" -Description "report directory"
Assert-Contains -Text $text -Pattern "status: extracted" -Description "comparison status"

$summaryPath = Join-Path $reportDir "compare-summary.json"
$reportPath = Join-Path $reportDir "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "diagnostic summary"
Assert-FileExists -Path $reportPath -Description "diagnostic report"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected diagnostic summary comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "Diagnostic summary must not claim conformance"
}
if ($summary.tolerance_policy -ne "none") {
    throw "Unexpected diagnostic summary tolerance_policy: $($summary.tolerance_policy)"
}
if ($summary.status -ne "extracted") {
    throw "Unexpected diagnostic summary status: $($summary.status)"
}
if ($summary.zone -ne "ZONE ONE") {
    throw "Unexpected diagnostic summary zone: $($summary.zone)"
}
if ($summary.samples -ne 24) {
    throw "Unexpected diagnostic summary samples: $($summary.samples)"
}
if ($summary.heat_balance_timesteps -ne 96) {
    throw "Unexpected diagnostic summary heat_balance_timesteps: $($summary.heat_balance_timesteps)"
}
if ($null -eq $summary.first_delta_sample) {
    throw "Diagnostic summary did not include first_delta_sample"
}
if ($null -eq $summary.max_delta_sample) {
    throw "Diagnostic summary did not include max_delta_sample"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Zone Temperature Diagnostic Report" -Description "report header"
Assert-Contains -Text $reportText -Pattern "comparison_class: diagnostic-only" -Description "report comparison class"
Assert-Contains -Text $reportText -Pattern "tolerance_policy: none" -Description "report tolerance boundary"
Assert-Contains -Text $reportText -Pattern "runtime_class: heat-balance-state-shell" -Description "report runtime class"
Assert-Contains -Text $reportText -Pattern "first_delta_sample" -Description "report first delta"
Assert-Contains -Text $reportText -Pattern "max_delta_sample" -Description "report max delta"

Write-Host "Zone temperature comparison smoke passed."
