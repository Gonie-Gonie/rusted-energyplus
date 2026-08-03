# CP368 maps only PurchasedAirManager.cc physical executable line 2239's
# untyped-default supply-humidity-ratio case break.
$cp368Stem = "cooling_default_supply_humidity_ratio_case_break"
$cp367StemForCp368 = "cooling_default_supply_humidity_ratio_mixed_air_assignment"
$cp368PipelineStem = "purchased_air_$cp368Stem"
$cp368TypeStem = "PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreak"
$cp368Lifecycle = "purchased_air_calc_${cp368Stem}_lifecycle"
$cp368SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp368SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp368Sites = @(
    "exit-purchased-air-dehumidification-control-default-case-via-break"
)
$cp368Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp368Module = "crates\ep_runtime\src\ideal_loads\calc\$cp368Stem.rs"
$cp368ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp368Stem"
$cp368State = "$cp368ModuleRoot\state.rs"
$cp368Transition = "$cp368ModuleRoot\transition.rs"
$cp368Release = "$cp368ModuleRoot\release.rs"
$cp368Prefix = "$cp368ModuleRoot\release\prefix_validation.rs"
$cp368Private = "$cp368ModuleRoot\release\private_counterfactual.rs"
$cp368Runtime = "$cp368ModuleRoot\release\runtime_validation.rs"
$cp368Snapshot = "$cp368ModuleRoot\release\snapshot_validation.rs"
$cp368CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp368Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp368BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp368Stem.rs"
$cp368BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp368BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp368Stem}_tests.rs"
$cp368ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp368InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp368InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp368InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp368InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp368Stem.rs"
$cp368CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp368Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp368Stem}_validation.rs"
$cp368CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp368CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp368.rs"
$cp368FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp368Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp368Stem}_fixture.rs"
$cp368PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp368Pipeline = "crates\ep_run\src\pipeline\$cp368PipelineStem.rs"
$cp368PipelineValidation = "crates\ep_run\src\pipeline\$cp368PipelineStem\validation.rs"
$cp368PipelineTests = "crates\ep_run\src\pipeline\$cp368PipelineStem\validation\tests.rs"
$cp368Serialization = "crates\ep_run\src\pipeline\$cp368PipelineStem\serialization.rs"
$cp368SnapshotSerialization = "crates\ep_run\src\pipeline\$cp368PipelineStem\serialization\snapshot.rs"
$cp368ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp367_assertions.rs"
$cp368ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp368_assertions.rs"
$cp368Audit = "scripts\quality\ideal-loads-structure-audit\cp368-cooling-default-supply-humidity-ratio-case-break.ps1"

function Assert-Cp368TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP368 $Description missing" }
}

function Assert-Cp368TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP368 $Description unexpectedly present" }
}

function Get-Cp368RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "$Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "$Description has no opening brace" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "$Description has no complete brace block"
}

foreach ($required in @(
        $cp368Source, $cp368Module, $cp368State, $cp368Transition, $cp368Release,
        $cp368Prefix, $cp368Private, $cp368Runtime, $cp368Snapshot,
        $cp368BindingAdapter, $cp368BindingTests, $cp368InitWitness, $cp368Coupled,
        $cp368CoupledTests, $cp368Fixture, $cp368Pipeline, $cp368PipelineValidation,
        $cp368PipelineTests, $cp368Serialization, $cp368SnapshotSerialization,
        $cp368ParentAssertions, $cp368ArbitraryAssertions, $cp368Audit
    )) {
    Assert-FileExists -Path $required -Description "CP368 structure"
}
foreach ($bounded in @(
        $cp368Module, $cp368State, $cp368Transition, $cp368Release, $cp368Prefix,
        $cp368Private, $cp368Runtime, $cp368Snapshot, $cp368BindingAdapter,
        $cp368BindingTests, $cp368InitWitness, $cp368Coupled, $cp368CoupledTests,
        $cp368Fixture, $cp368Pipeline, $cp368PipelineValidation, $cp368PipelineTests,
        $cp368Serialization, $cp368SnapshotSerialization, $cp368ArbitraryAssertions,
        $cp368Audit
    )) {
    Assert-LineLimit -Path $bounded -Limit 500 -Description "CP368 bounded structure"
}

