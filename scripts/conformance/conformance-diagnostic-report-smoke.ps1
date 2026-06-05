[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\conformance-diagnostic\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\zone_temperature_diagnostic_001\case.toml"

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

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required diagnostic-report input: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating manifest-driven zone-temperature diagnostic report."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance diagnostic-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Conformance diagnostic report generation failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Conformance Diagnostic Report" -Description "diagnostic header"
Assert-Contains -Text $text -Pattern "id: zone_temperature_diagnostic_001" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "samples: 24" -Description "sample count"
Assert-Contains -Text $text -Pattern "tolerance_policy: none" -Description "tolerance boundary"
Assert-Contains -Text $text -Pattern "status: extracted" -Description "diagnostic status"

$CaseOutput = Join-Path $OutputRoot "zone_temperature_diagnostic_001"
$OracleCaseOutput = Join-Path $CaseOutput "oracle"
$CompareOutput = Join-Path $CaseOutput "compare"

Assert-FileExists -Path (Join-Path $OracleCaseOutput "input.idf") -Description "staged IDF"
Assert-FileExists -Path (Join-Path $OracleCaseOutput "input.epJSON") -Description "converted epJSON"
Assert-FileExists -Path (Join-Path $OracleCaseOutput "eplusout.eso") -Description "EnergyPlus ESO"
Assert-FileExists -Path (Join-Path $OracleCaseOutput "eplusout.err") -Description "EnergyPlus ERR"

$summaryPath = Join-Path $CompareOutput "compare-summary.json"
$reportPath = Join-Path $CompareOutput "compare-report.md"
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

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Zone Temperature Diagnostic Report" -Description "report header"
Assert-Contains -Text $reportText -Pattern "comparison_class: diagnostic-only" -Description "report comparison class"
Assert-Contains -Text $reportText -Pattern "tolerance_policy: none" -Description "report tolerance boundary"
Assert-Contains -Text $reportText -Pattern "status: extracted" -Description "report status"

Write-Host "Conformance diagnostic report smoke passed."
