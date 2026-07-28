# CP323 maps only the PurchasedAirManager.cc line-2157 EMS supply-mass-flow
# override guard. Line 2158 is the first excluded executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp323Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard.rs"
$cp323State = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\state.rs"
$cp323Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\transition.rs"
$cp323Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\release.rs"
$cp323PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\release\prefix_validation.rs"
$cp323RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\release\runtime_validation.rs"
$cp323SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\release\snapshot_validation.rs"
$cp323Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\tests\mod.rs"
$cp323ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\ems_override_guard\tests\release_corruption.rs"
$cp323BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_supply_mass_flow_ems_override_guard_tests.rs"
$cp323CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_supply_mass_flow_ems_override_guard_validation.rs"
$cp323CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_supply_mass_flow_ems_override_guard_fixture.rs"
$cp323Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_guard.rs"
$cp323PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_guard\validation.rs"
$cp323PipelineSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_guard\validation\snapshot.rs"
$cp323PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_guard\serialization.rs"
$cp323PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_ems_override_guard\serialization\snapshot.rs"
$cp323DirectIntegrationAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_supply_mass_flow_ems_override_guard_assertions.rs"
$cp323ModelIdealLoads = "crates\ep_model\src\objects\ideal_loads.rs"

foreach ($cp323RequiredFile in @(
        $cp323Module,
        $cp323State,
        $cp323Transition,
        $cp323Release,
        $cp323PrefixValidation,
        $cp323RuntimeValidation,
        $cp323SnapshotValidation,
        $cp323Tests,
        $cp323ReleaseCorruptionTests,
        $cp323BindingTests,
        $cp323CoupledValidation,
        $cp323CoupledFixture,
        $cp323Pipeline,
        $cp323PipelineValidation,
        $cp323PipelineSnapshotValidation,
        $cp323PipelineSerialization,
        $cp323PipelineSnapshotSerialization,
        $cp323DirectIntegrationAssertions
    )) {
    Assert-FileExists -Path $cp323RequiredFile -Description "CP323 EMS override guard structure"
}

Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum.rs" -Pattern 'mod ems_override_guard;' -Description "CP323 nested module declaration"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum.rs" -Pattern 'pub use ems_override_guard::\*;' -Description "CP323 nested public re-export"
Assert-Contains -Path $cp323Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2157' -Description "CP323 exact source boundary"
Assert-Contains -Path $cp323Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2158' -Description "CP323 first excluded executable"
Assert-ExactStringArray -Path $cp323Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER" -Expected @(
    "read-ems-supply-mass-flow-override-flag",
    "evaluate-ems-supply-mass-flow-override-guard",
    "enter-ems-supply-mass-flow-override-body-if-enabled"
) -Description "CP323 exact three source sites"

