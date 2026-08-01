# CP338 maps only PurchasedAirManager.cc physical executable line 2196:
# CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat);
# Physical line 2197 is the first excluded lexical executable and CP339
# boundary. This file is dot-sourced after the shared assertions and paths.
$cp338Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment.rs"
$cp338State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment\state.rs"
$cp338Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment\transition.rs"
$cp338Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment\release.rs"
$cp338PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment\release\prefix_validation.rs"
$cp338RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment\release\runtime_validation.rs"
$cp338SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment\release\snapshot_validation.rs"
$cp338Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment\tests\mod.rs"
$cp338ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_cp_air_assignment\tests\release_corruption.rs"
$cp338CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp338Psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$cp338InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp338InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp338InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp338InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_capacity_limit_cp_air_assignment.rs"
$cp338Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp338Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp338BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_capacity_limit_cp_air_assignment.rs"
$cp338BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp338BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_capacity_limit_cp_air_assignment_tests.rs"
$cp338ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp338CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp338CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_capacity_limit_cp_air_assignment_validation.rs"
$cp338CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp338CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_capacity_limit_cp_air_assignment_fixture.rs"
$cp338PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp338Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_cp_air_assignment.rs"
$cp338PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_cp_air_assignment\validation.rs"
$cp338PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_cp_air_assignment\serialization.rs"
$cp338PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_cp_air_assignment\serialization\snapshot.rs"
$cp338RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp338DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_capacity_limit_cp_air_assignment_assertions.rs"
$cp338NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp338RequiredFile in @(
        $cp338Module,
        $cp338State,
        $cp338Transition,
        $cp338Release,
        $cp338PrefixValidation,
        $cp338RuntimeValidation,
        $cp338SnapshotValidation,
        $cp338Tests,
        $cp338ReleaseCorruptionTests,
        $cp338Psychrometrics,
        $cp338InitWitness,
        $cp338BindingAdapter,
        $cp338BindingTests,
        $cp338CoupledValidation,
        $cp338CoupledFixture,
        $cp338Pipeline,
        $cp338PipelineValidation,
        $cp338PipelineSerialization,
        $cp338PipelineSnapshotSerialization,
        $cp338DirectAssertions
    )) {
    Assert-FileExists -Path $cp338RequiredFile -Description "CP338 capacity-limit CpAir assignment structure"
}
Assert-LineLimit -Path $cp338Release -Limit 420 -Description "CP338 release root module"
Assert-LineLimit -Path $cp338ReleaseCorruptionTests -Limit 600 -Description "CP338 release corruption regressions"
Assert-LineLimit -Path $cp338CoupledValidation -Limit 400 -Description "CP338 coupled validation module"
Assert-LineLimit -Path $cp338Pipeline -Limit 320 -Description "CP338 pipeline module"

