# CP363 maps only PurchasedAirManager.cc line 2233 `} break;`.
$cp363Stem = "cooling_humidistat_case_break"
$cp362StemForCp363 = "cooling_humidistat_supply_humidity_ratio_mixed_air_limit"
$cp363PipelineStem = "purchased_air_$cp363Stem"
$cp363TypeStem = "PurchasedAirCalcCoolingHumidistatCaseBreak"
$cp363Lifecycle = "purchased_air_calc_${cp363Stem}_lifecycle"
$cp363SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp363SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp363Sites = @(
    "exit-purchased-air-dehumidification-control-humidistat-case-via-break"
)
$cp363Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp363Module = "crates\ep_runtime\src\ideal_loads\calc\$cp363Stem.rs"
$cp363ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp363Stem"
$cp363Release = "$cp363ModuleRoot\release.rs"
$cp363Prefix = "$cp363ModuleRoot\release\prefix_validation.rs"
$cp363Private = "$cp363ModuleRoot\release\private_counterfactual.rs"
$cp363Runtime = "$cp363ModuleRoot\release\runtime_validation.rs"
$cp363Snapshot = "$cp363ModuleRoot\release\snapshot_validation.rs"
$cp363Tests = "$cp363ModuleRoot\tests\mod.rs"
$cp363PublicReleaseTests = "$cp363ModuleRoot\tests\public_release.rs"
$cp363CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp363Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp363BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp363Stem.rs"
$cp363BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp363BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp363Stem}_tests.rs"
$cp363ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp363InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp363InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp363InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp363InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp363Stem.rs"
$cp363CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp363Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp363Stem}_validation.rs"
$cp363CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp363CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp363.rs"
$cp363FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp363Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp363Stem}_fixture.rs"
$cp363PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp363Pipeline = "crates\ep_run\src\pipeline\$cp363PipelineStem.rs"
$cp363PipelineValidation = "crates\ep_run\src\pipeline\$cp363PipelineStem\validation.rs"
$cp363PipelineTests = "crates\ep_run\src\pipeline\$cp363PipelineStem\validation\tests.rs"
$cp363Serialization = "crates\ep_run\src\pipeline\$cp363PipelineStem\serialization.rs"
$cp363SnapshotSerialization = "crates\ep_run\src\pipeline\$cp363PipelineStem\serialization\snapshot.rs"
$cp363ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp362_assertions.rs"
$cp363ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp363_assertions.rs"
$cp363ArbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp363Audit = "scripts\quality\ideal-loads-structure-audit\cp363-cooling-humidistat-case-break.ps1"

function Assert-Cp363TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP363 $Description missing" }
}

function Assert-Cp363TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP363 $Description unexpectedly present" }
}

