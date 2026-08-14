# CP352 maps only PurchasedAirManager.cc line 2219; line 2221 is excluded.
$cp352Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment"
$cp352PipelineStem = "purchased_air_$cp352Stem"
$cp352TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignment"
$cp352Lifecycle = "purchased_air_calc_${cp352Stem}_lifecycle"
$cp352SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp352Sites = @(
    "read-retained-mixed-air-enthalpy-for-constant-sensible-heat-ratio-supply-enthalpy-difference",
    "read-retained-cooling-total-output-for-constant-sensible-heat-ratio-specific-cooling-output-division",
    "read-retained-supply-mass-flow-rate-for-constant-sensible-heat-ratio-specific-cooling-output-division",
    "calculate-cooling-total-output-divided-by-supply-mass-flow-rate-for-constant-sensible-heat-ratio-supply-enthalpy",
    "calculate-mixed-air-enthalpy-minus-specific-cooling-output-for-constant-sensible-heat-ratio-supply-enthalpy",
    "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-case"
)
$cp352Module = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem.rs"
$cp352State = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\state.rs"
$cp352Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\transition.rs"
$cp352Release = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\release.rs"
$cp352Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\release\prefix_validation.rs"
$cp352Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\release\runtime_validation.rs"
$cp352Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\release\snapshot_validation.rs"
$cp352Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\tests\mod.rs"
$cp352IeeeTests = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\tests\ieee.rs"
$cp352PublicTests = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\tests\public_release.rs"
$cp352CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp352Stem\tests\release_corruption.rs"
$cp352CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp352Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp352Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp352BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp352Stem.rs"
$cp352BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp352Stem}_tests.rs"
$cp352BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp352ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp352InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp352InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp352InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp352InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp352Stem.rs"
$cp352CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp352Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp352Stem}_validation.rs"
$cp352CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp352FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp352Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp352Stem}_fixture.rs"
$cp352PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp352Pipeline = "crates\ep_run\src\pipeline\$cp352PipelineStem.rs"
$cp352PipelineValidation = "crates\ep_run\src\pipeline\$cp352PipelineStem\validation.rs"
$cp352PipelineTests = "crates\ep_run\src\pipeline\$cp352PipelineStem\validation\tests.rs"
$cp352Serialization = "crates\ep_run\src\pipeline\$cp352PipelineStem\serialization.rs"
$cp352SnapshotSerialization = "crates\ep_run\src\pipeline\$cp352PipelineStem\serialization\snapshot.rs"
$cp352ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp352Audit = "scripts\quality\ideal-loads-structure-audit\cp352-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-enthalpy-assignment.ps1"

