# CP345 maps only PurchasedAirManager.cc physical executable line 2208:
# PurchAir.SupplyHumRat = PurchAir.MixedAirHumRat;
# Physical line 2209 is the first excluded lexical executable.

$cp345Stem = "cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment"
$cp345PipelineStem = "purchased_air_$cp345Stem"
$cp345TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignment"
$cp345Module = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem.rs"
$cp345State = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\state.rs"
$cp345Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\transition.rs"
$cp345Release = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\release.rs"
$cp345PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\release\prefix_validation.rs"
$cp345RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\release\runtime_validation.rs"
$cp345SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\release\snapshot_validation.rs"
$cp345Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\tests\mod.rs"
$cp345PublicReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\tests\public_release.rs"
$cp345ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp345Stem\tests\release_corruption.rs"
$cp345CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp345Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp345Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp345ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp345BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp345Stem.rs"
$cp345BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp345BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp345Stem}_tests.rs"
$cp345InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp345InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp345InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp345InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp345Stem.rs"
$cp345CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp345CoupledRuntimeTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp345CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp345Stem}_validation.rs"
$cp345CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp345CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp345Stem}_fixture.rs"
$cp345PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp345Pipeline = "crates\ep_run\src\pipeline\$cp345PipelineStem.rs"
$cp345PipelineValidation = "crates\ep_run\src\pipeline\$cp345PipelineStem\validation.rs"
$cp345PipelineSerialization = "crates\ep_run\src\pipeline\$cp345PipelineStem\serialization.rs"
$cp345PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\$cp345PipelineStem\serialization\snapshot.rs"
$cp345ArbitraryIdealLoadsTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp345SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp345SourceStatementPattern =
    'PurchAir\.SupplyHumRat\s*=\s*PurchAir\.MixedAirHumRat\s*;'
$cp345FirstExcludedPattern =
    'switch\s*\(\s*PurchAir\.DehumidCtrlType\s*\)\s*\{'
$cp345OrderedSitesPattern = '(?s)' +
    'read-purchased-air-mixed-air-humidity-ratio.*?' +
    'assign-purchased-air-supply-humidity-ratio'
$cp345LifecycleField =
    'purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle'

function Get-Cp345RustBraceBlock {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$AnchorPattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) {
        throw "$Description expected exactly one anchor, found $($anchors.Count)"
    }
    $openingBrace = $Text.IndexOf("{", $anchors[0].Index)
    if ($openingBrace -lt 0) {
        throw "$Description opening brace is missing"
    }
    $depth = 0
    for ($index = $openingBrace; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") {
            $depth += 1
        } elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring(
                    $anchors[0].Index,
                    $index - $anchors[0].Index + 1
                )
            }
        }
    }
    throw "$Description closing brace is missing"
}

function Assert-Cp345PatternsInText {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string[]]$Patterns,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $cursor = 0
    for ($index = 0; $index -lt $Patterns.Count; $index += 1) {
        $match = [regex]::Match($Text.Substring($cursor), $Patterns[$index])
        if (-not $match.Success) {
            throw "$Description pattern $($index + 1) is missing or out of order"
        }
        $cursor += $match.Index + $match.Length
    }
}

