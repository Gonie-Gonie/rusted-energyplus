# CP358 maps only PurchasedAirManager.cc line 2228 `Humidistat` case entry.
$cp358Stem = "cooling_humidistat_case_entry"
$cp357StemForCp358 = "cooling_constant_shr_case_break"
$cp358PipelineStem = "purchased_air_$cp358Stem"
$cp358TypeStem = "PurchasedAirCalcCoolingHumidistatCaseEntry"
$cp358Lifecycle = "purchased_air_calc_${cp358Stem}_lifecycle"
$cp358SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp358Sites = @("enter-purchased-air-dehumidification-control-humidistat-case")

$cp358Module = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem.rs"
$cp358State = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem\state.rs"
$cp358Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem\transition.rs"
$cp358Release = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem\release.rs"
$cp358Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem\release\prefix_validation.rs"
$cp358Private = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem\release\private_counterfactual.rs"
$cp358Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem\release\runtime_validation.rs"
$cp358Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem\release\snapshot_validation.rs"
$cp358Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp358Stem\tests\mod.rs"
$cp358CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp358Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp358Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp358BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp358Stem.rs"
$cp358BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp358Stem}_tests.rs"
$cp358BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp358ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp358InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp358InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp358InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp358InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp358Stem.rs"
$cp358CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp358Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp358Stem}_validation.rs"
$cp358CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp358.rs"
$cp358FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp358Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp358Stem}_fixture.rs"
$cp358PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp358Pipeline = "crates\ep_run\src\pipeline\$cp358PipelineStem.rs"
$cp358PipelineValidation = "crates\ep_run\src\pipeline\$cp358PipelineStem\validation.rs"
$cp358PipelineTests = "crates\ep_run\src\pipeline\$cp358PipelineStem\validation\tests.rs"
$cp358Serialization = "crates\ep_run\src\pipeline\$cp358PipelineStem\serialization.rs"
$cp358SnapshotSerialization = "crates\ep_run\src\pipeline\$cp358PipelineStem\serialization\snapshot.rs"
$cp358ArbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp358ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp357_assertions.rs"
$cp358ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp358_assertions.rs"
$cp358Audit = "scripts\quality\ideal-loads-structure-audit\cp358-cooling-humidistat-case-entry.ps1"

function Get-Cp358RustBraceBlock {
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

function Assert-Cp358BindingContract {
    param([string]$Text)
    $cp357 = $Text.IndexOf("let calculation_$cp357StemForCp358 =")
    $cp358 = $Text.IndexOf("let calculation_$cp358Stem =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp357 -lt 0 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP357 then CP358 then CP359 before numerical coupling"
    }
    if ([regex]::Matches(
            $Text,
            '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;'
        ).Count -ne 1) {
        throw "Binding must execute the exact CP358 release call once"
    }
    $dto = Get-Cp358RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP358 numerical DTO"
    if ($dto -match '(?i)cp358|humidistat_case_entry') {
        throw "CP358 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp358PipelineContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp358RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp358Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp358Lifecycle) + '\s*=\s*Some\s*\(').Count
    $shorthand = [regex]::Matches($execute, '(?m)^\s*' + [regex]::Escape($cp358Lifecycle) + '\s*,\s*$').Count
    if ($none -ne 3 -or $some -ne 1 -or $shorthand -ne 1) {
        throw "Pipeline must expose one direct CP358 Some/shorthand and three non-direct None constructors"
    }
    $firewall = Get-Cp358RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp358Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP358 evidence exactly once"
    }
    $cp357Validator = $production.IndexOf("purchased_air_cooling_constant_shr_case_break::validate_direct_lifecycle(")
    $cp358Validator = $production.IndexOf("purchased_air_cooling_humidistat_case_entry::validate_direct_lifecycle(")
    if ($cp357Validator -lt 0 -or $cp358Validator -le $cp357Validator) {
        throw "Pipeline must validate CP358 immediately after its CP357 predecessor"
    }
}

function Assert-Cp358ProductionHasNoPanics {
    param([string]$Path)
    $text = Read-RepoText -Path $Path
    $tests = [regex]::Match($text, '(?m)^\s*#\[cfg\(test\)\]\s*\r?\n\s*mod\s+tests\b')
    $production = if ($tests.Success) { $text.Substring(0, $tests.Index) } else { $text }
    if ($production -match '(?m)\b(?:expect|unwrap)\s*\(|panic!\s*\(') {
        throw "CP358 production path contains expect/unwrap/panic: $Path"
    }
}

foreach ($required in @(
        $cp358Module, $cp358State, $cp358Transition, $cp358Release, $cp358Prefix,
        $cp358Private, $cp358Runtime, $cp358Snapshot, $cp358Tests, $cp358BindingAdapter,
        $cp358BindingTests, $cp358InitWitness, $cp358Coupled, $cp358CoupledTests,
        $cp358Fixture, $cp358Pipeline, $cp358PipelineValidation, $cp358PipelineTests,
        $cp358Serialization, $cp358SnapshotSerialization, $cp358ParentAssertions,
        $cp358ArbitraryAssertions, $cp358Audit
    )) {
    Assert-FileExists -Path $required -Description "CP358 structure"
}
foreach ($limited in @(
        $cp358Transition, $cp358Release, $cp358Prefix, $cp358Private, $cp358Runtime,
        $cp358Snapshot, $cp358Coupled, $cp358PipelineValidation, $cp358PipelineTests,
        $cp358Serialization, $cp358SnapshotSerialization, $cp358ArbitraryAssertions, $cp358Audit
    )) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP358 bounded structure"
}
Assert-LineLimit -Path $cp358ArbitraryRoot -Limit 1200 -Description "arbitrary-run integration"
foreach ($production in @(
        $cp358State, $cp358Transition, $cp358Release, $cp358Prefix, $cp358Private,
        $cp358Runtime, $cp358Snapshot, $cp358BindingAdapter, $cp358Coupled,
        $cp358Pipeline, $cp358PipelineValidation, $cp358Serialization,
        $cp358SnapshotSerialization
    )) {
    Assert-Cp358ProductionHasNoPanics -Path $production
}

