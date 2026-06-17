[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-constant-supply-humidity-heating-conformance\26.1.0"
$CaseId = "ideal_loads_constant_supply_humidity_heating_conformance_candidate_001"
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
    Assert-FileExists -Path $path -Description "required IdealLoads constant-supply-humidity heating conformance input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads constant-supply-humidity heating conformance candidate artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads constant-supply-humidity heating conformance candidate comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "tolerance_policy: conformance-gate" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "conformance status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"

Assert-FileExists -Path $summaryPath -Description "IdealLoads constant-supply-humidity heating conformance compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads constant-supply-humidity heating conformance markdown report"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads constant-supply-humidity heating conformance Rust result store"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads constant-supply-humidity heating conformance tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads constant-supply-humidity heating conformance stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "IdealLoads constant-supply-humidity heating conformance summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected IdealLoads constant-supply-humidity heating status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "Expected zero tolerance failures, found $($summary.tolerance_failures)"
}
if ($summary.meter_tolerance_failures -ne 0) {
    throw "Expected zero meter tolerance failures, found $($summary.meter_tolerance_failures)"
}
if ($summary.series_count -ne 36) {
    throw "Unexpected IdealLoads constant-supply-humidity heating series count: $($summary.series_count)"
}
if ($summary.samples -le 0) {
    throw "Expected positive detailed sample count"
}
$conformanceRows = @($summary.series | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 11) {
    throw "Expected 11 conformance-level output rows, found $($conformanceRows.Count)"
}
if (@($conformanceRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All conformance-level constant-supply-humidity heating rows must pass"
}
$diagnosticRows = @($summary.series | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 25) {
    throw "Expected 25 diagnostic proof rows, found $($diagnosticRows.Count)"
}
if (@($diagnosticRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All diagnostic proof rows must pass"
}

foreach ($expected in @(
    @("ZONE ONE", "Zone Thermostat Heating Setpoint Temperature", "conformance"),
    @("ZONE ONE", "Zone Thermostat Cooling Setpoint Temperature", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Zone Total Heating Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Zone Sensible Heating Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Zone Latent Heating Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Supply Air Sensible Heating Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Supply Air Latent Heating Rate", "conformance"),
    @("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Supply Air Total Heating Rate", "conformance"),
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

foreach ($variable in @(
    "Zone Ideal Loads Zone Latent Heating Rate",
    "Zone Ideal Loads Supply Air Latent Heating Rate",
    "Zone Ideal Loads Zone Total Heating Rate",
    "Zone Ideal Loads Supply Air Total Heating Rate",
    "System Node Humidity Ratio"
)) {
    $rows = @($summary.series | Where-Object { $_.variable -eq $variable })
    if ($rows.Count -lt 1) {
        throw "Missing expected constant-supply-humidity heating summary row: $variable"
    }
    if (@($rows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
        throw "Expected $variable row(s) to pass"
    }
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 36 -or $resultStore.sample_count -ne $summary.samples) {
    throw "Unexpected result-store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$zoneLatentHeating = Get-ResultSeries -ResultStore $resultStore -Variable "Zone Ideal Loads Zone Latent Heating Rate"
$supplyLatentHeating = Get-ResultSeries -ResultStore $resultStore -Variable "Zone Ideal Loads Supply Air Latent Heating Rate"
$supplyHumidity = Get-ResultSeries -ResultStore $resultStore -Key "ZONE ONE INLET" -Variable "System Node Humidity Ratio"
$returnHumidity = Get-ResultSeries -ResultStore $resultStore -Key "ZONE ONE RETURN" -Variable "System Node Humidity Ratio"
$maxZoneLatentHeating = ($zoneLatentHeating.values | Measure-Object -Maximum).Maximum
$maxSupplyLatentHeating = ($supplyLatentHeating.values | Measure-Object -Maximum).Maximum
$maxSupplyHumidity = ($supplyHumidity.values | Measure-Object -Maximum).Maximum
$maxSupplyMinusReturnHumidity = 0.0
for ($i = 0; $i -lt $supplyHumidity.values.Count; $i++) {
    $deltaHumidity = [double]$supplyHumidity.values[$i] - [double]$returnHumidity.values[$i]
    if ($deltaHumidity -gt $maxSupplyMinusReturnHumidity) {
        $maxSupplyMinusReturnHumidity = $deltaHumidity
    }
}
if ($maxZoneLatentHeating -le 100.0) {
    throw "Expected active zone latent heating in constant-supply-humidity heating diagnostic"
}
if ($maxSupplyLatentHeating -le 100.0) {
    throw "Expected active supply-air latent heating in constant-supply-humidity heating diagnostic"
}
if ([Math]::Abs($maxSupplyHumidity - 0.0300) -gt 0.000000001) {
    throw "Expected constant-supply-humidity heating to use the maximum heating supply humidity ratio"
}
if ($maxSupplyMinusReturnHumidity -le 0.005) {
    throw "Expected supply humidity ratio to exceed return humidity ratio during constant-supply-humidity heating"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) row(s)"
}

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw | ConvertFrom-Json
if ($stageSummary.branch -ne "no-oa-no-limit-sensible") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.selected_purchased_air_branch -ne "constant_supply_humidity_heating") {
    throw "Unexpected PurchasedAir branch: $($stageSummary.selected_purchased_air_branch)"
}
if ($stageSummary.declared_ideal_loads_branch -ne "constant_supply_humidity_heating") {
    throw "Unexpected declared IdealLoads branch: $($stageSummary.declared_ideal_loads_branch)"
}
if ($stageSummary.zone_demand_synthetic_rc_model -ne $false) {
    throw "Stage summary must record that no RC demand shortcut is used"
}
if ($stageSummary.node_output_update_source -ne "UpdatePurchasedAir") {
    throw "Unexpected node output update source: $($stageSummary.node_output_update_source)"
}
if ($stageSummary.node_output_report_source -ne "ReportPurchasedAir") {
    throw "Unexpected node output report source: $($stageSummary.node_output_report_source)"
}
if ($stageSummary.rate_output_source -ne "ReportPurchasedAir after UpdatePurchasedAir") {
    throw "Unexpected rate output source: $($stageSummary.rate_output_source)"
}
if ($stageSummary.energy_output_level_policy -ne "diagnostic-only until rate-to-energy parity is separately proven") {
    throw "Unexpected energy output level policy: $($stageSummary.energy_output_level_policy)"
}
if ($stageSummary.fuel_energy_output_level_policy -ne "diagnostic-only until fuel-efficiency path is separately proven") {
    throw "Unexpected fuel energy output level policy: $($stageSummary.fuel_energy_output_level_policy)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance no-OA ConstantSupplyHumidityRatio heating IdealLoads branch for declared variables only" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_compat" -Description "markdown source-order wrapper"
Assert-Contains -Text $reportText -Pattern "ideal_loads_invocation_path: zone-equipment-validated source-order PurchasedAir wrapper" -Description "markdown IdealLoads invocation path"
Assert-Contains -Text $reportText -Pattern "direct_calc_helper_invocation: false" -Description "markdown direct calc helper invocation"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_execution_boundary: validated typed ZoneEquipmentManager path; report generator invokes source-order PurchasedAir wrapper" -Description "markdown zone-equipment execution boundary"
Assert-Contains -Text $reportText -Pattern "ideal_loads_runtime_binding_source: compile-stage typed IdealLoadsAirSystemId, ZoneId, and NodeId binding" -Description "markdown typed-ID binding source"
Assert-Contains -Text $reportText -Pattern "purchased_air_name_lookup_policy: PurchAirName string lookup is compile/report only; simulation loop uses prebound typed IDs" -Description "markdown runtime string lookup policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_flags:" -Description "markdown IdealLoads feature flags"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_dispatch_policy: compile feature flags select branch-specific source-order compat functions; unsupported active feature combinations emit diagnostics instead of approximate fallback" -Description "markdown IdealLoads feature dispatch policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_prebound_id_contract: compile-stage IdealLoadsAirSystemId, ZoneId, supply NodeId, return NodeId, zone air NodeId, optional outdoor air NodeId, availability ScheduleId, heating availability ScheduleId, and cooling availability ScheduleId" -Description "markdown IdealLoads prebound ID contract"
Assert-Contains -Text $reportText -Pattern "trace_level_source: case manifest [trace].level" -Description "markdown trace level source"
Assert-Contains -Text $reportText -Pattern "trace_result_invariance_policy: trace level selects evidence payload only; ResultStore values are computed before report serialization" -Description "markdown trace result invariance policy"
Assert-Contains -Text $reportText -Pattern "trace_overhead_accounting: trace/report serialization overhead is outside numerical conformance comparison and measured separately from simulation results" -Description "markdown trace overhead accounting"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown PurchasedAir source order"
Assert-Contains -Text $reportText -Pattern "selected_purchased_air_branch: constant_supply_humidity_heating" -Description "markdown PurchasedAir branch"
Assert-Contains -Text $reportText -Pattern "declared_ideal_loads_branch: constant_supply_humidity_heating" -Description "markdown declared branch"
Assert-Contains -Text $reportText -Pattern "inactive_branches: outdoor_air, economizer, heat_recovery, humidistat, dcv, autosizing, saturation_limit" -Description "markdown inactive branches"
Assert-Contains -Text $reportText -Pattern "source_map_anchor: docs/src/porting-map/ideal-loads-source-map.md" -Description "markdown source-map anchor"
Assert-Contains -Text $reportText -Pattern "node_output_timestamp_alignment: timestamp" -Description "markdown node timestamp alignment"
Assert-Contains -Text $reportText -Pattern "node_output_update_source: UpdatePurchasedAir" -Description "markdown node output update source"
Assert-Contains -Text $reportText -Pattern "node_output_report_source: ReportPurchasedAir" -Description "markdown node output report source"
Assert-Contains -Text $reportText -Pattern "rate_output_source: ReportPurchasedAir after UpdatePurchasedAir" -Description "markdown rate output source"
Assert-Contains -Text $reportText -Pattern "energy_output_level_policy: diagnostic-only until rate-to-energy parity is separately proven" -Description "markdown energy output level policy"
Assert-Contains -Text $reportText -Pattern "fuel_energy_output_level_policy: diagnostic-only until fuel-efficiency path is separately proven" -Description "markdown fuel energy output level policy"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Latent Heating Rate | conformance" -Description "markdown zone latent heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Supply Air Latent Heating Rate | conformance" -Description "markdown supply latent heating row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE INLET | System Node Humidity Ratio | conformance" -Description "markdown supply humidity row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE AIR NODE | System Node Humidity Ratio | diagnostic" -Description "markdown zone humidity proof row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE RETURN | System Node Humidity Ratio | diagnostic" -Description "markdown return humidity proof row"

Write-Host "IdealLoads constant-supply-humidity heating conformance candidate artifacts generated."
