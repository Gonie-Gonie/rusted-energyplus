# CP419 maps PurchasedAirManager.cc physical executable line 2330's not-dehumidifying `CpAir` assignment.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignment'
$predecessorTypeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntry'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$predecessorModule = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem.rs"
$predecessorRoot = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem"
$predecessorRelease = "$predecessorRoot\release.rs"
$predecessorTests = "$predecessorRoot\tests.rs"
$ownerModule = 'crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call.rs'
$ownerRelease = 'crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\release.rs'
$ownerTests = 'crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\tests.rs'
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$transitionAccounting = "$root\transition\accounting.rs"
$tests = "$root\tests.rs"
$edgeTests = "$root\tests\edge_cases.rs"
$overflowTests = "$root\tests\overflow.rs"
$release = "$root\release.rs"
$releaseError = "$root\release\error.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$releaseSnapshot = "$root\release\snapshot.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp419.rs'
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineValidationTests = "crates\ep_run\src\pipeline\$pipelineStem\validation\tests.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serializationRoot = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp419_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp418_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp419-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air-assignment.ps1'

function Assert-Cp419Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP419 $Description missing '$Pattern'" }
}

function Get-Cp419BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP419 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP419 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP419 $Description closing brace missing"
}

$required = @(
    $source,$module,$predecessorModule,$predecessorRelease,$predecessorTests,$ownerModule,$ownerRelease,$ownerTests,$state,$transition,$transitionAccounting,$tests,$edgeTests,$overflowTests,$release,$releaseError,$runtimeValidation,$releaseSnapshot,
    $binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipelineRoot,$pipeline,$pipelineValidation,$pipelineValidationTests,$pipelineLineage,
    $serializationRoot,$serialization,$snapshotJsonTests,$arbitrary,$arbitraryPredecessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP419 implementation/audit file' }
foreach ($file in @($module,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipeline,$pipelineValidation,$pipelineLineage,$serializationRoot,
    $serialization,$snapshotJsonTests,$arbitrary,$audit)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP419 file'
}
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
if ($coreFiles.Count -ne 13) { throw 'CP419 exact thirteen-file bounded core subtree drift' }
foreach ($file in $coreFiles) { Assert-LineLimit -Path $file.FullName -Limit 500 -Description 'bounded CP419 core file' }

$registrations = @(
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\calc.rs'; Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs'; Pattern="mod\s+$stem" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs'; Pattern="$($stem)_tests" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs'; Pattern="mod\s+$($stem)_validation" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs'; Pattern='test_coupled_runtime_cp419\.rs' },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs'; Pattern="$($stem)_fixture" },
    [PSCustomObject]@{ Path=$module; Pattern='(?s)mod\s+release;.*?mod\s+state;.*?mod\s+tests;.*?mod\s+transition;' },
    [PSCustomObject]@{ Path=$release; Pattern='(?s)mod\s+error;.*?mod\s+runtime_validation;.*?mod\s+snapshot;' },
    [PSCustomObject]@{ Path=$transition; Pattern='mod\s+accounting;' },
    [PSCustomObject]@{ Path=$pipelineRoot; Pattern="mod\s+$pipelineStem;" },
    [PSCustomObject]@{ Path=$pipeline; Pattern='(?s)mod\s+serialization;.*?mod\s+validation;' },
    [PSCustomObject]@{ Path=$pipelineValidation; Pattern='mod\s+lineage;' },
    [PSCustomObject]@{ Path=$serializationRoot; Pattern='mod\s+snapshot;' },
    [PSCustomObject]@{ Path=$serialization; Pattern='mod\s+tests;' }
)
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP419 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP419 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
$boundary = @(
    [PSCustomObject]@{ Line=2327; Text='} else { // Not dehumidifying' },
    [PSCustomObject]@{ Line=2328; Text='// If not dehumidifying, compare sensible cooling to the limit' },
    [PSCustomObject]@{ Line=2329; Text='// This section will only increase supply temp, so no need to recheck for super-saturation' },
    [PSCustomObject]@{ Line=2330; Text='CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat);' },
    [PSCustomObject]@{ Line=2331; Text='CoolSensOutput = SupplyMassFlowRate * CpAir * (PurchAir.MixedAirTemp - PurchAir.SupplyTemp);' }
)
foreach ($item in $boundary) {
    if ($sourceLines[$item.Line - 1].Trim() -cne $item.Text) { throw "CP419 source boundary drift at physical line $($item.Line)" }
}

