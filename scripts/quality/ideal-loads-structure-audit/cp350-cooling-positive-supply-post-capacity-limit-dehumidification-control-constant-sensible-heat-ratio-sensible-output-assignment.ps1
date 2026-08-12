# CP350 maps only PurchasedAirManager.cc line 2217; line 2218 is excluded.
$cp350Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment"
$cp350PipelineStem = "purchased_air_$cp350Stem"
$cp350TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignment"
$cp350Lifecycle = "purchased_air_calc_${cp350Stem}_lifecycle"
$cp350SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp350Sites = @(
    "read-retained-supply-mass-flow-rate-for-constant-sensible-heat-ratio-sensible-output-first-product",
    "read-local-cp-air-for-constant-sensible-heat-ratio-sensible-output-first-product",
    "calculate-supply-mass-flow-rate-times-cp-air-for-constant-sensible-heat-ratio-sensible-output",
    "read-purchased-air-mixed-air-temperature-for-constant-sensible-heat-ratio-sensible-output-difference",
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-sensible-output-difference",
    "calculate-mixed-air-temperature-minus-supply-temperature-for-constant-sensible-heat-ratio-sensible-output",
    "calculate-mass-flow-cp-air-product-times-temperature-difference-for-constant-sensible-heat-ratio-sensible-output",
    "assign-local-cooling-sensible-output-for-constant-sensible-heat-ratio-case"
)
$cp350Module = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem.rs"
$cp350State = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\state.rs"
$cp350Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\transition.rs"
$cp350Release = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\release.rs"
$cp350Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\release\prefix_validation.rs"
$cp350PrefixSnapshotMatching = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\release\prefix_validation\snapshot_matching.rs"
$cp349Prefix = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\release\prefix_validation.rs"
$cp350Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\release\runtime_validation.rs"
$cp350Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\release\snapshot_validation.rs"
$cp350Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\tests\mod.rs"
$cp350IeeeTests = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\tests\ieee.rs"
$cp350PublicTests = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\tests\public_release.rs"
$cp350CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp350Stem\tests\release_corruption.rs"
$cp350CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp350Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp350Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp350BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp350Stem.rs"
$cp350BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp350Stem}_tests.rs"
$cp350BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp350ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp350InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp350InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp350InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp350InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp350Stem.rs"
$cp350CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp350Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp350Stem}_validation.rs"
$cp350CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp350FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp350Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp350Stem}_fixture.rs"
$cp350PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp350Pipeline = "crates\ep_run\src\pipeline\$cp350PipelineStem.rs"
$cp350PipelineValidation = "crates\ep_run\src\pipeline\$cp350PipelineStem\validation.rs"
$cp350PipelineTests = "crates\ep_run\src\pipeline\$cp350PipelineStem\validation\tests.rs"
$cp350Serialization = "crates\ep_run\src\pipeline\$cp350PipelineStem\serialization.rs"
$cp350SnapshotSerialization = "crates\ep_run\src\pipeline\$cp350PipelineStem\serialization\snapshot.rs"
$cp350ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp350Audit = "scripts\quality\ideal-loads-structure-audit\cp350-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1"

function Get-Cp350RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) {
        throw "$Description expected one anchor, found $($anchors.Count)"
    }
    $opening = $Text.IndexOf("{", $anchors[0].Index)
    $depth = 0
    for ($index = $opening; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") {
            $depth += 1
        } elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1)
            }
        }
    }
    throw "$Description has no complete brace block"
}

function Assert-Cp350MutationRejected {
    param([string]$Original, [string]$Mutated, [scriptblock]$Validator, [string]$Description)
    if ($Original -ceq $Mutated) {
        throw "CP350 self-test mutation was not applied: $Description"
    }
    try {
        & $Validator $Mutated
    } catch {
        return
    }
    throw "CP350 audit failed to reject mutation: $Description"
}

