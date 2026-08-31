# CP364 maps only PurchasedAirManager.cc line 2234's case label.
$cp364Stem = "cooling_constant_supply_humidity_ratio_case_entry"
$cp363StemForCp364 = "cooling_humidistat_case_break"
$cp364PipelineStem = "purchased_air_$cp364Stem"
$cp364TypeStem = "PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntry"
$cp364Lifecycle = "purchased_air_calc_${cp364Stem}_lifecycle"
$cp364SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp364SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp364Sites = @(
    "enter-purchased-air-dehumidification-control-constant-supply-humidity-ratio-case"
)
$cp364Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp364Module = "crates\ep_runtime\src\ideal_loads\calc\$cp364Stem.rs"
$cp364ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp364Stem"
$cp364State = "$cp364ModuleRoot\state.rs"
$cp364Transition = "$cp364ModuleRoot\transition.rs"
$cp364Release = "$cp364ModuleRoot\release.rs"
$cp364Prefix = "$cp364ModuleRoot\release\prefix_validation.rs"
$cp364Private = "$cp364ModuleRoot\release\private_counterfactual.rs"
$cp364Runtime = "$cp364ModuleRoot\release\runtime_validation.rs"
$cp364Snapshot = "$cp364ModuleRoot\release\snapshot_validation.rs"
$cp364Tests = "$cp364ModuleRoot\tests\mod.rs"
$cp364PublicReleaseTests = "$cp364ModuleRoot\tests\public_release.rs"
$cp364CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp364Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp364BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp364Stem.rs"
$cp364BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp364BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp364Stem}_tests.rs"
$cp364ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp364InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp364InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp364InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp364InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp364Stem.rs"
$cp364CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp364Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp364Stem}_validation.rs"
$cp364CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp364CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp364.rs"
$cp364FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp364Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp364Stem}_fixture.rs"
$cp364PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp364Pipeline = "crates\ep_run\src\pipeline\$cp364PipelineStem.rs"
$cp364PipelineValidation = "crates\ep_run\src\pipeline\$cp364PipelineStem\validation.rs"
$cp364PipelineTests = "crates\ep_run\src\pipeline\$cp364PipelineStem\validation\tests.rs"
$cp364Serialization = "crates\ep_run\src\pipeline\$cp364PipelineStem\serialization.rs"
$cp364SnapshotSerialization = "crates\ep_run\src\pipeline\$cp364PipelineStem\serialization\snapshot.rs"
$cp364ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp363_assertions.rs"
$cp364ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp364_assertions.rs"
$cp364ArbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp364Audit = "scripts\quality\ideal-loads-structure-audit\cp364-cooling-constant-supply-humidity-ratio-case-entry.ps1"

function Assert-Cp364TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP364 $Description missing" }
}

function Assert-Cp364TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP364 $Description unexpectedly present" }
}

