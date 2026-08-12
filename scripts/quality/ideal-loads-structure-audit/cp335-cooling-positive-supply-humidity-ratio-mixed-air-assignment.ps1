# CP335 maps only PurchasedAirManager.cc physical executable line 2190: the
# Cooling positive-supply mixed-air humidity-ratio assignment. Physical line
# 2191 is the first excluded executable and CP336 edge.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp335Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment.rs"
$cp335State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment\state.rs"
$cp335Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment\transition.rs"
$cp335Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment\release.rs"
$cp335PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment\release\prefix_validation.rs"
$cp335RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment\release\runtime_validation.rs"
$cp335SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment\release\snapshot_validation.rs"
$cp335Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment\tests\mod.rs"
$cp335ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_humidity_ratio_mixed_air_assignment\tests\release_corruption.rs"
$cp335CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp335Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp335Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp335ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp335BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_humidity_ratio_mixed_air_assignment.rs"
$cp335BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp335BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_humidity_ratio_mixed_air_assignment_tests.rs"
$cp335InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp335InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp335InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp335InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_humidity_ratio_mixed_air_assignment.rs"
$cp335CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp335CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_humidity_ratio_mixed_air_assignment_validation.rs"
$cp335CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp335CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_humidity_ratio_mixed_air_assignment_fixture.rs"
$cp335PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp335Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_humidity_ratio_mixed_air_assignment.rs"
$cp335PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_humidity_ratio_mixed_air_assignment\validation.rs"
$cp335PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_humidity_ratio_mixed_air_assignment\serialization.rs"
$cp335PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_humidity_ratio_mixed_air_assignment\serialization\snapshot.rs"
$cp335RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp335DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_humidity_ratio_mixed_air_assignment_assertions.rs"
$cp335NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp335RequiredFile in @(
        $cp335Module,
        $cp335State,
        $cp335Transition,
        $cp335Release,
        $cp335PrefixValidation,
        $cp335RuntimeValidation,
        $cp335SnapshotValidation,
        $cp335Tests,
        $cp335ReleaseCorruptionTests,
        $cp335ScheduledOutput,
        $cp335BindingAdapter,
        $cp335BindingTests,
        $cp335InitWitness,
        $cp335CoupledValidation,
        $cp335CoupledFixture,
        $cp335Pipeline,
        $cp335PipelineValidation,
        $cp335PipelineSerialization,
        $cp335PipelineSnapshotSerialization,
        $cp335DirectAssertions
    )) {
    Assert-FileExists -Path $cp335RequiredFile -Description "CP335 mixed-air humidity-ratio assignment structure"
}
Assert-LineLimit -Path $cp335Release -Limit 800 -Description "CP335 release root module"
Assert-LineLimit -Path $cp335RuntimeValidation -Limit 800 -Description "CP335 runtime validation module"
Assert-LineLimit -Path $cp335CoupledValidation -Limit 800 -Description "CP335 coupled validation module"
Assert-LineLimit -Path $cp335Pipeline -Limit 800 -Description "CP335 pipeline module"