function Assert-Cp350TransitionContract {
    param([string]$Text)
    foreach ($pattern in @(
            'Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned',
            'let cp_air\s*=\s*predecessor\.cp_air_j_per_kg_k\?;',
            '!cp_air\.is_finite\(\)',
            'input\.supply_mass_flow_rate_kg_per_s\s*\*\s*cp_air',
            'input\.mixed_air_temperature_c\s*-\s*input\.supply_temperature_c',
            'first_product\s*\*\s*difference',
            '(?s)calculated_cooling_sensible_output_w:\s*prepared\.calculated_cooling_sensible_output_w,.*?cooling_sensible_output_w:\s*prepared\.calculated_cooling_sensible_output_w',
            'supply_mass_flow_rate_read_count\s*\+=\s*1',
            'cp_air_read_count\s*\+=\s*1',
            'supply_mass_flow_rate_times_cp_air_calculation_count\s*\+=\s*1',
            'mixed_air_temperature_read_count\s*\+=\s*1',
            'supply_temperature_read_count\s*\+=\s*1',
            'mixed_air_minus_supply_temperature_calculation_count\s*\+=\s*1',
            'cooling_sensible_output_calculation_count\s*\+=\s*1',
            'cooling_sensible_output_assignment_write_count\s*\+=\s*1'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP350 transition contract missing '$pattern'"
        }
    }
    if ([regex]::Matches($Text, '\.is_finite\(\)').Count -ne 1 -or
        $Text -match '(?:mixed_air_temperature_c|supply_temperature_c|first_product|difference|output|calculated_cooling_sensible_output_w|cooling_sensible_output_w)\.is_(?:finite|nan)\(\)|mul_add|clamp|energyplus_psy_cp_air_fn_w|CoolSHR|CoolTotOutput') {
        throw "CP350 transition violates inherited-CpAir-only finite corroboration, exact AST, or line boundary"
    }
}

function Assert-Cp350RuntimeContract {
    param([string]$Text)
    foreach ($pattern in @(
            '(?s)unit_off_skip_count.*?non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?dehumidification_control_none_case_completed_skip_count.*?dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count.*?dehumidification_control_humidistat_case_selected_skip_count.*?dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count',
            'route_partition\s*==\s*state\.transition_count',
            '(?s)dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count\s*==\s*prior\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count',
            '(?s)assignments\.checked_mul\(\s*PURCHASED_AIR_CALC_.*?_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)',
            'state\.source_site_execution_count\s*==\s*expected_sites',
            'site_counts\.into_iter\(\)\.all\(\|count\|\s*count\s*==\s*assignments\)'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP350 lifecycle algebra missing '$pattern'"
        }
    }
    foreach ($counter in @(
            "supply_mass_flow_rate_read_count",
            "cp_air_read_count",
            "supply_mass_flow_rate_times_cp_air_calculation_count",
            "mixed_air_temperature_read_count",
            "supply_temperature_read_count",
            "mixed_air_minus_supply_temperature_calculation_count",
            "cooling_sensible_output_calculation_count",
            "cooling_sensible_output_assignment_write_count"
        )) {
        if ($Text -notmatch ('state\.' + $counter + ',')) {
            throw "CP350 per-site parity missing for '$counter'"
        }
    }
}

function Assert-Cp350BindingContract {
    param([string]$Text)
    $cp349 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =")
    $cp350 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =")
    $cp351 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =")
    $cp352 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =")
    $cp353 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
    $cp354 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =")
    $cp355 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =")
    $cp356 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =")
    $cp357 = $Text.IndexOf("let calculation_cooling_constant_shr_case_break =")
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp349 -lt 0 -or $cp350 -le $cp349 -or $cp351 -le $cp350 -or $cp352 -le $cp351 -or $cp353 -le $cp352 -or $cp354 -le $cp353 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP349 then CP350 then CP351 then CP352 then CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp350RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP350 numerical DTO"
    if ($dto -match '(?i)cp350|sensible_output_assignment') {
        throw "CP350 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp350PipelineRootContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp350RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp350Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp350Lifecycle) + '\s*=\s*Some\s*\(').Count
    if ($none -ne 3 -or $some -ne 1) {
        throw "Pipeline must expose one direct CP350 Some and three non-direct None constructors"
    }
    $firewall = Get-Cp350RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp350Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP350 evidence exactly once"
    }
}

