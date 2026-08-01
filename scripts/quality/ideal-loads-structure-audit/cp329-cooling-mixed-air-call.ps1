# CP329 maps only the complete Cooling CalcPurchAirMixedAir call and the
# bounded direct no-outdoor-air child fallback.

$cp329Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call.rs"
$cp329State = "crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\state.rs"
$cp329Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\transition.rs"
$cp329Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\release.rs"
$cp329RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\release\runtime_validation.rs"
$cp329ReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\release_tests.rs"
$cp329Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\tests.rs"
$cp329Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp329Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp329ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp329BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp329BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_mixed_air_call_tests.rs"
$cp329InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp329InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp329InitWitnesses = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_mixed_air_call.rs"
$cp329CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp329CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_mixed_air_call_validation.rs"
$cp329CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_mixed_air_call_fixture.rs"
$cp329CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp329PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp329Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_mixed_air_call.rs"
$cp329PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_mixed_air_call\validation.rs"
$cp329PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_mixed_air_call\serialization.rs"
$cp329PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_mixed_air_call\serialization\snapshot.rs"
$cp329RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp329DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_mixed_air_call_assertions.rs"

foreach ($cp329RequiredFile in @(
        $cp329Module,
        $cp329State,
        $cp329Transition,
        $cp329Release,
        $cp329RuntimeValidation,
        $cp329ReleaseTests,
        $cp329Tests,
        $cp329Binding,
        $cp329ScheduledOutput,
        $cp329BindingTestsRoot,
        $cp329BindingTests,
        $cp329InitState,
        $cp329InitWitnessRoot,
        $cp329InitWitnesses,
        $cp329CoupledRuntime,
        $cp329CoupledValidation,
        $cp329CoupledFixture,
        $cp329CoupledTests,
        $cp329PipelineRoot,
        $cp329Pipeline,
        $cp329PipelineValidation,
        $cp329PipelineSerialization,
        $cp329PipelineSnapshotSerialization,
        $cp329RunTests,
        $cp329DirectAssertions
    )) {
    Assert-FileExists -Path $cp329RequiredFile -Description "CP329 Cooling mixed-air-call structure"
}
Assert-LineLimit -Path $cp329Release -Limit 800 -Description "CP329 release root module"
Assert-LineLimit -Path $cp329RuntimeValidation -Limit 800 -Description "CP329 runtime validation module"

# The parent statement has nine textual sites. The child inventory is the
# exact source-shaped direct no-OA route, not the complete CP285 child.
Assert-Contains -Path $cp329Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2171-2178' -Description "CP329 exact caller source boundary"
Assert-Contains -Path $cp329Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2812-2939; bounded no-OA route 2851,2854-2861,2869-2874,2876,2878,2932-2937' -Description "CP329 bounded child source boundary"
Assert-Contains -Path $cp329Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2183' -Description "CP329 first excluded executable"
Assert-Contains -Path $cp329Module -Pattern 'Nine textual caller sites; this inventory claims no C\+\+ argument evaluation order' -Description "CP329 argument-order nonclaim"
Assert-ExactStringArray -Path $cp329Module -Name "PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER" -Expected @(
    "bind-state-reference",
    "read-purchased-air-number",
    "read-outdoor-air-mass-flow-rate",
    "read-supply-mass-flow-rate",
    "bind-mixed-air-temperature-output-reference",
    "bind-mixed-air-humidity-ratio-output-reference",
    "bind-mixed-air-enthalpy-output-reference",
    "read-operating-mode",
    "call-calc-purch-air-mixed-air"
) -Description "CP329 exact nine-site caller order"
Assert-ExactStringArray -Path $cp329Module -Name "PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER" -Expected @(
    "bind-purchased-air-alias",
    "copy-outdoor-air-node-number",
    "copy-recirculation-node-number",
    "initialize-recirculation-mass-flow-rate-positive-zero",
    "read-recirculation-temperature",
    "read-recirculation-humidity-ratio",
    "read-recirculation-enthalpy-projection",
    "evaluate-outdoor-air-initialization-guard",
    "assign-outdoor-air-inlet-temperature-positive-zero",
    "assign-outdoor-air-inlet-humidity-ratio-positive-zero",
    "assign-outdoor-air-inlet-enthalpy-positive-zero",
    "assign-outdoor-air-after-heat-recovery-temperature",
    "assign-outdoor-air-after-heat-recovery-humidity-ratio",
    "assign-outdoor-air-after-heat-recovery-enthalpy",
    "assign-heat-recovery-on-false",
    "evaluate-outdoor-air-active-guard-first-operand",
    "assign-recirculation-mass-flow-rate-from-supply",
    "assign-mixed-air-temperature",
    "assign-mixed-air-humidity-ratio",
    "assign-mixed-air-enthalpy-projection",
    "assign-heat-recovery-sensible-output-positive-zero",
    "assign-heat-recovery-latent-output-positive-zero"
) -Description "CP329 exact direct no-OA child order"

