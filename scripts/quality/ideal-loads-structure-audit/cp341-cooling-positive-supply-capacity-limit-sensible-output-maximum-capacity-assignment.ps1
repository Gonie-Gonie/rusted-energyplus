# CP341 maps only PurchasedAirManager.cc physical executable line 2199:
# CoolSensOutput = PurchAir.MaxCoolTotCap;
# Physical line 2200 is the first excluded executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp341Stem = "cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment"
$cp341PipelineStem = "purchased_air_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment"
$cp341Module = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem.rs"
$cp341State = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem\state.rs"
$cp341Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem\transition.rs"
$cp341Release = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem\release.rs"
$cp341PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem\release\prefix_validation.rs"
$cp341RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem\release\runtime_validation.rs"
$cp341SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem\release\snapshot_validation.rs"
$cp341Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem\tests\mod.rs"
$cp341PublicReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\$cp341Stem\tests\public_release.rs"
$cp341CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp341Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp341ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp341BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp341Stem.rs"
$cp341BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp341BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp341Stem}_tests.rs"
$cp341InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp341InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp341InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp341InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp341Stem.rs"
$cp341CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp341CoupledRuntimeTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp341CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp341Stem}_validation.rs"
$cp341CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp341CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp341Stem}_fixture.rs"
$cp341PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp341Pipeline = "crates\ep_run\src\pipeline\$cp341PipelineStem.rs"
$cp341PipelineValidation = "crates\ep_run\src\pipeline\$cp341PipelineStem\validation.rs"
$cp341PipelineSerialization = "crates\ep_run\src\pipeline\$cp341PipelineStem\serialization.rs"
$cp341PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\$cp341PipelineStem\serialization\snapshot.rs"

foreach ($cp341RequiredFile in @(
        $cp341Module,
        $cp341State,
        $cp341Transition,
        $cp341Release,
        $cp341PrefixValidation,
        $cp341RuntimeValidation,
        $cp341SnapshotValidation,
        $cp341Tests,
        $cp341PublicReleaseTests,
        $cp341BindingAdapter,
        $cp341BindingTests,
        $cp341InitWitness,
        $cp341CoupledValidation,
        $cp341CoupledFixture,
        $cp341Pipeline,
        $cp341PipelineValidation,
        $cp341PipelineSerialization,
        $cp341PipelineSnapshotSerialization,
        $cp341CoupledRuntimeTests
    )) {
    Assert-FileExists -Path $cp341RequiredFile -Description "CP341 maximum-capacity assignment structure"
}
Assert-LineLimit -Path $cp341Release -Limit 800 -Description "CP341 release module"
Assert-LineLimit -Path $cp341CoupledValidation -Limit 800 -Description "CP341 coupled validation module"
Assert-LineLimit -Path $cp341Pipeline -Limit 800 -Description "CP341 pipeline module"

$cp341SourceStatementPattern =
    'CoolSensOutput\s*=\s*PurchAir\.MaxCoolTotCap\s*;'
$cp341FirstExcludedStatementPattern =
    'SupplyEnthalpy\s*=\s*MixedAirEnthalpy\s*-\s*CoolSensOutput\s*/\s*SupplyMassFlowRate\s*;'
$cp341OrderedSourceSitesPattern =
    '(?s)read-retained-maximum-total-cooling-capacity-for-sensible-output-assignment.*?assign-local-cooling-sensible-output-from-maximum-total-cooling-capacity'