function Assert-Cp350RecursiveOwnerContract {
    param([string]$PrefixText, [string]$Cp349PrefixText)
    $active = Get-Cp350RustBraceBlock -Text $PrefixText -AnchorPattern '(?m)^pub\(in crate::ideal_loads\) fn active_input_from_retained_owners\s*\(' -Description "CP350 active-owner proof"
    foreach ($pattern in @(
            '(?s)system:\s*&IdealLoadsAirSystem,\s*predecessor:\s*Predecessor',
            '(?s)calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\s*\.latest\?;.*?cp_air_assignment_latest_witness\(\s*system\.id,\s*\)\?;.*?predecessor_snapshots_match_bit_exact\(\s*retained_predecessor,\s*retained_predecessor_witness,\s*\).*?cp_air_assignment_snapshot_is_exact_direct_release\(\s*retained_predecessor,\s*\).*?\bcompleted_direct_.*?_cp_air_assignment_is_consistent\(\s*runtime,\s*unit,\s*system,\s*retained_predecessor,\s*Some\(retained_predecessor_witness\),\s*\)',
            '(?s)calc_cooling_supply_mass_flow_positive_guard\.latest\?;.*?positive_guard_latest_witness\(\s*system\.id\s*\)\?;.*?cp330_snapshots_match\(\s*flow_owner,\s*flow_witness\s*\).*?positive_guard_snapshot_is_exact_direct_release\(\s*flow_owner\s*\).*?\bcompleted_direct_cooling_supply_mass_flow_positive_guard_is_consistent\(\s*runtime,\s*unit,\s*system,\s*flow_owner,\s*Some\(flow_witness\),\s*\)',
            '(?s)calc_cooling_mixed_air_call\.latest\?;.*?cooling_mixed_air_call_latest_witness\(\s*system\.id\s*\)\?;.*?cooling_mixed_air_call_snapshots_match_bit_exact\(\s*mixed_owner,\s*mixed_witness\s*\).*?cooling_mixed_air_call_snapshot_is_exact_direct_release\(\s*mixed_owner\s*\).*?\bcompleted_direct_cooling_mixed_air_call_is_consistent\(\s*runtime,\s*unit,\s*system,\s*mixed_owner,\s*Some\(mixed_witness\),\s*\).*?\bcp349_private_active_counterfactual_links_to_direct_release\(\s*retained_predecessor,\s*predecessor,\s*mixed_owner,\s*\)',
            '(?s)calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\s*\.latest\?;.*?humidity_ratio_mixed_air_assignment_latest_witness\(\s*system\.id,\s*\)\?;.*?cp345_snapshots_match\(\s*provenance,\s*provenance_witness\s*\).*?humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release\(\s*provenance,\s*\).*?\bcompleted_direct_.*?_humidity_ratio_mixed_air_assignment_is_consistent\(\s*runtime,\s*unit,\s*system,\s*provenance,\s*Some\(provenance_witness\),\s*\)',
            '(?s)calc_cooling_positive_supply_temperature_mixed_air_limit\s*\.latest\?;.*?supply_temperature_mixed_air_limit_latest_witness\(\s*system\.id\s*\)\?;.*?cp334_snapshots_match\(\s*owner,\s*witness\s*\).*?supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release\(\s*owner,\s*\).*?\bcompleted_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent\(\s*runtime,\s*unit,\s*system,\s*owner,\s*Some\(witness\),\s*\)',
            '(?s)calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\s*\.latest\?;.*?sensible_output_supply_temperature_mixed_air_limit_latest_witness\(\s*system\.id,\s*\)\?;.*?cp344_snapshots_match\(\s*owner,\s*witness\s*\).*?sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release\(\s*owner,\s*\).*?\bcompleted_direct_.*?_sensible_output_supply_temperature_mixed_air_limit_is_consistent\(\s*runtime,\s*unit,\s*system,\s*owner,\s*Some\(witness\),\s*\)'
        )) {
        if ($active -notmatch $pattern) { throw "CP350 recursive active-owner proof missing '$pattern'" }
    }
    $counterfactual = Get-Cp350RustBraceBlock -Text $Cp349PrefixText -AnchorPattern '(?m)^pub\(in crate::ideal_loads::calc\) fn private_active_counterfactual_links_to_direct_release\s*\(' -Description "CP350 restricted CP349 counterfactual"
    foreach ($pattern in @(
            '(?s)snapshot_route\(direct\)\s*==\s*Some\(Route::DehumidificationControlNoneCaseCompletedSkip\).*?snapshot_route\(counterfactual\)\s*==\s*Some\(Route::DehumidificationControlConstantSensibleHeatRatioCpAirAssigned\)',
            '(?s)direct\.system\s*==\s*counterfactual\.system.*?direct\.parent_call_ordinal\s*==\s*counterfactual\.parent_call_ordinal.*?direct\.controlled_zone\s*==\s*counterfactual\.controlled_zone',
            '(?s)direct\.unit_body_entered\s*==\s*counterfactual\.unit_body_entered.*?direct\.predecessor_cooling_body_entered.*?direct\.predecessor_no_outdoor_air_fallback_entered.*?direct\.predecessor_positive_supply_mass_flow_body_entered.*?direct\.unit_off_skipped.*?direct\.non_cooling_skipped.*?direct\.positive_guard_false_fallthrough_skipped',
            '(?s)owner\.system\s*==\s*counterfactual\.system.*?owner\.parent_call_ordinal\s*==\s*counterfactual\.parent_call_ordinal.*?owner\.controlled_zone\s*==\s*counterfactual\.controlled_zone.*?owner\.mixed_air_humidity_ratio_assigned.*?option_bits_match\(\s*owner\.mixed_air_humidity_ratio,\s*counterfactual\.mixed_air_humidity_ratio'
        )) {
        if ($counterfactual -notmatch $pattern) { throw "CP350 restricted CP349 counterfactual proof missing '$pattern'" }
    }
}

