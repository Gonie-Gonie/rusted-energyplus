# CP360 maps PurchasedAirManager.cc line 2230 local Humidistat humidity-ratio assignment.
$cp360Stem = "cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment"
$cp359StemForCp360 = "cooling_humidistat_moisture_demand_assignment"
$cp360PipelineStem = "purchased_air_$cp360Stem"
$cp360TypeStem = "PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignment"
$cp360Lifecycle = "purchased_air_calc_${cp360Stem}_lifecycle"
$cp360SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp360Sites = @(
    "read-local-zone-dehumidifying-setpoint-moisture-demand-for-supply-humidity-ratio-division",
    "read-retained-supply-mass-flow-rate-for-supply-humidity-ratio-division",
    "calculate-zone-dehumidifying-setpoint-moisture-demand-divided-by-supply-mass-flow-rate",
    "read-zone-node-humidity-ratio-for-dehumidification-supply-humidity-ratio",
    "add-zone-node-humidity-ratio-to-moisture-demand-derived-supply-humidity-ratio",
    "assign-local-supply-humidity-ratio-for-dehumidification"
)

$cp360Module = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem.rs"
$cp360State = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\state.rs"
$cp360Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\transition.rs"
$cp360Predecessor = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\transition\predecessor.rs"
$cp360Release = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\release.rs"
$cp360Error = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\release\error.rs"
$cp360Operand = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\release\operand_validation.rs"
$cp360Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\release\prefix_validation.rs"
$cp360Private = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\release\private_counterfactual.rs"
$cp360Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\release\runtime_validation.rs"
$cp360Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\release\snapshot_validation.rs"
$cp360SnapshotRoute = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\release\snapshot_validation\route.rs"
$cp360TestsRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\tests\mod.rs"
$cp360TestsRoutes = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\tests\routes.rs"
$cp360TestsIeee = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\tests\ieee.rs"
$cp360TestsOverflow = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\tests\overflow.rs"
$cp360TestsRelease = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\tests\release.rs"
$cp360TestsCorruption = "crates\ep_runtime\src\ideal_loads\calc\$cp360Stem\tests\release_corruption.rs"
$cp360CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp360Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp360BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp360Stem.rs"
$cp360BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp360Stem}_tests.rs"
$cp360BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp360ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp360InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp360InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp360InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp360InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp360Stem.rs"
$cp360CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp360Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp360Stem}_validation.rs"
$cp360CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp360Stem}_validation\snapshot_validation.rs"
$cp360FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp360Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp360Stem}_fixture.rs"
$cp360PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp360Pipeline = "crates\ep_run\src\pipeline\$cp360PipelineStem.rs"
$cp360PipelineValidation = "crates\ep_run\src\pipeline\$cp360PipelineStem\validation.rs"
$cp360PipelineTests = "crates\ep_run\src\pipeline\$cp360PipelineStem\validation\tests.rs"
$cp360Serialization = "crates\ep_run\src\pipeline\$cp360PipelineStem\serialization.rs"
$cp360SnapshotSerialization = "crates\ep_run\src\pipeline\$cp360PipelineStem\serialization\snapshot.rs"
$cp360ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp359_assertions.rs"
$cp360ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp360_assertions.rs"
$cp360Audit = "scripts\quality\ideal-loads-structure-audit\cp360-cooling-humidistat-supply-humidity-ratio-for-dehumidification-assignment.ps1"

