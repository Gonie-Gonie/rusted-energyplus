# CP339 maps only PurchasedAirManager.cc physical executable line 2197:
# CoolSensOutput = SupplyMassFlowRate * (MixedAirEnthalpy - SupplyEnthalpy);
# Physical line 2198 is the first excluded lexical executable and CP340
# boundary. This file is dot-sourced after the shared assertions and paths.
$cp339Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment.rs"
$cp339State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\state.rs"
$cp339Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\transition.rs"
$cp339Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\release.rs"
$cp339PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\release\prefix_validation.rs"
$cp339RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\release\runtime_validation.rs"
$cp339SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\release\snapshot_validation.rs"
$cp339Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\tests\mod.rs"
$cp339IeeeTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\tests\ieee.rs"
$cp339ReleaseFixture = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\tests\release_fixture.rs"
$cp339ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\tests\release_corruption.rs"
$cp339ReleaseEdgeCaseTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_assignment\tests\release_edge_cases.rs"
$cp339CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp339InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp339InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp339InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp339InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_capacity_limit_sensible_output_assignment.rs"
$cp339Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp339Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp339BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_capacity_limit_sensible_output_assignment.rs"
$cp339BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp339BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_capacity_limit_sensible_output_assignment_tests.rs"
$cp339ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp339CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp339CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_capacity_limit_sensible_output_assignment_validation.rs"
$cp339CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp339CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_capacity_limit_sensible_output_assignment_fixture.rs"
$cp339PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp339Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_sensible_output_assignment.rs"
$cp339PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_sensible_output_assignment\validation.rs"
$cp339PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_sensible_output_assignment\serialization.rs"
$cp339PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_sensible_output_assignment\serialization\snapshot.rs"
$cp339RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp339DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_capacity_limit_sensible_output_assignment_assertions.rs"
$cp339NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp339RequiredFile in @(
        $cp339Module,
        $cp339State,
        $cp339Transition,
        $cp339Release,
        $cp339PrefixValidation,
        $cp339RuntimeValidation,
        $cp339SnapshotValidation,
        $cp339Tests,
        $cp339IeeeTests,
        $cp339ReleaseFixture,
        $cp339ReleaseCorruptionTests,
        $cp339ReleaseEdgeCaseTests,
        $cp339InitWitness,
        $cp339BindingAdapter,
        $cp339BindingTests,
        $cp339CoupledValidation,
        $cp339CoupledFixture,
        $cp339Pipeline,
        $cp339PipelineValidation,
        $cp339PipelineSerialization,
        $cp339PipelineSnapshotSerialization,
        $cp339DirectAssertions
    )) {
    Assert-FileExists -Path $cp339RequiredFile -Description "CP339 capacity-limit sensible-output assignment structure"
}
Assert-LineLimit -Path $cp339CalcRoot -Limit 80 -Description "IdealLoads calc root"
Assert-LineLimit -Path $cp339Release -Limit 520 -Description "CP339 release root module"
Assert-LineLimit -Path $cp339ReleaseCorruptionTests -Limit 650 -Description "CP339 release corruption regressions"
Assert-LineLimit -Path $cp339CoupledValidation -Limit 700 -Description "CP339 coupled validation module"
Assert-LineLimit -Path $cp339Pipeline -Limit 450 -Description "CP339 pipeline module"

$cp339SourceStatementPattern =
    'CoolSensOutput\s*=\s*SupplyMassFlowRate\s*\*\s*\(MixedAirEnthalpy\s*-\s*SupplyEnthalpy\);'
$cp339FirstExcludedStatementPattern =
    'if\s*\(CoolSensOutput\s*>=\s*PurchAir\.MaxCoolTotCap\)\s*\{'
$cp339OrderedSourceSitesPattern =
    '(?s)read-retained-supply-mass-flow-rate-for-sensible-output-product.*?' +
    'read-retained-mixed-air-enthalpy-for-sensible-output-difference.*?' +
    'read-retained-supply-enthalpy-for-sensible-output-difference.*?' +
    'calculate-mixed-air-enthalpy-minus-supply-enthalpy.*?' +
    'calculate-supply-mass-flow-rate-times-enthalpy-difference.*?' +
    'assign-local-cooling-sensible-output'
$cp339SerializationOwnershipPattern =
    '(?is)Rust\s+snapshots?.*?Some\(value\).*?' +
    'serde\s+JSON\s+serialization.*?nonfinite\s+numeric.*?null.*?' +
    'separate\s+IEEE\s+bit\s+string.*?non-null.*?' +
    'skipped\s+Rust\s+snapshots?.*?None.*?' +
    'serialized\s+JSON.*?numeric.*?bit-string\s+fields\s+null'

