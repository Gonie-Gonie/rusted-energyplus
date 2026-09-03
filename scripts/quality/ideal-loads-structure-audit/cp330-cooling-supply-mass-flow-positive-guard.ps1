# CP330 maps only PurchasedAirManager.cc executable line 2183: the complete
# Cooling positive-supply-flow guard. Line 2185 is the first excluded lexical
# executable; line 2184 is commentary.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp330Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard.rs"
$cp330State = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\state.rs"
$cp330Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\transition.rs"
$cp330Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\release.rs"
$cp330PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\release\prefix_validation.rs"
$cp330RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\release\runtime_validation.rs"
$cp330SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\release\snapshot_validation.rs"
$cp330Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\tests\mod.rs"
$cp330ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\tests\release_corruption.rs"
$cp330Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp330Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp330ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp330BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_supply_mass_flow_positive_guard.rs"
$cp330BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp330BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_supply_mass_flow_positive_guard_tests.rs"
$cp330InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp330InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp330InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp330InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_supply_mass_flow_positive_guard.rs"
$cp330CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp330CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_supply_mass_flow_positive_guard_validation.rs"
$cp330CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_supply_mass_flow_positive_guard_fixture.rs"
$cp330CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp330PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp330Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_positive_guard.rs"
$cp330PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_positive_guard\validation.rs"
$cp330PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_positive_guard\serialization.rs"
$cp330PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_positive_guard\serialization\snapshot.rs"
$cp330RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp330DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_supply_mass_flow_positive_guard_assertions.rs"
$cp330NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp330RequiredFile in @(
        $cp330Module,
        $cp330State,
        $cp330Transition,
        $cp330Release,
        $cp330PrefixValidation,
        $cp330RuntimeValidation,
        $cp330SnapshotValidation,
        $cp330Tests,
        $cp330ReleaseCorruptionTests,
        $cp330ScheduledOutput,
        $cp330BindingAdapter,
        $cp330BindingTests,
        $cp330InitUnit,
        $cp330InitWitness,
        $cp330CoupledValidation,
        $cp330CoupledFixture,
        $cp330Pipeline,
        $cp330PipelineValidation,
        $cp330PipelineSerialization,
        $cp330PipelineSnapshotSerialization,
        $cp330DirectAssertions
    )) {
    Assert-FileExists -Path $cp330RequiredFile -Description "CP330 positive-supply guard structure"
}
Assert-LineLimit -Path $cp330Release -Limit 800 -Description "CP330 release root module"
Assert-LineLimit -Path $cp330RuntimeValidation -Limit 800 -Description "CP330 runtime validation module"