Assert-Contains -Path $cp329Module -Pattern 'pub struct PurchasedAirCalcCoolingMixedAirCallSnapshot' -Description "CP329 public snapshot"
Assert-Contains -Path $cp329State -Pattern 'pub struct PurchasedAirCalcCoolingMixedAirCallRuntimeState' -Description "CP329 persistent state"
Assert-Contains -Path $cp329Module -Pattern 'pub struct PurchasedAirCalcCoolingMixedAirCallLifecycleSummary' -Description "CP329 lifecycle summary"
Assert-Contains -Path $cp329Module -Pattern 'pub fn purchased_air_calc_cooling_mixed_air_call_lifecycle_summary\s*\(' -Description "CP329 lifecycle accessor"
Assert-Contains -Path $cp329Module -Pattern 'mod release_tests;' -Description "CP329 release regression module"
Assert-Contains -Path $cp329Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_mixed_air_call\s*\(' -Description "CP329 exact-direct release wrapper"
Assert-Contains -Path $cp329Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_mixed_air_call_state\s*\(' -Description "CP329 pure transition"

# UnitOff and non-cooling skip the entire call. Every active Cooling
# predecessor executes all nine caller sites and all 22 bounded child sites,
# including a zero-flow CP328 result.
Assert-PatternsInOrder -Path $cp329Transition -Patterns @(
    'state\.transition_count \+= 1;',
    'if predecessor\.unit_off_skipped',
    'state\.unit_off_skip_count \+= 1;',
    'else if predecessor\.non_cooling_skipped',
    'state\.non_cooling_skip_count \+= 1;',
    'state\.cooling_call_count \+= 1;',
    'state\.caller_source_site_execution_count \+=',
    'PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER\.len\(\);',
    'state\.child_source_site_execution_count \+=',
    'PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER\.len\(\);',
    'state\.state_reference_bind_count \+= 1;',
    'state\.purchased_air_number_read_count \+= 1;',
    'state\.outdoor_air_mass_flow_rate_read_count \+= 1;',
    'state\.supply_mass_flow_rate_read_count \+= 1;',
    'state\.mixed_air_output_reference_bind_count \+= 3;',
    'state\.operating_mode_read_count \+= 1;',
    'state\.mixed_air_child_call_count \+= 1;',
    'state\.no_outdoor_air_fallback_count \+= 1;',
    'state\.recirculation_enthalpy_projection_count \+= 1;',
    'state\.mixed_air_output_assignment_count \+= 3;',
    'state\.heat_recovery_output_positive_zero_assignment_count \+= 2;'
) -Description "CP329 route partition and exact source-site counters"
Assert-Contains -Path $cp329Transition -Pattern 'let zero = active_input\.map\(\|_\| 0\.0_f64\);' -Description "CP329 source positive-zero materialization"
Assert-Contains -Path $cp329Transition -Pattern 'operating_mode: cooling\.then_some\(IdealLoadsSensibleMode::Cooling\)' -Description "CP329 Cooling mode read"
Assert-Contains -Path $cp329Transition -Pattern 'outdoor_air_enabled: active_input\.map\(\|_\| false\)' -Description "CP329 first OutdoorAir false result"
Assert-Contains -Path $cp329Transition -Pattern 'outdoor_air_mass_flow_positive_comparison_evaluated: false' -Description "CP329 short-circuited OA-flow comparison"
Assert-Contains -Path $cp329Transition -Pattern 'resulting_recirculation_mass_flow_rate_kg_per_s: active_input[\r\n\s.]+map\(\|input\| input\.supply_mass_flow_rate_kg_per_s\)' -Description "CP329 dead child recirculation-flow assignment"
Assert-Contains -Path $cp329Transition -Pattern 'mixed_air_temperature_c: active_input\.map\(\|input\| input\.recirculation_temperature_c\)' -Description "CP329 mixed temperature copy"
Assert-Contains -Path $cp329Transition -Pattern 'mixed_air_humidity_ratio: active_input\.map\(\|input\| input\.recirculation_humidity_ratio\)' -Description "CP329 mixed humidity copy"
Assert-Contains -Path $cp329Transition -Pattern 'mixed_air_enthalpy_projection_j_per_kg: active_input[\r\n\s.]+map\(\|input\| input\.recirculation_enthalpy_projection_j_per_kg\)' -Description "CP329 mixed coherent-enthalpy copy"
Assert-Contains -Path $cp329Transition -Pattern 'heat_recovery_sensible_output_w: zero' -Description "CP329 sensible recovery positive zero"
Assert-Contains -Path $cp329Transition -Pattern 'heat_recovery_latent_output_w: zero' -Description "CP329 latent recovery positive zero"
Assert-NotContains -Path $cp329Transition -Pattern 'moist_air_enthalpy_j_per_kg|mixed_air_state|psychrometric|complete_direct_zone_purchased_air_coupling|latest_numerical|TimeHtRecActive|time_ht_rec_active' -Description "CP329 child replacement, numerical DTO, or heat-recovery-time write"

foreach ($cp329Test in @(
        "active_cooling_calls_child_for_positive_and_zero_supply",
        "unit_off_and_non_cooling_skip_all_call_and_child_sites",
        "release_predicate_rejects_negative_zero_recovery_write"
    )) {
    Assert-Contains -Path $cp329Tests -Pattern $cp329Test -Description "CP329 pure regression '$cp329Test'"
}

# Release consumes the completed same-call CP328 snapshot and its private
# witness. It derives only the admitted coherent enthalpy projection from
# already-bound direct Zone state and consumes retained no-OA positive zero.
Assert-Contains -Path $cp329Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_mixed_air_call\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp328: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,\s*zone_state: &ZoneHeatBalanceState,\s*\)' -Description "CP329 exact wrapper arguments"
Assert-Contains -Path $cp329Release -Pattern 'cooling_supply_mass_flow_very_small_guard_body_latest_witness\s*\(' -Description "CP329 retained CP328 private witness"
Assert-Contains -Path $cp329Release -Pattern 'unit[\r\n\s.]+calc_cooling_supply_mass_flow_very_small_guard_body[\r\n\s.]+latest' -Description "CP329 retained CP328 latest snapshot"
Assert-Contains -Path $cp329Release -Pattern 'cooling_mixed_air_call_predecessors_match_bit_exact\s*\(' -Description "CP329 bit-exact CP328 validation"
Assert-Contains -Path $cp329Release -Pattern 'cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release\s*\(' -Description "CP329 completed CP328 validation"
Assert-Contains -Path $cp329Release -Pattern 'mod runtime_validation;' -Description "CP329 split runtime hardening module"
Assert-Contains -Path $cp329RuntimeValidation -Pattern '(?s)pub\(in crate::ideal_loads::calc\) fn completed_direct_cooling_mixed_air_call_is_consistent\(\s*runtime: &PurchasedAirRuntimeState,\s*unit: &PurchasedAirUnitRuntimeState,\s*system: &IdealLoadsAirSystem,\s*snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot,\s*witness: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>' -Description "CP329 runtime-aware completed helper"
Assert-Contains -Path $cp329RuntimeValidation -Pattern '(?s)completed_direct_cooling_supply_mass_flow_very_small_guard_body_is_consistent\(\s*runtime,\s*unit,\s*system,\s*predecessor,\s*runtime\.cooling_supply_mass_flow_very_small_guard_body_latest_witness\(system\.id\),\s*\)' -Description "CP329 recursive CP328 completed/private-witness proof"
Assert-Contains -Path $cp329RuntimeValidation -Pattern 'completed_mixed_air_history_links_to_predecessor\(unit\)' -Description "CP329 completed predecessor-history parity"
Assert-Contains -Path $cp329RuntimeValidation -Pattern '(?s)fn completed_mixed_air_history_links_to_predecessor\(.*?state\.transition_count == predecessor\.transition_count.*?state\.unit_off_skip_count == predecessor\.unit_off_skip_count.*?state\.non_cooling_skip_count == predecessor\.non_cooling_skip_count.*?state\.cooling_call_count == predecessor\.cooling_body_entry_count' -Description "CP329 exact completed route-history equality"
Assert-Contains -Path $cp329RuntimeValidation -Pattern '(?s)fn pending_mixed_air_history_links_to_predecessor\(.*?unit_off_skip_count.*?checked_add\(usize::from\(predecessor\.unit_off_skipped\)\).*?non_cooling_skip_count.*?checked_add\(usize::from\(predecessor\.non_cooling_skipped\)\).*?cooling_call_count.*?checked_add\(usize::from\(predecessor\.cooling_body_entered\)\)' -Description "CP329 pending route-history parity with current CP328 route"
Assert-Contains -Path $cp329RuntimeValidation -Pattern 'pub\(in crate::ideal_loads::calc\) fn next_mixed_air_transition_fits\s*\(' -Description "CP329 route-aware next-transition preflight"
foreach ($cp329PreflightCounter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "cooling_call_count",
        "caller_source_site_execution_count",
        "child_source_site_execution_count",
        "state_reference_bind_count",
        "purchased_air_number_read_count",
        "outdoor_air_mass_flow_rate_read_count",
        "supply_mass_flow_rate_read_count",
        "mixed_air_output_reference_bind_count",
        "operating_mode_read_count",
        "mixed_air_child_call_count",
        "no_outdoor_air_fallback_count",
        "recirculation_enthalpy_projection_count",
        "mixed_air_output_assignment_count",
        "heat_recovery_output_positive_zero_assignment_count"
    )) {
    Assert-Contains -Path $cp329RuntimeValidation -Pattern ($cp329PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP329 checked preflight counter '$cp329PreflightCounter'"
}
$cp329ReleaseText = Read-RepoText -Path $cp329Release
$cp329ReleaseWrapper = [regex]::Match(
    $cp329ReleaseText,
    '(?s)pub fn advance_direct_no_oa_calc_cooling_mixed_air_call\(.*?(?=\r?\nfn mixed_air_call_links_to_predecessor\()'
)
if (-not $cp329ReleaseWrapper.Success) {
    throw "CP329 exact release wrapper must remain structurally bounded"
}
$cp329WrapperText = $cp329ReleaseWrapper.Value
$cp329ActiveInputIndex = $cp329WrapperText.IndexOf("let active_input =")
$cp329MutationIndex = $cp329WrapperText.IndexOf("let snapshot =")
if ($cp329ActiveInputIndex -lt 0 -or $cp329MutationIndex -le $cp329ActiveInputIndex) {
    throw "CP329 active-input construction and mutation boundary must remain ordered"
}
foreach ($cp329ValidationCall in @(
        "pending_mixed_air_history_links_to_predecessor(",
        "state_is_consistent(",
        "next_mixed_air_transition_fits(",
        "completed_mixed_air_predecessor_is_consistent("
    )) {
    $cp329ValidationIndex = $cp329WrapperText.IndexOf($cp329ValidationCall)
    if ($cp329ValidationIndex -lt 0 -or $cp329ValidationIndex -ge $cp329ActiveInputIndex) {
        throw "CP329 validation '$cp329ValidationCall' must complete before active-input construction and mutation"
    }
}
$cp329NextFitIndex = $cp329WrapperText.IndexOf("next_mixed_air_transition_fits(")
$cp329RecursivePredecessorIndex =
    $cp329WrapperText.IndexOf("completed_mixed_air_predecessor_is_consistent(")
if ($cp329RecursivePredecessorIndex -le $cp329NextFitIndex) {
    throw "CP329 checked-overflow preflight must precede recursive predecessor validation in the release short-circuit"
}
Assert-PatternsInOrder -Path $cp329Release -Patterns @(
    'let active_input = if predecessor_cp328\.cooling_body_entered',
    'let snapshot = \{',
    '\.get_mut\(&selected\)',
    'advance_cooling_mixed_air_call_state\(',
    'set_cooling_mixed_air_call_latest_witness\('
) -Description "CP329 validated active-input, transition, and witness mutation order"
Assert-Contains -Path $cp329Release -Pattern 'minimum_oa_links_to_predecessor\s*\(' -Description "CP329 retained same-call no-OA lineage"
Assert-Contains -Path $cp329Release -Pattern 'option_is_positive_zero\(minimum_oa\.working_outdoor_air_mass_flow_rate_kg_per_s\)' -Description "CP329 retained OA positive zero"
Assert-Contains -Path $cp329Release -Pattern 'minimum_oa\.psychrometric_call_count == 0' -Description "CP329 no-OA prefix psychrometric firewall"
Assert-Contains -Path $cp329Release -Pattern 'pub enum PurchasedAirCalcCoolingMixedAirCallRecirculationInput' -Description "CP329 finite-input attribution"
Assert-Contains -Path $cp329Release -Pattern 'NonFiniteRecirculationState' -Description "CP329 nonfinite release error"
Assert-Contains -Path $cp329Release -Pattern 'validate_recirculation_state_and_project_enthalpy\s*\(' -Description "CP329 coherent finite-state validator"
Assert-Contains -Path $cp329Release -Pattern 'if !value\.is_finite\(\)' -Description "CP329 finite T/W validation"
Assert-Contains -Path $cp329Release -Pattern 'if !enthalpy_projection_j_per_kg\.is_finite\(\)' -Description "CP329 finite enthalpy-projection validation"
Assert-Contains -Path $cp329Release -Pattern 'moist_air_enthalpy_j_per_kg\(temperature_c, humidity_ratio\)' -Description "CP329 coherent enthalpy projection"
Assert-Contains -Path $cp329Release -Pattern 'zone_state\.mean_air_temperature_c' -Description "CP329 bound recirculation temperature"
Assert-Contains -Path $cp329Release -Pattern 'zone_state\.air_humidity_ratio' -Description "CP329 bound recirculation humidity"
Assert-PatternsInOrder -Path $cp329Release -Patterns @(
    'let active_input = if predecessor_cp328\.cooling_body_entered',
    'validate_recirculation_state_and_project_enthalpy\(',
    '\)\?;',
    'let snapshot = \{',
    '\.get_mut\(&selected\)',
    'advance_cooling_mixed_air_call_state\('
) -Description "CP329 finite validation before mutation"
Assert-Contains -Path $cp329Release -Pattern 'advance_cooling_mixed_air_call_state\(' -Description "CP329 dedicated bounded transition"
Assert-Contains -Path $cp329Release -Pattern 'set_cooling_mixed_air_call_latest_witness\(' -Description "CP329 private witness commit"
Assert-Contains -Path $cp329Release -Pattern 'cooling_mixed_air_call_snapshot_is_exact_direct_release\(' -Description "CP329 post-transition exact validation"
Assert-Contains -Path $cp329Release -Pattern 'cooling_mixed_air_call_snapshots_match_bit_exact\(' -Description "CP329 bit-exact retained witness validation"
Assert-Contains -Path $cp329Release -Pattern 'recirculation_temperature\.is_finite\(\)' -Description "CP329 exact snapshot finite temperature"
Assert-Contains -Path $cp329Release -Pattern 'recirculation_humidity\.is_finite\(\)' -Description "CP329 exact snapshot finite humidity"
Assert-Contains -Path $cp329Release -Pattern 'recirculation_enthalpy\.is_finite\(\)' -Description "CP329 exact snapshot finite enthalpy"
Assert-Contains -Path $cp329Release -Pattern '(?s)recirculation_enthalpy\.to_bits\(\)\s*== moist_air_enthalpy_j_per_kg\(recirculation_temperature, recirculation_humidity\)[\r\n\s.]+to_bits\(\)' -Description "CP329 exact snapshot coherent enthalpy"
Assert-Contains -Path $cp329ReleaseTests -Pattern 'active_nonfinite_recirculation_inputs_fail_before_any_cp329_mutation' -Description "CP329 nonfinite transactional regression"
Assert-Contains -Path $cp329ReleaseTests -Pattern 'assert_eq!\(case_runtime, before\);' -Description "CP329 nonfinite no-mutation assertion"
Assert-Contains -Path $cp329ReleaseTests -Pattern 'public_cp329_completed_and_pending_history_redistribution_is_rejected_without_mutation' -Description "CP329 completed/pending retained-route redistribution regression"
Assert-Contains -Path $cp329ReleaseTests -Pattern '(?s)assert!\(\s*!super::release::completed_direct_cooling_mixed_air_call_is_consistent\(' -Description "CP329 completed retained-history predicate regression"
Assert-Contains -Path $cp329ReleaseTests -Pattern 'active_cp329_child_site_increment_overflow_is_rejected_without_mutation' -Description "CP329 route-aware overflow transaction regression"
Assert-Contains -Path $cp329ReleaseTests -Pattern '(?s)assert!\(\s*!super::release::next_mixed_air_transition_fits_for_test\(' -Description "CP329 direct next-transition overflow predicate regression"
Assert-Contains -Path $cp329ReleaseTests -Pattern 'assert_eq!\(runtime, before\);' -Description "CP329 retained-history/overflow no-mutation assertions"
Assert-Contains -Path $cp329Tests -Pattern 'release_predicate_rejects_nonfinite_or_incoherent_recirculation_projection' -Description "CP329 finite/coherent snapshot regression"
Assert-NotContains -Path $cp329Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_mixed_air_call\([^)]*(?:outdoor_air_mass_flow_rate|supply_mass_flow_rate|recirculation_enthalpy)_.*:' -Description "duplicate caller scalar in CP329 release"
Assert-NotContains -Path $cp329Release -Pattern 'mixed_air_state|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|TimeHtRecActive|time_ht_rec_active' -Description "broader child, numerical DTO, or heat-recovery-time mutation in CP329 release"

Assert-Contains -Path $cp329InitState -Pattern '(?s)cooling_mixed_air_call_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingMixedAirCallSnapshot' -Description "runtime-root private CP329 witness map"
Assert-NotContains -Path $cp329InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_mixed_air_call_latest_witnesses:' -Description "public runtime-root CP329 witness map"
Assert-Contains -Path $cp329InitWitnessRoot -Pattern 'mod cooling_mixed_air_call;' -Description "runtime-root CP329 witness module"
Assert-Contains -Path $cp329InitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_mixed_air_call_latest_witness\s*\(' -Description "runtime-root CP329 witness getter"
Assert-Contains -Path $cp329InitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_mixed_air_call_latest_witness\s*\(' -Description "runtime-root CP329 witness setter"
Assert-Contains -Path $cp329InitState -Pattern 'pub calc_cooling_mixed_air_call:\s*PurchasedAirCalcCoolingMixedAirCallRuntimeState' -Description "per-unit CP329 persistent state"

# The scheduled binding is the only
# CP328 -> CP329 -> CP330 -> CP331 -> CP332 -> CP333 -> CP334 -> CP335 ->
# CP336 -> CP337 -> CP338 -> CP339 -> numerical placement.
$cp329BindingText = Read-RepoText -Path $cp329Binding
$cp328BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_supply_mass_flow_very_small_guard_body =")
$cp329BindingIndex = $cp329BindingText.IndexOf("let calculation_cooling_mixed_air_call =")
$cp330BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_supply_mass_flow_positive_guard =")
$cp331BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_cp_air_assignment =")
$cp332BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_assignment =")
$cp333BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_minimum_limit =")
$cp334BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_mixed_air_limit =")
$cp335BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp329 = $cp329BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp329 = $cp329BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp328BindingIndexForCp329 -lt 0 -or
    $cp329BindingIndex -le $cp328BindingIndexForCp329 -or
    $cp330BindingIndexForCp329 -le $cp329BindingIndex -or
    $cp331BindingIndexForCp329 -le $cp330BindingIndexForCp329 -or
    $cp332BindingIndexForCp329 -le $cp331BindingIndexForCp329 -or
    $cp333BindingIndexForCp329 -le $cp332BindingIndexForCp329 -or
    $cp334BindingIndexForCp329 -le $cp333BindingIndexForCp329 -or
    $cp335BindingIndexForCp329 -le $cp334BindingIndexForCp329 -or
    $cp336BindingIndexForCp329 -le $cp335BindingIndexForCp329 -or
    $cp337BindingIndexForCp329 -le $cp336BindingIndexForCp329 -or
    $cp338BindingIndexForCp329 -le $cp337BindingIndexForCp329 -or
    $cp339BindingIndexForCp329 -le $cp338BindingIndexForCp329 -or
    $numericalBindingIndexForCp329 -le $cp339BindingIndexForCp329
) {
    throw "Binding must retain exact CP328 -> CP329 -> CP330 -> CP331 -> CP332 -> CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $cp329Binding -Pattern '(?s)let calculation_cooling_mixed_air_call =\s*advance_direct_no_oa_calc_cooling_mixed_air_call\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_supply_mass_flow_very_small_guard_body,\s*&\*input\.zone_state,\s*\)' -Description "binding exact CP328-to-CP329 wrapper call"
Assert-Contains -Path $cp329Binding -Pattern 'CalculationCoolingMixedAirCall\(PurchasedAirCalcCoolingMixedAirCallError\)' -Description "CP329 scheduled binding error boundary"
Assert-Contains -Path $cp329ScheduledOutput -Pattern 'pub calculation_cooling_mixed_air_call:\s*PurchasedAirCalcCoolingMixedAirCallSnapshot' -Description "CP329 scheduled output evidence"
Assert-Contains -Path $cp329BindingTestsRoot -Pattern '#\[path = "binding/cooling_mixed_air_call_tests\.rs"\]' -Description "CP329 binding test module path"
foreach ($cp329BindingTest in @(
        "scheduled_binding_executes_the_nine_site_call_and_exact_no_oa_child",
        "scheduled_binding_skips_every_cp329_site_when_cooling_is_inactive",
        "public_cp329_release_rejects_replay_and_forged_cp328_ordinal_without_mutation"
    )) {
    Assert-Contains -Path $cp329BindingTests -Pattern $cp329BindingTest -Description "CP329 binding regression '$cp329BindingTest'"
}
$cp329BindingCall = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_mixed_air_call =\s*advance_direct_no_oa_calc_cooling_mixed_air_call\(.*?CalculationCoolingMixedAirCall,?\s*\)\?;'
)
if (-not $cp329BindingCall.Success) {
    throw "Binding must retain the complete CP329 exact release call"
}
$cp329BindingCallEnd = $cp329BindingCall.Index + $cp329BindingCall.Length
$cp330BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_positive_guard =\s*advance_positive_guard\([^;]+?\)\?;'
)
if (-not $cp330BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP330 exact release call after CP329"
}
$cp330BindingCallEndForCp329 =
    $cp330BindingCallForCp329.Index + $cp330BindingCallForCp329.Length
$cp331BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_cp_air_assignment =\s*advance_positive_supply_cp_air_assignment\([^;]+?\)\?;'
)
if (-not $cp331BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP331 exact release call after CP330"
}
$cp331BindingCallEndForCp329 =
    $cp331BindingCallForCp329.Index + $cp331BindingCallForCp329.Length
