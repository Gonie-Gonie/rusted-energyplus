[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\surface-temperature-conformance\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\surface_temperature_nomass_001\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot "surface_temperature_nomass_001"
$CompareRoot = Join-Path $CaseOutputRoot "compare"

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
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required surface-temperature conformance file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running v0.9 surface-temperature conformance gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance heat-balance-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Surface-temperature conformance gate failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Conformance Heat Balance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: surface_temperature_nomass_001" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance claim"
Assert-Contains -Text $text -Pattern "tolerance_policy: zone-state max_abs=0.000001000000 max_rmse=0.000001000000 max_rel=none; surface-state max_abs=0.000001000000 max_rmse=0.000001000000 max_rel=none" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "gate status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "surface-temperature conformance summary"
Assert-FileExists -Path $reportPath -Description "surface-temperature conformance report"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne "surface_temperature_nomass_001") {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Surface-temperature summary must claim conformance for this gated case"
}
if ($summary.gate.blocking -ne $true) {
    throw "Surface-temperature summary gate must be blocking"
}
if ($summary.status -ne "pass") {
    throw "Unexpected surface-temperature conformance status: $($summary.status)"
}
if ($summary.samples -ne 24) {
    throw "Unexpected sample count: $($summary.samples)"
}
if ($summary.heat_balance_timesteps -ne 96) {
    throw "Unexpected heat-balance timestep count: $($summary.heat_balance_timesteps)"
}
if ($summary.series_count -ne 3) {
    throw "Unexpected series_count: $($summary.series_count)"
}
if ($summary.max_abs_delta_c -gt 0.000001) {
    throw "max_abs_delta_c exceeded tolerance: $($summary.max_abs_delta_c)"
}
if ($summary.rmse_delta_c -gt 0.000001) {
    throw "rmse_delta_c exceeded tolerance: $($summary.rmse_delta_c)"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Surface Inside Face Temperature" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Inside Face Temperature series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Surface Outside Face Temperature" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Outside Face Temperature series"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Heat Balance Conformance Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Temperature" -Description "markdown inside surface variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Temperature" -Description "markdown outside surface variable"
Assert-Contains -Text $reportText -Pattern "comparison_class: conformance" -Description "markdown comparison class"
Assert-Contains -Text $reportText -Pattern "conformance_claim: true" -Description "markdown conformance claim"
Assert-Contains -Text $reportText -Pattern "gate_blocking: true" -Description "markdown blocking gate"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "markdown status"

Write-Host "Surface-temperature conformance gate passed."