# Exact source boundary and three-site lexical inventory. The zero literal is
# part of the comparison, while body entry is dynamically executed only on true.
Assert-Contains -Path $cp330Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2183' -Description "CP330 exact source boundary"
Assert-Contains -Path $cp330Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2185' -Description "CP330 first excluded lexical executable"
Assert-Contains -Path $cp330Module -Pattern 'Exact three textual source sites represented by CP330' -Description "CP330 exact lexical-site count"
Assert-ExactStringArray -Path $cp330Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER" -Expected @(
    "read-retained-supply-mass-flow-rate",
    "compare-supply-mass-flow-rate-strictly-greater-than-positive-zero",
    "enter-positive-supply-mass-flow-body-if-satisfied"
) -Description "CP330 exact three lexical source sites"
Assert-Contains -Path $cp330Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot' -Description "CP330 public snapshot"
Assert-Contains -Path $cp330State -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState' -Description "CP330 persistent state"
Assert-Contains -Path $cp330Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary' -Description "CP330 lifecycle summary"
Assert-Contains -Path $cp330Module -Pattern 'pub fn purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary\s*\(' -Description "CP330 lifecycle accessor"
Assert-Contains -Path $cp330Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard\s*\(' -Description "CP330 exact-direct wrapper"
Assert-Contains -Path $cp330Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_supply_mass_flow_positive_guard_state\s*\(' -Description "CP330 pure transition"

# UnitOff/non-cooling skip both unconditional sites. Active Cooling executes the
# retained read and strict comparison once, then the entry site only when true.
Assert-PatternsInOrder -Path $cp330Transition -Patterns @(
    'let cooling = predecessor\.cooling_call_executed;',
    'let supply_mass_flow_rate_kg_per_s = if cooling',
    'predecessor\.supply_mass_flow_rate_kg_per_s',
    'else \{\s*None\s*\};',
    '\.map\(\|supply\| supply > 0\.0\);',
    'let body_entered = strictly_positive == Some\(true\);',
    'let false_fallthrough = strictly_positive == Some\(false\);',
    'if predecessor\.unit_off_skipped',
    'else if predecessor\.non_cooling_skipped',
    'state\.cooling_body_entry_count \+= 1;',
    'state\.source_site_execution_count \+= 2 \+ usize::from\(body_entered\);',
    'state\.supply_mass_flow_rate_read_count \+= 1;',
    'state\.supply_mass_flow_rate_strictly_positive_comparison_count \+= 1;',
    'if body_entered',
    'state\.positive_supply_mass_flow_body_entry_count \+= 1;',
    'state\.active_guard_false_fallthrough_count \+= 1;'
) -Description "CP330 raw comparison, conditional entry, and counter order"
Assert-Contains -Path $cp330Transition -Pattern 'PositiveSupplyMassFlowBodyEntered' -Description "CP330 true body-entry route"
Assert-Contains -Path $cp330Transition -Pattern 'ActiveGuardFalseFallthrough' -Description "CP330 active false route"
Assert-NotContains -Path $cp330Transition -Pattern '\.abs\(|\.is_finite\(|total_cmp|partial_cmp|\.clamp\(|f64::(?:min|max)|\.(?:min|max)\(' -Description "CP330 replacement comparison or normalization"
Assert-NotContains -Path $cp330Transition -Pattern 'PsyCp|PsyH|supply_(?:temperature|humidity|enthalpy)|capacity|Heat/DeadBand|2454|2465' -Description "CP330 line-2185-or-later source behavior"

# Snapshot/runtime validation independently recomputes raw IEEE `> +0.0` and
# enforces the dynamic 2*active+true-entry source-site total.
Assert-Contains -Path $cp330SnapshotValidation -Pattern 'let strictly_positive = supply_mass_flow_rate_kg_per_s > 0\.0;' -Description "CP330 snapshot source comparison"
Assert-Contains -Path $cp330SnapshotValidation -Pattern 'snapshot\.positive_supply_mass_flow_body_entered == strictly_positive' -Description "CP330 true-entry consistency"
Assert-Contains -Path $cp330SnapshotValidation -Pattern 'snapshot\.active_guard_false_fallthrough != strictly_positive' -Description "CP330 false-route consistency"
Assert-Contains -Path $cp330SnapshotValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "CP330 exact retained supply bits"
Assert-Contains -Path $cp330RuntimeValidation -Pattern '(?s)cooling_body_entry_count\s*\.checked_mul\(2\).*?checked_add\(state\.positive_supply_mass_flow_body_entry_count\)' -Description "CP330 dynamic source-site formula"
foreach ($cp330Counter in @(
        "source_site_execution_count",
        "supply_mass_flow_rate_read_count",
        "supply_mass_flow_rate_strictly_positive_comparison_count",
        "positive_supply_mass_flow_body_entry_count",
        "active_guard_false_fallthrough_count"
    )) {
    Assert-Contains -Path $cp330RuntimeValidation -Pattern $cp330Counter -Description "CP330 runtime counter '$cp330Counter'"
}
Assert-Contains -Path $cp330State -Pattern 'pub\(super\) witnessed_positive_supply_mass_flow_body_entry_count:\s*usize' -Description "CP330 private true-route witness"
Assert-Contains -Path $cp330State -Pattern 'pub\(super\) witnessed_active_guard_false_fallthrough_count:\s*usize' -Description "CP330 private false-route witness"
Assert-Contains -Path $cp330Tests -Pattern 'strict_positive_guard_preserves_source_double_comparison_semantics' -Description "CP330 raw IEEE comparator regression"
Assert-Contains -Path $cp330Tests -Pattern 'skipped_routes_execute_no_cp330_source_sites' -Description "CP330 skip-route regression"
Assert-Contains -Path $cp330Tests -Pattern 'counters_partition_routes_and_count_only_executed_source_sites' -Description "CP330 dynamic site-count regression"
Assert-Contains -Path $cp330Tests -Pattern 'exact_predicate_rejects_forged_provenance_and_comparison_results' -Description "CP330 exact snapshot regression"

# Exact release proves the full retained CP329 -> CP328 chain, including each
# latest snapshot, private witness, and retained route-history partition.
Assert-Contains -Path $cp330Release -Pattern 'PurchasedAirCalcCoolingMixedAirCallSnapshot' -Description "CP330 CP329 predecessor type"
Assert-Contains -Path $cp330Release -Pattern 'completed_direct_cooling_mixed_air_call_is_consistent\s*\(' -Description "CP330 completed CP329 validation"
Assert-Contains -Path $cp330Release -Pattern 'cooling_mixed_air_call_latest_witness\s*\(' -Description "CP330 retained CP329 private witness"
Assert-Contains -Path $cp330Release -Pattern 'unit\.calc_cooling_mixed_air_call[\r\n\s.]+latest' -Description "CP330 retained CP329 latest snapshot"
Assert-Contains -Path $cp330PrefixValidation -Pattern '(?s)guard\.supply_mass_flow_rate_kg_per_s,.*?predecessor\.supply_mass_flow_rate_kg_per_s' -Description "CP330 bit-exact CP329 supply lineage"
Assert-Contains -Path $cp330Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp329: PurchasedAirCalcCoolingMixedAirCallSnapshot,\s*\)' -Description "CP330 exact wrapper arguments"
Assert-Contains -Path $cp330Release -Pattern '(?s)pub\(in crate::ideal_loads::calc\) fn completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent\(\s*runtime: &PurchasedAirRuntimeState,\s*unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,\s*system: &IdealLoadsAirSystem,\s*snapshot: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,\s*witness: Option<PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot>' -Description "CP330 runtime-aware completed helper"
Assert-Contains -Path $cp330Release -Pattern '(?s)completed_direct_cooling_mixed_air_call_is_consistent\(\s*runtime,\s*unit,\s*system,\s*predecessor,\s*runtime\.cooling_mixed_air_call_latest_witness\(system\.id\),\s*\)' -Description "CP330 recursive CP329 completed/private-witness proof"
Assert-Contains -Path $cp330RuntimeValidation -Pattern '(?s)fn pending_positive_guard_state_is_consistent\(.*?unit_off_skip_count.*?checked_add\(usize::from\(predecessor\.unit_off_skipped\)\).*?non_cooling_skip_count.*?checked_add\(usize::from\(predecessor\.non_cooling_skipped\)\).*?cooling_body_entry_count.*?checked_add\(usize::from\(predecessor\.cooling_call_executed\)\)' -Description "CP330 pending route-history parity with current CP329 route"
Assert-Contains -Path $cp330RuntimeValidation -Pattern '(?s)fn completed_positive_guard_state_is_consistent\(.*?state\.transition_count == predecessor\.transition_count.*?state\.unit_off_skip_count == predecessor\.unit_off_skip_count.*?state\.non_cooling_skip_count == predecessor\.non_cooling_skip_count.*?state\.cooling_body_entry_count == predecessor\.cooling_call_count' -Description "CP330 exact completed CP329 route-history equality"
Assert-Contains -Path $cp330RuntimeValidation -Pattern 'pub\(in crate::ideal_loads::calc\) fn next_positive_guard_transition_fits\s*\(' -Description "CP330 route-aware next-transition preflight"
foreach ($cp330PreflightCounter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "cooling_body_entry_count",
        "source_site_execution_count",
        "supply_mass_flow_rate_read_count",
        "supply_mass_flow_rate_strictly_positive_comparison_count",
        "positive_supply_mass_flow_body_entry_count",
        "witnessed_positive_supply_mass_flow_body_entry_count",
        "active_guard_false_fallthrough_count",
        "witnessed_active_guard_false_fallthrough_count"
    )) {
    Assert-Contains -Path $cp330RuntimeValidation -Pattern ($cp330PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP330 checked preflight counter '$cp330PreflightCounter'"
}
$cp330ReleaseText = Read-RepoText -Path $cp330Release
$cp330ReleaseWrapper = [regex]::Match(
    $cp330ReleaseText,
    '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard\(.*?(?=\r?\nfn call_order_error\()'
)
if (-not $cp330ReleaseWrapper.Success) {
    throw "CP330 exact release wrapper must remain structurally bounded"
}
$cp330WrapperText = $cp330ReleaseWrapper.Value
$cp330MutationIndex = $cp330WrapperText.IndexOf("let snapshot =")
if ($cp330MutationIndex -lt 0) {
    throw "CP330 release mutation boundary must remain explicit"
}
foreach ($cp330ValidationCall in @(
        "pending_positive_guard_state_is_consistent(",
        "next_positive_guard_transition_fits(",
        "completed_direct_cooling_mixed_air_call_is_consistent("
    )) {
    $cp330ValidationIndex = $cp330WrapperText.IndexOf($cp330ValidationCall)
    if ($cp330ValidationIndex -lt 0 -or $cp330ValidationIndex -ge $cp330MutationIndex) {
        throw "CP330 validation '$cp330ValidationCall' must complete before transition mutation"
    }
}
$cp330NextFitIndex = $cp330WrapperText.IndexOf("next_positive_guard_transition_fits(")
$cp330RecursivePredecessorIndex =
    $cp330WrapperText.IndexOf("completed_direct_cooling_mixed_air_call_is_consistent(")
if ($cp330RecursivePredecessorIndex -le $cp330NextFitIndex) {
    throw "CP330 checked-overflow preflight must precede recursive predecessor validation in the release short-circuit"
}
Assert-PatternsInOrder -Path $cp330Release -Patterns @(
    'let snapshot = \{',
    '\.get_mut\(&selected\)',
    'advance_cooling_supply_mass_flow_positive_guard_state\(',
    'set_cooling_supply_mass_flow_positive_guard_latest_witness\('
) -Description "CP330 validated transition and witness mutation order"
Assert-NotContains -Path $cp330Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard\([^)]*supply_mass_flow_rate_kg_per_s\s*:' -Description "duplicate caller scalar in CP330 release"
Assert-NotContains -Path $cp330Release -Pattern 'ems_actuator|ems_service|node_service|psychrometric|schedule_service|diagnostic_service' -Description "live service input in CP330 release"
Assert-NotContains -Path $cp330Release -Pattern 'complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow' -Description "numerical DTO input in CP330 release"
Assert-Contains -Path $cp330ReleaseCorruptionTests -Pattern 'public_release_commits_once_and_rejects_replay_without_mutation' -Description "CP330 release replay regression"
Assert-Contains -Path $cp330ReleaseCorruptionTests -Pattern 'forged_cp329_snapshot_or_private_witness_fails_before_cp330_mutation' -Description "CP330 CP329 witness corruption regression"
Assert-Contains -Path $cp330ReleaseCorruptionTests -Pattern 'corrupted_cp328_private_witness_invalidates_cp329_and_cp330_admission' -Description "CP330 recursive CP328 private-witness corruption regression"
Assert-Contains -Path $cp330ReleaseCorruptionTests -Pattern 'completed_cp329_counter_corruption_fails_before_cp330_mutation' -Description "CP330 completed predecessor corruption regression"
Assert-Contains -Path $cp330ReleaseCorruptionTests -Pattern 'active_source_site_increment_overflow_is_fail_closed_and_non_mutating' -Description "CP330 overflow transaction regression"
Assert-Contains -Path $cp330ReleaseCorruptionTests -Pattern '(?s)assert!\(\s*!super::super::release::next_positive_guard_transition_fits_for_test\(' -Description "CP330 direct next-transition overflow predicate regression"
Assert-Contains -Path $cp330ReleaseCorruptionTests -Pattern 'public_only_skipped_route_redistribution_fails_completed_and_pending_links_without_mutation' -Description "CP330 public retained-route redistribution regression"
Assert-Contains -Path $cp330ReleaseCorruptionTests -Pattern 'coordinated_public_cp330_and_cp329_corruption_cannot_bypass_cp328_chain' -Description "CP330 recursive CP328-chain corruption regression"

Assert-Contains -Path $cp330InitState -Pattern '(?s)cooling_supply_mass_flow_positive_guard_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot' -Description "runtime-root private CP330 witness map"
Assert-NotContains -Path $cp330InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_supply_mass_flow_positive_guard_latest_witnesses:' -Description "public runtime-root CP330 witness map"
Assert-Contains -Path $cp330InitWitnessRoot -Pattern 'mod cooling_supply_mass_flow_positive_guard;' -Description "runtime-root CP330 witness module"
Assert-Contains -Path $cp330InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_supply_mass_flow_positive_guard_latest_witness\s*\(' -Description "runtime-root CP330 witness getter"
Assert-Contains -Path $cp330InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_supply_mass_flow_positive_guard_latest_witness\s*\(' -Description "runtime-root CP330 witness setter"
Assert-Contains -Path $cp330InitState -Pattern 'pub calc_cooling_supply_mass_flow_positive_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState' -Description "per-unit CP330 persistent state"
Assert-Contains -Path $cp330InitUnit -Pattern '(?s)calc_cooling_supply_mass_flow_positive_guard:\s*PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new\(system\)' -Description "per-unit CP330 state initialization"

# Binding order is CP329 -> CP330 -> CP331 -> CP332 -> CP333 -> CP334 ->
# CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> unchanged numerical DTO.
$cp330BindingText = Read-RepoText -Path $cp330Binding
$cp329BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_mixed_air_call =")
$cp330BindingIndex = $cp330BindingText.IndexOf("let calculation_cooling_supply_mass_flow_positive_guard =")
$cp331BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_cp_air_assignment =")
$cp332BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_assignment =")
$cp333BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_minimum_limit =")
$cp334BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_mixed_air_limit =")
$cp335BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp330 = $cp330BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp330 = $cp330BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp329BindingIndexForCp330 -lt 0 -or
    $cp330BindingIndex -le $cp329BindingIndexForCp330 -or
    $cp331BindingIndexForCp330 -le $cp330BindingIndex -or
    $cp332BindingIndexForCp330 -le $cp331BindingIndexForCp330 -or
    $cp333BindingIndexForCp330 -le $cp332BindingIndexForCp330 -or
    $cp334BindingIndexForCp330 -le $cp333BindingIndexForCp330 -or
    $cp335BindingIndexForCp330 -le $cp334BindingIndexForCp330 -or
    $cp336BindingIndexForCp330 -le $cp335BindingIndexForCp330 -or
    $cp337BindingIndexForCp330 -le $cp336BindingIndexForCp330 -or
    $cp338BindingIndexForCp330 -le $cp337BindingIndexForCp330 -or
    $cp339BindingIndexForCp330 -le $cp338BindingIndexForCp330 -or
    $numericalBindingIndexForCp330 -le $cp339BindingIndexForCp330
) {
    throw "Binding must retain exact CP329 -> CP330 -> CP331 -> CP332 -> CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $cp330Binding -Pattern '(?s)let calculation_cooling_supply_mass_flow_positive_guard =\s*advance_positive_guard\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_mixed_air_call,\s*\)\?;' -Description "binding exact CP329-to-CP330 adapter call"
Assert-Contains -Path $cp330BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_guard\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingMixedAirCallSnapshot,\s*\)' -Description "CP330 binding adapter arguments"
Assert-Contains -Path $cp330BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard\(runtime, system, predecessor\)\s*\.map_err\(\s*DirectZonePurchasedAirScheduledCouplingError::CalculationCoolingSupplyMassFlowPositiveGuard,\s*\)' -Description "CP330 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp330BindingAdapter -Pattern 'complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|DirectZonePurchasedAirCouplingInput' -Description "numerical DTO dependency in CP330 binding adapter"
Assert-Contains -Path $cp330Binding -Pattern 'PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError as CoolingPositiveGuardError' -Description "CP330 scheduled binding error alias"
Assert-Contains -Path $cp330Binding -Pattern 'CalculationCoolingSupplyMassFlowPositiveGuard\(CoolingPositiveGuardError\)' -Description "CP330 scheduled binding error boundary"
Assert-Contains -Path $cp330ScheduledOutput -Pattern 'pub calculation_cooling_supply_mass_flow_positive_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot' -Description "CP330 scheduled output evidence"
Assert-Contains -Path $cp330BindingTestsRoot -Pattern '#\[path = "binding/cooling_supply_mass_flow_positive_guard_tests\.rs"\]' -Description "CP330 binding test module path"
Assert-Contains -Path $cp330BindingTests -Pattern 'scheduled_binding_enters_cp330_body_only_for_strictly_positive_supply' -Description "CP330 active binding regression"
Assert-Contains -Path $cp330BindingTests -Pattern 'scheduled_binding_skips_every_cp330_site_when_cooling_is_inactive' -Description "CP330 skipped binding regression"
$cp330BindingCall = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_positive_guard =\s*advance_positive_guard\([^;]+?\)\?;'
)
if (-not $cp330BindingCall.Success) {
    throw "Binding must retain the complete CP330 exact release call"
}
$cp330BindingCallEnd = $cp330BindingCall.Index + $cp330BindingCall.Length
if ($cp331BindingIndexForCp330 -lt $cp330BindingCallEnd) {
    throw "CP330 exact release call must complete before CP331"
}
$postCp330BeforeCp331 = $cp330BindingText.Substring(
    $cp330BindingCallEnd,
    $cp331BindingIndexForCp330 - $cp330BindingCallEnd
)
$postCp330BeforeCp331Code = [regex]::Replace($postCp330BeforeCp331, '(?m)//.*$', '')
if ($postCp330BeforeCp331Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No source helper call other than CP331 may execute after CP330 and before CP331"
}
$cp331BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_cp_air_assignment =\s*advance_positive_supply_cp_air_assignment\([^;]+?\)\?;'
)
if (-not $cp331BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP331 exact release call after CP330"
}
$cp331BindingCallEndForCp330 =
    $cp331BindingCallForCp330.Index + $cp331BindingCallForCp330.Length