# Pinned line 2239 is the only mapped executable; line 2245 is first excluded.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp368Source).Hash -cne $cp368SourceHash) {
    throw "CP368 pinned PurchasedAirManager.cc hash drifted"
}
$cp368SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp368Source
if ($cp368SourceLines[2236].Trim() -cne 'default: {' -or
    $cp368SourceLines[2237].Trim() -cne 'PurchAir.SupplyHumRat = PurchAir.MixedAirHumRat;' -or
    $cp368SourceLines[2238].Trim() -cne '} break;' -or
    $cp368SourceLines[2239].Trim() -cne '}' -or
    $cp368SourceLines[2241].Trim() -cne '// Check supply humidity ratio for humidification (SupplyHumRatForHum should always be < SupplyHumRatForDehum)' -or
    $cp368SourceLines[2242].Trim() -cne '// This section is the cooling section, so humidification should activate only if humidification control = humidistat' -or
    $cp368SourceLines[2243].Trim() -cne '//   and if dehumidification control = humidistat or none' -or
    $cp368SourceLines[2244].Trim() -cne 'if (HeatOn) {') {
    throw "CP368 pinned physical lines 2237 through 2245 drifted"
}
Assert-Contains -Path $cp368Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2239' -Description "CP368 source line"
Assert-Contains -Path $cp368Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2245' -Description "CP368 first excluded executable"
Assert-ExactStringArray -Path $cp368Module -Name "PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER" -Expected $cp368Sites -Description "CP368 sole source site"

# Seven named routes universally skip the untyped default break.
$cp368RouteBlock = Get-Cp368RustBraceBlock -Text (Read-RepoText -Path $cp368State) -AnchorPattern 'enum PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRetainedRoute\s*\{' -Description "CP368 retained-route enum"
[string[]]$cp368Routes = @([regex]::Matches($cp368RouteBlock, '(?m)^\s{4}(?<route>[A-Z][A-Za-z0-9]+),\s*$') | ForEach-Object { $_.Groups["route"].Value })
$cp368ExpectedRoutes = @(
    "UnitOff", "NonCooling", "PositiveGuardFalseFallthrough",
    "DehumidificationControlNoneCaseCompletedSkip",
    "DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip",
    "DehumidificationControlHumidistatCaseCompletedSkip",
    "DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip"
)
if (($cp368Routes -join "|") -cne ($cp368ExpectedRoutes -join "|")) {
    throw "CP368 retained routes must be exactly U/N/P/C0/Q/H/CSH"
}
Assert-Cp368TextNotContains -Text $cp368RouteBlock -Pattern '(?m)^\s{4}(?:Default|Invalid|Unknown|Eighth)[A-Za-z0-9]*,\s*$' -Description "invalid or eighth route"
foreach ($counter in @(
        "transition_count", "unit_off_skip_count", "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_completed_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
        "dehumidification_control_default_supply_humidity_ratio_case_break_count",
        "source_site_execution_count"
    )) {
    Assert-Contains -Path $cp368State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP368 counter '$counter'"
}
Assert-Contains -Path $cp368Transition -Pattern 'dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:\s*false' -Description "all-route default break skip"
Assert-NotContains -Path $cp368Transition -Pattern '(?:default_supply_humidity_ratio_case_break_count|source_site_execution_count)\s*(?:\+=|=\s*[^;]*checked_add)' -Description "source counter execution"
foreach ($zero in @("dehumidification_control_default_supply_humidity_ratio_case_break_count", "source_site_execution_count")) {
    Assert-Contains -Path $cp368Runtime -Pattern ('state\s*\.\s*' + $zero + '\s*==\s*0') -Description "zero source counter '$zero'"
}
foreach ($identity in @(
        'route_partition\s*==\s*state\.transition_count',
        'selected\s*==\s*recursively_witnessed',
        'latest_route_is_counted\(state,\s*latest\)',
        'SOURCE_ORDER\s*[\r\n\s]*\.len\(\)\s*[\r\n\s]*==\s*1'
    )) {
    Assert-Contains -Path $cp368Runtime -Pattern $identity -Description "checked route/source identity '$identity'"
}

