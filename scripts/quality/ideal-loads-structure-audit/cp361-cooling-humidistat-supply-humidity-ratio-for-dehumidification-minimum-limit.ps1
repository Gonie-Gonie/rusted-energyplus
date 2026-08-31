# CP361 maps PurchasedAirManager.cc line 2231 local Humidistat minimum limit.
$cp361Stem = "cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit"
$cp360StemForCp361 = "cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment"
$cp361PipelineStem = "purchased_air_$cp361Stem"
$cp361TypeStem = "PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimit"
$cp361Lifecycle = "purchased_air_calc_${cp361Stem}_lifecycle"
$cp361SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp361SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp361Sites = @(
    "read-local-supply-humidity-ratio-for-dehumidification-for-humidistat-minimum-limit-maximum",
    "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-humidistat-minimum-limit-maximum",
    "apply-source-shaped-two-argument-maximum-for-humidistat-minimum-limit",
    "assign-local-supply-humidity-ratio-for-dehumidification-for-humidistat-minimum-limit"
)

$cp361Module = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem.rs"
$cp361State = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\state.rs"
$cp361Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\transition.rs"
$cp361Predecessor = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\transition\predecessor.rs"
$cp361Release = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\release.rs"
$cp361Error = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\release\error.rs"
$cp361Operand = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\release\operand_validation.rs"
$cp361Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\release\prefix_validation.rs"
$cp361Private = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\release\private_counterfactual.rs"
$cp361Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\release\runtime_validation.rs"
$cp361Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\release\snapshot_validation.rs"
$cp361SnapshotRoute = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\release\snapshot_validation\route.rs"
$cp361TestsRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\tests\mod.rs"
$cp361TestsRoutes = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\tests\routes.rs"
$cp361TestsIeee = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\tests\ieee.rs"
$cp361TestsOverflow = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\tests\overflow.rs"
$cp361TestsRelease = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\tests\release.rs"
$cp361TestsCorruption = "crates\ep_runtime\src\ideal_loads\calc\$cp361Stem\tests\release_corruption.rs"
$cp361CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp361Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp361BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp361Stem.rs"
$cp361BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp361Stem}_tests.rs"
$cp361BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp361ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp361InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp361InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp361InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp361InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp361Stem.rs"
$cp361CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp361Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp361Stem}_validation.rs"
$cp361CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp361Stem}_validation\snapshot_validation.rs"
$cp361CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp361CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp361.rs"
$cp361FixtureHarness = "crates\ep_runtime\src\ideal_loads\coupled_output.rs"
$cp361FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp361Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp361Stem}_fixture.rs"
$cp361PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp361Pipeline = "crates\ep_run\src\pipeline\$cp361PipelineStem.rs"
$cp361PipelineValidation = "crates\ep_run\src\pipeline\$cp361PipelineStem\validation.rs"
$cp361PipelineTests = "crates\ep_run\src\pipeline\$cp361PipelineStem\validation\tests.rs"
$cp361Serialization = "crates\ep_run\src\pipeline\$cp361PipelineStem\serialization.rs"
$cp361SnapshotSerialization = "crates\ep_run\src\pipeline\$cp361PipelineStem\serialization\snapshot.rs"
$cp361ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp360_assertions.rs"
$cp361ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp361_assertions.rs"
$cp361Audit = "scripts\quality\ideal-loads-structure-audit\cp361-cooling-humidistat-supply-humidity-ratio-for-dehumidification-minimum-limit.ps1"

function Get-Cp361RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) {
        throw "$Description expected one anchor, found $($anchors.Count)"
    }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) {
        throw "$Description has no opening brace"
    }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") {
            $depth += 1
        } elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1)
            }
        }
    }
    throw "$Description has no complete brace block"
}

function Assert-Cp361ProductionHasNoPanics {
    param([string]$Path)
    $text = Read-RepoText -Path $Path
    $tests = [regex]::Match($text, '(?m)^\s*#\[cfg\(test\)\]\s*\r?\n\s*mod\s+tests\b')
    $production = if ($tests.Success) { $text.Substring(0, $tests.Index) } else { $text }
    if ($production -match '(?m)\b(?:expect|unwrap)\s*\(|panic!\s*\(') {
        throw "CP361 production path contains expect/unwrap/panic: $Path"
    }
}