if ($cp332BindingIndexForCp330 -lt $cp331BindingCallEndForCp330) {
    throw "CP331 exact release call must complete before CP332"
}
$postCp331BeforeCp332ForCp330 = $cp330BindingText.Substring(
    $cp331BindingCallEndForCp330,
    $cp332BindingIndexForCp330 - $cp331BindingCallEndForCp330
)
$postCp331BeforeCp332CodeForCp330 =
    [regex]::Replace($postCp331BeforeCp332ForCp330, '(?m)//.*$', '')
if ($postCp331BeforeCp332CodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP331 and before CP332"
}
$cp332BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_assignment =\s*advance_positive_supply_temperature_assignment\([^;]+?\)\?;'
)
if (-not $cp332BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP332 exact release call after CP331"
}
$cp332BindingCallEndForCp330 =
    $cp332BindingCallForCp330.Index + $cp332BindingCallForCp330.Length
$cp333BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_minimum_limit =\s*advance_positive_supply_temperature_minimum_limit\([^;]+?\)\?;'
)
if (-not $cp333BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP333 exact release call after CP332"
}
$cp333BindingCallEndForCp330 =
    $cp333BindingCallForCp330.Index + $cp333BindingCallForCp330.Length
$cp334BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
if (-not $cp334BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP334 exact release call after CP333"
}
$cp334BindingCallEndForCp330 =
    $cp334BindingCallForCp330.Index + $cp334BindingCallForCp330.Length
