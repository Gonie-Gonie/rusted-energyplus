# CP369 maps PurchasedAirManager.cc physical executable line 2245's
# Cooling supply-humidity-ratio humidification HeatOn guard and its two sites.
$cp369Stem = "cooling_supply_humidity_ratio_humidification_heating_availability_guard"
$cp368StemForCp369 = "cooling_default_supply_humidity_ratio_case_break"
$cp369PipelineStem = "purchased_air_$cp369Stem"
$cp369TypeStem = "PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuard"
$cp369Lifecycle = "purchased_air_calc_${cp369Stem}_lifecycle"
$cp369SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp369SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp369Sites = @(
    "read-local-heating-on-for-cooling-humidification-guard",
    "enter-cooling-supply-humidity-ratio-humidification-body-if-heating-on"
)
$cp369Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp369Module = "crates\ep_runtime\src\ideal_loads\calc\$cp369Stem.rs"
$cp369ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp369Stem"
$cp369State = "$cp369ModuleRoot\state.rs"
$cp369Transition = "$cp369ModuleRoot\transition.rs"
$cp369Release = "$cp369ModuleRoot\release.rs"
$cp369Prefix = "$cp369ModuleRoot\release\prefix_validation.rs"
$cp369Private = "$cp369ModuleRoot\release\private_counterfactual.rs"
$cp369Runtime = "$cp369ModuleRoot\release\runtime_validation.rs"
$cp369Snapshot = "$cp369ModuleRoot\release\snapshot_validation.rs"
$cp369CoreTests = "$cp369ModuleRoot\tests\mod.rs"
$cp369ReleaseTests = "$cp369ModuleRoot\tests\release.rs"
$cp369CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp369Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp369BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp369Stem.rs"
$cp369BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp369BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp369Stem}_tests.rs"
$cp369ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp369InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp369InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp369InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp369InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp369Stem.rs"
$cp369CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp369Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp369Stem}_validation.rs"
$cp369CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp369CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp369.rs"
$cp369FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp369Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp369Stem}_fixture.rs"
$cp369PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp369Pipeline = "crates\ep_run\src\pipeline\$cp369PipelineStem.rs"
$cp369PipelineValidation = "crates\ep_run\src\pipeline\$cp369PipelineStem\validation.rs"
$cp369PipelineTests = "crates\ep_run\src\pipeline\$cp369PipelineStem\validation\tests.rs"
$cp369Serialization = "crates\ep_run\src\pipeline\$cp369PipelineStem\serialization.rs"
$cp369SnapshotSerialization = "crates\ep_run\src\pipeline\$cp369PipelineStem\serialization\snapshot.rs"
$cp369ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp368_assertions.rs"
$cp369ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp369_assertions.rs"
$cp369Audit = "scripts\quality\ideal-loads-structure-audit\cp369-cooling-supply-humidity-ratio-humidification-heating-availability-guard.ps1"

function Assert-Cp369TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP369 $Description missing" }
}
function Assert-Cp369TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP369 $Description unexpectedly present" }
}
function Get-Cp369RustBraceBlock {
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
        $cp369Source, $cp369Module, $cp369State, $cp369Transition, $cp369Release,
        $cp369Prefix, $cp369Private, $cp369Runtime, $cp369Snapshot, $cp369CoreTests,
        $cp369ReleaseTests, $cp369BindingAdapter, $cp369BindingTests, $cp369InitWitness,
        $cp369Coupled, $cp369CoupledTests, $cp369Fixture, $cp369Pipeline,
        $cp369PipelineValidation, $cp369PipelineTests, $cp369Serialization,
        $cp369SnapshotSerialization, $cp369ParentAssertions, $cp369ArbitraryAssertions, $cp369Audit
    )) {
    Assert-FileExists -Path $required -Description "CP369 structure"
}
foreach ($bounded in @(
        $cp369Module, $cp369State, $cp369Transition, $cp369Release, $cp369Prefix,
        $cp369Private, $cp369Runtime, $cp369Snapshot, $cp369CoreTests, $cp369ReleaseTests,
        $cp369BindingAdapter, $cp369BindingTests, $cp369InitWitness, $cp369Coupled,
        $cp369CoupledTests, $cp369Fixture, $cp369Pipeline, $cp369PipelineValidation,
        $cp369PipelineTests, $cp369Serialization, $cp369SnapshotSerialization,
        $cp369ArbitraryAssertions, $cp369Audit
    )) {
    Assert-LineLimit -Path $bounded -Limit 500 -Description "CP369 bounded structure"
}

