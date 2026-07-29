# CP340 maps only PurchasedAirManager.cc physical executable line 2198:
# if (CoolSensOutput >= PurchAir.MaxCoolTotCap) {
# Physical line 2199 is the first excluded executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp340Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard.rs"
$cp340State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard\state.rs"
$cp340Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard\transition.rs"
$cp340Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard\release.rs"
$cp340PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard\release\prefix_validation.rs"
$cp340RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard\release\runtime_validation.rs"
$cp340SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard\release\snapshot_validation.rs"
$cp340Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard\tests\mod.rs"
$cp340CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp340Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp340ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp340BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_capacity_limit_sensible_output_guard.rs"
$cp340BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp340BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_capacity_limit_sensible_output_guard_tests.rs"
$cp340InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp340InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp340InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp340InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_capacity_limit_sensible_output_guard.rs"
$cp340CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp340CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_capacity_limit_sensible_output_guard_validation.rs"
$cp340CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp340CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_capacity_limit_sensible_output_guard_fixture.rs"
$cp340PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp340Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_sensible_output_guard.rs"
$cp340PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_sensible_output_guard\validation.rs"
$cp340PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_sensible_output_guard\serialization.rs"
$cp340PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_sensible_output_guard\serialization\snapshot.rs"
$cp340RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp340DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_capacity_limit_sensible_output_guard_assertions.rs"
$cp340NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp340RequiredFile in @(
        $cp340Module,
        $cp340State,
        $cp340Transition,
        $cp340Release,
        $cp340PrefixValidation,
        $cp340RuntimeValidation,
        $cp340SnapshotValidation,
        $cp340Tests,
        $cp340BindingAdapter,
        $cp340BindingTests,
        $cp340InitWitness,
        $cp340CoupledValidation,
        $cp340CoupledFixture,
        $cp340Pipeline,
        $cp340PipelineValidation,
        $cp340PipelineSerialization,
        $cp340PipelineSnapshotSerialization,
        $cp340DirectAssertions
    )) {
    Assert-FileExists -Path $cp340RequiredFile -Description "CP340 sensible-output guard structure"
}
Assert-LineLimit -Path $cp340Release -Limit 800 -Description "CP340 release module"
Assert-LineLimit -Path $cp340CoupledValidation -Limit 800 -Description "CP340 coupled validation module"
Assert-LineLimit -Path $cp340Pipeline -Limit 800 -Description "CP340 pipeline module"

$cp340SourceStatementPattern =
    'if\s*\(\s*CoolSensOutput\s*>=\s*PurchAir\.MaxCoolTotCap\s*\)\s*\{'
$cp340FirstExcludedStatementPattern =
    'CoolSensOutput\s*=\s*PurchAir\.MaxCoolTotCap\s*;'
$cp340OrderedSourceSitesPattern =
    '(?s)read-retained-cooling-sensible-output-for-maximum-capacity-comparison.*?read-retained-maximum-total-cooling-capacity-for-sensible-output-comparison.*?compare-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity.*?enter-cooling-capacity-adjustment-body-if-comparison-satisfied'