$cp335BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
if (-not $cp335BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP335 exact release call after CP334"
}
$cp335BindingCallEndForCp330 =
    $cp335BindingCallForCp330.Index + $cp335BindingCallForCp330.Length
$cp336BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
if (-not $cp336BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP336 exact release call after CP335"
}
$cp336BindingCallEndForCp330 =
    $cp336BindingCallForCp330.Index + $cp336BindingCallForCp330.Length
$cp337BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
if (-not $cp337BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP337 exact release call after CP336"
}
$cp337BindingCallEndForCp330 =
    $cp337BindingCallForCp330.Index + $cp337BindingCallForCp330.Length
$cp338BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
if (-not $cp338BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP338 exact release call after CP337"
}
$cp338BindingCallEndForCp330 =
    $cp338BindingCallForCp330.Index + $cp338BindingCallForCp330.Length
$cp339BindingCallForCp330 = [regex]::Match(
    $cp330BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (-not $cp339BindingCallForCp330.Success) {
    throw "Binding must retain the complete CP339 exact release call after CP338"
}
$cp339BindingCallEndForCp330 =
    $cp339BindingCallForCp330.Index + $cp339BindingCallForCp330.Length
if (
    $cp333BindingIndexForCp330 -lt $cp332BindingCallEndForCp330 -or
    $cp334BindingIndexForCp330 -lt $cp333BindingCallEndForCp330 -or
    $cp335BindingIndexForCp330 -lt $cp334BindingCallEndForCp330 -or
    $cp336BindingIndexForCp330 -lt $cp335BindingCallEndForCp330 -or
    $cp337BindingIndexForCp330 -lt $cp336BindingCallEndForCp330 -or
    $cp338BindingIndexForCp330 -lt $cp337BindingCallEndForCp330 -or
    $cp339BindingIndexForCp330 -lt $cp338BindingCallEndForCp330 -or
    $numericalBindingIndexForCp330 -lt $cp339BindingCallEndForCp330
) {
    throw "CP332, CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
$postCp332BeforeCp333ForCp330 = $cp330BindingText.Substring(
    $cp332BindingCallEndForCp330,
    $cp333BindingIndexForCp330 - $cp332BindingCallEndForCp330
)
$postCp332BeforeCp333CodeForCp330 =
    [regex]::Replace($postCp332BeforeCp333ForCp330, '(?m)//.*$', '')
if ($postCp332BeforeCp333CodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP332 and before CP333"
}
$postCp333BeforeCp334ForCp330 = $cp330BindingText.Substring(
    $cp333BindingCallEndForCp330,
    $cp334BindingIndexForCp330 - $cp333BindingCallEndForCp330
)
$postCp333BeforeCp334CodeForCp330 =
    [regex]::Replace($postCp333BeforeCp334ForCp330, '(?m)//.*$', '')
if ($postCp333BeforeCp334CodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP333 and before CP334"
}
$postCp334BeforeCp335ForCp330 = $cp330BindingText.Substring(
    $cp334BindingCallEndForCp330,
    $cp335BindingIndexForCp330 - $cp334BindingCallEndForCp330
)
$postCp334BeforeCp335CodeForCp330 =
    [regex]::Replace($postCp334BeforeCp335ForCp330, '(?m)//.*$', '')
if ($postCp334BeforeCp335CodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP334 and before CP335"
}
$postCp335BeforeCp336ForCp330 = $cp330BindingText.Substring(
    $cp335BindingCallEndForCp330,
    $cp336BindingIndexForCp330 - $cp335BindingCallEndForCp330
)
$postCp335BeforeCp336CodeForCp330 =
    [regex]::Replace($postCp335BeforeCp336ForCp330, '(?m)//.*$', '')
if ($postCp335BeforeCp336CodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP335 and before CP336"
}
$postCp336BeforeCp337ForCp330 = $cp330BindingText.Substring(
    $cp336BindingCallEndForCp330,
    $cp337BindingIndexForCp330 - $cp336BindingCallEndForCp330
)
$postCp336BeforeCp337CodeForCp330 =
    [regex]::Replace($postCp336BeforeCp337ForCp330, '(?m)//.*$', '')
if ($postCp336BeforeCp337CodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP336 and before CP337"
}
$postCp337BeforeCp338ForCp330 = $cp330BindingText.Substring(
    $cp337BindingCallEndForCp330,
    $cp338BindingIndexForCp330 - $cp337BindingCallEndForCp330
)
$postCp337BeforeCp338CodeForCp330 =
    [regex]::Replace($postCp337BeforeCp338ForCp330, '(?m)//.*$', '')
if ($postCp337BeforeCp338CodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP337 and before CP338"
}
$postCp338BeforeCp339ForCp330 = $cp330BindingText.Substring(
    $cp338BindingCallEndForCp330,
    $cp339BindingIndexForCp330 - $cp338BindingCallEndForCp330
)
$postCp338BeforeCp339CodeForCp330 =
    [regex]::Replace($postCp338BeforeCp339ForCp330, '(?m)//.*$', '')
if ($postCp338BeforeCp339CodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP338 and before CP339"
}
$postCp339BeforeNumericalForCp330 = $cp330BindingText.Substring(
    $cp339BindingCallEndForCp330,
    $numericalBindingIndexForCp330 - $cp339BindingCallEndForCp330
)
$postCp339BeforeNumericalCodeForCp330 =
    [regex]::Replace($postCp339BeforeNumericalForCp330, '(?m)//.*$', '')
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\([^;]+?\)\?;)',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\([^;]+?\)\?;)',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry =\s*advance_cooling_supply_mass_flow_positive_guard_else_branch_entry\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment =\s*advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment =\s*advance_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment\([^;]+?\)\?;|let calculation_heating_or_no_load_case_entry =\s*advance_heating_or_no_load_case_entry\([^;]+?\)\?;|let calculation_heating_mode_guard =\s*advance_heating_mode_guard\([^;]+?\)\?;|let calculation_heating_operating_mode_heat_assignment =\s*advance_heating_operating_mode_heat_assignment\([^;]+?\)\?;)',
    ''
)
$postCp339BeforeNumericalCodeForCp330 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp330,
    '(?s)(?:let calculation_heating_mode_guard_else_branch_entry =\s*advance_heating_mode_guard_else_branch_entry\([^;]+?\)\?;|let calculation_heating_operating_mode_deadband_assignment =\s*advance_heating_operating_mode_deadband_assignment\([^;]+?\)\?;|let calculation_heating_outdoor_air_maximum_flow_guard =\s*advance_heating_outdoor_air_maximum_flow_guard\([^;]+?\)\?;|let calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment =\s*advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment\([^;]+?\)\?;|let calculation_heating_outdoor_air_maximum_flow_first_warning_guard =\s*advance_heating_outdoor_air_maximum_flow_first_warning_guard\([^;]+?\)\?;|let calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment =\s*advance_heating_outdoor_air_maximum_flow_first_warning_counter_increment\([^;]+?\)\?;|let calculation_heating_outdoor_air_maximum_flow_first_warning_call =\s*advance_heating_outdoor_air_maximum_flow_first_warning_call\([^;]+?\)\?;)',
    ''
)
if ($postCp339BeforeNumericalCodeForCp330 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No helper other than the audited CP340 through CP439 releases may execute after CP339 and before numerical Calc"
}

