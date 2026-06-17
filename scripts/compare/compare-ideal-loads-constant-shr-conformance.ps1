[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-constant-shr-conformance\26.1.0"
$CaseId = "ideal_loads_constant_shr_conformance_001"
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

function Assert-PurchasedAirSourceOrder {
    param([Parameter(Mandatory = $true)]$StageSummary)

    $purchasedAirStages = @($StageSummary.purchased_air_stages)
    $expectedPurchasedAirRoutines = @(
        "GetPurchasedAir",
        "InitPurchasedAir",
        "CalcPurchAirLoads",
        "UpdatePurchasedAir",
        "ReportPurchasedAir"
    )
    if ($purchasedAirStages.Count -ne $expectedPurchasedAirRoutines.Count) {
        throw "Expected $($expectedPurchasedAirRoutines.Count) PurchasedAir stages, found $($purchasedAirStages.Count)"
    }
    for ($stageIndex = 0; $stageIndex -lt $expectedPurchasedAirRoutines.Count; $stageIndex++) {
        $actualRoutine = $purchasedAirStages[$stageIndex].source_routine
        if ($actualRoutine -ne $expectedPurchasedAirRoutines[$stageIndex]) {
            throw "Unexpected PurchasedAir stage at index ${stageIndex}: $actualRoutine"
        }
    }
    Write-Host "OK PurchasedAir source order: $($expectedPurchasedAirRoutines -join ' -> ')"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    Assert-FileExists -Path $path -Description "required IdealLoads constant-SHR conformance input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads constant-SHR conformance comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads constant-SHR conformance comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 18" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 96" -Description "detailed sample count"
Assert-Contains -Text $text -Pattern "tolerance_failures_count: 0" -Description "zero tolerance failures"
Assert-Contains -Text $text -Pattern "tolerance_policy: conformance-gate" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "conformance status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"

Assert-FileExists -Path $summaryPath -Description "IdealLoads constant-SHR conformance compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads constant-SHR conformance markdown report"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads constant-SHR conformance Rust result store"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads constant-SHR conformance tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads constant-SHR conformance stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "IdealLoads constant-SHR conformance summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected IdealLoads constant-SHR status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "Expected zero tolerance failures, found $($summary.tolerance_failures)"
}
if ($summary.series_count -ne 18) {
    throw "Unexpected IdealLoads constant-SHR series count: $($summary.series_count)"
}
if ($summary.samples -ne 96) {
    throw "Unexpected IdealLoads sample count: $($summary.samples)"
}
$conformanceRows = @($summary.series | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 11) {
    throw "Expected 11 conformance-level output rows, found $($conformanceRows.Count)"
}
if (@($conformanceRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All conformance-level constant-SHR output rows must pass"
}
$diagnosticRows = @($summary.series | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 7) {
    throw "Expected 7 diagnostic proof rows, found $($diagnosticRows.Count)"
}
if (@($diagnosticRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All diagnostic proof rows must pass"
}

foreach ($expected in @(
    @("ZONE ONE", "Zone Thermostat Heating Setpoint Temperature", "conformance"),
    @("ZONE ONE", "Zone Thermostat Cooling Setpoint Temperature", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Zone Total Cooling Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Zone Sensible Cooling Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Zone Latent Cooling Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Supply Air Sensible Cooling Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Supply Air Latent Cooling Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Supply Air Total Cooling Rate", "conformance"),
    @("ZONE ONE INLET", "System Node Temperature", "conformance"),
    @("ZONE ONE INLET", "System Node Mass Flow Rate", "conformance"),
    @("ZONE ONE INLET", "System Node Humidity Ratio", "conformance"),
    @("ZONE ONE", "Zone System Predicted Sensible Load to Setpoint Heat Transfer Rate", "diagnostic"),
    @("ZONE ONE AIR NODE", "System Node Humidity Ratio", "diagnostic"),
    @("ZONE ONE RETURN", "System Node Humidity Ratio", "diagnostic")
)) {
    $row = @($summary.series | Where-Object { $_.key -eq $expected[0] -and $_.variable -eq $expected[1] })
    if ($row.Count -ne 1) {
        throw "Expected one summary row, found $($row.Count): $($expected[0]) / $($expected[1])"
    }
    if ($row[0].level -ne $expected[2]) {
        throw "Unexpected level for $($expected[0]) / $($expected[1]): $($row[0].level)"
    }
    if ($row[0].status -ne "pass") {
        throw "Expected $($expected[0]) / $($expected[1]) to pass, got $($row[0].status)"
    }
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 18 -or $resultStore.sample_count -ne $summary.samples) {
    throw "Unexpected result-store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$zoneLatentCooling = Get-ResultSeries -ResultStore $resultStore -Variable "Zone Ideal Loads Zone Latent Cooling Rate"
$supplyLatentCooling = Get-ResultSeries -ResultStore $resultStore -Variable "Zone Ideal Loads Supply Air Latent Cooling Rate"
$supplyHumidity = Get-ResultSeries -ResultStore $resultStore -Key "ZONE ONE INLET" -Variable "System Node Humidity Ratio"
$maxZoneLatentCooling = ($zoneLatentCooling.values | Measure-Object -Maximum).Maximum
$maxSupplyLatentCooling = ($supplyLatentCooling.values | Measure-Object -Maximum).Maximum
$minSupplyHumidity = ($supplyHumidity.values | Measure-Object -Minimum).Minimum
if ($maxZoneLatentCooling -le 0.0) {
    throw "Expected active zone latent cooling in constant-SHR diagnostic"
}
if ($maxSupplyLatentCooling -le 0.0) {
    throw "Expected active supply-air latent cooling in constant-SHR diagnostic"
}
if ($minSupplyHumidity -gt 0.007700001) {
    throw "Expected constant-SHR cooling to clamp supply humidity near the minimum cooling supply humidity ratio"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) row(s)"
}

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw | ConvertFrom-Json
if ($stageSummary.branch -ne "no-oa-no-limit-sensible") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.zone_demand_synthetic_rc_model -ne $false) {
    throw "Stage summary must record that no RC demand shortcut is used"
}
Assert-PurchasedAirSourceOrder -StageSummary $stageSummary

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance no-OA ConstantSensibleHeatRatio cooling IdealLoads branch for declared variables only" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "zone_demand_synthetic_rc_model: false" -Description "markdown demand source guard"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_compat" -Description "markdown source-order wrapper"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown PurchasedAir source order"
Assert-Contains -Text $reportText -Pattern "recirculation_node: ZONE ONE RETURN" -Description "markdown recirculation node"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Latent Cooling Rate | conformance" -Description "markdown zone latent cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Latent Cooling Rate | conformance" -Description "markdown supply latent cooling row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE INLET | System Node Humidity Ratio | conformance" -Description "markdown supply humidity row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE RETURN | System Node Humidity Ratio | diagnostic" -Description "markdown return humidity proof row"

Write-Host "IdealLoads constant-SHR conformance comparison artifacts generated."