# Locked line boundary, exact two-site order, public types, and six routes.
Assert-Contains -Path $cp341Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2199' -Description "CP341 exact physical source boundary"
Assert-Contains -Path $cp341Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2200' -Description "CP341 first excluded physical executable"
Assert-Contains -Path $cp341Module -Pattern 'Exact two textual source sites represented by CP341' -Description "CP341 source-site count"
Assert-ExactStringArray -Path $cp341Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-retained-maximum-total-cooling-capacity-for-sensible-output-assignment",
    "assign-local-cooling-sensible-output-from-maximum-total-cooling-capacity"
) -Description "CP341 deterministic RHS-read-to-LHS-write witness"
Assert-Contains -Path $cp341Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot' -Description "CP341 public snapshot"
Assert-Contains -Path $cp341State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState' -Description "CP341 persistent state"
Assert-Contains -Path $cp341Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary' -Description "CP341 lifecycle summary"
Assert-Contains -Path $cp341Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle_summary\s*\(' -Description "CP341 lifecycle accessor"
Assert-Contains -Path $cp341Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\s*\(' -Description "CP341 exact-direct release"
Assert-Contains -Path $cp341Transition -Pattern 'advance_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_state\s*\(' -Description "CP341 pure transition"
Assert-Contains -Path $cp341CalcRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;' -Description "CP341 Calc module declaration"

foreach ($cp341Route in @(
        'UnitOff',
        'NonCooling',
        'PositiveGuardFalseFallthrough',
        'ActiveCapacityLimitGuardFalseFallthrough',
        'CapacityLimitSensibleOutputGuardFalseFallthrough',
        'CapacityLimitSensibleOutputMaximumCapacityAssigned'
    )) {
    Assert-Contains -Path $cp341State -Pattern $cp341Route -Description "CP341 retained route '$cp341Route'"
}
foreach ($cp341Counter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'capacity_limit_guard_false_fallthrough_skip_count',
        'capacity_limit_sensible_output_guard_false_fallthrough_count',
        'capacity_limit_sensible_output_maximum_capacity_assignment_count',
        'source_site_execution_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_sensible_output_assignment_write_count'
    )) {
    Assert-Contains -Path $cp341State -Pattern ('pub ' + $cp341Counter + ':\s*usize') -Description "CP341 state counter '$cp341Counter'"
}

# The pure transition preserves false-route bits and witnesses retained RHS
# read before the local LHS assignment. It performs no arithmetic or coercion.
Assert-PatternsInOrder -Path $cp341Transition -Patterns @(
    'let maximum_total_cooling_capacity_w = if assignment_executed',
    'predecessor\.maximum_total_cooling_capacity_w',
    'let assigned_cooling_sensible_output_w = if assignment_executed',
    'maximum_total_cooling_capacity_w',
    'let resulting_cooling_sensible_output_w = if assignment_executed',
    'assigned_cooling_sensible_output_w',
    'state\.source_site_execution_count \+= 2;',
    'state\.maximum_total_cooling_capacity_read_count \+= 1;',
    'state\.cooling_sensible_output_assignment_write_count \+= 1;'
) -Description "CP341 retained RHS-read-to-LHS-write and two-site counter order"
Assert-NotContains -Path $cp341Transition -Pattern 'calc_cooling_capacity_zero_flow_reset|total_cmp|partial_cmp|\.clamp\(|is_finite|\.max\(|\.min\(' -Description "CP341 transition excludes CP321 reach-through, ordering helpers, and numeric coercion"
Assert-Contains -Path $cp341Tests -Pattern 'source_boundary_and_exact_two_sites_are_stable' -Description "CP341 source-boundary regression"
Assert-Contains -Path $cp341Tests -Pattern 'all_six_routes_have_exact_local_value_shapes' -Description "CP341 six-route regression"
Assert-Contains -Path $cp341Tests -Pattern 'false_route_preserves_nan_and_signed_zero_bits_without_rhs_or_write' -Description "CP341 false-route IEEE preservation"
Assert-Contains -Path $cp341Tests -Pattern 'true_route_replaces_positive_infinity_with_finite_maximum_bit_exact' -Description "CP341 true-route infinity-to-finite regression"
Assert-Contains -Path $cp341Tests -Pattern 'pure_transition_bit_copies_arbitrary_rhs_payload_without_normalization' -Description "CP341 pure arbitrary IEEE characterization"
Assert-Contains -Path $cp341Tests -Pattern 'counters_partition_six_routes_and_apply_two_m_identity' -Description "CP341 T partition and 2*M regression"

# Runtime validation enforces T=U+N+P+G+F+M, A=F+M,
# M=CP340 adjustment-body entries, 2*M, per-site parity, and checked preflight.
Assert-Contains -Path $cp341RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_guard_false_fallthrough_count\s*\.checked_add\(\s*state\.capacity_limit_sensible_output_maximum_capacity_assignment_count' -Description "CP341 A=F+M partition"
Assert-Contains -Path $cp341RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_maximum_capacity_assignment_count\s*\.checked_mul\(2\)' -Description "CP341 checked 2*M source formula"
Assert-Contains -Path $cp341RuntimeValidation -Pattern '(?s)route_partition == state\.transition_count.*?source_site_execution_count == expected_source_sites.*?maximum_total_cooling_capacity_read_count == assigned.*?cooling_sensible_output_assignment_write_count == assigned' -Description "CP341 completed T/source/site identities"
Assert-Contains -Path $cp341RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_maximum_capacity_assignment_count\s*==\s*predecessor\s*\.capacity_limit_sensible_output_adjustment_body_entry_count' -Description "CP341 M equals CP340 body entries"
Assert-Contains -Path $cp341RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_guard_false_fallthrough_count\s*\.checked_add\(\s*state\.capacity_limit_sensible_output_maximum_capacity_assignment_count,\s*\)\s*== Some\(predecessor\.capacity_limit_sensible_output_guard_evaluation_count\)' -Description "CP341 F+M equals CP340 active A"
foreach ($cp341PreflightCounter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'capacity_limit_guard_false_fallthrough_skip_count',
        'capacity_limit_sensible_output_guard_false_fallthrough_count',
        'capacity_limit_sensible_output_maximum_capacity_assignment_count',
        'source_site_execution_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_sensible_output_assignment_write_count',
        'witnessed_capacity_limit_sensible_output_maximum_capacity_assignment_count'
    )) {
    Assert-Contains -Path $cp341RuntimeValidation -Pattern ($cp341PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP341 checked preflight '$cp341PreflightCounter'"
}
Assert-Contains -Path $cp341SnapshotValidation -Pattern '(?s)fn false_fallthrough_snapshot_is_exact\(.*?preexisting\.to_bits\(\) == resulting\.to_bits\(\)' -Description "CP341 false route preserves result bits"
Assert-Contains -Path $cp341SnapshotValidation -Pattern '(?s)fn assigned_snapshot_is_exact\(.*?maximum\.to_bits\(\) == assigned\.to_bits\(\).*?assigned\.to_bits\(\) == resulting\.to_bits\(\)' -Description "CP341 true route assigns retained maximum bits"
Assert-Contains -Path $cp341SnapshotValidation -Pattern '(?s)fn skipped_snapshot_is_exact\(.*?preexisting_cooling_sensible_output_w\.is_none\(\).*?maximum_total_cooling_capacity_w\.is_none\(\).*?resulting_cooling_sensible_output_w\.is_none\(\)' -Description "CP341 complete-null inherited skip firewall"

# Public API takes only runtime/system/CP340. The supplied snapshot is
# admission evidence; actual values come from retained CP340 latest/private
# evidence, with full recursive validation and finite strictly-positive max.
Assert-Contains -Path $cp341Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp340:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,\s*\)' -Description "CP341 exact public arguments"
Assert-Contains -Path $cp341Release -Pattern 'unit\s*\.calc_cooling_positive_supply_capacity_limit_sensible_output_guard\s*\.latest' -Description "CP341 retained CP340 latest predecessor"
Assert-Contains -Path $cp341Release -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness\s*\(' -Description "CP341 CP340 private witness"
Assert-Contains -Path $cp341Release -Pattern 'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent\s*\(' -Description "CP341 recursive CP340 proof"
Assert-Contains -Path $cp341PrefixValidation -Pattern '(?s)sensible_output_guard_snapshots_match_bit_exact.*?cooling_sensible_output_w.*?maximum_total_cooling_capacity_w.*?to_bits\(\)' -Description "CP341 bit-exact supplied/latest/private parity"
Assert-Contains -Path $cp341PrefixValidation -Pattern 'maximum\.is_finite\(\) && maximum > 0\.0' -Description "CP341 public active finite strictly-positive maximum"
Assert-Contains -Path $cp341PrefixValidation -Pattern '(?s)predecessor_output\.to_bits\(\) != preexisting\.to_bits\(\).*?predecessor_maximum\.to_bits\(\) == maximum\.to_bits\(\).*?maximum\.to_bits\(\) == assigned_value\.to_bits\(\).*?assigned_value\.to_bits\(\) == result\.to_bits\(\)' -Description "CP341 retained CP340 values own false/true results"
Assert-NotContains -Path $cp341Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^)]*(cooling_sensible_output|maximum_total_cooling_capacity)\s*:' -Description "CP341 duplicate caller operands"
Assert-NotContains -Path $cp341Release -Pattern 'calc_cooling_capacity_zero_flow_reset|cooling_capacity_zero_flow_reset|AutosizeOrNumber|sized_|latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|DirectZonePurchasedAirCouplingInput|ZoneHeatBalanceState|zone_state|system\.maximum_total_cooling_capacity_w' -Description "CP341 CP321/model/sizing/service/numerical exclusions"
Assert-PatternsInOrder -Path $cp341Release -Patterns @(
    'pending_maximum_capacity_assignment_state_is_consistent\(',
    'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent\(',
    'call_order_is_pending\(',
    'next_maximum_capacity_assignment_transition_fits\(',
    'runtime\.units\.get_mut',
    'advance_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_state\([^;]+?retained_predecessor',
    'set_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness\('
) -Description "CP341 validate-before-mutation and retained-predecessor execution order"
Assert-Contains -Path $cp341PublicReleaseTests -Pattern 'supplied_public_and_private_cp340_drift_are_transactional' -Description "CP341 CP340 public/private drift rejection"
Assert-Contains -Path $cp341PublicReleaseTests -Pattern 'forged_nonpositive_and_nonfinite_active_capacities_are_rejected' -Description "CP341 public active capacity domain rejection"
Assert-Contains -Path $cp341PublicReleaseTests -Pattern 'recursive_cp339_private_witness_corruption_is_rejected_before_mutation' -Description "CP341 recursive lineage corruption rejection"
Assert-Contains -Path $cp341PublicReleaseTests -Pattern 'every_assignment_counter_increment_is_preflighted_transactionally' -Description "CP341 transactional overflow regression"