# Coupled validation reconstructs CP330 only from CP329 and retains exact bits.
Assert-Contains -Path $cp330CoupledRuntime -Pattern 'mod cooling_supply_mass_flow_positive_guard_validation;' -Description "coupled CP330 validator declaration"
Assert-Contains -Path $cp330CoupledRuntime -Pattern 'pub calc_cooling_supply_mass_flow_positive_guard_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary' -Description "coupled CP330 lifecycle"
Assert-Contains -Path $cp330CoupledRuntime -Pattern 'cooling_supply_mass_flow_positive_guard_validation::snapshot_matches_release' -Description "coupled per-timestep CP330 validation"
Assert-Contains -Path $cp330CoupledRuntime -Pattern 'cooling_supply_mass_flow_positive_guard_validation::validate_lifecycle' -Description "coupled final CP330 validation"
Assert-Contains -Path $cp330CoupledValidation -Pattern 'let predecessor = output\.calculation_cooling_mixed_air_call;' -Description "coupled CP329 predecessor"
Assert-Contains -Path $cp330CoupledValidation -Pattern 'let snapshot = output\.calculation_cooling_supply_mass_flow_positive_guard;' -Description "coupled CP330 output"
Assert-Contains -Path $cp330CoupledValidation -Pattern 'fn source_strictly_positive\(value: f64\) -> bool \{\s*value > 0\.0\s*\}' -Description "coupled raw source comparator"
Assert-Contains -Path $cp330CoupledValidation -Pattern '(?s)cooling_body_entry_count,\s*2,.*?positive_supply_mass_flow_body_entry_count' -Description "coupled CP330 dynamic site count"
Assert-Contains -Path $cp330CoupledValidation -Pattern 'options_have_exact_bits' -Description "coupled CP330 exact-bit validation"
Assert-Contains -Path $cp330CoupledValidation -Pattern 'source_comparison_preserves_nan_signed_zero_and_infinity_semantics' -Description "coupled CP330 IEEE regression"
Assert-Contains -Path $cp330CoupledValidation -Pattern 'snapshot_comparison_detects_signed_zero_bit_corruption' -Description "coupled CP330 signed-zero corruption"
Assert-Contains -Path $cp330CoupledTests -Pattern 'cooling_supply_mass_flow_positive_guard_partition_overflow_fails_closed' -Description "coupled CP330 overflow regression"
$cp330CoupledRuntimeText = (Read-RepoText -Path $cp330CoupledValidation).Split("#[cfg(test)]")[0]
if ($cp330CoupledRuntimeText -match 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|complete_direct_zone_purchased_air_coupling') {
    throw "Coupled CP330 runtime validation must not reconcile a later numerical DTO"
}
Assert-Contains -Path $cp330CoupledFixture -Pattern 'calculation_cooling_supply_mass_flow_positive_guard_snapshot\s*\(' -Description "coupled CP330 fixture"
Assert-Contains -Path $cp330CoupledFixture -Pattern '\.map\(\|value\| value > 0\.0\)' -Description "coupled fixture CP330 source comparison"