# Exact source boundary, seven routes, sole H site, and evidence-only behavior.
Assert-Contains -Path $cp358Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2228' -Description "CP358 source line"
Assert-Contains -Path $cp358Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2229' -Description "CP358 first excluded executable"
Assert-Contains -Path $cp358Module -Pattern '(?s)Line 2229.*first statement.*Humidistat.*case body' -Description "CP358 exact line-2229 boundary"
Assert-Contains -Path $cp358Module -Pattern '(?s)line 2245.*Neither continuation is represented by CP358' -Description "CP358 dynamic Q continuation exclusion"
Assert-ExactStringArray -Path $cp358Module -Name "PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER" -Expected $cp358Sites -Description "CP358 sole H entry site"
Assert-Contains -Path $cp358State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,\s*DehumidificationControlHumidistatCaseEntered,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP358 seven routes"
foreach ($counter in @(
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_case_entry_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
        "source_site_execution_count"
    )) {
    Assert-Contains -Path $cp358State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP358 counter '$counter'"
}
Assert-Contains -Path $cp358Transition -Pattern '(?s)Route::DehumidificationControlHumidistatCaseEntered\s*=>.*?humidistat_case_entry_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=' -Description "CP358 sole H-site increment"
Assert-Contains -Path $cp358Transition -Pattern '(?s)ConstantSensibleHeatRatio.*?case_exited_via_break.*?Some\(Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip\)' -Description "CP358 Q completed skip"
Assert-Contains -Path $cp358Transition -Pattern '(?s)Humidistat.*?humidistat_case_selected_skip.*?Some\(Route::DehumidificationControlHumidistatCaseEntered\)' -Description "CP358 H entry selection"
Assert-Contains -Path $cp358Transition -Pattern 'next_transition_fits' -Description "CP358 overflow proof"
Assert-NotContains -Path $cp358Transition -Pattern 'f64|to_bits|from_bits|Psy[A-Za-z_]*|DirectZonePurchasedAirCouplingInput|\.is_finite\(\)|\.clamp\(' -Description "CP358 transition numerical firewall"
Assert-Contains -Path $cp358Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "CP358 route partition"
Assert-Contains -Path $cp358Runtime -Pattern 'state\.source_site_execution_count\s*==\s*humidistat_entry' -Description "CP358 one-site H equality"
Assert-Contains -Path $cp358Runtime -Pattern '(?s)completed_before_entry.*?checked_add\(constant_shr_completed\).*?at_or_after_entry.*?checked_add\(later_case_skip\)' -Description "CP358 checked control-flow algebra"
Assert-Contains -Path $cp358Prefix -Pattern 'PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot as Predecessor' -Description "CP358 exact CP357 predecessor"
Assert-Contains -Path $cp358Prefix -Pattern 'cp357_private_humidistat_counterfactual_links_to_direct_release' -Description "CP358 recursive CP357 owner"
Assert-NotContains -Path $cp358Prefix -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|f64' -Description "CP358 prefix numerical firewall"

