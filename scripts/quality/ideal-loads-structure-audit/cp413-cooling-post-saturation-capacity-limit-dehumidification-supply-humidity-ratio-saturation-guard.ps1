# CP413 maps PurchasedAirManager.cc physical executable line 2315's strict local humidity-ratio guard.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment'
$successorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment'
$terminalStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuard'
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
$release = "$root\release.rs"
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
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp413.rs'
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
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp413_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp412_assertions.rs'
$arbitrarySuccessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp414_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp413-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-saturation-guard.ps1'

function Assert-Cp413Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP413 $Description missing '$Pattern'" }
}

function Get-Cp413BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP413 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP413 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP413 $Description closing brace missing"
}

$required = @(
    $source,$module,$state,$transition,$accounting,$routes,$tests,$release,$prefixValidation,
    $runtimeValidation,$snapshotValidation,$privateCharacterization,$binding,$scheduledOutput,
    $adapter,$adapterTests,$coupled,$coupledLineage,$coupledTests,$coupledFixture,$witness,
    $pipelineRoot,$pipeline,$pipelineValidation,$pipelineValidationTests,$pipelineLineage,
    $serializationRoot,$serialization,$snapshotJsonTests,$arbitrary,$arbitraryPredecessor,$arbitrarySuccessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP413 implementation/audit file' }
foreach ($file in @($module,$adapter,$adapterTests,$coupled,$coupledLineage,$coupledTests,
    $coupledFixture,$witness,$pipeline,$pipelineValidation,$pipelineValidationTests,$pipelineLineage,
    $serializationRoot,$serialization,$snapshotJsonTests,$arbitrary,$audit)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP413 file'
}
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
if ($coreFiles.Count -ne 15) { throw 'CP413 exact fifteen-file bounded core subtree drift' }
$coreText = ($coreFiles | ForEach-Object {
    Assert-LineLimit -Path $_.FullName -Limit 500 -Description 'bounded CP413 core file'
    Read-RepoText -Path $_.FullName
}) -join [Environment]::NewLine
$coreTests = @(Get-ChildItem -LiteralPath "$root\tests" -File -Filter '*.rs')
if ($coreTests.Count -ne 4) { throw 'CP413 requires exactly four bounded split core test files' }
$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine + (($coreTests | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine)

$registrations = @(
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\calc.rs'; Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs'; Pattern="mod\s+$stem" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs'; Pattern="$($stem)_tests" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs'; Pattern="mod\s+$($stem)_validation" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs'; Pattern='test_coupled_runtime_cp413\.rs' },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs'; Pattern="$($stem)_fixture" },
    [PSCustomObject]@{ Path=$module; Pattern='(?s)mod\s+release;.*?mod\s+state;.*?mod\s+tests;.*?mod\s+transition;' },
    [PSCustomObject]@{ Path=$tests; Pattern='(?s)mod\s+exhaustive;.*?mod\s+ieee;.*?mod\s+overflow;.*?mod\s+release;' },
    [PSCustomObject]@{ Path=$release; Pattern='(?s)mod\s+error;.*?mod\s+prefix_validation;.*?mod\s+private_characterization;.*?mod\s+runtime_validation;.*?mod\s+snapshot_validation;' },
    [PSCustomObject]@{ Path=$pipelineRoot; Pattern="mod\s+$pipelineStem;" },
    [PSCustomObject]@{ Path=$pipeline; Pattern='(?s)mod\s+serialization;.*?mod\s+validation;' },
    [PSCustomObject]@{ Path=$pipelineValidation; Pattern='(?s)mod\s+lineage;.*?mod\s+tests;' },
    [PSCustomObject]@{ Path=$serializationRoot; Pattern='mod\s+snapshot;' },
    [PSCustomObject]@{ Path=$serialization; Pattern='mod\s+tests;' }
)
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP413 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP413 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2313].Trim() -cne 'SupplyHumRatSat = PsyWFnTdbRhPb(state, PurchAir.SupplyTemp, 1.0, state.dataEnvrn->OutBaroPress, RoutineName);' -or
    $sourceLines[2314].Trim() -cne 'if (SupplyHumRatSat < SupplyHumRatOrig) {' -or
    $sourceLines[2315].Trim() -cne 'PurchAir.SupplyTemp = PsyTsatFnHPb(state, SupplyEnthalpy, state.dataEnvrn->OutBaroPress, RoutineName);') {
    throw 'CP413 source/first-excluded boundary drift'
}