$cp332BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_assignment =\s*advance_positive_supply_temperature_assignment\([^;]+?\)\?;'
)
if (-not $cp332BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP332 exact release call after CP331"
}
$cp332BindingCallEndForCp329 =
    $cp332BindingCallForCp329.Index + $cp332BindingCallForCp329.Length
$cp333BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_minimum_limit =\s*advance_positive_supply_temperature_minimum_limit\([^;]+?\)\?;'
)
if (-not $cp333BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP333 exact release call after CP332"
}
$cp333BindingCallEndForCp329 =
    $cp333BindingCallForCp329.Index + $cp333BindingCallForCp329.Length
$cp334BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
if (-not $cp334BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP334 exact release call after CP333"
}
$cp334BindingCallEndForCp329 =
    $cp334BindingCallForCp329.Index + $cp334BindingCallForCp329.Length
$cp335BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
if (-not $cp335BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP335 exact release call after CP334"
}
$cp335BindingCallEndForCp329 =
    $cp335BindingCallForCp329.Index + $cp335BindingCallForCp329.Length
$cp336BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
if (-not $cp336BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP336 exact release call after CP335"
}
$cp336BindingCallEndForCp329 =
    $cp336BindingCallForCp329.Index + $cp336BindingCallForCp329.Length
$cp337BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
if (-not $cp337BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP337 exact release call after CP336"
}
$cp337BindingCallEndForCp329 =
    $cp337BindingCallForCp329.Index + $cp337BindingCallForCp329.Length