foreach ($test in @(
        [PSCustomObject]@{ Path = $cp358Tests; Pattern = 'source_boundary_single_site_and_seven_route_algebra_are_exact' },
        [PSCustomObject]@{ Path = $cp358Tests; Pattern = 'transition_is_evidence_only_and_overflow_is_transactional' },
        [PSCustomObject]@{ Path = $cp358Tests; Pattern = 'public_direct_routes_skip_entry_and_private_h_uses_only_cp357_bridge' },
        [PSCustomObject]@{ Path = $cp358Tests; Pattern = 'canonical_private_q_completes_before_humidistat_without_fallthrough' },
        [PSCustomObject]@{ Path = $cp358Tests; Pattern = 'corruption_identity_replay_and_runtime_forge_reject_without_mutation' },
        [PSCustomObject]@{ Path = $cp358BindingTests; Pattern = 'scheduled_binding_places_cp358_after_cp357_and_preserves_cp345_numerical_owner' },
        [PSCustomObject]@{ Path = $cp358BindingTests; Pattern = 'scheduled_binding_preserves_u_n_p_skips_and_rejects_private_case_routes' },
        [PSCustomObject]@{ Path = $cp358CoupledTests; Pattern = 'cp358_coupled_direct_none_is_exact_skip_and_cp345_remains_numerical_owner' },
        [PSCustomObject]@{ Path = $cp358Coupled; Pattern = 'direct_counts_validate_and_non_direct_counts_reject' },
        [PSCustomObject]@{ Path = $cp358Coupled; Pattern = 'partition_overflow_and_source_corruption_fail_closed' },
        [PSCustomObject]@{ Path = $cp358SnapshotSerialization; Pattern = 'direct_none_release_serializes_complete_false_entry_skip' },
        [PSCustomObject]@{ Path = $cp358SnapshotSerialization; Pattern = 'active_humidistat_entry_serializes_true_without_numeric_payload' },
        [PSCustomObject]@{ Path = $cp358PipelineTests; Pattern = 'route_partition_overflow_fails_closed' },
        [PSCustomObject]@{ Path = $cp358PipelineTests; Pattern = 'source_counter_mismatch_fails_closed' },
        [PSCustomObject]@{ Path = $cp358PipelineTests; Pattern = 'inherited_u_n_p_and_c0_routes_validate' },
        [PSCustomObject]@{ Path = $cp358PipelineTests; Pattern = 'self_consistent_q_h_and_csh_routes_are_rejected' },
        [PSCustomObject]@{ Path = $cp358PipelineTests; Pattern = 'snapshot_source_system_ordinal_zone_route_and_entry_corruptions_fail_closed' }
    )) {
    Assert-Contains -Path $test.Path -Pattern $test.Pattern -Description "CP358 regression '$($test.Pattern)'"
}

