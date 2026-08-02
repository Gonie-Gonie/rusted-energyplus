# CP332 maps only PurchasedAirManager.cc physical executable line 2186: the
# complete Cooling positive-supply temperature arithmetic and assignment.
# Physical line 2187 is the first excluded lexical executable and CP333 edge.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp332Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment.rs"
$cp332State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment\state.rs"
$cp332Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment\transition.rs"
$cp332Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment\release.rs"
$cp332PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment\release\prefix_validation.rs"
$cp332RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment\release\runtime_validation.rs"
$cp332SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment\release\snapshot_validation.rs"
$cp332Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment\tests\mod.rs"
$cp332ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_assignment\tests\release_corruption.rs"
$cp332CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp332Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp332Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp332ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp332BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_temperature_assignment.rs"
$cp332BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp332BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_temperature_assignment_tests.rs"
$cp332InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp332InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp332InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp332InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_temperature_assignment.rs"
$cp332CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp332CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_temperature_assignment_validation.rs"
$cp332CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp332CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_temperature_assignment_fixture.rs"
$cp332CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp332PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp332Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_assignment.rs"
$cp332PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_assignment\validation.rs"
$cp332PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_assignment\serialization.rs"
$cp332PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_assignment\serialization\snapshot.rs"
$cp332RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp332DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_temperature_assignment_assertions.rs"
$cp332NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp332RequiredFile in @(
        $cp332Module,
        $cp332State,
        $cp332Transition,
        $cp332Release,
        $cp332PrefixValidation,
        $cp332RuntimeValidation,
        $cp332SnapshotValidation,
        $cp332Tests,
        $cp332ReleaseCorruptionTests,
        $cp332ScheduledOutput,
        $cp332BindingAdapter,
        $cp332BindingTests,
        $cp332InitWitness,
        $cp332CoupledValidation,
        $cp332CoupledFixture,
        $cp332Pipeline,
        $cp332PipelineValidation,
        $cp332PipelineSerialization,
        $cp332PipelineSnapshotSerialization,
        $cp332DirectAssertions
    )) {
    Assert-FileExists -Path $cp332RequiredFile -Description "CP332 supply-temperature-assignment structure"
}
Assert-LineLimit -Path $cp332Release -Limit 800 -Description "CP332 release root module"
Assert-LineLimit -Path $cp332RuntimeValidation -Limit 800 -Description "CP332 runtime validation module"
Assert-LineLimit -Path $cp332CoupledValidation -Limit 800 -Description "CP332 coupled validation module"
Assert-LineLimit -Path $cp332Pipeline -Limit 800 -Description "CP332 pipeline module"