function Get-Cp364RustBraceBlock {
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

function Assert-Cp364ProductionHasNoPanics {
    param([string]$Path)
    if ((Read-RepoText -Path $Path) -match '(?m)\b(?:expect|unwrap)\s*\(|panic!\s*\(') {
        throw "CP364 production path contains expect/unwrap/panic: $Path"
    }
}

foreach ($required in @(
        $cp364Source, $cp364Module, $cp364State, $cp364Transition, $cp364Release,
        $cp364Prefix, $cp364Private, $cp364Runtime, $cp364Snapshot, $cp364Tests,
        $cp364PublicReleaseTests, $cp364BindingAdapter, $cp364BindingTestsRoot,
        $cp364BindingTests, $cp364InitWitness, $cp364Coupled, $cp364CoupledTestsRoot,
        $cp364CoupledTests, $cp364Fixture, $cp364Pipeline, $cp364PipelineValidation,
        $cp364PipelineTests, $cp364Serialization, $cp364SnapshotSerialization,
        $cp364ParentAssertions, $cp364ArbitraryAssertions, $cp364Audit
    )) {
    Assert-FileExists -Path $required -Description "CP364 structure"
}
$cp364CalcFiles = @($cp364Module) + @(
    Get-ChildItem -LiteralPath $cp364ModuleRoot -Recurse -File -Filter "*.rs" |
        ForEach-Object { $_.FullName }
)
$cp364TestFiles = @($cp364CalcFiles | Where-Object {
        $_ -match '(?:_tests\.rs$|[\\/]tests[\\/])'
    })
if ($cp364TestFiles.Count -eq 0) { throw "CP364 calc regression tests missing" }
$cp364ProductionFiles = @($cp364CalcFiles | Where-Object { $cp364TestFiles -notcontains $_ })
$cp364Limited = @($cp364CalcFiles) + @(
    $cp364BindingAdapter, $cp364InitWitness, $cp364Coupled, $cp364Pipeline,
    $cp364PipelineValidation, $cp364PipelineTests, $cp364Serialization,
    $cp364SnapshotSerialization, $cp364ArbitraryAssertions, $cp364Audit
)
foreach ($limited in $cp364Limited | Select-Object -Unique) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP364 bounded structure"
}
Assert-LineLimit -Path $cp364ArbitraryRoot -Limit 1200 -Description "arbitrary-run integration"
foreach ($production in @($cp364ProductionFiles) + @(
        $cp364BindingAdapter, $cp364InitWitness, $cp364Coupled, $cp364Pipeline,
        $cp364PipelineValidation, $cp364Serialization, $cp364SnapshotSerialization
    ) | Select-Object -Unique) {
    Assert-Cp364ProductionHasNoPanics -Path $production
}