# Release, binding, coupled runtime, pipeline, JSON, and numerical nonfeed.
Assert-Contains -Path $cp358Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp357:\s*Predecessor,\s*\)' -Description "CP358 exact public arguments"
Assert-Contains -Path $cp358Release -Pattern 'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None' -Description "CP358 direct C0 subset"
Assert-Contains -Path $cp358Release -Pattern 'advance_cooling_humidistat_case_entry_state\(\s*&mut unit\.calc_cooling_humidistat_case_entry,\s*retained_predecessor' -Description "CP358 evidence-only release"
Assert-NotContains -Path $cp358Release -Pattern 'PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|f64' -Description "CP358 public numerical firewall"
$cp358BindingText = Read-RepoText -Path $cp358Binding
$cp358PipelineRootText = Read-RepoText -Path $cp358PipelineRoot
Assert-Cp358BindingContract -Text $cp358BindingText
Assert-Cp358PipelineContract -Text $cp358PipelineRootText
Assert-Contains -Path $cp358CalcRoot -Pattern ('mod ' + [regex]::Escape($cp358Stem) + ';') -Description "CP358 calc module"
Assert-Contains -Path $cp358BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp358Stem)) -Description "CP358 binding adapter"
Assert-Contains -Path $cp358ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp358Stem) + ':') -Description "CP358 scheduled output"
Assert-Contains -Path $cp358BindingTestsRoot -Pattern ([regex]::Escape("${cp358Stem}_tests.rs")) -Description "CP358 binding tests"
Assert-Contains -Path $cp358InitState -Pattern $cp358Stem -Description "CP358 init state"
Assert-Contains -Path $cp358InitUnit -Pattern $cp358Stem -Description "CP358 unit state"
Assert-Contains -Path $cp358InitWitnessRoot -Pattern $cp358Stem -Description "CP358 witness module"
Assert-Contains -Path $cp358CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp358Stem) + '_validation;') -Description "CP358 coupled validator"
Assert-Contains -Path $cp358Coupled -Pattern 'PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState' -Description "coupled CP357 predecessor"
Assert-Contains -Path $cp358Coupled -Pattern '(?s)entries\s*\.checked_mul\(PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER\.len\(\)' -Description "coupled checked H source count"
Assert-NotContains -Path $cp358Coupled -Pattern 'complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|f64' -Description "coupled CP358 numerical firewall"
Assert-Contains -Path $cp358FixtureRoot -Pattern $cp358Stem -Description "CP358 fixture registration"
Assert-Contains -Path $cp358Fixture -Pattern ('calculation_' + [regex]::Escape($cp358Stem) + '_snapshot') -Description "CP358 output fixture"
Assert-Contains -Path $cp358PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp358PipelineStem) + ';') -Description "CP358 pipeline module"
Assert-Contains -Path $cp358PipelineRoot -Pattern ('"' + $cp358Lifecycle + '":\s*result\s*\.' + $cp358Lifecycle) -Description "CP358 lifecycle JSON"
Assert-Contains -Path $cp358PipelineValidation -Pattern 'case_break_cp357' -Description "pipeline CP357 predecessor"
Assert-Contains -Path $cp358PipelineValidation -Pattern 'source_site_execution_count' -Description "pipeline one-site source validation"
Assert-Contains -Path $cp358PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp371_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp358ParentAssertions -Pattern 'mod cp358_assertions;' -Description "arbitrary CP358 module delegation"
Assert-Contains -Path $cp358ParentAssertions -Pattern 'cp358_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP358 direct delegation"
Assert-Contains -Path $cp358ParentAssertions -Pattern 'cp358_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP358 non-direct delegation"
Assert-Contains -Path $cp358ArbitraryAssertions -Pattern 'CP358_KEY' -Description "arbitrary CP358 lifecycle"
Assert-Contains -Path $cp358ArbitraryAssertions -Pattern 'cp359_assertions::assert_direct\(runtime, results\)' -Description "CP359 assertion delegation preserves cumulative numerical nonfeed"
Assert-NotContains -Path $cp358Serialization -Pattern '_ieee_bits|json_number|to_bits|f64' -Description "CP358 lifecycle JSON numerical firewall"
$cp358SnapshotSerializationText = Read-RepoText -Path $cp358SnapshotSerialization
$cp358SnapshotTestBoundary = [regex]::Match(
    $cp358SnapshotSerializationText,
    '(?m)^\s*#\[cfg\(test\)\]\s*\r?\n\s*mod\s+tests\b'
)
if (
    -not $cp358SnapshotTestBoundary.Success -or
    $cp358SnapshotSerializationText.Substring(0, $cp358SnapshotTestBoundary.Index) -match
        '_ieee_bits|json_number|to_bits|f64'
) {
    throw "CP358 production snapshot JSON must contain no numerical payload"
}
Assert-Contains -Path $cp358SnapshotSerialization -Pattern '(?s)assigned_supply_humidity_ratio_ieee_bits.*?is_none' -Description "CP358 JSON numerical-payload absence regression"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$cp358AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp358CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp358AlgorithmAddenda = [regex]::Matches($cp358AlgorithmText, '(?m)^\s*"CP358 supersedes only CP357[^"\r\n]+",\s*$')
$cp358CapabilityAddenda = [regex]::Matches($cp358CapabilityText, '(?m)^\s*"CP358 additionally requires[^"\r\n]+",\s*$')
if ($cp358AlgorithmAddenda.Count -ne 2 -or $cp358CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP358 addenda"
}
foreach ($claim in @($cp358AlgorithmAddenda) + @($cp358CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp358SourceHash, 'physical line 2228', 'Humidistat', 'physical executable line 2229',
            'line 2245', $cp358Sites[0], 'T=U\+N\+P\+C0\+Q\+H\+CSH',
            'S=C0\+Q\+H\+CSH=R=G\+F\+L', 'A=F\+L',
            'source_site_execution_count=humidistat_case_entry_count=H', 'C0=S',
            'Q=H=CSH=0', 'CP357', 'sole predecessor owner', 'no numeric operand',
            'CP357-to-CP358-to-unchanged-numerical', $cp358Lifecycle,
            'first/last supply-humidity bits remain unchanged', 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '296 total', '240 public', '56 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP358 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp358Stem/release\.rs::advance_direct_no_oa_calc_$cp358Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp358Stem\.rs::purchased_air_calc_${cp358Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp358Stem\.rs::${cp358TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp358Stem\.rs::${cp358TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp358AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP358 target count failed for '$($target.Pattern)'"
    }
}
$cp358Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP358 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP358 Source-Ordered Cooling Humidistat Case Entry\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP358 Humidistat Case Entry\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP358 Humidistat Case Entry in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP358 Humidistat Case-Entry Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp358Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP358 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp358SourceHash, '2228', 'Humidistat', '2229', 'first excluded', '2245',
            $cp358Sites[0], 'U/N/P/C0/Q/H/CSH', 'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L', 'A\s*=\s*F\+L',
            'source_site_execution_count\s*=\s*humidistat_case_entry_count\s*=\s*H',
            'C0\s*=\s*S', 'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP357',
            '(?s)(?:sole|solely).*?predecessor', '(?s)no.{0,80}numeric',
            'CP357-to-CP358-to-unchanged-numerical', $cp358Lifecycle,
            'first/last', 'CP345', '32\s+algorithms', '293\s+routines',
            '58\s+[^,\r\n]*state[_-]mapped', '235\s+[^,\r\n]*source[_-]mapped',
            '170\s+required', '296\s+total', '240\s+public', '56\s+internal',
            'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP358 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP358\b' -Description "CP358 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP358 supersedes only CP357' -Description "generated CP358 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP358 additionally requires' -Description "generated CP358 capability addendum"

