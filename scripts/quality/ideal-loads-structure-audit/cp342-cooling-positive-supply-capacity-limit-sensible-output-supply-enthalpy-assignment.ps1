# CP342 maps only PurchasedAirManager.cc physical executable line 2200:
# SupplyEnthalpy = MixedAirEnthalpy - CoolSensOutput / SupplyMassFlowRate;
# Physical line 2201 is the first excluded executable.

$cp342Stem = "cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment"
$cp342PipelineStem = "purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment"
$cp342TypeStem = "PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignment"
$cp342Module = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem.rs"
$cp342State = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem\state.rs"
$cp342Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem\transition.rs"
$cp342Release = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem\release.rs"
$cp342PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem\release\prefix_validation.rs"
$cp342RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem\release\runtime_validation.rs"
$cp342SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem\release\snapshot_validation.rs"
$cp342Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem\tests\mod.rs"
$cp342PublicReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\$cp342Stem\tests\public_release.rs"
$cp342CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp342Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp342Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp342ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp342BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp342Stem.rs"
$cp342BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp342BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp342Stem}_tests.rs"
$cp342InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp342InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp342InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp342InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp342Stem.rs"
$cp342CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp342CoupledRuntimeTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp342CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp342Stem}_validation.rs"
$cp342CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp342CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp342Stem}_fixture.rs"
$cp342PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp342Pipeline = "crates\ep_run\src\pipeline\$cp342PipelineStem.rs"
$cp342PipelineValidation = "crates\ep_run\src\pipeline\$cp342PipelineStem\validation.rs"
$cp342PipelineSerialization = "crates\ep_run\src\pipeline\$cp342PipelineStem\serialization.rs"
$cp342PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\$cp342PipelineStem\serialization\snapshot.rs"

foreach ($cp342RequiredFile in @(
        $cp342Module,
        $cp342State,
        $cp342Transition,
        $cp342Release,
        $cp342PrefixValidation,
        $cp342RuntimeValidation,
        $cp342SnapshotValidation,
        $cp342Tests,
        $cp342PublicReleaseTests,
        $cp342BindingAdapter,
        $cp342BindingTests,
        $cp342InitWitness,
        $cp342CoupledValidation,
        $cp342CoupledFixture,
        $cp342Pipeline,
        $cp342PipelineValidation,
        $cp342PipelineSerialization,
        $cp342PipelineSnapshotSerialization,
        $cp342CoupledRuntimeTests
    )) {
    Assert-FileExists -Path $cp342RequiredFile -Description "CP342 supply-enthalpy assignment structure"
}
Assert-LineLimit -Path $cp342Release -Limit 800 -Description "CP342 release module"
Assert-LineLimit -Path $cp342CoupledValidation -Limit 800 -Description "CP342 coupled validation module"
Assert-LineLimit -Path $cp342Pipeline -Limit 800 -Description "CP342 pipeline module"

$cp342SourceStatementPattern =
    'SupplyEnthalpy\s*=\s*MixedAirEnthalpy\s*-\s*CoolSensOutput\s*/\s*SupplyMassFlowRate\s*;'
$cp342FirstExcludedStatementPattern =
    'PurchAir\.SupplyTemp\s*=\s*PsyTdbFnHW\(\s*SupplyEnthalpy\s*,\s*PurchAir\.SupplyHumRat\s*\)\s*;'
$cp342OrderedSourceSitesPattern = '(?s)' +
    'read-retained-mixed-air-enthalpy-for-supply-enthalpy-difference.*?' +
    'read-retained-cooling-sensible-output-for-specific-cooling-output-division.*?' +
    'read-retained-supply-mass-flow-rate-for-specific-cooling-output-division.*?' +
    'calculate-cooling-sensible-output-divided-by-supply-mass-flow-rate.*?' +
    'calculate-mixed-air-enthalpy-minus-specific-cooling-output.*?' +
    'assign-local-supply-enthalpy'

