# CP353 maps only PurchasedAirManager.cc line 2221; line 2222 is excluded.
$cp353Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit"
$cp353PipelineStem = "purchased_air_$cp353Stem"
$cp353TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimit"
$cp353Lifecycle = "purchased_air_calc_${cp353Stem}_lifecycle"
$cp353SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp353Sites = @(
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit-maximum",
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-overdrying-limit-enthalpy",
    "evaluate-psy-h-fn-tdb-w-at-minimum-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
    "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit"
)
$cp353Module = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem.rs"
$cp353State = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\state.rs"
$cp353Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\transition.rs"
$cp353Release = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\release.rs"
$cp353Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\release\prefix_validation.rs"
$cp353Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\release\runtime_validation.rs"
$cp353Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\release\snapshot_validation.rs"
$cp353Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\tests\mod.rs"
$cp353IeeeTests = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\tests\ieee.rs"
$cp353PublicTests = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\tests\public_release.rs"
$cp353CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp353Stem\tests\release_corruption.rs"
$cp352Prefix = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\release\prefix_validation.rs"
$cp353CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp353Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp353Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp353BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp353Stem.rs"
$cp353BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp353Stem}_tests.rs"
$cp353BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp353ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp353InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp353InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp353InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp353InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp353Stem.rs"
$cp353CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp353Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp353Stem}_validation.rs"
$cp353CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp353FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp353Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp353Stem}_fixture.rs"
$cp353PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp353Pipeline = "crates\ep_run\src\pipeline\$cp353PipelineStem.rs"
$cp353PipelineValidation = "crates\ep_run\src\pipeline\$cp353PipelineStem\validation.rs"
$cp353PipelineTests = "crates\ep_run\src\pipeline\$cp353PipelineStem\validation\tests.rs"
$cp353Serialization = "crates\ep_run\src\pipeline\$cp353PipelineStem\serialization.rs"
$cp353SnapshotSerialization = "crates\ep_run\src\pipeline\$cp353PipelineStem\serialization\snapshot.rs"
$cp353ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp353ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp353_assertions.rs"
$cp353Psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$cp353Audit = "scripts\quality\ideal-loads-structure-audit\cp353-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1"