function Assert-Cp345ReleaseContract {
    param([Parameter(Mandatory = $true)][string]$Text)

    if ($Text -notmatch ('(?s)pub fn advance_direct_no_oa_calc_' +
            [regex]::Escape($cp345Stem) +
            '\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp344:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,\s*\)')) {
        throw "CP345 public release arguments are not exact"
    }
    $body = Get-Cp345RustBraceBlock `
        -Text $Text `
        -AnchorPattern ('(?m)^\s*pub fn advance_direct_no_oa_calc_' + [regex]::Escape($cp345Stem) + '\s*\(') `
        -Description "CP345 public release body"
    $proofPatterns = @(
        'predecessor_snapshots_match_bit_exact\(',
        'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release\(',
        'owner_lineage_is_exact\(',
        'completed_direct_cooling_mixed_air_call_is_consistent\(',
        'corroboration_lineage_is_exact\(',
        'completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent\(',
        'pending_state_is_consistent\(',
        'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent\(',
        'active_input_from_owner\(',
        'next_transition_fits\('
    )
    $lastProofEnd = -1
    foreach ($pattern in $proofPatterns) {
        $matches = [regex]::Matches($body, $pattern)
        if ($matches.Count -lt 1) {
            throw "CP345 public release proof is missing: $pattern"
        }
        $lastMatch = $matches[$matches.Count - 1]
        $end = $lastMatch.Index + $lastMatch.Length
        if ($end -gt $lastProofEnd) {
            $lastProofEnd = $end
        }
    }
    $firstMutation = [regex]::Match(
        $body,
        '(?:runtime\s*\.\s*units\s*\.\s*get_mut\s*\(|advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state\s*\(|set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness\s*\()'
    )
    if (-not $firstMutation.Success -or $firstMutation.Index -lt $lastProofEnd) {
        throw "CP345 public release may mutate only after all owner, corroboration, predecessor, and overflow proofs"
    }
    Assert-Cp345PatternsInText -Text $body -Patterns @(
        'owner_lineage_is_exact\(',
        'completed_direct_cooling_mixed_air_call_is_consistent\(',
        'corroboration_lineage_is_exact\(',
        'completed_direct_cooling_positive_supply_humidity_ratio_mixed_air_assignment_is_consistent\(',
        'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent\(',
        'active_input_from_owner\(',
        'next_transition_fits\(',
        'runtime\s*\.\s*units\s*\.\s*get_mut\s*\(',
        'advance_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_state\(',
        'set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness\('
    ) -Description "CP345 validate-before-mutation order"
    if ($body -match '(?i)DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|ZoneHeatBalanceState|zone_state|psychrometric_service|cache|diagnostic') {
        throw "CP345 public release admits a forbidden service or numerical DTO"
    }
    if ($body -match '(?s)advance_direct_no_oa_calc_[^(]+\([^)]*(?:mixed_air_humidity_ratio|assigned_supply_humidity_ratio)\s*:') {
        throw "CP345 public release admits a duplicate humidity scalar"
    }
}

function Assert-Cp345RuntimeAlgebraContract {
    param([Parameter(Mandatory = $true)][string]$Text)

    foreach ($pattern in @(
            '(?s)g\.checked_add\(f\).*?checked_add\(l\)\)\s*==\s*Some\(active\)',
            'let capacity_body = f\.checked_add\(l\);',
            '(?s)active\s*==\s*unit\s*\.calc_cooling_supply_mass_flow_positive_guard\s*\.positive_supply_mass_flow_body_entry_count',
            '(?s)active\s*==\s*unit\s*\.calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment\s*\.supply_humidity_ratio_mixed_air_assignment_count',
            '(?s)active\s*==\s*unit\s*\.calc_cooling_positive_supply_enthalpy_assignment\s*\.supply_enthalpy_assignment_count',
            '(?s)active\s*==\s*unit\s*\.calc_cooling_positive_supply_capacity_limit_guard\s*\.capacity_limit_guard_evaluation_count',
            '(?s)g\s*==\s*unit\s*\.calc_cooling_positive_supply_capacity_limit_guard\s*\.active_guard_false_fallthrough_count',
            '(?s)capacity_body\s*==\s*Some\(\s*unit\.calc_cooling_positive_supply_capacity_limit_guard\s*\.capacity_limit_body_entry_count',
            '(?s)route_partition\s*==\s*state\.transition_count.*?provenance_partition\s*==\s*active',
            '(?s)active\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)',
            '(?s)source_site_execution_count\s*==\s*expected_source_sites.*?mixed_air_humidity_ratio_read_count\s*==\s*active.*?supply_humidity_ratio_assignment_count\s*==\s*active',
            '(?s)witnessed_assignment_after_capacity_limit_guard_false_fallthrough_count\s*==\s*state\.assignment_after_capacity_limit_guard_false_fallthrough_count',
            '(?s)witnessed_assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count\s*==\s*state\s*\.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count',
            '(?s)witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count\s*==\s*state\s*\.assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP345 checked lifecycle algebra is missing '$pattern'"
        }
    }
    if ($Text -match '(?s)(?:active|post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count)\s*==\s*l\b') {
        throw "CP345 must not collapse R to L"
    }
}

function Assert-Cp345BindingContract {
    param([Parameter(Mandatory = $true)][string]$Text)

    $body = Get-Cp345RustBraceBlock `
        -Text $Text `
        -AnchorPattern '(?m)^\s*pub fn couple_model_bound_direct_zone_purchased_air\s*\(' `
        -Description "direct-zone purchased-air binding function"
    $cp344Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
    )
    $cp345Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
    )
    $cp346Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;'
    )
    $cp347Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;'
    )
    $cp348Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;'
    )
    $cp349Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;'
    )
    $cp350Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;'
    )
    $cp351Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;'
    )
    $cp352Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;'
    )
    $cp353Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;'
    )
    $cp354Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;'
    )
    $cp355Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;'
    )
    $cp356Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;'
    )
    $cp357Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;'
    )
    $cp358Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;'
    )
    $cp359Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;'
    )
    $cp360Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;'
    )
    $cp361Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;'
    )
    $cp362Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;'
    )
    $cp363Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;'
    )
    $cp364Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;'
    )
    $cp365Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;'
    )
    $cp366Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;'
    )
    $cp367Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;'
    )
    $cp368Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;'
    )
    $cp369Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;'
    )
    $cp370Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;'
    )
    $cp371Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;'
    )
    $cp372Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;'
    )
    $cp373Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;'
    )
    $cp374Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;'
    )
    $cp375Call = [regex]::Match(
        $body,
        '(?s)let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;'
    )
    $numerical = [regex]::Match(
        $body,
        '(?s)let\s+coupling\s*=\s*complete_direct_zone_purchased_air_coupling\s*\(\s*DirectZonePurchasedAirCouplingInput\s*\{'
    )
    if (
        -not $cp344Call.Success -or
        -not $cp345Call.Success -or
        -not $cp346Call.Success -or
        -not $cp347Call.Success -or
        -not $cp348Call.Success -or
        -not $cp349Call.Success -or
        -not $cp350Call.Success -or
        -not $cp351Call.Success -or
        -not $cp352Call.Success -or
        -not $cp353Call.Success -or
        -not $cp354Call.Success -or
        -not $cp355Call.Success -or
        -not $cp356Call.Success -or
        -not $cp357Call.Success -or
        -not $cp358Call.Success -or
        -not $cp359Call.Success -or
        -not $cp360Call.Success -or
        -not $cp361Call.Success -or
        -not $cp362Call.Success -or
        -not $cp363Call.Success -or
        -not $cp364Call.Success -or
        -not $cp365Call.Success -or
        -not $cp366Call.Success -or
        -not $cp367Call.Success -or
        -not $cp368Call.Success -or
        -not $cp369Call.Success -or
        -not $cp370Call.Success -or
        -not $cp371Call.Success -or
        -not $cp372Call.Success -or
        -not $cp373Call.Success -or
        -not $cp374Call.Success -or
        -not $cp375Call.Success -or
        -not $numerical.Success -or
        $cp345Call.Index -lt ($cp344Call.Index + $cp344Call.Length) -or
        $cp346Call.Index -lt ($cp345Call.Index + $cp345Call.Length) -or
        $cp347Call.Index -lt ($cp346Call.Index + $cp346Call.Length) -or
        $cp348Call.Index -lt ($cp347Call.Index + $cp347Call.Length) -or
        $cp349Call.Index -lt ($cp348Call.Index + $cp348Call.Length) -or
        $cp350Call.Index -lt ($cp349Call.Index + $cp349Call.Length) -or
        $cp351Call.Index -lt ($cp350Call.Index + $cp350Call.Length) -or
        $cp352Call.Index -lt ($cp351Call.Index + $cp351Call.Length) -or
        $cp353Call.Index -lt ($cp352Call.Index + $cp352Call.Length) -or
        $cp354Call.Index -lt ($cp353Call.Index + $cp353Call.Length) -or
        $cp355Call.Index -lt ($cp354Call.Index + $cp354Call.Length) -or
        $cp356Call.Index -lt ($cp355Call.Index + $cp355Call.Length) -or
        $cp357Call.Index -lt ($cp356Call.Index + $cp356Call.Length) -or
        $cp358Call.Index -lt ($cp357Call.Index + $cp357Call.Length) -or
        $cp359Call.Index -lt ($cp358Call.Index + $cp358Call.Length) -or
        $cp360Call.Index -lt ($cp359Call.Index + $cp359Call.Length) -or
        $cp361Call.Index -lt ($cp360Call.Index + $cp360Call.Length) -or
        $cp362Call.Index -lt ($cp361Call.Index + $cp361Call.Length) -or
        $cp363Call.Index -lt ($cp362Call.Index + $cp362Call.Length) -or
        $cp364Call.Index -lt ($cp363Call.Index + $cp363Call.Length) -or
        $cp365Call.Index -lt ($cp364Call.Index + $cp364Call.Length) -or
        $cp366Call.Index -lt ($cp365Call.Index + $cp365Call.Length) -or
        $cp367Call.Index -lt ($cp366Call.Index + $cp366Call.Length) -or
        $cp368Call.Index -lt ($cp367Call.Index + $cp367Call.Length) -or
        $cp369Call.Index -lt ($cp368Call.Index + $cp368Call.Length) -or
        $cp370Call.Index -lt ($cp369Call.Index + $cp369Call.Length) -or
        $cp371Call.Index -lt ($cp370Call.Index + $cp370Call.Length) -or
        $cp372Call.Index -lt ($cp371Call.Index + $cp371Call.Length) -or
        $cp373Call.Index -lt ($cp372Call.Index + $cp372Call.Length) -or
        $cp374Call.Index -lt ($cp373Call.Index + $cp373Call.Length) -or
        $cp375Call.Index -lt ($cp374Call.Index + $cp374Call.Length) -or
        $numerical.Index -lt ($cp375Call.Index + $cp375Call.Length)
    ) {
        throw "Binding must execute CP344 through CP375 in source order before unchanged numerical coupling"
    }
    foreach ($interval in @(
            [PSCustomObject]@{
                Start = $cp344Call.Index + $cp344Call.Length
                End = $cp345Call.Index
                Description = "CP344-to-CP345"
            },
            [PSCustomObject]@{
                Start = $cp345Call.Index + $cp345Call.Length
                End = $cp346Call.Index
                Description = "CP345-to-CP346"
            },
            [PSCustomObject]@{
                Start = $cp346Call.Index + $cp346Call.Length
                End = $cp347Call.Index
                Description = "CP346-to-CP347"
            },
            [PSCustomObject]@{
                Start = $cp347Call.Index + $cp347Call.Length
                End = $cp348Call.Index
                Description = "CP347-to-CP348"
            },
            [PSCustomObject]@{
                Start = $cp348Call.Index + $cp348Call.Length
                End = $cp349Call.Index
                Description = "CP348-to-CP349"
            },
            [PSCustomObject]@{
                Start = $cp349Call.Index + $cp349Call.Length
                End = $cp350Call.Index
                Description = "CP349-to-CP350"
            },
            [PSCustomObject]@{
                Start = $cp350Call.Index + $cp350Call.Length
                End = $cp351Call.Index
                Description = "CP350-to-CP351"
            },
            [PSCustomObject]@{
                Start = $cp351Call.Index + $cp351Call.Length
                End = $cp352Call.Index
                Description = "CP351-to-CP352"
            },
            [PSCustomObject]@{
                Start = $cp352Call.Index + $cp352Call.Length
                End = $cp353Call.Index
                Description = "CP352-to-CP353"
            },
            [PSCustomObject]@{
                Start = $cp353Call.Index + $cp353Call.Length
                End = $cp354Call.Index
                Description = "CP353-to-CP354"
            },
            [PSCustomObject]@{
                Start = $cp354Call.Index + $cp354Call.Length
                End = $cp355Call.Index
                Description = "CP354-to-CP355"
            },
            [PSCustomObject]@{
                Start = $cp355Call.Index + $cp355Call.Length
                End = $cp356Call.Index
                Description = "CP355-to-CP356"
            },
            [PSCustomObject]@{
                Start = $cp356Call.Index + $cp356Call.Length
                End = $cp357Call.Index
                Description = "CP356-to-CP357"
            },
            [PSCustomObject]@{
                Start = $cp357Call.Index + $cp357Call.Length
                End = $cp358Call.Index
                Description = "CP357-to-CP358"
            },
            [PSCustomObject]@{
                Start = $cp358Call.Index + $cp358Call.Length
                End = $cp359Call.Index
                Description = "CP358-to-CP359"
            },
            [PSCustomObject]@{
                Start = $cp359Call.Index + $cp359Call.Length
                End = $cp360Call.Index
                Description = "CP359-to-CP360"
            },
            [PSCustomObject]@{
                Start = $cp360Call.Index + $cp360Call.Length
                End = $cp361Call.Index
                Description = "CP360-to-CP361"
            },
            [PSCustomObject]@{
                Start = $cp361Call.Index + $cp361Call.Length
                End = $cp362Call.Index
                Description = "CP361-to-CP362"
            },
            [PSCustomObject]@{
                Start = $cp362Call.Index + $cp362Call.Length
                End = $cp363Call.Index
                Description = "CP362-to-CP363"
            },
            [PSCustomObject]@{
                Start = $cp363Call.Index + $cp363Call.Length
                End = $cp364Call.Index
                Description = "CP363-to-CP364"
            },
            [PSCustomObject]@{
                Start = $cp364Call.Index + $cp364Call.Length
                End = $cp365Call.Index
                Description = "CP364-to-CP365"
            },
            [PSCustomObject]@{
                Start = $cp365Call.Index + $cp365Call.Length
                End = $cp366Call.Index
                Description = "CP365-to-CP366"
            },
            [PSCustomObject]@{
                Start = $cp366Call.Index + $cp366Call.Length
                End = $cp367Call.Index
                Description = "CP366-to-CP367"
            },
            [PSCustomObject]@{
                Start = $cp367Call.Index + $cp367Call.Length
                End = $cp368Call.Index
                Description = "CP367-to-CP368"
            },
            [PSCustomObject]@{
                Start = $cp368Call.Index + $cp368Call.Length
                End = $cp369Call.Index
                Description = "CP368-to-CP369"
            },
            [PSCustomObject]@{
                Start = $cp369Call.Index + $cp369Call.Length
                End = $cp370Call.Index
                Description = "CP369-to-CP370"
            },
            [PSCustomObject]@{
                Start = $cp370Call.Index + $cp370Call.Length
                End = $cp371Call.Index
                Description = "CP370-to-CP371"
            },
            [PSCustomObject]@{
                Start = $cp371Call.Index + $cp371Call.Length
                End = $cp372Call.Index
                Description = "CP371-to-CP372"
            },
            [PSCustomObject]@{
                Start = $cp372Call.Index + $cp372Call.Length
                End = $cp373Call.Index
                Description = "CP372-to-CP373"
            },
            [PSCustomObject]@{
                Start = $cp373Call.Index + $cp373Call.Length
                End = $cp374Call.Index
                Description = "CP373-to-CP374"
            },
            [PSCustomObject]@{
                Start = $cp374Call.Index + $cp374Call.Length; End = $cp375Call.Index
                Description = "CP374-to-CP375"
            },
            [PSCustomObject]@{
                Start = $cp375Call.Index + $cp375Call.Length; End = $numerical.Index
                Description = "CP375-to-numerical"
            }
        )) {
        $code = $body.Substring($interval.Start, $interval.End - $interval.Start)
        $code = [regex]::Replace($code, '(?s)/\*.*?\*/|(?m)//.*$', '')
        if ($code -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
            throw "No intermediary helper call may execute in the $($interval.Description) interval"
        }
    }
    $numericalSuffix = $body.Substring($numerical.Index)
    $dto = Get-Cp345RustBraceBlock `
        -Text $numericalSuffix `
        -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' `
        -Description "CP345 numerical coupling DTO"
    $dtoCode = [regex]::Replace($dto, '(?s)/\*.*?\*/|(?m)//.*$', '')
    if ($dtoCode -match ('(?i)\bcp345\b|' + [regex]::Escape($cp345Stem) + '|' + [regex]::Escape($cp345TypeStem))) {
        throw "CP345 evidence must not feed DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp345PipelineRootContract {
    param([Parameter(Mandatory = $true)][string]$Text)

    $productionBoundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $productionBoundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $productionBoundary.Index)
    $execute = Get-Cp345RustBraceBlock `
        -Text $production `
        -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' `
        -Description "pipeline Rust runtime constructor"
    $noneCount = [regex]::Matches(
        $execute,
        [regex]::Escape($cp345LifecycleField) + '\s*:\s*None'
    ).Count
    $someCount = [regex]::Matches(
        $execute,
        'let\s+' + [regex]::Escape($cp345LifecycleField) + '\s*=\s*Some\s*\('
    ).Count
    $shorthandCount = [regex]::Matches(
        $execute,
        '(?m)^\s*' + [regex]::Escape($cp345LifecycleField) + '\s*,\s*$'
    ).Count
    $constructorCount = [regex]::Matches($execute, 'Ok\s*\(\s*RustRuntimeResult\s*\{').Count
    if (
        $constructorCount -ne 4 -or
        $noneCount -ne 3 -or
        $someCount -ne 1 -or
        $shorthandCount -ne 1
    ) {
        throw "Pipeline must expose CP345 through one direct Some/result and three non-direct None constructors"
    }
    $provenance = Get-Cp345RustBraceBlock `
        -Text $production `
        -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' `
        -Description "pipeline runtime-demand provenance validator"
    $disjunctCount = [regex]::Matches(
        $provenance,
        'result\s*\.\s*' + [regex]::Escape($cp345LifecycleField) + '\s*\.\s*is_some\s*\(\s*\)'
    ).Count
    if (
        $disjunctCount -ne 1 -or
        $provenance -notmatch 'persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime'
    ) {
        throw "Pipeline non-direct rejection OR must include CP345 lifecycle is_some() exactly once"
    }
}