# Exact source boundary, public types, routes, and six-site inventory.
Assert-Contains -Path $cp342Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2200' -Description "CP342 exact physical source boundary"
Assert-Contains -Path $cp342Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2201' -Description "CP342 first excluded physical executable"
Assert-Contains -Path $cp342Module -Pattern 'Exact six textual source sites represented by CP342' -Description "CP342 source-site count"
Assert-ExactStringArray -Path $cp342Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-retained-mixed-air-enthalpy-for-supply-enthalpy-difference",
    "read-retained-cooling-sensible-output-for-specific-cooling-output-division",
    "read-retained-supply-mass-flow-rate-for-specific-cooling-output-division",
    "calculate-cooling-sensible-output-divided-by-supply-mass-flow-rate",
    "calculate-mixed-air-enthalpy-minus-specific-cooling-output",
    "assign-local-supply-enthalpy"
) -Description "CP342 deterministic source witness"
Assert-Contains -Path $cp342Module -Pattern ('pub struct ' + $cp342TypeStem + 'Snapshot') -Description "CP342 public snapshot"
Assert-Contains -Path $cp342State -Pattern ('pub struct ' + $cp342TypeStem + 'RuntimeState') -Description "CP342 persistent state"
Assert-Contains -Path $cp342Module -Pattern ('pub struct ' + $cp342TypeStem + 'LifecycleSummary') -Description "CP342 lifecycle summary"
Assert-Contains -Path $cp342Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle_summary\s*\(' -Description "CP342 lifecycle accessor"
Assert-Contains -Path $cp342Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\s*\(' -Description "CP342 exact-direct release"
Assert-Contains -Path $cp342Transition -Pattern 'advance_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_state\s*\(' -Description "CP342 pure transition"
Assert-Contains -Path $cp342CalcRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment;' -Description "CP342 Calc module declaration"

foreach ($cp342Route in @(
        "UnitOff",
        "NonCooling",
        "PositiveGuardFalseFallthrough",
        "ActiveCapacityLimitGuardFalseFallthrough",
        "CapacityLimitSensibleOutputGuardFalseFallthrough",
        "CapacityLimitSensibleOutputSupplyEnthalpyAssigned"
    )) {
    Assert-Contains -Path $cp342State -Pattern $cp342Route -Description "CP342 retained route '$cp342Route'"
}
foreach ($cp342Counter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "capacity_limit_guard_false_fallthrough_skip_count",
        "capacity_limit_sensible_output_guard_false_fallthrough_count",
        "capacity_limit_sensible_output_supply_enthalpy_assignment_count",
        "source_site_execution_count",
        "mixed_air_enthalpy_read_count",
        "cooling_sensible_output_read_count",
        "supply_mass_flow_rate_read_count",
        "specific_cooling_output_calculation_count",
        "supply_enthalpy_calculation_count",
        "supply_enthalpy_assignment_write_count"
    )) {
    Assert-Contains -Path $cp342State -Pattern ('pub ' + $cp342Counter + ':\s*usize') -Description "CP342 state counter '$cp342Counter'"
}

# Arithmetic must preserve division-before-subtraction and raw IEEE payloads.
Assert-PatternsInOrder -Path $cp342Transition -Patterns @(
    'operands\.cooling_sensible_output_w\s*[\r\n\s]*/\s*operands\.supply_mass_flow_rate_kg_per_s',
    'operands\.mixed_air_enthalpy_j_per_kg\s*-\s*specific_cooling_output',
    'assigned_supply_enthalpy_j_per_kg\s*=\s*[\r\n\s]*calculated_supply_enthalpy_j_per_kg',
    'resulting_supply_enthalpy_j_per_kg\s*=\s*if assignment_executed'
) -Description "CP342 exact division, subtraction, assignment, and result order"
Assert-NotContains -Path $cp342Transition -Pattern 'mul_add|total_cmp|partial_cmp|\.clamp\(|is_finite|\.max\(|\.min\(' -Description "CP342 transition excludes reassociation and numeric coercion"
Assert-Contains -Path $cp342Tests -Pattern 'source_boundary_and_exact_six_sites_are_stable' -Description "CP342 source-boundary regression"
Assert-Contains -Path $cp342Tests -Pattern 'all_six_routes_have_exact_local_shapes' -Description "CP342 route-shape regression"
Assert-Contains -Path $cp342Tests -Pattern 'arithmetic_divides_then_subtracts_without_reassociation' -Description "CP342 grouping regression"
Assert-Contains -Path $cp342Tests -Pattern 'pure_ieee_transition_keeps_positive_zero_and_negative_infinity' -Description "CP342 IEEE regression"
Assert-Contains -Path $cp342Tests -Pattern 'guard_false_preserves_arbitrary_preexisting_bits_without_sites' -Description "CP342 false-route preservation"
Assert-Contains -Path $cp342Tests -Pattern 'counters_partition_routes_and_apply_six_h_identity' -Description "CP342 T and 6*H regression"

# Checked lifecycle algebra is T=U+N+P+G+F+H, A=F+H,
# H=M=CP340 body entries, and every site count is H.
Assert-Contains -Path $cp342RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_supply_enthalpy_assignment_count\s*\.checked_mul\(6\)' -Description "CP342 checked 6*H source formula"
Assert-Contains -Path $cp342RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_supply_enthalpy_assignment_count\s*==\s*predecessor\s*\.capacity_limit_sensible_output_maximum_capacity_assignment_count' -Description "CP342 H equals CP341 M"
Assert-Contains -Path $cp342RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_supply_enthalpy_assignment_count\s*==\s*guard\.capacity_limit_sensible_output_adjustment_body_entry_count' -Description "CP342 H equals CP340 body entries"
Assert-Contains -Path $cp342RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_guard_false_fallthrough_count\s*\.checked_add\(\s*state\.capacity_limit_sensible_output_supply_enthalpy_assignment_count,\s*\)\s*== Some\(guard\.capacity_limit_sensible_output_guard_evaluation_count\)' -Description "CP342 A=F+H"
Assert-Contains -Path $cp342RuntimeValidation -Pattern '(?s)route_partition == state\.transition_count.*?source_site_execution_count == expected_source_sites.*?mixed_air_enthalpy_read_count == assigned.*?cooling_sensible_output_read_count == assigned.*?supply_mass_flow_rate_read_count == assigned.*?specific_cooling_output_calculation_count == assigned.*?supply_enthalpy_calculation_count == assigned.*?supply_enthalpy_assignment_write_count == assigned' -Description "CP342 route/source/site identities"
foreach ($cp342PreflightCounter in @(
        "capacity_limit_sensible_output_supply_enthalpy_assignment_count",
        "source_site_execution_count",
        "mixed_air_enthalpy_read_count",
        "cooling_sensible_output_read_count",
        "supply_mass_flow_rate_read_count",
        "specific_cooling_output_calculation_count",
        "supply_enthalpy_calculation_count",
        "supply_enthalpy_assignment_write_count",
        "witnessed_capacity_limit_sensible_output_supply_enthalpy_assignment_count"
    )) {
    Assert-Contains -Path $cp342RuntimeValidation -Pattern ($cp342PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP342 checked preflight '$cp342PreflightCounter'"
}

# Snapshot validation distinguishes four inherited skips, immediate guard-false
# preservation, and the six-site assignment.
Assert-Contains -Path $cp342SnapshotValidation -Pattern '(?s)fn false_fallthrough_snapshot_is_exact\(.*?preexisting\.is_finite\(\).*?preexisting\.to_bits\(\) == resulting\.to_bits\(\).*?source_values_are_none' -Description "CP342 false route preserves finite CP339 enthalpy"
Assert-Contains -Path $cp342SnapshotValidation -Pattern '(?s)fn assigned_snapshot_is_exact\(.*?expected_quotient = output / flow.*?expected_enthalpy = mixed_air - expected_quotient.*?assigned\.to_bits\(\) == calculated\.to_bits\(\).*?resulting\.to_bits\(\) == assigned\.to_bits\(\)' -Description "CP342 exact arithmetic and assignment bits"
Assert-Contains -Path $cp342SnapshotValidation -Pattern '(?s)fn skipped_snapshot_is_exact\(.*?preexisting_supply_enthalpy_j_per_kg\.is_none\(\).*?resulting_supply_enthalpy_j_per_kg\.is_none\(\).*?source_values_are_none' -Description "CP342 inherited complete-null firewall"

# The public surface accepts only runtime/system/CP341 and obtains the stable
# locals from CP339 plus the post-capacity CoolSensOutput from CP341.
Assert-Contains -Path $cp342Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp341:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,\s*\)' -Description "CP342 exact public arguments"
Assert-Contains -Path $cp342Release -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witness\s*\(' -Description "CP342 CP341 private witness"
Assert-Contains -Path $cp342Release -Pattern 'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_is_consistent\s*\(' -Description "CP342 recursive CP341 proof"
Assert-Contains -Path $cp342Release -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness\s*\(' -Description "CP342 CP339 private witness"
Assert-Contains -Path $cp342Release -Pattern 'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent\s*\(' -Description "CP342 recursive CP339 proof"
Assert-Contains -Path $cp342PrefixValidation -Pattern 'retained_cp339_lineage_is_exact\s*\(' -Description "CP342 CP339 lineage validation"
Assert-Contains -Path $cp342PrefixValidation -Pattern '(?s)predecessor\.resulting_cooling_sensible_output_w.*?assignment\.cooling_sensible_output_w' -Description "CP342 post-capacity CP341 output ownership"
Assert-Contains -Path $cp342PrefixValidation -Pattern '(?s)cp339\.mixed_air_enthalpy_j_per_kg.*?cp339\.supply_mass_flow_rate_kg_per_s.*?predecessor\.resulting_cooling_sensible_output_w' -Description "CP342 CP339/CP341 operand extraction"
Assert-Contains -Path $cp342PrefixValidation -Pattern '(?s)cp339_supply_enthalpy\.to_bits\(\) != preexisting\.to_bits\(\).*?preexisting\.to_bits\(\) == resulting\.to_bits\(\)' -Description "CP342 guard-false CP339 preservation"
Assert-Contains -Path $cp342PrefixValidation -Pattern '(?s)resulting_cooling_sensible_output_w\s*\.is_some_and\(\|value\| value\.is_finite\(\) && value > 0\.0\).*?flow > 0\.0.*?mixed_air\.is_finite\(\).*?preexisting\.is_finite\(\)' -Description "CP342 complete public operand domains"
Assert-NotContains -Path $cp342Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^)]*(mixed_air_enthalpy|cooling_sensible_output|supply_mass_flow_rate|supply_enthalpy)\s*:' -Description "CP342 duplicate caller operands"
Assert-NotContains -Path $cp342Release -Pattern 'calc_cooling_capacity_zero_flow_reset|cooling_capacity_zero_flow_reset|AutosizeOrNumber|sized_|latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|DirectZonePurchasedAirCouplingInput|ZoneHeatBalanceState|zone_state|system\.maximum_total_cooling_capacity_w' -Description "CP342 CP321/model/sizing/service/numerical exclusions"
Assert-Contains -Path $cp342PublicReleaseTests -Pattern 'public_false_route_preserves_retained_cp339_supply_enthalpy_without_sites' -Description "CP342 public false preservation"
Assert-Contains -Path $cp342PublicReleaseTests -Pattern 'public_true_route_uses_only_retained_cp339_and_cp341_operands' -Description "CP342 public provenance"
Assert-Contains -Path $cp342PublicReleaseTests -Pattern 'full_public_cp339_nan_chain_skips_arithmetic_and_preserves_supply_enthalpy' -Description "CP342 public NaN guard-false route"
Assert-Contains -Path $cp342PublicReleaseTests -Pattern 'full_public_cp339_positive_infinity_chain_uses_cp341_finite_maximum' -Description "CP342 public finite-flow capacity-capping route"
Assert-Contains -Path $cp342PublicReleaseTests -Pattern 'supplied_and_retained_cp341_and_cp339_drift_are_transactional' -Description "CP342 predecessor drift firewall"
Assert-Contains -Path $cp342PublicReleaseTests -Pattern 'assignment_counter_overflow_is_preflighted_transactionally' -Description "CP342 transactional overflow"

# CP342 state and latest witness remain private and selected-system rooted.
Assert-Contains -Path $cp342InitState -Pattern ('(?s)' + $cp342Stem + '_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*' + $cp342TypeStem + 'Snapshot') -Description "runtime-root private CP342 witness map"
Assert-NotContains -Path $cp342InitState -Pattern ('(?m)^\s*pub(?:\([^)]*\))?\s+' + $cp342Stem + '_latest_witnesses:') -Description "CP342 witness map remains private"
Assert-Contains -Path $cp342InitWitnessRoot -Pattern ('mod ' + $cp342Stem + ';') -Description "CP342 witness module"
Assert-Contains -Path $cp342InitWitness -Pattern ($cp342Stem + '_latest_witness\s*\(') -Description "CP342 private witness getter"
Assert-Contains -Path $cp342InitWitness -Pattern ('set_' + $cp342Stem + '_latest_witness\s*\(') -Description "CP342 private witness setter"
Assert-Contains -Path $cp342InitState -Pattern ('pub calc_' + $cp342Stem + ':') -Description "per-unit CP342 state"
Assert-Contains -Path $cp342InitUnit -Pattern ('(?s)calc_' + $cp342Stem + ':\s*' + $cp342TypeStem + 'RuntimeState::new\(\s*system') -Description "per-unit CP342 initialization"

# Binding is exactly CP341 -> CP342 -> unchanged numerical coupling.
$cp342BindingText = Read-RepoText -Path $cp342Binding
$cp341BindingCallForCp342 = [regex]::Match(
    $cp342BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;'
)
$cp342BindingCall = [regex]::Match(
    $cp342BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;'
)
$cp342NumericalIndex = $cp342BindingText.IndexOf('let coupling = complete_direct_zone_purchased_air_coupling(')
if (
    -not $cp341BindingCallForCp342.Success -or
    -not $cp342BindingCall.Success -or
    $cp342BindingCall.Index -lt ($cp341BindingCallForCp342.Index + $cp341BindingCallForCp342.Length) -or
    $cp342NumericalIndex -lt ($cp342BindingCall.Index + $cp342BindingCall.Length)
) {
    throw "Binding must complete CP341 then CP342 before unchanged numerical coupling"
}
foreach ($cp342BindingInterval in @(
        [PSCustomObject]@{
            Start = $cp341BindingCallForCp342.Index + $cp341BindingCallForCp342.Length
            End = $cp342BindingCall.Index
            Description = "after CP341 and before CP342"
        },
        [PSCustomObject]@{
            Start = $cp342BindingCall.Index + $cp342BindingCall.Length
            End = $cp342NumericalIndex
            Description = "after CP342 and before numerical coupling"
        }
    )) {
    $cp342BindingIntervalText = $cp342BindingText.Substring(
        $cp342BindingInterval.Start,
        $cp342BindingInterval.End - $cp342BindingInterval.Start
    )
    $cp342BindingIntervalCode =
        [regex]::Replace($cp342BindingIntervalText, '(?m)//.*$', '')
    $cp342BindingIntervalCode = [regex]::Replace(
        $cp342BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
    $cp342BindingIntervalCode = [regex]::Replace(
        $cp342BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    $cp342BindingIntervalCode = [regex]::Replace(
        $cp342BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp342BindingIntervalCode = [regex]::Replace(
        $cp342BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp342BindingIntervalCode = [regex]::Replace(
        $cp342BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp342BindingIntervalCode = [regex]::Replace(
    $cp342BindingIntervalCode,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;)',
    ''
)
    if ($cp342BindingIntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp342BindingInterval.Description)"
    }
}
Assert-Contains -Path $cp342Binding -Pattern '(?s)advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment,\s*\)\?;' -Description "binding exact CP341-to-CP342 call"
Assert-Contains -Path $cp342BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,\s*\)' -Description "CP342 binding adapter arguments"
Assert-NotContains -Path $cp342BindingAdapter -Pattern 'mixed_air_enthalpy\s*:|cooling_sensible_output\s*:|supply_mass_flow_rate\s*:|supply_enthalpy\s*:|sized_|latest_numerical|complete_direct_zone_purchased_air_coupling' -Description "CP342 binding excludes duplicate operands"
Assert-Contains -Path $cp342ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment:' -Description "CP342 scheduled output"
Assert-Contains -Path $cp342BindingTestsRoot -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_tests\.rs' -Description "CP342 binding test module"
Assert-Contains -Path $cp342BindingTests -Pattern 'scheduled_binding_preserves_false_and_true_supply_enthalpy_routes' -Description "CP342 binding route regression"

# Coupled runtime and pipeline expose direct-only CP342 evidence reconstructed
# from CP341/CP339 retained evidence, never from the numerical DTO.
Assert-Contains -Path $cp342CoupledRuntime -Pattern ('mod ' + $cp342Stem + '_validation;') -Description "coupled CP342 validator"
Assert-Contains -Path $cp342CoupledRuntime -Pattern ('pub calc_' + $cp342Stem + '_lifecycle:') -Description "coupled CP342 lifecycle"
Assert-Contains -Path $cp342CoupledRuntime -Pattern ($cp342Stem + '_validation::snapshot_matches_release') -Description "coupled CP342 snapshot validation"
Assert-Contains -Path $cp342CoupledRuntime -Pattern ($cp342Stem + '_validation::validate_lifecycle') -Description "coupled CP342 final validation"
Assert-Contains -Path $cp342CoupledValidation -Pattern 'output\s*\.\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment' -Description "coupled CP341 predecessor"
Assert-Contains -Path $cp342CoupledValidation -Pattern 'output\s*\.\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment' -Description "coupled retained CP339 operands"
Assert-Contains -Path $cp342CoupledValidation -Pattern '(?s)checked_mul\(\s*state\.capacity_limit_sensible_output_supply_enthalpy_assignment_count,\s*6,' -Description "coupled CP342 checked 6*H"
Assert-NotContains -Path $cp342CoupledValidation -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|system\.maximum_total_cooling_capacity_w' -Description "coupled CP342 numerical/model firewall"
Assert-Contains -Path $cp342CoupledFixtureRoot -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_fixture\.rs' -Description "CP342 coupled fixture module"
Assert-Contains -Path $cp342PipelineRoot -Pattern ('mod ' + $cp342PipelineStem + ';') -Description "pipeline CP342 module"
Assert-Contains -Path $cp342PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle' -Description "pipeline CP342 lifecycle and JSON key"
Assert-Contains -Path $cp342Pipeline -Pattern 'predecessor_cp341' -Description "pipeline CP341 predecessor"
Assert-Contains -Path $cp342PipelineValidation -Pattern 'checked_mul\(6\)' -Description "pipeline CP342 checked 6*H"
Assert-NotContains -Path $cp342Pipeline -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|sized_|system\.maximum_total_cooling_capacity_w' -Description "pipeline CP342 numerical/model firewall"
foreach ($cp342JsonField in @(
        "preexisting_supply_enthalpy_j_per_kg",
        "mixed_air_enthalpy_j_per_kg",
        "cooling_sensible_output_w",
        "supply_mass_flow_rate_kg_per_s",
        "specific_cooling_output_j_per_kg",
        "calculated_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg"
    )) {
    Assert-Contains -Path $cp342PipelineSnapshotSerialization -Pattern ('"' + $cp342JsonField + '"') -Description "pipeline CP342 JSON field '$cp342JsonField'"
}
Assert-Contains -Path $cp342PipelineSnapshotSerialization -Pattern '(?s)fn json_number\(value: Option<f64>\) -> Value.*?filter\(\|value\| value\.is_finite\(\)\).*?map_or\(Value::Null' -Description "CP342 nonfinite numeric null projection"
Assert-Contains -Path $cp342PipelineSnapshotSerialization -Pattern 'value\.map\(\|value\| format!\("0x\{:016x\}", value\.to_bits\(\)\)\)' -Description "CP342 authoritative IEEE bits"
Assert-Contains -Path $cp342PipelineRoot -Pattern '(?s)purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle:\s*None' -Description "non-direct CP342 null evidence"
Assert-Contains -Path $cp342PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp398_lifecycle_evidence' -Description "non-direct CP342 through CP363 evidence rejection"

# Exactly two algorithm addenda, two capability addenda, and a 2+4 target
# distribution carry the same source/provenance/IEEE/exclusion contract.
$cp342AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp342AlgorithmAddenda = [regex]::Matches(
    $cp342AlgorithmText,
    '(?m)^\s*"CP342 supersedes only CP341[^"\r\n]+",\s*$'
)
if ($cp342AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP342 claim addenda"
}
$cp342TargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\.rs::purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp342Target in $cp342TargetCounts) {
    $cp342TargetCount = [regex]::Matches($cp342AlgorithmText, $cp342Target.Pattern).Count
    if ($cp342TargetCount -ne $cp342Target.Expected) {
        throw "CP342 target '$($cp342Target.Pattern)' expected $($cp342Target.Expected), found $cp342TargetCount"
    }
}
$cp342CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp342CapabilityAddenda = [regex]::Matches(
    $cp342CapabilityText,
    '(?m)^\s*"CP342 additionally requires[^"\r\n]+",\s*$'
)
if ($cp342CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP342 claim addenda"
}
foreach ($cp342Claim in @($cp342AlgorithmAddenda) + @($cp342CapabilityAddenda)) {
    foreach ($cp342Pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            $cp342SourceStatementPattern,
            $cp342OrderedSourceSitesPattern,
            'C\+\+ built-in `/`, `-`, or `=` operand-evaluation order',
            'CapacityLimitSensibleOutputMaximumCapacityAssigned',
            'CapacityLimitSensibleOutputSupplyEnthalpyAssigned',
            'T=U\+N\+P\+G\+F\+H',
            'A=F\+H',
            'H=M=CP340 adjustment-body entries',
            'source_site_execution_count=6\*H',
            'No invariant equates `H` with `A`',
            'supplied.*?(?:retained-latest|latest).*?private CP341',
            'resulting_cooling_sensible_output_w',
            'never CP339.*?pre-capacity output',
            'same-call CP339 latest/private witness',
            'CP340 false route preserves',
            'finite strictly positive CP341 cooling output',
            'finite\s+strictly\s+positive\s+supply\s+(?:mass\s+)?flow',
            'publicly witnessed nonfinite predecessor uses finite flow',
            '(?s)`\+infinity` supply flow.*?CP339 product NaN.*?false route',
            '(?s)`\+infinity`-flow positive-zero quotient.*?characterization only.*?not public assignment-reachability claims',
            'raw IEEE binary64',
            'derived finite-result coercion',
            'Some\(value\)',
            'serde JSON',
            'IEEE bit strings',
            'CP341-to-CP342-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle',
            $cp342FirstExcludedStatementPattern,
            'line\s+2208',
            'Roadmap promotion'
        )) {
        if ($cp342Claim.Value -notmatch $cp342Pattern) {
            throw "CP342 spec addendum missing '$cp342Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP342 supersedes only CP341' -Description "generated CP342 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP342 additionally requires' -Description "generated CP342 capability index"

# Five hand-authored contracts repeat the source, lineage, IEEE, JSON,
# numerical-firewall, exclusion, and no-promotion boundary.
$cp342DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP342 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP342 Source-Ordered Cooling Positive-Supply Capacity-Limit Supply-Enthalpy Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP342 Cooling Positive-Supply Capacity-Limit Supply-Enthalpy Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP342 Positive-Supply Capacity-Limit Supply-Enthalpy Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP342 Cooling Positive-Supply Capacity-Limit Supply-Enthalpy Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp342Documentation in $cp342DocumentationSections) {
    $cp342DocumentText = Read-RepoText -Path $cp342Documentation.Path
    $cp342Matches = [regex]::Matches($cp342DocumentText, $cp342Documentation.Pattern)
    if ($cp342Matches.Count -ne 1) {
        throw "CP342 documentation expected one scoped section in $($cp342Documentation.Path), found $($cp342Matches.Count)"
    }
    $cp342Section = $cp342Matches[0].Value
    foreach ($cp342Pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            $cp342SourceStatementPattern,
            $cp342OrderedSourceSitesPattern,
            '(?s)C\+\+.*?operand-evaluation',
            'CapacityLimitSensibleOutputMaximumCapacityAssigned',
            'UnitOff',
            'NonCooling',
            'PositiveGuardFalseFallthrough',
            'ActiveCapacityLimitGuardFalseFallthrough',
            'CapacityLimitSensibleOutputGuardFalseFallthrough',
            'CapacityLimitSensibleOutputSupplyEnthalpyAssigned',
            'T\s*=\s*U\+N\+P\+G\+F\+H',
            'A\s*=\s*F\+H',
            'H\s*=\s*M\s*=\s*CP340 adjustment-body entries',
            '6\*H',
            '(?s)supplied.*?(?:CP341.*?(?:latest|retained-latest).*?private|(?:latest|retained-latest).*?private.*?CP341)',
            'resulting_cooling_sensible_output_w',
            '(?s)same-call\s+CP339\s+latest/private\s+witness.*?recursive\s+completion',
            '(?s)false.*?(?:preserves|keeps).*?without.*?(?:read|RHS)',
            '(?s)finite.*?strictly positive.*?CP341',
            'finite\s+strictly\s+positive\s+supply\s+(?:mass\s+)?flow',
            '(?s)publicly\s+witnessed\s+nonfinite\s+predecessor.*?finite\s+flow',
            '(?s)`\+infinity`\s+supply\s+flow.*?CP339\s+product\s+NaN.*?false\s+route',
            '(?s)characterization\s+only.*?not\s+(?:claims\s+of\s+)?public\s+assignment(?:-|\s+)reachability',
            'raw\s+IEEE',
            'Some\(value\)',
            'Serde\s+JSON',
            'IEEE\s+bit\s+string',
            'CP341-to-CP342-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle',
            'None',
            $cp342FirstExcludedStatementPattern,
            'line\s+2208',
            '(?i)numerical[- ]DTO',
            'Roadmap'
        )) {
        if ($cp342Section -notmatch $cp342Pattern) {
            throw "CP342 documentation in $($cp342Documentation.Path) missing '$cp342Pattern'"
        }
    }
}

# Historical interval and firewall audits must recognize CP342 without
# admitting arbitrary helpers.
foreach ($cp342HistoricalBindingAudit in @(
        "scripts\quality\ideal-loads-structure-audit\cp326-cooling-supply-mass-flow-limit-body.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp329-cooling-mixed-air-call.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp330-cooling-supply-mass-flow-positive-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp331-cooling-positive-supply-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp332-cooling-positive-supply-temperature-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp333-cooling-positive-supply-temperature-minimum-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1"
    )) {
    Assert-Contains -Path $cp342HistoricalBindingAudit -Pattern 'calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment' -Description "historical binding interval admits only CP342"
}
foreach ($cp342HistoricalFirewallAudit in @(
        "scripts\quality\ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1"
    )) {
    Assert-Contains -Path $cp342HistoricalFirewallAudit -Pattern 'non_direct_runtime_rejects_cp316_through_cp398_lifecycle_evidence' -Description "historical non-direct firewall reaches CP362"
}

# Root reachability and generated inventory add exactly one internal script:
# 284 executable records, 240 public, 44 internal, and zero uncalled.
$cp342MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp341DotSourceIndexForCp342 = $cp342MainAuditText.IndexOf('ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1')
$cp342DotSourceIndex = $cp342MainAuditText.IndexOf('ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1')
$cp342AuditCompletionIndex = $cp342MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp341DotSourceIndexForCp342 -lt 0 -or
    $cp342DotSourceIndex -le $cp341DotSourceIndexForCp342 -or
    $cp342AuditCompletionIndex -le $cp342DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP342 after CP341 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 336' -Description "CP342 cumulative inventory total through CP358"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment\.ps1"' -Description "CP342 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment\.ps1::dot_sources' -Description "CP342 main-audit callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 336 \|' -Description "CP342 generated script count through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP342 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 96 \|' -Description "CP342 generated internal script count through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP342 generated uncalled script count"