function Get-Cp360RustBraceBlock {
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

function Assert-Cp360ProductionHasNoPanics {
    param([string]$Path)
    $text = Read-RepoText -Path $Path
    $tests = [regex]::Match($text, '(?m)^\s*#\[cfg\(test\)\]\s*\r?\n\s*mod\s+tests\b')
    $production = if ($tests.Success) { $text.Substring(0, $tests.Index) } else { $text }
    if ($production -match '(?m)\b(?:expect|unwrap)\s*\(|panic!\s*\(') {
        throw "CP360 production path contains expect/unwrap/panic: $Path"
    }
}

foreach ($required in @(
        $cp360Module, $cp360State, $cp360Transition, $cp360Predecessor,
        $cp360Release, $cp360Error, $cp360Operand, $cp360Prefix, $cp360Private, $cp360Runtime,
        $cp360Snapshot, $cp360SnapshotRoute, $cp360TestsRoot, $cp360TestsRoutes,
        $cp360TestsIeee, $cp360TestsOverflow, $cp360TestsRelease,
        $cp360TestsCorruption, $cp360BindingAdapter, $cp360BindingTests,
        $cp360InitWitness, $cp360Coupled, $cp360CoupledSnapshot, $cp360Fixture,
        $cp360Pipeline, $cp360PipelineValidation, $cp360PipelineTests,
        $cp360Serialization, $cp360SnapshotSerialization, $cp360ParentAssertions,
        $cp360ArbitraryAssertions, $cp360Audit
    )) {
    Assert-FileExists -Path $required -Description "CP360 structure"
}
foreach ($limited in @(
        $cp360State, $cp360Transition, $cp360Predecessor, $cp360Release, $cp360Error,
        $cp360Operand, $cp360Prefix, $cp360Private, $cp360Runtime,
        $cp360Snapshot, $cp360SnapshotRoute, $cp360TestsRoutes, $cp360TestsIeee,
        $cp360TestsOverflow, $cp360TestsRelease, $cp360TestsCorruption,
        $cp360Coupled, $cp360CoupledSnapshot, $cp360PipelineValidation,
        $cp360PipelineTests, $cp360Serialization, $cp360SnapshotSerialization,
        $cp360ArbitraryAssertions, $cp360Audit
    )) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP360 bounded structure"
}
foreach ($production in @(
        $cp360State, $cp360Transition, $cp360Predecessor, $cp360Release, $cp360Error,
        $cp360Operand, $cp360Prefix, $cp360Private, $cp360Runtime,
        $cp360Snapshot, $cp360SnapshotRoute, $cp360BindingAdapter, $cp360Coupled,
        $cp360CoupledSnapshot, $cp360Pipeline, $cp360PipelineValidation,
        $cp360Serialization, $cp360SnapshotSerialization
    )) {
    Assert-Cp360ProductionHasNoPanics -Path $production
}

# Exact source boundary, routes, six H sites, grouping, and owner contracts.
Assert-Contains -Path $cp360Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2230' -Description "CP360 source line"
Assert-Contains -Path $cp360Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2231' -Description "CP360 first excluded executable"
Assert-ExactStringArray -Path $cp360Module -Name "PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER" -Expected $cp360Sites -Description "CP360 six-site source order"
Assert-Contains -Path $cp360State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,\s*DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP360 seven routes"
foreach ($counter in @(
        "dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count",
        "source_site_execution_count",
        "zone_dehumidifying_setpoint_moisture_demand_read_count",
        "supply_mass_flow_rate_read_count",
        "moisture_demand_derived_supply_humidity_ratio_calculation_count",
        "zone_node_humidity_ratio_read_count",
        "supply_humidity_ratio_for_dehumidification_calculation_count",
        "supply_humidity_ratio_for_dehumidification_assignment_count"
    )) {
    Assert-Contains -Path $cp360State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP360 counter '$counter'"
}
Assert-Contains -Path $cp360Transition -Pattern '(?s)DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted\s*=>.*?source_site_execution_count\s*\+=.*?SOURCE_ORDER\.len\(\).*?zone_dehumidifying_setpoint_moisture_demand_read_count\s*\+=\s*1;.*?supply_mass_flow_rate_read_count\s*\+=\s*1;.*?moisture_demand_derived_supply_humidity_ratio_calculation_count\s*\+=\s*1;.*?zone_node_humidity_ratio_read_count\s*\+=\s*1;.*?supply_humidity_ratio_for_dehumidification_calculation_count\s*\+=\s*1;.*?supply_humidity_ratio_for_dehumidification_assignment_count\s*\+=\s*1;' -Description "CP360 H six-site counters"
Assert-Contains -Path $cp360Transition -Pattern 'let quotient = demand / operands\.supply_mass_flow_rate_kg_per_s;' -Description "CP360 raw division"
Assert-Contains -Path $cp360Transition -Pattern 'let calculated = quotient \+ operands\.zone_node_humidity_ratio;' -Description "CP360 staged addition"
Assert-NotContains -Path $cp360Transition -Pattern 'mul_add|recip|\.is_finite\(\)|\.clamp\(|f64::min|f64::max|Psy[A-Za-z_]*|DirectZonePurchasedAirCouplingInput' -Description "CP360 arithmetic/service firewall"
Assert-Contains -Path $cp360Operand -Pattern 'cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release' -Description "CP330 exact owner proof"
Assert-Contains -Path $cp360Operand -Pattern 'completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent' -Description "CP330 recursive owner proof"
Assert-Contains -Path $cp360Operand -Pattern '(?s)flow\.partial_cmp\(&0\.0\),\s*Some\(std::cmp::Ordering::Greater\)' -Description "CP330 positive denominator"
Assert-NotContains -Path $cp360Operand -Pattern 'cooling_mixed_air_call|recirculation|ZoneNodeNum|humidity|\.is_finite\(' -Description "CP329 recirculation non-owner and no finite gate"
Assert-Contains -Path $cp360Private -Pattern 'cp359_private_humidistat_counterfactual_from_direct_release' -Description "CP359 numerator bridge"
Assert-Contains -Path $cp360Private -Pattern 'pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s:\s*f64' -Description "explicit numerator characterization"
Assert-Contains -Path $cp360Private -Pattern 'pre_sampled_zone_node_humidity_ratio:\s*f64' -Description "explicit Zone-node humidity characterization"
Assert-Contains -Path $cp360Private -Pattern 'supply_mass_flow_rate_from_retained_owner' -Description "CP330-owned denominator use"
Assert-Contains -Path $cp360Private -Pattern 'CP329 recirculation humidity is not a Zone-node owner' -Description "CP329 correction contract"
Assert-NotContains -Path $cp360Private -Pattern 'DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|calc::humidity|\.is_finite\(\)|\.clamp\(|f64::min|f64::max' -Description "CP360 private nonfeed/noncoercion"
Assert-Contains -Path $cp360Runtime -Pattern '(?s)h\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "CP360 checked 6H"
Assert-Contains -Path $cp360Prefix -Pattern 'PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Predecessor' -Description "CP359 immediate predecessor"

