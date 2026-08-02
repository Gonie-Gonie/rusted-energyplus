# CP347 maps the complete PurchasedAirManager.cc physical lines 2210-2212
# `HumControl::None` case. Line 2213 is the first excluded lexical construct,
# line 2216 the first subsequent executable, and line 2245 the direct dynamic
# continuation after the source `break`.

$cp347Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case"
$cp347PipelineStem = "purchased_air_$cp347Stem"
$cp347TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCase"
$cp347Module = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem.rs"
$cp347State = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\state.rs"
$cp347Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\transition.rs"
$cp347Release = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\release.rs"
$cp347Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\release\prefix_validation.rs"
$cp347Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\release\runtime_validation.rs"
$cp347Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\release\snapshot_validation.rs"
$cp347Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\tests\mod.rs"
$cp347PublicTests = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\tests\public_release.rs"
$cp347CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp347Stem\tests\release_corruption.rs"
$cp347Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp347Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp347BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp347Stem.rs"
$cp347BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp347Stem}_tests.rs"
$cp347Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$cp347Stem`_validation.rs"
$cp347CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp347CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$cp347Stem`_fixture.rs"
$cp347PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp347Pipeline = "crates\ep_run\src\pipeline\$cp347PipelineStem.rs"
$cp347PipelineValidation = "crates\ep_run\src\pipeline\$cp347PipelineStem\validation.rs"
$cp347Serialization = "crates\ep_run\src\pipeline\$cp347PipelineStem\serialization\snapshot.rs"
$cp347ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp347Audit = "scripts\quality\ideal-loads-structure-audit\cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1"
$cp347Lifecycle = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle"
$cp347SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp347Sites = @(
    "enter-purchased-air-dehumidification-control-none-case",
    "read-purchased-air-mixed-air-humidity-ratio-for-none-case",
    "assign-purchased-air-supply-humidity-ratio-in-none-case",
    "exit-purchased-air-dehumidification-control-none-case-via-break"
)

function Get-Cp347RustBraceBlock {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$AnchorPattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
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
                return $Text.Substring(
                    $anchors[0].Index,
                    $index - $anchors[0].Index + 1
                )
            }
        }
    }
    throw "$Description closing brace is missing"
}

