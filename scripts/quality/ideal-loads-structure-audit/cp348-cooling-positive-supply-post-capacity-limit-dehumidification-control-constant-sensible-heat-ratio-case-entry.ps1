# CP348 maps only PurchasedAirManager.cc physical line 2213, the
# `HumControl::ConstantSensibleHeatRatio` case entry. Lines 2214-2215 are
# comments and line 2216 is the first excluded executable.

$cp348Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry"
$cp348PipelineStem = "purchased_air_$cp348Stem"
$cp348TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntry"
$cp348Module = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem.rs"
$cp348State = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\state.rs"
$cp348Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\transition.rs"
$cp348Release = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\release.rs"
$cp348Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\release\prefix_validation.rs"
$cp348Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\release\runtime_validation.rs"
$cp348Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\release\snapshot_validation.rs"
$cp348Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\tests\mod.rs"
$cp348PublicTests = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\tests\public_release.rs"
$cp348CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp348Stem\tests\release_corruption.rs"
$cp348Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp348Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp348BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp348Stem.rs"
$cp348BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp348Stem}_tests.rs"
$cp348Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$cp348Stem`_validation.rs"
$cp348CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp348CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$cp348Stem`_fixture.rs"
$cp348PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp348Pipeline = "crates\ep_run\src\pipeline\$cp348PipelineStem.rs"
$cp348PipelineValidation = "crates\ep_run\src\pipeline\$cp348PipelineStem\validation.rs"
$cp348Serialization = "crates\ep_run\src\pipeline\$cp348PipelineStem\serialization.rs"
$cp348SnapshotSerialization = "crates\ep_run\src\pipeline\$cp348PipelineStem\serialization\snapshot.rs"
$cp348ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp348Audit = "scripts\quality\ideal-loads-structure-audit\cp348-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-entry.ps1"
$cp348Lifecycle = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle"
$cp348SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp348Sites = @(
    "enter-purchased-air-dehumidification-control-constant-sensible-heat-ratio-case"
)

