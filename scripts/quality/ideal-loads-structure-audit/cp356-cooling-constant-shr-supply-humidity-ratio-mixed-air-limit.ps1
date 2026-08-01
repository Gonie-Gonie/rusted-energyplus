# CP356 maps only PurchasedAirManager.cc line 2226; line 2227 break is excluded.
$cp356Stem = "cooling_constant_shr_supply_humidity_ratio_mixed_air_limit"
$cp355StemForCp356 = "cooling_constant_shr_supply_humidity_ratio_minimum_limit"
$cp356PipelineStem = "purchased_air_$cp356Stem"
$cp356TypeStem = "PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimit"
$cp356Lifecycle = "purchased_air_calc_${cp356Stem}_lifecycle"
$cp356SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp356Sites = @(
    "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit-minimum",
    "read-purchased-air-mixed-air-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-constant-sensible-heat-ratio-mixed-air-limit",
    "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-mixed-air-limit"
)
$cp356Module = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem.rs"
$cp356State = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\state.rs"
$cp356Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\transition.rs"
$cp356Release = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\release.rs"
$cp356Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\release\prefix_validation.rs"
$cp356Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\release\runtime_validation.rs"
$cp356Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\release\snapshot_validation.rs"
$cp356Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\tests\mod.rs"
$cp356ReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\tests\public_release.rs"
$cp356CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp356Stem\tests\release_corruption.rs"
$cp356MinimumHelper = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\transition.rs"
$cp356CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp356Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp356Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp356BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp356Stem.rs"
$cp356BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp356Stem}_tests.rs"
$cp356BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp356ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp356InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp356InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp356InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp356InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp356Stem.rs"
$cp356CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp356Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp356Stem}_validation.rs"
$cp356CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp356.rs"
$cp356FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp356Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp356Stem}_fixture.rs"
$cp356PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp356Pipeline = "crates\ep_run\src\pipeline\$cp356PipelineStem.rs"
$cp356PipelineValidation = "crates\ep_run\src\pipeline\$cp356PipelineStem\validation.rs"
$cp356PipelineTests = "crates\ep_run\src\pipeline\$cp356PipelineStem\validation\tests.rs"
$cp356Serialization = "crates\ep_run\src\pipeline\$cp356PipelineStem\serialization.rs"
$cp356SnapshotSerialization = "crates\ep_run\src\pipeline\$cp356PipelineStem\serialization\snapshot.rs"
$cp356ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp356ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp353_assertions.rs"
$cp356Audit = "scripts\quality\ideal-loads-structure-audit\cp356-cooling-constant-shr-supply-humidity-ratio-mixed-air-limit.ps1"

function Get-Cp356RustBraceBlock {
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

function Assert-Cp356BindingContract {
    param([string]$Text)
    $cp355 = $Text.IndexOf("let calculation_$cp355StemForCp356 =")
    $cp356 = $Text.IndexOf("let calculation_$cp356Stem =")
    $cp357 = $Text.IndexOf("let calculation_cooling_constant_shr_case_break =")
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp355 -lt 0 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp356RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP356 numerical DTO"
    if ($dto -match '(?i)cp35[5-8]|minimum_limit|mixed_air_limit|case_break|humidistat_case_entry') {
        throw "CP355/CP356/CP357/CP358 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp356PipelineRootContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp356RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp356Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp356Lifecycle) + '\s*=\s*Some\s*\(').Count
    if ($none -ne 3 -or $some -ne 1) {
        throw "Pipeline must expose one direct CP356 Some and three non-direct None constructors"
    }
    $firewall = Get-Cp356RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp356Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP356 evidence exactly once"
    }
}

foreach ($required in @(
        $cp356Module, $cp356State, $cp356Transition, $cp356Release, $cp356Prefix,
        $cp356Runtime, $cp356Snapshot, $cp356Tests, $cp356ReleaseTests,
        $cp356CorruptionTests, $cp356MinimumHelper, $cp356BindingAdapter,
        $cp356BindingTests, $cp356InitWitness, $cp356Coupled, $cp356CoupledTests,
        $cp356Fixture, $cp356Pipeline, $cp356PipelineValidation,
        $cp356PipelineTests, $cp356Serialization, $cp356SnapshotSerialization,
        $cp356ArbitraryAssertions, $cp356Audit
    )) {
    Assert-FileExists -Path $required -Description "CP356 structure"
}
foreach ($limited in @(
        $cp356Transition, $cp356Release, $cp356Prefix, $cp356Runtime,
        $cp356Snapshot, $cp356Coupled, $cp356PipelineValidation, $cp356Audit
    )) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP356 bounded structure"
}

