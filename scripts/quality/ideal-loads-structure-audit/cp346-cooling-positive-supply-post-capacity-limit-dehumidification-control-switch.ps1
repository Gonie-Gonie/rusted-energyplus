# CP346 maps only PurchasedAirManager.cc physical line 2209:
# switch (PurchAir.DehumidCtrlType) {
# Line 2210 is the first excluded lexical construct; line 2211 is the first
# excluded executable.

$cp346Stem = "cooling_positive_supply_post_capacity_limit_dehumidification_control_switch"
$cp346PipelineStem = "purchased_air_$cp346Stem"
$cp346TypeStem = "PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitch"
$cp346Module = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem.rs"
$cp346State = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\state.rs"
$cp346Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\transition.rs"
$cp346Release = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\release.rs"
$cp346Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\release\prefix_validation.rs"
$cp346Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\release\runtime_validation.rs"
$cp346Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\release\snapshot_validation.rs"
$cp346Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\tests\mod.rs"
$cp346PublicTests = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\tests\public_release.rs"
$cp346CorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp346Stem\tests\release_corruption.rs"
$cp346Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp346Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp346BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp346Stem.rs"
$cp346BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp346Stem}_tests.rs"
$cp346Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$cp346Stem`_validation.rs"
$cp346CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp346PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp346Pipeline = "crates\ep_run\src\pipeline\$cp346PipelineStem.rs"
$cp346PipelineValidation = "crates\ep_run\src\pipeline\$cp346PipelineStem\validation.rs"
$cp346Serialization = "crates\ep_run\src\pipeline\$cp346PipelineStem\serialization\snapshot.rs"
$cp346ArbitraryTests = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp346Audit = "scripts\quality\ideal-loads-structure-audit\cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch.ps1"
$cp346Lifecycle = "purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle"
$cp346SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp346SourcePattern = 'switch\s*\(\s*PurchAir\.DehumidCtrlType\s*\)\s*\{'
$cp346LexicalExcludedPattern = 'case\s+HumControl::None\s*:\s*\{'
$cp346ExecutableExcludedPattern =
    'PurchAir\.SupplyHumRat\s*=\s*PurchAir\.MixedAirHumRat\s*;'

function Get-Cp346RustBraceBlock {
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

function Assert-Cp346ReleaseContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $body = Get-Cp346RustBraceBlock `
        -Text $Text `
        -AnchorPattern ('(?m)^\s*pub fn advance_direct_no_oa_calc_' + [regex]::Escape($cp346Stem) + '\s*\(') `
        -Description "CP346 release body"
    foreach ($pattern in @(
            'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None',
            'predecessor_snapshots_match_bit_exact\(',
            'predecessor_is_exact_direct\(',
            'active_cp319_corroborates_owner\(',
            'completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent\(',
            'pending_state_is_consistent\(',
            'next_transition_fits\('
        )) {
        if ($body -notmatch $pattern) {
            throw "CP346 release proof missing '$pattern'"
        }
    }
    $lastProof = $body.LastIndexOf("next_transition_fits(")
    $firstMutation = $body.IndexOf("runtime.units.get_mut(")
    if ($lastProof -lt 0 -or $firstMutation -le $lastProof) {
        throw "CP346 release must validate owner, lineage, state, and overflow before mutation"
    }
    if ($body -match '(?i)DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply|psychrometric_service|diagnostic') {
        throw "CP346 release admits a numerical DTO or forbidden service"
    }
}

