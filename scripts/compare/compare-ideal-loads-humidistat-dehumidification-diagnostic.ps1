[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-humidistat-dehumidification\26.1.0"
$CaseId = "ideal_loads_humidistat_dehumidification_diagnostic_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
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

function Get-ResultSeries {
    param(
        [Parameter(Mandatory = $true)]$ResultStore,
        [Parameter(Mandatory = $true)][string]$Variable,
        [string]$Key = ""
    )
    $series = @($ResultStore.series | Where-Object {
        $_.variable_name -eq $Variable -and ($Key -eq "" -or $_.key -eq $Key)
    })
    if ($series.Count -ne 1) {
        throw "Expected one result-store series for $Key/$Variable, found $($series.Count)"
    }
    return $series[0]
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    Assert-FileExists -Path $path -Description "required IdealLoads humidistat-dehumidification compare input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads humidistat-dehumidification diagnostic comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads humidistat-dehumidification diagnostic comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: diagnostic-draft" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: diagnostic" -Description "diagnostic status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"

Assert-FileExists -Path $summaryPath -Description "IdealLoads humidistat-dehumidification compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads humidistat-dehumidification markdown report"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads humidistat-dehumidification Rust result store"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads humidistat-dehumidification tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads humidistat-dehumidification stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "IdealLoads humidistat-dehumidification diagnostic must not set conformance_claim=true"
}
if ($summary.status -ne "diagnostic") {
    throw "Unexpected IdealLoads humidistat-dehumidification status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "Expected zero tolerance failures, found $($summary.tolerance_failures)"
}
if ($summary.meter_tolerance_failures -ne 0) {
    throw "Expected zero meter tolerance failures, found $($summary.meter_tolerance_failures)"
}
if ($summary.series_count -ne 38) {
    throw "Unexpected IdealLoads humidistat-dehumidification series count: $($summary.series_count)"
}
if ($summary.samples -le 0) {
    throw "Expected positive detailed sample count"
}
if (@($summary.series | Where-Object { $_.level -ne "diagnostic" }).Count -ne 0) {
    throw "All humidistat-dehumidification diagnostic rows must remain diagnostic-level"
}
if (@($summary.series | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All humidistat-dehumidification diagnostic rows must pass"
}

foreach ($variable in @(
    "Zone Ideal Loads Zone Latent Cooling Rate",
    "Zone Ideal Loads Supply Air Latent Cooling Rate",
    "Zone Ideal Loads Zone Total Cooling Rate",
    "Zone Ideal Loads Supply Air Total Cooling Rate",
    "Zone System Predicted Moisture Load to Dehumidifying Setpoint Moisture Transfer Rate",
    "System Node Humidity Ratio"
)) {
    $rows = @($summary.series | Where-Object { $_.variable -eq $variable })
    if ($rows.Count -lt 1) {
        throw "Missing expected humidistat-dehumidification summary row: $variable"
    }
    if (@($rows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
        throw "Expected $variable row(s) to pass"
    }
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 38 -or $resultStore.sample_count -ne $summary.samples) {
    throw "Unexpected result-store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$zoneLatentCooling = Get-ResultSeries -ResultStore $resultStore -Variable "Zone Ideal Loads Zone Latent Cooling Rate"
$supplyLatentCooling = Get-ResultSeries -ResultStore $resultStore -Variable "Zone Ideal Loads Supply Air Latent Cooling Rate"
$dehumidifyingMoistureDemand = Get-ResultSeries -ResultStore $resultStore -Key "ZONE ONE" -Variable "Zone System Predicted Moisture Load to Dehumidifying Setpoint Moisture Transfer Rate"
$supplyHumidity = Get-ResultSeries -ResultStore $resultStore -Key "ZONE ONE INLET" -Variable "System Node Humidity Ratio"
$maxZoneLatentCooling = ($zoneLatentCooling.values | Measure-Object -Maximum).Maximum
$maxSupplyLatentCooling = ($supplyLatentCooling.values | Measure-Object -Maximum).Maximum
$minDehumidifyingMoistureDemand = ($dehumidifyingMoistureDemand.values | Measure-Object -Minimum).Minimum
$minSupplyHumidity = ($supplyHumidity.values | Measure-Object -Minimum).Minimum
if ($maxZoneLatentCooling -le 0.0) {
    throw "Expected active zone latent cooling in humidistat-dehumidification diagnostic"
}
if ($maxSupplyLatentCooling -le 0.0) {
    throw "Expected active supply-air latent cooling in humidistat-dehumidification diagnostic"
}
if ($minDehumidifyingMoistureDemand -ge 0.0) {
    throw "Expected active negative dehumidifying moisture demand in humidistat-dehumidification diagnostic"
}
if ($minSupplyHumidity -gt 0.007700001) {
    throw "Expected humidistat dehumidification to reach the minimum cooling supply humidity ratio"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) row(s)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Latent Cooling Rate | diagnostic" -Description "markdown zone latent cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Latent Cooling Rate | diagnostic" -Description "markdown supply latent cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE | Zone System Predicted Moisture Load to Dehumidifying Setpoint Moisture Transfer Rate | diagnostic" -Description "markdown dehumidifying moisture demand row"

Write-Host "IdealLoads humidistat-dehumidification diagnostic comparison passed."