$cp364CalcText = ($cp364CalcFiles | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
$cp364SemanticText = (@(
        $cp364Tests, $cp364PublicReleaseTests, $cp364BindingTests, $cp364CoupledTests,
        $cp364PipelineTests, $cp364SnapshotSerialization
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"

# Pinned line 2234 label, first excluded assignment, one site, and seven routes.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp364Source).Hash -cne $cp364SourceHash) {
    throw "CP364 pinned PurchasedAirManager.cc hash drifted"
}
$cp364SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp364Source
if ($cp364SourceLines[2233].Trim() -cne 'case HumControl::ConstantSupplyHumidityRatio: {' -or
    $cp364SourceLines[2234].Trim() -cne 'PurchAir.SupplyHumRat = PurchAir.MinCoolSuppAirHumRat;' -or
    $cp364SourceLines[2235].Trim() -cne '} break;' -or
    $cp364SourceLines[2236].Trim() -cne 'default: {' -or
    $cp364SourceLines[2244].Trim() -cne 'if (HeatOn) {') {
    throw "CP364 physical lines 2234 through 2237 or 2245 drifted"
}
Assert-Contains -Path $cp364Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2234' -Description "CP364 source line"
Assert-Contains -Path $cp364Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2235' -Description "CP364 first excluded source"
Assert-ExactStringArray -Path $cp364Module -Name "PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER" -Expected $cp364Sites -Description "CP364 sole case-entry site"
foreach ($route in @(
        "UnitOff", "NonCooling", "PositiveGuardFalseFallthrough",
        "DehumidificationControlNoneCaseCompletedSkip",
        "DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip",
        "DehumidificationControlHumidistatCaseCompletedSkip",
        "DehumidificationControlConstantSupplyHumidityRatioCaseEntered"
    )) {
    Assert-Cp364TextContains -Text $cp364CalcText -Pattern $route -Description "route '$route'"
}
foreach ($counter in @(
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_completed_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_entry_count",
        "source_site_execution_count"
    )) {
    Assert-Cp364TextContains -Text $cp364CalcText -Pattern ('pub ' + $counter + ':\s*usize') -Description "counter '$counter'"
}
Assert-Contains -Path $cp364Transition -Pattern '(?s)Route::DehumidificationControlConstantSupplyHumidityRatioCaseEntered\s*=>\s*\{.*?case_entry_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=.*?SOURCE_ORDER.*?\.len\(\);.*?witnessed_dehumidification_control_constant_supply_humidity_ratio_case_entry_count\s*\+=\s*1;' -Description "sole CSH case-entry increment"
Assert-Contains -Path $cp364Transition -Pattern '(?s)DehumidificationControlType::Humidistat.*?case_exited_via_break.*?Some\(Route::DehumidificationControlHumidistatCaseCompletedSkip\)' -Description "H completes before CP364 entry"
Assert-Contains -Path $cp364Transition -Pattern '(?s)DehumidificationControlType::ConstantSupplyHumidityRatio.*?case_selected_skip.*?Some\(Route::DehumidificationControlConstantSupplyHumidityRatioCaseEntered\)' -Description "CSH enters CP364 label"
Assert-Contains -Path $cp364Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "seven-route partition"
Assert-Contains -Path $cp364Runtime -Pattern 'selected\s*==\s*recursively_witnessed' -Description "selected recursive witness equality"
Assert-Contains -Path $cp364Runtime -Pattern 'selected\s*==\s*control_flow_partition' -Description "checked S algebra"
Assert-Contains -Path $cp364Runtime -Pattern 'state\.source_site_execution_count\s*==\s*entered_constant_supply' -Description "one-site CSH equality"
Assert-Contains -Path $cp364Runtime -Pattern '(?s)case_entry_count\s*==\s*prior.*?case_selected_skip_count' -Description "CSH inherits CP363"
Assert-Contains -Path $cp364Snapshot -Pattern '(?s)Some\(DehumidificationControlType::None\).*?!snapshot\.dehumidification_control_constant_supply_humidity_ratio_case_entered' -Description "direct C0 false entry"

# Exact CP363 predecessor, canonical private CSH bridge, named variants, and numerical firewall.
Assert-Contains -Path $cp364Prefix -Pattern 'PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot as Predecessor' -Description "exact CP363 predecessor"
foreach ($pattern in @(
        'completed_direct_cooling_humidistat_case_break_is_consistent',
        'cooling_humidistat_case_break_snapshot_is_exact_direct_release',
        'cooling_humidistat_case_break_snapshots_match_bit_exact',
        'cooling_humidistat_case_break_latest_witness'
    )) {
    Assert-Cp364TextContains -Text $cp364CalcText -Pattern $pattern -Description "recursive CP363 owner '$pattern'"
}
Assert-Contains -Path $cp364Coupled -Pattern 'PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary' -Description "CP363 predecessor lifecycle"
Assert-Contains -Path $cp364Private -Pattern 'private_constant_supply_humidity_ratio_counterfactual_from_direct_release as cp363_private_constant_supply_counterfactual_from_direct_release' -Description "canonical CP363 private CSH bridge"
Assert-Contains -Path $cp364Private -Pattern 'private_constant_supply_humidity_ratio_case_entry_counterfactual_from_direct_release' -Description "private CP364 CSH entry"
Assert-Contains -Path $cp364Private -Pattern 'private_constant_supply_humidity_ratio_case_entry_counterfactual_links_to_direct_release' -Description "private CP364 bridge proof"
Assert-Cp364TextContains -Text $cp364CalcText -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor_cp363:\s*Predecessor,\s*\)' -Description "public CP363 arguments"
foreach ($variant in @("None", "ConstantSensibleHeatRatio", "Humidistat", "ConstantSupplyHumidityRatio")) {
    Assert-Contains -Path $cp364Transition -Pattern ("DehumidificationControlType::" + $variant) -Description "named control variant '$variant'"
}
Assert-NotContains -Path $cp364Transition -Pattern 'std::mem::discriminant|discriminant\s*\(|\bas\s+(?:u|i)(?:8|16|32|64|128|size)\b|(?<!De)HumidificationControlType|(?<!de)humidification_control_type' -Description "enum ordinal/default/humidification promotion"
$cp364NumericalForbidden = 'f64|to_bits|from_bits|\.is_finite\(\)|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|MinCoolSuppAirHumRat|minimum_cooling_supply_air_humidity_ratio|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio|DirectZonePurchasedAirCouplingInput|PurchasedAirSizedLimits|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply'
foreach ($path in @(
        $cp364Module, $cp364State, $cp364Transition, $cp364Release, $cp364Prefix,
        $cp364Private, $cp364Runtime, $cp364Snapshot, $cp364BindingAdapter,
        $cp364InitWitness, $cp364Coupled, $cp364Pipeline, $cp364PipelineValidation,
        $cp364Serialization
    )) {
    Assert-NotContains -Path $path -Pattern $cp364NumericalForbidden -Description "CP364 numerical firewall"
}
$cp364SnapshotDto = Get-Cp364RustBraceBlock -Text (Read-RepoText -Path $cp364Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot\s*\{' -Description "CP364 snapshot DTO"
Assert-Cp364TextNotContains -Text $cp364SnapshotDto -Pattern 'f64|ieee|bits|Option\s*<\s*f64\s*>|[A-Za-z_]+_(?:c|w|kg_per_s|j_per_kg)\s*:' -Description "snapshot numeric payload"
$cp364SnapshotSerializationText = Read-RepoText -Path $cp364SnapshotSerialization
$cp364SnapshotTestBoundary = [regex]::Match($cp364SnapshotSerializationText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
$cp364SnapshotProduction = if ($cp364SnapshotTestBoundary.Success) {
    $cp364SnapshotSerializationText.Substring(0, $cp364SnapshotTestBoundary.Index)
} else {
    $cp364SnapshotSerializationText
}
Assert-Cp364TextNotContains -Text $cp364SnapshotProduction -Pattern '_ieee_bits|json_number|Value::Null|to_bits|f64|minimum_cooling_supply_air_humidity_ratio|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio' -Description "control-only JSON"

# Searchable semantic regressions and integration registration.
foreach ($test in @(
        "source_boundary_single_site_and_seven_route_algebra_are_exact",
        "all_counter_overflow_is_transactional",
        "public_direct_routes_skip_case_entry_and_retain_exact_lifecycle",
        "private_constant_supply_bridge_enters_only_cp364_case_label",
        "corruption_replay_and_runtime_forge_reject_without_mutation",
        "binding_orders_cp363_then_cp364_before_numerical_and_does_not_feed_result",
        "binding_cp364_is_complete_skip_for_direct_none",
        "binding_rejects_corrupt_cp363_without_mutation",
        "cp364_lifecycle_matches_outputs_and_cp363",
        "cp364_route_source_latest_and_predecessor_corruption_are_rejected",
        "cp364_expected_snapshot_maps_valid_q_h_and_csh_predecessor_routes",
        "direct_none_release_serializes_complete_entry_skip",
        "active_constant_supply_entry_has_no_numeric_payload"
    )) {
    Assert-Cp364TextContains -Text $cp364SemanticText -Pattern ('(?m)fn\s+' + $test + '\s*\(') -Description "semantic regression '$test'"
}
foreach ($test in @(
        "missing_direct_lifecycle_fails_closed",
        "checked_partitions_and_source_counts_fail_closed",
        "expected_snapshot_maps_only_constant_supply_to_entry",
        "direct_release_and_immediate_predecessor_are_strict"
    )) {
    Assert-Contains -Path $cp364PipelineTests -Pattern $test -Description "pipeline regression '$test'"
}
Assert-Contains -Path $cp364PipelineTests -Pattern '(?s)checked_partitions_and_source_counts_fail_closed.*?usize::MAX.*?validate_route_partition.*?case_entry_count\s*=\s*1.*?validate_source_counters' -Description "pipeline checked partition/source corruption"
Assert-Contains -Path $cp364PipelineTests -Pattern '(?s)expected_snapshot_maps_only_constant_supply_to_entry.*?Route::NoneCase.*?Route::ConstantShr.*?Route::Humidistat.*?Route::ConstantSupplyHumidityRatio' -Description "pipeline C0/Q/H/CSH routing"
Assert-Contains -Path $cp364PublicReleaseTests -Pattern '(?s)corruption_replay_and_runtime_forge_reject_without_mutation.*?forged_predecessor\.source_order\s*=\s*&\[\].*?calc_cooling_humidistat_case_break\.latest\s*=\s*Some\(forged\).*?set_cooling_humidistat_case_break_latest_witness.*?assert_rejected_unchanged' -Description "coordinated CP363 lineage forge"
Assert-Contains -Path $cp364BindingTests -Pattern '(?s)binding_rejects_corrupt_cp363_without_mutation.*?latest\.source_order\s*=\s*&\[\].*?let before\s*=\s*runtime\.clone\(\).*?advance_cooling_constant_supply_humidity_ratio_case_entry.*?\.is_err\(\).*?assert_eq!\(runtime,\s*before\)' -Description "transactional binding CP363 rejection"

$cp364BindingText = Read-RepoText -Path $cp364Binding
$cp363BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_${cp363StemForCp364} =")
$cp364BindingIndex = $cp364BindingText.IndexOf("let calculation_${cp364Stem} =")
$cp365BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_assignment =")
$cp366BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_break =")
$cp367BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =")
$cp368BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_case_break =")
$cp369BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp364 = $cp364BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp364NumericalIndex = $cp364BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp363BindingIndexForCp364 -lt 0 -or $cp364BindingIndex -le $cp363BindingIndexForCp364 -or $cp365BindingIndexForCp364 -le $cp364BindingIndex -or $cp366BindingIndexForCp364 -le $cp365BindingIndexForCp364 -or $cp367BindingIndexForCp364 -le $cp366BindingIndexForCp364 -or $cp368BindingIndexForCp364 -le $cp367BindingIndexForCp364 -or
    $cp364NumericalIndex -le $cp368BindingIndexForCp364 -or
    $cp369BindingIndexForCp364 -le $cp368BindingIndexForCp364 -or
    $cp370BindingIndexForCp364 -le $cp369BindingIndexForCp364 -or
    $cp371BindingIndexForCp364 -le $cp370BindingIndexForCp364 -or
    $cp372BindingIndexForCp364 -le $cp371BindingIndexForCp364 -or
    $cp373BindingIndexForCp364 -le $cp372BindingIndexForCp364 -or
    $cp374BindingIndexForCp364 -le $cp373BindingIndexForCp364 -or
    $cp375BindingIndexForCp364 -le $cp374BindingIndexForCp364 -or
    $cp376BindingIndexForCp364 -le $cp375BindingIndexForCp364 -or $cp377BindingIndexForCp364 -le $cp376BindingIndexForCp364 -or $cp378BindingIndexForCp364 -le $cp377BindingIndexForCp364 -or $cp379BindingIndexForCp364 -le $cp378BindingIndexForCp364 -or $cp380BindingIndexForCp364 -le $cp379BindingIndexForCp364 -or $cp381BindingIndexForCp364 -le $cp380BindingIndexForCp364 -or $cp382BindingIndexForCp364 -le $cp381BindingIndexForCp364 -or $cp383BindingIndexForCp364 -le $cp382BindingIndexForCp364 -or $cp384BindingIndexForCp364 -le $cp383BindingIndexForCp364 -or $cp385BindingIndexForCp364 -le $cp384BindingIndexForCp364 -or $cp364NumericalIndex -le $cp385BindingIndexForCp364) {
    throw "Binding must execute CP363 through CP370 before numerical coupling"
}
$cp364Dto = Get-Cp364RustBraceBlock -Text $cp364BindingText.Substring($cp364NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP364 numerical DTO"
if ($cp364Dto -match '(?i)cp364|constant_supply_humidity_ratio_case_entry') {
    throw "CP364 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp364CalcRoot -Pattern $cp364Stem -Description "calc registration"
Assert-Contains -Path $cp364BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + $cp364Stem) -Description "binding adapter"
Assert-Contains -Path $cp364ScheduledOutput -Pattern ('pub calculation_' + $cp364Stem + ':') -Description "scheduled output"
Assert-Contains -Path $cp364BindingTestsRoot -Pattern $cp364Stem -Description "binding-test registration"
Assert-Contains -Path $cp364InitState -Pattern $cp364Stem -Description "runtime state"
Assert-Contains -Path $cp364InitUnit -Pattern $cp364Stem -Description "unit state"
Assert-Contains -Path $cp364InitWitnessRoot -Pattern $cp364Stem -Description "witness registration"
Assert-Contains -Path $cp364CoupledRoot -Pattern ('mod ' + $cp364Stem + '_validation;') -Description "coupled validator"
Assert-Contains -Path $cp364Coupled -Pattern ('calculation_' + $cp363StemForCp364) -Description "coupled CP363 predecessor"
Assert-Contains -Path $cp364FixtureRoot -Pattern $cp364Stem -Description "fixture registration"
Assert-Contains -Path $cp364Fixture -Pattern ('calculation_' + $cp364Stem + '_snapshot') -Description "output fixture"
Assert-Contains -Path $cp364CoupledTestsRoot -Pattern 'coupled_runtime_tests_cp364' -Description "coupled-test registration"
Assert-Contains -Path $cp364PipelineRoot -Pattern ('mod ' + $cp364PipelineStem + ';') -Description "pipeline module"
Assert-Contains -Path $cp364PipelineRoot -Pattern ('"' + $cp364Lifecycle + '":\s*result\s*\.' + $cp364Lifecycle) -Description "lifecycle JSON"
Assert-Contains -Path $cp364PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp364ParentAssertions -Pattern 'mod cp364_assertions;' -Description "arbitrary delegation module"
Assert-Contains -Path $cp364ParentAssertions -Pattern 'cp364_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp364ParentAssertions -Pattern 'cp364_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp364ParentAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP363 relinquishes terminal nonfeed"
Assert-Contains -Path $cp364ArbitraryAssertions -Pattern 'mod cp365_assertions;' -Description "arbitrary CP365 delegation module"
Assert-Contains -Path $cp364ArbitraryAssertions -Pattern 'cp365_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP365 direct delegation"
Assert-Contains -Path $cp364ArbitraryAssertions -Pattern 'cp365_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP365 non-direct delegation"
Assert-NotContains -Path $cp364ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP364 relinquishes terminal numerical nonfeed to CP365"
Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp365_assertions.rs" -Pattern 'mod cp366_assertions;' -Description "CP365 delegates terminal evidence to CP366"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp365_assertions.rs" -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP365 relinquishes terminal numerical nonfeed to CP366"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp366_assertions.rs" -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP366 relinquishes terminal numerical nonfeed to CP367"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp368_assertions.rs" -Pattern 'assert_numerical_nonfeed' -Description "CP368 relinquishes terminal numerical nonfeed to CP369"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp369_assertions.rs" -Pattern 'assert_numerical_nonfeed' -Description "CP369 relinquishes terminal numerical nonfeed to CP370"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp370_assertions.rs" -Pattern 'assert_numerical_nonfeed' -Description "CP370 relinquishes terminal numerical nonfeed to CP371"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp371_assertions.rs" -Pattern 'assert_numerical_nonfeed' -Description "CP371 relinquishes terminal numerical nonfeed to CP372"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp373_assertions.rs" -Pattern 'assert_numerical_nonfeed' -Description "CP373 relinquishes terminal numerical nonfeed to CP374"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp374_assertions.rs" -Pattern 'assert_numerical_nonfeed' -Description "CP374 relinquishes terminal numerical nonfeed to CP375"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp375_assertions.rs" -Pattern 'assert_numerical_nonfeed' -Description "CP375 relinquishes terminal numerical nonfeed to CP376"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp377_assertions.rs" -Pattern 'assert_numerical_nonfeed' -Description "CP377 relinquishes terminal numerical evidence to CP378"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp378_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_exact_reconciliation\(' -Description "CP378 terminal numerical reconciliation"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp379_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP379 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP380 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed firewall"
Assert-Contains -Path $cp364ArbitraryAssertions -Pattern 'runtime\.contains_key\(CP364_KEY\)' -Description "CP364 non-direct key"
Assert-Contains -Path $cp364ArbitraryAssertions -Pattern 'runtime\[CP364_KEY\]\.is_null\(\)' -Description "CP364 non-direct null"

# Two spec addenda, 2+2+1+1 targets, five hand sections, and generated docs.
$cp364AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp364CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp364AlgorithmAddenda = [regex]::Matches($cp364AlgorithmText, '(?m)^\s*"CP364 supersedes only CP363[^"\r\n]+",\s*$')
$cp364CapabilityAddenda = [regex]::Matches($cp364CapabilityText, '(?m)^\s*"CP364 additionally requires[^"\r\n]+",\s*$')
if ($cp364AlgorithmAddenda.Count -ne 2 -or $cp364CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP364 addenda"
}
foreach ($claim in @($cp364AlgorithmAddenda) + @($cp364CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp364SourceCommit, $cp364SourceHash, 'physical line 2234',
            'case HumControl::ConstantSupplyHumidityRatio', $cp364Sites[0],
            'line 2235', 'CP365', 'line 2236', 'line 2237', 'line 2245',
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=constant_supply_humidity_ratio_case_entry_count=CSH',
            'C0=S', 'Q=H=CSH=0', 'false.*?entry', 'Private.*?CSH.*?true entry',
            'C0/Q/H.*?completed skips', 'CP363', 'sole predecessor owner',
            'named `DehumidificationControlType` variants', 'discriminant', 'f64',
            'CP363-to-CP364-to-unchanged-numerical', $cp364Lifecycle,
            'first/last supply-humidity bits remain unchanged', 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '302 total', '240 public', '62 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP364 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp364Stem/release\.rs::advance_direct_no_oa_calc_$cp364Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp364Stem\.rs::purchased_air_calc_${cp364Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp364Stem\.rs::${cp364TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp364Stem\.rs::${cp364TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp364AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP364 target count failed for '$($target.Pattern)'"
    }
}
$cp364Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP364 Cooling Constant-Supply-Humidity-Ratio Case Entry\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP364 Source-Ordered Cooling Constant-Supply-Humidity-Ratio Case Entry\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP364 Constant-Supply-Humidity-Ratio Case Entry\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP364 Constant-Supply-Humidity-Ratio Case Entry in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP364 Constant-Supply-Humidity-Ratio Case-Entry Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp364Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP364 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp364SourceHash, '2234', 'ConstantSupplyHumidityRatio', $cp364Sites[0],
            '2235', 'CP365', '2236', '2237', '2245',
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L', 'A\s*=\s*F\+L',
            'source_site_execution_count', 'C0\s*=\s*S',
            'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'false', 'CP363',
            '(?s)(?:sole|solely).*?predecessor', '(?s)named.*?DehumidificationControlType',
            '(?s)no.{0,90}numeric', 'CP363-to-CP364-to-unchanged-numerical',
            $cp364Lifecycle, 'CP345', '32\s+algorithms', '293\s+routines',
            '302\s+total', '240\s+public', '62\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP364 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP364\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP364 supersedes only CP363' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP364 additionally requires' -Description "generated capability addendum"

# Historical propagation, master order, and generated script inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..363 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_constant_supply_humidity_ratio_case_entry' -Description "historical CP364 binding order"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_constant_supply_humidity_ratio_case_entry' -Description "historical CP364 helper whitelist"
}
foreach ($historical in 334..363) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description "historical CP364 firewall"
}
foreach ($historical in 335..363) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 373 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 133 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..363) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 373' -Description "historical inventory total"
}
$cp364MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp363AuditIndexForCp364 = $cp364MainAuditText.IndexOf("cp363-cooling-humidistat-case-break.ps1")
$cp364AuditIndex = $cp364MainAuditText.IndexOf("cp364-cooling-constant-supply-humidity-ratio-case-entry.ps1")
$cp364CompletionIndex = $cp364MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp363AuditIndexForCp364 -lt 0 -or $cp364AuditIndex -le $cp363AuditIndexForCp364 -or $cp364CompletionIndex -le $cp364AuditIndex) {
    throw "Master audit must dot-source CP364 after CP363 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 373' -Description "script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp364-' -Description "inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp364-cooling-constant-supply-humidity-ratio-case-entry\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 373 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 133 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed firewall"
Write-Host "CP364 constant-supply-humidity-ratio case-entry structure audit passed."
