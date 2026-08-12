# CP333 maps only PurchasedAirManager.cc physical executable line 2187: the
# Cooling positive-supply temperature minimum limit. Physical line 2188 is
# commentary and physical line 2189 is the first excluded executable/CP334 edge.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp333Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit.rs"
$cp333State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\state.rs"
$cp333Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\transition.rs"
$cp333Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\release.rs"
$cp333PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\release\prefix_validation.rs"
$cp333RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\release\runtime_validation.rs"
$cp333SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\release\snapshot_validation.rs"
$cp333Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\tests\mod.rs"
$cp333ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\tests\release_corruption.rs"
$cp333CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp333Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp333Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp333ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp333BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_temperature_minimum_limit.rs"
$cp333BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp333BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_temperature_minimum_limit_tests.rs"
$cp333InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp333InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp333InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp333InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_temperature_minimum_limit.rs"
$cp333CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp333CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_temperature_minimum_limit_validation.rs"
$cp333CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp333CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_temperature_minimum_limit_fixture.rs"
$cp333CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp333PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp333Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_minimum_limit.rs"
$cp333PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_minimum_limit\validation.rs"
$cp333PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_minimum_limit\serialization.rs"
$cp333PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_temperature_minimum_limit\serialization\snapshot.rs"
$cp333RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp333DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_temperature_minimum_limit_assertions.rs"
$cp333NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp333RequiredFile in @(
        $cp333Module,
        $cp333State,
        $cp333Transition,
        $cp333Release,
        $cp333PrefixValidation,
        $cp333RuntimeValidation,
        $cp333SnapshotValidation,
        $cp333Tests,
        $cp333ReleaseCorruptionTests,
        $cp333ScheduledOutput,
        $cp333BindingAdapter,
        $cp333BindingTests,
        $cp333InitWitness,
        $cp333CoupledValidation,
        $cp333CoupledFixture,
        $cp333Pipeline,
        $cp333PipelineValidation,
        $cp333PipelineSerialization,
        $cp333PipelineSnapshotSerialization,
        $cp333DirectAssertions
    )) {
    Assert-FileExists -Path $cp333RequiredFile -Description "CP333 temperature-minimum-limit structure"
}
Assert-LineLimit -Path $cp333Release -Limit 800 -Description "CP333 release root module"
Assert-LineLimit -Path $cp333RuntimeValidation -Limit 800 -Description "CP333 runtime validation module"
Assert-LineLimit -Path $cp333CoupledValidation -Limit 800 -Description "CP333 coupled validation module"
Assert-LineLimit -Path $cp333Pipeline -Limit 800 -Description "CP333 pipeline module"