foreach ($required in @(
        $cp361Module, $cp361State, $cp361Transition, $cp361Predecessor,
        $cp361Release, $cp361Error, $cp361Operand, $cp361Prefix, $cp361Private,
        $cp361Runtime, $cp361Snapshot, $cp361SnapshotRoute, $cp361TestsRoot,
        $cp361TestsRoutes, $cp361TestsIeee, $cp361TestsOverflow,
        $cp361TestsRelease, $cp361TestsCorruption, $cp361BindingAdapter,
        $cp361BindingTests, $cp361InitWitness, $cp361Coupled,
        $cp361CoupledSnapshot, $cp361CoupledTestsRoot, $cp361CoupledTests,
        $cp361FixtureHarness, $cp361Fixture, $cp361Pipeline,
        $cp361PipelineValidation, $cp361PipelineTests, $cp361Serialization,
        $cp361SnapshotSerialization, $cp361ParentAssertions,
        $cp361ArbitraryAssertions, $cp361Audit
    )) {
    Assert-FileExists -Path $required -Description "CP361 structure"
}
foreach ($limited in @(
        $cp361State, $cp361Transition, $cp361Predecessor, $cp361Release,
        $cp361Error, $cp361Operand, $cp361Prefix, $cp361Private, $cp361Runtime,
        $cp361Snapshot, $cp361SnapshotRoute, $cp361TestsRoutes, $cp361TestsIeee,
        $cp361TestsOverflow, $cp361TestsRelease, $cp361TestsCorruption,
        $cp361Coupled, $cp361CoupledSnapshot, $cp361CoupledTests,
        $cp361PipelineValidation,
        $cp361PipelineTests, $cp361Serialization, $cp361SnapshotSerialization,
        $cp361ArbitraryAssertions, $cp361Audit
    )) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP361 bounded structure"
}
foreach ($production in @(
        $cp361State, $cp361Transition, $cp361Predecessor, $cp361Release,
        $cp361Error, $cp361Operand, $cp361Prefix, $cp361Private, $cp361Runtime,
        $cp361Snapshot, $cp361SnapshotRoute, $cp361BindingAdapter, $cp361Coupled,
        $cp361CoupledSnapshot, $cp361Pipeline, $cp361PipelineValidation,
        $cp361Serialization, $cp361SnapshotSerialization
    )) {
    Assert-Cp361ProductionHasNoPanics -Path $production
}

# Exact source boundary, routes, source sites, and typed-owner arithmetic.
Assert-Contains -Path $cp361Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2231' -Description "CP361 source line"
Assert-Contains -Path $cp361Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2232' -Description "CP361 first excluded executable"
Assert-ExactStringArray -Path $cp361Module -Name "PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER" -Expected $cp361Sites -Description "CP361 four-site source order"
Assert-Contains -Path $cp361State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,\s*DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP361 seven routes"
foreach ($counter in @(
        "dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count",
        "source_site_execution_count",
        "supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count",
        "minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "supply_humidity_ratio_for_dehumidification_assignment_count"
    )) {
    Assert-Contains -Path $cp361State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP361 counter '$counter'"
}
foreach ($field in @(
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification",
        "supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read",
        "supply_humidity_ratio_for_dehumidification_before_minimum_limit",
        "minimum_cooling_supply_air_humidity_ratio_for_maximum_read",
        "minimum_cooling_supply_air_humidity_ratio",
        "source_shaped_two_argument_maximum_evaluated",
        "maximum_supply_humidity_ratio_for_dehumidification",
        "supply_humidity_ratio_for_dehumidification_assignment_performed",
        "assigned_supply_humidity_ratio_for_dehumidification",
        "resulting_supply_humidity_ratio_for_dehumidification"
    )) {
    Assert-Contains -Path $cp361Module -Pattern ('pub ' + $field + ':') -Description "CP361 snapshot field '$field'"
}
Assert-Contains -Path $cp361Transition -Pattern '(?s)DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted\s*=>.*?source_site_execution_count\s*\+=.*?SOURCE_ORDER\.len\(\).*?supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count\s*\+=\s*1;.*?minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count\s*\+=\s*1;.*?source_shaped_two_argument_maximum_evaluation_count\s*\+=\s*1;.*?supply_humidity_ratio_for_dehumidification_assignment_count\s*\+=\s*1;' -Description "CP361 H four-site counters"
Assert-Contains -Path $cp361Transition -Pattern 'source_shaped_two_argument_maximum\(left, right\)' -Description "CP361 source-shaped maximum"
Assert-NotContains -Path $cp361Transition -Pattern 'source_shaped_two_argument_maximum\(right, left\)|f64::max|\.max\(|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand' -Description "CP361 operand order and numerical firewall"
Assert-Contains -Path $cp361Runtime -Pattern '(?s)h\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "CP361 checked 4H"
Assert-Contains -Path $cp361Transition -Pattern 'PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Predecessor' -Description "CP360 immediate predecessor type"
Assert-Contains -Path $cp361Predecessor -Pattern 'PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER' -Description "CP360 predecessor provenance"
Assert-Contains -Path $cp361Prefix -Pattern 'cp360_private_humidistat_counterfactual_links_to_direct_release' -Description "CP360 private lineage"
Assert-Contains -Path $cp361Private -Pattern 'cp360_private_humidistat_counterfactual_from_direct_release' -Description "CP360 private bridge"
Assert-Contains -Path $cp361Private -Pattern 'private_cp360\.resulting_supply_humidity_ratio_for_dehumidification' -Description "CP360 left owner"
Assert-Contains -Path $cp361Operand -Pattern 'system\.minimum_cooling_supply_air_humidity_ratio' -Description "selected typed right owner"
Assert-Contains -Path $cp361Operand -Pattern 'minimum\.is_finite\(\)\.then_some\(minimum\)' -Description "private finite right-owner gate"
Assert-NotContains -Path $cp361Operand -Pattern 'PurchasedAirSizedLimits|sizing|CP319|cp319|CP329|cp329|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply' -Description "CP361 excluded right owners"

