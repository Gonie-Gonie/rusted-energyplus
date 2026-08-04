# CP357 maps only PurchasedAirManager.cc line 2227 `break;`.
$cp357Stem = "cooling_constant_shr_case_break"
$cp356StemForCp357 = "cooling_constant_shr_supply_humidity_ratio_mixed_air_limit"
$cp357PipelineStem = "purchased_air_$cp357Stem"
$cp357TypeStem = "PurchasedAirCalcCoolingConstantShrCaseBreak"
$cp357Lifecycle = "purchased_air_calc_${cp357Stem}_lifecycle"
$cp357SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp357Sites = @(
    "exit-purchased-air-dehumidification-control-constant-sensible-heat-ratio-case-via-break"
)
$cp357Module = "crates\ep_runtime\src\ideal_loads\calc\$cp357Stem.rs"
$cp357State = "crates\ep_runtime\src\ideal_loads\calc\$cp357Stem\state.rs"
$cp357Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp357Stem\transition.rs"
$cp357Release = "crates\ep_runtime\src\ideal_loads\calc\$cp357Stem\release.rs"
$cp357Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp357Stem\release\prefix_validation.rs"
$cp357Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp357Stem\release\runtime_validation.rs"
$cp357Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp357Stem\release\snapshot_validation.rs"
$cp357Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp357Stem\tests\mod.rs"
$cp357CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp357Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp357Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp357BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp357Stem.rs"
$cp357BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp357Stem}_tests.rs"
$cp357BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp357ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp357InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp357InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp357InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp357InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp357Stem.rs"
$cp357CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp357Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp357Stem}_validation.rs"
$cp357CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp357.rs"
$cp357FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp357Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp357Stem}_fixture.rs"
$cp357PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp357Pipeline = "crates\ep_run\src\pipeline\$cp357PipelineStem.rs"
$cp357PipelineValidation = "crates\ep_run\src\pipeline\$cp357PipelineStem\validation.rs"
$cp357PipelineTests = "crates\ep_run\src\pipeline\$cp357PipelineStem\validation\tests.rs"
$cp357Serialization = "crates\ep_run\src\pipeline\$cp357PipelineStem\serialization.rs"
$cp357SnapshotSerialization = "crates\ep_run\src\pipeline\$cp357PipelineStem\serialization\snapshot.rs"
$cp357ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp357CumulativeAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp353_assertions.rs"
$cp357ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp357_assertions.rs"
$cp357Audit = "scripts\quality\ideal-loads-structure-audit\cp357-cooling-constant-shr-case-break.ps1"

function Get-Cp357RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) {
        throw "$Description expected one anchor, found $($anchors.Count)"
    }
    $opening = $Text.IndexOf("{", $anchors[0].Index)
    $depth = 0
    for ($index = $opening; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") {
            $depth += 1
        }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1)
            }
        }
    }
    throw "$Description has no complete brace block"
}

function Assert-Cp357BindingContract {
    param([string]$Text)
    $cp356 = $Text.IndexOf("let calculation_$cp356StemForCp357 =")
    $cp357 = $Text.IndexOf("let calculation_$cp357Stem =")
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp356 -lt 0 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp357RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP357 numerical DTO"
    if ($dto -match '(?i)cp35[78]|case_break|humidistat_case_entry') {
        throw "CP357/CP358 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp357PipelineRootContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp357RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp357Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp357Lifecycle) + '\s*=\s*Some\s*\(').Count
    $shorthand = [regex]::Matches($execute, '(?m)^\s*' + [regex]::Escape($cp357Lifecycle) + '\s*,\s*$').Count
    if ($none -ne 3 -or $some -ne 1 -or $shorthand -ne 1) {
        throw "Pipeline must expose one direct CP357 Some/shorthand and three non-direct None constructors"
    }
    $firewall = Get-Cp357RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp357Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP357 evidence exactly once"
    }
}

foreach ($required in @(
        $cp357Module, $cp357State, $cp357Transition, $cp357Release, $cp357Prefix,
        $cp357Runtime, $cp357Snapshot, $cp357Tests, $cp357BindingAdapter,
        $cp357BindingTests, $cp357InitWitness, $cp357Coupled, $cp357CoupledTests,
        $cp357Fixture, $cp357Pipeline, $cp357PipelineValidation,
        $cp357PipelineTests, $cp357Serialization, $cp357SnapshotSerialization,
        $cp357CumulativeAssertions, $cp357ArbitraryAssertions, $cp357Audit
    )) {
    Assert-FileExists -Path $required -Description "CP357 structure"
}
foreach ($limited in @(
        $cp357Transition, $cp357Release, $cp357Prefix, $cp357Runtime,
        $cp357Snapshot, $cp357Coupled, $cp357PipelineValidation,
        $cp357CumulativeAssertions, $cp357ArbitraryAssertions, $cp357Audit
    )) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP357 bounded structure"
}
Assert-LineLimit -Path $cp357ArbitraryTests -Limit 1200 -Description "arbitrary-run integration"

