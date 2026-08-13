# CP411 maps PurchasedAirManager.cc physical executable line 2313's local original-humidity-ratio copy.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break'
$successorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignment'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$routes = "$root\transition\routes.rs"
$tests = "$root\tests.rs"
$exhaustiveTests = "$root\tests\exhaustive.rs"
$release = "$root\release.rs"
$error = "$root\release\error.rs"
$prefixValidation = "$root\release\prefix_validation.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledLineage = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation\lineage.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp411.rs'
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineValidationTests = "crates\ep_run\src\pipeline\$pipelineStem\validation\tests.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp411_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp410_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp411-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-pre-saturation-original-assignment.ps1'

function Assert-Cp411Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP411 $Description missing '$Pattern'" }
}

function Get-Cp411BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP411 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP411 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP411 $Description closing brace missing"
}

$required = @(
    $source,$module,$state,$transition,$accounting,$routes,$tests,$exhaustiveTests,$release,$error,
    $prefixValidation,$runtimeValidation,$snapshotValidation,$privateCharacterization,
    $binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledLineage,$coupledTests,
    $coupledFixture,$witness,$pipelineRoot,$pipeline,$pipelineValidation,
    $pipelineValidationTests,$pipelineLineage,$serialization,$snapshotJsonTests,
    $arbitrary,$arbitraryPredecessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP411 implementation/audit file' }
foreach ($file in @(
    $module,$state,$transition,$accounting,$routes,$tests,$exhaustiveTests,$release,$error,
    $prefixValidation,$runtimeValidation,$snapshotValidation,$privateCharacterization,
    $adapter,$adapterTests,$coupled,$coupledLineage,$coupledTests,$coupledFixture,$witness,
    $pipeline,$pipelineValidation,$pipelineValidationTests,$pipelineLineage,$serialization,
    $snapshotJsonTests,$arbitrary,$audit
)) { Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP411 file' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) {
    throw 'CP411 PurchasedAirManager.cc SHA-256 drift'
}
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2312].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;' -or
    $sourceLines[2313].Trim() -cne 'SupplyHumRatSat = PsyWFnTdbRhPb(state, PurchAir.SupplyTemp, 1.0, state.dataEnvrn->OutBaroPress, RoutineName);' -or
    $sourceLines[2314].Trim() -cne 'if (SupplyHumRatSat < SupplyHumRatOrig) {') {
    throw 'CP411 source/first-excluded/continuation boundary drift'
}

$moduleText = Read-RepoText -Path $module
Assert-Cp411Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2313' -Description 'source constant'
Assert-Cp411Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2314' -Description 'first excluded constant'
$orderMatch = [regex]::Match($moduleText, '(?s)PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP411 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSites = @(
    'read-purchased-air-supply-humidity-ratio-before-saturation-limit',
    'assign-local-original-supply-humidity-ratio-before-saturation-limit'
)
if (($sites -join '|') -cne ($expectedSites -join '|')) { throw 'CP411 two-site source order drift' }