$cp338BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
if (-not $cp338BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP338 exact release call after CP337"
}
$cp338BindingCallEndForCp329 =
    $cp338BindingCallForCp329.Index + $cp338BindingCallForCp329.Length
$cp339BindingCallForCp329 = [regex]::Match(
    $cp329BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (-not $cp339BindingCallForCp329.Success) {
    throw "Binding must retain the complete CP339 exact release call after CP338"
}
$cp339BindingCallEndForCp329 =
    $cp339BindingCallForCp329.Index + $cp339BindingCallForCp329.Length
if (
    $cp330BindingIndexForCp329 -lt $cp329BindingCallEnd -or
    $cp331BindingIndexForCp329 -lt $cp330BindingCallEndForCp329 -or
    $cp332BindingIndexForCp329 -lt $cp331BindingCallEndForCp329 -or
    $cp333BindingIndexForCp329 -lt $cp332BindingCallEndForCp329 -or
    $cp334BindingIndexForCp329 -lt $cp333BindingCallEndForCp329 -or
    $cp335BindingIndexForCp329 -lt $cp334BindingCallEndForCp329 -or
    $cp336BindingIndexForCp329 -lt $cp335BindingCallEndForCp329 -or
    $cp337BindingIndexForCp329 -lt $cp336BindingCallEndForCp329 -or
    $cp338BindingIndexForCp329 -lt $cp337BindingCallEndForCp329 -or
    $cp339BindingIndexForCp329 -lt $cp338BindingCallEndForCp329 -or
    $numericalBindingIndexForCp329 -lt $cp339BindingCallEndForCp329
) {
    throw "CP329, CP330, CP331, CP332, CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
$postCp329BeforeCp330 = $cp329BindingText.Substring(
    $cp329BindingCallEnd,
    $cp330BindingIndexForCp329 - $cp329BindingCallEnd
)
$postCp329BeforeCp330Code = [regex]::Replace($postCp329BeforeCp330, '(?m)//.*$', '')
if ($postCp329BeforeCp330Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP329 and before CP330"
}
$postCp330BeforeCp331ForCp329 = $cp329BindingText.Substring(
    $cp330BindingCallEndForCp329,
    $cp331BindingIndexForCp329 - $cp330BindingCallEndForCp329
)
$postCp330BeforeCp331CodeForCp329 =
    [regex]::Replace($postCp330BeforeCp331ForCp329, '(?m)//.*$', '')
if ($postCp330BeforeCp331CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP330 and before CP331"
}
$postCp331BeforeCp332ForCp329 = $cp329BindingText.Substring(
    $cp331BindingCallEndForCp329,
    $cp332BindingIndexForCp329 - $cp331BindingCallEndForCp329
)
$postCp331BeforeCp332CodeForCp329 =
    [regex]::Replace($postCp331BeforeCp332ForCp329, '(?m)//.*$', '')
if ($postCp331BeforeCp332CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP331 and before CP332"
}
$postCp332BeforeCp333ForCp329 = $cp329BindingText.Substring(
    $cp332BindingCallEndForCp329,
    $cp333BindingIndexForCp329 - $cp332BindingCallEndForCp329
)
$postCp332BeforeCp333CodeForCp329 =
    [regex]::Replace($postCp332BeforeCp333ForCp329, '(?m)//.*$', '')
if ($postCp332BeforeCp333CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP332 and before CP333"
}
$postCp333BeforeCp334ForCp329 = $cp329BindingText.Substring(
    $cp333BindingCallEndForCp329,
    $cp334BindingIndexForCp329 - $cp333BindingCallEndForCp329
)
$postCp333BeforeCp334CodeForCp329 =
    [regex]::Replace($postCp333BeforeCp334ForCp329, '(?m)//.*$', '')
if ($postCp333BeforeCp334CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP333 and before CP334"
}
$postCp334BeforeCp335ForCp329 = $cp329BindingText.Substring(
    $cp334BindingCallEndForCp329,
    $cp335BindingIndexForCp329 - $cp334BindingCallEndForCp329
)
$postCp334BeforeCp335CodeForCp329 =
    [regex]::Replace($postCp334BeforeCp335ForCp329, '(?m)//.*$', '')
if ($postCp334BeforeCp335CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP334 and before CP335"
}
$postCp335BeforeCp336ForCp329 = $cp329BindingText.Substring(
    $cp335BindingCallEndForCp329,
    $cp336BindingIndexForCp329 - $cp335BindingCallEndForCp329
)
$postCp335BeforeCp336CodeForCp329 =
    [regex]::Replace($postCp335BeforeCp336ForCp329, '(?m)//.*$', '')
if ($postCp335BeforeCp336CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP335 and before CP336"
}
$postCp336BeforeCp337ForCp329 = $cp329BindingText.Substring(
    $cp336BindingCallEndForCp329,
    $cp337BindingIndexForCp329 - $cp336BindingCallEndForCp329
)
$postCp336BeforeCp337CodeForCp329 =
    [regex]::Replace($postCp336BeforeCp337ForCp329, '(?m)//.*$', '')
if ($postCp336BeforeCp337CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP336 and before CP337"
}
$postCp337BeforeCp338ForCp329 = $cp329BindingText.Substring(
    $cp337BindingCallEndForCp329,
    $cp338BindingIndexForCp329 - $cp337BindingCallEndForCp329
)
$postCp337BeforeCp338CodeForCp329 =
    [regex]::Replace($postCp337BeforeCp338ForCp329, '(?m)//.*$', '')
if ($postCp337BeforeCp338CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP337 and before CP338"
}
$postCp338BeforeCp339ForCp329 = $cp329BindingText.Substring(
    $cp338BindingCallEndForCp329,
    $cp339BindingIndexForCp329 - $cp338BindingCallEndForCp329
)
$postCp338BeforeCp339CodeForCp329 =
    [regex]::Replace($postCp338BeforeCp339ForCp329, '(?m)//.*$', '')
if ($postCp338BeforeCp339CodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP338 and before CP339"
}
$postCp339BeforeNumericalForCp329 = $cp329BindingText.Substring(
    $cp339BindingCallEndForCp329,
    $numericalBindingIndexForCp329 - $cp339BindingCallEndForCp329
)
$postCp339BeforeNumericalCodeForCp329 =
    [regex]::Replace($postCp339BeforeNumericalForCp329, '(?m)//.*$', '')
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp329 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp329,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;)',
    ''
)
if ($postCp339BeforeNumericalCodeForCp329 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No helper other than the audited CP340 through CP348 releases may execute after CP339 and before numerical Calc"
}