$moduleText = Read-RepoText -Path $module
Assert-Cp419Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2330' -Description 'source constant'
Assert-Cp419Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2331' -Description 'first excluded executable constant'
$orderMatch = [regex]::Match($moduleText, '(?s)GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP419 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSites = @(
    'read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air',
    'evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air',
    'assign-local-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch'
)
if (($sites -join '|') -cne ($expectedSites -join '|')) { throw 'CP419 exact three-site source order drift' }

$snapshotStruct = Get-Cp419BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$predecessorText = Read-RepoText -Path $predecessorModule
$predecessorStruct = Get-Cp419BraceBlock -Text $predecessorText -AnchorPattern "pub\s+struct\s+$($predecessorTypeStem)Snapshot\s*" -Description 'CP418 snapshot'
$predecessorFields = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$predecessorMarker = 'predecessor_post_saturation_capacity_limit_dehumidification_guard_else_branch_entered'
$localFields = @(
    'post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed',
    'cp329_retained_mixed_air_humidity_ratio_owned_read','mixed_air_humidity_ratio_for_cp_air_read',
    'mixed_air_humidity_ratio_for_cp_air','psychrometric_cp_air_evaluated',
    'psychrometric_cp_air_result_j_per_kg_k','cp_air_assigned','cp_air_j_per_kg_k'
)
$terminal = @('resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c')
$predecessorTerminal = @('predecessor_cp418_resulting_supply_humidity_ratio','predecessor_cp418_resulting_supply_enthalpy_j_per_kg','predecessor_cp418_resulting_supply_temperature_c')
$expectedFields = @($predecessorFields[0..158]) + $predecessorTerminal + $predecessorMarker + $localFields + $terminal
if ($predecessorFields.Count -ne 163 -or $fields.Count -ne 174 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP419 snapshot must retain CP418 fields 0..158, append the exact predecessor/local block, and re-emit final W/H/T' }
$predecessorNumeric = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$expectedNumeric = @($predecessorNumeric[0..50]) + $predecessorTerminal + @('mixed_air_humidity_ratio_for_cp_air','psychrometric_cp_air_result_j_per_kg_k','cp_air_j_per_kg_k') + $terminal
if ($predecessorNumeric.Count -ne 54 -or $numericFields.Count -ne 60 -or ($numericFields -join '|') -cne ($expectedNumeric -join '|')) { throw 'CP419 exact sixty numeric-carrier schema drift' }
if ([regex]::Matches($snapshotStruct, 'Option<bool>').Count -ne 1 -or [regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP419 optional comparison/control carrier drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @(
    'predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts','predecessor_guard_body_entry_route_counts',
    'predecessor_supply_temperature_saturation_assignment_route_counts','predecessor_supply_temperature_mixed_air_limit_route_counts',
    'predecessor_supply_humidity_ratio_assignment_route_counts','predecessor_supply_enthalpy_assignment_route_counts',
    'predecessor_dehumidification_guard_else_branch_entry_route_counts','dehumidification_guard_else_branch_cp_air_assignment_route_counts'
)
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP419 exact nine width-36 route arrays drift' }
$expectedCounters = @(
    'transition_count','inactive_transition_count','predecessor_supply_temperature_saturation_assignment_count',
    'predecessor_supply_temperature_saturation_mixed_air_limit_count','predecessor_supply_humidity_ratio_assignment_count',
    'predecessor_supply_enthalpy_assignment_count','predecessor_dehumidification_guard_else_branch_entry_count',
    'dehumidification_guard_else_branch_cp_air_assignment_count','source_site_execution_count',
    'cp418_supply_humidity_ratio_state_owner_count','unchanged_supply_humidity_ratio_preservation_count',
    'cp418_supply_enthalpy_state_owner_count','unchanged_supply_enthalpy_preservation_count',
    'cp418_supply_temperature_state_owner_count','unchanged_supply_temperature_preservation_count',
    'cp419_psychrometric_cp_air_state_owner_count','cp329_retained_mixed_air_humidity_ratio_owned_read_count',
    'mixed_air_humidity_ratio_for_cp_air_read_count','psychrometric_cp_air_evaluation_count','cp_air_assignment_write_count'
)
$counterFields = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*usize') | ForEach-Object { $_.Groups['name'].Value })
if (($counterFields -join '|') -cne ($expectedCounters -join '|')) { throw 'CP419 exact runtime counter set drift' }

$transitionText = Read-RepoText -Path $transition
$accountingText = Read-RepoText -Path $transitionAccounting
foreach ($pattern in @(
    'GuardElseBranchEntrySnapshot as Predecessor','predecessor_post_saturation_capacity_limit_dehumidification_guard_else_branch_entered',
    'active:\s*predecessor\s*\.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered',
    'post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed:\s*route\.active',
    'cp329_retained_mixed_air_humidity_ratio_owned_read:\s*route\.active',
    'psychrometric_cp_air_result_j_per_kg_k:\s*prepared\.cp_air_j_per_kg_k',
    'cp_air_assigned:\s*route\.active','cp_air_j_per_kg_k:\s*prepared\.cp_air_j_per_kg_k',
    'resulting_supply_humidity_ratio:\s*predecessor\.resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg:\s*predecessor\.resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c:\s*predecessor\.resulting_supply_temperature_c'
    'energyplus_psy_cp_air_fn_w\(mixed_air_humidity_ratio\)'
)) { Assert-Cp419Text -Text $transitionText -Pattern "(?s)$pattern" -Description 'not-dehumidifying CpAir-assignment transition contract' }
foreach ($pattern in @('dehumidification_guard_else_branch_cp_air_assignment_count\s*\+=\s*1','dehumidification_guard_else_branch_cp_air_assignment_route_counts\[index\]\s*\+=\s*1','source_site_execution_count\s*\+=\s*3','cp419_psychrometric_cp_air_state_owner_count\s*\+=\s*1')) {
    Assert-Cp419Text -Text $accountingText -Pattern $pattern -Description 'CP419 route accounting'
}
foreach ($file in @($transition,$transitionAccounting,$release,$releaseError,$runtimeValidation,$releaseSnapshot,$adapter,$coupled,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}
Assert-NotContains -Path $transition -Pattern '(?i)(?:fast|raw)_psy_cp_air|dwSave|cpaSave|mul_add\s*\(|clamp\s*\(|warning|diagnostic|PsychrometricService' -Description 'noncanonical psychrometric substitution'

$predecessorModuleText = Read-RepoText -Path $predecessorModule
$predecessorReleaseText = Read-RepoText -Path $predecessorRelease
Assert-Cp419Text -Text $predecessorModuleText -Pattern 'pub\(in crate::ideal_loads::calc\) use release::\{(?s:.*?)cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_committed_latest_route' -Description 'calc-private sealed CP418 route accessor re-export'
Assert-Cp419Text -Text $predecessorModuleText -Pattern 'RetainedRoute as PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryCommittedRoute' -Description 'calc-private sealed CP418 route type'
$committedAccessor = Get-Cp419BraceBlock -Text $predecessorReleaseText -AnchorPattern 'pub\(in crate::ideal_loads::calc\) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_committed_latest_route\s*\(' -Description 'sealed CP418 committed-route accessor'
foreach ($pattern in @('state\.latest_route\?','committed_route_counts_match\s*\(','snapshot_matches_validated_predecessor\s*\(')) { Assert-Cp419Text -Text $committedAccessor -Pattern $pattern -Description 'sealed CP418 committed-route proof' }
$ownerModuleText = Read-RepoText -Path $ownerModule
$ownerReleaseText = Read-RepoText -Path $ownerRelease
Assert-Cp419Text -Text $ownerModuleText -Pattern 'cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio' -Description 'calc-private sealed CP329 owner re-export'
$ownerAccessor = Get-Cp419BraceBlock -Text $ownerReleaseText -AnchorPattern 'pub\(in crate::ideal_loads::calc\) fn cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio\s*\(' -Description 'sealed CP329 humidity owner accessor'
foreach ($pattern in @('committed_no_oa_humidity_owner_state_is_consistent\s*\(','committed_no_oa_humidity_owner_snapshot_has_exact_shape\s*\(','retained\.to_bits\(\)\s*==\s*source\.to_bits\(\)')) { Assert-Cp419Text -Text $ownerAccessor -Pattern $pattern -Description 'sealed CP329 humidity owner proof' }
foreach ($pattern in @('cooling_mixed_air_call_snapshot_is_exact_direct_release\s*\(','completed_direct_cooling_mixed_air_call_is_consistent\s*\(','(?<![A-Za-z0-9_])predecessor_route\s*\(')) { if ($ownerAccessor -match $pattern) { throw "CP419 sealed CP329 owner recursively validates through '$pattern'" } }

$releaseText = Read-RepoText -Path $release
$hotRelease = Get-Cp419BraceBlock -Text $releaseText -AnchorPattern 'pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment\s*\(' -Description 'CP419 public hot release'
foreach ($pattern in @(
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_committed_latest_route\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_route_from_validated_predecessor\s*\(',
    'advance_with_validated_route\s*\('
)) { Assert-Cp419Text -Text $hotRelease -Pattern $pattern -Description 'non-recursive CP419 hot release' }
foreach ($pattern in @(
    '(?<![A-Za-z0-9_])predecessor_route\s*\(','private_[a-z0-9_]*characterization\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact_direct_release\s*\('
)) { if ($hotRelease -match $pattern) { throw "CP419 public hot release recursively re-derived exact route via '$pattern'" } }
foreach ($pattern in @('mixed_air_humidity_ratio\.is_finite\(\)','mixed_air_humidity_ratio\s*<\s*0\.0','cp_air\.is_finite\(\)')) { Assert-Cp419Text -Text $hotRelease -Pattern $pattern -Description 'direct finite/range admission' }
Assert-Cp419Text -Text $transitionText -Pattern 'let cp_air_j_per_kg_k\s*=\s*energyplus_psy_cp_air_fn_w\(mixed_air_humidity_ratio\);' -Description 'generic canonical raw evaluator'
Assert-NotContains -Path $transition -Pattern 'mixed_air_humidity_ratio\.is_finite\(\)|mixed_air_humidity_ratio\s*<\s*0\.0|cp_air_j_per_kg_k\.is_finite\(\)' -Description 'generic transition direct-only finite rejection'

$coupledText = Read-RepoText -Path $coupled
Assert-Cp419Text -Text $coupledText -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_has_exact_cp418_prefix_and_local_assignment\s*\(' -Description 'cheap coupled CP418-prefix/local validation'
foreach ($pattern in @(
    '(?<![A-Za-z0-9_])predecessor_route\s*\(','private_[a-z0-9_]*characterization\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact_direct_release\s*\('
)) { if ($coupledText -match $pattern) { throw "CP419 coupled validator recursively re-derived exact route via '$pattern'" } }

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine +
    (Read-RepoText -Path $edgeTests) + [Environment]::NewLine +
    (Read-RepoText -Path $overflowTests) + [Environment]::NewLine +
    (Read-RepoText -Path $predecessorTests) + [Environment]::NewLine +
    (Read-RepoText -Path $ownerTests) + [Environment]::NewLine +
    (Read-RepoText -Path $adapterTests) + [Environment]::NewLine +
    (Read-RepoText -Path $coupledTests) + [Environment]::NewLine +
    (Read-RepoText -Path $pipelineValidation) + [Environment]::NewLine +
    (Read-RepoText -Path $pipelineValidationTests) + [Environment]::NewLine +
    (Read-RepoText -Path $snapshotJsonTests) + [Environment]::NewLine +
    (Read-RepoText -Path $arbitrary)
foreach ($pattern in @(
    'cp419_boundary_and_three_sites_are_exact','exhaustive_54_outcomes_49_inactive_five_assignments_and_nine_arrays_are_exact',
    'outer_guard_false_is_distinct_from_later_saturation_guard_false','marker_and_predecessor_forgery_are_rejected',
    'entry_counter_overflow_is_transactional','cp419_binding_contract_is_source_ordered_after_cp418',
    'cp419_conceptual_contract_has_54_outcomes_49_inactive_5_assignments_and_15_sites',
    'cp419_snapshot_serializer_retains_cp418_prefix_through_field_159_and_declares_234_lossless_keys',
    'committed_route_accessor_is_nonrecursive_and_checks_committed_shape',
    'sealed_committed_humidity_owner_accepts_exact_witness_and_rejects_forgery',
    'sealed_humidity_owner_source_has_no_recursive_exact_validation',
    'validated_route_advance_matches_cold_recursive_advance_bit_exact',
    'validated_route_advance_rejects_forged_route_transactionally',
    'public_release_hot_path_has_no_recursive_exact_route_derivation',
    'active_raw_ieee_inputs_use_canonical_cp_air_and_direct_predicates_reject_them',
    'inactive_route_requires_no_owner_and_rejects_supplied_owner_transactionally',
    'cp419_direct_validator_uses_local_lineage_without_recursive_exact_characterization'
)) { Assert-Cp419Text -Text $testText -Pattern $pattern -Description 'CP419 regression coverage' }

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\s*=','let\s+calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry\s*=','let\s+calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment\s*=','let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP418-to-CP419-to-CP420-to-CP421-to-CP422-to-CP423-to-CP424-to-CP425-to-CP426 binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+coupling\s*:') -Description 'CP418-to-CP419 scheduled output order'
$bindingText = Read-RepoText -Path $binding
$dto = Get-Cp419BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp418|cp419|else_branch_(?:entry|cp_air)') { throw 'CP418/CP419 evidence entered numerical DTO' }
Assert-Contains -Path $coupled -Pattern 'predecessor_cp418:\s*&PredecessorLifecycle' -Description 'coupled CP418 predecessor'
Assert-Contains -Path $coupled -Pattern 'snapshots_match_bit_exact' -Description 'coupled bit-exact lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $witness -Pattern ("set_" + $stem + "_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle") -Description 'pipeline CP418-to-CP419 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp418:\s*Option<&PredecessorLifecycle>' -Description 'pipeline CP418 predecessor'
Assert-Contains -Path $pipelineLineage -Pattern '(?s)fn\s+cp418_prefix_is_exact.*?snapshot_json\(snapshot\).*?predecessor_json\(predecessor\).*?predecessor\.iter\(\)\.all' -Description 'bit-exact JSON-sidecar pipeline lineage'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp419_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp419_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp419_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-NotContains -Path $arbitrary -Pattern 'coupling_input.*(?:cp419|else_branch_entry)|(?:cp419|else_branch_entry).*coupling_input' -Description 'arbitrary numerical DTO feed'

$serializationText = Read-RepoText -Path $serialization
$predecessorSerialization = "crates\ep_run\src\pipeline\purchased_air_$predecessorStem\serialization\snapshot.rs"
$predecessorSerializationText = Read-RepoText -Path $predecessorSerialization
$predecessorJsonKeys = @([regex]::Matches($predecessorSerializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$jsonTail = @(
    'predecessor_cp418_resulting_supply_humidity_ratio','predecessor_cp418_resulting_supply_humidity_ratio_ieee_bits',
    'predecessor_cp418_resulting_supply_enthalpy_j_per_kg','predecessor_cp418_resulting_supply_enthalpy_j_per_kg_ieee_bits',
    'predecessor_cp418_resulting_supply_temperature_c','predecessor_cp418_resulting_supply_temperature_c_ieee_bits',
    $predecessorMarker,$localFields[0],$localFields[1],$localFields[2],
    'mixed_air_humidity_ratio_for_cp_air','mixed_air_humidity_ratio_for_cp_air_ieee_bits',$localFields[4],
    'psychrometric_cp_air_result_j_per_kg_k','psychrometric_cp_air_result_j_per_kg_k_ieee_bits',$localFields[6],
    'cp_air_j_per_kg_k','cp_air_j_per_kg_k_ieee_bits',
    'resulting_supply_humidity_ratio','resulting_supply_humidity_ratio_ieee_bits',
    'resulting_supply_enthalpy_j_per_kg','resulting_supply_enthalpy_j_per_kg_ieee_bits',
    'resulting_supply_temperature_c','resulting_supply_temperature_c_ieee_bits'
)
if ($predecessorJsonKeys.Count -ne 217 -or $jsonKeys.Count -ne 234 -or ($jsonKeys[0..209] -join '|') -cne ($predecessorJsonKeys[0..209] -join '|') -or ($jsonKeys[210..233] -join '|') -cne ($jsonTail -join '|')) { throw 'CP419 JSON must preserve CP418 keys 0..209 and append the exact twenty-four-key tail' }
foreach ($field in $numericFields) {
    $escaped = [regex]::Escape($field)
    Assert-Cp419Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar'
}

$heading = 'CP419 post-saturation capacity-limit dehumidification-guard else-branch `CpAir` assignment'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    $headings = [regex]::Matches($docText,'(?m)^## CP(?<number>40[9]|41[0-9])\b')
    $numbers = @($headings | ForEach-Object { [int]$_.Groups['number'].Value })
    if (($numbers -join '|') -cne '409|410|411|412|413|414|415|416|417|418|419') { throw "CP409-CP419 documentation order drift in $doc" }
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP419 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'physical executable line 2330 exactly','line 2331.*?first excluded.*?CP420',
        'read-purchased-air-mixed-air-humidity-ratio.*?evaluate-psy-cp-air-fn-w.*?assign-local-cp-air',
        'fifty-four flattened conceptual outcomes','4, 7, 10, 13, and 16','forty-nine','T419=54','Z419=49','A419=5','S419=3\*A419=15','17/37',
        'active public indices.*?4.*?7','active private indices.*?10.*?13.*?16','mutually exclusive.*?eighteen CP417','Nine.*?width-36',
        'CP418.*?sole.*?immediate route predecessor','sealed same-call CP329','Inactive routes acquire no numeric owner',
        'energyplus_psy_cp_air_fn_w','finite.*?greater than or equal to zero','1\.00484e3.*?1\.85895e3','dwSave.*?cpaSave',
        'zero generic.*?recursive exact-route derivations','174 base fields','first 159 fields','predecessor-CP418 triple',
        'eight.*?local fields','sixty `Option<f64>`','234 unique keys','sixty adjacent IEEE sidecars','36/41/51','owns only five local `CpAir` values',
        'CP418-to-CP419-to-unchanged-numerical','109 to 110','no numerical, coupling-input, or.*?output DTO','never feeds','32 algorithms, 293 routines',
        '58.*?state_mapped','235.*?source_mapped','357 total','240 public','117\s+internal','238\s+development commands'
    )) { Assert-Cp419Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
}

foreach ($specAddendum in @(
    [PSCustomObject]@{ Path='specs\algorithm_ledger.toml'; Anchor='CP419 supersedes only CP418' },
    [PSCustomObject]@{ Path='specs\capabilities.toml'; Anchor='CP419 additionally requires' }
)) {
    $specText = Read-RepoText -Path $specAddendum.Path
    $matches = [regex]::Matches($specText, '(?m)^\s*"(?<body>' + [regex]::Escape($specAddendum.Anchor) + '.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP419 expected one bounded addendum in $($specAddendum.Path), found $($matches.Count)" }
    $body = $matches[0].Groups['body'].Value
    foreach ($claim in @(
        'physical executable line 2330','line 2331.*?first excluded.*?CP420','three.*?dependency-ordered.*?sites|Exact sites are',
        '54.*?49.*?5.*?15','17/37','4 and 7','10, 13, and 16','Nine width-36',
        'CP418.*?sole immediate route predecessor','sealed same-call CP329','finite.*?>=0\.0|finite.*?greater than or equal to zero',
        'energyplus_psy_cp_air_fn_w|PsyCpAirFnW','1\.00484e3.*?1\.85895e3','174 base fields','sixty.*?Option<f64>|sixty numeric optionals',
        '234 JSON keys','first 210 keys','twenty-four-key tail','36/41/51','five local CpAir|five local `CpAir`',
        'CP418-to-CP419-to-unchanged-numerical','109 to 110','357 total, 240 public, 117 internal'
    )) { Assert-Cp419Text -Text $body -Pattern "(?is)$claim" -Description "bounded addendum claim in $($specAddendum.Path)" }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP419 supersedes only CP418' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP419 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP419\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP419\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP419 algorithm/routine ledger counts drift' }

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 419) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 419) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 373' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 419) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 133' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 419) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 373 \|')) -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 133 \|')) -Description 'historical generated internal total'
    }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP419 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminalAudits = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 418)) })