foreach ($required in @(
        $cp350Module, $cp350State, $cp350Transition, $cp350Release, $cp350Prefix,
        $cp350PrefixSnapshotMatching, $cp349Prefix,
        $cp350Runtime, $cp350Snapshot, $cp350Tests, $cp350IeeeTests,
        $cp350PublicTests, $cp350CorruptionTests, $cp350BindingAdapter, $cp350BindingTests,
        $cp350InitWitness, $cp350Coupled, $cp350Fixture, $cp350Pipeline,
        $cp350PipelineValidation, $cp350PipelineTests, $cp350Serialization,
        $cp350SnapshotSerialization, $cp350Audit
    )) {
    Assert-FileExists -Path $required -Description "CP350 structure"
}
Assert-LineLimit -Path $cp350Transition -Limit 450 -Description "CP350 transition"
Assert-LineLimit -Path $cp350Release -Limit 450 -Description "CP350 release"
Assert-LineLimit -Path $cp350Runtime -Limit 350 -Description "CP350 runtime validation"
Assert-LineLimit -Path $cp350Snapshot -Limit 350 -Description "CP350 snapshot validation"
Assert-LineLimit -Path $cp350Coupled -Limit 500 -Description "CP350 coupled validation"
Assert-LineLimit -Path $cp350PipelineValidation -Limit 500 -Description "CP350 pipeline validation"
Assert-LineLimit -Path $cp350Audit -Limit 500 -Description "CP350 structure audit"