function Get-Cp348RustBraceBlock {
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

function Assert-Cp348ReleaseContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    if ($Text -notmatch ('(?s)pub fn advance_direct_no_oa_calc_' +
            [regex]::Escape($cp348Stem) +
            '\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp347:\s*PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,\s*\)')) {
        throw "CP348 public release arguments are not exact"
    }
    $body = Get-Cp348RustBraceBlock `
        -Text $Text `
        -AnchorPattern ('(?m)^\s*pub fn advance_direct_no_oa_calc_' + [regex]::Escape($cp348Stem) + '\s*\(') `
        -Description "CP348 release body"
    $proofPatterns = @(
        'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None',
        'predecessor_snapshots_match_bit_exact\(',
        'cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release\(',
        'completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent\(',
        'pending_state_is_consistent\(',
        'next_transition_fits\('
    )
    $lastProof = -1
    foreach ($pattern in $proofPatterns) {
        $matches = [regex]::Matches($body, $pattern)
        if ($matches.Count -lt 1) {
            throw "CP348 release proof missing '$pattern'"
        }
        $last = $matches[$matches.Count - 1]
        $lastProof = [Math]::Max($lastProof, $last.Index + $last.Length)
    }
    $firstMutation = $body.IndexOf("runtime.units.get_mut(")
    if ($firstMutation -lt $lastProof) {
        throw "CP348 release must prove CP347 and overflow safety before mutation"
    }
    if ($body -match '(?i)DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|mixed_air_humidity_ratio|assigned_supply_humidity_ratio|Psy[A-Za-z0-9_]*\s*\(|latest_numerical|numerical_supply|final_supply|cache|diagnostic') {
        throw "CP348 release admits an operand, numerical DTO, or forbidden service"
    }
}

function Assert-Cp348RuntimeContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    foreach ($pattern in @(
            '(?s)unit_off_skip_count.*?checked_add\(state\.non_cooling_skip_count\).*?positive_guard_false_fallthrough_skip_count.*?dehumidification_control_none_case_completed_skip_count.*?dehumidification_control_constant_sensible_heat_ratio_case_entry_count.*?dehumidification_control_humidistat_case_selected_skip_count.*?dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count',
            '(?s)dehumidification_control_none_case_completed_skip_count\s*\.checked_add\(\s*state\.dehumidification_control_constant_sensible_heat_ratio_case_entry_count.*?dehumidification_control_humidistat_case_selected_skip_count.*?dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count',
            '(?s)dehumidification_control_constant_sensible_heat_ratio_case_entry_count\s*\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER\s*\.len\(\)',
            'expected_none\s*=\s*usize::from\(selector\s*==\s*DehumidificationControlType::None\)\s*\*\s*active',
            'expected_constant_sensible\s*=\s*[\r\n\s]*usize::from\(selector\s*==\s*DehumidificationControlType::ConstantSensibleHeatRatio\)\s*\*\s*active',
            'route_partition\s*==\s*state\.transition_count',
            'source_site_execution_count\s*==\s*expected_source_sites',
            'dehumidification_control_none_case_completed_skip_count\s*==\s*expected_none',
            'dehumidification_control_constant_sensible_heat_ratio_case_entry_count\s*[\r\n\s]*==\s*expected_constant_sensible',
            'dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count\s*[\r\n\s]*==\s*expected_constant_sensible'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP348 checked algebra missing '$pattern'"
        }
    }
}

function Assert-Cp348BindingContract {
    param([Parameter(Mandatory = $true)][string]$Text)
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
    $cp357 = $Text.IndexOf(
        "let calculation_cooling_constant_shr_case_break ="
    )
    $cp358 = $Text.IndexOf("let calculation_cooling_humidistat_case_entry =")
    $cp359 = $Text.IndexOf("let calculation_cooling_humidistat_moisture_demand_assignment =")
    $numerical = $Text.IndexOf(
        "let coupling = complete_direct_zone_purchased_air_coupling("
    )
    if ($cp347 -lt 0 -or $cp348 -le $cp347 -or $cp349 -le $cp348 -or $cp350 -le $cp349 -or $cp351 -le $cp350 -or $cp352 -le $cp351 -or $cp353 -le $cp352 -or $cp354 -le $cp353 -or $cp355 -le $cp354 -or $cp356 -le $cp355 -or $cp357 -le $cp356 -or $cp358 -le $cp357 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP347 then CP348 then CP349 then CP350 then CP351 then CP352 then CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
    }
    $dto = Get-Cp348RustBraceBlock `
        -Text $Text.Substring($numerical) `
        -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' `
        -Description "CP348 numerical DTO"
    if ($dto -match '(?i)cp348|constant_sensible_heat_ratio_case_entry|ConstantSensibleHeatRatioCaseEntry') {
        throw "CP348 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp348PipelineRootContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp348RustBraceBlock `
        -Text $production `
        -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' `
        -Description "pipeline execute_rust_runtime"
    $none = [regex]::Matches(
        $execute,
        [regex]::Escape($cp348Lifecycle) + '\s*:\s*None'
    ).Count
    $some = [regex]::Matches(
        $execute,
        'let\s+' + [regex]::Escape($cp348Lifecycle) + '\s*=\s*Some\s*\('
    ).Count
    $shorthand = [regex]::Matches(
        $execute,
        '(?m)^\s*' + [regex]::Escape($cp348Lifecycle) + '\s*,\s*$'
    ).Count
    if ($none -ne 3 -or $some -ne 1 -or $shorthand -ne 1) {
        throw "Pipeline must expose CP348 through one direct Some and three non-direct None constructors"
    }
    $provenance = Get-Cp348RustBraceBlock `
        -Text $production `
        -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' `
        -Description "pipeline non-direct firewall"
    if (
        [regex]::Matches(
            $provenance,
            'result\s*\.\s*' + [regex]::Escape($cp348Lifecycle) + '\s*\.\s*is_some\s*\(\s*\)'
        ).Count -ne 1
    ) {
        throw "Pipeline non-direct rejection must include CP348 is_some exactly once"
    }
}

function Assert-Cp348SerializationContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    foreach ($field in @(
            "predecessor_dehumidification_control_none_case_completed",
            "dehumidification_control_none_case_completed_skip",
            "dehumidification_control_constant_sensible_heat_ratio_case_entered",
            "dehumidification_control_humidistat_case_selected_skip",
            "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip"
        )) {
        if ([regex]::Matches($Text, '"' + $field + '"\s*:\s*snapshot\.' + $field).Count -ne 1) {
            throw "CP348 JSON must map '$field' exactly once"
        }
    }
    foreach ($mapping in @(
            'DehumidificationControlType::None\s*=>\s*"None"',
            'DehumidificationControlType::ConstantSensibleHeatRatio\s*=>\s*"ConstantSensibleHeatRatio"',
            'DehumidificationControlType::Humidistat\s*=>\s*"Humidistat"',
            'DehumidificationControlType::ConstantSupplyHumidityRatio\s*=>\s*"ConstantSupplyHumidityRatio"'
        )) {
        if ($Text -notmatch $mapping) {
            throw "CP348 symbolic selector JSON missing '$mapping'"
        }
    }
    if ($Text -match '(?i)dehumidification_control_type_(?:ordinal|discriminant|ieee_bits)|dehumidification_control_type\s+as\s+(?:usize|isize|u\d+|i\d+)|mixed_air_humidity_ratio|assigned_supply_humidity_ratio|resulting_supply') {
        throw "CP348 JSON admits ordinal or numerical case-body state"
    }
}