# The raw source and exact one-line boundary are pinned; line 2258 is dynamic false continuation only.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp369Source).Hash -cne $cp369SourceHash) {
    throw "CP369 pinned PurchasedAirManager.cc hash drifted"
}
$cp369SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp369Source
if ($cp369SourceLines[2244].Trim() -cne 'if (HeatOn) {' -or
    $cp369SourceLines[2245].Trim() -cne 'if (PurchAir.HumidCtrlType == HumControl::Humidistat) {' -or
    $cp369SourceLines[2257].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP369 pinned physical lines 2245, 2246, or 2258 drifted"
}
Assert-Contains -Path $cp369Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2245' -Description "CP369 source line"
Assert-Contains -Path $cp369Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2246' -Description "CP369 first excluded executable"
Assert-ExactStringArray -Path $cp369Module -Name "PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER" -Expected $cp369Sites -Description "CP369 exact source sites"

# Routes and counters encode U/N/P plus active true/false guard outcomes.
$cp369RouteBlock = Get-Cp369RustBraceBlock -Text (Read-RepoText -Path $cp369State) -AnchorPattern 'enum PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRetainedRoute\s*\{' -Description "CP369 retained-route enum"
[string[]]$cp369Routes = @([regex]::Matches($cp369RouteBlock, '(?m)^\s{4}(?<route>[A-Z][A-Za-z0-9]+),\s*$') | ForEach-Object { $_.Groups["route"].Value })
$cp369ExpectedRoutes = @(
    "UnitOff", "NonCooling", "PositiveGuardFalseFallthrough",
    "HeatingAvailabilityBodyEntered", "HeatingAvailabilityGuardFalseFallthrough"
)
if (($cp369Routes -join "|") -cne ($cp369ExpectedRoutes -join "|")) {
    throw "CP369 retained routes must be exactly U/N/P/entered/false-fallthrough"
}
foreach ($counter in @(
        "transition_count", "unit_off_skip_count", "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_completed_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count",
        "heating_on_read_count", "heating_on_body_entry_count",
        "heating_on_guard_false_fallthrough_count", "source_site_execution_count"
    )) {
    Assert-Contains -Path $cp369State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP369 counter '$counter'"
}
foreach ($field in @(
        'pub heating_on_read:\s*bool', 'pub heating_on:\s*Option<bool>',
        'pub cooling_supply_humidity_ratio_humidification_body_entered:\s*bool',
        'pub heating_on_guard_false_fallthrough:\s*bool'
    )) {
    Assert-Contains -Path $cp369Module -Pattern $field -Description "CP369 snapshot field '$field'"
}

