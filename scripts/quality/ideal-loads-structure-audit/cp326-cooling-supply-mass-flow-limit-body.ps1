# CP326 maps only PurchasedAirManager.cc executable line 2163: the complete
# cooling supply-mass-flow limit body. Line 2166 is the first excluded
# executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp326ParentModule = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard.rs"
$cp326Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body.rs"
$cp326State = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body\state.rs"
$cp326Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body\transition.rs"
$cp326Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body\release.rs"
$cp326PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body\release\prefix_validation.rs"
$cp326RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body\release\runtime_validation.rs"
$cp326SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body\release\snapshot_validation.rs"
$cp326Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body\tests\mod.rs"
$cp326ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\body\tests\release_corruption.rs"
$cp326ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp326BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_supply_mass_flow_limit_body_tests.rs"
$cp326CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_supply_mass_flow_limit_body_validation.rs"
$cp326CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_supply_mass_flow_limit_body_fixture.rs"
$cp326Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_body.rs"
$cp326PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_body\validation.rs"
$cp326PipelineSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_body\validation\snapshot.rs"
$cp326PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_body\serialization.rs"
$cp326PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_body\serialization\snapshot.rs"
$cp326DirectIntegrationAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_supply_mass_flow_limit_body_assertions.rs"

foreach ($cp326RequiredFile in @(
        $cp326Module,
        $cp326State,
        $cp326Transition,
        $cp326Release,
        $cp326PrefixValidation,
        $cp326RuntimeValidation,
        $cp326SnapshotValidation,
        $cp326Tests,
        $cp326ReleaseCorruptionTests,
        $cp326ScheduledOutput,
        $cp326BindingTests,
        $cp326CoupledValidation,
        $cp326CoupledFixture,
        $cp326Pipeline,
        $cp326PipelineValidation,
        $cp326PipelineSnapshotValidation,
        $cp326PipelineSerialization,
        $cp326PipelineSnapshotSerialization,
        $cp326DirectIntegrationAssertions
    )) {
    Assert-FileExists -Path $cp326RequiredFile -Description "CP326 cooling supply-mass-flow limit body structure"
}

Assert-Contains -Path $cp326ParentModule -Pattern 'mod body;' -Description "CP326 nested body module declaration"
Assert-Contains -Path $cp326ParentModule -Pattern 'pub use body::\*;' -Description "CP326 nested body public re-export"
Assert-Contains -Path $cp326Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2163' -Description "CP326 exact source boundary"
Assert-Contains -Path $cp326Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2166' -Description "CP326 first excluded executable"
Assert-ExactStringArray -Path $cp326Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER" -Expected @(
    "read-supply-mass-flow-rate-for-minimum",
    "reread-maximum-cooling-air-mass-flow-rate-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-supply-mass-flow-rate"
) -Description "CP326 exact four lexical source sites"

