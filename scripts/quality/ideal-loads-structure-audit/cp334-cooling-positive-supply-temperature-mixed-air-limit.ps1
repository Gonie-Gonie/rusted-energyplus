# CP334 maps only PurchasedAirManager.cc physical executable line 2189: the
# Cooling positive-supply temperature mixed-air limit. Physical line 2190 is
# the first excluded executable and CP335 edge.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp334Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit.rs"
$cp334State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\state.rs"
$cp334Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\transition.rs"
$cp334Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\release.rs"
$cp334PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\release\prefix_validation.rs"
$cp334RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\release\runtime_validation.rs"
$cp334SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\release\snapshot_validation.rs"
$cp334Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\tests\mod.rs"
$cp334ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\tests\release_corruption.rs"
$cp334CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp334Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp334Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp334ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp334BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_temperature_mixed_air_limit.rs"
$cp334BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp334BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_temperature_mixed_air_limit_tests.rs"
$cp334InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp334InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp334InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp334InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_temperature_mixed_air_limit.rs"
$cp334CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp334CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_temperature_mixed_air_limit_validation.rs"
$cp334CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp334CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_temperature_mixed_air_limit_fixture.rs"
$cp334PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp334Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_mixed_air_limit.rs"
$cp334PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_mixed_air_limit\validation.rs"
$cp334PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_mixed_air_limit\serialization.rs"
$cp334PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_mixed_air_limit\serialization\snapshot.rs"
$cp334RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp334DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_temperature_mixed_air_limit_assertions.rs"
$cp334NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp334RequiredFile in @(
        $cp334Module,
        $cp334State,
        $cp334Transition,
        $cp334Release,
        $cp334PrefixValidation,
        $cp334RuntimeValidation,
        $cp334SnapshotValidation,
        $cp334Tests,
        $cp334ReleaseCorruptionTests,
        $cp334ScheduledOutput,
        $cp334BindingAdapter,
        $cp334BindingTests,
        $cp334InitWitness,
        $cp334CoupledValidation,
        $cp334CoupledFixture,
        $cp334Pipeline,
        $cp334PipelineValidation,
        $cp334PipelineSerialization,
        $cp334PipelineSnapshotSerialization,
        $cp334DirectAssertions
    )) {
    Assert-FileExists -Path $cp334RequiredFile -Description "CP334 mixed-air temperature-limit structure"
}
Assert-LineLimit -Path $cp334Release -Limit 800 -Description "CP334 release root module"
Assert-LineLimit -Path $cp334RuntimeValidation -Limit 800 -Description "CP334 runtime validation module"
Assert-LineLimit -Path $cp334CoupledValidation -Limit 800 -Description "CP334 coupled validation module"
Assert-LineLimit -Path $cp334Pipeline -Limit 800 -Description "CP334 pipeline module"