function Get-Cp352RustBraceBlock {
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

function Assert-Cp352TransitionContract {
    param([string]$Text)
    foreach ($pattern in @(
            'Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned',
            '(?s)let specific\s*=\s*operands\.cooling_total_output_w\s*/\s*operands\.supply_mass_flow_rate_kg_per_s;',
            'let enthalpy\s*=\s*operands\.mixed_air_enthalpy_j_per_kg\s*-\s*specific;',
            '(?s)specific_cooling_output_j_per_kg:\s*Some\(specific\).*?calculated_supply_enthalpy_j_per_kg:\s*Some\(enthalpy\)',
            '(?s)assigned_supply_enthalpy_j_per_kg:\s*prepared\.calculated_supply_enthalpy_j_per_kg,.*?resulting_supply_enthalpy_j_per_kg:\s*prepared\.calculated_supply_enthalpy_j_per_kg',
            'mixed_air_enthalpy_read_count\s*\+=\s*1',
            'cooling_total_output_read_count\s*\+=\s*1',
            'supply_mass_flow_rate_read_count\s*\+=\s*1',
            'specific_cooling_output_calculation_count\s*\+=\s*1',
            'supply_enthalpy_calculation_count\s*\+=\s*1',
            'supply_enthalpy_assignment_write_count\s*\+=\s*1'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP352 transition contract missing '$pattern'"
        }
    }
    if ($Text -match '\.is_(?:finite|nan)\(\)|\.clamp\(|\.recip\(\)|mul_add|PsyHFnTdbW|\bmax\s*\(') {
        throw "CP352 transition violates exact divide/subtract grouping or the line-2221 boundary"
    }
}

function Assert-Cp352RuntimeContract {
    param([string]$Text)
    foreach ($pattern in @(
            'route_partition\s*==\s*state\.transition_count',
            '(?s)dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_count\s*==\s*prior\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count',
            '(?s)assignments\.checked_mul\(\s*PURCHASED_AIR_CALC_.*?_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)',
            'state\.source_site_execution_count\s*==\s*expected_sites',
            'site_counts\.into_iter\(\)\.all\(\|count\|\s*count\s*==\s*assignments\)'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP352 lifecycle algebra missing '$pattern'"
        }
    }
}

function Assert-Cp352BindingContract {
    param([string]$Text)
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
    if ($cp351 -lt 0 -or $cp352 -le $cp351 -or $cp353 -le $cp352 -or $cp354 -le $cp353 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP351 then CP352 then CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp352RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP352 numerical DTO"
    if ($dto -match '(?i)cp35[123]|total_output_assignment|constant_sensible_heat_ratio_supply_enthalpy_assignment|overdrying_limit') {
        throw "CP351/CP352/CP353 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp352PipelineRootContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp352RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp352Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp352Lifecycle) + '\s*=\s*Some\s*\(').Count
    if ($none -ne 3 -or $some -ne 1) {
        throw "Pipeline must expose one direct CP352 Some and three non-direct None constructors"
    }
    $firewall = Get-Cp352RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp352Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP352 evidence exactly once"
    }
}

foreach ($required in @(
        $cp352Module, $cp352State, $cp352Transition, $cp352Release, $cp352Prefix,
        $cp352Runtime, $cp352Snapshot, $cp352Tests, $cp352IeeeTests,
        $cp352PublicTests, $cp352CorruptionTests, $cp352BindingAdapter,
        $cp352BindingTests, $cp352InitWitness, $cp352Coupled, $cp352Fixture,
        $cp352Pipeline, $cp352PipelineValidation, $cp352PipelineTests,
        $cp352Serialization, $cp352SnapshotSerialization, $cp352Audit
    )) {
    Assert-FileExists -Path $required -Description "CP352 structure"
}
Assert-LineLimit -Path $cp352Transition -Limit 450 -Description "CP352 transition"
Assert-LineLimit -Path $cp352Release -Limit 450 -Description "CP352 release"
Assert-LineLimit -Path $cp352Prefix -Limit 350 -Description "CP352 prefix validation"
Assert-LineLimit -Path $cp352Runtime -Limit 350 -Description "CP352 runtime validation"
Assert-LineLimit -Path $cp352Snapshot -Limit 350 -Description "CP352 snapshot validation"
Assert-LineLimit -Path $cp352Coupled -Limit 500 -Description "CP352 coupled validation"
Assert-LineLimit -Path $cp352PipelineValidation -Limit 500 -Description "CP352 pipeline validation"
Assert-LineLimit -Path $cp352Audit -Limit 500 -Description "CP352 structure audit"

# Exact source boundary, route algebra, grouping, and complete-null skips.
Assert-Contains -Path $cp352Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2219' -Description "CP352 source line"
Assert-Contains -Path $cp352Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2221' -Description "CP352 first excluded line"
Assert-ExactStringArray -Path $cp352Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER" -Expected $cp352Sites -Description "CP352 six sites"
Assert-Contains -Path $cp352State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP352 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_count",
        "source_site_execution_count", "mixed_air_enthalpy_read_count",
        "cooling_total_output_read_count", "supply_mass_flow_rate_read_count",
        "specific_cooling_output_calculation_count", "supply_enthalpy_calculation_count",
        "supply_enthalpy_assignment_write_count"
    )) {
    Assert-Contains -Path $cp352State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP352 counter '$counter'"
}
$cp352TransitionText = Read-RepoText -Path $cp352Transition
$cp352RuntimeText = Read-RepoText -Path $cp352Runtime
$cp352BindingText = Read-RepoText -Path $cp352Binding
$cp352PipelineRootText = Read-RepoText -Path $cp352PipelineRoot
Assert-Cp352TransitionContract -Text $cp352TransitionText
Assert-Cp352RuntimeContract -Text $cp352RuntimeText
Assert-Cp352BindingContract -Text $cp352BindingText
Assert-Cp352PipelineRootContract -Text $cp352PipelineRootText
Assert-Contains -Path $cp352Snapshot -Pattern '(?s)specific\.to_bits\(\)\s*==\s*\(total\s*/\s*flow\)\.to_bits\(\).*?calculated\.to_bits\(\)\s*==\s*\(mixed\s*-\s*specific\)\.to_bits\(\).*?assigned\.to_bits\(\)\s*==\s*calculated\.to_bits\(\).*?resulting\.to_bits\(\)\s*==\s*assigned\.to_bits\(\)' -Description "CP352 exact IEEE divide/subtract/assignment"
Assert-Contains -Path $cp352Snapshot -Pattern '(?s)!snapshot\.mixed_air_enthalpy_read.*?mixed_air_enthalpy_j_per_kg\.is_none\(\).*?!snapshot\.cooling_total_output_read.*?cooling_total_output_w\.is_none\(\).*?!snapshot\.supply_mass_flow_rate_read.*?supply_mass_flow_rate_kg_per_s\.is_none\(\).*?!snapshot\.specific_cooling_output_calculated.*?specific_cooling_output_j_per_kg\.is_none\(\).*?!snapshot\.supply_enthalpy_calculated.*?calculated_supply_enthalpy_j_per_kg\.is_none\(\).*?!snapshot\.supply_enthalpy_assigned.*?assigned_supply_enthalpy_j_per_kg\.is_none\(\).*?resulting_supply_enthalpy_j_per_kg\.is_none\(\)' -Description "CP352 complete-null skip"
Assert-NotContains -Path $cp352Snapshot -Pattern '\.is_(?:finite|nan)\(\)|\.clamp\(|\.recip\(\)|mul_add' -Description "CP352 line-local derived-result gate"