# Every active C0/Q/H/CSH route reads HeatOn; true executes both sites and false only the read.
$cp369TransitionText = Read-RepoText -Path $cp369Transition
foreach ($pattern in @(
        'PredecessorRoute::NoneCaseCompleted',
        'PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted',
        'PredecessorRoute::HumidistatCaseCompleted',
        'PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted',
        '(?s)if active \{.*?heating_on_read_count \+= 1;.*?source_site_execution_count \+= 1;.*?if heating_on \{.*?heating_on_body_entry_count \+= 1;.*?source_site_execution_count \+= 1;.*?\} else \{.*?heating_on_guard_false_fallthrough_count \+= 1;',
        'heating_on:\s*active\.then_some\(heating_on\)',
        'cooling_supply_humidity_ratio_humidification_body_entered:\s*active && heating_on',
        'heating_on_guard_false_fallthrough:\s*active && !heating_on',
        'source_site_execution_count\.checked_add\(2\)',
        'source_site_execution_count\.checked_add\(1\)'
    )) {
    Assert-Cp369TextContains -Text $cp369TransitionText -Pattern $pattern -Description "transition algebra '$pattern'"
}
if ([regex]::Matches($cp369TransitionText, 'state\.source_site_execution_count \+= 1;').Count -ne 2) {
    throw "CP369 transition must contain exactly the ordered read and true-body source increments"
}
$cp369RuntimeText = Read-RepoText -Path $cp369Runtime
foreach ($pattern in @(
        '(?s)Some\(active\)\s*=\s*checked_sum\(&\[.*?none_case_completed_skip_count.*?constant_sensible_heat_ratio_case_completed_skip_count.*?humidistat_case_completed_skip_count.*?constant_supply_humidity_ratio_case_completed_skip_count',
        '(?s)Some\(guard_partition\)\s*=\s*state\s*\.heating_on_body_entry_count\s*\.checked_add\(state\.heating_on_guard_false_fallthrough_count\)',
        '(?s)Some\(source_count\)\s*=\s*state\s*\.heating_on_read_count\s*\.checked_add\(state\.heating_on_body_entry_count\)',
        'state\.heating_on_read_count == active', 'guard_partition == active',
        'state\.source_site_execution_count == source_count',
        'route_partition == state\.transition_count', 'latest_route_is_counted'
    )) {
    Assert-Cp369TextContains -Text $cp369RuntimeText -Pattern $pattern -Description "checked lifecycle identity '$pattern'"
}