# The core owns exactly line 2196 and exactly three deterministic Rust sites.
Assert-Contains -Path $cp338Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2196' -Description "CP338 exact physical source boundary"
Assert-Contains -Path $cp338Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2197' -Description "CP338 first excluded physical executable"
Assert-Contains -Path $cp338Module -Pattern 'Exact three textual source sites represented by CP338' -Description "CP338 exact textual-site count"
Assert-ExactStringArray -Path $cp338Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-purchased-air-mixed-air-humidity-ratio",
    "evaluate-psy-cp-air-fn-w",
    "assign-local-cp-air"
) -Description "CP338 deterministic Rust witness order"
Assert-Contains -Path $cp338Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot' -Description "CP338 public snapshot"
Assert-Contains -Path $cp338State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState' -Description "CP338 persistent state"
Assert-Contains -Path $cp338Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary' -Description "CP338 lifecycle summary"
Assert-Contains -Path $cp338Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary\s*\(' -Description "CP338 lifecycle accessor"
Assert-Contains -Path $cp338Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment\s*\(' -Description "CP338 exact-direct wrapper"
Assert-Contains -Path $cp338Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_capacity_limit_cp_air_assignment_state\s*\(' -Description "CP338 pure transition"
Assert-Contains -Path $cp338CalcRoot -Pattern 'mod cooling_positive_supply_capacity_limit_cp_air_assignment;' -Description "CP338 calc module declaration"
Assert-Contains -Path $cp338CalcRoot -Pattern 'pub use (?:cooling_positive_supply_capacity_limit_cp_air_assignment::\*;|\{[^}]*cooling_positive_supply_capacity_limit_cp_air_assignment::\*)' -Description "CP338 calc public surface"

foreach ($cp338SnapshotField in @(
        'predecessor_capacity_limit_guard_evaluated',
        'predecessor_capacity_limit_body_entered',
        'predecessor_active_capacity_limit_guard_false_fallthrough',
        'capacity_limit_guard_false_fallthrough_skipped',
        'capacity_limit_cp_air_assignment_executed',
        'mixed_air_humidity_ratio_read',
        'mixed_air_humidity_ratio',
        'psychrometric_cp_air_evaluated',
        'psychrometric_cp_air_result_j_per_kg_k',
        'cp_air_assigned',
        'cp_air_j_per_kg_k'
    )) {
    Assert-Contains -Path $cp338Module -Pattern ('pub ' + $cp338SnapshotField + ':') -Description "CP338 snapshot field '$cp338SnapshotField'"
}
foreach ($cp338Route in @(
        'UnitOff',
        'NonCooling',
        'PositiveGuardFalseFallthrough',
        'ActiveCapacityLimitGuardFalseFallthrough',
        'CapacityLimitCpAirAssigned'
    )) {
    Assert-Contains -Path $cp338State -Pattern $cp338Route -Description "CP338 retained route '$cp338Route'"
}
foreach ($cp338Counter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'capacity_limit_guard_false_fallthrough_skip_count',
        'capacity_limit_cp_air_assignment_count',
        'source_site_execution_count',
        'mixed_air_humidity_ratio_read_count',
        'psychrometric_cp_air_evaluation_count',
        'cp_air_assignment_write_count'
    )) {
    Assert-Contains -Path $cp338State -Pattern ('pub ' + $cp338Counter + ': usize') -Description "CP338 state counter '$cp338Counter'"
}

# CP337 body entry alone activates the three-site transition. Skips retain no
# operand or result, and the active result comes only from the canonical helper.
Assert-Contains -Path $cp338Transition -Pattern 'let assignment_executed = predecessor\.capacity_limit_body_entered;' -Description "CP337 body-entry activation"
Assert-Contains -Path $cp338Transition -Pattern 'mixed_air_humidity_ratio\.map\(energyplus_psy_cp_air_fn_w\)' -Description "canonical CP338 psychrometric evaluation"
Assert-Contains -Path $cp338Transition -Pattern 'let cp_air_j_per_kg_k = psychrometric_cp_air_result_j_per_kg_k;' -Description "bit-identical local CpAir assignment"
Assert-Contains -Path $cp338Transition -Pattern '(?s)source_site_execution_count \+=\s*PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_CP_AIR_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "three-site active counter"
Assert-PatternsInOrder -Path $cp338Transition -Patterns @(
    'predecessor\.unit_off_skipped',
    'predecessor\.non_cooling_skipped',
    'predecessor\.positive_guard_false_fallthrough_skipped',
    'predecessor\.active_guard_false_fallthrough',
    'capacity_limit_cp_air_assignment_count \+= 1',
    'mixed_air_humidity_ratio_read_count \+= 1',
    'psychrometric_cp_air_evaluation_count \+= 1',
    'cp_air_assignment_write_count \+= 1'
) -Description "CP338 five-route and active-site transition order"
Assert-Contains -Path $cp338RuntimeValidation -Pattern '(?s)unit_off_skip_count.*?checked_add\(state\.non_cooling_skip_count\).*?positive_guard_false_fallthrough_skip_count.*?capacity_limit_guard_false_fallthrough_skip_count.*?capacity_limit_cp_air_assignment_count' -Description "CP338 checked five-route partition"
Assert-Contains -Path $cp338RuntimeValidation -Pattern '(?s)capacity_limit_cp_air_assignment_count\s*\.checked_mul\(3\)' -Description "CP338 checked 3*A source-site formula"
Assert-Contains -Path $cp338RuntimeValidation -Pattern 'capacity_limit_cp_air_assignment_count\s*== predecessor\.capacity_limit_body_entry_count' -Description "CP338 A equals CP337 B"
foreach ($cp338PerSiteCounter in @(
        'mixed_air_humidity_ratio_read_count',
        'psychrometric_cp_air_evaluation_count',
        'cp_air_assignment_write_count'
    )) {
    Assert-Contains -Path $cp338RuntimeValidation -Pattern ($cp338PerSiteCounter + '\s*== state\.capacity_limit_cp_air_assignment_count') -Description "CP338 per-site count equals A for '$cp338PerSiteCounter'"
}
Assert-Contains -Path $cp338Psychrometrics -Pattern 'pub fn energyplus_psy_cp_air_fn_w\s*\(' -Description "canonical EnergyPlus CpAir helper"
Assert-NotContains -Path $cp338Transition -Pattern 'moist_air_specific_heat|energyplus_psy_cp_air_fn_w_fast|zone_air|maximum.*capacity|sizing|CoolSensOutput|SupplyMassFlowRate|MixedAirEnthalpy|SupplyEnthalpy' -Description "noncanonical, later-body, or excluded CP338 transition input"

# Public release accepts only the same-call CP337 predecessor and reads the
# active RHS from the completed CP329 latest/private witness.
Assert-Contains -Path $cp338Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp337: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,\s*\)' -Description "CP338 release argument boundary"
Assert-Contains -Path $cp338Release -Pattern 'cooling_positive_supply_capacity_limit_guard_latest_witness\(selected\)' -Description "CP338 CP337 private predecessor witness"
Assert-Contains -Path $cp338Release -Pattern 'unit\s*\.calc_cooling_positive_supply_capacity_limit_guard\s*\.latest' -Description "CP338 CP337 public predecessor latest"
Assert-Contains -Path $cp338Release -Pattern 'unit\s*\.calc_cooling_mixed_air_call\.latest' -Description "CP338 CP329 public RHS latest"
Assert-Contains -Path $cp338Release -Pattern 'cooling_mixed_air_call_latest_witness\(selected\)' -Description "CP338 CP329 private RHS witness"
Assert-Contains -Path $cp338Release -Pattern 'let mixed_air_humidity_ratio = mixed_air\.mixed_air_humidity_ratio' -Description "CP329-owned retained humidity operand"
Assert-Contains -Path $cp338Release -Pattern 'completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent' -Description "recursive CP337 release proof"
Assert-Contains -Path $cp338Release -Pattern 'completed_direct_cooling_mixed_air_call_is_consistent' -Description "recursive CP329 release proof"
Assert-Contains -Path $cp338Release -Pattern 'energyplus_psy_cp_air_fn_w\(mixed_air_humidity_ratio\)\.is_finite\(\)' -Description "canonical result preflight"
Assert-Contains -Path $cp338Release -Pattern 'set_cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness' -Description "CP338 private witness commit"
Assert-Contains -Path $cp338PrefixValidation -Pattern '(?s)mixed_air\s*\.mixed_air_humidity_ratio\s*\.is_some_and\(\|value\| value\.to_bits\(\) == operand\.to_bits\(\)\)' -Description "CP329 bit-exact RHS provenance"
Assert-Contains -Path $cp338SnapshotValidation -Pattern 'energyplus_psy_cp_air_fn_w\(humidity_ratio\)' -Description "snapshot canonical result validation"
Assert-Contains -Path $cp338SnapshotValidation -Pattern 'assigned\.to_bits\(\) == result\.to_bits\(\)' -Description "snapshot bit-exact assignment"
Assert-Contains -Path $cp338SnapshotValidation -Pattern '(?s)!snapshot\.mixed_air_humidity_ratio_read.*?mixed_air_humidity_ratio\.is_none\(\).*?!snapshot\.psychrometric_cp_air_evaluated.*?psychrometric_cp_air_result_j_per_kg_k\s*\.is_none\(\).*?!snapshot\.cp_air_assigned.*?cp_air_j_per_kg_k\.is_none\(\)' -Description "complete-null CP338 skip validation"
Assert-NotContains -Path $cp338Release -Pattern 'zone_air_humidity|controlled_zone_humidity|maximum.*capacity|sizing_value|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "duplicate Zone, capacity, sizing, or numerical input in CP338 release"

foreach ($cp338CoreTest in @(
        'source_boundary_and_exact_three_sites_are_stable',
        'capacity_body_executes_three_sites_and_assigns_canonical_cp_air',
        'all_four_skipped_routes_execute_no_cp338_sites_or_scalar_work',
        'counters_partition_all_five_routes_and_count_three_sites_per_assignment',
        'exact_predicate_and_bit_matcher_reject_corruption_and_signed_zero_drift'
    )) {
    Assert-Contains -Path $cp338Tests -Pattern $cp338CoreTest -Description "CP338 core regression '$cp338CoreTest'"
}
foreach ($cp338ReleaseTest in @(
        'public_active_release_uses_retained_cp329_operand_and_rejects_replay',
        'public_release_preserves_all_four_zero_site_skip_routes',
        'supplied_public_or_private_cp337_drift_is_transactional',
        'active_cp329_public_and_private_operand_lineage_drift_is_transactional',
        'orphan_cp338_public_private_and_retained_metadata_are_fail_closed',
        'active_assignment_counter_overflows_are_preflighted_transactionally',
        'route_partition_product_corruption_and_post_commit_drift_are_detected'
    )) {
    Assert-Contains -Path $cp338ReleaseCorruptionTests -Pattern $cp338ReleaseTest -Description "CP338 release regression '$cp338ReleaseTest'"
}

# CP338 state and its latest witness are private to the selected runtime unit.
Assert-Contains -Path $cp338InitState -Pattern '(?s)cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot' -Description "runtime-root private CP338 witness map"
Assert-NotContains -Path $cp338InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witnesses:' -Description "public runtime-root CP338 witness map"
Assert-Contains -Path $cp338InitWitnessRoot -Pattern 'mod cooling_positive_supply_capacity_limit_cp_air_assignment;' -Description "runtime-root CP338 witness module"
Assert-Contains -Path $cp338InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness\s*\(' -Description "runtime-root CP338 witness getter"
Assert-Contains -Path $cp338InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witness\s*\(' -Description "runtime-root CP338 witness setter"
Assert-Contains -Path $cp338InitState -Pattern 'pub calc_cooling_positive_supply_capacity_limit_cp_air_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState' -Description "per-unit CP338 persistent state"
Assert-Contains -Path $cp338InitUnit -Pattern '(?s)calc_cooling_positive_supply_capacity_limit_cp_air_assignment:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new\(\s*system\s*,?\s*\)' -Description "per-unit CP338 state initialization"

# Binding preserves exact CP337 -> CP338 -> CP339 -> unchanged numerical order and
# allows no intermediary helper execution.
$cp338BindingText = Read-RepoText -Path $cp338Binding
$cp337BindingIndexForCp338 = $cp338BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndex = $cp338BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp338 = $cp338BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp338 = $cp338BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp337BindingIndexForCp338 -lt 0 -or
    $cp338BindingIndex -le $cp337BindingIndexForCp338 -or
    $cp339BindingIndexForCp338 -le $cp338BindingIndex -or
    $numericalBindingIndexForCp338 -le $cp339BindingIndexForCp338
) {
    throw "Binding must retain exact CP337 -> CP338 -> CP339 -> numerical Calc order"
}
$cp337BindingCallForCp338 = [regex]::Match(
    $cp338BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
$cp338BindingCall = [regex]::Match(
    $cp338BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
$cp339BindingCallForCp338 = [regex]::Match(
    $cp338BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (
    -not $cp337BindingCallForCp338.Success -or
    -not $cp338BindingCall.Success -or
    -not $cp339BindingCallForCp338.Success
) {
    throw "Binding must retain complete CP337, CP338, and CP339 exact release calls"
}
$cp337BindingCallEndForCp338 =
    $cp337BindingCallForCp338.Index + $cp337BindingCallForCp338.Length
$cp338BindingCallEnd = $cp338BindingCall.Index + $cp338BindingCall.Length
$cp339BindingCallEndForCp338 =
    $cp339BindingCallForCp338.Index + $cp339BindingCallForCp338.Length
if (
    $cp338BindingIndex -lt $cp337BindingCallEndForCp338 -or
    $cp339BindingIndexForCp338 -lt $cp338BindingCallEnd -or
    $numericalBindingIndexForCp338 -lt $cp339BindingCallEndForCp338
) {
    throw "CP337, CP338, and CP339 exact release calls must complete before numerical Calc"
}
foreach ($cp338Interval in @(
        [PSCustomObject]@{
            Start = $cp337BindingCallEndForCp338
            End = $cp338BindingIndex
            Description = "after CP337 and before CP338"
        },
        [PSCustomObject]@{
            Start = $cp338BindingCallEnd
            End = $cp339BindingIndexForCp338
            Description = "after CP338 and before CP339"
        },
        [PSCustomObject]@{
            Start = $cp339BindingCallEndForCp338
            End = $numericalBindingIndexForCp338
            Description = "after CP339 and before numerical Calc"
        }
    )) {
    $cp338IntervalText = $cp338BindingText.Substring(
        $cp338Interval.Start,
        $cp338Interval.End - $cp338Interval.Start
    )
    $cp338IntervalCode = [regex]::Replace($cp338IntervalText, '(?m)//.*$', '')
    $cp338IntervalCode = [regex]::Replace(
        $cp338IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
        ''
    )
    $cp338IntervalCode = [regex]::Replace(
        $cp338IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
    $cp338IntervalCode = [regex]::Replace(
        $cp338IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
        ''
    )
    $cp338IntervalCode = [regex]::Replace(
        $cp338IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
    $cp338IntervalCode = [regex]::Replace(
        $cp338IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    $cp338IntervalCode = [regex]::Replace(
        $cp338IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp338IntervalCode = [regex]::Replace(
        $cp338IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp338IntervalCode = [regex]::Replace(
        $cp338IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp338IntervalCode = [regex]::Replace(
    $cp338IntervalCode,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;)',
    ''
)
    if ($cp338IntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp338Interval.Description)"
    }
}
Assert-Contains -Path $cp338Binding -Pattern '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_capacity_limit_guard,\s*\)\?;' -Description "binding exact CP337-to-CP338 adapter call"
Assert-Contains -Path $cp338BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_capacity_limit_cp_air_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,\s*\)' -Description "CP338 binding adapter arguments"
Assert-Contains -Path $cp338BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment\(\s*runtime,\s*system,\s*predecessor,\s*\).*?CalculationCoolingPositiveSupplyCapacityLimitCpAirAssignment' -Description "CP338 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp338BindingAdapter -Pattern 'humidity_ratio\s*:|zone_air|maximum.*capacity|sizing|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "extra scalar, Zone, capacity, sizing, or numerical input in CP338 adapter"
Assert-Contains -Path $cp338ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_capacity_limit_cp_air_assignment:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot' -Description "CP338 scheduled output evidence"
Assert-Contains -Path $cp338BindingTestsRoot -Pattern '#\[rustfmt::skip\]\s*#\[path = "binding/cooling_positive_supply_capacity_limit_cp_air_assignment_tests\.rs"\]\s*mod cooling_positive_supply_capacity_limit_cp_air_assignment_tests;' -Description "CP338 binding test module"
foreach ($cp338BindingTest in @(
        'scheduled_binding_assigns_cp_air_for_both_capacity_limit_selectors',
        'scheduled_binding_preserves_capacity_guard_false_fallthroughs_as_complete_null',
        'scheduled_binding_preserves_inherited_complete_null_skip_routes'
    )) {
    Assert-Contains -Path $cp338BindingTests -Pattern $cp338BindingTest -Description "CP338 binding regression '$cp338BindingTest'"
}

# Coupled runtime and pipeline independently validate CP337 activation, CP329
# operand lineage, exact counters, direct-only exposure, and JSON shape.
Assert-Contains -Path $cp338CoupledRuntime -Pattern 'mod cooling_positive_supply_capacity_limit_cp_air_assignment_validation;' -Description "coupled CP338 validator declaration"
Assert-Contains -Path $cp338CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary' -Description "coupled CP338 lifecycle"
Assert-Contains -Path $cp338CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_cp_air_assignment_validation::snapshot_matches_release' -Description "coupled per-timestep CP338 validation"
Assert-Contains -Path $cp338CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_cp_air_assignment_validation::validate_lifecycle' -Description "coupled final CP338 validation"
Assert-Contains -Path $cp338CoupledValidation -Pattern 'output\.calculation_cooling_positive_supply_capacity_limit_guard' -Description "coupled CP337 predecessor"
Assert-Contains -Path $cp338CoupledValidation -Pattern 'output\.calculation_cooling_mixed_air_call' -Description "coupled CP329 RHS provenance"
Assert-Contains -Path $cp338CoupledValidation -Pattern 'energyplus_psy_cp_air_fn_w' -Description "coupled canonical CpAir reconstruction"
Assert-Contains -Path $cp338CoupledValidation -Pattern '(?s)capacity_limit_cp_air_assignment_count,\s*3,.*?source_site_execution_count' -Description "coupled checked 3*A formula"
Assert-Contains -Path $cp338CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_capacity_limit_cp_air_assignment_fixture;' -Description "coupled CP338 fixture declaration"
Assert-Contains -Path $cp338CoupledFixture -Pattern 'calculation_cooling_positive_supply_capacity_limit_cp_air_assignment_snapshot' -Description "coupled CP338 fixture output"

Assert-Contains -Path $cp338PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_capacity_limit_cp_air_assignment;' -Description "pipeline CP338 module declaration"
Assert-Contains -Path $cp338PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle' -Description "pipeline CP338 lifecycle field and JSON key"
Assert-Contains -Path $cp338PipelineRoot -Pattern 'calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle' -Description "pipeline CP338 coupled lifecycle transfer"
Assert-Contains -Path $cp338Pipeline -Pattern '(?s)validate_direct_lifecycle\(.*?predecessor_cp337.*?predecessor_cp329.*?init_lifecycle.*?coupling_call_count' -Description "pipeline CP338 validated inputs"
Assert-Contains -Path $cp338Pipeline -Pattern 'predecessor_state\.capacity_limit_body_entry_count' -Description "pipeline CP337 body-entry parity"
Assert-Contains -Path $cp338Pipeline -Pattern 'validate_source_counters\(state\)' -Description "pipeline CP338 source-counter validation"
Assert-Contains -Path $cp338PipelineValidation -Pattern 'checked_product\(assignments,\s*3,\s*"source-site count"\)' -Description "pipeline checked 3*A source-site formula"
Assert-Contains -Path $cp338PipelineValidation -Pattern 'source_counter_overflow_fails_closed' -Description "pipeline CP338 overflow regression"
Assert-Contains -Path $cp338PipelineSerialization -Pattern '"capacity_limit_cp_air_assignment_count"' -Description "pipeline CP338 lifecycle serialization"
foreach ($cp338JsonField in @(
        'predecessor_capacity_limit_guard_evaluated',
        'predecessor_capacity_limit_body_entered',
        'predecessor_active_capacity_limit_guard_false_fallthrough',
        'capacity_limit_guard_false_fallthrough_skipped',
        'capacity_limit_cp_air_assignment_executed',
        'mixed_air_humidity_ratio_read',
        'mixed_air_humidity_ratio',
        'psychrometric_cp_air_evaluated',
        'psychrometric_cp_air_result_j_per_kg_k',
        'cp_air_assigned',
        'cp_air_j_per_kg_k'
    )) {
    Assert-Contains -Path $cp338PipelineSnapshotSerialization -Pattern ('"' + $cp338JsonField + '"') -Description "pipeline CP338 snapshot field '$cp338JsonField'"
}
Assert-Contains -Path $cp338RunTests -Pattern 'cooling_positive_supply_capacity_limit_cp_air_assignment_assertions' -Description "direct-run CP338 assertion module"
Assert-Contains -Path $cp338DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*3\]\s*=' -Description "direct-run CP338 source order"
Assert-Contains -Path $cp338DirectAssertions -Pattern 'source_site_execution_count"\], assignments \* 3' -Description "direct-run CP338 3*A formula"
Assert-Contains -Path $cp338DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle' -Description "direct-run CP337 predecessor evidence"
Assert-Contains -Path $cp338DirectAssertions -Pattern 'purchased_air_calc_cooling_mixed_air_call_lifecycle' -Description "direct-run CP329 RHS evidence"
Assert-Contains -Path $cp338NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle' -Description "non-direct CP338 null evidence"
Assert-Contains -Path $cp338PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp383_lifecycle_evidence' -Description "non-direct CP338 through CP363 evidence rejection"
Assert-NotContains -Path $cp338Pipeline -Pattern 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|complete_direct_zone_purchased_air_coupling' -Description "numerical DTO reconciliation in CP338 pipeline"

# Exactly two algorithm addenda, two capability addenda, and six target
# occurrences extend inventory without promoting support or readiness.
$cp338AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp338AlgorithmAddenda = [regex]::Matches(
    $cp338AlgorithmText,
    '(?m)^\s*"CP338 supersedes only CP337[^"\r\n]+",\s*$'
)
if ($cp338AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP338 claim addenda"
}
$cp338TargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_cp_air_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_cp_air_assignment\.rs::purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_cp_air_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_cp_air_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp338Target in $cp338TargetCounts) {
    $cp338TargetCount = [regex]::Matches($cp338AlgorithmText, $cp338Target.Pattern).Count
    if ($cp338TargetCount -ne $cp338Target.Expected) {
        throw "CP338 target '$($cp338Target.Pattern)' expected $($cp338Target.Expected), found $cp338TargetCount"
    }
}
$cp338CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp338CapabilityAddenda = [regex]::Matches(
    $cp338CapabilityText,
    '(?m)^\s*"CP338 additionally requires[^"\r\n]+",\s*$'
)
if ($cp338CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP338 claim addenda"
}
foreach ($cp338Claim in @($cp338AlgorithmAddenda) + @($cp338CapabilityAddenda)) {
    foreach ($cp338Pattern in @(
            '(?s)PurchasedAirManager\.cc.*?2196',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            'read-purchased-air-mixed-air-humidity-ratio',
            'evaluate-psy-cp-air-fn-w',
            'assign-local-cp-air',
            'CapacityLimitBodyEntered',
            'LimitCapacity',
            'LimitFlowRateAndCapacity',
            'UnitOff',
            'NonCooling',
            'PositiveGuardFalseFallthrough',
            'ActiveCapacityLimitGuardFalseFallthrough',
            'CapacityLimitCpAirAssigned',
            '3\*A\s*=\s*3\*B',
            'CP329 latest/private witness',
            'mixed_air_humidity_ratio',
            'energyplus_psy_cp_air_fn_w',
            'CP337-to-CP338-to-numerical',
            '2197',
            'CP339',
            'numerical-DTO',
            '(?:Roadmap state remain unchanged|No [^.;]*Roadmap state changes)'
        )) {
        if ($cp338Claim.Value -notmatch $cp338Pattern) {
            throw "CP338 spec addendum missing '$cp338Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP338 supersedes only CP337' -Description "generated CP338 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP338 additionally requires' -Description "generated CP338 capability index"

# Each hand-authored contract has one scoped CP338 section carrying the same
# source, operand, routes, counters, placement, exclusion, and nonpromotion.
$cp338DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP338 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP338 Source-Ordered Cooling Positive-Supply Capacity-Limit CpAir Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP338 Cooling Positive-Supply Capacity-Limit CpAir Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP338 Positive-Supply Capacity-Limit CpAir Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP338 Cooling Positive-Supply Capacity-Limit CpAir Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp338Documentation in $cp338DocumentationSections) {
    $cp338DocumentText = Read-RepoText -Path $cp338Documentation.Path
    $cp338Matches = [regex]::Matches($cp338DocumentText, $cp338Documentation.Pattern)
    if ($cp338Matches.Count -ne 1) {
        throw "CP338 documentation expected one scoped section in $($cp338Documentation.Path), found $($cp338Matches.Count)"
    }
    $cp338Section = $cp338Matches[0].Value
    foreach ($cp338Pattern in @(
            '(?s)PurchasedAirManager\.cc.*?2196',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            'read-purchased-air-mixed-air-humidity-ratio',
            'evaluate-psy-cp-air-fn-w',
            'assign-local-cp-air',
            'CapacityLimitBodyEntered',
            'LimitCapacity',
            'LimitFlowRateAndCapacity',
            'UnitOff',
            'NonCooling',
            'PositiveGuardFalseFallthrough',
            'ActiveCapacityLimitGuardFalseFallthrough',
            'CapacityLimitCpAirAssigned',
            '3\*A\s*=\s*3\*B',
            'CP329',
            'latest/private witness',
            'mixed_air_humidity_ratio',
            'energyplus_psy_cp_air_fn_w',
            'CP337-to-CP338-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle',
            '2197',
            'CP339',
            '(?i)numerical[- ]DTO',
            'Roadmap'
        )) {
        if ($cp338Section -notmatch $cp338Pattern) {
            throw "CP338 documentation in $($cp338Documentation.Path) missing '$cp338Pattern'"
        }
    }
}

# Root reachability and generated inventory account for this one new internal
# script: 284 executable records, 240 public, 44 internal, and zero uncalled.
$cp338MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp337DotSourceIndexForCp338 = $cp338MainAuditText.IndexOf('ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1')
$cp338DotSourceIndex = $cp338MainAuditText.IndexOf('ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1')
$cp339DotSourceIndexForCp338 = $cp338MainAuditText.IndexOf('ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1')
$cp338AuditCompletionIndex = $cp338MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp337DotSourceIndexForCp338 -lt 0 -or
    $cp338DotSourceIndex -le $cp337DotSourceIndexForCp338 -or
    $cp339DotSourceIndexForCp338 -le $cp338DotSourceIndex -or
    $cp338AuditCompletionIndex -le $cp339DotSourceIndexForCp338
) {
    throw "Main IdealLoads audit must dot-source CP338 then CP339 after CP337 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 321' -Description "CP338 cumulative inventory total through CP358"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp338-cooling-positive-supply-capacity-limit-cp-air-assignment\.ps1"' -Description "CP338 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment\.ps1"' -Description "CP339 internal script inventory record after CP338"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp338-cooling-positive-supply-capacity-limit-cp-air-assignment\.ps1::dot_sources' -Description "CP338 main-audit callee evidence"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment\.ps1::dot_sources' -Description "CP339 main-audit callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 321 \|' -Description "CP338 generated script count through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP338 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 81 \|' -Description "CP338 generated internal script count through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP338 generated uncalled script count"