function Assert-Cp347ReleaseContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    if ($Text -notmatch ('(?s)pub fn advance_direct_no_oa_calc_' +
            [regex]::Escape($cp347Stem) +
            '\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp346:\s*PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,\s*\)')) {
        throw "CP347 public release arguments are not exact"
    }
    $body = Get-Cp347RustBraceBlock `
        -Text $Text `
        -AnchorPattern ('(?m)^\s*pub fn advance_direct_no_oa_calc_' + [regex]::Escape($cp347Stem) + '\s*\(') `
        -Description "CP347 release body"
    $proofPatterns = @(
        'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None',
        'predecessor_snapshots_match_bit_exact\(',
        'cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release\(',
        'completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent\(',
        'owner_lineage_is_exact\(',
        'completed_direct_cooling_mixed_air_call_is_consistent\(',
        'humidity_ratio_lineage_is_exact\(',
        'pending_state_is_consistent\(',
        'active_input_from_owner\(',
        'next_transition_fits\('
    )
    $lastProof = -1
    foreach ($pattern in $proofPatterns) {
        $matches = [regex]::Matches($body, $pattern)
        if ($matches.Count -lt 1) {
            throw "CP347 release proof missing '$pattern'"
        }
        $last = $matches[$matches.Count - 1]
        $lastProof = [Math]::Max($lastProof, $last.Index + $last.Length)
    }
    $firstMutation = $body.IndexOf("runtime.units.get_mut(")
    if ($firstMutation -lt $lastProof) {
        throw "CP347 release must prove CP346, CP329 ownership, CP345 corroboration, and overflow before mutation"
    }
    if ($body -match '(?i)DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|psychrometric_service|cache|diagnostic') {
        throw "CP347 release admits a numerical DTO or forbidden service"
    }
    if ($body -match '(?s)advance_direct_no_oa_calc_[^(]+\([^)]*(?:mixed_air_humidity_ratio|assigned_supply_humidity_ratio)\s*:') {
        throw "CP347 release admits a duplicate humidity scalar"
    }
}

function Assert-Cp347PrefixContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    foreach ($pattern in @(
            '(?s)predecessor_selects_none_case.*?DehumidificationControlType::None',
            '(?s)active_input_from_owner.*?owner\?\.mixed_air_humidity_ratio\?',
            'owner_lineage_is_exact\(',
            'cooling_mixed_air_call_snapshots_match_bit_exact\(',
            'humidity_ratio_lineage_is_exact\(',
            '(?s)cp345\.mixed_air_humidity_ratio,\s*Some\(owner_value\)',
            '(?s)cp345\.assigned_supply_humidity_ratio,\s*Some\(owner_value\)',
            '(?s)predecessor\.predecessor_assigned_supply_humidity_ratio,\s*Some\(owner_value\)',
            '(?s)Some\(left\), Some\(right\)\)\s*=>\s*left\.to_bits\(\)\s*==\s*right\.to_bits\(\)'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP347 owner/corroboration contract missing '$pattern'"
        }
    }
}

function Assert-Cp347RuntimeContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    foreach ($pattern in @(
            '(?s)unit_off_skip_count.*?checked_add\(state\.non_cooling_skip_count\).*?positive_guard_false_fallthrough_skip_count.*?dehumidification_control_none_case_completion_count.*?constant_sensible_heat_ratio_case_selection_count.*?humidistat_case_selection_count.*?constant_supply_humidity_ratio_case_selection_count',
            '(?s)let Some\(active\).*?none_case_completion_count.*?constant_sensible_heat_ratio_case_selection_count.*?humidistat_case_selection_count.*?constant_supply_humidity_ratio_case_selection_count',
            '(?s)dehumidification_control_none_case_completion_count\s*\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER\s*\.len\(\)',
            'expected_none\s*=\s*usize::from\(selector\s*==\s*DehumidificationControlType::None\)\s*\*\s*active',
            'route_partition\s*==\s*state\.transition_count',
            'source_site_execution_count\s*==\s*expected_source_sites',
            'dehumidification_control_none_case_entry_count\s*==\s*expected_none',
            'mixed_air_humidity_ratio_read_count\s*==\s*expected_none',
            'supply_humidity_ratio_assignment_count\s*==\s*expected_none',
            'dehumidification_control_none_case_break_count\s*==\s*expected_none',
            'witnessed_dehumidification_control_none_case_completion_count',
            'witnessed_dehumidification_control_constant_sensible_heat_ratio_case_selection_count',
            'witnessed_dehumidification_control_humidistat_case_selection_count',
            'witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selection_count'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP347 checked algebra or witnessed parity missing '$pattern'"
        }
    }
}

function Assert-Cp347BindingContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $cp346 = $Text.IndexOf(
        "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch ="
    )
    $cp347 = $Text.IndexOf(
        "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case ="
    )
    $cp348 = $Text.IndexOf(
        "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry ="
    )
    $cp349 = $Text.IndexOf(
        "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment ="
    )
    $cp350 = $Text.IndexOf(
        "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment ="
    )
    $cp351 = $Text.IndexOf(
        "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment ="
    )
    $cp352 = $Text.IndexOf(
        "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment ="
    )
    $cp353 = $Text.IndexOf(
        "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit ="
    )
    $cp354 = $Text.IndexOf(
        "let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit ="
    )
    $cp355 = $Text.IndexOf(
        "let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit ="
    )
    $cp356 = $Text.IndexOf(
        "let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit ="
    )
    $cp357 = $Text.IndexOf("let calculation_cooling_constant_shr_case_break =")
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf(
        "let coupling = complete_direct_zone_purchased_air_coupling("
    )
    if ($cp346 -lt 0 -or $cp347 -le $cp346 -or $cp348 -le $cp347 -or $cp349 -le $cp348 -or $cp350 -le $cp349 -or $cp351 -le $cp350 -or $cp352 -le $cp351 -or $cp353 -le $cp352 -or $cp354 -le $cp353 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP346 then CP347 then CP348 then CP349 then CP350 then CP351 then CP352 then CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp347RustBraceBlock `
        -Text $Text.Substring($numerical) `
        -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' `
        -Description "CP347 numerical DTO"
    if ($dto -match '(?i)cp347|dehumidification_control_none_case|DehumidificationControlNoneCase') {
        throw "CP347 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp347PipelineRootContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $execute = Get-Cp347RustBraceBlock `
        -Text $Text.Substring(0, $boundary.Index) `
        -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' `
        -Description "pipeline execute_rust_runtime"
    $none = [regex]::Matches(
        $execute,
        [regex]::Escape($cp347Lifecycle) + '\s*:\s*None'
    ).Count
    $some = [regex]::Matches(
        $execute,
        'let\s+' + [regex]::Escape($cp347Lifecycle) + '\s*=\s*Some\s*\('
    ).Count
    $shorthand = [regex]::Matches(
        $execute,
        '(?m)^\s*' + [regex]::Escape($cp347Lifecycle) + '\s*,\s*$'
    ).Count
    if ($none -ne 3 -or $some -ne 1 -or $shorthand -ne 1) {
        throw "Pipeline must expose CP347 through one direct Some and three non-direct None constructors"
    }
    $provenance = Get-Cp347RustBraceBlock `
        -Text $Text.Substring(0, $boundary.Index) `
        -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' `
        -Description "pipeline non-direct firewall"
    if (
        [regex]::Matches(
            $provenance,
            'result\s*\.\s*' + [regex]::Escape($cp347Lifecycle) + '\s*\.\s*is_some\s*\(\s*\)'
        ).Count -ne 1
    ) {
        throw "Pipeline non-direct rejection must include CP347 is_some exactly once"
    }
}

