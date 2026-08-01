# CP331 maps only PurchasedAirManager.cc executable line 2185: the complete
# Cooling positive-supply CpAir assignment. Line 2186 is the first excluded
# lexical executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp331Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment.rs"
$cp331State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment\state.rs"
$cp331Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment\transition.rs"
$cp331Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment\release.rs"
$cp331PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment\release\prefix_validation.rs"
$cp331RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment\release\runtime_validation.rs"
$cp331SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment\release\snapshot_validation.rs"
$cp331Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment\tests\mod.rs"
$cp331ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_cp_air_assignment\tests\release_corruption.rs"
$cp331CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp331Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp331Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp331ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp331BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_cp_air_assignment.rs"
$cp331BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp331BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_cp_air_assignment_tests.rs"
$cp331InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp331InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp331InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp331InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_cp_air_assignment.rs"
$cp331CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp331CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_cp_air_assignment_validation.rs"
$cp331CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp331CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_cp_air_assignment_fixture.rs"
$cp331CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp331PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp331Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_cp_air_assignment.rs"
$cp331PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_cp_air_assignment\validation.rs"
$cp331PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_cp_air_assignment\serialization.rs"
$cp331PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_cp_air_assignment\serialization\snapshot.rs"
$cp331RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp331DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_cp_air_assignment_assertions.rs"
$cp331NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp331RequiredFile in @(
        $cp331Module,
        $cp331State,
        $cp331Transition,
        $cp331Release,
        $cp331PrefixValidation,
        $cp331RuntimeValidation,
        $cp331SnapshotValidation,
        $cp331Tests,
        $cp331ReleaseCorruptionTests,
        $cp331ScheduledOutput,
        $cp331BindingAdapter,
        $cp331BindingTests,
        $cp331InitWitness,
        $cp331CoupledValidation,
        $cp331CoupledFixture,
        $cp331Pipeline,
        $cp331PipelineValidation,
        $cp331PipelineSerialization,
        $cp331PipelineSnapshotSerialization,
        $cp331DirectAssertions
    )) {
    Assert-FileExists -Path $cp331RequiredFile -Description "CP331 CpAir-assignment structure"
}
Assert-LineLimit -Path $cp331Release -Limit 800 -Description "CP331 release root module"
Assert-LineLimit -Path $cp331RuntimeValidation -Limit 800 -Description "CP331 runtime validation module"
Assert-LineLimit -Path $cp331CoupledValidation -Limit 800 -Description "CP331 coupled validation module"
Assert-LineLimit -Path $cp331Pipeline -Limit 800 -Description "CP331 pipeline module"

