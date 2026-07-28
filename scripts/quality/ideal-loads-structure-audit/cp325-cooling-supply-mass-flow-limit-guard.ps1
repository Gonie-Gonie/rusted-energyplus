# CP325 maps only the PurchasedAirManager.cc lines-2161-2162 cooling
# supply-mass-flow limit guard. Line 2163 is the first excluded executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp325ParentModule = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum.rs"
$cp325Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard.rs"
$cp325State = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\state.rs"
$cp325Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\transition.rs"
$cp325Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\release.rs"
$cp325PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\release\prefix_validation.rs"
$cp325RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\release\runtime_validation.rs"
$cp325SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\release\snapshot_validation.rs"
$cp325Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\tests\mod.rs"
$cp325ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\flow_limit_guard\tests\release_corruption.rs"
$cp325BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_supply_mass_flow_limit_guard_tests.rs"
$cp325CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_supply_mass_flow_limit_guard_validation.rs"
$cp325CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_supply_mass_flow_limit_guard_fixture.rs"
$cp325Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_guard.rs"
$cp325PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_guard\validation.rs"
$cp325PipelineSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_guard\validation\snapshot.rs"
$cp325PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_guard\serialization.rs"
$cp325PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_limit_guard\serialization\snapshot.rs"
$cp325DirectIntegrationAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_supply_mass_flow_limit_guard_assertions.rs"

foreach ($cp325RequiredFile in @(
        $cp325Module,
        $cp325State,
        $cp325Transition,
        $cp325Release,
        $cp325PrefixValidation,
        $cp325RuntimeValidation,
        $cp325SnapshotValidation,
        $cp325Tests,
        $cp325ReleaseCorruptionTests,
        $cp325BindingTests,
        $cp325CoupledValidation,
        $cp325CoupledFixture,
        $cp325Pipeline,
        $cp325PipelineValidation,
        $cp325PipelineSnapshotValidation,
        $cp325PipelineSerialization,
        $cp325PipelineSnapshotSerialization,
        $cp325DirectIntegrationAssertions
    )) {
    Assert-FileExists -Path $cp325RequiredFile -Description "CP325 cooling supply-mass-flow limit guard structure"
}

Assert-Contains -Path $cp325ParentModule -Pattern 'mod flow_limit_guard;' -Description "CP325 nested module declaration"
Assert-Contains -Path $cp325ParentModule -Pattern 'pub use flow_limit_guard::\*;' -Description "CP325 nested public re-export"
Assert-Contains -Path $cp325Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2161-2162' -Description "CP325 exact source boundary"
Assert-Contains -Path $cp325Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2163' -Description "CP325 first excluded executable"
Assert-ExactStringArray -Path $cp325Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER" -Expected @(
    "read-cooling-limit-for-flow-rate-comparison",
    "compare-cooling-limit-equal-to-flow-rate",
    "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity",
    "read-maximum-cooling-air-mass-flow-rate-after-limit-condition-true",
    "compare-maximum-cooling-air-mass-flow-rate-strictly-above-zero",
    "enter-supply-mass-flow-limit-body-if-compound-condition-satisfied"
) -Description "CP325 exact seven lexical source sites"