function Assert-Cp348MutationRejected {
    param(
        [Parameter(Mandatory = $true)][string]$Original,
        [Parameter(Mandatory = $true)][string]$Mutated,
        [Parameter(Mandatory = $true)][scriptblock]$Validator,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Original -ceq $Mutated) {
        throw "CP348 self-test mutation was not applied: $Description"
    }
    $rejected = $false
    try {
        & $Validator $Mutated
    } catch {
        $rejected = $true
    }
    if (-not $rejected) {
        throw "CP348 audit failed to reject mutation: $Description"
    }
}

foreach ($file in @(
        $cp348Module, $cp348State, $cp348Transition, $cp348Release,
        $cp348Prefix, $cp348Runtime, $cp348Snapshot, $cp348Tests,
        $cp348PublicTests, $cp348CorruptionTests, $cp348BindingAdapter,
        $cp348BindingTests, $cp348Coupled, $cp348CoupledFixture,
        $cp348Pipeline, $cp348PipelineValidation, $cp348Serialization,
        $cp348SnapshotSerialization, $cp348ArbitraryTests, $cp348CoupledTests
    )) {
    Assert-FileExists -Path $file -Description "CP348 structure"
}
Assert-LineLimit -Path $cp348Release -Limit 500 -Description "CP348 release module"
Assert-LineLimit -Path $cp348Runtime -Limit 450 -Description "CP348 runtime validation"
Assert-LineLimit -Path $cp348Coupled -Limit 500 -Description "CP348 coupled validation"
Assert-LineLimit -Path $cp348PipelineValidation -Limit 450 -Description "CP348 pipeline validation"
Assert-LineLimit -Path $cp348Audit -Limit 600 -Description "CP348 audit"