Assert-Contains -Path $cp326Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot' -Description "CP326 public snapshot"
Assert-Contains -Path $cp326State -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState' -Description "CP326 persistent state"
Assert-Contains -Path $cp326Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary' -Description "CP326 lifecycle summary"
Assert-Contains -Path $cp326Module -Pattern 'pub fn purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle_summary\s*\(' -Description "CP326 lifecycle accessor"
Assert-Contains -Path $cp326Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body\s*\(' -Description "CP326 exact direct wrapper"
Assert-Contains -Path $cp326Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_supply_mass_flow_limit_body_state\s*\(' -Description "CP326 pure transition"

# The source minimum is ObjexxFCL's exact `a < b ? a : b` double overload.
# The two lexical operand reads are tracked without inventing an evaluation
# order between C++ function arguments.
Assert-PatternsInOrder -Path $cp326Transition -Patterns @(
    'let cooling = predecessor\.cooling_body_entered;',
    'let body_entered = predecessor\.supply_mass_flow_limit_body_entered;',
    'let supply_before = if body_entered',
    'let maximum = body_entered\.then_some\(input\.maximum_cooling_air_mass_flow_rate_kg_per_s\);',
    'let minimum = supply_before',
    '\.zip\(maximum\)',
    '\.map\(\|\(supply, maximum\)\| source_min\(supply, maximum\)\);',
    'let resulting = if cooling',
    'if body_entered',
    'state\.supply_mass_flow_rate_for_minimum_read_count \+= 1;',
    'state\.maximum_cooling_air_mass_flow_rate_for_minimum_read_count \+= 1;',
    'state\.source_shaped_two_argument_minimum_evaluation_count \+= 1;',
    'state\.supply_mass_flow_rate_assignment_count \+= 1;'
) -Description "CP326 operand, minimum, assignment, and counter order"
Assert-Contains -Path $cp326Transition -Pattern '(?s)fn source_min\(left: f64, right: f64\) -> f64 \{\s*if left < right \{\s*left\s*\} else \{\s*right\s*\}\s*\}' -Description "CP326 strict less-than source minimum"
Assert-Contains -Path $cp326Transition -Pattern 'if predecessor\.unit_off_skipped' -Description "CP326 UnitOff complete skip"
Assert-Contains -Path $cp326Transition -Pattern 'else if predecessor\.non_cooling_skipped' -Description "CP326 non-cooling complete skip"
Assert-Contains -Path $cp326Transition -Pattern 'ActiveGuardFalseFallthrough' -Description "CP326 active guard-false complete skip"
Assert-Contains -Path $cp326Transition -Pattern 'SupplyMassFlowLimitApplied' -Description "CP326 applied route"
Assert-Contains -Path $cp326Tests -Pattern 'source_boundary_and_exact_four_sites_are_stable' -Description "CP326 exact source-order regression"
Assert-Contains -Path $cp326Tests -Pattern 'source_min_preserves_ties_unordered_payloads_and_infinities' -Description "CP326 IEEE source-minimum regression"
Assert-Contains -Path $cp326Tests -Pattern 'unit_off_non_cooling_and_active_guard_false_skip_every_lexical_site' -Description "CP326 complete-skip route regression"
Assert-Contains -Path $cp326Tests -Pattern 'bit_exact_snapshot_comparison_rejects_one_sided_signed_zero_corruption' -Description "CP326 signed-zero corruption regression"
Assert-Contains -Path $cp326ReleaseCorruptionTests -Pattern 'private_cp325_witness_or_cp322_supply_corruption_is_rejected_without_mutation' -Description "CP326 retained-lineage corruption regression"
Assert-Contains -Path $cp326ReleaseCorruptionTests -Pattern 'invalid_cached_maximum_and_counter_underflow_fail_closed_without_mutation' -Description "CP326 cache/counter corruption regression"

foreach ($cp326ForbiddenHelper in @(
        '(?<![A-Za-z0-9_])f64::min\s*\(',
        '\.min\s*\(',
        '\.(?:total_cmp|partial_cmp|clamp)\s*\(',
        '\.is_finite\(\)'
    )) {
    Assert-NotContains -Path $cp326Transition -Pattern $cp326ForbiddenHelper -Description "replacement minimum or normalized comparison in CP326 transition"
}

# Line 2166 and all numerical/load behavior remain outside the core CP326
# release boundary. Coupled and pipeline layers validate retained provenance;
# the line-2163 checkpoint is not reconciled with the downstream final DTO.
foreach ($cp326ScopeFile in @(
        $cp326Module,
        $cp326State,
        $cp326Transition,
        $cp326Release,
        $cp326PrefixValidation,
        $cp326RuntimeValidation,
        $cp326SnapshotValidation
    )) {
    Assert-NotContains -Path $cp326ScopeFile -Pattern 'VerySmallMassFlow|CalcPurchAirMixedAir|mixed[_-]?air|positive[_-]?zero[_-]?reset' -Description "line-2166-or-later behavior in CP326"
    Assert-NotContains -Path $cp326ScopeFile -Pattern '(?i)(?<![A-Za-z0-9_])(?:QZnCoolSP|QZnHeatSP|zone_(?:sensible_)?demand(?:_w)?|(?:remaining|requested)_(?:heating_|cooling_|sensible_|latent_)?(?:load|output)(?:_w)?|(?:sensible|latent|heating|cooling)_load(?:_w)?)(?![A-Za-z0-9_])' -Description "load or demand dependency in CP326"
    Assert-NotContains -Path $cp326ScopeFile -Pattern 'DirectZonePurchasedAirCoupling(?:Input|Output)?|complete_direct_zone_purchased_air_coupling' -Description "numerical DTO dependency in CP326 core release"
}

function Assert-Cp326AllowedRustCalls {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string[]]$Allowed,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $code = Read-RepoText -Path $Path
    $code = [regex]::Replace($code, '(?m)//.*$', '')
    $code = [regex]::Replace($code, '(?s)/\*.*?\*/', '')
    $callMatches = [regex]::Matches(
        $code,
        '(?<![A-Za-z0-9_:])!?(?<name>(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*)(?<macro>!)?\s*\('
    )
    foreach ($callMatch in $callMatches) {
        $callName = $callMatch.Groups["name"].Value + $callMatch.Groups["macro"].Value
        if ($callName -cin @("allow", "derive", "match", "pub")) {
            continue
        }
        if ($Allowed -cnotcontains $callName) {
            throw "$Description has unclassified call '$callName' in $Path"
        }
    }
}

Assert-Cp326AllowedRustCalls -Path $cp326Release -Allowed @(
    "advance_cooling_supply_mass_flow_limit_body_state",
    "advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body",
    "calc_state_identities_match",
    "call_order_error",
    "call_order_is_pending_body",
    "cfg",
    "classify_no_oa_sensible_subset",
    "completed_body_state_is_consistent",
    "completed_direct_cooling_supply_mass_flow_limit_body_is_consistent",
    "completed_direct_cooling_supply_mass_flow_limit_guard_is_consistent",
    "cooling_supply_mass_flow_limit_body_latest_witness",
    "cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release",
    "cooling_supply_mass_flow_limit_guard_latest_witness",
    "cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release",
    "debug_assert!",
    "Err",
    "exact_direct_initialization_is_consistent",
    "get",
    "get_mut",
    "is_finite",
    "is_none",
    "is_none_or",
    "is_some",
    "is_some_and",
    "is_supported",
    "limit_body_inputs_link_to_supply_maximum_and_cache",
    "limit_body_links_to_guard",
    "Ok",
    "ok_or",
    "pending_body_state_is_consistent",
    "PurchasedAirSizedLimits::from_system",
    "set_cooling_supply_mass_flow_limit_body_latest_witness",
    "Some"
) -Description "CP326 public release boundary"
Assert-Cp326AllowedRustCalls -Path $cp326PrefixValidation -Allowed @(
    "has_bits",
    "is_none",
    "is_some_and",
    "limit_body_inputs_link_to_supply_maximum_and_cache",
    "limit_body_links_to_guard",
    "Some",
    "source_min",
    "to_bits",
    "zip"
) -Description "CP326 prefix validator"
Assert-Cp326AllowedRustCalls -Path $cp326RuntimeValidation -Allowed @(
    "and_then",
    "calc_state_identities_match",
    "call_order_is_pending_body",
    "checked_add",
    "checked_sub",
    "completed_body_history_links_to_guard",
    "completed_body_state_is_consistent",
    "cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release",
    "is_some_and",
    "latest_is_valid",
    "partition_is_consistent",
    "pending_body_state_is_consistent",
    "snapshot_route",
    "snapshots_match_bit_exact",
    "Some",
    "source_counters_are_consistent",
    "usize::from"
) -Description "CP326 retained-runtime validator"
Assert-Cp326AllowedRustCalls -Path $cp326SnapshotValidation -Allowed @(
    "applied_fields_are_exact",
    "cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release",
    "fallthrough_fields_are_exact",
    "is_none",
    "is_some",
    "option_bits_match",
    "skipped_fields_are_exact",
    "skipped_lexical_fields_are_exact",
    "snapshot_is_exact_source_characterization",
    "snapshot_route",
    "snapshots_match_bit_exact",
    "Some",
    "source_min",
    "to_bits",
    "usize::from"
) -Description "CP326 snapshot validator"

# Exact release consumes retained CP325/CP322/Init state and accepts no
# duplicate caller flow scalar.
Assert-Contains -Path $cp326Release -Pattern 'PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot' -Description "CP326 CP325 predecessor type"
Assert-Contains -Path $cp326Release -Pattern 'completed_direct_cooling_supply_mass_flow_limit_guard_is_consistent\s*\(' -Description "CP326 completed CP325 prefix validation"
Assert-Contains -Path $cp326Release -Pattern 'cooling_supply_mass_flow_limit_guard_latest_witness\s*\(' -Description "CP326 retained CP325 private witness"
Assert-Contains -Path $cp326Release -Pattern 'unit\.calc_cooling_supply_mass_flow_maximum\.latest' -Description "CP326 retained CP322 supply-flow source"
Assert-Contains -Path $cp326Release -Pattern 'maximum_snapshot\.resulting_supply_mass_flow_rate_kg_per_s' -Description "CP326 pre-clamp CP322 result"
Assert-Contains -Path $cp326Release -Pattern 'unit\.maximum_cooling_air_mass_flow_rate_kg_per_s' -Description "CP326 retained Init maximum-flow source"
Assert-Contains -Path $cp326Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp325: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,\s*\)' -Description "CP326 exact wrapper arguments without duplicate flow scalar"
Assert-Contains -Path $cp326PrefixValidation -Pattern '(?s)retained_supply.*resulting_supply_mass_flow_rate_kg_per_s.*result\.to_bits\(\) == expected\.to_bits\(\)' -Description "CP326 CP322-result exact-bit lineage"
Assert-Contains -Path $cp326SnapshotValidation -Pattern 'minimum\.to_bits\(\) == source_min\(supply, maximum\)\.to_bits\(\)' -Description "CP326 exact source-minimum bits"
Assert-NotContains -Path $cp326Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body\([^)]*(?:supply_mass_flow_rate|maximum_cooling_air_mass_flow_rate)_kg_per_s\s*:' -Description "duplicate caller flow scalar in CP326 release"
Assert-NotContains -Path $cp326Release -Pattern 'ems_actuator|ems_service|node_service|psychrometric|schedule_service|diagnostic_service' -Description "live service input in CP326 release"

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_supply_mass_flow_limit_body_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot' -Description "runtime-root private CP326 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_supply_mass_flow_limit_body_latest_witnesses:' -Description "public runtime-root CP326 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_supply_mass_flow_limit_body_latest_witness\s*\(' -Description "runtime-root CP326 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_supply_mass_flow_limit_body_latest_witness\s*\(' -Description "runtime-root CP326 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_supply_mass_flow_limit_body:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState' -Description "per-unit CP326 persistent state"

# Binding order is CP325 -> CP326 -> CP327 -> CP328 -> CP329 -> CP330 ->
# CP331 -> CP332 -> CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 ->
# the unchanged numerical DTO.
$cp326BindingText = Read-RepoText -Path $idealLoadsBinding
if ($cp326BindingText -notmatch '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(') {
    throw "Historical binding audit must retain CP359 then CP360 before numerical coupling"
}
$cp325BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_supply_mass_flow_limit_guard =")
$cp326BindingIndex = $cp326BindingText.IndexOf("let calculation_cooling_supply_mass_flow_limit_body =")
$cp327BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_supply_mass_flow_very_small_guard =")
$cp328BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_supply_mass_flow_very_small_guard_body =")
$cp329BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_mixed_air_call =")
$cp330BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_supply_mass_flow_positive_guard =")
$cp331BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_cp_air_assignment =")
$cp332BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_assignment =")
$cp333BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_minimum_limit =")
$cp334BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_temperature_mixed_air_limit =")
$cp335BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =")
$cp336BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_enthalpy_assignment =")
$cp337BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_guard =")
$cp338BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =")
$cp339BindingIndexForCp326 = $cp326BindingText.IndexOf("let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =")
$numericalBindingIndexForCp326 = $cp326BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp325BindingIndexForCp326 -lt 0 -or
    $cp326BindingIndex -le $cp325BindingIndexForCp326 -or
    $cp327BindingIndexForCp326 -le $cp326BindingIndex -or
    $cp328BindingIndexForCp326 -le $cp327BindingIndexForCp326 -or
    $cp329BindingIndexForCp326 -le $cp328BindingIndexForCp326 -or
    $cp330BindingIndexForCp326 -le $cp329BindingIndexForCp326 -or
    $cp331BindingIndexForCp326 -le $cp330BindingIndexForCp326 -or
    $cp332BindingIndexForCp326 -le $cp331BindingIndexForCp326 -or
    $cp333BindingIndexForCp326 -le $cp332BindingIndexForCp326 -or
    $cp334BindingIndexForCp326 -le $cp333BindingIndexForCp326 -or
    $cp335BindingIndexForCp326 -le $cp334BindingIndexForCp326 -or
    $cp336BindingIndexForCp326 -le $cp335BindingIndexForCp326 -or
    $cp337BindingIndexForCp326 -le $cp336BindingIndexForCp326 -or
    $cp338BindingIndexForCp326 -le $cp337BindingIndexForCp326 -or
    $cp339BindingIndexForCp326 -le $cp338BindingIndexForCp326 -or
    $numericalBindingIndexForCp326 -le $cp339BindingIndexForCp326
) {
    throw "Binding must retain exact CP325 -> CP326 -> CP327 -> CP328 -> CP329 -> CP330 -> CP331 -> CP332 -> CP333 -> CP334 -> CP335 -> CP336 -> CP337 -> CP338 -> CP339 -> numerical Calc order"
}
Assert-Contains -Path $idealLoadsBinding -Pattern '(?s)let calculation_cooling_supply_mass_flow_limit_body =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_supply_mass_flow_limit_guard,\s*\)' -Description "binding exact CP325-to-CP326 wrapper call without flow scalar"
$cp325BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_limit_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard\(.*?CalculationCoolingSupplyMassFlowLimitGuard,\s*\)\?;'
)
$cp326BindingCall = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_limit_body =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body\(.*?CalculationCoolingSupplyMassFlowLimitBody,\s*\)\?;'
)
$cp327BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_very_small_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard\(.*?CalculationCoolingSupplyMassFlowVerySmallGuard,\s*\)\?;'
)
$cp328BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_very_small_guard_body =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body\(.*?CalculationCoolingSupplyMassFlowVerySmallGuardBody,\s*\)\?;'
)
$cp329BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_mixed_air_call =\s*advance_direct_no_oa_calc_cooling_mixed_air_call\(.*?CalculationCoolingMixedAirCall,?\s*\)\?;'
)
$cp330BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_positive_guard =\s*advance_positive_guard\([^;]+?\)\?;'
)
$cp331BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_cp_air_assignment =\s*advance_positive_supply_cp_air_assignment\([^;]+?\)\?;'
)
$cp332BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_assignment =\s*advance_positive_supply_temperature_assignment\([^;]+?\)\?;'
)
$cp333BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_minimum_limit =\s*advance_positive_supply_temperature_minimum_limit\([^;]+?\)\?;'
)
$cp334BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_temperature_mixed_air_limit =\s*advance_positive_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
$cp335BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
)
$cp336BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_enthalpy_assignment =\s*advance_positive_supply_enthalpy_assignment\([^;]+?\)\?;'
)
$cp337BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_guard =\s*advance_positive_supply_capacity_limit_guard\([^;]+?\)\?;'
)
$cp338BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_cp_air_assignment =\s*advance_positive_supply_capacity_limit_cp_air_assignment\([^;]+?\)\?;'
)
$cp339BindingCallForCp326 = [regex]::Match(
    $cp326BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_assignment\([^;]+?\)\?;'
)
if (
    -not $cp325BindingCallForCp326.Success -or
    -not $cp326BindingCall.Success -or
    -not $cp327BindingCallForCp326.Success -or
    -not $cp328BindingCallForCp326.Success -or
    -not $cp329BindingCallForCp326.Success -or
    -not $cp330BindingCallForCp326.Success -or
    -not $cp331BindingCallForCp326.Success -or
    -not $cp332BindingCallForCp326.Success -or
    -not $cp333BindingCallForCp326.Success -or
    -not $cp334BindingCallForCp326.Success -or
    -not $cp335BindingCallForCp326.Success -or
    -not $cp336BindingCallForCp326.Success -or
    -not $cp337BindingCallForCp326.Success -or
    -not $cp338BindingCallForCp326.Success -or
    -not $cp339BindingCallForCp326.Success
) {
    throw "Binding must retain complete CP325, CP326, CP327, CP328, CP329, CP330, CP331, CP332, CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls"
}
$cp325BindingCallEndForCp326 =
    $cp325BindingCallForCp326.Index + $cp325BindingCallForCp326.Length