# CP351 total output, CP329 mixed enthalpy, and CP330 positive flow are sole owners.
Assert-Contains -Path $cp352Release -Pattern 'PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot as Predecessor' -Description "CP352 exact CP351 predecessor"
Assert-Contains -Path $cp352Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp351:\s*Predecessor,\s*\)' -Description "CP352 exact public arguments"
foreach ($pattern in @(
        'calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\s*\.latest\?',
        'total_output_assignment_latest_witness',
        'total_output_assignment_snapshots_match_bit_exact\(\s*direct,\s*direct_witness\s*\)',
        'total_output_assignment_snapshot_is_exact_direct_release',
        'completed_direct_.*?_total_output_assignment_is_consistent',
        'private_active_counterfactual_links_to_direct_release\(\s*runtime,\s*unit,\s*system,\s*direct,\s*predecessor',
        'calc_cooling_mixed_air_call\.latest\?',
        'cooling_mixed_air_call_latest_witness',
        'mixed_air_enthalpy_projection_j_per_kg',
        'completed_direct_cooling_mixed_air_call_is_consistent',
        'calc_cooling_supply_mass_flow_positive_guard\.latest\?',
        'cooling_supply_mass_flow_positive_guard_latest_witness',
        'positive_supply_mass_flow_body_entered',
        'completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent',
        'flow\s*<=\s*0\.0',
        'flow\.is_nan\(\)'
    )) {
    Assert-Contains -Path $cp352Prefix -Pattern $pattern -Description "CP352 recursive owner '$pattern'"
}
Assert-NotContains -Path $cp352Prefix -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|PsyHFnTdbW' -Description "CP352 helper/numerical substitution"
Assert-NotContains -Path $cp352Release -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|mixed_air_enthalpy_j_per_kg\s*:|cooling_total_output_w\s*:|supply_mass_flow_rate_kg_per_s\s*:' -Description "CP352 public scalar/numerical substitution"