# Coupled validation reconstructs the projection from CP328 and the existing
# direct recirculation state. It never reconciles against the numerical DTO.
Assert-Contains -Path $cp329CoupledRuntime -Pattern 'mod cooling_mixed_air_call_validation;' -Description "coupled CP329 validator declaration"
Assert-Contains -Path $cp329CoupledRuntime -Pattern 'pub calc_cooling_mixed_air_call_lifecycle:\s*PurchasedAirCalcCoolingMixedAirCallLifecycleSummary' -Description "coupled CP329 lifecycle"
Assert-Contains -Path $cp329CoupledRuntime -Pattern 'cooling_mixed_air_call_validation::snapshot_matches_release' -Description "coupled per-timestep CP329 validation"
Assert-Contains -Path $cp329CoupledRuntime -Pattern 'cooling_mixed_air_call_validation::validate_lifecycle' -Description "coupled final CP329 validation"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'let predecessor = output\.calculation_cooling_supply_mass_flow_very_small_guard_body;' -Description "coupled CP328 predecessor"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'let snapshot = output\.calculation_cooling_mixed_air_call;' -Description "coupled CP329 output"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'binding\.return_node' -Description "coupled retained recirculation-node provenance"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'let Some\(recirculation_temperature_c\) = snapshot\.recirculation_temperature_c' -Description "coupled retained recirculation-temperature provenance"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'let Some\(recirculation_humidity_ratio\) = snapshot\.recirculation_humidity_ratio' -Description "coupled retained recirculation-humidity provenance"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'moist_air_enthalpy_j_per_kg\(' -Description "coupled coherent enthalpy projection"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'options_have_exact_bits\(' -Description "coupled exact-bit copy validation"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'option_has_bits\(snapshot\.heat_recovery_sensible_output_w, Some\(0\.0\)\)' -Description "coupled sensible recovery positive zero"
Assert-Contains -Path $cp329CoupledValidation -Pattern 'option_has_bits\(snapshot\.heat_recovery_latent_output_w, Some\(0\.0\)\)' -Description "coupled latent recovery positive zero"
$cp329CoupledRuntimeText = (Read-RepoText -Path $cp329CoupledValidation).Split("#[cfg(test)]")[0]
if ($cp329CoupledRuntimeText -match 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|mixed_air_state|TimeHtRecActive|time_ht_rec_active') {
    throw "Coupled CP329 runtime validation must not substitute another child or reconcile a later DTO"
}
Assert-Contains -Path $cp329CoupledFixture -Pattern 'calculation_cooling_mixed_air_call_snapshot\s*\(' -Description "coupled CP329 fixture"
Assert-Contains -Path $cp329CoupledFixture -Pattern 'moist_air_enthalpy_j_per_kg\(' -Description "coupled fixture coherent enthalpy projection"
Assert-Contains -Path $cp329CoupledTests -Pattern 'cooling_mixed_air_call_partition_overflow_fails_closed' -Description "coupled CP329 overflow regression"