$cp326BindingCallEnd = $cp326BindingCall.Index + $cp326BindingCall.Length
$cp327BindingCallEndForCp326 =
    $cp327BindingCallForCp326.Index + $cp327BindingCallForCp326.Length
$cp328BindingCallEndForCp326 =
    $cp328BindingCallForCp326.Index + $cp328BindingCallForCp326.Length
$cp329BindingCallEndForCp326 =
    $cp329BindingCallForCp326.Index + $cp329BindingCallForCp326.Length
$cp330BindingCallEndForCp326 =
    $cp330BindingCallForCp326.Index + $cp330BindingCallForCp326.Length
$cp331BindingCallEndForCp326 =
    $cp331BindingCallForCp326.Index + $cp331BindingCallForCp326.Length
$cp332BindingCallEndForCp326 =
    $cp332BindingCallForCp326.Index + $cp332BindingCallForCp326.Length
$cp333BindingCallEndForCp326 =
    $cp333BindingCallForCp326.Index + $cp333BindingCallForCp326.Length
$cp334BindingCallEndForCp326 =
    $cp334BindingCallForCp326.Index + $cp334BindingCallForCp326.Length
$cp335BindingCallEndForCp326 =
    $cp335BindingCallForCp326.Index + $cp335BindingCallForCp326.Length
