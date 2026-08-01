# CP370 maps PurchasedAirManager.cc physical executable line 2246's
# Cooling supply-humidity-ratio humidification-control Humidistat guard and its three sites.
$cp370Stem = "cooling_supply_humidity_ratio_humidification_control_humidistat_guard"
$cp369StemForCp370 = "cooling_supply_humidity_ratio_humidification_heating_availability_guard"
$cp370PipelineStem = "purchased_air_$cp370Stem"
$cp370TypeStem = "PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuard"
$cp370Lifecycle = "purchased_air_calc_${cp370Stem}_lifecycle"
$cp370SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp370SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp370Sites = @(
    "read-purchased-air-humidification-control-type-for-cooling-supply-humidity-ratio-humidification-guard",
    "compare-purchased-air-humidification-control-type-equal-to-humidistat",
    "enter-cooling-supply-humidity-ratio-humidification-control-body-if-humidistat"
)
$cp370Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp370Module = "crates\ep_runtime\src\ideal_loads\calc\$cp370Stem.rs"
$cp370ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp370Stem"
$cp370State = "$cp370ModuleRoot\state.rs"
$cp370Transition = "$cp370ModuleRoot\transition.rs"
$cp370Release = "$cp370ModuleRoot\release.rs"
$cp370Prefix = "$cp370ModuleRoot\release\prefix_validation.rs"
$cp370Private = "$cp370ModuleRoot\release\private_counterfactual.rs"
$cp370Runtime = "$cp370ModuleRoot\release\runtime_validation.rs"
$cp370Snapshot = "$cp370ModuleRoot\release\snapshot_validation.rs"
$cp370CoreTests = "$cp370ModuleRoot\tests\mod.rs"
$cp370ReleaseTests = "$cp370ModuleRoot\tests\release.rs"
$cp370CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp370Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp370BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp370Stem.rs"
$cp370BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp370BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp370Stem}_tests.rs"
$cp370ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp370InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp370InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp370InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp370InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp370Stem.rs"
$cp370CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp370Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp370Stem}_validation.rs"
$cp370CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp370CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp370.rs"
$cp370FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp370Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp370Stem}_fixture.rs"
$cp370PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp370Pipeline = "crates\ep_run\src\pipeline\$cp370PipelineStem.rs"
$cp370PipelineValidation = "crates\ep_run\src\pipeline\$cp370PipelineStem\validation.rs"
$cp370PipelineTests = "crates\ep_run\src\pipeline\$cp370PipelineStem\validation\tests.rs"
$cp370Serialization = "crates\ep_run\src\pipeline\$cp370PipelineStem\serialization.rs"
$cp370SnapshotSerialization = "crates\ep_run\src\pipeline\$cp370PipelineStem\serialization\snapshot.rs"
$cp370ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp369_assertions.rs"
$cp370ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp370_assertions.rs"
$cp370Audit = "scripts\quality\ideal-loads-structure-audit\cp370-cooling-supply-humidity-ratio-humidification-control-humidistat-guard.ps1"

function Assert-Cp370TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP370 $Description missing" }
}
function Assert-Cp370TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP370 $Description unexpectedly present" }
}
function Get-Cp370RustBraceBlock {
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
        $cp370Source, $cp370Module, $cp370State, $cp370Transition, $cp370Release,
        $cp370Prefix, $cp370Private, $cp370Runtime, $cp370Snapshot, $cp370CoreTests,
        $cp370ReleaseTests, $cp370BindingAdapter, $cp370BindingTests, $cp370InitWitness,
        $cp370Coupled, $cp370CoupledTests, $cp370Fixture, $cp370Pipeline,
        $cp370PipelineValidation, $cp370PipelineTests, $cp370Serialization,
        $cp370SnapshotSerialization, $cp370ParentAssertions, $cp370ArbitraryAssertions, $cp370Audit
    )) {
    Assert-FileExists -Path $required -Description "CP370 structure"
}
foreach ($bounded in @(
        $cp370Module, $cp370State, $cp370Transition, $cp370Release, $cp370Prefix,
        $cp370Private, $cp370Runtime, $cp370Snapshot, $cp370CoreTests, $cp370ReleaseTests,
        $cp370BindingAdapter, $cp370BindingTests, $cp370InitWitness, $cp370Coupled,
        $cp370CoupledTests, $cp370Fixture, $cp370Pipeline, $cp370PipelineValidation,
        $cp370PipelineTests, $cp370Serialization, $cp370SnapshotSerialization,
        $cp370ArbitraryAssertions, $cp370Audit
    )) {
    Assert-LineLimit -Path $bounded -Limit 500 -Description "CP370 bounded structure"
}