# CP367 is the only predecessor; CP368 is numeric-free and control-only.
Assert-Contains -Path $cp368Transition -Pattern 'PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot as Predecessor' -Description "exact CP367 predecessor type"
$cp368CalcText = (@($cp368Module) + @(
        Get-ChildItem -LiteralPath $cp368ModuleRoot -Recurse -File -Filter "*.rs" | ForEach-Object { $_.FullName }
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($pattern in @(
        'completed_direct_cooling_default_supply_humidity_ratio_mixed_air_assignment_is_consistent',
        'cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release',
        'cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshots_match_exact',
        'cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness',
        'private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_from_direct_release'
    )) {
    Assert-Cp368TextContains -Text $cp368CalcText -Pattern $pattern -Description "recursive CP367 owner '$pattern'"
}
$cp368SnapshotDto = Get-Cp368RustBraceBlock -Text (Read-RepoText -Path $cp368Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot\s*\{' -Description "CP368 snapshot DTO"
$cp368RuntimeStateDto = Get-Cp368RustBraceBlock -Text (Read-RepoText -Path $cp368State) -AnchorPattern 'pub struct PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRuntimeState\s*\{' -Description "CP368 runtime-state DTO"
$cp368LifecycleDto = Get-Cp368RustBraceBlock -Text (Read-RepoText -Path $cp368Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleSummary\s*\{' -Description "CP368 lifecycle DTO"
foreach ($dto in @($cp368SnapshotDto, $cp368RuntimeStateDto, $cp368LifecycleDto)) {
    Assert-Cp368TextNotContains -Text $dto -Pattern 'f64|ieee|bits|Option\s*<\s*f64\s*>|(?:mixed_air|supply|assigned|resulting|minimum|maximum|final)[A-Za-z0-9_]*humidity_ratio\s*:\s*(?:f64|Option)' -Description "DTO numerical humidity payload"
}
foreach ($path in @($cp368Transition, $cp368Prefix, $cp368Private)) {
    Assert-NotContains -Path $path -Pattern 'assigned_supply_humidity_ratio|resulting_supply_humidity_ratio|to_bits|from_bits|\.is_finite\(\)|f64::|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply' -Description "CP368 pure numeric firewall"
}
# Searchable regressions and control-only serialization.
$cp368SemanticText = (@(
        Get-ChildItem -LiteralPath $cp368ModuleRoot -Recurse -File -Filter "*.rs" |
            Where-Object { $_.FullName -match '(?:_tests\.rs$|[\\/]tests[\\/])' } |
            ForEach-Object { $_.FullName }
    ) + @(
        $cp368Runtime, $cp368BindingTests, $cp368CoupledTests, $cp368PipelineTests,
        $cp368SnapshotSerialization, $cp368ArbitraryAssertions, $cp368PipelineRoot
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($test in @(
        "source_boundary_one_site_and_seven_route_zero_execution_algebra_are_exact",
        "provenance_selector_and_one_hot_corruption_are_transactional",
        "every_route_counter_overflow_and_every_nonzero_source_counter_are_transactional",
        "public_direct_none_is_numeric_lazy_and_records_only_a_completed_skip",
        "private_csh_delegates_cp367_and_still_skips_the_untyped_default",
        "private_csh_rejects_nonfinite_cp365_owner_after_lazy_direct_release",
        "corruption_replay_and_nonzero_source_counter_reject_transactionally",
        "binding_orders_cp367_then_cp368_before_numerical_and_does_not_feed_result",
        "binding_cp368_is_complete_skip_for_every_direct_route",
        "binding_rejects_corrupt_cp367_without_mutation",
        "cp368_lifecycle_matches_outputs_and_cp367_without_feeding_numerical_result",
        "cp368_route_source_latest_and_predecessor_corruption_are_rejected",
        "cp368_expected_snapshot_maps_all_typed_selected_routes_to_default_break_skip",
        "missing_direct_lifecycle_fails_closed",
        "checked_partition_and_all_zero_source_counters_fail_closed",
        "expected_snapshot_preserves_all_typed_routes_and_never_executes_default_case_break",
        "direct_release_and_immediate_cp367_predecessor_are_strict",
        "latest_direct_route_requires_matching_current_and_predecessor_cumulative_evidence",
        "non_direct_runtime_rejects_cp316_through_cp412_lifecycle_evidence"
    )) {
    Assert-Cp368TextContains -Text $cp368SemanticText -Pattern ('(?m)fn\s+' + $test + '\s*\(') -Description "semantic regression '$test'"
}
Assert-Contains -Path $cp368PipelineValidation -Pattern 'latest_route_has_cumulative_evidence' -Description "CP368 route/latest cumulative evidence validation"
$cp368SnapshotSerializationText = Read-RepoText -Path $cp368SnapshotSerialization
$cp368SnapshotTestBoundary = [regex]::Match($cp368SnapshotSerializationText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
$cp368SnapshotProduction = if ($cp368SnapshotTestBoundary.Success) {
    $cp368SnapshotSerializationText.Substring(0, $cp368SnapshotTestBoundary.Index)
} else { $cp368SnapshotSerializationText }
Assert-Cp368TextNotContains -Text $cp368SnapshotProduction -Pattern '_ieee_bits|json_number|Value::Null|to_bits|f64|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio' -Description "control-only JSON"

# CP367 -> CP368 -> unchanged numerical; CP345 remains the numerical owner.
$cp368BindingText = Read-RepoText -Path $cp368Binding
$cp367BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_${cp367StemForCp368} =")
$cp368BindingIndex = $cp368BindingText.IndexOf("let calculation_${cp368Stem} =")
$cp369BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp368 = $cp368BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp368NumericalIndex = $cp368BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp367BindingIndexForCp368 -lt 0 -or $cp368BindingIndex -le $cp367BindingIndexForCp368 -or $cp369BindingIndexForCp368 -le $cp368BindingIndex -or $cp370BindingIndexForCp368 -le $cp369BindingIndexForCp368 -or
    $cp371BindingIndexForCp368 -le $cp370BindingIndexForCp368 -or
    $cp372BindingIndexForCp368 -le $cp371BindingIndexForCp368 -or
    $cp373BindingIndexForCp368 -le $cp372BindingIndexForCp368 -or
    $cp374BindingIndexForCp368 -le $cp373BindingIndexForCp368 -or
    $cp375BindingIndexForCp368 -le $cp374BindingIndexForCp368 -or
    $cp376BindingIndexForCp368 -le $cp375BindingIndexForCp368 -or $cp377BindingIndexForCp368 -le $cp376BindingIndexForCp368 -or $cp378BindingIndexForCp368 -le $cp377BindingIndexForCp368 -or $cp379BindingIndexForCp368 -le $cp378BindingIndexForCp368 -or $cp380BindingIndexForCp368 -le $cp379BindingIndexForCp368 -or $cp381BindingIndexForCp368 -le $cp380BindingIndexForCp368 -or $cp382BindingIndexForCp368 -le $cp381BindingIndexForCp368 -or $cp383BindingIndexForCp368 -le $cp382BindingIndexForCp368 -or $cp384BindingIndexForCp368 -le $cp383BindingIndexForCp368 -or $cp385BindingIndexForCp368 -le $cp384BindingIndexForCp368 -or $cp368NumericalIndex -le $cp385BindingIndexForCp368) {
    throw "Binding must execute CP367 then CP368 then CP369 then CP370 before unchanged numerical coupling"
}
$cp368Dto = Get-Cp368RustBraceBlock -Text $cp368BindingText.Substring($cp368NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP368 numerical DTO"
if ($cp368Dto -match '(?i)cp368|default_supply_humidity_ratio_case_break') {
    throw "CP368 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp368CoupledTests -Pattern 'numerical_owner\.map\(f64::to_bits\)' -Description "CP345 numerical-owner bit identity"
Assert-Contains -Path $cp368ArbitraryAssertions -Pattern 'mod cp369_assertions;' -Description "CP369 arbitrary delegation module"
Assert-Contains -Path $cp368ArbitraryAssertions -Pattern 'cp369_assertions::assert_direct\(runtime, results\)' -Description "CP369 arbitrary direct delegation"
Assert-Contains -Path $cp368ArbitraryAssertions -Pattern 'cp369_assertions::assert_non_direct\(runtime\)' -Description "CP369 arbitrary non-direct delegation"
Assert-NotContains -Path $cp368ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP368 relinquishes terminal nonfeed"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp368CalcRoot; Pattern = $cp368Stem; Description = "calc registration" },
        [PSCustomObject]@{ Path = $cp368BindingAdapter; Pattern = "advance_direct_no_oa_calc_$cp368Stem"; Description = "binding adapter" },
        [PSCustomObject]@{ Path = $cp368ScheduledOutput; Pattern = "pub calculation_${cp368Stem}:"; Description = "scheduled output" },
        [PSCustomObject]@{ Path = $cp368BindingTestsRoot; Pattern = $cp368Stem; Description = "binding-test registration" },
        [PSCustomObject]@{ Path = $cp368InitState; Pattern = $cp368Stem; Description = "runtime state" },
        [PSCustomObject]@{ Path = $cp368InitUnit; Pattern = $cp368Stem; Description = "unit state" },
        [PSCustomObject]@{ Path = $cp368InitWitnessRoot; Pattern = $cp368Stem; Description = "witness registration" },
        [PSCustomObject]@{ Path = $cp368CoupledRoot; Pattern = "mod ${cp368Stem}_validation;"; Description = "coupled validator" },
        [PSCustomObject]@{ Path = $cp368FixtureRoot; Pattern = $cp368Stem; Description = "fixture registration" },
        [PSCustomObject]@{ Path = $cp368CoupledTestsRoot; Pattern = "coupled_runtime_tests_cp368"; Description = "coupled-test registration" },
        [PSCustomObject]@{ Path = $cp368PipelineRoot; Pattern = "mod ${cp368PipelineStem};"; Description = "pipeline module" },
        [PSCustomObject]@{ Path = $cp368PipelineRoot; Pattern = "`"$cp368Lifecycle`":\s*result\s*\.$cp368Lifecycle"; Description = "lifecycle JSON" }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP368 $($registration.Description)"
}
Assert-Contains -Path $cp368ParentAssertions -Pattern 'mod cp368_assertions;' -Description "arbitrary delegation module"
Assert-Contains -Path $cp368ParentAssertions -Pattern 'cp368_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp368ParentAssertions -Pattern 'cp368_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp368ParentAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP367 relinquishes terminal nonfeed"
Assert-Contains -Path $cp368ArbitraryAssertions -Pattern 'runtime\.contains_key\(CP368_KEY\)' -Description "CP368 non-direct key"
Assert-Contains -Path $cp368ArbitraryAssertions -Pattern 'runtime\[CP368_KEY\]\.is_null\(\)' -Description "CP368 non-direct null"

# Two spec addenda, target multiplicities, and five hand-doc sections.
$cp368AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp368CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp368AlgorithmAddenda = [regex]::Matches($cp368AlgorithmText, '(?m)^\s*"CP368 supersedes only CP367[^"\r\n]+",\s*$')
$cp368CapabilityAddenda = [regex]::Matches($cp368CapabilityText, '(?m)^\s*"CP368 additionally requires[^"\r\n]+",\s*$')
if ($cp368AlgorithmAddenda.Count -ne 2 -or $cp368CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP368 addenda"
}
foreach ($claim in @($cp368AlgorithmAddenda) + @($cp368CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp368SourceCommit, $cp368SourceHash, 'physical executable line 2239',
            $cp368Sites[0], 'line 2237', 'unclaimed', 'physical executable line 2238',
            'line 2240', 'lines 2241-2244', 'physical executable line 2245',
            'seven named-enum routes', 'no invalid-enum or eighth default route',
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=default_supply_humidity_ratio_case_break_count=B=0',
            'false break', 'no typed or private route executes a true default break', 'CP367',
            'sole predecessor owner', 'CP367-to-CP368-to-unchanged-numerical',
            $cp368Lifecycle, 'CP345', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', '170 required', 'Roadmap',
            '306 total', '240 public', '66 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP368 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp368Stem/release.rs::advance_direct_no_oa_calc_$cp368Stem"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp368Stem.rs::purchased_air_calc_${cp368Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp368Stem.rs::${cp368TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp368Stem.rs::${cp368TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp368AlgorithmText, [regex]::Escape($target.Value)).Count -ne $target.Expected) {
        throw "CP368 target count failed for '$($target.Value)'"
    }
}
$cp368Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP368 Cooling Default Supply-Humidity-Ratio Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP368 Source-Ordered Cooling Default Supply-Humidity-Ratio Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP368 Default Supply-Humidity-Ratio Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP368 Default Supply-Humidity-Ratio Case Break in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP368 Default Supply-Humidity-Ratio Case-Break Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp368Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP368 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp368SourceCommit, $cp368SourceHash, '2239', 'break', '2240', '2241-2244', '2245',
            $cp368Sites[0], 'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L', 'A\s*=\s*F\+L',
            'B\s*=\s*default_break\s*=\s*source_site\s*=\s*0', 'CP367',
            'sole\s+predecessor', 'CP367-to-CP368-to-unchanged-numerical',
            $cp368Lifecycle, 'CP345', '32\s+algorithms', '293\s+routines',
            '58\s+`?state_mapped`?', '235\s+`?source_mapped`?', '170\s+required',
            '306\s+total', '240\s+public', '66\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP368 documentation in $($doc.Path) missing '$pattern'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP368\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP368 supersedes only CP367' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP368 additionally requires' -Description "generated capability addendum"
# Historical source order, helper whitelist, firewall, generated totals, and inventory.
$cp368HistoricalHelperToken = 'advance_cooling_default_supply_humidity_ratio_' + 'case_break'
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..368 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_default_supply_humidity_ratio_case_break' -Description "historical CP368 binding order"
}
foreach ($historical in 327..328) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'calculation_cooling_default_supply_humidity_ratio_case_break' -Description "out-of-range CP368 binding token"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern $cp368HistoricalHelperToken -Description "historical CP368 helper whitelist"
}
foreach ($historical in @(327, 328) + @(346..368)) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern $cp368HistoricalHelperToken -Description "out-of-range CP368 helper token"
}
foreach ($historical in 334..368) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp412_lifecycle_evidence' -Description "historical CP368 firewall"
}
foreach ($historical in 326..333) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp412_lifecycle_evidence' -Description "out-of-range CP368 firewall token"
}
foreach ($historical in 335..368) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 350 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 110 \|')) -Description "historical generated internal"
}
foreach ($historical in 326..334) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 350 \|')) -Description "out-of-range generated-total token"
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 110 \|')) -Description "out-of-range generated-internal token"
}
foreach ($historical in 337..368) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 350' -Description "historical inventory total"
}
foreach ($historical in 326..336) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 350' -Description "out-of-range inventory-total token"
}
$cp368MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp367AuditIndexForCp368 = $cp368MainAuditText.IndexOf("cp367-cooling-default-supply-humidity-ratio-mixed-air-assignment.ps1")
$cp368AuditIndex = $cp368MainAuditText.IndexOf("cp368-cooling-default-supply-humidity-ratio-case-break.ps1")
$cp368CompletionIndex = $cp368MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp367AuditIndexForCp368 -lt 0 -or $cp368AuditIndex -le $cp367AuditIndexForCp368 -or $cp368CompletionIndex -le $cp368AuditIndex) {
    throw "Master audit must dot-source CP368 after CP367 before completion"
}
$cp368InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp368TextContains -Text $cp368InventoryText -Pattern 'script_count = 350' -Description "script total"
Assert-Cp368TextContains -Text $cp368InventoryText -Pattern 'unused_script_count = 0' -Description "zero uncalled"
if ([regex]::Matches($cp368InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp368InventoryText, '(?m)^classification = "internal"$').Count -ne 110) {
throw "CP368 inventory must be exactly 240 public and 110 internal scripts"
}
Assert-Cp368TextContains -Text $cp368InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp368-' -Description "inventory record"
Assert-Cp368TextContains -Text $cp368InventoryText -Pattern 'cp368-cooling-default-supply-humidity-ratio-case-break\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 350 \|' -Description "CP368 generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP368 generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 110 \|' -Description "CP368 generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP368 generated uncalled"

Write-Host "CP368 default supply-humidity-ratio case-break structure audit passed."
