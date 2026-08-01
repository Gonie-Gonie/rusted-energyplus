# CP367 maps only PurchasedAirManager.cc physical executable line 2238's
# default supply-humidity-ratio mixed-air assignment.
$cp367Stem = "cooling_default_supply_humidity_ratio_mixed_air_assignment"
$cp366StemForCp367 = "cooling_constant_supply_humidity_ratio_case_break"
$cp367PipelineStem = "purchased_air_$cp367Stem"
$cp367TypeStem = "PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignment"
$cp367Lifecycle = "purchased_air_calc_${cp367Stem}_lifecycle"
$cp367SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp367SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp367Sites = @(
    "read-purchased-air-mixed-air-humidity-ratio-for-dehumidification-control-default-assignment",
    "assign-purchased-air-supply-humidity-ratio-for-dehumidification-control-default-case"
)
$cp367Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp367Module = "crates\ep_runtime\src\ideal_loads\calc\$cp367Stem.rs"
$cp367ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp367Stem"
$cp367State = "$cp367ModuleRoot\state.rs"
$cp367Transition = "$cp367ModuleRoot\transition.rs"
$cp367Release = "$cp367ModuleRoot\release.rs"
$cp367Prefix = "$cp367ModuleRoot\release\prefix_validation.rs"
$cp367Private = "$cp367ModuleRoot\release\private_counterfactual.rs"
$cp367Runtime = "$cp367ModuleRoot\release\runtime_validation.rs"
$cp367Snapshot = "$cp367ModuleRoot\release\snapshot_validation.rs"
$cp367CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp367Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp367BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp367Stem.rs"
$cp367BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp367BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp367Stem}_tests.rs"
$cp367ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp367InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp367InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp367InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp367InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp367Stem.rs"
$cp367CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp367Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp367Stem}_validation.rs"
$cp367CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp367CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp367.rs"
$cp367FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp367Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp367Stem}_fixture.rs"
$cp367PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp367Pipeline = "crates\ep_run\src\pipeline\$cp367PipelineStem.rs"
$cp367PipelineValidation = "crates\ep_run\src\pipeline\$cp367PipelineStem\validation.rs"
$cp367PipelineTests = "crates\ep_run\src\pipeline\$cp367PipelineStem\validation\tests.rs"
$cp367Serialization = "crates\ep_run\src\pipeline\$cp367PipelineStem\serialization.rs"
$cp367SnapshotSerialization = "crates\ep_run\src\pipeline\$cp367PipelineStem\serialization\snapshot.rs"
$cp367ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp366_assertions.rs"
$cp367ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp367_assertions.rs"
$cp367Audit = "scripts\quality\ideal-loads-structure-audit\cp367-cooling-default-supply-humidity-ratio-mixed-air-assignment.ps1"

function Assert-Cp367TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP367 $Description missing" }
}

function Assert-Cp367TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP367 $Description unexpectedly present" }
}

function Get-Cp367RustBraceBlock {
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
            if ($depth -eq 0) {
                return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1)
            }
        }
    }
    throw "$Description has no complete brace block"
}

foreach ($required in @(
        $cp367Source, $cp367Module, $cp367State, $cp367Transition, $cp367Release,
        $cp367Prefix, $cp367Private, $cp367Runtime, $cp367Snapshot,
        $cp367BindingAdapter, $cp367BindingTests, $cp367InitWitness, $cp367Coupled,
        $cp367CoupledTests, $cp367Fixture, $cp367Pipeline, $cp367PipelineValidation,
        $cp367PipelineTests, $cp367Serialization, $cp367SnapshotSerialization,
        $cp367ParentAssertions, $cp367ArbitraryAssertions, $cp367Audit
    )) {
    Assert-FileExists -Path $required -Description "CP367 structure"
}
foreach ($bounded in @(
        $cp367Module, $cp367State, $cp367Transition, $cp367Release, $cp367Prefix,
        $cp367Private, $cp367Runtime, $cp367Snapshot, $cp367BindingAdapter,
        $cp367BindingTests, $cp367InitWitness, $cp367Coupled, $cp367CoupledTests,
        $cp367Fixture, $cp367Pipeline, $cp367PipelineValidation, $cp367PipelineTests,
        $cp367Serialization, $cp367SnapshotSerialization, $cp367ArbitraryAssertions,
        $cp367Audit
    )) {
    Assert-LineLimit -Path $bounded -Limit 500 -Description "CP367 bounded structure"
}

