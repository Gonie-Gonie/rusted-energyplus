# CP355 maps only PurchasedAirManager.cc line 2224; line 2226 is excluded.
$cp355Stem = "cooling_constant_shr_supply_humidity_ratio_minimum_limit"
$cp354Stem = "cooling_constant_shr_supply_humidity_ratio_overdrying_limit"
$cp355PipelineStem = "purchased_air_$cp355Stem"
$cp355TypeStem = "PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimit"
$cp355Lifecycle = "purchased_air_calc_${cp355Stem}_lifecycle"
$cp355SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp355Sites = @(
    "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit-maximum",
    "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit-maximum",
    "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-minimum-limit",
    "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-minimum-limit"
)
$cp355Module = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem.rs"
$cp355State = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\state.rs"
$cp355Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\transition.rs"
$cp355Release = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\release.rs"
$cp355Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\release\prefix_validation.rs"
$cp355Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\release\runtime_validation.rs"
$cp355Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\release\snapshot_validation.rs"
$cp355Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\tests\mod.rs"
$cp355ReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\tests\public_release.rs"
$cp355CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp355Stem\tests\release_corruption.rs"
$cp355MaximumHelper = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\transition.rs"
$cp355CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp355Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp355Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362 binding order"
$cp355BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp355Stem.rs"
$cp355BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp355Stem}_tests.rs"
$cp355BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp355ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp355InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp355InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp355InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp355InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp355Stem.rs"
$cp355CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp355Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp355Stem}_validation.rs"
$cp355CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp355.rs"
$cp355FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp355Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp355Stem}_fixture.rs"
$cp355PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp355Pipeline = "crates\ep_run\src\pipeline\$cp355PipelineStem.rs"
$cp355PipelineValidation = "crates\ep_run\src\pipeline\$cp355PipelineStem\validation.rs"
$cp355PipelineTests = "crates\ep_run\src\pipeline\$cp355PipelineStem\validation\tests.rs"
$cp355Serialization = "crates\ep_run\src\pipeline\$cp355PipelineStem\serialization.rs"
$cp355SnapshotSerialization = "crates\ep_run\src\pipeline\$cp355PipelineStem\serialization\snapshot.rs"
$cp355ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp355ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp353_assertions.rs"
$cp355Audit = "scripts\quality\ideal-loads-structure-audit\cp355-cooling-constant-shr-supply-humidity-ratio-minimum-limit.ps1"

function Get-Cp355RustBraceBlock {
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

function Assert-Cp355BindingContract {
    param([string]$Text)
    $cp354 = $Text.IndexOf("let calculation_$cp354Stem =")
    $cp355 = $Text.IndexOf("let calculation_$cp355Stem =")
    $cp356 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =")
    $cp357 = $Text.IndexOf("let calculation_cooling_constant_shr_case_break =")
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp354 -lt 0 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp355RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP355 numerical DTO"
    if ($dto -match '(?i)cp35[45]|overdrying_limit|minimum_limit|minimum_cooling_supply_air_humidity_ratio') {
        throw "CP354/CP355 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp355PipelineRootContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp355RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp355Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp355Lifecycle) + '\s*=\s*Some\s*\(').Count
    if ($none -ne 3 -or $some -ne 1) {
        throw "Pipeline must expose one direct CP355 Some and three non-direct None constructors"
    }
    $firewall = Get-Cp355RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp355Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP355 evidence exactly once"
    }
}

foreach ($required in @(
        $cp355Module, $cp355State, $cp355Transition, $cp355Release, $cp355Prefix,
        $cp355Runtime, $cp355Snapshot, $cp355Tests, $cp355ReleaseTests,
        $cp355CorruptionTests, $cp355MaximumHelper, $cp355BindingAdapter,
        $cp355BindingTests, $cp355InitWitness, $cp355Coupled, $cp355CoupledTests,
        $cp355Fixture, $cp355Pipeline, $cp355PipelineValidation,
        $cp355PipelineTests, $cp355Serialization, $cp355SnapshotSerialization,
        $cp355ArbitraryAssertions, $cp355Audit
    )) {
    Assert-FileExists -Path $required -Description "CP355 structure"
}
foreach ($limited in @(
        $cp355Transition, $cp355Release, $cp355Prefix, $cp355Runtime,
        $cp355Snapshot, $cp355Coupled, $cp355PipelineValidation, $cp355Audit
    )) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP355 bounded structure"
}