$cp336BindingCallEndForCp326 =
    $cp336BindingCallForCp326.Index + $cp336BindingCallForCp326.Length
$cp337BindingCallEndForCp326 =
    $cp337BindingCallForCp326.Index + $cp337BindingCallForCp326.Length
$cp338BindingCallEndForCp326 =
    $cp338BindingCallForCp326.Index + $cp338BindingCallForCp326.Length
$cp339BindingCallEndForCp326 =
    $cp339BindingCallForCp326.Index + $cp339BindingCallForCp326.Length
if (
    $cp326BindingIndex -lt $cp325BindingCallEndForCp326 -or
    $cp327BindingIndexForCp326 -lt $cp326BindingCallEnd -or
    $cp328BindingIndexForCp326 -lt $cp327BindingCallEndForCp326 -or
    $cp329BindingIndexForCp326 -lt $cp328BindingCallEndForCp326 -or
    $cp330BindingIndexForCp326 -lt $cp329BindingCallEndForCp326 -or
    $cp331BindingIndexForCp326 -lt $cp330BindingCallEndForCp326 -or
    $cp332BindingIndexForCp326 -lt $cp331BindingCallEndForCp326 -or
    $cp333BindingIndexForCp326 -lt $cp332BindingCallEndForCp326 -or
    $cp334BindingIndexForCp326 -lt $cp333BindingCallEndForCp326 -or
    $cp335BindingIndexForCp326 -lt $cp334BindingCallEndForCp326 -or
    $cp336BindingIndexForCp326 -lt $cp335BindingCallEndForCp326 -or
    $cp337BindingIndexForCp326 -lt $cp336BindingCallEndForCp326 -or
    $cp338BindingIndexForCp326 -lt $cp337BindingCallEndForCp326 -or
    $cp339BindingIndexForCp326 -lt $cp338BindingCallEndForCp326 -or
    $numericalBindingIndexForCp326 -lt $cp339BindingCallEndForCp326
) {
    throw "CP325, CP326, CP327, CP328, CP329, CP330, CP331, CP332, CP333, CP334, CP335, CP336, CP337, CP338, and CP339 exact release calls must complete in source order before numerical Calc"
}
$postCp325BeforeCp326 = $cp326BindingText.Substring(
    $cp325BindingCallEndForCp326,
    $cp326BindingIndex - $cp325BindingCallEndForCp326
)
if ($postCp325BeforeCp326 -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP325 and before CP326"
}
$postCp326BeforeCp327 = $cp326BindingText.Substring(
    $cp326BindingCallEnd,
    $cp327BindingIndexForCp326 - $cp326BindingCallEnd
)
$postCp326BeforeCp327Code = [regex]::Replace($postCp326BeforeCp327, '(?m)//.*$', '')
if ($postCp326BeforeCp327Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP326 and before CP327"
}
$postCp327BeforeCp328 = $cp326BindingText.Substring(
    $cp327BindingCallEndForCp326,
    $cp328BindingIndexForCp326 - $cp327BindingCallEndForCp326
)
$postCp327BeforeCp328Code = [regex]::Replace($postCp327BeforeCp328, '(?m)//.*$', '')
if ($postCp327BeforeCp328Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP327 and before CP328"
}
$postCp328BeforeCp329 = $cp326BindingText.Substring(
    $cp328BindingCallEndForCp326,
    $cp329BindingIndexForCp326 - $cp328BindingCallEndForCp326
)
$postCp328BeforeCp329Code = [regex]::Replace($postCp328BeforeCp329, '(?m)//.*$', '')
if ($postCp328BeforeCp329Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP328 and before CP329"
}
$postCp329BeforeCp330 = $cp326BindingText.Substring(
    $cp329BindingCallEndForCp326,
    $cp330BindingIndexForCp326 - $cp329BindingCallEndForCp326
)
$postCp329BeforeCp330Code = [regex]::Replace($postCp329BeforeCp330, '(?m)//.*$', '')
if ($postCp329BeforeCp330Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP329 and before CP330"
}
$postCp330BeforeCp331ForCp326 = $cp326BindingText.Substring(
    $cp330BindingCallEndForCp326,
    $cp331BindingIndexForCp326 - $cp330BindingCallEndForCp326
)
$postCp330BeforeCp331CodeForCp326 =
    [regex]::Replace($postCp330BeforeCp331ForCp326, '(?m)//.*$', '')
if ($postCp330BeforeCp331CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP330 and before CP331"
}
$postCp331BeforeCp332ForCp326 = $cp326BindingText.Substring(
    $cp331BindingCallEndForCp326,
    $cp332BindingIndexForCp326 - $cp331BindingCallEndForCp326
)
$postCp331BeforeCp332CodeForCp326 =
    [regex]::Replace($postCp331BeforeCp332ForCp326, '(?m)//.*$', '')