# Exact source boundary, seven-route partition, and direct complete skip.
Assert-Contains -Path $cp348Module -Pattern 'PurchasedAirManager\.cc:2213' -Description "CP348 exact case-entry source"
Assert-Contains -Path $cp348Module -Pattern 'PurchasedAirManager\.cc:2216' -Description "CP348 first excluded executable"
Assert-ExactStringArray -Path $cp348Module `
    -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER" `
    -Expected $cp348Sites `
    -Description "CP348 exact one-site source order"
Assert-Contains -Path $cp348State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseEntered,\s*DehumidificationControlHumidistatCaseSelectedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP348 seven routes"
Assert-Contains -Path $cp348Transition -Pattern '(?s)DehumidificationControlConstantSensibleHeatRatioCaseEntered\s*=>\s*\{.*?dehumidification_control_constant_sensible_heat_ratio_case_entry_count.*?source_site_execution_count.*?dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count' -Description "CP348 CSHR-only site"
Assert-NotContains -Path $cp348Transition -Pattern '(?i)Psy[A-Za-z0-9_]*\s*\(|let\s+(?:mixed_air_humidity_ratio|assigned_supply_humidity_ratio|resulting_supply_[A-Za-z0-9_]*)\b|f64::(?:min|max)|\.clamp\(|total_cmp|partial_cmp|is_finite\(\)|diagnostic|cache' -Description "CP348 no new operand or numerical work"

$cp348ReleaseText = Read-RepoText -Path $cp348Release
$cp348RuntimeText = Read-RepoText -Path $cp348Runtime
$cp348BindingText = Read-RepoText -Path $cp348Binding
$cp348PipelineRootText = Read-RepoText -Path $cp348PipelineRoot
$cp348SerializationText = Read-RepoText -Path $cp348SnapshotSerialization
Assert-Cp348ReleaseContract -Text $cp348ReleaseText
Assert-Cp348RuntimeContract -Text $cp348RuntimeText
Assert-Cp348BindingContract -Text $cp348BindingText
Assert-Cp348PipelineRootContract -Text $cp348PipelineRootText
Assert-Cp348SerializationContract -Text $cp348SerializationText
Assert-Contains -Path $cp348Prefix -Pattern 'case_entry_links_to_predecessor\s*\(' -Description "CP348 exact CP347 lineage"
Assert-Contains -Path $cp348Snapshot -Pattern '(?s)DehumidificationControlType::None.*?!snapshot\.dehumidification_control_constant_sensible_heat_ratio_case_entered.*?!snapshot\.dehumidification_control_humidistat_case_selected_skip' -Description "CP348 exact direct complete skip"
Assert-Contains -Path $cp348Release -Pattern 'DehumidificationControlTypeOutsideDirectSubset' -Description "CP348 direct None gate"
Assert-Contains -Path $cp348Tests -Pattern 'pure_transition_partitions_all_seven_routes_and_only_e_executes_the_site' -Description "CP348 private E characterization"
Assert-Contains -Path $cp348PublicTests -Pattern 'public_active_routes_complete_skip_the_constant_shr_entry_site' -Description "CP348 direct active complete skip"
Assert-Contains -Path $cp348PublicTests -Pattern 'public_u_n_p_routes_also_skip_the_case_entry_site' -Description "CP348 direct U/N/P skips"
Assert-Contains -Path $cp348CorruptionTests -Pattern 'supplied_retained_and_private_cp347_drift_are_rejected_transactionally' -Description "CP348 CP347 corruption rejection"
Assert-Contains -Path $cp348CorruptionTests -Pattern 'every_public_counter_increment_is_preflighted' -Description "CP348 public overflow preflight"
Assert-Contains -Path $cp348CorruptionTests -Pattern 'every_private_entry_counter_increment_is_preflighted' -Description "CP348 private E overflow preflight"
Assert-Contains -Path $cp348BindingTests -Pattern 'scheduled_binding_completes_cp348_direct_none_route_as_case_entry_skip' -Description "CP348 binding direct skip"
Assert-Contains -Path $cp348BindingTests -Pattern 'scheduled_binding_skips_cp348_source_site_on_u_n_and_p_routes' -Description "CP348 binding U/N/P skips"
Assert-Contains -Path $cp348CoupledTests -Pattern 'cp348_coupled_direct_none_route_is_complete_skip_and_corruption_fails_closed' -Description "CP348 coupled direct skip and corruption"
Assert-Contains -Path $cp348PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp386_lifecycle_evidence' -Description "CP348 cumulative non-direct firewall"
Assert-Contains -Path $cp348PipelineRoot -Pattern ('"' + $cp348Lifecycle + '":\s*result\s*\.' + $cp348Lifecycle) -Description "CP348 lifecycle JSON"
Assert-Contains -Path $cp348Coupled -Pattern 'direct_constant_sensible_heat_ratio_case_entry_count",\s*0' -Description "coupled direct E zero"
Assert-Contains -Path $cp348PipelineValidation -Pattern 'lifecycle_route_partition_corruption_fails_closed' -Description "pipeline CP348 partition corruption"
Assert-Contains -Path $cp348PipelineValidation -Pattern 'missing_cp347_predecessor_fails_closed' -Description "pipeline CP348 missing predecessor"
Assert-Contains -Path $cp348ArbitraryTests -Pattern 'ideal_loads_no_oa_branch_runs_declared_compatibility_runtime' -Description "arbitrary direct CP348 evidence"
Assert-Contains -Path $cp348ArbitraryTests -Pattern 'ideal_loads_fixture_demand_fallbacks_fail_closed_in_compatibility_mode' -Description "arbitrary non-direct CP348 firewall"
Assert-NotContains -Path $cp348Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled CP348 numerical firewall"
Assert-NotContains -Path $cp348PipelineValidation -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply' -Description "pipeline CP348 numerical firewall"

# Exactly two algorithm and two capability addenda and the 2+2+1+1 targets.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches(
    $algorithmText,
    '(?m)^\s*"CP348 supersedes only CP347[^"\r\n]+",\s*$'
)
$capabilityAddenda = [regex]::Matches(
    $capabilityText,
    '(?m)^\s*"CP348 additionally requires[^"\r\n]+",\s*$'
)
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP348 addenda"
}
$cp348Claims = @($algorithmAddenda | ForEach-Object { $_.Value }) +
    @($capabilityAddenda | ForEach-Object { $_.Value })
foreach ($claim in $cp348Claims) {
    foreach ($pattern in @(
            $cp348SourceHash,
            'physical line 2213',
            $cp348Sites[0],
            '2214-2215',
            'line 2216',
            'T=U\+N\+P\+C0\+E\+H\+CSH',
            'S=C0\+E\+H\+CSH=R=G\+F\+L',
            'A=F\+L',
            'source_site_execution_count=E',
            'C0=S',
            'E=H=CSH=0',
            'CP347.*?immediate',
            'no numerical operand or new numerical state',
            'CP347-to-CP348-to-unchanged-numerical',
            $cp348Lifecycle,
            'DirectZonePurchasedAirCouplingInput',
            '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped',
            'Roadmap (?:promotion|state)'
        )) {
        if ($claim -notmatch $pattern) {
            throw "CP348 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{
            Pattern = "$cp348Stem/release\.rs::advance_direct_no_oa_calc_$cp348Stem"
            Expected = 2
        },
        [PSCustomObject]@{
            Pattern = "$cp348Stem\.rs::purchased_air_calc_${cp348Stem}_lifecycle_summary"
            Expected = 2
        },
        [PSCustomObject]@{
            Pattern = "$cp348Stem\.rs::${cp348TypeStem}RuntimeState"
            Expected = 1
        },
        [PSCustomObject]@{
            Pattern = "$cp348Stem\.rs::${cp348TypeStem}LifecycleSummary"
            Expected = 1
        }
    )) {
    $count = [regex]::Matches($algorithmText, $target.Pattern).Count
    if ($count -ne $target.Expected) {
        throw "CP348 target '$($target.Pattern)' expected $($target.Expected), found $count"
    }
}

# Exactly five hand-authored sections carry the same non-promotion boundary.
$documentation = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP348 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP348 Source-Ordered Cooling Positive-Supply Constant-Sensible-Heat-Ratio Case Entry\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP348 Constant-Sensible-Heat-Ratio Case Entry\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP348 Constant-Sensible-Heat-Ratio Case Entry in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP348 Constant-Sensible-Heat-Ratio Case-Entry Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($doc in $documentation) {
    $matches = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($matches.Count -ne 1) {
        throw "CP348 documentation expected one scoped section in $($doc.Path)"
    }
    $section = $matches[0].Value
    foreach ($pattern in @(
            $cp348SourceHash,
            '2213',
            $cp348Sites[0],
            '2214-2215',
            '2216',
            'T\s*=\s*U\+N\+P\+C0\+E\+H\+CSH',
            'S\s*=\s*C0\+E\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L',
            'source_site_execution_count\s*=\s*E',
            'C0\s*=\s*S',
            'E\s*=\s*H\s*=\s*CSH\s*=\s*0',
            '(?s)CP347.*?(?:immediate|predecessor)',
            'CP347-to-CP348-to-unchanged-numerical',
            $cp348Lifecycle,
            'DirectZonePurchasedAirCouplingInput',
            '32\s+algorithms',
            '293\s+routines',
            'Roadmap'
        )) {
        if ($section -notmatch $pattern) {
            throw "CP348 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP348\b' -Description "CP348 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP348 supersedes only CP347' -Description "generated CP348 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP348 additionally requires' -Description "generated CP348 capability addendum"

# Historical binding whitelists and cumulative firewalls advance to CP348.
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
        "cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1"
    )) {
    Assert-Contains `
        -Path "scripts\quality\ideal-loads-structure-audit\$historical" `
        -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry' `
        -Description "historical CP348 binding whitelist"
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
        "cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1"
    )) {
    Assert-Contains `
        -Path "scripts\quality\ideal-loads-structure-audit\$historical" `
        -Pattern 'non_direct_runtime_rejects_cp316_through_cp386_lifecycle_evidence' `
        -Description "historical cumulative non-direct firewall"
}
$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp347AuditIndex = $mainAuditText.IndexOf(
    "cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1"
)
$cp348AuditIndex = $mainAuditText.IndexOf(
    "cp348-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-entry.ps1"
)
$completionIndex = $mainAuditText.IndexOf(
    'Write-Host "IdealLoads structure audit complete."'
)
if ($cp347AuditIndex -lt 0 -or $cp348AuditIndex -le $cp347AuditIndex -or $completionIndex -le $cp348AuditIndex) {
    throw "Main IdealLoads audit must dot-source CP348 after CP347 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 324' -Description "CP348 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP348 zero uncalled scripts"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp348-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-entry\.ps1"' -Description "CP348 internal record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp348-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-entry\.ps1::dot_sources' -Description "CP348 caller/callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 324 \|' -Description "generated CP348 script total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 84 \|' -Description "generated internal total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated zero uncalled"

# Mutation self-tests prove predecessor, algebra, DTO, direct-only, and
# symbolic-serialization regressions are rejected.
Assert-Cp348MutationRejected `
    -Original $cp348ReleaseText `
    -Mutated $cp348ReleaseText.Replace(
        'completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent(',
        'completed_cp347_proof_was_bypassed('
    ) `
    -Validator { param($text) Assert-Cp348ReleaseContract -Text $text } `
    -Description "CP347 recursive predecessor proof removal"
Assert-Cp348MutationRejected `
    -Original $cp348RuntimeText `
    -Mutated $cp348RuntimeText.Replace('.checked_mul(', '.checked_add(') `
    -Validator { param($text) Assert-Cp348RuntimeContract -Text $text } `
    -Description "one-site multiplier corruption"
Assert-Cp348MutationRejected `
    -Original $cp348BindingText `
    -Mutated $cp348BindingText.Replace(
        'DirectZonePurchasedAirCouplingInput {',
        "DirectZonePurchasedAirCouplingInput {`n            cp348_evidence: calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry,"
    ) `
    -Validator { param($text) Assert-Cp348BindingContract -Text $text } `
    -Description "CP348 evidence injection into numerical DTO"
$cp348NonDirectPattern =
    'result\s*\.\s*' + [regex]::Escape($cp348Lifecycle) + '\s*\.\s*is_some\s*\(\s*\)'
$cp348NonDirectMutation = [regex]::Replace(
    $cp348PipelineRootText,
    $cp348NonDirectPattern,
    ('result.' + $cp348Lifecycle + '.is_none()'),
    1
)
Assert-Cp348MutationRejected `
    -Original $cp348PipelineRootText `
    -Mutated $cp348NonDirectMutation `
    -Validator { param($text) Assert-Cp348PipelineRootContract -Text $text } `
    -Description "non-direct CP348 is_some firewall mutation"
Assert-Cp348MutationRejected `
    -Original $cp348SerializationText `
    -Mutated $cp348SerializationText.Replace(
        '"dehumidification_control_constant_sensible_heat_ratio_case_entered":',
        '"dehumidification_control_constant_sensible_heat_ratio_case_entered_mutated":'
    ) `
    -Validator { param($text) Assert-Cp348SerializationContract -Text $text } `
    -Description "CP348 JSON entry-field mutation"

Write-Host "CP348 constant-sensible-heat-ratio case-entry audit complete."