function Assert-Cp347SerializationContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    $production = if ($boundary.Success) { $Text.Substring(0, $boundary.Index) } else { $Text }
    foreach ($field in @(
            "predecessor_assigned_supply_humidity_ratio",
            "mixed_air_humidity_ratio",
            "assigned_supply_humidity_ratio",
            "resulting_supply_humidity_ratio"
        )) {
        if (
            [regex]::Matches(
                $production,
                '"' + $field + '"\s*:\s*json_number\(\s*snapshot\.' + $field
            ).Count -ne 1 -or
            [regex]::Matches(
                $production,
                '"' + $field + '_ieee_bits"\s*:\s*ieee_bits\(\s*snapshot\.' + $field
            ).Count -ne 1
        ) {
            throw "CP347 JSON must map '$field' value and IEEE bits exactly once"
        }
    }
    foreach ($mapping in @(
            'DehumidificationControlType::None\s*=>\s*"None"',
            'DehumidificationControlType::ConstantSensibleHeatRatio\s*=>\s*"ConstantSensibleHeatRatio"',
            'DehumidificationControlType::Humidistat\s*=>\s*"Humidistat"',
            'DehumidificationControlType::ConstantSupplyHumidityRatio\s*=>\s*"ConstantSupplyHumidityRatio"'
        )) {
        if ($production -notmatch $mapping) {
            throw "CP347 symbolic selector JSON missing '$mapping'"
        }
    }
    if (
        $production -notmatch '(?s)filter\(\s*\|value\| value\.is_finite\(\)\s*\).*?Value::Null' -or
        $production -notmatch 'format!\("0x\{:016x\}", value\.to_bits\(\)\)' -or
        $production -match '(?i)dehumidification_control_type_(?:ordinal|discriminant|ieee_bits)|dehumidification_control_type\s+as\s+(?:usize|isize|u\d+|i\d+)'
    ) {
        throw "CP347 JSON must retain symbolic selectors and defensive value-plus-bits humidity projection"
    }
}