foreach ($test in @(
        [PSCustomObject]@{ Path = $cp360TestsRoutes; Pattern = 'source_boundary_six_sites_and_seven_routes_are_exact' },
        [PSCustomObject]@{ Path = $cp360TestsRoutes; Pattern = 'active_operand_contract_and_predecessor_shape_are_transactional' },
        [PSCustomObject]@{ Path = $cp360TestsIeee; Pattern = 'division_then_addition_preserves_staged_binary64_evidence' },
        [PSCustomObject]@{ Path = $cp360TestsIeee; Pattern = 'predecessor_nan_payload_mismatch_is_rejected_without_mutation' },
        [PSCustomObject]@{ Path = $cp360TestsOverflow; Pattern = 'every_counter_overflow_rejects_without_mutation' },
        [PSCustomObject]@{ Path = $cp360TestsOverflow; Pattern = 'six_site_increment_preflight_rejects_max_minus_five' },
        [PSCustomObject]@{ Path = $cp360TestsRelease; Pattern = 'public_direct_routes_are_complete_null' },
        [PSCustomObject]@{ Path = $cp360TestsRelease; Pattern = 'private_h_is_parametric_except_for_cp330_owned_flow' },
        [PSCustomObject]@{ Path = $cp360TestsCorruption; Pattern = 'cp330_latest_witness_and_coordinated_flow_corruption_reject_private_h' },
        [PSCustomObject]@{ Path = $cp360BindingTests; Pattern = 'scheduled_binding_places_cp360_after_cp359_as_an_exact_null_c0_skip' },
        [PSCustomObject]@{ Path = $cp360BindingTests; Pattern = 'scheduled_binding_preserves_cp360_u_n_and_p_skips' },
        [PSCustomObject]@{ Path = $cp360Coupled; Pattern = 'partition_overflow_and_six_site_counter_corruption_fail_closed' },
        [PSCustomObject]@{ Path = $cp360SnapshotSerialization; Pattern = 'direct_none_release_serializes_null_numeric_values_and_bits' },
        [PSCustomObject]@{ Path = $cp360SnapshotSerialization; Pattern = 'finite_and_nonfinite_characterization_preserves_exact_bits' },
        [PSCustomObject]@{ Path = $cp360PipelineTests; Pattern = 'six_site_counters_are_exact_and_fail_closed_on_each_mismatch' },
        [PSCustomObject]@{ Path = $cp360PipelineTests; Pattern = 'direct_expected_snapshot_is_complete_null_and_exact_bit_comparison_is_strict' }
    )) {
    Assert-Contains -Path $test.Path -Pattern $test.Pattern -Description "CP360 regression '$($test.Pattern)'"
}