# Binding, coupled runtime, pipeline, serialization, and strict nonfeed.
Assert-Contains -Path $cp352CalcRoot -Pattern ('mod ' + [regex]::Escape($cp352Stem) + ';') -Description "CP352 calc module"
Assert-Contains -Path $cp352BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp352Stem)) -Description "CP352 binding adapter"
Assert-NotContains -Path $cp352BindingAdapter -Pattern 'mixed_air_enthalpy\s*:|cooling_total_output\s*:|supply_mass_flow_rate\s*:|DirectZonePurchasedAirCouplingInput|latest_numerical' -Description "CP352 binding scalar/DTO firewall"
Assert-Contains -Path $cp352ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp352Stem) + ':') -Description "CP352 scheduled output"
Assert-Contains -Path $cp352BindingTestsRoot -Pattern ([regex]::Escape("${cp352Stem}_tests.rs")) -Description "CP352 binding tests"
Assert-Contains -Path $cp352BindingTests -Pattern 'cp352' -Description "CP352 binding regression"
Assert-Contains -Path $cp352InitState -Pattern $cp352Stem -Description "CP352 init state"
Assert-Contains -Path $cp352InitUnit -Pattern $cp352Stem -Description "CP352 unit state"
Assert-Contains -Path $cp352InitWitnessRoot -Pattern $cp352Stem -Description "CP352 witness module"
Assert-Contains -Path $cp352CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp352Stem) + '_validation;') -Description "CP352 coupled validator"
Assert-Contains -Path $cp352Coupled -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment' -Description "coupled CP351 predecessor"
Assert-Contains -Path $cp352Coupled -Pattern '(?s)assigned\s*\.checked_mul\(.*?SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked 6Q"
Assert-NotContains -Path $cp352Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled DTO firewall"
Assert-Contains -Path $cp352FixtureRoot -Pattern $cp352Stem -Description "CP352 fixture registration"
Assert-Contains -Path $cp352CoupledTests -Pattern 'cp352_coupled_direct_none_route_is_complete_skip_and_numerical_enthalpy_remains_unfed' -Description "CP352 numerical nonfeed test"
Assert-Contains -Path $cp352PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp352PipelineStem) + ';') -Description "CP352 pipeline module"
Assert-Contains -Path $cp352PipelineRoot -Pattern ('"' + $cp352Lifecycle + '":\s*result\s*\.' + $cp352Lifecycle) -Description "CP352 lifecycle JSON"
Assert-Contains -Path $cp352PipelineValidation -Pattern 'total_output_assignment_cp351' -Description "pipeline CP351 predecessor"
Assert-Contains -Path $cp352PipelineValidation -Pattern '(?s)assigned\s*\.checked_mul\(.*?SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 6Q"
Assert-Contains -Path $cp352PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp425_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp352ArbitraryTests -Pattern $cp352Lifecycle -Description "arbitrary CP352 lifecycle"
foreach ($field in @(
        "mixed_air_enthalpy_j_per_kg", "cooling_total_output_w",
        "supply_mass_flow_rate_kg_per_s", "specific_cooling_output_j_per_kg",
        "calculated_supply_enthalpy_j_per_kg", "assigned_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg"
    )) {
    Assert-Contains -Path $cp352SnapshotSerialization -Pattern ('"' + $field + '"') -Description "CP352 JSON '$field'"
    Assert-Contains -Path $cp352SnapshotSerialization -Pattern ('"' + $field + '_ieee_bits"') -Description "CP352 JSON bits '$field'"
}
Assert-Contains -Path $cp352SnapshotSerialization -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "CP352 nonfinite numeric null"
Assert-Contains -Path $cp352SnapshotSerialization -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "CP352 authoritative bits"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches($algorithmText, '(?m)^\s*"CP352 supersedes only CP351[^"\r\n]+",\s*$')
$capabilityAddenda = [regex]::Matches($capabilityText, '(?m)^\s*"CP352 additionally requires[^"\r\n]+",\s*$')
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP352 addenda"
}
foreach ($claim in @($algorithmAddenda) + @($capabilityAddenda)) {
    foreach ($pattern in @(
            $cp352SourceHash, 'physical executable line 2219', 'line 2220', 'line 2221',
            $cp352Sites[0], $cp352Sites[1], $cp352Sites[2], $cp352Sites[3], $cp352Sites[4], $cp352Sites[5],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'CP351 total-output assignments', 'source_site_execution_count=6\*Q',
            'C0=S', 'Q=H=CSH=0', 'completed recursive', 'counterfactual bridge',
            'CP329.*?mixed_air_enthalpy_projection_j_per_kg',
            'CP351.*?cooling_total_output_w', 'CP330.*?supply_mass_flow_rate_kg_per_s',
            'never re-reads.*?cooling_sensible_heat_ratio', 'supply_mass_flow_rate > 0\.0',
            'zero/negative-flow', 'private-release reachability', 'NaN', 'infinity',
            'CP353', 'sole-owner candidate',
            'CP351-to-CP352-to-unchanged-numerical', $cp352Lifecycle,
            'DirectZonePurchasedAirCouplingInput', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP352 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp352Stem/release\.rs::advance_direct_no_oa_calc_$cp352Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp352Stem\.rs::purchased_air_calc_${cp352Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp352Stem\.rs::${cp352TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp352Stem\.rs::${cp352TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($algorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP352 target count failed for '$($target.Pattern)'"
    }
}
$documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP352 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP352 Source-Ordered Cooling Positive-Supply Constant-Sensible-Heat-Ratio Supply-Enthalpy Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP352 Constant-Sensible-Heat-Ratio Supply-Enthalpy Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP352 Constant-Sensible-Heat-Ratio Supply-Enthalpy Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP352 Constant-Sensible-Heat-Ratio Supply-Enthalpy Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP352 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp352SourceHash, '2219', '2220', '2221', $cp352Sites[0], $cp352Sites[5],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L', '6\*Q', 'C0\s*=\s*S', 'Q\s*=\s*H\s*=\s*CSH\s*=\s*0',
            'CP329', 'CP330', 'CP351', 'cooling_total_output_w', 'cooling_sensible_heat_ratio',
            '(?:flow|supply_mass_flow_rate)[^\r\n>]{0,16}>\s*0\.0', 'zero/negative-flow',
            'private-release\s+reachability', 'NaN', 'infinit', 'CP353', 'sole-owner candidate',
            'CP351-to-CP352-to-unchanged-numerical', $cp352Lifecycle,
            'DirectZonePurchasedAirCouplingInput', '32\s+algorithms', '293\s+routines',
            'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP352 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP352\b' -Description "CP352 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP352 supersedes only CP351' -Description "generated CP352 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP352 additionally requires' -Description "generated CP352 capability addendum"

# Historical order/firewall audits, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..351 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment' -Description "historical CP352 binding order"
}
foreach ($historical in 334..351) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp425_lifecycle_evidence' -Description "historical CP363 firewall"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp351AuditIndex = $mainAuditText.IndexOf("cp351-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-total-output-assignment.ps1")
$cp352AuditIndex = $mainAuditText.IndexOf("cp352-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-enthalpy-assignment.ps1")
$completionIndex = $mainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp351AuditIndex -lt 0 -or $cp352AuditIndex -le $cp351AuditIndex -or $completionIndex -le $cp352AuditIndex) {
    throw "Master audit must dot-source CP352 after CP351 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 363' -Description "CP352 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP352 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp352-' -Description "CP352 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp352-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-enthalpy-assignment\.ps1::dot_sources' -Description "CP352 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 363 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 123 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP352 constant-SHR supply-enthalpy-assignment structure audit passed."