# The raw source and exact one-line boundary are pinned; line 2258 is dynamic false continuation only.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp370Source).Hash -cne $cp370SourceHash) {
    throw "CP370 pinned PurchasedAirManager.cc hash drifted"
}
$cp370SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp370Source
if ($cp370SourceLines[2245].Trim() -cne 'if (PurchAir.HumidCtrlType == HumControl::Humidistat) {' -or
    $cp370SourceLines[2246].Trim() -cne 'if ((PurchAir.DehumidCtrlType == HumControl::Humidistat) || (PurchAir.DehumidCtrlType == HumControl::None)) {' -or
    $cp370SourceLines[2257].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP370 pinned physical lines 2246, 2247, or 2258 drifted"
}
Assert-Contains -Path $cp370Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2246' -Description "CP370 source line"
Assert-Contains -Path $cp370Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2247' -Description "CP370 first excluded executable"
Assert-ExactStringArray -Path $cp370Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_CONTROL_HUMIDISTAT_GUARD_SOURCE_ORDER" -Expected $cp370Sites -Description "CP370 exact source sites"

# Routes, counters, and snapshots retain U/N/P, CP369 HeatOn-false, and active true/false outcomes.
$cp370RouteBlock = Get-Cp370RustBraceBlock -Text (Read-RepoText -Path $cp370State) -AnchorPattern 'enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRetainedRoute\s*\{' -Description "CP370 retained-route enum"
[string[]]$cp370Routes = @([regex]::Matches($cp370RouteBlock, '(?m)^\s{4}(?<route>[A-Z][A-Za-z0-9]+),\s*$') | ForEach-Object { $_.Groups["route"].Value })
$cp370ExpectedRoutes = @(
    "UnitOff", "NonCooling", "PositiveGuardFalseFallthrough",
    "HeatingAvailabilityGuardFalseFallthrough", "HumidificationControlBodyEntered",
    "HumidificationControlGuardFalseFallthrough"
)
if (($cp370Routes -join "|") -cne ($cp370ExpectedRoutes -join "|")) {
    throw "CP370 retained routes must be exactly U/N/P/HeatOn-false/Humidistat-body/false-fallthrough"
}
foreach ($counter in @(
        "transition_count", "unit_off_skip_count", "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_completed_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
        "heating_on_read_count", "heating_on_body_entry_count",
        "heating_on_guard_false_fallthrough_count",
        "humidification_control_type_read_count",
        "humidification_control_type_humidistat_comparison_count",
        "humidification_control_body_entry_count",
        "humidification_control_guard_false_fallthrough_count",
        "source_site_execution_count"
    )) {
    Assert-Contains -Path $cp370State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP370 counter '$counter'"
}
foreach ($field in @(
        'pub humidification_control_type_read:\s*bool',
        'pub humidification_control_type:\s*Option<HumidificationControlType>',
        'pub humidification_control_type_humidistat:\s*Option<bool>',
        'pub humidification_control_body_entered:\s*bool',
        'pub humidification_control_guard_false_fallthrough:\s*bool'
    )) {
    Assert-Contains -Path $cp370Module -Pattern $field -Description "CP370 snapshot field '$field'"
}