# Pinned physical boundary: line 2237 is unclaimed, line 2238 is mapped, and
# line 2239 is the first excluded executable statement.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp367Source).Hash -cne $cp367SourceHash) {
    throw "CP367 pinned PurchasedAirManager.cc hash drifted"
}
$cp367SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp367Source
if ($cp367SourceLines[2235].Trim() -cne '} break;' -or
    $cp367SourceLines[2236].Trim() -cne 'default: {' -or
    $cp367SourceLines[2237].Trim() -cne 'PurchAir.SupplyHumRat = PurchAir.MixedAirHumRat;' -or
    $cp367SourceLines[2238].Trim() -cne '} break;' -or
    $cp367SourceLines[2239].Trim() -cne '}' -or
    $cp367SourceLines[2241].Trim() -cne '// Check supply humidity ratio for humidification (SupplyHumRatForHum should always be < SupplyHumRatForDehum)' -or
    $cp367SourceLines[2242].Trim() -cne '// This section is the cooling section, so humidification should activate only if humidification control = humidistat' -or
    $cp367SourceLines[2243].Trim() -cne '//   and if dehumidification control = humidistat or none' -or
    $cp367SourceLines[2244].Trim() -cne 'if (HeatOn) {') {
    throw "CP367 pinned physical lines 2236 through 2245 drifted"
}
Assert-Contains -Path $cp367Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2238' -Description "CP367 source line"
Assert-Contains -Path $cp367Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2239' -Description "CP367 first excluded executable"
Assert-NotContains -Path $cp367Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2237' -Description "untyped default label claim"
Assert-ExactStringArray -Path $cp367Module -Name "PURCHASED_AIR_CALC_COOLING_DEFAULT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER" -Expected $cp367Sites -Description "CP367 two-site source order"

# Exactly seven retained typed routes and no invalid/eighth default route.
$cp367RouteBlock = Get-Cp367RustBraceBlock -Text (Read-RepoText -Path $cp367State) -AnchorPattern 'enum PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRetainedRoute\s*\{' -Description "CP367 retained-route enum"
[string[]]$cp367Routes = @(
    [regex]::Matches($cp367RouteBlock, '(?m)^\s{4}(?<route>[A-Z][A-Za-z0-9]+),\s*$') |
        ForEach-Object { $_.Groups["route"].Value }
)
$cp367ExpectedRoutes = @(
    "UnitOff", "NonCooling", "PositiveGuardFalseFallthrough",
    "DehumidificationControlNoneCaseCompletedSkip",
    "DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip",
    "DehumidificationControlHumidistatCaseCompletedSkip",
    "DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip"
)
if (($cp367Routes -join "|") -cne ($cp367ExpectedRoutes -join "|")) {
    throw "CP367 retained routes must be exactly U/N/P/C0/Q/H/CSH"
}
Assert-Cp367TextNotContains -Text $cp367RouteBlock -Pattern '(?m)^\s{4}(?:Default|Invalid|Unknown|Eighth)[A-Za-z0-9]*,\s*$' -Description "invalid or eighth route"
foreach ($counter in @(
        "transition_count", "unit_off_skip_count", "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_completed_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
        "mixed_air_humidity_ratio_read_count", "supply_humidity_ratio_assignment_count",
        "source_site_execution_count"
    )) {
    Assert-Contains -Path $cp367State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP367 counter '$counter'"
}
Assert-Contains -Path $cp367Transition -Pattern 'dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed:\s*false' -Description "all-route default assignment skip"
Assert-NotContains -Path $cp367Transition -Pattern '(?:mixed_air_humidity_ratio_read_count|supply_humidity_ratio_assignment_count|source_site_execution_count)\s*(?:\+=|=\s*[^;]*checked_add)' -Description "source counter execution"
foreach ($zero in @("mixed_air_humidity_ratio_read_count", "supply_humidity_ratio_assignment_count", "source_site_execution_count")) {
    Assert-Contains -Path $cp367Runtime -Pattern ('state\.' + $zero + '\s*==\s*0') -Description "zero source counter '$zero'"
}
foreach ($identity in @(
        'route_partition\s*==\s*state\.transition_count',
        'selected\s*==\s*recursively_witnessed',
        'latest_route_is_counted\(state,\s*latest\)',
        'witnessed_positive_guard_false_fallthrough_skip_count\s*[\r\n\s]*==\s*state\.positive_guard_false_fallthrough_skip_count',
        'SOURCE_ORDER\s*[\r\n\s]*\.len\(\)\s*[\r\n\s]*==\s*2'
    )) {
    Assert-Contains -Path $cp367Runtime -Pattern $identity -Description "checked route/source identity '$identity'"
}