# Exact source boundary, seven routes, sole break site, and evidence-only transition.
Assert-Contains -Path $cp357Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2227' -Description "CP357 source line"
Assert-Contains -Path $cp357Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2229' -Description "CP357 first excluded executable"
Assert-Contains -Path $cp357Module -Pattern 'line 2228 `Humidistat` label is CP358' -Description "CP358 case-label candidate"
Assert-Contains -Path $cp357Module -Pattern '(?s)line 2245.*Neither.*represented by CP357' -Description "unimplemented dynamic continuation"
Assert-ExactStringArray -Path $cp357Module -Name "PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER" -Expected $cp357Sites -Description "CP357 sole break site"
Assert-Contains -Path $cp357State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseBreak,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP357 seven routes"
foreach ($counter in @(
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_break_count",
        "dehumidification_control_humidistat_case_selected_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
        "source_site_execution_count"
    )) {
    Assert-Contains -Path $cp357State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP357 counter '$counter'"
}
Assert-Contains -Path $cp357Transition -Pattern '(?s)Route::DehumidificationControlConstantSensibleHeatRatioCaseBreak\s*=>.*?case_break_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=\s*PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER\.len\(\)' -Description "CP357 sole break-site increment"
Assert-Contains -Path $cp357Transition -Pattern '(?s)ConstantSensibleHeatRatio.*?mixed_air_limit_executed.*?Some\(Route::DehumidificationControlConstantSensibleHeatRatioCaseBreak\)' -Description "CP356 Q selects CP357 break"
Assert-Contains -Path $cp357Transition -Pattern '(?s)case_exited_via_break:\s*route\s*==\s*Route::DehumidificationControlConstantSensibleHeatRatioCaseBreak' -Description "CP357 true active break flag"
Assert-NotContains -Path $cp357Transition -Pattern 'f64|to_bits|from_bits|Psy[A-Za-z_]*|DirectZonePurchasedAirCouplingInput|\.is_finite\(\)|\.clamp\(' -Description "CP357 transition has no numerical behavior"
Assert-Contains -Path $cp357Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "CP357 route partition"
Assert-Contains -Path $cp357Runtime -Pattern '(?s)case_break_count\s*==\s*prior\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count' -Description "CP357 Q inherits CP356"
Assert-Contains -Path $cp357Runtime -Pattern 'state\.source_site_execution_count\s*==\s*break_flow' -Description "CP357 one-site Q equality"
Assert-Contains -Path $cp357Runtime -Pattern '(?s)completed_skip\s*\.checked_add\(break_flow\).*?checked_add\(later_case_skip\)' -Description "CP357 checked S algebra"
Assert-Contains -Path $cp357Runtime -Pattern '(?s)witnessed_dehumidification_control_constant_sensible_heat_ratio_case_break_count\s*==\s*state\.dehumidification_control_constant_sensible_heat_ratio_case_break_count' -Description "CP357 witnessed Q equality"
Assert-Contains -Path $cp357Prefix -Pattern 'PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor' -Description "CP357 exact CP356 predecessor"
Assert-Contains -Path $cp357Prefix -Pattern 'cp356_private_active_counterfactual_links_to_direct_release' -Description "CP357 recursive CP356 owner"
Assert-NotContains -Path $cp357Prefix -Pattern 'CP355|CP329|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply' -Description "CP357 alternate-owner and numerical firewall"