# Pipeline evidence is direct-only, source-ordered, and bit-exact.
Assert-Contains -Path $cp330PipelineRoot -Pattern 'mod purchased_air_cooling_supply_mass_flow_positive_guard;' -Description "pipeline CP330 module declaration"
Assert-Contains -Path $cp330PipelineRoot -Pattern '"purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle"' -Description "pipeline CP330 lifecycle JSON key"
Assert-Contains -Path $cp330PipelineRoot -Pattern 'purchased_air_cooling_supply_mass_flow_positive_guard::validate_direct_lifecycle' -Description "pipeline CP330 direct firewall"
Assert-Contains -Path $cp330Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER' -Description "pipeline CP329-to-CP330 lineage"
Assert-Contains -Path $cp330Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER' -Description "pipeline CP330 source order"
Assert-Contains -Path $cp330PipelineValidation -Pattern '(?s)checked_product\(cooling, 2,.*?positive_supply_mass_flow_body_entry_count' -Description "pipeline CP330 dynamic source-site formula"
Assert-Contains -Path $cp330PipelineValidation -Pattern 'let positive = supply > 0\.0;' -Description "pipeline CP330 source comparison"
Assert-Contains -Path $cp330PipelineValidation -Pattern 'option_has_bits\(predecessor_supply, supply\)' -Description "pipeline CP329 retained bits"
Assert-NotContains -Path $cp330Pipeline -Pattern 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow' -Description "final numerical DTO reconciliation in CP330 pipeline"
Assert-Contains -Path $cp330PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP330 latest serialization"
Assert-Contains -Path $cp330PipelineSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP330 source-order JSON"
Assert-Contains -Path $cp330PipelineSnapshotSerialization -Pattern '"supply_mass_flow_rate_kg_per_s_ieee_bits"' -Description "pipeline CP330 IEEE supply bits"
Assert-Contains -Path $cp330PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP330 exact IEEE serialization"
Assert-Contains -Path $cp330Pipeline -Pattern 'json_preserves_signed_zero_infinity_and_nan_supply_bits' -Description "pipeline CP330 IEEE JSON regression"
Assert-Contains -Path $cp330RunTests -Pattern 'mod cooling_supply_mass_flow_positive_guard_assertions;' -Description "direct integration CP330 assertion module"
Assert-Contains -Path $cp330RunTests -Pattern 'assert_cooling_supply_mass_flow_positive_guard\(' -Description "direct integration CP330 assertion calls"
Assert-Contains -Path $cp330DirectAssertions -Pattern 'expected_cooling_entries \* 2 \+ positive_entries' -Description "direct integration CP330 dynamic site count"
Assert-Contains -Path $cp330DirectAssertions -Pattern 'purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle' -Description "direct integration CP330 lifecycle key"
Assert-Contains -Path $cp330DirectAssertions -Pattern 'f64_from_ieee_bits' -Description "direct integration CP330 raw IEEE reconstruction"
Assert-Contains -Path $cp330NonDirectTests -Pattern 'purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle' -Description "non-direct CP330 null-evidence boundary"