# Pipeline evidence is direct-only and preserves exact IEEE payloads for each
# scalar copy and positive-zero child assignment.
Assert-Contains -Path $cp329PipelineRoot -Pattern 'mod purchased_air_cooling_mixed_air_call;' -Description "pipeline CP329 module declaration"
Assert-Contains -Path $cp329PipelineRoot -Pattern '"purchased_air_calc_cooling_mixed_air_call_lifecycle"' -Description "pipeline CP329 lifecycle JSON key"
Assert-Contains -Path $cp329PipelineRoot -Pattern 'purchased_air_cooling_mixed_air_call::validate_direct_lifecycle' -Description "pipeline CP329 direct firewall"
Assert-Contains -Path $cp329Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER' -Description "pipeline CP329 caller order"
Assert-Contains -Path $cp329Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER' -Description "pipeline CP329 child order"
Assert-Contains -Path $cp329Pipeline -Pattern 'predecessor_state\.cooling_body_entry_count' -Description "pipeline CP328 active-call provenance"
Assert-Contains -Path $cp329PipelineValidation -Pattern 'snapshot\.outdoor_air_enabled == Some\(false\)' -Description "pipeline no-OA child route"
Assert-Contains -Path $cp329PipelineValidation -Pattern '!snapshot\.outdoor_air_mass_flow_positive_comparison_evaluated' -Description "pipeline OA comparison short circuit"
Assert-Contains -Path $cp329PipelineValidation -Pattern 'six_oa_locals_are_positive_zero' -Description "pipeline OA local positive zeros"
Assert-Contains -Path $cp329PipelineValidation -Pattern 'enthalpy_projection_is_coherent\s*\(' -Description "pipeline coherent enthalpy firewall"
Assert-Contains -Path $cp329PipelineValidation -Pattern 'moist_air_enthalpy_j_per_kg\(temperature_c, humidity_ratio\)' -Description "pipeline T/W-derived enthalpy projection"
Assert-Contains -Path $cp329PipelineValidation -Pattern 'coherent_enthalpy_projection_rejects_a_shared_finite_forgery' -Description "pipeline shared finite enthalpy forgery regression"
Assert-Contains -Path $cp329PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP329 latest serialization"
Assert-Contains -Path $cp329PipelineSnapshotSerialization -Pattern '"source_order"\s*:\s*snapshot\.source_order|field!\("source_order", snapshot\.source_order\)' -Description "pipeline CP329 caller-order JSON"
Assert-Contains -Path $cp329PipelineSnapshotSerialization -Pattern '(?s)"no_oa_child_source_order"\s*:\s*snapshot\.no_oa_child_source_order|field!\(\s*"no_oa_child_source_order",\s*snapshot\.no_oa_child_source_order\s*\)' -Description "pipeline CP329 child-order JSON"
foreach ($cp329BitField in @(
        "outdoor_air_mass_flow_rate_kg_per_s",
        "supply_mass_flow_rate_kg_per_s",
        "recirculation_temperature_c",
        "recirculation_humidity_ratio",
        "recirculation_enthalpy_projection_j_per_kg",
        "child_supply_mass_flow_rate_kg_per_s",
        "resulting_recirculation_mass_flow_rate_kg_per_s",
        "mixed_air_temperature_c",
        "mixed_air_humidity_ratio",
        "mixed_air_enthalpy_projection_j_per_kg",
        "heat_recovery_sensible_output_w",
        "heat_recovery_latent_output_w"
    )) {
    Assert-Contains -Path $cp329PipelineSnapshotSerialization -Pattern ('"' + [regex]::Escape($cp329BitField) + '_ieee_bits"') -Description "pipeline CP329 IEEE field '$cp329BitField'"
}
Assert-Contains -Path $cp329PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP329 exact IEEE serialization"
Assert-Contains -Path $cp329RunTests -Pattern 'mod cooling_mixed_air_call_assertions;' -Description "direct integration CP329 assertion module"
Assert-Contains -Path $cp329RunTests -Pattern 'assert_cooling_mixed_air_call\(' -Description "direct integration CP329 assertion calls"
Assert-Contains -Path $cp329DirectAssertions -Pattern 'purchased_air_calc_cooling_mixed_air_call_lifecycle' -Description "direct integration CP329 lifecycle key"
Assert-Contains -Path $cp329DirectAssertions -Pattern 'const POSITIVE_ZERO_BITS: &str = "0x0000000000000000";' -Description "direct integration CP329 exact positive zero"
Assert-Contains -Path $cp329DirectAssertions -Pattern 'string_array\(&latest\["source_order"\]\)' -Description "direct integration CP329 caller order"
Assert-Contains -Path $cp329DirectAssertions -Pattern 'string_array\(&latest\["no_oa_child_source_order"\]\)' -Description "direct integration CP329 child order"