foreach ($test in @(
        [PSCustomObject]@{ Path = $cp357Tests; Pattern = 'source_boundary_single_site_and_seven_route_algebra_are_exact' },
        [PSCustomObject]@{ Path = $cp357Tests; Pattern = 'transition_is_evidence_only_and_overflow_is_transactional' },
        [PSCustomObject]@{ Path = $cp357Tests; Pattern = 'public_direct_routes_skip_break_and_private_q_uses_only_cp356_bridge' },
        [PSCustomObject]@{ Path = $cp357Tests; Pattern = 'corruption_identity_replay_and_runtime_forge_reject_without_mutation' },
        [PSCustomObject]@{ Path = $cp357BindingTests; Pattern = 'scheduled_binding_places_cp357_after_cp356_as_complete_skip_without_feeding_numerical_output' },
        [PSCustomObject]@{ Path = $cp357BindingTests; Pattern = 'scheduled_binding_rejects_non_direct_case_break_routes_before_runtime_mutation' },
        [PSCustomObject]@{ Path = $cp357CoupledTests; Pattern = 'cp357_coupled_direct_none_route_is_complete_skip_and_numerical_humidity_remains_unfed' },
        [PSCustomObject]@{ Path = $cp357Coupled; Pattern = 'partition_overflow_and_source_counter_corruption_fail_closed' },
        [PSCustomObject]@{ Path = $cp357SnapshotSerialization; Pattern = 'direct_none_release_serializes_complete_false_break_skip' },
        [PSCustomObject]@{ Path = $cp357SnapshotSerialization; Pattern = 'active_case_break_serializes_true_without_humidistat_fallthrough' },
        [PSCustomObject]@{ Path = $cp357PipelineTests; Pattern = 'route_partition_overflow_fails_closed' },
        [PSCustomObject]@{ Path = $cp357PipelineTests; Pattern = 'source_counter_mismatch_fails_closed' },
        [PSCustomObject]@{ Path = $cp357PipelineTests; Pattern = 'inherited_u_n_p_and_c0_routes_validate' },
        [PSCustomObject]@{ Path = $cp357PipelineTests; Pattern = 'self_consistent_q_h_and_csh_routes_are_rejected' },
        [PSCustomObject]@{ Path = $cp357PipelineTests; Pattern = 'snapshot_source_identity_route_and_break_corruptions_fail_closed' }
    )) {
    Assert-Contains -Path $test.Path -Pattern $test.Pattern -Description "CP357 regression '$($test.Pattern)'"
}