# Source boundary, route partition, counters, arithmetic, and null skips.
Assert-Contains -Path $cp350Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2217' -Description "CP350 source line"
Assert-Contains -Path $cp350Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2218' -Description "CP350 first excluded line"
Assert-ExactStringArray -Path $cp350Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER" -Expected $cp350Sites -Description "CP350 eight sites"
Assert-Contains -Path $cp350State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP350 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count",
        "source_site_execution_count", "supply_mass_flow_rate_read_count",
        "cp_air_read_count", "supply_mass_flow_rate_times_cp_air_calculation_count",
        "mixed_air_temperature_read_count", "supply_temperature_read_count",
        "mixed_air_minus_supply_temperature_calculation_count",
        "cooling_sensible_output_calculation_count",
        "cooling_sensible_output_assignment_write_count"
    )) {
    Assert-Contains -Path $cp350State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP350 counter '$counter'"
}
$cp350TransitionText = Read-RepoText -Path $cp350Transition
$cp350RuntimeText = Read-RepoText -Path $cp350Runtime
$cp350ReleaseText = Read-RepoText -Path $cp350Release
$cp350PrefixText = Read-RepoText -Path $cp350Prefix
$cp349PrefixText = Read-RepoText -Path $cp349Prefix
$cp350BindingText = Read-RepoText -Path $cp350Binding
$cp350PipelineRootText = Read-RepoText -Path $cp350PipelineRoot
$cp350SnapshotJsonText = Read-RepoText -Path $cp350SnapshotSerialization
Assert-Cp350TransitionContract -Text $cp350TransitionText
Assert-Cp350RuntimeContract -Text $cp350RuntimeText
Assert-Cp350RecursiveOwnerContract -PrefixText $cp350PrefixText -Cp349PrefixText $cp349PrefixText
Assert-Cp350BindingContract -Text $cp350BindingText
Assert-Cp350PipelineRootContract -Text $cp350PipelineRootText
Assert-Contains -Path $cp350Snapshot -Pattern '(?s)first_product\.to_bits\(\)\s*==\s*\(flow\s*\*\s*cp_air\)\.to_bits\(\).*?difference\.to_bits\(\)\s*==\s*\(mixed\s*-\s*supply\)\.to_bits\(\).*?calculated\.to_bits\(\)\s*==\s*\(first_product\s*\*\s*difference\)\.to_bits\(\).*?assigned\.to_bits\(\)\s*==\s*calculated\.to_bits\(\)' -Description "CP350 exact IEEE arithmetic"
Assert-Contains -Path $cp350Snapshot -Pattern '(?s)!snapshot\.supply_mass_flow_rate_read.*?supply_mass_flow_rate_kg_per_s\.is_none\(\).*?!snapshot\.cp_air_read.*?cp_air_j_per_kg_k\.is_none\(\).*?!snapshot\.mixed_air_temperature_read.*?mixed_air_temperature_c\.is_none\(\).*?!snapshot\.supply_temperature_read.*?supply_temperature_c\.is_none\(\).*?cooling_sensible_output_w\.is_none\(\)' -Description "CP350 complete-null skip"
Assert-NotContains -Path $cp350Snapshot -Pattern '(?:first_product|difference|calculated|assigned)\.is_finite\(\)' -Description "CP350 derived finite rejection"
Assert-Contains -Path $cp350Tests -Pattern 'source_boundary_eight_sites_and_seven_route_algebra_are_exact' -Description "CP350 private route algebra test"
Assert-Contains -Path $cp350IeeeTests -Pattern 'source_ast_groups_first_product_before_temperature_difference_product' -Description "CP350 AST grouping test"
Assert-Contains -Path $cp350IeeeTests -Pattern 'signed_zero_and_positive_infinity_flow_are_preserved_as_some_bits' -Description "CP350 signed-zero/nonfinite test"
Assert-Contains -Path $cp350IeeeTests -Pattern 'nonfinite_supply_temperature_produces_bit_exact_some_values' -Description "CP350 nonfinite supply test"
Assert-Contains -Path $cp350IeeeTests -Pattern 'cp350_reads_predecessor_cp_air_without_reinvoking_humidity_helper' -Description "CP350 CP349-owned CpAir test"
Assert-Contains -Path $cp350PublicTests -Pattern 'public_none_and_inherited_routes_are_complete_null_skips' -Description "CP350 direct null-skip test"
Assert-Contains -Path $cp350PublicTests -Pattern 'lifecycle_summary_and_replay_are_exact_and_transactional' -Description "CP350 direct replay test"
Assert-Contains -Path $cp350CorruptionTests -Pattern 'supplied_latest_and_private_cp349_corruption_are_transactional' -Description "CP350 CP349 corruption test"
Assert-Contains -Path $cp350CorruptionTests -Pattern 'identity_replay_and_public_counter_overflow_are_transactional' -Description "CP350 replay/overflow test"
Assert-Contains -Path $cp350CorruptionTests -Pattern 'private_active_owner_helper_selects_g_f_from_cp334_and_l_from_cp344' -Description "CP350 G/F/L owner test"
Assert-Contains -Path $cp350CorruptionTests -Pattern 'wrong_owner_witnesses_and_provenance_are_rejected' -Description "CP350 owner corruption test"
Assert-Contains -Path $cp350CorruptionTests -Pattern 'coordinated_owner_and_cp349_corruption_is_rejected' -Description "CP350 coordinated owner/CP349 corruption test"
Assert-Contains -Path $cp350CorruptionTests -Pattern 'every_active_counter_overflow_rejects_before_mutation' -Description "CP350 counter preflight test"