$moduleText = Read-RepoText -Path $module
Assert-Cp413Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2315' -Description 'source constant'
Assert-Cp413Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2316' -Description 'first excluded constant'
$orderMatch = [regex]::Match($moduleText, '(?s)SATURATION_GUARD_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP413 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSites = @(
    'read-local-saturation-supply-humidity-ratio-for-saturation-guard',
    'read-local-original-supply-humidity-ratio-for-saturation-guard',
    'compare-local-saturation-supply-humidity-ratio-strictly-less-than-local-original-supply-humidity-ratio',
    'enter-saturation-supply-humidity-ratio-guard-body-if-comparison-satisfied'
)
if (($sites -join '|') -cne ($expectedSites -join '|')) { throw 'CP413 four-site source order drift' }

$snapshotStruct = Get-Cp413BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$expectedFields = @(
    'source','first_excluded_source','source_order','system','parent_call_ordinal','controlled_zone','unit_off_skipped','non_cooling_skipped','positive_guard_false_fallthrough_skipped','heating_availability_guard_false_fallthrough','humidification_control_guard_false_fallthrough','dehumidification_control_humidistat_maximum_assignment_executed','dehumidification_control_none_maximum_assignment_executed','dehumidification_control_guard_false_fallthrough',
    'predecessor_capacity_limit_guard_evaluated','predecessor_capacity_limit_body_entered','predecessor_active_capacity_limit_guard_false_fallthrough','predecessor_dehumidification_guard_evaluated','predecessor_dehumidification_body_entered','predecessor_dehumidification_guard_false_fallthrough','predecessor_dehumidification_total_output_assignment_executed','predecessor_dehumidification_total_output_capacity_guard_evaluated','predecessor_dehumidification_total_output_capacity_adjustment_body_entered','predecessor_dehumidification_total_output_capacity_guard_false_fallthrough','dehumidification_total_output_capacity_guard_false_fallthrough','dehumidification_total_output_maximum_capacity_assignment_executed','predecessor_supply_enthalpy_assignment_executed','predecessor_dehumidification_control_type_read','predecessor_dehumidification_control_type','predecessor_dehumidification_control_switch_dispatched','predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered','predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break','predecessor_dehumidification_control_humidistat_case_entered','predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed','predecessor_dehumidification_control_humidistat_case_exited_via_break','predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered','predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough','predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed','predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break',
    'predecessor_cp409_resulting_supply_humidity_ratio','predecessor_cp409_resulting_supply_enthalpy_j_per_kg','predecessor_cp409_resulting_supply_temperature_c','predecessor_dehumidification_control_default_case_exited_via_break','predecessor_cp410_resulting_supply_humidity_ratio','predecessor_cp410_resulting_supply_enthalpy_j_per_kg','predecessor_cp410_resulting_supply_temperature_c','post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed','cp410_retained_supply_humidity_ratio_state_owned','cp410_retained_supply_enthalpy_state_owned','cp410_retained_supply_temperature_state_owned','cp410_retained_supply_humidity_ratio_owned_read','purchased_air_supply_humidity_ratio_read','purchased_air_supply_humidity_ratio_before_saturation_check','local_supply_humidity_ratio_original_assignment_performed','assigned_supply_humidity_ratio_original','resulting_supply_humidity_ratio_original','predecessor_cp411_resulting_supply_humidity_ratio','predecessor_cp411_resulting_supply_enthalpy_j_per_kg','predecessor_cp411_resulting_supply_temperature_c',
    'post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed','cp411_retained_supply_humidity_ratio_state_owned','cp411_retained_supply_enthalpy_state_owned','cp411_retained_supply_temperature_state_owned','cp411_retained_supply_temperature_owned_read','purchased_air_supply_temperature_for_saturation_humidity_ratio_read','supply_temperature_for_saturation_humidity_ratio_c','environment_outdoor_barometric_pressure_owned_read','environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read','outdoor_barometric_pressure_pa','psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated','saturation_supply_humidity_ratio','local_saturation_supply_humidity_ratio_assignment_performed','assigned_saturation_supply_humidity_ratio','resulting_saturation_supply_humidity_ratio','predecessor_cp412_resulting_supply_humidity_ratio','predecessor_cp412_resulting_supply_enthalpy_j_per_kg','predecessor_cp412_resulting_supply_temperature_c',
    'post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated','cp412_saturation_supply_humidity_ratio_owned_read','saturation_supply_humidity_ratio_for_guard_read','saturation_supply_humidity_ratio_for_guard','cp411_original_supply_humidity_ratio_owned_read','cp412_same_call_original_supply_humidity_ratio_bit_corroborated','original_supply_humidity_ratio_for_guard_read','original_supply_humidity_ratio_for_guard','saturation_original_supply_humidity_ratio_comparison_evaluated','saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio','saturation_supply_humidity_ratio_guard_body_entered','saturation_supply_humidity_ratio_guard_false_fallthrough','cp412_retained_supply_humidity_ratio_state_owned','cp412_retained_supply_enthalpy_state_owned','cp412_retained_supply_temperature_state_owned','resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
if ($fields.Count -ne 95 -or $expectedFields.Count -ne 95 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP413 snapshot must expose exactly 95 canonical fields' }
$expectedNumeric = @(
    'predecessor_cp409_resulting_supply_humidity_ratio','predecessor_cp409_resulting_supply_enthalpy_j_per_kg','predecessor_cp409_resulting_supply_temperature_c','predecessor_cp410_resulting_supply_humidity_ratio','predecessor_cp410_resulting_supply_enthalpy_j_per_kg','predecessor_cp410_resulting_supply_temperature_c','purchased_air_supply_humidity_ratio_before_saturation_check','assigned_supply_humidity_ratio_original','resulting_supply_humidity_ratio_original','predecessor_cp411_resulting_supply_humidity_ratio','predecessor_cp411_resulting_supply_enthalpy_j_per_kg','predecessor_cp411_resulting_supply_temperature_c','supply_temperature_for_saturation_humidity_ratio_c','outdoor_barometric_pressure_pa','saturation_supply_humidity_ratio','assigned_saturation_supply_humidity_ratio','resulting_saturation_supply_humidity_ratio','predecessor_cp412_resulting_supply_humidity_ratio','predecessor_cp412_resulting_supply_enthalpy_j_per_kg','predecessor_cp412_resulting_supply_temperature_c','saturation_supply_humidity_ratio_for_guard','original_supply_humidity_ratio_for_guard','resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
if ($numericFields.Count -ne 25 -or ($numericFields -join '|') -cne ($expectedNumeric -join '|')) { throw 'CP413 twenty-five numeric carrier fields drift' }
if ([regex]::Matches($snapshotStruct, 'Option<bool>').Count -ne 1 -or [regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP413 optional comparison/control carrier drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @('predecessor_route_counts','guard_false_fallthrough_route_counts','guard_body_entry_route_counts')
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP413 three width-36 route arrays drift' }
$counters = @(
    'transition_count','inactive_transition_count','saturation_supply_humidity_ratio_guard_evaluation_count','source_site_execution_count',
    'cp412_supply_humidity_ratio_state_owner_count','unchanged_supply_humidity_ratio_preservation_count',
    'cp412_supply_enthalpy_state_owner_count','unchanged_supply_enthalpy_preservation_count',
    'cp412_supply_temperature_state_owner_count','unchanged_supply_temperature_preservation_count',
    'cp412_saturation_supply_humidity_ratio_owned_read_count','saturation_supply_humidity_ratio_for_guard_read_count',
    'cp411_original_supply_humidity_ratio_owned_read_count','cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count',
    'original_supply_humidity_ratio_for_guard_read_count','saturation_original_supply_humidity_ratio_comparison_count',
    'saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count',
    'saturation_supply_humidity_ratio_guard_body_entry_count','saturation_supply_humidity_ratio_guard_false_fallthrough_count'
)
foreach ($counter in $counters) { Assert-Cp413Text -Text $stateText -Pattern ("pub\s+" + [regex]::Escape($counter) + "\s*:\s*usize") -Description 'state counter' }

$transitionText = Read-RepoText -Path $transition
Assert-Cp413Text -Text $transitionText -Pattern 'SaturationAssignmentSnapshot as Predecessor' -Description 'sole CP412 predecessor type'
foreach ($pattern in @(
    'let\s+active\s*=\s*route_is_active\(base_route\)',
    'predecessor\.resulting_saturation_supply_humidity_ratio\?',
    'predecessor\.resulting_supply_humidity_ratio_original\?',
    'predecessor\.predecessor_cp411_resulting_supply_humidity_ratio\?',
    'original\.to_bits\(\)\s*!=\s*cp411_terminal\.to_bits\(\)',
    'source_strict_less_than\(saturation,\s*original\)',
    'saturation_supply_humidity_ratio_guard_body_entered:\s*comparison\s*==\s*Some\(true\)',
    'saturation_supply_humidity_ratio_guard_false_fallthrough:\s*comparison\s*==\s*Some\(false\)'
)) { Assert-Cp413Text -Text $transitionText -Pattern $pattern -Description 'strict guard transition contract' }
$comparisonBlock = Get-Cp413BraceBlock -Text $transitionText -AnchorPattern 'fn\s+source_strict_less_than\s*\(' -Description 'raw strict comparison'
Assert-Cp413Text -Text $comparisonBlock -Pattern 'fn\s+source_strict_less_than\(left:\s*f64,\s*right:\s*f64\)\s*->\s*bool\s*\{\s*left\s*<\s*right\s*\}' -Description 'raw strict comparison shape'
foreach ($forbidden in @('ActiveInput','PsyTsatFnHPb','energyplus_psy','\.is_finite\s*\(','f64::min','f64::max','\.min\s*\(','\.max\s*\(','clamp\s*\(','epsilon|tolerance|approx','DirectZonePurchasedAirCouplingInput')) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description 'pure guard forbidden dependency'
    Assert-NotContains -Path $privateCharacterization -Pattern $forbidden -Description 'private guard forbidden dependency'
}
Assert-Contains -Path $routes -Pattern 'route\.active\s*&&\s*matches!\(route\.predecessor_index,\s*18\.\.=29\)' -Description 'underlying active predecessor routes'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)' -Description 'six split predecessor indices'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description 'public predecessor reconstruction'
Assert-Contains -Path $accounting -Pattern 'source_site_execution_count\s*\+=\s*3\s*\+\s*usize::from\(route\.body_entered\)' -Description 'three-or-four-site increment'
foreach ($counter in @('cp412_saturation_supply_humidity_ratio_owned_read_count','saturation_supply_humidity_ratio_for_guard_read_count','cp411_original_supply_humidity_ratio_owned_read_count','cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count','original_supply_humidity_ratio_for_guard_read_count','saturation_original_supply_humidity_ratio_comparison_count')) {
    Assert-Contains -Path $accounting -Pattern ([regex]::Escape($counter) + '\s*\+=\s*1') -Description 'active guard counter increment'
}

$releaseText = Read-RepoText -Path $release
$publicRelease = Get-Cp413BraceBlock -Text $releaseText -AnchorPattern "pub\s+fn\s+advance_direct_no_oa_calc_$stem\s*\(" -Description 'public release'
Assert-Cp413Text -Text $publicRelease -Pattern 'predecessor_cp412:\s*Predecessor\s*,' -Description 'operand-free CP412-only public signature'
foreach ($pattern in @(
    'predecessor_cp412\s*\.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed',
    'predecessor_cp412\s*\.resulting_saturation_supply_humidity_ratio',
    'predecessor_cp412\s*\.resulting_supply_humidity_ratio_original',
    'predecessor_cp412\s*\.predecessor_cp411_resulting_supply_humidity_ratio',
    'original\.to_bits\(\)\s*!=\s*cp411_terminal\.to_bits\(\)',
    'saturation\.is_finite\(\)','original\.is_finite\(\)',
    'SaturationHumidityRatioOutsideDirectSubset','OriginalHumidityRatioOutsideDirectSubset'
)) { Assert-Cp413Text -Text $publicRelease -Pattern "(?s)$pattern" -Description 'active-scoped public finite/ownership gate' }
foreach ($file in @($transition,$accounting,$routes,$release,$prefixValidation,$runtimeValidation,$snapshotValidation,$adapter,$coupled,$coupledLineage,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}
foreach ($pattern in @(
    'exhaustive_54_outcome_characterization_is_exact','public_outcomes,\s*17','private_outcomes,\s*37',
    'transition_count,\s*54','inactive_transition_count,\s*18',
    'saturation_supply_humidity_ratio_guard_evaluation_count,\s*36','source_site_execution_count,\s*126',
    'guard_false_fallthrough_count,\s*18','guard_body_entry_count,\s*18',
    'cp412_supply_humidity_ratio_state_owner_count,\s*36','cp412_supply_enthalpy_state_owner_count,\s*41',
    'cp412_supply_temperature_state_owner_count,\s*51','raw_strict_less_than_covers_ieee_edges',
    'NEG_INFINITY','INFINITY','-0\.0','from_bits\(1\)','NaN|nan','usize::MAX','transactional'
)) { Assert-Cp413Text -Text $testText -Pattern "(?is)$pattern" -Description 'exhaustive/IEEE/overflow characterization' }
foreach ($pattern in @(
    'predecessor_total\s*==\s*Some\(state\.transition_count\)',
    'inactive_total\s*==\s*Some\(state\.inactive_transition_count\)',
    'checked_mul\(3\)',
    'cp412_supply_humidity_ratio_state_owner_count','cp412_supply_enthalpy_state_owner_count',
    'cp412_supply_temperature_state_owner_count','guard_false_fallthrough_route_counts','guard_body_entry_route_counts'
)) { Assert-Contains -Path $runtimeValidation -Pattern "(?s)$pattern" -Description 'exact CP413 runtime accounting' }
Assert-Contains -Path $snapshotValidation -Pattern 'to_bits\(\)' -Description 'raw IEEE snapshot equality'
Assert-Contains -Path $privateCharacterization -Pattern 'SaturationAssignmentSnapshot as Predecessor' -Description 'private CP412-only characterization'

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=","let\s+calculation_$successorStem\s*=","let\s+calculation_$terminalStem\s*=",'let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP412-to-CP413-to-CP414-to-CP415-to-numerical binding order'
Assert-Contains -Path $binding -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\s*=' -Description 'CP414 binding successor'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:","pub\s+calculation_$successorStem\s*:","pub\s+calculation_$terminalStem\s*:",'pub\s+coupling\s*:') -Description 'CP412-to-CP413-to-CP414-to-CP415 scheduled output order'
$bindingText = Read-RepoText -Path $binding
if ([regex]::Matches($bindingText,"\bcalculation_$predecessorStem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$stem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$successorStem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$terminalStem\b").Count -ne 3) { throw 'CP412/CP413/CP414/CP415 binding evidence occurrence drift' }
$dto = Get-Cp413BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp412|cp413|cp414|cp415|saturation_guard|saturation_supply_humidity_ratio|original_supply_humidity_ratio|supply_temperature_saturation_(assignment|mixed_air_limit)') { throw 'CP412/CP413/CP414/CP415 evidence entered numerical DTO' }
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp413_after_cp412_before_unchanged_numerical_coupling' -Description 'binding order regression'
Assert-Contains -Path $adapterTests -Pattern 'binding_cp413_active_nonfinite_operands_are_transactional_and_fail_closed' -Description 'active nonfinite fail-closed regression'
Assert-Contains -Path $adapterTests -Pattern 'SaturationHumidityRatioOutsideDirectSubset|OriginalHumidityRatioOutsideDirectSubset' -Description 'active operand public error assertion'
Assert-Contains -Path $coupledLineage -Pattern 'option_bits_equal|to_bits' -Description 'bit-exact coupled lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $coupledTests -Pattern 'cp413_evidence_does_not_feed_numerical_result' -Description 'coupled numerical nonfeed regression'
Assert-Contains -Path $witness -Pattern ("set_" + $stem + "_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle","$successorStem::\s*validate_direct_lifecycle","$terminalStem::\s*validate_direct_lifecycle") -Description 'pipeline CP412-to-CP413-to-CP414-to-CP415 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp424_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp412\s*:\s*Option<&PredecessorLifecycle>' -Description 'sole pipeline predecessor'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp413_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp413_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp413_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-Contains -Path $arbitrary -Pattern 'mod\s+cp414_assertions' -Description 'CP413 arbitrary successor module'
Assert-Contains -Path $arbitrary -Pattern 'cp414_assertions::assert_direct\(runtime,\s*results\)' -Description 'CP413 direct arbitrary successor delegation'
Assert-Contains -Path $arbitrary -Pattern 'cp414_assertions::assert_non_direct\(runtime\)' -Description 'CP413 non-direct arbitrary successor delegation'
Assert-Contains -Path $arbitrarySuccessor -Pattern 'mod\s+cp415_assertions' -Description 'CP414 arbitrary successor module'
Assert-Contains -Path $arbitrary -Pattern 'Some\(120\)' -Description 'arbitrary 120-key schema'
Assert-Contains -Path $arbitrary -Pattern 'Some\(25\)' -Description 'arbitrary twenty-five-sidecar schema'

$serializationText = Read-RepoText -Path $serialization
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$numericSet = @{}; foreach ($field in $expectedNumeric) { $numericSet[$field] = $true }
$expectedJson = @(); foreach ($field in $expectedFields) { $expectedJson += $field; if ($numericSet.ContainsKey($field)) { $expectedJson += ($field + '_ieee_bits') } }
if ($jsonKeys.Count -ne 120 -or $expectedJson.Count -ne 120 -or ($jsonKeys -join '|') -cne ($expectedJson -join '|')) { throw 'CP413 JSON must expose 120 canonical keys' }
foreach ($field in $expectedNumeric) {
    $escaped = [regex]::Escape($field)
    Assert-Cp413Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar'
}
Assert-Contains -Path $snapshotJsonTests -Pattern '120.*(?:key|field)|(?:key|field).*120' -Description '120-key JSON regression'
Assert-Contains -Path $snapshotJsonTests -Pattern 'twenty_five_sidecar_schema|twenty-five.*sidecar' -Description 'twenty-five-sidecar JSON regression'
Assert-Contains -Path $snapshotJsonTests -Pattern 'Some\(25\)' -Description 'twenty-five-sidecar JSON count'

$heading = 'CP413 post-saturation saturation supply-humidity-ratio guard'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP413 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'line 2315 exactly','four exact.*?source sites','line 2316.*?first excluded','CP414 candidate',
        'routes 0 through 17.*?inactive','routes 18 through 35.*?evaluated-.*?false.*?body-entered',
        '17/37','20, 21, 26, and 27','fourteen active predecessors','T413=54','I413=18','F413=18','B413=18',
        'source_site_execution_count=3\*F413\+4\*B413=126','Three width-36 arrays','CP412.*?sole immediate',
        'resulting_saturation_supply_humidity_ratio.*?solely owns','resulting_supply_humidity_ratio_original.*?solely owns',
        'predecessor_cp411_resulting_supply_humidity_ratio.*?corroboration','36/41/51','finite saturation and original',
        'inactive routes.*?ignore','NaN.*?unordered.*?false','signed-zero equality is false','no arithmetic, psychrometric',
        'exactly\s+95 base\s+fields','twenty-five `Option<f64>`','one `Option<bool>`','120 unique keys','twenty-five adjacent',
        'CP412-to-CP413-to-unchanged-numerical','no `ActiveInput`','32 algorithms, 293','58 `state_mapped`, 235 `source_mapped`',
        '351 total, 240 public, 111 internal','238 development commands'
    )) { Assert-Cp413Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
}
$specAddenda = @(
    [PSCustomObject]@{ Path='specs\algorithm_ledger.toml'; Anchor='CP413 supersedes only CP412' },
    [PSCustomObject]@{ Path='specs\capabilities.toml'; Anchor='CP413 additionally requires' }
)
foreach ($specAddendum in $specAddenda) {
    $specText = Read-RepoText -Path $specAddendum.Path
    $matches = [regex]::Matches($specText, '(?m)^\s*"(?<body>' + [regex]::Escape($specAddendum.Anchor) + '.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP413 expected one bounded addendum in $($specAddendum.Path), found $($matches.Count)" }
    $body = $matches[0].Groups['body'].Value
    foreach ($claim in @(
        'line[- ]2315|physical executable line 2315','line 2316.*?CP414','54.*?126','17/37','20, 21, 26, and 27',
        'three width-36','resulting_supply_humidity_ratio_original.*?sole','(?:CP411 terminal W|predecessor_cp411_resulting_supply_humidity_ratio).*?corroborat',
        '95 base fields','twenty-five.*?numeric|twenty-five `Option<f64>`','120 JSON keys',
        'CP412-to-CP413-to-unchanged-numerical','351 total, 240 public, 111 internal'
    )) { Assert-Cp413Text -Text $body -Pattern "(?is)$claim" -Description "bounded addendum claim in $($specAddendum.Path)" }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP413 supersedes only CP412' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP413 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP413\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP413\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP413 algorithm/routine ledger counts drift' }
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern '(?s)routine\.psy_w_fn_tdb_rh_pb.*?completion_status\s*=\s*"state_mapped"' -Description 'psychrometric routine preservation'
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern '(?s)routine\.psy_tsat_fn_h_pb\.source_file.*?routine\.psy_tsat_fn_h_pb\.completion_status\s*=\s*"source_mapped"' -Description 'CP414 psychrometric routine status preservation'

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 413) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp424_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 413) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 362' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 413) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 122' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 413) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 362 \|')) -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 122 \|')) -Description 'historical generated internal total'
    }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP413 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminal = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 413)) })
