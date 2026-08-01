# CP337 maps only PurchasedAirManager.cc physical executable line 2195: the
# complete Cooling positive-supply Capacity-or-FlowRateAndCapacity guard.
# Physical line 2196 is the first excluded executable and CP338 boundary; a
# false guard next dynamically executes line 2208.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp337Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard.rs"
$cp337State = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard\state.rs"
$cp337Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard\transition.rs"
$cp337Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard\release.rs"
$cp337PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard\release\prefix_validation.rs"
$cp337RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard\release\runtime_validation.rs"
$cp337SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard\release\snapshot_validation.rs"
$cp337Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard\tests\mod.rs"
$cp337ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_guard\tests\release_corruption.rs"
$cp337CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp337Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp337Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp337ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp337BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_capacity_limit_guard.rs"
$cp337BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp337BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_positive_supply_capacity_limit_guard_tests.rs"
$cp337InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp337InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp337InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp337InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_positive_supply_capacity_limit_guard.rs"
$cp337CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp337CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_positive_supply_capacity_limit_guard_validation.rs"
$cp337CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp337CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_positive_supply_capacity_limit_guard_fixture.rs"
$cp337PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp337Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_guard.rs"
$cp337PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_guard\validation.rs"
$cp337PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_guard\serialization.rs"
$cp337PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_positive_supply_capacity_limit_guard\serialization\snapshot.rs"
$cp337RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp337DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_positive_supply_capacity_limit_guard_assertions.rs"
$cp337NonDirectTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"

foreach ($cp337RequiredFile in @(
        $cp337Module,
        $cp337State,
        $cp337Transition,
        $cp337Release,
        $cp337PrefixValidation,
        $cp337RuntimeValidation,
        $cp337SnapshotValidation,
        $cp337Tests,
        $cp337ReleaseCorruptionTests,
        $cp337ScheduledOutput,
        $cp337BindingAdapter,
        $cp337BindingTests,
        $cp337InitWitness,
        $cp337CoupledValidation,
        $cp337CoupledFixture,
        $cp337Pipeline,
        $cp337PipelineValidation,
        $cp337PipelineSerialization,
        $cp337PipelineSnapshotSerialization,
        $cp337DirectAssertions
    )) {
    Assert-FileExists -Path $cp337RequiredFile -Description "CP337 capacity-limit guard structure"
}
Assert-LineLimit -Path $cp337Release -Limit 800 -Description "CP337 release root module"
Assert-LineLimit -Path $cp337ReleaseCorruptionTests -Limit 800 -Description "CP337 release corruption regressions"
Assert-LineLimit -Path $cp337CoupledValidation -Limit 800 -Description "CP337 coupled validation module"
Assert-LineLimit -Path $cp337Pipeline -Limit 800 -Description "CP337 pipeline module"

