# CP336 maps only PurchasedAirManager.cc physical executable line 2191: the
# Cooling positive-supply enthalpy assignment. Physical line 2195 is the first
# excluded executable after one blank line and two comment lines, and is the
# CP337 edge.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp336Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment.rs"
$cp336State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment\state.rs"
$cp336Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment\transition.rs"
$cp336Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment\release.rs"
$cp336PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment\release\prefix_validation.rs"
$cp336RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment\release\runtime_validation.rs"
$cp336SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment\release\snapshot_validation.rs"
$cp336Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment\tests\mod.rs"
$cp336ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_enthalpy_assignment\tests\release_corruption.rs"
$cp336CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp336Psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$cp336Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp336Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp336ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp336BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_enthalpy_assignment.rs"
$cp336BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp336BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_enthalpy_assignment_tests.rs"
$cp336InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp336InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp336InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp336InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_enthalpy_assignment.rs"
$cp336CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp336CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_enthalpy_assignment_validation.rs"
$cp336CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp336CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_enthalpy_assignment_fixture.rs"
$cp336PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp336Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_enthalpy_assignment.rs"
$cp336PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_enthalpy_assignment\validation.rs"
$cp336PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_enthalpy_assignment\serialization.rs"
$cp336PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_enthalpy_assignment\serialization\snapshot.rs"
$cp336RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp336DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_enthalpy_assignment_assertions.rs"
$cp336NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp336RequiredFile in @(
        $cp336Module,
        $cp336State,
        $cp336Transition,
        $cp336Release,
        $cp336PrefixValidation,
        $cp336RuntimeValidation,
        $cp336SnapshotValidation,
        $cp336Tests,
        $cp336ReleaseCorruptionTests,
        $cp336Psychrometrics,
        $cp336ScheduledOutput,
        $cp336BindingAdapter,
        $cp336BindingTests,
        $cp336InitWitness,
        $cp336CoupledValidation,
        $cp336CoupledFixture,
        $cp336Pipeline,
        $cp336PipelineValidation,
        $cp336PipelineSerialization,
        $cp336PipelineSnapshotSerialization,
        $cp336DirectAssertions
    )) {
    Assert-FileExists -Path $cp336RequiredFile -Description "CP336 supply-enthalpy assignment structure"
}
Assert-LineLimit -Path $cp336Release -Limit 800 -Description "CP336 release root module"
Assert-LineLimit -Path $cp336ReleaseCorruptionTests -Limit 800 -Description "CP336 release corruption regressions"
Assert-LineLimit -Path $cp336CoupledValidation -Limit 800 -Description "CP336 coupled validation module"
Assert-LineLimit -Path $cp336Pipeline -Limit 800 -Description "CP336 pipeline module"