foreach ($test in @(
        [PSCustomObject]@{ Path = $cp361TestsRoutes; Pattern = 'source_boundary_four_sites_and_seven_routes_are_exact' },
        [PSCustomObject]@{ Path = $cp361TestsRoutes; Pattern = 'active_operand_contract_and_predecessor_shape_are_transactional' },
        [PSCustomObject]@{ Path = $cp361TestsIeee; Pattern = 'source_shaped_maximum_is_left_biased_for_ties_zeroes_and_nan' },
        [PSCustomObject]@{ Path = $cp361TestsIeee; Pattern = 'left_nan_payload_mismatch_is_rejected_without_mutation' },
        [PSCustomObject]@{ Path = $cp361TestsOverflow; Pattern = 'every_counter_overflow_rejects_without_mutation' },
        [PSCustomObject]@{ Path = $cp361TestsOverflow; Pattern = 'four_site_increment_preflight_rejects_max_minus_three' },
        [PSCustomObject]@{ Path = $cp361TestsRelease; Pattern = 'public_direct_routes_are_complete_null_and_do_not_validate_rhs' },
        [PSCustomObject]@{ Path = $cp361TestsRelease; Pattern = 'private_h_reuses_cp360_bridge_and_selected_typed_minimum' },
        [PSCustomObject]@{ Path = $cp361TestsCorruption; Pattern = 'corruption_replay_and_witness_redistribution_reject_transactionally' },
        [PSCustomObject]@{ Path = $cp361TestsCorruption; Pattern = 'private_typed_owner_gate_is_finite_only' },
        [PSCustomObject]@{ Path = $cp361TestsCorruption; Pattern = 'private_numeric_corruption_and_release_overflow_reject' },
        [PSCustomObject]@{ Path = $cp361BindingTests; Pattern = 'scheduled_binding_places_cp361_after_cp360_as_an_exact_null_c0_skip' },
        [PSCustomObject]@{ Path = $cp361BindingTests; Pattern = 'scheduled_binding_preserves_cp361_u_n_and_p_skips' },
        [PSCustomObject]@{ Path = $cp361Coupled; Pattern = 'partition_overflow_and_four_site_counter_corruption_fail_closed' },
        [PSCustomObject]@{ Path = $cp361CoupledTests; Pattern = 'cp361_coupled_direct_none_is_exact_null_skip_and_cp345_stays_owner' },
        [PSCustomObject]@{ Path = $cp361SnapshotSerialization; Pattern = 'direct_none_release_serializes_six_null_values_and_bits' },
        [PSCustomObject]@{ Path = $cp361SnapshotSerialization; Pattern = 'finite_and_nonfinite_characterization_preserves_exact_bits' },
        [PSCustomObject]@{ Path = $cp361PipelineTests; Pattern = 'four_site_counters_are_exact_and_fail_closed_on_each_mismatch' },
        [PSCustomObject]@{ Path = $cp361PipelineTests; Pattern = 'direct_expected_snapshot_is_complete_null_and_exact_bit_comparison_is_strict' },
        [PSCustomObject]@{ Path = $cp361PipelineTests; Pattern = 'predecessor_numeric_bits_are_preserved_in_expected_snapshot' }
    )) {
    Assert-Contains -Path $test.Path -Pattern $test.Pattern -Description "CP361 regression '$($test.Pattern)'"
}