# Direct admission, binding/pipeline placement, JSON, and numerical nonfeed.
$cp360ReleaseText = Read-RepoText -Path $cp360Release
$cp360PublicRelease = Get-Cp360RustBraceBlock -Text $cp360ReleaseText -AnchorPattern '(?m)^pub fn advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\s*\(' -Description "CP360 public release"
if ($cp360PublicRelease -notmatch 'predecessor_cp359:\s*Predecessor' -or $cp360PublicRelease -match 'pre_sampled|zone_node_humidity_ratio:\s*f64|supply_mass_flow_rate_kg_per_s:\s*f64') {
    throw "CP360 public direct release must accept CP359 only and no numeric operands"
}
if ($cp360PublicRelease -notmatch 'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None' -or $cp360PublicRelease -notmatch '(?s)advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state\(.*?None') {
    throw "CP360 public direct release must be a complete-null None-selector skip"
}
if ($cp360PublicRelease -match 'DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply') {
    throw "CP360 public release admits a forbidden numerical/demand feed"
}
$cp360BindingText = Read-RepoText -Path $cp360Binding
$cp359BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
$cp360BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =")
$cp361BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =")
$cp362BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =")
$cp363BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_humidistat_case_break =")
$cp364BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_entry =")
$cp365BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_assignment =")
$cp366BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_break =")
$cp367BindingIndex = $cp360BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =")
$cp368BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_case_break =")
$cp369BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp360 = $cp360BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp360NumericalIndex = $cp360BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if (
    $cp359BindingIndex -lt 0 -or
    $cp360BindingIndex -le $cp359BindingIndex -or
    $cp361BindingIndex -le $cp360BindingIndex -or
    $cp362BindingIndex -le $cp361BindingIndex -or
    $cp363BindingIndex -le $cp362BindingIndex -or
    $cp364BindingIndex -le $cp363BindingIndex -or
    $cp365BindingIndex -le $cp364BindingIndex -or
    $cp366BindingIndex -le $cp365BindingIndex -or
    $cp367BindingIndex -le $cp366BindingIndex -or
    $cp368BindingIndexForCp360 -le $cp367BindingIndex -or
    $cp360NumericalIndex -le $cp368BindingIndexForCp360 -or
    $cp369BindingIndexForCp360 -le $cp368BindingIndexForCp360 -or
    $cp370BindingIndexForCp360 -le $cp369BindingIndexForCp360 -or
    $cp371BindingIndexForCp360 -le $cp370BindingIndexForCp360 -or
    $cp372BindingIndexForCp360 -le $cp371BindingIndexForCp360 -or
    $cp373BindingIndexForCp360 -le $cp372BindingIndexForCp360 -or
    $cp374BindingIndexForCp360 -le $cp373BindingIndexForCp360 -or
    $cp375BindingIndexForCp360 -le $cp374BindingIndexForCp360 -or
    $cp376BindingIndexForCp360 -le $cp375BindingIndexForCp360 -or $cp377BindingIndexForCp360 -le $cp376BindingIndexForCp360 -or $cp378BindingIndexForCp360 -le $cp377BindingIndexForCp360 -or $cp379BindingIndexForCp360 -le $cp378BindingIndexForCp360 -or $cp380BindingIndexForCp360 -le $cp379BindingIndexForCp360 -or $cp381BindingIndexForCp360 -le $cp380BindingIndexForCp360 -or $cp382BindingIndexForCp360 -le $cp381BindingIndexForCp360 -or $cp383BindingIndexForCp360 -le $cp382BindingIndexForCp360 -or $cp384BindingIndexForCp360 -le $cp383BindingIndexForCp360 -or $cp385BindingIndexForCp360 -le $cp384BindingIndexForCp360 -or $cp360NumericalIndex -le $cp385BindingIndexForCp360) {
    throw "Binding must execute CP359 through CP370 before numerical coupling"
}
$cp360Dto = Get-Cp360RustBraceBlock -Text $cp360BindingText.Substring($cp360NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP360 numerical DTO"
if ($cp360Dto -match '(?i)cp360|supply_humidity_ratio_for_dehumidification|zone_node_humidity_ratio') {
    throw "CP360 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp360CalcRoot -Pattern ('mod ' + [regex]::Escape($cp360Stem) + ';') -Description "CP360 calc module"
Assert-Contains -Path $cp360BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp360Stem)) -Description "CP360 binding adapter"
Assert-Contains -Path $cp360ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp360Stem) + ':') -Description "CP360 scheduled output"
Assert-Contains -Path $cp360BindingTestsRoot -Pattern ([regex]::Escape("${cp360Stem}_tests.rs")) -Description "CP360 binding tests"
Assert-Contains -Path $cp360InitState -Pattern $cp360Stem -Description "CP360 init state"
Assert-Contains -Path $cp360InitUnit -Pattern $cp360Stem -Description "CP360 unit state"
Assert-Contains -Path $cp360InitWitnessRoot -Pattern $cp360Stem -Description "CP360 witness module"
Assert-Contains -Path $cp360CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp360Stem) + '_validation;') -Description "CP360 coupled validator"
Assert-Contains -Path $cp360Coupled -Pattern 'PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState' -Description "coupled CP359 predecessor"
Assert-NotContains -Path $cp360Coupled -Pattern 'complete_direct_zone_purchased_air_coupling|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply' -Description "coupled CP360 numerical firewall"
Assert-Contains -Path $cp360FixtureRoot -Pattern $cp360Stem -Description "CP360 fixture registration"
Assert-Contains -Path $cp360Fixture -Pattern ('calculation_' + [regex]::Escape($cp360Stem) + '_snapshot') -Description "CP360 output fixture"
Assert-Contains -Path $cp360PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp360PipelineStem) + ';') -Description "CP360 pipeline module"
Assert-Contains -Path $cp360PipelineRoot -Pattern ('"' + $cp360Lifecycle + '":\s*result\s*\.' + $cp360Lifecycle) -Description "CP360 lifecycle JSON"
Assert-Contains -Path $cp360PipelineValidation -Pattern 'moisture_demand_assignment_cp359' -Description "pipeline CP359 predecessor"
Assert-Contains -Path $cp360PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp416_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp360ParentAssertions -Pattern 'mod cp360_assertions;' -Description "arbitrary CP360 delegation"
Assert-Contains -Path $cp360ParentAssertions -Pattern 'cp360_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP360 direct delegation"
Assert-Contains -Path $cp360ParentAssertions -Pattern 'cp360_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP360 non-direct delegation"
Assert-Contains -Path $cp360ArbitraryAssertions -Pattern 'CP360_KEY' -Description "arbitrary CP360 lifecycle"
Assert-Contains -Path $cp360ArbitraryAssertions -Pattern 'mod cp361_assertions;' -Description "arbitrary CP361 delegation"
Assert-Contains -Path $cp360ArbitraryAssertions -Pattern 'cp361_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP361 direct delegation"
Assert-Contains -Path $cp360ArbitraryAssertions -Pattern 'cp361_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP361 non-direct delegation"
Assert-NotContains -Path $cp360ArbitraryAssertions -Pattern 'super::super::super::assert_numerical_nonfeed\(runtime, results\)' -Description "CP345 numerical nonfeed ownership moves to CP361"
Assert-Contains -Path $cp360SnapshotSerialization -Pattern 'json_number' -Description "CP360 finite JSON projection"
Assert-Contains -Path $cp360SnapshotSerialization -Pattern '_ieee_bits' -Description "CP360 authoritative IEEE sidecars"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$cp360AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp360CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp360AlgorithmAddenda = [regex]::Matches($cp360AlgorithmText, '(?m)^\s*"CP360 supersedes only CP359[^"\r\n]+",\s*$')
$cp360CapabilityAddenda = [regex]::Matches($cp360CapabilityText, '(?m)^\s*"CP360 additionally requires[^"\r\n]+",\s*$')
if ($cp360AlgorithmAddenda.Count -ne 2 -or $cp360CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP360 addenda"
}
foreach ($claim in @($cp360AlgorithmAddenda) + @($cp360CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp360SourceHash, 'physical executable line 2230', 'SupplyHumRatForDehum',
            'physical executable line 2231', $cp360Sites[0], $cp360Sites[1],
            $cp360Sites[2], $cp360Sites[3], $cp360Sites[4], $cp360Sites[5],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=6H', 'C0=S', 'Q=H=CSH=0',
            'CP359 resulting local owns the numerator only within explicit parametric characterization',
            'CP330 positive', 'CP329 flow parity', 'second explicit pre-sampled',
            'recirculation node is not `ZoneNodeNum`', 'CP319', 'calc::humidity',
            'raw IEEE-754 division', 'CP359-to-CP360-to-unchanged-numerical',
            $cp360Lifecycle, 'does not assign `PurchAir\.SupplyHumRat`', 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '298 total', '240 public', '58 internal',
            'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP360 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp360Stem/release\.rs::advance_direct_no_oa_calc_$cp360Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp360Stem\.rs::purchased_air_calc_${cp360Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp360Stem\.rs::${cp360TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp360Stem\.rs::${cp360TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp360AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP360 target count failed for '$($target.Pattern)'"
    }
}
$cp360Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP360 Cooling Humidistat Supply-Humidity-Ratio-for-Dehumidification Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP360 Source-Ordered Cooling Humidistat Supply-Humidity-Ratio Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP360 Humidistat Supply-Humidity-Ratio-for-Dehumidification Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP360 Humidistat Supply-Humidity-Ratio Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP360 Humidistat Supply-Humidity-Ratio Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp360Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP360 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp360SourceHash, '2230', 'SupplyHumRatForDehum', '2231',
            $cp360Sites[0], $cp360Sites[1], $cp360Sites[2], $cp360Sites[3],
            $cp360Sites[4], $cp360Sites[5], 'U/N/P/C0/Q/H/CSH',
            'source_site_execution_count\s*=\s*6H', 'C0\s*=\s*S',
            'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP359', 'numerator',
            'CP330', 'denominator', 'CP329', 'ZoneNodeNum', 'second explicit',
            'no retained', 'live Node', 'CP319', 'calc::humidity',
            'division', 'addition', 'CP359-to-CP360-to-unchanged-numerical',
            $cp360Lifecycle, 'PurchAir\.SupplyHumRat', 'CP345',
            '32\s+algorithms', '293\s+routines', '58\s+[^,\r\n]*state[_-]mapped',
            '235\s+[^,\r\n]*source[_-]mapped', '170\s+required', '298\s+total',
            '240\s+public', '58\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP360 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP360\b' -Description "CP360 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP360 supersedes only CP359' -Description "generated CP360 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP360 additionally requires' -Description "generated CP360 capability addendum"

