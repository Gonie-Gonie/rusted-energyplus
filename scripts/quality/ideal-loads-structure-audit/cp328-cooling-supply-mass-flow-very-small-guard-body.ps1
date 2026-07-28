# CP328 maps only PurchasedAirManager.cc executable line 2167: the single
# cooling supply-mass-flow positive-zero assignment. Line 2168 is a
# non-executable closing delimiter; line 2171 is the first excluded executable.
#
# This file is dot-sourced after the shared assertion helpers and paths exist.
$cp328Module = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body.rs"
$cp328State = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body\state.rs"
$cp328Transition = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body\transition.rs"
$cp328Release = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body\release.rs"
$cp328PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body\release\prefix_validation.rs"
$cp328RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body\release\runtime_validation.rs"
$cp328SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body\release\snapshot_validation.rs"
$cp328Tests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body\tests\mod.rs"
$cp328ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_very_small_guard\body\tests\release_corruption.rs"
$cp328Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp328ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp328BindingTests = "crates\ep_runtime\src\ideal_loads\binding\cooling_supply_mass_flow_very_small_guard_body_tests.rs"
$cp328InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp328InitWitnesses = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\cooling_supply_mass_flow_very_small_guard_body.rs"
$cp328CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp328CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_supply_mass_flow_very_small_guard_body_validation.rs"
$cp328CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\cooling_supply_mass_flow_very_small_guard_body_fixture.rs"
$cp328PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp328Pipeline = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard_body.rs"
$cp328PipelineValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard_body\validation.rs"
$cp328PipelineSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard_body\validation\snapshot.rs"
$cp328PipelineSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard_body\serialization.rs"
$cp328PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_very_small_guard_body\serialization\snapshot.rs"
$cp328RunTests = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled.rs"
$cp328DirectAssertions = "crates\ep_run\tests\arbitrary_run_direct_zone_coupled\cooling_supply_mass_flow_very_small_guard_body_assertions.rs"

foreach ($cp328RequiredFile in @(
        $cp328Module,
        $cp328State,
        $cp328Transition,
        $cp328Release,
        $cp328PrefixValidation,
        $cp328RuntimeValidation,
        $cp328SnapshotValidation,
        $cp328Tests,
        $cp328ReleaseCorruptionTests,
        $cp328ScheduledOutput,
        $cp328BindingTests,
        $cp328CoupledValidation,
        $cp328CoupledFixture,
        $cp328Pipeline,
        $cp328PipelineValidation,
        $cp328PipelineSnapshotValidation,
        $cp328PipelineSerialization,
        $cp328PipelineSnapshotSerialization,
        $cp328DirectAssertions
    )) {
    Assert-FileExists -Path $cp328RequiredFile -Description "CP328 positive-zero reset-body structure"
}

# Source boundary and exact one-site lexical inventory.
Assert-Contains -Path $cp328Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2167' -Description "CP328 exact source boundary"
Assert-Contains -Path $cp328Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2171' -Description "CP328 first excluded executable"
Assert-ExactStringArray -Path $cp328Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER" -Expected @(
    "assign-supply-mass-flow-rate-positive-zero"
) -Description "CP328 exact single lexical source site"
if ([BitConverter]::DoubleToInt64Bits([double]0.0) -ne 0) {
    throw "CP328 source 0.0 must retain positive-zero IEEE bits 0x0000000000000000"
}