function Assert-Cp346PrefixContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $body = Get-Cp346RustBraceBlock `
        -Text $Text `
        -AnchorPattern '(?m)^\s*pub\(super\) fn active_cp319_corroborates_owner\s*\(' `
        -Description "CP346 active CP319 corroboration"
    foreach ($pattern in @(
            '(?s)if\s+!predecessor_is_active\(predecessor\)\s*\{.*?return true;',
            'cp319\s*==\s*cp319_witness',
            'cp319\.dehumidification_control_type_read',
            'cp319\.dehumidification_control_type\s*==\s*Some\(owner\)',
            'cooling_dehumidification_flow_snapshot_is_exact_direct_release\(cp319\)',
            'completed_direct_cooling_dehumidification_flow_is_consistent\('
        )) {
        if ($body -notmatch $pattern) {
            throw "CP346 active-only CP319 corroboration missing '$pattern'"
        }
    }
}

function Assert-Cp346RuntimeContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    foreach ($pattern in @(
            '(?s)unit_off_skip_count.*?checked_add\(state\.non_cooling_skip_count\).*?checked_add\(state\.positive_guard_false_fallthrough_skip_count\).*?checked_add\(state\.dehumidification_control_switch_count\)',
            '(?s)dehumidification_control_none_case_selection_count.*?checked_add\(\s*state\.dehumidification_control_constant_sensible_heat_ratio_case_selection_count.*?checked_add\(state\.dehumidification_control_humidistat_case_selection_count\).*?checked_add\(\s*state\.dehumidification_control_constant_supply_humidity_ratio_case_selection_count',
            'dehumidification_control_switch_count\s*\.checked_mul\(',
            'route_partition\s*==\s*state\.transition_count',
            'case_partition\s*==\s*active',
            'dehumidification_control_type_read_count\s*==\s*active',
            'dehumidification_control_switch_dispatch_count\s*==\s*active',
            'witnessed_dehumidification_control_none_case_selection_count',
            'witnessed_dehumidification_control_constant_sensible_heat_ratio_case_selection_count',
            'witnessed_dehumidification_control_humidistat_case_selection_count',
            'witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selection_count'
        )) {
        if ($Text -notmatch $pattern) {
            throw "CP346 checked algebra or witnessed parity missing '$pattern'"
        }
    }
}

function Assert-Cp346SerializationContract {
    param([Parameter(Mandatory = $true)][string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    $production = if ($boundary.Success) {
        $Text.Substring(0, $boundary.Index)
    } else {
        $Text
    }
    foreach ($mapping in @(
            'DehumidificationControlType::None\s*=>\s*"None"',
            'DehumidificationControlType::ConstantSensibleHeatRatio\s*=>\s*"ConstantSensibleHeatRatio"',
            'DehumidificationControlType::Humidistat\s*=>\s*"Humidistat"',
            'DehumidificationControlType::ConstantSupplyHumidityRatio\s*=>\s*"ConstantSupplyHumidityRatio"'
        )) {
        if ($production -notmatch $mapping) {
            throw "CP346 symbolic JSON mapping missing '$mapping'"
        }
    }
    if ($production -match '(?i)dehumidification_control_type_(?:ordinal|discriminant|ieee_bits)|dehumidification_control_type\s+as\s+(?:usize|isize|u\d+|i\d+)') {
        throw "CP346 production JSON must not serialize selector ordinals or IEEE bits"
    }
}

function Assert-Cp346MutationRejected {
    param(
        [Parameter(Mandatory = $true)][string]$Original,
        [Parameter(Mandatory = $true)][string]$Mutated,
        [Parameter(Mandatory = $true)][scriptblock]$Validator,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Original -ceq $Mutated) {
        throw "CP346 self-test mutation was not applied: $Description"
    }
    $rejected = $false
    try {
        & $Validator $Mutated
    } catch {
        $rejected = $true
    }
    if (-not $rejected) {
        throw "CP346 audit failed to reject mutation: $Description"
    }
}

foreach ($file in @(
        $cp346Module, $cp346State, $cp346Transition, $cp346Release,
        $cp346Prefix, $cp346Runtime, $cp346Snapshot, $cp346Tests,
        $cp346PublicTests, $cp346CorruptionTests, $cp346BindingAdapter,
        $cp346BindingTests, $cp346Coupled, $cp346Pipeline,
        $cp346PipelineValidation, $cp346Serialization, $cp346ArbitraryTests
    )) {
    Assert-FileExists -Path $file -Description "CP346 structure"
}
Assert-LineLimit -Path $cp346Release -Limit 450 -Description "CP346 release module"
Assert-LineLimit -Path $cp346Runtime -Limit 400 -Description "CP346 runtime validation"
Assert-LineLimit -Path $cp346Coupled -Limit 500 -Description "CP346 coupled validation"
Assert-LineLimit -Path $cp346Audit -Limit 600 -Description "CP346 audit"

# Exact source boundary and named C++ case-order characterization.
Assert-Contains -Path $cp346Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2209' -Description "CP346 physical source"
Assert-Contains -Path $cp346Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2211' -Description "CP346 first excluded executable"
Assert-ExactStringArray -Path $cp346Module `
    -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER" `
    -Expected @(
        "read-purchased-air-dehumidification-control-type",
        "dispatch-dehumidification-control-switch"
    ) `
    -Description "CP346 exact two-site source order"
Assert-Contains -Path $cp346State -Pattern '(?s)DehumidificationControlNoneCaseSelected,\s*DehumidificationControlConstantSensibleHeatRatioCaseSelected,\s*DehumidificationControlHumidistatCaseSelected,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelected' -Description "CP346 C++ case order"
Assert-PatternsInOrder -Path $cp346Transition `
    -Patterns @(
        'Some\(DehumidificationControlType::None\)',
        'Some\(DehumidificationControlType::ConstantSensibleHeatRatio\)',
        'Some\(DehumidificationControlType::Humidistat\)',
        'Some\(DehumidificationControlType::ConstantSupplyHumidityRatio\)'
    ) `
    -Description "CP346 named variant dispatch"
Assert-NotContains -Path $cp346Transition -Pattern '(?i)dehumidification_control_type_(?:ordinal|discriminant)|dehumidification_control_type\s+as\s+(?:usize|isize|u\d+|i\d+)|discriminant\s*\(\s*dehumidification_control_type' -Description "CP346 transition ordinal mapping"

$cp346ReleaseText = Read-RepoText -Path $cp346Release
$cp346PrefixText = Read-RepoText -Path $cp346Prefix
$cp346RuntimeText = Read-RepoText -Path $cp346Runtime
$cp346SerializationText = Read-RepoText -Path $cp346Serialization
Assert-Cp346ReleaseContract -Text $cp346ReleaseText
Assert-Cp346PrefixContract -Text $cp346PrefixText
Assert-Cp346RuntimeContract -Text $cp346RuntimeText
Assert-Cp346SerializationContract -Text $cp346SerializationText
Assert-Contains -Path $cp346Snapshot -Pattern '(?s)dehumidification_control_type\s*==\s*Some\(DehumidificationControlType::None\)' -Description "CP346 exact direct None snapshot"
Assert-Contains -Path $cp346Release -Pattern 'DehumidificationControlTypeOutsideDirectSubset' -Description "CP346 non-None direct error"
Assert-Contains -Path $cp346CorruptionTests -Pattern 'typed_owner_mutation_is_rejected_transactionally' -Description "CP346 non-None direct rejection"
Assert-Contains -Path $cp346Release -Pattern 'DehumidificationControlTypeLineageMismatch' -Description "CP346 CP319 lineage error"
Assert-Contains -Path $cp346CorruptionTests -Pattern 'active_cp319_selector_corruption_is_rejected_transactionally' -Description "CP346 CP319 late-mutation rejection"
Assert-Contains -Path $cp346Tests -Pattern 'every_typed_selector_maps_by_name_to_its_case_without_discriminant_coupling' -Description "CP346 all-mode characterization"

# Binding, coupled runtime, and pipeline retain CP345 -> CP346 -> numerical order.
$cp346BindingText = Read-RepoText -Path $cp346Binding
$cp345Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment ="
)
$cp346Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch ="
)
$cp347Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case ="
)
$cp348Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry ="
)
$cp349Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment ="
)
$cp350Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment ="
)
$cp351Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment ="
)
$cp352Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment ="
)
$cp353Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit ="
)
$cp354Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit ="
)
$cp355Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit ="
)
$cp356Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit ="
)
$cp357Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_constant_shr_case_break ="
)
$cp358Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_humidistat_case_entry ="
)
$cp359Call = $cp346BindingText.IndexOf(
    "let calculation_cooling_humidistat_moisture_demand_assignment ="
)
$numericalCall = $cp346BindingText.IndexOf(
    "let coupling = complete_direct_zone_purchased_air_coupling("
)
if (
    $cp345Call -lt 0 -or
    $cp346Call -le $cp345Call -or
    $cp347Call -le $cp346Call -or
    $cp348Call -le $cp347Call -or
    $cp349Call -le $cp348Call -or
    $cp350Call -le $cp349Call -or
    $cp351Call -le $cp350Call -or
    $cp352Call -le $cp351Call -or
    $cp353Call -le $cp352Call -or
    $cp354Call -le $cp353Call -or
    $cp355Call -le $cp354Call -or
    $cp356Call -le $cp355Call -or
    $cp357Call -le $cp356Call -or
    $cp358Call -le $cp357Call -or
    $cp359Call -le $cp358Call -or
    $numericalCall -le $cp359Call
) {
    throw "Binding must execute CP345 then CP346 then CP347 then CP348 then CP349 then CP350 then CP351 then CP352 then CP353 then CP354 then CP355 then CP356 then CP357 then CP358 then CP359 before numerical coupling"
}
$dto = Get-Cp346RustBraceBlock `
    -Text $cp346BindingText.Substring($numericalCall) `
    -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' `
    -Description "CP346 numerical DTO"