# CP366 is the sole predecessor and private bridge. CP367 carries no humidity
# numeric argument, snapshot payload, owner read, or JSON sidecar.
Assert-Contains -Path $cp367Transition -Pattern 'PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor' -Description "exact CP366 predecessor type"
$cp367CalcText = (@($cp367Module) + @(
        Get-ChildItem -LiteralPath $cp367ModuleRoot -Recurse -File -Filter "*.rs" |
            ForEach-Object { $_.FullName }
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
$cp367PredecessorAliases = [regex]::Matches(
    $cp367CalcText,
    'PurchasedAirCalc[A-Za-z0-9]+Snapshot as Predecessor'
)
if ($cp367PredecessorAliases.Count -lt 4 -or
    @($cp367PredecessorAliases | Where-Object {
            $_.Value -cne 'PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor'
        }).Count -ne 0) {
    throw "CP367 predecessor aliases must name only CP366"
}
foreach ($pattern in @(
        'completed_direct_cooling_constant_supply_humidity_ratio_case_break_is_consistent',
        'cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release',
        'cooling_constant_supply_humidity_ratio_case_break_snapshots_match_exact',
        'cooling_constant_supply_humidity_ratio_case_break_latest_witness',
        'private_constant_supply_humidity_ratio_case_break_counterfactual_from_direct_release'
    )) {
    Assert-Cp367TextContains -Text $cp367CalcText -Pattern $pattern -Description "recursive CP366 owner '$pattern'"
}
Assert-Contains -Path $cp367Private -Pattern 'cp366_private_csh_case_break_from_direct_release' -Description "sole CP366 private bridge"
$cp367SnapshotDto = Get-Cp367RustBraceBlock -Text (Read-RepoText -Path $cp367Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot\s*\{' -Description "CP367 snapshot DTO"
Assert-Cp367TextNotContains -Text $cp367SnapshotDto -Pattern 'f64|ieee|bits|Option\s*<\s*f64\s*>|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio|mixed_air_humidity_ratio:\s*Option' -Description "snapshot numerical payload"
$cp367RuntimeStateDto = Get-Cp367RustBraceBlock -Text (Read-RepoText -Path $cp367State) -AnchorPattern 'pub struct PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState\s*\{' -Description "CP367 runtime-state DTO"
$cp367LifecycleSummaryDto = Get-Cp367RustBraceBlock -Text (Read-RepoText -Path $cp367Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary\s*\{' -Description "CP367 lifecycle-summary DTO"
foreach ($dto in @(
        [PSCustomObject]@{ Text = $cp367RuntimeStateDto; Description = "runtime-state DTO" },
        [PSCustomObject]@{ Text = $cp367LifecycleSummaryDto; Description = "lifecycle-summary DTO" }
    )) {
    Assert-Cp367TextNotContains -Text $dto.Text -Pattern 'f64|Option\s*<\s*f64\s*>|(?:mixed_air|supply|assigned|resulting|minimum|maximum|final|numerical)[A-Za-z0-9_]*humidity_ratio\s*:\s*(?:f64|Option)' -Description "$($dto.Description) numerical humidity payload"
}
$cp367PublicRelease = Get-Cp367RustBraceBlock -Text (Read-RepoText -Path $cp367Release) -AnchorPattern '(?m)^pub fn advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment\s*\(' -Description "CP367 public release"
Assert-Cp367TextContains -Text $cp367PublicRelease -Pattern 'predecessor_cp366:\s*Predecessor' -Description "public CP366 predecessor"
Assert-Cp367TextNotContains -Text $cp367PublicRelease -Pattern ':\s*f64|Option\s*<\s*f64\s*>|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio|mixed_air_humidity_ratio\s*:' -Description "public numerical operand"
foreach ($path in @($cp367Transition, $cp367Prefix, $cp367Private)) {
    Assert-NotContains -Path $path -Pattern 'assigned_supply_humidity_ratio|resulting_supply_humidity_ratio|to_bits|from_bits|\.is_finite\(\)|f64::|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply' -Description "CP367 pure numeric firewall"
}
$cp367SnapshotSerializationText = Read-RepoText -Path $cp367SnapshotSerialization
$cp367SnapshotTestBoundary = [regex]::Match($cp367SnapshotSerializationText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
$cp367SnapshotProduction = if ($cp367SnapshotTestBoundary.Success) {
    $cp367SnapshotSerializationText.Substring(0, $cp367SnapshotTestBoundary.Index)
} else {
    $cp367SnapshotSerializationText
}
Assert-Cp367TextNotContains -Text $cp367SnapshotProduction -Pattern '_ieee_bits|json_number|Value::Null|to_bits|f64|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio|"mixed_air_humidity_ratio"\s*:' -Description "control-only JSON"

# Searchable semantic regressions include route/latest cumulative evidence and
# the direct/non-direct pipeline contract.
$cp367SemanticText = (@(
        Get-ChildItem -LiteralPath $cp367ModuleRoot -Recurse -File -Filter "*.rs" |
            Where-Object { $_.FullName -match '(?:_tests\.rs$|[\\/]tests[\\/])' } |
            ForEach-Object { $_.FullName }
    ) + @(
        $cp367Runtime, $cp367BindingTests, $cp367CoupledTests, $cp367PipelineTests,
        $cp367SnapshotSerialization, $cp367ArbitraryAssertions, $cp367PipelineRoot
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($test in @(
        "source_boundary_two_sites_and_seven_route_zero_execution_algebra_are_exact",
        "provenance_selector_and_one_hot_corruption_are_transactional",
        "every_route_counter_overflow_and_every_nonzero_source_counter_are_transactional",
        "latest_route_counter_transfer_is_rejected_without_state_mutation",
        "public_direct_none_is_numeric_lazy_and_records_only_a_completed_skip",
        "private_csh_delegates_cp366_and_still_skips_the_untyped_default",
        "private_csh_rejects_nonfinite_cp365_owner_after_lazy_direct_release",
        "corruption_replay_and_nonzero_source_counter_reject_transactionally",
        "binding_orders_cp366_then_cp367_before_numerical_and_does_not_feed_result",
        "binding_cp367_is_complete_skip_for_every_direct_route",
        "binding_rejects_corrupt_cp366_without_mutation",
        "cp367_lifecycle_matches_outputs_and_cp366_without_feeding_numerical_result",
        "cp367_route_source_latest_and_predecessor_corruption_are_rejected",
        "cp367_expected_snapshot_maps_all_typed_selected_routes_to_default_skip",
        "missing_direct_lifecycle_fails_closed",
        "checked_partition_and_all_zero_source_counters_fail_closed",
        "expected_snapshot_preserves_all_typed_routes_and_never_executes_default_assignment",
        "direct_release_and_immediate_cp366_predecessor_are_strict",
        "latest_direct_route_requires_matching_current_and_predecessor_cumulative_evidence",
        "direct_none_release_serializes_typed_default_assignment_skip",
        "active_constant_supply_route_also_skips_default_assignment_without_numeric_payload",
        "non_direct_runtime_rejects_cp316_through_cp372_lifecycle_evidence"
    )) {
    Assert-Cp367TextContains -Text $cp367SemanticText -Pattern ('(?m)fn\s+' + $test + '\s*\(') -Description "semantic regression '$test'"
}
Assert-Contains -Path $cp367PipelineValidation -Pattern 'latest_route_has_cumulative_evidence' -Description "CP367 route/latest cumulative evidence validation"

# CP366 -> CP367 -> unchanged numerical; CP345 remains the numerical owner.
$cp367BindingText = Read-RepoText -Path $cp367Binding
$cp366BindingIndexForCp367 = $cp367BindingText.IndexOf("let calculation_${cp366StemForCp367} =")
$cp367BindingIndex = $cp367BindingText.IndexOf("let calculation_${cp367Stem} =")
$cp368BindingIndexForCp367 = $cp367BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_case_break =")
$cp369BindingIndexForCp367 = $cp367BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp367 = $cp367BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp367 = $cp367BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp367 = $cp367BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp367NumericalIndex = $cp367BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp366BindingIndexForCp367 -lt 0 -or $cp367BindingIndex -le $cp366BindingIndexForCp367 -or $cp368BindingIndexForCp367 -le $cp367BindingIndex -or $cp367NumericalIndex -le $cp368BindingIndexForCp367 -or
    $cp369BindingIndexForCp367 -le $cp368BindingIndexForCp367 -or
    $cp370BindingIndexForCp367 -le $cp369BindingIndexForCp367 -or
    $cp371BindingIndexForCp367 -le $cp370BindingIndexForCp367 -or
    $cp372BindingIndexForCp367 -le $cp371BindingIndexForCp367 -or
    $cp367NumericalIndex -le $cp372BindingIndexForCp367) {
    throw "Binding must execute CP366 then CP367 then CP368 before unchanged numerical coupling"
}
$cp367Dto = Get-Cp367RustBraceBlock -Text $cp367BindingText.Substring($cp367NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP367 numerical DTO"
if ($cp367Dto -match '(?i)cp367|default_supply_humidity_ratio_mixed_air_assignment') {
    throw "CP367 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp367CoupledTests -Pattern 'numerical_owner\.map\(f64::to_bits\)' -Description "CP345 numerical-owner bit identity"
Assert-Contains -Path $cp367ArbitraryAssertions -Pattern 'mod cp368_assertions;' -Description "CP368 arbitrary delegation module"
Assert-Contains -Path $cp367ArbitraryAssertions -Pattern 'cp368_assertions::assert_direct\(runtime, results\)' -Description "CP368 arbitrary direct delegation"
Assert-Contains -Path $cp367ArbitraryAssertions -Pattern 'cp368_assertions::assert_non_direct\(runtime\)' -Description "CP368 arbitrary non-direct delegation"
Assert-NotContains -Path $cp367ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP367 relinquishes terminal nonfeed"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp367CalcRoot; Pattern = $cp367Stem; Description = "calc registration" },
        [PSCustomObject]@{ Path = $cp367BindingAdapter; Pattern = "advance_direct_no_oa_calc_$cp367Stem"; Description = "binding adapter" },
        [PSCustomObject]@{ Path = $cp367ScheduledOutput; Pattern = "pub calculation_${cp367Stem}:"; Description = "scheduled output" },
        [PSCustomObject]@{ Path = $cp367BindingTestsRoot; Pattern = $cp367Stem; Description = "binding-test registration" },
        [PSCustomObject]@{ Path = $cp367InitState; Pattern = $cp367Stem; Description = "runtime state" },
        [PSCustomObject]@{ Path = $cp367InitUnit; Pattern = $cp367Stem; Description = "unit state" },
        [PSCustomObject]@{ Path = $cp367InitWitnessRoot; Pattern = $cp367Stem; Description = "witness registration" },
        [PSCustomObject]@{ Path = $cp367CoupledRoot; Pattern = "mod ${cp367Stem}_validation;"; Description = "coupled validator" },
        [PSCustomObject]@{ Path = $cp367FixtureRoot; Pattern = $cp367Stem; Description = "fixture registration" },
        [PSCustomObject]@{ Path = $cp367CoupledTestsRoot; Pattern = "coupled_runtime_tests_cp367"; Description = "coupled-test registration" },
        [PSCustomObject]@{ Path = $cp367PipelineRoot; Pattern = "mod ${cp367PipelineStem};"; Description = "pipeline module" },
        [PSCustomObject]@{ Path = $cp367PipelineRoot; Pattern = "`"$cp367Lifecycle`":\s*result\s*\.$cp367Lifecycle"; Description = "lifecycle JSON" }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP367 $($registration.Description)"
}
Assert-Contains -Path $cp367ParentAssertions -Pattern 'mod cp367_assertions;' -Description "arbitrary delegation module"
Assert-Contains -Path $cp367ParentAssertions -Pattern 'cp367_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp367ParentAssertions -Pattern 'cp367_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp367ParentAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP366 relinquishes terminal nonfeed"
Assert-Contains -Path $cp367ArbitraryAssertions -Pattern 'runtime\.contains_key\(CP367_KEY\)' -Description "CP367 non-direct key"
Assert-Contains -Path $cp367ArbitraryAssertions -Pattern 'runtime\[CP367_KEY\]\.is_null\(\)' -Description "CP367 non-direct null"

# Exactly two addenda in each spec, exact target multiplicity, five hand-doc
# sections, generated docs, historical propagation, and 305/240/65/0 scripts.
$cp367AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp367CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp367AlgorithmAddenda = [regex]::Matches($cp367AlgorithmText, '(?m)^\s*"CP367 supersedes only CP366[^"\r\n]+",\s*$')
$cp367CapabilityAddenda = [regex]::Matches($cp367CapabilityText, '(?m)^\s*"CP367 additionally requires[^"\r\n]+",\s*$')
if ($cp367AlgorithmAddenda.Count -ne 2 -or $cp367CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP367 addenda"
}
foreach ($claim in @($cp367AlgorithmAddenda) + @($cp367CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp367SourceCommit, $cp367SourceHash, 'physical executable line 2238',
            $cp367Sites[0], $cp367Sites[1], 'line 2237', 'unclaimed',
            'physical executable line 2239', 'first excluded', 'line 2245',
            'seven named-enum routes', 'no invalid-enum or eighth default route',
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'D=read=assignment=source_site=0', 'CP366',
            'sole predecessor owner', 'canonical private bridge',
            'CP366-to-CP367-to-unchanged-numerical', $cp367Lifecycle, 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '305 total', '240 public', '65 internal',
            'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP367 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp367Stem/release.rs::advance_direct_no_oa_calc_$cp367Stem"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp367Stem.rs::purchased_air_calc_${cp367Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp367Stem.rs::${cp367TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp367Stem.rs::${cp367TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp367AlgorithmText, [regex]::Escape($target.Value)).Count -ne $target.Expected) {
        throw "CP367 target count failed for '$($target.Value)'"
    }
}
$cp367Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP367 Cooling Default Supply-Humidity-Ratio Mixed-Air Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP367 Source-Ordered Cooling Default Supply-Humidity-Ratio Mixed-Air Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP367 Default Supply-Humidity-Ratio Mixed-Air Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP367 Default Supply-Humidity-Ratio Mixed-Air Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP367 Default Supply-Humidity-Ratio Mixed-Air Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp367Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP367 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp367SourceCommit, $cp367SourceHash, '2238', 'SupplyHumRat\s*=\s*PurchAir\.MixedAirHumRat',
            '2237', 'not\s+claimed', '2239', 'first excluded', '2245',
            '(?s)(?:read-purchased-air-mixed-air-humidity-ratio.{0,300}assign-purchased-air-supply-humidity-ratio|two\s+(?:dependency-)?ordered|two ordered read\s+and\s+assignment|ordered\s+mixed-air-humidity read\s+and\s+supply-humidity assignment)',
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH', 'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L', 'D\s*=\s*read\s*=\s*assignment\s*=\s*source_site\s*=\s*0',
            '(?i)no\s+invalid(?:-enum|\s+discriminant)[^.\r\n]{0,50}eighth', 'CP366',
            'sole\s+predecessor', '(?s)(?:canonical private.{0,30}(?:bridge|break)|sole\s+predecessor.{0,50}private bridge|typed private.{0,20}break)',
            'CP366-to-CP367-to-unchanged-numerical', $cp367Lifecycle, 'CP345',
            '32\s+algorithms', '293\s+routines', '58\s+[^0-9\r\n]{0,5}state[_-]mapped',
            '235\s+[^0-9\r\n]{0,5}source[_-]mapped',
            '170\s+required', '305\s+total', '240\s+public', '65\s+internal',
            'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP367 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP367\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP367 supersedes only CP366' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP367 additionally requires' -Description "generated capability addendum"

foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..366 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment' -Description "historical CP367 binding order"
}
foreach ($historical in 327..328) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment' -Description "out-of-range CP367 binding token"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_default_supply_humidity_ratio_mixed_air_assignment' -Description "historical CP367 helper whitelist"
}
foreach ($historical in @(327, 328) + @(346..366)) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'advance_cooling_default_supply_humidity_ratio_mixed_air_assignment' -Description "out-of-range CP367 helper-whitelist token"
}
foreach ($historical in 334..366) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp372_lifecycle_evidence' -Description "historical CP367 firewall"
}
foreach ($historical in 326..333) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp372_lifecycle_evidence' -Description "out-of-range CP367 firewall token"
}
foreach ($historical in 335..366) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 310 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 70 \|')) -Description "historical generated internal"
}
foreach ($historical in 326..334) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 310 \|')) -Description "out-of-range generated-total token"
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 70 \|')) -Description "out-of-range generated-internal token"
}
foreach ($historical in 337..366) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 310' -Description "historical inventory total"
}
foreach ($historical in 326..336) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 310' -Description "out-of-range inventory-total token"
}
$cp367MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp366AuditIndexForCp367 = $cp367MainAuditText.IndexOf("cp366-cooling-constant-supply-humidity-ratio-case-break.ps1")
$cp367AuditIndex = $cp367MainAuditText.IndexOf("cp367-cooling-default-supply-humidity-ratio-mixed-air-assignment.ps1")
$cp367CompletionIndex = $cp367MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp366AuditIndexForCp367 -lt 0 -or $cp367AuditIndex -le $cp366AuditIndexForCp367 -or $cp367CompletionIndex -le $cp367AuditIndex) {
    throw "Master audit must dot-source CP367 after CP366 before completion"
}
$cp367InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp367TextContains -Text $cp367InventoryText -Pattern 'script_count = 310' -Description "script total"
Assert-Cp367TextContains -Text $cp367InventoryText -Pattern 'unused_script_count = 0' -Description "zero uncalled"
if ([regex]::Matches($cp367InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($cp367InventoryText, '(?m)^classification = "internal"$').Count -ne 70) {
    throw "CP367 inventory must be exactly 240 public and 70 internal scripts"
}
Assert-Cp367TextContains -Text $cp367InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp367-' -Description "inventory record"
Assert-Cp367TextContains -Text $cp367InventoryText -Pattern 'cp367-cooling-default-supply-humidity-ratio-mixed-air-assignment\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 310 \|' -Description "CP367 generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP367 generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 70 \|' -Description "CP367 generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP367 generated uncalled"

Write-Host "CP367 default supply-humidity-ratio mixed-air-assignment structure audit passed."