# The core owns exactly physical line 2197 and six deterministic Rust sites.
Assert-Contains -Path $cp339Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2197' -Description "CP339 exact physical source boundary"
Assert-Contains -Path $cp339Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2198' -Description "CP339 first excluded physical executable"
Assert-Contains -Path $cp339Module -Pattern 'Exact six textual source sites represented by CP339' -Description "CP339 exact textual-site count"
Assert-ExactStringArray -Path $cp339Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-retained-supply-mass-flow-rate-for-sensible-output-product",
    "read-retained-mixed-air-enthalpy-for-sensible-output-difference",
    "read-retained-supply-enthalpy-for-sensible-output-difference",
    "calculate-mixed-air-enthalpy-minus-supply-enthalpy",
    "calculate-supply-mass-flow-rate-times-enthalpy-difference",
    "assign-local-cooling-sensible-output"
) -Description "CP339 deterministic Rust witness order"
Assert-Contains -Path $cp339Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot' -Description "CP339 public snapshot"
Assert-Contains -Path $cp339State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState' -Description "CP339 persistent state"
Assert-Contains -Path $cp339Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary' -Description "CP339 lifecycle summary"
Assert-Contains -Path $cp339Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary\s*\(' -Description "CP339 lifecycle accessor"
Assert-Contains -Path $cp339Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment\s*\(' -Description "CP339 exact-direct wrapper"
Assert-Contains -Path $cp339Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_capacity_limit_sensible_output_assignment_state\s*\(' -Description "CP339 pure transition"
Assert-Contains -Path $cp339CalcRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_assignment;' -Description "CP339 calc module declaration"
Assert-Contains -Path $cp339CalcRoot -Pattern 'pub use (?:cooling_positive_supply_capacity_limit_sensible_output_assignment::\*;|\{[^}]*cooling_positive_supply_capacity_limit_sensible_output_assignment::\*)' -Description "CP339 calc public surface"

foreach ($cp339SnapshotField in @(
        'predecessor_capacity_limit_cp_air_assignment_executed',
        'capacity_limit_sensible_output_assignment_executed',
        'supply_mass_flow_rate_read',
        'supply_mass_flow_rate_kg_per_s',
        'mixed_air_enthalpy_read',
        'mixed_air_enthalpy_j_per_kg',
        'supply_enthalpy_read',
        'supply_enthalpy_j_per_kg',
        'enthalpy_difference_calculated',
        'mixed_air_minus_supply_enthalpy_j_per_kg',
        'cooling_sensible_output_calculated',
        'calculated_cooling_sensible_output_w',
        'cooling_sensible_output_assigned',
        'cooling_sensible_output_w'
    )) {
    Assert-Contains -Path $cp339Module -Pattern ('pub ' + $cp339SnapshotField + ':') -Description "CP339 snapshot field '$cp339SnapshotField'"
}
foreach ($cp339Route in @(
        'UnitOff',
        'NonCooling',
        'PositiveGuardFalseFallthrough',
        'ActiveCapacityLimitGuardFalseFallthrough',
        'CapacityLimitSensibleOutputAssigned'
    )) {
    Assert-Contains -Path $cp339State -Pattern $cp339Route -Description "CP339 retained route '$cp339Route'"
}
foreach ($cp339Counter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'capacity_limit_guard_false_fallthrough_skip_count',
        'capacity_limit_sensible_output_assignment_count',
        'source_site_execution_count',
        'supply_mass_flow_rate_read_count',
        'mixed_air_enthalpy_read_count',
        'supply_enthalpy_read_count',
        'enthalpy_difference_calculation_count',
        'cooling_sensible_output_calculation_count',
        'cooling_sensible_output_assignment_write_count'
    )) {
    Assert-Contains -Path $cp339State -Pattern ('pub ' + $cp339Counter + ': usize') -Description "CP339 state counter '$cp339Counter'"
}

# Only CP338 assignment activates the exact subtraction-then-multiplication
# statement. Derived NaN/infinity is retained rather than rejected.
Assert-Contains -Path $cp339Transition -Pattern 'let assignment_executed = predecessor\.capacity_limit_cp_air_assignment_executed;' -Description "CP338 assignment activation"
Assert-PatternsInOrder -Path $cp339Transition -Patterns @(
    'input\.mixed_air_enthalpy_j_per_kg - input\.supply_enthalpy_j_per_kg',
    'input\.supply_mass_flow_rate_kg_per_s \* enthalpy_difference',
    'let cooling_sensible_output_w = calculated_cooling_sensible_output_w;'
) -Description "CP339 exact subtraction, multiplication, assignment grouping"
Assert-Contains -Path $cp339Transition -Pattern '(?s)source_site_execution_count \+=\s*PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "six-site active counter"
Assert-PatternsInOrder -Path $cp339Transition -Patterns @(
    'predecessor\.unit_off_skipped',
    'predecessor\.non_cooling_skipped',
    'predecessor\.positive_guard_false_fallthrough_skipped',
    'predecessor\.capacity_limit_guard_false_fallthrough_skipped',
    'capacity_limit_sensible_output_assignment_count \+= 1',
    'supply_mass_flow_rate_read_count \+= 1',
    'mixed_air_enthalpy_read_count \+= 1',
    'supply_enthalpy_read_count \+= 1',
    'enthalpy_difference_calculation_count \+= 1',
    'cooling_sensible_output_calculation_count \+= 1',
    'cooling_sensible_output_assignment_write_count \+= 1'
) -Description "CP339 five-route and active-site transition order"
Assert-Contains -Path $cp339RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_assignment_count\s*\.checked_mul\(6\)' -Description "CP339 checked 6*A source-site formula"
Assert-Contains -Path $cp339RuntimeValidation -Pattern 'capacity_limit_sensible_output_assignment_count\s*== predecessor\.capacity_limit_cp_air_assignment_count' -Description "CP339 A equals CP338 assignment count"
foreach ($cp339PerSiteCounter in @(
        'supply_mass_flow_rate_read_count',
        'mixed_air_enthalpy_read_count',
        'supply_enthalpy_read_count',
        'enthalpy_difference_calculation_count',
        'cooling_sensible_output_calculation_count',
        'cooling_sensible_output_assignment_write_count'
    )) {
    Assert-Contains -Path $cp339RuntimeValidation -Pattern ($cp339PerSiteCounter + '\s*== assignment_count') -Description "CP339 per-site count equals A for '$cp339PerSiteCounter'"
}
Assert-Contains -Path $cp339SnapshotValidation -Pattern 'enthalpy_difference\.to_bits\(\) == expected_difference\.to_bits\(\)' -Description "CP339 raw IEEE difference validation"
Assert-Contains -Path $cp339SnapshotValidation -Pattern 'calculated\.to_bits\(\) == expected_calculated\.to_bits\(\)' -Description "CP339 raw IEEE product validation"
Assert-Contains -Path $cp339SnapshotValidation -Pattern 'assigned\.to_bits\(\) == calculated\.to_bits\(\)' -Description "CP339 bit-exact local assignment"
Assert-Contains -Path $cp339SnapshotValidation -Pattern '(?s)!snapshot\.supply_mass_flow_rate_read.*?supply_mass_flow_rate_kg_per_s\.is_none\(\).*?!snapshot\.mixed_air_enthalpy_read.*?mixed_air_enthalpy_j_per_kg\.is_none\(\).*?!snapshot\.supply_enthalpy_read.*?supply_enthalpy_j_per_kg\.is_none\(\).*?!snapshot\.enthalpy_difference_calculated.*?mixed_air_minus_supply_enthalpy_j_per_kg\s*\.is_none\(\).*?!snapshot\.cooling_sensible_output_calculated.*?calculated_cooling_sensible_output_w\.is_none\(\).*?!snapshot\.cooling_sensible_output_assigned.*?cooling_sensible_output_w\.is_none\(\)' -Description "complete-null CP339 skip validation"
Assert-NotContains -Path $cp339Transition -Pattern 'is_finite|f64::max|f64::min|mul_add|recip|clamp|energyplus_psy_cp_air_fn_w|cp_air_j_per_kg_k|MaxCoolTotCap|maximum_total_cooling|sizing' -Description "derived finite rejection, reassociation, CpAir, capacity, or sizing work in CP339 transition"
Assert-NotContains -Path $cp339SnapshotValidation -Pattern '(?:expected_difference|expected_calculated|enthalpy_difference|calculated|assigned)\.is_finite\(\)' -Description "derived finite-result rejection in CP339 snapshot validator"
Assert-NotContains -Path $cp339Release -Pattern 'mixed_air_minus_supply_enthalpy_j_per_kg\.is_finite|cooling_sensible_output_w\.is_finite|energyplus_psy_cp_air_fn_w|cp_air_j_per_kg_k|MaxCoolTotCap|maximum_total_cooling|sizing_value|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "derived finite, CpAir, capacity, sizing, or numerical input in CP339 release"

# Public release accepts only CP338 and derives active operands from the
# same-call retained CP330, CP329, and CP336 public/private witnesses.
Assert-Contains -Path $cp339Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp338:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,\s*\)' -Description "CP339 release argument boundary"
Assert-Contains -Path $cp339Release -Pattern 'cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness\(selected\)' -Description "CP339 CP338 private predecessor witness"
Assert-Contains -Path $cp339Release -Pattern 'unit\s*\.calc_cooling_positive_supply_capacity_limit_cp_air_assignment\s*\.latest' -Description "CP339 CP338 public predecessor latest"
Assert-Contains -Path $cp339Release -Pattern 'unit\s*\.calc_cooling_supply_mass_flow_positive_guard\s*\.latest' -Description "CP339 CP330 public operand latest"
Assert-Contains -Path $cp339Release -Pattern 'cooling_supply_mass_flow_positive_guard_latest_witness\(selected\)' -Description "CP339 CP330 private operand witness"
Assert-Contains -Path $cp339Release -Pattern 'unit\.calc_cooling_mixed_air_call\.latest' -Description "CP339 CP329 public operand latest"
Assert-Contains -Path $cp339Release -Pattern 'cooling_mixed_air_call_latest_witness\(selected\)' -Description "CP339 CP329 private operand witness"
Assert-Contains -Path $cp339Release -Pattern 'unit\s*\.calc_cooling_positive_supply_enthalpy_assignment\s*\.latest' -Description "CP339 CP336 public operand latest"
Assert-Contains -Path $cp339Release -Pattern 'cooling_positive_supply_enthalpy_assignment_latest_witness\(selected\)' -Description "CP339 CP336 private operand witness"
Assert-Contains -Path $cp339Release -Pattern 'positive_guard\.supply_mass_flow_rate_kg_per_s\.ok_or' -Description "CP330-owned retained flow operand"
Assert-Contains -Path $cp339Release -Pattern 'mixed_air\.mixed_air_enthalpy_projection_j_per_kg\.ok_or' -Description "CP329-owned retained mixed-air enthalpy operand"
Assert-Contains -Path $cp339Release -Pattern 'supply_enthalpy\.supply_enthalpy_j_per_kg\.ok_or' -Description "CP336-owned retained supply enthalpy operand"
Assert-Contains -Path $cp339Release -Pattern 'completed_direct_cooling_positive_supply_capacity_limit_cp_air_assignment_is_consistent' -Description "recursive CP338 release proof"
Assert-Contains -Path $cp339Release -Pattern 'completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent' -Description "recursive CP330 release proof"
Assert-Contains -Path $cp339Release -Pattern 'completed_direct_cooling_mixed_air_call_is_consistent' -Description "recursive CP329 release proof"
Assert-Contains -Path $cp339Release -Pattern 'completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent' -Description "recursive CP336 release proof"
Assert-Contains -Path $cp339Release -Pattern 'set_cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness' -Description "CP339 private witness commit"
Assert-Contains -Path $cp339PrefixValidation -Pattern '(?s)positive_guard\s*\.supply_mass_flow_rate_kg_per_s\s*\.is_some_and\(\|value\| value\.to_bits\(\) == supply_mass_flow_rate_kg_per_s\.to_bits\(\)\)' -Description "CP330 bit-exact flow provenance"
Assert-Contains -Path $cp339PrefixValidation -Pattern '(?s)mixed_air\s*\.mixed_air_enthalpy_projection_j_per_kg\s*\.is_some_and\(\|value\| value\.to_bits\(\) == mixed_air_enthalpy_j_per_kg\.to_bits\(\)\)' -Description "CP329 bit-exact mixed-air enthalpy provenance"
Assert-Contains -Path $cp339PrefixValidation -Pattern '(?s)supply_enthalpy\s*\.supply_enthalpy_j_per_kg\s*\.is_some_and\(\|value\| value\.to_bits\(\) == supply_enthalpy_j_per_kg\.to_bits\(\)\)' -Description "CP336 bit-exact supply enthalpy provenance"

foreach ($cp339CoreTest in @(
        'source_boundary_and_exact_six_sites_are_stable',
        'active_assignment_executes_six_sites_in_source_grouping',
        'all_four_skipped_routes_are_complete_null_and_execute_no_sites',
        'counters_partition_five_routes_and_count_each_site_once_per_assignment',
        'exact_predicate_rejects_derived_value_and_route_corruption'
    )) {
    Assert-Contains -Path $cp339Tests -Pattern $cp339CoreTest -Description "CP339 core regression '$cp339CoreTest'"
}
foreach ($cp339IeeeTest in @(
        'pure_transition_preserves_raw_ieee_subtraction_then_multiplication',
        'source_grouping_is_not_distributed_or_reassociated',
        'exact_snapshot_accepts_derived_nan_and_matcher_is_bit_exact',
        'signed_zero_is_preserved_through_delta_product_and_assignment'
    )) {
    Assert-Contains -Path $cp339IeeeTests -Pattern $cp339IeeeTest -Description "CP339 IEEE regression '$cp339IeeeTest'"
}
foreach ($cp339ReleaseTest in @(
        'public_active_release_uses_only_retained_cp330_cp329_and_cp336_operands',
        'public_release_preserves_all_four_complete_null_skip_routes',
        'supplied_public_and_private_cp338_drift_is_transactional',
        'active_cp330_public_and_private_operand_drift_is_transactional',
        'active_cp329_public_and_private_operand_drift_is_transactional',
        'active_cp336_public_and_private_operand_drift_is_transactional',
        'every_active_counter_overflow_is_preflighted_transactionally',
        'every_skip_route_counter_overflow_is_preflighted_transactionally',
        'route_partition_product_corruption_and_post_commit_drift_are_detected'
    )) {
    Assert-Contains -Path $cp339ReleaseCorruptionTests -Pattern $cp339ReleaseTest -Description "CP339 release regression '$cp339ReleaseTest'"
}
foreach ($cp339ReleaseEdgeCaseTest in @(
        'public_active_complete_lineage_allows_infinite_flow_times_zero_delta_nan',
        'public_skip_does_not_require_available_active_operand_values',
        'nan_flow_and_nonfinite_enthalpy_lineage_are_transactionally_rejected'
    )) {
    Assert-Contains -Path $cp339ReleaseEdgeCaseTests -Pattern $cp339ReleaseEdgeCaseTest -Description "CP339 release edge-case regression '$cp339ReleaseEdgeCaseTest'"
}

# CP339 state and latest witness are private to the selected runtime unit.
Assert-Contains -Path $cp339InitState -Pattern '(?s)cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot' -Description "runtime-root private CP339 witness map"
Assert-NotContains -Path $cp339InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witnesses:' -Description "public runtime-root CP339 witness map"
Assert-Contains -Path $cp339InitWitnessRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_assignment;' -Description "runtime-root CP339 witness module"
Assert-Contains -Path $cp339InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness\s*\(' -Description "runtime-root CP339 witness getter"
Assert-Contains -Path $cp339InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness\s*\(' -Description "runtime-root CP339 witness setter"
Assert-Contains -Path $cp339InitState -Pattern 'pub calc_cooling_positive_supply_capacity_limit_sensible_output_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState' -Description "per-unit CP339 persistent state"
Assert-Contains -Path $cp339InitUnit -Pattern '(?s)calc_cooling_positive_supply_capacity_limit_sensible_output_assignment:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new\(\s*system\s*,?\s*\)' -Description "per-unit CP339 state initialization"

# Binding preserves exact CP338 -> CP339 -> numerical order with no helper in
# either firewall interval.
$cp339BindingText = Read-RepoText -Path $cp339Binding
$cp338BindingIndexForCp339 = $cp339BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndex = $cp339BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp339 = $cp339BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp338BindingIndexForCp339 -lt 0 -or
    $cp339BindingIndex -le $cp338BindingIndexForCp339 -or
    $numericalBindingIndexForCp339 -le $cp339BindingIndex
) {
    throw "Binding must retain exact CP338 -> CP339 -> numerical Calc order"
}
$cp338BindingCallForCp339 = [regex]::Match(
    $cp339BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
$cp339BindingCall = [regex]::Match(
    $cp339BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (-not $cp338BindingCallForCp339.Success -or -not $cp339BindingCall.Success) {
    throw "Binding must retain complete CP338 and CP339 exact release calls"
}
$cp338BindingCallEndForCp339 =
    $cp338BindingCallForCp339.Index + $cp338BindingCallForCp339.Length
$cp339BindingCallEnd = $cp339BindingCall.Index + $cp339BindingCall.Length
if (
    $cp339BindingIndex -lt $cp338BindingCallEndForCp339 -or
    $numericalBindingIndexForCp339 -lt $cp339BindingCallEnd
) {
    throw "CP338 and CP339 exact release calls must complete before numerical Calc"
}
foreach ($cp339Interval in @(
        [PSCustomObject]@{
            Start = $cp338BindingCallEndForCp339
            End = $cp339BindingIndex
            Description = "after CP338 and before CP339"
        },
        [PSCustomObject]@{
            Start = $cp339BindingCallEnd
            End = $numericalBindingIndexForCp339
            Description = "after CP339 and before numerical Calc"
        }
    )) {
    $cp339IntervalText = $cp339BindingText.Substring(
        $cp339Interval.Start,
        $cp339Interval.End - $cp339Interval.Start
    )
    $cp339IntervalCode = [regex]::Replace($cp339IntervalText, '(?m)//.*$', '')
    $cp339IntervalCode = [regex]::Replace(
        $cp339IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
        ''
    )
    $cp339IntervalCode = [regex]::Replace(
        $cp339IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
    $cp339IntervalCode = [regex]::Replace(
        $cp339IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
        ''
    )
    $cp339IntervalCode = [regex]::Replace(
        $cp339IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
    $cp339IntervalCode = [regex]::Replace(
        $cp339IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    $cp339IntervalCode = [regex]::Replace(
        $cp339IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp339IntervalCode = [regex]::Replace(
        $cp339IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp339IntervalCode = [regex]::Replace(
        $cp339IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp339IntervalCode = [regex]::Replace(
    $cp339IntervalCode,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;)',
    ''
)
    if ($cp339IntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp339Interval.Description)"
    }
}
Assert-Contains -Path $cp339Binding -Pattern '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_capacity_limit_cp_air_assignment,\s*\)\?;' -Description "binding exact CP338-to-CP339 adapter call"
Assert-Contains -Path $cp339BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_capacity_limit_sensible_output_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,\s*\)' -Description "CP339 binding adapter arguments"
Assert-Contains -Path $cp339BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment\(\s*runtime,\s*system,\s*predecessor,\s*\).*?CalculationCoolingPositiveSupplyCapacityLimitSensibleOutputAssignment' -Description "CP339 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp339BindingAdapter -Pattern 'supply_mass_flow_rate\s*:|mixed_air_enthalpy\s*:|supply_enthalpy\s*:|cp_air_j_per_kg_k|MaxCoolTotCap|maximum.*capacity|sizing|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "extra scalar, CpAir, capacity, sizing, or numerical input in CP339 adapter"
Assert-Contains -Path $cp339ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot' -Description "CP339 scheduled output evidence"
Assert-Contains -Path $cp339BindingTestsRoot -Pattern '#\[rustfmt::skip\]\s*#\[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_assignment_tests\.rs"\]\s*mod cooling_positive_supply_capacity_limit_sensible_output_assignment_tests;' -Description "CP339 binding test module"
Assert-Contains -Path $cp339BindingTests -Pattern 'scheduled_binding_assigns_exact_sensible_output_for_both_capacity_limit_selectors' -Description "CP339 active binding regression"
Assert-Contains -Path $cp339BindingTests -Pattern 'scheduled_binding_preserves_all_complete_null_skip_routes' -Description "CP339 complete-null binding regression"

# Coupled runtime and pipeline independently reconstruct CP339 from CP338 plus
# retained CP330/CP329/CP336 operands and serialize numeric/bit projections.
Assert-Contains -Path $cp339CoupledRuntime -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_assignment_validation;' -Description "coupled CP339 validator declaration"
Assert-Contains -Path $cp339CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary' -Description "coupled CP339 lifecycle"
Assert-Contains -Path $cp339CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_assignment_validation::snapshot_matches_release' -Description "coupled per-timestep CP339 validation"
Assert-Contains -Path $cp339CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_assignment_validation::validate_lifecycle' -Description "coupled final CP339 validation"
foreach ($cp339CoupledOperand in @(
        'output\.calculation_cooling_positive_supply_capacity_limit_cp_air_assignment',
        'output\.calculation_cooling_supply_mass_flow_positive_guard',
        'output\.calculation_cooling_mixed_air_call',
        'output\.calculation_cooling_positive_supply_enthalpy_assignment'
    )) {
    Assert-Contains -Path $cp339CoupledValidation -Pattern $cp339CoupledOperand -Description "coupled CP339 retained predecessor/operand '$cp339CoupledOperand'"
}
Assert-PatternsInOrder -Path $cp339CoupledValidation -Patterns @(
    'mixed_air_enthalpy_j_per_kg - supply_enthalpy_j_per_kg',
    'supply_mass_flow_rate_kg_per_s \* mixed_air_minus_supply_enthalpy_j_per_kg'
) -Description "coupled CP339 exact arithmetic grouping"
Assert-Contains -Path $cp339CoupledValidation -Pattern '(?s)checked_mul\(\s*state\.capacity_limit_sensible_output_assignment_count,\s*6,' -Description "coupled checked 6*A formula"
Assert-Contains -Path $cp339CoupledValidation -Pattern '(?s)"selector_capacity_limit_body_entry_count",\s*selector\.capacity_limit_body_entry_count,\s*state\.capacity_limit_sensible_output_assignment_count' -Description "coupled CP337 body-entry parity"
Assert-Contains -Path $cp339CoupledValidation -Pattern 'same_call_identity_and_fixed_selector_forgeries_are_rejected' -Description "coupled CP337 selector-forgery regression"
Assert-Contains -Path $cp339CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_assignment_fixture;' -Description "coupled CP339 fixture declaration"
Assert-Contains -Path $cp339CoupledFixture -Pattern 'calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot' -Description "coupled CP339 fixture output"

Assert-Contains -Path $cp339PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_assignment;' -Description "pipeline CP339 module declaration"
Assert-Contains -Path $cp339PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle' -Description "pipeline CP339 lifecycle field and JSON key"
Assert-Contains -Path $cp339PipelineRoot -Pattern 'calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle' -Description "pipeline CP339 coupled lifecycle transfer"
Assert-Contains -Path $cp339Pipeline -Pattern '(?s)validate_direct_lifecycle\(.*?predecessor_cp338.*?selector_cp337.*?supply_flow_cp330.*?mixed_air_cp329.*?supply_enthalpy_cp336.*?init_lifecycle.*?cooling_limit.*?coupling_call_count' -Description "pipeline CP339 validated inputs"
Assert-Contains -Path $cp339Pipeline -Pattern 'predecessor_state\.capacity_limit_cp_air_assignment_count' -Description "pipeline CP338 assignment parity"
Assert-Contains -Path $cp339Pipeline -Pattern '(?s)"selector_capacity_limit_body_entry_count",\s*selector_state\.capacity_limit_body_entry_count,\s*state\.capacity_limit_sensible_output_assignment_count' -Description "pipeline CP337 body-entry parity"
Assert-Contains -Path $cp339Pipeline -Pattern 'validate_source_counters\(state\)' -Description "pipeline CP339 source-counter validation"
Assert-Contains -Path $cp339PipelineValidation -Pattern 'checked_product\(assignments,\s*6,\s*"source-site count"\)' -Description "pipeline checked 6*A source-site formula"
Assert-Contains -Path $cp339PipelineValidation -Pattern 'source_counter_overflow_fails_closed' -Description "pipeline CP339 overflow regression"
Assert-Contains -Path $cp339PipelineSerialization -Pattern '"capacity_limit_sensible_output_assignment_count"' -Description "pipeline CP339 lifecycle serialization"
foreach ($cp339JsonField in @(
        'supply_mass_flow_rate_kg_per_s',
        'supply_mass_flow_rate_kg_per_s_ieee_bits',
        'mixed_air_enthalpy_j_per_kg',
        'mixed_air_enthalpy_j_per_kg_ieee_bits',
        'supply_enthalpy_j_per_kg',
        'supply_enthalpy_j_per_kg_ieee_bits',
        'mixed_air_minus_supply_enthalpy_j_per_kg',
        'mixed_air_minus_supply_enthalpy_j_per_kg_ieee_bits',
        'calculated_cooling_sensible_output_w',
        'calculated_cooling_sensible_output_w_ieee_bits',
        'cooling_sensible_output_w',
        'cooling_sensible_output_w_ieee_bits'
    )) {
    Assert-Contains -Path $cp339PipelineSnapshotSerialization -Pattern ('"' + $cp339JsonField + '"') -Description "pipeline CP339 snapshot field '$cp339JsonField'"
}
Assert-Contains -Path $cp339PipelineSnapshotSerialization -Pattern '(?s)fn json_number\(value: Option<f64>\) -> Value.*?filter\(\|value\| value\.is_finite\(\)\).*?map_or\(Value::Null' -Description "CP339 serde nonfinite numeric null projection"
Assert-Contains -Path $cp339PipelineSnapshotSerialization -Pattern 'value\.map\(\|value\| format!\("0x\{:016x\}", value\.to_bits\(\)\)\)' -Description "CP339 non-null IEEE bit projection"
Assert-Contains -Path $cp339PipelineSnapshotSerialization -Pattern 'nonfinite_numeric_is_null_while_ieee_bits_remain_authoritative' -Description "CP339 JSON nonfinite projection regression"
Assert-Contains -Path $cp339PipelineSnapshotSerialization -Pattern 'full_snapshot_json_preserves_active_nonfinite_bits_and_skip_nulls' -Description "CP339 active-bit and skip-null JSON regression"
Assert-Contains -Path $cp339RunTests -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_assignment_assertions' -Description "direct-run CP339 assertion module"
Assert-Contains -Path $cp339DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*6\]\s*=' -Description "direct-run CP339 source order"
Assert-Contains -Path $cp339DirectAssertions -Pattern 'source_site_execution_count"\], assignments \* 6' -Description "direct-run CP339 6*A formula"
Assert-Contains -Path $cp339DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle' -Description "direct-run CP338 predecessor evidence"
Assert-Contains -Path $cp339DirectAssertions -Pattern 'purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle' -Description "direct-run CP330 operand evidence"
Assert-Contains -Path $cp339DirectAssertions -Pattern 'purchased_air_calc_cooling_mixed_air_call_lifecycle' -Description "direct-run CP329 operand evidence"
Assert-Contains -Path $cp339DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle' -Description "direct-run CP336 operand evidence"
Assert-Contains -Path $cp339NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle' -Description "non-direct CP339 null evidence"
Assert-Contains -Path $cp339PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp369_lifecycle_evidence' -Description "non-direct CP339 through CP363 evidence rejection"
Assert-NotContains -Path $cp339Pipeline -Pattern 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|complete_direct_zone_purchased_air_coupling|cp_air_j_per_kg_k|MaxCoolTotCap|maximum_total_cooling|sizing' -Description "numerical DTO, CpAir, capacity, or sizing feed in CP339 pipeline"

# Exactly two algorithm addenda, two capability addenda, and six target
# occurrences extend inventory without support/readiness promotion.
$cp339AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp339AlgorithmAddenda = [regex]::Matches(
    $cp339AlgorithmText,
    '(?m)^\s*"CP339 supersedes only CP338[^"\r\n]+",\s*$'
)
if ($cp339AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP339 claim addenda"
}
$cp339TargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_assignment\.rs::purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp339Target in $cp339TargetCounts) {
    $cp339TargetCount = [regex]::Matches($cp339AlgorithmText, $cp339Target.Pattern).Count
    if ($cp339TargetCount -ne $cp339Target.Expected) {
        throw "CP339 target '$($cp339Target.Pattern)' expected $($cp339Target.Expected), found $cp339TargetCount"
    }
}
$cp339CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp339CapabilityAddenda = [regex]::Matches(
    $cp339CapabilityText,
    '(?m)^\s*"CP339 additionally requires[^"\r\n]+",\s*$'
)
if ($cp339CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP339 claim addenda"
}
foreach ($cp339Claim in @($cp339AlgorithmAddenda) + @($cp339CapabilityAddenda)) {
    foreach ($cp339Pattern in @(
            '(?s)PurchasedAirManager\.cc.*?2197',
            $cp339SourceStatementPattern,
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            $cp339OrderedSourceSitesPattern,
            'CapacityLimitCpAirAssigned',
            'CapacityLimitSensibleOutputAssigned',
            '6\*A\s*=\s*6\*B',
            'supply_mass_flow_rate_kg_per_s',
            'mixed_air_enthalpy_projection_j_per_kg',
            'supply_enthalpy_j_per_kg',
            'raw IEEE',
            'derived finite-result rejection',
            '\+infinity',
            'serde JSON',
            'nonfinite numeric',
            'bit\s+string',
            $cp339SerializationOwnershipPattern,
            'CP338-to-CP339-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle',
            $cp339FirstExcludedStatementPattern,
            '2198',
            'CP340',
            '(?:Roadmap state changes|Roadmap promotion)'
        )) {
        if ($cp339Claim.Value -notmatch $cp339Pattern) {
            throw "CP339 spec addendum missing '$cp339Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP339 supersedes only CP338' -Description "generated CP339 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP339 additionally requires' -Description "generated CP339 capability index"

# Each hand-authored contract has one scoped CP339 section carrying the same
# source, routes, operand lineage, IEEE/JSON boundary, exclusion, and no-promotion.
$cp339DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP339 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP339 Source-Ordered Cooling Positive-Supply Capacity-Limit Sensible-Output Assignment\r?\n.*?Roadmap state remain unchanged\.\r?\n'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP339 Cooling Positive-Supply Capacity-Limit Sensible-Output Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP339 Positive-Supply Capacity-Limit Sensible-Output Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP339 Cooling Positive-Supply Capacity-Limit Sensible-Output Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp339Documentation in $cp339DocumentationSections) {
    $cp339DocumentText = Read-RepoText -Path $cp339Documentation.Path
    $cp339Matches = [regex]::Matches($cp339DocumentText, $cp339Documentation.Pattern)
    if ($cp339Matches.Count -ne 1) {
        throw "CP339 documentation expected one scoped section in $($cp339Documentation.Path), found $($cp339Matches.Count)"
    }
    $cp339Section = $cp339Matches[0].Value
    foreach ($cp339Pattern in @(
            '(?s)PurchasedAirManager\.cc.*?2197',
            $cp339SourceStatementPattern,
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            $cp339OrderedSourceSitesPattern,
            'CapacityLimitCpAirAssigned',
            'UnitOff',
            'NonCooling',
            'PositiveGuardFalseFallthrough',
            'ActiveCapacityLimitGuardFalseFallthrough',
            'CapacityLimitSensibleOutputAssigned',
            '6\*A\s*=\s*6\*B',
            'CP330 latest/private\s+witness',
            'supply_mass_flow_rate_kg_per_s',
            'CP329 latest/private\s+witness',
            'mixed_air_enthalpy_projection_j_per_kg',
            'CP336 latest/private\s+witness',
            'supply_enthalpy_j_per_kg',
            'raw IEEE',
            'derived finite-result rejection',
            '\+infinity',
            'Serde\s+JSON',
            'nonfinite numeric',
            'bit\s+string',
            $cp339SerializationOwnershipPattern,
            'CP338-to-CP339-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle',
            $cp339FirstExcludedStatementPattern,
            '2198',
            'CP340',
            '(?i)numerical[- ]DTO',
            'Roadmap'
        )) {
        if ($cp339Section -notmatch $cp339Pattern) {
            throw "CP339 documentation in $($cp339Documentation.Path) missing '$cp339Pattern'"
        }
    }
}

# Root reachability and generated inventory account for one new internal
# script: 284 executable records, 240 public, 44 internal, and zero uncalled.
$cp339MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp338DotSourceIndexForCp339 = $cp339MainAuditText.IndexOf('ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1')
$cp339DotSourceIndex = $cp339MainAuditText.IndexOf('ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1')
$cp339AuditCompletionIndex = $cp339MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp338DotSourceIndexForCp339 -lt 0 -or
    $cp339DotSourceIndex -le $cp338DotSourceIndexForCp339 -or
    $cp339AuditCompletionIndex -le $cp339DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP339 after CP338 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 307' -Description "CP339 cumulative inventory total through CP358"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment\.ps1"' -Description "CP339 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment\.ps1::dot_sources' -Description "CP339 main-audit callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 307 \|' -Description "CP339 generated script count through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP339 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 67 \|' -Description "CP339 generated internal script count through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP339 generated uncalled script count"