# The ledgers repeat the same bounded evidence-only claim in both parent
# algorithms/capabilities without routine, count, readiness, or support
# promotion.
$cp329AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp329AlgorithmAddenda = [regex]::Matches(
    $cp329AlgorithmText,
    '(?m)^\s*"CP329 supersedes only CP328[^"\r\n]+",\s*$'
)
if ($cp329AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP329 support addenda"
}
foreach ($cp329AlgorithmAddendum in $cp329AlgorithmAddenda) {
    $cp329Text = $cp329AlgorithmAddendum.Value
    foreach ($cp329Pattern in @(
            'lines-2171-2178',
            'nine textual sites',
            'does not claim a C\+\+ function-argument evaluation order',
            'UnitOff and non-cooling skip',
            'zero supply flow',
            'same-call CP328',
            'OAMassFlowRate=\+0\.0',
            'coherent enthalpy projection',
            'no OA-node or psychrometric work',
            'positive zero',
            'does not write the already-reset heat-recovery active time',
            'stored-H inconsistency',
            'CP328-to-CP329-to-numerical',
            'Line 2183 is the first excluded executable',
            '2454-2461',
            '`OutdoorAir`, `Economizer`, `HeatRecovery`, `EMS`, and Autosizing remain forbidden',
            'remain `source_mapped`',
            'Roadmap state stay unchanged'
        )) {
        if ($cp329Text -notmatch $cp329Pattern) {
            throw "CP329 algorithm addendum missing '$cp329Pattern'"
        }
    }
}
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_mixed_air_call/release\.rs::advance_direct_no_oa_calc_cooling_mixed_air_call' -Description "CP329 algorithm wrapper target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_mixed_air_call\.rs::purchased_air_calc_cooling_mixed_air_call_lifecycle_summary' -Description "CP329 algorithm lifecycle target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_mixed_air_call\.rs::PurchasedAirCalcCoolingMixedAirCallRuntimeState' -Description "CP329 routine state target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_mixed_air_call\.rs::PurchasedAirCalcCoolingMixedAirCallLifecycleSummary' -Description "CP329 routine lifecycle target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'routine\.calc_purch_air_loads\.completion_status = "source_mapped"' -Description "CP329 retained CalcPurchAirLoads status"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'routine\.calc_purch_air_mixed_air\.completion_status = "source_mapped"' -Description "CP329 retained CalcPurchAirMixedAir status"