# Locked source boundary and exact four-site textual inventory.
Assert-Contains -Path $cp333Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2187' -Description "CP333 exact physical source boundary"
Assert-Contains -Path $cp333Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2189' -Description "CP333 first excluded physical executable"
Assert-Contains -Path $cp333Module -Pattern 'Exact four textual source sites represented by CP333' -Description "CP333 exact lexical-site count"
Assert-ExactStringArray -Path $cp333Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER" -Expected @(
    "read-purchased-air-supply-temperature-for-maximum",
    "reread-minimum-cooling-supply-air-temperature-for-maximum",
    "apply-source-shaped-two-argument-maximum",
    "assign-purchased-air-supply-temperature"
) -Description "CP333 exact four-site source order"
Assert-Contains -Path $cp333Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot' -Description "CP333 public snapshot"
Assert-Contains -Path $cp333State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState' -Description "CP333 persistent state"
Assert-Contains -Path $cp333Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary' -Description "CP333 lifecycle summary"
Assert-Contains -Path $cp333Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary\s*\(' -Description "CP333 lifecycle accessor"
Assert-Contains -Path $cp333Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit\s*\(' -Description "CP333 exact-direct wrapper"
Assert-Contains -Path $cp333Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_temperature_minimum_limit_state\s*\(' -Description "CP333 pure transition"
Assert-Contains -Path $cp333CalcRoot -Pattern 'mod cooling_positive_supply_temperature_minimum_limit;' -Description "CP333 calc module declaration"
Assert-Contains -Path $cp333CalcRoot -Pattern 'pub use (?:cooling_positive_supply_temperature_minimum_limit::\*;|\{[^}]*cooling_positive_supply_temperature_minimum_limit::\*)' -Description "CP333 calc public surface"

# ObjexxFCL maximum semantics are strict-< and preserve the left operand on a
# tie or unordered comparison. No broader Rust maximum/clamp policy is allowed.
Assert-Contains -Path $cp333Transition -Pattern '(?s)fn source_shaped_two_argument_maximum\(\s*left: f64,\s*right: f64,\s*\) -> f64 \{\s*if left < right \{ right \} else \{ left \}\s*\}' -Description "CP333 exact ObjexxFCL maximum"
Assert-NotContains -Path $cp333Transition -Pattern 'f64::max|\.max\(|total_cmp|partial_cmp|clamp\(|\.min\(|is_finite|is_nan|normalize' -Description "CP333 forbidden broadened maximum semantics"
Assert-PatternsInOrder -Path $cp333Transition -Patterns @(
    'let limit_executed = predecessor\.supply_temperature_assignment_executed;',
    'let supply_temperature_before_minimum_limit_c',
    'let minimum_cooling_supply_air_temperature_c',
    'source_shaped_two_argument_maximum\(',
    'let assigned_supply_temperature_c = maximum_supply_temperature_c;',
    'state\.transition_count \+= 1;',
    'state\.source_site_execution_count \+=',
    'PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_TEMPERATURE_MINIMUM_LIMIT_SOURCE_ORDER\.len\(\);'
) -Description "CP333 source transition order"
foreach ($cp333Counter in @(
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'supply_temperature_minimum_limit_count',
        'source_site_execution_count',
        'supply_temperature_for_maximum_read_count',
        'minimum_cooling_supply_air_temperature_for_maximum_read_count',
        'source_shaped_two_argument_maximum_evaluation_count',
        'supply_temperature_assignment_count'
    )) {
    Assert-Contains -Path $cp333State -Pattern ('pub ' + $cp333Counter + ':\s*usize') -Description "CP333 state counter '$cp333Counter'"
}
Assert-Contains -Path $cp333RuntimeValidation -Pattern 'supply_temperature_minimum_limit_count\s*\.\s*checked_mul\(4\)' -Description "CP333 four-sites-per-active formula"
Assert-Contains -Path $cp333RuntimeValidation -Pattern '(?s)pending_supply_temperature_minimum_limit_state_is_consistent\(.*?unit_off_skip_count.*?predecessor_state\.unit_off_skip_count.*?non_cooling_skip_count.*?predecessor_state\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor_state\.positive_guard_false_fallthrough_skip_count.*?supply_temperature_minimum_limit_count.*?predecessor_state\.supply_temperature_assignment_count' -Description "CP333 pending CP332 four-route parity"
Assert-Contains -Path $cp333RuntimeValidation -Pattern '(?s)completed_supply_temperature_minimum_limit_state_is_consistent\(.*?state\.transition_count == predecessor\.transition_count.*?state\.unit_off_skip_count == predecessor\.unit_off_skip_count.*?state\.non_cooling_skip_count == predecessor\.non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?predecessor\.positive_guard_false_fallthrough_skip_count.*?supply_temperature_minimum_limit_count.*?predecessor\.supply_temperature_assignment_count' -Description "CP333 completed CP332 four-route parity"
foreach ($cp333PreflightCounter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'witnessed_positive_guard_false_fallthrough_skip_count',
        'supply_temperature_minimum_limit_count',
        'source_site_execution_count',
        'supply_temperature_for_maximum_read_count',
        'minimum_cooling_supply_air_temperature_for_maximum_read_count',
        'source_shaped_two_argument_maximum_evaluation_count',
        'supply_temperature_assignment_count',
        'witnessed_supply_temperature_minimum_limit_count'
    )) {
    Assert-Contains -Path $cp333RuntimeValidation -Pattern ($cp333PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP333 checked preflight '$cp333PreflightCounter'"
}

# Snapshot proof distinguishes four routes and retains all active values by bits.
foreach ($cp333SnapshotField in @(
        'supply_temperature_before_minimum_limit_c',
        'minimum_cooling_supply_air_temperature_c',
        'maximum_supply_temperature_c',
        'assigned_supply_temperature_c'
    )) {
    Assert-Contains -Path $cp333SnapshotValidation -Pattern $cp333SnapshotField -Description "CP333 snapshot field '$cp333SnapshotField'"
}
Assert-Contains -Path $cp333SnapshotValidation -Pattern 'fn limited_snapshot_is_exact' -Description "CP333 active snapshot proof"
Assert-Contains -Path $cp333SnapshotValidation -Pattern 'fn skipped_snapshot_is_exact' -Description "CP333 skipped snapshot proof"
Assert-Contains -Path $cp333SnapshotValidation -Pattern 'fn snapshots_match_bit_exact' -Description "CP333 bit-exact retained matcher"
Assert-Contains -Path $cp333SnapshotValidation -Pattern 'source_shaped_two_argument_maximum\(left, right\)' -Description "CP333 snapshot source maximum"
Assert-Contains -Path $cp333SnapshotValidation -Pattern 'to_bits\(\)' -Description "CP333 exact-bit snapshot checks"

# Release owns no numerical DTO or duplicate scalar. CP332 owns the left
# operand; the selected typed system owns the right; CP318 is lineage only.
Assert-Contains -Path $cp333Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp332: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,\s*\)' -Description "CP333 exact wrapper arguments"
Assert-Contains -Path $cp333Release -Pattern 'predecessor_cp332\.supply_temperature_c' -Description "CP333 CP332 left-operand provenance"
Assert-Contains -Path $cp333Release -Pattern 'system\.minimum_cooling_supply_air_temperature_c' -Description "CP333 typed-system right-operand reread"
Assert-Contains -Path $cp333PrefixValidation -Pattern 'sensible_flow\.minimum_cooling_supply_air_temperature_c' -Description "CP333 CP318 retained minimum lineage"
Assert-Contains -Path $cp333PrefixValidation -Pattern 'predecessor\.supply_temperature_c' -Description "CP333 retained CP332 assignment lineage"
Assert-Contains -Path $cp333Release -Pattern 'completed_direct_cooling_positive_supply_temperature_assignment_is_consistent\s*\(' -Description "CP333 recursive CP332 completed proof"
Assert-Contains -Path $cp333Release -Pattern 'cooling_positive_supply_temperature_assignment_latest_witness\s*\(' -Description "CP333 retained CP332 private witness"
Assert-Contains -Path $cp333Release -Pattern 'cooling_positive_supply_temperature_minimum_limit_latest_witness\s*\(' -Description "CP333 private latest witness"
Assert-NotContains -Path $cp333Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit\([^)]*(?:supply_temperature|minimum_cooling)\w*\s*:\s*f64' -Description "duplicate caller scalar in CP333 release"
Assert-NotContains -Path $cp333Release -Pattern 'complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "numerical DTO dependency in CP333 release"
Assert-PatternsInOrder -Path $cp333Release -Patterns @(
    'pending_supply_temperature_minimum_limit_state_is_consistent\(',
    'next_supply_temperature_minimum_limit_transition_fits\(',
    'completed_direct_cooling_positive_supply_temperature_assignment_is_consistent\(',
    'advance_cooling_positive_supply_temperature_minimum_limit_state\(',
    'set_cooling_positive_supply_temperature_minimum_limit_latest_witness\('
) -Description "CP333 validate-before-mutation order"

# Pure and release tests must cover source selection, all skips, provenance,
# replay/corruption, and checked-overflow transactionality.
foreach ($cp333TestPattern in @(
        'signed_zero|zero',
        'NaN|nan',
        'infinity|infinite',
        'UnitOff|unit_off',
        'non_cooling',
        'guard_false',
        'four.*site|source_site',
        'counter'
    )) {
    Assert-Contains -Path $cp333Tests -Pattern $cp333TestPattern -Description "CP333 pure regression '$cp333TestPattern'"
}
foreach ($cp333CorruptionPattern in @(
        'replay',
        'forged|corrupt',
        'witness',
        'overflow',
        'without_mutation|transaction'
    )) {
    Assert-Contains -Path $cp333ReleaseCorruptionTests -Pattern $cp333CorruptionPattern -Description "CP333 release regression '$cp333CorruptionPattern'"
}

# Runtime-root witness ownership remains private and per-unit state is explicit.
Assert-NotContains -Path $cp333InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_temperature_minimum_limit_latest_witnesses:' -Description "public runtime-root CP333 witness map"
Assert-Contains -Path $cp333InitWitnessRoot -Pattern 'mod cooling_positive_supply_temperature_minimum_limit;' -Description "runtime-root CP333 witness module"
Assert-Contains -Path $cp333InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_temperature_minimum_limit_latest_witness\s*\(' -Description "runtime-root CP333 witness getter"
Assert-Contains -Path $cp333InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_temperature_minimum_limit_latest_witness\s*\(' -Description "runtime-root CP333 witness setter"
Assert-Contains -Path $cp333InitState -Pattern 'pub calc_cooling_positive_supply_temperature_minimum_limit:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState' -Description "per-unit CP333 persistent state"
Assert-Contains -Path $cp333InitUnit -Pattern '(?s)calc_cooling_positive_supply_temperature_minimum_limit:\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new\(\s*system\s*,?\s*\)' -Description "per-unit CP333 state initialization"

# Binding must be CP332 -> CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339
# -> numerical with no hidden source helper.
$cp333BindingText = Read-RepoText -Path $cp333Binding
$cp332BindingIndexForCp333 = $cp333BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_assignment =")
$cp333BindingIndex = $cp333BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_minimum_limit =")
$cp334BindingIndexForCp333 = $cp333BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_mixed_air_limit =")
$cp335BindingIndexForCp333 = $cp333BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndexForCp333 = $cp333BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp333 = $cp333BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp333 = $cp333BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp333 = $cp333BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp333 = $cp333BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp332BindingIndexForCp333 -lt 0 -or
    $cp333BindingIndex -le $cp332BindingIndexForCp333 -or
    $cp334BindingIndexForCp333 -le $cp333BindingIndex -or
    $cp335BindingIndexForCp333 -le $cp334BindingIndexForCp333 -or
    $cp336BindingIndexForCp333 -le $cp335BindingIndexForCp333 -or
    $cp337BindingIndexForCp333 -le $cp336BindingIndexForCp333 -or
    $cp338BindingIndexForCp333 -le $cp337BindingIndexForCp333 -or
    $cp339BindingIndexForCp333 -le $cp338BindingIndexForCp333 -or
    $numericalBindingIndexForCp333 -le $cp339BindingIndexForCp333
) {
    throw "Binding must retain exact CP332 -> CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $cp333Binding -Pattern '(?s)let calculation_cooling_positive_supply_temperature_minimum_limit =\s*advance_positive_supply_temperature_minimum_limit\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_temperature_assignment,\s*\)\?;' -Description "binding exact CP332-to-CP333 adapter call"
Assert-Contains -Path $cp333BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_temperature_minimum_limit\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,' -Description "CP333 binding adapter arguments"
Assert-Contains -Path $cp333BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit\(\s*runtime,\s*system,\s*predecessor,\s*\).*?CalculationCoolingPositiveSupplyTemperatureMinimumLimit' -Description "CP333 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp333BindingAdapter -Pattern 'zone_state|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "extra operand or numerical DTO in CP333 adapter"
Assert-Contains -Path $cp333ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_temperature_minimum_limit:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot' -Description "CP333 scheduled output evidence"
Assert-Contains -Path $cp333BindingTestsRoot -Pattern '#\[rustfmt::skip\]\s*#\[path = "binding/cooling_positive_supply_temperature_minimum_limit_tests\.rs"\]\s*mod cooling_positive_supply_temperature_minimum_limit_tests;' -Description "CP333 binding test module"
Assert-Contains -Path $cp333BindingTests -Pattern 'calculation_cooling_positive_supply_temperature_minimum_limit' -Description "CP333 binding regressions"
$cp333BindingCall = [regex]::Match(
    $cp333BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_minimum_limit =\s*advance_positive_supply_temperature_minimum_limit\([^;]+?\)\?;'
)
if (-not $cp333BindingCall.Success) {
    throw "Binding must retain the complete CP333 exact release call"
}
$cp333BindingCallEnd = $cp333BindingCall.Index + $cp333BindingCall.Length
$cp334BindingCallForCp333 = [regex]::Match(
    $cp333BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
if (-not $cp334BindingCallForCp333.Success) {
    throw "Binding must retain the complete CP334 exact release call after CP333"
}
$cp334BindingCallEndForCp333 =
    $cp334BindingCallForCp333.Index + $cp334BindingCallForCp333.Length
$cp335BindingCallForCp333 = [regex]::Match(
    $cp333BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
if (-not $cp335BindingCallForCp333.Success) {
    throw "Binding must retain the complete CP335 exact release call after CP334"
}
$cp335BindingCallEndForCp333 =
    $cp335BindingCallForCp333.Index + $cp335BindingCallForCp333.Length
$cp336BindingCallForCp333 = [regex]::Match(
    $cp333BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
if (-not $cp336BindingCallForCp333.Success) {
    throw "Binding must retain the complete CP336 exact release call after CP335"
}
$cp336BindingCallEndForCp333 =
    $cp336BindingCallForCp333.Index + $cp336BindingCallForCp333.Length
$cp337BindingCallForCp333 = [regex]::Match(
    $cp333BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
if (-not $cp337BindingCallForCp333.Success) {
    throw "Binding must retain the complete CP337 exact release call after CP336"
}
$cp337BindingCallEndForCp333 =
    $cp337BindingCallForCp333.Index + $cp337BindingCallForCp333.Length
$cp338BindingCallForCp333 = [regex]::Match(
    $cp333BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
if (-not $cp338BindingCallForCp333.Success) {
    throw "Binding must retain the complete CP338 exact release call after CP337"
}
$cp338BindingCallEndForCp333 =
    $cp338BindingCallForCp333.Index + $cp338BindingCallForCp333.Length
$cp339BindingCallForCp333 = [regex]::Match(
    $cp333BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (-not $cp339BindingCallForCp333.Success) {
    throw "Binding must retain the complete CP339 exact release call after CP338"
}
$cp339BindingCallEndForCp333 =
    $cp339BindingCallForCp333.Index + $cp339BindingCallForCp333.Length
if (
    $cp334BindingIndexForCp333 -lt $cp333BindingCallEnd -or
    $cp335BindingIndexForCp333 -lt $cp334BindingCallEndForCp333 -or
    $cp336BindingIndexForCp333 -lt $cp335BindingCallEndForCp333 -or
    $cp337BindingIndexForCp333 -lt $cp336BindingCallEndForCp333 -or
    $cp338BindingIndexForCp333 -lt $cp337BindingCallEndForCp333 -or
    $cp339BindingIndexForCp333 -lt $cp338BindingCallEndForCp333 -or
    $numericalBindingIndexForCp333 -lt $cp339BindingCallEndForCp333
) {
    throw "CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
$postCp333BeforeCp334 = $cp333BindingText.Substring(
    $cp333BindingCallEnd,
    $cp334BindingIndexForCp333 - $cp333BindingCallEnd
)
$postCp333BeforeCp334Code = [regex]::Replace($postCp333BeforeCp334, '(?m)//.*$', '')
if ($postCp333BeforeCp334Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP333 and before CP334"
}
$postCp334BeforeCp335ForCp333 = $cp333BindingText.Substring(
    $cp334BindingCallEndForCp333,
    $cp335BindingIndexForCp333 - $cp334BindingCallEndForCp333
)
$postCp334BeforeCp335CodeForCp333 =
    [regex]::Replace($postCp334BeforeCp335ForCp333, '(?m)//.*$', '')
if ($postCp334BeforeCp335CodeForCp333 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP334 and before CP335"
}
$postCp335BeforeCp336ForCp333 = $cp333BindingText.Substring(
    $cp335BindingCallEndForCp333,
    $cp336BindingIndexForCp333 - $cp335BindingCallEndForCp333
)
$postCp335BeforeCp336CodeForCp333 =
    [regex]::Replace($postCp335BeforeCp336ForCp333, '(?m)//.*$', '')
if ($postCp335BeforeCp336CodeForCp333 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP335 and before CP336"
}
$postCp336BeforeCp337ForCp333 = $cp333BindingText.Substring(
    $cp336BindingCallEndForCp333,
    $cp337BindingIndexForCp333 - $cp336BindingCallEndForCp333
)
$postCp336BeforeCp337CodeForCp333 =
    [regex]::Replace($postCp336BeforeCp337ForCp333, '(?m)//.*$', '')
if ($postCp336BeforeCp337CodeForCp333 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP336 and before CP337"
}
$postCp337BeforeCp338ForCp333 = $cp333BindingText.Substring(
    $cp337BindingCallEndForCp333,
    $cp338BindingIndexForCp333 - $cp337BindingCallEndForCp333
)
$postCp337BeforeCp338CodeForCp333 =
    [regex]::Replace($postCp337BeforeCp338ForCp333, '(?m)//.*$', '')
if ($postCp337BeforeCp338CodeForCp333 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP337 and before CP338"
}
$postCp338BeforeCp339ForCp333 = $cp333BindingText.Substring(
    $cp338BindingCallEndForCp333,
    $cp339BindingIndexForCp333 - $cp338BindingCallEndForCp333
)
$postCp338BeforeCp339CodeForCp333 =
    [regex]::Replace($postCp338BeforeCp339ForCp333, '(?m)//.*$', '')
if ($postCp338BeforeCp339CodeForCp333 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP338 and before CP339"
}
$postCp339BeforeNumericalForCp333 = $cp333BindingText.Substring(
    $cp339BindingCallEndForCp333,
    $numericalBindingIndexForCp333 - $cp339BindingCallEndForCp333
)
$postCp339BeforeNumericalCodeForCp333 =
    [regex]::Replace($postCp339BeforeNumericalForCp333, '(?m)//.*$', '')
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\([^;]+?\)\?;)',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp333 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp333,
    '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\([^;]+?\)\?;)',
    ''
)
if ($postCp339BeforeNumericalCodeForCp333 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No helper other than the audited CP340 through CP409 releases may execute after CP339 and before numerical Calc"
}

# Coupled runtime and pipeline expose direct-only CP333 evidence.
Assert-Contains -Path $cp333CoupledRuntime -Pattern 'mod cooling_positive_supply_temperature_minimum_limit_validation;' -Description "coupled CP333 validator declaration"
Assert-Contains -Path $cp333CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_temperature_minimum_limit_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary' -Description "coupled CP333 lifecycle"
Assert-Contains -Path $cp333CoupledRuntime -Pattern 'cooling_positive_supply_temperature_minimum_limit_validation::snapshot_matches_release' -Description "coupled per-timestep CP333 validation"
Assert-Contains -Path $cp333CoupledRuntime -Pattern 'cooling_positive_supply_temperature_minimum_limit_validation::validate_lifecycle' -Description "coupled final CP333 validation"
Assert-Contains -Path $cp333CoupledValidation -Pattern 'calculation_cooling_positive_supply_temperature_assignment' -Description "coupled CP332 predecessor"
Assert-Contains -Path $cp333CoupledValidation -Pattern 'calculation_cooling_positive_supply_temperature_minimum_limit' -Description "coupled CP333 result"
Assert-Contains -Path $cp333CoupledValidation -Pattern 'checked_mul\(4\)|SOURCE_ORDER.*len\(\)' -Description "coupled CP333 four-site count"
Assert-Contains -Path $cp333CoupledValidation -Pattern 'to_bits\(\)|options_have_exact_bits' -Description "coupled CP333 exact-bit validation"
Assert-Contains -Path $cp333CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_temperature_minimum_limit_fixture;' -Description "coupled CP333 fixture declaration"
Assert-Contains -Path $cp333CoupledFixture -Pattern 'calculation_cooling_positive_supply_temperature_minimum_limit' -Description "coupled CP333 fixture output"
Assert-Contains -Path $cp333CoupledTests -Pattern 'cooling_positive_supply_temperature_minimum_limit' -Description "coupled CP333 regressions"
Assert-Contains -Path $cp333PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_temperature_minimum_limit;' -Description "pipeline CP333 module declaration"
Assert-Contains -Path $cp333PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle' -Description "pipeline CP333 lifecycle field and JSON key"
Assert-Contains -Path $cp333PipelineRoot -Pattern 'calc_cooling_positive_supply_temperature_minimum_limit_lifecycle' -Description "pipeline CP333 coupled lifecycle transfer"
Assert-Contains -Path $cp333Pipeline -Pattern 'validate_direct_lifecycle' -Description "pipeline CP333 direct lifecycle validation"
Assert-Contains -Path $cp333PipelineValidation -Pattern 'validate_source_counters' -Description "pipeline CP333 source-counter validation"
Assert-Contains -Path $cp333PipelineSerialization -Pattern 'minimum_cooling_supply_air_temperature_for_maximum_read_count' -Description "pipeline CP333 lifecycle serialization"
Assert-Contains -Path $cp333PipelineSnapshotSerialization -Pattern 'minimum_cooling_supply_air_temperature_for_maximum_read' -Description "pipeline CP333 snapshot serialization"
Assert-Contains -Path $cp333RunTests -Pattern 'cooling_positive_supply_temperature_minimum_limit_assertions' -Description "direct-run CP333 assertion module"
Assert-Contains -Path $cp333DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle' -Description "direct-run CP333 JSON evidence"
Assert-Contains -Path $cp333DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*4\]\s*=' -Description "direct-run CP333 source-order declaration"
Assert-Contains -Path $cp333DirectAssertions -Pattern '(?:executions|assignments) \* (?:SOURCE_ORDER\.len\(\) as u64|4)' -Description "direct-run CP333 dynamic source-site count"
Assert-Contains -Path $cp333NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle' -Description "non-direct CP333 null evidence"

# Registries repeat the boundary exactly twice and add only target inventory.
$cp333AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp333AlgorithmAddenda = [regex]::Matches(
    $cp333AlgorithmText,
    '(?m)^\s*"CP333 supersedes only CP332[^"\r\n]+",\s*$'
)
if ($cp333AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP333 boundary addenda"
}
foreach ($cp333AlgorithmAddendum in $cp333AlgorithmAddenda) {
    foreach ($cp333Pattern in @(
            'physical executable line 2187',
            'exactly four lexical sites',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            'a < b \? b : a',
            'bit-exact assigned result',
            'minimum_cooling_supply_air_temperature_c',
            'CP318',
            'CP332-to-CP333-to-numerical',
            'physical line 2189 is the first excluded lexical executable and CP334 boundary',
            'both parents remain `scaffold`/`none`',
            'Roadmap state stay unchanged'
        )) {
        if ($cp333AlgorithmAddendum.Value -notmatch $cp333Pattern) {
            throw "CP333 algorithm addendum missing '$cp333Pattern'"
        }
    }
}
foreach ($cp333TargetPattern in @(
        'cooling_positive_supply_temperature_minimum_limit/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit',
        'cooling_positive_supply_temperature_minimum_limit\.rs::purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary',
        'cooling_positive_supply_temperature_minimum_limit\.rs::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState',
        'cooling_positive_supply_temperature_minimum_limit\.rs::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary'
    )) {
    Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern $cp333TargetPattern -Description "CP333 algorithm target '$cp333TargetPattern'"
}
$cp333CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp333CapabilityAddenda = [regex]::Matches(
    $cp333CapabilityText,
    '(?m)^\s*"CP333 additionally requires[^"\r\n]+",\s*$'
)
if ($cp333CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP333 claim addenda"
}
foreach ($cp333CapabilityAddendum in $cp333CapabilityAddenda) {
    foreach ($cp333Pattern in @(
            'physical executable line 2187',
            'four-site',
            'textual inventory',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            'a < b \? b : a',
            'bit-exact CP332 result',
            'minimum_cooling_supply_air_temperature_c',
            'CP318',
            'same-call lineage evidence, not a substitute operand',
            'No duplicate caller scalar',
            'CP332-to-CP333-to-numerical',
            'physical line 2189 is the first excluded lexical executable and CP334 boundary',
            'This changes no support level',
            'both Calc routines remain `source_mapped`'
        )) {
        if ($cp333CapabilityAddendum.Value -notmatch $cp333Pattern) {
            throw "CP333 capability addendum missing '$cp333Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP333 supersedes only CP332' -Description "generated CP333 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP333 additionally requires' -Description "generated CP333 capability index"

# Historical CP332 sections remain separate; each hand doc has one new CP333
# section with source, provenance, exclusions, and explicit non-promotion.
$cp333DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP333 maps only the single Cooling positive-supply.*?^conformance, capability, and Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP333 Source-Ordered Cooling Positive-Supply Temperature Minimum Limit\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP333 Cooling Positive-Supply Temperature Minimum Limit\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP333 Positive-Supply Temperature Minimum Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP333 Cooling Positive-Supply Temperature Minimum Limit Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp333Documentation in $cp333DocumentationSections) {
    $cp333DocumentText = Read-RepoText -Path $cp333Documentation.Path
    $cp333Matches = [regex]::Matches($cp333DocumentText, $cp333Documentation.Pattern)
    if ($cp333Matches.Count -ne 1) {
        throw "CP333 documentation expected one scoped section in $($cp333Documentation.Path), found $($cp333Matches.Count)"
    }
    $cp333Section = $cp333Matches[0].Value
    foreach ($cp333Pattern in @(
            'physical\s+(?:executable\s+)?(?:line\s+)?2187',
            '(?:exactly )?four(?:-site|\s+textual|\s+sites)|all four',
            'a < b \? b : a',
            '4 \* supply_temperature_assignment_count',
            '4 \* cp_air_assignment_count',
            '4 \* positive_supply_mass_flow_body_entries',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?i)guard-false',
            '(?i)CP332',
            'minimum_cooling_supply_air_temperature_c',
            '(?i)CP318',
            '(?i)lineage',
            '(?i)checked',
            '(?i)transaction|before mutation',
            'purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle',
            'CP332-to-CP333-to-numerical',
            '(?is)(?:does not|neither|without).{0,120}(?:consum(?:e|ing)|reconcil(?:e|ing)).{0,180}numerical\s+DTO',
            'line 2189',
            '(?i)CP334',
            '2340',
            '2454-2461',
            '2465',
            '(?i)scaffold',
            '(?i)source_mapped',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp333Section -notmatch $cp333Pattern) {
            throw "CP333 documentation in $($cp333Documentation.Path) missing '$cp333Pattern'"
        }
    }
}

# Main audit order and generated script inventory.
$cp333MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp332DotSourceIndexForCp333 = $cp333MainAuditText.IndexOf('ideal-loads-structure-audit\cp332-cooling-positive-supply-temperature-assignment.ps1')
$cp333DotSourceIndex = $cp333MainAuditText.IndexOf('ideal-loads-structure-audit\cp333-cooling-positive-supply-temperature-minimum-limit.ps1')
$cp333AuditCompletionIndex = $cp333MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp332DotSourceIndexForCp333 -lt 0 -or
    $cp333DotSourceIndex -le $cp332DotSourceIndexForCp333 -or
    $cp333AuditCompletionIndex -le $cp333DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP333 after CP332 and before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp333-cooling-positive-supply-temperature-minimum-limit\.ps1"' -Description "CP333 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp333-cooling-positive-supply-temperature-minimum-limit\.ps1::dot_sources' -Description "CP333 main-audit callee evidence"
