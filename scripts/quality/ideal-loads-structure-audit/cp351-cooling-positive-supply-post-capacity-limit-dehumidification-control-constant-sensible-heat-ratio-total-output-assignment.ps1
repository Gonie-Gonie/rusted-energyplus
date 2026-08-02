# CP351 maps only PurchasedAirManager.cc line 2218; line 2219 is excluded.
$cp351Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment"
$cp351PipelineStem = "purchased_air_$cp351Stem"
$cp351TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignment"
$cp351Lifecycle = "purchased_air_calc_${cp351Stem}_lifecycle"
$cp351SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp351Sites = @(
    "read-retained-cooling-sensible-output-for-constant-sensible-heat-ratio-total-output-numerator",
    "read-purchased-air-cooling-sensible-heat-ratio-for-constant-sensible-heat-ratio-total-output-denominator",
    "calculate-cooling-sensible-output-divided-by-cooling-sensible-heat-ratio-for-constant-sensible-heat-ratio-total-output",
    "assign-local-cooling-total-output-for-constant-sensible-heat-ratio-case"
)
$cp351Module = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem.rs"
$cp351State = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\state.rs"
$cp351Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\transition.rs"
$cp351Release = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\release.rs"
$cp351Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\release\prefix_validation.rs"
$cp351Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\release\runtime_validation.rs"
$cp351Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\release\snapshot_validation.rs"
$cp351Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\tests\mod.rs"
$cp351IeeeTests = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\tests\ieee.rs"
$cp351PublicTests = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\tests\public_release.rs"
$cp351CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp351Stem\tests\release_corruption.rs"
$cp351CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp351Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp351Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp351BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp351Stem.rs"
$cp351BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp351Stem}_tests.rs"
$cp351BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp351ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp351InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp351InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp351InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp351InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp351Stem.rs"
$cp351CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp351Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp351Stem}_validation.rs"
$cp351CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp351FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp351Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp351Stem}_fixture.rs"
$cp351PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp351Pipeline = "crates\ep_run\src\pipeline\$cp351PipelineStem.rs"
$cp351PipelineValidation = "crates\ep_run\src\pipeline\$cp351PipelineStem\validation.rs"
$cp351PipelineTests = "crates\ep_run\src\pipeline\$cp351PipelineStem\validation\tests.rs"
$cp351Serialization = "crates\ep_run\src\pipeline\$cp351PipelineStem\serialization.rs"
$cp351SnapshotSerialization = "crates\ep_run\src\pipeline\$cp351PipelineStem\serialization\snapshot.rs"
$cp351ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp351Compiler = "crates\ep_compiler\src\compiler.rs"
$cp351Audit = "scripts\quality\ideal-loads-structure-audit\cp351-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-total-output-assignment.ps1"

function Get-Cp351RustBraceBlock {
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
        } elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1)
            }
        }
    }
    throw "$Description has no complete brace block"
}

function Assert-Cp351TransitionContract {
    param([string]$Text)
    foreach ($pattern in @(
            'Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned',
            'let sensible\s*=\s*predecessor\.cooling_sensible_output_w\?;',
            'let total\s*=\s*sensible\s*/\s*input\.cooling_sensible_heat_ratio;',
            '(?s)cooling_sensible_output_w:\s*Some\(sensible\).*?cooling_sensible_heat_ratio:\s*Some\(input\.cooling_sensible_heat_ratio\).*?calculated_cooling_total_output_w:\s*Some\(total\)',
            '(?s)calculated_cooling_total_output_w:\s*prepared\.calculated_cooling_total_output_w,.*?cooling_total_output_w:\s*prepared\.calculated_cooling_total_output_w',
            'cooling_sensible_output_read_count\s*\+=\s*1',
            'cooling_sensible_heat_ratio_read_count\s*\+=\s*1',
            'cooling_total_output_calculation_count\s*\+=\s*1',
            'cooling_total_output_assignment_write_count\s*\+=\s*1'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP351 transition contract missing '$pattern'"
        }
    }
    if ($Text -match '\.is_(?:finite|nan)\(\)|\.clamp\(|\.recip\(\)|mul_add|SupplyEnthalpy|MixedAirEnthalpy|SupplyMassFlowRate') {
        throw "CP351 transition violates exact division, line-local gate, or source boundary"
    }
}