# Locked source boundary and exact four-site textual witness inventory.
Assert-Contains -Path $cp336Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2191' -Description "CP336 exact physical source boundary"
Assert-Contains -Path $cp336Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2195' -Description "CP336 first excluded physical executable"
Assert-Contains -Path $cp336Module -Pattern 'Exact four textual source sites represented by CP336' -Description "CP336 exact textual-site count"
Assert-ExactStringArray -Path $cp336Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-purchased-air-supply-temperature-for-enthalpy",
    "read-purchased-air-supply-humidity-ratio-for-enthalpy",
    "evaluate-psy-h-fn-tdb-w",
    "assign-local-supply-enthalpy"
) -Description "CP336 deterministic Rust witness order"
Assert-Contains -Path $cp336Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot' -Description "CP336 public snapshot"
Assert-Contains -Path $cp336State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState' -Description "CP336 persistent state"
Assert-Contains -Path $cp336Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary' -Description "CP336 lifecycle summary"
Assert-Contains -Path $cp336Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle_summary\s*\(' -Description "CP336 lifecycle accessor"
Assert-Contains -Path $cp336Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment\s*\(' -Description "CP336 exact-direct wrapper"
Assert-Contains -Path $cp336Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_enthalpy_assignment_state\s*\(' -Description "CP336 pure transition"
Assert-Contains -Path $cp336CalcRoot -Pattern 'mod cooling_positive_supply_enthalpy_assignment;' -Description "CP336 calc module declaration"
Assert-Contains -Path $cp336CalcRoot -Pattern 'pub use (?:cooling_positive_supply_enthalpy_assignment::\*;|\{[^}]*cooling_positive_supply_enthalpy_assignment::\*)' -Description "CP336 calc public surface"

# The transition preserves the four textual witness sites in deterministic Rust
# order and calls only the canonical source-shaped psychrometric helper.
Assert-PatternsInOrder -Path $cp336Transition -Patterns @(
    'let assignment_executed =',
    'let supply_temperature_c = active_input\.map',
    'let supply_humidity_ratio = active_input\.map',
    'let psychrometric_supply_enthalpy_result_j_per_kg = active_input\.map',
    'energyplus_psy_h_fn_tdb_w\(input\.supply_temperature_c, input\.supply_humidity_ratio\)',
    'let supply_enthalpy_j_per_kg = psychrometric_supply_enthalpy_result_j_per_kg;',
    'state\.transition_count \+= 1;',
    'state\.source_site_execution_count \+=',
    'PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER\.len\(\);',
    'supply_temperature_for_enthalpy_read: assignment_executed,',
    'supply_humidity_ratio_for_enthalpy_read: assignment_executed,',
    'psychrometric_supply_enthalpy_evaluated: assignment_executed,',
    'supply_enthalpy_assigned: assignment_executed,'
) -Description "CP336 canonical four-site transition order"
Assert-NotContains -Path $cp336Transition -Pattern 'moist_air_enthalpy_j_per_kg|energyplus_psy_h_fn_tdb_w_fast|energyplus_psy_h_fn_tdb_w_raw|1\.004_84e3|2\.500_94e6|f64::max|\.max\(' -Description "CP336 forbidden alternate or regrouped transition helper"
Assert-Contains -Path $cp336Psychrometrics -Pattern 'const ENERGYPLUS_MIN_HUMIDITY_RATIO: f64 = 1\.0e-5;' -Description "canonical EnergyPlus humidity floor constant"
Assert-Contains -Path $cp336Psychrometrics -Pattern '(?s)fn energyplus_humidity_ratio_floor\(humidity_ratio: f64\).*?if humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO \{\s*ENERGYPLUS_MIN_HUMIDITY_RATIO\s*\} else \{\s*humidity_ratio\s*\}' -Description "canonical source max humidity-floor behavior"
Assert-Contains -Path $cp336Psychrometrics -Pattern '(?s)pub fn energyplus_psy_h_fn_tdb_w\(dry_bulb_c: f64, humidity_ratio: f64\) -> f64 \{\s*energyplus_psy_h_fn_tdb_w_raw\(dry_bulb_c, energyplus_humidity_ratio_floor\(humidity_ratio\)\)\s*\}' -Description "canonical enthalpy helper uses source floor"

foreach ($cp336Route in @(
        'UnitOff',
        'NonCooling',
        'PositiveGuardFalseFallthrough',
        'SupplyEnthalpyAssigned'
    )) {
    Assert-Contains -Path $cp336State -Pattern $cp336Route -Description "CP336 retained route '$cp336Route'"
}
foreach ($cp336Counter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'supply_enthalpy_assignment_count',
        'source_site_execution_count',
        'supply_temperature_for_enthalpy_read_count',
        'supply_humidity_ratio_for_enthalpy_read_count',
        'psychrometric_supply_enthalpy_evaluation_count',
        'supply_enthalpy_assignment_write_count'
    )) {
    Assert-Contains -Path $cp336State -Pattern ('pub ' + $cp336Counter + ':\s*usize') -Description "CP336 state counter '$cp336Counter'"
}
Assert-Contains -Path $cp336RuntimeValidation -Pattern '(?s)supply_enthalpy_assignment_count\.checked_mul\(\s*super::super::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "CP336 four-sites-per-active formula"
Assert-Contains -Path $cp336RuntimeValidation -Pattern '(?s)pending_supply_enthalpy_assignment_state_is_consistent\(.*?unit_off_skip_count.*?predecessor_state\.unit_off_skip_count.*?non_cooling_skip_count.*?predecessor_state\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor_state\.positive_guard_false_fallthrough_skip_count.*?supply_enthalpy_assignment_count.*?predecessor_state\.supply_humidity_ratio_mixed_air_assignment_count' -Description "CP336 pending CP335 four-route parity"
Assert-Contains -Path $cp336RuntimeValidation -Pattern '(?s)completed_supply_enthalpy_assignment_state_is_consistent\(.*?state\.transition_count == predecessor\.transition_count.*?state\.unit_off_skip_count == predecessor\.unit_off_skip_count.*?state\.non_cooling_skip_count == predecessor\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor\.positive_guard_false_fallthrough_skip_count.*?supply_enthalpy_assignment_count.*?predecessor\.supply_humidity_ratio_mixed_air_assignment_count' -Description "CP336 completed CP335 four-route parity"
foreach ($cp336PreflightCounter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'witnessed_positive_guard_false_fallthrough_skip_count',
        'supply_enthalpy_assignment_count',
        'source_site_execution_count',
        'supply_temperature_for_enthalpy_read_count',
        'supply_humidity_ratio_for_enthalpy_read_count',
        'psychrometric_supply_enthalpy_evaluation_count',
        'supply_enthalpy_assignment_write_count',
        'witnessed_supply_enthalpy_assignment_count'
    )) {
    Assert-Contains -Path $cp336RuntimeValidation -Pattern ($cp336PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP336 checked preflight '$cp336PreflightCounter'"
}

# Pure IEEE characterization is broader than exact-direct admission. Exact
# release admits finite T, finite nonnegative W (including -0), and finite
# canonical result; every skipped route retains the complete null firewall.
Assert-Contains -Path $cp336SnapshotValidation -Pattern '(?s)assigned_snapshot_is_exact\(.*?supply_temperature_c\.is_finite\(\).*?supply_humidity_ratio\.is_finite\(\).*?supply_humidity_ratio >= 0\.0.*?psychrometric_result\.is_finite\(\).*?psychrometric_result\.to_bits\(\) == expected\.to_bits\(\).*?assigned\.to_bits\(\) == psychrometric_result\.to_bits\(\)' -Description "CP336 exact active admission and result"
Assert-Contains -Path $cp336SnapshotValidation -Pattern '(?s)skipped_snapshot_is_exact\(.*?!snapshot\.supply_temperature_for_enthalpy_read.*?snapshot\.supply_temperature_c\.is_none\(\).*?!snapshot\.supply_humidity_ratio_for_enthalpy_read.*?snapshot\.supply_humidity_ratio\.is_none\(\).*?!snapshot\.psychrometric_supply_enthalpy_evaluated.*?psychrometric_supply_enthalpy_result_j_per_kg\s*\.is_none\(\).*?!snapshot\.supply_enthalpy_assigned.*?snapshot\.supply_enthalpy_j_per_kg\.is_none\(\)' -Description "CP336 skipped null firewall"
foreach ($cp336Test in @(
        'source_order_is_the_exact_four_site_enthalpy_assignment_slice',
        'pure_transition_matches_canonical_psychrometric_ieee_behavior_bit_exactly',
        'humidity_floor_and_source_grouping_are_locked_by_bits',
        'exact_validator_accepts_finite_release_domain_including_negative_zero_humidity',
        'exact_validator_rejects_nonfinite_or_negative_active_domain_and_corruption',
        'skipped_routes_enforce_the_complete_null_operand_firewall',
        'counters_partition_routes_and_count_four_sites_per_assignment',
        'bit_exact_snapshot_matching_detects_signed_zero_and_nan_payload_drift'
    )) {
    Assert-Contains -Path $cp336Tests -Pattern $cp336Test -Description "CP336 pure regression '$cp336Test'"
}

# The public wrapper accepts only CP335. Active T and W are derived from the
# same-call completed CP334/CP335 public-private pairs before any mutation.
Assert-Contains -Path $cp336Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp335: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,\s*\)' -Description "CP336 exact wrapper arguments"
Assert-Contains -Path $cp336Release -Pattern 'unit\s*\.calc_cooling_positive_supply_temperature_mixed_air_limit\s*\.latest' -Description "CP336 retained CP334 latest snapshot"
Assert-Contains -Path $cp336Release -Pattern 'cooling_positive_supply_temperature_mixed_air_limit_latest_witness\s*\(' -Description "CP336 retained CP334 private witness"
Assert-Contains -Path $cp336Release -Pattern 'completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent\s*\(' -Description "CP336 recursive CP334 completed proof"
Assert-Contains -Path $cp336Release -Pattern 'completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent\s*\(' -Description "CP336 recursive CP335 completed proof"
Assert-Contains -Path $cp336Release -Pattern 'temperature_assignment\s*\.assigned_supply_temperature_c' -Description "CP336 CP334 temperature provenance"
Assert-Contains -Path $cp336Release -Pattern 'predecessor_cp335\s*\.assigned_supply_humidity_ratio' -Description "CP336 CP335 humidity provenance"
Assert-Contains -Path $cp336Release -Pattern '(?s)if !supply_temperature_c\.is_finite\(\).*?if !supply_humidity_ratio\.is_finite\(\) \|\| supply_humidity_ratio < 0\.0.*?let supply_enthalpy_j_per_kg =\s*energyplus_psy_h_fn_tdb_w\(supply_temperature_c, supply_humidity_ratio\);.*?if !supply_enthalpy_j_per_kg\.is_finite\(\)' -Description "CP336 exact direct finite-domain admission"
Assert-Contains -Path $cp336PrefixValidation -Pattern '(?s)active_operands_link_to_retained_prefix\(.*?temperature_snapshots_match_bit_exact.*?options_match_bits\(\s*supply_temperature_c,\s*temperature_assignment\.assigned_supply_temperature_c.*?options_match_bits\(\s*supply_humidity_ratio,\s*predecessor\.assigned_supply_humidity_ratio.*?supply_temperature_c\.is_some_and\(f64::is_finite\).*?humidity_ratio\.is_finite\(\) && humidity_ratio >= 0\.0' -Description "CP336 exact CP334/CP335 operand lineage"
Assert-NotContains -Path $cp336Release -Pattern 'ZoneHeatBalanceState|zone_state|typed_model|moist_air_enthalpy_j_per_kg|energyplus_psy_h_fn_tdb_w_fast|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "forbidden substitute CP336 release inputs or helper"
Assert-NotContains -Path $cp336Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment\([^)]*(?:supply_temperature|supply_humidity|supply_enthalpy)\w*\s*:\s*f64' -Description "duplicate caller scalar in CP336 release"
Assert-PatternsInOrder -Path $cp336Release -Patterns @(
    'pending_supply_enthalpy_assignment_state_is_consistent\(',
    'next_supply_enthalpy_assignment_transition_fits\(',
    'completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent\(',
    'completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent\(',
    'energyplus_psy_h_fn_tdb_w\(supply_temperature_c, supply_humidity_ratio\);',
    'active_operands_link_to_retained_prefix\(',
    'runtime\.units\.get_mut',
    'advance_cooling_positive_supply_enthalpy_assignment_state\(',
    'set_cooling_positive_supply_enthalpy_assignment_latest_witness\('
) -Description "CP336 validate-before-mutation order"
foreach ($cp336ReleaseTest in @(
        'public_release_reads_only_same_call_cp334_and_cp335_operands_and_rejects_replay',
        'skipped_routes_never_read_or_project_cp334_or_cp335_operands',
        'active_operands_require_exact_same_call_cp334_public_private_pair_and_cp335_humidity',
        'forged_cp335_argument_or_private_witness_fails_without_mutation',
        'cp334_public_or_private_drift_fails_recursively_without_mutation',
        'every_active_counter_overflow_is_preflighted_transactionally',
        'active_source_site_product_overflow_fails_pending_validation_transactionally',
        'every_skipped_route_counter_overflow_fails_without_mutation',
        'orphan_public_or_private_cp336_latest_fails_without_mutation',
        'completed_proof_detects_post_commit_result_and_witness_drift',
        'recursive_cp335_state_corruption_fails_closed_without_mutation',
        'lifecycle_accessor_returns_the_retained_cp336_state'
    )) {
    Assert-Contains -Path $cp336ReleaseCorruptionTests -Pattern $cp336ReleaseTest -Description "CP336 release regression '$cp336ReleaseTest'"
}

# CP336 state and private witness are owned by the selected runtime unit.
Assert-Contains -Path $cp336InitState -Pattern '(?s)cooling_positive_supply_enthalpy_assignment_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot' -Description "runtime-root private CP336 witness map"
Assert-NotContains -Path $cp336InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_enthalpy_assignment_latest_witnesses:' -Description "public runtime-root CP336 witness map"
Assert-Contains -Path $cp336InitWitnessRoot -Pattern 'mod cooling_positive_supply_enthalpy_assignment;' -Description "runtime-root CP336 witness module"
Assert-Contains -Path $cp336InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_enthalpy_assignment_latest_witness\s*\(' -Description "runtime-root CP336 witness getter"
Assert-Contains -Path $cp336InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_enthalpy_assignment_latest_witness\s*\(' -Description "runtime-root CP336 witness setter"
Assert-Contains -Path $cp336InitState -Pattern 'pub calc_cooling_positive_supply_enthalpy_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState' -Description "per-unit CP336 persistent state"
Assert-Contains -Path $cp336InitUnit -Pattern '(?s)calc_cooling_positive_supply_enthalpy_assignment:\s*PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new\(\s*system\s*,?\s*\)' -Description "per-unit CP336 state initialization"

# Binding order is exact CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> the unchanged
# numerical DTO, with no hidden helper in any source-order interval.
$cp336BindingText = Read-RepoText -Path $cp336Binding
$cp335BindingIndexForCp336 = $cp336BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndex = $cp336BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp336 = $cp336BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp336 = $cp336BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp336 = $cp336BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp336 = $cp336BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp335BindingIndexForCp336 -lt 0 -or
    $cp336BindingIndex -le $cp335BindingIndexForCp336 -or
    $cp337BindingIndexForCp336 -le $cp336BindingIndex -or
    $cp338BindingIndexForCp336 -le $cp337BindingIndexForCp336 -or
    $cp339BindingIndexForCp336 -le $cp338BindingIndexForCp336 -or
    $numericalBindingIndexForCp336 -le $cp339BindingIndexForCp336
) {
    throw "Binding must retain exact CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $cp336Binding -Pattern '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment,\s*\)\?;' -Description "binding exact CP335-to-CP336 adapter call"
Assert-Contains -Path $cp336BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_enthalpy_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,' -Description "CP336 binding adapter arguments"
Assert-Contains -Path $cp336BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment\(\s*runtime,\s*system,\s*predecessor,\s*\).*?CalculationCoolingPositiveSupplyEnthalpyAssignment' -Description "CP336 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp336BindingAdapter -Pattern 'zone_state|ZoneHeatBalanceState|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "extra operand or numerical DTO in CP336 adapter"
Assert-Contains -Path $cp336ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_enthalpy_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot' -Description "CP336 scheduled output evidence"
Assert-Contains -Path $cp336BindingTestsRoot -Pattern '#\[rustfmt::skip\]\s*#\[path = "binding/cooling_positive_supply_enthalpy_assignment_tests\.rs"\]\s*mod cooling_positive_supply_enthalpy_assignment_tests;' -Description "CP336 binding test module"
foreach ($cp336BindingTest in @(
        'scheduled_binding_evaluates_and_assigns_canonical_supply_enthalpy',
        'scheduled_binding_skips_cp336_after_the_positive_guard_falls_through',
        'scheduled_binding_preserves_unit_off_and_non_cooling_cp336_skip_routes'
    )) {
    Assert-Contains -Path $cp336BindingTests -Pattern $cp336BindingTest -Description "CP336 binding regression '$cp336BindingTest'"
}
$cp335BindingCallForCp336 = [regex]::Match(
    $cp336BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
$cp336BindingCall = [regex]::Match(
    $cp336BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
$cp337BindingCallForCp336 = [regex]::Match(
    $cp336BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
$cp338BindingCallForCp336 = [regex]::Match(
    $cp336BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
$cp339BindingCallForCp336 = [regex]::Match(
    $cp336BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (
    -not $cp335BindingCallForCp336.Success -or
    -not $cp336BindingCall.Success -or
    -not $cp337BindingCallForCp336.Success -or
    -not $cp338BindingCallForCp336.Success -or
    -not $cp339BindingCallForCp336.Success
) {
    throw "Binding must retain complete CP335, CP336, CP337, CP338, and CP339 exact release calls"
}
$cp335BindingCallEndForCp336 =
    $cp335BindingCallForCp336.Index + $cp335BindingCallForCp336.Length
$cp336BindingCallEnd = $cp336BindingCall.Index + $cp336BindingCall.Length
$cp337BindingCallEndForCp336 =
    $cp337BindingCallForCp336.Index + $cp337BindingCallForCp336.Length
$cp338BindingCallEndForCp336 =
    $cp338BindingCallForCp336.Index + $cp338BindingCallForCp336.Length
$cp339BindingCallEndForCp336 =
    $cp339BindingCallForCp336.Index + $cp339BindingCallForCp336.Length
if (
    $cp336BindingIndex -lt $cp335BindingCallEndForCp336 -or
    $cp337BindingIndexForCp336 -lt $cp336BindingCallEnd -or
    $cp338BindingIndexForCp336 -lt $cp337BindingCallEndForCp336 -or
    $cp339BindingIndexForCp336 -lt $cp338BindingCallEndForCp336 -or
    $numericalBindingIndexForCp336 -lt $cp339BindingCallEndForCp336
) {
    throw "CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
foreach ($cp336Interval in @(
        [PSCustomObject]@{
            Start = $cp335BindingCallEndForCp336
            End = $cp336BindingIndex
            Description = "after CP335 and before CP336"
        },
        [PSCustomObject]@{
            Start = $cp336BindingCallEnd
            End = $cp337BindingIndexForCp336
            Description = "after CP336 and before CP337"
        },
        [PSCustomObject]@{
            Start = $cp337BindingCallEndForCp336
            End = $cp338BindingIndexForCp336
            Description = "after CP337 and before CP338"
        },
        [PSCustomObject]@{
            Start = $cp338BindingCallEndForCp336
            End = $cp339BindingIndexForCp336
            Description = "after CP338 and before CP339"
        },
        [PSCustomObject]@{
            Start = $cp339BindingCallEndForCp336
            End = $numericalBindingIndexForCp336
            Description = "after CP339 and before numerical Calc"
        }
    )) {
    $cp336IntervalText = $cp336BindingText.Substring(
        $cp336Interval.Start,
        $cp336Interval.End - $cp336Interval.Start
    )
    $cp336IntervalCode = [regex]::Replace($cp336IntervalText, '(?m)//.*$', '')
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
        ''
    )
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
        ''
    )
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\([^;]+?\)\?;)',
    ''
)
    $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\([^;]+?\)\?;',
        ''
    )
        $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
        $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\([^;]+?\)\?;',
        ''
    )
        $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
        $cp336IntervalCode = [regex]::Replace(
        $cp336IntervalCode,
        '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\([^;]+?\)\?;)',
        ''
    )
$cp336IntervalCode = [regex]::Replace(
    $cp336IntervalCode,
    '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry =\s*advance_cooling_supply_mass_flow_positive_guard_else_branch_entry\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment =\s*advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment =\s*advance_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment\([^;]+?\)\?;|let calculation_heating_or_no_load_case_entry =\s*advance_heating_or_no_load_case_entry\([^;]+?\)\?;)',
    ''
)
    if ($cp336IntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp336Interval.Description)"
    }
}

# Coupled and pipeline validators independently reconstruct CP336 from exact
# CP334/CP335 evidence, serialize exact bits, and remain direct-only.
Assert-Contains -Path $cp336CoupledRuntime -Pattern 'mod cooling_positive_supply_enthalpy_assignment_validation;' -Description "coupled CP336 validator declaration"
Assert-Contains -Path $cp336CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_enthalpy_assignment_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary' -Description "coupled CP336 lifecycle"
Assert-Contains -Path $cp336CoupledRuntime -Pattern 'cooling_positive_supply_enthalpy_assignment_validation::snapshot_matches_release' -Description "coupled per-timestep CP336 validation"
Assert-Contains -Path $cp336CoupledRuntime -Pattern 'cooling_positive_supply_enthalpy_assignment_validation::validate_lifecycle' -Description "coupled final CP336 validation"
Assert-Contains -Path $cp336CoupledValidation -Pattern 'output\.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment' -Description "coupled CP335 predecessor"
Assert-Contains -Path $cp336CoupledValidation -Pattern 'output\.calculation_cooling_positive_supply_temperature_mixed_air_limit' -Description "coupled CP334 temperature source"
Assert-Contains -Path $cp336CoupledValidation -Pattern 'energyplus_psy_h_fn_tdb_w\(supply_temperature_c, supply_humidity_ratio\)' -Description "coupled canonical helper reconstruction"
Assert-Contains -Path $cp336CoupledValidation -Pattern 'options_have_exact_bits' -Description "coupled CP336 exact-bit validation"
Assert-Contains -Path $cp336CoupledValidation -Pattern 'checked_mul\(' -Description "coupled CP336 four-site checked multiplication"
Assert-Contains -Path $cp336CoupledValidation -Pattern 'transition_partition_addition_overflow_fails_closed' -Description "coupled CP336 partition overflow regression"
Assert-Contains -Path $cp336CoupledValidation -Pattern 'source_site_count_multiplication_overflow_fails_closed' -Description "coupled CP336 source-count overflow regression"
Assert-Contains -Path $cp336CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_enthalpy_assignment_fixture;' -Description "coupled CP336 fixture declaration"
Assert-Contains -Path $cp336CoupledFixture -Pattern 'calculation_cooling_positive_supply_enthalpy_assignment_snapshot' -Description "coupled CP336 fixture output"
Assert-Contains -Path $cp336PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_enthalpy_assignment;' -Description "pipeline CP336 module declaration"
Assert-Contains -Path $cp336PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle' -Description "pipeline CP336 lifecycle field and JSON key"
Assert-Contains -Path $cp336PipelineRoot -Pattern 'calc_cooling_positive_supply_enthalpy_assignment_lifecycle' -Description "pipeline CP336 coupled lifecycle transfer"
Assert-Contains -Path $cp336Pipeline -Pattern '(?s)validate_direct_lifecycle\(.*?predecessor_cp335.*?temperature_cp334' -Description "pipeline CP336 validates both retained inputs"
Assert-Contains -Path $cp336PipelineValidation -Pattern 'validate_source_counters' -Description "pipeline CP336 source-counter validation"
Assert-Contains -Path $cp336PipelineValidation -Pattern '(?s)fn source_humidity_is_finite_nonnegative\(value: f64\) -> bool \{\s*value\.is_finite\(\) && value >= 0\.0\s*\}' -Description "pipeline CP336 finite nonnegative humidity predicate"
Assert-Contains -Path $cp336PipelineValidation -Pattern 'energyplus_psy_h_fn_tdb_w\(supply_temperature_c, supply_humidity_ratio\)' -Description "pipeline CP336 canonical reconstruction"
Assert-Contains -Path $cp336PipelineSerialization -Pattern 'psychrometric_supply_enthalpy_evaluation_count' -Description "pipeline CP336 lifecycle serialization"
foreach ($cp336BitField in @(
        'supply_temperature_c',
        'supply_humidity_ratio',
        'psychrometric_supply_enthalpy_result_j_per_kg',
        'supply_enthalpy_j_per_kg'
    )) {
    Assert-Contains -Path $cp336PipelineSnapshotSerialization -Pattern ('"' + $cp336BitField + '_ieee_bits"') -Description "pipeline CP336 IEEE field '$cp336BitField'"
}
Assert-Contains -Path $cp336PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP336 exact IEEE serialization"
Assert-Contains -Path $cp336Pipeline -Pattern 'json_preserves_signed_zero_humidity_and_the_canonical_floor_result' -Description "pipeline CP336 signed-zero floor JSON regression"
Assert-Contains -Path $cp336Pipeline -Pattern 'json_keeps_non_finite_bits_when_raw_values_are_null' -Description "pipeline CP336 nonfinite JSON regression"
Assert-Contains -Path $cp336RunTests -Pattern 'cooling_positive_supply_enthalpy_assignment_assertions' -Description "direct-run CP336 assertion module"
Assert-Contains -Path $cp336DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*4\]\s*=' -Description "direct-run CP336 source order"
Assert-Contains -Path $cp336DirectAssertions -Pattern 'executions \* SOURCE_ORDER\.len\(\) as u64' -Description "direct-run CP336 dynamic source count"
Assert-Contains -Path $cp336DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle' -Description "direct-run CP335 bit provenance"
Assert-Contains -Path $cp336DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle' -Description "direct-run CP334 bit provenance"
Assert-Contains -Path $cp336DirectAssertions -Pattern 'energyplus_psy_h_fn_tdb_w\(supply_temperature_c, supply_humidity_ratio\)' -Description "direct-run CP336 canonical result"
Assert-Contains -Path $cp336NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle' -Description "non-direct CP336 null evidence"
Assert-Contains -Path $cp336PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp430_lifecycle_evidence' -Description "non-direct CP336 through CP363 evidence rejection"

# Registries carry exactly two CP336 addenda and the two parent target arrays.
$cp336AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp336AlgorithmAddenda = [regex]::Matches(
    $cp336AlgorithmText,
    '(?m)^\s*"CP336 supersedes only CP335[^"\r\n]+",\s*$'
)
if ($cp336AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP336 boundary addenda"
}
foreach ($cp336AlgorithmAddendum in $cp336AlgorithmAddenda) {
    foreach ($cp336Pattern in @(
            'physical executable line 2191',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            'exactly four lexical sites',
            'read-purchased-air-supply-temperature-for-enthalpy',
            'read-purchased-air-supply-humidity-ratio-for-enthalpy',
            'evaluate-psy-h-fn-tdb-w',
            'assign-local-supply-enthalpy',
            'no C\+\+ function-argument evaluation-order claim',
            'SupplyHumidityRatioMixedAirAssigned',
            'CP334.+assigned_supply_temperature_c',
            'CP335 `assigned_supply_humidity_ratio`',
            'energyplus_psy_h_fn_tdb_w',
            'max\(W, 1\.0e-5\)',
            'moist_air_enthalpy_j_per_kg',
            'finite `>= 0\.0` humidity including negative zero',
            '4 \* supply_enthalpy_assignment_count',
            '4 \* supply_humidity_ratio_mixed_air_assignment_count',
            '4 \* supply_temperature_mixed_air_limit_count',
            '4 \* supply_temperature_minimum_limit_count',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            'CP335-to-CP336-to-numerical',
            'Physical line 2192 is blank and lines 2193-2194 are comments',
            'physical line 2195 is the first excluded lexical executable and CP337 boundary',
            'both parents remain `scaffold`/`none`',
            'Roadmap state stay unchanged'
        )) {
        if ($cp336AlgorithmAddendum.Value -notmatch $cp336Pattern) {
            throw "CP336 algorithm addendum missing '$cp336Pattern'"
        }
    }
}
$cp336AlgorithmTargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_enthalpy_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_enthalpy_assignment\.rs::purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_enthalpy_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_enthalpy_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp336Target in $cp336AlgorithmTargetCounts) {
    $cp336TargetCount = [regex]::Matches($cp336AlgorithmText, $cp336Target.Pattern).Count
    if ($cp336TargetCount -ne $cp336Target.Expected) {
        throw "CP336 target '$($cp336Target.Pattern)' expected $($cp336Target.Expected), found $cp336TargetCount"
    }
}