if ($terminal.Count -ne 37) { throw 'CP414 terminal propagation set drift' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP412-to-CP413' -Description 'historical terminal interval' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP413-to-CP414' -Description 'historical terminal interval' }
$cp345 = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp412Call\s*=','\$cp413Call\s*=','\$cp414Call\s*=','CP411-to-CP412','CP412-to-CP413','CP413-to-CP414','CP414-to-CP415')) { Assert-Contains -Path $cp345 -Pattern $pattern -Description 'CP345 terminal chain' }
Assert-LineLimit -Path $cp345 -Limit 1201 -Description 'CP345 fixed structural cap'
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405|406|407|408|409|410|411|412|413)-' })) { Assert-Contains -Path $file.FullName -Pattern "calculation_$successorStem" -Description 'recent CP414 binding order' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406|407|408|409|410|411|412|413)-' })) { Assert-Contains -Path $file.FullName -Pattern '\$cp414Call' -Description 'recent CP414 terminal capture' }

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp412-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-saturation-assignment.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$successorIndex = $master.IndexOf('cp414-cooling-post-saturation-capacity-limit-dehumidification-supply-temperature-saturation-assignment.ps1')
$terminalIndex = $master.IndexOf('cp415-cooling-post-saturation-capacity-limit-dehumidification-supply-temperature-saturation-mixed-air-limit.ps1')
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $successorIndex -le $currentIndex -or $terminalIndex -le $successorIndex -or $completionIndex -le $terminalIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP412-to-CP413-to-CP414-to-CP415 registration order drift' }

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 362','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp413Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 122) { throw 'CP413 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp413-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-saturation-guard\.ps1' -Description 'inventory record'
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp414-cooling-post-saturation-capacity-limit-dehumidification-supply-temperature-saturation-assignment\.ps1' -Description 'CP414 inventory record'
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp415-cooling-post-saturation-capacity-limit-dehumidification-supply-temperature-saturation-mixed-air-limit\.ps1' -Description 'CP415 inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 362 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 122 \|' -Description 'generated internal total'

Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\s*=' -Description 'CP415 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp415Call' -Description 'CP415 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP414-to-CP415' -Description 'CP414-to-CP415 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\s*=' -Description 'CP416 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp416Call' -Description 'CP416 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP415-to-CP416' -Description 'CP415-to-CP416 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp417Call' -Description 'CP417 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP416-to-CP417' -Description 'CP416-to-CP417 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-numerical' -Description 'CP424 terminal-to-numerical interval'
Write-Host 'CP413 post-saturation saturation humidity-ratio guard structure audit passed.'
}