# CP368 is the sole immediate predecessor; CP310 owns HeatOn and CP320 only corroborates it.
Assert-Contains -Path $cp369Transition -Pattern 'PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot as Predecessor' -Description "exact CP368 predecessor type"
$cp369CoreText = (@($cp369Module) + @(
        Get-ChildItem -LiteralPath $cp369ModuleRoot -Recurse -File -Filter "*.rs" | ForEach-Object { $_.FullName }
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($pattern in @(
        'completed_direct_cooling_default_supply_humidity_ratio_case_break_is_consistent',
        'cooling_default_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release',
        'cooling_default_supply_humidity_ratio_case_break_snapshots_match_exact',
        'cooling_default_supply_humidity_ratio_case_break_latest_witness',
        'private_default_supply_humidity_ratio_case_break_csh_counterfactual_from_direct_release'
    )) {
    Assert-Cp369TextContains -Text $cp369CoreText -Pattern $pattern -Description "recursive CP368 owner '$pattern'"
}
Assert-Contains -Path $cp369Release -Pattern '(?s)let heating_on = unit\s*\.calc_entry\s*\.latest.*?\.heating_on;' -Description "sole CP310 retained HeatOn operand"
foreach ($pattern in @(
        'fn heating_on_provenance_is_exact', 'unit\.calc_entry\.latest',
        'unit\.calc_cooling_humidification_flow\.latest',
        'cooling_humidification_flow_latest_witness',
        'cooling_humidification_flow_snapshot_is_exact_direct_release',
        'cp320 == cp320_witness', 'cp320\.heating_on == Some\(heating_on\)'
    )) {
    Assert-Contains -Path $cp369Prefix -Pattern $pattern -Description "CP310 owner / CP320 corroboration '$pattern'"
}
$cp369PublicRelease = Get-Cp369RustBraceBlock -Text (Read-RepoText -Path $cp369Release) -AnchorPattern 'pub fn advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard\s*\(' -Description "CP369 public release"
Assert-Cp369TextNotContains -Text $cp369PublicRelease -Pattern 'heating_on\s*:\s*bool' -Description "caller-supplied HeatOn operand"
Assert-Cp369TextContains -Text $cp369PublicRelease -Pattern 'if !heating_on' -Description "direct HeatOn-false rejection"
Assert-Cp369TextContains -Text $cp369PublicRelease -Pattern 'system\.dehumidification_control_type != DehumidificationControlType::None' -Description "direct C0 selector"
Assert-Contains -Path $cp369Snapshot -Pattern '(?s)HeatingAvailabilityBodyEntered\).*?DehumidificationControlType::None.*?dehumidification_control_none_case_completed_skip' -Description "exact direct C0 body entry"
Assert-Contains -Path $cp369Snapshot -Pattern 'HeatingAvailabilityGuardFalseFallthrough\) \| None => false' -Description "false guard excluded from release"
Assert-Contains -Path $cp369Private -Pattern '(?s)cp368_private_csh_from_direct_release\(.*?\)\?;.*?advance\(&mut state, private_cp368, true\)\?' -Description "canonical private CSH true-body entry"

# CP369 is Boolean/control-only and never carries numerical humidity work.
$cp369SnapshotDto = Get-Cp369RustBraceBlock -Text (Read-RepoText -Path $cp369Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot\s*\{' -Description "CP369 snapshot DTO"
$cp369StateDto = Get-Cp369RustBraceBlock -Text (Read-RepoText -Path $cp369State) -AnchorPattern 'pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState\s*\{' -Description "CP369 runtime-state DTO"
$cp369LifecycleDto = Get-Cp369RustBraceBlock -Text (Read-RepoText -Path $cp369Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleSummary\s*\{' -Description "CP369 lifecycle DTO"
foreach ($dto in @($cp369SnapshotDto, $cp369StateDto, $cp369LifecycleDto)) {
    Assert-Cp369TextNotContains -Text $dto -Pattern 'f64|ieee|bits|Option\s*<\s*f64\s*>|(?:assigned|resulting|minimum|maximum|final)[A-Za-z0-9_]*humidity_ratio\s*:' -Description "DTO numerical humidity payload"
}
foreach ($file in @($cp369Transition, $cp369Release, $cp369Prefix, $cp369Private)) {
    Assert-NotContains -Path $file -Pattern 'to_bits|from_bits|\.is_finite\(\)|f64::|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply|SupplyHumRatOrig' -Description "CP369 numeric/psychrometric firewall"
}
Assert-NotContains -Path $cp369Serialization -Pattern '_ieee_bits|json_number|to_bits|from_bits|\bf64\b|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio' -Description "CP369 control-only lifecycle JSON"
$cp369SnapshotSerializationText = Read-RepoText -Path $cp369SnapshotSerialization
$cp369SnapshotTestBoundary = [regex]::Match($cp369SnapshotSerializationText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
$cp369SnapshotProduction = if ($cp369SnapshotTestBoundary.Success) {
    $cp369SnapshotSerializationText.Substring(0, $cp369SnapshotTestBoundary.Index)
} else { $cp369SnapshotSerializationText }
Assert-Cp369TextNotContains -Text $cp369SnapshotProduction -Pattern '_ieee_bits|json_number|to_bits|from_bits|\bf64\b|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio' -Description "control-only snapshot JSON"

# Searchable regressions, registration, source order, and terminal arbitrary-run ownership.
$cp369SemanticText = (@(
        $cp369CoreTests, $cp369ReleaseTests, $cp369BindingTests, $cp369CoupledTests,
        $cp369PipelineRoot, $cp369PipelineTests, $cp369SnapshotSerialization, $cp369ArbitraryAssertions
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($test in @(
        "active_true_and_false_routes_use_two_site_contract",
        "all_four_completed_selector_routes_reach_the_guard",
        "inactive_routes_skip_both_sites_and_input_boolean",
        "executed_untyped_default_break_is_rejected",
        "binding_orders_cp368_then_cp369_before_numerical_and_does_not_feed_result",
        "binding_cp369_skips_heat_on_for_u_n_and_p_routes",
        "two_site_guard_partition_and_source_product_are_checked",
        "expected_snapshot_preserves_inactive_nulls_and_active_true_body",
        "direct_release_and_immediate_cp368_predecessor_are_strict",
        "latest_direct_route_requires_matching_current_and_predecessor_cumulative_evidence",
        "missing_direct_lifecycle_fails_closed",
        "non_direct_runtime_rejects_cp316_through_cp395_lifecycle_evidence"
    )) {
    Assert-Cp369TextContains -Text $cp369SemanticText -Pattern ('(?m)fn\s+' + $test + '\s*\(') -Description "semantic regression '$test'"
}
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp369CalcRoot; Pattern = $cp369Stem; Description = "calc registration" },
        [PSCustomObject]@{ Path = $cp369BindingAdapter; Pattern = "advance_direct_no_oa_calc_$cp369Stem"; Description = "binding adapter" },
        [PSCustomObject]@{ Path = $cp369ScheduledOutput; Pattern = "pub calculation_${cp369Stem}:"; Description = "scheduled output" },
        [PSCustomObject]@{ Path = $cp369BindingTestsRoot; Pattern = $cp369Stem; Description = "binding-test registration" },
        [PSCustomObject]@{ Path = $cp369InitState; Pattern = $cp369Stem; Description = "runtime state" },
        [PSCustomObject]@{ Path = $cp369InitUnit; Pattern = $cp369Stem; Description = "unit state" },
        [PSCustomObject]@{ Path = $cp369InitWitnessRoot; Pattern = $cp369Stem; Description = "witness registration" },
        [PSCustomObject]@{ Path = $cp369CoupledRoot; Pattern = "mod ${cp369Stem}_validation;"; Description = "coupled validator" },
        [PSCustomObject]@{ Path = $cp369FixtureRoot; Pattern = $cp369Stem; Description = "fixture registration" },
        [PSCustomObject]@{ Path = $cp369CoupledTestsRoot; Pattern = "coupled_runtime_tests_cp369"; Description = "coupled-test registration" },
        [PSCustomObject]@{ Path = $cp369PipelineRoot; Pattern = "mod ${cp369PipelineStem};"; Description = "pipeline module" },
        [PSCustomObject]@{ Path = $cp369PipelineRoot; Pattern = "`"$cp369Lifecycle`":\s*result\s*\.$cp369Lifecycle"; Description = "lifecycle JSON" }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP369 $($registration.Description)"
}
$cp369BindingText = Read-RepoText -Path $cp369Binding
$cp368BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_${cp368StemForCp369} =")
$cp369BindingIndex = $cp369BindingText.IndexOf("let calculation_${cp369Stem} =")
$cp370BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp369 = $cp369BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp369NumericalIndex = $cp369BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp368BindingIndexForCp369 -lt 0 -or $cp369BindingIndex -le $cp368BindingIndexForCp369 -or
    $cp370BindingIndexForCp369 -le $cp369BindingIndex -or $cp371BindingIndexForCp369 -le $cp370BindingIndexForCp369 -or
    $cp372BindingIndexForCp369 -le $cp371BindingIndexForCp369 -or
    $cp373BindingIndexForCp369 -le $cp372BindingIndexForCp369 -or
    $cp374BindingIndexForCp369 -le $cp373BindingIndexForCp369 -or
    $cp375BindingIndexForCp369 -le $cp374BindingIndexForCp369 -or
    $cp376BindingIndexForCp369 -le $cp375BindingIndexForCp369 -or $cp377BindingIndexForCp369 -le $cp376BindingIndexForCp369 -or $cp378BindingIndexForCp369 -le $cp377BindingIndexForCp369 -or $cp379BindingIndexForCp369 -le $cp378BindingIndexForCp369 -or $cp380BindingIndexForCp369 -le $cp379BindingIndexForCp369 -or $cp381BindingIndexForCp369 -le $cp380BindingIndexForCp369 -or $cp382BindingIndexForCp369 -le $cp381BindingIndexForCp369 -or $cp383BindingIndexForCp369 -le $cp382BindingIndexForCp369 -or $cp384BindingIndexForCp369 -le $cp383BindingIndexForCp369 -or $cp385BindingIndexForCp369 -le $cp384BindingIndexForCp369 -or $cp369NumericalIndex -le $cp385BindingIndexForCp369) {
    throw "Binding must execute CP368 then CP369 then CP370 before unchanged numerical coupling"
}
$cp369NumericalDto = Get-Cp369RustBraceBlock -Text $cp369BindingText.Substring($cp369NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP369 numerical DTO"
if ($cp369NumericalDto -match '(?i)cp369|heating_availability_guard') {
    throw "CP369 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp369ParentAssertions -Pattern 'mod cp369_assertions;' -Description "CP369 arbitrary delegation module"
Assert-Contains -Path $cp369ParentAssertions -Pattern 'cp369_assertions::assert_direct\(runtime, results\)' -Description "CP369 arbitrary direct delegation"
Assert-Contains -Path $cp369ParentAssertions -Pattern 'cp369_assertions::assert_non_direct\(runtime\)' -Description "CP369 arbitrary non-direct delegation"
Assert-NotContains -Path $cp369ParentAssertions -Pattern 'assert_numerical_nonfeed\(\s*runtime, results' -Description "CP368 relinquishes terminal nonfeed"
Assert-Contains -Path $cp369ArbitraryAssertions -Pattern 'mod cp370_assertions;' -Description "CP370 arbitrary delegation module"
Assert-Contains -Path $cp369ArbitraryAssertions -Pattern 'cp370_assertions::assert_direct\(runtime, results\)' -Description "CP370 arbitrary direct delegation"
Assert-Contains -Path $cp369ArbitraryAssertions -Pattern 'cp370_assertions::assert_non_direct\(runtime\)' -Description "CP370 arbitrary non-direct delegation"
Assert-NotContains -Path $cp369ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(\s*runtime, results' -Description "CP369 relinquishes terminal nonfeed"
Assert-Contains -Path $cp369ArbitraryAssertions -Pattern 'runtime\.contains_key\(CP369_KEY\)' -Description "CP369 non-direct key"
Assert-Contains -Path $cp369ArbitraryAssertions -Pattern 'runtime\[CP369_KEY\]\.is_null\(\)' -Description "CP369 non-direct null"

# Exactly two algorithm/capability addenda and stable targets.
$cp369AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp369CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp369AlgorithmAddenda = [regex]::Matches($cp369AlgorithmText, '(?m)^\s*"CP369 supersedes only CP368[^"\r\n]+",\s*$')
$cp369CapabilityAddenda = [regex]::Matches($cp369CapabilityText, '(?m)^\s*"CP369 additionally requires[^"\r\n]+",\s*$')
if ($cp369AlgorithmAddenda.Count -ne 2 -or $cp369CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP369 addenda"
}
foreach ($claim in @($cp369AlgorithmAddenda) + @($cp369CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp369SourceCommit, $cp369SourceHash, 'physical executable line 2245',
            $cp369Sites[0], $cp369Sites[1], 'physical executable line 2246',
            'physical executable line 2258', 'T=U\+N\+P\+C0\+Q\+H\+CSH',
            'S=C0\+Q\+H\+CSH=R=G\+F\+L', 'A=F\+L', 'E=S=B\+Z',
            'source_site_execution_count=E\+B', 'C0=S=E=B', 'Q=H=CSH=Z=0',
            'source_site_execution_count=2\*E', 'canonical private `CSH`',
            'false.*pure-transition', 'CP368', 'sole immediate source-order predecessor',
            'CP310 `calc_entry\.latest\.heating_on`', 'CP320', 'corroboration',
            'CP368-to-CP369-to-unchanged-numerical', $cp369Lifecycle, 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '307 total', '240 public', '67 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP369 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp369Stem/release.rs::advance_direct_no_oa_calc_$cp369Stem"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp369Stem.rs::purchased_air_calc_${cp369Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp369Stem.rs::${cp369TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp369Stem.rs::${cp369TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp369AlgorithmText, [regex]::Escape($target.Value)).Count -ne $target.Expected) {
        throw "CP369 target count failed for '$($target.Value)'"
    }
}

# Five hand-doc sections and non-promotion to psychrometrics.
$cp369Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP369 Cooling Supply-Humidity-Ratio Humidification Heating-Availability Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP369 Source-Ordered Cooling Supply-Humidity-Ratio Humidification Heating-Availability Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP369 Cooling Supply-Humidity-Ratio Humidification Heating-Availability Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP369 Cooling Supply-Humidity-Ratio Humidification Heating-Availability Guard in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP369 Cooling Supply-Humidity-Ratio Humidification Heating-Availability Guard Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp369Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP369 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp369SourceCommit, $cp369SourceHash, '2245', '2246', '2258',
            $cp369Sites[0], $cp369Sites[1], 'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L', 'E\s*=\s*S\s*=\s*B\+Z',
            'source_site_execution_count\s*=\s*E\+B', 'C0\s*=\s*S\s*=\s*E\s*=\s*B',
            'CP368', '(?:sole\s+immediate|immediate\s+and\s+sole)\s+source-order\s+predecessor',
            'CP310', 'calc_entry\.latest\.heating_on', 'CP320', 'corroborat',
            'CP368-to-CP369-to-unchanged-numerical', $cp369Lifecycle, 'CP345',
            '32\s+algorithms', '293\s+routines', '58\s+`?state_mapped`?',
            '235\s+`?source_mapped`?', '170\s+required', '307\s+total',
            '240\s+public', '67\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP369 documentation in $($doc.Path) missing '$pattern'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP369\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP369 supersedes only CP368' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP369 additionally requires' -Description "generated capability addendum"

# Historical source order, helper whitelist, firewall, generated totals, and inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..368 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard' -Description "historical CP369 binding order"
}
foreach ($historical in 327..328) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard' -Description "out-of-range CP369 binding token"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard' -Description "historical CP369 helper whitelist"
}
foreach ($historical in @(327, 328) + @(346..368)) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard' -Description "out-of-range CP369 helper token"
}
foreach ($historical in 334..368) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp395_lifecycle_evidence' -Description "historical CP369 firewall"
}
foreach ($historical in 326..333) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp395_lifecycle_evidence' -Description "out-of-range CP369 firewall token"
}
foreach ($historical in 335..368) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 333 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 93 \|')) -Description "historical generated internal"
}
foreach ($historical in 326..334) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 333 \|')) -Description "out-of-range generated-total token"
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 93 \|')) -Description "out-of-range generated-internal token"
}
foreach ($historical in 337..368) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 333' -Description "historical inventory total"
}
foreach ($historical in 326..336) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 333' -Description "out-of-range inventory-total token"
}

$cp369MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp368AuditIndexForCp369 = $cp369MainAuditText.IndexOf("cp368-cooling-default-supply-humidity-ratio-case-break.ps1")
$cp369AuditIndex = $cp369MainAuditText.IndexOf("cp369-cooling-supply-humidity-ratio-humidification-heating-availability-guard.ps1")
$cp369CompletionIndex = $cp369MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp368AuditIndexForCp369 -lt 0 -or $cp369AuditIndex -le $cp368AuditIndexForCp369 -or $cp369CompletionIndex -le $cp369AuditIndex) {
    throw "Master audit must dot-source CP369 after CP368 before completion"
}
$cp369InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp369TextContains -Text $cp369InventoryText -Pattern 'script_count = 333' -Description "script total"
Assert-Cp369TextContains -Text $cp369InventoryText -Pattern 'unused_script_count = 0' -Description "zero uncalled"
if ([regex]::Matches($cp369InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp369InventoryText, '(?m)^classification = "internal"$').Count -ne 93) {
throw "CP369 inventory must be exactly 240 public and 93 internal scripts"
}
Assert-Cp369TextContains -Text $cp369InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp369-' -Description "inventory record"
Assert-Cp369TextContains -Text $cp369InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 333 \|' -Description "CP369 generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP369 generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 93 \|' -Description "CP369 generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP369 generated uncalled"

Write-Host "CP369 Cooling supply-humidity-ratio humidification heating-availability guard structure audit passed."