# Exact line-2226 boundary, routes, four-site algebra, strict minimum, and null skip.
Assert-Contains -Path $cp356Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2226' -Description "CP356 source line"
Assert-Contains -Path $cp356Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2227' -Description "CP356 first excluded break"
Assert-ExactStringArray -Path $cp356Module -Name "PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER" -Expected $cp356Sites -Description "CP356 four sites"
Assert-Contains -Path $cp356State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMixedAirLimitExecuted,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP356 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count",
        "source_site_execution_count",
        "supply_humidity_ratio_for_mixed_air_limit_minimum_read_count",
        "mixed_air_humidity_ratio_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_assignment_write_count"
    )) {
    Assert-Contains -Path $cp356State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP356 counter '$counter'"
}
Assert-Contains -Path $cp356Transition -Pattern '(?s)let left = predecessor\.resulting_supply_humidity_ratio\?;\s*let right = active_operands\?\.mixed_air_humidity_ratio;\s*let minimum = source_shaped_two_argument_minimum\(left, right\);' -Description "CP356 sole ordered operands"
Assert-Contains -Path $cp356Transition -Pattern '(?s)assigned_supply_humidity_ratio:\s*prepared\.minimum_supply_humidity_ratio,.*?resulting_supply_humidity_ratio:\s*prepared\.minimum_supply_humidity_ratio' -Description "CP356 assignment bits"
foreach ($counter in @(
        "supply_humidity_ratio_for_mixed_air_limit_minimum_read_count",
        "mixed_air_humidity_ratio_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_assignment_write_count"
    )) {
    Assert-Contains -Path $cp356Transition -Pattern ($counter + '\s*\+=\s*1') -Description "CP356 site increment '$counter'"
}
Assert-NotContains -Path $cp356Transition -Pattern 'f64::min|\.min\s*\(|total_cmp|partial_cmp|\.is_(?:finite|nan)\(\)|\.clamp\(' -Description "CP356 pure transition has no substitute minimum or gate"
Assert-Contains -Path $cp356MinimumHelper -Pattern '(?s)fn source_shaped_two_argument_minimum\(.*?left:\s*f64,.*?right:\s*f64,.*?\)\s*->\s*f64\s*\{\s*if left < right \{ left \} else \{ right \}\s*\}' -Description "CP334 strict right-biased minimum"
Assert-Contains -Path $cp356Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "CP356 route partition"
Assert-Contains -Path $cp356Runtime -Pattern '(?s)mixed_air_limit_count\s*==\s*prior\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_count' -Description "CP356 Q inherits CP355"
Assert-Contains -Path $cp356Runtime -Pattern '(?s)assignments\.checked_mul\(\s*PURCHASED_AIR_CALC_.*?_MIXED_AIR_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "CP356 checked 4Q"
Assert-Contains -Path $cp356Snapshot -Pattern '(?s)source_shaped_two_argument_minimum\(left,\s*right\).*?minimum\.to_bits\(\)\s*==\s*expected\.to_bits\(\).*?assigned\.to_bits\(\)\s*==\s*minimum\.to_bits\(\).*?resulting\.to_bits\(\)\s*==\s*assigned\.to_bits\(\)' -Description "CP356 exact minimum and assignment bits"
Assert-Contains -Path $cp356Snapshot -Pattern '(?s)!snapshot\.supply_humidity_ratio_for_mixed_air_limit_minimum_read.*?supply_humidity_ratio_before_mixed_air_limit\s*\.is_none\(\).*?!snapshot\.mixed_air_humidity_ratio_for_minimum_read.*?mixed_air_humidity_ratio\.is_none\(\).*?!snapshot\.source_shaped_two_argument_minimum_evaluated.*?minimum_supply_humidity_ratio\.is_none\(\).*?!snapshot\.supply_humidity_ratio_assignment_performed.*?assigned_supply_humidity_ratio\.is_none\(\).*?resulting_supply_humidity_ratio\.is_none\(\)' -Description "CP356 five-field complete-null skip"
foreach ($test in @(
        [PSCustomObject]@{ Path = $cp356Tests; Pattern = 'source_boundary_four_sites_and_seven_route_algebra_are_exact' },
        [PSCustomObject]@{ Path = $cp356Tests; Pattern = 'active_transition_uses_only_cp355_left_and_cp329_right' },
        [PSCustomObject]@{ Path = $cp356Tests; Pattern = 'source_shaped_minimum_preserves_right_bias_and_ieee_bits' },
        [PSCustomObject]@{ Path = $cp356Tests; Pattern = 'pure_active_transition_has_no_numeric_gate' },
        [PSCustomObject]@{ Path = $cp356Tests; Pattern = 'inactive_routes_are_complete_null_and_direct_none_is_exact' },
        [PSCustomObject]@{ Path = $cp356Tests; Pattern = 'bit_exact_snapshot_matching_and_active_overflow_are_transactional' },
        [PSCustomObject]@{ Path = $cp356CorruptionTests; Pattern = 'private_active_operand_uses_only_same_call_retained_witnessed_cp329_owner' },
        [PSCustomObject]@{ Path = $cp356CorruptionTests; Pattern = 'coordinated_cp355_direct_and_witness_forge_breaks_recursive_bridge' },
        [PSCustomObject]@{ Path = $cp356ReleaseTests; Pattern = 'public_inherited_routes_are_exact_complete_null_skips' }
    )) {
    Assert-Contains -Path $test.Path -Pattern $test.Pattern -Description "CP356 regression '$($test.Pattern)'"
}

