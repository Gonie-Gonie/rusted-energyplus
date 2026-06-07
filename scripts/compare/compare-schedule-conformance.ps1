[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\schedule_constant_001\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot "schedule_constant_001"
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
        throw "Missing required schedule conformance file: $path"
    }
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running v0.22 schedule conformance gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Schedule conformance gate failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Time, Weather, and Schedule Conformance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: schedule_constant_001" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance claim"
Assert-Contains -Text $text -Pattern "conformance_series: 1" -Description "conformance series count"
Assert-Contains -Text $text -Pattern "status: pass" -Description "gate status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "schedule conformance summary"
Assert-FileExists -Path $reportPath -Description "schedule conformance report"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne "schedule_constant_001") {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Schedule summary must claim conformance for this gated case"
}
if ($summary.status -ne "pass") {
    throw "Unexpected schedule conformance status: $($summary.status)"
}
if ($summary.time_axis_samples -ne 24) {
    throw "Unexpected schedule sample count: $($summary.time_axis_samples)"
}
if ($summary.conformance_series_count -ne 1) {
    throw "Unexpected conformance_series_count: $($summary.conformance_series_count)"
}
$series = $summary.series | Where-Object { $_.variable -eq "Schedule Value" }
if ($null -eq $series) {
    throw "Missing Schedule Value series"
}
if ($series.level -ne "conformance") {
    throw "Unexpected Schedule Value level: $($series.level)"
}
if ($series.alignment -ne "timestamp") {
    throw "Schedule Value must use timestamp alignment"
}
if ($series.max_abs_delta -gt 0.000000000001) {
    throw "Schedule max_abs_delta exceeded tolerance: $($series.max_abs_delta)"
}
if ($series.rmse_delta -gt 0.000000000001) {
    throw "Schedule rmse_delta exceeded tolerance: $($series.rmse_delta)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Time, Weather, and Schedule Conformance Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "gate_blocking: true" -Description "markdown blocking gate"
Assert-Contains -Text $reportText -Pattern "timestamp_rule: hour-ending hourly samples aligned by EnergyPlus ESO timestamp labels" -Description "timestamp rule"
Assert-Contains -Text $reportText -Pattern "Schedule Value" -Description "markdown schedule variable"
Assert-Contains -Text $reportText -Pattern "| ALWAYSON | Schedule Value | conformance" -Description "markdown conformance row"

Write-Host "Schedule conformance gate passed."