$snapshotStruct = Get-Cp411BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$expectedFields = @(
    'source','first_excluded_source','source_order','system','parent_call_ordinal','controlled_zone',
    'unit_off_skipped','non_cooling_skipped','positive_guard_false_fallthrough_skipped',
    'heating_availability_guard_false_fallthrough','humidification_control_guard_false_fallthrough',
    'dehumidification_control_humidistat_maximum_assignment_executed','dehumidification_control_none_maximum_assignment_executed','dehumidification_control_guard_false_fallthrough',
    'predecessor_capacity_limit_guard_evaluated','predecessor_capacity_limit_body_entered','predecessor_active_capacity_limit_guard_false_fallthrough',
    'predecessor_dehumidification_guard_evaluated','predecessor_dehumidification_body_entered','predecessor_dehumidification_guard_false_fallthrough',
    'predecessor_dehumidification_total_output_assignment_executed','predecessor_dehumidification_total_output_capacity_guard_evaluated',
    'predecessor_dehumidification_total_output_capacity_adjustment_body_entered','predecessor_dehumidification_total_output_capacity_guard_false_fallthrough',
    'dehumidification_total_output_capacity_guard_false_fallthrough','dehumidification_total_output_maximum_capacity_assignment_executed',
    'predecessor_supply_enthalpy_assignment_executed','predecessor_dehumidification_control_type_read','predecessor_dehumidification_control_type',
    'predecessor_dehumidification_control_switch_dispatched','predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered',
    'predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break','predecessor_dehumidification_control_humidistat_case_entered',
    'predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed','predecessor_dehumidification_control_humidistat_case_exited_via_break',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break',
    'predecessor_cp409_resulting_supply_humidity_ratio','predecessor_cp409_resulting_supply_enthalpy_j_per_kg','predecessor_cp409_resulting_supply_temperature_c',
    'predecessor_dehumidification_control_default_case_exited_via_break',
    'predecessor_cp410_resulting_supply_humidity_ratio','predecessor_cp410_resulting_supply_enthalpy_j_per_kg','predecessor_cp410_resulting_supply_temperature_c',
    'post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed',
    'cp410_retained_supply_humidity_ratio_state_owned','cp410_retained_supply_enthalpy_state_owned','cp410_retained_supply_temperature_state_owned',
    'cp410_retained_supply_humidity_ratio_owned_read','purchased_air_supply_humidity_ratio_read',
    'purchased_air_supply_humidity_ratio_before_saturation_check','local_supply_humidity_ratio_original_assignment_performed',
    'assigned_supply_humidity_ratio_original','resulting_supply_humidity_ratio_original',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
if ($fields.Count -ne 59 -or $expectedFields.Count -ne 59 -or ($fields -join '|') -cne ($expectedFields -join '|')) {
    throw 'CP411 snapshot must expose exactly 59 canonical fields'
}
$expectedNumeric = @(
    'predecessor_cp409_resulting_supply_humidity_ratio','predecessor_cp409_resulting_supply_enthalpy_j_per_kg','predecessor_cp409_resulting_supply_temperature_c',
    'predecessor_cp410_resulting_supply_humidity_ratio','predecessor_cp410_resulting_supply_enthalpy_j_per_kg','predecessor_cp410_resulting_supply_temperature_c',
    'purchased_air_supply_humidity_ratio_before_saturation_check','assigned_supply_humidity_ratio_original','resulting_supply_humidity_ratio_original',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
if (($numericFields -join '|') -cne ($expectedNumeric -join '|')) { throw 'CP411 twelve numeric carrier fields drift' }
if ([regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP411 optional control enum drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*30\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @(
    'predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts',
    'predecessor_maximum_capacity_assignment_route_counts',
    'supply_humidity_ratio_pre_saturation_original_assignment_route_counts'
)
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP411 four width-30 route arrays drift' }
foreach ($counter in @(
    'transition_count','inactive_transition_count','predecessor_guard_false_fallthrough_count',
    'predecessor_maximum_capacity_assignment_count','supply_humidity_ratio_pre_saturation_original_assignment_count',
    'source_site_execution_count','cp410_supply_humidity_ratio_state_owner_count',
    'unchanged_supply_humidity_ratio_preservation_count','cp410_supply_enthalpy_state_owner_count',
    'unchanged_supply_enthalpy_preservation_count','cp410_supply_temperature_state_owner_count',
    'unchanged_supply_temperature_preservation_count','cp410_retained_supply_humidity_ratio_owned_read_count',
    'purchased_air_supply_humidity_ratio_before_saturation_limit_read_count',
    'local_supply_humidity_ratio_original_assignment_write_count'
)) {
    Assert-Cp411Text -Text $stateText -Pattern ("pub\s+" + [regex]::Escape($counter) + "\s*:\s*usize") -Description 'state counter'
}
Assert-NotContains -Path $state -Pattern 'enthalpy_owned_read_count|temperature_owned_read_count|enthalpy.*read_count|temperature.*read_count' -Description 'forbidden H/T source reads'

$transitionText = Read-RepoText -Path $transition
$transitionBlock = Get-Cp411BraceBlock -Text $transitionText -AnchorPattern "fn\s+advance_$($stem)_state\s*\(" -Description 'transition'
Assert-Cp411Text -Text $transitionText -Pattern 'ControlDefaultCaseBreakSnapshot as Predecessor' -Description 'sole CP410 predecessor type'
Assert-Cp411Text -Text $transitionBlock -Pattern '(?s)let\s+active\s*=\s*route_is_active\(route\).*?let\s+value\s*=\s*if\s+active\s*\{\s*Some\(predecessor\.resulting_supply_humidity_ratio\?\)\s*\}\s*else\s*\{\s*None\s*\}' -Description 'CP410 W sole active operand'
foreach ($pattern in @(
    'post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed:\s*active',
    'cp410_retained_supply_humidity_ratio_owned_read:\s*active','purchased_air_supply_humidity_ratio_read:\s*active',
    'purchased_air_supply_humidity_ratio_before_saturation_check:\s*value',
    'local_supply_humidity_ratio_original_assignment_performed:\s*active',
    'assigned_supply_humidity_ratio_original:\s*value','resulting_supply_humidity_ratio_original:\s*value',
    'resulting_supply_humidity_ratio:\s*predecessor\.resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg:\s*predecessor\.resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c:\s*predecessor\.resulting_supply_temperature_c'
)) { Assert-Cp411Text -Text $transitionBlock -Pattern $pattern -Description 'source copy and carrier preservation' }
foreach ($forbidden in @('DirectZonePurchasedAirCouplingInput','ZoneHeatBalanceState','energyplus_psy','Psy[A-Z]','f64::min','f64::max','\.min\s*\(','\.max\s*\(','is_finite\s*\(','clamp\s*\(','mul_add\s*\(')) {
    Assert-Cp411Text -Text $transitionBlock -Pattern "^(?![\s\S]*$forbidden)[\s\S]*$" -Description 'copy-only transition'
}

Assert-Contains -Path $routes -Pattern 'active:\s*matches!\(route\.predecessor_index,\s*18\.\.=29\)' -Description 'underlying active routes 18 through 29'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)' -Description 'six split predecessor indices'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description 'thirteen-route public reconstruction'
Assert-Contains -Path $routes -Pattern '(?s)predecessor_has_supply_humidity_ratio.*?route_is_active\(route\)' -Description 'W-presence equals active set'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)' -Description 'H-presence mapping'
Assert-Contains -Path $routes -Pattern '(?s)predecessor_has_supply_temperature.*?index\s*>=\s*3' -Description 'T-presence mapping'
Assert-Contains -Path $accounting -Pattern 'supply_humidity_ratio_pre_saturation_original_assignment_count\s*\+=\s*1' -Description 'assignment count increment'
Assert-Contains -Path $accounting -Pattern 'source_site_execution_count\s*\+=\s*ORDER\.len\(\)' -Description 'two-site increment'
foreach ($counter in @(
    'cp410_retained_supply_humidity_ratio_owned_read_count',
    'purchased_air_supply_humidity_ratio_before_saturation_limit_read_count',
    'local_supply_humidity_ratio_original_assignment_write_count'
)) { Assert-Contains -Path $accounting -Pattern ([regex]::Escape($counter) + '\s*\+=\s*1') -Description 'active read/write counter increment' }

Assert-Contains -Path $release -Pattern 'ControlDefaultCaseBreakSnapshot as Predecessor' -Description 'exact CP410 public predecessor'
Assert-Contains -Path $prefixValidation -Pattern 'completed_direct_.*control_default_case_break_is_consistent' -Description 'recursive CP410 completion'
$releaseText = Read-RepoText -Path $release
Assert-Cp411Text -Text $releaseText -Pattern "(?s)pub fn advance_direct_no_oa_calc_$stem\s*\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor_cp410:\s*Predecessor,\s*\)" -Description 'operand-free public signature'
foreach ($file in @($transition,$accounting,$routes,$release,$prefixValidation,$runtimeValidation,$snapshotValidation,$adapter,$coupled,$coupledLineage,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}
$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine + (Read-RepoText -Path $exhaustiveTests)
foreach ($pattern in @(
    'test_counts_are_exact','transition_count,\s*36',
    'supply_humidity_ratio_pre_saturation_original_assignment_count,\s*18',
    'inactive_transition_count,\s*18','source_site_execution_count,\s*36',
    'active_public.*?4|public_active.*?4','active_private.*?14|private_active.*?14'
)) { Assert-Cp411Text -Text $testText -Pattern "(?is)$pattern" -Description '36/18/18/36 exhaustive characterization' }
foreach ($pattern in @(
    'inactive_total.*?index\s*<\s*18',
    'matches!\(index,\s*18\.\.=29\).*?assignment.*?predecessor',
    'checked_mul\(ORDER\.len\(\)\).*?source_site_execution_count',
    'cp410_supply_humidity_ratio_state_owner_count.*?supply_humidity_ratio_pre_saturation_original_assignment_count',
    'cp410_supply_enthalpy_state_owner_count.*?enthalpy_total',
    'cp410_supply_temperature_state_owner_count.*?temperature_total',
    'cp410_retained_supply_humidity_ratio_owned_read_count.*?supply_humidity_ratio_pre_saturation_original_assignment_count',
    'purchased_air_supply_humidity_ratio_before_saturation_limit_read_count.*?supply_humidity_ratio_pre_saturation_original_assignment_count',
    'local_supply_humidity_ratio_original_assignment_write_count.*?supply_humidity_ratio_pre_saturation_original_assignment_count'
)) { Assert-Contains -Path $runtimeValidation -Pattern "(?s)$pattern" -Description 'exact CP411 runtime accounting' }
Assert-Contains -Path $snapshotValidation -Pattern 'to_bits\(\)' -Description 'raw IEEE snapshot equality'

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=","let\s+calculation_$successorStem\s*=",'let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP410-to-CP411-to-CP412-to-CP413-to-CP414-to-CP415 binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:","pub\s+calculation_$successorStem\s*:",'pub\s+coupling\s*:') -Description 'CP410-to-CP411-to-CP412 scheduled output order'
$bindingText = Read-RepoText -Path $binding
if ([regex]::Matches($bindingText,"\bcalculation_$predecessorStem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$stem\b").Count -ne 3) { throw 'CP411 binding evidence occurrence drift' }
$dto = Get-Cp411BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp410|cp411|cp412|default_case_break|pre_saturation_original_assignment|saturation_supply_humidity_ratio') { throw 'CP410/CP411/CP412 evidence entered numerical DTO' }
Assert-Contains -Path $coupledLineage -Pattern 'option_bits_equal|to_bits' -Description 'bit-exact coupled lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $witness -Pattern ("set_" + $stem + "_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle","$successorStem::\s*validate_direct_lifecycle") -Description 'pipeline CP410-to-CP411-to-CP412 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp424_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp410\s*:\s*Option<&PredecessorLifecycle>' -Description 'sole pipeline predecessor'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp411_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp411_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp411_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-Contains -Path $arbitrary -Pattern 'mod\s+cp412_assertions' -Description 'CP411 arbitrary successor module'
Assert-Contains -Path $arbitrary -Pattern 'cp412_assertions::assert_direct\(runtime,\s*results\)' -Description 'CP411 direct arbitrary successor delegation'
Assert-Contains -Path $arbitrary -Pattern 'cp412_assertions::assert_non_direct\(runtime\)' -Description 'CP411 non-direct arbitrary successor delegation'
Assert-Contains -Path $arbitrary -Pattern 'Some\(71\)' -Description 'arbitrary 71-key schema'
Assert-Contains -Path $arbitrary -Pattern 'Some\(12\)' -Description 'arbitrary twelve-sidecar schema'

$serializationText = Read-RepoText -Path $serialization
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$numericSet = @{}; foreach ($field in $expectedNumeric) { $numericSet[$field] = $true }
$expectedJson = @(); foreach ($field in $expectedFields) { $expectedJson += $field; if ($numericSet.ContainsKey($field)) { $expectedJson += ($field + '_ieee_bits') } }
if ($jsonKeys.Count -ne 71 -or $expectedJson.Count -ne 71 -or ($jsonKeys -join '|') -cne ($expectedJson -join '|')) { throw 'CP411 JSON must expose 71 canonical keys' }
foreach ($field in $expectedNumeric) {
    $escaped = [regex]::Escape($field)
    Assert-Cp411Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar'
}
Assert-Contains -Path $snapshotJsonTests -Pattern '71.*(?:key|field)|(?:key|field).*71' -Description '71-key JSON regression'

$heading = 'CP411 post-saturation pre-saturation-original supply-humidity-ratio assignment'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP411 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'line 2313 exactly:\s*`SupplyHumRatOrig = PurchAir\.SupplyHumRat;`','two exact.*?source sites',
        'read-purchased-air-supply-humidity-ratio-before-saturation-limit','assign-local-original-supply-humidity-ratio-before-saturation-limit',
        'line 2314.*?first excluded','CP412 candidate','routes 18 through 35 are active','routes 0 through 17.*?inactive',
        '13/23 public/private','20, 21, 26, and 27','fourteen.*?private','T411=T410=36','A411=18','I411=18','source_site_execution_count=2\*A411=36',
        'Three width-30 arrays','one width-30 CP411 array','underlying.*?18 through 29','20, 21, 24, 25, 27, and 29',
        'CP410.*?sole immediate route','present on exactly 18, 23, and 33','resulting_supply_humidity_ratio.*?owns the source operand','read.*?exactly 18 times','enthalpy,?\s+and\s+temperature.*?read\s+zero\s+times',
        'raw binary64.*?bit-for-bit','exactly 59 base fields','twelve\s*`Option<f64>`','71\s+unique\s+keys','twelve.*?IEEE-bit sidecars',
        'CP410-to-CP411-to-unchanged-numerical','line 2314.*?same flattened routes 18 through 35','4 public and 14 private',
        '32 algorithms, 293 routines','58\s*`state_mapped`, 235\s*`source_mapped`','170 required','349 total, 240 public, 109 internal','238 development commands'
    )) { Assert-Cp411Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
}
$specAddenda = @(
    [PSCustomObject]@{ Path = 'specs\algorithm_ledger.toml'; Anchor = 'CP411 supersedes only CP410' },
    [PSCustomObject]@{ Path = 'specs\capabilities.toml'; Anchor = 'CP411 additionally requires' }
)
foreach ($specAddendum in $specAddenda) {
    $specText = Read-RepoText -Path $specAddendum.Path
    $pattern = '(?m)^\s*"(?<body>' + [regex]::Escape($specAddendum.Anchor) + '.*)",\r?$'
    $matches = [regex]::Matches($specText, $pattern)
    if ($matches.Count -ne 1) { throw "CP411 expected one bounded addendum in $($specAddendum.Path), found $($matches.Count)" }
    $body = $matches[0].Groups['body'].Value
    foreach ($claim in @(
        'line 2313 exactly','line 2314.*?CP412','flattened.*?18 through 35.*?active','0 through 17.*?inactive',
        '20, 21, 26, and 27','36/18/18/36|T411=T410=36','18/23/33','sole scalar owner','59 base fields','twelve `Option<f64>`','71 JSON keys',
        'CP410-to-CP411-to-unchanged-numerical','line 2314.*?same flattened(?: routes)?(?: 18 through 35| 18-through-35).*?4 public.*?14 private','349 total, 240 public, 109 internal'
    )) { Assert-Cp411Text -Text $body -Pattern "(?is)$claim" -Description "bounded addendum claim in $($specAddendum.Path)" }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP411 supersedes only CP410' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP411 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP411\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP411\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP411 algorithm/routine ledger counts drift' }

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 410) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp424_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 410) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 362' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 410) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 122' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 410) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 362 \|')) -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 122 \|')) -Description 'historical generated internal total'
    }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP411 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminal = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 411)) })