# CP341 state and latest witness remain private and selected-system rooted.
Assert-Contains -Path $cp341InitState -Pattern '(?s)cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot' -Description "runtime-root private CP341 witness map"
Assert-NotContains -Path $cp341InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witnesses:' -Description "CP341 witness map remains private"
Assert-Contains -Path $cp341InitWitnessRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;' -Description "CP341 witness module"
Assert-Contains -Path $cp341InitWitness -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness\s*\(' -Description "CP341 private witness getter"
Assert-Contains -Path $cp341InitWitness -Pattern 'set_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness\s*\(' -Description "CP341 private witness setter"
Assert-Contains -Path $cp341InitState -Pattern 'pub calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment:' -Description "per-unit CP341 state"
Assert-Contains -Path $cp341InitUnit -Pattern '(?s)calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new\(\s*system' -Description "per-unit CP341 initialization"

# Binding is exactly CP340 -> CP341 -> unchanged numerical coupling.
$cp341BindingText = Read-RepoText -Path $cp341Binding
$cp340BindingCallForCp341 = [regex]::Match(
    $cp341BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;'
)
$cp341BindingCall = [regex]::Match(
    $cp341BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;'
)
$cp341NumericalIndex = $cp341BindingText.IndexOf('let coupling = complete_direct_zone_purchased_air_coupling(')
if (
    -not $cp340BindingCallForCp341.Success -or
    -not $cp341BindingCall.Success -or
    $cp341BindingCall.Index -lt ($cp340BindingCallForCp341.Index + $cp340BindingCallForCp341.Length) -or
    $cp341NumericalIndex -lt ($cp341BindingCall.Index + $cp341BindingCall.Length)
) {
    throw "Binding must complete CP340 then CP341 before unchanged numerical coupling"
}
foreach ($cp341BindingInterval in @(
        [PSCustomObject]@{
            Start = $cp340BindingCallForCp341.Index + $cp340BindingCallForCp341.Length
            End = $cp341BindingCall.Index
            Description = "after CP340 and before CP341"
        },
        [PSCustomObject]@{
            Start = $cp341BindingCall.Index + $cp341BindingCall.Length
            End = $cp341NumericalIndex
            Description = "after CP341 and before numerical coupling"
        }
    )) {
    $cp341BindingIntervalText = $cp341BindingText.Substring(
        $cp341BindingInterval.Start,
        $cp341BindingInterval.End - $cp341BindingInterval.Start
    )
    $cp341BindingIntervalCode =
        [regex]::Replace($cp341BindingIntervalText, '(?m)//.*$', '')
    $cp341BindingIntervalCode = [regex]::Replace(
        $cp341BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
        ''
    )
    $cp341BindingIntervalCode = [regex]::Replace(
        $cp341BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
    $cp341BindingIntervalCode = [regex]::Replace(
        $cp341BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    if ($cp341BindingIntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp341BindingInterval.Description)"
    }
}
Assert-Contains -Path $cp341Binding -Pattern '(?s)advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_guard,\s*\)\?;' -Description "binding exact CP340-to-CP341 call"
Assert-Contains -Path $cp341BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,\s*\)' -Description "CP341 binding adapter arguments"
Assert-NotContains -Path $cp341BindingAdapter -Pattern 'cooling_sensible_output\s*:|maximum_total_cooling_capacity\s*:|sized_|latest_numerical|complete_direct_zone_purchased_air_coupling' -Description "CP341 binding excludes duplicate operands"
Assert-Contains -Path $cp341ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment:' -Description "CP341 scheduled output"
Assert-Contains -Path $cp341BindingTestsRoot -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_tests\.rs' -Description "CP341 binding test module"

# Coupled runtime must reconstruct CP341 only from scheduled CP340 evidence.
Assert-Contains -Path $cp341CoupledRuntime -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_validation;' -Description "coupled CP341 validator"
Assert-Contains -Path $cp341CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle:' -Description "coupled CP341 lifecycle"
Assert-Contains -Path $cp341CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_validation::snapshot_matches_release' -Description "coupled CP341 snapshot validation"
Assert-Contains -Path $cp341CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_validation::validate_lifecycle' -Description "coupled CP341 final validation"
Assert-Contains -Path $cp341CoupledValidation -Pattern 'output\.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard' -Description "coupled CP340-only predecessor"
Assert-Contains -Path $cp341CoupledValidation -Pattern 'predecessor\.capacity_limit_sensible_output_adjustment_body_entry_count' -Description "coupled M equals CP340 body entries"
Assert-Contains -Path $cp341CoupledValidation -Pattern '(?s)capacity_limit_sensible_output_guard_false_fallthrough_count.*?capacity_limit_sensible_output_maximum_capacity_assignment_count.*?active_partition' -Description "coupled A=F+M"
Assert-Contains -Path $cp341CoupledValidation -Pattern '(?s)checked_mul\(\s*state\.capacity_limit_sensible_output_maximum_capacity_assignment_count,\s*2' -Description "coupled checked 2*M"
Assert-Contains -Path $cp341CoupledValidation -Pattern 'capacity_w\.is_finite\(\) && capacity_w > 0\.0' -Description "coupled public active capacity domain"
Assert-Contains -Path $cp341CoupledValidation -Pattern 'source_site_count_overflow_fails_closed' -Description "coupled CP341 overflow regression"
Assert-Contains -Path $cp341CoupledValidation -Pattern 'exact_bits_preserve_nan_and_distinguish_signed_zero' -Description "coupled CP341 IEEE regression"
Assert-Contains -Path $cp341CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_fixture;' -Description "coupled CP341 fixture module"
Assert-Contains -Path $cp341CoupledFixture -Pattern 'calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_snapshot' -Description "coupled CP341 fixture"
Assert-NotContains -Path $cp341CoupledValidation -Pattern 'calc_cooling_capacity_zero_flow_reset|calculation_cooling_capacity_zero_flow_reset|system\.maximum_total_cooling_capacity_w|AutosizeOrNumber|sized_|latest_numerical' -Description "coupled CP341 excludes CP321/model/sizing/numerical reads"

# Pipeline and direct-run assertions preserve direct-only lifecycle, source
# algebra, exact bits, JSON projection, and the unchanged numerical DTO.
Assert-Contains -Path $cp341PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment;' -Description "pipeline CP341 module"
Assert-Contains -Path $cp341PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle' -Description "pipeline CP341 lifecycle and JSON key"
Assert-Contains -Path $cp341Pipeline -Pattern 'predecessor_cp340' -Description "pipeline CP340-only predecessor"
Assert-Contains -Path $cp341Pipeline -Pattern 'capacity_limit_sensible_output_adjustment_body_entry_count' -Description "pipeline CP341 M/CP340-body parity"
Assert-Contains -Path $cp341PipelineValidation -Pattern 'checked_mul' -Description "pipeline CP341 checked 2*M"
Assert-Contains -Path $cp341PipelineValidation -Pattern 'source_counter_overflow_fails_closed' -Description "pipeline CP341 source-counter overflow regression"
Assert-Contains -Path $cp341PipelineValidation -Pattern 'exact_bits_preserve_nan_and_distinguish_signed_zero' -Description "pipeline CP341 exact IEEE regression"
Assert-Contains -Path $cp341PipelineValidation -Pattern 'forged_active_capacity_domain_is_rejected' -Description "pipeline CP341 public active-capacity firewall"
Assert-NotContains -Path $cp341Pipeline -Pattern 'calc_cooling_capacity_zero_flow_reset|cooling_capacity_zero_flow_reset|latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|sized_|system\.maximum_total_cooling_capacity_w' -Description "pipeline CP341 excludes CP321/model/sizing/numerical operands"
foreach ($cp341JsonField in @(
        'preexisting_cooling_sensible_output_w',
        'preexisting_cooling_sensible_output_w_ieee_bits',
        'maximum_total_cooling_capacity_w',
        'maximum_total_cooling_capacity_w_ieee_bits',
        'assigned_cooling_sensible_output_w',
        'assigned_cooling_sensible_output_w_ieee_bits',
        'resulting_cooling_sensible_output_w',
        'resulting_cooling_sensible_output_w_ieee_bits'
    )) {
    Assert-Contains -Path $cp341PipelineSnapshotSerialization -Pattern ('"' + $cp341JsonField + '"') -Description "pipeline CP341 JSON field '$cp341JsonField'"
}
Assert-Contains -Path $cp341PipelineSnapshotSerialization -Pattern '(?s)fn json_number\(value: Option<f64>\) -> Value.*?filter\(\|value\| value\.is_finite\(\)\).*?map_or\(Value::Null' -Description "CP341 nonfinite numeric null projection"
Assert-Contains -Path $cp341PipelineSnapshotSerialization -Pattern 'value\.map\(\|value\| format!\("0x\{:016x\}", value\.to_bits\(\)\)\)' -Description "CP341 authoritative IEEE bits"
Assert-Contains -Path $cp341PipelineSnapshotSerialization -Pattern 'true_assignment_serializes_nonfinite_predecessor_and_finite_rhs_assigned_result' -Description "CP341 true +infinity-to-finite JSON regression"
Assert-Contains -Path $cp341PipelineSnapshotSerialization -Pattern 'false_guard_serializes_nan_predecessor_and_result_with_null_rhs_and_assigned' -Description "CP341 false NaN-preservation JSON regression"
Assert-Contains -Path $cp341PipelineSnapshotSerialization -Pattern 'inherited_skip_serializes_every_optional_value_and_bits_as_null' -Description "CP341 inherited-skip JSON null regression"
Assert-Contains -Path $cp341CoupledRuntimeTests -Pattern 'cp344_direct_coupled_runtime_accepts_true_false_and_inherited_skip_routes' -Description "cumulative direct coupled CP341-through-CP344 true/false/skip regression"
Assert-Contains -Path $cp341CoupledRuntimeTests -Pattern 'calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle' -Description "direct coupled CP340 predecessor evidence"
Assert-Contains -Path $cp341CoupledRuntimeTests -Pattern 'calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle' -Description "direct coupled CP341 lifecycle evidence"
Assert-Contains -Path $cp341PipelineRoot -Pattern '(?s)purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle:\s*None' -Description "non-direct CP341 null evidence"
Assert-Contains -Path $cp341PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp344_lifecycle_evidence' -Description "non-direct CP341 through CP344 evidence rejection"

# Exactly two algorithm addenda, two capability addenda, and six targets
# distributed 2+4 across the two parent algorithms.
$cp341AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp341AlgorithmAddenda = [regex]::Matches(
    $cp341AlgorithmText,
    '(?m)^\s*"CP341 supersedes only CP340[^"\r\n]+",\s*$'
)
if ($cp341AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP341 claim addenda"
}
$cp341TargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\.rs::purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp341Target in $cp341TargetCounts) {
    $cp341TargetCount = [regex]::Matches($cp341AlgorithmText, $cp341Target.Pattern).Count
    if ($cp341TargetCount -ne $cp341Target.Expected) {
        throw "CP341 target '$($cp341Target.Pattern)' expected $($cp341Target.Expected), found $cp341TargetCount"
    }
}
$cp341CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp341CapabilityAddenda = [regex]::Matches(
    $cp341CapabilityText,
    '(?m)^\s*"CP341 additionally requires[^"\r\n]+",\s*$'
)
if ($cp341CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP341 claim addenda"
}
foreach ($cp341Claim in @($cp341AlgorithmAddenda) + @($cp341CapabilityAddenda)) {
    foreach ($cp341Pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            $cp341SourceStatementPattern,
            $cp341OrderedSourceSitesPattern,
            'right-hand-side-read-to-left-hand-side-write',
            'C\+\+ built-in `=` evaluation order',
            'CapacityLimitSensibleOutputAdjustmentBodyEntered',
            'CapacityLimitSensibleOutputMaximumCapacityAssigned',
            'T=U\+N\+P\+G\+F\+M',
            'A=F\+M',
            'M=CP340 adjustment-body entries',
            'source_site_execution_count=2\*M',
            'No invariant equates `M` with `A`',
            'CP321 aggregate maximum-capacity reads',
            'retained same-call CP340 latest snapshot',
            'supplied predecessor and private witness',
            'false CP340 route preserves predecessor output bits without an RHS read or LHS write',
            'true route reads the retained maximum and assigns that exact binary64 payload',
            'finite and strictly greater than zero',
            'Arbitrary IEEE payload copying.*?characterization only',
            'false-route NaN',
            '\+infinity',
            'serde JSON',
            'IEEE bit strings',
            'CP340-to-CP341-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle',
            $cp341FirstExcludedStatementPattern,
            'line 2208',
            'Roadmap promotion'
        )) {
        if ($cp341Claim.Value -notmatch $cp341Pattern) {
            throw "CP341 spec addendum missing '$cp341Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP341 supersedes only CP340' -Description "generated CP341 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP341 additionally requires' -Description "generated CP341 capability index"

# Five hand-authored contracts repeat the same source, lineage, IEEE, JSON,
# numerical-firewall, exclusion, and no-promotion boundary.
$cp341DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP341 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP341 Source-Ordered Cooling Positive-Supply Maximum-Capacity Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP341 Cooling Positive-Supply Maximum-Capacity Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP341 Positive-Supply Maximum-Capacity Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP341 Cooling Positive-Supply Maximum-Capacity Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp341Documentation in $cp341DocumentationSections) {
    $cp341DocumentText = Read-RepoText -Path $cp341Documentation.Path
    $cp341Matches = [regex]::Matches($cp341DocumentText, $cp341Documentation.Pattern)
    if ($cp341Matches.Count -ne 1) {
        throw "CP341 documentation expected one scoped section in $($cp341Documentation.Path), found $($cp341Matches.Count)"
    }
    $cp341Section = $cp341Matches[0].Value
    foreach ($cp341Pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            $cp341SourceStatementPattern,
            $cp341OrderedSourceSitesPattern,
            '(?s)Rust.*?(?:right-hand-side.*?left-hand-side|RHS-read-to-LHS-write)',
            'C\+\+\s+built-in `=` evaluation',
            'CapacityLimitSensibleOutputAdjustmentBodyEntered',
            'UnitOff',
            'NonCooling',
            'PositiveGuardFalseFallthrough',
            'ActiveCapacityLimitGuardFalseFallthrough',
            'CapacityLimitSensibleOutputGuardFalseFallthrough',
            'CapacityLimitSensibleOutputMaximumCapacityAssigned',
            'T\s*=\s*U\+N\+P\+G\+F\+M',
            'A\s*=\s*F\+M',
            'M\s*=\s*CP340 adjustment-body entries',
            '2\*M',
            '(?s)(?:No invariant|Neither.*?required\s+invariant|no required\s+equality)',
            '(?s)retained\s+same-call CP340 latest\s+snapshot.*?private witness',
            '(?i)CP321',
            '(?s)false.*?without an RHS read or LHS\s+write',
            '(?s)true route.*?retained[- ]maximum',
            '(?s)finite.*?strictly\s+(?:positive|greater than zero)',
            'arbitrary IEEE',
            'characterization only',
            'NaN',
            '\+infinity',
            'Some\(value\)',
            'Serde\s+JSON',
            'IEEE bit strings',
            'CP340-to-CP341-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle',
            'None',
            $cp341FirstExcludedStatementPattern,
            'line 2208',
            '(?i)numerical[- ]DTO',
            'Roadmap'
        )) {
        if ($cp341Section -notmatch $cp341Pattern) {
            throw "CP341 documentation in $($cp341Documentation.Path) missing '$cp341Pattern'"
        }
    }
}

# Root reachability and generated inventory add exactly one internal script:
# 282 executable records, 240 public, 42 internal, and zero uncalled.
$cp341MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp340DotSourceIndexForCp341 = $cp341MainAuditText.IndexOf('ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1')
$cp341DotSourceIndex = $cp341MainAuditText.IndexOf('ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1')
$cp341AuditCompletionIndex = $cp341MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp340DotSourceIndexForCp341 -lt 0 -or
    $cp341DotSourceIndex -le $cp340DotSourceIndexForCp341 -or
    $cp341AuditCompletionIndex -le $cp341DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP341 after CP340 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 282' -Description "CP341 cumulative inventory total through CP344"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment\.ps1"' -Description "CP341 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment\.ps1::dot_sources' -Description "CP341 main-audit callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 282 \|' -Description "CP341 generated script count through CP344"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP341 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 42 \|' -Description "CP341 generated internal script count through CP344"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP341 generated uncalled script count"