function Assert-Cp351RuntimeContract {
    param([string]$Text)
    foreach ($pattern in @(
            'route_partition\s*==\s*state\.transition_count',
            '(?s)dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count\s*==\s*prior\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count',
            '(?s)assignments\.checked_mul\(\s*PURCHASED_AIR_CALC_.*?_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)',
            'state\.source_site_execution_count\s*==\s*expected_sites',
            'site_counts\.into_iter\(\)\.all\(\|count\|\s*count\s*==\s*assignments\)'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP351 lifecycle algebra missing '$pattern'"
        }
    }
    foreach ($counter in @(
            "cooling_sensible_output_read_count",
            "cooling_sensible_heat_ratio_read_count",
            "cooling_total_output_calculation_count",
            "cooling_total_output_assignment_write_count"
        )) {
        if ($Text -notmatch ('state\.' + $counter + ',')) {
            throw "CP351 per-site parity missing for '$counter'"
        }
    }
}

function Assert-Cp351BindingContract {
    param([string]$Text)
    $cp350 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =")
    $cp351 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =")
    $cp352 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =")
    $cp353 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
    $cp354 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =")
    $cp355 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =")
    $cp356 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =")
    $cp357 = $Text.IndexOf("let calculation_cooling_constant_shr_case_break =")
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp350 -lt 0 -or $cp351 -le $cp350 -or $cp352 -le $cp351 -or $cp353 -le $cp352 -or $cp354 -le $cp353 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP350 then CP351 then CP352 then CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp351RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP351 numerical DTO"
    if ($dto -match '(?i)cp35[013]|sensible_output_assignment|total_output_assignment|overdrying_limit') {
        throw "CP350/CP351/CP353 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp351PipelineRootContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp351RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp351Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp351Lifecycle) + '\s*=\s*Some\s*\(').Count
    if ($none -ne 3 -or $some -ne 1) {
        throw "Pipeline must expose one direct CP351 Some and three non-direct None constructors"
    }
    $firewall = Get-Cp351RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp351Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP351 evidence exactly once"
    }
}

foreach ($required in @(
        $cp351Module, $cp351State, $cp351Transition, $cp351Release, $cp351Prefix,
        $cp351Runtime, $cp351Snapshot, $cp351Tests, $cp351IeeeTests,
        $cp351PublicTests, $cp351CorruptionTests, $cp351BindingAdapter,
        $cp351BindingTests, $cp351InitWitness, $cp351Coupled, $cp351Fixture,
        $cp351Pipeline, $cp351PipelineValidation, $cp351PipelineTests,
        $cp351Serialization, $cp351SnapshotSerialization, $cp351Audit
    )) {
    Assert-FileExists -Path $required -Description "CP351 structure"
}
Assert-LineLimit -Path $cp351Transition -Limit 450 -Description "CP351 transition"
Assert-LineLimit -Path $cp351Release -Limit 450 -Description "CP351 release"
Assert-LineLimit -Path $cp351Runtime -Limit 350 -Description "CP351 runtime validation"
Assert-LineLimit -Path $cp351Snapshot -Limit 350 -Description "CP351 snapshot validation"
Assert-LineLimit -Path $cp351Coupled -Limit 500 -Description "CP351 coupled validation"
Assert-LineLimit -Path $cp351PipelineValidation -Limit 500 -Description "CP351 pipeline validation"
Assert-LineLimit -Path $cp351Audit -Limit 500 -Description "CP351 structure audit"

