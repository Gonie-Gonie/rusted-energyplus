# CP354 maps only PurchasedAirManager.cc line 2222; line 2224 is excluded.
$cp354Stem = "cooling_constant_shr_supply_humidity_ratio_overdrying_limit"
$cp353Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit"
$cp354PipelineStem = "purchased_air_$cp354Stem"
$cp354TypeStem = "PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimit"
$cp354Lifecycle = "purchased_air_calc_${cp354Stem}_lifecycle"
$cp354SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp354Sites = @(
    "read-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit-minimum",
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-constant-sensible-heat-ratio-overdrying-limit",
    "apply-source-shaped-two-argument-minimum-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit"
)
$cp354Module = "crates\ep_runtime\src\ideal_loads\calc\$cp354Stem.rs"
$cp354State = "crates\ep_runtime\src\ideal_loads\calc\$cp354Stem\state.rs"
$cp354Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp354Stem\transition.rs"
$cp354Release = "crates\ep_runtime\src\ideal_loads\calc\$cp354Stem\release.rs"
$cp354Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp354Stem\release\prefix_validation.rs"
$cp354Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp354Stem\release\runtime_validation.rs"
$cp354Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp354Stem\release\snapshot_validation.rs"
$cp354Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp354Stem\tests\mod.rs"
$cp354MinimumHelper = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\transition.rs"
$cp354Psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$cp354CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp354Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp354Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp354BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp354Stem.rs"
$cp354BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp354Stem}_tests.rs"
$cp354BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp354ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp354InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp354InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp354InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp354InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp354Stem.rs"
$cp354CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp354Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp354Stem}_validation.rs"
$cp354CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp354FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp354Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp354Stem}_fixture.rs"
$cp354PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp354Pipeline = "crates\ep_run\src\pipeline\$cp354PipelineStem.rs"
$cp354PipelineValidation = "crates\ep_run\src\pipeline\$cp354PipelineStem\validation.rs"
$cp354PipelineTests = "crates\ep_run\src\pipeline\$cp354PipelineStem\validation\tests.rs"
$cp354Serialization = "crates\ep_run\src\pipeline\$cp354PipelineStem\serialization.rs"
$cp354SnapshotSerialization = "crates\ep_run\src\pipeline\$cp354PipelineStem\serialization\snapshot.rs"
$cp354ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp354ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp353_assertions.rs"
$cp354Audit = "scripts\quality\ideal-loads-structure-audit\cp354-cooling-constant-shr-supply-humidity-ratio-overdrying-limit.ps1"