# Direct admission, binding/pipeline placement, JSON, and numerical nonfeed.
$cp361ReleaseText = Read-RepoText -Path $cp361Release
$cp361PublicRelease = Get-Cp361RustBraceBlock -Text $cp361ReleaseText -AnchorPattern '(?m)^pub fn advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\s*\(' -Description "CP361 public release"
if ($cp361PublicRelease -notmatch 'system:\s*&IdealLoadsAirSystem' -or $cp361PublicRelease -notmatch 'predecessor_cp360:\s*Predecessor') {
    throw "CP361 public direct release must accept selected system and CP360 only"
}
if ($cp361PublicRelease -match 'minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner|system\.minimum_cooling_supply_air_humidity_ratio|:\s*f64') {
    throw "CP361 public direct release must not read, validate, or accept the unexecuted right operand"
}
if ($cp361PublicRelease -notmatch 'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None' -or $cp361PublicRelease -notmatch '(?s)advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_state\(.*?None') {
    throw "CP361 public direct release must be a complete-null None-selector skip"
}
$cp361BindingText = Read-RepoText -Path $cp361Binding
$cp360BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =")
$cp361BindingIndex = $cp361BindingText.IndexOf("let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =")
$cp362BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =")
$cp363BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_humidistat_case_break =")
$cp364BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_entry =")
$cp365BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_assignment =")
$cp366BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_break =")
$cp367BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =")
$cp368BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_case_break =")
$cp369BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp361 = $cp361BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp361NumericalIndex = $cp361BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp360BindingIndexForCp361 -lt 0 -or $cp361BindingIndex -le $cp360BindingIndexForCp361 -or $cp362BindingIndexForCp361 -le $cp361BindingIndex -or $cp363BindingIndexForCp361 -le $cp362BindingIndexForCp361 -or $cp364BindingIndexForCp361 -le $cp363BindingIndexForCp361 -or $cp365BindingIndexForCp361 -le $cp364BindingIndexForCp361 -or $cp366BindingIndexForCp361 -le $cp365BindingIndexForCp361 -or $cp367BindingIndexForCp361 -le $cp366BindingIndexForCp361 -or $cp368BindingIndexForCp361 -le $cp367BindingIndexForCp361 -or
    $cp361NumericalIndex -le $cp368BindingIndexForCp361 -or
    $cp369BindingIndexForCp361 -le $cp368BindingIndexForCp361 -or
    $cp370BindingIndexForCp361 -le $cp369BindingIndexForCp361 -or
    $cp371BindingIndexForCp361 -le $cp370BindingIndexForCp361 -or
    $cp372BindingIndexForCp361 -le $cp371BindingIndexForCp361 -or
    $cp373BindingIndexForCp361 -le $cp372BindingIndexForCp361 -or
    $cp374BindingIndexForCp361 -le $cp373BindingIndexForCp361 -or
    $cp375BindingIndexForCp361 -le $cp374BindingIndexForCp361 -or
    $cp376BindingIndexForCp361 -le $cp375BindingIndexForCp361 -or $cp377BindingIndexForCp361 -le $cp376BindingIndexForCp361 -or $cp378BindingIndexForCp361 -le $cp377BindingIndexForCp361 -or $cp379BindingIndexForCp361 -le $cp378BindingIndexForCp361 -or $cp380BindingIndexForCp361 -le $cp379BindingIndexForCp361 -or $cp381BindingIndexForCp361 -le $cp380BindingIndexForCp361 -or $cp382BindingIndexForCp361 -le $cp381BindingIndexForCp361 -or $cp383BindingIndexForCp361 -le $cp382BindingIndexForCp361 -or $cp384BindingIndexForCp361 -le $cp383BindingIndexForCp361 -or $cp385BindingIndexForCp361 -le $cp384BindingIndexForCp361 -or $cp361NumericalIndex -le $cp385BindingIndexForCp361) {
    throw "Binding must execute CP360 through CP370 before numerical coupling"
}
$cp361Dto = Get-Cp361RustBraceBlock -Text $cp361BindingText.Substring($cp361NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP361 numerical DTO"
if ($cp361Dto -match '(?i)cp361|minimum_limit|supply_humidity_ratio_for_dehumidification') {
    throw "CP361 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp361CalcRoot -Pattern ('mod ' + [regex]::Escape($cp361Stem) + ';') -Description "CP361 calc module"
Assert-Contains -Path $cp361BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp361Stem)) -Description "CP361 binding adapter"
Assert-Contains -Path $cp361ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp361Stem) + ':') -Description "CP361 scheduled output"
Assert-Contains -Path $cp361BindingTestsRoot -Pattern ([regex]::Escape("${cp361Stem}_tests.rs")) -Description "CP361 binding tests"
Assert-Contains -Path $cp361InitState -Pattern $cp361Stem -Description "CP361 init state"
Assert-Contains -Path $cp361InitUnit -Pattern $cp361Stem -Description "CP361 unit state"
Assert-Contains -Path $cp361InitWitnessRoot -Pattern $cp361Stem -Description "CP361 witness module"
Assert-Contains -Path $cp361CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp361Stem) + '_validation;') -Description "CP361 coupled validator"
Assert-Contains -Path $cp361CoupledRoot -Pattern 'path = "coupled_runtime_tests\.rs"' -Description "coupled regression harness"
Assert-Contains -Path $cp361Coupled -Pattern 'PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState' -Description "coupled CP360 predecessor"
Assert-Contains -Path $cp361CoupledTestsRoot -Pattern 'coupled_runtime_tests_cp361\.rs' -Description "CP361 coupled regression registration"
Assert-Contains -Path $cp361FixtureHarness -Pattern 'path = "coupled_output_tests\.rs"' -Description "coupled-output fixture harness"
Assert-Contains -Path $cp361FixtureRoot -Pattern $cp361Stem -Description "CP361 fixture registration"
Assert-Contains -Path $cp361Fixture -Pattern ('calculation_' + [regex]::Escape($cp361Stem) + '_snapshot') -Description "CP361 output fixture"
Assert-Contains -Path $cp361PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp361PipelineStem) + ';') -Description "CP361 pipeline module"
Assert-Contains -Path $cp361PipelineRoot -Pattern ('"' + $cp361Lifecycle + '":\s*result\s*\.' + $cp361Lifecycle) -Description "CP361 lifecycle JSON"
Assert-Contains -Path $cp361PipelineValidation -Pattern 'supply_humidity_ratio_assignment_cp360' -Description "pipeline CP360 predecessor"
Assert-Contains -Path $cp361PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp361ParentAssertions -Pattern 'mod cp361_assertions;' -Description "arbitrary CP361 delegation"
Assert-Contains -Path $cp361ParentAssertions -Pattern 'cp361_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP361 direct delegation"
Assert-Contains -Path $cp361ParentAssertions -Pattern 'cp361_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP361 non-direct delegation"
Assert-Contains -Path $cp361ArbitraryAssertions -Pattern 'CP361_KEY' -Description "arbitrary CP361 lifecycle"
Assert-Contains -Path $cp361ArbitraryAssertions -Pattern 'mod cp362_assertions;' -Description "arbitrary CP362 delegation"
Assert-Contains -Path $cp361ArbitraryAssertions -Pattern 'cp362_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP362 direct delegation"
Assert-Contains -Path $cp361ArbitraryAssertions -Pattern 'cp362_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP362 non-direct delegation"
Assert-NotContains -Path $cp361ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP361 relinquishes numerical nonfeed ownership to CP362"
Assert-Contains -Path $cp361SnapshotSerialization -Pattern 'json_number' -Description "CP361 finite JSON projection"
Assert-Contains -Path $cp361SnapshotSerialization -Pattern '_ieee_bits' -Description "CP361 authoritative IEEE sidecars"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$cp361AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp361CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp361AlgorithmAddenda = [regex]::Matches($cp361AlgorithmText, '(?m)^\s*"CP361 supersedes only CP360[^"\r\n]+",\s*$')
$cp361CapabilityAddenda = [regex]::Matches($cp361CapabilityText, '(?m)^\s*"CP361 additionally requires[^"\r\n]+",\s*$')
if ($cp361AlgorithmAddenda.Count -ne 2 -or $cp361CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP361 addenda"
}
foreach ($claim in @($cp361AlgorithmAddenda) + @($cp361CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp361SourceCommit, $cp361SourceHash, 'physical executable line 2231',
            'SupplyHumRatForDehum = max\(SupplyHumRatForDehum, PurchAir\.MinCoolSuppAirHumRat\);',
            'physical executable line 2232',
            'PurchAir\.SupplyHumRat = min\(PurchAir\.MixedAirHumRat, SupplyHumRatForDehum\);',
            $cp361Sites[0], $cp361Sites[1], $cp361Sites[2], $cp361Sites[3],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=4H', 'C0=S', 'Q=H=CSH=0',
            'CP360', 'resulting_supply_humidity_ratio_for_dehumidification',
            'minimum_cooling_supply_air_humidity_ratio', '\.is_finite\(\)',
            'pure transition/source statement', 'if left < right \{ right \} else \{ left \}',
            'f64::max', 'CP360-to-CP361-to-unchanged-numerical', $cp361Lifecycle,
            'does not assign `PurchAir\.SupplyHumRat`', 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '299 total', '240 public', '59 internal',
            'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP361 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp361Stem/release\.rs::advance_direct_no_oa_calc_$cp361Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp361Stem\.rs::purchased_air_calc_${cp361Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp361Stem\.rs::${cp361TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp361Stem\.rs::${cp361TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp361AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP361 target count failed for '$($target.Pattern)'"
    }
}
$cp361Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP361 Cooling Humidistat Supply-Humidity-Ratio Minimum Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP361 Source-Ordered Cooling Humidistat Minimum Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP361 Humidistat Supply-Humidity-Ratio Minimum Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP361 Humidistat Minimum Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP361 Humidistat Minimum-Limit Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp361Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP361 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp361SourceCommit, $cp361SourceHash, '2231', 'SupplyHumRatForDehum',
            '2232', 'PurchAir\.MixedAirHumRat', $cp361Sites[0], $cp361Sites[1],
            $cp361Sites[2], $cp361Sites[3], 'T=U\+N\+P\+C0\+Q\+H\+CSH',
            'source_site_execution_count\s*=\s*4H', 'C0\s*=\s*S',
            'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP360', 'left',
            'minimum_cooling_supply_air_humidity_ratio', '\.is_finite\(\)',
            'transition/source statement', 'if left < right',
            'CP360-to-CP361-to-unchanged-numerical', $cp361Lifecycle,
            'PurchAir\.SupplyHumRat', 'CP345', '32\s+algorithms', '293\s+routines',
            '58\s+[^,\r\n]*state[_-]mapped', '235\s+[^,\r\n]*source[_-]mapped',
            '170\s+required', '299\s+total', '240\s+public', '59\s+internal',
            'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP361 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP361\b' -Description "CP361 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP361 supersedes only CP360' -Description "generated CP361 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP361 additionally requires' -Description "generated CP361 capability addendum"