# Locked source boundary and exact five-site deterministic Rust witness.
Assert-Contains -Path $cp337Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2195' -Description "CP337 exact physical source boundary"
Assert-Contains -Path $cp337Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2196' -Description "CP337 first excluded physical executable"
Assert-Contains -Path $cp337Module -Pattern 'Exact five textual source sites represented by CP337' -Description "CP337 exact textual-site count"
Assert-ExactStringArray -Path $cp337Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE_ORDER" -Expected @(
    "read-cooling-limit-for-capacity-comparison",
    "compare-cooling-limit-equal-to-capacity",
    "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "enter-capacity-limit-body-if-compound-condition-satisfied"
) -Description "CP337 deterministic Rust witness order"
Assert-NotContains -Path $cp337Module -Pattern 'LimitCapacity|LimitFlowRateAndCapacity' -Description "enum literals excluded from CP337 textual-site inventory"
Assert-Contains -Path $cp337Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot' -Description "CP337 public snapshot"
Assert-Contains -Path $cp337State -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState' -Description "CP337 persistent state"
Assert-Contains -Path $cp337Module -Pattern 'pub struct PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary' -Description "CP337 lifecycle summary"
Assert-Contains -Path $cp337Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary\s*\(' -Description "CP337 lifecycle accessor"
Assert-Contains -Path $cp337Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard\s*\(' -Description "CP337 exact-direct wrapper"
Assert-Contains -Path $cp337Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_positive_supply_capacity_limit_guard_state\s*\(' -Description "CP337 pure transition"
Assert-Contains -Path $cp337CalcRoot -Pattern 'mod cooling_positive_supply_capacity_limit_guard;' -Description "CP337 calc module declaration"
Assert-Contains -Path $cp337CalcRoot -Pattern 'pub use (?:cooling_positive_supply_capacity_limit_guard::\*;|\{[^}]*cooling_positive_supply_capacity_limit_guard::\*)' -Description "CP337 calc public surface"

foreach ($cp337Route in @(
        'UnitOff',
        'NonCooling',
        'PositiveGuardFalseFallthrough',
        'CapacityLimitBodyEntered',
        'ActiveCapacityLimitGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp337State -Pattern $cp337Route -Description "CP337 retained route '$cp337Route'"
}
foreach ($cp337Counter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'capacity_limit_guard_evaluation_count',
        'source_site_execution_count',
        'first_cooling_limit_read_count',
        'cooling_limit_capacity_comparison_count',
        'cooling_limit_capacity_match_count',
        'second_cooling_limit_read_count',
        'cooling_limit_flow_rate_and_capacity_comparison_count',
        'cooling_limit_flow_rate_and_capacity_match_count',
        'cooling_limit_rejected_count',
        'capacity_limit_body_entry_count',
        'active_guard_false_fallthrough_count'
    )) {
    Assert-Contains -Path $cp337State -Pattern ('pub ' + $cp337Counter + ':\s*usize') -Description "CP337 state counter '$cp337Counter'"
}

# Transition order preserves lazy source || and counts 3/5/4/4 dynamic sites.
Assert-PatternsInOrder -Path $cp337Transition -Patterns @(
    'let guard_evaluated = predecessor\.supply_enthalpy_assignment_executed;',
    'let first_cooling_limit = active_input\.map',
    'first_cooling_limit\.map\(\|limit\| limit == IdealLoadsLimit::LimitCapacity\)',
    'let second_cooling_limit = if cooling_limit_capacity == Some\(false\)',
    'limit == IdealLoadsLimit::LimitFlowRateAndCapacity',
    'capacity \|\| cooling_limit_flow_rate_and_capacity == Some\(true\)',
    'let capacity_limit_body_entered = cooling_limit_condition_satisfied == Some\(true\);',
    'state\.capacity_limit_guard_evaluation_count \+= 1;',
    'state\.source_site_execution_count \+= 2;',
    'state\.second_cooling_limit_read_count \+= 1;',
    'state\.source_site_execution_count \+= 2;',
    'state\.capacity_limit_body_entry_count \+= 1;',
    'state\.source_site_execution_count \+= 1;'
) -Description "CP337 lazy five-site transition order"
Assert-NotContains -Path $cp337Transition -Pattern 'maximum_total_cooling_capacity|max_cooling|PsyCpAir|psychrometric|complete_direct_zone_purchased_air_coupling' -Description "CP337 transition excludes capacity-body and numerical work"
Assert-Contains -Path $cp337Tests -Pattern 'source_boundary_and_exact_five_sites_are_stable' -Description "CP337 source-boundary regression"
Assert-Contains -Path $cp337Tests -Pattern 'lazy_selector_guard_matches_all_four_limits_and_counts_dynamic_sites' -Description "CP337 four-selector regression"
Assert-Contains -Path $cp337Tests -Pattern '(?s)\(NoLimit,\s*false,\s*true,\s*false,\s*false,\s*4\).*?\(LimitFlowRate,\s*false,\s*true,\s*false,\s*false,\s*4\).*?\(LimitCapacity,\s*true,\s*false,\s*false,\s*true,\s*3\).*?\(LimitFlowRateAndCapacity,\s*false,\s*true,\s*true,\s*true,\s*5\)' -Description "CP337 exact 3/5/4/4 site matrix"
Assert-Contains -Path $cp337Tests -Pattern 'inherited_skips_execute_no_cp337_source_sites_or_selector_reads' -Description "CP337 inherited null-skip regression"
Assert-Contains -Path $cp337Tests -Pattern 'counters_partition_inherited_and_active_routes' -Description "CP337 five-route partition regression"
Assert-Contains -Path $cp337Tests -Pattern 'exact_predicate_rejects_provenance_short_circuit_and_redundant_false_drift' -Description "CP337 exact snapshot corruption regression"

# Lifecycle algebra is A/C/S/F/B with sites 2*A + 2*S + B and A-B active
# false fallthroughs. Every transition increment is preflighted.
Assert-Contains -Path $cp337RuntimeValidation -Pattern '(?s)capacity_limit_guard_evaluation_count\s*\.checked_mul\(2\).*?second_cooling_limit_read_count\s*\.checked_mul\(2\).*?checked_add\(state\.capacity_limit_body_entry_count\)' -Description "CP337 dynamic source-site formula"
Assert-Contains -Path $cp337RuntimeValidation -Pattern '(?s)let active = state\.capacity_limit_guard_evaluation_count;.*?let expected_capacity =.*?IdealLoadsLimit::LimitCapacity.*?let expected_second = active - expected_capacity;.*?let expected_combined =.*?IdealLoadsLimit::LimitFlowRateAndCapacity.*?let Some\(expected_body\) = expected_capacity\.checked_add\(expected_combined\).*?let expected_rejected = active - expected_body;' -Description "CP337 A/C/S/F/B partition formula"
Assert-Contains -Path $cp337RuntimeValidation -Pattern '(?s)active_partition == active.*?selector_partition == active.*?source_site_execution_count == expected_source_sites.*?active_guard_false_fallthrough_count == expected_rejected' -Description "CP337 completed active partition"
foreach ($cp337PreflightCounter in @(
        'transition_count',
        'unit_off_skip_count',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        'witnessed_positive_guard_false_fallthrough_skip_count',
        'capacity_limit_guard_evaluation_count',
        'source_site_execution_count',
        'first_cooling_limit_read_count',
        'cooling_limit_capacity_comparison_count',
        'cooling_limit_capacity_match_count',
        'second_cooling_limit_read_count',
        'cooling_limit_flow_rate_and_capacity_comparison_count',
        'cooling_limit_flow_rate_and_capacity_match_count',
        'cooling_limit_rejected_count',
        'capacity_limit_body_entry_count',
        'witnessed_capacity_limit_body_entry_count',
        'active_guard_false_fallthrough_count',
        'witnessed_active_guard_false_fallthrough_count'
    )) {
    Assert-Contains -Path $cp337RuntimeValidation -Pattern ($cp337PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP337 checked preflight '$cp337PreflightCounter'"
}
Assert-Contains -Path $cp337SnapshotValidation -Pattern '(?s)fn skipped_fields_are_exact\(.*?!snapshot\.first_cooling_limit_read.*?first_cooling_limit\.is_none\(\).*?!snapshot\.second_cooling_limit_read.*?second_cooling_limit\.is_none\(\).*?cooling_limit_condition_satisfied\.is_none\(\).*?!snapshot\.capacity_limit_body_entered.*?!snapshot\.active_guard_false_fallthrough' -Description "CP337 complete skipped null firewall"
Assert-Contains -Path $cp337SnapshotValidation -Pattern '(?s)let capacity = first_limit == IdealLoadsLimit::LimitCapacity;.*?let second_expected = !capacity;.*?IdealLoadsLimit::LimitFlowRateAndCapacity.*?let satisfied = capacity \|\| combined == Some\(true\);' -Description "CP337 exact lazy active snapshot shape"

# Release accepts only CP336, reads the live typed selector, and uses retained
# CP321/CP325 selectors only as same-call lineage evidence.
Assert-Contains -Path $cp337Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp336: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,\s*\)' -Description "CP337 exact wrapper arguments"
Assert-Contains -Path $cp337Release -Pattern 'completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent\s*\(' -Description "CP337 recursive CP336 completed proof"
Assert-Contains -Path $cp337Release -Pattern 'cooling_positive_supply_enthalpy_assignment_latest_witness\s*\(' -Description "CP337 retained CP336 private witness"
Assert-Contains -Path $cp337Release -Pattern 'unit\.calc_cooling_capacity_zero_flow_reset\.latest' -Description "CP337 retained CP321 selector lineage"
Assert-Contains -Path $cp337Release -Pattern 'unit\.calc_cooling_supply_mass_flow_limit_guard\.latest' -Description "CP337 retained CP325 selector lineage"
Assert-Contains -Path $cp337Release -Pattern 'active_cooling_limit_links_to_retained_prefix\(' -Description "CP337 same-call selector lineage proof"
Assert-Contains -Path $cp337Release -Pattern 'cooling_limit:\s*system\.cooling_limit' -Description "CP337 live typed selector source"
Assert-Contains -Path $cp337PrefixValidation -Pattern '(?s)active_cooling_limit_links_to_retained_prefix\(.*?capacity_reset\.first_cooling_limit == Some\(cooling_limit\).*?flow_limit_guard\.first_cooling_limit == Some\(cooling_limit\)' -Description "CP337 CP321/CP325 lineage only"
Assert-NotContains -Path $cp337Release -Pattern 'maximum_total_cooling_capacity|max_cooling|AutosizeOrNumber|PsyCpAir|psychrometric|ZoneHeatBalanceState|zone_state|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "forbidden CP337 body, service, or numerical inputs"
Assert-NotContains -Path $cp337Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard\([^)]*cooling_limit\s*:' -Description "duplicate caller selector in CP337 release"
Assert-PatternsInOrder -Path $cp337Release -Patterns @(
    'pending_capacity_limit_guard_state_is_consistent\(',
    'next_capacity_limit_guard_transition_fits\(',
    'completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent\(',
    'active_cooling_limit_links_to_retained_prefix\(',
    'runtime\.units\.get_mut',
    'advance_cooling_positive_supply_capacity_limit_guard_state\(',
    'set_cooling_positive_supply_capacity_limit_guard_latest_witness\('
) -Description "CP337 validate-before-mutation order"
foreach ($cp337ReleaseTest in @(
        'public_release_commits_once_and_rejects_replay_without_mutation',
        'all_inherited_skip_routes_commit_without_reading_the_selector',
        'supplied_retained_or_private_cp336_drift_is_rejected_before_cp337_mutation',
        'recursive_cp335_cp334_and_same_call_selector_corruption_fail_closed',
        'post_initialization_selector_mutation_is_rejected_without_mutation',
        'every_active_counter_overflow_is_preflighted_transactionally',
        'body_and_positive_skip_witness_overflows_are_preflighted',
        'every_inherited_skip_counter_overflow_is_transactional',
        'source_site_product_and_redundant_false_corruption_are_transactional',
        'orphan_public_or_private_cp337_latest_is_rejected_without_mutation',
        'completed_proof_detects_post_commit_public_and_private_drift',
        'coordinated_public_state_and_private_witness_selector_forgery_cannot_bypass_lineage',
        'lifecycle_accessor_returns_the_retained_cp337_state'
    )) {
    Assert-Contains -Path $cp337ReleaseCorruptionTests -Pattern $cp337ReleaseTest -Description "CP337 release regression '$cp337ReleaseTest'"
}

# CP337 state and its latest witness are privately rooted on the selected unit.
Assert-Contains -Path $cp337InitState -Pattern '(?s)cooling_positive_supply_capacity_limit_guard_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot' -Description "runtime-root private CP337 witness map"
Assert-NotContains -Path $cp337InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_positive_supply_capacity_limit_guard_latest_witnesses:' -Description "public runtime-root CP337 witness map"
Assert-Contains -Path $cp337InitWitnessRoot -Pattern 'mod cooling_positive_supply_capacity_limit_guard;' -Description "runtime-root CP337 witness module"
Assert-Contains -Path $cp337InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn cooling_positive_supply_capacity_limit_guard_latest_witness\s*\(' -Description "runtime-root CP337 witness getter"
Assert-Contains -Path $cp337InitWitness -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_positive_supply_capacity_limit_guard_latest_witness\s*\(' -Description "runtime-root CP337 witness setter"
Assert-Contains -Path $cp337InitState -Pattern 'pub calc_cooling_positive_supply_capacity_limit_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState' -Description "per-unit CP337 persistent state"
Assert-Contains -Path $cp337InitUnit -Pattern '(?s)calc_cooling_positive_supply_capacity_limit_guard:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new\(\s*system\s*,?\s*\)' -Description "per-unit CP337 state initialization"

# Binding order is exact CP336 -> CP337 -> CP338 -> CP339 -> unchanged numerical DTO,
# without a helper call in any source-order interval.
$cp337BindingText = Read-RepoText -Path $cp337Binding
$cp336BindingIndexForCp337 = $cp337BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndex = $cp337BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp337 = $cp337BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp337 = $cp337BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp337 = $cp337BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp336BindingIndexForCp337 -lt 0 -or
    $cp337BindingIndex -le $cp336BindingIndexForCp337 -or
    $cp338BindingIndexForCp337 -le $cp337BindingIndex -or
    $cp339BindingIndexForCp337 -le $cp338BindingIndexForCp337 -or
    $numericalBindingIndexForCp337 -le $cp339BindingIndexForCp337
) {
    throw "Binding must retain exact CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
$cp336BindingCallForCp337 = [regex]::Match(
    $cp337BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
$cp337BindingCall = [regex]::Match(
    $cp337BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
$cp338BindingCallForCp337 = [regex]::Match(
    $cp337BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
$cp339BindingCallForCp337 = [regex]::Match(
    $cp337BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (
    -not $cp336BindingCallForCp337.Success -or
    -not $cp337BindingCall.Success -or
    -not $cp338BindingCallForCp337.Success -or
    -not $cp339BindingCallForCp337.Success
) {
    throw "Binding must retain complete CP336, CP337, CP338, and CP339 exact release calls"
}
$cp336BindingCallEndForCp337 =
    $cp336BindingCallForCp337.Index + $cp336BindingCallForCp337.Length
$cp337BindingCallEnd = $cp337BindingCall.Index + $cp337BindingCall.Length
$cp338BindingCallEndForCp337 =
    $cp338BindingCallForCp337.Index + $cp338BindingCallForCp337.Length
$cp339BindingCallEndForCp337 =
    $cp339BindingCallForCp337.Index + $cp339BindingCallForCp337.Length
if (
    $cp337BindingIndex -lt $cp336BindingCallEndForCp337 -or
    $cp338BindingIndexForCp337 -lt $cp337BindingCallEnd -or
    $cp339BindingIndexForCp337 -lt $cp338BindingCallEndForCp337 -or
    $numericalBindingIndexForCp337 -lt $cp339BindingCallEndForCp337
) {
    throw "CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
foreach ($cp337Interval in @(
        [PSCustomObject]@{
            Start = $cp336BindingCallEndForCp337
            End = $cp337BindingIndex
            Description = "after CP336 and before CP337"
        },
        [PSCustomObject]@{
            Start = $cp337BindingCallEnd
            End = $cp338BindingIndexForCp337
            Description = "after CP337 and before CP338"
        },
        [PSCustomObject]@{
            Start = $cp338BindingCallEndForCp337
            End = $cp339BindingIndexForCp337
            Description = "after CP338 and before CP339"
        },
        [PSCustomObject]@{
            Start = $cp339BindingCallEndForCp337
            End = $numericalBindingIndexForCp337
            Description = "after CP339 and before numerical Calc"
        }
    )) {
    $cp337IntervalText = $cp337BindingText.Substring(
        $cp337Interval.Start,
        $cp337Interval.End - $cp337Interval.Start
    )
    $cp337IntervalCode = [regex]::Replace($cp337IntervalText, '(?m)//.*$', '')
    $cp337IntervalCode = [regex]::Replace(
        $cp337IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
        ''
    )
    $cp337IntervalCode = [regex]::Replace(
        $cp337IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
    $cp337IntervalCode = [regex]::Replace(
        $cp337IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
        ''
    )
    $cp337IntervalCode = [regex]::Replace(
        $cp337IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
    $cp337IntervalCode = [regex]::Replace(
        $cp337IntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    $cp337IntervalCode = [regex]::Replace(
        $cp337IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp337IntervalCode = [regex]::Replace(
        $cp337IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp337IntervalCode = [regex]::Replace(
        $cp337IntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp337IntervalCode = [regex]::Replace(
    $cp337IntervalCode,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;)',
    ''
)
    if ($cp337IntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp337Interval.Description)"
    }
}
Assert-Contains -Path $cp337Binding -Pattern '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_enthalpy_assignment,\s*\)\?;' -Description "binding exact CP336-to-CP337 adapter call"
Assert-Contains -Path $cp337BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_capacity_limit_guard\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,\s*\)' -Description "CP337 binding adapter arguments"
Assert-Contains -Path $cp337BindingAdapter -Pattern '(?s)advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard\(\s*runtime,\s*system,\s*predecessor,\s*\).*?CalculationCoolingPositiveSupplyCapacityLimitGuard' -Description "CP337 adapter exact wrapper and error mapping"
Assert-NotContains -Path $cp337BindingAdapter -Pattern 'cooling_limit\s*:|maximum.*capacity|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|DirectZonePurchasedAirCouplingInput' -Description "extra selector, body, or numerical input in CP337 adapter"
Assert-Contains -Path $cp337Binding -Pattern 'PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError as CoolingSupplyCapacityLimitGuardError' -Description "CP337 scheduled binding error alias"
Assert-Contains -Path $cp337Binding -Pattern 'CalculationCoolingPositiveSupplyCapacityLimitGuard\(CoolingSupplyCapacityLimitGuardError\)' -Description "CP337 scheduled binding error boundary"
Assert-Contains -Path $cp337ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_capacity_limit_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot' -Description "CP337 scheduled output evidence"
Assert-Contains -Path $cp337BindingTestsRoot -Pattern '#\[rustfmt::skip\]\s*#\[path = "binding/cooling_positive_supply_capacity_limit_guard_tests\.rs"\]\s*mod cooling_positive_supply_capacity_limit_guard_tests;' -Description "CP337 binding test module"
Assert-Contains -Path $cp337BindingTests -Pattern 'scheduled_binding_uses_only_the_typed_cooling_limit_selector' -Description "CP337 typed selector binding regression"
Assert-Contains -Path $cp337BindingTests -Pattern 'scheduled_binding_preserves_all_complete_null_skip_routes' -Description "CP337 complete binding skip regression"

# Coupled runtime independently reconstructs CP337 from CP336 plus the bound
# typed selector and validates the same checked counter algebra.
Assert-Contains -Path $cp337CoupledRuntime -Pattern 'mod cooling_positive_supply_capacity_limit_guard_validation;' -Description "coupled CP337 validator declaration"
Assert-Contains -Path $cp337CoupledRuntime -Pattern 'pub calc_cooling_positive_supply_capacity_limit_guard_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary' -Description "coupled CP337 lifecycle"
Assert-Contains -Path $cp337CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_guard_validation::snapshot_matches_release' -Description "coupled per-timestep CP337 validation"
Assert-Contains -Path $cp337CoupledRuntime -Pattern 'cooling_positive_supply_capacity_limit_guard_validation::validate_lifecycle' -Description "coupled final CP337 validation"
Assert-Contains -Path $cp337CoupledValidation -Pattern 'output\.calculation_cooling_positive_supply_enthalpy_assignment' -Description "coupled CP336 predecessor"
Assert-Contains -Path $cp337CoupledValidation -Pattern 'binding\.system\.cooling_limit' -Description "coupled typed selector"
Assert-Contains -Path $cp337CoupledValidation -Pattern '(?s)let capacity_match = active && cooling_limit == IdealLoadsLimit::LimitCapacity;.*?let second_comparison = active && !capacity_match;.*?IdealLoadsLimit::LimitFlowRateAndCapacity.*?let condition_satisfied = capacity_match \|\| combined_match;' -Description "coupled lazy CP337 reconstruction"
Assert-Contains -Path $cp337CoupledValidation -Pattern 'checked_sub\(' -Description "coupled CP337 checked subtraction"
Assert-Contains -Path $cp337CoupledValidation -Pattern 'checked_mul\(' -Description "coupled CP337 checked source-site multiplication"
Assert-Contains -Path $cp337CoupledFixtureRoot -Pattern 'mod cooling_positive_supply_capacity_limit_guard_fixture;' -Description "coupled CP337 fixture declaration"
Assert-Contains -Path $cp337CoupledFixture -Pattern 'calculation_cooling_positive_supply_capacity_limit_guard_snapshot' -Description "coupled CP337 fixture output"

# Pipeline evidence is direct-only, reconstructs CP337 independently, and
# serializes selector/comparison results without numerical DTO ownership.
Assert-Contains -Path $cp337PipelineRoot -Pattern 'mod purchased_air_cooling_positive_supply_capacity_limit_guard;' -Description "pipeline CP337 module declaration"
Assert-Contains -Path $cp337PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle' -Description "pipeline CP337 lifecycle field and JSON key"
Assert-Contains -Path $cp337PipelineRoot -Pattern 'calc_cooling_positive_supply_capacity_limit_guard_lifecycle' -Description "pipeline CP337 coupled lifecycle transfer"
Assert-Contains -Path $cp337Pipeline -Pattern '(?s)validate_direct_lifecycle\(.*?predecessor_cp336.*?init_lifecycle.*?model_cooling_limit.*?coupling_call_count' -Description "pipeline CP337 validated inputs"
Assert-Contains -Path $cp337Pipeline -Pattern 'predecessor_state\.supply_enthalpy_assignment_count' -Description "pipeline CP336 active-count parity"
Assert-Contains -Path $cp337PipelineValidation -Pattern '(?s)let active = state\.capacity_limit_guard_evaluation_count;.*?checked_sub\(active, capacity_matches.*?checked_add\(capacity_matches, combined_matches.*?checked_sub\(active, body_entries.*?checked_mul\(active, 2.*?checked_mul\(second_comparisons, 2.*?checked_add\(source_sites, body_entries' -Description "pipeline CP337 checked A/C/S/F/B formula"
Assert-Contains -Path $cp337PipelineValidation -Pattern 'source_counter_overflow_and_impossible_match_fail_closed' -Description "pipeline CP337 overflow regression"
Assert-Contains -Path $cp337PipelineValidation -Pattern 'fixed_selector_history_rejects_self_consistent_corruption' -Description "pipeline CP337 fixed-selector history regression"
Assert-Contains -Path $cp337PipelineSerialization -Pattern 'active_guard_false_fallthrough_count' -Description "pipeline CP337 lifecycle serialization"
foreach ($cp337SelectorField in @(
        'first_cooling_limit',
        'cooling_limit_capacity',
        'second_cooling_limit',
        'cooling_limit_flow_rate_and_capacity',
        'cooling_limit_condition_satisfied',
        'capacity_limit_body_entered',
        'active_guard_false_fallthrough'
    )) {
    Assert-Contains -Path $cp337PipelineSnapshotSerialization -Pattern ('"' + $cp337SelectorField + '"') -Description "pipeline CP337 snapshot field '$cp337SelectorField'"
}
Assert-Contains -Path $cp337RunTests -Pattern 'cooling_positive_supply_capacity_limit_guard_assertions' -Description "direct-run CP337 assertion module"
Assert-Contains -Path $cp337DirectAssertions -Pattern 'const SOURCE_ORDER:\s*\[&str;\s*5\]\s*=' -Description "direct-run CP337 source order"
Assert-Contains -Path $cp337DirectAssertions -Pattern 'let source_sites = 2 \* active \+ 2 \* second_comparisons \+ body_entries;' -Description "direct-run CP337 dynamic source formula"
Assert-Contains -Path $cp337DirectAssertions -Pattern 'purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle' -Description "direct-run CP336 predecessor evidence"
Assert-Contains -Path $cp337NonDirectTests -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle' -Description "non-direct CP337 null evidence"
Assert-Contains -Path $cp337PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp384_lifecycle_evidence' -Description "non-direct CP337 through CP363 evidence rejection"
Assert-NotContains -Path $cp337Pipeline -Pattern 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|complete_direct_zone_purchased_air_coupling' -Description "numerical DTO reconciliation in CP337 pipeline"

# Exactly two algorithm addenda, two capability addenda, and six target
# occurrences extend inventory without promoting support.
$cp337AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp337AlgorithmAddenda = [regex]::Matches(
    $cp337AlgorithmText,
    '(?m)^\s*"CP337 supersedes only CP336[^"\r\n]+",\s*$'
)
if ($cp337AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP337 claim addenda"
}
$cp337TargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_guard/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_guard\.rs::purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_guard\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_guard\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp337Target in $cp337TargetCounts) {
    $cp337TargetCount = [regex]::Matches($cp337AlgorithmText, $cp337Target.Pattern).Count
    if ($cp337TargetCount -ne $cp337Target.Expected) {
        throw "CP337 target '$($cp337Target.Pattern)' expected $($cp337Target.Expected), found $cp337TargetCount"
    }
}
$cp337CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp337CapabilityAddenda = [regex]::Matches(
    $cp337CapabilityText,
    '(?m)^\s*"CP337 additionally requires[^"\r\n]+",\s*$'
)
if ($cp337CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP337 claim addenda"
}
foreach ($cp337Claim in @($cp337AlgorithmAddenda) + @($cp337CapabilityAddenda)) {
    foreach ($cp337Pattern in @(
            '(?s)PurchasedAirManager\.cc.*?2195',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            'read-cooling-limit-for-capacity-comparison',
            'compare-cooling-limit-equal-to-capacity',
            'read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false',
            'compare-cooling-limit-equal-to-flow-rate-and-capacity',
            'enter-capacity-limit-body-if-compound-condition-satisfied',
            'Enum literals',
            'operand-evaluation-order',
            'short[- ]circuit',
            'LimitCapacity.*three|LimitCapacity.*3',
            'LimitFlowRateAndCapacity.*five|LimitFlowRateAndCapacity.*5',
            'NoLimit.*four|NoLimit.*4',
            '2\*A\s*\+\s*2\*S\s*\+\s*B',
            'CP336-to-CP337-to-numerical',
            '2196',
            '2208',
            'Roadmap state (?:remain|stay) unchanged'
        )) {
        if ($cp337Claim.Value -notmatch $cp337Pattern) {
            throw "CP337 spec addendum missing '$cp337Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP337 supersedes only CP336' -Description "generated CP337 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP337 additionally requires' -Description "generated CP337 capability index"

# Each hand-authored contract contains one scoped CP337 section carrying the
# same source, lazy-selector, counter, placement, exclusion, and non-promotion
# contract.
$cp337DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP337 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP337 Source-Ordered Cooling Positive-Supply Capacity-Limit Guard\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP337 Cooling Positive-Supply Capacity-Limit Guard\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP337 Positive-Supply Capacity-Limit Guard in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP337 Cooling Positive-Supply Capacity-Limit Guard Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp337Documentation in $cp337DocumentationSections) {
    $cp337DocumentText = Read-RepoText -Path $cp337Documentation.Path
    $cp337Matches = [regex]::Matches($cp337DocumentText, $cp337Documentation.Pattern)
    if ($cp337Matches.Count -ne 1) {
        throw "CP337 documentation expected one scoped section in $($cp337Documentation.Path), found $($cp337Matches.Count)"
    }
    $cp337Section = $cp337Matches[0].Value
    foreach ($cp337Pattern in @(
            '(?s)PurchasedAirManager\.cc.*?2195',
            '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005',
            'read-cooling-limit-for-capacity-comparison',
            'compare-cooling-limit-equal-to-capacity',
            'read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false',
            'compare-cooling-limit-equal-to-flow-rate-and-capacity',
            'enter-capacity-limit-body-if-compound-condition-satisfied',
            '(?is)enum literals.*?not',
            'operand-evaluation-order claim',
            '\|\|.*short|short-circuit',
            'CP336',
            'cooling_limit',
            'LimitCapacity',
            'LimitFlowRateAndCapacity',
            'NoLimit',
            'LimitFlowRate',
            'S\s*=\s*A\s*-\s*C|S=A-C',
            'B\s*=\s*C\s*\+\s*F|B=C\+F',
            '2\*A\s*\+\s*2\*S\s*\+\s*B',
            'CP336-to-CP337-to-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle',
            '2196',
            '2208',
            '(?i)numerical[- ]DTO',
            'Roadmap'
        )) {
        if ($cp337Section -notmatch $cp337Pattern) {
            throw "CP337 documentation in $($cp337Documentation.Path) missing '$cp337Pattern'"
        }
    }
}

# Root audit and inventory keep CP337 reachable while placing CP338 and CP339 after it.
# Generated script totals are 284 executable, 240 public, 44 internal, and zero
# uncalled.
$cp337MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp336DotSourceIndexForCp337 = $cp337MainAuditText.IndexOf('ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1')
$cp337DotSourceIndex = $cp337MainAuditText.IndexOf('ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1')
$cp338DotSourceIndexForCp337 = $cp337MainAuditText.IndexOf('ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1')
$cp339DotSourceIndexForCp337 = $cp337MainAuditText.IndexOf('ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1')
$cp337AuditCompletionIndex = $cp337MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp336DotSourceIndexForCp337 -lt 0 -or
    $cp337DotSourceIndex -le $cp336DotSourceIndexForCp337 -or
    $cp338DotSourceIndexForCp337 -le $cp337DotSourceIndex -or
    $cp339DotSourceIndexForCp337 -le $cp338DotSourceIndexForCp337 -or
    $cp337AuditCompletionIndex -le $cp339DotSourceIndexForCp337
) {
    throw "Main IdealLoads audit must dot-source CP337, CP338, and CP339 in order before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 322' -Description "CP337 cumulative inventory total through CP358"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp337-cooling-positive-supply-capacity-limit-guard\.ps1"' -Description "CP337 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp338-cooling-positive-supply-capacity-limit-cp-air-assignment\.ps1"' -Description "CP338 internal script inventory record after CP337"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment\.ps1"' -Description "CP339 internal script inventory record after CP338"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 322 \|' -Description "CP337 cumulative generated script count through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP337 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 82 \|' -Description "CP337 cumulative generated internal script count through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP337 generated uncalled script count"