# CP355 sole left owner, recursively proven same-call CP329 RHS, and no CP356 gate.
Assert-Contains -Path $cp356Release -Pattern 'PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot as Predecessor' -Description "CP356 exact CP355 predecessor"
Assert-Contains -Path $cp356Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp355:\s*Predecessor,\s*\)' -Description "CP356 exact public arguments"
Assert-Contains -Path $cp356Release -Pattern '(?s)advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_state\(\s*&mut .*?,\s*retained_predecessor,\s*None,\s*\)' -Description "direct C0 supplies no RHS"
foreach ($pattern in @(
        'calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit\s*\.latest\?',
        'cooling_constant_shr_supply_humidity_ratio_minimum_limit_latest_witness',
        'cooling_constant_shr_supply_humidity_ratio_minimum_limit_snapshot_is_exact_direct_release',
        'completed_direct_cooling_constant_shr_supply_humidity_ratio_minimum_limit_is_consistent',
        'private_active_counterfactual_links_to_direct_release',
        'predecessor\.resulting_supply_humidity_ratio',
        'calc_cooling_mixed_air_call\.latest\?',
        'cooling_mixed_air_call_latest_witness',
        'cooling_mixed_air_call_snapshots_match_bit_exact',
        'cooling_mixed_air_call_snapshot_is_exact_direct_release',
        'completed_direct_cooling_mixed_air_call_is_consistent',
        'mixed_air\.mixed_air_humidity_ratio\?'
    )) {
    Assert-Contains -Path $cp356Prefix -Pattern $pattern -Description "CP356 recursive owner '$pattern'"
}
Assert-NotContains -Path $cp356Prefix -Pattern '\.is_finite\(\)|\.clamp\(|PurchasedAirSizedLimits|CP345|CP319|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|system\.mixed_air_humidity_ratio|zone.*mixed_air_humidity_ratio' -Description "CP356 alternate owner/gate substitution"
Assert-NotContains -Path $cp356Release -Pattern 'PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|mixed_air_humidity_ratio\s*:' -Description "CP356 public scalar/numerical substitution"