# Exact source boundary, route algebra, operands, division, and complete null skips.
Assert-Contains -Path $cp351Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2218' -Description "CP351 source line"
Assert-Contains -Path $cp351Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2219' -Description "CP351 first excluded line"
Assert-ExactStringArray -Path $cp351Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER" -Expected $cp351Sites -Description "CP351 four sites"
Assert-Contains -Path $cp351State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP351 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count",
        "source_site_execution_count", "cooling_sensible_output_read_count",
        "cooling_sensible_heat_ratio_read_count", "cooling_total_output_calculation_count",
        "cooling_total_output_assignment_write_count"
    )) {
    Assert-Contains -Path $cp351State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP351 counter '$counter'"
}
$cp351TransitionText = Read-RepoText -Path $cp351Transition
$cp351RuntimeText = Read-RepoText -Path $cp351Runtime
$cp351BindingText = Read-RepoText -Path $cp351Binding
$cp351PipelineRootText = Read-RepoText -Path $cp351PipelineRoot
Assert-Cp351TransitionContract -Text $cp351TransitionText
Assert-Cp351RuntimeContract -Text $cp351RuntimeText
Assert-Cp351BindingContract -Text $cp351BindingText
Assert-Cp351PipelineRootContract -Text $cp351PipelineRootText
Assert-Contains -Path $cp351Snapshot -Pattern '(?s)calculated\.to_bits\(\)\s*==\s*\(sensible\s*/\s*ratio\)\.to_bits\(\).*?assigned\.to_bits\(\)\s*==\s*calculated\.to_bits\(\)' -Description "CP351 exact IEEE division and assignment"
Assert-Contains -Path $cp351Snapshot -Pattern '(?s)!snapshot\.cooling_sensible_output_read.*?cooling_sensible_output_w\.is_none\(\).*?!snapshot\.cooling_sensible_heat_ratio_read.*?cooling_sensible_heat_ratio\.is_none\(\).*?!snapshot\.cooling_total_output_calculated.*?calculated_cooling_total_output_w\.is_none\(\).*?!snapshot\.cooling_total_output_assigned.*?cooling_total_output_w\.is_none\(\)' -Description "CP351 complete-null skip"
Assert-NotContains -Path $cp351Snapshot -Pattern '\.is_(?:finite|nan)\(\)|\.clamp\(|\.recip\(\)' -Description "CP351 line-local result gate"

foreach ($test in @(
        "source_boundary_four_sites_and_seven_route_algebra_are_exact",
        "private_constant_shr_division_reads_cp350_owner_and_system_ratio_once"
    )) {
    Assert-Contains -Path $cp351Tests -Pattern $test -Description "CP351 private test '$test'"
}
foreach ($test in @(
        "zero_denominator_preserves_source_ieee_infinity_without_line_local_gate",
        "signed_zero_nan_and_infinite_division_bits_are_preserved"
    )) {
    Assert-Contains -Path $cp351IeeeTests -Pattern $test -Description "CP351 IEEE test '$test'"
}
foreach ($test in @(
        "public_none_and_inherited_routes_are_complete_null_skips",
        "lifecycle_summary_and_replay_are_exact_and_transactional"
    )) {
    Assert-Contains -Path $cp351PublicTests -Pattern $test -Description "CP351 public test '$test'"
}
foreach ($test in @(
        "supplied_latest_witness_identity_replay_and_overflow_are_transactional",
        "private_active_owner_requires_retained_cp350_recursive_proof",
        "coordinated_cp350_counterfactual_corruption_is_rejected",
        "model_owned_ratio_is_bit_exact_without_line_local_range_revalidation",
        "every_active_counter_overflow_rejects_before_mutation"
    )) {
    Assert-Contains -Path $cp351CorruptionTests -Pattern $test -Description "CP351 corruption test '$test'"
}

# CP350 is the sole numerator owner; the selected model is the sole denominator owner.
Assert-Contains -Path $cp351Release -Pattern 'PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Predecessor' -Description "CP351 exact CP350 predecessor type"
Assert-Contains -Path $cp351Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp350:\s*Predecessor,\s*\)' -Description "CP351 exact public arguments"
foreach ($pattern in @(
        'calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\s*\.latest\?',
        'cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_latest_witness',
        'predecessor_snapshots_match_bit_exact\(\s*direct,\s*direct_witness\s*\)',
        'cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release',
        'completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_is_consistent',
        'private_active_counterfactual_links_to_direct_release\(\s*runtime,\s*unit,\s*system,\s*direct,\s*predecessor',
        'cooling_sensible_heat_ratio:\s*system\.cooling_sensible_heat_ratio'
    )) {
    Assert-Contains -Path $cp351Prefix -Pattern $pattern -Description "CP351 recursive owner '$pattern'"
}
Assert-NotContains -Path $cp351Release -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|cooling_total_output_w\s*:' -Description "CP351 release scalar/numerical substitution"
Assert-NotContains -Path $cp351Prefix -Pattern '\.is_(?:finite|nan)\(\)|\.clamp\(|\.recip\(\)' -Description "CP351 owner range revalidation"