# Locked source boundary and exact four-site textual inventory.
Assert-Contains -Path $cp334Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2189' -Description "CP334 exact physical source boundary"
Assert-Contains -Path $cp334Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2190' -Description "CP334 first excluded physical executable"
Assert-Contains -Path $cp334Module -Pattern 'Exact four textual source sites represented by CP334' -Description "CP334 exact lexical-site count"
Assert-ExactStringArray -Path $cp334Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER" -Expected @(
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature"
) -Description "CP334 exact four-site source order"
Assert-Contains -Path $cp334Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot' -Description "CP334 public snapshot"
Assert-Contains -Path $cp334State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState' -Description "CP334 persistent state"
Assert-Contains -Path $cp334Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary' -Description "CP334 lifecycle summary"
Assert-Contains -Path $cp334Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle_summary\s*\(' -Description "CP334 lifecycle accessor"
Assert-Contains -Path $cp334Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit\s*\(' -Description "CP334 exact-direct wrapper"
Assert-Contains -Path $cp334Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_temperature_mixed_air_limit_state\s*\(' -Description "CP334 pure transition"
Assert-Contains -Path $cp334CalcRoot -Pattern 'mod cooling_positive_supply_temperature_mixed_air_limit;' -Description "CP334 calc module declaration"
Assert-Contains -Path $cp334CalcRoot -Pattern 'pub use (?:cooling_positive_supply_temperature_mixed_air_limit::\*;|\{[^}]*cooling_positive_supply_temperature_mixed_air_limit::\*)' -Description "CP334 calc public surface"

# ObjexxFCL minimum semantics are strict-< and select the right operand on a
# tie or unordered comparison. No broader Rust minimum/clamp policy is allowed.
Assert-Contains -Path $cp334Transition -Pattern '(?s)fn source_shaped_two_argument_minimum\(\s*left: f64,\s*right: f64,\s*\) -> f64 \{\s*if left < right \{ left \} else \{ right \}\s*\}' -Description "CP334 exact ObjexxFCL minimum"
Assert-NotContains -Path $cp334Transition -Pattern 'f64::min|\.min\(|total_cmp|partial_cmp|clamp\(|\.max\(|is_finite|is_nan|normalize' -Description "CP334 forbidden broadened minimum semantics"
Assert-PatternsInOrder -Path $cp334Transition -Patterns @(
    'let limit_executed = predecessor\.supply_temperature_minimum_limit_executed;',
    'let supply_temperature_before_mixed_air_limit_c',
    'let mixed_air_temperature_c',
    'source_shaped_two_argument_minimum\(',
    'let assigned_supply_temperature_c = minimum_supply_temperature_c;',
    'state\.transition_count \+= 1;',
    'state\.source_site_execution_count \+=',
    'PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER'
) -Description "CP334 source transition order"
foreach ($cp334Route in @(
        'UnitOff',
        'NonCooling',
        'PositiveGuardFalseFallthrough',
        'SupplyTemperatureMixedAirLimitExecuted'
    )) {
    Assert-Contains -Path $cp334State -Pattern $cp334Route -Description "CP334 retained route '$cp334Route'"
}
foreach ($cp334Counter in @(
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'supply_temperature_mixed_air_limit_count',
        'source_site_execution_count',
        'supply_temperature_for_minimum_read_count',
        'mixed_air_temperature_for_minimum_read_count',
        'source_shaped_two_argument_minimum_evaluation_count',
        'supply_temperature_assignment_count'
    )) {
    Assert-Contains -Path $cp334State -Pattern ('pub ' + $cp334Counter + ':\s*usize') -Description "CP334 state counter '$cp334Counter'"
}
Assert-Contains -Path $cp334RuntimeValidation -Pattern '(?s)supply_temperature_mixed_air_limit_count.*?checked_mul\(\s*super::super::PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "CP334 four-sites-per-active formula"
Assert-Contains -Path $cp334RuntimeValidation -Pattern '(?s)pending_supply_temperature_mixed_air_limit_state_is_consistent\(.*?unit_off_skip_count.*?predecessor_state\.unit_off_skip_count.*?non_cooling_skip_count.*?predecessor_state\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor_state\.positive_guard_false_fallthrough_skip_count.*?supply_temperature_mixed_air_limit_count.*?predecessor_state\.supply_temperature_minimum_limit_count' -Description "CP334 pending CP333 four-route parity"
Assert-Contains -Path $cp334RuntimeValidation -Pattern '(?s)completed_supply_temperature_mixed_air_limit_state_is_consistent\(.*?state\.transition_count == predecessor\.transition_count.*?state\.unit_off_skip_count == predecessor\.unit_off_skip_count.*?state\.non_cooling_skip_count == predecessor\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor\.positive_guard_false_fallthrough_skip_count.*?supply_temperature_mixed_air_limit_count.*?predecessor\.supply_temperature_minimum_limit_count' -Description "CP334 completed CP333 four-route parity"
foreach ($cp334PreflightCounter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'witnessed_positive_guard_false_fallthrough_skip_count',
        'supply_temperature_mixed_air_limit_count',
        'source_site_execution_count',
        'supply_temperature_for_minimum_read_count',
        'mixed_air_temperature_for_minimum_read_count',
        'source_shaped_two_argument_minimum_evaluation_count',
        'supply_temperature_assignment_count',
        'witnessed_supply_temperature_mixed_air_limit_count'
    )) {
    Assert-Contains -Path $cp334RuntimeValidation -Pattern ($cp334PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP334 checked preflight '$cp334PreflightCounter'"
}

# Snapshot proof distinguishes all four routes, retains every active operand by
# bits, and requires only the CP329-owned right operand to be finite.
foreach ($cp334SnapshotField in @(
        'supply_temperature_before_mixed_air_limit_c',
        'mixed_air_temperature_c',
        'minimum_supply_temperature_c',
        'assigned_supply_temperature_c'
    )) {
    Assert-Contains -Path $cp334SnapshotValidation -Pattern $cp334SnapshotField -Description "CP334 snapshot field '$cp334SnapshotField'"
}
Assert-Contains -Path $cp334SnapshotValidation -Pattern 'fn limited_snapshot_is_exact' -Description "CP334 active snapshot proof"
Assert-Contains -Path $cp334SnapshotValidation -Pattern 'fn skipped_snapshot_is_exact' -Description "CP334 skipped snapshot proof"
Assert-Contains -Path $cp334SnapshotValidation -Pattern 'fn snapshots_match_bit_exact' -Description "CP334 bit-exact retained matcher"
Assert-Contains -Path $cp334SnapshotValidation -Pattern 'source_shaped_two_argument_minimum\(left, right\)' -Description "CP334 snapshot source minimum"
Assert-Contains -Path $cp334SnapshotValidation -Pattern 'right\.is_finite\(\)' -Description "CP334 finite CP329 right operand"
Assert-Contains -Path $cp334SnapshotValidation -Pattern 'to_bits\(\)' -Description "CP334 exact-bit snapshot checks"

# Release consumes only the CP333 snapshot argument and retained CP329 latest
# evidence. It accepts no Zone-state/model/scalar/numerical DTO operand.
Assert-Contains -Path $cp334Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp333: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,\s*\)' -Description "CP334 exact wrapper arguments"
Assert-Contains -Path $cp334Release -Pattern 'predecessor_cp333[\r\n\s.]+assigned_supply_temperature_c' -Description "CP334 CP333 left-operand provenance"
Assert-Contains -Path $cp334Release -Pattern 'unit\.calc_cooling_mixed_air_call\.latest' -Description "CP334 retained CP329 latest snapshot"
Assert-Contains -Path $cp334Release -Pattern 'mixed_air\.mixed_air_temperature_c' -Description "CP334 CP329 right-operand provenance"
Assert-Contains -Path $cp334Release -Pattern 'mixed_air_temperature_c\.is_finite\(\)' -Description "CP334 finite right-operand admission"
Assert-Contains -Path $cp334PrefixValidation -Pattern 'predecessor\.assigned_supply_temperature_c' -Description "CP334 exact CP333 left linkage"
Assert-Contains -Path $cp334PrefixValidation -Pattern 'mixed_air\.mixed_air_temperature_c' -Description "CP334 exact CP329 right linkage"
Assert-Contains -Path $cp334PrefixValidation -Pattern 'mixed_air_temperature_c\.is_some_and\(f64::is_finite\)' -Description "CP334 finite retained right linkage"
Assert-Contains -Path $cp334Release -Pattern 'completed_direct_cooling_positive_supply_temperature_minimum_limit_is_consistent\s*\(' -Description "CP334 recursive CP333 completed proof"
Assert-Contains -Path $cp334Release -Pattern 'cooling_positive_supply_temperature_minimum_limit_latest_witness\s*\(' -Description "CP334 retained CP333 private witness"
Assert-Contains -Path $cp334Release -Pattern 'cooling_positive_supply_temperature_mixed_air_limit_latest_witness\s*\(' -Description "CP334 private latest witness"
Assert-NotContains -Path $cp334Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit\([^)]*(?:supply_temperature|mixed_air_temperature)\w*\s*:\s*f64' -Description "duplicate caller scalar in CP334 release"
Assert-NotContains -Path $cp334Release -Pattern 'zone_state|ZoneHeatBalanceState|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput|typed_model' -Description "Zone/model/numerical DTO dependency in CP334 release"
Assert-PatternsInOrder -Path $cp334Release -Patterns @(
    'pending_supply_temperature_mixed_air_limit_state_is_consistent\(',
    'next_supply_temperature_mixed_air_limit_transition_fits\(',
    'completed_direct_cooling_positive_supply_temperature_minimum_limit_is_consistent\(',
    'let active_input = if predecessor_cp333\.supply_temperature_minimum_limit_executed',
    'advance_cooling_positive_supply_temperature_mixed_air_limit_state\(',
    'set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness\('
) -Description "CP334 validate-before-mutation order"

# Pure and release tests cover source selection, every route, exact CP329
# lineage, replay/corruption, and checked-overflow transactionality.
foreach ($cp334TestPattern in @(
        'signed_zero|zero',
        'NaN|nan',
        'infinity|infinite',
        'UnitOff|unit_off',
        'non_cooling',
        'guard_false',
        'four.*site|source_site',
        'counter',
        'finite_mixed_air'
    )) {
    Assert-Contains -Path $cp334Tests -Pattern $cp334TestPattern -Description "CP334 pure regression '$cp334TestPattern'"
}
foreach ($cp334CorruptionPattern in @(
        'replay',
        'forged|corrupt',
        'witness',
        'overflow',
        'without_mutation|transaction',
        'cp329_mixed_air_latest_witness_or_source_drift_fails_closed_transactionally',
        'active_operands_link_only_to_cp333_assignment_and_cp329_mixed_air_output'
    )) {
    Assert-Contains -Path $cp334ReleaseCorruptionTests -Pattern $cp334CorruptionPattern -Description "CP334 release regression '$cp334CorruptionPattern'"
}

# Runtime-root witness ownership remains private and per-unit state is explicit.
Assert-NotContains -Path $cp334InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_temperature_mixed_air_limit_latest_witnesses:' -Description "public runtime-root CP334 witness map"
Assert-Contains -Path $cp334InitWitnessRoot -Pattern 'mod cooling_positive_supply_temperature_mixed_air_limit;' -Description "runtime-root CP334 witness module"
Assert-Contains -Path $cp334InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_temperature_mixed_air_limit_latest_witness\s*\(' -Description "runtime-root CP334 witness getter"
Assert-Contains -Path $cp334InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness\s*\(' -Description "runtime-root CP334 witness setter"
Assert-Contains -Path $cp334InitState -Pattern 'pub calc_cooling_positive_supply_temperature_mixed_air_limit:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState' -Description "per-unit CP334 persistent state"
Assert-Contains -Path $cp334InitUnit -Pattern '(?s)calc_cooling_positive_supply_temperature_mixed_air_limit:\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState::new\(\s*system\s*,?\s*\)' -Description "per-unit CP334 state initialization"

# Binding must be CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> the
# unchanged numerical DTO with no hidden source helper between any boundary.
$cp334BindingText = Read-RepoText -Path $cp334Binding
$cp333BindingIndexForCp334 = $cp334BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_minimum_limit =")
$cp334BindingIndex = $cp334BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_mixed_air_limit =")
$cp335BindingIndexForCp334 = $cp334BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndexForCp334 = $cp334BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp334 = $cp334BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp334 = $cp334BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp334 = $cp334BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp334 = $cp334BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp333BindingIndexForCp334 -lt 0 -or
    $cp334BindingIndex -le $cp333BindingIndexForCp334 -or
    $cp335BindingIndexForCp334 -le $cp334BindingIndex -or
    $cp336BindingIndexForCp334 -le $cp335BindingIndexForCp334 -or
    $cp337BindingIndexForCp334 -le $cp336BindingIndexForCp334 -or
    $cp338BindingIndexForCp334 -le $cp337BindingIndexForCp334 -or
    $cp339BindingIndexForCp334 -le $cp338BindingIndexForCp334 -or
    $numericalBindingIndexForCp334 -le $cp339BindingIndexForCp334
) {
    throw "Binding must retain exact CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $cp334Binding -Pattern '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_temperature_minimum_limit,\s*\)\?;' -Description "binding exact CP333-to-CP334 adapter call"
Assert-Contains -Path $cp334BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_temperature_mixed_air_limit\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,' -Description "CP334 binding adapter arguments"
Assert-Contains -Path $cp334BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit\(\s*runtime,\s*system,\s*predecessor,\s*\).*?CalculationCoolingPositiveSupplyTemperatureMixedAirLimit' -Description "CP334 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp334BindingAdapter -Pattern 'zone_state|ZoneHeatBalanceState|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "extra operand or numerical DTO in CP334 adapter"
Assert-Contains -Path $cp334ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_temperature_mixed_air_limit:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot' -Description "CP334 scheduled output evidence"
Assert-Contains -Path $cp334BindingTestsRoot -Pattern '#\[rustfmt::skip\]\s*#\[path = "binding/cooling_positive_supply_temperature_mixed_air_limit_tests\.rs"\]\s*mod cooling_positive_supply_temperature_mixed_air_limit_tests;' -Description "CP334 binding test module"
Assert-Contains -Path $cp334BindingTests -Pattern 'scheduled_binding_applies_source_shaped_mixed_air_limit_bit_exactly' -Description "CP334 active binding regression"
Assert-Contains -Path $cp334BindingTests -Pattern 'scheduled_binding_skips_cp334_after_the_positive_guard_falls_through' -Description "CP334 guard-false binding regression"
Assert-Contains -Path $cp334BindingTests -Pattern 'scheduled_binding_preserves_unit_off_and_non_cooling_cp334_skip_routes' -Description "CP334 UnitOff/non-cooling binding regression"
$cp333BindingCallForCp334 = [regex]::Match(
    $cp334BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_minimum_limit =\s*advance_positive_supply_temperature_minimum_limit\([^;]+?\)\?;'
)
$cp334BindingCall = [regex]::Match(
    $cp334BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
$cp335BindingCallForCp334 = [regex]::Match(
    $cp334BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
$cp336BindingCallForCp334 = [regex]::Match(
    $cp334BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
$cp337BindingCallForCp334 = [regex]::Match(
    $cp334BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
$cp338BindingCallForCp334 = [regex]::Match(
    $cp334BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
$cp339BindingCallForCp334 = [regex]::Match(
    $cp334BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (
    -not $cp333BindingCallForCp334.Success -or
    -not $cp334BindingCall.Success -or
    -not $cp335BindingCallForCp334.Success -or
    -not $cp336BindingCallForCp334.Success -or
    -not $cp337BindingCallForCp334.Success -or
    -not $cp338BindingCallForCp334.Success -or
    -not $cp339BindingCallForCp334.Success
) {
    throw "Binding must retain complete CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls"
}
$cp333BindingCallEndForCp334 =
    $cp333BindingCallForCp334.Index + $cp333BindingCallForCp334.Length
$cp334BindingCallEnd = $cp334BindingCall.Index + $cp334BindingCall.Length
$cp335BindingCallEndForCp334 =
    $cp335BindingCallForCp334.Index + $cp335BindingCallForCp334.Length
$cp336BindingCallEndForCp334 =
    $cp336BindingCallForCp334.Index + $cp336BindingCallForCp334.Length
$cp337BindingCallEndForCp334 =
    $cp337BindingCallForCp334.Index + $cp337BindingCallForCp334.Length
$cp338BindingCallEndForCp334 =
    $cp338BindingCallForCp334.Index + $cp338BindingCallForCp334.Length
$cp339BindingCallEndForCp334 =
    $cp339BindingCallForCp334.Index + $cp339BindingCallForCp334.Length
if (
    $cp334BindingIndex -lt $cp333BindingCallEndForCp334 -or
    $cp335BindingIndexForCp334 -lt $cp334BindingCallEnd -or
    $cp336BindingIndexForCp334 -lt $cp335BindingCallEndForCp334 -or
    $cp337BindingIndexForCp334 -lt $cp336BindingCallEndForCp334 -or
    $cp338BindingIndexForCp334 -lt $cp337BindingCallEndForCp334 -or
    $cp339BindingIndexForCp334 -lt $cp338BindingCallEndForCp334 -or
    $numericalBindingIndexForCp334 -lt $cp339BindingCallEndForCp334
) {
    throw "CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
$postCp333BeforeCp334ForCp334 = $cp334BindingText.Substring(
    $cp333BindingCallEndForCp334,
    $cp334BindingIndex - $cp333BindingCallEndForCp334
)
$postCp333BeforeCp334CodeForCp334 =
    [regex]::Replace($postCp333BeforeCp334ForCp334, '(?m)//.*$', '')
if ($postCp333BeforeCp334CodeForCp334 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP333 and before CP334"
}
$postCp334BeforeCp335ForCp334 = $cp334BindingText.Substring(
    $cp334BindingCallEnd,
    $cp335BindingIndexForCp334 - $cp334BindingCallEnd
)
$postCp334BeforeCp335CodeForCp334 =
    [regex]::Replace($postCp334BeforeCp335ForCp334, '(?m)//.*$', '')
if ($postCp334BeforeCp335CodeForCp334 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP334 and before CP335"
}
$postCp335BeforeCp336ForCp334 = $cp334BindingText.Substring(
    $cp335BindingCallEndForCp334,
    $cp336BindingIndexForCp334 - $cp335BindingCallEndForCp334
)
$postCp335BeforeCp336CodeForCp334 =
    [regex]::Replace($postCp335BeforeCp336ForCp334, '(?m)//.*$', '')
if ($postCp335BeforeCp336CodeForCp334 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP335 and before CP336"
}
$postCp336BeforeCp337ForCp334 = $cp334BindingText.Substring(
    $cp336BindingCallEndForCp334,
    $cp337BindingIndexForCp334 - $cp336BindingCallEndForCp334
)
$postCp336BeforeCp337CodeForCp334 =
    [regex]::Replace($postCp336BeforeCp337ForCp334, '(?m)//.*$', '')
if ($postCp336BeforeCp337CodeForCp334 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP336 and before CP337"
}
$postCp337BeforeCp338ForCp334 = $cp334BindingText.Substring(
    $cp337BindingCallEndForCp334,
    $cp338BindingIndexForCp334 - $cp337BindingCallEndForCp334
)
$postCp337BeforeCp338CodeForCp334 =
    [regex]::Replace($postCp337BeforeCp338ForCp334, '(?m)//.*$', '')
if ($postCp337BeforeCp338CodeForCp334 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP337 and before CP338"
}
$postCp338BeforeCp339ForCp334 = $cp334BindingText.Substring(
    $cp338BindingCallEndForCp334,
    $cp339BindingIndexForCp334 - $cp338BindingCallEndForCp334
)
$postCp338BeforeCp339CodeForCp334 =
    [regex]::Replace($postCp338BeforeCp339ForCp334, '(?m)//.*$', '')
if ($postCp338BeforeCp339CodeForCp334 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP338 and before CP339"
}
$postCp339BeforeNumericalForCp334 = $cp334BindingText.Substring(
    $cp339BindingCallEndForCp334,
    $numericalBindingIndexForCp334 - $cp339BindingCallEndForCp334
)
$postCp339BeforeNumericalCodeForCp334 =
    [regex]::Replace($postCp339BeforeNumericalForCp334, '(?m)//.*$', '')
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp334 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp334,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;)',
    ''
)
if ($postCp339BeforeNumericalCodeForCp334 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No helper other than the audited CP340 through CP348 releases may execute after CP339 and before numerical Calc"
}

# Coupled runtime independently reconstructs CP334 from exact CP333 and CP329
# output, and pipeline evidence remains direct-only and bit-exact.
Assert-Contains -Path $cp334CoupledRuntime -Pattern 'mod cooling_positive_supply_temperature_mixed_air_limit_validation;' -Description "coupled CP334 validator declaration"
Assert-Contains -Path $cp334CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary' -Description "coupled CP334 lifecycle"
Assert-Contains -Path $cp334CoupledRuntime -Pattern 'cooling_positive_supply_temperature_mixed_air_limit_validation::snapshot_matches_release' -Description "coupled per-timestep CP334 validation"
Assert-Contains -Path $cp334CoupledRuntime -Pattern 'cooling_positive_supply_temperature_mixed_air_limit_validation::validate_lifecycle' -Description "coupled final CP334 validation"
Assert-Contains -Path $cp334CoupledValidation -Pattern 'let predecessor = output\.calculation_cooling_positive_supply_temperature_minimum_limit;' -Description "coupled CP333 predecessor"
Assert-Contains -Path $cp334CoupledValidation -Pattern 'let mixed_air = output\.calculation_cooling_mixed_air_call;' -Description "coupled CP329 operand source"
Assert-Contains -Path $cp334CoupledValidation -Pattern 'let snapshot = output\.calculation_cooling_positive_supply_temperature_mixed_air_limit;' -Description "coupled CP334 result"
Assert-Contains -Path $cp334CoupledValidation -Pattern 'if left < right \{ left \} else \{ right \}' -Description "coupled CP334 source-shaped minimum"
Assert-Contains -Path $cp334CoupledValidation -Pattern 'SOURCE_ORDER\.len\(\)|MIXED_AIR_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled CP334 four-site count"
Assert-Contains -Path $cp334CoupledValidation -Pattern 'to_bits\(\)|options_have_exact_bits' -Description "coupled CP334 exact-bit validation"
Assert-Contains -Path $cp334CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_temperature_mixed_air_limit_fixture;' -Description "coupled CP334 fixture declaration"
Assert-Contains -Path $cp334CoupledFixture -Pattern 'calculation_cooling_positive_supply_temperature_mixed_air_limit_snapshot' -Description "coupled CP334 fixture output"
Assert-Contains -Path $cp334PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_temperature_mixed_air_limit;' -Description "pipeline CP334 module declaration"
Assert-Contains -Path $cp334PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle' -Description "pipeline CP334 lifecycle field and JSON key"
Assert-Contains -Path $cp334PipelineRoot -Pattern 'calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle' -Description "pipeline CP334 coupled lifecycle transfer"
Assert-Contains -Path $cp334Pipeline -Pattern '(?s)validate_direct_lifecycle\(.*?predecessor_cp333.*?mixed_air_cp329' -Description "pipeline CP334 validates both retained operands"
Assert-Contains -Path $cp334PipelineValidation -Pattern 'validate_source_counters' -Description "pipeline CP334 source-counter validation"
Assert-Contains -Path $cp334PipelineValidation -Pattern 'right\.is_finite\(\)' -Description "pipeline CP334 finite retained right"
Assert-Contains -Path $cp334PipelineSerialization -Pattern 'mixed_air_temperature_for_minimum_read_count' -Description "pipeline CP334 lifecycle serialization"
Assert-Contains -Path $cp334PipelineSnapshotSerialization -Pattern 'mixed_air_temperature_for_minimum_read' -Description "pipeline CP334 snapshot serialization"
foreach ($cp334BitField in @(
        'supply_temperature_before_mixed_air_limit_c',
        'mixed_air_temperature_c',
        'minimum_supply_temperature_c',
        'assigned_supply_temperature_c'
    )) {
    Assert-Contains -Path $cp334PipelineSnapshotSerialization -Pattern ('"' + $cp334BitField + '_ieee_bits"') -Description "pipeline CP334 IEEE field '$cp334BitField'"
}
Assert-Contains -Path $cp334PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP334 exact IEEE serialization"
Assert-Contains -Path $cp334RunTests -Pattern 'cooling_positive_supply_temperature_mixed_air_limit_assertions' -Description "direct-run CP334 assertion module"
Assert-Contains -Path $cp334DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle' -Description "direct-run CP334 JSON evidence"
Assert-Contains -Path $cp334DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*4\]\s*=' -Description "direct-run CP334 source-order declaration"
Assert-Contains -Path $cp334DirectAssertions -Pattern 'executions \* SOURCE_ORDER\.len\(\) as u64' -Description "direct-run CP334 dynamic source-site count"
Assert-Contains -Path $cp334DirectAssertions -Pattern 'purchased_air_calc_cooling_mixed_air_call_lifecycle' -Description "direct-run CP329 bit provenance"
Assert-Contains -Path $cp334NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle' -Description "non-direct CP334 null evidence"
Assert-Contains -Path $cp334PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp398_lifecycle_evidence' -Description "non-direct CP334 through CP363 evidence rejection"

# Registries repeat the boundary exactly twice and add target inventory only.
$cp334AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp334AlgorithmAddenda = [regex]::Matches(
    $cp334AlgorithmText,
    '(?m)^\s*"CP334 supersedes only CP333[^"\r\n]+",\s*$'
)
if ($cp334AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP334 boundary addenda"
}
foreach ($cp334AlgorithmAddendum in $cp334AlgorithmAddenda) {
    foreach ($cp334Pattern in @(
            'physical executable line 2189',
            'exactly four lexical sites',
            '4 \* supply_temperature_mixed_air_limit_count',
            '4 \* supply_temperature_minimum_limit_count',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            'a < b \? a : b',
            'CP333.+bit-exact assigned result',
            'CP329.+finite `mixed_air_temperature_c`',
            'CP333-to-CP334-to-numerical',
            'Physical line 2190 is the first excluded lexical executable and CP335 boundary',
            'both parents remain `scaffold`/`none`',
            'Roadmap state stay unchanged'
        )) {
        if ($cp334AlgorithmAddendum.Value -notmatch $cp334Pattern) {
            throw "CP334 algorithm addendum missing '$cp334Pattern'"
        }
    }
}
foreach ($cp334TargetPattern in @(
        'cooling_positive_supply_temperature_mixed_air_limit/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit',
        'cooling_positive_supply_temperature_mixed_air_limit\.rs::purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle_summary',
        'cooling_positive_supply_temperature_mixed_air_limit\.rs::PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState',
        'cooling_positive_supply_temperature_mixed_air_limit\.rs::PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary'
    )) {
    Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern $cp334TargetPattern -Description "CP334 algorithm target '$cp334TargetPattern'"
}
$cp334CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp334CapabilityAddenda = [regex]::Matches(
    $cp334CapabilityText,
    '(?m)^\s*"CP334 additionally requires[^"\r\n]+",\s*$'
)
if ($cp334CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP334 claim addenda"
}
foreach ($cp334CapabilityAddendum in $cp334CapabilityAddenda) {
    foreach ($cp334Pattern in @(
            'physical executable line 2189',
            'four-site',
            'textual inventory',
            '4 \* supply_temperature_mixed_air_limit_count',
            '4 \* supply_temperature_minimum_limit_count',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            'a < b \? a : b',
            '(?:CP333.+bit-exact result|bit-exact CP333 result)',
            'CP329.+finite `mixed_air_temperature_c`',
            'No duplicate caller scalar, Zone-state re-read',
            'CP333-to-CP334-to-numerical',
            'Physical line 2190 is the first excluded lexical executable and CP335 boundary',
            'This changes no support level',
            'both Calc routines remain `source_mapped`'
        )) {
        if ($cp334CapabilityAddendum.Value -notmatch $cp334Pattern) {
            throw "CP334 capability addendum missing '$cp334Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP334 supersedes only CP333' -Description "generated CP334 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP334 additionally requires' -Description "generated CP334 capability index"

# Each hand-authored contract carries one scoped CP334 section with source,
# operand provenance, transactionality, exclusions, and explicit non-promotion.
$cp334DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP334 maps only the single Cooling positive-supply.*?^Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP334 Source-Ordered Cooling Positive-Supply Temperature Mixed-Air Limit\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP334 Cooling Positive-Supply Temperature Mixed-Air Limit\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP334 Positive-Supply Temperature Mixed-Air Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP334 Cooling Positive-Supply Temperature Mixed-Air Limit Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp334Documentation in $cp334DocumentationSections) {
    $cp334DocumentText = Read-RepoText -Path $cp334Documentation.Path
    $cp334Matches = [regex]::Matches($cp334DocumentText, $cp334Documentation.Pattern)
    if ($cp334Matches.Count -ne 1) {
        throw "CP334 documentation expected one scoped section in $($cp334Documentation.Path), found $($cp334Matches.Count)"
    }
    $cp334Section = $cp334Matches[0].Value
    foreach ($cp334Pattern in @(
            'physical\s+(?:executable\s+)?(?:line\s+)?2189',
            '(?:exactly )?four(?:-site|\s+textual|\s+sites)|all four',
            'a < b \? a : b',
            '4 \* supply_temperature_mixed_air_limit_count',
            '4 \* supply_temperature_minimum_limit_count',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?i)guard-false',
            '(?i)CP333',
            '(?i)CP329',
            'mixed_air_temperature_c',
            '(?i)finite',
            '(?i)latest',
            '(?i)private witness|private-witness',
            '(?i)checked',
            '(?i)transaction|before mutation',
            'purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle',
            'CP333-to-CP334-to-numerical',
            '(?is)(?:does not|neither|without).{0,120}(?:consum(?:e|ing)|reconcil(?:e|ing)).{0,180}numerical\s+DTO',
            'line 2190',
            '(?i)CP335',
            '2340',
            '2454-2461',
            '2465',
            '(?i)scaffold',
            '(?i)source_mapped',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp334Section -notmatch $cp334Pattern) {
            throw "CP334 documentation in $($cp334Documentation.Path) missing '$cp334Pattern'"
        }
    }
}

# Main audit and generated script inventory remain in source-checkpoint order.
$cp334MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp333DotSourceIndexForCp334 = $cp334MainAuditText.IndexOf('ideal-loads-structure-audit\cp333-cooling-positive-supply-temperature-minimum-limit.ps1')
$cp334DotSourceIndex = $cp334MainAuditText.IndexOf('ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1')
$cp334AuditCompletionIndex = $cp334MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp333DotSourceIndexForCp334 -lt 0 -or
    $cp334DotSourceIndex -le $cp333DotSourceIndexForCp334 -or
    $cp334AuditCompletionIndex -le $cp334DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP334 after CP333 and before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp334-cooling-positive-supply-temperature-mixed-air-limit\.ps1"' -Description "CP334 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp334-cooling-positive-supply-temperature-mixed-air-limit\.ps1::dot_sources' -Description "CP334 main-audit callee evidence"
