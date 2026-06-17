[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-capacity-limit-conformance\26.1.0"
$CaseId = "ideal_loads_capacity_limit_conformance_001"
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

function Assert-SeriesRow {
    param(
        [Parameter(Mandatory = $true)][object[]]$Rows,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Variable,
        [Parameter(Mandatory = $true)][string]$Level,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $row = @($Rows | Where-Object { $_.key -eq $Key -and $_.variable -eq $Variable })
    if ($row.Count -ne 1) {
        throw "Expected one $Description row, found $($row.Count): $Key / $Variable"
    }
    if ($row[0].level -ne $Level) {
        throw "Unexpected level for $Description`: $($row[0].level)"
    }
    if ($row[0].status -ne "pass") {
        throw "$Description must pass, got $($row[0].status)"
    }
    Write-Host "OK $Description`: $Key / $Variable"
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

function Assert-ZoneEquipmentDispatch {
    param([Parameter(Mandatory = $true)]$StageSummary)

    $expectedPath = "ZoneEquipmentManager::ManageZoneEquipment -> SimZoneEquipment -> ZoneEquipType::PurchasedAir -> PurchasedAirManager::SimPurchasedAir"
    if ($StageSummary.zone_equipment_dispatch_path -ne $expectedPath) {
        throw "Unexpected zone equipment dispatch path: $($StageSummary.zone_equipment_dispatch_path)"
    }
    if ($StageSummary.zone_equipment_dispatch_validation -ne "pass") {
        throw "Zone equipment dispatch validation did not pass: $($StageSummary.zone_equipment_dispatch_validation)"
    }
    if ($StageSummary.zone_equipment_conformance_candidate -ne "pass") {
        throw "Zone equipment dispatch is not a conformance candidate: $($StageSummary.zone_equipment_conformance_candidate)"
    }
    if ($StageSummary.zone_equipment_scope -ne "single-zone-single-equipment") {
        throw "Unexpected zone equipment dispatch scope: $($StageSummary.zone_equipment_scope)"
    }
    if (@($StageSummary.zone_equipment_dispatch_issues).Count -ne 0) {
        throw "Unexpected zone equipment dispatch issues: $($StageSummary.zone_equipment_dispatch_issues -join ', ')"
    }
    if (@($StageSummary.zone_equipment_dispatch_warnings).Count -ne 0) {
        throw "Unexpected zone equipment dispatch warnings: $($StageSummary.zone_equipment_dispatch_warnings -join ', ')"
    }
    Write-Host "OK zone equipment dispatch path: $expectedPath"
}

function Assert-ZoneSysEnergyDemandEvidence {
    param([Parameter(Mandatory = $true)]$StageSummary)

    if ($StageSummary.zone_demand_struct_source -ne "src/EnergyPlus/DataZoneEnergyDemands.hh::ZoneSysEnergyDemand") {
        throw "Unexpected ZoneSysEnergyDemand source: $($StageSummary.zone_demand_struct_source)"
    }
    if ($StageSummary.zone_demand_heating_field -ne "RemainingOutputReqToHeatSP") {
        throw "Unexpected heating demand field: $($StageSummary.zone_demand_heating_field)"
    }
    if ($StageSummary.zone_demand_cooling_field -ne "RemainingOutputReqToCoolSP") {
        throw "Unexpected cooling demand field: $($StageSummary.zone_demand_cooling_field)"
    }
    if ($StageSummary.zone_demand_mismatch_classification -ne "upstream_zone_heat_balance_input") {
        throw "Unexpected demand mismatch classification: $($StageSummary.zone_demand_mismatch_classification)"
    }
    if ($StageSummary.zone_demand_fixture_mode -ne "source-order-oracle-demand-input") {
        throw "Unexpected demand fixture mode: $($StageSummary.zone_demand_fixture_mode)"
    }
    Write-Host "OK ZoneSysEnergyDemand source/sign metadata"
}

function Assert-NodeOutputEvidence {
    param([Parameter(Mandatory = $true)]$StageSummary)

    if ($StageSummary.node_output_store_type -ne "ep_runtime::ResultStore") {
        throw "Unexpected node output store type: $($StageSummary.node_output_store_type)"
    }
    if ($StageSummary.node_output_state_struct -ne "ep_runtime::node::IdealLoadsSupplyNodeUpdate") {
        throw "Unexpected node output state struct: $($StageSummary.node_output_state_struct)"
    }
    if ($StageSummary.node_output_update_source -ne "UpdatePurchasedAir") {
        throw "Unexpected node output update source: $($StageSummary.node_output_update_source)"
    }
    if ($StageSummary.node_output_report_source -ne "ReportPurchasedAir") {
        throw "Unexpected node output report source: $($StageSummary.node_output_report_source)"
    }
    Write-Host "OK node output store/update/report metadata"
}
function Assert-ReportPurchasedAirOutputEvidence {
    param(
        [Parameter(Mandatory = $true)]$Summary,
        [Parameter(Mandatory = $true)]$StageSummary
    )

    $expectedRateSource = "ReportPurchasedAir after UpdatePurchasedAir"
    $expectedRateTimestep = "Detailed system timestep values"
    $expectedEnergyTimestep = "ReportPurchasedAir rate * TimeStepSysSec"
    $expectedEnergyPolicy = "diagnostic-only until rate-to-energy parity is separately proven"
    $expectedFuelPolicy = "diagnostic-only until fuel-efficiency path is separately proven"

    if ($Summary.rate_output_source -ne $expectedRateSource) {
        throw "Unexpected rate output source: $($Summary.rate_output_source)"
    }
    if ($Summary.rate_output_timestep_source -ne $expectedRateTimestep) {
        throw "Unexpected rate output timestep source: $($Summary.rate_output_timestep_source)"
    }
    if ($Summary.energy_output_timestep_source -ne $expectedEnergyTimestep) {
        throw "Unexpected energy output timestep source: $($Summary.energy_output_timestep_source)"
    }
    if ($Summary.energy_output_level_policy -ne $expectedEnergyPolicy) {
        throw "Unexpected energy output level policy: $($Summary.energy_output_level_policy)"
    }
    if ($Summary.fuel_energy_output_level_policy -ne $expectedFuelPolicy) {
        throw "Unexpected fuel energy output level policy: $($Summary.fuel_energy_output_level_policy)"
    }
    if ($StageSummary.rate_output_source -ne $expectedRateSource) {
        throw "Unexpected stage rate output source: $($StageSummary.rate_output_source)"
    }
    if ($StageSummary.rate_output_timestep_source -ne $expectedRateTimestep) {
        throw "Unexpected stage rate output timestep source: $($StageSummary.rate_output_timestep_source)"
    }
    if ($StageSummary.energy_output_timestep_source -ne $expectedEnergyTimestep) {
        throw "Unexpected stage energy output timestep source: $($StageSummary.energy_output_timestep_source)"
    }
    if ($StageSummary.energy_output_level_policy -ne $expectedEnergyPolicy) {
        throw "Unexpected stage energy output level policy: $($StageSummary.energy_output_level_policy)"
    }
    if ($StageSummary.fuel_energy_output_level_policy -ne $expectedFuelPolicy) {
        throw "Unexpected stage fuel energy output level policy: $($StageSummary.fuel_energy_output_level_policy)"
    }
    Write-Host "OK ReportPurchasedAir output source/timestep/policy metadata"
}
foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    Assert-FileExists -Path $path -Description "required IdealLoads compare input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads no-OA capacity-limit conformance comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA capacity-limit conformance comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 18" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 188" -Description "detailed sample count"
Assert-Contains -Text $text -Pattern "tolerance_failures_count: 0" -Description "zero tolerance failures"
Assert-Contains -Text $text -Pattern "tolerance_policy: conformance-gate" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "conformance status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$selectedOutputsPath = Join-Path $CompareRoot "selected_outputs.json"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$variableDeltasPath = Join-Path $CompareRoot "variable-deltas.csv"
$firstDivergencePath = Join-Path $CompareRoot "first-divergence.csv"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"

Assert-FileExists -Path $summaryPath -Description "IdealLoads compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads markdown report"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads oracle selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads first divergence CSV"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "IdealLoads capacity-limit conformance summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected IdealLoads capacity-limit conformance status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads capacity-limit conformance should have zero tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.samples -ne 188) {
    throw "Unexpected IdealLoads sample count: $($summary.samples)"
}
if ($summary.series_count -ne 18) {
    throw "Unexpected IdealLoads series count: $($summary.series_count)"
}
if ($summary.zone_demand_synthetic_rc_model -ne $false) {
    throw "IdealLoads capacity-limit conformance must not synthesize zone demand from an RC shortcut"
}
if ($summary.recirculation_node -ne "ZONE ONE RETURN") {
    throw "Unexpected recirculation node: $($summary.recirculation_node)"
}
if ($summary.recirculation_state_source -ne "EnergyPlus return/exhaust recirculation node same-call state for finite-limit no-OA mixed-air and report calculations") {
    throw "Unexpected recirculation state source: $($summary.recirculation_state_source)"
}

$rows = @($summary.series)
$conformanceRows = @($rows | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 10) {
    throw "Expected 10 conformance-level output rows, found $($conformanceRows.Count)"
}
if (@($conformanceRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All conformance-level capacity-limit output rows must pass"
}
$diagnosticRows = @($rows | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 8) {
    throw "Expected 8 diagnostic proof rows, found $($diagnosticRows.Count)"
}
if (@($diagnosticRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All diagnostic proof rows must pass"
}

Assert-SeriesRow -Rows $rows -Key "ZONE ONE" -Variable "Zone Thermostat Heating Setpoint Temperature" -Level "conformance" -Description "heating thermostat"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE" -Variable "Zone Thermostat Cooling Setpoint Temperature" -Level "conformance" -Description "cooling thermostat"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE IDEAL LOADS" -Variable "Zone Ideal Loads Zone Total Heating Rate" -Level "conformance" -Description "zone total heating rate"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE IDEAL LOADS" -Variable "Zone Ideal Loads Zone Total Cooling Rate" -Level "conformance" -Description "zone total cooling rate"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE IDEAL LOADS" -Variable "Zone Ideal Loads Zone Sensible Heating Rate" -Level "conformance" -Description "zone sensible heating rate"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE IDEAL LOADS" -Variable "Zone Ideal Loads Zone Sensible Cooling Rate" -Level "conformance" -Description "zone sensible cooling rate"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE IDEAL LOADS" -Variable "Zone Ideal Loads Supply Air Total Heating Rate" -Level "conformance" -Description "supply-air total heating rate"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE IDEAL LOADS" -Variable "Zone Ideal Loads Supply Air Total Cooling Rate" -Level "conformance" -Description "supply-air total cooling rate"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE INLET" -Variable "System Node Temperature" -Level "conformance" -Description "supply-node temperature"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE INLET" -Variable "System Node Mass Flow Rate" -Level "conformance" -Description "supply-node mass flow"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE RETURN" -Variable "System Node Temperature" -Level "diagnostic" -Description "return-node temperature proof"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE RETURN" -Variable "System Node Humidity Ratio" -Level "diagnostic" -Description "return-node humidity proof"
Assert-SeriesRow -Rows $rows -Key "ZONE ONE INLET" -Variable "System Node Humidity Ratio" -Level "diagnostic" -Description "supply-node humidity proof"

$nodeFlow = @($rows | Where-Object { $_.key -eq "ZONE ONE INLET" -and $_.variable -eq "System Node Mass Flow Rate" })
if ($nodeFlow[0].alignment -ne "timestamp") {
    throw "System Node Mass Flow Rate must use timestamp alignment"
}
if ($nodeFlow[0].rust_source -ne "rust-ideal-loads-no-oa-sensible-limited-calc") {
    throw "Unexpected node flow Rust source: $($nodeFlow[0].rust_source)"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) row(s)"
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 18 -or $resultStore.sample_count -ne 188) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$selectedOutputs = Get-Content -LiteralPath $selectedOutputsPath -Raw | ConvertFrom-Json
if (@($selectedOutputs.series).Count -ne 18) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw | ConvertFrom-Json
if ($stageSummary.branch -ne "no-oa-finite-limit-sensible") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.selected_purchased_air_branch -ne "finite_capacity") {
    throw "Unexpected PurchasedAir branch: $($stageSummary.selected_purchased_air_branch)"
}
if ($stageSummary.declared_ideal_loads_branch -ne "finite_capacity") {
    throw "Unexpected declared IdealLoads branch: $($stageSummary.declared_ideal_loads_branch)"
}
if ($stageSummary.finite_limit_conformance -ne $true) {
    throw "Stage summary must mark finite_limit_conformance=true for the promoted capacity-limit case"
}
if ($stageSummary.zone_demand_synthetic_rc_model -ne $false) {
    throw "Stage summary must record that no RC demand shortcut is used"
}
Assert-PurchasedAirSourceOrder -StageSummary $stageSummary
Assert-ZoneEquipmentDispatch -StageSummary $stageSummary
Assert-ZoneSysEnergyDemandEvidence -StageSummary $stageSummary
Assert-NodeOutputEvidence -StageSummary $stageSummary
Assert-ReportPurchasedAirOutputEvidence -Summary $summary -StageSummary $stageSummary

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance no-OA finite-limit sensible IdealLoads branch for declared variables only" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "zone_demand_synthetic_rc_model: false" -Description "markdown demand source guard"
Assert-Contains -Text $reportText -Pattern "zone_demand_source: EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs" -Description "markdown ZoneSysEnergyDemand source"
Assert-Contains -Text $reportText -Pattern "zone_demand_struct_source: src/EnergyPlus/DataZoneEnergyDemands.hh::ZoneSysEnergyDemand" -Description "markdown ZoneSysEnergyDemand source struct"
Assert-Contains -Text $reportText -Pattern "zone_demand_heating_sign_convention: positive W requests heating; non-positive means no active heating request" -Description "markdown heating demand sign"
Assert-Contains -Text $reportText -Pattern "zone_demand_cooling_sign_convention: negative W requests cooling; non-negative means no active cooling request" -Description "markdown cooling demand sign"
Assert-Contains -Text $reportText -Pattern "zone_demand_mismatch_classification: upstream_zone_heat_balance_input" -Description "markdown demand mismatch classification"
Assert-Contains -Text $reportText -Pattern "zone_demand_fixture_mode: source-order-oracle-demand-input" -Description "markdown demand fixture mode"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_compat" -Description "markdown source-order wrapper"
Assert-Contains -Text $reportText -Pattern "ideal_loads_invocation_path: zone-equipment-validated source-order PurchasedAir wrapper" -Description "markdown IdealLoads invocation path"
Assert-Contains -Text $reportText -Pattern "direct_calc_helper_invocation: false" -Description "markdown direct calc helper invocation"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_execution_boundary: validated typed ZoneEquipmentManager path; report generator invokes source-order PurchasedAir wrapper" -Description "markdown zone-equipment execution boundary"
Assert-Contains -Text $reportText -Pattern "ideal_loads_runtime_binding_source: compile-stage typed IdealLoadsAirSystemId, ZoneId, and NodeId binding" -Description "markdown typed-ID binding source"
Assert-Contains -Text $reportText -Pattern "purchased_air_name_lookup_policy: PurchAirName string lookup is compile/report only; simulation loop uses prebound typed IDs" -Description "markdown runtime string lookup policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_flags:" -Description "markdown IdealLoads feature flags"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_path: ZoneEquipmentManager::ManageZoneEquipment -> SimZoneEquipment -> ZoneEquipType::PurchasedAir -> PurchasedAirManager::SimPurchasedAir" -Description "markdown zone equipment dispatch path"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_validation: pass" -Description "markdown zone equipment dispatch validation"
Assert-Contains -Text $reportText -Pattern "zone_equipment_conformance_candidate: pass" -Description "markdown zone equipment conformance candidate"
Assert-Contains -Text $reportText -Pattern "zone_equipment_scope: single-zone-single-equipment" -Description "markdown zone equipment scope"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_issues: none" -Description "markdown zone equipment dispatch issues"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_warnings: none" -Description "markdown zone equipment dispatch warnings"
Assert-Contains -Text $reportText -Pattern "selected_purchased_air_branch: finite_capacity" -Description "markdown PurchasedAir branch"
Assert-Contains -Text $reportText -Pattern "declared_ideal_loads_branch: finite_capacity" -Description "markdown declared branch"
Assert-Contains -Text $reportText -Pattern "inactive_branches: outdoor_air, economizer, heat_recovery, humidistat, dcv, autosizing, saturation_limit" -Description "markdown inactive branches"
Assert-Contains -Text $reportText -Pattern "source_map_anchor: docs/src/porting-map/ideal-loads-source-map.md" -Description "markdown source-map anchor"
Assert-Contains -Text $reportText -Pattern "node_output_timestamp_alignment: timestamp" -Description "markdown node timestamp alignment"
Assert-Contains -Text $reportText -Pattern "node_output_store_type: ep_runtime::ResultStore" -Description "markdown node output store type"
Assert-Contains -Text $reportText -Pattern "node_output_state_struct: ep_runtime::node::IdealLoadsSupplyNodeUpdate" -Description "markdown node output state struct"
Assert-Contains -Text $reportText -Pattern "node_output_update_source: UpdatePurchasedAir" -Description "markdown node output update source"
Assert-Contains -Text $reportText -Pattern "node_output_report_source: ReportPurchasedAir" -Description "markdown node output report source"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown PurchasedAir source order"
Assert-Contains -Text $reportText -Pattern "rate_output_source: ReportPurchasedAir after UpdatePurchasedAir" -Description "markdown rate output source"
Assert-Contains -Text $reportText -Pattern "rate_output_timestep_source: Detailed system timestep values" -Description "markdown rate output timestep source"
Assert-Contains -Text $reportText -Pattern "energy_output_timestep_source: ReportPurchasedAir rate * TimeStepSysSec" -Description "markdown energy output timestep source"
Assert-Contains -Text $reportText -Pattern "energy_output_level_policy: diagnostic-only until rate-to-energy parity is separately proven" -Description "markdown energy output level policy"
Assert-Contains -Text $reportText -Pattern "fuel_energy_output_level_policy: diagnostic-only until fuel-efficiency path is separately proven" -Description "markdown fuel energy output level policy"
Assert-Contains -Text $reportText -Pattern "recirculation_node: ZONE ONE RETURN" -Description "markdown recirculation node"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE INLET | System Node Mass Flow Rate | conformance" -Description "markdown node flow row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE RETURN | System Node Temperature | diagnostic" -Description "markdown recirculation temperature row"

Write-Host "IdealLoads no-OA capacity-limit conformance comparison artifacts generated."