function Get-Cp363RustBraceBlock {
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

function Assert-Cp363ProductionHasNoPanics {
    param([string]$Path)
    $text = Read-RepoText -Path $Path
    if ($text -match '(?m)\b(?:expect|unwrap)\s*\(|panic!\s*\(') {
        throw "CP363 production path contains expect/unwrap/panic: $Path"
    }
}

foreach ($required in @(
        $cp363Source, $cp363Module, $cp363Release, $cp363Prefix, $cp363Private,
        $cp363Runtime, $cp363Snapshot, $cp363Tests, $cp363PublicReleaseTests,
        $cp363BindingAdapter, $cp363BindingTestsRoot, $cp363BindingTests,
        $cp363InitWitness, $cp363Coupled, $cp363CoupledTestsRoot,
        $cp363CoupledTests, $cp363Fixture, $cp363Pipeline,
        $cp363PipelineValidation, $cp363PipelineTests, $cp363Serialization,
        $cp363SnapshotSerialization, $cp363ParentAssertions,
        $cp363ArbitraryAssertions, $cp363Audit
    )) {
    Assert-FileExists -Path $required -Description "CP363 structure"
}
$cp363CalcFiles = @($cp363Module) + @(
    Get-ChildItem -LiteralPath $cp363ModuleRoot -Recurse -File -Filter "*.rs" |
        ForEach-Object { $_.FullName }
)
$cp363TestFiles = @($cp363CalcFiles | Where-Object {
        $_ -match '(?:_tests\.rs$|[\\/]tests[\\/])'
    })
if ($cp363TestFiles.Count -eq 0) { throw "CP363 calc regression tests missing" }
$cp363ProductionFiles = @($cp363CalcFiles | Where-Object { $cp363TestFiles -notcontains $_ })
$cp363Limited = @($cp363CalcFiles) + @(
    $cp363BindingAdapter, $cp363InitWitness, $cp363Coupled, $cp363Pipeline,
    $cp363PipelineValidation, $cp363PipelineTests, $cp363Serialization,
    $cp363SnapshotSerialization, $cp363ArbitraryAssertions, $cp363Audit
)
foreach ($limited in $cp363Limited | Select-Object -Unique) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP363 bounded structure"
}
Assert-LineLimit -Path $cp363ArbitraryRoot -Limit 1200 -Description "arbitrary-run integration"
foreach ($production in @($cp363ProductionFiles) + @(
        $cp363BindingAdapter, $cp363InitWitness, $cp363Coupled, $cp363Pipeline,
        $cp363PipelineValidation, $cp363Serialization, $cp363SnapshotSerialization
    ) | Select-Object -Unique) {
    Assert-Cp363ProductionHasNoPanics -Path $production
}

