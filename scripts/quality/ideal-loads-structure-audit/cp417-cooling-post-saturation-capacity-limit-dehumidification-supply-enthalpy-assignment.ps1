# CP417 maps PurchasedAirManager.cc physical executable line 2321's psychrometric enthalpy assignment.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignment'
$predecessorTypeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignment'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$predecessorModule = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem.rs"
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
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp417.rs'
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
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp417_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp416_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp417-cooling-post-saturation-capacity-limit-dehumidification-supply-enthalpy-assignment.ps1'

function Assert-Cp417Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP417 $Description missing '$Pattern'" }
}

function Get-Cp417BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP417 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP417 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP417 $Description closing brace missing"
}

$required = @(
    $source,$module,$predecessorModule,$state,$transition,$transitionAccounting,$tests,$release,$releaseError,$runtimeValidation,$releaseSnapshot,
    $binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipelineRoot,$pipeline,$pipelineValidation,$pipelineValidationTests,$pipelineLineage,
    $serializationRoot,$serialization,$snapshotJsonTests,$arbitrary,$arbitraryPredecessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP417 implementation/audit file' }
foreach ($file in @($module,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipeline,$pipelineValidation,$pipelineValidationTests,$pipelineLineage,$serializationRoot,
    $serialization,$snapshotJsonTests,$arbitrary,$audit)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP417 file'
}
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
if ($coreFiles.Count -ne 8) { throw 'CP417 exact eight-file bounded core subtree drift' }
foreach ($file in $coreFiles) { Assert-LineLimit -Path $file.FullName -Limit 500 -Description 'bounded CP417 core file' }

$registrations = @(
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\calc.rs'; Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs'; Pattern="mod\s+$stem" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs'; Pattern="$($stem)_tests" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs'; Pattern="mod\s+$($stem)_validation" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs'; Pattern='test_coupled_runtime_cp417\.rs' },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs'; Pattern="$($stem)_fixture" },
    [PSCustomObject]@{ Path=$module; Pattern='(?s)mod\s+release;.*?mod\s+state;.*?mod\s+tests;.*?mod\s+transition;' },
    [PSCustomObject]@{ Path=$release; Pattern='(?s)mod\s+error;.*?mod\s+runtime_validation;.*?mod\s+snapshot;' },
    [PSCustomObject]@{ Path=$transition; Pattern='mod\s+accounting;' },
    [PSCustomObject]@{ Path=$pipelineRoot; Pattern="mod\s+$pipelineStem;" },
    [PSCustomObject]@{ Path=$pipeline; Pattern='(?s)mod\s+serialization;.*?mod\s+validation;' },
    [PSCustomObject]@{ Path=$pipelineValidation; Pattern='(?s)mod\s+lineage;.*?mod\s+tests;' },
    [PSCustomObject]@{ Path=$serializationRoot; Pattern='mod\s+snapshot;' },
    [PSCustomObject]@{ Path=$serialization; Pattern='mod\s+tests;' }
)
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP417 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP417 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
$boundary = @(
    'SupplyEnthalpy = PsyHFnTdbW(PurchAir.SupplyTemp, PurchAir.SupplyHumRat);',
    '// CpAir = PsyCpAirFnW(MixedAirHumRat)',
    '// CoolSensOutput = SupplyMassFlowRate * CpAir * (MixedAirTemp - SupplyTemp)',
    '// CoolTotOutput = SupplyMassFlowRate * (MixedAirEnthalpy - SupplyEnthalpy)',
    '}',
    '} // Capacity limit exceeded',
    '} else { // Not dehumidifying',
    '// If not dehumidifying, compare sensible cooling to the limit',
    '// This section will only increase supply temp, so no need to recheck for super-saturation',
    'CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat);'
)
for ($index = 0; $index -lt $boundary.Count; $index += 1) {
    if ($sourceLines[2320 + $index].Trim() -cne $boundary[$index]) { throw "CP417 source boundary drift at physical line $(2321 + $index)" }
}