if ($terminal.Count -ne 35) { throw 'CP411 terminal propagation set drift' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP411-to-CP412' -Description 'historical terminal interval' }
$cp345 = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp411Call\s*=','\$cp412Call\s*=','\$cp413Call\s*=','\$cp414Call\s*=','CP410-to-CP411','CP411-to-CP412','CP412-to-CP413','CP413-to-CP414','CP414-to-CP415')) { Assert-Contains -Path $cp345 -Pattern $pattern -Description 'CP345 terminal chain' }
Assert-LineLimit -Path $cp345 -Limit 1201 -Description 'CP345 fixed structural cap'
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405|406|407|408|409|410)-' })) { Assert-Contains -Path $file.FullName -Pattern "calculation_$stem" -Description 'recent CP411 binding order' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406|407|408|409|410)-' })) { Assert-Contains -Path $file.FullName -Pattern '\$cp411Call' -Description 'recent CP411 terminal capture' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405|406|407|408|409|410|411)-' })) { Assert-Contains -Path $file.FullName -Pattern "calculation_$successorStem" -Description 'recent CP412 binding order' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406|407|408|409|410|411)-' })) { Assert-Contains -Path $file.FullName -Pattern '\$cp412Call' -Description 'recent CP412 terminal capture' }
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp410-cooling-post-saturation-capacity-limit-dehumidification-control-default-case-break.ps1' -Pattern 'calculation_\$stem\\b"\)\.Count -ne 3' -Description 'CP410 successor-consumption binding count'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 99' -Description 'calc-root structural cap'

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp410-cooling-post-saturation-capacity-limit-dehumidification-control-default-case-break.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP411 registration order drift' }

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 362','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp411Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 122) { throw 'CP411 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp411-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-pre-saturation-original-assignment\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 362 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 122 \|' -Description 'generated internal total'

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP411-to-CP412' -Description 'CP345 CP411-to-CP412 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\s*=' -Description 'CP412 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp412Call\s*=' -Description 'CP345 CP412 call capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP412-to-CP413' -Description 'CP345 CP412-to-CP413 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP413-to-CP414' -Description 'CP345 CP413-to-CP414 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\s*=' -Description 'CP413 historical binding order'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\s*=' -Description 'CP414 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp413Call\s*=' -Description 'CP345 CP413 call capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp414Call\s*=' -Description 'CP345 CP414 call capture'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\s*=' -Description 'CP415 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp415Call' -Description 'CP415 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP414-to-CP415' -Description 'CP414-to-CP415 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\s*=' -Description 'CP416 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp416Call' -Description 'CP416 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP415-to-CP416' -Description 'CP415-to-CP416 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp417Call' -Description 'CP417 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP416-to-CP417' -Description 'CP416-to-CP417 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-numerical' -Description 'CP424 terminal-to-numerical interval'
Write-Host 'CP411 post-saturation pre-saturation-original humidity-ratio assignment structure audit passed.'
}
