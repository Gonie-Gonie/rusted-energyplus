[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\internal-gains-conformance\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\internal_gains_001\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot "internal_gains_001"
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
    Assert-FileExists -Path $path -Description "required internal-gains conformance input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running v0.26 internal convective gain conformance gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance internal-gains-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Internal convective gain conformance gate failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Internal Gains Conformance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: internal_gains_001" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance claim"
Assert-Contains -Text $text -Pattern "conformance_series: 1" -Description "conformance series count"
Assert-Contains -Text $text -Pattern "status: pass" -Description "gate status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "internal-gains conformance summary"
Assert-FileExists -Path $reportPath -Description "internal-gains conformance report"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne "internal_gains_001") {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Internal-gains summary must claim conformance for this gated case"
}
if ($summary.gate.blocking -ne $true) {
    throw "Internal-gains summary gate must be blocking"
}
if ($summary.status -ne "pass") {
    throw "Unexpected internal-gains conformance status: $($summary.status)"
}
if ($summary.time_axis_samples -ne 24) {
    throw "Unexpected time_axis_samples: $($summary.time_axis_samples)"
}
if ($summary.conformance_series_count -ne 1) {
    throw "Unexpected conformance_series_count: $($summary.conformance_series_count)"
}
$series = $summary.series | Where-Object { $_.variable -eq "Zone Total Internal Convective Heating Rate" }
if ($null -eq $series) {
    throw "Missing Zone Total Internal Convective Heating Rate series"
}
if ($series.level -ne "conformance") {
    throw "Unexpected internal-gain series level: $($series.level)"
}
if ($series.alignment -ne "timestamp") {
    throw "Internal-gain series must use timestamp alignment"
}
if ($series.compared_samples -ne 24) {
    throw "Unexpected internal-gain compared_samples: $($series.compared_samples)"
}
if ($series.max_abs_delta -gt 0.000000001) {
    throw "Internal-gain max_abs_delta exceeded tolerance: $($series.max_abs_delta)"
}
if ($series.rmse_delta -gt 0.000000001) {
    throw "Internal-gain rmse_delta exceeded tolerance: $($series.rmse_delta)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Internal Gains Conformance Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "gate_blocking: true" -Description "markdown blocking gate"
Assert-Contains -Text $reportText -Pattern "claim_boundary: Zone Total Internal Convective Heating Rate only" -Description "claim boundary"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE | Zone Total Internal Convective Heating Rate | conformance" -Description "markdown conformance row"

Write-Host "Internal convective gain conformance gate passed."