function Get-Cp354RustBraceBlock {
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

function Assert-Cp354TransitionContract {
    param([string]$Text)
    foreach ($pattern in @(
            'Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted',
            '(?s)energyplus_psy_w_fn_tdb_h\(\s*operands\.supply_temperature_c,\s*operands\.supply_enthalpy_j_per_kg,\s*\)',
            '(?s)source_shaped_two_argument_minimum\(\s*operands\.supply_humidity_ratio_before_overdrying_limit,\s*psychrometric_humidity_ratio,\s*\)',
            '(?s)assigned_supply_humidity_ratio:\s*prepared\.minimum_supply_humidity_ratio,.*?resulting_supply_humidity_ratio:\s*prepared\.minimum_supply_humidity_ratio',
            'supply_humidity_ratio_for_overdrying_limit_minimum_read_count\s*\+=\s*1',
            'supply_temperature_for_humidity_ratio_inversion_read_count\s*\+=\s*1',
            'supply_enthalpy_for_humidity_ratio_inversion_read_count\s*\+=\s*1',
            'psychrometric_supply_humidity_ratio_evaluation_count\s*\+=\s*1',
            'source_shaped_two_argument_minimum_evaluation_count\s*\+=\s*1',
            'supply_humidity_ratio_assignment_write_count\s*\+=\s*1'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP354 transition contract missing '$pattern'"
        }
    }
    if ($Text -match 'f64::min|\.min\s*\(|total_cmp|partial_cmp|mul_add|\.is_(?:finite|nan)\(\)|\.clamp\(') {
        throw "CP354 transition violates exact source-shaped minimum/IEEE behavior"
    }
}

function Assert-Cp354RuntimeContract {
    param([string]$Text)
    foreach ($pattern in @(
            'route_partition\s*==\s*state\.transition_count',
            '(?s)supply_humidity_ratio_overdrying_limit_count\s*==\s*prior\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count',
            '(?s)assignments\.checked_mul\(\s*PURCHASED_AIR_CALC_.*?_OVERDRYING_LIMIT_SOURCE_ORDER\s*\.len\(\)',
            'state\.source_site_execution_count\s*==\s*expected_sites',
            'site_counts\.into_iter\(\)\.all\(\|count\|\s*count\s*==\s*assignments\)'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP354 lifecycle algebra missing '$pattern'"
        }
    }
}

function Assert-Cp354BindingContract {
    param([string]$Text)
    $cp353 = $Text.IndexOf("let calculation_$cp353Stem =")
    $cp354 = $Text.IndexOf("let calculation_$cp354Stem =")
    $cp355 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =")
    $cp356 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =")
    $cp357 = $Text.IndexOf("let calculation_cooling_constant_shr_case_break =")
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp353 -lt 0 -or $cp354 -le $cp353 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp354RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP354 numerical DTO"
    if ($dto -match '(?i)cp35[34]|overdrying_limit|supply_humidity_ratio') {
        throw "CP353/CP354 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp354PipelineRootContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp354RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp354Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp354Lifecycle) + '\s*=\s*Some\s*\(').Count
    if ($none -ne 3 -or $some -ne 1) {
        throw "Pipeline must expose one direct CP354 Some and three non-direct None constructors"
    }
    $firewall = Get-Cp354RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp354Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP354 evidence exactly once"
    }
}

foreach ($required in @(
        $cp354Module, $cp354State, $cp354Transition, $cp354Release, $cp354Prefix,
        $cp354Runtime, $cp354Snapshot, $cp354Tests, $cp354MinimumHelper,
        $cp354BindingAdapter, $cp354BindingTests, $cp354InitWitness,
        $cp354Coupled, $cp354Fixture, $cp354Pipeline, $cp354PipelineValidation,
        $cp354PipelineTests, $cp354Serialization, $cp354SnapshotSerialization,
        $cp354ArbitraryAssertions, $cp354Audit
    )) {
    Assert-FileExists -Path $required -Description "CP354 structure"
}
foreach ($limited in @(
        $cp354Transition, $cp354Release, $cp354Prefix, $cp354Runtime,
        $cp354Snapshot, $cp354Coupled, $cp354PipelineValidation, $cp354Audit
    )) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP354 bounded structure"
}