function Get-Cp353RustBraceBlock {
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

function Assert-Cp353TransitionContract {
    param([string]$Text)
    foreach ($pattern in @(
            'Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted',
            '(?s)energyplus_psy_h_fn_tdb_w\(\s*operands\.supply_temperature_c,\s*1\.0e-5\s*\)',
            '(?s)source_shaped_two_argument_maximum\(\s*operands\.supply_enthalpy_before_overdrying_limit_j_per_kg,\s*minimum_enthalpy,\s*\)',
            '(?s)assigned_supply_enthalpy_j_per_kg:\s*prepared\.maximum_supply_enthalpy_j_per_kg,.*?resulting_supply_enthalpy_j_per_kg:\s*prepared\.maximum_supply_enthalpy_j_per_kg',
            '(?s)fn source_shaped_two_argument_maximum\(.*?left:\s*f64,.*?right:\s*f64,.*?\)\s*->\s*f64\s*\{\s*if left < right \{ right \} else \{ left \}\s*\}',
            'supply_enthalpy_for_overdrying_limit_maximum_read_count\s*\+=\s*1',
            'supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count\s*\+=\s*1',
            'psychrometric_minimum_supply_enthalpy_evaluation_count\s*\+=\s*1',
            'source_shaped_two_argument_maximum_evaluation_count\s*\+=\s*1',
            'supply_enthalpy_assignment_write_count\s*\+=\s*1'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP353 transition contract missing '$pattern'"
        }
    }
    if ($Text -match 'f64::max|\.max\s*\(|total_cmp|partial_cmp|mul_add|\.is_(?:finite|nan)\(\)|\.clamp\(') {
        throw "CP353 transition violates source-shaped maximum or exact line-local IEEE behavior"
    }
}

function Assert-Cp353RuntimeContract {
    param([string]$Text)
    foreach ($pattern in @(
            'route_partition\s*==\s*state\.transition_count',
            '(?s)dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count\s*==\s*prior\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_count',
            '(?s)assignments\.checked_mul\(\s*PURCHASED_AIR_CALC_.*?_OVERDRYING_LIMIT_SOURCE_ORDER\s*\.len\(\)',
            'state\.source_site_execution_count\s*==\s*expected_sites',
            'site_counts\.into_iter\(\)\.all\(\|count\|\s*count\s*==\s*assignments\)'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP353 lifecycle algebra missing '$pattern'"
        }
    }
}

function Assert-Cp353BindingContract {
    param([string]$Text)
    $cp352 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =")
    $cp353 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
    $cp354 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =")
    $cp355 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =")
    $cp356 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =")
    $cp357 = $Text.IndexOf("let calculation_cooling_constant_shr_case_break =")
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp352 -lt 0 -or $cp353 -le $cp352 -or $cp354 -le $cp353 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP352 then CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp353RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP353 numerical DTO"
    if ($dto -match '(?i)cp35[23]|supply_enthalpy_assignment|overdrying_limit') {
        throw "CP352/CP353 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp353PipelineRootContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp353RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp353Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp353Lifecycle) + '\s*=\s*Some\s*\(').Count
    if ($none -ne 3 -or $some -ne 1) {
        throw "Pipeline must expose one direct CP353 Some and three non-direct None constructors"
    }
    $firewall = Get-Cp353RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp353Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP353 evidence exactly once"
    }
}

foreach ($required in @(
        $cp353Module, $cp353State, $cp353Transition, $cp353Release, $cp353Prefix,
        $cp353Runtime, $cp353Snapshot, $cp353Tests, $cp353IeeeTests,
        $cp353PublicTests, $cp353CorruptionTests, $cp353BindingAdapter,
        $cp353BindingTests, $cp353InitWitness, $cp353Coupled, $cp353Fixture,
        $cp353Pipeline, $cp353PipelineValidation, $cp353PipelineTests,
        $cp353Serialization, $cp353SnapshotSerialization, $cp353ArbitraryAssertions,
        $cp353Audit
    )) {
    Assert-FileExists -Path $required -Description "CP353 structure"
}
Assert-LineLimit -Path $cp353Transition -Limit 450 -Description "CP353 transition"
Assert-LineLimit -Path $cp353Release -Limit 450 -Description "CP353 release"
Assert-LineLimit -Path $cp353Prefix -Limit 500 -Description "CP353 prefix validation"
Assert-LineLimit -Path $cp353Runtime -Limit 350 -Description "CP353 runtime validation"
Assert-LineLimit -Path $cp353Snapshot -Limit 350 -Description "CP353 snapshot validation"
Assert-LineLimit -Path $cp353Coupled -Limit 500 -Description "CP353 coupled validation"
Assert-LineLimit -Path $cp353PipelineValidation -Limit 500 -Description "CP353 pipeline validation"
Assert-LineLimit -Path $cp353Audit -Limit 500 -Description "CP353 structure audit"

# Exact source boundary, route algebra, psychrometrics, maximum, and complete-null skips.
Assert-Contains -Path $cp353Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2221' -Description "CP353 source line"
Assert-Contains -Path $cp353Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2222' -Description "CP353 first excluded line"
Assert-ExactStringArray -Path $cp353Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER" -Expected $cp353Sites -Description "CP353 five sites"
Assert-Contains -Path $cp353State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP353 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count",
        "source_site_execution_count", "supply_enthalpy_for_overdrying_limit_maximum_read_count",
        "supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count",
        "psychrometric_minimum_supply_enthalpy_evaluation_count",
        "source_shaped_two_argument_maximum_evaluation_count",
        "supply_enthalpy_assignment_write_count"
    )) {
    Assert-Contains -Path $cp353State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP353 counter '$counter'"
}
$cp353TransitionText = Read-RepoText -Path $cp353Transition
$cp353RuntimeText = Read-RepoText -Path $cp353Runtime
$cp353BindingText = Read-RepoText -Path $cp353Binding
$cp353PipelineRootText = Read-RepoText -Path $cp353PipelineRoot
Assert-Cp353TransitionContract -Text $cp353TransitionText
Assert-Cp353RuntimeContract -Text $cp353RuntimeText
Assert-Cp353BindingContract -Text $cp353BindingText
Assert-Cp353PipelineRootContract -Text $cp353PipelineRootText
Assert-Contains -Path $cp353Psychrometrics -Pattern 'const ENERGYPLUS_MIN_HUMIDITY_RATIO:\s*f64\s*=\s*1\.0e-5;' -Description "canonical humidity floor literal"
Assert-Contains -Path $cp353Psychrometrics -Pattern '(?s)fn energyplus_psy_h_fn_tdb_w_raw\(.*?\)\s*->\s*f64\s*\{\s*1\.004_84e3 \* dry_bulb_c \+ humidity_ratio \* \(2\.500_94e6 \+ 1\.858_95e3 \* dry_bulb_c\)\s*\}' -Description "canonical PsyHFnTdbW grouping"
Assert-Contains -Path $cp353Psychrometrics -Pattern '(?s)pub fn energyplus_psy_h_fn_tdb_w\(.*?\)\s*->\s*f64\s*\{\s*energyplus_psy_h_fn_tdb_w_raw\(dry_bulb_c,\s*energyplus_humidity_ratio_floor\(humidity_ratio\)\)\s*\}' -Description "canonical PsyHFnTdbW floor path"
Assert-Contains -Path $cp353Snapshot -Pattern '(?s)energyplus_psy_h_fn_tdb_w\(temperature,\s*1\.0e-5\).*?source_shaped_two_argument_maximum\(pre_limit,\s*expected_psychrometric_minimum\).*?psychrometric_minimum\.to_bits\(\)\s*==\s*expected_psychrometric_minimum\.to_bits\(\).*?maximum\.to_bits\(\)\s*==\s*expected_maximum\.to_bits\(\).*?assigned\.to_bits\(\)\s*==\s*maximum\.to_bits\(\).*?resulting\.to_bits\(\)\s*==\s*assigned\.to_bits\(\)' -Description "CP353 exact IEEE psychrometric/maximum/assignment"
Assert-Contains -Path $cp353Snapshot -Pattern '(?s)!snapshot\.supply_enthalpy_for_overdrying_limit_maximum_read.*?supply_enthalpy_before_overdrying_limit_j_per_kg\s*\.is_none\(\).*?!snapshot\.supply_temperature_for_minimum_humidity_ratio_enthalpy_read.*?supply_temperature_c\.is_none\(\).*?!snapshot\.psychrometric_minimum_supply_enthalpy_evaluated.*?psychrometric_minimum_supply_enthalpy_j_per_kg\s*\.is_none\(\).*?!snapshot\.source_shaped_two_argument_maximum_evaluated.*?maximum_supply_enthalpy_j_per_kg\.is_none\(\).*?!snapshot\.supply_enthalpy_assignment_performed.*?assigned_supply_enthalpy_j_per_kg\.is_none\(\).*?resulting_supply_enthalpy_j_per_kg\.is_none\(\)' -Description "CP353 complete-null skip"
Assert-Contains -Path $cp353IeeeTests -Pattern 'source_shaped_maximum_is_left_biased_for_nan_ties_zero_and_infinity' -Description "CP353 IEEE maximum characterization"
Assert-Contains -Path $cp353IeeeTests -Pattern 'transition_preserves_nan_payload_and_canonical_psychrometric_bits' -Description "CP353 IEEE result characterization"