# Release ownership is CP330/CP349/CP329 plus CP345-selected CP334-or-CP344.
Assert-Contains -Path $cp350Release -Pattern 'PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor' -Description "CP350 exact CP349 predecessor type"
Assert-Contains -Path $cp350Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp349:\s*Predecessor,\s*\)' -Description "CP350 exact public arguments"
Assert-Contains -Path $cp350Release -Pattern 'cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness' -Description "CP350 CP349 predecessor witness"
foreach ($pattern in @(
        'cooling_supply_mass_flow_positive_guard_latest_witness',
        'cooling_mixed_air_call_latest_witness',
        'cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness',
        'cooling_positive_supply_temperature_mixed_air_limit_latest_witness',
        'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness',
        'supply_mass_flow_rate_kg_per_s',
        'cp_air_j_per_kg_k',
        'mixed_air_temperature_c',
        'assigned_supply_temperature_c',
        'resulting_supply_temperature_c'
    )) {
    Assert-Contains -Path $cp350Prefix -Pattern $pattern -Description "CP350 owner '$pattern'"
}
Assert-Contains -Path $cp350Prefix -Pattern '(?s)let g\s*=\s*provenance\.capacity_limit_guard_false_fallthrough_skipped;.*?let f\s*=\s*provenance\.capacity_limit_sensible_output_guard_false_fallthrough;.*?let l\s*=\s*provenance\.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;.*?usize::from\(g\).*?usize::from\(f\).*?usize::from\(l\).*?!=\s*1.*?let supply\s*=\s*if g\s*\|\|\s*f\s*\{.*?calc_cooling_positive_supply_temperature_mixed_air_limit.*?assigned_supply_temperature_c.*?\}\s*else\s*\{.*?calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit.*?resulting_supply_temperature_c' -Description "CP345 G/F/L supply-temperature owner mux"
Assert-NotContains -Path $cp350Prefix -Pattern '(?:mixed|supply|mixed_air_temperature_c|supply_temperature_c)\.is_(?:finite|nan)\(\)' -Description "CP350 mixed/supply finite gate"
Assert-NotContains -Path $cp350Release -Pattern 'cooling_positive_supply_cp_air_assignment_latest|cooling_positive_supply_capacity_limit_cp_air_assignment_latest|energyplus_psy_cp_air_fn_w|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply' -Description "CP331/338/helper/numerical substitutions"

