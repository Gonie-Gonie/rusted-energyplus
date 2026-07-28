# CP324 maps only the PurchasedAirManager.cc lines-2158-2159 EMS
# supply-mass-flow override body. Line 2161 is the first excluded executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp324ParentModule = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard.rs"
$cp324Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body.rs"
$cp324State = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body\state.rs"
$cp324Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body\transition.rs"
$cp324Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body\release.rs"
$cp324PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body\release\prefix_validation.rs"
$cp324RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body\release\runtime_validation.rs"
$cp324SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body\release\snapshot_validation.rs"
$cp324Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body\tests\mod.rs"
$cp324ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\body\tests\release_corruption.rs"
$cp324BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_supply_mass_flow_ems_override_body_tests.rs"
$cp324CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_supply_mass_flow_ems_override_body_validation.rs"
$cp324CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_supply_mass_flow_ems_override_body_fixture.rs"
$cp324Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_body.rs"
$cp324PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_body\validation.rs"
$cp324PipelineSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_body\validation\snapshot.rs"
$cp324PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_body\serialization.rs"
$cp324PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_body\serialization\snapshot.rs"
$cp324DirectIntegrationAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_supply_mass_flow_ems_override_body_assertions.rs"
$cp324ModelIdealLoads = "crates\ep_model\src\objects\ideal_loads.rs"

foreach ($cp324RequiredFile in @(
        $cp324Module,
        $cp324State,
        $cp324Transition,
        $cp324Release,
        $cp324PrefixValidation,
        $cp324RuntimeValidation,
        $cp324SnapshotValidation,
        $cp324Tests,
        $cp324ReleaseCorruptionTests,
        $cp324BindingTests,
        $cp324CoupledValidation,
        $cp324CoupledFixture,
        $cp324Pipeline,
        $cp324PipelineValidation,
        $cp324PipelineSnapshotValidation,
        $cp324PipelineSerialization,
        $cp324PipelineSnapshotSerialization,
        $cp324DirectIntegrationAssertions
    )) {
    Assert-FileExists -Path $cp324RequiredFile -Description "CP324 EMS override body structure"
}

Assert-Contains -Path $cp324ParentModule -Pattern 'mod body;' -Description "CP324 nested module declaration"
Assert-Contains -Path $cp324ParentModule -Pattern 'pub use body::\*;' -Description "CP324 nested public re-export"
Assert-Contains -Path $cp324Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2158-2159' -Description "CP324 exact source boundary"
Assert-Contains -Path $cp324Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2161' -Description "CP324 first excluded executable"
Assert-ExactStringArray -Path $cp324Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER" -Expected @(
    "read-ems-supply-mass-flow-override-value",
    "assign-supply-mass-flow-rate-from-ems-override",
    "read-outdoor-air-mass-flow-rate-for-minimum",
    "read-supply-mass-flow-rate-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-outdoor-air-mass-flow-rate"
) -Description "CP324 exact six source sites"