$cp336CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp336CapabilityAddenda = [regex]::Matches(
    $cp336CapabilityText,
    '(?m)^\s*"CP336 additionally requires[^"\r\n]+",\s*$'
)
if ($cp336CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP336 claim addenda"
}
foreach ($cp336CapabilityAddendum in $cp336CapabilityAddenda) {
    foreach ($cp336Pattern in @(
            'physical executable line 2191',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            'single four-site',
            'read-purchased-air-supply-temperature-for-enthalpy',
            'read-purchased-air-supply-humidity-ratio-for-enthalpy',
            'evaluate-psy-h-fn-tdb-w',
            'assign-local-supply-enthalpy',
            'no C\+\+ function-argument evaluation-order claim',
            'SupplyHumidityRatioMixedAirAssigned',
            'CP334.+assigned_supply_temperature_c',
            'CP335 `assigned_supply_humidity_ratio`',
            'energyplus_psy_h_fn_tdb_w',
            'max\(W, 1\.0e-5\)',
            'moist_air_enthalpy_j_per_kg',
            'finite `>= 0\.0` humidity including negative zero',
            '4 \* supply_enthalpy_assignment_count',
            '4 \* supply_humidity_ratio_mixed_air_assignment_count',
            '4 \* supply_temperature_mixed_air_limit_count',
            '4 \* supply_temperature_minimum_limit_count',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            'CP335-to-CP336-to-numerical',
            'Physical line 2192 is blank and lines 2193-2194 are comments',
            'physical line 2195 is the first excluded lexical executable and CP337 boundary',
            'This changes no support level',
            'both Calc routines remain `source_mapped`'
        )) {
        if ($cp336CapabilityAddendum.Value -notmatch $cp336Pattern) {
            throw "CP336 capability addendum missing '$cp336Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP336 supersedes only CP335' -Description "generated CP336 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP336 additionally requires' -Description "generated CP336 capability index"

# Each hand-authored contract carries one scoped CP336 section. The two
# argument reads are explicitly textual inventory, not a C++ evaluation-order
# claim; the listed order is explicitly the deterministic Rust witness.
$cp336DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP336 maps only the single Cooling positive-supply enthalpy assignment.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP336 Source-Ordered Cooling Positive-Supply Enthalpy Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP336 Cooling Positive-Supply Enthalpy Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP336 Positive-Supply Enthalpy Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP336 Cooling Positive-Supply Enthalpy Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp336Documentation in $cp336DocumentationSections) {
    $cp336DocumentText = Read-RepoText -Path $cp336Documentation.Path
    $cp336Matches = [regex]::Matches($cp336DocumentText, $cp336Documentation.Pattern)
    if ($cp336Matches.Count -ne 1) {
        throw "CP336 documentation expected one scoped section in $($cp336Documentation.Path), found $($cp336Matches.Count)"
    }
    $cp336Section = $cp336Matches[0].Value
    foreach ($cp336Pattern in @(
            'physical\s+(?:executable\s+)?(?:line\s+)?2191',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            'read-purchased-air-supply-temperature-for-enthalpy',
            'read-purchased-air-supply-humidity-ratio-for-enthalpy',
            'evaluate-psy-h-fn-tdb-w',
            'assign-local-supply-enthalpy',
            '(?i)textual',
            'C\+\+',
            '(?i)evaluation-order|evaluation order',
            'deterministic Rust\s+witness contract',
            'SupplyHumidityRatioMixedAirAssigned',
            '(?i)CP334',
            '(?i)CP335',
            'assigned_supply_temperature_c',
            'assigned_supply_humidity_ratio',
            '(?i)latest/private|private witness|private-witness',
            'energyplus_psy_h_fn_tdb_w',
            'max\(W, 1\.0e-5\)',
            'moist_air_enthalpy_j_per_kg',
            '(?i)finite',
            '>= 0\.0',
            '(?i)negative zero',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?i)guard-false',
            '(?i)null|no operands or result',
            '4 \* supply_enthalpy_assignment_count',
            '4 \* supply_humidity_ratio_mixed_air_assignment_count',
            '4 \* supply_temperature_mixed_air_limit_count',
            '4 \* supply_temperature_minimum_limit_count',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            '(?i)checked',
            '(?i)transaction|before mutation',
            'purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle',
            'CP335-to-CP336-to-numerical',
            '(?i)numerical[- ]DTO',
            '(?i)None|no CP336 lifecycle',
            'line 2192',
            '2193-2194',
            'line 2195',
            '(?i)CP337',
            '2195-2337',
            '2339-2345',
            '2340',
            '2347-2348',
            '2454-2461',
            '2465',
            '(?i)scaffold',
            '(?i)source_mapped',
            '(?i)support',
            '(?i)conformance',
            '(?i)capability',
            '(?i)output',
            '(?i)Roadmap'
        )) {
        if ($cp336Section -notmatch $cp336Pattern) {
            throw "CP336 documentation in $($cp336Documentation.Path) missing '$cp336Pattern'"
        }
    }
}

# Main audit and generated script inventory remain in checkpoint order.
$cp336MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp335DotSourceIndexForCp336 = $cp336MainAuditText.IndexOf('ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1')
$cp336DotSourceIndex = $cp336MainAuditText.IndexOf('ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1')
$cp336AuditCompletionIndex = $cp336MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp335DotSourceIndexForCp336 -lt 0 -or
    $cp336DotSourceIndex -le $cp335DotSourceIndexForCp336 -or
    $cp336AuditCompletionIndex -le $cp336DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP336 after CP335 and before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp336-cooling-positive-supply-enthalpy-assignment\.ps1"' -Description "CP336 internal script inventory record"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 368 \|' -Description "CP336 cumulative generated script count through CP403"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP336 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 128 \|' -Description "CP336 cumulative generated internal script count through CP403"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP336 generated uncalled script count"