$cp363CalcText = ($cp363CalcFiles | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
$cp363TestsText = ($cp363TestFiles | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
$cp363SemanticFiles = @(Get-ChildItem -LiteralPath "crates" -Recurse -File -Filter "*.rs" |
    Where-Object {
        $candidate = Read-RepoText -Path $_.FullName
        $candidate -match '#\[test\]' -and $candidate -match "(?:$cp363Stem|cp363)"
    })
$cp363SemanticText = ($cp363SemanticFiles | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join "`n"

# Pinned source, exact boundary, one site, seven routes, and H-only break.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp363Source).Hash -cne $cp363SourceHash) {
    throw "CP363 pinned PurchasedAirManager.cc hash drifted"
}
$cp363SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp363Source
if ($cp363SourceLines[2232].Trim() -cne '} break;' -or
    $cp363SourceLines[2233].Trim() -cne 'case HumControl::ConstantSupplyHumidityRatio: {' -or
    $cp363SourceLines[2234].Trim() -cne 'PurchAir.SupplyHumRat = PurchAir.MinCoolSuppAirHumRat;') {
    throw "CP363 physical lines 2233 through 2235 drifted"
}
Assert-Contains -Path $cp363Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2233' -Description "CP363 source line"
Assert-Contains -Path $cp363Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2235' -Description "CP363 first excluded executable"
Assert-Contains -Path $cp363Module -Pattern '(?s)line 2234.*?ConstantSupplyHumidityRatio.*?CP364' -Description "CP364 case-label candidate"
Assert-Contains -Path $cp363Module -Pattern '(?s)line 2245.*?(?:not|Neither).*?represented by CP363' -Description "excluded active-H continuation"
Assert-ExactStringArray -Path $cp363Module -Name "PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER" -Expected $cp363Sites -Description "CP363 sole break site"
foreach ($route in @(
        "UnitOff", "NonCooling", "PositiveGuardFalseFallthrough",
        "DehumidificationControlNoneCaseCompletedSkip",
        "DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip",
        "DehumidificationControlHumidistatCaseBreak",
        "DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip"
    )) {
    Assert-Cp363TextContains -Text $cp363CalcText -Pattern $route -Description "route '$route'"
}
foreach ($counter in @(
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_break_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
        "source_site_execution_count"
    )) {
    Assert-Cp363TextContains -Text $cp363CalcText -Pattern ('pub ' + $counter + ':\s*usize') -Description "counter '$counter'"
}
Assert-Contains -Path "$cp363ModuleRoot\transition.rs" -Pattern '(?s)Route::DehumidificationControlHumidistatCaseBreak\s*=>.*?humidistat_case_break_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=\s*PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_BREAK_SOURCE_ORDER\.len\(\)' -Description "sole H break-site increment"
Assert-Contains -Path "$cp363ModuleRoot\transition.rs" -Pattern '(?s)DehumidificationControlType::Humidistat.*?mixed_air_limit_executed.*?Some\(Route::DehumidificationControlHumidistatCaseBreak\)' -Description "CP362 H selects CP363 break"
Assert-Contains -Path "$cp363ModuleRoot\transition.rs" -Pattern '(?s)case_exited_via_break:\s*route\s*==\s*Route::DehumidificationControlHumidistatCaseBreak' -Description "true H break flag"
Assert-Contains -Path $cp363Runtime -Pattern 'state\.source_site_execution_count\s*==\s*break_flow' -Description "one-site H equality"
Assert-Contains -Path $cp363Runtime -Pattern '(?s)humidistat_case_break_count\s*==\s*prior.*?humidistat_supply_humidity_ratio_mixed_air_limit_count' -Description "H inherits CP362"
Assert-Contains -Path $cp363Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "seven-route partition"
Assert-Contains -Path $cp363Runtime -Pattern '(?s)let earlier_case_skip\s*=.*?constant_sensible_heat_ratio_case_completed_skip_count;.*?let break_flow\s*=.*?humidistat_case_break_count;.*?let later_case_skip\s*=.*?constant_supply_humidity_ratio_case_selected_skip_count;.*?checked_sum\(&\[earlier_case_skip,\s*break_flow,\s*later_case_skip\]\).*?completed_skip\.checked_add\(after_none_case\)' -Description "checked S algebra"
Assert-Contains -Path $cp363Runtime -Pattern '(?s)witnessed_dehumidification_control_humidistat_case_break_count\s*==\s*state\s*\.\s*dehumidification_control_humidistat_case_break_count' -Description "private witnessed H equality"

# Exact CP362 recursive owner, CP364 bridge, and complete numerical firewall.
Assert-Contains -Path $cp363Prefix -Pattern 'PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor' -Description "exact CP362 predecessor"
foreach ($pattern in @(
        'completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent',
        'cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release',
        'cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness',
        'private_humidistat_counterfactual_links_to_direct_release'
    )) {
    Assert-Cp363TextContains -Text $cp363CalcText -Pattern $pattern -Description "recursive CP362 owner '$pattern'"
}
Assert-Cp363TextContains -Text $cp363CalcText -Pattern 'private_constant_supply_humidity_ratio_counterfactual_from_direct_release' -Description "CP364 canonical bridge"
Assert-Cp363TextContains -Text $cp363CalcText -Pattern 'private_constant_supply_humidity_ratio_counterfactual_links_to_direct_release' -Description "CP364 bridge proof"
Assert-Cp363TextContains -Text $cp363CalcText -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor_cp362:\s*Predecessor,\s*\)' -Description "public CP362 arguments"
foreach ($path in @(
        "$cp363ModuleRoot\transition.rs", $cp363Release, $cp363Runtime,
        $cp363Snapshot, $cp363BindingAdapter,
        $cp363Coupled, $cp363PipelineValidation, $cp363SnapshotSerialization
    )) {
    Assert-NotContains -Path $path -Pattern 'f64|to_bits|from_bits|\.is_finite\(\)|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply' -Description "CP363 numerical firewall"
}
foreach ($path in @($cp363Prefix, $cp363Private)) {
    Assert-NotContains -Path $path -Pattern 'to_bits|from_bits|\.is_finite\(\)|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply|[+\-*/]\s*pre_sampled_|pre_sampled_.*?[+\-*/]' -Description "CP363 forwarded-scalar firewall"
    foreach ($scalar in @(
            "pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s",
            "pre_sampled_zone_node_humidity_ratio"
        )) {
        Assert-Contains -Path $path -Pattern ($scalar + ':\s*f64') -Description "CP362 forwarded scalar '$scalar'"
    }
}
if ([regex]::Matches((Read-RepoText -Path $cp363Prefix), ':\s*f64').Count -ne 2 -or
    [regex]::Matches((Read-RepoText -Path $cp363Private), ':\s*f64').Count -ne 4) {
    throw "CP363 may only forward CP362's two explicit scalars through its H bridge and proof"
}
$cp363SnapshotDto = Get-Cp363RustBraceBlock -Text (Read-RepoText -Path $cp363Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot\s*\{' -Description "CP363 snapshot DTO"
Assert-Cp363TextNotContains -Text $cp363SnapshotDto -Pattern 'f64|ieee|bits|Option\s*<\s*f64\s*>|[A-Za-z_]+_(?:c|w|kg_per_s|j_per_kg)\s*:' -Description "snapshot numeric payload"
$cp363SnapshotSerializationText = Read-RepoText -Path $cp363SnapshotSerialization
$cp363SnapshotTestBoundary = [regex]::Match($cp363SnapshotSerializationText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
$cp363SnapshotProduction = if ($cp363SnapshotTestBoundary.Success) {
    $cp363SnapshotSerializationText.Substring(0, $cp363SnapshotTestBoundary.Index)
} else {
    $cp363SnapshotSerializationText
}
Assert-Cp363TextNotContains -Text $cp363SnapshotProduction -Pattern '_ieee_bits|json_number|Value::Null|to_bits|f64' -Description "control-only JSON"

# Searchable semantic regressions, including registration evidence.
foreach ($test in @(
        "source_boundary_single_site_and_seven_route_algebra_are_exact",
        "transition_is_evidence_only_and_all_overflow_is_transactional",
        "public_direct_routes_skip_break_and_private_h_uses_only_cp362_bridge",
        "private_constant_supply_bridge_is_canonical_for_cp364",
        "corruption_identity_replay_and_runtime_forge_reject_without_mutation",
        "binding_orders_cp362_then_cp363_before_numerical_and_does_not_feed_result",
        "binding_cp363_is_complete_skip_for_direct_none",
        "binding_rejects_corrupt_cp362_without_mutation",
        "cp363_lifecycle_matches_outputs_and_cp362",
        "cp363_route_source_latest_and_predecessor_corruption_are_rejected",
        "cp363_expected_snapshot_maps_q_h_and_csh_predecessor_routes",
        "direct_none_release_serializes_complete_false_break_skip",
        "active_humidistat_break_has_no_numeric_payload"
    )) {
    Assert-Cp363TextContains -Text $cp363SemanticText -Pattern ('(?m)fn\s+' + $test + '\s*\(') -Description "semantic regression '$test'"
}
foreach ($test in @(
        "missing_direct_lifecycle_fails_closed",
        "checked_partitions_and_source_counts_fail_closed",
        "expected_snapshot_maps_only_humidistat_to_break",
        "direct_release_and_immediate_predecessor_are_strict"
    )) {
    Assert-Contains -Path $cp363PipelineTests -Pattern $test -Description "pipeline regression '$test'"
}
Assert-Contains -Path $cp363PipelineTests -Pattern '(?s)checked_partitions_and_source_counts_fail_closed.*?usize::MAX.*?validate_route_partition.*?humidistat_case_break_count\s*=\s*1.*?validate_source_counters' -Description "pipeline checked partition/source corruption"
Assert-Contains -Path $cp363PipelineTests -Pattern '(?s)expected_snapshot_maps_only_humidistat_to_break.*?Route::ConstantShr.*?Route::Humidistat.*?Route::ConstantSupplyHumidityRatio' -Description "pipeline Q/H/CSH routing"
Assert-Contains -Path $cp363PipelineTests -Pattern '(?s)direct_release_and_immediate_predecessor_are_strict.*?case_exited_via_break\s*=\s*true.*?mixed_air_humidity_ratio\s*=\s*Some\(0\.0\)' -Description "pipeline CP363 and CP362 corruption"
$cp363CorruptionTest = Get-Cp363RustBraceBlock -Text (Read-RepoText -Path $cp363PublicReleaseTests) -AnchorPattern 'fn\s+corruption_identity_replay_and_runtime_forge_reject_without_mutation\s*\(\)' -Description "CP363 coordinated CP362 forge regression"
Assert-Cp363TextContains -Text $cp363CorruptionTest -Pattern '(?s)let mut forged\s*=\s*predecessor;.*?forged\.mixed_air_humidity_ratio\s*=\s*Some\(0\.009\);.*?latest\s*=\s*Some\(forged\);.*?set_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness\(.*?forged,\s*\);.*?assert_rejected_unchanged\(&mut coordinated,\s*&system,\s*forged\)' -Description "coordinated supplied/retained/witness CP362 numeric-shape forge"
$cp363BindingCorruptionTest = Get-Cp363RustBraceBlock -Text (Read-RepoText -Path $cp363BindingTests) -AnchorPattern 'fn\s+binding_rejects_corrupt_cp362_without_mutation\s*\(\)' -Description "CP363 binding CP362 corruption regression"
Assert-Cp363TextContains -Text $cp363BindingCorruptionTest -Pattern '(?s)let mut canonical_pending\s*=\s*runtime\.clone\(\);.*?advance_cooling_humidistat_case_break\(.*?&mut canonical_pending,.*?\)\s*\.is_ok\(\).*?latest\.source_order\s*=\s*&\[\];.*?let before\s*=\s*runtime\.clone\(\);.*?advance_cooling_humidistat_case_break\(.*?&mut runtime,.*?predecessor,.*?\)\s*\.is_err\(\).*?assert_eq!\(runtime,\s*before\)' -Description "binding pending control and transactional CP362 corruption rejection"
Assert-Contains -Path $cp363CoupledTests -Pattern '(?s)cp363_expected_snapshot_maps_q_h_and_csh_predecessor_routes.*?\(false,\s*true,\s*false,\s*false\).*?\(false,\s*false,\s*true,\s*false\).*?\(false,\s*false,\s*false,\s*true\).*?expected_snapshot\(predecessor\).*?\(expected\.1,\s*expected\.2,\s*expected\.3\)' -Description "coupled Q/H/CSH predecessor route mapping"

# Binding, coupled runtime, pipeline, arbitrary chain, and numerical nonfeed.
$cp363BindingText = Read-RepoText -Path $cp363Binding
$cp362BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_${cp362StemForCp363} =")
$cp363BindingIndex = $cp363BindingText.IndexOf("let calculation_${cp363Stem} =")
$cp364BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_entry =")
$cp365BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_assignment =")
$cp366BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_break =")
$cp367BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =")
$cp368BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_case_break =")
$cp369BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp363 = $cp363BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp363NumericalIndex = $cp363BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp362BindingIndexForCp363 -lt 0 -or $cp363BindingIndex -le $cp362BindingIndexForCp363 -or $cp364BindingIndexForCp363 -le $cp363BindingIndex -or $cp365BindingIndexForCp363 -le $cp364BindingIndexForCp363 -or $cp366BindingIndexForCp363 -le $cp365BindingIndexForCp363 -or $cp367BindingIndexForCp363 -le $cp366BindingIndexForCp363 -or $cp368BindingIndexForCp363 -le $cp367BindingIndexForCp363 -or
    $cp363NumericalIndex -le $cp368BindingIndexForCp363 -or
    $cp369BindingIndexForCp363 -le $cp368BindingIndexForCp363 -or
    $cp370BindingIndexForCp363 -le $cp369BindingIndexForCp363 -or
    $cp371BindingIndexForCp363 -le $cp370BindingIndexForCp363 -or
    $cp372BindingIndexForCp363 -le $cp371BindingIndexForCp363 -or
    $cp373BindingIndexForCp363 -le $cp372BindingIndexForCp363 -or
    $cp374BindingIndexForCp363 -le $cp373BindingIndexForCp363 -or
    $cp375BindingIndexForCp363 -le $cp374BindingIndexForCp363 -or
    $cp376BindingIndexForCp363 -le $cp375BindingIndexForCp363 -or $cp377BindingIndexForCp363 -le $cp376BindingIndexForCp363 -or $cp378BindingIndexForCp363 -le $cp377BindingIndexForCp363 -or $cp379BindingIndexForCp363 -le $cp378BindingIndexForCp363 -or $cp380BindingIndexForCp363 -le $cp379BindingIndexForCp363 -or $cp381BindingIndexForCp363 -le $cp380BindingIndexForCp363 -or $cp382BindingIndexForCp363 -le $cp381BindingIndexForCp363 -or $cp383BindingIndexForCp363 -le $cp382BindingIndexForCp363 -or $cp384BindingIndexForCp363 -le $cp383BindingIndexForCp363 -or $cp385BindingIndexForCp363 -le $cp384BindingIndexForCp363 -or $cp363NumericalIndex -le $cp385BindingIndexForCp363) {
    throw "Binding must execute CP362 through CP370 before numerical coupling"
}
$cp363Dto = Get-Cp363RustBraceBlock -Text $cp363BindingText.Substring($cp363NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP363 numerical DTO"
if ($cp363Dto -match '(?i)cp363|humidistat_case_break') {
    throw "CP363 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp363CalcRoot -Pattern $cp363Stem -Description "calc registration"
Assert-Contains -Path $cp363BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + $cp363Stem) -Description "binding adapter"
Assert-Contains -Path $cp363ScheduledOutput -Pattern ('pub calculation_' + $cp363Stem + ':') -Description "scheduled output"
Assert-Contains -Path $cp363BindingTestsRoot -Pattern $cp363Stem -Description "binding-test registration"
Assert-Contains -Path $cp363InitState -Pattern $cp363Stem -Description "runtime state"
Assert-Contains -Path $cp363InitUnit -Pattern $cp363Stem -Description "unit state"
Assert-Contains -Path $cp363InitWitnessRoot -Pattern $cp363Stem -Description "witness registration"
Assert-Contains -Path $cp363CoupledRoot -Pattern ('mod ' + $cp363Stem + '_validation;') -Description "coupled validator"
Assert-Contains -Path $cp363Coupled -Pattern ('calculation_' + $cp362StemForCp363) -Description "coupled CP362 predecessor"
Assert-Contains -Path $cp363Coupled -Pattern '(?s)checked_mul\(.*?CASE_BREAK_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked H source count"
Assert-Contains -Path $cp363FixtureRoot -Pattern $cp363Stem -Description "fixture registration"
Assert-Contains -Path $cp363Fixture -Pattern ('calculation_' + $cp363Stem + '_snapshot') -Description "output fixture"
Assert-Contains -Path $cp363PipelineRoot -Pattern ('mod ' + $cp363PipelineStem + ';') -Description "pipeline module"
Assert-Contains -Path $cp363PipelineRoot -Pattern ('"' + $cp363Lifecycle + '":\s*result\s*\.' + $cp363Lifecycle) -Description "lifecycle JSON"
Assert-Contains -Path $cp363PipelineValidation -Pattern 'mixed_air_limit_cp362' -Description "pipeline CP362 predecessor"
Assert-Contains -Path $cp363PipelineValidation -Pattern 'source_site_execution_count' -Description "pipeline source validation"
Assert-Contains -Path $cp363PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp402_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp363ParentAssertions -Pattern 'mod cp363_assertions;' -Description "arbitrary delegation module"
Assert-Contains -Path $cp363ParentAssertions -Pattern 'cp363_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp363ParentAssertions -Pattern 'cp363_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp363ParentAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP362 relinquishes terminal nonfeed"
Assert-Contains -Path $cp363ArbitraryAssertions -Pattern 'mod cp364_assertions;' -Description "CP364 arbitrary delegation module"
Assert-Contains -Path $cp363ArbitraryAssertions -Pattern 'cp364_assertions::assert_direct\(runtime, results\)' -Description "CP364 arbitrary direct delegation"
Assert-Contains -Path $cp363ArbitraryAssertions -Pattern 'cp364_assertions::assert_non_direct\(runtime\)' -Description "CP364 arbitrary non-direct delegation"
Assert-NotContains -Path $cp363ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP363 relinquishes terminal nonfeed to CP364"

# Two spec addenda, 2+2+1+1 targets, five hand sections, and generated docs.
$cp363AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp363CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp363AlgorithmAddenda = [regex]::Matches($cp363AlgorithmText, '(?m)^\s*"CP363 supersedes only CP362[^"\r\n]+",\s*$')
$cp363CapabilityAddenda = [regex]::Matches($cp363CapabilityText, '(?m)^\s*"CP363 additionally requires[^"\r\n]+",\s*$')
if ($cp363AlgorithmAddenda.Count -ne 2 -or $cp363CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP363 addenda"
}
foreach ($claim in @($cp363AlgorithmAddenda) + @($cp363CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp363SourceCommit, $cp363SourceHash, 'physical executable line 2233',
            $cp363Sites[0], 'line 2234', 'ConstantSupplyHumidityRatio', 'CP364',
            'physical executable line 2235', 'line 2245',
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=humidistat_case_break_count=H',
            'C0=S', 'Q=H=CSH=0', 'false break', 'true break', 'CP362',
            'sole predecessor owner', 'no break-local numeric operand',
            'two explicit pre-sampled characterization scalars', 'no.*?finite/range gate',
            'CP362-to-CP363-to-unchanged-numerical', $cp363Lifecycle,
            'first/last supply-humidity bits remain unchanged', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', '170 required', 'Roadmap',
            '301 total', '240 public', '61 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP363 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp363Stem/release\.rs::advance_direct_no_oa_calc_$cp363Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp363Stem\.rs::purchased_air_calc_${cp363Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp363Stem\.rs::${cp363TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp363Stem\.rs::${cp363TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp363AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP363 target count failed for '$($target.Pattern)'"
    }
}
$cp363Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP363 Cooling Humidistat Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP363 Source-Ordered Cooling Humidistat Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP363 Humidistat Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP363 Humidistat Case Break in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP363 Humidistat Case-Break Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp363Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP363 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp363SourceHash, '2233', 'break', '2234', 'CP364', '2235', '2245',
            $cp363Sites[0], 'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L', 'A\s*=\s*F\+L',
            'source_site_execution_count', 'C0\s*=\s*S', 'Q\s*=\s*H\s*=\s*CSH\s*=\s*0',
            'false', 'true', 'ConstantSupplyHumidityRatio', 'CP362',
            '(?s)(?:sole|solely).*?predecessor', '(?s)no.{0,80}numeric', 'gate',
            'CP362-to-CP363-to-unchanged-numerical', $cp363Lifecycle, 'first/last',
            '32\s+algorithms', '293\s+routines', '301\s+total', '240\s+public',
            '61\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP363 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP363\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP363 supersedes only CP362' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP363 additionally requires' -Description "generated capability addendum"

# Historical propagation, master order, and generated script inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..362 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_humidistat_case_break' -Description "historical CP363 binding order"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_humidistat_case_break' -Description "historical CP363 helper whitelist"
}
foreach ($historical in 334..362) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp402_lifecycle_evidence' -Description "historical CP363 firewall"
}
foreach ($historical in 335..362) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 340 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 100 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..362) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 340' -Description "historical inventory total"
}
$cp363MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp362AuditIndexForCp363 = $cp363MainAuditText.IndexOf("cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1")
$cp363AuditIndex = $cp363MainAuditText.IndexOf("cp363-cooling-humidistat-case-break.ps1")
$cp363CompletionIndex = $cp363MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp362AuditIndexForCp363 -lt 0 -or $cp363AuditIndex -le $cp362AuditIndexForCp363 -or $cp363CompletionIndex -le $cp363AuditIndex) {
    throw "Master audit must dot-source CP363 after CP362 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 340' -Description "script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp363-' -Description "inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp363-cooling-humidistat-case-break\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 340 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 100 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP363 Humidistat case-break structure audit passed."
