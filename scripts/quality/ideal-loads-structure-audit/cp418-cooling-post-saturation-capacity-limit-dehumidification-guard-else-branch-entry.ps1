# CP418 maps PurchasedAirManager.cc physical control line 2327's outer dehumidification-guard else entry.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntry'
$predecessorTypeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignment'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$predecessorModule = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem.rs"
$predecessorRoot = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem"
$predecessorRelease = "$predecessorRoot\release.rs"
$predecessorTests = "$predecessorRoot\tests.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$transitionAccounting = "$root\transition\accounting.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$releaseError = "$root\release\error.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$releaseSnapshot = "$root\release\snapshot.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp418.rs'
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serializationRoot = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp418_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp417_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp418-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-entry.ps1'

function Assert-Cp418Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP418 $Description missing '$Pattern'" }
}

function Get-Cp418BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP418 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP418 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP418 $Description closing brace missing"
}

$required = @(
    $source,$module,$predecessorModule,$predecessorRelease,$predecessorTests,$state,$transition,$transitionAccounting,$tests,$release,$releaseError,$runtimeValidation,$releaseSnapshot,
    $binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipelineRoot,$pipeline,$pipelineValidation,$pipelineLineage,
    $serializationRoot,$serialization,$snapshotJsonTests,$arbitrary,$arbitraryPredecessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP418 implementation/audit file' }
foreach ($file in @($module,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipeline,$pipelineValidation,$pipelineLineage,$serializationRoot,
    $serialization,$snapshotJsonTests,$arbitrary,$audit)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP418 file'
}
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
if ($coreFiles.Count -ne 8) { throw 'CP418 exact eight-file bounded core subtree drift' }
foreach ($file in $coreFiles) { Assert-LineLimit -Path $file.FullName -Limit 500 -Description 'bounded CP418 core file' }

$registrations = @(
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\calc.rs'; Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs'; Pattern="mod\s+$stem" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs'; Pattern="$($stem)_tests" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs'; Pattern="mod\s+$($stem)_validation" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs'; Pattern='test_coupled_runtime_cp418\.rs' },
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
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP418 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP418 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
$boundary = @(
    [PSCustomObject]@{ Line=2266; Text='if (PurchAir.SupplyHumRat < PurchAir.MixedAirHumRat) { // Dehumidifying' },
    [PSCustomObject]@{ Line=2325; Text='}' },
    [PSCustomObject]@{ Line=2326; Text='} // Capacity limit exceeded' },
    [PSCustomObject]@{ Line=2327; Text='} else { // Not dehumidifying' },
    [PSCustomObject]@{ Line=2328; Text='// If not dehumidifying, compare sensible cooling to the limit' },
    [PSCustomObject]@{ Line=2329; Text='// This section will only increase supply temp, so no need to recheck for super-saturation' },
    [PSCustomObject]@{ Line=2330; Text='CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat);' }
)
foreach ($item in $boundary) {
    if ($sourceLines[$item.Line - 1].Trim() -cne $item.Text) { throw "CP418 source boundary drift at physical line $($item.Line)" }
}

$moduleText = Read-RepoText -Path $module
Assert-Cp418Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2327' -Description 'source constant'
Assert-Cp418Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2330' -Description 'first excluded executable constant'
$orderMatch = [regex]::Match($moduleText, '(?s)GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP418 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$soleSite = 'enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-after-guard-false-fallthrough'
if ($sites.Count -ne 1 -or $sites[0] -cne $soleSite) { throw 'CP418 sole source site drift' }

$snapshotStruct = Get-Cp418BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$predecessorText = Read-RepoText -Path $predecessorModule
$predecessorStruct = Get-Cp418BraceBlock -Text $predecessorText -AnchorPattern "pub\s+struct\s+$($predecessorTypeStem)Snapshot\s*" -Description 'CP417 snapshot'
$predecessorFields = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$marker = 'post_saturation_capacity_limit_dehumidification_guard_else_branch_entered'
$expectedFields = @($predecessorFields) + $marker
if ($predecessorFields.Count -ne 162 -or $fields.Count -ne 163 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP418 snapshot must preserve the exact CP417 162-field prefix and append only the final marker' }
$terminal = @('resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c')
if (($fields[159..161] -join '|') -cne ($terminal -join '|') -or $fields[162] -cne $marker) { throw 'CP418 terminal W/H/T reuse and final marker placement drift' }
$predecessorNumeric = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
if ($predecessorNumeric.Count -ne 54 -or $numericFields.Count -ne 54 -or ($numericFields -join '|') -cne ($predecessorNumeric -join '|')) { throw 'CP418 must reuse exactly the CP417 fifty-four numeric carriers' }
if ([regex]::Matches($snapshotStruct, 'Option<bool>').Count -ne 1 -or [regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP418 optional comparison/control carrier drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @(
    'predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts','predecessor_guard_body_entry_route_counts',
    'predecessor_supply_temperature_saturation_assignment_route_counts','predecessor_supply_temperature_mixed_air_limit_route_counts',
    'predecessor_supply_humidity_ratio_assignment_route_counts','predecessor_supply_enthalpy_assignment_route_counts',
    'dehumidification_guard_else_branch_entry_route_counts'
)
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP418 exact eight width-36 route arrays drift' }
$expectedCounters = @(
    'transition_count','inactive_transition_count','predecessor_supply_temperature_saturation_assignment_count',
    'predecessor_supply_temperature_saturation_mixed_air_limit_count','predecessor_supply_humidity_ratio_assignment_count',
    'predecessor_supply_enthalpy_assignment_count','dehumidification_guard_else_branch_entry_count','source_site_execution_count',
    'cp417_supply_humidity_ratio_state_owner_count','unchanged_supply_humidity_ratio_preservation_count',
    'cp417_supply_enthalpy_state_owner_count','unchanged_supply_enthalpy_preservation_count',
    'cp417_supply_temperature_state_owner_count','unchanged_supply_temperature_preservation_count'
)
$counterFields = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*usize') | ForEach-Object { $_.Groups['name'].Value })
if (($counterFields -join '|') -cne ($expectedCounters -join '|')) { throw 'CP418 exact runtime counter set drift' }

$transitionText = Read-RepoText -Path $transition
$accountingText = Read-RepoText -Path $transitionAccounting
foreach ($pattern in @(
    'SupplyEnthalpyAssignmentSnapshot as Predecessor','predecessor_dehumidification_guard_false_fallthrough',
    'active:\s*predecessor\.predecessor_dehumidification_guard_false_fallthrough',
    'post_saturation_capacity_limit_dehumidification_guard_else_branch_entered:\s*route\.active',
    'resulting_supply_humidity_ratio:\s*predecessor\.resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg:\s*predecessor\.resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c:\s*predecessor\.resulting_supply_temperature_c'
)) { Assert-Cp418Text -Text $transitionText -Pattern "(?s)$pattern" -Description 'outer dehumidification-guard else-entry transition contract' }
foreach ($pattern in @('dehumidification_guard_else_branch_entry_count\s*\+=\s*1','dehumidification_guard_else_branch_entry_route_counts\[index\]\s*\+=\s*1','source_site_execution_count\s*\+=\s*1')) {
    Assert-Cp418Text -Text $accountingText -Pattern $pattern -Description 'CP418 route accounting'
}
foreach ($file in @($transition,$transitionAccounting,$release,$releaseError,$runtimeValidation,$releaseSnapshot,$adapter,$coupled,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}
Assert-NotContains -Path $transition -Pattern '(?i)(?:energyplus_psy|Psy[A-Z][A-Za-z0-9_]*)\s*\(|is_finite\s*\(|clamp\s*\(|mul_add\s*\(|warning|diagnostic|PsychrometricService' -Description 'control-only semantic substitution'

$predecessorModuleText = Read-RepoText -Path $predecessorModule
$predecessorReleaseText = Read-RepoText -Path $predecessorRelease
Assert-Cp418Text -Text $predecessorModuleText -Pattern 'pub\(in crate::ideal_loads::calc\) use release::\{(?s:.*?)cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_committed_latest_route' -Description 'calc-private sealed CP417 route accessor re-export'
Assert-Cp418Text -Text $predecessorModuleText -Pattern 'RetainedRoute as PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentCommittedRoute' -Description 'calc-private sealed CP417 route type'
$committedAccessor = Get-Cp418BraceBlock -Text $predecessorReleaseText -AnchorPattern 'pub\(in crate::ideal_loads::calc\) fn cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_committed_latest_route\s*\(' -Description 'sealed CP417 committed-route accessor'
foreach ($pattern in @('state\.latest_route\?','committed_route_counts_match\s*\(','snapshot_matches_validated_predecessor\s*\(')) { Assert-Cp418Text -Text $committedAccessor -Pattern $pattern -Description 'sealed CP417 committed-route proof' }

$releaseText = Read-RepoText -Path $release
$hotRelease = Get-Cp418BraceBlock -Text $releaseText -AnchorPattern 'pub fn advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\s*\(' -Description 'CP418 public hot release'
foreach ($pattern in @(
    'cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_committed_latest_route\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_route_from_validated_predecessor\s*\(',
    'advance_with_validated_route\s*\('
)) { Assert-Cp418Text -Text $hotRelease -Pattern $pattern -Description 'non-recursive CP418 hot release' }
foreach ($pattern in @(
    '(?<![A-Za-z0-9_])predecessor_route\s*\(','private_[a-z0-9_]*characterization\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact_direct_release\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release\s*\('
)) { if ($hotRelease -match $pattern) { throw "CP418 public hot release recursively re-derived exact route via '$pattern'" } }

$coupledText = Read-RepoText -Path $coupled
Assert-Cp418Text -Text $coupledText -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_has_exact_cp417_prefix_and_marker\s*\(' -Description 'cheap coupled CP417-prefix/marker validation'
foreach ($pattern in @(
    '(?<![A-Za-z0-9_])predecessor_route\s*\(','private_[a-z0-9_]*characterization\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot_is_exact_direct_release\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release\s*\('
)) { if ($coupledText -match $pattern) { throw "CP418 coupled validator recursively re-derived exact route via '$pattern'" } }

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine +
    (Read-RepoText -Path $predecessorTests) + [Environment]::NewLine +
    (Read-RepoText -Path $adapterTests) + [Environment]::NewLine +
    (Read-RepoText -Path $coupledTests) + [Environment]::NewLine +
    (Read-RepoText -Path $pipelineValidation) + [Environment]::NewLine +
    (Read-RepoText -Path $snapshotJsonTests) + [Environment]::NewLine +
    (Read-RepoText -Path $arbitrary)
foreach ($pattern in @(
    'cp418_boundary_and_sole_site_are_exact','exhaustive_54_outcomes_49_inactive_five_entries_and_eight_arrays_are_exact',
    'public_release_enters_only_the_outer_dehumidification_guard_else_branch','marker_and_predecessor_forgery_are_rejected',
    'counter_overflow_is_transactional','cp418_binding_contract_is_source_ordered_after_cp417',
    'cp418_conceptual_contract_has_54_outcomes_5_else_entries_and_preserves_all_carriers',
    'cp418_snapshot_serializer_retains_cp417_prefix_and_appends_one_final_key',
    'sealed_committed_route_rejects_route_and_counter_forgery',
    'validated_route_advance_matches_cold_recursive_advance_bit_exact',
    'validated_route_advance_rejects_forged_route_transactionally',
    'public_release_hot_path_has_no_recursive_exact_route_derivation',
    'cold_recursive_exact_validator_accepts_scheduled_cp418_release_snapshot'
)) { Assert-Cp418Text -Text $testText -Pattern $pattern -Description 'CP418 regression coverage' }

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP417-to-CP418-to-numerical binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+coupling\s*:') -Description 'CP417-to-CP418 scheduled output order'
$bindingText = Read-RepoText -Path $binding
$dto = Get-Cp418BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp417|cp418|else_branch_entry') { throw 'CP417/CP418 evidence entered numerical DTO' }
Assert-Contains -Path $coupled -Pattern 'predecessor_cp417:\s*&PredecessorLifecycle' -Description 'coupled CP417 predecessor'
Assert-Contains -Path $coupled -Pattern 'snapshots_match_bit_exact' -Description 'coupled bit-exact lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $witness -Pattern ("set_" + $stem + "_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle") -Description 'pipeline CP417-to-CP418 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp418_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp417:\s*Option<&PredecessorLifecycle>' -Description 'pipeline CP417 predecessor'
Assert-Contains -Path $pipelineLineage -Pattern '(?s)snapshot_json\(snapshot\).*?predecessor_json\(predecessor\).*?inherited_fields_match' -Description 'bit-exact JSON-sidecar pipeline lineage'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp418_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp418_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp418_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-NotContains -Path $arbitrary -Pattern 'coupling_input.*(?:cp418|else_branch_entry)|(?:cp418|else_branch_entry).*coupling_input' -Description 'arbitrary numerical DTO feed'

$serializationText = Read-RepoText -Path $serialization
$predecessorSerialization = "crates\ep_run\src\pipeline\purchased_air_$predecessorStem\serialization\snapshot.rs"
$predecessorSerializationText = Read-RepoText -Path $predecessorSerialization
$predecessorJsonKeys = @([regex]::Matches($predecessorSerializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$expectedJson = @($predecessorJsonKeys) + $marker
if ($predecessorJsonKeys.Count -ne 216 -or $jsonKeys.Count -ne 217 -or ($jsonKeys -join '|') -cne ($expectedJson -join '|')) { throw 'CP418 JSON must preserve the exact CP417 216-key prefix and append only the final marker key' }
foreach ($field in $numericFields) {
    $escaped = [regex]::Escape($field)
    Assert-Cp418Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar'
}

$heading = 'CP418 post-saturation capacity-limit dehumidification-guard else-branch entry'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    $headings = [regex]::Matches($docText,'(?m)^## CP(?<number>40[9]|41[0-8])\b')
    $numbers = @($headings | ForEach-Object { [int]$_.Groups['number'].Value })
    if (($numbers -join '|') -cne '409|410|411|412|413|414|415|416|417|418') { throw "CP409-CP418 documentation order drift in $doc" }
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP418 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'physical control line 2327 exactly','sibling else entry of CP381.*?line-2266','line 2325 closes CP413.*?line 2326.*?line-2268',
        'neither a CP413.*?false result nor a CP417.*?exit','enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-after-guard-false-fallthrough',
        'lines 2328 and 2329.*?comment-only','line 2330.*?first excluded.*?CP419','fifty-four flattened conceptual outcomes',
        '4, 7, 10, 13, and 16','forty-nine','T418=54','Z418=49','E418=5','S418=E418=5','17/37',
        'active public indices.*?4.*?7','active private indices.*?10.*?13.*?16','mutually exclusive.*?eighteen CP417','Eight width-36',
        'CP417.*?sole immediate predecessor','no scalar active input','no.*?owner read','36/41/51','acquires no new owner',
        'exact first 162 fields','existing terminal W/H/T','post_saturation_capacity_limit_dehumidification_guard_else_branch_entered',
        '163 base fields','fifty-four.*?Option<f64>','217 unique keys','fifty-four adjacent','CP417-to-CP418-to-unchanged-numerical',
        '108 to 109','adds no numerical or coupling-input DTO','no output DTO','never feeds','32\s+algorithms,\s+293',
        '58.*?state_mapped','235.*?source_mapped','356 total','240 public','116\s+internal','238\s+development commands'
    )) { Assert-Cp418Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
}

foreach ($specAddendum in @(
    [PSCustomObject]@{ Path='specs\algorithm_ledger.toml'; Anchor='CP418 supersedes only CP417' },
    [PSCustomObject]@{ Path='specs\capabilities.toml'; Anchor='CP418 additionally requires' }
)) {
    $specText = Read-RepoText -Path $specAddendum.Path
    $matches = [regex]::Matches($specText, '(?m)^\s*"(?<body>' + [regex]::Escape($specAddendum.Anchor) + '.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP418 expected one bounded addendum in $($specAddendum.Path), found $($matches.Count)" }
    $body = $matches[0].Groups['body'].Value
    foreach ($claim in @(
        'physical control line 2327','line 2325.*?CP413','line 2326.*?line-2268','CP381.*?physical-line-2266',
        'line 2330.*?first excluded.*?CP419','54.*?49.*?5','17/37','4 and 7','10, 13, and 16','mutually exclusive',
        'Eight width-36','CP417.*?sole','36/41/51','first 162 fields','post_saturation_capacity_limit_dehumidification_guard_else_branch_entered',
        '163 base fields','fifty-four.*?Option<f64>|fifty-four numeric optionals','217 JSON keys','CP417-to-CP418-to-unchanged-numerical',
        '108 to 109','356 total, 240 public, 116 internal'
    )) { Assert-Cp418Text -Text $body -Pattern "(?is)$claim" -Description "bounded addendum claim in $($specAddendum.Path)" }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP418 supersedes only CP417' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP418 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP418\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP418\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP418 algorithm/routine ledger counts drift' }

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 418) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp418_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 418) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 356' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 418) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 116' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 418) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 356 \|')) -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 116 \|')) -Description 'historical generated internal total'
    }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP418 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminalAudits = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 417)) })
if ($terminalAudits.Count -ne 41) { throw 'CP418 terminal propagation set drift' }
foreach ($file in $terminalAudits) {
    Assert-Contains -Path $file.FullName -Pattern '\$cp418Call' -Description 'CP418 terminal capture'
    Assert-Contains -Path $file.FullName -Pattern 'CP417-to-CP418' -Description 'historical CP417-to-CP418 interval'
    Assert-Contains -Path $file.FullName -Pattern 'CP418-to-numerical' -Description 'CP418 terminal-to-numerical interval'
}

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp417-cooling-post-saturation-capacity-limit-dehumidification-supply-enthalpy-assignment.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP417-to-CP418 registration order drift' }
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 97' -Description 'CP418 calc-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1' -Pattern 'Assert-LineLimit -Path \$cp339CalcRoot -Limit 97' -Description 'historical calc-root structural cap'

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 356','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp418Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 116) { throw 'CP418 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp418-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-entry\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 356 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 116 \|' -Description 'generated internal total'

if ((Get-Content -LiteralPath 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1').Count -gt 1200) { throw 'CP345 line cap exceeded after CP418 terminal propagation' }
Write-Host 'CP418 post-saturation dehumidification-guard else-branch entry structure audit passed.'
}