# Exact source boundary, routes, four-site algebra, strict maximum, and null skip.
Assert-Contains -Path $cp355Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2224' -Description "CP355 source line"
Assert-Contains -Path $cp355Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2226' -Description "CP355 first excluded line"
Assert-ExactStringArray -Path $cp355Module -Name "PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MINIMUM_LIMIT_SOURCE_ORDER" -Expected $cp355Sites -Description "CP355 four sites"
Assert-Contains -Path $cp355State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMinimumLimitExecuted,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP355 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_count",
        "source_site_execution_count",
        "supply_humidity_ratio_for_minimum_limit_maximum_read_count",
        "minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "supply_humidity_ratio_assignment_write_count"
    )) {
    Assert-Contains -Path $cp355State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP355 counter '$counter'"
}
Assert-Contains -Path $cp355Transition -Pattern '(?s)let left = predecessor\.resulting_supply_humidity_ratio\?;\s*let right = active_operands\?\.minimum_cooling_supply_air_humidity_ratio;\s*let maximum = source_shaped_two_argument_maximum\(left, right\);' -Description "CP355 sole ordered operands and maximum"
Assert-Contains -Path $cp355Transition -Pattern '(?s)assigned_supply_humidity_ratio:\s*prepared\.maximum_supply_humidity_ratio,.*?resulting_supply_humidity_ratio:\s*prepared\.maximum_supply_humidity_ratio' -Description "CP355 assignment bits"
foreach ($counter in @(
        "supply_humidity_ratio_for_minimum_limit_maximum_read_count",
        "minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "supply_humidity_ratio_assignment_write_count"
    )) {
    Assert-Contains -Path $cp355Transition -Pattern ($counter + '\s*\+=\s*1') -Description "CP355 site increment '$counter'"
}
Assert-NotContains -Path $cp355Transition -Pattern 'f64::max|\.max\s*\(|total_cmp|partial_cmp|\.is_(?:finite|nan)\(\)|\.clamp\(' -Description "CP355 pure transition has no substitute maximum or gate"
Assert-Contains -Path $cp355MaximumHelper -Pattern '(?s)fn source_shaped_two_argument_maximum\(.*?left:\s*f64,.*?right:\s*f64,.*?\)\s*->\s*f64\s*\{\s*if left < right \{ right \} else \{ left \}\s*\}' -Description "CP333 strict-left-biased maximum"
Assert-NotContains -Path $cp355MaximumHelper -Pattern 'fn source_shaped_two_argument_maximum(?s:.*?)f64::max' -Description "maximum does not use f64::max"
Assert-Contains -Path $cp355Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "CP355 route partition"
Assert-Contains -Path $cp355Runtime -Pattern '(?s)minimum_limit_count\s*==\s*prior\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count' -Description "CP355 Q inherits CP354"
Assert-Contains -Path $cp355Runtime -Pattern '(?s)assignments\.checked_mul\(\s*PURCHASED_AIR_CALC_.*?_MINIMUM_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "CP355 checked 4Q"
Assert-Contains -Path $cp355Snapshot -Pattern '(?s)source_shaped_two_argument_maximum\(left,\s*right\).*?maximum\.to_bits\(\)\s*==\s*expected\.to_bits\(\).*?assigned\.to_bits\(\)\s*==\s*maximum\.to_bits\(\).*?resulting\.to_bits\(\)\s*==\s*assigned\.to_bits\(\)' -Description "CP355 exact maximum and assignment bits"
Assert-Contains -Path $cp355Snapshot -Pattern '(?s)!snapshot\.supply_humidity_ratio_for_minimum_limit_maximum_read.*?supply_humidity_ratio_before_minimum_limit\s*\.is_none\(\).*?!snapshot\.minimum_cooling_supply_air_humidity_ratio_for_maximum_read.*?minimum_cooling_supply_air_humidity_ratio\.is_none\(\).*?!snapshot\.source_shaped_two_argument_maximum_evaluated.*?maximum_supply_humidity_ratio\.is_none\(\).*?!snapshot\.supply_humidity_ratio_assignment_performed.*?assigned_supply_humidity_ratio\.is_none\(\).*?resulting_supply_humidity_ratio\.is_none\(\)' -Description "CP355 complete-null skip"
foreach ($test in @(
        [PSCustomObject]@{ Path = $cp355Tests; Pattern = 'source_boundary_four_sites_and_seven_route_algebra_are_exact' },
        [PSCustomObject]@{ Path = $cp355Tests; Pattern = 'active_transition_uses_cp354_left_and_typed_minimum_right' },
        [PSCustomObject]@{ Path = $cp355Tests; Pattern = 'source_shaped_maximum_preserves_left_bias_and_ieee_bits' },
        [PSCustomObject]@{ Path = $cp355Tests; Pattern = 'inactive_routes_are_complete_null_and_direct_none_is_exact' },
        [PSCustomObject]@{ Path = $cp355Tests; Pattern = 'bit_exact_snapshot_matching_and_active_overflow_are_transactional' },
        [PSCustomObject]@{ Path = $cp355CorruptionTests; Pattern = 'private_active_operand_uses_selected_typed_system_owner' },
        [PSCustomObject]@{ Path = $cp355CorruptionTests; Pattern = 'private_active_nonfinite_typed_owner_is_rejected_without_mutation' },
        [PSCustomObject]@{ Path = $cp355ReleaseTests; Pattern = 'public_inherited_routes_are_exact_complete_null_skips' }
    )) {
    Assert-Contains -Path $test.Path -Pattern $test.Pattern -Description "CP355 regression '$($test.Pattern)'"
}