# CP369 body entries read and compare the typed selector; true executes three sites and false two.
$cp370TransitionText = Read-RepoText -Path $cp370Transition
foreach ($pattern in @(
        'PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Predecessor',
        'humidification_control_type == HumidificationControlType::Humidistat',
        'humidification_control_type_read_count \+= 1;',
        'humidification_control_type_humidistat_comparison_count \+= 1;',
        'humidification_control_body_entry_count \+= 1;',
        'humidification_control_guard_false_fallthrough_count \+= 1;',
        'source_site_execution_count \+= 3;',
        'source_site_execution_count \+= 2;',
        'humidification_control_type:\s*evaluate\.then_some\(humidification_control_type\)',
        'humidification_control_type_humidistat:\s*humidistat',
        'humidification_control_body_entered:\s*body_entered',
        'humidification_control_guard_false_fallthrough:\s*false_fallthrough',
        'source_site_execution_count\.checked_add\(3\)',
        'source_site_execution_count\.checked_add\(2\)'
    )) {
    Assert-Cp370TextContains -Text $cp370TransitionText -Pattern $pattern -Description "transition contract '$pattern'"
}
if ([regex]::Matches($cp370TransitionText, 'state\.source_site_execution_count \+= 3;').Count -ne 1 -or
    [regex]::Matches($cp370TransitionText, 'state\.source_site_execution_count \+= 2;').Count -ne 1) {
    throw "CP370 transition must contain exactly one three-site true increment and one two-site false increment"
}
$cp370RuntimeText = Read-RepoText -Path $cp370Runtime
foreach ($pattern in @(
        'humidification_control_type_read_count',
        'humidification_control_type_humidistat_comparison_count',
        'humidification_control_body_entry_count',
        'humidification_control_guard_false_fallthrough_count',
        'source_site_execution_count', 'checked_mul\(2\)', 'checked_add',
        'route_partition == state\.transition_count', 'latest_route_is_counted'
    )) {
    Assert-Cp370TextContains -Text $cp370RuntimeText -Pattern $pattern -Description "checked lifecycle identity '$pattern'"
}

