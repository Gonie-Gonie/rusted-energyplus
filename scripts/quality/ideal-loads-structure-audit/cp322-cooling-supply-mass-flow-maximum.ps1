# CP322 maps only the exact source-shaped cooling supply-mass-flow maximum at
# PurchasedAirManager.cc line 2155. Line 2157 is the first excluded executable.
#
# This file is dot-sourced by ideal-loads-structure-audit.ps1 after its assertion
# helpers and shared path variables have been defined.
$calcCoolingSupplyMassFlowMaximum = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum.rs"
$calcCoolingSupplyMassFlowMaximumState = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\state.rs"
$calcCoolingSupplyMassFlowMaximumTransition = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\transition.rs"
$calcCoolingSupplyMassFlowMaximumRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\release.rs"
$calcCoolingSupplyMassFlowMaximumRuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\release\runtime_validation.rs"
$calcCoolingSupplyMassFlowMaximumSnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\release\snapshot_validation.rs"
$calcCoolingSupplyMassFlowMaximumTests = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_maximum\tests\mod.rs"
$idealLoadsBinding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$idealLoadsCoupledCoolingSupplyMassFlowMaximumValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\cooling_supply_mass_flow_maximum_validation.rs"
$runPurchasedAirCoolingSupplyMassFlowMaximum = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_maximum.rs"
$runPurchasedAirCoolingSupplyMassFlowMaximumSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_maximum\serialization.rs"
$runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotSerialization = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_maximum\serialization\snapshot.rs"
$runPurchasedAirCoolingSupplyMassFlowMaximumValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_maximum\validation.rs"
$runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotValidation = "crates\ep_run\src\pipeline\purchased_air_cooling_supply_mass_flow_maximum\validation\snapshot.rs"

foreach ($cp322RequiredFile in @(
        $calcCoolingSupplyMassFlowMaximum,
        $calcCoolingSupplyMassFlowMaximumState,
        $calcCoolingSupplyMassFlowMaximumTransition,
        $calcCoolingSupplyMassFlowMaximumRelease,
        $calcCoolingSupplyMassFlowMaximumRuntimeValidation,
        $calcCoolingSupplyMassFlowMaximumSnapshotValidation,
        $calcCoolingSupplyMassFlowMaximumTests,
        $idealLoadsCoupledCoolingSupplyMassFlowMaximumValidation,
        $runPurchasedAirCoolingSupplyMassFlowMaximum,
        $runPurchasedAirCoolingSupplyMassFlowMaximumSerialization,
        $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotSerialization,
        $runPurchasedAirCoolingSupplyMassFlowMaximumValidation,
        $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotValidation
    )) {
    Assert-FileExists -Path $cp322RequiredFile -Description "CP322 cooling supply-mass-flow maximum structure"
}

Assert-Contains -Path $calcRoot -Pattern 'mod cooling_supply_mass_flow_maximum;' -Description "CP322 calc submodule declaration"
Assert-Contains -Path $calcRoot -Pattern 'pub use cooling_supply_mass_flow_maximum::\*;' -Description "CP322 calc public re-export"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximum -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2155' -Description "CP322 exact source boundary"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximum -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2157' -Description "CP322 first excluded executable"
Assert-ExactStringArray -Path $calcCoolingSupplyMassFlowMaximum -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER" -Expected @(
    "read-outdoor-air-mass-flow-rate",
    "read-supply-mass-flow-rate-for-cool",
    "read-supply-mass-flow-rate-for-dehumidification",
    "read-supply-mass-flow-rate-for-humidification",
    "apply-source-shaped-five-argument-maximum-with-positive-zero-floor",
    "assign-supply-mass-flow-rate"
) -Description "CP322 exact six lexical source sites"

Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumState -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState' -Description "CP322 persistent public state"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximum -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot' -Description "CP322 public snapshot"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximum -Pattern 'pub struct PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary' -Description "CP322 public lifecycle summary"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximum -Pattern 'pub fn purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle_summary\s*\(' -Description "CP322 lifecycle accessor"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRelease -Pattern 'pub fn advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum\s*\(' -Description "CP322 exact direct release wrapper"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern 'pub\(in crate::ideal_loads::calc\) fn advance_cooling_supply_mass_flow_maximum_state\s*\(' -Description "CP322 pure transition"

