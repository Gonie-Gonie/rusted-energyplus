# CP327 maps only PurchasedAirManager.cc executable line 2166: the complete
# cooling supply-mass-flow very-small-flow guard. Line 2167 is the first
# excluded executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp327Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard.rs"
$cp327State = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\state.rs"
$cp327Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\transition.rs"
$cp327Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\release.rs"
$cp327PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\release\prefix_validation.rs"
$cp327RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\release\runtime_validation.rs"
$cp327SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\release\snapshot_validation.rs"
$cp327Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\tests\mod.rs"
$cp327ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\tests\release_corruption.rs"
$cp327Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp327BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_supply_mass_flow_very_small_guard_tests.rs"
$cp327InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp327InitWitnesses = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp327CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp327CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_supply_mass_flow_very_small_guard_validation.rs"
$cp327CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_supply_mass_flow_very_small_guard_fixture.rs"
$cp327PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp327Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard.rs"
$cp327PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard\validation.rs"
$cp327PipelineSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard\validation\snapshot.rs"
$cp327PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard\serialization.rs"
$cp327PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard\serialization\snapshot.rs"
$cp327RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp327DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_supply_mass_flow_very_small_guard_assertions.rs"

foreach ($cp327RequiredFile in @(
        $cp327Module,
        $cp327State,
        $cp327Transition,
        $cp327Release,
        $cp327PrefixValidation,
        $cp327RuntimeValidation,
        $cp327SnapshotValidation,
        $cp327Tests,
        $cp327ReleaseCorruptionTests,
        $cp327BindingTests,
        $cp327CoupledValidation,
        $cp327CoupledFixture,
        $cp327Pipeline,
        $cp327PipelineValidation,
        $cp327PipelineSnapshotValidation,
        $cp327PipelineSerialization,
        $cp327PipelineSnapshotSerialization,
        $cp327DirectAssertions
    )) {
    Assert-FileExists -Path $cp327RequiredFile -Description "CP327 very-small-flow guard structure"
}

# Source boundary, constant provenance, and exact four-site lexical inventory.
Assert-Contains -Path $cp327Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2166' -Description "CP327 exact source boundary"
Assert-Contains -Path $cp327Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2167' -Description "CP327 first excluded executable"
Assert-Contains -Path $cp327Module -Pattern 'EnergyPlus 26\.1 DataHVACGlobals\.hh:89' -Description "CP327 threshold source provenance"
Assert-Contains -Path $cp327Module -Pattern 'pub const ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S:\s*f64\s*=\s*1\.0e-30;' -Description "CP327 exact EnergyPlus threshold"
Assert-Contains -Path $cp327Module -Pattern '(?is)lexical inventory.{0,100}no claim about C\+\+ operand.{0,30}evaluation order' -Description "CP327 no relational-operand order claim"
Assert-ExactStringArray -Path $cp327Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER" -Expected @(
    "read-retained-supply-mass-flow-rate",
    "read-hvac-very-small-mass-flow",
    "compare-supply-mass-flow-rate-less-than-or-equal-to-hvac-very-small-mass-flow",
    "enter-zero-flow-reset-body-if-at-or-below-threshold"
) -Description "CP327 exact four lexical source sites"
if ([BitConverter]::DoubleToInt64Bits([double]1.0e-30) -ne [long]0x39b4484bfeebc2a0) {
    throw "CP327 pinned threshold must retain IEEE bits 0x39b4484bfeebc2a0"
}