if ($postCp331BeforeCp332CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP331 and before CP332"
}
$postCp332BeforeCp333ForCp326 = $cp326BindingText.Substring(
    $cp332BindingCallEndForCp326,
    $cp333BindingIndexForCp326 - $cp332BindingCallEndForCp326
)
$postCp332BeforeCp333CodeForCp326 =
    [regex]::Replace($postCp332BeforeCp333ForCp326, '(?m)//.*$', '')
if ($postCp332BeforeCp333CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP332 and before CP333"
}
$postCp333BeforeCp334ForCp326 = $cp326BindingText.Substring(
    $cp333BindingCallEndForCp326,
    $cp334BindingIndexForCp326 - $cp333BindingCallEndForCp326
)
$postCp333BeforeCp334CodeForCp326 =
    [regex]::Replace($postCp333BeforeCp334ForCp326, '(?m)//.*$', '')
if ($postCp333BeforeCp334CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP333 and before CP334"
}
$postCp334BeforeCp335ForCp326 = $cp326BindingText.Substring(
    $cp334BindingCallEndForCp326,
    $cp335BindingIndexForCp326 - $cp334BindingCallEndForCp326
)
$postCp334BeforeCp335CodeForCp326 =
    [regex]::Replace($postCp334BeforeCp335ForCp326, '(?m)//.*$', '')
if ($postCp334BeforeCp335CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP334 and before CP335"
}
$postCp335BeforeCp336ForCp326 = $cp326BindingText.Substring(
    $cp335BindingCallEndForCp326,
    $cp336BindingIndexForCp326 - $cp335BindingCallEndForCp326
)
$postCp335BeforeCp336CodeForCp326 =
    [regex]::Replace($postCp335BeforeCp336ForCp326, '(?m)//.*$', '')
if ($postCp335BeforeCp336CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP335 and before CP336"
}
$postCp336BeforeCp337ForCp326 = $cp326BindingText.Substring(
    $cp336BindingCallEndForCp326,
    $cp337BindingIndexForCp326 - $cp336BindingCallEndForCp326
)
$postCp336BeforeCp337CodeForCp326 =
    [regex]::Replace($postCp336BeforeCp337ForCp326, '(?m)//.*$', '')
if ($postCp336BeforeCp337CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP336 and before CP337"
}
$postCp337BeforeCp338ForCp326 = $cp326BindingText.Substring(
    $cp337BindingCallEndForCp326,
    $cp338BindingIndexForCp326 - $cp337BindingCallEndForCp326
)
$postCp337BeforeCp338CodeForCp326 =
    [regex]::Replace($postCp337BeforeCp338ForCp326, '(?m)//.*$', '')
if ($postCp337BeforeCp338CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP337 and before CP338"
}
$postCp338BeforeCp339ForCp326 = $cp326BindingText.Substring(
    $cp338BindingCallEndForCp326,
    $cp339BindingIndexForCp326 - $cp338BindingCallEndForCp326
)
$postCp338BeforeCp339CodeForCp326 =
    [regex]::Replace($postCp338BeforeCp339ForCp326, '(?m)//.*$', '')
if ($postCp338BeforeCp339CodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP338 and before CP339"
}
$postCp339BeforeNumericalForCp326 = $cp326BindingText.Substring(
    $cp339BindingCallEndForCp326,
    $numericalBindingIndexForCp326 - $cp339BindingCallEndForCp326
)
$postCp339BeforeNumericalCodeForCp326 =
    [regex]::Replace($postCp339BeforeNumericalForCp326, '(?m)//.*$', '')
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_guard =\s*advance_positive_supply_capacity_limit_sensible_output_guard\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\([^;]+?\)\?;)',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\([^;]+?\)\?;',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\([^;]+?\)\?;)',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry =\s*advance_cooling_supply_mass_flow_positive_guard_else_branch_entry\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment =\s*advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment =\s*advance_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment\([^;]+?\)\?;|let calculation_heating_or_no_load_case_entry =\s*advance_heating_or_no_load_case_entry\([^;]+?\)\?;|let calculation_heating_mode_guard =\s*advance_heating_mode_guard\([^;]+?\)\?;|let calculation_heating_operating_mode_heat_assignment =\s*advance_heating_operating_mode_heat_assignment\([^;]+?\)\?;)',
    ''
)
$postCp339BeforeNumericalCodeForCp326 = [regex]::Replace(
    $postCp339BeforeNumericalCodeForCp326,
    '(?s)(?:let calculation_heating_mode_guard_else_branch_entry =\s*advance_heating_mode_guard_else_branch_entry\([^;]+?\)\?;|let calculation_heating_operating_mode_deadband_assignment =\s*advance_heating_operating_mode_deadband_assignment\([^;]+?\)\?;|let calculation_heating_outdoor_air_maximum_flow_guard =\s*advance_heating_outdoor_air_maximum_flow_guard\([^;]+?\)\?;|let calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment =\s*advance_heating_outdoor_air_maximum_flow_body_volume_flow_assignment\([^;]+?\)\?;)',
    ''
)
if ($postCp339BeforeNumericalCodeForCp326 -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No helper other than the audited CP340 through CP436 releases may execute after CP339 and before numerical Calc"
}

Assert-Contains -Path $idealLoadsBinding -Pattern 'CalculationCoolingSupplyMassFlowLimitBody\(\s*PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError,?\s*\)' -Description "CP326 scheduled binding error boundary"
Assert-Contains -Path $cp326ScheduledOutput -Pattern 'pub calculation_cooling_supply_mass_flow_limit_body:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot' -Description "CP326 scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_supply_mass_flow_limit_body_tests\.rs"\]' -Description "CP326 binding test module path"
Assert-Contains -Path $cp326BindingTests -Pattern 'scheduled_binding_applies_line_2163_only_after_true_cp325_body_entry' -Description "CP326 applied/fallthrough binding regression"
Assert-Contains -Path $cp326BindingTests -Pattern 'scheduled_binding_skips_all_cp326_sites_when_cooling_is_inactive' -Description "CP326 UnitOff/non-cooling binding regression"
Assert-Contains -Path $cp326BindingTests -Pattern 'public_cp326_release_rejects_replay_and_forged_cp325_ordinal_without_mutation' -Description "CP326 release corruption regression"