# Binding, coupled runtime, pipeline, serialization, and direct-only firewall.
Assert-Contains -Path $cp350CalcRoot -Pattern ('mod ' + [regex]::Escape($cp350Stem) + ';') -Description "CP350 calc module"
Assert-Contains -Path $cp350BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp350Stem)) -Description "CP350 binding adapter"
Assert-NotContains -Path $cp350BindingAdapter -Pattern 'supply_mass_flow_rate\s*:|cp_air\s*:|mixed_air_temperature\s*:|supply_temperature\s*:|DirectZonePurchasedAirCouplingInput|latest_numerical' -Description "CP350 binding scalar/DTO firewall"
Assert-Contains -Path $cp350ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp350Stem) + ':') -Description "CP350 scheduled output"
Assert-Contains -Path $cp350BindingTestsRoot -Pattern ([regex]::Escape("${cp350Stem}_tests.rs")) -Description "CP350 binding tests"
Assert-Contains -Path $cp350InitState -Pattern $cp350Stem -Description "CP350 init state"
Assert-Contains -Path $cp350InitUnit -Pattern $cp350Stem -Description "CP350 unit state"
Assert-Contains -Path $cp350InitWitnessRoot -Pattern $cp350Stem -Description "CP350 witness module"
Assert-Contains -Path $cp350CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp350Stem) + '_validation;') -Description "CP350 coupled validator"
Assert-Contains -Path $cp350Coupled -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment' -Description "coupled CP349 predecessor"
Assert-Contains -Path $cp350Coupled -Pattern '(?s)assigned\s*\.checked_mul\(.*?SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked 8Q"
Assert-Contains -Path $cp350Coupled -Pattern 'direct_constant_sensible_heat_ratio_sensible_output_assignment_count",\s*0' -Description "coupled direct Q zero"
Assert-NotContains -Path $cp350Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled DTO firewall"
Assert-Contains -Path $cp350FixtureRoot -Pattern $cp350Stem -Description "CP350 fixture registration"
Assert-Contains -Path $cp350PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp350PipelineStem) + ';') -Description "CP350 pipeline module"
Assert-Contains -Path $cp350PipelineRoot -Pattern ('"' + $cp350Lifecycle + '":\s*result\s*\.' + $cp350Lifecycle) -Description "CP350 lifecycle JSON"
Assert-Contains -Path $cp350PipelineValidation -Pattern 'cp_air_assignment_cp349' -Description "pipeline CP349 predecessor"
Assert-Contains -Path $cp350PipelineValidation -Pattern '(?s)assigned\s*\.checked_mul\(.*?SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 8Q"
Assert-Contains -Path $cp350PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp419_lifecycle_evidence' -Description "cumulative firewall"
Assert-Contains -Path $cp350ArbitraryTests -Pattern $cp350Lifecycle -Description "arbitrary CP350 lifecycle"
foreach ($field in @(
        "supply_mass_flow_rate_kg_per_s", "cp_air_j_per_kg_k",
        "supply_mass_flow_rate_times_cp_air_w_per_k", "mixed_air_temperature_c",
        "supply_temperature_c", "mixed_air_minus_supply_temperature_k",
        "calculated_cooling_sensible_output_w", "cooling_sensible_output_w"
    )) {
    Assert-Contains -Path $cp350SnapshotSerialization -Pattern ('"' + $field + '"') -Description "CP350 JSON '$field'"
    Assert-Contains -Path $cp350SnapshotSerialization -Pattern ('"' + $field + '_ieee_bits"') -Description "CP350 JSON bits '$field'"
}
Assert-Contains -Path $cp350SnapshotSerialization -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "CP350 nonfinite numeric null"
Assert-Contains -Path $cp350SnapshotSerialization -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "CP350 authoritative bits"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches($algorithmText, '(?m)^\s*"CP350 supersedes only CP349[^"\r\n]+",\s*$')
$capabilityAddenda = [regex]::Matches($capabilityText, '(?m)^\s*"CP350 additionally requires[^"\r\n]+",\s*$')
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP350 addenda"
}
foreach ($claim in @($algorithmAddenda) + @($capabilityAddenda)) {
    foreach ($pattern in @(
            $cp350SourceHash, 'physical executable line 2217', 'line 2218',
            $cp350Sites[0], $cp350Sites[1], $cp350Sites[2], $cp350Sites[3],
            $cp350Sites[4], $cp350Sites[5], $cp350Sites[6], $cp350Sites[7],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'Q=K', 'source_site_execution_count=8\*Q',
            'C0=S', 'Q=H=CSH=0', 'direct.*?completed recursive CP349',
            'private.*?counterfactual', 'CP330.*?CP329.*?CP345.*?CP334-or-CP344.*?completed recursive',
            'coordinated.*?corruption.*?transactionally', 'CP330', 'CP329',
            'CP334.*?G/F', 'CP344.*?L', 'CP331/CP338', 'without reassociation',
            'CP349-to-CP350-to-unchanged-numerical', $cp350Lifecycle,
            'DirectZonePurchasedAirCouplingInput', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', 'Roadmap (?:promotion|state)'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP350 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp350Stem/release\.rs::advance_direct_no_oa_calc_$cp350Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp350Stem\.rs::purchased_air_calc_${cp350Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp350Stem\.rs::${cp350TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp350Stem\.rs::${cp350TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($algorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP350 target count failed for '$($target.Pattern)'"
    }
}
$documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP350 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP350 Source-Ordered Cooling Positive-Supply Constant-Sensible-Heat-Ratio Sensible-Output Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP350 Constant-Sensible-Heat-Ratio Sensible-Output Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP350 Constant-Sensible-Heat-Ratio Sensible-Output Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP350 Constant-Sensible-Heat-Ratio Sensible-Output Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $documentation) {
    $sectionMatches = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sectionMatches.Count -ne 1) {
        throw "CP350 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp350SourceHash, '2217', '2218', $cp350Sites[0], $cp350Sites[7],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'Q\s*=\s*K', '8\*Q', 'C0\s*=\s*S', 'Q\s*=\s*H\s*=\s*CSH\s*=\s*0',
            'CP330', 'CP349', 'CP329', 'CP334', 'CP344', 'CP345',
            '(?s)direct.*?CP349.*?completed recursive',
            '(?s)private.*?counterfactual',
            '(?s)CP330.*?CP329.*?CP345.*?CP334-or-CP344.*?completed recursive',
            '(?s)coordinated.*?corruption.*?transactionally',
            'CP349-to-CP350-to-unchanged-numerical', $cp350Lifecycle,
            'DirectZonePurchasedAirCouplingInput', '32\s+algorithms', '293\s+routines',
            'Roadmap'
        )) {
        if ($sectionMatches[0].Value -notmatch $pattern) {
            throw "CP350 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP350\b' -Description "CP350 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP350 supersedes only CP349' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP350 additionally requires' -Description "generated capability addendum"

# Historical order, cumulative firewalls, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..349 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment' -Description "historical CP350 binding order"
}
foreach ($historical in 334..349) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp419_lifecycle_evidence' -Description "historical CP363 firewall"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp349AuditIndex = $mainAuditText.IndexOf("cp349-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1")
$cp350AuditIndex = $mainAuditText.IndexOf("cp350-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
$completionIndex = $mainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp349AuditIndex -lt 0 -or $cp350AuditIndex -le $cp349AuditIndex -or $completionIndex -le $cp350AuditIndex) {
    throw "Master audit must dot-source CP350 after CP349 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 357' -Description "CP350 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP350 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp350-' -Description "CP350 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp350-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment\.ps1::dot_sources' -Description "CP350 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 357 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 117 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

# Audit self-tests reject arithmetic, algebra, DTO, firewall, and IEEE drift.
Assert-Cp350MutationRejected -Original $cp350TransitionText -Mutated $cp350TransitionText.Replace(
    'let output = first_product * difference;',
    'let output = input.supply_mass_flow_rate_kg_per_s * (cp_air * difference);'
) -Validator { param($text) Assert-Cp350TransitionContract -Text $text } -Description "source AST reassociation"
Assert-Cp350MutationRejected -Original $cp350RuntimeText -Mutated $cp350RuntimeText.Replace(
    'assignments.checked_mul(',
    'assignments.checked_add('
) -Validator { param($text) Assert-Cp350RuntimeContract -Text $text } -Description "8Q multiplier corruption"
Assert-Cp350MutationRejected -Original $cp350PrefixText -Mutated $cp350PrefixText.Replace(
    'completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(',
    'bypassed_completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent('
) -Validator { param($text) Assert-Cp350RecursiveOwnerContract -PrefixText $text -Cp349PrefixText $cp349PrefixText } -Description "CP330 recursive owner-proof removal"
Assert-Cp350MutationRejected -Original $cp349PrefixText -Mutated $cp349PrefixText.Replace(
    'private_active_counterfactual_links_to_direct_release(',
    'bypassed_private_active_counterfactual_links_to_direct_release('
) -Validator { param($text) Assert-Cp350RecursiveOwnerContract -PrefixText $cp350PrefixText -Cp349PrefixText $text } -Description "restricted CP349 counterfactual-proof removal"
Assert-Cp350MutationRejected -Original $cp350BindingText -Mutated $cp350BindingText.Replace(
    'DirectZonePurchasedAirCouplingInput {',
    "DirectZonePurchasedAirCouplingInput {`n            cp350_evidence: calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment,"
) -Validator { param($text) Assert-Cp350BindingContract -Text $text } -Description "numerical DTO injection"
$cp350FirewallPattern = 'result\s*\.\s*' + [regex]::Escape($cp350Lifecycle) + '\s*\.\s*is_some\s*\(\s*\)'
$cp350FirewallMutation = [regex]::Replace($cp350PipelineRootText, $cp350FirewallPattern, ('result.' + $cp350Lifecycle + '.is_none()'), 1)
Assert-Cp350MutationRejected -Original $cp350PipelineRootText -Mutated $cp350FirewallMutation -Validator { param($text) Assert-Cp350PipelineRootContract -Text $text } -Description "non-direct firewall mutation"
Assert-Cp350MutationRejected -Original $cp350SnapshotJsonText -Mutated $cp350SnapshotJsonText.Replace(
    '"cooling_sensible_output_w_ieee_bits"',
    '"cooling_sensible_output_w_ieee_bits_mutated"'
) -Validator {
    param($text)
    if ($text -notmatch '"cooling_sensible_output_w_ieee_bits"') {
        throw "CP350 result IEEE key missing"
    }
} -Description "authoritative IEEE key mutation"
Write-Host "CP350 constant-SHR sensible-output-assignment structure audit passed."