Assert-Contains -Path $cp328Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot' -Description "CP328 public snapshot"
Assert-Contains -Path $cp328State -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState' -Description "CP328 persistent state"
Assert-Contains -Path $cp328Module -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary' -Description "CP328 lifecycle summary"
Assert-Contains -Path $cp328Module -Pattern 'pub fn purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle_summary\s*\(' -Description "CP328 lifecycle accessor"
Assert-Contains -Path $cp328Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body\s*\(' -Description "CP328 exact-direct wrapper"
Assert-Contains -Path $cp328Transition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_supply_mass_flow_very_small_guard_body_state\s*\(' -Description "CP328 pure transition"

# CP328 consumes CP327's retained decision. It neither re-reads the threshold
# nor repeats the comparison. True assigns exact +0; false preserves bits.
Assert-PatternsInOrder -Path $cp328Transition -Patterns @(
    'let cooling = predecessor\.cooling_body_entered;',
    'let body_entered = predecessor\.zero_flow_reset_body_entered;',
    'let body_skipped = !body_entered;',
    'let supply_before = if cooling',
    'predecessor\.supply_mass_flow_rate_kg_per_s',
    'let assigned = if body_entered \{ Some\(0\.0_f64\) \} else \{ None \};',
    'let resulting = supply_before\.map\(\|supply\| assigned\.unwrap_or\(supply\)\);',
    'state\.transition_count \+= 1;',
    'if predecessor\.unit_off_skipped',
    'state\.unit_off_skip_count \+= 1;',
    'else if predecessor\.non_cooling_skipped',
    'state\.non_cooling_skip_count \+= 1;',
    'state\.cooling_body_entry_count \+= 1;',
    'if body_entered',
    'state\.zero_flow_reset_body_entry_count \+= 1;',
    'state\.supply_mass_flow_rate_positive_zero_assignment_count \+= 1;',
    'state\.active_guard_false_fallthrough_count \+= 1;'
) -Description "CP328 predecessor-decision, assignment, route, and counter order"
Assert-Contains -Path $cp328State -Pattern 'PositiveZeroAssigned' -Description "CP328 assignment route"
Assert-Contains -Path $cp328State -Pattern 'ActiveGuardFalseFallthrough' -Description "CP328 active false route"
Assert-NotContains -Path $cp328Transition -Pattern '<=|ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW|VerySmallMassFlow|\.abs\(|\.is_finite\(|total_cmp|partial_cmp|\.clamp\(|f64::(?:min|max)|\.(?:min|max)\(' -Description "CP328 repeated guard or replacement normalization"
Assert-NotContains -Path $cp328Transition -Pattern 'CalcPurchAirMixedAir|mixed_air|numerical' -Description "CP328 line-2171-or-later behavior"

foreach ($cp328Test in @(
        "source_boundary_and_exact_single_assignment_site_are_stable",
        "entered_body_assigns_exact_positive_zero_without_retesting_the_guard",
        "active_guard_false_preserves_predecessor_bits_and_skips_the_site",
        "unit_off_and_non_cooling_skip_the_site_and_retain_no_flow",
        "counters_partition_assignment_fallthrough_and_skip_routes",
        "bit_exact_snapshot_comparison_rejects_one_sided_signed_zero_corruption"
    )) {
    Assert-Contains -Path $cp328Tests -Pattern $cp328Test -Description "CP328 pure regression '$cp328Test'"
}
foreach ($cp328ReleaseTest in @(
        "public_release_consumes_only_retained_cp327_and_replay_is_transactional",
        "supplied_retained_or_private_cp327_corruption_is_rejected_without_mutation",
        "invalid_cache_and_counter_overflow_fail_closed_without_mutation",
        "private_cumulative_route_witness_rejects_a_public_counter_partition_forgery",
        "typed_system_identity_mismatch_is_rejected_without_mutation"
    )) {
    Assert-Contains -Path $cp328ReleaseCorruptionTests -Pattern $cp328ReleaseTest -Description "CP328 release regression '$cp328ReleaseTest'"
}

# Validators retain positive-zero and predecessor/result bits exactly and keep
# public counter partitions tied to private route witnesses.
Assert-Contains -Path $cp328SnapshotValidation -Pattern 'assigned\.to_bits\(\) == 0' -Description "CP328 exact assigned positive zero"
Assert-Contains -Path $cp328SnapshotValidation -Pattern 'resulting\.to_bits\(\) == 0' -Description "CP328 exact resulting positive zero"
Assert-Contains -Path $cp328SnapshotValidation -Pattern 'resulting\.to_bits\(\) == predecessor\.to_bits\(\)' -Description "CP328 exact false-route retention"
Assert-Contains -Path $cp328SnapshotValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "CP328 exact snapshot bits"
foreach ($cp328Counter in @(
        "transition_count",
        "cooling_body_entry_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "zero_flow_reset_body_entry_count",
        "body_skip_count",
        "active_guard_false_fallthrough_count",
        "supply_mass_flow_rate_positive_zero_assignment_count"
    )) {
    Assert-Contains -Path $cp328RuntimeValidation -Pattern $cp328Counter -Description "CP328 runtime counter '$cp328Counter'"
}
Assert-Contains -Path $cp328State -Pattern 'pub\(super\) witnessed_zero_flow_reset_body_entry_count:\s*usize' -Description "CP328 private cumulative assignment-route witness"
Assert-Contains -Path $cp328State -Pattern 'pub\(super\) witnessed_active_guard_false_fallthrough_count:\s*usize' -Description "CP328 private cumulative false-route witness"
Assert-Contains -Path $cp328RuntimeValidation -Pattern 'state\.zero_flow_reset_body_entry_count[\r\n\s]+== state\.witnessed_zero_flow_reset_body_entry_count' -Description "CP328 public/private assignment-route reconciliation"
Assert-Contains -Path $cp328RuntimeValidation -Pattern 'state\.active_guard_false_fallthrough_count[\r\n\s]+== state\.witnessed_active_guard_false_fallthrough_count' -Description "CP328 public/private false-route reconciliation"
Assert-Contains -Path $cp328RuntimeValidation -Pattern 'state\.supply_mass_flow_rate_positive_zero_assignment_count[\r\n\s]+== state\.zero_flow_reset_body_entry_count' -Description "CP328 assignment/site counter identity"

# Exact release consumes only the completed same-call CP327 latest/private
# witness. The wrapper has no duplicate scalar or live service.
Assert-Contains -Path $cp328Release -Pattern 'PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot' -Description "CP328 CP327 predecessor type"
Assert-Contains -Path $cp328Release -Pattern 'completed_direct_cooling_supply_mass_flow_very_small_guard_is_consistent\s*\(' -Description "CP328 completed CP327 validation"
Assert-Contains -Path $cp328Release -Pattern 'cooling_supply_mass_flow_very_small_guard_latest_witness\s*\(' -Description "CP328 retained CP327 private witness"
Assert-Contains -Path $cp328Release -Pattern 'unit\.calc_cooling_supply_mass_flow_very_small_guard[\r\n\s.]+latest' -Description "CP328 retained CP327 latest snapshot"
Assert-Contains -Path $cp328Release -Pattern 'cooling_supply_mass_flow_very_small_guard_snapshots_match_bit_exact\s*\(' -Description "CP328 bit-exact CP327 validation"
Assert-Contains -Path $cp328PrefixValidation -Pattern 'guard\.supply_mass_flow_rate_kg_per_s' -Description "CP328 CP327 flow lineage"
Assert-Contains -Path $cp328PrefixValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "CP328 bit-exact predecessor lineage"
Assert-Contains -Path $cp328Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp327: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,\s*\)' -Description "CP328 exact wrapper arguments"
Assert-NotContains -Path $cp328Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body\([^)]*(?:supply_mass_flow_rate|very_small_mass_flow)_kg_per_s\s*:' -Description "duplicate caller scalar in CP328 release"
Assert-NotContains -Path $cp328Release -Pattern 'ems_actuator|ems_service|node_service|psychrometric|schedule_service|sizing_service|diagnostic_service' -Description "live service input in CP328 release"
Assert-NotContains -Path $cp328Release -Pattern 'numerical|calculation\.supply_mass_flow_rate|CalcPurchAirMixedAir' -Description "numerical DTO or later source behavior in CP328 release"

Assert-Contains -Path $cp328InitState -Pattern '(?s)cooling_supply_mass_flow_very_small_guard_body_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot' -Description "runtime-root private CP328 witness map"
Assert-NotContains -Path $cp328InitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_supply_mass_flow_very_small_guard_body_latest_witnesses:' -Description "public runtime-root CP328 witness map"
Assert-Contains -Path $cp328InitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_supply_mass_flow_very_small_guard_body_latest_witness\s*\(' -Description "runtime-root CP328 witness getter"
Assert-Contains -Path $cp328InitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_supply_mass_flow_very_small_guard_body_latest_witness\s*\(' -Description "runtime-root CP328 witness setter"
Assert-Contains -Path $cp328InitState -Pattern 'pub calc_cooling_supply_mass_flow_very_small_guard_body:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState' -Description "per-unit CP328 persistent state"

# Binding order is CP327 -> CP328 -> the unchanged numerical DTO.
$cp328BindingText = Read-RepoText -Path $cp328Binding
$cp327BindingIndexForCp328 = $cp328BindingText.IndexOf("let calculation_cooling_supply_mass_flow_very_small_guard =")
$cp328BindingIndex = $cp328BindingText.IndexOf("let calculation_cooling_supply_mass_flow_very_small_guard_body =")
$numericalBindingIndexForCp328 = $cp328BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp327BindingIndexForCp328 -lt 0 -or
    $cp328BindingIndex -le $cp327BindingIndexForCp328 -or
    $numericalBindingIndexForCp328 -le $cp328BindingIndex
) {
    throw "Binding must retain exact CP327 -> CP328 -> numerical Calc order"
}
Assert-Contains -Path $cp328Binding -Pattern '(?s)let calculation_cooling_supply_mass_flow_very_small_guard_body =\s*advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_supply_mass_flow_very_small_guard,\s*\)' -Description "binding exact CP327-to-CP328 wrapper call"
Assert-Contains -Path $cp328Binding -Pattern 'CalculationCoolingSupplyMassFlowVerySmallGuardBody\(\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError,?\s*\)' -Description "CP328 scheduled binding error boundary"
Assert-Contains -Path $cp328ScheduledOutput -Pattern 'pub calculation_cooling_supply_mass_flow_very_small_guard_body:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot' -Description "CP328 scheduled output evidence"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding_tests.rs" -Pattern '#\[path = "binding/cooling_supply_mass_flow_very_small_guard_body_tests\.rs"\]' -Description "CP328 binding test module path"
foreach ($cp328BindingTest in @(
        "scheduled_binding_consumes_cp327_body_entry_and_assigns_positive_zero",
        "scheduled_binding_skips_the_cp328_site_when_cooling_is_inactive",
        "public_cp328_release_rejects_replay_and_forged_cp327_ordinal_without_mutation"
    )) {
    Assert-Contains -Path $cp328BindingTests -Pattern $cp328BindingTest -Description "CP328 binding regression '$cp328BindingTest'"
}

# Coupled validation reconstructs CP328 solely from CP327. Its runtime section
# does not repeat <= or reconcile against the downstream numerical DTO.
Assert-Contains -Path $cp328CoupledRuntime -Pattern 'mod cooling_supply_mass_flow_very_small_guard_body_validation;' -Description "coupled CP328 validator declaration"
Assert-Contains -Path $cp328CoupledRuntime -Pattern 'pub calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary' -Description "coupled CP328 lifecycle"
Assert-Contains -Path $cp328CoupledRuntime -Pattern 'cooling_supply_mass_flow_very_small_guard_body_validation::snapshot_matches_release' -Description "coupled per-timestep CP328 validation"
Assert-Contains -Path $cp328CoupledRuntime -Pattern 'cooling_supply_mass_flow_very_small_guard_body_validation::validate_lifecycle' -Description "coupled final CP328 validation"
Assert-PatternsInOrder -Path $cp328CoupledValidation -Patterns @(
    'let cooling = predecessor\.cooling_body_entered;',
    'let body_entered = predecessor\.zero_flow_reset_body_entered;',
    'let supply_before = if cooling',
    'predecessor\.supply_mass_flow_rate_kg_per_s',
    'let assigned = body_entered\.then_some\(0\.0_f64\);',
    'let resulting = supply_before\.map\(\|supply\| assigned\.unwrap_or\(supply\)\);'
) -Description "coupled CP327-decision reconstruction"
Assert-Contains -Path $cp328CoupledValidation -Pattern 'options_have_exact_bits' -Description "coupled CP328 exact-bit validation"
foreach ($cp328CoupledTest in @(
        "expected_snapshot_assigns_positive_zero_only_on_predecessor_true_route",
        "expected_snapshot_retains_false_route_bits_without_assignment",
        "snapshot_comparison_detects_signed_zero_result_corruption"
    )) {
    Assert-Contains -Path $cp328CoupledValidation -Pattern $cp328CoupledTest -Description "coupled CP328 regression '$cp328CoupledTest'"
}
$cp328CoupledRuntimeText = (Read-RepoText -Path $cp328CoupledValidation).Split("#[cfg(test)]")[0]
if ($cp328CoupledRuntimeText -match '<=|ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW|resulting_flow_matches_numerical|latest_numerical|CalcPurchAirMixedAir') {
    throw "Coupled CP328 runtime validation must consume CP327 without repeating its guard or reconciling a later DTO"
}
Assert-Contains -Path $cp328CoupledFixture -Pattern 'let body_entered = predecessor\.zero_flow_reset_body_entered;' -Description "coupled fixture CP327 decision provenance"
Assert-Contains -Path $cp328CoupledFixture -Pattern 'body_entered\.then_some\(0\.0_f64\)' -Description "coupled fixture positive-zero assignment"
Assert-NotContains -Path $cp328CoupledFixture -Pattern '<=|ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW|CalcPurchAirMixedAir|numerical' -Description "coupled fixture repeated guard or later DTO"

# Pipeline evidence is direct-only, bit-exact, and distinct from the final
# numerical PurchasedAir DTO.
Assert-Contains -Path $cp328PipelineRoot -Pattern 'mod purchased_air_cooling_supply_mass_flow_very_small_guard_body;' -Description "pipeline CP328 module declaration"
Assert-Contains -Path $cp328PipelineRoot -Pattern 'purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle' -Description "pipeline CP328 lifecycle JSON key"
Assert-Contains -Path $cp328PipelineRoot -Pattern 'purchased_air_cooling_supply_mass_flow_very_small_guard_body::validate_direct_lifecycle' -Description "pipeline CP328 direct firewall"
Assert-Contains -Path $cp328Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER' -Description "pipeline CP327-to-CP328 lineage"
Assert-Contains -Path $cp328Pipeline -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER' -Description "pipeline CP328 source order"
Assert-Contains -Path $cp328PipelineValidation -Pattern 'supply_mass_flow_rate_positive_zero_assignment_count' -Description "pipeline CP328 assignment counter"
Assert-Contains -Path $cp328Pipeline -Pattern 'zero_flow_reset_body_entry_count' -Description "pipeline CP328 entry counter"
Assert-Contains -Path $cp328Pipeline -Pattern 'active_guard_false_fallthrough_count' -Description "pipeline CP328 false-route counter"
Assert-Contains -Path $cp328PipelineSnapshotValidation -Pattern 'let body_entered = predecessor\.zero_flow_reset_body_entered;' -Description "pipeline CP327 decision provenance"
Assert-Contains -Path $cp328PipelineSnapshotValidation -Pattern 'option_has_bits\(snapshot\.assigned_supply_mass_flow_rate_kg_per_s, 0\.0\)' -Description "pipeline CP328 assignment bits"
Assert-Contains -Path $cp328PipelineSnapshotValidation -Pattern 'value\.to_bits\(\) == expected\.to_bits\(\)' -Description "pipeline CP328 exact-bit snapshot validation"
$cp328PipelineSnapshotRuntimeText = (Read-RepoText -Path $cp328PipelineSnapshotValidation).Split("#[cfg(test)]")[0]
if ($cp328PipelineSnapshotRuntimeText -match '<=|ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW|latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow|CalcPurchAirMixedAir') {
    throw "Pipeline CP328 runtime validation must consume CP327 without repeating its guard or reconciling a later DTO"
}
Assert-NotContains -Path $cp328Pipeline -Pattern 'latest_numerical|numerical_supply_mass_flow|final_supply_mass_flow' -Description "final numerical DTO reconciliation in CP328 pipeline"
Assert-Contains -Path $cp328PipelineSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP328 latest serialization"
Assert-Contains -Path $cp328PipelineSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP328 source-order JSON"
foreach ($cp328ValueField in @(
        "predecessor_supply_mass_flow_rate_kg_per_s",
        "assigned_supply_mass_flow_rate_kg_per_s",
        "resulting_supply_mass_flow_rate_kg_per_s"
    )) {
    Assert-Contains -Path $cp328PipelineSnapshotSerialization -Pattern ('"' + [regex]::Escape($cp328ValueField) + '_ieee_bits"') -Description "pipeline CP328 IEEE field '$cp328ValueField'"
}
Assert-Contains -Path $cp328PipelineSnapshotSerialization -Pattern 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -Description "pipeline CP328 exact IEEE serialization"
Assert-Contains -Path $cp328Pipeline -Pattern 'json_preserves_false_route_nan_and_true_route_positive_zero_bits' -Description "pipeline CP328 JSON bit regression"
Assert-Contains -Path $cp328RunTests -Pattern 'mod cooling_supply_mass_flow_very_small_guard_body_assertions;' -Description "direct integration CP328 assertion module"
Assert-Contains -Path $cp328RunTests -Pattern 'assert_cooling_supply_mass_flow_very_small_guard_body\(' -Description "direct integration CP328 assertion calls"
Assert-Contains -Path $cp328DirectAssertions -Pattern 'purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle' -Description "direct integration CP328 lifecycle key"
foreach ($cp328BitField in @(
        "predecessor_supply_mass_flow_rate_kg_per_s_ieee_bits",
        "assigned_supply_mass_flow_rate_kg_per_s_ieee_bits",
        "resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"
    )) {
    Assert-Contains -Path $cp328DirectAssertions -Pattern $cp328BitField -Description "direct integration CP328 IEEE field '$cp328BitField'"
}

# Specs and generated docs preserve the non-promotion boundary.
$cp328AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp328AlgorithmAddenda = [regex]::Matches(
    $cp328AlgorithmText,
    '(?m)^\s*"CP328 supersedes only CP327[^"\r\n]+",\s*$'
)
if ($cp328AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP328 support addenda"
}
foreach ($cp328AlgorithmAddendum in $cp328AlgorithmAddenda) {
    $cp328Text = $cp328AlgorithmAddendum.Value
    foreach ($cp328Pattern in @(
            'line-2167',
            'one lexical site',
            '0x0000000000000000',
            'CP327-to-CP328-to-numerical',
            'Line 2168 is an excluded non-executable\s+closing delimiter',
            'line 2171 is the first excluded executable',
            '`EMS` and Autosizing remain forbidden',
            'Roadmap state remain unchanged'
        )) {
        if ($cp328Text -notmatch $cp328Pattern) {
            throw "CP328 algorithm addendum missing '$cp328Pattern'"
        }
    }
}
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_very_small_guard/body/release\.rs::advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body' -Description "CP328 algorithm wrapper target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'cooling_supply_mass_flow_very_small_guard/body\.rs::purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle_summary' -Description "CP328 algorithm lifecycle target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'body\.rs::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState' -Description "CP328 routine state target"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'body\.rs::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary' -Description "CP328 routine lifecycle target"

$cp328CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp328CapabilityAddenda = [regex]::Matches(
    $cp328CapabilityText,
    '(?m)^\s*"CP328 additionally requires[^"\r\n]+",\s*$'
)
if ($cp328CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP328 claim addenda"
}
foreach ($cp328CapabilityAddendum in $cp328CapabilityAddenda) {
    $cp328Text = $cp328CapabilityAddendum.Value
    foreach ($cp328Pattern in @(
            'line 2167',
            'one lexical site',
            '0x0000000000000000',
            'Line 2168 is an excluded non-executable closing delimiter',
            'line 2171 is the first excluded executable',
            '`EMS` and Autosizing remain forbidden',
            'This changes no support level'
        )) {
        if ($cp328Text -notmatch $cp328Pattern) {
            throw "CP328 capability addendum missing '$cp328Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP328 supersedes only CP327' -Description "generated CP328 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP328 additionally requires' -Description "generated CP328 capability index"

# Every hand-authored contract repeats the exact source/zero, retained CP327
# provenance, non-executable delimiter, first executable exclusion, and
# non-promotion terms.
$cp328DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP328 maps only the executable line-2167 Cooling supply mass-flow reset body.*?^and Roadmap state remain unchanged\.\s*$'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP328 Source-Ordered Cooling Supply Mass-Flow Positive-Zero Reset Body\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP328 Cooling Supply Mass-Flow Positive-Zero Reset Body\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP328 Cooling Supply Mass-Flow Positive-Zero Reset in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP328 Cooling Supply Mass-Flow Positive-Zero Reset Body Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp328Documentation in $cp328DocumentationSections) {
    $cp328DocumentText = Read-RepoText -Path $cp328Documentation.Path
    $cp328Matches = [regex]::Matches($cp328DocumentText, $cp328Documentation.Pattern)
    if ($cp328Matches.Count -ne 1) {
        throw "CP328 documentation expected one scoped section in $($cp328Documentation.Path), found $($cp328Matches.Count)"
    }
    $cp328Section = $cp328Matches[0].Value
    foreach ($cp328Pattern in @(
            'line 2167|line-2167',
            '(?:one|single)(?:-site|\s+lexical| exact lexical)',
            '0x0000000000000000',
            '(?is)CP327.{0,220}(?:bit|exact|same-call|retained)',
            '(?i)UnitOff',
            '(?i)non-cooling',
            '(?i)NaN',
            '-0\.0',
            'purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle',
            'Line 2168 is an excluded non-executable\s+closing delimiter',
            '(?is)line 2171.{0,80}first excluded\s+executable',
            '(?i)`EMS`\s+and\s+Autosizing\s+remain\s+forbidden',
            '(?i)support',
            '(?i)conformance',
            '(?i)Roadmap'
        )) {
        if ($cp328Section -notmatch $cp328Pattern) {
            throw "CP328 documentation in $($cp328Documentation.Path) missing '$cp328Pattern'"
        }
    }
}

# Main audit and generated script inventory remain ordered by source checkpoint.
$cp328MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp327DotSourceIndexForCp328 = $cp328MainAuditText.IndexOf('ideal-loads-structure-audit\cp327-cooling-supply-mass-flow-very-small-guard.ps1')
$cp328DotSourceIndex = $cp328MainAuditText.IndexOf('ideal-loads-structure-audit\cp328-cooling-supply-mass-flow-very-small-guard-body.ps1')
$cp329DotSourceIndexForCp328 = $cp328MainAuditText.IndexOf('ideal-loads-structure-audit\cp329-cooling-mixed-air-call.ps1')
$cp328AuditCompletionIndex = $cp328MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp327DotSourceIndexForCp328 -lt 0 -or
    $cp328DotSourceIndex -le $cp327DotSourceIndexForCp328 -or
    $cp329DotSourceIndexForCp328 -le $cp328DotSourceIndex -or
    $cp328AuditCompletionIndex -le $cp329DotSourceIndexForCp328
) {
    throw "Main IdealLoads audit must dot-source CP328 after CP327 and before CP329 and completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp328-cooling-supply-mass-flow-very-small-guard-body\.ps1"' -Description "CP328 audit script inventory entry"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp328-cooling-supply-mass-flow-very-small-guard-body\.ps1::dot_sources' -Description "CP328 main-audit callee evidence"