# Exact physical statement boundary and eight textual sites.
Assert-Contains -Path $cp332Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2186' -Description "CP332 exact physical source boundary"
Assert-Contains -Path $cp332Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2187' -Description "CP332 first excluded physical executable"
Assert-Contains -Path $cp332Module -Pattern 'Exact eight textual source sites represented by CP332' -Description "CP332 exact lexical-site count"
Assert-ExactStringArray -Path $cp332Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-zone-cooling-setpoint-load",
    "read-local-cp-air-for-denominator-product",
    "read-retained-supply-mass-flow-rate-for-denominator-product",
    "calculate-cp-air-times-supply-mass-flow-rate",
    "calculate-zone-cooling-setpoint-load-divided-by-denominator-product",
    "read-zone-node-temperature",
    "add-zone-node-temperature-to-load-derived-temperature",
    "assign-purchased-air-supply-temperature"
) -Description "CP332 exact eight lexical source sites"
Assert-Contains -Path $cp332Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot' -Description "CP332 public snapshot"
Assert-Contains -Path $cp332State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState' -Description "CP332 persistent state"
Assert-Contains -Path $cp332Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary' -Description "CP332 lifecycle summary"
Assert-Contains -Path $cp332Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary\s*\(' -Description "CP332 lifecycle accessor"
Assert-Contains -Path $cp332Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment\s*\(' -Description "CP332 exact-direct wrapper"
Assert-Contains -Path $cp332Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_temperature_assignment_state\s*\(' -Description "CP332 pure transition"
Assert-Contains -Path $cp332CalcRoot -Pattern 'mod cooling_positive_supply_temperature_assignment;' -Description "CP332 calc module declaration"
Assert-Contains -Path $cp332CalcRoot -Pattern 'pub use (?:cooling_positive_supply_temperature_assignment::\*;|\{[^}]*cooling_positive_supply_temperature_assignment::\*)' -Description "CP332 calc public surface"

# Only a CP331 assignment route executes the raw expression. The source
# grouping is multiplication, division, addition, then assignment.
Assert-PatternsInOrder -Path $cp332Transition -Patterns @(
    'let assignment_executed = predecessor\.cp_air_assignment_executed;',
    'debug_assert_eq!\(assignment_executed, active_input\.is_some\(\)\);',
    'let zone_cooling_setpoint_load_w =',
    'active_input\.map\(\|input\| input\.zone_cooling_setpoint_load_w\);',
    'let cp_air_j_per_kg_k = active_input\.map\(\|input\| input\.cp_air_j_per_kg_k\);',
    'let supply_mass_flow_rate_kg_per_s =',
    'active_input\.map\(\|input\| input\.supply_mass_flow_rate_kg_per_s\);',
    'input\.cp_air_j_per_kg_k \* input\.supply_mass_flow_rate_kg_per_s',
    'input\.zone_cooling_setpoint_load_w / denominator',
    'let zone_node_temperature_c = active_input\.map\(\|input\| input\.zone_node_temperature_c\);',
    'quotient \+ zone_node_temperature',
    'let supply_temperature_c = calculated_supply_temperature_c;',
    'if predecessor\.unit_off_skipped',
    'state\.unit_off_skip_count \+= 1;',
    'else if predecessor\.non_cooling_skipped',
    'state\.non_cooling_skip_count \+= 1;',
    'else if predecessor\.positive_guard_false_fallthrough_skipped',
    'state\.positive_guard_false_fallthrough_skip_count \+= 1;',
    'state\.witnessed_positive_guard_false_fallthrough_skip_count \+= 1;',
    'state\.supply_temperature_assignment_count \+= 1;',
    'state\.source_site_execution_count \+=',
    'PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER\.len\(\);',
    'state\.zone_cooling_setpoint_load_read_count \+= 1;',
    'state\.cp_air_read_count \+= 1;',
    'state\.supply_mass_flow_rate_read_count \+= 1;',
    'state\.cp_air_times_supply_mass_flow_rate_calculation_count \+= 1;',
    'state\.zone_cooling_setpoint_load_over_denominator_calculation_count \+= 1;',
    'state\.zone_node_temperature_read_count \+= 1;',
    'state\.supply_temperature_calculation_count \+= 1;',
    'state\.supply_temperature_assignment_write_count \+= 1;',
    'state\.witnessed_supply_temperature_assignment_count \+= 1;'
) -Description "CP332 four-route raw source-order transition"
Assert-NotContains -Path $cp332Transition -Pattern 'mul_add|recip\(|clamp\(|\.min\(|\.max\(|minimum_cooling|mixed_air_temperature|capacity|enthalpy|humidity|2187|2189|2454|2465' -Description "CP332 reassociation or line-2187-and-later behavior"
Assert-NotContains -Path $cp332Transition -Pattern 'zone_cooling_setpoint_load_w\s*/\s*input\.cp_air_j_per_kg_k\s*/\s*input\.supply_mass_flow_rate_kg_per_s' -Description "CP332 two-division reassociation"
Assert-Contains -Path $cp332Tests -Pattern 'positive_route_executes_exact_eight_sites_and_retains_each_ieee_intermediate' -Description "CP332 exact eight-site active regression"
Assert-Contains -Path $cp332Tests -Pattern 'source_grouping_uses_product_then_division_without_reassociation' -Description "CP332 grouping-sensitive regression"
Assert-Contains -Path $cp332Tests -Pattern 'admitted_operands_preserve_infinite_derived_results_and_signed_zero_quotients' -Description "CP332 raw IEEE regression"
Assert-Contains -Path $cp332Tests -Pattern 'skipped_routes_execute_no_sites_or_operand_reads' -Description "CP332 three skipped routes regression"
Assert-Contains -Path $cp332Tests -Pattern 'counters_partition_all_four_routes_and_count_eight_sites_per_assignment' -Description "CP332 four-route counter regression"

# Exact snapshots preserve every raw intermediate and distinguish all four
# routes; runtime counters derive eight sites per CP331 assignment.
foreach ($cp332SnapshotField in @(
        "zone_cooling_setpoint_load_read",
        "zone_cooling_setpoint_load_w",
        "cp_air_read",
        "cp_air_j_per_kg_k",
        "supply_mass_flow_rate_read",
        "supply_mass_flow_rate_kg_per_s",
        "cp_air_times_supply_mass_flow_rate_calculated",
        "cp_air_times_supply_mass_flow_rate_w_per_k",
        "zone_cooling_setpoint_load_over_denominator_calculated",
        "zone_cooling_setpoint_load_over_denominator_c",
        "zone_node_temperature_read",
        "zone_node_temperature_c",
        "supply_temperature_calculated",
        "calculated_supply_temperature_c",
        "supply_temperature_assigned",
        "supply_temperature_c"
    )) {
    Assert-Contains -Path $cp332SnapshotValidation -Pattern $cp332SnapshotField -Description "CP332 snapshot field '$cp332SnapshotField'"
}
Assert-Contains -Path $cp332SnapshotValidation -Pattern 'fn skipped_snapshot_is_exact' -Description "CP332 exact skipped snapshot"
Assert-Contains -Path $cp332SnapshotValidation -Pattern 'fn snapshots_match_bit_exact' -Description "CP332 exact retained snapshot matcher"
Assert-Contains -Path $cp332SnapshotValidation -Pattern 'cp_air \* supply_mass_flow_rate' -Description "CP332 snapshot denominator grouping"
Assert-Contains -Path $cp332SnapshotValidation -Pattern 'zone_cooling_setpoint_load / denominator' -Description "CP332 snapshot division"
Assert-Contains -Path $cp332SnapshotValidation -Pattern 'quotient \+ zone_node_temperature' -Description "CP332 snapshot addition"
Assert-Contains -Path $cp332RuntimeValidation -Pattern 'state\s*\.\s*supply_temperature_assignment_count\s*\.\s*checked_mul\((?:8|PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER\s*\.\s*len\(\))\)' -Description "CP332 dynamic source-site formula"
foreach ($cp332Counter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "supply_temperature_assignment_count",
        "source_site_execution_count",
        "zone_cooling_setpoint_load_read_count",
        "cp_air_read_count",
        "supply_mass_flow_rate_read_count",
        "cp_air_times_supply_mass_flow_rate_calculation_count",
        "zone_cooling_setpoint_load_over_denominator_calculation_count",
        "zone_node_temperature_read_count",
        "supply_temperature_calculation_count",
        "supply_temperature_assignment_write_count"
    )) {
    Assert-Contains -Path $cp332RuntimeValidation -Pattern $cp332Counter -Description "CP332 runtime counter '$cp332Counter'"
}
Assert-Contains -Path $cp332State -Pattern 'pub\(super\) witnessed_positive_guard_false_fallthrough_skip_count:\s*usize' -Description "CP332 private guard-false witness count"
Assert-Contains -Path $cp332State -Pattern 'pub\(super\) witnessed_supply_temperature_assignment_count:\s*usize' -Description "CP332 private assignment witness count"
Assert-Contains -Path $cp332RuntimeValidation -Pattern '(?s)pending_supply_temperature_assignment_state_is_consistent\(.*?unit_off_skip_count.*?predecessor_state\.unit_off_skip_count.*?non_cooling_skip_count.*?predecessor_state\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor_state\.positive_guard_false_fallthrough_skip_count.*?supply_temperature_assignment_count.*?predecessor_state\.cp_air_assignment_count' -Description "CP332 pending CP331 four-route parity"
Assert-Contains -Path $cp332RuntimeValidation -Pattern '(?s)completed_supply_temperature_assignment_state_is_consistent\(.*?state\.transition_count == predecessor\.transition_count.*?state\.unit_off_skip_count == predecessor\.unit_off_skip_count.*?state\.non_cooling_skip_count == predecessor\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor\.positive_guard_false_fallthrough_skip_count.*?supply_temperature_assignment_count.*?predecessor\.cp_air_assignment_count' -Description "CP332 completed CP331 four-route parity"
foreach ($cp332PreflightCounter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "witnessed_positive_guard_false_fallthrough_skip_count",
        "supply_temperature_assignment_count",
        "source_site_execution_count",
        "zone_cooling_setpoint_load_read_count",
        "cp_air_read_count",
        "supply_mass_flow_rate_read_count",
        "cp_air_times_supply_mass_flow_rate_calculation_count",
        "zone_cooling_setpoint_load_over_denominator_calculation_count",
        "zone_node_temperature_read_count",
        "supply_temperature_calculation_count",
        "supply_temperature_assignment_write_count",
        "witnessed_supply_temperature_assignment_count"
    )) {
    Assert-Contains -Path $cp332RuntimeValidation -Pattern ($cp332PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP332 checked preflight counter '$cp332PreflightCounter'"
}

# Release derives all operands from retained owners and conditionally performs
# the only live Zone-node temperature read. It accepts no duplicate scalars or
# numerical DTO.
Assert-Contains -Path $cp332Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp331: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,\s*zone_state: &ZoneHeatBalanceState,' -Description "CP332 exact wrapper arguments"
Assert-Contains -Path $cp332Release -Pattern 'let zone_node_temperature_c = zone_state\.mean_air_temperature_c;' -Description "CP332 conditional live Zone-node temperature read"
Assert-Contains -Path $cp332Release -Pattern 'completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent\s*\(' -Description "CP332 recursive CP331 completed proof"
Assert-Contains -Path $cp332Release -Pattern 'cooling_positive_supply_cp_air_assignment_latest_witness\s*\(' -Description "CP332 retained CP331 private witness"
Assert-Contains -Path $cp332Release -Pattern 'cooling_positive_supply_temperature_assignment_latest_witness\s*\(' -Description "CP332 private assignment witness"
Assert-Contains -Path $cp332PrefixValidation -Pattern 'zone_cooling_setpoint_load' -Description "CP332 retained CP310 demand provenance"
Assert-Contains -Path $cp332PrefixValidation -Pattern 'cp_air_j_per_kg_k' -Description "CP332 retained CP331 CpAir provenance"
Assert-Contains -Path $cp332PrefixValidation -Pattern 'supply_mass_flow_rate_kg_per_s' -Description "CP332 retained CP330 flow provenance"
Assert-Contains -Path $cp332PrefixValidation -Pattern 'zone_node_temperature_c' -Description "CP332 live and retained Zone-temperature lineage"
Assert-Contains -Path $cp332PrefixValidation -Pattern 'recirculation_temperature_c' -Description "CP332 retained CP329 recirculation-temperature lineage"
Assert-Contains -Path $cp332PrefixValidation -Pattern 'mixed_air_temperature_c' -Description "CP332 retained CP329 mixed-air-temperature lineage"
Assert-Contains -Path $cp332Release -Pattern '(?s)completed_direct_cooling_positive_supply_temperature_assignment_is_consistent\(.*?completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent\(.*?completed_supply_temperature_assignment_state_is_consistent' -Description "CP332 recursive completed chain"
Assert-NotContains -Path $cp332Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment\([^)]*(?:zone_cooling_setpoint_load|cp_air|supply_mass_flow_rate|zone_node_temperature)\w*\s*:\s*f64' -Description "duplicate caller scalar in CP332 release"
Assert-NotContains -Path $cp332Release -Pattern 'energyplus_psy_cp_air_fn_w|latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|complete_direct_zone_purchased_air_coupling' -Description "CpAir recomputation or numerical DTO dependency in CP332 release"

$cp332ReleaseText = Read-RepoText -Path $cp332Release
$cp332ReleaseWrapper = [regex]::Match(
    $cp332ReleaseText,
    '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment\(.*?(?=\r?\nfn )'
)
if (-not $cp332ReleaseWrapper.Success) {
    throw "CP332 exact release wrapper must remain structurally bounded"
}
$cp332WrapperText = $cp332ReleaseWrapper.Value
$cp332MutationIndex = $cp332WrapperText.IndexOf("let snapshot = {")
if ($cp332MutationIndex -lt 0) {
    throw "CP332 release mutation boundary must remain explicit"
}
foreach ($cp332ValidationCall in @(
        "pending_supply_temperature_assignment_state_is_consistent(",
        "next_supply_temperature_assignment_transition_fits(",
        "completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent("
    )) {
    $cp332ValidationIndex = $cp332WrapperText.IndexOf($cp332ValidationCall)
    if ($cp332ValidationIndex -lt 0 -or $cp332ValidationIndex -ge $cp332MutationIndex) {
        throw "CP332 validation '$cp332ValidationCall' must complete before mutation"
    }
}
Assert-PatternsInOrder -Path $cp332Release -Patterns @(
    'let snapshot = \{',
    '\.get_mut\(&selected\)',
    'advance_cooling_positive_supply_temperature_assignment_state\(',
    'set_cooling_positive_supply_temperature_assignment_latest_witness\(selected, snapshot\)'
) -Description "CP332 validated transition and private-witness mutation order"
Assert-Contains -Path $cp332ReleaseCorruptionTests -Pattern 'public_release_commits_raw_source_grouping_once_and_rejects_replay' -Description "CP332 release commit/grouping/replay regression"
Assert-Contains -Path $cp332ReleaseCorruptionTests -Pattern 'skipped_routes_do_not_read_or_validate_live_zone_node_temperature' -Description "CP332 skipped-route live-read regression"
Assert-Contains -Path $cp332ReleaseCorruptionTests -Pattern 'live_zone_temperature_drift_fails_closed_and_transactionally' -Description "CP332 live Zone-temperature lineage regression"
Assert-Contains -Path $cp332ReleaseCorruptionTests -Pattern 'forged_cp331_snapshot_or_private_witness_fails_without_mutation' -Description "CP332 CP331/private-witness corruption regression"
Assert-Contains -Path $cp332ReleaseCorruptionTests -Pattern 'private_latest_or_witness_corruption_is_fail_closed_and_transactional' -Description "CP332 private latest/witness corruption regression"
Assert-Contains -Path $cp332ReleaseCorruptionTests -Pattern 'checked_preflight_rejects_every_active_counter_overflow_without_mutation' -Description "CP332 checked preflight regression"

# Runtime-root private witnesses, binding order, and direct scheduled evidence.
Assert-NotContains -Path $cp332InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_temperature_assignment_latest_witnesses:' -Description "public runtime-root CP332 witness map"
Assert-Contains -Path $cp332InitWitnessRoot -Pattern 'mod cooling_positive_supply_temperature_assignment;' -Description "runtime-root CP332 witness module"
Assert-Contains -Path $cp332InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_temperature_assignment_latest_witness\s*\(' -Description "runtime-root CP332 witness getter"
Assert-Contains -Path $cp332InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_temperature_assignment_latest_witness\s*\(' -Description "runtime-root CP332 witness setter"
Assert-Contains -Path $cp332InitState -Pattern 'pub calc_cooling_positive_supply_temperature_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState' -Description "per-unit CP332 persistent state"
Assert-Contains -Path $cp332InitUnit -Pattern '(?s)calc_cooling_positive_supply_temperature_assignment:\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new\(system\)' -Description "per-unit CP332 state initialization"

$cp332BindingText = Read-RepoText -Path $cp332Binding
$cp331BindingIndexForCp332 = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_cp_air_assignment =")
$cp332BindingIndex = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_assignment =")
$cp333BindingIndexForCp332 = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_minimum_limit =")
$cp334BindingIndexForCp332 = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_mixed_air_limit =")
$cp335BindingIndexForCp332 = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndexForCp332 = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp332 = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp332 = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp332 = $cp332BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp332 = $cp332BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp331BindingIndexForCp332 -lt 0 -or
    $cp332BindingIndex -le $cp331BindingIndexForCp332 -or
    $cp333BindingIndexForCp332 -le $cp332BindingIndex -or
    $cp334BindingIndexForCp332 -le $cp333BindingIndexForCp332 -or
    $cp335BindingIndexForCp332 -le $cp334BindingIndexForCp332 -or
    $cp336BindingIndexForCp332 -le $cp335BindingIndexForCp332 -or
    $cp337BindingIndexForCp332 -le $cp336BindingIndexForCp332 -or
    $cp338BindingIndexForCp332 -le $cp337BindingIndexForCp332 -or
    $cp339BindingIndexForCp332 -le $cp338BindingIndexForCp332 -or
    $numericalBindingIndexForCp332 -le $cp339BindingIndexForCp332
) {
    throw "Binding must retain exact CP331 -> CP332 -> CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $cp332Binding -Pattern '(?s)let calculation_cooling_positive_supply_temperature_assignment =\s*advance_positive_supply_temperature_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_cp_air_assignment,\s*&\*input\.zone_state,\s*\)\?;' -Description "binding exact CP331-to-CP332 adapter call"
Assert-Contains -Path $cp332BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_temperature_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,\s*zone_state: &ZoneHeatBalanceState,' -Description "CP332 binding adapter arguments"
Assert-Contains -Path $cp332BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment\(\s*runtime,\s*system,\s*predecessor,\s*zone_state,\s*\).*?CalculationCoolingPositiveSupplyTemperatureAssignment' -Description "CP332 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp332BindingAdapter -Pattern 'complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|DirectZonePurchasedAirCouplingInput' -Description "numerical DTO dependency in CP332 binding adapter"
Assert-Contains -Path $cp332ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_temperature_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot' -Description "CP332 scheduled output evidence"
Assert-Contains -Path $cp332BindingTestsRoot -Pattern '#\[rustfmt::skip\]\s*#\[path = "binding/cooling_positive_supply_temperature_assignment_tests\.rs"\]\s*mod cooling_positive_supply_temperature_assignment_tests;' -Description "CP332 binding test module path"
Assert-Contains -Path $cp332BindingTests -Pattern 'scheduled_binding_assigns_source_grouped_supply_temperature_bit_exactly' -Description "CP332 active binding regression"
Assert-Contains -Path $cp332BindingTests -Pattern 'scheduled_binding_skips_cp332_after_the_active_positive_guard_falls_through' -Description "CP332 guard-false binding regression"
Assert-Contains -Path $cp332BindingTests -Pattern 'scheduled_binding_preserves_unit_off_and_non_cooling_cp332_skip_routes' -Description "CP332 UnitOff/non-cooling binding regressions"
$cp332BindingCall = [regex]::Match(
    $cp332BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_assignment =\s*advance_positive_supply_temperature_assignment\([^;]+?\)\?;'
)
if (-not $cp332BindingCall.Success) {
    throw "Binding must retain the complete CP332 exact release call"
}
$cp332BindingCallEnd = $cp332BindingCall.Index + $cp332BindingCall.Length
$cp333BindingCallForCp332 = [regex]::Match(
    $cp332BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_minimum_limit =\s*advance_positive_supply_temperature_minimum_limit\([^;]+?\)\?;'
)
if (-not $cp333BindingCallForCp332.Success) {
    throw "Binding must retain the complete CP333 exact release call after CP332"
}
$cp333BindingCallEndForCp332 =
    $cp333BindingCallForCp332.Index + $cp333BindingCallForCp332.Length
$cp334BindingCallForCp332 = [regex]::Match(
    $cp332BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
if (-not $cp334BindingCallForCp332.Success) {
    throw "Binding must retain the complete CP334 exact release call after CP333"
}
$cp334BindingCallEndForCp332 =
    $cp334BindingCallForCp332.Index + $cp334BindingCallForCp332.Length
$cp335BindingCallForCp332 = [regex]::Match(
    $cp332BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
if (-not $cp335BindingCallForCp332.Success) {
    throw "Binding must retain the complete CP335 exact release call after CP334"
}
$cp335BindingCallEndForCp332 =
    $cp335BindingCallForCp332.Index + $cp335BindingCallForCp332.Length
$cp336BindingCallForCp332 = [regex]::Match(
    $cp332BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
if (-not $cp336BindingCallForCp332.Success) {
    throw "Binding must retain the complete CP336 exact release call after CP335"
}
$cp336BindingCallEndForCp332 =
    $cp336BindingCallForCp332.Index + $cp336BindingCallForCp332.Length
$cp337BindingCallForCp332 = [regex]::Match(
    $cp332BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
if (-not $cp337BindingCallForCp332.Success) {
    throw "Binding must retain the complete CP337 exact release call after CP336"
}
$cp337BindingCallEndForCp332 =
    $cp337BindingCallForCp332.Index + $cp337BindingCallForCp332.Length
$cp338BindingCallForCp332 = [regex]::Match(
    $cp332BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
if (-not $cp338BindingCallForCp332.Success) {
    throw "Binding must retain the complete CP338 exact release call after CP337"
}
$cp338BindingCallEndForCp332 =
    $cp338BindingCallForCp332.Index + $cp338BindingCallForCp332.Length
$cp339BindingCallForCp332 = [regex]::Match(
    $cp332BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (-not $cp339BindingCallForCp332.Success) {
    throw "Binding must retain the complete CP339 exact release call after CP338"
}
$cp339BindingCallEndForCp332 =
    $cp339BindingCallForCp332.Index + $cp339BindingCallForCp332.Length
if (
    $cp333BindingIndexForCp332 -lt $cp332BindingCallEnd -or
    $cp334BindingIndexForCp332 -lt $cp333BindingCallEndForCp332 -or
    $cp335BindingIndexForCp332 -lt $cp334BindingCallEndForCp332 -or
    $cp336BindingIndexForCp332 -lt $cp335BindingCallEndForCp332 -or
    $cp337BindingIndexForCp332 -lt $cp336BindingCallEndForCp332 -or
    $cp338BindingIndexForCp332 -lt $cp337BindingCallEndForCp332 -or
    $cp339BindingIndexForCp332 -lt $cp338BindingCallEndForCp332 -or
    $numericalBindingIndexForCp332 -lt $cp339BindingCallEndForCp332
) {
    throw "CP332, CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
$postCp332BeforeCp333 = $cp332BindingText.Substring(
    $cp332BindingCallEnd,
    $cp333BindingIndexForCp332 - $cp332BindingCallEnd
)
$postCp332BeforeCp333Code = [regex]::Replace($postCp332BeforeCp333, '(?m)//.*$', '')
if ($postCp332BeforeCp333Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP332 and before CP333"
}
$postCp333BeforeCp334ForCp332 = $cp332BindingText.Substring(
    $cp333BindingCallEndForCp332,
    $cp334BindingIndexForCp332 - $cp333BindingCallEndForCp332
)
$postCp333BeforeCp334CodeForCp332 =
    [regex]::Replace($postCp333BeforeCp334ForCp332, '(?m)//.*$', '')
if ($postCp333BeforeCp334CodeForCp332 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP333 and before CP334"
}
$postCp334BeforeCp335ForCp332 = $cp332BindingText.Substring(
    $cp334BindingCallEndForCp332,
    $cp335BindingIndexForCp332 - $cp334BindingCallEndForCp332
)
$postCp334BeforeCp335CodeForCp332 =
    [regex]::Replace($postCp334BeforeCp335ForCp332, '(?m)//.*$', '')
if ($postCp334BeforeCp335CodeForCp332 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP334 and before CP335"
}
$postCp335BeforeCp336ForCp332 = $cp332BindingText.Substring(
    $cp335BindingCallEndForCp332,
    $cp336BindingIndexForCp332 - $cp335BindingCallEndForCp332
)
$postCp335BeforeCp336CodeForCp332 =
    [regex]::Replace($postCp335BeforeCp336ForCp332, '(?m)//.*$', '')
if ($postCp335BeforeCp336CodeForCp332 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP335 and before CP336"
}
$postCp336BeforeCp337ForCp332 = $cp332BindingText.Substring(
    $cp336BindingCallEndForCp332,
    $cp337BindingIndexForCp332 - $cp336BindingCallEndForCp332
)
$postCp336BeforeCp337CodeForCp332 =
    [regex]::Replace($postCp336BeforeCp337ForCp332, '(?m)//.*$', '')
if ($postCp336BeforeCp337CodeForCp332 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP336 and before CP337"
}
$postCp337BeforeCp338ForCp332 = $cp332BindingText.Substring(
    $cp337BindingCallEndForCp332,
    $cp338BindingIndexForCp332 - $cp337BindingCallEndForCp332
)
$postCp337BeforeCp338CodeForCp332 =
    [regex]::Replace($postCp337BeforeCp338ForCp332, '(?m)//.*$', '')
if ($postCp337BeforeCp338CodeForCp332 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP337 and before CP338"
}
$postCp338BeforeCp339ForCp332 = $cp332BindingText.Substring(
    $cp338BindingCallEndForCp332,
    $cp339BindingIndexForCp332 - $cp338BindingCallEndForCp332
)
$postCp338BeforeCp339CodeForCp332 =
    [regex]::Replace($postCp338BeforeCp339ForCp332, '(?m)//.*$', '')
if ($postCp338BeforeCp339CodeForCp332 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP338 and before CP339"
}
$postCp339BeforeNumericalForCp332 = $cp332BindingText.Substring(
    $cp339BindingCallEndForCp332,
    $numericalBindingIndexForCp332 - $cp339BindingCallEndForCp332
)
$postCp339BeforeNumericalCodeForCp332 =
    [regex]::Replace($postCp339BeforeNumericalForCp332, '(?m)//.*$', '')
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp332 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp332,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\([^;]+?\)\?;)',
    ''
)
if ($postCp339BeforeNumericalCodeForCp332 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No helper other than the audited CP340 through CP348 releases may execute after CP339 and before numerical Calc"
}

# Coupled runtime and pipeline expose direct-only CP332 evidence and validate
# identity, four-route counts, raw grouping, and the recursive predecessor.
Assert-Contains -Path $cp332CoupledRuntime -Pattern 'mod cooling_positive_supply_temperature_assignment_validation;' -Description "coupled CP332 validator declaration"
Assert-Contains -Path $cp332CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_temperature_assignment_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary' -Description "coupled CP332 lifecycle"
Assert-Contains -Path $cp332CoupledRuntime -Pattern 'cooling_positive_supply_temperature_assignment_validation::snapshot_matches_release' -Description "coupled per-timestep CP332 validation"
Assert-Contains -Path $cp332CoupledRuntime -Pattern 'cooling_positive_supply_temperature_assignment_validation::validate_lifecycle' -Description "coupled final CP332 validation"
Assert-Contains -Path $cp332CoupledValidation -Pattern 'let predecessor = output\.calculation_cooling_positive_supply_cp_air_assignment;' -Description "coupled CP331 predecessor"
Assert-Contains -Path $cp332CoupledValidation -Pattern 'let snapshot = output\.calculation_cooling_positive_supply_temperature_assignment;' -Description "coupled CP332 output"
Assert-Contains -Path $cp332CoupledValidation -Pattern '(?s)state\s*\.\s*supply_temperature_assignment_count.{0,180}(?:8|PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER\s*\.\s*len\(\))' -Description "coupled CP332 dynamic source-site count"
Assert-Contains -Path $cp332CoupledValidation -Pattern 'cp_air.*\*.*supply_mass_flow_rate' -Description "coupled CP332 denominator grouping"
Assert-Contains -Path $cp332CoupledValidation -Pattern 'zone_cooling_setpoint_load.*\/.*denominator' -Description "coupled CP332 division"
Assert-Contains -Path $cp332CoupledValidation -Pattern 'quotient.*\+.*zone_node_temperature' -Description "coupled CP332 addition"
Assert-Contains -Path $cp332CoupledValidation -Pattern 'options_have_exact_bits' -Description "coupled CP332 exact-bit validation"
Assert-Contains -Path $cp332CoupledValidation -Pattern 'source_site_count_multiplication_overflow_fails_closed' -Description "coupled CP332 checked multiplication regression"
Assert-Contains -Path $cp332CoupledTests -Pattern 'cooling_positive_supply_temperature_assignment_partition_overflow_fails_closed' -Description "coupled CP332 checked partition regression"
Assert-Contains -Path $cp332CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_temperature_assignment_fixture;' -Description "coupled CP332 fixture declaration"
Assert-Contains -Path $cp332CoupledFixture -Pattern 'calculation_cooling_positive_supply_temperature_assignment' -Description "coupled CP332 fixture output"
Assert-Contains -Path $cp332PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_temperature_assignment;' -Description "pipeline CP332 module declaration"
Assert-Contains -Path $cp332PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle' -Description "pipeline CP332 lifecycle field and JSON key"
Assert-Contains -Path $cp332PipelineRoot -Pattern 'calc_cooling_positive_supply_temperature_assignment_lifecycle' -Description "pipeline CP332 coupled lifecycle transfer"
Assert-Contains -Path $cp332Pipeline -Pattern 'validate_direct_lifecycle' -Description "pipeline CP332 direct lifecycle validation"
Assert-Contains -Path $cp332PipelineValidation -Pattern 'validate_source_counters' -Description "pipeline CP332 source-counter validation"
Assert-Contains -Path $cp332PipelineValidation -Pattern 'source_counter_overflow_fails_closed' -Description "pipeline CP332 checked multiplication regression"
Assert-Contains -Path $cp332PipelineSerialization -Pattern 'zone_node_temperature_read_count' -Description "pipeline CP332 lifecycle serialization"
Assert-Contains -Path $cp332PipelineSnapshotSerialization -Pattern 'zone_node_temperature_read' -Description "pipeline CP332 Zone-node read serialization"
Assert-Contains -Path $cp332PipelineSnapshotSerialization -Pattern 'zone_node_temperature_c' -Description "pipeline CP332 Zone-node scalar serialization"
Assert-Contains -Path $cp332Pipeline -Pattern 'json_preserves_every_cp332_ieee_value_bit_pattern' -Description "pipeline CP332 exact IEEE serialization regression"
Assert-Contains -Path $cp332Pipeline -Pattern 'json_keeps_non_finite_bits_when_raw_value_is_null' -Description "pipeline CP332 nonfinite-bit serialization regression"
Assert-Contains -Path $cp332RunTests -Pattern 'cooling_positive_supply_temperature_assignment_assertions' -Description "direct-run CP332 assertion module"
Assert-Contains -Path $cp332RunTests -Pattern 'no_limit_cooling_publishes_active_cp331_and_cp332_json_lineage' -Description "direct-run CP332 active JSON lineage regression"
Assert-Contains -Path $cp332DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle' -Description "direct-run CP332 JSON evidence"
Assert-Contains -Path $cp332DirectAssertions -Pattern 'pub\(super\) fn assert_cooling_positive_supply_temperature_assignment' -Description "direct-run CP332 shared assertion"
Assert-Contains -Path $cp332DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*8\]\s*=' -Description "direct-run CP332 eight-site source-order declaration"
Assert-PatternsInOrder -Path $cp332DirectAssertions -Patterns @(
    '"read-zone-cooling-setpoint-load"',
    '"read-local-cp-air-for-denominator-product"',
    '"read-retained-supply-mass-flow-rate-for-denominator-product"',
    '"calculate-cp-air-times-supply-mass-flow-rate"',
    '"calculate-zone-cooling-setpoint-load-divided-by-denominator-product"',
    '"read-zone-node-temperature"',
    '"add-zone-node-temperature-to-load-derived-temperature"',
    '"assign-purchased-air-supply-temperature"'
) -Description "direct-run CP332 exact source order"
Assert-Contains -Path $cp332DirectAssertions -Pattern 'assignments \* SOURCE_ORDER\.len\(\) as u64' -Description "direct-run CP332 eight-site dynamic count"
Assert-Contains -Path $cp332DirectAssertions -Pattern 'cp_air \* supply_mass_flow_rate' -Description "direct-run CP332 denominator grouping"
Assert-Contains -Path $cp332DirectAssertions -Pattern 'zone_cooling_setpoint_load / denominator' -Description "direct-run CP332 division"
Assert-Contains -Path $cp332DirectAssertions -Pattern 'quotient \+ zone_node_temperature' -Description "direct-run CP332 addition"
Assert-Contains -Path $cp332NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle' -Description "non-direct CP332 null evidence"

# Ledger and capability registries repeat the boundary exactly twice without
# changing support, conformance, parent, routine, count, or Roadmap state.
$cp332AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp332AlgorithmAddenda = [regex]::Matches(
    $cp332AlgorithmText,
    '(?m)^\s*"CP332 supersedes only CP331[^"\r\n]+",\s*$'
)
if ($cp332AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP332 boundary addenda"
}
foreach ($cp332AlgorithmAddendum in $cp332AlgorithmAddenda) {
    foreach ($cp332Pattern in @(
            'physical executable line 2186',
            'exactly eight lexical sites',
            'textual inventory',
            '8 \* cp_air_assignment_count',
            '8 \* positive_supply_mass_flow_body_entries',
            'QZnCoolSP / \(CpAir \* SupplyMassFlowRate\) \+ ZoneTemp',
            'retained CP310',
            'bit-exact CP331',
            'retained CP330',
            'live Zone temperature',
            'CP318',
            'CP329',
            'same-call lineage evidence rather than substitute source operands',
            'no duplicate caller scalar',
            'CP331-to-CP332-to-numerical',
            'Physical line 2187 is the first excluded lexical executable and CP333 boundary',
            'line-2189',
            'line 2340',
            '2454-2461',
            '2465',
            'both parents remain `scaffold`/`none`',
            'Roadmap state stay unchanged'
        )) {
        if ($cp332AlgorithmAddendum.Value -notmatch $cp332Pattern) {
            throw "CP332 algorithm addendum missing '$cp332Pattern'"
        }
    }
}
foreach ($cp332TargetPattern in @(
        'cooling_positive_supply_temperature_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment',
        'cooling_positive_supply_temperature_assignment\.rs::purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary',
        'cooling_positive_supply_temperature_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState',
        'cooling_positive_supply_temperature_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary'
    )) {
    Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern $cp332TargetPattern -Description "CP332 algorithm target '$cp332TargetPattern'"
}

$cp332CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp332CapabilityAddenda = [regex]::Matches(
    $cp332CapabilityText,
    '(?m)^\s*"CP332 additionally requires[^"\r\n]+",\s*$'
)
if ($cp332CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP332 claim addenda"
}
foreach ($cp332CapabilityAddendum in $cp332CapabilityAddenda) {
    foreach ($cp332Pattern in @(
            'physical executable line 2186',
            'eight-site',
            'textual inventory',
            '8 \* cp_air_assignment_count',
            '8 \* positive_supply_mass_flow_body_entries',
            'QZnCoolSP / \(CpAir \* SupplyMassFlowRate\) \+ ZoneTemp',
            'retained CP310',
            'bit-exact CP331',
            'retained CP330',
            'live Zone temperature',
            'CP318',
            'CP329',
            'same-call lineage evidence, not substitute operands',
            'No duplicate caller scalar',
            'CP331-to-CP332-to-numerical',
            'Physical line 2187 is the first excluded lexical executable and CP333 boundary',
            'line-2189',
            '2340',
            '2454-2461',
            '2465',
            'This changes no support level',
            'both Calc routines remain `source_mapped`'
        )) {
        if ($cp332CapabilityAddendum.Value -notmatch $cp332Pattern) {
            throw "CP332 capability addendum missing '$cp332Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP332 supersedes only CP331' -Description "generated CP332 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP332 additionally requires' -Description "generated CP332 capability index"

# Every hand-authored contract repeats source, provenance, order,
# transactionality, first exclusion, and explicit non-promotion.
$cp332DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP332 maps only the single Cooling positive-supply.*?^Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP332 Source-Ordered Cooling Positive-Supply Temperature Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP332 Cooling Positive-Supply Temperature Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP332 Positive-Supply Temperature Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP332 Cooling Positive-Supply Temperature Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp332Documentation in $cp332DocumentationSections) {
    $cp332DocumentText = Read-RepoText -Path $cp332Documentation.Path
    $cp332Matches = [regex]::Matches($cp332DocumentText, $cp332Documentation.Pattern)
    if ($cp332Matches.Count -ne 1) {
        throw "CP332 documentation expected one scoped section in $($cp332Documentation.Path), found $($cp332Matches.Count)"
    }
    $cp332Section = $cp332Matches[0].Value
    foreach ($cp332Pattern in @(
            'physical\s+(?:executable\s+)?(?:`PurchasedAirManager\.cc`\s+)?line\s+2186',
            '(?:exactly )?eight(?:-site|\s+lexical|\s+sites)|all eight',
            '8 \* cp_air_assignment_count',
            '8 \* positive_supply_mass_flow_body_entries',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?i)active[- ]false|guard-false',
            'QZnCoolSP / \(CpAir \* SupplyMassFlowRate\)',
            '(?i)CP310',
            '(?i)CP331',
            '(?i)CP330',
            'ZoneHeatBalanceState::mean_air_temperature_c',
            '(?i)CP318',
            '(?i)CP329',
            '(?is)lineage.{0,180}(?:not substitute|not replace|without replacing)|same-call lineage evidence',
            '(?is)(?:No|no).{0,30}duplicate.{0,80}(?:scalar|input)',
            '(?is)psychrometric.{0,60}(?:re-evaluation|re-evaluate)|(?:re-evaluate).{0,60}psychrometric|PsyCpAirFnW',
            '(?i)latest/private witness|latest snapshot, private',
            '(?is)checked(?:-|\s+)arithmetic',
            '(?i)transaction|before mutation|state unchanged',
            'purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle',
            'CP331-to-CP332-to-numerical|immediately after CP331 and before',
            '(?is)(?:does not|neither).{0,120}(?:consume|reconcile).{0,180}numerical DTO',
            'line 2187',
            '(?i)CP333',
            'line-2189',
            '2340',
            '2454-2461',
            '2465',
            '(?i)scaffold',
            '(?i)source_mapped',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp332Section -notmatch $cp332Pattern) {
            throw "CP332 documentation in $($cp332Documentation.Path) missing '$cp332Pattern'"
        }
    }
}

# Main audit and generated script inventory remain in checkpoint order.
$cp332MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp331DotSourceIndexForCp332 = $cp332MainAuditText.IndexOf('ideal-loads-structure-audit\cp331-cooling-positive-supply-cp-air-assignment.ps1')
$cp332DotSourceIndex = $cp332MainAuditText.IndexOf('ideal-loads-structure-audit\cp332-cooling-positive-supply-temperature-assignment.ps1')
$cp332AuditCompletionIndex = $cp332MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp331DotSourceIndexForCp332 -lt 0 -or
    $cp332DotSourceIndex -le $cp331DotSourceIndexForCp332 -or
    $cp332AuditCompletionIndex -le $cp332DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP332 after CP331 and before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp332-cooling-positive-supply-temperature-assignment\.ps1"' -Description "CP332 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp332-cooling-positive-supply-temperature-assignment\.ps1::dot_sources' -Description "CP332 main-audit callee evidence"