Assert-Contains -Path $cp324Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot' -Description "CP324 public snapshot"
Assert-Contains -Path $cp324State -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState' -Description "CP324 persistent state"
Assert-Contains -Path $cp324Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary' -Description "CP324 lifecycle summary"
Assert-Contains -Path $cp324Module -Pattern 'pub fn purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle_summary\s*\(' -Description "CP324 lifecycle accessor"
Assert-Contains -Path $cp324Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body\s*\(' -Description "CP324 exact direct wrapper"
Assert-Contains -Path $cp324Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_supply_mass_flow_ems_override_body_state\s*\(' -Description "CP324 pure transition"

# Private true-body characterization owns the six sites. The source two-
# argument Objexx minimum is `a < b ? a : b`, so the right operand wins on a
# tie or unordered comparison. Public direct release must complete-skip it.
Assert-PatternsInOrder -Path $cp324Transition -Patterns @(
    'let cooling = predecessor\.cooling_body_entered;',
    'let body_entered = predecessor\.ems_supply_mass_flow_override_body_entered;',
    'if predecessor\.unit_off_skipped',
    'else if predecessor\.non_cooling_skipped',
    'if body_entered',
    'body_entry_count \+= 1',
    'ems_supply_mass_flow_override_value_read_count \+= 1',
    'supply_mass_flow_rate_override_assignment_count \+= 1',
    'outdoor_air_mass_flow_rate_for_minimum_read_count \+= 1',
    'supply_mass_flow_rate_for_minimum_read_count \+= 1',
    'source_shaped_two_argument_minimum_evaluation_count \+= 1',
    'outdoor_air_mass_flow_rate_assignment_count \+= 1',
    'ems_disabled_fallthrough_count \+= 1'
) -Description "CP324 predecessor routing and conditional six-site execution"
Assert-Contains -Path $cp324Transition -Pattern 'if left < right \{ left \} else \{ right \}' -Description "CP324 strict-less-than source minimum with explicit right fallback"
foreach ($cp324ForbiddenMinimum in @(
        '(?<![A-Za-z0-9_])f64::min\s*\(',
        '\.min\s*\(',
        '\.(?:total_cmp|partial_cmp|clamp)\s*\('
    )) {
    Assert-NotContains -Path $cp324Transition -Pattern $cp324ForbiddenMinimum -Description "replacement or normalized minimum in CP324 transition"
}

# Exact release consumes CP323's retained false route. It owns no public EMS
# value, live actuator/model field, or substitute flow service.
Assert-Contains -Path $cp324Release -Pattern 'PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot' -Description "CP324 CP323 predecessor type"
Assert-Contains -Path $cp324Release -Pattern 'completed_direct_prefix_through_ems_override_guard_is_consistent\s*\(' -Description "CP324 completed CP323 prefix validation"
Assert-Contains -Path $cp324RuntimeValidation -Pattern 'pub\(super\) fn completed_guard_state_is_consistent\s*\(' -Description "CP324 completed CP323 state validation"
Assert-Contains -Path $cp324RuntimeValidation -Pattern '(?s)state\.latest == Some\(predecessor\).*witness == Some\(predecessor\).*cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release\(predecessor\)' -Description "CP324 exact retained CP323 lineage"
Assert-Contains -Path $cp324SnapshotValidation -Pattern 'snapshot\.body_skipped' -Description "CP324 direct body complete skip"
Assert-Contains -Path $cp324SnapshotValidation -Pattern 'snapshot\.ems_disabled_fallthrough' -Description "CP324 direct false-guard fallthrough"
foreach ($cp324DirectZeroSite in @(
        'ems_supply_mass_flow_override_value_read',
        'supply_mass_flow_rate_override_assignment_performed',
        'outdoor_air_mass_flow_rate_for_minimum_read',
        'supply_mass_flow_rate_for_minimum_read',
        'source_shaped_two_argument_minimum_evaluated',
        'outdoor_air_mass_flow_rate_assignment_performed'
    )) {
    Assert-Contains -Path $cp324SnapshotValidation -Pattern "!snapshot\.$cp324DirectZeroSite" -Description "CP324 direct zero site $cp324DirectZeroSite"
}
Assert-NotContains -Path $cp324Release -Pattern 'EMSValueMassFlowRate|ems_value_mass_flow_rate|ems_actuator|ems_service|outdoor_air_mass_flow_rate_kg_per_s|supply_mass_flow_rate_kg_per_s' -Description "public EMS value, actuator, or substitute flow input in CP324 release"
Assert-NotContains -Path $cp324ModelIdealLoads -Pattern 'EMSOverrideMdotOn|EMSValueMassFlowRate|ems_supply_mass_flow_override' -Description "fabricated IdealLoads EMS actuator input field"

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_supply_mass_flow_ems_override_body_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot' -Description "runtime-root private CP324 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_supply_mass_flow_ems_override_body_latest_witnesses:' -Description "public runtime-root CP324 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_supply_mass_flow_ems_override_body_latest_witness\s*\(' -Description "runtime-root CP324 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_supply_mass_flow_ems_override_body_latest_witness\s*\(' -Description "runtime-root CP324 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_supply_mass_flow_ems_override_body:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState' -Description "per-unit CP324 persistent state"

# Nothing at line 2161 or later belongs in the bounded CP324 core.
foreach ($cp324ScopeFile in @(
        $cp324Module,
        $cp324State,
        $cp324Transition,
        $cp324SnapshotValidation
    )) {
    Assert-NotContains -Path $cp324ScopeFile -Pattern 'MaxCoolMassFlowRate|max_cool_mass_flow|maximum_cooling_(?:air_)?mass_flow|VerySmallMassFlow|CalcPurchAirMixedAir|mixed[_-]?air' -Description "line-2161-or-later flow behavior in CP324"
    Assert-NotContains -Path $cp324ScopeFile -Pattern 'cooling_limit_(?:flow_rate|flow_rate_and_capacity)|LimitFlowRateAndCapacity' -Description "line-2161 cooling selector in CP324"
}

# Binding order is CP323 -> CP324 -> CP325 -> numerical DTO.
$cp324BindingText = Read-RepoText -Path $idealLoadsBinding
$cp323BindingIndexForCp324 = $cp324BindingText.IndexOf("let calculation_cooling_supply_mass_flow_ems_override_guard =")
$cp324BindingIndex = $cp324BindingText.IndexOf("let calculation_cooling_supply_mass_flow_ems_override_body =")
$cp325BindingIndexForCp324 = $cp324BindingText.IndexOf("let calculation_cooling_supply_mass_flow_limit_guard =")
$numericalBindingIndexForCp324 = $cp324BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp323BindingIndexForCp324 -lt 0 -or
    $cp324BindingIndex -le $cp323BindingIndexForCp324 -or
    $cp325BindingIndexForCp324 -le $cp324BindingIndex -or
    $numericalBindingIndexForCp324 -le $cp325BindingIndexForCp324
) {
    throw "Binding must retain exact CP323 -> CP324 -> CP325 -> numerical Calc order"
}
Assert-Contains -Path $idealLoadsBinding -Pattern '(?s)let calculation_cooling_supply_mass_flow_ems_override_body =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_supply_mass_flow_ems_override_guard,\s*\)' -Description "binding exact CP323-to-CP324 wrapper call without EMS or flow scalar"
$cp323BindingCallForCp324 = [regex]::Match(
    $cp324BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_ems_override_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard\(.*?CalculationCoolingSupplyMassFlowEmsOverrideGuard,\s*\)\?;'
)
$cp324BindingCall = [regex]::Match(
    $cp324BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_ems_override_body =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body\(.*?CalculationCoolingSupplyMassFlowEmsOverrideBody,\s*\)\?;'
)
$cp325BindingCallForCp324 = [regex]::Match(
    $cp324BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_limit_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard\(.*?CalculationCoolingSupplyMassFlowLimitGuard,\s*\)\?;'
)
if (
    -not $cp323BindingCallForCp324.Success -or
    -not $cp324BindingCall.Success -or
    -not $cp325BindingCallForCp324.Success
) {
    throw "Binding must retain complete CP323, CP324, and CP325 exact release calls"
}
$cp323BindingCallEndForCp324 =
    $cp323BindingCallForCp324.Index + $cp323BindingCallForCp324.Length
$cp324BindingCallEnd = $cp324BindingCall.Index + $cp324BindingCall.Length
$cp325BindingCallEndForCp324 =
    $cp325BindingCallForCp324.Index + $cp325BindingCallForCp324.Length
if (
    $cp324BindingIndex -lt $cp323BindingCallEndForCp324 -or
    $cp325BindingIndexForCp324 -lt $cp324BindingCallEnd -or
    $numericalBindingIndexForCp324 -lt $cp325BindingCallEndForCp324
) {
    throw "CP323, CP324, and CP325 exact release calls must complete in source order before numerical Calc"
}
$postCp323BeforeCp324 = $cp324BindingText.Substring(
    $cp323BindingCallEndForCp324,
    $cp324BindingIndex - $cp323BindingCallEndForCp324
)
if ($postCp323BeforeCp324 -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP323 and before CP324"
}
$postCp324BeforeCp325 = $cp324BindingText.Substring(
    $cp324BindingCallEnd,
    $cp325BindingIndexForCp324 - $cp324BindingCallEnd
)
if ($postCp324BeforeCp325 -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP324 and before CP325"
}
$postCp325BeforeNumericalForCp324 = $cp324BindingText.Substring(
    $cp325BindingCallEndForCp324,
    $numericalBindingIndexForCp324 - $cp325BindingCallEndForCp324
)
if ($postCp325BeforeNumericalForCp324 -match 'VerySmallMassFlow|CalcPurchAirMixedAir|SupplyMassFlowRate\s*=|(?i)(?:ems|psychrometric|diagnostic|node_service)\s*\(') {
    throw "No line-2163-or-later or live EMS behavior may execute after CP325 and before numerical Calc"
}

Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_supply_mass_flow_ems_override_body_validation;' -Description "coupled CP324 validator declaration"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_supply_mass_flow_ems_override_body_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary' -Description "coupled CP324 lifecycle"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_supply_mass_flow_ems_override_body_validation::snapshot_matches_release' -Description "coupled per-timestep CP324 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_supply_mass_flow_ems_override_body_validation::validate_lifecycle' -Description "coupled final CP324 validation"
Assert-Contains -Path $cp324CoupledValidation -Pattern 'body_entry_count",\s*0' -Description "coupled zero CP324 body execution"
foreach ($cp324DirectCounter in @(
        'ems_supply_mass_flow_override_value_read_count',
        'supply_mass_flow_rate_override_assignment_count',
        'outdoor_air_mass_flow_rate_for_minimum_read_count',
        'supply_mass_flow_rate_for_minimum_read_count',
        'source_shaped_two_argument_minimum_evaluation_count',
        'outdoor_air_mass_flow_rate_assignment_count'
    )) {
    $cp324DirectCounterPattern = [regex]::Escape($cp324DirectCounter) + '",\s*0'
    Assert-Contains -Path $cp324CoupledValidation -Pattern $cp324DirectCounterPattern -Description "coupled zero CP324 counter $cp324DirectCounter"
}

Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_supply_mass_flow_ems_override_body;' -Description "pipeline CP324 module declaration"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle' -Description "pipeline CP324 lifecycle JSON key"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_supply_mass_flow_ems_override_body::validate_direct_lifecycle' -Description "pipeline CP324 direct firewall"
Assert-Contains -Path $cp324Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER' -Description "pipeline CP323-to-CP324 lineage"
Assert-Contains -Path $cp324Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER' -Description "pipeline CP324 source order"
Assert-Contains -Path $cp324PipelineValidation -Pattern 'body_entry_count",\s*0' -Description "pipeline zero CP324 body execution"
Assert-Contains -Path $cp324PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP324 latest serialization"
Assert-Contains -Path $cp324PipelineSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP324 source-order JSON"
Assert-Contains -Path $cp324PipelineSnapshotSerialization -Pattern '"body_skipped"' -Description "pipeline CP324 skip-route JSON"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'mod cooling_supply_mass_flow_ems_override_body_assertions;' -Description "direct integration CP324 assertion module"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'assert_cooling_supply_mass_flow_ems_override_body\(' -Description "direct integration CP324 assertion calls"
Assert-Contains -Path $cp324DirectIntegrationAssertions -Pattern 'purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle' -Description "direct integration CP324 lifecycle key"
Assert-Contains -Path $cp324DirectIntegrationAssertions -Pattern 'body_entry_count' -Description "direct integration CP324 zero body execution"

Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern '"CP324 supersedes only CP323' -Description "CP324 algorithm support boundary"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'body\.rs::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState' -Description "CP324 algorithm state target"
Assert-Contains -Path "specs\capabilities.toml" -Pattern '"CP324 additionally requires' -Description "CP324 capability boundary"
Assert-Contains -Path "specs\capabilities.toml" -Pattern '(?s)forbidden_active_features\s*=\s*\[.*?"EMS"' -Description "EMS remains forbidden"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP324 supersedes only CP323' -Description "generated CP324 algorithm boundary"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP324 additionally requires' -Description "generated CP324 capability boundary"
foreach ($cp324Doc in @(
        "docs\src\current\current-status.md",
        "docs\src\current\project-contract.md",
        "docs\src\porting-map\ideal-loads-source-map.md",
        "docs\src\porting-map\heat-balance-source-map.md",
        "docs\src\porting-map\zone-air-update-map.md"
    )) {
    Assert-Contains -Path $cp324Doc -Pattern 'CP324' -Description "CP324 documentation boundary"
    Assert-Contains -Path $cp324Doc -Pattern '2161' -Description "CP324 first excluded executable documentation"
    Assert-Contains -Path $cp324Doc -Pattern 'EMS.*forbidden|`EMS` remains forbidden' -Description "CP324 EMS-forbidden documentation"
}