function Assert-Cp347MutationRejected {
    param(
        [Parameter(Mandatory = $true)][string]$Original,
        [Parameter(Mandatory = $true)][string]$Mutated,
        [Parameter(Mandatory = $true)][scriptblock]$Validator,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Original -ceq $Mutated) {
        throw "CP347 self-test mutation was not applied: $Description"
    }
    $rejected = $false
    try {
        & $Validator $Mutated
    } catch {
        $rejected = $true
    }
    if (-not $rejected) {
        throw "CP347 audit failed to reject mutation: $Description"
    }
}

foreach ($file in @(
        $cp347Module, $cp347State, $cp347Transition, $cp347Release,
        $cp347Prefix, $cp347Runtime, $cp347Snapshot, $cp347Tests,
        $cp347PublicTests, $cp347CorruptionTests, $cp347BindingAdapter,
        $cp347BindingTests, $cp347Coupled, $cp347CoupledFixture,
        $cp347Pipeline, $cp347PipelineValidation, $cp347Serialization,
        $cp347ArbitraryTests, $cp347CoupledTests
    )) {
    Assert-FileExists -Path $file -Description "CP347 structure"
}
Assert-LineLimit -Path $cp347Release -Limit 500 -Description "CP347 release module"
Assert-LineLimit -Path $cp347Runtime -Limit 450 -Description "CP347 runtime validation"
Assert-LineLimit -Path $cp347Coupled -Limit 500 -Description "CP347 coupled validation"
Assert-LineLimit -Path $cp347PipelineValidation -Limit 450 -Description "CP347 pipeline validation"
Assert-LineLimit -Path $cp347Audit -Limit 600 -Description "CP347 audit"

