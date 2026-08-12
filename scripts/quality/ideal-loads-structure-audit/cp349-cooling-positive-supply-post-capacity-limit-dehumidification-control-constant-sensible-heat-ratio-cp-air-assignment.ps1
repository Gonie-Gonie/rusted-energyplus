# CP349 maps line 2216 CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat); line 2217 is excluded.
$cp349Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment"
$cp349PipelineStem = "purchased_air_$cp349Stem"
$cp349TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignment"
$cp349Lifecycle = "purchased_air_calc_${cp349Stem}_lifecycle"
$cp349SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp349Sites = @("read-purchased-air-mixed-air-humidity-ratio-for-constant-sensible-heat-ratio-cp-air", "evaluate-psy-cp-air-fn-w-for-constant-sensible-heat-ratio-cp-air", "assign-local-cp-air-for-constant-sensible-heat-ratio-case")
$cp349Module = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem.rs"
$cp349State = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\state.rs"
$cp349Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\transition.rs"
$cp349Release = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\release.rs"
$cp349Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\release\prefix_validation.rs"
$cp349Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\release\runtime_validation.rs"
$cp349Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\release\snapshot_validation.rs"
$cp349Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\tests\mod.rs"
$cp349PublicTests = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\tests\public_release.rs"
$cp349CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp349Stem\tests\release_corruption.rs"
$cp349CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp349Psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$cp349Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp349Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp349BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp349Stem.rs"
$cp349BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp349Stem}_tests.rs"
$cp349BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp349ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp349InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp349InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp349InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp349InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp349Stem.rs"
$cp349CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp349Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp349Stem}_validation.rs"
$cp349CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp349CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp349CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp349Stem}_fixture.rs"
$cp349PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp349Pipeline = "crates\ep_run\src\pipeline\$cp349PipelineStem.rs"
$cp349PipelineValidation = "crates\ep_run\src\pipeline\$cp349PipelineStem\validation.rs"
$cp349Serialization = "crates\ep_run\src\pipeline\$cp349PipelineStem\serialization.rs"
$cp349SnapshotSerialization = "crates\ep_run\src\pipeline\$cp349PipelineStem\serialization\snapshot.rs"
$cp349ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp349Audit = "scripts\quality\ideal-loads-structure-audit\cp349-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1"
function Get-Cp349RustBraceBlock {
    param([Parameter(Mandatory = $true)][string]$Text, [Parameter(Mandatory = $true)][string]$AnchorPattern, [Parameter(Mandatory = $true)][string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) {
        throw "$Description expected one anchor, found $($anchors.Count)"
    }
    $opening = $Text.IndexOf("{", $anchors[0].Index)
    if ($opening -lt 0) {
        throw "$Description opening brace is missing"
    }
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
    throw "$Description closing brace is missing"
}
function Assert-Cp349ReleaseContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $signature = '(?s)pub fn advance_direct_no_oa_calc_' +
        [regex]::Escape($cp349Stem) +
        '\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp348:\s*PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,\s*\)'
    if ($Text -notmatch $signature) {
        throw "CP349 public release arguments are not exact"
    }
    $body = Get-Cp349RustBraceBlock `
        -Text $Text `
        -AnchorPattern ('(?m)^\s*pub fn advance_direct_no_oa_calc_' + [regex]::Escape($cp349Stem) + '\s*\(') `
        -Description "CP349 release body"
    $proofs = @(
        'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None',
        'predecessor_snapshots_match_exact\(',
        'cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_snapshot_is_exact_direct_release\(',
        'completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_is_consistent\(',
        'pending_state_is_consistent\(',
        'next_transition_fits\('
    )
    $lastProof = -1
    foreach ($pattern in $proofs) {
        $matches = [regex]::Matches($body, $pattern)
        if ($matches.Count -lt 1) {
            throw "CP349 release proof missing '$pattern'"
        }
        $last = $matches[$matches.Count - 1]
        $lastProof = [Math]::Max($lastProof, $last.Index + $last.Length)
    }
    $mutation = $body.IndexOf("runtime.units.get_mut(")
    if ($mutation -lt $lastProof) {
        throw "CP349 release must finish proof and overflow preflight before mutation"
    }
    if ([regex]::Matches($body, '(?m)^\s*None,\s*$').Count -lt 2) {
        throw "CP349 exact direct route must pass no active operand"
    }
    if ($body -match '(?i)energyplus_psy_cp_air_fn_w|humidity_ratio\s*:|DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply') {
        throw "CP349 public wrapper admits private scalar work or numerical DTO state"
    }
}
function Assert-Cp349RuntimeContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    foreach ($pattern in @(
            '(?s)unit_off_skip_count.*?checked_add\(state\.non_cooling_skip_count\).*?positive_guard_false_fallthrough_skip_count.*?dehumidification_control_none_case_completed_skip_count.*?dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count.*?dehumidification_control_humidistat_case_selected_skip_count.*?dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count',
            '(?s)dehumidification_control_none_case_completed_skip_count\s*\.checked_add\(\s*state\s*\.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count.*?dehumidification_control_humidistat_case_selected_skip_count.*?dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count',
            '(?s)assignments\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)',
            'route_partition\s*==\s*state\.transition_count',
            'assignments\s*==\s*expected_constant_sensible',
            'source_site_execution_count\s*==\s*expected_source_sites',
            'mixed_air_humidity_ratio_read_count\s*==\s*assignments',
            'psychrometric_cp_air_evaluation_count\s*==\s*assignments',
            'cp_air_assignment_write_count\s*==\s*assignments'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP349 checked lifecycle algebra missing '$pattern'"
        }
    }
}
function Assert-Cp349TransitionContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    foreach ($pattern in @(
            'Route::DehumidificationControlConstantSensibleHeatRatioCpAirAssigned',
            'mixed_air_humidity_ratio\s*=\s*active_input\?\.mixed_air_humidity_ratio',
            '!mixed_air_humidity_ratio\.is_finite\(\)\s*\|\|\s*mixed_air_humidity_ratio\s*<\s*0\.0',
            'energyplus_psy_cp_air_fn_w\(mixed_air_humidity_ratio\)',
            '!cp_air_j_per_kg_k\.is_finite\(\)',
            'mixed_air_humidity_ratio_read_count\s*\+=\s*1',
            'psychrometric_cp_air_evaluation_count\s*\+=\s*1',
            'cp_air_assignment_write_count\s*\+=\s*1'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP349 transition contract missing '$pattern'"
        }
    }
    if ($Text -match 'energyplus_moist_air_specific_heat|energyplus_psy_cp_air_fn_w_fast|dwSave|cpaSave|-100\.0|static|Mutex|OnceLock|thread_local|CoolSensOutput|SupplyMassFlowRate|MixedAirTemp|SupplyTemp|CoolSHR') {
        throw "CP349 transition admits a cache, noncanonical helper, or line-2217-or-later work"
    }
}
function Assert-Cp349BindingContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $cp348 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =")
    $cp349 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =")
    $cp350 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =")
    $cp351 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =")
    $cp352 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =")
    $cp353 = $Text.IndexOf("let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
    $cp354 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =")
    $cp355 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =")
    $cp356 = $Text.IndexOf("let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit ="); $cp357 = $Text.IndexOf("let calculation_cooling_constant_shr_case_break ="); $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry ="); $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp348 -lt 0 -or $cp349 -le $cp348 -or $cp350 -le $cp349 -or $cp351 -le $cp350 -or $cp352 -le $cp351 -or $cp353 -le $cp352 -or $cp354 -le $cp353 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP348 then CP349 then CP350 then CP351 then CP352 then CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp349RustBraceBlock `
        -Text $Text.Substring($numerical) `
        -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' `
        -Description "CP349 numerical DTO"
    if ($dto -match '(?i)cp349|constant_sensible_heat_ratio_cp_air_assignment|ConstantSensibleHeatRatioCpAirAssignment') {
        throw "CP349 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}
function Assert-Cp349PipelineRootContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp349RustBraceBlock `
        -Text $production `
        -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' `
        -Description "pipeline execute_rust_runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp349Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp349Lifecycle) + '\s*=\s*Some\s*\(').Count
    $shorthand = [regex]::Matches($execute, '(?m)^\s*' + [regex]::Escape($cp349Lifecycle) + '\s*,\s*$').Count
    if ($none -ne 3 -or $some -ne 1 -or $shorthand -ne 1) {
        throw "Pipeline must expose one direct CP349 Some and three non-direct None constructors"
    }
    $provenance = Get-Cp349RustBraceBlock `
        -Text $production `
        -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' `
        -Description "pipeline non-direct firewall"
    $pattern = 'result\s*\.\s*' + [regex]::Escape($cp349Lifecycle) + '\s*\.\s*is_some\s*\(\s*\)'
    if ([regex]::Matches($provenance, $pattern).Count -ne 1) {
        throw "Pipeline non-direct rejection must include CP349 is_some exactly once"
    }
}
function Assert-Cp349SerializationContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    foreach ($field in @(
            "mixed_air_humidity_ratio",
            "mixed_air_humidity_ratio_ieee_bits",
            "psychrometric_cp_air_result_j_per_kg_k",
            "psychrometric_cp_air_result_j_per_kg_k_ieee_bits",
            "cp_air_j_per_kg_k",
            "cp_air_j_per_kg_k_ieee_bits"
        )) {
        if ($Text -notmatch ('"' + $field + '"')) {
            throw "CP349 snapshot JSON missing '$field'"
        }
    }
    if ($Text -notmatch 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)') {
        throw "CP349 JSON must preserve authoritative IEEE bits"
    }
}
function Assert-Cp349MutationRejected {
    param([Parameter(Mandatory = $true)][string]$Original, [Parameter(Mandatory = $true)][string]$Mutated, [Parameter(Mandatory = $true)][scriptblock]$Validator, [Parameter(Mandatory = $true)][string]$Description)
    if ($Original -ceq $Mutated) {
        throw "CP349 self-test mutation was not applied: $Description"
    }
    $rejected = $false
    try {
        & $Validator $Mutated
    } catch {
        $rejected = $true
    }
    if (-not $rejected) {
        throw "CP349 audit failed to reject mutation: $Description"
    }
}
foreach ($required in @(
        $cp349Module, $cp349State, $cp349Transition, $cp349Release,
        $cp349Prefix, $cp349Runtime, $cp349Snapshot, $cp349Tests,
        $cp349PublicTests, $cp349CorruptionTests, $cp349BindingAdapter,
        $cp349BindingTests, $cp349InitWitness, $cp349Coupled,
        $cp349CoupledFixture, $cp349Pipeline, $cp349PipelineValidation,
        $cp349Serialization, $cp349SnapshotSerialization, $cp349Audit
    )) {
    Assert-FileExists -Path $required -Description "CP349 structure"
}
Assert-LineLimit -Path $cp349Transition -Limit 450 -Description "CP349 transition"
Assert-LineLimit -Path $cp349Release -Limit 450 -Description "CP349 release"
Assert-LineLimit -Path $cp349Runtime -Limit 350 -Description "CP349 runtime validation"
Assert-LineLimit -Path $cp349Snapshot -Limit 350 -Description "CP349 snapshot validation"
Assert-LineLimit -Path $cp349Coupled -Limit 450 -Description "CP349 coupled validation"
Assert-LineLimit -Path $cp349PipelineValidation -Limit 500 -Description "CP349 pipeline validation"
Assert-LineLimit -Path $cp349Audit -Limit 500 -Description "CP349 structure audit"
# Exact source boundary, route partition, scalar ownership, and cache exclusion.
Assert-Contains -Path $cp349Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2216' -Description "CP349 exact physical source"
Assert-Contains -Path $cp349Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2217' -Description "CP349 first excluded executable"
Assert-ExactStringArray `
    -Path $cp349Module `
    -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER" `
    -Expected $cp349Sites `
    -Description "CP349 exact three-site order"
Assert-Contains -Path $cp349State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCpAirAssigned,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP349 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count",
        "source_site_execution_count",
        "mixed_air_humidity_ratio_read_count",
        "psychrometric_cp_air_evaluation_count",
        "cp_air_assignment_write_count"
    )) {
    Assert-Contains -Path $cp349State -Pattern ('pub ' + $counter + ': usize') -Description "CP349 counter '$counter'"
}
$cp349TransitionText = Read-RepoText -Path $cp349Transition
$cp349ReleaseText = Read-RepoText -Path $cp349Release
$cp349RuntimeText = Read-RepoText -Path $cp349Runtime
$cp349BindingText = Read-RepoText -Path $cp349Binding
$cp349PipelineRootText = Read-RepoText -Path $cp349PipelineRoot
$cp349SnapshotSerializationText = Read-RepoText -Path $cp349SnapshotSerialization
Assert-Cp349TransitionContract -Text $cp349TransitionText
Assert-Cp349ReleaseContract -Text $cp349ReleaseText
Assert-Cp349RuntimeContract -Text $cp349RuntimeText
Assert-Cp349BindingContract -Text $cp349BindingText
Assert-Cp349PipelineRootContract -Text $cp349PipelineRootText
Assert-Cp349SerializationContract -Text $cp349SnapshotSerializationText
Assert-Contains -Path $cp349Psychrometrics -Pattern 'pub fn energyplus_psy_cp_air_fn_w\s*\(' -Description "canonical stateless CpAir helper"
Assert-Contains -Path $cp349Prefix -Pattern 'owner\.mixed_air_humidity_ratio' -Description "CP329-owned operand"
Assert-Contains -Path $cp349Prefix -Pattern 'cooling_mixed_air_call_snapshots_match_bit_exact\(owner,\s*owner_witness\)' -Description "CP329 latest/private parity"
Assert-Contains -Path $cp349Prefix -Pattern '(?s)operand\.is_finite\(\).*?operand\s*>=\s*0\.0.*?energyplus_psy_cp_air_fn_w\(operand\)\.is_finite\(\)' -Description "private K physical-domain gate"
Assert-Contains -Path $cp349Snapshot -Pattern '(?s)humidity\.is_finite\(\).*?humidity\s*>=\s*0\.0.*?result\.is_finite\(\).*?result\.to_bits\(\)\s*==\s*expected\.to_bits\(\).*?assigned\.to_bits\(\)\s*==\s*result\.to_bits\(\)' -Description "canonical result and bit-exact assignment"
Assert-NotContains -Path $cp349Release -Pattern 'cooling_positive_supply_cp_air_assignment_latest|cooling_positive_supply_capacity_limit_cp_air_assignment_latest' -Description "CP331/CP338 result reuse"
Assert-NotContains -Path $cp349Prefix -Pattern 'dwSave|cpaSave|-100\.0|Mutex|OnceLock|thread_local' -Description "C++ last-call cache lifecycle"

Assert-Contains -Path $cp349Tests -Pattern 'pure_transition_partitions_all_seven_routes_and_only_k_executes_three_sites' -Description "private K-only regression"
Assert-Contains -Path $cp349Tests -Pattern 'pure_transition_rejects_route_input_and_identity_mismatch_before_mutation' -Description "private finite/input rejection"
Assert-Contains -Path $cp349Tests -Pattern 'private_owner_input_requires_exact_same_call_cp329_bits' -Description "CP329 owner regression"
Assert-Contains -Path $cp349PublicTests -Pattern 'public_none_route_complete_skips_all_three_cp349_sites' -Description "direct None complete skip"
Assert-Contains -Path $cp349PublicTests -Pattern 'public_u_n_p_routes_also_skip_every_cp349_value' -Description "direct U/N/P skips"
Assert-Contains -Path $cp349CorruptionTests -Pattern 'supplied_retained_and_private_cp348_drift_are_rejected_transactionally' -Description "CP348 predecessor corruption"
Assert-Contains -Path $cp349CorruptionTests -Pattern 'private_numeric_snapshot_and_witness_matching_is_bit_exact' -Description "private numeric bit identity"
Assert-Contains -Path $cp349CorruptionTests -Pattern 'every_private_k_counter_increment_is_preflighted' -Description "private K overflow preflight"
Assert-Contains -Path $cp349CalcRoot -Pattern ('mod ' + [regex]::Escape($cp349Stem) + ';') -Description "CP349 calc module"
Assert-Contains -Path $cp349BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp349Stem)) -Description "CP349 binding adapter"
Assert-NotContains -Path $cp349BindingAdapter -Pattern 'humidity_ratio\s*:|DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply' -Description "binding scalar/DTO firewall"
Assert-Contains -Path $cp349ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp349Stem) + ':') -Description "CP349 scheduled output"
Assert-Contains -Path $cp349BindingTestsRoot -Pattern ([regex]::Escape("${cp349Stem}_tests.rs")) -Description "CP349 binding tests registration"
Assert-Contains -Path $cp349BindingTests -Pattern 'scheduled_binding_completes_cp349_direct_none_route_without_operand_or_numeric_work' -Description "binding direct skip"
Assert-Contains -Path $cp349InitState -Pattern $cp349Stem -Description "CP349 init state"
Assert-Contains -Path $cp349InitUnit -Pattern $cp349Stem -Description "CP349 unit state"
Assert-Contains -Path $cp349InitWitnessRoot -Pattern $cp349Stem -Description "CP349 witness module"
Assert-Contains -Path $cp349CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp349Stem) + '_validation;') -Description "CP349 coupled validator"
Assert-Contains -Path $cp349Coupled -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry' -Description "coupled CP348 predecessor"
Assert-Contains -Path $cp349Coupled -Pattern '(?s)assigned\s*\.checked_mul\(.*?CP_AIR_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "coupled checked 3*K"
Assert-Contains -Path $cp349Coupled -Pattern 'direct_constant_sensible_heat_ratio_cp_air_assignment_count",\s*0' -Description "coupled direct K zero"
Assert-Contains -Path $cp349CoupledFixtureRoot -Pattern $cp349Stem -Description "CP349 coupled fixture registration"
Assert-Contains -Path $cp349CoupledTests -Pattern 'cp349_coupled_direct_none_route_is_complete_skip_and_corruption_fails_closed' -Description "coupled CP349 direct skip"
Assert-NotContains -Path $cp349Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled numerical firewall"
Assert-Contains -Path $cp349PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp349PipelineStem) + ';') -Description "CP349 pipeline module"
Assert-Contains -Path $cp349PipelineRoot -Pattern ('"' + $cp349Lifecycle + '":\s*result\s*\.' + $cp349Lifecycle) -Description "CP349 lifecycle JSON"
Assert-Contains -Path $cp349PipelineValidation -Pattern 'case_entry_cp348' -Description "pipeline CP348 predecessor"
Assert-Contains -Path $cp349PipelineValidation -Pattern '(?s)assignments\s*\.checked_mul\(.*?CP_AIR_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 3*K"
Assert-Contains -Path $cp349Serialization -Pattern 'lifecycle_serializes_cp349_direct_none_complete_skip_and_zero_source_counters' -Description "pipeline lifecycle skip JSON"
Assert-Contains -Path $cp349SnapshotSerialization -Pattern 'direct_none_skip_serializes_null_numeric_values_and_bits' -Description "pipeline null numeric JSON"
Assert-Contains -Path $cp349SnapshotSerialization -Pattern 'finite_and_nonfinite_numeric_serialization_preserves_authoritative_bits' -Description "defensive serializer bits"
Assert-Contains -Path $cp349PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp420_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp349ArbitraryTests -Pattern 'ideal_loads_no_oa_branch_runs_declared_compatibility_runtime' -Description "arbitrary direct evidence"
Assert-Contains -Path $cp349ArbitraryTests -Pattern $cp349Lifecycle -Description "arbitrary CP349 lifecycle"
Assert-NotContains -Path $cp349PipelineValidation -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "pipeline numerical firewall"
# Exactly two algorithm and capability addenda plus 2+2+1+1 targets.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches($algorithmText, '(?m)^\s*"CP349 supersedes only CP348[^"\r\n]+",\s*$')
$capabilityAddenda = [regex]::Matches($capabilityText, '(?m)^\s*"CP349 additionally requires[^"\r\n]+",\s*$')
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP349 addenda"
}
foreach ($claim in @($algorithmAddenda) + @($capabilityAddenda)) {
    foreach ($pattern in @(
            $cp349SourceHash, 'physical executable line 2216', $cp349Sites[0],
            $cp349Sites[1], $cp349Sites[2], 'line 2217',
            'T=U\+N\+P\+C0\+K\+H\+CSH', 'S=C0\+K\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=3\*K',
            'C0=S', 'K=H=CSH=0', 'CP348.*?immediate',
            'CP329.*?solely owns', 'CP345.*?corroborat',
            'CP331.*?CP338.*?(?:substitut|reus)', 'finite and `>=0\.0`',
            'dwSave.*?cpaSave', 'CP348-to-CP349-to-unchanged-numerical',
            $cp349Lifecycle, 'DirectZonePurchasedAirCouplingInput',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            'Roadmap (?:promotion|state)'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP349 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp349Stem/release\.rs::advance_direct_no_oa_calc_$cp349Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp349Stem\.rs::purchased_air_calc_${cp349Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp349Stem\.rs::${cp349TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp349Stem\.rs::${cp349TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    $count = [regex]::Matches($algorithmText, $target.Pattern).Count
    if ($count -ne $target.Expected) {
        throw "CP349 target '$($target.Pattern)' expected $($target.Expected), found $count"
    }
}
# Five hand-authored sections carry the same bounded, non-promoting contract.
$documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP349 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP349 Source-Ordered Cooling Positive-Supply Constant-Sensible-Heat-Ratio CpAir Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP349 Constant-Sensible-Heat-Ratio CpAir Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP349 Constant-Sensible-Heat-Ratio CpAir Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP349 Constant-Sensible-Heat-Ratio CpAir-Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $documentation) {
    $matches = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($matches.Count -ne 1) {
        throw "CP349 documentation expected one scoped section in $($doc.Path)"
    }
    $section = $matches[0].Value
    foreach ($pattern in @(
            $cp349SourceHash, '2216', $cp349Sites[0], $cp349Sites[1], $cp349Sites[2],
            '2217', 'T\s*=\s*U\+N\+P\+C0\+K\+H\+CSH',
            'S\s*=\s*C0\+K\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'source_site_execution_count\s*=\s*3\*K', 'C0\s*=\s*S',
            'K\s*=\s*H\s*=\s*CSH\s*=\s*0', '(?s)CP348.*?(?:immediate|predecessor)',
            'CP329', 'CP345', '(?s)CP331.*?CP338', 'finite', '>=0\.0',
            '(?s)dwSave.*?cpaSave', 'CP348-to-CP349-to-unchanged-numerical',
            $cp349Lifecycle, 'DirectZonePurchasedAirCouplingInput',
            '32\s+algorithms', '293\s+routines', 'Roadmap'
        )) {
        if ($section -notmatch $pattern) {
            throw "CP349 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP349\b' -Description "CP349 call-site-only psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP349 supersedes only CP348' -Description "generated CP349 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP349 additionally requires' -Description "generated CP349 capability addendum"
# Historical order whitelists, cumulative firewalls, master order, and inventory.
foreach ($historical in @(
        "cp326-cooling-supply-mass-flow-limit-body.ps1",
        "cp329-cooling-mixed-air-call.ps1",
        "cp330-cooling-supply-mass-flow-positive-guard.ps1",
        "cp331-cooling-positive-supply-cp-air-assignment.ps1",
        "cp332-cooling-positive-supply-temperature-assignment.ps1",
        "cp333-cooling-positive-supply-temperature-minimum-limit.ps1",
        "cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1",
        "cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1",
        "cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1",
        "cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit.ps1",
        "cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1",
        "cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch.ps1",
        "cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1",
        "cp348-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-entry.ps1"
    )) {
    Assert-Contains `
        -Path "scripts\quality\ideal-loads-structure-audit\$historical" `
        -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment' `
        -Description "historical CP349 binding whitelist"
}
foreach ($historical in @(
        "cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1",
        "cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1",
        "cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1",
        "cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit.ps1",
        "cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1",
        "cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch.ps1",
        "cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1",
        "cp348-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-entry.ps1"
    )) {
    Assert-Contains `
        -Path "scripts\quality\ideal-loads-structure-audit\$historical" `
        -Pattern 'non_direct_runtime_rejects_cp316_through_cp420_lifecycle_evidence' `
        -Description "historical cumulative non-direct firewall"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp348AuditIndex = $mainAuditText.IndexOf("cp348-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-entry.ps1")
$cp349AuditIndex = $mainAuditText.IndexOf("cp349-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1")
$completionIndex = $mainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp348AuditIndex -lt 0 -or $cp349AuditIndex -le $cp348AuditIndex -or $completionIndex -le $cp349AuditIndex) {
    throw "Main IdealLoads audit must dot-source CP349 after CP348 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 358' -Description "CP349 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP349 zero uncalled scripts"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern ('path = "scripts/quality/ideal-loads-structure-audit/' + [regex]::Escape((Split-Path $cp349Audit -Leaf)) + '"') -Description "CP349 internal inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern ([regex]::Escape(($cp349Audit -replace '\\', '/')) + '::dot_sources') -Description "CP349 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 358 \|' -Description "generated CP349 script total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 118 \|' -Description "generated internal total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated zero uncalled"
Assert-Cp349MutationRejected `
    -Original $cp349ReleaseText `
    -Mutated $cp349ReleaseText.Replace(
        'completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_is_consistent(',
        'completed_cp348_proof_was_bypassed('
    ) `
    -Validator { param($text) Assert-Cp349ReleaseContract -Text $text } `
    -Description "CP348 recursive proof removal"
Assert-Cp349MutationRejected `
    -Original $cp349TransitionText `
    -Mutated $cp349TransitionText.Replace(
        'energyplus_psy_cp_air_fn_w(mixed_air_humidity_ratio)',
        'reused_cp338_cp_air(mixed_air_humidity_ratio)'
    ) `
    -Validator { param($text) Assert-Cp349TransitionContract -Text $text } `
    -Description "canonical helper replacement"
Assert-Cp349MutationRejected `
    -Original $cp349RuntimeText `
    -Mutated $cp349RuntimeText.Replace('assignments.checked_mul(', 'assignments.checked_add(') `
    -Validator { param($text) Assert-Cp349RuntimeContract -Text $text } `
    -Description "3*K multiplier corruption"
Assert-Cp349MutationRejected `
    -Original $cp349BindingText `
    -Mutated $cp349BindingText.Replace(
        'DirectZonePurchasedAirCouplingInput {',
        "DirectZonePurchasedAirCouplingInput {`n            cp349_evidence: calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,"
    ) `
    -Validator { param($text) Assert-Cp349BindingContract -Text $text } `
    -Description "CP349 numerical DTO injection"
$cp349NonDirectPattern = 'result\s*\.\s*' + [regex]::Escape($cp349Lifecycle) + '\s*\.\s*is_some\s*\(\s*\)'
$cp349NonDirectMutation = [regex]::Replace(
    $cp349PipelineRootText,
    $cp349NonDirectPattern,
    ('result.' + $cp349Lifecycle + '.is_none()'),
    1
)
Assert-Cp349MutationRejected `
    -Original $cp349PipelineRootText `
    -Mutated $cp349NonDirectMutation `
    -Validator { param($text) Assert-Cp349PipelineRootContract -Text $text } `
    -Description "non-direct CP349 firewall mutation"
Assert-Cp349MutationRejected `
    -Original $cp349SnapshotSerializationText `
    -Mutated $cp349SnapshotSerializationText.Replace(
        '"cp_air_j_per_kg_k_ieee_bits"',
        '"cp_air_j_per_kg_k_ieee_bits_mutated"'
    ) `
    -Validator { param($text) Assert-Cp349SerializationContract -Text $text } `
    -Description "CP349 authoritative bits JSON mutation"
Write-Host "CP349 constant-SHR CpAir-assignment structure audit passed."