# Preserve the raw ObjexxFCL max(a,b,c,d,e) tournament. The four calls must be:
# max(+0, OA), max(Cool, Dehum), max(the pair winners), then max(that, Humid).
Assert-PatternsInOrder -Path $calcCoolingSupplyMassFlowMaximumTransition -Patterns @(
    'source_pair\(\s*\(Operand::PositiveZeroFloor,\s*0\.0\),\s*\(Operand::OutdoorAir,\s*outdoor_air\),?\s*\)',
    'source_pair\(\s*\(Operand::Cooling,\s*cool\),\s*\(Operand::Dehumidification,\s*dehumidification\),?\s*\)',
    'source_pair\(positive_zero_outdoor_air\.1,\s*cooling_dehumidification\.1\)',
    'source_pair\(\s*leading_candidate_pair\.1,\s*\(Operand::Humidification,\s*humidification\),?\s*\)',
    'let right_wins = left\.1 < right\.1;',
    '\(right_wins,\s*if right_wins \{ right \} else \{ left \}\)'
) -Description "CP322 raw strict-less-than Objexx five-argument maximum tree"
$cp322TransitionText = Read-RepoText -Path $calcCoolingSupplyMassFlowMaximumTransition
$cp322SourcePairCallCount = [regex]::Matches($cp322TransitionText, '(?m)^\s*(?:let\s+\w+\s*=\s*)?source_pair\(').Count
if ($cp322SourcePairCallCount -ne 4) {
    throw "CP322 transition must contain exactly four source_pair tournament calls; found $cp322SourcePairCallCount"
}
foreach ($cp322ForbiddenNumerics in @(
        [pscustomobject]@{ Pattern = '(?<![A-Za-z0-9_])f64::max\s*\('; Description = "f64::max replacement" },
        [pscustomobject]@{ Pattern = '\.max\s*\('; Description = "method max replacement" },
        [pscustomobject]@{ Pattern = '\.(?:total_cmp|partial_cmp|is_finite|clamp)\s*\('; Description = "normalization or alternate floating comparison" }
    )) {
    Assert-NotContains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern $cp322ForbiddenNumerics.Pattern -Description "$($cp322ForbiddenNumerics.Description) in CP322 transition"
}

# UnitOff and non-cooling routes must skip every line-2155 read, comparison,
# evaluation, and assignment site.
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern 'let cooling = predecessor\.cooling_body_entered;' -Description "CP322 predecessor cooling route"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern 'let outdoor_air = cooling\.then_some\(input\.outdoor_air_mass_flow_rate_kg_per_s\);' -Description "CP322 conditional outdoor-air read witness"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern '\.resulting_supply_mass_flow_rate_for_cool_kg_per_s\s*[\r\n]+\s*\.filter\(\|_\| cooling\)' -Description "CP322 conditional cooling-candidate read witness"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern '\.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s\s*[\r\n]+\s*\.filter\(\|_\| cooling\)' -Description "CP322 conditional dehumidification-candidate read witness"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern '\.resulting_supply_mass_flow_rate_for_humidification_kg_per_s\s*[\r\n]+\s*\.filter\(\|_\| cooling\)' -Description "CP322 conditional humidification-candidate read witness"
Assert-PatternsInOrder -Path $calcCoolingSupplyMassFlowMaximumTransition -Patterns @(
    'if predecessor\.unit_off_skipped',
    'else if predecessor\.non_cooling_skipped',
    'else \{',
    'outdoor_air_mass_flow_rate_read_count \+= 1',
    'maximum_evaluation_count \+= 1',
    'supply_mass_flow_rate_assignment_count \+= 1'
) -Description "CP322 UnitOff/non-cooling skip before all active-site counters"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTests -Pattern 'unit_off_and_non_cooling_skip_all_six_sites' -Description "CP322 complete skip regression"

# CP322 consumes only the exact completed CP321 snapshot and the retained CP311
# working OA value. Candidate lineage is from CP321's post-reset results.
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRelease -Pattern 'PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot' -Description "CP322 CP321 predecessor type"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRelease -Pattern 'completed_capacity_zero_reset_is_consistent\s*\(' -Description "CP322 completed CP321 validation call"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRuntimeValidation -Pattern 'pub\(super\) fn completed_capacity_zero_reset_is_consistent\s*\(' -Description "CP322 completed CP321 validator"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRuntimeValidation -Pattern '(?s)state\.latest == Some\(predecessor\).*witness == Some\(predecessor\).*cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release\(predecessor\)' -Description "CP322 retained and exact CP321 lineage validation"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern 'resulting_supply_mass_flow_rate_for_cool_kg_per_s' -Description "CP322 CP321 cooling-result lineage"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern 'resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s' -Description "CP322 CP321 dehumidification-result lineage"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumTransition -Pattern 'resulting_supply_mass_flow_rate_for_humidification_kg_per_s' -Description "CP322 CP321 humidification-result lineage"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRelease -Pattern 'unit\.calc_minimum_oa_prefix\.latest' -Description "CP322 retained CP311 latest snapshot lookup"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRelease -Pattern 'working_outdoor_air_mass_flow_rate_kg_per_s' -Description "CP322 CP311 working outdoor-air lineage"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRelease -Pattern 'completed_direct_prefix_through_economizer_guard_is_consistent\(unit,\s*system,\s*guard\)' -Description "CP322 exact retained full-prefix validation through CP311 and economizer guard"
Assert-Contains -Path $calcCoolingSupplyMassFlowMaximumRelease -Pattern 'outdoor_air\.to_bits\(\) != 0\.0_f64\.to_bits\(\)' -Description "CP322 exact direct-no-OA CP311 value validation"