# Historical current-state expectations and master/inventory reachability.
$cp361BindingHistory = @(326) + @(329..360)
foreach ($number in $cp361BindingHistory) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit' -Description "historical CP361 binding/order/whitelist"
}
foreach ($number in 334..360) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description "historical CP363 firewall"
}
foreach ($number in 335..360) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 373 \|')) -Description "historical current generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 133 \|')) -Description "historical current generated internal"
}
foreach ($number in 337..360) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 373' -Description "historical current script inventory total"
}
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp360-cooling-humidistat-supply-humidity-ratio-for-dehumidification-assignment.ps1" -Pattern '298 total' -Description "CP360 historical script claim"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp360-cooling-humidistat-supply-humidity-ratio-for-dehumidification-assignment.ps1" -Pattern '58 internal' -Description "CP360 historical internal claim"
$cp361MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp360AuditIndexForCp361 = $cp361MainAuditText.IndexOf("cp360-cooling-humidistat-supply-humidity-ratio-for-dehumidification-assignment.ps1")
$cp361AuditIndex = $cp361MainAuditText.IndexOf("cp361-cooling-humidistat-supply-humidity-ratio-for-dehumidification-minimum-limit.ps1")
$cp361CompletionIndex = $cp361MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp360AuditIndexForCp361 -lt 0 -or $cp361AuditIndex -le $cp360AuditIndexForCp361 -or $cp361CompletionIndex -le $cp361AuditIndex) {
    throw "Master audit must dot-source CP361 after CP360 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 373' -Description "CP361 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP361 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp361-' -Description "CP361 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp361-cooling-humidistat-supply-humidity-ratio-for-dehumidification-minimum-limit\.ps1::dot_sources' -Description "CP361 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 373 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 133 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP361 Humidistat supply-humidity-ratio-for-dehumidification minimum-limit structure audit passed."