# CP354 sole left owner, selected immutable typed RHS, finite-only active trust.
Assert-Contains -Path $cp355Release -Pattern 'PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot as Predecessor' -Description "CP355 exact CP354 predecessor"
Assert-Contains -Path $cp355Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp354:\s*Predecessor,\s*\)' -Description "CP355 exact public arguments"
Assert-Contains -Path $cp355Release -Pattern '(?s)advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit_state\(\s*&mut .*?,\s*retained_predecessor,\s*None,\s*\)' -Description "direct C0 supplies no RHS"
foreach ($pattern in @(
        'calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\s*\.latest\?',
        'cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witness',
        'cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release',
        'completed_direct_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_is_consistent',
        'private_active_counterfactual_links_to_direct_release',
        'predecessor\.resulting_supply_humidity_ratio',
        'system\.minimum_cooling_supply_air_humidity_ratio',
        'minimum\.is_finite\(\)\.then_some'
    )) {
    Assert-Contains -Path $cp355Prefix -Pattern $pattern -Description "CP355 recursive/typed owner '$pattern'"
}
Assert-NotContains -Path $cp355Prefix -Pattern 'PurchasedAirSizedLimits|calc_cooling_dehumidification_flow|minimum_cooling_supply_air_humidity_ratio_read|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|0\.0\s*\.\.=' -Description "CP355 alternate owner/range substitution"
Assert-NotContains -Path $cp355Release -Pattern 'PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|minimum_cooling_supply_air_humidity_ratio\s*:' -Description "CP355 public scalar/numerical substitution"