Assert-Contains -Path $idealLoadsInitState -Pattern '(?s)cooling_supply_mass_flow_maximum_latest_witnesses:\s*BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot>' -Description "runtime-root private CP322 witness map"
Assert-NotContains -Path $idealLoadsInitState -Pattern '(?m)^\s*pub(?:\([^)]*\))?\s+cooling_supply_mass_flow_maximum_latest_witnesses:' -Description "public runtime-root CP322 witness map"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn cooling_supply_mass_flow_maximum_latest_witness\s*\(' -Description "runtime-root CP322 witness getter"
Assert-Contains -Path $idealLoadsInitWitnesses -Pattern 'pub\(in crate::ideal_loads\) fn set_cooling_supply_mass_flow_maximum_latest_witness\s*\(' -Description "runtime-root CP322 witness setter"
Assert-Contains -Path $idealLoadsInitState -Pattern 'pub calc_cooling_supply_mass_flow_maximum:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState' -Description "per-unit CP322 persistent state"

# Nothing from line 2157 or later belongs in the bounded CP322 implementation.
foreach ($cp322ScopeFile in @(
        $calcCoolingSupplyMassFlowMaximum,
        $calcCoolingSupplyMassFlowMaximumState,
        $calcCoolingSupplyMassFlowMaximumTransition,
        $calcCoolingSupplyMassFlowMaximumRelease,
        $calcCoolingSupplyMassFlowMaximumRuntimeValidation,
        $calcCoolingSupplyMassFlowMaximumSnapshotValidation
    )) {
    Assert-NotContains -Path $cp322ScopeFile -Pattern 'EMSOverrideMdotOn|EMSOverrideMdotValue|VerySmallMassFlow|CalcPurchAirMixedAir|mixed[_-]?air' -Description "line-2157-or-later scope creep in CP322"
}

# The scheduled binding must execute CP321, then CP322, then CP323, then the
# existing numerical coupling without any shadow maximum or excluded service.
$cp322BindingText = Read-RepoText -Path $idealLoadsBinding
$cp321BindingIndexForCp322 = $cp322BindingText.IndexOf("let calculation_cooling_capacity_zero_flow_reset =")
$cp322BindingIndex = $cp322BindingText.IndexOf("let calculation_cooling_supply_mass_flow_maximum =")
$cp323BindingIndexForCp322 = $cp322BindingText.IndexOf("let calculation_cooling_supply_mass_flow_ems_override_guard =")
$numericalBindingIndexForCp322 = $cp322BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling")
if (
    $cp321BindingIndexForCp322 -lt 0 -or
    $cp322BindingIndex -le $cp321BindingIndexForCp322 -or
    $cp323BindingIndexForCp322 -le $cp322BindingIndex -or
    $numericalBindingIndexForCp322 -le $cp323BindingIndexForCp322
) {
    throw "Binding must retain exact CP321 -> CP322 -> CP323 -> numerical Calc order"
}
$betweenCp321AndCp322 = $cp322BindingText.Substring(
    $cp321BindingIndexForCp322,
    $cp322BindingIndex - $cp321BindingIndexForCp322
)
$betweenCp322AndCp323 = $cp322BindingText.Substring(
    $cp322BindingIndex,
    $cp323BindingIndexForCp322 - $cp322BindingIndex
)
foreach ($cp322Intermediary in @(
        [pscustomobject]@{ Pattern = '(?<![A-Za-z0-9_])f64::max\s*\(|\.max\s*\('; Description = "shadow floating maximum" },
        [pscustomobject]@{ Pattern = '\.(?:total_cmp|partial_cmp|is_finite|clamp)\s*\('; Description = "normalization or alternate comparison" },
        [pscustomobject]@{ Pattern = '(?i)(?:ems|psychrometric|diagnostic|schedule_service|node_service)\s*\('; Description = "excluded live service" },
        [pscustomobject]@{ Pattern = 'VerySmallMassFlow|CalcPurchAirMixedAir'; Description = "later source behavior" }
    )) {
    if ($betweenCp321AndCp322 -match $cp322Intermediary.Pattern) {
        throw "$($cp322Intermediary.Description) unexpectedly present between CP321 and CP322"
    }
    if ($betweenCp322AndCp323 -match $cp322Intermediary.Pattern) {
        throw "$($cp322Intermediary.Description) unexpectedly present between CP322 and CP323"
    }
}

Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'mod cooling_supply_mass_flow_maximum_validation;' -Description "coupled runtime CP322 validator declaration"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern 'pub calc_cooling_supply_mass_flow_maximum_lifecycle:\s*[\r\n]+\s*PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary' -Description "coupled runtime CP322 lifecycle"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_supply_mass_flow_maximum_validation::snapshot_matches_release\(\s*output,\s*timestep_index \+ 1,\s*&binding,\s*\)' -Description "coupled runtime per-timestep CP322 validation"
Assert-Contains -Path $idealLoadsCoupledRuntime -Pattern '(?s)cooling_supply_mass_flow_maximum_validation::validate_lifecycle\(\s*&calc_cooling_supply_mass_flow_maximum_lifecycle,\s*&calc_cooling_capacity_zero_flow_reset_lifecycle,' -Description "coupled runtime final CP322 validation"
Assert-Contains -Path $idealLoadsCoupledCoolingSupplyMassFlowMaximumValidation -Pattern 'cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release' -Description "coupled CP322 exact snapshot validator"

# Direct-run evidence must expose the lifecycle key and serialize both the
# lexical source order and resulting assigned flow.
Assert-Contains -Path $runPipeline -Pattern 'mod purchased_air_cooling_supply_mass_flow_maximum;' -Description "pipeline CP322 evidence module declaration"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle' -Description "pipeline CP322 lifecycle JSON key"
Assert-Contains -Path $runPipeline -Pattern 'purchased_air_cooling_supply_mass_flow_maximum::validate_direct_lifecycle' -Description "pipeline CP322 direct-only firewall"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximum -Pattern 'mod serialization;' -Description "pipeline CP322 serializer submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximum -Pattern 'mod validation;' -Description "pipeline CP322 validator submodule declaration"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximum -Pattern 'pub\(super\) use serialization::lifecycle_json;' -Description "pipeline CP322 lifecycle serializer wiring"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximum -Pattern 'pub\(super\) fn validate_direct_lifecycle\s*\(' -Description "pipeline CP322 direct validator entry"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximum -Pattern 'PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER' -Description "pipeline CP321-to-CP322 SOURCE_ORDER lineage"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximum -Pattern 'PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER' -Description "pipeline CP322 SOURCE_ORDER validation"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSerialization -Pattern 'mod snapshot;' -Description "pipeline CP322 snapshot serializer declaration"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSerialization -Pattern 'pub\(in crate::pipeline\) fn lifecycle_json\s*\(' -Description "pipeline CP322 lifecycle JSON serializer"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSerialization -Pattern '"latest": state\.latest\.map\(snapshot_json\)' -Description "pipeline CP322 latest snapshot serializer wiring"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotSerialization -Pattern 'pub\(super\) fn snapshot_json\s*\(' -Description "pipeline CP322 snapshot JSON serializer"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotSerialization -Pattern '"source_order": snapshot\.source_order' -Description "pipeline CP322 SOURCE_ORDER JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotSerialization -Pattern '"resulting_supply_mass_flow_rate_kg_per_s"' -Description "pipeline CP322 resulting flow JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotSerialization -Pattern '"resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"' -Description "pipeline CP322 bit-exact resulting flow JSON evidence"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotSerialization -Pattern 'fn ieee_bits\(value: Option<f64>\) -> Option<String>' -Description "pipeline CP322 IEEE-bit JSON helper"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumValidation -Pattern 'mod snapshot;' -Description "pipeline CP322 snapshot validator declaration"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumValidation -Pattern 'validate_source_counters' -Description "pipeline CP322 source-counter validator"
Assert-Contains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotValidation -Pattern 'left\.to_bits\(\) == right\.to_bits\(\)' -Description "pipeline CP322 bitwise floating validation"
Assert-NotContains -Path $runPurchasedAirCoolingSupplyMassFlowMaximumSnapshotValidation -Pattern '\.is_finite\(\)' -Description "pipeline CP322 finite-only operand filter"