# Binding, coupled runtime, pipeline, serialization, and strict numerical nonfeed.
$cp356BindingText = Read-RepoText -Path $cp356Binding
$cp356PipelineRootText = Read-RepoText -Path $cp356PipelineRoot
Assert-Cp356BindingContract -Text $cp356BindingText
Assert-Cp356PipelineRootContract -Text $cp356PipelineRootText
Assert-Contains -Path $cp356CalcRoot -Pattern ('mod ' + [regex]::Escape($cp356Stem) + ';') -Description "CP356 calc module"
Assert-Contains -Path $cp356BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp356Stem)) -Description "CP356 binding adapter"
Assert-NotContains -Path $cp356BindingAdapter -Pattern 'mixed_air_humidity_ratio\s*:|DirectZonePurchasedAirCouplingInput|latest_numerical' -Description "CP356 binding scalar/DTO firewall"
Assert-Contains -Path $cp356ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp356Stem) + ':') -Description "CP356 scheduled output"
Assert-Contains -Path $cp356BindingTestsRoot -Pattern ([regex]::Escape("${cp356Stem}_tests.rs")) -Description "CP356 binding tests"
Assert-Contains -Path $cp356BindingTests -Pattern 'scheduled_binding_places_cp356_after_cp355_as_complete_null_none_skip_without_feeding_numerical_output' -Description "CP356 binding order regression"
Assert-Contains -Path $cp356InitState -Pattern $cp356Stem -Description "CP356 init state"
Assert-Contains -Path $cp356InitUnit -Pattern $cp356Stem -Description "CP356 unit state"
Assert-Contains -Path $cp356InitWitnessRoot -Pattern $cp356Stem -Description "CP356 witness module"
Assert-Contains -Path $cp356CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp356Stem) + '_validation;') -Description "CP356 coupled validator"
Assert-Contains -Path $cp356Coupled -Pattern ('calculation_' + [regex]::Escape($cp355StemForCp356)) -Description "coupled CP355 predecessor"
Assert-Contains -Path $cp356Coupled -Pattern '(?s)executed\s*\.checked_mul\(.*?MIXED_AIR_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked 4Q"
Assert-NotContains -Path $cp356Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled DTO firewall"
Assert-Contains -Path $cp356CoupledTests -Pattern 'cp356_coupled_direct_none_route_is_complete_skip_and_numerical_humidity_remains_unfed' -Description "CP356 numerical nonfeed test"
Assert-Contains -Path $cp356Coupled -Pattern 'partition_overflow_and_source_counter_corruption_fail_closed' -Description "CP356 coupled corruption regression"
Assert-Contains -Path $cp356FixtureRoot -Pattern $cp356Stem -Description "CP356 fixture registration"
Assert-Contains -Path $cp356Fixture -Pattern ('calculation_' + [regex]::Escape($cp356Stem) + '_snapshot') -Description "CP356 output fixture"
Assert-Contains -Path $cp356PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp356PipelineStem) + ';') -Description "CP356 pipeline module"
Assert-Contains -Path $cp356PipelineRoot -Pattern ('"' + $cp356Lifecycle + '":\s*result\s*\.' + $cp356Lifecycle) -Description "CP356 lifecycle JSON"
Assert-Contains -Path $cp356PipelineValidation -Pattern 'minimum_limit_cp355' -Description "pipeline CP355 predecessor"
Assert-Contains -Path $cp356PipelineValidation -Pattern '(?s)executed\s*\.checked_mul\(.*?MIXED_AIR_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 4Q"
Assert-Contains -Path $cp356PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp376_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp356ArbitraryTests -Pattern 'cp353_assertions' -Description "arbitrary cumulative module"
Assert-Contains -Path $cp356ArbitraryAssertions -Pattern 'CP356_KEY' -Description "arbitrary CP356 lifecycle"
Assert-Contains -Path $cp356ArbitraryAssertions -Pattern 'assert_cp356\(runtime, cp355\)' -Description "arbitrary CP355-to-CP356 lineage"
foreach ($field in @(
        "supply_humidity_ratio_before_mixed_air_limit", "mixed_air_humidity_ratio",
        "minimum_supply_humidity_ratio", "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio"
    )) {
    Assert-Contains -Path $cp356SnapshotSerialization -Pattern ('"' + $field + '"') -Description "CP356 JSON '$field'"
    Assert-Contains -Path $cp356SnapshotSerialization -Pattern ('"' + $field + '_ieee_bits"') -Description "CP356 JSON bits '$field'"
}
Assert-Contains -Path $cp356SnapshotSerialization -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "CP356 nonfinite numeric null"
Assert-Contains -Path $cp356SnapshotSerialization -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "CP356 authoritative bits"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$cp356AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp356CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp356AlgorithmAddenda = [regex]::Matches($cp356AlgorithmText, '(?m)^\s*"CP356 supersedes only CP355[^"\r\n]+",\s*$')
$cp356CapabilityAddenda = [regex]::Matches($cp356CapabilityText, '(?m)^\s*"CP356 additionally requires[^"\r\n]+",\s*$')
if ($cp356AlgorithmAddenda.Count -ne 2 -or $cp356CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP356 addenda"
}
foreach ($claim in @($cp356AlgorithmAddenda) + @($cp356CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp356SourceHash, 'physical executable line 2226', 'line 2227', 'break', 'line-2228', 'line-2229',
            $cp356Sites[0], $cp356Sites[1], $cp356Sites[2], $cp356Sites[3],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L', 'A=F\+L',
            'source_site_execution_count=4\*Q', 'C0=S', 'Q=H=CSH=0',
            'CP355.*?resulting_supply_humidity_ratio', 'CP329.*?mixed_air_humidity_ratio',
            'completed recursive CP329 proof', 'no finite', 'CP345', 'CP319', 'numerical DTO',
            'if left < right \{ left \} else \{ right \}', 'f64::min',
            'CP355-to-CP356-to-unchanged-numerical', $cp356Lifecycle,
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP356 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp356Stem/release\.rs::advance_direct_no_oa_calc_$cp356Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp356Stem\.rs::purchased_air_calc_${cp356Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp356Stem\.rs::${cp356TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp356Stem\.rs::${cp356TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp356AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP356 target count failed for '$($target.Pattern)'"
    }
}
$cp356Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP356 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP356 Source-Ordered Cooling Constant-SHR Supply-Humidity-Ratio Mixed-Air Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP356 Constant-SHR Supply-Humidity-Ratio Mixed-Air Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP356 Constant-SHR Supply-Humidity-Ratio Mixed-Air Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP356 Constant-SHR Supply-Humidity-Ratio Mixed-Air Limit Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp356Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP356 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp356SourceHash, '2226', '2227', 'break', '2228', '2229',
            $cp356Sites[0], $cp356Sites[1], $cp356Sites[2], $cp356Sites[3],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH', '4\*Q', 'C0\s*=\s*S',
            'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP355', 'resulting_supply_humidity_ratio',
            'CP329', 'mixed_air_humidity_ratio', '(?:completed recursive|recursively complete)',
            'adds?\s+no(?:\s+line-local)?\s+finite',
            'CP345', 'CP319', 'numerical DTO', 'if left < right \{ left \} else \{ right \}',
            'f64::min', 'CP355-to-CP356-to-unchanged-numerical', $cp356Lifecycle,
            '32\s+algorithms', '293\s+routines', '296\s+total', '240\s+public',
            '56\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP356 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP356 supersedes only CP355' -Description "generated CP356 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP356 additionally requires' -Description "generated CP356 capability addendum"

# Historical order/firewall audits, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..356 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit' -Description "historical CP356 binding order"
}
foreach ($historical in 334..356) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp376_lifecycle_evidence' -Description "historical CP363 firewall"
}
$cp356MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp355AuditIndexForCp356 = $cp356MainAuditText.IndexOf("cp355-cooling-constant-shr-supply-humidity-ratio-minimum-limit.ps1")
$cp356AuditIndex = $cp356MainAuditText.IndexOf("cp356-cooling-constant-shr-supply-humidity-ratio-mixed-air-limit.ps1")
$cp356CompletionIndex = $cp356MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp355AuditIndexForCp356 -lt 0 -or $cp356AuditIndex -le $cp355AuditIndexForCp356 -or $cp356CompletionIndex -le $cp356AuditIndex) {
    throw "Master audit must dot-source CP356 after CP355 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 314' -Description "CP356 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP356 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp356-' -Description "CP356 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp356-cooling-constant-shr-supply-humidity-ratio-mixed-air-limit\.ps1::dot_sources' -Description "CP356 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 314 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 74 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP356 supply-humidity-ratio mixed-air-limit structure audit passed."