$cp329CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp329CapabilityAddenda = [regex]::Matches(
    $cp329CapabilityText,
    '(?m)^\s*"CP329 additionally requires[^"\r\n]+",\s*$'
)
if ($cp329CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP329 claim addenda"
}
foreach ($cp329CapabilityAddendum in $cp329CapabilityAddenda) {
    $cp329Text = $cp329CapabilityAddendum.Value
    foreach ($cp329Pattern in @(
            'lines 2171-2178',
            'Nine textual caller sites',
            'without claiming C\+\+ argument evaluation order',
            'UnitOff and non-cooling skip',
            'zero supply flow',
            'coherent enthalpy projection',
            'no OA-node or psychrometric work',
            'Line 2183 is the first excluded executable',
            '2454-2461',
            '`OutdoorAir`, `Economizer`, `HeatRecovery`, `EMS`, and Autosizing remain forbidden',
            'This changes no support level',
            'both Calc routines remain `source_mapped`'
        )) {
        if ($cp329Text -notmatch $cp329Pattern) {
            throw "CP329 capability addendum missing '$cp329Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP329 supersedes only CP328' -Description "generated CP329 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP329 additionally requires' -Description "generated CP329 capability index"
Assert-Contains -Path "docs\src\current\project-contract.md" -Pattern 'direct no-OA child route at lines 2851, 2854-2861, 2869-2874, 2876, 2878, and[\r\n\s]+2932-2937' -Description "project contract exact CP329 child route"
Assert-Contains -Path "docs\src\porting-map\ideal-loads-source-map.md" -Pattern 'child route at lines 2851, 2854-2861, 2869-2874, 2876, 2878, and 2932-2937' -Description "IdealLoads source map exact CP329 child route"

# Every hand-authored contract repeats the bounded caller/child, coherent
# projection limitation, direct-only evidence, exclusions, and non-promotion.
$cp329DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP329 maps the complete Cooling `CalcPurchAirMixedAir` call statement.*?^conformance, and Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP329 Source-Ordered Cooling `CalcPurchAirMixedAir` Call\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP329 Cooling `CalcPurchAirMixedAir` Call and No-OA Fallback\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP329 Cooling Mixed-Air Call in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP329 Cooling Mixed-Air Call Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp329Documentation in $cp329DocumentationSections) {
    $cp329DocumentText = Read-RepoText -Path $cp329Documentation.Path
    $cp329Matches = [regex]::Matches($cp329DocumentText, $cp329Documentation.Pattern)
    if ($cp329Matches.Count -ne 1) {
        throw "CP329 documentation expected one scoped section in $($cp329Documentation.Path), found $($cp329Matches.Count)"
    }
    $cp329Section = $cp329Matches[0].Value
    foreach ($cp329Pattern in @(
            '2171-2178',
            '(?is)nine.{0,40}(?:textual|caller|source).{0,30}site',
            '(?is)(?:(?:does|do)\s+not\s+claim\s+a|without\s+claiming)\s+C\+\+.{0,40}argument.{0,30}evaluation\s+order',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?is)zero\s+supply\s+flow|supply\s+flow\s+is\s+zero',
            '(?i)CP328',
            '(?i)no-OA',
            '(?i)positive(?:-|\s+)zero|\+0\.0',
            '(?i)coherent.{0,80}enthalpy|enthalpy.{0,80}coherent',
            '(?i)stored(?:-| )H|stored enthalpy|Node\.Enthalpy',
            '(?is)(?:no|the)\s+OA(?:-|\s+)node.{0,250}psychrometric|does\s+not\s+dereference.{0,120}psychrometric',
            'purchased_air_calc_cooling_mixed_air_call_lifecycle',
            '(?i)runtime-aware',
            '(?is)completed.{0,160}pending|pending.{0,160}completed',
            '(?is)CP328.{0,160}private\s+(?:latest\s+)?witness|private\s+witness.{0,160}CP328',
            '(?is)UnitOff.{0,100}non-cooling.{0,100}(?:active|route)',
            '(?is)route-aware.{0,80}checked-arithmetic',
            '(?is)(?:fail|rejected|returns an error).{0,180}(?:unchanged|before.{0,100}mutat|without.{0,100}(?:changing|mutat))',
            '(?i)line-?2183|line 2183',
            '2454-2461',
            '(?is)`OutdoorAir`.{0,100}`Economizer`.{0,100}`HeatRecovery`.{0,100}`EMS`.{0,100}Autosizing.{0,30}remain\s+forbidden',
            '(?i)source_mapped',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp329Section -notmatch $cp329Pattern) {
            throw "CP329 documentation in $($cp329Documentation.Path) missing '$cp329Pattern'"
        }
    }
}

# Main audit and generated script inventory remain ordered by source checkpoint.
$cp329MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp328DotSourceIndexForCp329 = $cp329MainAuditText.IndexOf('ideal-loads-structure-audit\cp328-cooling-supply-mass-flow-very-small-guard-body.ps1')
$cp329DotSourceIndex = $cp329MainAuditText.IndexOf('ideal-loads-structure-audit\cp329-cooling-mixed-air-call.ps1')
$cp330DotSourceIndexForCp329 = $cp329MainAuditText.IndexOf('ideal-loads-structure-audit\cp330-cooling-supply-mass-flow-positive-guard.ps1')
$cp329AuditCompletionIndex = $cp329MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp328DotSourceIndexForCp329 -lt 0 -or
    $cp329DotSourceIndex -le $cp328DotSourceIndexForCp329 -or
    $cp330DotSourceIndexForCp329 -le $cp329DotSourceIndex -or
    $cp329AuditCompletionIndex -le $cp330DotSourceIndexForCp329
) {
    throw "Main IdealLoads audit must dot-source CP329 after CP328 and before CP330/completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp329-cooling-mixed-air-call\.ps1"' -Description "CP329 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp329-cooling-mixed-air-call\.ps1::dot_sources' -Description "CP329 main-audit callee evidence"