# Coupled validation independently reconstructs CP326 from retained CP322 and
# Init state with exact IEEE bits. The downstream final DTO is not a line-2163
# comparison target.
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_supply_mass_flow_limit_body_validation;' -Description "coupled CP326 validator declaration"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_supply_mass_flow_limit_body_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary' -Description "coupled CP326 lifecycle"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_supply_mass_flow_limit_body_validation::snapshot_matches_release' -Description "coupled per-timestep CP326 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_supply_mass_flow_limit_body_validation::validate_lifecycle' -Description "coupled final CP326 validation"
Assert-Contains -Path $cp326CoupledValidation -Pattern 'calculation_cooling_supply_mass_flow_maximum[\r\n\s.]+resulting_supply_mass_flow_rate_kg_per_s' -Description "coupled CP322 supply-flow reconstruction"
Assert-Contains -Path $cp326CoupledValidation -Pattern 'initialization[\r\n\s.]+maximum_cooling_air_mass_flow_rate_kg_per_s' -Description "coupled retained Init maximum reconstruction"
Assert-Contains -Path $cp326CoupledValidation -Pattern '(?s)fn source_min\(left: f64, right: f64\) -> f64 \{\s*if left < right \{\s*left\s*\} else \{\s*right\s*\}\s*\}' -Description "coupled source-shaped minimum"
Assert-Contains -Path $cp326CoupledValidation -Pattern '(?s)snapshots_match_exact_bits.*?options_have_exact_bits.*?to_bits\(\)' -Description "coupled CP326 retained-provenance exact-bit validation"
foreach ($cp326ExactBitField in @(
        "supply_mass_flow_rate_before_limit_kg_per_s",
        "maximum_cooling_air_mass_flow_rate_kg_per_s",
        "minimum_supply_mass_flow_rate_kg_per_s",
        "assigned_supply_mass_flow_rate_kg_per_s",
        "resulting_supply_mass_flow_rate_kg_per_s"
    )) {
    $escapedCp326ExactBitField = [regex]::Escape($cp326ExactBitField)
    Assert-Contains -Path $cp326CoupledValidation -Pattern (
        '(?s)left\.' + $escapedCp326ExactBitField +
        '\s*,\s*right\.' + $escapedCp326ExactBitField
    ) -Description "coupled CP326 exact-bit comparison for '$cp326ExactBitField'"
    Assert-Contains -Path $cp326SnapshotValidation -Pattern (
        '(?s)left\.' + $escapedCp326ExactBitField +
        '\s*,\s*right\.' + $escapedCp326ExactBitField
    ) -Description "core CP326 exact-bit comparison for '$cp326ExactBitField'"
}
Assert-NotContains -Path $cp326CoupledValidation -Pattern 'resulting_flow_matches_numerical|(?s)output\s*\.\s*coupling\s*\.\s*purchased_air\s*\.\s*calculation\s*\.\s*supply_mass_flow_rate_kg_per_s' -Description "final numerical DTO result reconciliation in CP326 coupled validation"
foreach ($cp326Counter in @(
        "supply_mass_flow_rate_for_minimum_read_count",
        "maximum_cooling_air_mass_flow_rate_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_mass_flow_rate_assignment_count"
    )) {
    Assert-Contains -Path $cp326CoupledValidation -Pattern $cp326Counter -Description "coupled CP326 source counter '$cp326Counter'"
}
Assert-Contains -Path $cp326CoupledValidation -Pattern 'latest_result_signed_zero_corruption_fails_closed' -Description "coupled CP326 exact-bit corruption regression"
Assert-Contains -Path $cp326CoupledFixture -Pattern '(?s)fn source_min\(left: f64, right: f64\) -> f64 \{\s*if left < right \{\s*left\s*\} else \{\s*right\s*\}\s*\}' -Description "coupled fixture source-shaped minimum"

# The run pipeline preserves retained CP322/Init provenance and exact CP326
# bits, rejects non-direct evidence, and does not compare the checkpoint with
# the downstream final numerical DTO.
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_supply_mass_flow_limit_body;' -Description "pipeline CP326 module declaration"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle' -Description "pipeline CP326 lifecycle JSON key"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_supply_mass_flow_limit_body::validate_direct_lifecycle' -Description "pipeline CP326 direct firewall"
Assert-Contains -Path $cp326Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER' -Description "pipeline CP322-to-CP326 lineage"
Assert-Contains -Path $cp326Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER' -Description "pipeline CP325-to-CP326 lineage"
Assert-Contains -Path $cp326Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER' -Description "pipeline CP326 source order"
Assert-Contains -Path $cp326PipelineValidation -Pattern 'supply_mass_flow_rate_for_minimum_read_count' -Description "pipeline CP326 operand-read counter"
Assert-Contains -Path $cp326PipelineValidation -Pattern 'source_shaped_two_argument_minimum_evaluation_count' -Description "pipeline CP326 minimum counter"
Assert-Contains -Path $cp326PipelineValidation -Pattern 'supply_mass_flow_rate_assignment_count' -Description "pipeline CP326 assignment counter"
Assert-Contains -Path $cp326PipelineSnapshotValidation -Pattern '(?s)fn source_min\(left: f64, right: f64\) -> f64 \{\s*if left < right \{\s*left\s*\} else \{\s*right\s*\}\s*\}' -Description "pipeline source-shaped minimum"
Assert-Contains -Path $cp326PipelineSnapshotValidation -Pattern 'to_bits\(\) == expected\.to_bits\(\)' -Description "pipeline exact-bit snapshot validation"
Assert-NotContains -Path $cp326Pipeline -Pattern 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow' -Description "final numerical DTO result reconciliation in CP326 pipeline"
Assert-Contains -Path $cp326PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP326 latest serialization"
Assert-Contains -Path $cp326PipelineSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP326 source-order JSON"
foreach ($cp326ValueField in @(
        "supply_mass_flow_rate_before_limit_kg_per_s",
        "maximum_cooling_air_mass_flow_rate_kg_per_s",
        "minimum_supply_mass_flow_rate_kg_per_s",
        "assigned_supply_mass_flow_rate_kg_per_s",
        "resulting_supply_mass_flow_rate_kg_per_s"
    )) {
    Assert-Contains -Path $cp326PipelineSnapshotSerialization -Pattern ('"' + [regex]::Escape($cp326ValueField) + '_ieee_bits"') -Description "pipeline CP326 IEEE field '$cp326ValueField'"
}
Assert-Contains -Path $cp326PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP326 exact IEEE serialization"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'mod cooling_supply_mass_flow_limit_body_assertions;' -Description "direct integration CP326 assertion module"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'assert_cooling_supply_mass_flow_limit_body\(' -Description "direct integration CP326 assertion calls"
Assert-Contains -Path $cp326DirectIntegrationAssertions -Pattern 'purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle' -Description "direct integration CP326 lifecycle key"
Assert-Contains -Path $cp326DirectIntegrationAssertions -Pattern 'resulting_supply_mass_flow_rate_kg_per_s_ieee_bits' -Description "direct integration CP326 exact-bit evidence"

function Assert-Cp326ScopedText {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ($Text -notmatch $Pattern) {
        throw "$Description missing from its CP326-scoped entry"
    }
}