Assert-Contains -Path $cp327Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot' -Description "CP327 public snapshot"
Assert-Contains -Path $cp327State -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState' -Description "CP327 persistent state"
Assert-Contains -Path $cp327Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary' -Description "CP327 lifecycle summary"
Assert-Contains -Path $cp327Module -Pattern 'pub fn purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle_summary\s*\(' -Description "CP327 lifecycle accessor"
Assert-Contains -Path $cp327Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard\s*\(' -Description "CP327 exact-direct wrapper"
Assert-Contains -Path $cp327Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_supply_mass_flow_very_small_guard_state\s*\(' -Description "CP327 pure transition"

# Every active Cooling predecessor evaluates ordinary binary64 <=. UnitOff and
# non-cooling predecessors skip all four sites.
Assert-PatternsInOrder -Path $cp327Transition -Patterns @(
    'let cooling = predecessor\.cooling_body_entered;',
    'let supply_mass_flow_rate_kg_per_s = if cooling',
    'let hvac_very_small_mass_flow_kg_per_s = if cooling',
    'Some\(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S\)',
    'else \{\s*None\s*\};',
    '\.map\(\|\(supply, threshold\)\| supply <= threshold\);',
    'let body_entered = at_or_below == Some\(true\);',
    'let false_fallthrough = at_or_below == Some\(false\);',
    'if predecessor\.unit_off_skipped',
    'else if predecessor\.non_cooling_skipped',
    'state\.supply_mass_flow_rate_read_count \+= 1;',
    'state\.hvac_very_small_mass_flow_read_count \+= 1;',
    'state\.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count \+= 1;',
    'state\.zero_flow_reset_body_entry_count \+= 1;',
    'state\.active_guard_false_fallthrough_count \+= 1;'
) -Description "CP327 operand, comparison, route, and counter order"
Assert-Contains -Path $cp327Transition -Pattern 'ZeroFlowResetBodyEntered' -Description "CP327 true body-entry route"
Assert-Contains -Path $cp327Transition -Pattern 'ActiveGuardFalseFallthrough' -Description "CP327 active false route"
Assert-NotContains -Path $cp327Transition -Pattern '\.abs\(|\.is_finite\(|total_cmp|partial_cmp|\.clamp\(|f64::(?:min|max)|\.(?:min|max)\(' -Description "CP327 replacement comparison or normalization"
Assert-NotContains -Path $cp327Transition -Pattern 'CalcPurchAirMixedAir|mixed_air|supply_mass_flow_rate[^;\r\n]*=\s*0(?:\.0)?' -Description "CP327 line-2167-or-later behavior"
Assert-Contains -Path $cp327Tests -Pattern 'source_boundary_constant_provenance_and_exact_four_textual_sites_are_stable' -Description "CP327 source boundary regression"
Assert-Contains -Path $cp327Tests -Pattern 'source_less_than_or_equal_preserves_ieee_nan_signed_zero_and_infinities' -Description "CP327 IEEE comparison regression"
Assert-Contains -Path $cp327Tests -Pattern 'unit_off_and_non_cooling_skip_both_operands_comparison_and_body_entry' -Description "CP327 complete-skip regression"
Assert-Contains -Path $cp327Tests -Pattern 'bit_exact_snapshot_comparison_rejects_one_sided_signed_zero_corruption' -Description "CP327 bit-exact snapshot regression"
Assert-Contains -Path $cp327ReleaseCorruptionTests -Pattern 'public_release_reads_only_retained_cp326_supply_and_replay_is_transactional' -Description "CP327 retained-supply release regression"
Assert-Contains -Path $cp327ReleaseCorruptionTests -Pattern 'supplied_retained_or_private_cp326_corruption_is_rejected_without_mutation' -Description "CP327 CP326 corruption regression"
Assert-Contains -Path $cp327ReleaseCorruptionTests -Pattern 'private_cumulative_route_witness_rejects_a_public_counter_partition_forgery' -Description "CP327 private route-witness regression"

# Snapshot and runtime validators retain exact threshold/result bits and the
# complete skip/active partitions.
Assert-Contains -Path $cp327SnapshotValidation -Pattern 'let expected = supply <= threshold;' -Description "CP327 snapshot source comparison"
Assert-Contains -Path $cp327SnapshotValidation -Pattern 'threshold\.to_bits\(\) == ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S\.to_bits\(\)' -Description "CP327 exact threshold bits"
Assert-Contains -Path $cp327SnapshotValidation -Pattern 'snapshot\.zero_flow_reset_body_entered == expected' -Description "CP327 body-entry consistency"
Assert-Contains -Path $cp327SnapshotValidation -Pattern 'snapshot\.active_guard_false_fallthrough != expected' -Description "CP327 false-fallthrough consistency"
Assert-Contains -Path $cp327SnapshotValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "CP327 exact retained bits"
foreach ($cp327Counter in @(
        "supply_mass_flow_rate_read_count",
        "hvac_very_small_mass_flow_read_count",
        "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count",
        "zero_flow_reset_body_entry_count",
        "active_guard_false_fallthrough_count"
    )) {
    Assert-Contains -Path $cp327RuntimeValidation -Pattern $cp327Counter -Description "CP327 runtime counter '$cp327Counter'"
}
Assert-Contains -Path $cp327State -Pattern 'pub\(super\) witnessed_zero_flow_reset_body_entry_count:\s*usize' -Description "CP327 private cumulative body-entry witness"
Assert-Contains -Path $cp327State -Pattern 'pub\(super\) witnessed_active_guard_false_fallthrough_count:\s*usize' -Description "CP327 private cumulative false-route witness"
Assert-Contains -Path $cp327RuntimeValidation -Pattern 'state\.zero_flow_reset_body_entry_count[\r\n\s]+== state\.witnessed_zero_flow_reset_body_entry_count' -Description "CP327 public/private body-entry reconciliation"
Assert-Contains -Path $cp327RuntimeValidation -Pattern 'state\.active_guard_false_fallthrough_count[\r\n\s]+== state\.witnessed_active_guard_false_fallthrough_count' -Description "CP327 public/private false-route reconciliation"

# Exact release consumes only the completed same-call CP326 result and private
# witness; it has no duplicate flow/threshold scalar or live service.
Assert-Contains -Path $cp327Release -Pattern 'PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot' -Description "CP327 CP326 predecessor type"
Assert-Contains -Path $cp327Release -Pattern 'completed_direct_cooling_supply_mass_flow_limit_body_is_consistent\s*\(' -Description "CP327 completed CP326 validation"
Assert-Contains -Path $cp327Release -Pattern 'cooling_supply_mass_flow_limit_body_latest_witness\s*\(' -Description "CP327 retained CP326 private witness"
Assert-Contains -Path $cp327Release -Pattern 'unit\.calc_cooling_supply_mass_flow_limit_body\.latest' -Description "CP327 retained CP326 latest snapshot"
Assert-Contains -Path $cp327Release -Pattern 'predecessor_cp326[\r\n\s.]+resulting_supply_mass_flow_rate_kg_per_s' -Description "CP327 retained post-CP326 supply provenance"
Assert-Contains -Path $cp327PrefixValidation -Pattern '(?s)guard\.supply_mass_flow_rate_kg_per_s,.*?body\.resulting_supply_mass_flow_rate_kg_per_s' -Description "CP327 bit-exact CP326 lineage"
Assert-Contains -Path $cp327Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp326: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,\s*\)' -Description "CP327 exact wrapper arguments"
Assert-NotContains -Path $cp327Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard\([^)]*(?:supply_mass_flow_rate|very_small_mass_flow)_kg_per_s\s*:' -Description "duplicate caller scalar in CP327 release"
Assert-NotContains -Path $cp327Release -Pattern 'ems_actuator|ems_service|node_service|psychrometric|schedule_service|diagnostic_service' -Description "live service input in CP327 release"
Assert-NotContains -Path $cp327Release -Pattern 'numerical|calculation\.supply_mass_flow_rate|CalcPurchAirMixedAir' -Description "numerical DTO or later source behavior in CP327 release"

Assert-Contains -Path $cp327InitState -Pattern '(?s)cooling_supply_mass_flow_very_small_guard_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot' -Description "runtime-root private CP327 witness map"
Assert-NotContains -Path $cp327InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_supply_mass_flow_very_small_guard_latest_witnesses:' -Description "public runtime-root CP327 witness map"
Assert-Contains -Path $cp327InitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_supply_mass_flow_very_small_guard_latest_witness\s*\(' -Description "runtime-root CP327 witness getter"
Assert-Contains -Path $cp327InitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_supply_mass_flow_very_small_guard_latest_witness\s*\(' -Description "runtime-root CP327 witness setter"
Assert-Contains -Path $cp327InitState -Pattern 'pub calc_cooling_supply_mass_flow_very_small_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState' -Description "per-unit CP327 persistent state"

# Binding order is CP326 -> CP327 -> the unchanged numerical DTO.
$cp327BindingText = Read-RepoText -Path $cp327Binding
$cp326BindingIndexForCp327 = $cp327BindingText.IndexOf("let calculation_cooling_supply_mass_flow_limit_body =")
$cp327BindingIndex = $cp327BindingText.IndexOf("let calculation_cooling_supply_mass_flow_very_small_guard =")
$numericalBindingIndexForCp327 = $cp327BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp326BindingIndexForCp327 -lt 0 -or
    $cp327BindingIndex -le $cp326BindingIndexForCp327 -or
    $numericalBindingIndexForCp327 -le $cp327BindingIndex
) {
    throw "Binding must retain exact CP326 -> CP327 -> numerical Calc order"
}
Assert-Contains -Path $cp327Binding -Pattern '(?s)let calculation_cooling_supply_mass_flow_very_small_guard =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_supply_mass_flow_limit_body,\s*\)' -Description "binding exact CP326-to-CP327 wrapper call"
Assert-Contains -Path $cp327Binding -Pattern 'CalculationCoolingSupplyMassFlowVerySmallGuard\(\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError,?\s*\)' -Description "CP327 scheduled binding error boundary"
Assert-Contains -Path $cp327Binding -Pattern 'pub calculation_cooling_supply_mass_flow_very_small_guard:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot' -Description "CP327 scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_supply_mass_flow_very_small_guard_tests\.rs"\]' -Description "CP327 binding test module path"
Assert-Contains -Path $cp327BindingTests -Pattern 'scheduled_binding_consumes_the_retained_cp326_supply_bits_for_the_guard' -Description "CP327 active binding regression"
Assert-Contains -Path $cp327BindingTests -Pattern 'scheduled_binding_skips_all_cp327_sites_when_cooling_is_inactive' -Description "CP327 skip binding regression"
Assert-Contains -Path $cp327BindingTests -Pattern 'public_cp327_release_rejects_replay_and_forged_cp326_ordinal_without_mutation' -Description "CP327 release corruption regression"

# Coupled validation independently reconstructs CP327 from the retained CP326
# result and never reconciles against the downstream numerical DTO.
Assert-Contains -Path $cp327CoupledRuntime -Pattern 'mod cooling_supply_mass_flow_very_small_guard_validation;' -Description "coupled CP327 validator declaration"
Assert-Contains -Path $cp327CoupledRuntime -Pattern 'pub calc_cooling_supply_mass_flow_very_small_guard_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary' -Description "coupled CP327 lifecycle"
Assert-Contains -Path $cp327CoupledRuntime -Pattern 'cooling_supply_mass_flow_very_small_guard_validation::snapshot_matches_release' -Description "coupled per-timestep CP327 validation"
Assert-Contains -Path $cp327CoupledRuntime -Pattern 'cooling_supply_mass_flow_very_small_guard_validation::validate_lifecycle' -Description "coupled final CP327 validation"
Assert-Contains -Path $cp327CoupledValidation -Pattern 'predecessor\.resulting_supply_mass_flow_rate_kg_per_s' -Description "coupled retained CP326 supply provenance"
Assert-Contains -Path $cp327CoupledValidation -Pattern '\.map\(\|\(supply, threshold\)\| supply <= threshold\)' -Description "coupled source comparison"
Assert-Contains -Path $cp327CoupledValidation -Pattern 'options_have_exact_bits' -Description "coupled CP327 exact-bit validation"
Assert-Contains -Path $cp327CoupledValidation -Pattern 'expected_snapshot_preserves_cp326_bits_and_characterizes_threshold_cases' -Description "coupled CP327 IEEE cases"
Assert-Contains -Path $cp327CoupledValidation -Pattern 'snapshot_comparison_detects_signed_zero_bit_corruption' -Description "coupled CP327 signed-zero corruption"
Assert-NotContains -Path $cp327CoupledValidation -Pattern 'resulting_flow_matches_numerical|(?s)output\s*\.\s*coupling\s*\.\s*purchased_air\s*\.\s*calculation\s*\.\s*supply_mass_flow_rate_kg_per_s' -Description "final numerical DTO reconciliation in CP327 coupled validation"
Assert-Contains -Path $cp327CoupledFixture -Pattern '\.map\(\|\(supply, threshold\)\| supply <= threshold\)' -Description "coupled fixture CP327 source comparison"

# Pipeline evidence is direct-only, bit-exact, and distinct from the final
# numerical PurchasedAir DTO.
Assert-Contains -Path $cp327PipelineRoot -Pattern 'mod purchased_air_cooling_supply_mass_flow_very_small_guard;' -Description "pipeline CP327 module declaration"
Assert-Contains -Path $cp327PipelineRoot -Pattern 'purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle' -Description "pipeline CP327 lifecycle JSON key"
Assert-Contains -Path $cp327PipelineRoot -Pattern 'purchased_air_cooling_supply_mass_flow_very_small_guard::validate_direct_lifecycle' -Description "pipeline CP327 direct firewall"
Assert-Contains -Path $cp327Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER' -Description "pipeline CP326-to-CP327 lineage"
Assert-Contains -Path $cp327Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER' -Description "pipeline CP327 source order"
Assert-Contains -Path $cp327PipelineValidation -Pattern 'supply_mass_flow_rate_read_count' -Description "pipeline CP327 supply-read counter"
Assert-Contains -Path $cp327PipelineValidation -Pattern 'hvac_very_small_mass_flow_read_count' -Description "pipeline CP327 threshold-read counter"
Assert-Contains -Path $cp327PipelineValidation -Pattern 'supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count' -Description "pipeline CP327 comparison counter"
Assert-Contains -Path $cp327PipelineSnapshotValidation -Pattern 'predecessor_supply <= ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S' -Description "pipeline CP327 source comparison"
Assert-Contains -Path $cp327PipelineSnapshotValidation -Pattern 'to_bits\(\)' -Description "pipeline CP327 exact-bit snapshot validation"
Assert-NotContains -Path $cp327Pipeline -Pattern 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow' -Description "final numerical DTO result reconciliation in CP327 pipeline"
Assert-Contains -Path $cp327PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP327 latest serialization"
Assert-Contains -Path $cp327PipelineSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP327 source-order JSON"
foreach ($cp327ValueField in @(
        "supply_mass_flow_rate_kg_per_s",
        "hvac_very_small_mass_flow_kg_per_s"
    )) {
    Assert-Contains -Path $cp327PipelineSnapshotSerialization -Pattern ('"' + [regex]::Escape($cp327ValueField) + '_ieee_bits"') -Description "pipeline CP327 IEEE field '$cp327ValueField'"
}
Assert-Contains -Path $cp327PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP327 exact IEEE serialization"
Assert-Contains -Path $cp327RunTests -Pattern 'mod cooling_supply_mass_flow_very_small_guard_assertions;' -Description "direct integration CP327 assertion module"
Assert-Contains -Path $cp327RunTests -Pattern 'assert_cooling_supply_mass_flow_very_small_guard\(' -Description "direct integration CP327 assertion calls"
Assert-Contains -Path $cp327DirectAssertions -Pattern 'purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle' -Description "direct integration CP327 lifecycle key"
Assert-Contains -Path $cp327DirectAssertions -Pattern 'hvac_very_small_mass_flow_kg_per_s_ieee_bits' -Description "direct integration CP327 threshold-bit evidence"

# Specs and generated docs preserve the non-promotion boundary.
$cp327AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp327AlgorithmAddenda = [regex]::Matches(
    $cp327AlgorithmText,
    '(?m)^\s*"CP327 supersedes only CP326[^"\r\n]+",\s*$'
)
if ($cp327AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP327 support addenda"
}
foreach ($cp327AlgorithmAddendum in $cp327AlgorithmAddenda) {
    $cp327Text = $cp327AlgorithmAddendum.Value
    foreach ($cp327Pattern in @(
            'line-2166',
            'four lexical sites',
            'DataHVACGlobals\.hh:89',
            '0x39b4484bfeebc2a0',
            'without claiming a C\+\+ built-in relational-operand evaluation order',
            'CP326-to-CP327-to-numerical',
            'Line 2167 is the first excluded executable',
            '`EMS` and Autosizing remain forbidden',
            'Roadmap state remain unchanged'
        )) {
        if ($cp327Text -notmatch $cp327Pattern) {
            throw "CP327 algorithm addendum missing '$cp327Pattern'"
        }
    }
}
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_very_small_guard/release\.rs::advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard' -Description "CP327 algorithm wrapper target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_very_small_guard\.rs::purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle_summary' -Description "CP327 algorithm lifecycle target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_very_small_guard\.rs::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState' -Description "CP327 routine state target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_very_small_guard\.rs::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary' -Description "CP327 routine lifecycle target"

$cp327CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp327CapabilityAddenda = [regex]::Matches(
    $cp327CapabilityText,
    '(?m)^\s*"CP327 additionally requires[^"\r\n]+",\s*$'
)
if ($cp327CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP327 claim addenda"
}
foreach ($cp327CapabilityAddendum in $cp327CapabilityAddenda) {
    $cp327Text = $cp327CapabilityAddendum.Value
    foreach ($cp327Pattern in @(
            'line 2166',
            'four-site',
            'DataHVACGlobals\.hh:89',
            'without claiming a C\+\+ built-in relational-operand evaluation order',
            'Line 2167 is the first excluded executable',
            '`EMS` and Autosizing remain forbidden',
            'This changes no support level'
        )) {
        if ($cp327Text -notmatch $cp327Pattern) {
            throw "CP327 capability addendum missing '$cp327Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP327 supersedes only CP326' -Description "generated CP327 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP327 additionally requires' -Description "generated CP327 capability index"

# Every hand-authored contract repeats the exact source/constant, C++ operand
# caveat, retained CP326 provenance, first exclusion, and non-promotion terms.
$cp327DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP327 maps only the complete Cooling supply mass-flow very-small-flow guard.*?^conformance, and Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP327 Source-Ordered Cooling Supply Mass-Flow Very-Small-Flow Guard\r?\n.*?Roadmap item\.\s*'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP327 Cooling Supply Mass-Flow Very-Small-Flow Guard\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP327 Cooling Supply Mass-Flow Very-Small-Flow Guard in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP327 Cooling Supply Mass-Flow Very-Small-Flow Guard Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp327Documentation in $cp327DocumentationSections) {
    $cp327DocumentText = Read-RepoText -Path $cp327Documentation.Path
    $cp327Matches = [regex]::Matches($cp327DocumentText, $cp327Documentation.Pattern)
    if ($cp327Matches.Count -ne 1) {
        throw "CP327 documentation expected one scoped section in $($cp327Documentation.Path), found $($cp327Matches.Count)"
    }
    $cp327Section = $cp327Matches[0].Value
    foreach ($cp327Pattern in @(
            'line 2166|line-2166',
            'four(?:-site|\s+lexical)',
            'DataHVACGlobals\.hh:89',
            'VerySmallMassFlow\(1\.0E-30\)',
            '0x39b4484bfeebc2a0',
            '(?is)no(?:t| claim).{0,100}C\+\+\s+built-in\s+relational-operand evaluation order',
            '(?is)CP326.{0,160}(?:bit-exact|exact)',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?i)NaN|unordered',
            '(?is)signed\s+zeros|\+0\.0.{0,40}-0\.0',
            'purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle',
            'Line 2167 is the first excluded executable',
            '(?i)`EMS`\s+and\s+Autosizing\s+remain\s+forbidden',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp327Section -notmatch $cp327Pattern) {
            throw "CP327 documentation in $($cp327Documentation.Path) missing '$cp327Pattern'"
        }
    }
}

# Main audit and generated script inventory remain ordered by source checkpoint.
$cp327MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp326DotSourceIndexForCp327 = $cp327MainAuditText.IndexOf('ideal-loads-structure-audit\cp326-cooling-supply-mass-flow-limit-body.ps1')
$cp327DotSourceIndex = $cp327MainAuditText.IndexOf('ideal-loads-structure-audit\cp327-cooling-supply-mass-flow-very-small-guard.ps1')
$cp328DotSourceIndexForCp327 = $cp327MainAuditText.IndexOf('ideal-loads-structure-audit\cp328-cooling-supply-mass-flow-very-small-guard-body.ps1')
$cp327AuditCompletionIndex = $cp327MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp326DotSourceIndexForCp327 -lt 0 -or
    $cp327DotSourceIndex -le $cp326DotSourceIndexForCp327 -or
    $cp328DotSourceIndexForCp327 -le $cp327DotSourceIndex -or
    $cp327AuditCompletionIndex -le $cp328DotSourceIndexForCp327
) {
    throw "Main IdealLoads audit must dot-source CP327 after CP326 and before CP328/completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp327-cooling-supply-mass-flow-very-small-guard\.ps1"' -Description "CP327 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp327-cooling-supply-mass-flow-very-small-guard\.ps1::dot_sources' -Description "CP327 main-audit callee evidence"