# CP352 owns pre-limit enthalpy; CP345 G/F/L selects recursively proven CP334 or CP344 temperature.
Assert-Contains -Path $cp353Release -Pattern 'PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot as Predecessor' -Description "CP353 exact CP352 predecessor"
Assert-Contains -Path $cp353Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp352:\s*Predecessor,\s*\)' -Description "CP353 exact public arguments"
foreach ($pattern in @(
        'calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\s*\.latest\?',
        'supply_enthalpy_assignment_latest_witness',
        'supply_enthalpy_assignment_snapshot_is_exact_direct_release',
        'completed_direct_.*?_supply_enthalpy_assignment_is_consistent',
        'private_active_counterfactual_links_to_direct_release',
        'resulting_supply_enthalpy_j_per_kg\?',
        'calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\s*\.latest\?',
        'humidity_ratio_mixed_air_assignment_latest_witness',
        'capacity_limit_guard_false_fallthrough_skipped',
        'capacity_limit_sensible_output_guard_false_fallthrough',
        'capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed',
        'calc_cooling_positive_supply_temperature_mixed_air_limit\s*\.latest\?',
        'assigned_supply_temperature_c\?',
        'calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\s*\.latest\?',
        'resulting_supply_temperature_c\?'
    )) {
    Assert-Contains -Path $cp353Prefix -Pattern $pattern -Description "CP353 recursive owner '$pattern'"
}
foreach ($pattern in @(
        'calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\s*\.latest\?',
        'calc_cooling_mixed_air_call\.latest\?',
        'mixed_air_enthalpy_projection_j_per_kg',
        'calc_cooling_supply_mass_flow_positive_guard\.latest\?',
        'positive_supply_mass_flow_body_entered'
    )) {
    Assert-Contains -Path $cp352Prefix -Pattern $pattern -Description "CP353 transitive CP352 bridge '$pattern'"
}
Assert-NotContains -Path $cp353Prefix -Pattern 'calc_cooling_positive_supply_enthalpy_assignment\s*\.latest|constant_sensible_heat_ratio_sensible_output_assignment\s*\.latest|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply' -Description "CP353 duplicate/numerical owner substitution"
Assert-NotContains -Path $cp353Release -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|supply_enthalpy_before_overdrying_limit_j_per_kg\s*:|supply_temperature_c\s*:' -Description "CP353 public scalar/numerical substitution"
Assert-Contains -Path $cp353CorruptionTests -Pattern 'private_active_operands_resolve_cp352_and_selected_temperature_owner' -Description "CP353 recursive owner resolution"
Assert-Contains -Path $cp353CorruptionTests -Pattern 'coordinated_private_cp352_and_provenance_owner_forgeries_are_rejected' -Description "CP353 coordinated owner corruption"