if ($dto -match '(?i)cp346|dehumidification_control_switch') {
    throw "CP346 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-NotContains -Path $cp346Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled CP346 numerical firewall"
Assert-Contains -Path $cp346Coupled -Pattern 'direct_none_case_selection_count' -Description "coupled direct None selection"
Assert-Contains -Path $cp346CoupledTests -Pattern 'cp347_direct_coupled_runtime_completes_none_case_after_g_f_l_and_skips_unit_off' -Description "coupled CP346/CP347 G/F/L/U regression"
Assert-Contains -Path $cp346CoupledTests -Pattern 'cp347_direct_coupled_runtime_covers_non_cooling_and_positive_guard_false_skips' -Description "coupled CP346/CP347 N/P regression"
Assert-Contains -Path $cp346PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp379_lifecycle_evidence' -Description "pipeline cumulative non-direct firewall"
Assert-Contains -Path $cp346PipelineRoot -Pattern ('"' + $cp346Lifecycle + '":\s*result\s*\.' + $cp346Lifecycle) -Description "pipeline CP346 lifecycle JSON"
Assert-Contains -Path $cp346PipelineValidation -Pattern 'post_capacity_assignment_cp345' -Description "pipeline CP345 predecessor"
Assert-Contains -Path $cp346PipelineValidation -Pattern 'dehumidification_flow_cp319' -Description "pipeline CP319 corroboration"
Assert-Contains -Path $cp346ArbitraryTests -Pattern 'same-call CP319 None selector is corroboration, not CP346 operand ownership' -Description "arbitrary CP319 owner regression"

# Exactly two algorithm and two capability addenda; target inventory only.
$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmAddenda = [regex]::Matches(
    $algorithmText,
    '(?m)^\s*"CP346 supersedes only CP345[^"\r\n]+",\s*$'
)
$capabilityAddenda = [regex]::Matches(
    $capabilityText,
    '(?m)^\s*"CP346 additionally requires[^"\r\n]+",\s*$'
)
if ($algorithmAddenda.Count -ne 2 -or $capabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP346 addenda"
}
$cp346SpecClaims = @($algorithmAddenda | ForEach-Object { $_.Value }) +
    @($capabilityAddenda | ForEach-Object { $_.Value })
foreach ($claim in $cp346SpecClaims) {
    foreach ($pattern in @(
            $cp346SourceHash,
            $cp346SourcePattern,
            $cp346LexicalExcludedPattern,
            $cp346ExecutableExcludedPattern,
            'active (?:G/F/L|CP346-active G/F/L) routes',
            'T=U\+N\+P\+S',
            'S=D0\+DSHR\+DH\+DCSH=R=G\+F\+L',
            'A=F\+L',
            'source_site_execution_count=2\*S',
            'CP345-to-CP346-to-unchanged-numerical',
            '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped',
            'Roadmap (?:promotion|state)'
        )) {
        if ($claim -notmatch $pattern) {
            throw "CP346 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{
            Pattern = "$cp346Stem/release\.rs::advance_direct_no_oa_calc_$cp346Stem"
            Expected = 2
        },
        [PSCustomObject]@{
            Pattern = "$cp346Stem\.rs::purchased_air_calc_${cp346Stem}_lifecycle_summary"
            Expected = 2
        },
        [PSCustomObject]@{
            Pattern = "$cp346Stem\.rs::${cp346TypeStem}RuntimeState"
            Expected = 1
        },
        [PSCustomObject]@{
            Pattern = "$cp346Stem\.rs::${cp346TypeStem}LifecycleSummary"
            Expected = 1
        }
    )) {
    $count = [regex]::Matches($algorithmText, $target.Pattern).Count
    if ($count -ne $target.Expected) {
        throw "CP346 target '$($target.Pattern)' expected $($target.Expected), found $count"
    }
}

# Exactly five hand-authored sections carry the same bounded contract.
$documentation = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP346 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP346 Source-Ordered Cooling Positive-Supply Post-Capacity-Limit Dehumidification-Control Switch\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP346 Cooling Positive-Supply Post-Capacity-Limit Dehumidification-Control Switch\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP346 Dehumidification-Control Switch in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP346 Cooling Positive-Supply Dehumidification-Control Switch Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($doc in $documentation) {
    $sectionMatches = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sectionMatches.Count -ne 1) {
        throw "CP346 documentation expected one scoped section in $($doc.Path)"
    }
    $sectionText = $sectionMatches[0].Value
    foreach ($pattern in @(
            $cp346SourceHash,
            $cp346SourcePattern,
            $cp346LexicalExcludedPattern,
            $cp346ExecutableExcludedPattern,
            'active (?:G/F/L|CP346-active G/F/L) routes',
            'T\s*=\s*U\+N\+P\+S',
            'S\s*=\s*D0\+DSHR\+DH\+DCSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L',
            '2\*S',
            'CP345-to-CP346-to-unchanged-numerical',
            '32\s+algorithms',
            '293\s+routines',
            'Roadmap'
        )) {
        if ($sectionText -notmatch $pattern) {
            throw "CP346 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP346\b' -Description "CP346 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP346 supersedes only CP345' -Description "generated CP346 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP346 additionally requires' -Description "generated CP346 capability addendum"

# Historical whitelists and cumulative firewalls all reach CP346.
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
        "cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
    )) {
    Assert-Contains `
        -Path "scripts\quality\ideal-loads-structure-audit\$historical" `
        -Pattern 'calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch' `
        -Description "historical CP346 binding whitelist"
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
        "cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
    )) {
    Assert-Contains `
        -Path "scripts\quality\ideal-loads-structure-audit\$historical" `
        -Pattern 'non_direct_runtime_rejects_cp316_through_cp379_lifecycle_evidence' `
        -Description "historical cumulative non-direct firewall through CP352"
}

$mainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp345AuditIndex = $mainAuditText.IndexOf(
    "cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
)
$cp346AuditIndex = $mainAuditText.IndexOf(
    "cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch.ps1"
)
$auditCompletionIndex = $mainAuditText.IndexOf(
    'Write-Host "IdealLoads structure audit complete."'
)
if ($cp345AuditIndex -lt 0 -or $cp346AuditIndex -le $cp345AuditIndex -or $auditCompletionIndex -le $cp346AuditIndex) {
    throw "Main IdealLoads audit must dot-source CP346 after CP345 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 317' -Description "CP346 script inventory total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP346 zero uncalled scripts"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch\.ps1"' -Description "CP346 internal script record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp346-cooling-positive-supply-post-capacity-limit-dehumidification-control-switch\.ps1::dot_sources' -Description "CP346 caller/callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 317 \|' -Description "generated CP346 script total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public script total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 77 \|' -Description "generated internal script total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated zero uncalled scripts"

# Mutation self-tests prove the audit rejects representative boundary escapes.
Assert-Cp346MutationRejected `
    -Original $cp346ReleaseText `
    -Mutated $cp346ReleaseText.Replace(
        'system.dehumidification_control_type != DehumidificationControlType::None',
        'false'
    ) `
    -Validator { param($text) Assert-Cp346ReleaseContract -Text $text } `
    -Description "direct None gate removal"
Assert-Cp346MutationRejected `
    -Original $cp346PrefixText `
    -Mutated $cp346PrefixText.Replace(
        'if !predecessor_is_active(predecessor) {',
        'if false {'
    ) `
    -Validator { param($text) Assert-Cp346PrefixContract -Text $text } `
    -Description "active-only CP319 guard removal"
Assert-Cp346MutationRejected `
    -Original $cp346RuntimeText `
    -Mutated $cp346RuntimeText.Replace(
        '&& case_partition == active',
        '&& case_partition == state.transition_count'
    ) `
    -Validator { param($text) Assert-Cp346RuntimeContract -Text $text } `
    -Description "case partition corruption"
Assert-Cp346MutationRejected `
    -Original $cp346SerializationText `
    -Mutated $cp346SerializationText.Replace(
        'DehumidificationControlType::Humidistat => "Humidistat"',
        'DehumidificationControlType::Humidistat => "ConstantSupplyHumidityRatio"'
    ) `
    -Validator { param($text) Assert-Cp346SerializationContract -Text $text } `
    -Description "symbolic Humidistat JSON corruption"

Write-Host "CP346 post-capacity dehumidification-control switch audit complete."