Assert-Contains -Path $cp323Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot' -Description "CP323 public snapshot"
Assert-Contains -Path $cp323State -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState' -Description "CP323 persistent state"
Assert-Contains -Path $cp323Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary' -Description "CP323 lifecycle summary"
Assert-Contains -Path $cp323Module -Pattern 'pub fn purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle_summary\s*\(' -Description "CP323 lifecycle accessor"
Assert-Contains -Path $cp323Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard\s*\(' -Description "CP323 exact direct wrapper"
Assert-Contains -Path $cp323Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_supply_mass_flow_ems_override_guard_state\s*\(' -Description "CP323 pure transition"

# The three sites execute only for the CP322 Cooling route. Public release is
# EMS-disabled and may not execute any source statement from line 2158 onward.
Assert-PatternsInOrder -Path $cp323Transition -Patterns @(
    'let cooling = predecessor\.cooling_body_entered;',
    'let enabled = cooling\.then_some\(ems_supply_mass_flow_override_enabled\);',
    'let body_entered = enabled == Some\(true\);',
    'if predecessor\.unit_off_skipped',
    'else if predecessor\.non_cooling_skipped',
    'ems_supply_mass_flow_override_flag_read_count \+= 1',
    'ems_supply_mass_flow_override_guard_evaluation_count \+= 1'
) -Description "CP323 predecessor routing and conditional source sites"
Assert-Contains -Path $cp323Release -Pattern '(?s)advance_cooling_supply_mass_flow_ems_override_guard_state\(\s*&mut unit\.calc_cooling_supply_mass_flow_ems_override_guard,\s*predecessor_cp322,\s*false,\s*\)' -Description "CP323 exact direct false guard"
Assert-Contains -Path $cp323SnapshotValidation -Pattern 'snapshot\.ems_supply_mass_flow_override_enabled != Some\(true\)' -Description "CP323 true route rejected by direct snapshot validation"
Assert-Contains -Path $cp323SnapshotValidation -Pattern '!snapshot\.ems_supply_mass_flow_override_body_entered' -Description "CP323 zero direct body entry"
Assert-Contains -Path $cp323SnapshotValidation -Pattern 'snapshot\.ems_supply_mass_flow_override_guard_false_fallthrough' -Description "CP323 direct false fallthrough"

foreach ($cp323ScopeFile in @(
        $cp323Module,
        $cp323State,
        $cp323Transition,
        $cp323Release,
        $cp323PrefixValidation,
        $cp323RuntimeValidation,
        $cp323SnapshotValidation
    )) {
    Assert-NotContains -Path $cp323ScopeFile -Pattern 'EMSValueMassFlowRate|ems_value_mass_flow_rate|VerySmallMassFlow|CalcPurchAirMixedAir|mixed[_-]?air' -Description "line-2158-or-later value/service scope creep in CP323"
    Assert-NotContains -Path $cp323ScopeFile -Pattern '(?<![A-Za-z0-9_])(?:min|max|clamp)\s*\(' -Description "line-2158-or-later flow operation in CP323"
    Assert-NotContains -Path $cp323ScopeFile -Pattern '(?m)^\s*(?:supply_mass_flow_rate|outdoor_air_mass_flow_rate|o_a_mass_flow_rate)\s*=' -Description "line-2158-or-later mass-flow assignment in CP323"
}
Assert-NotContains -Path $cp323ModelIdealLoads -Pattern 'EMSOverrideMdotOn|EMSValueMassFlowRate|ems_supply_mass_flow_override' -Description "fabricated IdealLoads EMS actuator input field"

# CP323 consumes the exact completed CP322 state and private witness.
Assert-Contains -Path $cp323Release -Pattern 'PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot' -Description "CP323 CP322 predecessor type"
Assert-Contains -Path $cp323Release -Pattern 'completed_direct_prefix_through_supply_maximum_is_consistent\s*\(' -Description "CP323 completed CP322 prefix validation"
Assert-Contains -Path $cp323RuntimeValidation -Pattern 'pub\(super\) fn completed_supply_maximum_state_is_consistent\s*\(' -Description "CP323 completed CP322 state validation"
Assert-Contains -Path $cp323RuntimeValidation -Pattern '(?s)state\.latest == Some\(predecessor\).*witness == Some\(predecessor\).*cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release\(predecessor\)' -Description "CP323 exact retained CP322 lineage"
Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_supply_mass_flow_ems_override_guard_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot' -Description "runtime-root private CP323 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_supply_mass_flow_ems_override_guard_latest_witnesses:' -Description "public runtime-root CP323 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_supply_mass_flow_ems_override_guard_latest_witness\s*\(' -Description "runtime-root CP323 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_supply_mass_flow_ems_override_guard_latest_witness\s*\(' -Description "runtime-root CP323 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_supply_mass_flow_ems_override_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState' -Description "per-unit CP323 persistent state"

# Binding order is CP322 -> CP323 -> the pre-existing numerical DTO.
$cp323BindingText = Read-RepoText -Path $idealLoadsBinding
$cp322BindingIndexForCp323 = $cp323BindingText.IndexOf("let calculation_cooling_supply_mass_flow_maximum =")
$cp323BindingIndex = $cp323BindingText.IndexOf("let calculation_cooling_supply_mass_flow_ems_override_guard =")
$numericalBindingIndexForCp323 = $cp323BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp322BindingIndexForCp323 -lt 0 -or
    $cp323BindingIndex -le $cp322BindingIndexForCp323 -or
    $numericalBindingIndexForCp323 -le $cp323BindingIndex
) {
    throw "Binding must retain exact CP322 -> CP323 -> numerical Calc order"
}
Assert-Contains -Path $idealLoadsBinding -Pattern '(?s)let calculation_cooling_supply_mass_flow_ems_override_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_supply_mass_flow_maximum,\s*\)' -Description "binding exact CP322-to-CP323 wrapper call"
$cp322BindingCallForCp323 = [regex]::Match(
    $cp323BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_maximum =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum\(.*?CalculationCoolingSupplyMassFlowMaximum,\s*\)\?;'
)
$cp323BindingCall = [regex]::Match(
    $cp323BindingText,
    '(?s)let calculation_cooling_supply_mass_flow_ems_override_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard\(.*?CalculationCoolingSupplyMassFlowEmsOverrideGuard,\s*\)\?;'
)
if (-not $cp322BindingCallForCp323.Success -or -not $cp323BindingCall.Success) {
    throw "Binding must retain complete CP322 and CP323 exact release calls"
}
$cp322BindingCallEndForCp323 =
    $cp322BindingCallForCp323.Index + $cp322BindingCallForCp323.Length
$cp323BindingCallEnd = $cp323BindingCall.Index + $cp323BindingCall.Length
if (
    $cp323BindingIndex -lt $cp322BindingCallEndForCp323 -or
    $numericalBindingIndexForCp323 -lt $cp323BindingCallEnd
) {
    throw "CP322 and CP323 exact release calls must complete in source order before numerical Calc"
}
$postCp322BeforeCp323 = $cp323BindingText.Substring(
    $cp322BindingCallEndForCp323,
    $cp323BindingIndex - $cp322BindingCallEndForCp323
)
if ($postCp322BeforeCp323 -match '(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)\s*\(') {
    throw "No intermediary helper call may execute after CP322 and before CP323"
}
$postCp323BeforeNumerical = $cp323BindingText.Substring(
    $cp323BindingCallEnd,
    $numericalBindingIndexForCp323 - $cp323BindingCallEnd
)
if ($postCp323BeforeNumerical -match '(?i)(?:ems|psychrometric|diagnostic|node_service)\s*\(|EMSValueMassFlowRate|VerySmallMassFlow|CalcPurchAirMixedAir') {
    throw "No excluded live EMS or later-source behavior may execute after CP323 and before numerical Calc"
}

Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_supply_mass_flow_ems_override_guard_validation;' -Description "coupled CP323 validator declaration"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_supply_mass_flow_ems_override_guard_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary' -Description "coupled CP323 lifecycle"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_supply_mass_flow_ems_override_guard_validation::snapshot_matches_release' -Description "coupled per-timestep CP323 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'cooling_supply_mass_flow_ems_override_guard_validation::validate_lifecycle' -Description "coupled final CP323 validation"
Assert-Contains -Path $cp323CoupledValidation -Pattern 'ems_supply_mass_flow_override_enabled:\s*cooling\.then_some\(false\)' -Description "coupled expected false guard"
Assert-Contains -Path $cp323CoupledValidation -Pattern 'ems_supply_mass_flow_override_body_entry_count",\s*0' -Description "coupled zero body-entry invariant"

Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_supply_mass_flow_ems_override_guard;' -Description "pipeline CP323 module declaration"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle' -Description "pipeline CP323 lifecycle JSON key"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_supply_mass_flow_ems_override_guard::validate_direct_lifecycle' -Description "pipeline CP323 direct firewall"
Assert-Contains -Path $cp323Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER' -Description "pipeline CP322-to-CP323 lineage"
Assert-Contains -Path $cp323Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER' -Description "pipeline CP323 source order"
Assert-Contains -Path $cp323PipelineValidation -Pattern 'ems_supply_mass_flow_override_body_entry_count",\s*0' -Description "pipeline zero body-entry invariant"
Assert-Contains -Path $cp323PipelineSnapshotValidation -Pattern 'ems_supply_mass_flow_override_enabled == Some\(false\)' -Description "pipeline false direct route"
Assert-Contains -Path $cp323PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP323 latest serialization"
Assert-Contains -Path $cp323PipelineSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP323 source-order JSON"
Assert-Contains -Path $cp323PipelineSnapshotSerialization -Pattern '"ems_supply_mass_flow_override_enabled"' -Description "pipeline CP323 false-guard JSON"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'mod cooling_supply_mass_flow_ems_override_guard_assertions;' -Description "direct integration CP323 assertion module"
Assert-Contains -Path $runDirectZoneCoupledTests -Pattern 'assert_cooling_supply_mass_flow_ems_override_guard\(' -Description "direct integration CP323 assertion calls"
Assert-Contains -Path $cp323DirectIntegrationAssertions -Pattern 'purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle' -Description "direct integration CP323 lifecycle key"
Assert-Contains -Path $cp323DirectIntegrationAssertions -Pattern 'ems_supply_mass_flow_override_enabled' -Description "direct integration CP323 false route"

Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern '"CP323 supersedes only CP322' -Description "CP323 algorithm support boundary"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'ems_override_guard\.rs::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState' -Description "CP323 algorithm state target"
Assert-Contains -Path "specs\capabilities.toml" -Pattern '"CP323 additionally requires' -Description "CP323 capability boundary"
Assert-Contains -Path "specs\capabilities.toml" -Pattern '(?s)forbidden_active_features\s*=\s*\[.*?"EMS"' -Description "EMS remains forbidden"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP323 supersedes only CP322' -Description "generated CP323 algorithm boundary"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP323 additionally requires' -Description "generated CP323 capability boundary"
foreach ($cp323Doc in @(
        "docs\src\current\current-status.md",
        "docs\src\current\project-contract.md",
        "docs\src\porting-map\ideal-loads-source-map.md",
        "docs\src\porting-map\heat-balance-source-map.md",
        "docs\src\porting-map\zone-air-update-map.md"
    )) {
    Assert-Contains -Path $cp323Doc -Pattern 'CP323' -Description "CP323 documentation boundary"
    Assert-Contains -Path $cp323Doc -Pattern '2158' -Description "CP323 first excluded executable documentation"
    Assert-Contains -Path $cp323Doc -Pattern 'EMS.*forbidden|`EMS` remains forbidden' -Description "CP323 EMS-forbidden documentation"
}