# Binding, coupled runtime, pipeline, serialization, and strict numerical nonfeed.
Assert-Contains -Path $cp353CalcRoot -Pattern ('mod ' + [regex]::Escape($cp353Stem) + ';') -Description "CP353 calc module"
Assert-Contains -Path $cp353BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp353Stem)) -Description "CP353 binding adapter"
Assert-NotContains -Path $cp353BindingAdapter -Pattern 'supply_enthalpy_before_overdrying_limit\s*:|supply_temperature\s*:|DirectZonePurchasedAirCouplingInput|latest_numerical' -Description "CP353 binding scalar/DTO firewall"
Assert-Contains -Path $cp353ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp353Stem) + ':') -Description "CP353 scheduled output"
Assert-Contains -Path $cp353BindingTestsRoot -Pattern ([regex]::Escape("${cp353Stem}_tests.rs")) -Description "CP353 binding tests"
Assert-Contains -Path $cp353BindingTests -Pattern 'scheduled_binding_places_cp353_after_cp352_as_a_complete_null_none_skip' -Description "CP353 binding regression"
Assert-Contains -Path $cp353InitState -Pattern $cp353Stem -Description "CP353 init state"
Assert-Contains -Path $cp353InitUnit -Pattern $cp353Stem -Description "CP353 unit state"
Assert-Contains -Path $cp353InitWitnessRoot -Pattern $cp353Stem -Description "CP353 witness module"
Assert-Contains -Path $cp353CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp353Stem) + '_validation;') -Description "CP353 coupled validator"
Assert-Contains -Path $cp353Coupled -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment' -Description "coupled CP352 predecessor"
Assert-Contains -Path $cp353Coupled -Pattern '(?s)executed\s*\.checked_mul\(.*?OVERDRYING_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked 5Q"
Assert-NotContains -Path $cp353Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled DTO firewall"
Assert-Contains -Path $cp353FixtureRoot -Pattern $cp353Stem -Description "CP353 fixture registration"
Assert-Contains -Path $cp353CoupledTests -Pattern 'cp353_coupled_direct_none_route_is_complete_skip_and_numerical_enthalpy_remains_unfed' -Description "CP353 numerical nonfeed test"
Assert-Contains -Path $cp353PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp353PipelineStem) + ';') -Description "CP353 pipeline module"
Assert-Contains -Path $cp353PipelineRoot -Pattern ('"' + $cp353Lifecycle + '":\s*result\s*\.' + $cp353Lifecycle) -Description "CP353 lifecycle JSON"
Assert-Contains -Path $cp353PipelineValidation -Pattern 'supply_enthalpy_assignment_cp352' -Description "pipeline CP352 predecessor"
Assert-Contains -Path $cp353PipelineValidation -Pattern '(?s)executed\s*\.checked_mul\(.*?OVERDRYING_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 5Q"
Assert-Contains -Path $cp353PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp440_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp353ArbitraryTests -Pattern 'cp353_assertions' -Description "arbitrary CP353 module"
Assert-Contains -Path $cp353ArbitraryAssertions -Pattern $cp353Lifecycle -Description "arbitrary CP353 lifecycle"
foreach ($field in @(
        "supply_enthalpy_before_overdrying_limit_j_per_kg", "supply_temperature_c",
        "psychrometric_minimum_supply_enthalpy_j_per_kg", "maximum_supply_enthalpy_j_per_kg",
        "assigned_supply_enthalpy_j_per_kg", "resulting_supply_enthalpy_j_per_kg"
    )) {
    Assert-Contains -Path $cp353SnapshotSerialization -Pattern ('"' + $field + '"') -Description "CP353 JSON '$field'"
    Assert-Contains -Path $cp353SnapshotSerialization -Pattern ('"' + $field + '_ieee_bits"') -Description "CP353 JSON bits '$field'"
}
Assert-Contains -Path $cp353SnapshotSerialization -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "CP353 nonfinite numeric null"
Assert-Contains -Path $cp353SnapshotSerialization -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "CP353 authoritative bits"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches($algorithmText, '(?m)^\s*"CP353 supersedes only CP352[^"\r\n]+",\s*$')
$capabilityAddenda = [regex]::Matches($capabilityText, '(?m)^\s*"CP353 additionally requires[^"\r\n]+",\s*$')
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP353 addenda"
}
foreach ($claim in @($algorithmAddenda) + @($capabilityAddenda)) {
    foreach ($pattern in @(
            $cp353SourceHash, 'physical executable line 2221', 'line 2222', 'CP354',
            $cp353Sites[0], $cp353Sites[1], $cp353Sites[2], $cp353Sites[3], $cp353Sites[4],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=5\*Q', 'C0=S', 'Q=H=CSH=0',
            'CP352.*?resulting_supply_enthalpy_j_per_kg', 'CP345', 'CP334', 'CP344',
            'counterfactual bridge', 'energyplus_psy_h_fn_tdb_w', '0x3ee4f8b588e368f1',
            'if left < right \{ right \} else \{ left \}', 'f64::max', 'left NaN',
            'right NaN', 'CP354', 'owner candidate',
            'CP352-to-CP353-to-unchanged-numerical', $cp353Lifecycle,
            'DirectZonePurchasedAirCouplingInput', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP353 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp353Stem/release\.rs::advance_direct_no_oa_calc_$cp353Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp353Stem\.rs::purchased_air_calc_${cp353Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp353Stem\.rs::${cp353TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp353Stem\.rs::${cp353TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($algorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP353 target count failed for '$($target.Pattern)'"
    }
}
$documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP353 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP353 Source-Ordered Cooling Positive-Supply Constant-Sensible-Heat-Ratio Overdrying Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP353 Constant-Sensible-Heat-Ratio Overdrying Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP353 Constant-Sensible-Heat-Ratio Overdrying Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP353 Constant-Sensible-Heat-Ratio Overdrying Limit Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP353 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp353SourceHash, '2221', '2222', 'CP354',
            $cp353Sites[0], $cp353Sites[1], $cp353Sites[2], $cp353Sites[3], $cp353Sites[4],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH', '5\*Q', 'C0\s*=\s*S',
            'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP352', 'CP345', 'CP334', 'CP344',
            'energyplus_psy_h_fn_tdb_w', '0x3ee4f8b588e368f1', 'f64::max',
            '(?:left NaN|NaN\s+left\s+operand)', '(?:right NaN|NaN\s+psychrometric\s+right\s+operand)', 'CP352-to-CP353-to-unchanged-numerical',
            $cp353Lifecycle, 'DirectZonePurchasedAirCouplingInput',
            '32\s+algorithms', '293\s+routines', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP353 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP353\b' -Description "CP353 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP353 supersedes only CP352' -Description "generated CP353 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP353 additionally requires' -Description "generated CP353 capability addendum"

# Historical order/firewall audits, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..352 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit' -Description "historical CP353 binding order"
}
foreach ($historical in 334..352) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp440_lifecycle_evidence' -Description "historical CP363 firewall"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp352AuditIndex = $mainAuditText.IndexOf("cp352-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-enthalpy-assignment.ps1")
$cp353AuditIndex = $mainAuditText.IndexOf("cp353-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$completionIndex = $mainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp352AuditIndex -lt 0 -or $cp353AuditIndex -le $cp352AuditIndex -or $completionIndex -le $cp353AuditIndex) {
    throw "Master audit must dot-source CP353 after CP352 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 378' -Description "CP353 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP353 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp353-' -Description "CP353 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp353-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit\.ps1::dot_sources' -Description "CP353 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 378 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 138 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP353 constant-SHR overdrying-limit structure audit passed."