# Exact source boundary, routes, six-site algebra, numerical path, and null skip.
Assert-Contains -Path $cp354Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2222' -Description "CP354 source line"
Assert-Contains -Path $cp354Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2224' -Description "CP354 first excluded line"
Assert-ExactStringArray -Path $cp354Module -Name "PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER" -Expected $cp354Sites -Description "CP354 six sites"
Assert-Contains -Path $cp354State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP354 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_count",
        "source_site_execution_count",
        "supply_humidity_ratio_for_overdrying_limit_minimum_read_count",
        "supply_temperature_for_humidity_ratio_inversion_read_count",
        "supply_enthalpy_for_humidity_ratio_inversion_read_count",
        "psychrometric_supply_humidity_ratio_evaluation_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_assignment_write_count"
    )) {
    Assert-Contains -Path $cp354State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP354 counter '$counter'"
}
$cp354TransitionText = Read-RepoText -Path $cp354Transition
$cp354RuntimeText = Read-RepoText -Path $cp354Runtime
$cp354BindingText = Read-RepoText -Path $cp354Binding
$cp354PipelineRootText = Read-RepoText -Path $cp354PipelineRoot
Assert-Cp354TransitionContract -Text $cp354TransitionText
Assert-Cp354RuntimeContract -Text $cp354RuntimeText
Assert-Cp354BindingContract -Text $cp354BindingText
Assert-Cp354PipelineRootContract -Text $cp354PipelineRootText
Assert-Contains -Path $cp354MinimumHelper -Pattern '(?s)fn source_shaped_two_argument_minimum\(.*?left:\s*f64,.*?right:\s*f64,.*?\)\s*->\s*f64\s*\{\s*if left < right \{ left \} else \{ right \}\s*\}' -Description "source-shaped right-biased minimum"
Assert-NotContains -Path $cp354MinimumHelper -Pattern 'fn source_shaped_two_argument_minimum(?s:.*?)f64::min' -Description "minimum does not use f64::min"
Assert-Contains -Path $cp354Psychrometrics -Pattern 'const ENERGYPLUS_MIN_HUMIDITY_RATIO:\s*f64\s*=\s*1\.0e-5;' -Description "canonical humidity fallback"
Assert-Contains -Path $cp354Psychrometrics -Pattern '(?s)pub fn energyplus_psy_w_fn_tdb_h\(dry_bulb_c:\s*f64,\s*enthalpy_j_per_kg:\s*f64\)\s*->\s*f64\s*\{\s*let humidity_ratio =\s*\(enthalpy_j_per_kg - 1\.004_84e3 \* dry_bulb_c\)\s*/\s*\(2\.500_94e6 \+ 1\.858_95e3 \* dry_bulb_c\);\s*if humidity_ratio < 0\.0 \{\s*ENERGYPLUS_MIN_HUMIDITY_RATIO\s*\}\s*else\s*\{\s*humidity_ratio\s*\}\s*\}' -Description "canonical exact PsyWFnTdbH path"
Assert-Contains -Path $cp354Snapshot -Pattern '(?s)energyplus_psy_w_fn_tdb_h\(temperature,\s*enthalpy\).*?source_shaped_two_argument_minimum\(pre_limit,\s*expected_psychrometric\).*?psychrometric\.to_bits\(\)\s*==\s*expected_psychrometric\.to_bits\(\).*?minimum\.to_bits\(\)\s*==\s*expected_minimum\.to_bits\(\).*?assigned\.to_bits\(\)\s*==\s*minimum\.to_bits\(\).*?resulting\.to_bits\(\)\s*==\s*assigned\.to_bits\(\)' -Description "CP354 exact IEEE inversion/minimum/assignment"
Assert-Contains -Path $cp354Snapshot -Pattern '(?s)!snapshot\.supply_humidity_ratio_for_overdrying_limit_minimum_read.*?supply_humidity_ratio_before_overdrying_limit\s*\.is_none\(\).*?!snapshot\.supply_temperature_for_humidity_ratio_inversion_read.*?supply_temperature_c\.is_none\(\).*?!snapshot\.supply_enthalpy_for_humidity_ratio_inversion_read.*?supply_enthalpy_j_per_kg\.is_none\(\).*?!snapshot\.psychrometric_supply_humidity_ratio_evaluated.*?psychrometric_supply_humidity_ratio\.is_none\(\).*?!snapshot\.source_shaped_two_argument_minimum_evaluated.*?minimum_supply_humidity_ratio\.is_none\(\).*?!snapshot\.supply_humidity_ratio_assignment_performed.*?assigned_supply_humidity_ratio\.is_none\(\).*?resulting_supply_humidity_ratio\.is_none\(\)' -Description "CP354 complete-null skip"
foreach ($test in @(
        'source_boundary_six_sites_and_seven_route_algebra_are_exact',
        'active_transition_uses_canonical_inversion_and_source_shaped_minimum',
        'inactive_routes_are_complete_null_and_direct_none_is_exact',
        'source_shaped_minimum_preserves_right_bias_and_ieee_bits',
        'canonical_humidity_ratio_inversion_keeps_floor_and_ieee_behavior',
        'bit_exact_snapshot_matching_distinguishes_nan_payloads',
        'active_counter_overflow_is_transactional'
    )) {
    Assert-Contains -Path $cp354Tests -Pattern $test -Description "CP354 regression '$test'"
}

# CP353/CP345 retained owners and recursive private bridge; no scalar/DTO substitution.
Assert-Contains -Path $cp354Release -Pattern 'PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Predecessor' -Description "CP354 exact CP353 predecessor"
Assert-Contains -Path $cp354Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp353:\s*Predecessor,\s*\)' -Description "CP354 exact public arguments"
foreach ($pattern in @(
        'calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\s*\.latest\?',
        'overdrying_limit_latest_witness',
        'overdrying_limit_snapshot_is_exact_direct_release',
        'completed_direct_.*?_overdrying_limit_is_consistent',
        'private_active_counterfactual_links_to_direct_release',
        'calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\s*\.latest\?',
        'humidity_ratio_mixed_air_assignment_latest_witness',
        'assigned_supply_humidity_ratio\?',
        'predecessor\.supply_temperature_c\?',
        'predecessor\.resulting_supply_enthalpy_j_per_kg\?'
    )) {
    Assert-Contains -Path $cp354Prefix -Pattern $pattern -Description "CP354 recursive owner '$pattern'"
}
Assert-NotContains -Path $cp354Prefix -Pattern 'calc_cooling_positive_supply_enthalpy_assignment\s*\.latest|constant_sensible_heat_ratio_sensible_output_assignment\s*\.latest|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply' -Description "CP354 duplicate/numerical owner substitution"
Assert-NotContains -Path $cp354Release -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|supply_humidity_ratio_before_overdrying_limit\s*:|supply_temperature_c\s*:|supply_enthalpy_j_per_kg\s*:' -Description "CP354 public scalar/numerical substitution"