function Get-Cp326TomlArrayEntry {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Table,
        [Parameter(Mandatory = $true)][string]$Id,
        [Parameter(Mandatory = $true)][string]$ArrayName,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    $tableName = [regex]::Escape($Table)
    $idText = [regex]::Escape($Id)
    $sectionMatches = [regex]::Matches(
        $text,
        "(?ms)^\[\[$tableName\]\]\r?\nid\s*=\s*`"$idText`"\r?\n(?<section>.*?)(?=^\[\[$tableName\]\]|\z)"
    )
    if ($sectionMatches.Count -ne 1) {
        throw "$Description expected one '$Id' table in $Path, found $($sectionMatches.Count)"
    }
    $section = $sectionMatches[0].Value
    $arrayNameText = [regex]::Escape($ArrayName)
    $arrayMatches = [regex]::Matches(
        $section,
        "(?ms)^$arrayNameText\s*=\s*\[\s*\r?\n(?<array>.*?)(?=^\]\s*$)"
    )
    if ($arrayMatches.Count -ne 1) {
        throw "$Description expected one '$ArrayName' array in '$Id'"
    }
    $cp326Matches = [regex]::Matches(
        $arrayMatches[0].Groups["array"].Value,
        '(?m)^\s*"(?<entry>CP326 [^"\r\n]+)",\s*$'
    )
    if ($cp326Matches.Count -ne 1) {
        throw "$Description expected one CP326 addendum in '$Id', found $($cp326Matches.Count)"
    }
    return [PSCustomObject]@{
        Section = $section
        Entry = $cp326Matches[0].Groups["entry"].Value
    }
}

function Get-Cp326GeneratedRow {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Id,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    $idText = [regex]::Escape($Id)
    $rowMatches = [regex]::Matches($text, "(?m)^\|\s*$idText\s*\|[^\r\n]*$")
    if ($rowMatches.Count -ne 1) {
        throw "$Description expected one generated '$Id' row in $Path, found $($rowMatches.Count)"
    }
    return $rowMatches[0].Value
}

function Assert-Cp326BoundaryStatement {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-Cp326ScopedText -Text $Text -Pattern '(?:executable\s+)?line(?:-|\s+)2163' -Description "$Description exact source line"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)(?:complete\s+)?four(?:-site|\s+lexical\s+(?:source\s+)?sites)' -Description "$Description exact four sites"
    Assert-Cp326ScopedText -Text $Text -Pattern 'a < b \? a : b' -Description "$Description ObjexxFCL source minimum"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)(?:tie|equal)' -Description "$Description equality/tie semantics"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)(?:unordered|NaN)' -Description "$Description unordered semantics"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)(?:second|right)(?:-|\s+)(?:maximum(?:-flow)?\s+)?operand' -Description "$Description right-operand selection"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)UnitOff' -Description "$Description UnitOff route"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)non-cooling' -Description "$Description non-cooling route"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)(?:guard-false|guard is false)' -Description "$Description active guard-false route"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)skip' -Description "$Description complete skip semantics"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)Line 2166,\s*not line 2167,\s*is the first excluded executable|Line 2166 is the first excluded executable,\s*not line 2167' -Description "$Description first excluded executable"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?is)(?:bit-exact|exact bits).{0,180}CP322|CP322.{0,180}(?:bit-exact|exact bits)' -Description "$Description bit-exact CP322 provenance"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?is)(?:do\s+not\s+reconcile.{0,180}line-2163|line-2163.{0,100}not\s+reconciled).{0,180}(?:final\s+)?numerical DTO' -Description "$Description no final numerical DTO reconciliation"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?is)(?:DTO|numerical DTO).{0,180}line-2166-and-later|line-2166-and-later.{0,180}(?:DTO|numerical DTO)' -Description "$Description downstream final DTO distinction"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?is)(?:does\s+not|neither).{0,100}consum' -Description "$Description no numerical DTO input"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)(?:DTO|numerical)' -Description "$Description numerical DTO boundary"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?is)(?:does\s+not|neither|nor).{0,140}(?:feed|replace)' -Description "$Description no numerical DTO feed or replacement"
    Assert-Cp326ScopedText -Text $Text -Pattern '(?i)`EMS`\s+and\s+Autosizing\s+remain\s+forbidden' -Description "$Description forbidden EMS and Autosizing"
}

$cp326AlgorithmSpec = Get-Cp326TomlArrayEntry `
    -Path "specs\algorithm_ledger.toml" `
    -Table "algorithm" `
    -Id "ideal_loads_zone_equipment_purchased_air_source_order" `
    -ArrayName "support_boundary_addenda" `
    -Description "CP326 IdealLoads algorithm ledger"
Assert-Cp326BoundaryStatement -Text $cp326AlgorithmSpec.Entry -Description "CP326 algorithm ledger addendum"
Assert-Cp326ScopedText -Text $cp326AlgorithmSpec.Entry -Pattern 'both parents remain `scaffold`/`none`' -Description "CP326 parent status non-promotion"
Assert-Cp326ScopedText -Text $cp326AlgorithmSpec.Entry -Pattern '`routine\.calc_purch_air_loads` remains `source_mapped`' -Description "CP326 routine status non-promotion"
Assert-Cp326ScopedText -Text $cp326AlgorithmSpec.Entry -Pattern 'support and counts stay unchanged' -Description "CP326 support/count non-promotion"
Assert-Cp326ScopedText -Text $cp326AlgorithmSpec.Entry -Pattern 'readiness, capability, evidence, numerical conformance, and Roadmap state remain unchanged' -Description "CP326 readiness/evidence/conformance/Roadmap non-promotion"
Assert-Cp326ScopedText -Text $cp326AlgorithmSpec.Section -Pattern 'flow_limit_guard/body/release\.rs::advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body' -Description "CP326 algorithm wrapper target"
Assert-Cp326ScopedText -Text $cp326AlgorithmSpec.Section -Pattern 'flow_limit_guard/body\.rs::purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle_summary' -Description "CP326 algorithm lifecycle target"

$cp326CapabilityIds = @(
    "ideal_loads_no_oa_sensible",
    "ideal_loads_finite_limits"
)
foreach ($cp326CapabilityId in $cp326CapabilityIds) {
    $cp326CapabilitySpec = Get-Cp326TomlArrayEntry `
        -Path "specs\capabilities.toml" `
        -Table "capability" `
        -Id $cp326CapabilityId `
        -ArrayName "claim_boundary_addenda" `
        -Description "CP326 capability boundary"
    Assert-Cp326BoundaryStatement -Text $cp326CapabilitySpec.Entry -Description "CP326 '$cp326CapabilityId' addendum"
    Assert-Cp326ScopedText -Text $cp326CapabilitySpec.Entry -Pattern 'This changes no support level, run state, required or forbidden feature, evidence case, or numerical conformance' -Description "CP326 '$cp326CapabilityId' claim non-promotion"
    Assert-Cp326ScopedText -Text $cp326CapabilitySpec.Entry -Pattern 'finite-limit support remains unchanged' -Description "CP326 '$cp326CapabilityId' finite-limit non-promotion"
    $forbiddenFeatureMatch = [regex]::Match(
        $cp326CapabilitySpec.Section,
        '(?ms)^forbidden_active_features\s*=\s*\[(?<features>.*?)^\]\s*$'
    )
    if (-not $forbiddenFeatureMatch.Success) {
        throw "CP326 '$cp326CapabilityId' forbidden feature array missing"
    }
    Assert-Cp326ScopedText -Text $forbiddenFeatureMatch.Groups["features"].Value -Pattern '(?m)^\s*"EMS",?\s*$' -Description "CP326 '$cp326CapabilityId' EMS feature firewall"
    Assert-Cp326ScopedText -Text $forbiddenFeatureMatch.Groups["features"].Value -Pattern '(?m)^\s*"Autosizing",?\s*$' -Description "CP326 '$cp326CapabilityId' Autosizing feature firewall"
}

$cp326GeneratedAlgorithmRow = Get-Cp326GeneratedRow `
    -Path "docs\src\generated\algorithm-ledger.md" `
    -Id "ideal_loads_zone_equipment_purchased_air_source_order" `
    -Description "generated CP326 algorithm ledger"
Assert-Cp326BoundaryStatement -Text $cp326GeneratedAlgorithmRow -Description "generated CP326 algorithm row"
Assert-Cp326ScopedText -Text $cp326GeneratedAlgorithmRow -Pattern 'both parents remain `scaffold`/`none`' -Description "generated CP326 parent status non-promotion"
Assert-Cp326ScopedText -Text $cp326GeneratedAlgorithmRow -Pattern 'support and counts stay unchanged' -Description "generated CP326 support/count non-promotion"
Assert-Cp326ScopedText -Text $cp326GeneratedAlgorithmRow -Pattern 'readiness, capability, evidence, numerical conformance, and Roadmap state remain unchanged' -Description "generated CP326 readiness/evidence/conformance/Roadmap non-promotion"

foreach ($cp326CapabilityId in $cp326CapabilityIds) {
    $cp326GeneratedCapabilityRow = Get-Cp326GeneratedRow `
        -Path "docs\src\generated\capability-index.md" `
        -Id $cp326CapabilityId `
        -Description "generated CP326 capability index"
    Assert-Cp326BoundaryStatement -Text $cp326GeneratedCapabilityRow -Description "generated CP326 '$cp326CapabilityId' row"
    Assert-Cp326ScopedText -Text $cp326GeneratedCapabilityRow -Pattern 'This changes no support level, run state, required or forbidden feature, evidence case, or numerical conformance' -Description "generated CP326 '$cp326CapabilityId' non-promotion"
    Assert-Cp326ScopedText -Text $cp326GeneratedCapabilityRow -Pattern '`Autosizing`<br>`EMS`' -Description "generated CP326 '$cp326CapabilityId' feature firewall"
}

$cp326DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP326 maps only the Cooling supply mass-flow limit body.*?^conformance, and Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP326 Source-Ordered Cooling Supply Mass-Flow Limit Body\r?\n.*?Roadmap item\.\s*'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP326 Cooling Supply Mass-Flow Limit Body\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP326 Cooling Supply Mass-Flow Limit Body in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP326 Cooling Supply Mass-Flow Limit Body Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp326Documentation in $cp326DocumentationSections) {
    $cp326DocumentText = Read-RepoText -Path $cp326Documentation.Path
    $cp326SectionMatches = [regex]::Matches(
        $cp326DocumentText,
        $cp326Documentation.Pattern
    )
    if ($cp326SectionMatches.Count -ne 1) {
        throw "CP326 documentation expected one scoped section in $($cp326Documentation.Path), found $($cp326SectionMatches.Count)"
    }
    $cp326Section = $cp326SectionMatches[0].Value
    Assert-Cp326BoundaryStatement -Text $cp326Section -Description "CP326 documentation in $($cp326Documentation.Path)"
    Assert-Cp326ScopedText -Text $cp326Section -Pattern '(?is)(?:does\s+not|do\s+not)\s+claim\s+a\s+C\+\+\s+function(?:-|\s+)argument\s+evaluation\s+order' -Description "CP326 no argument-order claim in $($cp326Documentation.Path)"
    Assert-Cp326ScopedText -Text $cp326Section -Pattern '(?is)CP325.{0,100}(?:latest snapshot|snapshot).{0,80}(?:private witness|witness)' -Description "CP326 CP325 retained lineage in $($cp326Documentation.Path)"
    Assert-Cp326ScopedText -Text $cp326Section -Pattern '(?is)(?:retained|bit-validated).{0,140}CP322.{0,500}(?:retained\s+(?:Init|BeginEnvrn)\s+cache|Init\s+cache)' -Description "CP326 CP322/Init provenance in $($cp326Documentation.Path)"
    Assert-Cp326ScopedText -Text $cp326Section -Pattern 'purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle' -Description "CP326 JSON key in $($cp326Documentation.Path)"
    foreach ($cp326NonPromotionTerm in @(
            '\bsupport\b',
            '\bstatus(?:es)?\b',
            '\breadiness\b',
            '\bevidence\b',
            '\bconformance\b',
            '\bRoadmap\b'
        )) {
        Assert-Cp326ScopedText -Text $cp326Section -Pattern $cp326NonPromotionTerm -Description "CP326 documentation non-promotion term in $($cp326Documentation.Path)"
    }
}

# Keep the dot-source and script inventory order synchronized with the source
# checkpoint order.
$cp326MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp325DotSourceIndex = $cp326MainAuditText.IndexOf('ideal-loads-structure-audit\cp325-cooling-supply-mass-flow-limit-guard.ps1')
$cp326DotSourceIndex = $cp326MainAuditText.IndexOf('ideal-loads-structure-audit\cp326-cooling-supply-mass-flow-limit-body.ps1')
$cp327DotSourceIndexForCp326 = $cp326MainAuditText.IndexOf('ideal-loads-structure-audit\cp327-cooling-supply-mass-flow-very-small-guard.ps1')
$auditCompletionIndex = $cp326MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp325DotSourceIndex -lt 0 -or
    $cp326DotSourceIndex -le $cp325DotSourceIndex -or
    $cp327DotSourceIndexForCp326 -le $cp326DotSourceIndex -or
    $auditCompletionIndex -le $cp327DotSourceIndexForCp326
) {
    throw "Main IdealLoads audit must dot-source CP325 -> CP326 -> CP327 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp326-cooling-supply-mass-flow-limit-body\.ps1"' -Description "CP326 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp326-cooling-supply-mass-flow-limit-body\.ps1::dot_sources' -Description "CP326 main-audit callee evidence"
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'advance_heating_mode_guard_else_branch_entry' -Description 'CP433 helper whitelist'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'advance_heating_operating_mode_deadband_assignment' -Description 'audited CP340 through CP436 helper whitelist'