# Exact statement boundary and lexical inventory.
Assert-Contains -Path $cp331Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2185' -Description "CP331 exact source boundary"
Assert-Contains -Path $cp331Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2186' -Description "CP331 first excluded lexical executable"
Assert-Contains -Path $cp331Module -Pattern 'Exact three textual source sites represented by CP331' -Description "CP331 exact lexical-site count"
Assert-ExactStringArray -Path $cp331Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-controlled-zone-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air"
) -Description "CP331 exact three lexical source sites"
Assert-Contains -Path $cp331Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot' -Description "CP331 public snapshot"
Assert-Contains -Path $cp331State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState' -Description "CP331 persistent state"
Assert-Contains -Path $cp331Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary' -Description "CP331 lifecycle summary"
Assert-Contains -Path $cp331Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary\s*\(' -Description "CP331 lifecycle accessor"
Assert-Contains -Path $cp331Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment\s*\(' -Description "CP331 exact-direct wrapper"
Assert-Contains -Path $cp331Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_cp_air_assignment_state\s*\(' -Description "CP331 pure transition"
Assert-Contains -Path $cp331CalcRoot -Pattern 'mod cooling_positive_supply_cp_air_assignment;' -Description "CP331 calc module declaration"
Assert-Contains -Path $cp331CalcRoot -Pattern 'pub use (?:cooling_positive_supply_cp_air_assignment::\*;|\{[^}]*cooling_positive_supply_cp_air_assignment::\*)' -Description "CP331 calc public surface"

# Only a CP330 positive-body route evaluates the live operand, canonical scalar,
# and direct local assignment. All three skipped routes execute zero sites.
Assert-PatternsInOrder -Path $cp331Transition -Patterns @(
    'let assignment_executed = predecessor\.positive_supply_mass_flow_body_entered;',
    'debug_assert_eq!\(assignment_executed, active_input\.is_some\(\)\);',
    'let zone_humidity_ratio = active_input\.map\(\|input\| input\.zone_humidity_ratio\);',
    'zone_humidity_ratio\.map\(energyplus_psy_cp_air_fn_w\);',
    'let cp_air_j_per_kg_k = psychrometric_cp_air_result_j_per_kg_k;',
    'if predecessor\.unit_off_skipped',
    'state\.unit_off_skip_count \+= 1;',
    'else if predecessor\.non_cooling_skipped',
    'state\.non_cooling_skip_count \+= 1;',
    'else if predecessor\.active_guard_false_fallthrough',
    'state\.positive_guard_false_fallthrough_skip_count \+= 1;',
    'state\.witnessed_positive_guard_false_fallthrough_skip_count \+= 1;',
    'state\.cp_air_assignment_count \+= 1;',
    'state\.source_site_execution_count \+=',
    'PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER\.len\(\);',
    'state\.zone_humidity_ratio_read_count \+= 1;',
    'state\.psychrometric_cp_air_evaluation_count \+= 1;',
    'state\.cp_air_assignment_write_count \+= 1;',
    'state\.witnessed_cp_air_assignment_count \+= 1;'
) -Description "CP331 conditional source-order transition"
Assert-NotContains -Path $cp331Transition -Pattern 'energyplus_moist_air_specific_heat_j_per_kg_k' -Description "legacy NaN-normalizing CpAir helper in CP331"
Assert-NotContains -Path $cp331Transition -Pattern 'dwSave|cpaSave|-100\.0|static|Mutex|OnceLock|thread_local' -Description "source cache lifecycle in CP331 transition"
Assert-NotContains -Path $cp331Transition -Pattern 'supply_air_temperature|minimum_cooling_supply|mixed_air_temperature|capacity_control|enthalpy_assignment|2454|2465' -Description "CP331 line-2186-or-later source behavior"
Assert-Contains -Path $cp331Tests -Pattern 'positive_route_executes_exact_three_sites_and_assigns_canonical_cp_air' -Description "CP331 positive-route regression"
Assert-Contains -Path $cp331Tests -Pattern 'canonical_scalar_characterization_preserves_raw_humidity_classes' -Description "CP331 stateless scalar characterization"
Assert-Contains -Path $cp331Tests -Pattern 'skipped_routes_do_not_validate_zone_humidity' -Description "CP331 skipped-route regression"
Assert-Contains -Path $cp331Tests -Pattern 'counters_partition_all_four_routes_and_count_three_sites_per_assignment' -Description "CP331 route and dynamic-site regression"
Assert-Contains -Path $cp331Tests -Pattern 'bit_exact_matcher_rejects_signed_zero_and_result_corruption' -Description "CP331 bit-exact corruption regression"

# Exact release snapshots admit only finite, nonnegative humidity; recompute the
# canonical scalar and require result-to-assignment bit identity.
Assert-Contains -Path $cp331SnapshotValidation -Pattern 'let expected = energyplus_psy_cp_air_fn_w\(humidity_ratio\);' -Description "CP331 canonical snapshot result"
Assert-PatternsInOrder -Path $cp331SnapshotValidation -Patterns @(
    'snapshot\.zone_humidity_ratio_read',
    'humidity_ratio\.is_finite\(\)',
    'humidity_ratio >= 0\.0',
    'snapshot\.psychrometric_cp_air_evaluated',
    'result\.is_finite\(\)',
    'result\.to_bits\(\) == expected\.to_bits\(\)',
    'snapshot\.cp_air_assigned',
    'assigned\.to_bits\(\) == result\.to_bits\(\)'
) -Description "CP331 admitted physical-domain snapshot"
Assert-Contains -Path $cp331SnapshotValidation -Pattern 'fn skipped_snapshot_is_exact' -Description "CP331 exact skipped snapshot"
Assert-Contains -Path $cp331SnapshotValidation -Pattern 'fn snapshots_match_bit_exact' -Description "CP331 exact retained snapshot matcher"

# Persistent counters mirror all CP330 routes, derive dynamic sites as
# 3*positive-body entries, and preflight every route-specific increment.
Assert-Contains -Path $cp331RuntimeValidation -Pattern 'state\.cp_air_assignment_count\.checked_mul\(3\)' -Description "CP331 dynamic source-site formula"
foreach ($cp331Counter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "cp_air_assignment_count",
        "source_site_execution_count",
        "zone_humidity_ratio_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_write_count"
    )) {
    Assert-Contains -Path $cp331RuntimeValidation -Pattern $cp331Counter -Description "CP331 runtime counter '$cp331Counter'"
}
Assert-Contains -Path $cp331State -Pattern 'pub\(super\) witnessed_positive_guard_false_fallthrough_skip_count:\s*usize' -Description "CP331 private guard-false witness count"
Assert-Contains -Path $cp331State -Pattern 'pub\(super\) witnessed_cp_air_assignment_count:\s*usize' -Description "CP331 private assignment witness count"
Assert-Contains -Path $cp331RuntimeValidation -Pattern '(?s)pending_cp_air_assignment_state_is_consistent\(.*?unit_off_skip_count.*?predecessor_state\.unit_off_skip_count.*?non_cooling_skip_count.*?predecessor_state\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor_state\.active_guard_false_fallthrough_count.*?cp_air_assignment_count.*?predecessor_state\.positive_supply_mass_flow_body_entry_count' -Description "CP331 pending CP330 route-history parity"
Assert-Contains -Path $cp331RuntimeValidation -Pattern '(?s)completed_cp_air_assignment_state_is_consistent\(.*?state\.transition_count == predecessor\.transition_count.*?state\.unit_off_skip_count == predecessor\.unit_off_skip_count.*?state\.non_cooling_skip_count == predecessor\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor\.active_guard_false_fallthrough_count.*?cp_air_assignment_count.*?predecessor\.positive_supply_mass_flow_body_entry_count' -Description "CP331 completed CP330 route-history parity"
foreach ($cp331PreflightCounter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "witnessed_positive_guard_false_fallthrough_skip_count",
        "cp_air_assignment_count",
        "source_site_execution_count",
        "zone_humidity_ratio_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_write_count",
        "witnessed_cp_air_assignment_count"
    )) {
    Assert-Contains -Path $cp331RuntimeValidation -Pattern ($cp331PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP331 checked preflight counter '$cp331PreflightCounter'"
}

# The public wrapper owns the live Zone read. CP329's two no-OA copies prove
# same-call lineage but are never substituted for the source operand.
Assert-Contains -Path $cp331Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp330: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,\s*zone_state: &ZoneHeatBalanceState,' -Description "CP331 exact wrapper arguments"
Assert-Contains -Path $cp331Release -Pattern 'let zone_humidity_ratio = zone_state\.air_humidity_ratio;' -Description "CP331 live controlled-Zone humidity read"
Assert-Contains -Path $cp331Release -Pattern 'mixed_air_humidity_matches_zone_bits\(mixed_air, zone_humidity_ratio\)' -Description "CP331 CP329 same-call humidity lineage"
Assert-PatternsInOrder -Path $cp331Release -Patterns @(
    'mixed_air\.recirculation_humidity_ratio',
    'mixed_air\.mixed_air_humidity_ratio',
    'value\.to_bits\(\) == zone_humidity_ratio\.to_bits\(\)'
) -Description "CP331 CP329 recirculation and mixed-air bit evidence"
Assert-Contains -Path $cp331PrefixValidation -Pattern '(?s)mixed_air\.recirculation_humidity_ratio,.*?mixed_air\.mixed_air_humidity_ratio' -Description "CP331 retained CP329 humidity linkage"
Assert-Contains -Path $cp331Release -Pattern 'energyplus_psy_cp_air_fn_w\(zone_humidity_ratio\)\.is_finite\(\)' -Description "CP331 canonical scalar admission"
Assert-Contains -Path $cp331Release -Pattern 'completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent\s*\(' -Description "CP331 recursive CP330 completed proof"
Assert-Contains -Path $cp331Release -Pattern 'cooling_supply_mass_flow_positive_guard_latest_witness\s*\(' -Description "CP331 retained CP330 private witness"
Assert-Contains -Path $cp331Release -Pattern 'cooling_positive_supply_cp_air_assignment_latest_witness\s*\(' -Description "CP331 private assignment witness"
Assert-Contains -Path $cp331Release -Pattern '(?s)completed_direct_cooling_positive_supply_cp_air_assignment_is_consistent\(.*?completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent\(.*?cp_air_assignment_links_to_positive_guard\(snapshot, predecessor\).*?cp_air_assignment_humidity_links_to_mixed_air\(snapshot, mixed_air\).*?completed_cp_air_assignment_state_is_consistent' -Description "CP331 recursive completed chain and lineage"
Assert-NotContains -Path $cp331Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment\([^)]*humidity_ratio\s*:\s*f64' -Description "duplicate caller humidity scalar in CP331 release"
Assert-NotContains -Path $cp331Release -Pattern 'energyplus_moist_air_specific_heat_j_per_kg_k|latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|complete_direct_zone_purchased_air_coupling' -Description "legacy helper or numerical DTO dependency in CP331 release"

$cp331ReleaseText = Read-RepoText -Path $cp331Release
$cp331ReleaseWrapper = [regex]::Match(
    $cp331ReleaseText,
    '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment\(.*?(?=\r?\nfn mixed_air_humidity_matches_zone_bits\()'
)
if (-not $cp331ReleaseWrapper.Success) {
    throw "CP331 exact release wrapper must remain structurally bounded"
}
$cp331WrapperText = $cp331ReleaseWrapper.Value
$cp331MutationIndex = $cp331WrapperText.IndexOf("let snapshot = {")
if ($cp331MutationIndex -lt 0) {
    throw "CP331 release mutation boundary must remain explicit"
}
foreach ($cp331ValidationCall in @(
        "pending_cp_air_assignment_state_is_consistent(",
        "next_cp_air_assignment_transition_fits(",
        "completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(",
        "mixed_air_humidity_matches_zone_bits(",
        "energyplus_psy_cp_air_fn_w("
    )) {
    $cp331ValidationIndex = $cp331WrapperText.IndexOf($cp331ValidationCall)
    if ($cp331ValidationIndex -lt 0 -or $cp331ValidationIndex -ge $cp331MutationIndex) {
        throw "CP331 validation '$cp331ValidationCall' must complete before transition mutation"
    }
}
$cp331NextFitIndex = $cp331WrapperText.IndexOf("next_cp_air_assignment_transition_fits(")
$cp331RecursivePredecessorIndex =
    $cp331WrapperText.IndexOf("completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(")
if ($cp331RecursivePredecessorIndex -le $cp331NextFitIndex) {
    throw "CP331 checked-overflow preflight must precede recursive CP330 validation"
}
Assert-PatternsInOrder -Path $cp331Release -Patterns @(
    'let snapshot = \{',
    '\.get_mut\(&selected\)',
    'advance_cooling_positive_supply_cp_air_assignment_state\(',
    'set_cooling_positive_supply_cp_air_assignment_latest_witness\(selected, snapshot\)'
) -Description "CP331 validated transition and private-witness mutation order"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'public_release_commits_once_from_live_zone_humidity_and_rejects_replay' -Description "CP331 release commit/replay regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'private_latest_or_witness_corruption_is_fail_closed_and_transactional' -Description "CP331 private latest/witness corruption regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'recursive_cp329_chain_corruption_is_fail_closed_and_transactional' -Description "CP331 recursive CP329-chain corruption regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'public_skipped_routes_do_not_validate_zone_humidity' -Description "CP331 public skipped-route no-validation regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'forged_cp330_snapshot_or_private_witness_fails_without_mutation' -Description "CP331 CP330/private-witness corruption regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'positive_route_rejects_zone_humidity_drift_negative_and_nonfinite_values' -Description "CP331 live humidity admission regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'wrong_zone_identity_fails_before_cp331_mutation' -Description "CP331 Zone identity regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'completed_cp330_counter_corruption_fails_before_cp331_mutation' -Description "CP331 recursive predecessor corruption regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'active_false_route_preflight_rejects_counter_overflow_transactionally' -Description "CP331 guard-false overflow preflight regression"
Assert-Contains -Path $cp331ReleaseCorruptionTests -Pattern 'assigned_route_preflight_rejects_each_counter_overflow_transactionally' -Description "CP331 assignment overflow preflight regression"

# Per-unit state and private latest-witness ownership are persistent.
Assert-Contains -Path $cp331InitState -Pattern '(?s)cooling_positive_supply_cp_air_assignment_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot' -Description "runtime-root private CP331 witness map"
Assert-NotContains -Path $cp331InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_cp_air_assignment_latest_witnesses:' -Description "public runtime-root CP331 witness map"
Assert-Contains -Path $cp331InitWitnessRoot -Pattern 'mod cooling_positive_supply_cp_air_assignment;' -Description "runtime-root CP331 witness module"
Assert-Contains -Path $cp331InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_cp_air_assignment_latest_witness\s*\(' -Description "runtime-root CP331 witness getter"
Assert-Contains -Path $cp331InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_cp_air_assignment_latest_witness\s*\(' -Description "runtime-root CP331 witness setter"
Assert-Contains -Path $cp331InitState -Pattern 'pub calc_cooling_positive_supply_cp_air_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState' -Description "per-unit CP331 persistent state"
Assert-Contains -Path $cp331InitUnit -Pattern '(?s)calc_cooling_positive_supply_cp_air_assignment:\s*PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new\(system\)' -Description "per-unit CP331 state initialization"

# Binding order is CP330 -> CP331 -> CP332 -> CP333 -> CP334 -> CP335 ->
# CP336 -> CP337 -> CP338 -> CP339 -> unchanged numerical DTO. CP331's adapter still
# passes the live Zone state without adding a duplicate scalar.
$cp331BindingText = Read-RepoText -Path $cp331Binding
$cp330BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_supply_mass_flow_positive_guard =")
$cp331BindingIndex = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_cp_air_assignment =")
$cp332BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_assignment =")
$cp333BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_minimum_limit =")
$cp334BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_mixed_air_limit =")
$cp335BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp331 = $cp331BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp331 = $cp331BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp330BindingIndexForCp331 -lt 0 -or
    $cp331BindingIndex -le $cp330BindingIndexForCp331 -or
    $cp332BindingIndexForCp331 -le $cp331BindingIndex -or
    $cp333BindingIndexForCp331 -le $cp332BindingIndexForCp331 -or
    $cp334BindingIndexForCp331 -le $cp333BindingIndexForCp331 -or
    $cp335BindingIndexForCp331 -le $cp334BindingIndexForCp331 -or
    $cp336BindingIndexForCp331 -le $cp335BindingIndexForCp331 -or
    $cp337BindingIndexForCp331 -le $cp336BindingIndexForCp331 -or
    $cp338BindingIndexForCp331 -le $cp337BindingIndexForCp331 -or
    $cp339BindingIndexForCp331 -le $cp338BindingIndexForCp331 -or
    $numericalBindingIndexForCp331 -le $cp339BindingIndexForCp331
) {
    throw "Binding must retain exact CP330 -> CP331 -> CP332 -> CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $cp331Binding -Pattern '(?s)let calculation_cooling_positive_supply_cp_air_assignment =\s*advance_positive_supply_cp_air_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_supply_mass_flow_positive_guard,\s*&\*input\.zone_state,\s*\)\?;' -Description "binding exact CP330-to-CP331 adapter call"
Assert-Contains -Path $cp331BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_cp_air_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,\s*zone_state: &ZoneHeatBalanceState,' -Description "CP331 binding adapter arguments"
Assert-Contains -Path $cp331BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment\(\s*runtime,\s*system,\s*predecessor,\s*zone_state,\s*\).*?CalculationCoolingPositiveSupplyCpAirAssignment' -Description "CP331 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp331BindingAdapter -Pattern 'complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|DirectZonePurchasedAirCouplingInput' -Description "numerical DTO dependency in CP331 binding adapter"
Assert-Contains -Path $cp331Binding -Pattern 'PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError as CoolingCpAirAssignmentError' -Description "CP331 scheduled binding error alias"
Assert-Contains -Path $cp331Binding -Pattern 'CalculationCoolingPositiveSupplyCpAirAssignment\(CoolingCpAirAssignmentError\)' -Description "CP331 scheduled binding error boundary"
Assert-Contains -Path $cp331ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_cp_air_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot' -Description "CP331 scheduled output evidence"
Assert-Contains -Path $cp331BindingTestsRoot -Pattern '#\[path = "binding/cooling_positive_supply_cp_air_assignment_tests\.rs"\]' -Description "CP331 binding test module path"
Assert-Contains -Path $cp331BindingTests -Pattern 'scheduled_binding_assigns_cp_air_from_the_live_zone_humidity_bit_exactly' -Description "CP331 active binding regression"
Assert-Contains -Path $cp331BindingTests -Pattern 'scheduled_binding_accepts_negative_zero_humidity_with_exact_cp329_lineage' -Description "CP331 signed-zero binding admission regression"
Assert-Contains -Path $cp331BindingTests -Pattern 'scheduled_binding_skips_cp331_after_the_active_positive_guard_falls_through' -Description "CP331 guard-false binding regression"
Assert-Contains -Path $cp331BindingTests -Pattern 'scheduled_binding_preserves_unit_off_and_non_cooling_cp331_skip_routes' -Description "CP331 skipped binding regression"
$cp331BindingCall = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_cp_air_assignment =\s*advance_positive_supply_cp_air_assignment\([^;]+?\)\?;'
)
if (-not $cp331BindingCall.Success) {
    throw "Binding must retain the complete CP331 exact release call"
}
$cp332BindingCallForCp331 = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_assignment =\s*advance_positive_supply_temperature_assignment\([^;]+?\)\?;'
)
if (-not $cp332BindingCallForCp331.Success) {
    throw "Binding must retain the complete CP332 exact release call after CP331"
}
$cp332BindingCallEndForCp331 =
    $cp332BindingCallForCp331.Index + $cp332BindingCallForCp331.Length
$cp333BindingCallForCp331 = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_minimum_limit =\s*advance_positive_supply_temperature_minimum_limit\([^;]+?\)\?;'
)
if (-not $cp333BindingCallForCp331.Success) {
    throw "Binding must retain the complete CP333 exact release call after CP332"
}
$cp333BindingCallEndForCp331 =
    $cp333BindingCallForCp331.Index + $cp333BindingCallForCp331.Length
$cp334BindingCallForCp331 = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
if (-not $cp334BindingCallForCp331.Success) {
    throw "Binding must retain the complete CP334 exact release call after CP333"
}
$cp334BindingCallEndForCp331 =
    $cp334BindingCallForCp331.Index + $cp334BindingCallForCp331.Length
$cp335BindingCallForCp331 = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
if (-not $cp335BindingCallForCp331.Success) {
    throw "Binding must retain the complete CP335 exact release call after CP334"
}
$cp335BindingCallEndForCp331 =
    $cp335BindingCallForCp331.Index + $cp335BindingCallForCp331.Length
$cp336BindingCallForCp331 = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
if (-not $cp336BindingCallForCp331.Success) {
    throw "Binding must retain the complete CP336 exact release call after CP335"
}
$cp336BindingCallEndForCp331 =
    $cp336BindingCallForCp331.Index + $cp336BindingCallForCp331.Length
$cp337BindingCallForCp331 = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
if (-not $cp337BindingCallForCp331.Success) {
    throw "Binding must retain the complete CP337 exact release call after CP336"
}
$cp337BindingCallEndForCp331 =
    $cp337BindingCallForCp331.Index + $cp337BindingCallForCp331.Length
$cp338BindingCallForCp331 = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
if (-not $cp338BindingCallForCp331.Success) {
    throw "Binding must retain the complete CP338 exact release call after CP337"
}
$cp338BindingCallEndForCp331 =
    $cp338BindingCallForCp331.Index + $cp338BindingCallForCp331.Length
$cp339BindingCallForCp331 = [regex]::Match(
    $cp331BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (-not $cp339BindingCallForCp331.Success) {
    throw "Binding must retain the complete CP339 exact release call after CP338"
}
$cp339BindingCallEndForCp331 =
    $cp339BindingCallForCp331.Index + $cp339BindingCallForCp331.Length
if (
    $cp333BindingIndexForCp331 -lt $cp332BindingCallEndForCp331 -or
    $cp334BindingIndexForCp331 -lt $cp333BindingCallEndForCp331 -or
    $cp335BindingIndexForCp331 -lt $cp334BindingCallEndForCp331 -or
    $cp336BindingIndexForCp331 -lt $cp335BindingCallEndForCp331 -or
    $cp337BindingIndexForCp331 -lt $cp336BindingCallEndForCp331 -or
    $cp338BindingIndexForCp331 -lt $cp337BindingCallEndForCp331 -or
    $cp339BindingIndexForCp331 -lt $cp338BindingCallEndForCp331 -or
    $numericalBindingIndexForCp331 -lt $cp339BindingCallEndForCp331
) {
    throw "CP332, CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
$postCp332BeforeCp333ForCp331 = $cp331BindingText.Substring(
    $cp332BindingCallEndForCp331,
    $cp333BindingIndexForCp331 - $cp332BindingCallEndForCp331
)
$postCp332BeforeCp333CodeForCp331 =
    [regex]::Replace($postCp332BeforeCp333ForCp331, '(?m)//.*$', '')
if ($postCp332BeforeCp333CodeForCp331 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP332 and before CP333"
}
$postCp333BeforeCp334ForCp331 = $cp331BindingText.Substring(
    $cp333BindingCallEndForCp331,
    $cp334BindingIndexForCp331 - $cp333BindingCallEndForCp331
)
$postCp333BeforeCp334CodeForCp331 =
    [regex]::Replace($postCp333BeforeCp334ForCp331, '(?m)//.*$', '')
if ($postCp333BeforeCp334CodeForCp331 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP333 and before CP334"
}
$postCp334BeforeCp335ForCp331 = $cp331BindingText.Substring(
    $cp334BindingCallEndForCp331,
    $cp335BindingIndexForCp331 - $cp334BindingCallEndForCp331
)
$postCp334BeforeCp335CodeForCp331 =
    [regex]::Replace($postCp334BeforeCp335ForCp331, '(?m)//.*$', '')
if ($postCp334BeforeCp335CodeForCp331 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP334 and before CP335"
}
$postCp335BeforeCp336ForCp331 = $cp331BindingText.Substring(
    $cp335BindingCallEndForCp331,
    $cp336BindingIndexForCp331 - $cp335BindingCallEndForCp331
)
$postCp335BeforeCp336CodeForCp331 =
    [regex]::Replace($postCp335BeforeCp336ForCp331, '(?m)//.*$', '')
if ($postCp335BeforeCp336CodeForCp331 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP335 and before CP336"
}
$postCp336BeforeCp337ForCp331 = $cp331BindingText.Substring(
    $cp336BindingCallEndForCp331,
    $cp337BindingIndexForCp331 - $cp336BindingCallEndForCp331
)
$postCp336BeforeCp337CodeForCp331 =
    [regex]::Replace($postCp336BeforeCp337ForCp331, '(?m)//.*$', '')
if ($postCp336BeforeCp337CodeForCp331 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP336 and before CP337"
}
$postCp337BeforeCp338ForCp331 = $cp331BindingText.Substring(
    $cp337BindingCallEndForCp331,
    $cp338BindingIndexForCp331 - $cp337BindingCallEndForCp331
)
$postCp337BeforeCp338CodeForCp331 =
    [regex]::Replace($postCp337BeforeCp338ForCp331, '(?m)//.*$', '')
if ($postCp337BeforeCp338CodeForCp331 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP337 and before CP338"
}
$postCp338BeforeCp339ForCp331 = $cp331BindingText.Substring(
    $cp338BindingCallEndForCp331,
    $cp339BindingIndexForCp331 - $cp338BindingCallEndForCp331
)
$postCp338BeforeCp339CodeForCp331 =
    [regex]::Replace($postCp338BeforeCp339ForCp331, '(?m)//.*$', '')
if ($postCp338BeforeCp339CodeForCp331 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP338 and before CP339"
}
$postCp339BeforeNumericalForCp331 = $cp331BindingText.Substring(
    $cp339BindingCallEndForCp331,
    $numericalBindingIndexForCp331 - $cp339BindingCallEndForCp331
)
$postCp339BeforeNumericalCodeForCp331 =
    [regex]::Replace($postCp339BeforeNumericalForCp331, '(?m)//.*$', '')
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp331 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp331,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;)',
    ''
)
if ($postCp339BeforeNumericalCodeForCp331 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No helper other than the audited CP340 through CP348 releases may execute after CP339 and before numerical Calc"
}

# Coupled validation independently reconstructs CP331 from the CP330 route and
# retained CP329 lineage, reconciles dynamic counters, and retains exact bits.
Assert-Contains -Path $cp331CoupledRuntime -Pattern 'mod cooling_positive_supply_cp_air_assignment_validation;' -Description "coupled CP331 validator declaration"
Assert-Contains -Path $cp331CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_cp_air_assignment_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary' -Description "coupled CP331 lifecycle"
Assert-Contains -Path $cp331CoupledRuntime -Pattern 'cooling_positive_supply_cp_air_assignment_validation::snapshot_matches_release' -Description "coupled per-timestep CP331 validation"
Assert-Contains -Path $cp331CoupledRuntime -Pattern 'cooling_positive_supply_cp_air_assignment_validation::validate_lifecycle' -Description "coupled final CP331 validation"
Assert-Contains -Path $cp331CoupledValidation -Pattern 'let predecessor = output\.calculation_cooling_supply_mass_flow_positive_guard;' -Description "coupled CP330 predecessor"
Assert-Contains -Path $cp331CoupledValidation -Pattern 'let snapshot = output\.calculation_cooling_positive_supply_cp_air_assignment;' -Description "coupled CP331 output"
Assert-Contains -Path $cp331CoupledValidation -Pattern '(?s)let mixed_air = output\.calculation_cooling_mixed_air_call;.*?let source_humidity_ratio = snapshot.*?zone_humidity_ratio.*?\.or\(mixed_air\.recirculation_humidity_ratio\)' -Description "coupled CP331 retained source lineage"
Assert-Contains -Path $cp331CoupledValidation -Pattern '(?s)source_lineage_matches.*?mixed_air\.recirculation_humidity_ratio.*?mixed_air\.mixed_air_humidity_ratio' -Description "coupled CP331 CP329 humidity-bit lineage"
Assert-Contains -Path $cp331CoupledValidation -Pattern 'state\.cp_air_assignment_count,\s*3,' -Description "coupled CP331 dynamic source-site count"
Assert-Contains -Path $cp331CoupledValidation -Pattern 'zone_humidity_ratio\.map\(energyplus_psy_cp_air_fn_w\)' -Description "coupled CP331 canonical scalar"
Assert-Contains -Path $cp331CoupledValidation -Pattern 'options_have_exact_bits' -Description "coupled CP331 exact-bit validation"
Assert-Contains -Path $cp331CoupledValidation -Pattern 'source_site_count_multiplication_overflow_fails_closed' -Description "coupled CP331 source-count overflow regression"
Assert-Contains -Path $cp331CoupledValidation -Pattern 'snapshot_comparison_detects_signed_zero_source_corruption' -Description "coupled CP331 signed-zero corruption regression"
Assert-Contains -Path $cp331CoupledTests -Pattern 'cooling_positive_supply_cp_air_assignment_partition_overflow_fails_closed' -Description "coupled CP331 partition-overflow regression"
Assert-Contains -Path $cp331CoupledTests -Pattern 'all_hard_sized_finite_limit_branches_run_with_source_threshold_demand' -Description "coupled CP331 positive-route coverage"
Assert-Contains -Path $cp331CoupledTests -Pattern 'cooling_mixed_air_call_executes_for_active_positive_zero_supply_flow' -Description "coupled CP331 guard-false coverage"
Assert-Contains -Path $cp331CoupledFixtureRoot -Pattern 'cooling_positive_supply_cp_air_assignment_fixture\.rs' -Description "coupled CP331 fixture module"
Assert-Contains -Path $cp331CoupledFixture -Pattern 'calculation_cooling_positive_supply_cp_air_assignment_snapshot\s*\(' -Description "coupled CP331 fixture"
Assert-Contains -Path $cp331CoupledFixture -Pattern 'zone_humidity_ratio\.map\(energyplus_psy_cp_air_fn_w\)' -Description "coupled fixture CP331 canonical scalar"

# Pipeline evidence is direct-only, source-ordered, CP329-lineage checked, and
# serializes each f64 evidence field with exact IEEE bits.
Assert-Contains -Path $cp331PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_cp_air_assignment;' -Description "pipeline CP331 module declaration"
Assert-Contains -Path $cp331PipelineRoot -Pattern '"purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle"' -Description "pipeline CP331 lifecycle JSON key"
Assert-Contains -Path $cp331PipelineRoot -Pattern 'purchased_air_cooling_positive_supply_cp_air_assignment::validate_direct_lifecycle' -Description "pipeline CP331 direct firewall"
Assert-Contains -Path $cp331Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_POSITIVE_GUARD_SOURCE_ORDER' -Description "pipeline CP330-to-CP331 lineage"
Assert-Contains -Path $cp331Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER' -Description "pipeline CP329 humidity lineage"
Assert-Contains -Path $cp331Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CP_AIR_ASSIGNMENT_SOURCE_ORDER' -Description "pipeline CP331 source order"
Assert-Contains -Path $cp331PipelineValidation -Pattern 'checked_product\(assignments, 3, "source-site count"\)' -Description "pipeline CP331 dynamic source-site formula"
Assert-Contains -Path $cp331PipelineValidation -Pattern 'option_has_bits\(mixed_air\.recirculation_humidity_ratio, humidity_ratio\)' -Description "pipeline CP331 recirculation humidity bits"
Assert-Contains -Path $cp331PipelineValidation -Pattern 'option_has_bits\(mixed_air\.mixed_air_humidity_ratio, humidity_ratio\)' -Description "pipeline CP331 mixed-air humidity bits"
Assert-Contains -Path $cp331PipelineValidation -Pattern 'energyplus_psy_cp_air_fn_w\(humidity_ratio\)' -Description "pipeline CP331 canonical scalar"
Assert-NotContains -Path $cp331Pipeline -Pattern 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow' -Description "final numerical DTO reconciliation in CP331 pipeline"
Assert-Contains -Path $cp331PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP331 latest serialization"
foreach ($cp331BitsField in @(
        "zone_humidity_ratio_ieee_bits",
        "psychrometric_cp_air_result_j_per_kg_k_ieee_bits",
        "cp_air_j_per_kg_k_ieee_bits"
    )) {
    Assert-Contains -Path $cp331PipelineSnapshotSerialization -Pattern ('"' + $cp331BitsField + '"') -Description "pipeline CP331 IEEE field '$cp331BitsField'"
}
Assert-Contains -Path $cp331PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP331 exact IEEE serialization"
Assert-Contains -Path $cp331Pipeline -Pattern 'json_preserves_zone_humidity_and_cp_air_ieee_bits' -Description "pipeline CP331 IEEE JSON regression"
Assert-Contains -Path $cp331RunTests -Pattern 'mod cooling_positive_supply_cp_air_assignment_assertions;' -Description "direct integration CP331 assertion module"
Assert-Contains -Path $cp331RunTests -Pattern 'assert_cooling_positive_supply_cp_air_assignment\(' -Description "direct integration CP331 assertion calls"
Assert-Contains -Path $cp331DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle' -Description "direct integration CP331 lifecycle key"
Assert-Contains -Path $cp331DirectAssertions -Pattern 'assignments \* 3' -Description "direct integration CP331 dynamic site count"
Assert-Contains -Path $cp331DirectAssertions -Pattern 'mixed_air\["recirculation_humidity_ratio_ieee_bits"\]' -Description "direct integration CP331 recirculation lineage"
Assert-Contains -Path $cp331DirectAssertions -Pattern 'mixed_air\["mixed_air_humidity_ratio_ieee_bits"\]' -Description "direct integration CP331 mixed-air lineage"
Assert-Contains -Path $cp331DirectAssertions -Pattern 'f64_from_ieee_bits' -Description "direct integration CP331 raw IEEE reconstruction"
Assert-Contains -Path $cp331NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle' -Description "non-direct CP331 null-evidence boundary"

# Specs retain exactly two parent addenda and targets without any promotion.
$cp331AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp331AlgorithmAddenda = [regex]::Matches(
    $cp331AlgorithmText,
    '(?m)^\s*"CP331 supersedes only CP330[^"\r\n]+",\s*$'
)
if ($cp331AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP331 support addenda"
}
foreach ($cp331AlgorithmAddendum in $cp331AlgorithmAddenda) {
    foreach ($cp331Pattern in @(
            'line-2185',
            'exactly three lexical sites',
            '3 \* positive_supply_mass_flow_body_entries',
            'live Zone humidity',
            'CP329',
            'recirculation- and mixed-air',
            'same-call lineage evidence rather than substitute source operands',
            '(?i)Positive-route admission',
            '(?is)(?:finite.{0,40}(?:live humidity|humidity)|(?:live humidity|humidity).{0,40}finite)',
            '>= 0\.0',
            '(?is)(?:both\s+signed\s+zeros\s+(?:are\s+)?admitted|admits\s+both\s+signed\s+zeros)',
            '(?is)canonical.{0,30}CpAir.{0,80}finite',
            'stateless canonical scalar',
            'dwSave',
            'cpaSave',
            'raw-`-100\.0` first-call anomaly',
            'CP330-to-CP331-to-numerical',
            'Line 2186 is the first excluded lexical executable',
            'line 2340',
            '2465',
            'both parents remain `scaffold`/`none`',
            'Roadmap state stay unchanged'
        )) {
        if ($cp331AlgorithmAddendum.Value -notmatch $cp331Pattern) {
            throw "CP331 algorithm addendum missing '$cp331Pattern'"
        }
    }
}
foreach ($cp331TargetPattern in @(
        'cooling_positive_supply_cp_air_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment',
        'cooling_positive_supply_cp_air_assignment\.rs::purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary',
        'cooling_positive_supply_cp_air_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState',
        'cooling_positive_supply_cp_air_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary'
    )) {
    Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern $cp331TargetPattern -Description "CP331 algorithm target '$cp331TargetPattern'"
}

$cp331CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp331CapabilityAddenda = [regex]::Matches(
    $cp331CapabilityText,
    '(?m)^\s*"CP331 additionally requires[^"\r\n]+",\s*$'
)
if ($cp331CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP331 claim addenda"
}
foreach ($cp331CapabilityAddendum in $cp331CapabilityAddenda) {
    foreach ($cp331Pattern in @(
            'line 2185',
            'three-site',
            '3 \* positive_supply_mass_flow_body_entries',
            'live Zone humidity',
            'CP329',
            'recirculation- and mixed-air',
            'same-call lineage evidence rather than substitute source operands',
            '(?i)Positive-route admission',
            '(?is)(?:finite.{0,40}(?:live humidity|humidity)|(?:live humidity|humidity).{0,40}finite)',
            '>= 0\.0',
            '(?is)(?:both\s+signed\s+zeros\s+(?:are\s+)?admitted|admits\s+both\s+signed\s+zeros)',
            '(?is)canonical.{0,30}CpAir.{0,80}finite',
            'stateless canonical scalar',
            'dwSave',
            'cpaSave',
            'raw-`-100\.0` first-call anomaly',
            'CP330-to-CP331-to-numerical',
            'Line 2186 is the first excluded lexical executable',
            '2340',
            '2465',
            'This changes no support level',
            'both Calc routines remain `source_mapped`'
        )) {
        if ($cp331CapabilityAddendum.Value -notmatch $cp331Pattern) {
            throw "CP331 capability addendum missing '$cp331Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP331 supersedes only CP330' -Description "generated CP331 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP331 additionally requires' -Description "generated CP331 capability index"

# Every hand-authored contract repeats the statement, live-Zone ownership,
# CP329-only lineage, cache exclusions, transactional chain, and non-promotion.
$cp331DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP331 maps only the single Cooling positive-supply.*?^and Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP331 Source-Ordered Cooling Positive-Supply CpAir Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP331 Cooling Positive-Supply CpAir Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP331 Positive-Supply CpAir Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP331 Cooling Positive-Supply CpAir Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp331Documentation in $cp331DocumentationSections) {
    $cp331DocumentText = Read-RepoText -Path $cp331Documentation.Path
    $cp331Matches = [regex]::Matches($cp331DocumentText, $cp331Documentation.Pattern)
    if ($cp331Matches.Count -ne 1) {
        throw "CP331 documentation expected one scoped section in $($cp331Documentation.Path), found $($cp331Matches.Count)"
    }
    $cp331Section = $cp331Matches[0].Value
    foreach ($cp331Pattern in @(
            'line 2185|line-2185',
            '(?:exactly )?three(?:-site|\s+lexical|\s+sites)|all three',
            '3 \* positive_supply_mass_flow_body_entries',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?i)active[- ]false|guard-false',
            '(?is)live.{0,120}Zone|ZoneHeatBalanceState::air_humidity_ratio',
            'CP329',
            '(?is)recirculation.{0,100}mixed-air|mixed-air.{0,100}recirculation',
            '(?is)lineage.{0,160}(?:not substituted|not replace|rather than substitute)|same-call lineage evidence',
            'energyplus_psy_cp_air_fn_w',
            '(?i)Positive-route admission',
            '(?is)(?:finite.{0,40}(?:live humidity|humidity)|(?:live humidity|humidity).{0,40}finite)',
            '>= 0\.0',
            '(?is)(?:both\s+signed\s+zeros\s+(?:are\s+)?admitted|admits\s+both\s+signed\s+zeros)',
            '(?is)canonical.{0,30}CpAir.{0,80}finite',
            'dwSave',
            'cpaSave',
            '-100\.0',
            '(?i)anomaly',
            '(?is)all.{0,20}`?f64`?',
            '(?i)CP330',
            '(?i)latest/private witness|latest snapshot, private',
            '(?is)completed.{0,180}pending|pending.{0,180}completed',
            '(?is)checked(?:-|\s+)arithmetic',
            '(?i)transaction|before mutation|state unchanged',
            'purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle',
            '(?is)CP330-to-CP331-to-numerical|CP330, CP331, then|after CP330.{0,100}before.{0,100}numerical',
            '(?is)(?:does not|neither).{0,120}(?:consume|reconcile).{0,180}numerical DTO',
            'line 2186',
            '2340',
            '2454-2461',
            '2465',
            '(?i)source_mapped',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp331Section -notmatch $cp331Pattern) {
            throw "CP331 documentation in $($cp331Documentation.Path) missing '$cp331Pattern'"
        }
    }
}

# Main audit and generated script inventory remain in checkpoint order.
$cp331MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp330DotSourceIndexForCp331 = $cp331MainAuditText.IndexOf('ideal-loads-structure-audit\cp330-cooling-supply-mass-flow-positive-guard.ps1')
$cp331DotSourceIndex = $cp331MainAuditText.IndexOf('ideal-loads-structure-audit\cp331-cooling-positive-supply-cp-air-assignment.ps1')
$cp331AuditCompletionIndex = $cp331MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp330DotSourceIndexForCp331 -lt 0 -or
    $cp331DotSourceIndex -le $cp330DotSourceIndexForCp331 -or
    $cp331AuditCompletionIndex -le $cp331DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP331 after CP330 and before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp331-cooling-positive-supply-cp-air-assignment\.ps1"' -Description "CP331 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp331-cooling-positive-supply-cp-air-assignment\.ps1::dot_sources' -Description "CP331 main-audit callee evidence"