# Binding, coupled runtime, pipeline, serialization, and strict numerical nonfeed.
Assert-Contains -Path $cp354CalcRoot -Pattern ('mod ' + [regex]::Escape($cp354Stem) + ';') -Description "CP354 calc module"
Assert-Contains -Path $cp354BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp354Stem)) -Description "CP354 binding adapter"
Assert-NotContains -Path $cp354BindingAdapter -Pattern 'supply_humidity_ratio_before_overdrying_limit\s*:|supply_temperature_c\s*:|supply_enthalpy_j_per_kg\s*:|DirectZonePurchasedAirCouplingInput|latest_numerical' -Description "CP354 binding scalar/DTO firewall"
Assert-Contains -Path $cp354ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp354Stem) + ':') -Description "CP354 scheduled output"
Assert-Contains -Path $cp354BindingTestsRoot -Pattern ([regex]::Escape("${cp354Stem}_tests.rs")) -Description "CP354 binding tests"
Assert-Contains -Path $cp354BindingTests -Pattern 'scheduled_binding_places_cp354_after_cp353_as_a_complete_null_none_skip' -Description "CP354 binding regression"
Assert-Contains -Path $cp354InitState -Pattern $cp354Stem -Description "CP354 init state"
Assert-Contains -Path $cp354InitUnit -Pattern $cp354Stem -Description "CP354 unit state"
Assert-Contains -Path $cp354InitWitnessRoot -Pattern $cp354Stem -Description "CP354 witness module"
Assert-Contains -Path $cp354CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp354Stem) + '_validation;') -Description "CP354 coupled validator"
Assert-Contains -Path $cp354Coupled -Pattern ('calculation_' + [regex]::Escape($cp353Stem)) -Description "coupled CP353 predecessor"
Assert-Contains -Path $cp354Coupled -Pattern '(?s)executed\s*\.checked_mul\(.*?OVERDRYING_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked 6Q"
Assert-NotContains -Path $cp354Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled DTO firewall"
Assert-Contains -Path $cp354FixtureRoot -Pattern $cp354Stem -Description "CP354 fixture registration"
Assert-Contains -Path $cp354CoupledTests -Pattern 'cp354_coupled_direct_none_route_is_complete_skip_and_numerical_humidity_remains_unfed' -Description "CP354 numerical nonfeed test"
Assert-Contains -Path $cp354PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp354PipelineStem) + ';') -Description "CP354 pipeline module"
Assert-Contains -Path $cp354PipelineRoot -Pattern ('"' + $cp354Lifecycle + '":\s*result\s*\.' + $cp354Lifecycle) -Description "CP354 lifecycle JSON"
Assert-Contains -Path $cp354PipelineValidation -Pattern 'enthalpy_overdrying_limit_cp353' -Description "pipeline CP353 predecessor"
Assert-Contains -Path $cp354PipelineValidation -Pattern '(?s)executed\s*\.checked_mul\(.*?OVERDRYING_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 6Q"
Assert-Contains -Path $cp354PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp397_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp354ArbitraryTests -Pattern 'cp353_assertions' -Description "arbitrary cumulative module"
Assert-Contains -Path $cp354ArbitraryAssertions -Pattern 'CP354_KEY' -Description "arbitrary CP354 lifecycle"
foreach ($field in @(
        "supply_humidity_ratio_before_overdrying_limit", "supply_temperature_c",
        "supply_enthalpy_j_per_kg", "psychrometric_supply_humidity_ratio",
        "minimum_supply_humidity_ratio", "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio"
    )) {
    Assert-Contains -Path $cp354SnapshotSerialization -Pattern ('"' + $field + '"') -Description "CP354 JSON '$field'"
    Assert-Contains -Path $cp354SnapshotSerialization -Pattern ('"' + $field + '_ieee_bits"') -Description "CP354 JSON bits '$field'"
}
Assert-Contains -Path $cp354SnapshotSerialization -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "CP354 nonfinite numeric null"
Assert-Contains -Path $cp354SnapshotSerialization -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "CP354 authoritative bits"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches($algorithmText, '(?m)^\s*"CP354 supersedes only CP353[^"\r\n]+",\s*$')
$capabilityAddenda = [regex]::Matches($capabilityText, '(?m)^\s*"CP354 additionally requires[^"\r\n]+",\s*$')
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP354 addenda"
}
foreach ($claim in @($algorithmAddenda) + @($capabilityAddenda)) {
    foreach ($pattern in @(
            $cp354SourceHash, 'physical executable line 2222', 'line 2223', 'line 2224', 'CP355',
            $cp354Sites[0], $cp354Sites[1], $cp354Sites[2], $cp354Sites[3], $cp354Sites[4], $cp354Sites[5],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=6\*Q', 'C0=S', 'Q=H=CSH=0',
            'CP345.*?assigned_supply_humidity_ratio', 'CP353.*?resulting_supply_enthalpy_j_per_kg',
            'CP334', 'CP344', 'counterfactual bridge', 'energyplus_psy_w_fn_tdb_h',
            'raw < 0\.0', '0x3ee4f8b588e368f1',
            'if left < right \{ left \} else \{ right \}', 'f64::min',
            'CP353-to-CP354-to-unchanged-numerical', $cp354Lifecycle,
            'DirectZonePurchasedAirCouplingInput', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP354 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp354Stem/release\.rs::advance_direct_no_oa_calc_$cp354Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp354Stem\.rs::purchased_air_calc_${cp354Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp354Stem\.rs::${cp354TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp354Stem\.rs::${cp354TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($algorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP354 target count failed for '$($target.Pattern)'"
    }
}
$documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP354 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP354 Source-Ordered Cooling Constant-SHR Supply-Humidity-Ratio Overdrying Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP354 Constant-SHR Supply-Humidity-Ratio Overdrying Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP354 Constant-SHR Supply-Humidity-Ratio Overdrying Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP354 Constant-SHR Supply-Humidity-Ratio Overdrying Limit Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP354 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp354SourceHash, '2222', '2223', '2224', 'CP355',
            $cp354Sites[0], $cp354Sites[1], $cp354Sites[2], $cp354Sites[3], $cp354Sites[4], $cp354Sites[5],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH', '6\*Q', 'C0\s*=\s*S',
            'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP345', 'CP353', 'CP334', 'CP344',
            'assigned_supply_humidity_ratio', 'resulting_supply_enthalpy_j_per_kg',
            'energyplus_psy_w_fn_tdb_h', 'raw\s*<\s*0\.0', '0x3ee4f8b588e368f1',
            'f64::min', '(?:left\s+NaN|NaN\s+left)', '(?:right\s+NaN|NaN\s+right)',
            'CalledFrom', 'SuppressWarnings', 'CP353-to-CP354-to-unchanged-numerical',
            $cp354Lifecycle, 'DirectZonePurchasedAirCouplingInput',
            '32\s+algorithms', '293\s+routines', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP354 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP354\b' -Description "CP354 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP354 supersedes only CP353' -Description "generated CP354 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP354 additionally requires' -Description "generated CP354 capability addendum"

# Historical order/firewall audits, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..353 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit' -Description "historical CP354 binding order"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit' -Description "historical CP355 binding order"
}
foreach ($historical in 334..353) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp397_lifecycle_evidence' -Description "historical CP363 firewall"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp353AuditIndex = $mainAuditText.IndexOf("cp353-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp354AuditIndex = $mainAuditText.IndexOf("cp354-cooling-constant-shr-supply-humidity-ratio-overdrying-limit.ps1")
$completionIndex = $mainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp353AuditIndex -lt 0 -or $cp354AuditIndex -le $cp353AuditIndex -or $completionIndex -le $cp354AuditIndex) {
    throw "Master audit must dot-source CP354 after CP353 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 335' -Description "CP354 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP354 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp354-' -Description "CP354 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp354-cooling-constant-shr-supply-humidity-ratio-overdrying-limit\.ps1::dot_sources' -Description "CP354 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 335 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 95 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP354 supply-humidity-ratio overdrying-limit structure audit passed."