# Historical current-state expectations and master/inventory reachability.
$cp360BindingHistory = @(326) + @(329..360)
foreach ($number in $cp360BindingHistory) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment' -Description "historical CP360 binding/order/whitelist"
}
foreach ($number in 334..360) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp416_lifecycle_evidence' -Description "historical CP363 firewall"
}
foreach ($number in 335..360) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 354 \|')) -Description "historical current generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 114 \|')) -Description "historical current generated internal"
}
foreach ($number in 337..360) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 354' -Description "historical current script inventory total"
}
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp359-cooling-humidistat-moisture-demand-assignment.ps1" -Pattern '297 total' -Description "CP359 historical script claim"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp359-cooling-humidistat-moisture-demand-assignment.ps1" -Pattern '57 internal' -Description "CP359 historical internal claim"
Assert-Contains -Path $cp360Audit -Pattern '298 total' -Description "CP360 historical script claim"
Assert-Contains -Path $cp360Audit -Pattern '58 internal' -Description "CP360 historical internal claim"
$cp360MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp359AuditIndexForCp360 = $cp360MainAuditText.IndexOf("cp359-cooling-humidistat-moisture-demand-assignment.ps1")
$cp360AuditIndex = $cp360MainAuditText.IndexOf("cp360-cooling-humidistat-supply-humidity-ratio-for-dehumidification-assignment.ps1")
$cp360CompletionIndex = $cp360MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp359AuditIndexForCp360 -lt 0 -or $cp360AuditIndex -le $cp359AuditIndexForCp360 -or $cp360CompletionIndex -le $cp360AuditIndex) {
    throw "Master audit must dot-source CP360 after CP359 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 354' -Description "CP360 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP360 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp360-' -Description "CP360 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp360-cooling-humidistat-supply-humidity-ratio-for-dehumidification-assignment\.ps1::dot_sources' -Description "CP360 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 354 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 114 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP360 Humidistat supply-humidity-ratio-for-dehumidification assignment structure audit passed."