function Assert-Cp345SerializationContract {
    param([Parameter(Mandatory = $true)][string]$Text)

    $testBoundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    $production = if ($testBoundary.Success) {
        $Text.Substring(0, $testBoundary.Index)
    } else {
        $Text
    }
    foreach ($field in @("mixed_air_humidity_ratio", "assigned_supply_humidity_ratio")) {
        $valuePattern = '"' + [regex]::Escape($field) + '"\s*:\s*json_number\(\s*snapshot\.' + [regex]::Escape($field) + '\s*\)'
        $bitsPattern = '"' + [regex]::Escape($field + "_ieee_bits") + '"\s*:\s*ieee_bits\(\s*snapshot\.' + [regex]::Escape($field) + '\s*\)'
        if (
            [regex]::Matches($production, $valuePattern).Count -ne 1 -or
            [regex]::Matches($production, $bitsPattern).Count -ne 1
        ) {
            throw "Production CP345 JSON must map '$field' and its IEEE bits exactly once"
        }
    }
    if (
        $production -notmatch '(?s)fn json_number\(\s*value: Option<f64>\s*\) -> Value\s*\{.*?filter\(\s*\|value\| value\.is_finite\(\)\s*\).*?map_or\(Value::Null' -or
        $production -notmatch '(?s)fn ieee_bits\(\s*value: Option<f64>\s*\) -> Option<String>\s*\{.*?format!\("0x\{:016x\}", value\.to_bits\(\)\)'
    ) {
        throw "Production CP345 JSON helpers must use defensive null projection and authoritative IEEE bits"
    }
}