# Specs and generated documents retain two parent addenda without promotion.
$cp330AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp330AlgorithmAddenda = [regex]::Matches(
    $cp330AlgorithmText,
    '(?m)^\s*"CP330 supersedes only CP329[^"\r\n]+",\s*$'
)
if ($cp330AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP330 support addenda"
}
foreach ($cp330AlgorithmAddendum in $cp330AlgorithmAddenda) {
    $cp330Text = $cp330AlgorithmAddendum.Value
    foreach ($cp330Pattern in @(
            'line-2183',
            'exactly three lexical sites',
            '2 \* active \+ positive_body_entries',
            'signed zeros',
            'positive infinity',
            'NaN false',
            'CP329-to-CP330-to-numerical',
            'line 2185 is the first excluded lexical executable',
            'line 2340',
            'line 2465',
            'both parents remain `scaffold`/`none`',
            'Roadmap state stay unchanged'
        )) {
        if ($cp330Text -notmatch $cp330Pattern) {
            throw "CP330 algorithm addendum missing '$cp330Pattern'"
        }
    }
}
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_positive_guard/release\.rs::advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard' -Description "CP330 algorithm wrapper target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_positive_guard\.rs::purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary' -Description "CP330 algorithm lifecycle target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_positive_guard\.rs::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState' -Description "CP330 routine state target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_positive_guard\.rs::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary' -Description "CP330 routine lifecycle target"