if ($terminalAudits.Count -ne 42) { throw 'CP419 terminal propagation set drift' }
foreach ($file in $terminalAudits) {
    Assert-Contains -Path $file.FullName -Pattern '\$cp419Call' -Description 'CP419 terminal capture'
    Assert-Contains -Path $file.FullName -Pattern 'CP418-to-CP419' -Description 'historical CP418-to-CP419 interval'
    Assert-Contains -Path $file.FullName -Pattern '\$cp420Call' -Description 'CP420 terminal capture'
    Assert-Contains -Path $file.FullName -Pattern 'CP419-to-CP420' -Description 'historical CP419-to-CP420 interval'
    Assert-Contains -Path $file.FullName -Pattern '\$cp421Call' -Description 'CP421 terminal capture'
    Assert-Contains -Path $file.FullName -Pattern 'CP420-to-CP421' -Description 'historical CP420-to-CP421 interval'
    Assert-Contains -Path $file.FullName -Pattern '\$cp422Call' -Description 'CP422 terminal capture'
    Assert-Contains -Path $file.FullName -Pattern 'CP421-to-CP422' -Description 'historical CP421-to-CP422 interval'
    Assert-Contains -Path $file.FullName -Pattern 'CP422-to-CP423' -Description 'CP422 terminal-to-numerical interval'
}

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp418-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-entry.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP418-to-CP419 registration order drift' }
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 99' -Description 'CP419 calc-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$idealLoadsInitWitnesses -Limit 272' -Description 'CP419 witness-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1' -Pattern 'Assert-LineLimit -Path \$cp339CalcRoot -Limit 99' -Description 'historical calc-root structural cap'

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 373','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp419Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 133) { throw 'CP419 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp419-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air-assignment\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 373 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 133 \|' -Description 'generated internal total'

if ((Get-Content -LiteralPath 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1').Count -gt 1200) { throw 'CP345 line cap exceeded after CP419 terminal propagation' }
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal-to-numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host 'CP419 post-saturation dehumidification-guard else-branch CpAir assignment structure audit passed.'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment' -Description 'CP425 binding successor registration'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment' -Description 'CP426 binding successor registration'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment' -Description 'CP427 recent binding propagation'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment' -Description 'CP428 recent binding propagation'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'