# Binding, coupled runtime, pipeline, serialization, and strict numerical nonfeed.
$cp355BindingText = Read-RepoText -Path $cp355Binding
$cp355PipelineRootText = Read-RepoText -Path $cp355PipelineRoot
Assert-Cp355BindingContract -Text $cp355BindingText
Assert-Cp355PipelineRootContract -Text $cp355PipelineRootText
Assert-Contains -Path $cp355CalcRoot -Pattern ('mod ' + [regex]::Escape($cp355Stem) + ';') -Description "CP355 calc module"
Assert-Contains -Path $cp355BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp355Stem)) -Description "CP355 binding adapter"
Assert-NotContains -Path $cp355BindingAdapter -Pattern 'minimum_cooling_supply_air_humidity_ratio\s*:|DirectZonePurchasedAirCouplingInput|latest_numerical' -Description "CP355 binding scalar/DTO firewall"
Assert-Contains -Path $cp355ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp355Stem) + ':') -Description "CP355 scheduled output"
Assert-Contains -Path $cp355BindingTestsRoot -Pattern ([regex]::Escape("${cp355Stem}_tests.rs")) -Description "CP355 binding tests"
Assert-Contains -Path $cp355BindingTests -Pattern 'scheduled_binding_places_cp355_after_cp354_as_a_complete_null_none_skip' -Description "CP355 binding order regression"
Assert-Contains -Path $cp355BindingTests -Pattern 'minimum_cooling_supply_air_humidity_ratio\s*=\s*f64::NAN' -Description "direct C0 does not validate RHS"
Assert-Contains -Path $cp355InitState -Pattern $cp355Stem -Description "CP355 init state"
Assert-Contains -Path $cp355InitUnit -Pattern $cp355Stem -Description "CP355 unit state"
Assert-Contains -Path $cp355InitWitnessRoot -Pattern $cp355Stem -Description "CP355 witness module"
Assert-Contains -Path $cp355CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp355Stem) + '_validation;') -Description "CP355 coupled validator"
Assert-Contains -Path $cp355Coupled -Pattern ('calculation_' + [regex]::Escape($cp354Stem)) -Description "coupled CP354 predecessor"
Assert-Contains -Path $cp355Coupled -Pattern '(?s)executed\s*\.checked_mul\(.*?MINIMUM_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked 4Q"
Assert-NotContains -Path $cp355Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled DTO firewall"
Assert-Contains -Path $cp355CoupledTests -Pattern 'cp355_coupled_direct_none_route_is_complete_skip_and_numerical_humidity_remains_unfed' -Description "CP355 numerical nonfeed test"
Assert-Contains -Path $cp355Coupled -Pattern 'partition_overflow_and_source_counter_corruption_fail_closed' -Description "CP355 coupled corruption regression"
Assert-Contains -Path $cp355FixtureRoot -Pattern $cp355Stem -Description "CP355 fixture registration"
Assert-Contains -Path $cp355Fixture -Pattern ('calculation_' + [regex]::Escape($cp355Stem) + '_snapshot') -Description "CP355 output fixture"
Assert-Contains -Path $cp355PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp355PipelineStem) + ';') -Description "CP355 pipeline module"
Assert-Contains -Path $cp355PipelineRoot -Pattern ('"' + $cp355Lifecycle + '":\s*result\s*\.' + $cp355Lifecycle) -Description "CP355 lifecycle JSON"
Assert-Contains -Path $cp355PipelineValidation -Pattern 'overdrying_limit_cp354' -Description "pipeline CP354 predecessor"
Assert-Contains -Path $cp355PipelineValidation -Pattern '(?s)executed\s*\.checked_mul\(.*?MINIMUM_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 4Q"
Assert-Contains -Path $cp355PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp362_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp355ArbitraryTests -Pattern 'cp353_assertions' -Description "arbitrary cumulative module"
Assert-Contains -Path $cp355ArbitraryAssertions -Pattern 'CP355_KEY' -Description "arbitrary CP355 lifecycle"
Assert-Contains -Path $cp355ArbitraryAssertions -Pattern 'assert_cp355\(runtime, cp354\)' -Description "arbitrary CP354-to-CP355 lineage"
foreach ($field in @(
        "supply_humidity_ratio_before_minimum_limit",
        "minimum_cooling_supply_air_humidity_ratio",
        "maximum_supply_humidity_ratio", "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio"
    )) {
    Assert-Contains -Path $cp355SnapshotSerialization -Pattern ('"' + $field + '"') -Description "CP355 JSON '$field'"
    Assert-Contains -Path $cp355SnapshotSerialization -Pattern ('"' + $field + '_ieee_bits"') -Description "CP355 JSON bits '$field'"
}
Assert-Contains -Path $cp355SnapshotSerialization -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "CP355 nonfinite numeric null"
Assert-Contains -Path $cp355SnapshotSerialization -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "CP355 authoritative bits"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches($algorithmText, '(?m)^\s*"CP355 supersedes only CP354[^"\r\n]+",\s*$')
$capabilityAddenda = [regex]::Matches($capabilityText, '(?m)^\s*"CP355 additionally requires[^"\r\n]+",\s*$')
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP355 addenda"
}
foreach ($claim in @($algorithmAddenda) + @($capabilityAddenda)) {
    foreach ($pattern in @(
            $cp355SourceHash, 'physical executable line 2224', 'line 2225', 'line 2226',
            $cp355Sites[0], $cp355Sites[1], $cp355Sites[2], $cp355Sites[3],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH',
            'source_site_execution_count=4\*Q', 'C0=S', 'Q=H=CSH=0', 'no RHS read',
            'CP354.*?resulting_supply_humidity_ratio', 'IdealLoadsAirSystem.*?minimum_cooling_supply_air_humidity_ratio',
            '0\.0077', '0\.0\.\.=1\.0', 'finite', 'CP319', 'numerical DTO',
            'if left < right \{ right \} else \{ left \}', 'f64::max',
            'CP354-to-CP355-to-unchanged-numerical', $cp355Lifecycle,
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP355 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp355Stem/release\.rs::advance_direct_no_oa_calc_$cp355Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp355Stem\.rs::purchased_air_calc_${cp355Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp355Stem\.rs::${cp355TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp355Stem\.rs::${cp355TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($algorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP355 target count failed for '$($target.Pattern)'"
    }
}
$documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP355 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP355 Source-Ordered Cooling Constant-SHR Supply-Humidity-Ratio Minimum Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP355 Constant-SHR Supply-Humidity-Ratio Minimum Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP355 Constant-SHR Supply-Humidity-Ratio Minimum Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP355 Constant-SHR Supply-Humidity-Ratio Minimum Limit Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP355 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp355SourceHash, '2224', '2225', '2226',
            $cp355Sites[0], $cp355Sites[1], $cp355Sites[2], $cp355Sites[3],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH', '4\*Q', 'C0\s*=\s*S',
            'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP354', 'resulting_supply_humidity_ratio',
            'IdealLoadsAirSystem\.minimum_cooling_supply_air_humidity_ratio',
            '0\.0077', '0\.0\.\.=1\.0', 'finite', 'CP319', 'numerical DTO',
            'if left < right \{ right \} else \{ left \}', 'f64::max',
            'CP354-to-CP355-to-unchanged-numerical', $cp355Lifecycle,
            '32\s+algorithms', '293\s+routines', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP355 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP355 supersedes only CP354' -Description "generated CP355 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP355 additionally requires' -Description "generated CP355 capability addendum"

# Historical order/firewall audits, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..354 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit' -Description "historical CP355 binding order"
}
foreach ($historical in 334..354) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp362_lifecycle_evidence' -Description "historical CP362 firewall"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp354AuditIndex = $mainAuditText.IndexOf("cp354-cooling-constant-shr-supply-humidity-ratio-overdrying-limit.ps1")
$cp355AuditIndex = $mainAuditText.IndexOf("cp355-cooling-constant-shr-supply-humidity-ratio-minimum-limit.ps1")
$completionIndex = $mainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp354AuditIndex -lt 0 -or $cp355AuditIndex -le $cp354AuditIndex -or $completionIndex -le $cp355AuditIndex) {
    throw "Master audit must dot-source CP355 after CP354 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 300' -Description "CP355 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP355 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp355-' -Description "CP355 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp355-cooling-constant-shr-supply-humidity-ratio-minimum-limit\.ps1::dot_sources' -Description "CP355 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 300 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 60 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP355 supply-humidity-ratio minimum-limit structure audit passed."