# Locked line boundary and exact four-site order.
Assert-Contains -Path $cp340Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2198' -Description "CP340 exact physical source boundary"
Assert-Contains -Path $cp340Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2199' -Description "CP340 first excluded physical executable"
Assert-Contains -Path $cp340Module -Pattern 'Exact four textual source sites represented by CP340' -Description "CP340 source-site count"
Assert-ExactStringArray -Path $cp340Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER" -Expected @(
    "read-retained-cooling-sensible-output-for-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-sensible-output-comparison",
    "compare-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
    "enter-cooling-capacity-adjustment-body-if-comparison-satisfied"
) -Description "CP340 deterministic source-site order"
Assert-Contains -Path $cp340Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot' -Description "CP340 public snapshot"
Assert-Contains -Path $cp340State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState' -Description "CP340 persistent state"
Assert-Contains -Path $cp340Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary' -Description "CP340 lifecycle summary"
Assert-Contains -Path $cp340Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary\s*\(' -Description "CP340 lifecycle accessor"
Assert-Contains -Path $cp340Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard\s*\(' -Description "CP340 exact-direct release"
Assert-Contains -Path $cp340Transition -Pattern 'advance_cooling_positive_supply_capacity_limit_sensible_output_guard_state\s*\(' -Description "CP340 pure transition"
Assert-Contains -Path $cp340CalcRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_guard;' -Description "CP340 Calc module declaration"

foreach ($cp340Route in @(
        'UnitOff',
        'NonCooling',
        'PositiveGuardFalseFallthrough',
        'ActiveCapacityLimitGuardFalseFallthrough',
        'CapacityLimitSensibleOutputGuardFalseFallthrough',
        'CapacityLimitSensibleOutputAdjustmentBodyEntered'
    )) {
    Assert-Contains -Path $cp340State -Pattern $cp340Route -Description "CP340 retained route '$cp340Route'"
}
foreach ($cp340Counter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'capacity_limit_guard_false_fallthrough_skip_count',
        'capacity_limit_sensible_output_guard_evaluation_count',
        'source_site_execution_count',
        'cooling_sensible_output_read_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_sensible_output_maximum_capacity_comparison_count',
        'capacity_limit_sensible_output_guard_false_fallthrough_count',
        'capacity_limit_sensible_output_adjustment_body_entry_count'
    )) {
    Assert-Contains -Path $cp340State -Pattern ('pub ' + $cp340Counter + ':\s*usize') -Description "CP340 state counter '$cp340Counter'"
}

# Pure transition performs raw >= once and counts 3*A+E.
Assert-PatternsInOrder -Path $cp340Transition -Patterns @(
    'let guard_evaluated =\s*predecessor\.capacity_limit_sensible_output_assignment_executed;',
    'input\.cooling_sensible_output_w\s*>=\s*input\.maximum_total_cooling_capacity_w',
    'let body_entered = at_or_above_maximum == Some\(true\);',
    'let false_fallthrough = at_or_above_maximum == Some\(false\);',
    'state\.capacity_limit_sensible_output_guard_evaluation_count \+= 1;',
    'state\.source_site_execution_count \+= 3 \+ usize::from\(body_entered\);',
    'state\.cooling_sensible_output_read_count \+= 1;',
    'state\.maximum_total_cooling_capacity_read_count \+= 1;',
    'state\.cooling_sensible_output_maximum_capacity_comparison_count \+= 1;'
) -Description "CP340 raw comparison and source-counter order"
Assert-NotContains -Path $cp340Transition -Pattern 'total_cmp|partial_cmp|clamp|is_finite' -Description "CP340 transition excludes ordering helpers and finite coercion"
Assert-Contains -Path $cp340Tests -Pattern 'source_boundary_and_exact_four_sites_are_stable' -Description "CP340 source-boundary regression"
Assert-Contains -Path $cp340Tests -Pattern 'pure_transition_preserves_raw_ieee_greater_than_or_equal_behavior' -Description "CP340 NaN/infinity comparison regression"
Assert-Contains -Path $cp340Tests -Pattern 'all_four_inherited_skips_execute_no_cp340_sites' -Description "CP340 inherited-skip regression"
Assert-Contains -Path $cp340Tests -Pattern 'counters_partition_six_routes_and_apply_three_a_plus_e_identity' -Description "CP340 route and 3*A+E regression"
Assert-Contains -Path $cp340Tests -Pattern 'exact_predicate_and_matcher_reject_forgery_but_retain_nan_bits' -Description "CP340 bit-exact forgery regression"

# Completed and pending state enforce T partition, A=F+E, 3*A+E, predecessor
# parity, CP338/CP337 lineage, witnessed parity, and checked preflight.
Assert-Contains -Path $cp340RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_guard_false_fallthrough_count\s*\.checked_add\(state\.capacity_limit_sensible_output_adjustment_body_entry_count\)' -Description "CP340 active F+E partition"
Assert-Contains -Path $cp340RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_guard_evaluation_count\s*\.checked_mul\(3\).*?checked_add\(\s*state\.capacity_limit_sensible_output_adjustment_body_entry_count' -Description "CP340 checked 3*A+E formula"
Assert-Contains -Path $cp340RuntimeValidation -Pattern '(?s)route_partition == state\.transition_count.*?active_partition == active.*?source_site_execution_count == expected_source_sites' -Description "CP340 completed T/A/source identities"
Assert-Contains -Path $cp340RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_guard_evaluation_count\s*== cp338\.capacity_limit_cp_air_assignment_count.*?capacity_limit_sensible_output_guard_evaluation_count\s*== cp337\.capacity_limit_body_entry_count' -Description "CP340 CP338/CP337 active parity"
Assert-Contains -Path $cp340RuntimeValidation -Pattern '(?s)witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count.*?== state\.capacity_limit_sensible_output_guard_false_fallthrough_count.*?witnessed_capacity_limit_sensible_output_adjustment_body_entry_count.*?== state\.capacity_limit_sensible_output_adjustment_body_entry_count' -Description "CP340 witnessed F/E parity"
foreach ($cp340PreflightCounter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'capacity_limit_guard_false_fallthrough_skip_count',
        'capacity_limit_sensible_output_guard_evaluation_count',
        'source_site_execution_count',
        'cooling_sensible_output_read_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_sensible_output_maximum_capacity_comparison_count',
        'capacity_limit_sensible_output_guard_false_fallthrough_count',
        'capacity_limit_sensible_output_adjustment_body_entry_count',
        'witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count',
        'witnessed_capacity_limit_sensible_output_adjustment_body_entry_count'
    )) {
    Assert-Contains -Path $cp340RuntimeValidation -Pattern ($cp340PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP340 checked preflight '$cp340PreflightCounter'"
}
Assert-Contains -Path $cp340SnapshotValidation -Pattern '(?s)let satisfied = output >= maximum;.*?maximum\.is_finite\(\).*?maximum >= 0\.0.*?cooling_sensible_output_at_or_above_maximum_capacity == Some\(satisfied\)' -Description "CP340 exact raw active snapshot"
Assert-Contains -Path $cp340SnapshotValidation -Pattern '(?s)fn skipped_snapshot_is_exact\(.*?cooling_sensible_output_w\.is_none\(\).*?maximum_total_cooling_capacity_w\.is_none\(\).*?cooling_sensible_output_at_or_above_maximum_capacity.*?\.is_none\(\)' -Description "CP340 complete-null skip firewall"

# Public API takes only runtime/system/CP339; active operands come from exact
# CP339 and CP321 latest/private witnesses, never caller/model/sizing services.
Assert-Contains -Path $cp340Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp339:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,\s*\)' -Description "CP340 exact public arguments"
Assert-Contains -Path $cp340Release -Pattern 'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent\s*\(' -Description "CP340 recursive CP339 proof"
Assert-Contains -Path $cp340Release -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness\s*\(' -Description "CP340 CP339 private witness"
Assert-Contains -Path $cp340Release -Pattern 'unit\.calc_cooling_capacity_zero_flow_reset\.latest' -Description "CP340 retained CP321 operand"
Assert-Contains -Path $cp340Release -Pattern 'cooling_capacity_zero_flow_reset_latest_witness\s*\(' -Description "CP340 CP321 private witness"
Assert-Contains -Path $cp340PrefixValidation -Pattern '(?s)predecessor\s*\.cooling_sensible_output_w.*?to_bits\(\) == cooling_sensible_output_w\.to_bits\(\).*?capacity_reset\s*\.maximum_total_cooling_capacity_w.*?to_bits\(\) == maximum_total_cooling_capacity_w\.to_bits\(\).*?maximum_total_cooling_capacity_w\.is_finite\(\).*?maximum_total_cooling_capacity_w >= 0\.0' -Description "CP340 bit-exact CP339/CP321 operands"
Assert-Contains -Path $cp340PrefixValidation -Pattern 'capacity_reset_snapshots_match_bit_exact\(capacity_reset, capacity_reset_witness\)' -Description "CP340 CP321 public/private parity"
Assert-NotContains -Path $cp340Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard\([^)]*(cooling_sensible_output|maximum_total_cooling_capacity)\s*:' -Description "CP340 duplicate caller operands"
Assert-NotContains -Path $cp340Release -Pattern 'AutosizeOrNumber|sized_|latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|DirectZonePurchasedAirCouplingInput|ZoneHeatBalanceState|zone_state' -Description "CP340 model, sized-limit, service, and numerical exclusions"
Assert-PatternsInOrder -Path $cp340Release -Patterns @(
    'pending_capacity_limit_sensible_output_guard_state_is_consistent\(',
    'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent\(',
    'active_operands_link_to_retained_prefix\(',
    'next_capacity_limit_sensible_output_guard_transition_fits\(',
    'runtime\.units\.get_mut',
    'advance_cooling_positive_supply_capacity_limit_sensible_output_guard_state\(',
    'set_cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness\('
) -Description "CP340 validate-before-mutation order"

# CP340 state and private latest witness are rooted per selected system.
Assert-Contains -Path $cp340InitState -Pattern '(?s)cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot' -Description "runtime-root private CP340 witness map"
Assert-NotContains -Path $cp340InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witnesses:' -Description "CP340 witness map remains private"
Assert-Contains -Path $cp340InitWitnessRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_guard;' -Description "CP340 witness module"
Assert-Contains -Path $cp340InitWitness -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness\s*\(' -Description "CP340 private witness getter"
Assert-Contains -Path $cp340InitWitness -Pattern 'set_cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness\s*\(' -Description "CP340 private witness setter"
Assert-Contains -Path $cp340InitState -Pattern 'pub calc_cooling_positive_supply_capacity_limit_sensible_output_guard:' -Description "per-unit CP340 state"
Assert-Contains -Path $cp340InitUnit -Pattern '(?s)calc_cooling_positive_supply_capacity_limit_sensible_output_guard:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState::new\(\s*system' -Description "per-unit CP340 initialization"

# Binding is exact CP339 -> CP340 -> unchanged numerical coupling.
$cp340BindingText = Read-RepoText -Path $cp340Binding
$cp339BindingCallForCp340 = [regex]::Match(
    $cp340BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
$cp340BindingCall = [regex]::Match(
    $cp340BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;'
)
$cp340NumericalIndex = $cp340BindingText.IndexOf('let coupling = complete_direct_zone_purchased_air_coupling(')
if (
    -not $cp339BindingCallForCp340.Success -or
    -not $cp340BindingCall.Success -or
    $cp340BindingCall.Index -lt ($cp339BindingCallForCp340.Index + $cp339BindingCallForCp340.Length) -or
    $cp340NumericalIndex -lt ($cp340BindingCall.Index + $cp340BindingCall.Length)
) {
    throw "Binding must complete CP339 then CP340 before unchanged numerical coupling"
}
foreach ($cp340BindingInterval in @(
        [PSCustomObject]@{
            Start = $cp339BindingCallForCp340.Index + $cp339BindingCallForCp340.Length
            End = $cp340BindingCall.Index
            Description = "after CP339 and before CP340"
        },
        [PSCustomObject]@{
            Start = $cp340BindingCall.Index + $cp340BindingCall.Length
            End = $cp340NumericalIndex
            Description = "after CP340 and before numerical coupling"
        }
    )) {
    $cp340BindingIntervalText = $cp340BindingText.Substring(
        $cp340BindingInterval.Start,
        $cp340BindingInterval.End - $cp340BindingInterval.Start
    )
    $cp340BindingIntervalCode =
        [regex]::Replace($cp340BindingIntervalText, '(?m)//.*$', '')
    $cp340BindingIntervalCode = [regex]::Replace(
        $cp340BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
    $cp340BindingIntervalCode = [regex]::Replace(
        $cp340BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
        ''
    )
    $cp340BindingIntervalCode = [regex]::Replace(
        $cp340BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
    $cp340BindingIntervalCode = [regex]::Replace(
        $cp340BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    $cp340BindingIntervalCode = [regex]::Replace(
        $cp340BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp340BindingIntervalCode = [regex]::Replace(
        $cp340BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp340BindingIntervalCode = [regex]::Replace(
        $cp340BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp340BindingIntervalCode = [regex]::Replace(
    $cp340BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp340BindingIntervalCode = [regex]::Replace(
    $cp340BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp340BindingIntervalCode = [regex]::Replace(
    $cp340BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp340BindingIntervalCode = [regex]::Replace(
    $cp340BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp340BindingIntervalCode = [regex]::Replace(
    $cp340BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
    if ($cp340BindingIntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp340BindingInterval.Description)"
    }
}
Assert-Contains -Path $cp340Binding -Pattern '(?s)advance_positive_supply_capacity_limit_sensible_output_guard\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment,\s*\)\?;' -Description "binding exact CP339-to-CP340 call"
Assert-Contains -Path $cp340BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_capacity_limit_sensible_output_guard\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,\s*\)' -Description "CP340 binding adapter arguments"
Assert-NotContains -Path $cp340BindingAdapter -Pattern 'cooling_sensible_output\s*:|maximum_total_cooling_capacity\s*:|sized_|latest_numerical|complete_direct_zone_purchased_air_coupling' -Description "CP340 binding excludes duplicate operands"
Assert-Contains -Path $cp340ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_capacity_limit_sensible_output_guard:' -Description "CP340 scheduled output"
Assert-Contains -Path $cp340BindingTestsRoot -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_guard_tests\.rs' -Description "CP340 binding test module"
Assert-Contains -Path $cp340BindingTests -Pattern 'scheduled_binding_preserves_exact_operands_and_both_comparison_routes' -Description "CP340 active binding routes"
Assert-Contains -Path $cp340BindingTests -Pattern 'scheduled_binding_preserves_all_complete_null_skip_routes' -Description "CP340 binding null routes"

# Coupled runtime and pipeline independently validate partition, operands,
# exact bits, JSON projection, direct-only lifecycle, and source order.
Assert-Contains -Path $cp340CoupledRuntime -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_guard_validation;' -Description "coupled CP340 validator"
Assert-Contains -Path $cp340CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle:' -Description "coupled CP340 lifecycle"
Assert-Contains -Path $cp340CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_guard_validation::snapshot_matches_release' -Description "coupled CP340 snapshot validation"
Assert-Contains -Path $cp340CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_guard_validation::validate_lifecycle' -Description "coupled CP340 final validation"
Assert-Contains -Path $cp340CoupledValidation -Pattern 'output\.calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment' -Description "coupled CP339 operand"
Assert-Contains -Path $cp340CoupledValidation -Pattern 'output\.calculation_cooling_capacity_zero_flow_reset' -Description "coupled CP321 operand"
Assert-Contains -Path $cp340CoupledValidation -Pattern 'cooling_sensible_output_w >= maximum_total_cooling_capacity_w' -Description "coupled raw >= reconstruction"
Assert-Contains -Path $cp340CoupledValidation -Pattern '(?s)checked_mul\(\s*state\.capacity_limit_sensible_output_guard_evaluation_count,\s*3' -Description "coupled checked 3*A"
Assert-Contains -Path $cp340CoupledValidation -Pattern 'source_site_count_overflow_fails_closed' -Description "coupled CP340 overflow regression"
Assert-Contains -Path $cp340CoupledValidation -Pattern 'exact_bits_distinguish_signed_zero_capacity' -Description "coupled signed-zero bit regression"
Assert-Contains -Path $cp340CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_guard_fixture;' -Description "coupled CP340 fixture module"
Assert-Contains -Path $cp340CoupledFixture -Pattern 'calculation_cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot' -Description "coupled CP340 fixture"

Assert-Contains -Path $cp340PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_capacity_limit_sensible_output_guard;' -Description "pipeline CP340 module"
Assert-Contains -Path $cp340PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle' -Description "pipeline CP340 lifecycle and JSON"
Assert-Contains -Path $cp340Pipeline -Pattern '(?s)validate_direct_lifecycle\(.*?predecessor_cp339:.*?capacity_cp321:.*?init_lifecycle:.*?coupling_call_count' -Description "pipeline CP340 validated inputs"
Assert-Contains -Path $cp340Pipeline -Pattern 'predecessor_state\.capacity_limit_sensible_output_assignment_count' -Description "pipeline CP339 active parity"
Assert-Contains -Path $cp340Pipeline -Pattern 'capacity_state\.latest\.as_ref\(\)' -Description "pipeline retained CP321 latest snapshot"
Assert-NotContains -Path $cp340Pipeline -Pattern 'capacity_state\.maximum_total_cooling_capacity_read_count' -Description "pipeline does not require false CP321 aggregate-read equality"
Assert-Contains -Path $cp340PipelineValidation -Pattern '(?s)evaluations\s*\.checked_mul\(3\).*?capacity_limit_sensible_output_adjustment_body_entry_count' -Description "pipeline checked 3*A+E formula"
Assert-Contains -Path $cp340PipelineValidation -Pattern 'source_counter_overflow_fails_closed' -Description "pipeline CP340 source overflow regression"
Assert-Contains -Path $cp340PipelineValidation -Pattern 'exact_bits_distinguish_signed_zero_capacity' -Description "pipeline signed-zero bit regression"
foreach ($cp340JsonField in @(
        'cooling_sensible_output_w',
        'cooling_sensible_output_w_ieee_bits',
        'maximum_total_cooling_capacity_w',
        'maximum_total_cooling_capacity_w_ieee_bits',
        'cooling_sensible_output_at_or_above_maximum_capacity'
    )) {
    Assert-Contains -Path $cp340PipelineSnapshotSerialization -Pattern ('"' + $cp340JsonField + '"') -Description "pipeline CP340 JSON field '$cp340JsonField'"
}
Assert-Contains -Path $cp340PipelineSnapshotSerialization -Pattern '(?s)fn json_number\(value: Option<f64>\) -> Value.*?filter\(\|value\| value\.is_finite\(\)\).*?map_or\(Value::Null' -Description "CP340 nonfinite numeric null projection"
Assert-Contains -Path $cp340PipelineSnapshotSerialization -Pattern 'value\.map\(\|value\| format!\("0x\{:016x\}", value\.to_bits\(\)\)\)' -Description "CP340 authoritative IEEE bits"
Assert-Contains -Path $cp340PipelineSnapshotSerialization -Pattern 'active_nonfinite_left_retains_bits_and_non_null_comparison' -Description "CP340 nonfinite JSON regression"
Assert-Contains -Path $cp340PipelineSnapshotSerialization -Pattern 'skipped_snapshot_serializes_optional_evidence_as_null' -Description "CP340 skip JSON regression"
Assert-Contains -Path $cp340RunTests -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_guard_assertions' -Description "direct-run CP340 assertion module"
Assert-Contains -Path $cp340DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*4\]' -Description "direct-run CP340 source order"
Assert-Contains -Path $cp340DirectAssertions -Pattern '3 \* evaluations \+ expected_adjustment_body_entries' -Description "direct-run CP340 3*A+E"
Assert-Contains -Path $cp340DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle' -Description "direct-run CP339 evidence"
Assert-Contains -Path $cp340DirectAssertions -Pattern 'purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle' -Description "direct-run CP321 evidence"
Assert-Contains -Path $cp340NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle' -Description "non-direct CP340 null evidence"
Assert-Contains -Path $cp340PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp352_lifecycle_evidence' -Description "non-direct CP340 through CP352 evidence rejection"
Assert-NotContains -Path $cp340Pipeline -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|sized_' -Description "CP340 pipeline excludes numerical and sized-limit operands"

# Exactly two algorithm addenda, two capability addenda, and six targets.
$cp340AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp340AlgorithmAddenda = [regex]::Matches(
    $cp340AlgorithmText,
    '(?m)^\s*"CP340 supersedes only CP339[^"\r\n]+",\s*$'
)
if ($cp340AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP340 claim addenda"
}
$cp340TargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_guard/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_guard\.rs::purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_guard\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_guard\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp340Target in $cp340TargetCounts) {
    $cp340TargetCount = [regex]::Matches($cp340AlgorithmText, $cp340Target.Pattern).Count
    if ($cp340TargetCount -ne $cp340Target.Expected) {
        throw "CP340 target '$($cp340Target.Pattern)' expected $($cp340Target.Expected), found $cp340TargetCount"
    }
}
$cp340CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp340CapabilityAddenda = [regex]::Matches(
    $cp340CapabilityText,
    '(?m)^\s*"CP340 additionally requires[^"\r\n]+",\s*$'
)
if ($cp340CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP340 claim addenda"
}
$cp340ObsoletePositiveDifferencePattern =
    '(?is)\+infinity`?\s+flow\s+times\s+(?:a\s+)?positive\s+(?:enthalpy\s+)?difference'
Assert-NotContains -Path "specs\algorithm_ledger.toml" -Pattern $cp340ObsoletePositiveDifferencePattern -Description "obsolete CP340 infinite-flow positive-difference reachability claim"
Assert-NotContains -Path "specs\capabilities.toml" -Pattern $cp340ObsoletePositiveDifferencePattern -Description "obsolete CP340 infinite-flow positive-difference capability claim"
foreach ($cp340Claim in @($cp340AlgorithmAddenda) + @($cp340CapabilityAddenda)) {
    foreach ($cp340Pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            $cp340SourceStatementPattern,
            $cp340OrderedSourceSitesPattern,
            'CapacityLimitSensibleOutputAssigned',
            'CapacityLimitSensibleOutputGuardFalseFallthrough',
            'CapacityLimitSensibleOutputAdjustmentBodyEntered',
            'T=U\+N\+P\+G\+F\+E=U\+N\+P\+G\+A',
            'A=F\+E=CP338 assignments=CP337 body entries',
            '3\*A\+E',
            'retained CP339 latest snapshot.*?supplied predecessor and private witness',
            'retained same-call CP321 latest snapshot.*?private-witness parity',
            'aggregate reads may exceed',
            'C\+\+ built-in.*?operand-evaluation-order claim',
            'raw IEEE',
            'NaN',
            '\+infinity',
            '-infinity',
            'zero reset',
            '(?i)public(?:ly reachable| active chain| reachability)',
            '(?s)\+infinity`?\s+flow.*?exact-zero\s+enthalpy\s+difference.*?CP339.*?NaN.*?false\s+route',
            '(?s)finite\s+positive\s+flow.*?finite\s+enthalpy\s+operands.*?subtraction\s+overflows\s+to\s+`\+infinity`.*?CP339.*?`\+infinity`.*?CP340.*?(?:true\s+route|enters\s+the\s+body)',
            'Some\(value\)',
            'serde JSON',
            'IEEE\s+bit\s+string',
            'CP339-to-CP340-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle',
            $cp340FirstExcludedStatementPattern,
            'line 2208',
            'Roadmap promotion'
        )) {
        if ($cp340Claim.Value -notmatch $cp340Pattern) {
            throw "CP340 spec addendum missing '$cp340Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP340 supersedes only CP339' -Description "generated CP340 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP340 additionally requires' -Description "generated CP340 capability index"
Assert-NotContains -Path "docs\src\generated\algorithm-ledger.md" -Pattern $cp340ObsoletePositiveDifferencePattern -Description "obsolete generated CP340 algorithm reachability claim"
Assert-NotContains -Path "docs\src\generated\capability-index.md" -Pattern $cp340ObsoletePositiveDifferencePattern -Description "obsolete generated CP340 capability reachability claim"

# Five hand-authored contracts repeat the scoped CP340 source/lineage boundary.
$cp340DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP340 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP340 Source-Ordered Cooling Positive-Supply Capacity-Limit Sensible-Output Guard\r?\n.*?Roadmap state remain unchanged\.\r?\n'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP340 Cooling Positive-Supply Capacity-Limit Sensible-Output Guard\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP340 Positive-Supply Capacity-Limit Sensible-Output Guard in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP340 Cooling Positive-Supply Capacity-Limit Sensible-Output Guard Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp340Documentation in $cp340DocumentationSections) {
    $cp340DocumentText = Read-RepoText -Path $cp340Documentation.Path
    $cp340Matches = [regex]::Matches($cp340DocumentText, $cp340Documentation.Pattern)
    if ($cp340Matches.Count -ne 1) {
        throw "CP340 documentation expected one scoped section in $($cp340Documentation.Path), found $($cp340Matches.Count)"
    }
    $cp340Section = $cp340Matches[0].Value
    foreach ($cp340Pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            $cp340SourceStatementPattern,
            $cp340OrderedSourceSitesPattern,
            'CapacityLimitSensibleOutputAssigned',
            'UnitOff',
            'NonCooling',
            'PositiveGuardFalseFallthrough',
            'ActiveCapacityLimitGuardFalseFallthrough',
            'CapacityLimitSensibleOutputGuardFalseFallthrough',
            'CapacityLimitSensibleOutputAdjustmentBodyEntered',
            'T\s*=\s*U\+N\+P\+G\+F\+E\s*=\s*U\+N\+P\+G\+A',
            'A\s*=\s*F\+E\s*=\s*CP338 assignments\s*=\s*CP337 body entries',
            '3\*A\+E',
            '(?s)retained.*?CP339 latest snapshot.*?private witness',
            'cooling_sensible_output_w',
            '(?s)retained.*?CP321 latest snapshot.*?private.witness',
            'maximum_total_cooling_capacity_w',
            '(?s)aggregate.*?may exceed',
            '(?s)C\+\+.*?operand-evaluation.order',
            'raw IEEE',
            'NaN',
            '\+infinity',
            '-infinity',
            'zero reset',
            'public.*?reach',
            '(?s)\+infinity`?\s+flow.*?exact-zero\s+enthalpy\s+difference.*?CP339.*?NaN.*?false\s+route',
            '(?s)finite\s+positive\s+flow.*?finite\s+enthalpy\s+operands.*?subtraction\s+overflows\s+to\s+`\+infinity`.*?CP339.*?`\+infinity`.*?CP340.*?(?:true\s+route|enters\s+the\s+body)',
            'Some\(value\)',
            'Serde\s+JSON',
            'IEEE\s+bit\s+string',
            'CP339-to-CP340-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle',
            $cp340FirstExcludedStatementPattern,
            'line 2208',
            '(?i)numerical[- ]DTO',
            'Roadmap'
        )) {
        if ($cp340Section -notmatch $cp340Pattern) {
            throw "CP340 documentation in $($cp340Documentation.Path) missing '$cp340Pattern'"
        }
    }
    Assert-NotContains -Path $cp340Documentation.Path -Pattern $cp340ObsoletePositiveDifferencePattern -Description "obsolete CP340 infinite-flow positive-difference documentation claim"
}

# Root reachability and generated inventory add one internal audit script.
$cp340MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp339DotSourceIndexForCp340 = $cp340MainAuditText.IndexOf('ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1')
$cp340DotSourceIndex = $cp340MainAuditText.IndexOf('ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1')
$cp340AuditCompletionIndex = $cp340MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp339DotSourceIndexForCp340 -lt 0 -or
    $cp340DotSourceIndex -le $cp339DotSourceIndexForCp340 -or
    $cp340AuditCompletionIndex -le $cp340DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP340 after CP339 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 290' -Description "CP340 cumulative inventory total through CP352"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp340-cooling-positive-supply-capacity-limit-sensible-output-guard\.ps1"' -Description "CP340 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp340-cooling-positive-supply-capacity-limit-sensible-output-guard\.ps1::dot_sources' -Description "CP340 main-audit callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 290 \|' -Description "CP340 generated script count through CP352"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP340 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 50 \|' -Description "CP340 generated internal script count through CP352"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP340 generated uncalled script count"