function Assert-Cp345MutationRejected {
    param(
        [Parameter(Mandatory = $true)][string]$Original,
        [Parameter(Mandatory = $true)][string]$Mutated,
        [Parameter(Mandatory = $true)][scriptblock]$Validator,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ($Original -ceq $Mutated) {
        throw "CP345 audit self-test mutation was not applied: $Description"
    }
    $rejected = $false
    try {
        & $Validator $Mutated
    } catch {
        $rejected = $true
    }
    if (-not $rejected) {
        throw "CP345 audit failed to reject mutation: $Description"
    }
}

foreach ($cp345RequiredFile in @(
        $cp345Module,
        $cp345State,
        $cp345Transition,
        $cp345Release,
        $cp345PrefixValidation,
        $cp345RuntimeValidation,
        $cp345SnapshotValidation,
        $cp345Tests,
        $cp345PublicReleaseTests,
        $cp345ReleaseCorruptionTests,
        $cp345BindingAdapter,
        $cp345BindingTests,
        $cp345InitWitness,
        $cp345CoupledValidation,
        $cp345CoupledFixture,
        $cp345Pipeline,
        $cp345PipelineValidation,
        $cp345PipelineSerialization,
        $cp345PipelineSnapshotSerialization,
        $cp345ArbitraryIdealLoadsTests,
        $cp345CoupledRuntimeTests
    )) {
    Assert-FileExists -Path $cp345RequiredFile -Description "CP345 post-capacity humidity assignment structure"
}
Assert-LineLimit -Path $cp345Release -Limit 900 -Description "CP345 release module"
Assert-LineLimit -Path $cp345RuntimeValidation -Limit 700 -Description "CP345 runtime validation module"
Assert-LineLimit -Path $cp345CoupledValidation -Limit 800 -Description "CP345 coupled validation module"
Assert-LineLimit -Path $cp345Pipeline -Limit 700 -Description "CP345 pipeline module"

# Exact source boundary, public surface, snapshot, and collapsed retained route.
Assert-Contains -Path $cp345Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2208' -Description "CP345 exact physical source boundary"
Assert-Contains -Path $cp345Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2209' -Description "CP345 first excluded physical executable"
Assert-Contains -Path $cp345Module -Pattern 'Exact two textual source sites represented by CP345' -Description "CP345 exact lexical-site count"
Assert-ExactStringArray -Path $cp345Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_HUMIDITY_RATIO_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-purchased-air-mixed-air-humidity-ratio",
    "assign-purchased-air-supply-humidity-ratio"
) -Description "CP345 exact two-site source order"
Assert-Contains -Path $cp345Module -Pattern ('pub struct ' + $cp345TypeStem + 'Snapshot') -Description "CP345 public snapshot"
Assert-Contains -Path $cp345State -Pattern ('pub struct ' + $cp345TypeStem + 'RuntimeState') -Description "CP345 persistent state"
Assert-Contains -Path $cp345Module -Pattern ('pub struct ' + $cp345TypeStem + 'LifecycleSummary') -Description "CP345 lifecycle summary"
Assert-Contains -Path $cp345Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary\s*\(' -Description "CP345 lifecycle accessor"
Assert-Contains -Path $cp345CalcRoot -Pattern 'mod cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;' -Description "CP345 Calc module declaration"
Assert-Contains -Path $cp345CalcRoot -Pattern 'pub use (?:cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::\*;|\{[^}]*cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::\*)' -Description "CP345 Calc public surface"
foreach ($field in @(
        "capacity_limit_guard_false_fallthrough_skipped",
        "capacity_limit_sensible_output_guard_false_fallthrough",
        "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed",
        "post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed",
        "mixed_air_humidity_ratio_read",
        "mixed_air_humidity_ratio",
        "supply_humidity_ratio_assignment_performed",
        "assigned_supply_humidity_ratio"
    )) {
    Assert-Contains -Path $cp345Module -Pattern ('pub ' + $field + ':') -Description "CP345 snapshot field '$field'"
}
$cp345StateText = Read-RepoText -Path $cp345State
$cp345RouteBlock = Get-Cp345RustBraceBlock `
    -Text $cp345StateText `
    -AnchorPattern ('(?m)^\s*pub\(in crate::ideal_loads::calc\) enum ' + [regex]::Escape($cp345TypeStem) + 'RetainedRoute\b') `
    -Description "CP345 retained route enum"
[string[]]$cp345Routes = @(
    [regex]::Matches($cp345RouteBlock, '(?m)^\s*([A-Z][A-Za-z0-9_]*)\s*,\s*$') |
        ForEach-Object { $_.Groups[1].Value }
)
[string[]]$cp345ExpectedRoutes = @(
    "UnitOff",
    "NonCooling",
    "PositiveGuardFalseFallthrough",
    "SupplyHumidityRatioMixedAirAssigned"
)
if ($cp345Routes.Count -ne $cp345ExpectedRoutes.Count) {
    throw "CP345 retained route enum must contain exactly four collapsed routes"
}
for ($index = 0; $index -lt $cp345ExpectedRoutes.Count; $index += 1) {
    if ($cp345Routes[$index] -cne $cp345ExpectedRoutes[$index]) {
        throw "CP345 retained route $($index + 1) expected '$($cp345ExpectedRoutes[$index])', found '$($cp345Routes[$index])'"
    }
}
foreach ($counter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count",
        "assignment_after_capacity_limit_guard_false_fallthrough_count",
        "assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count",
        "assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count",
        "source_site_execution_count",
        "mixed_air_humidity_ratio_read_count",
        "supply_humidity_ratio_assignment_count"
    )) {
    Assert-Contains -Path $cp345State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP345 public state counter '$counter'"
}
foreach ($counter in @(
        "witnessed_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count",
        "witnessed_assignment_after_capacity_limit_guard_false_fallthrough_count",
        "witnessed_assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count",
        "witnessed_assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count"
    )) {
    Assert-Contains -Path $cp345State -Pattern ('pub\(super\) ' + $counter + ':\s*usize') -Description "CP345 private witnessed counter '$counter'"
    Assert-NotContains -Path $cp345State -Pattern ('(?m)^\s*pub ' + $counter + ':') -Description "CP345 witnessed counter '$counter' must not be public"
}

# Pure transition is a raw two-site copy with G/F/L provenance and no new
# numerical policy. Direct release recursively inherits CP329's finite gate.
Assert-PatternsInOrder -Path $cp345Transition -Patterns @(
    'let assignment_after_capacity_guard',
    'let assignment_after_sensible_guard',
    'let assignment_after_temperature_limit',
    'let assignment_executed =',
    'let mixed_air_humidity_ratio = active_input\.map',
    'let assigned_supply_humidity_ratio = mixed_air_humidity_ratio;',
    'state\.transition_count \+= 1;',
    'state\.source_site_execution_count \+=',
    'state\.mixed_air_humidity_ratio_read_count \+= 1;',
    'state\.supply_humidity_ratio_assignment_count \+= 1;'
) -Description "CP345 G/F/L join and bit-copy transition order"
Assert-NotContains -Path $cp345Transition -Pattern 'Psy|psychrometric|f64::min|f64::max|\.min\(|\.max\(|clamp\(|total_cmp|partial_cmp|is_finite|is_nan|normalize' -Description "CP345 pure transition adds no numerical gate or coercion"
Assert-Contains -Path $cp345SnapshotValidation -Pattern '(?s)fn active_values_are_exact\(.*?mixed\.to_bits\(\) == assigned\.to_bits\(\)' -Description "CP345 active bit-copy validator"
Assert-Contains -Path $cp345SnapshotValidation -Pattern '(?s)fn inactive_values_are_exact\(.*?!snapshot\.mixed_air_humidity_ratio_read.*?mixed_air_humidity_ratio\.is_none\(\).*?!snapshot\.supply_humidity_ratio_assignment_performed.*?assigned_supply_humidity_ratio\.is_none\(\)' -Description "CP345 skipped null firewall"
Assert-NotContains -Path $cp345SnapshotValidation -Pattern 'mixed\.is_finite\(\)|assigned\.is_finite\(\)|mixed\s*>=\s*0\.0|assigned\s*>=\s*0\.0' -Description "CP345 local snapshot predicate adds no finite gate"
foreach ($test in @(
        "source_boundary_and_exact_two_cp345_site_labels_are_stable",
        "inherited_u_n_p_routes_execute_no_cp345_source_site",
        "g_f_l_routes_collapse_to_one_assignment_route_but_retain_provenance_counts",
        "pure_transition_copies_every_binary64_payload_without_a_new_numeric_gate",
        "accumulated_state_obeys_t_partition_r_join_and_two_r_sites"
    )) {
    Assert-Contains -Path $cp345Tests -Pattern $test -Description "CP345 pure regression '$test'"
}
Assert-Contains -Path $cp345ReleaseCorruptionTests -Pattern 'nonfinite_owner_and_matching_corroboration_cannot_bypass_recursive_release' -Description "CP345 inherited finite-owner firewall regression"

$cp345RuntimeText = Read-RepoText -Path $cp345RuntimeValidation
Assert-Cp345RuntimeAlgebraContract -Text $cp345RuntimeText
$cp345ReleaseText = Read-RepoText -Path $cp345Release
Assert-Cp345ReleaseContract -Text $cp345ReleaseText
Assert-Contains -Path $cp345PrefixValidation -Pattern '(?s)pub\(super\) fn owner_lineage_is_exact\(.*?owner\.mixed_air_humidity_ratio_assigned.*?owner\.mixed_air_humidity_ratio\.is_some\(\).*?cooling_mixed_air_call_snapshot_is_exact_direct_release.*?cooling_mixed_air_call_snapshots_match_bit_exact' -Description "CP345 CP329 latest/private sole owner"
Assert-Contains -Path $cp345PrefixValidation -Pattern '(?s)pub\(super\) fn corroboration_lineage_is_exact\(.*?let Some\(owner_value\) = owner\.mixed_air_humidity_ratio.*?corroboration\.assigned_supply_humidity_ratio.*?Some\(owner_value\).*?humidity_assignment_snapshots_match_bit_exact' -Description "CP345 CP335 bit-exact corroboration"
Assert-Contains -Path $cp345PrefixValidation -Pattern '(?s)pub\(super\) fn active_input_from_owner\(.*?mixed_air_humidity_ratio: owner\?\.mixed_air_humidity_ratio\?' -Description "CP345 active input comes only from CP329 owner"
Assert-NotContains -Path $cp345PrefixValidation -Pattern '(?s)active_input_from_owner\(.*?corroboration' -Description "CP345 corroboration never substitutes for owner"
foreach ($test in @(
        "public_g_f_l_routes_copy_only_same_call_cp329_owner_bits",
        "public_u_n_p_routes_read_no_humidity_operand",
        "lifecycle_summary_reports_completed_cp345_and_duplicate_release_is_transactional"
    )) {
    Assert-Contains -Path $cp345PublicReleaseTests -Pattern $test -Description "CP345 public release regression '$test'"
}
foreach ($test in @(
        "supplied_cp344_numeric_bit_drift_is_rejected_transactionally",
        "retained_cp344_latest_drift_is_rejected_transactionally",
        "private_cp344_witness_drift_is_rejected_transactionally",
        "malformed_cp344_double_active_route_is_rejected_even_when_all_copies_match",
        "cp329_public_owner_corruption_is_rejected_transactionally",
        "cp329_private_owner_corruption_is_rejected_transactionally",
        "cp335_public_corroboration_corruption_is_rejected_transactionally",
        "cp335_private_corroboration_corruption_is_rejected_transactionally",
        "nonfinite_owner_and_matching_corroboration_cannot_bypass_recursive_release",
        "every_active_counter_increment_is_preflighted",
        "malformed_retained_counter_state_rejects_without_mutation",
        "source_site_multiplication_overflow_state_is_rejected_transactionally"
    )) {
    Assert-Contains -Path $cp345ReleaseCorruptionTests -Pattern $test -Description "CP345 corruption regression '$test'"
}

# Runtime-root witness stays private and system-rooted.
Assert-Contains -Path $cp345InitState -Pattern ('(?s)' + $cp345Stem + '_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*' + $cp345TypeStem + 'Snapshot') -Description "runtime-root private CP345 witness map"
Assert-NotContains -Path $cp345InitState -Pattern ('(?m)^\s*pub(?:\([^)]*\))?\s+' + $cp345Stem + '_latest_witnesses:') -Description "CP345 witness map remains private"
Assert-Contains -Path $cp345InitWitnessRoot -Pattern ('mod ' + $cp345Stem + ';') -Description "CP345 witness module"
Assert-Contains -Path $cp345InitWitness -Pattern ($cp345Stem + '_latest_witness\s*\(') -Description "CP345 private witness getter"
Assert-Contains -Path $cp345InitWitness -Pattern ('set_' + $cp345Stem + '_latest_witness\s*\(') -Description "CP345 private witness setter"
Assert-Contains -Path $cp345InitState -Pattern ('pub calc_' + $cp345Stem + ':') -Description "per-unit CP345 state"
Assert-Contains -Path $cp345InitUnit -Pattern ('(?s)calc_' + $cp345Stem + ':\s*' + $cp345TypeStem + 'RuntimeState::new\(\s*system') -Description "per-unit CP345 initialization"

# Binding is exactly CP344 -> CP345 -> unchanged numerical DTO.
$cp345BindingText = Read-RepoText -Path $cp345Binding
Assert-Cp345BindingContract -Text $cp345BindingText
Assert-Contains -Path $cp345Binding -Pattern '(?s)advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit,\s*\)\?;' -Description "binding exact CP344-to-CP345 call"
Assert-Contains -Path $cp345BindingAdapter -Pattern '(?s)fn advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,\s*\)' -Description "CP345 binding adapter arguments"
Assert-NotContains -Path $cp345BindingAdapter -Pattern 'mixed_air_humidity_ratio\s*:|assigned_supply_humidity_ratio\s*:|latest_numerical|complete_direct_zone_purchased_air_coupling|DirectZonePurchasedAirCouplingInput' -Description "CP345 binding adapter operand/numerical firewall"
Assert-Contains -Path $cp345ScheduledOutput -Pattern ('pub calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment:\s*' + $cp345TypeStem + 'Snapshot') -Description "scheduled output exposes CP345 after numerical DTO construction"
Assert-Contains -Path $cp345BindingTestsRoot -Pattern 'mod cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_tests;' -Description "CP345 binding regressions wired"
foreach ($test in @(
        "scheduled_binding_executes_cp345_after_every_cp344_active_route",
        "scheduled_binding_skips_cp345_only_on_u_n_and_p_routes"
    )) {
    Assert-Contains -Path $cp345BindingTests -Pattern $test -Description "CP345 binding regression '$test'"
}

# Coupled lifecycle owns CP344 order, CP329 source, CP335 corroboration, and the
# R=G+F+L/2R identities without introducing numerical coupling.
Assert-Contains -Path $cp345CoupledRuntime -Pattern ('pub calc_' + $cp345Stem + '_lifecycle:\s*' + $cp345TypeStem + 'LifecycleSummary') -Description "coupled runtime CP345 lifecycle"
Assert-Contains -Path $cp345CoupledValidation -Pattern 'pub\(super\) fn snapshot_matches_release\s*\(' -Description "coupled CP345 snapshot matcher"
Assert-Contains -Path $cp345CoupledValidation -Pattern 'pub\(super\) fn validate_lifecycle\s*\(' -Description "coupled CP345 lifecycle validator"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)predecessor_lifecycle:.*SupplyTemperatureMixedAirLimitLifecycleSummary.*?mixed_air_lifecycle:.*MixedAirCallLifecycleSummary.*?corroborating_lifecycle:.*HumidityRatioMixedAirAssignmentLifecycleSummary' -Description "coupled CP344/CP329/CP335 lineage arguments"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)positive_guard_lifecycle:.*SupplyMassFlowPositiveGuardLifecycleSummary.*?enthalpy_lifecycle:.*PositiveSupplyEnthalpyAssignmentLifecycleSummary.*?capacity_limit_guard_lifecycle:.*PositiveSupplyCapacityLimitGuardLifecycleSummary' -Description "coupled CP330/CP336/CP337 parity arguments"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)"positive_guard_transition_count",\s*positive_guard\.transition_count,\s*state\.transition_count.*?"enthalpy_transition_count",\s*enthalpy\.transition_count,\s*state\.transition_count.*?"capacity_limit_guard_transition_count",\s*capacity_limit_guard\.transition_count,\s*state\.transition_count' -Description "coupled CP330/CP336/CP337 transition parity"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)assignment_after_capacity_limit_guard_false_fallthrough_count.*?assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count.*?assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count.*?assignment_route_partition.*?executions' -Description "coupled CP345 R=G+F+L algebra"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)corroborating_supply_humidity_ratio_mixed_air_assignment_count.*?corroborating\.supply_humidity_ratio_mixed_air_assignment_count' -Description "coupled CP345 R equals CP335 assignment count"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)"positive_guard_positive_supply_mass_flow_body_entry_count",\s*positive_guard\.positive_supply_mass_flow_body_entry_count,\s*executions' -Description "coupled CP345 R equals CP330 positive-body count"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)"enthalpy_supply_enthalpy_assignment_count",\s*enthalpy\.supply_enthalpy_assignment_count,\s*executions' -Description "coupled CP345 R equals CP336 assignment count"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)"capacity_limit_guard_evaluation_count",\s*capacity_limit_guard\.capacity_limit_guard_evaluation_count,\s*executions' -Description "coupled CP345 R equals CP337 evaluation count"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)"capacity_limit_guard_active_guard_false_fallthrough_count",\s*capacity_limit_guard\.active_guard_false_fallthrough_count,\s*state\.assignment_after_capacity_limit_guard_false_fallthrough_count' -Description "coupled CP345 G equals CP337 guard-false count"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)let capacity_body_routes = checked_add\(\s*state\.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,\s*state\s*\.assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count.*?"capacity_limit_guard_body_entry_count",\s*capacity_limit_guard\.capacity_limit_body_entry_count,\s*capacity_body_routes' -Description "coupled CP345 F+L equals CP337 capacity-body count"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)source_site_execution_count.*?mixed_air_humidity_ratio_read_count.*?supply_humidity_ratio_assignment_count' -Description "coupled CP345 2R/read/write parity"
Assert-Contains -Path $cp345CoupledValidation -Pattern '(?s)let Some\(source\) = mixed_air\.mixed_air_humidity_ratio.*?corroborating\.assigned_supply_humidity_ratio.*?snapshot\.mixed_air_humidity_ratio.*?snapshot\.assigned_supply_humidity_ratio' -Description "coupled CP329 owner and CP335 corroboration"
Assert-NotContains -Path $cp345CoupledValidation -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled CP345 numerical firewall"
Assert-Contains -Path $cp345CoupledFixtureRoot -Pattern ('mod ' + $cp345Stem + '_fixture;') -Description "CP345 coupled fixture wiring"
Assert-Contains -Path $cp345CoupledFixture -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot\s*\(' -Description "CP345 coupled fixture"
Assert-Contains -Path $cp345CoupledRuntimeTests -Pattern 'cp347_direct_coupled_runtime_completes_none_case_after_g_f_l_and_skips_unit_off' -Description "cumulative CP345-CP347 direct coupled G/F/L and U regression"
Assert-Contains -Path $cp345CoupledRuntimeTests -Pattern 'cp347_direct_coupled_runtime_covers_non_cooling_and_positive_guard_false_skips' -Description "CP345-CP347 direct coupled N/P skip regression"
$cp345CoupledRuntimeTestsText = Read-RepoText -Path $cp345CoupledRuntimeTests
$cp345NonCoolingPositiveSkipTest = Get-Cp345RustBraceBlock `
    -Text $cp345CoupledRuntimeTestsText `
    -AnchorPattern '(?m)^\s*fn cp347_direct_coupled_runtime_covers_non_cooling_and_positive_guard_false_skips\s*\(' `
    -Description "CP345 coupled N/P skip regression body"
foreach ($pattern in @(
        '(?s)true,\s*false\),.*?false,\s*true\),',
        'non_cooling_skip_count',
        'positive_guard_false_fallthrough_skip_count',
        '(?s)post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count,\s*0',
        '(?s)source_site_execution_count,\s*0',
        '!latest\.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed',
        '!latest\.mixed_air_humidity_ratio_read',
        '!latest\.supply_humidity_ratio_assignment_performed',
        'latest\.mixed_air_humidity_ratio\.is_none\(\)',
        'latest\.assigned_supply_humidity_ratio\.is_none\(\)'
    )) {
    if ($cp345NonCoolingPositiveSkipTest -notmatch $pattern) {
        throw "CP345 coupled N/P skip regression is missing '$pattern'"
    }
}

# Pipeline serialization is production-scoped, bit-authoritative, direct-only,
# and rejected on every non-direct runtime constructor.
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)pub\(super\) struct DirectLifecyclePredecessors<.*?\{.*?capacity_limit_temperature_cp344:.*SupplyTemperatureMixedAirLimitLifecycleSummary.*?mixed_air_cp329:.*MixedAirCallLifecycleSummary.*?corroborating_cp335:.*HumidityRatioMixedAirAssignmentLifecycleSummary' -Description "pipeline CP344/CP329/CP335 exact lineage bundle"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)pub\(super\) fn validate_direct_lifecycle\(\s*lifecycle:.*?predecessors:\s*DirectLifecyclePredecessors<.*?>,\s*init_lifecycle:.*?coupling_call_count:' -Description "pipeline CP345 exact validation arguments"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)positive_guard_cp330:.*SupplyMassFlowPositiveGuardLifecycleSummary.*?enthalpy_cp336:.*PositiveSupplyEnthalpyAssignmentLifecycleSummary.*?capacity_limit_guard_cp337:.*PositiveSupplyCapacityLimitGuardLifecycleSummary' -Description "pipeline CP330/CP336/CP337 parity arguments"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)"positive_guard_transition_count",\s*positive_guard_state\.transition_count,\s*state\.transition_count.*?"enthalpy_transition_count",\s*enthalpy_state\.transition_count,\s*state\.transition_count.*?"capacity_limit_guard_transition_count",\s*capacity_limit_guard_state\.transition_count,\s*state\.transition_count' -Description "pipeline CP330/CP336/CP337 transition parity"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)assignment_route_partition.*?post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count' -Description "pipeline CP345 R=G+F+L"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)corroborating_supply_humidity_ratio_mixed_air_assignment_count.*?corroborating_state\.supply_humidity_ratio_mixed_air_assignment_count' -Description "pipeline CP345 R equals CP335 assignment count"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)"positive_guard_positive_supply_mass_flow_body_entry_count",\s*positive_guard_state\.positive_supply_mass_flow_body_entry_count,\s*state\.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count' -Description "pipeline CP345 R equals CP330 positive-body count"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)"enthalpy_supply_enthalpy_assignment_count",\s*enthalpy_state\.supply_enthalpy_assignment_count,\s*state\.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count' -Description "pipeline CP345 R equals CP336 assignment count"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)"capacity_limit_guard_evaluation_count",\s*capacity_limit_guard_state\.capacity_limit_guard_evaluation_count,\s*state\.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_count' -Description "pipeline CP345 R equals CP337 evaluation count"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)"capacity_limit_guard_active_guard_false_fallthrough_count",\s*capacity_limit_guard_state\.active_guard_false_fallthrough_count,\s*state\.assignment_after_capacity_limit_guard_false_fallthrough_count' -Description "pipeline CP345 G equals CP337 guard-false count"
Assert-Contains -Path $cp345Pipeline -Pattern '(?s)let capacity_body_routes = checked_add\(\s*state\.assignment_after_capacity_limit_sensible_output_guard_false_fallthrough_count,\s*state\s*\.assignment_after_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count.*?"capacity_limit_guard_body_entry_count",\s*capacity_limit_guard_state\.capacity_limit_body_entry_count,\s*capacity_body_routes' -Description "pipeline CP345 F+L equals CP337 capacity-body count"
Assert-Contains -Path $cp345PipelineValidation -Pattern '(?s)let Some\(source\) = mixed_air\.mixed_air_humidity_ratio.*?source\.is_finite\(\).*?source >= 0\.0.*?corroborating\.assigned_supply_humidity_ratio.*?snapshot\.mixed_air_humidity_ratio.*?snapshot\.assigned_supply_humidity_ratio' -Description "pipeline recursively finite CP329 owner and CP335 corroboration"
Assert-Contains -Path $cp345PipelineSerialization -Pattern 'pub\(in crate::pipeline\) fn lifecycle_json\s*\(' -Description "pipeline CP345 lifecycle serializer"
$cp345SerializationText = Read-RepoText -Path $cp345PipelineSnapshotSerialization
Assert-Cp345SerializationContract -Text $cp345SerializationText
$cp345PipelineRootText = Read-RepoText -Path $cp345PipelineRoot
Assert-Cp345PipelineRootContract -Text $cp345PipelineRootText
Assert-Contains -Path $cp345PipelineRoot -Pattern ('"' + $cp345LifecycleField + '":\s*result\s*\.' + $cp345LifecycleField + '\s*\.as_ref\(\)\s*\.map\(') -Description "pipeline CP345 lifecycle JSON field"
Assert-Contains -Path $cp345PipelineRoot -Pattern 'purchased_air_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::validate_direct_lifecycle\s*\(' -Description "pipeline CP345 direct validator wiring"
Assert-Contains -Path $cp345PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp375_lifecycle_evidence' -Description "pipeline cumulative non-direct CP362 firewall regression"
$cp345ArbitraryIdealLoadsText = Read-RepoText -Path $cp345ArbitraryIdealLoadsTests
$cp345ArbitraryDirectJsonTest = Get-Cp345RustBraceBlock `
    -Text $cp345ArbitraryIdealLoadsText `
    -AnchorPattern '(?m)^\s*fn ideal_loads_no_oa_branch_runs_declared_compatibility_runtime\s*\(' `
    -Description "arbitrary direct IdealLoads JSON regression"
# The arbitrary direct JSON fixture must take the active CP344/CP345 G route,
# avoid every inherited skip route, and decode all CP329/CP335/CP345 IEEE-bit
# provenance fields as non-null strings before comparing them.
foreach ($pattern in @(
        '(?s)let cp344 = &summary\["rust_runtime"\]\["purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle"\].*?let cp345 = &summary\["rust_runtime"\]\["purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle"\]',
        '(?s)"capacity_limit_guard_false_fallthrough_skipped",\s*"capacity_limit_sensible_output_guard_false_fallthrough",\s*"capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed",\s*\].*?cp345\["latest"\]\[field\], cp344\["latest"\]\[field\]',
        '(?s)let cp329 =\s*&summary\["rust_runtime"\]\["purchased_air_calc_cooling_mixed_air_call_lifecycle"\]\["latest"\]',
        '(?s)let cp335 = &summary\["rust_runtime"\]\["purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle"\]\s*\["latest"\]',
        '(?s)let cp345_mixed_air_bits = cp345\["latest"\]\["mixed_air_humidity_ratio_ieee_bits"\]\s*\.as_str\(\)\s*\.expect\("active CP345 mixed-air humidity-ratio bits"\)',
        '(?s)let cp329_mixed_air_bits = cp329\["mixed_air_humidity_ratio_ieee_bits"\]\s*\.as_str\(\)\s*\.expect\("active CP329 mixed-air humidity-ratio bits"\)',
        '(?s)assert_eq!\(\s*cp345_mixed_air_bits,\s*cp329_mixed_air_bits,',
        '(?s)let cp345_assigned_bits = cp345\["latest"\]\["assigned_supply_humidity_ratio_ieee_bits"\]\s*\.as_str\(\)\s*\.expect\("active CP345 assigned humidity-ratio bits"\)',
        '(?s)let cp335_assigned_bits = cp335\["assigned_supply_humidity_ratio_ieee_bits"\]\s*\.as_str\(\)\s*\.expect\("active CP335 assigned humidity-ratio bits"\)',
        '(?s)assert_eq!\(\s*cp345_assigned_bits,\s*cp335_assigned_bits,',
        '(?s)let cp345_assignment_route_count = \[\s*"capacity_limit_guard_false_fallthrough_skipped",\s*"capacity_limit_sensible_output_guard_false_fallthrough",\s*"capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed",\s*\].*?\.filter\(\|field\| cp345\["latest"\]\[field\]\.as_bool\(\) == Some\(true\)\).*?\.count\(\)',
        '(?s)let cp345_skip_route_count = \[\s*"unit_off_skipped",\s*"non_cooling_skipped",\s*"positive_guard_false_fallthrough_skipped",\s*\].*?\.filter\(\|field\| cp345\["latest"\]\[field\]\.as_bool\(\) == Some\(true\)\).*?\.count\(\)',
        '(?s)assert_eq!\(\s*cp345_assignment_route_count,\s*1,\s*"fixture must execute one active CP345 G/F/L assignment route"\s*\)',
        '(?s)assert_eq!\(\s*cp345_skip_route_count,\s*0,\s*"active CP345 fixture must not take an inherited skip route"\s*\)',
        '(?s)assert_eq!\(\s*cp345\["latest"\]\["capacity_limit_guard_false_fallthrough_skipped"\],\s*true,\s*"no-limit fixture must exercise the CP345 G assignment route"\s*\)'
    )) {
    if ($cp345ArbitraryDirectJsonTest -notmatch $pattern) {
        throw "Arbitrary direct CP345 active-route/non-null JSON cross-lifecycle regression is missing '$pattern'"
    }
}

# Specs have exactly two algorithm and two capability addenda with the 2+2+1+1
# target distribution and no claim promotion.
$cp345AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp345AlgorithmAddenda = [regex]::Matches(
    $cp345AlgorithmText,
    '(?m)^\s*"CP345 supersedes only CP344[^"\r\n]+",\s*$'
)
if ($cp345AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP345 claim addenda"
}
foreach ($target in @(
        [PSCustomObject]@{
            Pattern = 'cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment'
            Expected = 2
        },
        [PSCustomObject]@{
            Pattern = 'cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\.rs::purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary'
            Expected = 2
        },
        [PSCustomObject]@{
            Pattern = 'cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState'
            Expected = 1
        },
        [PSCustomObject]@{
            Pattern = 'cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary'
            Expected = 1
        }
    )) {
    $count = [regex]::Matches($cp345AlgorithmText, $target.Pattern).Count
    if ($count -ne $target.Expected) {
        throw "CP345 target '$($target.Pattern)' expected $($target.Expected), found $count"
    }
}
$cp345CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp345CapabilityAddenda = [regex]::Matches(
    $cp345CapabilityText,
    '(?m)^\s*"CP345 additionally requires[^"\r\n]+",\s*$'
)
if ($cp345CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP345 claim addenda"
}
foreach ($claim in @($cp345AlgorithmAddenda) + @($cp345CapabilityAddenda)) {
    foreach ($pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            $cp345SourceHash,
            $cp345SourceStatementPattern,
            $cp345OrderedSitesPattern,
            'SupplyHumidityRatioMixedAirAssigned',
            'public provenance counters|publishing separate.*?provenance counters',
            'private witnessed parity|with witnessed parity',
            'T=U\+N\+P\+R',
            'R=G\+F\+L',
            'A=F\+L',
            'source_site_execution_count=2\*R|source-site executions equal `2\*R`',
            'no (?:invariant )?equates `R` with `L`|no `R=L`',
            'CP329 latest/private `mixed_air_humidity_ratio`',
            'CP335 `assigned_supply_humidity_ratio`',
            'CP344-to-CP345-to-unchanged-numerical',
            $cp345LifecycleField,
            'DirectZonePurchasedAirCouplingInput',
            $cp345FirstExcludedPattern,
            '(?is)pure.*?(?:signed zero|signed-zero).*?NaN.*?infinit',
            '(?is)complete direct release.*?CP329.*?finite.*?>= 0\.0',
            '(?is)no new finite gate|adds no finite gate',
            '(?is)JSON.*?nonfinite.*?null.*?IEEE.*?defensive',
            '32 algorithms and 293 routines',
            'Roadmap (?:promotion|state)'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP345 spec addendum missing '$pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP345 supersedes only CP344' -Description "generated CP345 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP345 additionally requires' -Description "generated CP345 capability index"

# Exactly five hand-authored contract sections carry CP345. The psychrometrics
# map remains unchanged.
$cp345DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP345 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP345 Source-Ordered Cooling Positive-Supply Post-Capacity-Limit Humidity-Ratio Mixed-Air Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP345 Cooling Positive-Supply Post-Capacity-Limit Humidity-Ratio Mixed-Air Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP345 Post-Capacity-Limit Humidity-Ratio Mixed-Air Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP345 Cooling Positive-Supply Post-Capacity-Limit Humidity-Ratio Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($documentation in $cp345DocumentationSections) {
    $documentText = Read-RepoText -Path $documentation.Path
    $matches = [regex]::Matches($documentText, $documentation.Pattern)
    if ($matches.Count -ne 1) {
        throw "CP345 documentation expected one scoped section in $($documentation.Path), found $($matches.Count)"
    }
    $section = $matches[0].Value
    foreach ($pattern in @(
            $cp345SourceHash,
            $cp345SourceStatementPattern,
            $cp345OrderedSitesPattern,
            'SupplyHumidityRatioMixedAirAssigned',
            '(?s)(?:Separate|separate) public.*?G.*?F.*?L.*?private witnessed parity',
            'T\s*=\s*U\+N\+P\+R',
            'R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L',
            '2\*R',
            '(?s)CP329.*?mixed_air_humidity_ratio.*?(?:solely|sole).*?owner',
            '(?s)CP335.*?assigned_supply_humidity_ratio.*?corroborat',
            'CP344-to-CP345-to-unchanged-numerical',
            $cp345LifecycleField,
            'DirectZonePurchasedAirCouplingInput',
            $cp345FirstExcludedPattern,
            '(?is)pure.*?(?:signed zero|signed-zero).*?NaN.*?infinit',
            '(?is)complete direct release.*?CP329.*?finite.*?>= 0\.0',
            '(?is)no (?:new )?finite gate|adds no finite gate',
            '(?is)JSON.*?nonfinite.*?null.*?IEEE.*?defensive',
            '32\s+algorithms',
            '293\s+routines',
            'state[_-]mapped',
            'source[_-]mapped',
            'Roadmap'
        )) {
        if ($section -notmatch $pattern) {
            throw "CP345 documentation in $($documentation.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP345\b' -Description "CP345 does not alter the psychrometrics source map"

# Historical binding audits admit the CP345 call; cumulative non-direct audits
# and coupled regression names reach CP345.
foreach ($audit in @(
        "scripts\quality\ideal-loads-structure-audit\cp326-cooling-supply-mass-flow-limit-body.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp329-cooling-mixed-air-call.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp330-cooling-supply-mass-flow-positive-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp331-cooling-positive-supply-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp332-cooling-positive-supply-temperature-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp333-cooling-positive-supply-temperature-minimum-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit.ps1"
    )) {
    Assert-Contains -Path $audit -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment' -Description "historical binding interval admits CP345"
}
foreach ($audit in @(
        "scripts\quality\ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit.ps1"
    )) {
    Assert-Contains -Path $audit -Pattern 'non_direct_runtime_rejects_cp316_through_cp375_lifecycle_evidence' -Description "historical non-direct firewall reaches CP362"
}
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1" -Pattern 'cp347_direct_coupled_runtime_completes_none_case_after_g_f_l_and_skips_unit_off' -Description "historical coupled audit reaches CP347"

# Root reachability and generated inventory add one internal script.
$cp345MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp344DotSourceIndex = $cp345MainAuditText.IndexOf('ideal-loads-structure-audit\cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit.ps1')
$cp345DotSourceIndex = $cp345MainAuditText.IndexOf('ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1')
$cp345AuditCompletionIndex = $cp345MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp344DotSourceIndex -lt 0 -or
    $cp345DotSourceIndex -le $cp344DotSourceIndex -or
    $cp345AuditCompletionIndex -le $cp345DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP345 after CP344 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 313' -Description "CP345 cumulative inventory total through CP358"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP345 cumulative uncalled inventory"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment\.ps1"' -Description "CP345 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment\.ps1::dot_sources' -Description "CP345 main-audit callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 313 \|' -Description "CP345 generated script total through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP345 generated public script total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 73 \|' -Description "CP345 generated internal script total through CP358"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP345 generated uncalled script total"

# The audit itself proves its scoped negative checks reject representative
# owner, corroboration, algebra, binding, JSON, and direct-only mutations.
Assert-Cp345MutationRejected `
    -Original $cp345ReleaseText `
    -Mutated $cp345ReleaseText.Replace('owner_lineage_is_exact(', 'owner_lineage_was_bypassed(') `
    -Validator { param($text) Assert-Cp345ReleaseContract -Text $text } `
    -Description "public release owner proof removal"
Assert-Cp345MutationRejected `
    -Original $cp345ReleaseText `
    -Mutated $cp345ReleaseText.Replace('corroboration_lineage_is_exact(', 'corroboration_lineage_was_bypassed(') `
    -Validator { param($text) Assert-Cp345ReleaseContract -Text $text } `
    -Description "public release corroboration proof removal"
Assert-Cp345MutationRejected `
    -Original $cp345RuntimeText `
    -Mutated $cp345RuntimeText.Replace('g.checked_add(f)', 'g.checked_add(l)') `
    -Validator { param($text) Assert-Cp345RuntimeAlgebraContract -Text $text } `
    -Description "R=G+F+L algebra corruption"
$cp345GuardFalseParityPattern =
    '(?s)\s*&& g == unit\s*\.calc_cooling_positive_supply_capacity_limit_guard\s*\.active_guard_false_fallthrough_count'
$cp345GuardFalseParityMutation = [regex]::Replace(
    $cp345RuntimeText,
    $cp345GuardFalseParityPattern,
    '',
    1
)
Assert-Cp345MutationRejected `
    -Original $cp345RuntimeText `
    -Mutated $cp345GuardFalseParityMutation `
    -Validator { param($text) Assert-Cp345RuntimeAlgebraContract -Text $text } `
    -Description "G equals CP337 active-guard-false parity removal"
Assert-Cp345MutationRejected `
    -Original $cp345BindingText `
    -Mutated $cp345BindingText.Replace(
        'DirectZonePurchasedAirCouplingInput {',
        "DirectZonePurchasedAirCouplingInput {`n            cp345_evidence: calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,"
    ) `
    -Validator { param($text) Assert-Cp345BindingContract -Text $text } `
    -Description "CP345 evidence injection into numerical DTO"