# Locked source boundary and exact two-site textual inventory.
Assert-Contains -Path $cp335Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2190' -Description "CP335 exact physical source boundary"
Assert-Contains -Path $cp335Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2191' -Description "CP335 first excluded physical executable"
Assert-Contains -Path $cp335Module -Pattern 'Exact two textual source sites represented by CP335' -Description "CP335 exact lexical-site count"
Assert-ExactStringArray -Path $cp335Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-purchased-air-mixed-air-humidity-ratio",
    "assign-purchased-air-supply-humidity-ratio"
) -Description "CP335 exact two-site source order"
Assert-Contains -Path $cp335Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot' -Description "CP335 public snapshot"
Assert-Contains -Path $cp335State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState' -Description "CP335 persistent state"
Assert-Contains -Path $cp335Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary' -Description "CP335 lifecycle summary"
Assert-Contains -Path $cp335Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary\s*\(' -Description "CP335 lifecycle accessor"
Assert-Contains -Path $cp335Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment\s*\(' -Description "CP335 exact-direct wrapper"
Assert-Contains -Path $cp335Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_humidity_ratio_mixed_air_assignment_state\s*\(' -Description "CP335 pure transition"
Assert-Contains -Path $cp335CalcRoot -Pattern 'mod cooling_positive_supply_humidity_ratio_mixed_air_assignment;' -Description "CP335 calc module declaration"
Assert-Contains -Path $cp335CalcRoot -Pattern 'pub use (?:cooling_positive_supply_humidity_ratio_mixed_air_assignment::\*;|\{[^}]*cooling_positive_supply_humidity_ratio_mixed_air_assignment::\*)' -Description "CP335 calc public surface"

# The source statement is a raw binary64 copy. Broader humidity policy and
# psychrometric work remain outside the pure transition.
Assert-PatternsInOrder -Path $cp335Transition -Patterns @(
    'let assignment_executed = predecessor\.supply_temperature_mixed_air_limit_executed;',
    'let mixed_air_humidity_ratio = active_input\.map',
    'let assigned_supply_humidity_ratio = mixed_air_humidity_ratio;',
    'state\.transition_count \+= 1;',
    'state\.source_site_execution_count \+=',
    'PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER',
    'mixed_air_humidity_ratio,',
    'assigned_supply_humidity_ratio,'
) -Description "CP335 raw-copy source transition order"
Assert-NotContains -Path $cp335Transition -Pattern 'Psy|psychrometric|f64::min|f64::max|\.min\(|\.max\(|clamp\(|total_cmp|partial_cmp|is_finite|is_nan|normalize' -Description "CP335 forbidden broadened pure semantics"
foreach ($cp335Route in @(
        'UnitOff',
        'NonCooling',
        'PositiveGuardFalseFallthrough',
        'SupplyHumidityRatioMixedAirAssigned'
    )) {
    Assert-Contains -Path $cp335State -Pattern $cp335Route -Description "CP335 retained route '$cp335Route'"
}
foreach ($cp335Counter in @(
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'supply_humidity_ratio_mixed_air_assignment_count',
        'source_site_execution_count',
        'mixed_air_humidity_ratio_read_count',
        'supply_humidity_ratio_assignment_count'
    )) {
    Assert-Contains -Path $cp335State -Pattern ('pub ' + $cp335Counter + ':\s*usize') -Description "CP335 state counter '$cp335Counter'"
}
Assert-Contains -Path $cp335RuntimeValidation -Pattern '(?s)supply_humidity_ratio_mixed_air_assignment_count.*?checked_mul\(\s*super::super::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "CP335 two-sites-per-active formula"
Assert-Contains -Path $cp335RuntimeValidation -Pattern '(?s)pending_supply_humidity_ratio_mixed_air_assignment_state_is_consistent\(.*?unit_off_skip_count.*?predecessor_state\.unit_off_skip_count.*?non_cooling_skip_count.*?predecessor_state\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor_state\.positive_guard_false_fallthrough_skip_count.*?supply_humidity_ratio_mixed_air_assignment_count.*?predecessor_state\.supply_temperature_mixed_air_limit_count' -Description "CP335 pending CP334 four-route parity"
Assert-Contains -Path $cp335RuntimeValidation -Pattern '(?s)completed_supply_humidity_ratio_mixed_air_assignment_state_is_consistent\(.*?state\.transition_count == predecessor\.transition_count.*?state\.unit_off_skip_count == predecessor\.unit_off_skip_count.*?state\.non_cooling_skip_count == predecessor\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor\.positive_guard_false_fallthrough_skip_count.*?supply_humidity_ratio_mixed_air_assignment_count.*?predecessor\.supply_temperature_mixed_air_limit_count' -Description "CP335 completed CP334 four-route parity"
foreach ($cp335PreflightCounter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'witnessed_positive_guard_false_fallthrough_skip_count',
        'supply_humidity_ratio_mixed_air_assignment_count',
        'source_site_execution_count',
        'mixed_air_humidity_ratio_read_count',
        'supply_humidity_ratio_assignment_count',
        'witnessed_supply_humidity_ratio_mixed_air_assignment_count'
    )) {
    Assert-Contains -Path $cp335RuntimeValidation -Pattern ($cp335PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP335 checked preflight '$cp335PreflightCounter'"
}

# Exact-direct validation narrows admission to finite nonnegative CP329
# evidence while retaining the source copy's exact bits and skipped firewall.
Assert-Contains -Path $cp335SnapshotValidation -Pattern '(?s)assigned_snapshot_is_exact\(.*?mixed_air_humidity_ratio\.is_finite\(\).*?mixed_air_humidity_ratio >= 0\.0.*?assigned_supply_humidity_ratio\.to_bits\(\) == mixed_air_humidity_ratio\.to_bits\(\)' -Description "CP335 exact active snapshot"
Assert-Contains -Path $cp335SnapshotValidation -Pattern '(?s)skipped_snapshot_is_exact\(.*?!snapshot\.mixed_air_humidity_ratio_read.*?snapshot\.mixed_air_humidity_ratio\.is_none\(\).*?!snapshot\.supply_humidity_ratio_assignment_performed.*?snapshot\.assigned_supply_humidity_ratio\.is_none\(\)' -Description "CP335 skipped null firewall"
foreach ($cp335Test in @(
        'source_order_is_the_exact_two_site_assignment_slice',
        'pure_transition_copies_every_ieee_bit_pattern_without_normalization',
        'exact_validator_accepts_finite_nonnegative_values_including_negative_zero',
        'exact_validator_rejects_negative_and_nonfinite_active_values',
        'skipped_routes_enforce_the_null_operand_firewall',
        'counters_partition_all_four_routes_and_count_two_sites_per_assignment',
        'bit_exact_snapshot_matching_detects_signed_zero_and_nan_payload_drift',
        'exact_validator_rejects_assignment_and_null_firewall_corruption'
    )) {
    Assert-Contains -Path $cp335Tests -Pattern $cp335Test -Description "CP335 pure regression '$cp335Test'"
}

# Release takes only CP334 as its public predecessor and obtains the RHS from
# the same-call completed CP329 latest/private pair before any mutation.
Assert-Contains -Path $cp335Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp334: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,\s*\)' -Description "CP335 exact wrapper arguments"
Assert-Contains -Path $cp335Release -Pattern 'unit\.calc_cooling_mixed_air_call\.latest' -Description "CP335 retained CP329 latest snapshot"
Assert-Contains -Path $cp335Release -Pattern 'cooling_mixed_air_call_latest_witness\s*\(' -Description "CP335 retained CP329 private witness"
Assert-Contains -Path $cp335Release -Pattern 'completed_direct_cooling_mixed_air_call_is_consistent\s*\(' -Description "CP335 recursive CP329 completed proof"
Assert-Contains -Path $cp335Release -Pattern 'completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent\s*\(' -Description "CP335 recursive CP334 completed proof"
Assert-Contains -Path $cp335Release -Pattern 'mixed_air\.mixed_air_humidity_ratio' -Description "CP335 CP329 source-field provenance"
Assert-Contains -Path $cp335PrefixValidation -Pattern '(?s)mixed_air\.mixed_air_humidity_ratio_assigned.*?cooling_mixed_air_call_snapshots_match_bit_exact.*?options_match_bits\(mixed_air_humidity_ratio, mixed_air\.mixed_air_humidity_ratio\).*?humidity_ratio\.is_finite\(\) && humidity_ratio >= 0\.0' -Description "CP335 exact CP329 active lineage"
Assert-NotContains -Path $cp335Release -Pattern 'ZoneHeatBalanceState|zone_state|typed_model|psychrometric|PsyH|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "forbidden substitute CP335 release inputs"
Assert-PatternsInOrder -Path $cp335Release -Patterns @(
    'pending_supply_humidity_ratio_mixed_air_assignment_state_is_consistent\(',
    'next_supply_humidity_ratio_mixed_air_assignment_transition_fits\(',
    'completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent\(',
    'completed_direct_cooling_mixed_air_call_is_consistent\(',
    'runtime\.units\.get_mut',
    'advance_cooling_positive_supply_humidity_ratio_mixed_air_assignment_state\(',
    'set_cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness\('
) -Description "CP335 validate-before-mutation order"
foreach ($cp335ReleaseTest in @(
        'public_release_copies_same_call_cp329_humidity_once_and_rejects_replay',
        'skipped_routes_never_project_cp329_humidity_into_cp335',
        'active_operand_requires_exact_same_call_cp329_public_private_pair',
        'forged_cp334_argument_or_private_witness_fails_without_mutation',
        'cp329_public_or_private_drift_fails_recursively_without_mutation',
        'active_counter_overflow_preflight_is_checked_and_transactional',
        'active_source_site_product_overflow_fails_pending_validation_transactionally',
        'every_skipped_route_counter_overflow_fails_without_mutation',
        'orphan_public_or_private_cp335_latest_fails_without_mutation',
        'recursive_cp334_state_corruption_fails_closed_without_mutation'
    )) {
    Assert-Contains -Path $cp335ReleaseCorruptionTests -Pattern $cp335ReleaseTest -Description "CP335 release regression '$cp335ReleaseTest'"
}

# CP335 state and private witness are owned by the selected runtime unit.
Assert-Contains -Path $cp335InitState -Pattern '(?s)cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot' -Description "runtime-root private CP335 witness map"
Assert-NotContains -Path $cp335InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witnesses:' -Description "public runtime-root CP335 witness map"
Assert-Contains -Path $cp335InitWitnessRoot -Pattern 'mod cooling_positive_supply_humidity_ratio_mixed_air_assignment;' -Description "runtime-root CP335 witness module"
Assert-Contains -Path $cp335InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness\s*\(' -Description "runtime-root CP335 witness getter"
Assert-Contains -Path $cp335InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness\s*\(' -Description "runtime-root CP335 witness setter"
Assert-Contains -Path $cp335InitState -Pattern 'pub calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState' -Description "per-unit CP335 persistent state"
Assert-Contains -Path $cp335InitUnit -Pattern '(?s)calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment:\s*PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new\(\s*system\s*,?\s*\)' -Description "per-unit CP335 state initialization"

# Binding order is exact CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> unchanged numerical DTO.
$cp335BindingText = Read-RepoText -Path $cp335Binding
$cp334BindingIndexForCp335 = $cp335BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_mixed_air_limit =")
$cp335BindingIndex = $cp335BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndexForCp335 = $cp335BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp335 = $cp335BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp335 = $cp335BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp335 = $cp335BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp335 = $cp335BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp334BindingIndexForCp335 -lt 0 -or
    $cp335BindingIndex -le $cp334BindingIndexForCp335 -or
    $cp336BindingIndexForCp335 -le $cp335BindingIndex -or
    $cp337BindingIndexForCp335 -le $cp336BindingIndexForCp335 -or
    $cp338BindingIndexForCp335 -le $cp337BindingIndexForCp335 -or
    $cp339BindingIndexForCp335 -le $cp338BindingIndexForCp335 -or
    $numericalBindingIndexForCp335 -le $cp339BindingIndexForCp335
) {
    throw "Binding must retain exact CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $cp335Binding -Pattern '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_temperature_mixed_air_limit,\s*\)\?;' -Description "binding exact CP334-to-CP335 adapter call"
Assert-Contains -Path $cp335BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_humidity_ratio_mixed_air_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,' -Description "CP335 binding adapter arguments"
Assert-Contains -Path $cp335BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment\(\s*runtime,\s*system,\s*predecessor,\s*\).*?CalculationCoolingPositiveSupplyHumidityRatioMixedAirAssignment' -Description "CP335 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp335BindingAdapter -Pattern 'zone_state|ZoneHeatBalanceState|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "extra operand or numerical DTO in CP335 adapter"
Assert-Contains -Path $cp335ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot' -Description "CP335 scheduled output evidence"
Assert-Contains -Path $cp335BindingTestsRoot -Pattern '#\[rustfmt::skip\]\s*#\[path = "binding/cooling_positive_supply_humidity_ratio_mixed_air_assignment_tests\.rs"\]\s*mod cooling_positive_supply_humidity_ratio_mixed_air_assignment_tests;' -Description "CP335 binding test module"
foreach ($cp335BindingTest in @(
        'scheduled_binding_assigns_the_retained_mixed_air_humidity_ratio_bit_exactly',
        'scheduled_binding_skips_cp335_after_the_positive_guard_falls_through',
        'scheduled_binding_preserves_unit_off_and_non_cooling_cp335_skip_routes'
    )) {
    Assert-Contains -Path $cp335BindingTests -Pattern $cp335BindingTest -Description "CP335 binding regression '$cp335BindingTest'"
}
$cp334BindingCallForCp335 = [regex]::Match(
    $cp335BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
$cp335BindingCall = [regex]::Match(
    $cp335BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
$cp336BindingCallForCp335 = [regex]::Match(
    $cp335BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
$cp337BindingCallForCp335 = [regex]::Match(
    $cp335BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
$cp338BindingCallForCp335 = [regex]::Match(
    $cp335BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
$cp339BindingCallForCp335 = [regex]::Match(
    $cp335BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (
    -not $cp334BindingCallForCp335.Success -or
    -not $cp335BindingCall.Success -or
    -not $cp336BindingCallForCp335.Success -or
    -not $cp337BindingCallForCp335.Success -or
    -not $cp338BindingCallForCp335.Success -or
    -not $cp339BindingCallForCp335.Success
) {
    throw "Binding must retain complete CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls"
}
$cp334BindingCallEndForCp335 =
    $cp334BindingCallForCp335.Index + $cp334BindingCallForCp335.Length
$cp335BindingCallEnd = $cp335BindingCall.Index + $cp335BindingCall.Length
$cp336BindingCallEndForCp335 =
    $cp336BindingCallForCp335.Index + $cp336BindingCallForCp335.Length
$cp337BindingCallEndForCp335 =
    $cp337BindingCallForCp335.Index + $cp337BindingCallForCp335.Length
$cp338BindingCallEndForCp335 =
    $cp338BindingCallForCp335.Index + $cp338BindingCallForCp335.Length
$cp339BindingCallEndForCp335 =
    $cp339BindingCallForCp335.Index + $cp339BindingCallForCp335.Length
if (
    $cp335BindingIndex -lt $cp334BindingCallEndForCp335 -or
    $cp336BindingIndexForCp335 -lt $cp335BindingCallEnd -or
    $cp337BindingIndexForCp335 -lt $cp336BindingCallEndForCp335 -or
    $cp338BindingIndexForCp335 -lt $cp337BindingCallEndForCp335 -or
    $cp339BindingIndexForCp335 -lt $cp338BindingCallEndForCp335 -or
    $numericalBindingIndexForCp335 -lt $cp339BindingCallEndForCp335
) {
    throw "CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
foreach ($cp335Interval in @(
        [PSCustomObject]@{
            Start = $cp334BindingCallEndForCp335
            End = $cp335BindingIndex
            Description = "after CP334 and before CP335"
        },
        [PSCustomObject]@{
            Start = $cp335BindingCallEnd
            End = $cp336BindingIndexForCp335
            Description = "after CP335 and before CP336"
        },
        [PSCustomObject]@{
            Start = $cp336BindingCallEndForCp335
            End = $cp337BindingIndexForCp335
            Description = "after CP336 and before CP337"
        },
        [PSCustomObject]@{
            Start = $cp337BindingCallEndForCp335
            End = $cp338BindingIndexForCp335
            Description = "after CP337 and before CP338"
        },
        [PSCustomObject]@{
            Start = $cp338BindingCallEndForCp335
            End = $cp339BindingIndexForCp335
            Description = "after CP338 and before CP339"
        },
        [PSCustomObject]@{
            Start = $cp339BindingCallEndForCp335
            End = $numericalBindingIndexForCp335
            Description = "after CP339 and before numerical Calc"
        }
    )) {
    $cp335IntervalText = $cp335BindingText.Substring(
        $cp335Interval.Start,
        $cp335Interval.End - $cp335Interval.Start
    )
    $cp335IntervalCode = [regex]::Replace($cp335IntervalText, '(?m)//.*$', '')
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
        ''
    )
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
        ''
    )
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\([^;]+?\)\?;)',
    ''
)
    $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\([^;]+?\)\?;',
        ''
    )
        $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
        $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\([^;]+?\)\?;',
        ''
    )
        $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
        $cp335IntervalCode = [regex]::Replace(
        $cp335IntervalCode,
        '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\([^;]+?\)\?;)',
        ''
    )
$cp335IntervalCode = [regex]::Replace(
    $cp335IntervalCode,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment\([^;]+?\)\?;',
    ''
)
    if ($cp335IntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp335Interval.Description)"
    }
}

# Coupled runtime independently reconstructs CP335 from exact CP334 and CP329
# output; pipeline evidence remains direct-only and bit-exact.
Assert-Contains -Path $cp335CoupledRuntime -Pattern 'mod cooling_positive_supply_humidity_ratio_mixed_air_assignment_validation;' -Description "coupled CP335 validator declaration"
Assert-Contains -Path $cp335CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary' -Description "coupled CP335 lifecycle"
Assert-Contains -Path $cp335CoupledRuntime -Pattern 'cooling_positive_supply_humidity_ratio_mixed_air_assignment_validation::snapshot_matches_release' -Description "coupled per-timestep CP335 validation"
Assert-Contains -Path $cp335CoupledRuntime -Pattern 'cooling_positive_supply_humidity_ratio_mixed_air_assignment_validation::validate_lifecycle' -Description "coupled final CP335 validation"
Assert-Contains -Path $cp335CoupledValidation -Pattern 'let predecessor = output\.calculation_cooling_positive_supply_temperature_mixed_air_limit;' -Description "coupled CP334 predecessor"
Assert-Contains -Path $cp335CoupledValidation -Pattern 'let mixed_air = output\.calculation_cooling_mixed_air_call;' -Description "coupled CP329 source"
Assert-Contains -Path $cp335CoupledValidation -Pattern 'let snapshot =\s*output\.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment;' -Description "coupled CP335 result"
Assert-Contains -Path $cp335CoupledValidation -Pattern 'assigned_supply_humidity_ratio: mixed_air_humidity_ratio' -Description "coupled CP335 raw copy reconstruction"
Assert-Contains -Path $cp335CoupledValidation -Pattern 'options_have_exact_bits' -Description "coupled CP335 exact-bit validation"
Assert-Contains -Path $cp335CoupledValidation -Pattern 'checked_mul\(' -Description "coupled CP335 two-site checked multiplication"
Assert-Contains -Path $cp335CoupledValidation -Pattern 'source_site_count_multiplication_overflow_fails_closed' -Description "coupled CP335 source-count overflow regression"
Assert-Contains -Path $cp335CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_humidity_ratio_mixed_air_assignment_fixture;' -Description "coupled CP335 fixture declaration"
Assert-Contains -Path $cp335CoupledFixture -Pattern 'calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot' -Description "coupled CP335 fixture output"
Assert-Contains -Path $cp335PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_humidity_ratio_mixed_air_assignment;' -Description "pipeline CP335 module declaration"
Assert-Contains -Path $cp335PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle' -Description "pipeline CP335 lifecycle field and JSON key"
Assert-Contains -Path $cp335PipelineRoot -Pattern 'calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle' -Description "pipeline CP335 coupled lifecycle transfer"
Assert-Contains -Path $cp335Pipeline -Pattern '(?s)validate_direct_lifecycle\(.*?predecessor_cp334.*?mixed_air_cp329' -Description "pipeline CP335 validates both retained inputs"
Assert-Contains -Path $cp335PipelineValidation -Pattern 'validate_source_counters' -Description "pipeline CP335 source-counter validation"
Assert-Contains -Path $cp335PipelineValidation -Pattern 'source_is_finite_nonnegative\(source\)' -Description "pipeline CP335 retained-source domain validation"
Assert-Contains -Path $cp335PipelineValidation -Pattern '(?s)fn source_is_finite_nonnegative\(value: f64\) -> bool \{\s*value\.is_finite\(\) && value >= 0\.0\s*\}' -Description "pipeline CP335 finite nonnegative retained-source predicate"
Assert-Contains -Path $cp335PipelineValidation -Pattern 'same_option\(snapshot\.assigned_supply_humidity_ratio, Some\(source\)\)' -Description "pipeline CP335 exact assignment copy"
Assert-Contains -Path $cp335PipelineSerialization -Pattern 'mixed_air_humidity_ratio_read_count' -Description "pipeline CP335 lifecycle serialization"
foreach ($cp335BitField in @(
        'mixed_air_humidity_ratio',
        'assigned_supply_humidity_ratio'
    )) {
    Assert-Contains -Path $cp335PipelineSnapshotSerialization -Pattern ('"' + $cp335BitField + '_ieee_bits"') -Description "pipeline CP335 IEEE field '$cp335BitField'"
}
Assert-Contains -Path $cp335PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP335 exact IEEE serialization"
Assert-Contains -Path $cp335RunTests -Pattern 'cooling_positive_supply_humidity_ratio_mixed_air_assignment_assertions' -Description "direct-run CP335 assertion module"
Assert-Contains -Path $cp335DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*2\]\s*=' -Description "direct-run CP335 source order"
Assert-Contains -Path $cp335DirectAssertions -Pattern 'executions \* SOURCE_ORDER\.len\(\) as u64' -Description "direct-run CP335 dynamic source count"
Assert-Contains -Path $cp335DirectAssertions -Pattern 'purchased_air_calc_cooling_mixed_air_call_lifecycle' -Description "direct-run CP329 bit provenance"
Assert-Contains -Path $cp335NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle' -Description "non-direct CP335 null evidence"
Assert-Contains -Path $cp335PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp419_lifecycle_evidence' -Description "non-direct CP335 through CP363 evidence rejection"

# Registries repeat the boundary exactly twice and add target inventory only.
$cp335AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp335AlgorithmAddenda = [regex]::Matches(
    $cp335AlgorithmText,
    '(?m)^\s*"CP335 supersedes only CP334[^"\r\n]+",\s*$'
)
if ($cp335AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP335 boundary addenda"
}
foreach ($cp335AlgorithmAddendum in $cp335AlgorithmAddenda) {
    foreach ($cp335Pattern in @(
            'physical executable line 2190',
            'exactly two lexical sites',
            'read-purchased-air-mixed-air-humidity-ratio',
            'assign-purchased-air-supply-humidity-ratio',
            '2 \* supply_humidity_ratio_mixed_air_assignment_count',
            '2 \* supply_temperature_mixed_air_limit_count',
            '2 \* supply_temperature_minimum_limit_count',
            '2 \* supply_temperature_assignment_count',
            '2 \* cp_air_assignment_count',
            '2 \* positive_supply_mass_flow_body_entries',
            'CP329.+`mixed_air_humidity_ratio`',
            'finite and `>= 0\.0`',
            'SupplyHumidityRatioMixedAirAssigned',
            'CP334-to-CP335-to-numerical',
            'Physical line 2191 is the first excluded lexical executable and CP336 boundary',
            'both parents remain `scaffold`/`none`',
            'Roadmap state stay unchanged'
        )) {
        if ($cp335AlgorithmAddendum.Value -notmatch $cp335Pattern) {
            throw "CP335 algorithm addendum missing '$cp335Pattern'"
        }
    }
}
foreach ($cp335TargetPattern in @(
        'cooling_positive_supply_humidity_ratio_mixed_air_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment',
        'cooling_positive_supply_humidity_ratio_mixed_air_assignment\.rs::purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary',
        'cooling_positive_supply_humidity_ratio_mixed_air_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState',
        'cooling_positive_supply_humidity_ratio_mixed_air_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary'
    )) {
    Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern $cp335TargetPattern -Description "CP335 algorithm target '$cp335TargetPattern'"
}
$cp335CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp335CapabilityAddenda = [regex]::Matches(
    $cp335CapabilityText,
    '(?m)^\s*"CP335 additionally requires[^"\r\n]+",\s*$'
)
if ($cp335CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP335 claim addenda"
}
foreach ($cp335CapabilityAddendum in $cp335CapabilityAddenda) {
    foreach ($cp335Pattern in @(
            'physical executable line 2190',
            'single two-site',
            'read-purchased-air-mixed-air-humidity-ratio',
            'assign-purchased-air-supply-humidity-ratio',
            '2 \* supply_humidity_ratio_mixed_air_assignment_count',
            '2 \* supply_temperature_mixed_air_limit_count',
            '2 \* supply_temperature_minimum_limit_count',
            '2 \* supply_temperature_assignment_count',
            '2 \* cp_air_assignment_count',
            '2 \* positive_supply_mass_flow_body_entries',
            'CP329.+`mixed_air_humidity_ratio`',
            'finite and `>= 0\.0`',
            'SupplyHumidityRatioMixedAirAssigned',
            'CP334-to-CP335-to-numerical',
            'Physical line 2191 is the first excluded lexical executable and CP336 boundary',
            'This changes no support level',
            'both Calc routines remain `source_mapped`'
        )) {
        if ($cp335CapabilityAddendum.Value -notmatch $cp335Pattern) {
            throw "CP335 capability addendum missing '$cp335Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP335 supersedes only CP334' -Description "generated CP335 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP335 additionally requires' -Description "generated CP335 capability index"

# Each hand-authored contract carries one scoped CP335 section with source,
# CP329 provenance, transactionality, exclusions, and explicit non-promotion.
$cp335DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP335 maps only the single Cooling positive-supply.*?^conformance, capability, output claims, and Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP335 Source-Ordered Cooling Positive-Supply Humidity-Ratio Mixed-Air Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP335 Cooling Positive-Supply Humidity-Ratio Mixed-Air Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP335 Positive-Supply Humidity-Ratio Mixed-Air Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP335 Cooling Positive-Supply Humidity-Ratio Mixed-Air Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp335Documentation in $cp335DocumentationSections) {
    $cp335DocumentText = Read-RepoText -Path $cp335Documentation.Path
    $cp335Matches = [regex]::Matches($cp335DocumentText, $cp335Documentation.Pattern)
    if ($cp335Matches.Count -ne 1) {
        throw "CP335 documentation expected one scoped section in $($cp335Documentation.Path), found $($cp335Matches.Count)"
    }
    $cp335Section = $cp335Matches[0].Value
    foreach ($cp335Pattern in @(
            'physical\s+(?:executable\s+)?(?:line\s+)?2190',
            '(?:exactly )?two(?:-site|\s+lexical|\s+sites)|both',
            'read-purchased-air-mixed-air-humidity-ratio',
            'assign-purchased-air-supply-humidity-ratio',
            '2 \* supply_humidity_ratio_mixed_air_assignment_count',
            '2 \* supply_temperature_mixed_air_limit_count',
            '2 \* supply_temperature_minimum_limit_count',
            '2 \* supply_temperature_assignment_count',
            '2 \* cp_air_assignment_count',
            '2 \* positive_supply_mass_flow_body_entries',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?i)guard-false',
            '(?i)CP334',
            '(?i)CP329',
            'mixed_air_humidity_ratio',
            '(?i)finite',
            '>= 0\.0',
            '(?i)latest',
            '(?i)private witness|private-witness',
            '(?i)checked',
            '(?i)transaction|before mutation',
            'purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle',
            'CP334-to-CP335-to-numerical',
            '(?is)(?:does not|neither|without).{0,120}(?:consum(?:e|ing)|reconcil(?:e|ing)).{0,180}numerical\s+DTO',
            'line 2191',
            '(?i)CP336',
            '2340',
            '2454-2461',
            '2465',
            '(?i)scaffold',
            '(?i)source_mapped',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp335Section -notmatch $cp335Pattern) {
            throw "CP335 documentation in $($cp335Documentation.Path) missing '$cp335Pattern'"
        }
    }
}

# Main audit and generated script inventory remain in source-checkpoint order.
$cp335MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp334DotSourceIndexForCp335 = $cp335MainAuditText.IndexOf('ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1')
$cp335DotSourceIndex = $cp335MainAuditText.IndexOf('ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1')
$cp335AuditCompletionIndex = $cp335MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp334DotSourceIndexForCp335 -lt 0 -or
    $cp335DotSourceIndex -le $cp334DotSourceIndexForCp335 -or
    $cp335AuditCompletionIndex -le $cp335DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP335 after CP334 and before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment\.ps1"' -Description "CP335 internal script inventory record"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 357 \|' -Description "CP335 cumulative generated script count through CP403"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 117 \|' -Description "CP335 cumulative generated internal script count through CP403"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP335 generated uncalled script count"