# Exact source boundary, route partition, and bit-copy behavior.
Assert-Contains -Path $cp347Module -Pattern 'PurchasedAirManager\.cc:2210-2212' -Description "CP347 complete None-case source"
Assert-Contains -Path $cp347Module -Pattern 'PurchasedAirManager\.cc:2216' -Description "CP347 first subsequent executable"
Assert-ExactStringArray -Path $cp347Module `
    -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER" `
    -Expected $cp347Sites `
    -Description "CP347 exact four-site source order"
Assert-Contains -Path $cp347State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompleted,\s*DehumidificationControlConstantSensibleHeatRatioCaseSelected,\s*DehumidificationControlHumidistatCaseSelected,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelected' -Description "CP347 seven routes"
Assert-Contains -Path $cp347Transition -Pattern '(?s)DehumidificationControlNoneCaseCompleted\s*=>\s*\{.*?source_site_execution_count.*?none_case_entry_count.*?mixed_air_humidity_ratio_read_count.*?supply_humidity_ratio_assignment_count.*?none_case_break_count' -Description "CP347 None-only four sites"
Assert-Contains -Path $cp347Transition -Pattern '(?s)let assigned_supply_humidity_ratio\s*=\s*none_case_completed\s*\.then_some\(mixed_air_humidity_ratio\)\s*\.flatten\(\);\s*let resulting_supply_humidity_ratio\s*=\s*assigned_supply_humidity_ratio;' -Description "CP347 exact bit copy"
Assert-NotContains -Path $cp347Transition -Pattern '(?i)Psy[A-Za-z0-9_]*\s*\(|f64::(?:min|max)|\.clamp\(|total_cmp|partial_cmp|is_finite\(\)|diagnostic|cache' -Description "CP347 no numerical or service work"

$cp347ReleaseText = Read-RepoText -Path $cp347Release
$cp347PrefixText = Read-RepoText -Path $cp347Prefix
$cp347RuntimeText = Read-RepoText -Path $cp347Runtime
$cp347BindingText = Read-RepoText -Path $cp347Binding
$cp347PipelineRootText = Read-RepoText -Path $cp347PipelineRoot
$cp347SerializationText = Read-RepoText -Path $cp347Serialization
Assert-Cp347ReleaseContract -Text $cp347ReleaseText
Assert-Cp347PrefixContract -Text $cp347PrefixText
Assert-Cp347RuntimeContract -Text $cp347RuntimeText
Assert-Cp347BindingContract -Text $cp347BindingText
Assert-Cp347PipelineRootContract -Text $cp347PipelineRootText
Assert-Cp347SerializationContract -Text $cp347SerializationText
Assert-Contains -Path $cp347Snapshot -Pattern '(?s)DehumidificationControlType::None.*?dehumidification_control_none_case_entered.*?mixed_air_humidity_ratio_read.*?supply_humidity_ratio_assignment_performed.*?dehumidification_control_none_case_exited_via_break' -Description "CP347 exact direct snapshot"
Assert-Contains -Path $cp347Release -Pattern 'DehumidificationControlTypeOutsideDirectSubset' -Description "CP347 direct None gate"
Assert-Contains -Path $cp347Tests -Pattern 'pure_transition_retains_each_deferred_non_none_selection_without_none_sites' -Description "CP347 non-None private skips"
Assert-Contains -Path $cp347PublicTests -Pattern 'public_g_f_l_routes_execute_the_exact_none_case_and_preserve_owner_bits' -Description "CP347 direct G/F/L"
Assert-Contains -Path $cp347PublicTests -Pattern 'public_u_n_p_routes_skip_all_none_case_sites' -Description "CP347 U/N/P skips"
Assert-Contains -Path $cp347CorruptionTests -Pattern 'cp329_owner_and_cp345_corroboration_corruption_are_rejected_transactionally' -Description "CP347 owner/corroboration rejection"
Assert-Contains -Path $cp347CorruptionTests -Pattern 'every_none_case_counter_increment_is_preflighted' -Description "CP347 overflow preflight"
Assert-Contains -Path $cp347BindingTests -Pattern 'scheduled_binding_completes_cp347_none_case_after_every_cp346_active_route' -Description "CP347 binding G/F/L"
Assert-Contains -Path $cp347CoupledTests -Pattern 'cp347_direct_coupled_runtime_completes_none_case_after_g_f_l_and_skips_unit_off' -Description "CP347 coupled G/F/L/U"
Assert-Contains -Path $cp347CoupledTests -Pattern 'cp347_direct_coupled_runtime_covers_non_cooling_and_positive_guard_false_skips' -Description "CP347 coupled N/P"
Assert-Contains -Path $cp347PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp396_lifecycle_evidence' -Description "CP347 cumulative non-direct firewall"
Assert-Contains -Path $cp347PipelineRoot -Pattern ('"' + $cp347Lifecycle + '":\s*result\s*\.' + $cp347Lifecycle) -Description "CP347 lifecycle JSON"
Assert-Contains -Path $cp347PipelineValidation -Pattern 'control_switch_cp346' -Description "pipeline CP346 predecessor"
Assert-Contains -Path $cp347PipelineValidation -Pattern 'mixed_air_cp329' -Description "pipeline CP329 owner"
Assert-Contains -Path $cp347Coupled -Pattern 'direct_none_case_completion_count' -Description "coupled direct C0 equals S"
Assert-Contains -Path $cp347Coupled -Pattern 'direct_constant_sensible_heat_ratio_case_selection_count' -Description "coupled direct CSHR zero"
Assert-Contains -Path $cp347Coupled -Pattern 'direct_humidistat_case_selection_count' -Description "coupled direct H zero"
Assert-Contains -Path $cp347Coupled -Pattern 'direct_constant_supply_humidity_ratio_case_selection_count' -Description "coupled direct CSH zero"
Assert-NotContains -Path $cp347Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled CP347 numerical firewall"
Assert-NotContains -Path $cp347PipelineValidation -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply' -Description "pipeline CP347 numerical firewall"

# Exactly two algorithm and two capability addenda and the 2+2+1+1 targets.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches(
    $algorithmText,
    '(?m)^\s*"CP347 supersedes only CP346[^"\r\n]+",\s*$'
)
$capabilityAddenda = [regex]::Matches(
    $capabilityText,
    '(?m)^\s*"CP347 additionally requires[^"\r\n]+",\s*$'
)
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP347 addenda"
}
$cp347Claims = @($algorithmAddenda | ForEach-Object { $_.Value }) +
    @($capabilityAddenda | ForEach-Object { $_.Value })
foreach ($claim in $cp347Claims) {
    foreach ($pattern in @(
            $cp347SourceHash,
            'physical(?:\s+|-+)lines(?:\s+|-+)2210-2212',
            '(?s)enter-purchased-air-dehumidification-control-none-case.*?read-purchased-air-mixed-air-humidity-ratio-for-none-case.*?assign-purchased-air-supply-humidity-ratio-in-none-case.*?exit-purchased-air-dehumidification-control-none-case-via-break',
            'line 2213',
            'line 2216',
            'line 2245',
            'T=U\+N\+P\+C0\+CSHR\+H\+CSH',
            'S=C0\+CSHR\+H\+CSH=R=G\+F\+L',
            'A=F\+L',
            'source_site_execution_count=4\*C0',
            'C0=S',
            'CSHR=H=CSH=0',
            'CP329 `mixed_air_humidity_ratio` solely owns',
            'CP345 `assigned_supply_humidity_ratio`',
            'CP346.*?immediate',
            'CP346-to-CP347-to-unchanged-numerical',
            $cp347Lifecycle,
            'DirectZonePurchasedAirCouplingInput',
            '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped',
            'Roadmap (?:promotion|state)'
        )) {
        if ($claim -notmatch $pattern) {
            throw "CP347 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{
            Pattern = "$cp347Stem/release\.rs::advance_direct_no_oa_calc_$cp347Stem"
            Expected = 2
        },
        [PSCustomObject]@{
            Pattern = "$cp347Stem\.rs::purchased_air_calc_${cp347Stem}_lifecycle_summary"
            Expected = 2
        },
        [PSCustomObject]@{
            Pattern = "$cp347Stem\.rs::${cp347TypeStem}RuntimeState"
            Expected = 1
        },
        [PSCustomObject]@{
            Pattern = "$cp347Stem\.rs::${cp347TypeStem}LifecycleSummary"
            Expected = 1
        }
    )) {
    $count = [regex]::Matches($algorithmText, $target.Pattern).Count
    if ($count -ne $target.Expected) {
        throw "CP347 target '$($target.Pattern)' expected $($target.Expected), found $count"
    }
}

# Exactly five hand-authored sections carry the same boundary.
$documentation = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP347 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP347 Source-Ordered Cooling Positive-Supply Post-Capacity-Limit Dehumidification-Control None Case\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP347 Cooling Positive-Supply Post-Capacity-Limit Dehumidification-Control None Case\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP347 Dehumidification-Control None Case in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP347 Cooling Positive-Supply Dehumidification-Control None-Case Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($doc in $documentation) {
    $matches = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($matches.Count -ne 1) {
        throw "CP347 documentation expected one scoped section in $($doc.Path)"
    }
    $section = $matches[0].Value
    foreach ($pattern in @(
            $cp347SourceHash,
            '2210-2212',
            '(?s)enter-purchased-air-dehumidification-control-none-case.*?read-purchased-air-mixed-air-humidity-ratio-for-none-case.*?assign-purchased-air-supply-humidity-ratio-in-none-case.*?exit-purchased-air-dehumidification-control-none-case-via-break',
            '2213',
            '2216',
            '2245',
            'T\s*=\s*U\+N\+P\+C0\+CSHR\+H\+CSH',
            'S\s*=\s*C0\+CSHR\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L',
            '4\*C0',
            'C0\s*=\s*S',
            'CSHR\s*=\s*H\s*=\s*CSH\s*=\s*0',
            '(?s)CP329.*?mixed_air_humidity_ratio.*?(?:sole|solely|only|owns|owner)',
            '(?s)CP345.*?corroborat',
            'CP346-to-CP347-to-unchanged-numerical',
            $cp347Lifecycle,
            'DirectZonePurchasedAirCouplingInput',
            '32\s+algorithms',
            '293\s+routines',
            'Roadmap'
        )) {
        if ($section -notmatch $pattern) {
            throw "CP347 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP347\b' -Description "CP347 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP347 supersedes only CP346' -Description "generated CP347 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP347 additionally requires' -Description "generated CP347 capability addendum"

# Historical binding whitelists, cumulative firewalls, root reachability, and
# the generated script inventory all advance to CP347.
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
        "cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch.ps1"
    )) {
    Assert-Contains `
        -Path "scripts\quality\ideal-loads-structure-audit\$historical" `
        -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case' `
        -Description "historical CP347 binding whitelist"
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
        "cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch.ps1"
    )) {
    Assert-Contains `
        -Path "scripts\quality\ideal-loads-structure-audit\$historical" `
        -Pattern 'non_direct_runtime_rejects_cp316_through_cp396_lifecycle_evidence' `
        -Description "historical cumulative non-direct firewall through CP352"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp346AuditIndex = $mainAuditText.IndexOf(
    "cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch.ps1"
)
$cp347AuditIndex = $mainAuditText.IndexOf(
    "cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1"
)
$completionIndex = $mainAuditText.IndexOf(
    'Write-Host "IdealLoads structure audit complete."'
)
if ($cp346AuditIndex -lt 0 -or $cp347AuditIndex -le $cp346AuditIndex -or $completionIndex -le $cp347AuditIndex) {
    throw "Main IdealLoads audit must dot-source CP347 after CP346 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 334' -Description "CP347 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP347 zero uncalled scripts"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case\.ps1"' -Description "CP347 internal record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case\.ps1::dot_sources' -Description "CP347 caller/callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 334 \|' -Description "generated CP347 script total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 94 \|' -Description "generated internal total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated zero uncalled"

# Mutation self-tests reject boundary, ownership, algebra, numerical-feed, direct-only, and JSON regressions.
Assert-Cp347MutationRejected `
    -Original $cp347ReleaseText `
    -Mutated $cp347ReleaseText.Replace(
        'owner_lineage_is_exact(',
        'owner_lineage_was_bypassed('
    ) `
    -Validator { param($text) Assert-Cp347ReleaseContract -Text $text } `
    -Description "CP329 owner proof removal"
Assert-Cp347MutationRejected `
    -Original $cp347RuntimeText `
    -Mutated $cp347RuntimeText.Replace('.checked_mul(', '.checked_add(') `
    -Validator { param($text) Assert-Cp347RuntimeContract -Text $text } `
    -Description "four-site multiplier corruption"
Assert-Cp347MutationRejected `
    -Original $cp347BindingText `
    -Mutated $cp347BindingText.Replace(
        'DirectZonePurchasedAirCouplingInput {',
        "DirectZonePurchasedAirCouplingInput {`n            cp347_evidence: calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,"
    ) `
    -Validator { param($text) Assert-Cp347BindingContract -Text $text } `
    -Description "CP347 evidence injection into numerical DTO"
$cp347NonDirectPattern = 'result\s*\.\s*' + [regex]::Escape($cp347Lifecycle) + '\s*\.\s*is_some\s*\(\s*\)'
$cp347NonDirectMutation = [regex]::Replace(
    $cp347PipelineRootText,
    $cp347NonDirectPattern,
    ('result.' + $cp347Lifecycle + '.is_none()'),
    1
)
Assert-Cp347MutationRejected `
    -Original $cp347PipelineRootText `
    -Mutated $cp347NonDirectMutation `
    -Validator { param($text) Assert-Cp347PipelineRootContract -Text $text } `
    -Description "non-direct CP347 is_some firewall mutation"
Assert-Cp347MutationRejected `
    -Original $cp347SerializationText `
    -Mutated $cp347SerializationText.Replace(
        '"mixed_air_humidity_ratio":',
        '"mixed_air_humidity_ratio_mutated":'
    ) `
    -Validator { param($text) Assert-Cp347SerializationContract -Text $text } `
    -Description "CP347 JSON owner-field mutation"

Write-Host "CP347 post-capacity dehumidification-control None-case audit complete."