# Release, binding, coupled runtime, pipeline, serialization, and numerical nonfeed.
Assert-Contains -Path $cp357Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp356:\s*Predecessor,\s*\)' -Description "CP357 exact public arguments"
Assert-Contains -Path $cp357Release -Pattern 'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None' -Description "direct C0 subset"
Assert-Contains -Path $cp357Release -Pattern 'advance_cooling_constant_shr_case_break_state\(\s*&mut unit\.calc_cooling_constant_shr_case_break,\s*retained_predecessor' -Description "CP357 evidence-only release"
Assert-NotContains -Path $cp357Release -Pattern 'PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|f64' -Description "CP357 public numerical firewall"
$cp357BindingText = Read-RepoText -Path $cp357Binding
$cp357PipelineRootText = Read-RepoText -Path $cp357PipelineRoot
Assert-Cp357BindingContract -Text $cp357BindingText
Assert-Cp357PipelineRootContract -Text $cp357PipelineRootText
Assert-Contains -Path $cp357CalcRoot -Pattern ('mod ' + [regex]::Escape($cp357Stem) + ';') -Description "CP357 calc module"
Assert-Contains -Path $cp357BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp357Stem)) -Description "CP357 binding adapter"
Assert-NotContains -Path $cp357BindingAdapter -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|f64' -Description "CP357 binding numerical firewall"
Assert-Contains -Path $cp357ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp357Stem) + ':') -Description "CP357 scheduled output"
Assert-Contains -Path $cp357BindingTestsRoot -Pattern ([regex]::Escape("${cp357Stem}_tests.rs")) -Description "CP357 binding tests"
Assert-Contains -Path $cp357InitState -Pattern $cp357Stem -Description "CP357 init state"
Assert-Contains -Path $cp357InitUnit -Pattern $cp357Stem -Description "CP357 unit state"
Assert-Contains -Path $cp357InitWitnessRoot -Pattern $cp357Stem -Description "CP357 witness module"
Assert-Contains -Path $cp357CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp357Stem) + '_validation;') -Description "CP357 coupled validator"
Assert-Contains -Path $cp357Coupled -Pattern ('calculation_' + [regex]::Escape($cp356StemForCp357)) -Description "coupled CP356 predecessor"
Assert-Contains -Path $cp357Coupled -Pattern '(?s)executed\s*\.checked_mul\(.*?CASE_BREAK_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked Q source count"
Assert-NotContains -Path $cp357Coupled -Pattern 'complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled numerical firewall"
Assert-Contains -Path $cp357FixtureRoot -Pattern $cp357Stem -Description "CP357 fixture registration"
Assert-Contains -Path $cp357Fixture -Pattern ('calculation_' + [regex]::Escape($cp357Stem) + '_snapshot') -Description "CP357 output fixture"
Assert-Contains -Path $cp357PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp357PipelineStem) + ';') -Description "CP357 pipeline module"
Assert-Contains -Path $cp357PipelineRoot -Pattern ('"' + $cp357Lifecycle + '":\s*result\s*\.' + $cp357Lifecycle) -Description "CP357 lifecycle JSON"
Assert-Contains -Path $cp357PipelineValidation -Pattern 'mixed_air_limit_cp356' -Description "pipeline CP356 predecessor"
Assert-Contains -Path $cp357PipelineValidation -Pattern 'source_site_execution_count' -Description "pipeline one-site source validation"
Assert-Contains -Path $cp357PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp414_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp357CumulativeAssertions -Pattern 'cp357_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP357 direct delegation"
Assert-Contains -Path $cp357CumulativeAssertions -Pattern 'cp357_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP357 non-direct delegation"
Assert-Contains -Path $cp357ArbitraryAssertions -Pattern 'CP357_KEY' -Description "arbitrary CP357 lifecycle"
Assert-Contains -Path $cp357ArbitraryAssertions -Pattern '(?s)for endpoint in \["first", "last"\].*?to_bits\(\).*?cp345_bits' -Description "actual result-store first/last bit-exact nonfeed"
Assert-NotContains -Path $cp357SnapshotSerialization -Pattern '_ieee_bits|json_number|to_bits|f64' -Description "CP357 JSON has no numerical payload"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$cp357AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp357CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp357AlgorithmAddenda = [regex]::Matches($cp357AlgorithmText, '(?m)^\s*"CP357 supersedes only CP356[^"\r\n]+",\s*$')
$cp357CapabilityAddenda = [regex]::Matches($cp357CapabilityText, '(?m)^\s*"CP357 additionally requires[^"\r\n]+",\s*$')
if ($cp357AlgorithmAddenda.Count -ne 2 -or $cp357CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP357 addenda"
}
foreach ($claim in @($cp357AlgorithmAddenda) + @($cp357CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp357SourceHash, 'physical executable line 2227', 'break;', 'line 2228', 'CP358',
            'physical executable line 2229', 'line 2245', $cp357Sites[0],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L', 'A=F\+L',
            'source_site_execution_count=constant_sensible_heat_ratio_case_break_count=Q',
            'C0=S', 'Q=H=CSH=0', 'false break', 'true break', 'Humidistat',
            'CP356', 'sole predecessor owner', 'no numeric operand', 'no.*?finite/range gate',
            'CP356-to-CP357-to-unchanged-numerical', $cp357Lifecycle,
            'first/last supply-humidity bits remain unchanged', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', '170 required', 'Roadmap',
            '296 total', '240 public', '56 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP357 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp357Stem/release\.rs::advance_direct_no_oa_calc_$cp357Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp357Stem\.rs::purchased_air_calc_${cp357Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp357Stem\.rs::${cp357TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp357Stem\.rs::${cp357TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp357AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP357 target count failed for '$($target.Pattern)'"
    }
}
$cp357Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP357 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP357 Source-Ordered Cooling Constant-SHR Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP357 Constant-SHR Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP357 Constant-SHR Case Break in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP357 Constant-SHR Case Break Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp357Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP357 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp357SourceHash, '2227', 'break', '2228', 'CP358', '2229', '2245',
            $cp357Sites[0], 'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L', 'A\s*=\s*F\+L',
            'source_site_execution_count', 'C0\s*=\s*S', 'Q\s*=\s*H\s*=\s*CSH\s*=\s*0',
            'false', 'true', 'Humidistat', 'CP356', '(?s)(?:sole|solely).*?predecessor',
            '(?s)no.{0,60}numeric', 'gate', 'CP356-to-CP357-to-unchanged-numerical',
            $cp357Lifecycle, 'first/last', '32\s+algorithms', '293\s+routines',
            '58\s+[^,\r\n]*state[_-]mapped', '235\s+[^,\r\n]*source[_-]mapped', '170\s+required',
            '296\s+total', '240\s+public', '56\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP357 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP357\b' -Description "CP357 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP357 supersedes only CP356' -Description "generated CP357 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP357 additionally requires' -Description "generated CP357 capability addendum"

# Historical order/firewall audits, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..356 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_constant_shr_case_break' -Description "historical CP357 binding order"
}
foreach ($historical in 334..356) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp414_lifecycle_evidence' -Description "historical CP363 firewall"
}
$cp357MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp356AuditIndexForCp357 = $cp357MainAuditText.IndexOf("cp356-cooling-constant-shr-supply-humidity-ratio-mixed-air-limit.ps1")
$cp357AuditIndex = $cp357MainAuditText.IndexOf("cp357-cooling-constant-shr-case-break.ps1")
$cp357CompletionIndex = $cp357MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp356AuditIndexForCp357 -lt 0 -or $cp357AuditIndex -le $cp356AuditIndexForCp357 -or $cp357CompletionIndex -le $cp357AuditIndex) {
    throw "Master audit must dot-source CP357 after CP356 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 352' -Description "CP357 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP357 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp357-' -Description "CP357 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp357-cooling-constant-shr-case-break\.ps1::dot_sources' -Description "CP357 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 352 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 112 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP357 constant-SHR case-break structure audit passed."