Assert-Contains -Path $cp325Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot' -Description "CP325 public snapshot"
Assert-Contains -Path $cp325State -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState' -Description "CP325 persistent state"
Assert-Contains -Path $cp325Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary' -Description "CP325 lifecycle summary"
Assert-Contains -Path $cp325Module -Pattern 'pub fn purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle_summary\s*\(' -Description "CP325 lifecycle accessor"
Assert-Contains -Path $cp325Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard\s*\(' -Description "CP325 exact direct wrapper"
Assert-Contains -Path $cp325Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_supply_mass_flow_limit_guard_state\s*\(' -Description "CP325 pure transition"

# The seven sites preserve the source's lazy `||` then `&&` evaluation. A
# positive selected maximum may enter the excluded body; CP325 stops there.
Assert-PatternsInOrder -Path $cp325Transition -Patterns @(
    'let cooling = predecessor\.cooling_body_entered;',
    'let first_limit = if cooling',
    'limit == IdealLoadsLimit::LimitFlowRate',
    'let second_limit = if is_flow_rate == Some\(false\)',
    'limit == IdealLoadsLimit::LimitFlowRateAndCapacity',
    'let limit_satisfied =',
    'if limit_satisfied == Some\(true\)',
    'maximum > 0\.0',
    'let body_entered = maximum_is_strictly_positive == Some\(true\);',
    'state\.first_cooling_limit_read_count \+= 1;',
    'state\.cooling_limit_flow_rate_comparison_count \+= 1;',
    'state\.second_cooling_limit_read_count \+= 1;',
    'state\.cooling_limit_flow_rate_and_capacity_comparison_count \+= 1;',
    'state\.maximum_cooling_air_mass_flow_rate_read_count \+= 1;',
    'state\.maximum_cooling_air_mass_flow_rate_positive_comparison_count \+= 1;',
    'state\.supply_mass_flow_limit_body_entry_count \+= 1;'
) -Description "CP325 lazy selectors, strict positive maximum, and body-entry order"
Assert-Contains -Path $cp325Transition -Pattern 'maximum_cooling_air_mass_flow_rate_kg_per_s\.map\(\|maximum\| maximum > 0\.0\)' -Description "CP325 strict maximum-flow positivity comparison"
Assert-Contains -Path $cp325Transition -Pattern 'if predecessor\.unit_off_skipped' -Description "CP325 UnitOff complete skip"
Assert-Contains -Path $cp325Transition -Pattern 'else if predecessor\.non_cooling_skipped' -Description "CP325 non-cooling complete skip"
Assert-Contains -Path $cp325Transition -Pattern 'FlowLimitBodyEntered' -Description "CP325 characterized true body-entry route"
Assert-Contains -Path $cp325Transition -Pattern 'MaximumCoolingMassFlowNotPositive' -Description "CP325 nonpositive maximum fallthrough route"

# Line 2163 and later must not leak into the guard. Cover the public release
# wrapper and every validator it can call, not only the pure transition:
# otherwise a supply-flow/load dependency or neutrally named helper could hide
# the excluded clamp behind an apparently guard-only API.
foreach ($cp325ScopeFile in @(
        $cp325Module,
        $cp325State,
        $cp325Transition,
        $cp325Release,
        $cp325PrefixValidation,
        $cp325RuntimeValidation,
        $cp325SnapshotValidation
    )) {
    Assert-NotContains -Path $cp325ScopeFile -Pattern 'SupplyMassFlowRate\s*=|VerySmallMassFlow|CalcPurchAirMixedAir|mixed[_-]?air' -Description "line-2163-or-later C++ behavior in CP325"
    Assert-NotContains -Path $cp325ScopeFile -Pattern 'read-supply-mass-flow-rate|reread-maximum-cooling-air-mass-flow.*minimum|apply-source-shaped.*minimum|assign(?:ed)?[_-](?:clamped[_-])?supply[_-]mass[_-]flow|resulting_supply_mass_flow' -Description "excluded supply-flow clamp site in CP325"
    Assert-NotContains -Path $cp325ScopeFile -Pattern '(?i)(?<![A-Za-z0-9_])(?:SupplyMassFlowRate|supply_mass_flow_rate(?:_kg_per_s)?|assigned_supply_mass_flow|resulting_supply_mass_flow|minimum_supply_mass_flow)(?![A-Za-z0-9_])' -Description "excluded supply-flow value dependency in CP325"
    Assert-NotContains -Path $cp325ScopeFile -Pattern '(?i)(?<![A-Za-z0-9_])(?:QZnCoolSP|QZnHeatSP|zone_(?:sensible_)?demand(?:_w)?|(?:remaining|requested)_(?:heating_|cooling_|sensible_|latent_)?(?:load|output)(?:_w)?|(?:sensible|latent|heating|cooling)_load(?:_w)?)(?![A-Za-z0-9_])' -Description "excluded load or demand dependency in CP325"
    Assert-NotContains -Path $cp325ScopeFile -Pattern 'DirectZonePurchasedAirCoupling(?:Input|Output)?|complete_direct_zone_purchased_air_coupling' -Description "unchanged numerical DTO dependency in CP325"
}
foreach ($cp325ForbiddenHelper in @(
        '(?<![A-Za-z0-9_])f64::min\s*\(',
        '\.min\s*\(',
        '\.(?:total_cmp|partial_cmp|clamp)\s*\(',
        '\.is_finite\(\)'
    )) {
    Assert-NotContains -Path $cp325Transition -Pattern $cp325ForbiddenHelper -Description "replacement clamp or normalized comparison in CP325 transition"
}

# Keep every executable call in the release boundary explicit. This allowlist
# includes validation, lookup, bit/finite admission, transition, and witness
# calls that are part of CP325 today. Any new helper name fails closed until
# the audit deliberately classifies it, so `apply_guard_result()` cannot hide
# line-2163 behavior behind a neutral name.
function Assert-Cp325AllowedRustCalls {
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

Assert-Cp325AllowedRustCalls -Path $cp325Release -Allowed @(
    "advance_cooling_supply_mass_flow_limit_guard_state",
    "advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard",
    "calc_state_identities_match",
    "call_order_error",
    "call_order_is_pending_guard",
    "classify_no_oa_sensible_subset",
    "completed_direct_cooling_supply_mass_flow_limit_guard_is_consistent",
    "completed_direct_cooling_supply_mass_flow_ems_override_body_is_consistent",
    "completed_guard_state_is_consistent",
    "cooling_supply_mass_flow_ems_override_body_latest_witness",
    "cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release",
    "cooling_supply_mass_flow_limit_guard_latest_witness",
    "cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release",
    "debug_assert!",
    "Err",
    "exact_direct_initialization_is_consistent",
    "flow_limit_guard_links_to_ems_override_body",
    "get",
    "get_mut",
    "is_finite",
    "is_none",
    "is_none_or",
    "is_supported",
    "Ok",
    "ok_or",
    "pending_guard_state_is_consistent",
    "PurchasedAirSizedLimits::from_system",
    "set_cooling_supply_mass_flow_limit_guard_latest_witness",
    "Some"
) -Description "CP325 public release boundary"
Assert-Cp325AllowedRustCalls -Path $cp325PrefixValidation -Allowed @(
    "flow_limit_guard_links_to_ems_override_body"
) -Description "CP325 prefix validator"
Assert-Cp325AllowedRustCalls -Path $cp325RuntimeValidation -Allowed @(
    "and_then",
    "calc_state_identities_match",
    "call_order_is_pending_guard",
    "checked_add",
    "completed_guard_history_links_to_body",
    "completed_guard_state_is_consistent",
    "cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release",
    "is_some_and",
    "latest_inputs_match",
    "latest_is_valid",
    "partition_is_consistent",
    "pending_guard_state_is_consistent",
    "snapshot_route",
    "Some",
    "source_counters_are_consistent",
    "to_bits",
    "usize::from"
) -Description "CP325 retained-runtime validator"
Assert-Cp325AllowedRustCalls -Path $cp325SnapshotValidation -Allowed @(
    "active_fields_are_exact",
    "cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release",
    "is_none",
    "is_some",
    "map",
    "skipped_fields_are_exact",
    "snapshot_is_exact_source_characterization",
    "snapshot_route",
    "Some",
    "usize::from"
) -Description "CP325 snapshot validator"

# Exact release consumes CP324's retained EMS-disabled complete skip and owns
# only retained model/Init inputs, never caller-supplied flow scalars.
Assert-Contains -Path $cp325Release -Pattern 'PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot' -Description "CP325 CP324 predecessor type"
Assert-Contains -Path $cp325Release -Pattern 'completed_direct_cooling_supply_mass_flow_ems_override_body_is_consistent\s*\(' -Description "CP325 completed CP324 prefix validation"
Assert-Contains -Path $cp325Release -Pattern 'cooling_supply_mass_flow_ems_override_body_latest_witness\s*\(' -Description "CP325 retained CP324 private witness"
Assert-Contains -Path $cp325Release -Pattern 'system\.cooling_limit' -Description "CP325 selected typed cooling-limit source"
Assert-Contains -Path $cp325Release -Pattern 'unit\.maximum_cooling_air_mass_flow_rate_kg_per_s' -Description "CP325 retained Init maximum-flow source"
Assert-Contains -Path $cp325Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp324: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,\s*\)' -Description "CP325 exact wrapper arguments without duplicate flow scalar"
Assert-Contains -Path $cp325RuntimeValidation -Pattern 'cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release' -Description "CP325 exact snapshot validation in retained state"
Assert-Contains -Path $cp325SnapshotValidation -Pattern 'snapshot\.predecessor_ems_supply_mass_flow_override_body_skipped' -Description "CP325 exact CP324 body skip"
Assert-Contains -Path $cp325SnapshotValidation -Pattern 'snapshot\.predecessor_ems_disabled_fallthrough' -Description "CP325 exact EMS-disabled predecessor"
Assert-NotContains -Path $cp325Release -Pattern 'supply_mass_flow_rate_kg_per_s\s*:' -Description "duplicate caller supply-flow scalar in CP325 release"
Assert-NotContains -Path $cp325Release -Pattern 'ems_actuator|ems_service|node_service|psychrometric|schedule_service|diagnostic_service' -Description "live service input in CP325 release"

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_supply_mass_flow_limit_guard_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot' -Description "runtime-root private CP325 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_supply_mass_flow_limit_guard_latest_witnesses:' -Description "public runtime-root CP325 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_supply_mass_flow_limit_guard_latest_witness\s*\(' -Description "runtime-root CP325 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_supply_mass_flow_limit_guard_latest_witness\s*\(' -Description "runtime-root CP325 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_supply_mass_flow_limit_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState' -Description "per-unit CP325 persistent state"

# Binding order is CP324 -> CP325 -> CP326 -> the pre-existing numerical DTO.
$cp325BindingText = Read-RepoText -Path $idealLoadsBinding
$cp324BindingIndexForCp325 = $cp325BindingText.IndexOf("let calculation_cooling_supply_mass_flow_ems_override_body =")
$cp325BindingIndex = $cp325BindingText.IndexOf("let calculation_cooling_supply_mass_flow_limit_guard =")
$cp326BindingIndexForCp325 = $cp325BindingText.IndexOf("let calculation_cooling_supply_mass_flow_limit_body =")
$numericalBindingIndexForCp325 = $cp325BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp324BindingIndexForCp325 -lt 0 -or
    $cp325BindingIndex -le $cp324BindingIndexForCp325 -or
    $cp326BindingIndexForCp325 -le $cp325BindingIndex -or
    $numericalBindingIndexForCp325 -le $cp326BindingIndexForCp325
) {
    throw "Binding must retain exact CP324 -> CP325 -> CP326 -> numerical Calc order"
}
Assert-Contains -Path $idealLoadsBinding -Pattern '(?s)let calculation_cooling_supply_mass_flow_limit_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_supply_mass_flow_ems_override_body,\s*\)' -Description "binding exact CP324-to-CP325 wrapper call without flow scalar"
$cp324BindingCallForCp325 = [regex]::Match(
    $cp325BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_ems_override_body =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body\(.*?CalculationCoolingSupplyMassFlowEmsOverrideBody,\s*\)\?;'
)
$cp325BindingCall = [regex]::Match(
    $cp325BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_limit_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard\(.*?CalculationCoolingSupplyMassFlowLimitGuard,\s*\)\?;'
)
$cp326BindingCallForCp325 = [regex]::Match(
    $cp325BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_limit_body =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body\(.*?CalculationCoolingSupplyMassFlowLimitBody,\s*\)\?;'
)
if (
    -not $cp324BindingCallForCp325.Success -or
    -not $cp325BindingCall.Success -or
    -not $cp326BindingCallForCp325.Success
) {
    throw "Binding must retain complete CP324, CP325, and CP326 exact release calls"
}
$cp324BindingCallEndForCp325 =
    $cp324BindingCallForCp325.Index + $cp324BindingCallForCp325.Length
$cp325BindingCallEnd = $cp325BindingCall.Index + $cp325BindingCall.Length
$cp326BindingCallEndForCp325 =
    $cp326BindingCallForCp325.Index + $cp326BindingCallForCp325.Length
if (
    $cp325BindingIndex -lt $cp324BindingCallEndForCp325 -or
    $cp326BindingIndexForCp325 -lt $cp325BindingCallEnd -or
    $numericalBindingIndexForCp325 -lt $cp326BindingCallEndForCp325
) {
    throw "CP324, CP325, and CP326 exact release calls must complete in source order before numerical Calc"
}
$postCp324BeforeCp325 = $cp325BindingText.Substring(
    $cp324BindingCallEndForCp325,
    $cp325BindingIndex - $cp324BindingCallEndForCp325
)
if ($postCp324BeforeCp325 -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP324 and before CP325"
}
$postCp325BeforeCp326 = $cp325BindingText.Substring(
    $cp325BindingCallEnd,
    $cp326BindingIndexForCp325 - $cp325BindingCallEnd
)
$postCp325BeforeCp326Code = [regex]::Replace(
    $postCp325BeforeCp326,
    '(?m)//.*$',
    ''
)
if ($postCp325BeforeCp326Code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
    throw "No intermediary helper call may execute after CP325 and before CP326"
}
$postCp326BeforeNumericalForCp325 = $cp325BindingText.Substring(
    $cp326BindingCallEndForCp325,
    $numericalBindingIndexForCp325 - $cp326BindingCallEndForCp325
)
if (
    $postCp326BeforeNumericalForCp325 -match 'VerySmallMassFlow|CalcPurchAirMixedAir' -or
    $postCp326BeforeNumericalForCp325 -match '(?i)(?:ems|psychrometric|diagnostic|node_service)\s*\('
) {
    throw "No line-2166-or-later or live service may execute after CP326 and before numerical Calc"
}

Assert-Contains -Path $idealLoadsBinding -Pattern 'CalculationCoolingSupplyMassFlowLimitGuard\(\s*PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError,?\s*\)' -Description "CP325 scheduled binding error boundary"
Assert-Contains -Path $idealLoadsBinding -Pattern 'pub calculation_cooling_supply_mass_flow_limit_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot' -Description "CP325 scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_supply_mass_flow_limit_guard_tests\.rs"\]' -Description "CP325 binding test module path"
Assert-Contains -Path $cp325BindingTests -Pattern 'scheduled_binding_preserves_both_selector_reads_and_strict_positive_guard' -Description "CP325 all-selector binding regression"
Assert-Contains -Path $cp325BindingTests -Pattern 'scheduled_binding_skips_all_cp325_sites_when_cooling_is_inactive' -Description "CP325 UnitOff/non-cooling binding regression"

Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_supply_mass_flow_limit_guard_validation;' -Description "coupled CP325 validator declaration"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_supply_mass_flow_limit_guard_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary' -Description "coupled CP325 lifecycle"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_supply_mass_flow_limit_guard_validation::snapshot_matches_release' -Description "coupled per-timestep CP325 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_supply_mass_flow_limit_guard_validation::validate_lifecycle' -Description "coupled final CP325 validation"
Assert-Contains -Path $cp325CoupledValidation -Pattern 'supply_mass_flow_limit_body_entry_count' -Description "coupled CP325 true body-entry reconciliation"
Assert-Contains -Path $cp325CoupledValidation -Pattern 'active_guard_false_fallthrough_count' -Description "coupled CP325 false fallthrough reconciliation"
Assert-NotContains -Path $cp325CoupledValidation -Pattern 'supply_mass_flow_rate_(?:read|assignment)|minimum_evaluation' -Description "excluded line-2163 coupled evidence"

Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_supply_mass_flow_limit_guard;' -Description "pipeline CP325 module declaration"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle' -Description "pipeline CP325 lifecycle JSON key"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_supply_mass_flow_limit_guard::validate_direct_lifecycle' -Description "pipeline CP325 direct firewall"
Assert-Contains -Path $cp325Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER' -Description "pipeline CP324-to-CP325 lineage"
Assert-Contains -Path $cp325Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER' -Description "pipeline CP325 source order"
Assert-Contains -Path $cp325PipelineValidation -Pattern 'supply_mass_flow_limit_body_entry_count' -Description "pipeline CP325 body-entry reconciliation"
Assert-Contains -Path $cp325PipelineValidation -Pattern 'active_guard_false_fallthrough_count' -Description "pipeline CP325 fallthrough reconciliation"
Assert-Contains -Path $cp325PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP325 latest serialization"
Assert-Contains -Path $cp325PipelineSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP325 source-order JSON"
Assert-Contains -Path $cp325PipelineSnapshotSerialization -Pattern '"supply_mass_flow_limit_body_entered"' -Description "pipeline CP325 body-entry JSON"
Assert-Contains -Path $cp325PipelineSnapshotSerialization -Pattern '"active_guard_false_fallthrough"' -Description "pipeline CP325 fallthrough JSON"
Assert-NotContains -Path $cp325PipelineSnapshotSerialization -Pattern '"(?:assigned|resulting)?_?supply_mass_flow_rate|minimum_supply_mass_flow' -Description "excluded line-2163 JSON evidence"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'mod cooling_supply_mass_flow_limit_guard_assertions;' -Description "direct integration CP325 assertion module"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'assert_cooling_supply_mass_flow_limit_guard\(' -Description "direct integration CP325 assertion calls"
Assert-Contains -Path $cp325DirectIntegrationAssertions -Pattern 'purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle' -Description "direct integration CP325 lifecycle key"
Assert-Contains -Path $cp325DirectIntegrationAssertions -Pattern 'supply_mass_flow_limit_body_entry_count' -Description "direct integration CP325 body-entry evidence"

function Assert-Cp325ScopedText {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ($Text -notmatch $Pattern) {
        throw "$Description missing from its CP325-scoped entry"
    }
}

function Get-Cp325TomlArrayEntry {
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
    $cp325Matches = [regex]::Matches(
        $arrayMatches[0].Groups["array"].Value,
        '(?m)^\s*"(?<entry>CP325 [^"\r\n]+)",\s*$'
    )
    if ($cp325Matches.Count -ne 1) {
        throw "$Description expected one CP325 addendum in '$Id', found $($cp325Matches.Count)"
    }
    return [PSCustomObject]@{
        Section = $section
        Entry = $cp325Matches[0].Groups["entry"].Value
    }
}

function Get-Cp325GeneratedRow {
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

function Assert-Cp325BoundaryStatement {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-Cp325ScopedText -Text $Text -Pattern '(?:executable\s+)?lines 2161-2162' -Description "$Description exact source lines"
    Assert-Cp325ScopedText -Text $Text -Pattern '(?i)(?:complete\s+)?seven(?:-site|\s+lexical\s+(?:source\s+)?sites)' -Description "$Description exact seven sites"
    Assert-Cp325ScopedText -Text $Text -Pattern '(?is)(?:selected\s+positive|positive(?:\s+(?:selected|initialized))?)\s+maximum.{0,180}?(?:may\s+(?:truthfully\s+)?(?:record|enter)).{0,60}?(?:body entry|excluded body)' -Description "$Description allowed true body entry"
    Assert-Cp325ScopedText -Text $Text -Pattern '(?i)performs no supply-flow read,\s*minimum,\s*(?:or\s+)?assignment' -Description "$Description guard-only behavior"
    Assert-Cp325ScopedText -Text $Text -Pattern '(?i)Line 2163 is the first excluded executable' -Description "$Description first excluded executable"
    Assert-Cp325ScopedText -Text $Text -Pattern '(?i)`EMS`\s+and\s+Autosizing\s+remain forbidden' -Description "$Description forbidden EMS and Autosizing"
}

$cp325AlgorithmSpec = Get-Cp325TomlArrayEntry `
    -Path "specs\algorithm_ledger.toml" `
    -Table "algorithm" `
    -Id "ideal_loads_zone_equipment_purchased_air_source_order" `
    -ArrayName "support_boundary_addenda" `
    -Description "CP325 IdealLoads algorithm ledger"
Assert-Cp325BoundaryStatement -Text $cp325AlgorithmSpec.Entry -Description "CP325 algorithm ledger addendum"
Assert-Cp325ScopedText -Text $cp325AlgorithmSpec.Entry -Pattern 'both parents remain `scaffold`/`none`' -Description "CP325 parent status non-promotion"
Assert-Cp325ScopedText -Text $cp325AlgorithmSpec.Entry -Pattern '`routine\.calc_purch_air_loads` remains `source_mapped`' -Description "CP325 routine status non-promotion"
Assert-Cp325ScopedText -Text $cp325AlgorithmSpec.Entry -Pattern 'support and counts stay unchanged' -Description "CP325 support/count non-promotion"
Assert-Cp325ScopedText -Text $cp325AlgorithmSpec.Entry -Pattern 'readiness, capability, evidence, numerical conformance, and Roadmap state remain unchanged' -Description "CP325 readiness/evidence/conformance/Roadmap non-promotion"
Assert-Cp325ScopedText -Text $cp325AlgorithmSpec.Section -Pattern 'flow_limit_guard\.rs::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState' -Description "CP325 algorithm state target"

$cp325CapabilityIds = @(
    "ideal_loads_no_oa_sensible",
    "ideal_loads_finite_limits"
)
foreach ($cp325CapabilityId in $cp325CapabilityIds) {
    $cp325CapabilitySpec = Get-Cp325TomlArrayEntry `
        -Path "specs\capabilities.toml" `
        -Table "capability" `
        -Id $cp325CapabilityId `
        -ArrayName "claim_boundary_addenda" `
        -Description "CP325 capability boundary"
    Assert-Cp325BoundaryStatement -Text $cp325CapabilitySpec.Entry -Description "CP325 '$cp325CapabilityId' addendum"
    Assert-Cp325ScopedText -Text $cp325CapabilitySpec.Entry -Pattern 'This changes no support level, run state, required or forbidden feature, evidence case, or numerical conformance' -Description "CP325 '$cp325CapabilityId' claim non-promotion"
    Assert-Cp325ScopedText -Text $cp325CapabilitySpec.Entry -Pattern 'finite-limit support remains unchanged' -Description "CP325 '$cp325CapabilityId' finite-limit non-promotion"
    $forbiddenFeatureMatch = [regex]::Match(
        $cp325CapabilitySpec.Section,
        '(?ms)^forbidden_active_features\s*=\s*\[(?<features>.*?)^\]\s*$'
    )
    if (-not $forbiddenFeatureMatch.Success) {
        throw "CP325 '$cp325CapabilityId' forbidden feature array missing"
    }
    Assert-Cp325ScopedText -Text $forbiddenFeatureMatch.Groups["features"].Value -Pattern '(?m)^\s*"EMS",?\s*$' -Description "CP325 '$cp325CapabilityId' EMS feature firewall"
    Assert-Cp325ScopedText -Text $forbiddenFeatureMatch.Groups["features"].Value -Pattern '(?m)^\s*"Autosizing",?\s*$' -Description "CP325 '$cp325CapabilityId' Autosizing feature firewall"
}

$cp325GeneratedAlgorithmRow = Get-Cp325GeneratedRow `
    -Path "docs\src\generated\algorithm-ledger.md" `
    -Id "ideal_loads_zone_equipment_purchased_air_source_order" `
    -Description "generated CP325 algorithm ledger"
Assert-Cp325BoundaryStatement -Text $cp325GeneratedAlgorithmRow -Description "generated CP325 algorithm row"
Assert-Cp325ScopedText -Text $cp325GeneratedAlgorithmRow -Pattern 'both parents remain `scaffold`/`none`' -Description "generated CP325 parent status non-promotion"
Assert-Cp325ScopedText -Text $cp325GeneratedAlgorithmRow -Pattern 'support and counts stay unchanged' -Description "generated CP325 support/count non-promotion"
Assert-Cp325ScopedText -Text $cp325GeneratedAlgorithmRow -Pattern 'readiness, capability, evidence, numerical conformance, and Roadmap state remain unchanged' -Description "generated CP325 readiness/evidence/conformance/Roadmap non-promotion"

foreach ($cp325CapabilityId in $cp325CapabilityIds) {
    $cp325GeneratedCapabilityRow = Get-Cp325GeneratedRow `
        -Path "docs\src\generated\capability-index.md" `
        -Id $cp325CapabilityId `
        -Description "generated CP325 capability index"
    Assert-Cp325BoundaryStatement -Text $cp325GeneratedCapabilityRow -Description "generated CP325 '$cp325CapabilityId' row"
    Assert-Cp325ScopedText -Text $cp325GeneratedCapabilityRow -Pattern 'This changes no support level, run state, required or forbidden feature, evidence case, or numerical conformance' -Description "generated CP325 '$cp325CapabilityId' non-promotion"
    Assert-Cp325ScopedText -Text $cp325GeneratedCapabilityRow -Pattern '`Autosizing`<br>`EMS`' -Description "generated CP325 '$cp325CapabilityId' feature firewall"
}