$moduleText = Read-RepoText -Path $module
Assert-Cp417Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2321' -Description 'source constant'
Assert-Cp417Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2330' -Description 'first excluded executable constant'
$orderMatch = [regex]::Match($moduleText, '(?s)SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP417 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSites = @(
    'read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-enthalpy',
    'read-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-enthalpy',
    'evaluate-psy-h-fn-tdb-w-for-post-saturation-capacity-limit-dehumidification',
    'assign-local-supply-enthalpy-after-post-saturation-capacity-limit-dehumidification-humidity-ratio-assignment'
)
if (($sites -join '|') -cne ($expectedSites -join '|')) { throw 'CP417 four-site source order drift' }

$snapshotStruct = Get-Cp417BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$predecessorText = Read-RepoText -Path $predecessorModule
$predecessorStruct = Get-Cp417BraceBlock -Text $predecessorText -AnchorPattern "pub\s+struct\s+$($predecessorTypeStem)Snapshot\s*" -Description 'CP416 snapshot'
$predecessorFields = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$terminal = @('resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c')
if ($predecessorFields.Count -ne 145 -or (($predecessorFields[142..144]) -join '|') -cne ($terminal -join '|')) { throw 'CP417 CP416 predecessor shape drift' }
$predecessorTriple = @('predecessor_cp416_resulting_supply_humidity_ratio','predecessor_cp416_resulting_supply_enthalpy_j_per_kg','predecessor_cp416_resulting_supply_temperature_c')
$suffix = @(
    'post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed',
    'cp416_retained_supply_humidity_ratio_state_owned','cp416_retained_supply_enthalpy_state_owned','cp416_retained_supply_temperature_state_owned',
    'cp416_retained_supply_temperature_owned_read','supply_temperature_for_enthalpy_read','supply_temperature_for_enthalpy_c',
    'cp416_retained_supply_humidity_ratio_owned_read','supply_humidity_ratio_for_enthalpy_read','supply_humidity_ratio_for_enthalpy',
    'psychrometric_supply_enthalpy_evaluated','psychrometric_supply_enthalpy_j_per_kg',
    'supply_enthalpy_assignment_performed','assigned_supply_enthalpy_j_per_kg',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
$expectedFields = @($predecessorFields[0..141]) + $predecessorTriple + $suffix
if ($fields.Count -ne 162 -or $expectedFields.Count -ne 162 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP417 snapshot must expose exactly 162 canonical fields' }
$predecessorNumeric = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
if ($predecessorNumeric.Count -ne 47 -or (($predecessorNumeric[44..46]) -join '|') -cne ($terminal -join '|')) { throw 'CP417 CP416 numeric predecessor shape drift' }
$localNumeric = @('supply_temperature_for_enthalpy_c','supply_humidity_ratio_for_enthalpy','psychrometric_supply_enthalpy_j_per_kg','assigned_supply_enthalpy_j_per_kg')
$expectedNumeric = @($predecessorNumeric[0..43]) + $predecessorTriple + $localNumeric + $terminal
if ($numericFields.Count -ne 54 -or ($numericFields -join '|') -cne ($expectedNumeric -join '|')) { throw 'CP417 fifty-four numeric carrier fields drift' }
if ([regex]::Matches($snapshotStruct, 'Option<bool>').Count -ne 1 -or [regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP417 optional comparison/control carrier drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @(
    'predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts','predecessor_guard_body_entry_route_counts',
    'predecessor_supply_temperature_saturation_assignment_route_counts','predecessor_supply_temperature_mixed_air_limit_route_counts',
    'predecessor_supply_humidity_ratio_assignment_route_counts','supply_enthalpy_assignment_route_counts'
)
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP417 seven width-36 route arrays drift' }
$expectedCounters = @(
    'transition_count','inactive_transition_count','predecessor_supply_temperature_saturation_assignment_count',
    'predecessor_supply_temperature_saturation_mixed_air_limit_count','predecessor_supply_humidity_ratio_assignment_count',
    'supply_enthalpy_assignment_count','source_site_execution_count','cp416_supply_humidity_ratio_state_owner_count',
    'unchanged_supply_humidity_ratio_preservation_count','cp416_supply_enthalpy_state_owner_count',
    'unchanged_supply_enthalpy_preservation_count','cp416_supply_temperature_state_owner_count',
    'unchanged_supply_temperature_preservation_count','cp417_psychrometric_supply_enthalpy_state_owner_count',
    'cp416_retained_supply_temperature_owned_read_count','supply_temperature_for_enthalpy_read_count',
    'cp416_retained_supply_humidity_ratio_owned_read_count','supply_humidity_ratio_for_enthalpy_read_count',
    'psychrometric_supply_enthalpy_evaluation_count','supply_enthalpy_assignment_write_count'
)
$counterFields = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*usize') | ForEach-Object { $_.Groups['name'].Value })
if (($counterFields -join '|') -cne ($expectedCounters -join '|')) { throw 'CP417 exact runtime counter set drift' }

$transitionText = Read-RepoText -Path $transition
$accountingText = Read-RepoText -Path $transitionAccounting
foreach ($pattern in @(
    'SupplyHumidityRatioAssignmentSnapshot as Predecessor','energyplus_psy_h_fn_tdb_w',
    'operands\s*=\s*if\s+route\.active','predecessor\.resulting_supply_temperature_c\?',
    'predecessor\.resulting_supply_humidity_ratio\?','energyplus_psy_h_fn_tdb_w\(temperature,\s*humidity_ratio\)',
    'psychrometric_supply_enthalpy\.or\(predecessor\.resulting_supply_enthalpy_j_per_kg\)',
    'resulting_supply_humidity_ratio:\s*predecessor\.resulting_supply_humidity_ratio',
    'resulting_supply_temperature_c:\s*predecessor\.resulting_supply_temperature_c'
)) { Assert-Cp417Text -Text $transitionText -Pattern "(?s)$pattern" -Description 'psychrometric enthalpy-assignment transition contract' }
foreach ($pattern in @('source_site_execution_count\s*\+=\s*4','supply_enthalpy_assignment_route_counts\[index\]\s*\+=\s*1','psychrometric_supply_enthalpy_evaluation_count\s*\+=\s*1')) {
    Assert-Cp417Text -Text $accountingText -Pattern $pattern -Description 'CP417 route accounting'
}
Assert-Contains -Path 'crates\ep_runtime\src\psychrometrics.rs' -Pattern '(?s)fn\s+energyplus_psy_h_fn_tdb_w_raw.*?1\.004_84e3\s*\*\s*dry_bulb_c\s*\+\s*humidity_ratio\s*\*\s*\(2\.500_94e6\s*\+\s*1\.858_95e3\s*\*\s*dry_bulb_c\)' -Description 'canonical PsyHFnTdbW grouping'
Assert-Contains -Path 'crates\ep_runtime\src\psychrometrics.rs' -Pattern '(?s)pub\s+fn\s+energyplus_psy_h_fn_tdb_w\(.*?energyplus_psy_h_fn_tdb_w_raw\(dry_bulb_c,\s*energyplus_humidity_ratio_floor\(humidity_ratio\)\)' -Description 'canonical PsyHFnTdbW floor delegation'
Assert-Contains -Path 'crates\ep_runtime\src\psychrometrics.rs' -Pattern '(?s)fn\s+energyplus_humidity_ratio_floor.*?if\s+humidity_ratio\s*<\s*ENERGYPLUS_MIN_HUMIDITY_RATIO\s*\{\s*ENERGYPLUS_MIN_HUMIDITY_RATIO\s*\}\s*else\s*\{\s*humidity_ratio\s*\}' -Description 'canonical strict humidity floor'
Assert-Contains -Path 'crates\ep_runtime\src\psychrometrics.rs' -Pattern 'const ENERGYPLUS_MIN_HUMIDITY_RATIO: f64 = 1\.0e-5;' -Description 'canonical minimum humidity literal'
foreach ($file in @($transition,$transitionAccounting,$release,$releaseError,$runtimeValidation,$releaseSnapshot,$adapter,$coupled,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}
Assert-NotContains -Path $transition -Pattern 'is_finite\s*\(|clamp\s*\(|mul_add\s*\(|energyplus_psy_h_fn_tdb_w_(?:fast|raw)|PsychrometricService|warning|diagnostic' -Description 'line-local semantic substitution'

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine +
    (Read-RepoText -Path $adapterTests) + [Environment]::NewLine +
    (Read-RepoText -Path $coupledTests) + [Environment]::NewLine +
    (Read-RepoText -Path $pipelineValidationTests) + [Environment]::NewLine +
    (Read-RepoText -Path $snapshotJsonTests) + [Environment]::NewLine +
    (Read-RepoText -Path $arbitrary)
foreach ($pattern in @(
    'cp417_boundary_and_four_sites_are_exact','exhaustive_54_outcomes_and_seven_route_partitions_are_exact',
    'public_route_transition_preserves_.*enthalpy.*edge_bit_semantics','inactive_routes_do_not_read_or_evaluate_operands',
    'snapshot_corruption_is_rejected','counter_overflow_is_transactional',
    'cp417_binding_contract_is_source_ordered_after_cp416','public_release_assigns_psychrometric_supply_enthalpy',
    'cp417_conceptual_contract_has_54_outcomes_72_sites_and_expected_carrier_ownership',
    'cp417_snapshot_serializer_declares_216_unique_json_entries_and_54_sidecars'
)) { Assert-Cp417Text -Text $testText -Pattern $pattern -Description 'CP417 regression coverage' }

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\s*=','let\s+calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry\s*=','let\s+calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment\s*=','let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP416-to-CP417-to-CP418-to-CP419-to-CP420-to-CP421-to-CP422-to-CP423-to-CP424-to-CP425-to-CP426 binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+coupling\s*:') -Description 'CP416-to-CP417 scheduled output order'
$bindingText = Read-RepoText -Path $binding
$dto = Get-Cp417BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp416|cp417|enthalpy_assignment|psychrometric_supply_enthalpy') { throw 'CP416/CP417 evidence entered numerical DTO' }
Assert-Contains -Path $coupled -Pattern 'predecessor_cp416:\s*&PredecessorLifecycle' -Description 'coupled CP416 predecessor'
Assert-Contains -Path $coupled -Pattern 'snapshots_match_bit_exact' -Description 'coupled bit-exact lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $witness -Pattern ("set_" + $stem + "_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle") -Description 'pipeline CP416-to-CP417 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp433_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp416:\s*Option<&PredecessorLifecycle>' -Description 'pipeline CP416 predecessor'
Assert-Contains -Path $pipelineLineage -Pattern 'option_bits_match|to_bits' -Description 'bit-exact pipeline lineage'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp417_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp417_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp417_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-NotContains -Path $arbitrary -Pattern 'coupling_input.*(?:cp417|enthalpy_assignment)|(?:cp417|enthalpy_assignment).*coupling_input' -Description 'arbitrary numerical DTO feed'

$serializationText = Read-RepoText -Path $serialization
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$numericSet = @{}; foreach ($field in $expectedNumeric) { $numericSet[$field] = $true }
$expectedJson = @(); foreach ($field in $expectedFields) { $expectedJson += $field; if ($numericSet.ContainsKey($field)) { $expectedJson += ($field + '_ieee_bits') } }
if ($jsonKeys.Count -ne 216 -or $expectedJson.Count -ne 216 -or ($jsonKeys -join '|') -cne ($expectedJson -join '|')) { throw 'CP417 JSON must expose 216 canonical keys' }
foreach ($field in $expectedNumeric) {
    $escaped = [regex]::Escape($field)
    Assert-Cp417Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar'
}

$heading = 'CP417 post-saturation capacity-limit dehumidification supply-enthalpy assignment'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    $headings = [regex]::Matches($docText,'(?m)^## CP(?<number>40[8-9]|41[0-7])\b')
    $numbers = @($headings | ForEach-Object { [int]$_.Groups['number'].Value })
    if (($numbers -join '|') -cne '408|409|410|411|412|413|414|415|416|417') { throw "CP408-CP417 documentation order drift in $doc" }
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP417 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'line 2321 exactly','lines 2322 through 2324.*?comment-only','lines 2325 and.*?2326.*?closing delimiters',
        'line 2327.*?first excluded lexical/control.*?CP418 candidate','lines 2328 and 2329.*?comment-only',
        'line 2330.*?first excluded executable','read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-enthalpy',
        'read-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-dehumidification-enthalpy',
        'evaluate-psy-h-fn-tdb-w-for-post-saturation-capacity-limit-dehumidification',
        'assign-local-supply-enthalpy-after-post-saturation-capacity-limit-dehumidification-humidity-ratio-assignment',
        'fifty-four flattened conceptual outcomes','Thirty-six inactive','eighteen.*?all four','T417=54','Z417=36','A417=18','S417=4\*A417=72',
        '17/37','23, 25, 35, and 37','Seven width-36','36/41/51','CP416.*?sole immediate',
        'resulting_supply_temperature_c.*?solely owns','resulting_supply_humidity_ratio.*?solely owns','energyplus_psy_h_fn_tdb_w',
        '1\.00484e3\*T.*?2\.50094e6.*?1\.85895e3\*T','0x3ee4f8b588e368f1','NaN/unordered',
        'first 142 fields','seventeen CP417','162 base fields','fifty-four.*?Option<f64>','216 unique keys','fifty-four adjacent',
        'CP416-to-CP417-to-unchanged-numerical','107 to 108','adds no numerical or coupling-input DTO','no output DTO','never feeds',
        '32\s+algorithms,\s+293','58.*?state_mapped','235.*?source_mapped','355 total','240 public','115\s+internal','238\s+development commands'
    )) { Assert-Cp417Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
}

foreach ($specAddendum in @(
    [PSCustomObject]@{ Path='specs\algorithm_ledger.toml'; Anchor='CP417 supersedes only CP416' },
    [PSCustomObject]@{ Path='specs\capabilities.toml'; Anchor='CP417 additionally requires' }
)) {
    $specText = Read-RepoText -Path $specAddendum.Path
    $matches = [regex]::Matches($specText, '(?m)^\s*"(?<body>' + [regex]::Escape($specAddendum.Anchor) + '.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP417 expected one bounded addendum in $($specAddendum.Path), found $($matches.Count)" }
    $body = $matches[0].Groups['body'].Value
    foreach ($claim in @(
        'physical executable line 2321','line 2327.*?CP418','line 2330.*?first excluded executable','54.*?36.*?18.*?72',
        '17/37','23, 25, 35, and 37','Seven width-36','CP416.*?sole','36/41/51','energyplus_psy_h_fn_tdb_w',
        '162 base fields','fifty-four.*?Option<f64>','216 JSON keys','CP416-to-CP417-to-unchanged-numerical',
        '107 to 108','355 total, 240 public, 115 internal'
    )) { Assert-Cp417Text -Text $body -Pattern "(?is)$claim" -Description "bounded addendum claim in $($specAddendum.Path)" }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP417 supersedes only CP416' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP417 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP417\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP417\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP417 algorithm/routine ledger counts drift' }

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 417) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp433_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 417) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 371' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 417) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 131' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 417) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 371 \|')) -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 131 \|')) -Description 'historical generated internal total'
    }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP417 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminalAudits = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 416)) })
if ($terminalAudits.Count -ne 40) { throw 'CP417 terminal propagation set drift' }
foreach ($file in $terminalAudits) {
    Assert-Contains -Path $file.FullName -Pattern '\$cp417Call' -Description 'CP417 terminal capture'
    Assert-Contains -Path $file.FullName -Pattern 'CP416-to-CP417' -Description 'historical CP416-to-CP417 interval'
    Assert-Contains -Path $file.FullName -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp416-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-assignment.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP416-to-CP417 registration order drift' }

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 371','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp417Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 131) { throw 'CP417 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp417-cooling-post-saturation-capacity-limit-dehumidification-supply-enthalpy-assignment\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 371 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 131 \|' -Description 'generated internal total'

if ((Get-Content -LiteralPath 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1').Count -gt 1200) { throw 'CP345 line cap exceeded after CP417 terminal propagation' }
Write-Host 'CP417 post-saturation supply-enthalpy assignment structure audit passed.'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment' -Description 'CP425 binding successor registration'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment' -Description 'CP426 binding successor registration'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'