$cp330CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp330CapabilityAddenda = [regex]::Matches(
    $cp330CapabilityText,
    '(?m)^\s*"CP330 additionally requires[^"\r\n]+",\s*$'
)
if ($cp330CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP330 claim addenda"
}
foreach ($cp330CapabilityAddendum in $cp330CapabilityAddenda) {
    $cp330Text = $cp330CapabilityAddendum.Value
    foreach ($cp330Pattern in @(
            'line 2183',
            'three-site',
            '2 \* active \+ positive_body_entries',
            'signed zeros',
            'positive infinity',
            'NaN false',
            'CP329-to-CP330-to-numerical',
            'line 2185 is the first excluded lexical executable',
            '2340',
            '2465',
            'This changes no support level',
            'both Calc routines remain `source_mapped`'
        )) {
        if ($cp330Text -notmatch $cp330Pattern) {
            throw "CP330 capability addendum missing '$cp330Pattern'"
        }
    }
}
$cp330HardeningAlgorithmAddenda = [regex]::Matches(
    $cp330AlgorithmText,
    '(?m)^\s*"CP329/CP330 fail-closed retained-chain hardening[^"\r\n]+",\s*$'
)
if ($cp330HardeningAlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP329/CP330 retained-chain hardening addenda"
}
foreach ($cp330HardeningAddendum in $cp330HardeningAlgorithmAddenda) {
    foreach ($cp330HardeningPattern in @(
            'runtime-aware completed helper',
            'recursively validates completed CP328',
            'pending and completed CP329',
            'route-aware checked-arithmetic preflight',
            'CP330 pending and completed validation',
            'coordinated CP329/CP330',
            'transactional',
            'private witnesses unchanged',
            'no numerical DTO consumption',
            'Roadmap state'
        )) {
        if ($cp330HardeningAddendum.Value -notmatch $cp330HardeningPattern) {
            throw "CP329/CP330 algorithm hardening addendum missing '$cp330HardeningPattern'"
        }
    }
}
$cp330HardeningCapabilityAddenda = [regex]::Matches(
    $cp330CapabilityText,
    '(?m)^\s*"CP329/CP330 fail-closed retained-chain hardening additionally requires[^"\r\n]+",\s*$'
)
if ($cp330HardeningCapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP329/CP330 retained-chain hardening addenda"
}
foreach ($cp330HardeningAddendum in $cp330HardeningCapabilityAddenda) {
    foreach ($cp330HardeningPattern in @(
            'full retained predecessor chain',
            'runtime-aware completed proof',
            'completed and pending CP329',
            'route-aware checked-arithmetic preflight',
            'CP330 completed and pending history',
            'coordinated CP329/CP330',
            'private witnesses unchanged',
            'no numerical DTO consumption',
            'Roadmap state'
        )) {
        if ($cp330HardeningAddendum.Value -notmatch $cp330HardeningPattern) {
            throw "CP329/CP330 capability hardening addendum missing '$cp330HardeningPattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP330 supersedes only CP329' -Description "generated CP330 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP330 additionally requires' -Description "generated CP330 capability index"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP329/CP330 fail-closed retained-chain hardening' -Description "generated CP329/CP330 algorithm hardening"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP329/CP330 fail-closed retained-chain hardening' -Description "generated CP329/CP330 capability hardening"

# Every hand-authored contract repeats the exact guard, dynamic count, raw IEEE
# partition, direct CP329 lineage, exclusions, and non-promotion terms.
$cp330DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP330 maps only the complete Cooling positive-supply guard.*?^and Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP330 Source-Ordered Cooling Positive-Supply Guard\r?\n.*?Roadmap item\.\s*'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP330 Cooling Positive-Supply Guard\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP330 Cooling Positive-Supply Guard in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP330 Cooling Positive-Supply Guard Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp330Documentation in $cp330DocumentationSections) {
    $cp330DocumentText = Read-RepoText -Path $cp330Documentation.Path
    $cp330Matches = [regex]::Matches($cp330DocumentText, $cp330Documentation.Pattern)
    if ($cp330Matches.Count -ne 1) {
        throw "CP330 documentation expected one scoped section in $($cp330Documentation.Path), found $($cp330Matches.Count)"
    }
    $cp330Section = $cp330Matches[0].Value
    foreach ($cp330Pattern in @(
            'line 2183|line-2183',
            '(?:exactly )?three(?:-site|\s+lexical)',
            '> \+0\.0',
            '2 \* active \+ positive_body_entries',
            'C\+\+',
            'evaluation[- ]order',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?is)positive\s+infinity',
            '(?is)negative\s+infinity',
            '(?i)NaN|unordered',
            '(?is)positive zero.{0,100}negative zero|signed zeros',
            '(?is)CP329.{0,200}(?:bit-exact|retained|supply)',
            'purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle',
            '(?i)runtime-aware',
            '(?is)completed.{0,160}pending|pending.{0,160}completed',
            '(?is)UnitOff.{0,100}non-cooling.{0,100}(?:active|route)',
            '(?is)coordinated.{0,160}CP329.{0,80}CP330|CP329/CP330.{0,160}corruption',
            '(?is)(?:fail|rejected).{0,180}(?:unchanged|before.{0,100}(?:transition|mutat|witness)|without.{0,100}(?:changing|mutat))',
            'line 2185',
            '2340',
            '2454-2461',
            '2465',
            '(?i)source_mapped',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp330Section -notmatch $cp330Pattern) {
            throw "CP330 documentation in $($cp330Documentation.Path) missing '$cp330Pattern'"
        }
    }
}

# Main audit and generated script inventory remain in source-checkpoint order.
$cp330MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp329DotSourceIndexForCp330 = $cp330MainAuditText.IndexOf('ideal-loads-structure-audit\cp329-cooling-mixed-air-call.ps1')
$cp330DotSourceIndex = $cp330MainAuditText.IndexOf('ideal-loads-structure-audit\cp330-cooling-supply-mass-flow-positive-guard.ps1')
$cp330AuditCompletionIndex = $cp330MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp329DotSourceIndexForCp330 -lt 0 -or
    $cp330DotSourceIndex -le $cp329DotSourceIndexForCp330 -or
    $cp330AuditCompletionIndex -le $cp330DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP330 after CP329 and before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp330-cooling-supply-mass-flow-positive-guard\.ps1"' -Description "CP330 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp330-cooling-supply-mass-flow-positive-guard\.ps1::dot_sources' -Description "CP330 main-audit callee evidence"
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'advance_heating_mode_guard_else_branch_entry' -Description 'CP433 helper whitelist'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'advance_heating_operating_mode_deadband_assignment' -Description 'audited CP340 through CP439 helper whitelist'