# Binding, coupled runtime, pipeline, serialization, and strict nonfeed.
Assert-Contains -Path $cp351CalcRoot -Pattern ('mod ' + [regex]::Escape($cp351Stem) + ';') -Description "CP351 calc module"
Assert-Contains -Path $cp351BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp351Stem)) -Description "CP351 binding adapter"
Assert-NotContains -Path $cp351BindingAdapter -Pattern 'cooling_sensible_output\s*:|cooling_sensible_heat_ratio\s*:|cooling_total_output\s*:|DirectZonePurchasedAirCouplingInput|latest_numerical' -Description "CP351 binding scalar/DTO firewall"
Assert-Contains -Path $cp351ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp351Stem) + ':') -Description "CP351 scheduled output"
Assert-Contains -Path $cp351BindingTestsRoot -Pattern ([regex]::Escape("${cp351Stem}_tests.rs")) -Description "CP351 binding tests"
Assert-Contains -Path $cp351BindingTests -Pattern 'scheduled_binding_completes_cp351_direct_none_route_without_operand_or_numeric_work' -Description "CP351 direct binding null test"
Assert-Contains -Path $cp351InitState -Pattern $cp351Stem -Description "CP351 init state"
Assert-Contains -Path $cp351InitUnit -Pattern $cp351Stem -Description "CP351 unit state"
Assert-Contains -Path $cp351InitWitnessRoot -Pattern $cp351Stem -Description "CP351 witness module"
Assert-Contains -Path $cp351CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp351Stem) + '_validation;') -Description "CP351 coupled validator"
Assert-Contains -Path $cp351Coupled -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment' -Description "coupled CP350 predecessor"
Assert-Contains -Path $cp351Coupled -Pattern '(?s)assigned\s*\.checked_mul\(.*?TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked 4Q"
Assert-Contains -Path $cp351Coupled -Pattern 'direct_constant_sensible_heat_ratio_total_output_assignment_count",\s*0' -Description "coupled direct Q zero"
Assert-NotContains -Path $cp351Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled DTO firewall"
Assert-Contains -Path $cp351FixtureRoot -Pattern $cp351Stem -Description "CP351 fixture registration"
Assert-Contains -Path $cp351CoupledTests -Pattern 'cp351_coupled_direct_none_route_is_complete_skip_and_numerical_output_remains_unfed' -Description "CP351 numerical nonfeed test"
Assert-Contains -Path $cp351PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp351PipelineStem) + ';') -Description "CP351 pipeline module"
Assert-Contains -Path $cp351PipelineRoot -Pattern ('"' + $cp351Lifecycle + '":\s*result\s*\.' + $cp351Lifecycle) -Description "CP351 lifecycle JSON"
Assert-Contains -Path $cp351PipelineValidation -Pattern 'sensible_output_assignment_cp350' -Description "pipeline CP350 predecessor"
Assert-Contains -Path $cp351PipelineValidation -Pattern '(?s)assigned\s*\.checked_mul\(.*?TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 4Q"
Assert-Contains -Path $cp351PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp395_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp351ArbitraryTests -Pattern $cp351Lifecycle -Description "arbitrary CP351 lifecycle"
foreach ($field in @(
        "cooling_sensible_output_w", "cooling_sensible_heat_ratio",
        "calculated_cooling_total_output_w", "cooling_total_output_w"
    )) {
    Assert-Contains -Path $cp351SnapshotSerialization -Pattern ('"' + $field + '"') -Description "CP351 JSON '$field'"
    Assert-Contains -Path $cp351SnapshotSerialization -Pattern ('"' + $field + '_ieee_bits"') -Description "CP351 JSON bits '$field'"
}
Assert-Contains -Path $cp351SnapshotSerialization -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "CP351 nonfinite numeric null"
Assert-Contains -Path $cp351SnapshotSerialization -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "CP351 authoritative bits"
Assert-Contains -Path $cp351Compiler -Pattern '(?s)"cooling_sensible_heat_ratio",\s*0\.7,\s*0\.0\.\.=1\.0' -Description "current compiler inclusive-zero range"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches($algorithmText, '(?m)^\s*"CP351 supersedes only CP350[^"\r\n]+",\s*$')
$capabilityAddenda = [regex]::Matches($capabilityText, '(?m)^\s*"CP351 additionally requires[^"\r\n]+",\s*$')
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP351 addenda"
}
foreach ($claim in @($algorithmAddenda) + @($capabilityAddenda)) {
    foreach ($pattern in @(
            $cp351SourceHash, 'physical executable line 2218', 'line 2219',
            $cp351Sites[0], $cp351Sites[1], $cp351Sites[2], $cp351Sites[3],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'CP350 sensible-output assignments', 'source_site_execution_count=4\*Q',
            'C0=S', 'Q=H=CSH=0', 'completed recursive CP350', 'counterfactual bridge',
            'cooling_sensible_output_w.*?solely owns the numerator',
            'IdealLoadsAirSystem\.cooling_sensible_heat_ratio.*?solely owns the denominator',
            'zero-denominator', 'NaN', 'infinity', '0\.0\.\.=1\.0',
            'minimum> 0\.0', 'maximum 1\.0', 'does not resolve',
            'CP350-to-CP351-to-unchanged-numerical', $cp351Lifecycle,
            'DirectZonePurchasedAirCouplingInput', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP351 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp351Stem/release\.rs::advance_direct_no_oa_calc_$cp351Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp351Stem\.rs::purchased_air_calc_${cp351Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp351Stem\.rs::${cp351TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp351Stem\.rs::${cp351TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($algorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP351 target count failed for '$($target.Pattern)'"
    }
}
$documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP351 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP351 Source-Ordered Cooling Positive-Supply Constant-Sensible-Heat-Ratio Total-Output Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP351 Constant-Sensible-Heat-Ratio Total-Output Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP351 Constant-Sensible-Heat-Ratio Total-Output Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP351 Constant-Sensible-Heat-Ratio Total-Output Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP351 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp351SourceHash, '2218', '2219', $cp351Sites[0], $cp351Sites[3],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L', '4\*Q', 'C0\s*=\s*S', 'Q\s*=\s*H\s*=\s*CSH\s*=\s*0',
            'CP350', 'cooling_sensible_output_w', 'cooling_sensible_heat_ratio',
            'signed[- ]zero', 'NaN', 'infinit', '0\.0\.\.=1\.0',
            '(?:minimum>\s*0\.0|>0\.0)', '1\.0',
            'CP350-to-CP351-to-unchanged-numerical', $cp351Lifecycle,
            'DirectZonePurchasedAirCouplingInput', '32\s+algorithms', '293\s+routines',
            'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP351 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP351\b' -Description "CP351 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP351 supersedes only CP350' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP351 additionally requires' -Description "generated capability addendum"

# Historical order, cumulative firewalls, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..350 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment' -Description "historical CP351 binding order"
}
foreach ($historical in 334..350) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp395_lifecycle_evidence' -Description "historical CP363 firewall"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp350AuditIndex = $mainAuditText.IndexOf("cp350-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
$cp351AuditIndex = $mainAuditText.IndexOf("cp351-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-total-output-assignment.ps1")
$completionIndex = $mainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp350AuditIndex -lt 0 -or $cp351AuditIndex -le $cp350AuditIndex -or $completionIndex -le $cp351AuditIndex) {
    throw "Master audit must dot-source CP351 after CP350 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 333' -Description "CP351 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP351 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp351-' -Description "CP351 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp351-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-total-output-assignment\.ps1::dot_sources' -Description "CP351 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 333 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 93 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP351 constant-SHR total-output-assignment structure audit passed."