$cp325DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP325 maps only the complete Cooling supply mass-flow limit guard.*?^state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP325 Source-Ordered Cooling Supply Mass-Flow Limit Guard.*?Roadmap item\.\s*'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP325 Cooling Supply Mass-Flow Limit Guard\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP325 Cooling Supply Mass-Flow Limit Guard in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP325 Cooling Supply Mass-Flow Limit Guard Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp325Documentation in $cp325DocumentationSections) {
    $cp325DocumentText = Read-RepoText -Path $cp325Documentation.Path
    $cp325SectionMatches = [regex]::Matches(
        $cp325DocumentText,
        $cp325Documentation.Pattern
    )
    if ($cp325SectionMatches.Count -ne 1) {
        throw "CP325 documentation expected one scoped section in $($cp325Documentation.Path), found $($cp325SectionMatches.Count)"
    }
    $cp325Section = $cp325SectionMatches[0].Value
    Assert-Cp325ScopedText -Text $cp325Section -Pattern '(?:executable\s+)?lines 2161-2162' -Description "CP325 documentation exact source lines in $($cp325Documentation.Path)"
    Assert-Cp325ScopedText -Text $cp325Section -Pattern '(?i)(?:exact\s+)?seven lexical(?: source)? sites' -Description "CP325 documentation exact seven sites in $($cp325Documentation.Path)"
    Assert-Cp325ScopedText -Text $cp325Section -Pattern '(?is)(?:(?:selected\s+positive|positive(?:\s+(?:selected|initialized))?)\s+maximum.{0,220}?(?:record|enter).{0,80}?(?:body entry|excluded body)|true body entry where applicable|enter the supply mass-flow limit body only when)' -Description "CP325 documentation allowed true body entry in $($cp325Documentation.Path)"
    Assert-Cp325ScopedText -Text $cp325Section -Pattern '(?is)no supply-flow read|neither reads nor\s+mutates supply flow|without reading\s+or assigning supply flow|performs no supply-flow read' -Description "CP325 guard-only documentation in $($cp325Documentation.Path)"
    Assert-Cp325ScopedText -Text $cp325Section -Pattern '(?i)Line 2163 is the first excluded executable' -Description "CP325 first excluded executable documentation in $($cp325Documentation.Path)"
    Assert-Cp325ScopedText -Text $cp325Section -Pattern '(?i)`EMS`\s+and\s+Autosizing\s+remain forbidden' -Description "CP325 forbidden-feature documentation in $($cp325Documentation.Path)"
    Assert-Cp325ScopedText -Text $cp325Section -Pattern '(?is)(?:CP325\s+(?:promotes|adds|changes)\s+(?:no|target inventory\s+and\s+lifecycle evidence\s+only)|\bsupport\b.{0,300}\bRoadmap\b.{0,60}\bremain unchanged)' -Description "CP325 documentation promotion denial in $($cp325Documentation.Path)"
    foreach ($cp325NonPromotionTerm in @(
            '\bsupport\b',
            '\bstatus(?:es)?\b',
            '\breadiness\b',
            '\bevidence\b',
            '\bconformance\b',
            '\bRoadmap\b'
        )) {
        Assert-Cp325ScopedText -Text $cp325Section -Pattern $cp325NonPromotionTerm -Description "CP325 documentation non-promotion term in $($cp325Documentation.Path)"
    }
}