# CP369 is the sole immediate predecessor; the selected system owns the operand and CP320 corroborates it.
$cp370CoreText = (@($cp370Module) + @(
        Get-ChildItem -LiteralPath $cp370ModuleRoot -Recurse -File -Filter "*.rs" | ForEach-Object { $_.FullName }
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($pattern in @(
        'completed_direct_cooling_supply_humidity_ratio_humidification_heating_availability_guard_is_consistent',
        'cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release',
        'cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact',
        'cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witness'
    )) {
    Assert-Cp370TextContains -Text $cp370CoreText -Pattern $pattern -Description "recursive CP369 predecessor '$pattern'"
}
$cp370PublicRelease = Get-Cp370RustBraceBlock -Text (Read-RepoText -Path $cp370Release) -AnchorPattern 'pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\s*\(' -Description "CP370 public release"
Assert-Cp370TextNotContains -Text $cp370PublicRelease -Pattern 'humidification_control_type\s*:\s*HumidificationControlType' -Description "caller-supplied selector operand"
Assert-Cp370TextContains -Text $cp370PublicRelease -Pattern 'system\.humidification_control_type' -Description "selected-system selector owner"
Assert-Cp370TextContains -Text $cp370PublicRelease -Pattern 'HumidificationControlType::None' -Description "direct None selector"
foreach ($pattern in @(
        'cooling_humidification_flow_latest_witness',
        'cooling_humidification_flow_snapshot_is_exact_direct_release',
        'let owner = system\.humidification_control_type',
        'cp320 == cp320_witness',
        'cp320\.humidification_control_type == Some\(owner\)'
    )) {
    Assert-Contains -Path $cp370Prefix -Pattern $pattern -Description "CP320 selector corroboration '$pattern'"
}
Assert-Contains -Path $cp370Snapshot -Pattern '(?s)HumidificationControlGuardFalseFallthrough\).*?HumidificationControlType::None' -Description "exact direct None false route"
Assert-Contains -Path $cp370Private -Pattern 'HumidificationControlType::Humidistat' -Description "canonical private typed Humidistat"
Assert-Contains -Path $cp370Private -Pattern 'calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard\s*\.latest' -Description "private CP369 lineage"

# CP370 is named-enum/control-only and never carries numerical humidity work.
$cp370SnapshotDto = Get-Cp370RustBraceBlock -Text (Read-RepoText -Path $cp370Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot\s*\{' -Description "CP370 snapshot DTO"
$cp370StateDto = Get-Cp370RustBraceBlock -Text (Read-RepoText -Path $cp370State) -AnchorPattern 'pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState\s*\{' -Description "CP370 runtime-state DTO"
$cp370LifecycleDto = Get-Cp370RustBraceBlock -Text (Read-RepoText -Path $cp370Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary\s*\{' -Description "CP370 lifecycle DTO"
foreach ($dto in @($cp370SnapshotDto, $cp370StateDto, $cp370LifecycleDto)) {
    Assert-Cp370TextNotContains -Text $dto -Pattern 'f64|ieee|bits|Option\s*<\s*f64\s*>|(?:assigned|resulting|minimum|maximum|final)[A-Za-z0-9_]*humidity_ratio\s*:' -Description "DTO numerical humidity payload"
}
foreach ($file in @($cp370Transition, $cp370Release, $cp370Prefix, $cp370Private)) {
    Assert-NotContains -Path $file -Pattern 'to_bits|from_bits|\.is_finite\(\)|f64::|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply|SupplyHumRatOrig|as\s+(?:u|i)(?:8|16|32|64|size)' -Description "CP370 numeric/discriminant/psychrometric firewall"
}
Assert-NotContains -Path $cp370Serialization -Pattern '_ieee_bits|json_number|to_bits|from_bits|\bf64\b|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio' -Description "CP370 control-only lifecycle JSON"
$cp370SnapshotSerializationText = Read-RepoText -Path $cp370SnapshotSerialization
$cp370SnapshotTestBoundary = [regex]::Match($cp370SnapshotSerializationText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
$cp370SnapshotProduction = if ($cp370SnapshotTestBoundary.Success) {
    $cp370SnapshotSerializationText.Substring(0, $cp370SnapshotTestBoundary.Index)
} else { $cp370SnapshotSerializationText }
Assert-Cp370TextNotContains -Text $cp370SnapshotProduction -Pattern '_ieee_bits|json_number|to_bits|from_bits|\bf64\b|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio' -Description "control-only snapshot JSON"
# Searchable regressions, registration, source order, and terminal arbitrary-run ownership.
$cp370SemanticText = (@(
        $cp370CoreTests, $cp370ReleaseTests, $cp370BindingTests, $cp370CoupledTests,
        $cp370PipelineRoot, $cp370PipelineTests, $cp370SnapshotSerialization,
        $cp370ArbitraryAssertions
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($pattern in @(
        'HumidificationControlType::None', 'HumidificationControlType::Humidistat',
        'source_site_execution_count',
        'non_direct_runtime_rejects_cp316_through_cp387_lifecycle_evidence'
    )) {
    Assert-Cp370TextContains -Text $cp370SemanticText -Pattern $pattern -Description "semantic regression '$pattern'"
}
foreach ($test in @(
        "missing_direct_lifecycle_fails_closed",
        "three_site_guard_partition_and_source_product_are_checked",
        "expected_snapshot_preserves_skips_and_direct_none_false_route",
        "direct_release_requires_exact_cp369_and_none_false_route",
        "direct_none_serializes_false_guard_with_two_sites",
        "private_humidistat_serializes_true_body_with_three_sites",
        "control_only_snapshot_exposes_no_numeric_humidity_payload",
        "binding_orders_cp369_then_cp370_before_numerical_and_does_not_feed_result",
        "binding_cp370_skips_control_sites_for_u_n_and_p_routes"
    )) {
    Assert-Cp370TextContains -Text $cp370SemanticText -Pattern ('(?m)fn\s+' + $test + '\s*\(') -Description "semantic regression '$test'"
}
Assert-Contains -Path $cp370PipelineValidation -Pattern 'heating_availability_guard_cp369' -Description "exact CP369 pipeline predecessor field"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp370CalcRoot; Pattern = $cp370Stem; Description = "calc registration" },
        [PSCustomObject]@{ Path = $cp370BindingAdapter; Pattern = "advance_direct_no_oa_calc_$cp370Stem"; Description = "binding adapter" },
        [PSCustomObject]@{ Path = $cp370ScheduledOutput; Pattern = "pub calculation_${cp370Stem}:"; Description = "scheduled output" },
        [PSCustomObject]@{ Path = $cp370BindingTestsRoot; Pattern = $cp370Stem; Description = "binding-test registration" },
        [PSCustomObject]@{ Path = $cp370InitState; Pattern = $cp370Stem; Description = "runtime state" },
        [PSCustomObject]@{ Path = $cp370InitUnit; Pattern = $cp370Stem; Description = "unit state" },
        [PSCustomObject]@{ Path = $cp370InitWitnessRoot; Pattern = $cp370Stem; Description = "witness registration" },
        [PSCustomObject]@{ Path = $cp370CoupledRoot; Pattern = "mod ${cp370Stem}_validation;"; Description = "coupled validator" },
        [PSCustomObject]@{ Path = $cp370FixtureRoot; Pattern = $cp370Stem; Description = "fixture registration" },
        [PSCustomObject]@{ Path = $cp370CoupledTestsRoot; Pattern = "coupled_runtime_tests_cp370"; Description = "coupled-test registration" },
        [PSCustomObject]@{ Path = $cp370PipelineRoot; Pattern = "mod ${cp370PipelineStem};"; Description = "pipeline module" },
        [PSCustomObject]@{ Path = $cp370PipelineRoot; Pattern = "`"$cp370Lifecycle`":\s*result\s*\.$cp370Lifecycle"; Description = "lifecycle JSON" }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP370 $($registration.Description)"
}
$cp370BindingText = Read-RepoText -Path $cp370Binding
$cp369BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_${cp369StemForCp370} =")
$cp370BindingIndex = $cp370BindingText.IndexOf("let calculation_${cp370Stem} =")
$cp371BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp370 = $cp370BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp370NumericalIndex = $cp370BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp369BindingIndexForCp370 -lt 0 -or $cp370BindingIndex -le $cp369BindingIndexForCp370 -or
    $cp371BindingIndexForCp370 -le $cp370BindingIndex -or
    $cp372BindingIndexForCp370 -le $cp371BindingIndexForCp370 -or
    $cp373BindingIndexForCp370 -le $cp372BindingIndexForCp370 -or
    $cp374BindingIndexForCp370 -le $cp373BindingIndexForCp370 -or
    $cp375BindingIndexForCp370 -le $cp374BindingIndexForCp370 -or
    $cp376BindingIndexForCp370 -le $cp375BindingIndexForCp370 -or $cp377BindingIndexForCp370 -le $cp376BindingIndexForCp370 -or $cp378BindingIndexForCp370 -le $cp377BindingIndexForCp370 -or $cp379BindingIndexForCp370 -le $cp378BindingIndexForCp370 -or $cp380BindingIndexForCp370 -le $cp379BindingIndexForCp370 -or $cp381BindingIndexForCp370 -le $cp380BindingIndexForCp370 -or $cp382BindingIndexForCp370 -le $cp381BindingIndexForCp370 -or $cp383BindingIndexForCp370 -le $cp382BindingIndexForCp370 -or $cp384BindingIndexForCp370 -le $cp383BindingIndexForCp370 -or $cp385BindingIndexForCp370 -le $cp384BindingIndexForCp370 -or $cp370NumericalIndex -le $cp385BindingIndexForCp370) {
    throw "Binding must execute CP369 then CP370 before unchanged numerical coupling"
}
$cp370NumericalDto = Get-Cp370RustBraceBlock -Text $cp370BindingText.Substring($cp370NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP370 numerical DTO"
if ($cp370NumericalDto -match '(?i)cp370|humidification_control_humidistat_guard') {
    throw "CP370 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp370ParentAssertions -Pattern 'mod cp370_assertions;' -Description "CP370 arbitrary delegation module"
Assert-Contains -Path $cp370ParentAssertions -Pattern 'cp370_assertions::assert_direct\(runtime, results\)' -Description "CP370 arbitrary direct delegation"
Assert-Contains -Path $cp370ParentAssertions -Pattern 'cp370_assertions::assert_non_direct\(runtime\)' -Description "CP370 arbitrary non-direct delegation"
Assert-NotContains -Path $cp370ParentAssertions -Pattern 'assert_numerical_nonfeed\(\s*runtime, results' -Description "CP369 relinquishes terminal nonfeed"
Assert-Contains -Path $cp370ArbitraryAssertions -Pattern 'mod cp371_assertions;' -Description "CP371 arbitrary delegation module"
Assert-Contains -Path $cp370ArbitraryAssertions -Pattern 'cp371_assertions::assert_direct\(runtime, results\)' -Description "CP371 arbitrary direct delegation"
Assert-Contains -Path $cp370ArbitraryAssertions -Pattern 'cp371_assertions::assert_non_direct\(runtime\)' -Description "CP371 arbitrary non-direct delegation"
Assert-NotContains -Path $cp370ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP370 relinquishes terminal nonfeed"
Assert-Contains -Path $cp370ArbitraryAssertions -Pattern 'runtime\.contains_key\(CP370_KEY\)' -Description "CP370 non-direct key"
Assert-Contains -Path $cp370ArbitraryAssertions -Pattern 'runtime\[CP370_KEY\]\.is_null\(\)' -Description "CP370 non-direct null"
# Exactly two algorithm/capability addenda and stable targets.
$cp370AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp370CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp370AlgorithmAddenda = [regex]::Matches($cp370AlgorithmText, '(?m)^\s*"CP370 supersedes only CP369[^"\r\n]+",\s*$')
$cp370CapabilityAddenda = [regex]::Matches($cp370CapabilityText, '(?m)^\s*"CP370 additionally requires[^"\r\n]+",\s*$')
if ($cp370AlgorithmAddenda.Count -ne 2 -or $cp370CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP370 addenda"
}
foreach ($claim in @($cp370AlgorithmAddenda) + @($cp370CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp370SourceCommit, $cp370SourceHash, 'physical executable line 2246',
            $cp370Sites[0], $cp370Sites[1], $cp370Sites[2],
            'physical executable line 2247', 'physical executable line 2258',
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'V=CP369 body entries=M\+Z',
            'source_site_execution_count=2\*V\+M',
            'humidification_control_type=None', 'M=0', 'Z=V', '2\*V',
            'typed-enum `Humidistat`', 'M=V', 'Z=0', '3\*V',
            'CP369', 'sole immediate source-order predecessor', 'CP320', 'corroborat',
            'named-enum', 'no discriminant or ordinal claim',
            'CP369-to-CP370-to-unchanged-?\s*numerical', $cp370Lifecycle, 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '308 total', '240 public', '68 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP370 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp370Stem/release.rs::advance_direct_no_oa_calc_$cp370Stem"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp370Stem.rs::purchased_air_calc_${cp370Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp370Stem.rs::${cp370TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp370Stem.rs::${cp370TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp370AlgorithmText, [regex]::Escape($target.Value)).Count -ne $target.Expected) {
        throw "CP370 target count failed for '$($target.Value)'"
    }
}

# Five hand-doc sections and non-promotion to psychrometrics.
$cp370Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP370 Cooling Supply-Humidity-Ratio Humidification-Control Humidistat Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP370 Source-Ordered Cooling Supply-Humidity-Ratio Humidification-Control Humidistat Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP370 Cooling Supply-Humidity-Ratio Humidification-Control Humidistat Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP370 Cooling Supply-Humidity-Ratio Humidification-Control Humidistat Guard in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP370 Cooling Supply-Humidity-Ratio Humidification-Control Humidistat Guard Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp370Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP370 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp370SourceCommit, $cp370SourceHash, '2246', '2247', '2258',
            $cp370Sites[0], $cp370Sites[1], $cp370Sites[2],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L', 'V\s*=\s*CP369 body entries\s*=\s*M\+Z',
            'source_site_execution_count\s*=\s*2\*V\+M',
            'HumidificationControlType::None|humidification_control_type=None',
            'HumidificationControlType::Humidistat|typed-enum\s+`Humidistat`|canonical private(?:\s+typed-enum)?\s+`Humidistat`',
            'CP369', '(?:sole\s+immediate|immediate\s+and\s+sole)\s+source-order\s+predecessor',
            'CP320', 'corroborat', 'named-enum', 'CP369-to-CP370-to-unchanged-?\s*numerical',
            $cp370Lifecycle, 'CP345', '32\s+algorithms', '293\s+routines',
            '58\s+`?state_mapped`?', '235\s+`?source_mapped`?', '170\s+required',
            '308\s+total', '240\s+public', '68\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP370 documentation in $($doc.Path) missing '$pattern'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP370\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP370 supersedes only CP369' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP370 additionally requires' -Description "generated capability addendum"

# Historical source order, helper whitelist, firewall, generated totals, and inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..369 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard' -Description "historical CP370 binding order"
}
foreach ($historical in 327..328) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard' -Description "out-of-range CP370 binding token"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard' -Description "historical CP370 helper whitelist"
}
foreach ($historical in @(327, 328) + @(346..369)) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard' -Description "out-of-range CP370 helper token"
}
foreach ($historical in 334..369) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp387_lifecycle_evidence' -Description "historical CP370 firewall"
}
foreach ($historical in 335..369) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 325 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 85 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..369) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 325' -Description "historical inventory total"
}

$cp370MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp369AuditIndexForCp370 = $cp370MainAuditText.IndexOf("cp369-cooling-supply-humidity-ratio-humidification-heating-availability-guard.ps1")
$cp370AuditIndex = $cp370MainAuditText.IndexOf("cp370-cooling-supply-humidity-ratio-humidification-control-humidistat-guard.ps1")
$cp370CompletionIndex = $cp370MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp369AuditIndexForCp370 -lt 0 -or $cp370AuditIndex -le $cp369AuditIndexForCp370 -or $cp370CompletionIndex -le $cp370AuditIndex) {
    throw "Master audit must dot-source CP370 after CP369 before completion"
}
$cp370InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp370TextContains -Text $cp370InventoryText -Pattern 'script_count = 325' -Description "script total"
Assert-Cp370TextContains -Text $cp370InventoryText -Pattern 'unused_script_count = 0' -Description "zero uncalled"
if ([regex]::Matches($cp370InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($cp370InventoryText, '(?m)^classification = "internal"$').Count -ne 85) {
    throw "CP370 inventory must be exactly 240 public and 85 internal scripts"
}
Assert-Cp370TextContains -Text $cp370InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp370-' -Description "inventory record"
Assert-Cp370TextContains -Text $cp370InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 325 \|' -Description "CP370 generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP370 generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 85 \|' -Description "CP370 generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP370 generated uncalled"

Write-Host "CP370 Cooling supply-humidity-ratio humidification-control Humidistat guard structure audit passed."