# Exact historical binding/firewall/inventory ranges and master reachability.
$cp358BindingHistory = @(326) + @(329..357)
foreach ($number in $cp358BindingHistory) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'calculation_cooling_humidistat_case_entry' -Description "historical CP358 binding/whitelist"
}
foreach ($number in 334..357) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp371_lifecycle_evidence' -Description "historical CP363 firewall"
}
foreach ($number in 335..357) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 309 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 69 \|')) -Description "historical generated internal"
}
foreach ($number in 337..357) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 309' -Description "historical script inventory total"
}
$cp358MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp357AuditIndexForCp358 = $cp358MainAuditText.IndexOf("cp357-cooling-constant-shr-case-break.ps1")
$cp358AuditIndex = $cp358MainAuditText.IndexOf("cp358-cooling-humidistat-case-entry.ps1")
$cp358CompletionIndex = $cp358MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp357AuditIndexForCp358 -lt 0 -or $cp358AuditIndex -le $cp357AuditIndexForCp358 -or $cp358CompletionIndex -le $cp358AuditIndex) {
    throw "Master audit must dot-source CP358 after CP357 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 309' -Description "CP358 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP358 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp358-' -Description "CP358 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp358-cooling-humidistat-case-entry\.ps1::dot_sources' -Description "CP358 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 309 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 69 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP358 Humidistat case-entry structure audit passed."