Assert-Cp345MutationRejected `
    -Original $cp345SerializationText `
    -Mutated $cp345SerializationText.Replace(
        '"mixed_air_humidity_ratio":',
        '"mixed_air_humidity_ratio_mutated":'
    ) `
    -Validator { param($text) Assert-Cp345SerializationContract -Text $text } `
    -Description "production JSON owner field mutation"
$cp345NonDirectSomePattern =
    'result\s*\.\s*' + [regex]::Escape($cp345LifecycleField) + '\s*\.\s*is_some\s*\(\s*\)'
$cp345NonDirectNoneMutation = [regex]::Replace(
    $cp345PipelineRootText,
    $cp345NonDirectSomePattern,
    ('result.' + $cp345LifecycleField + '.is_none()'),
    1
)
Assert-Cp345MutationRejected `
    -Original $cp345PipelineRootText `
    -Mutated $cp345NonDirectNoneMutation `
    -Validator { param($text) Assert-Cp345PipelineRootContract -Text $text } `
    -Description "non-direct is_some firewall mutation"
$cp345ConstructorMutation = [regex]::Replace(
    $cp345PipelineRootText,
    ([regex]::Escape($cp345LifecycleField) + '\s*:\s*None'),
    ($cp345LifecycleField + ': Some(unreachable!())'),
    1
)
Assert-Cp345MutationRejected `
    -Original $cp345PipelineRootText `
    -Mutated $cp345ConstructorMutation `
    -Validator { param($text) Assert-Cp345PipelineRootContract -Text $text } `
    -Description "one non-direct None constructor mutation"
