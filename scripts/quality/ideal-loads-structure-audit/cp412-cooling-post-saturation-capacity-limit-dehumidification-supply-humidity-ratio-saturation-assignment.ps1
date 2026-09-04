# CP412 maps PurchasedAirManager.cc physical executable line 2314's local saturation-humidity-ratio assignment.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment'
$successorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignment'
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
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledLineage = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation\lineage.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp412.rs'
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
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp412_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp411_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp412-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-saturation-assignment.ps1'

function Assert-Cp412Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP412 $Description missing '$Pattern'" }
}

function Get-Cp412BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP412 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP412 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP412 $Description closing brace missing"
}

$required = @(
    $source,$module,$state,$transition,$accounting,$routes,$tests,$release,$runtimeValidation,
    $snapshotValidation,$privateCharacterization,$binding,$scheduledOutput,$adapter,$adapterTests,
    $coupled,$coupledLineage,$coupledTests,$coupledFixture,$witness,$pipelineRoot,$pipeline,
    $pipelineValidation,$pipelineValidationTests,$pipelineLineage,$serializationRoot,$serialization,
    $snapshotJsonTests,$arbitrary,$arbitraryPredecessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP412 implementation/audit file' }
foreach ($file in @($module,$adapter,$adapterTests,$coupled,$coupledLineage,$coupledTests,$coupledFixture,
    $witness,$pipeline,$pipelineValidation,$pipelineValidationTests,$pipelineLineage,$serializationRoot,
    $serialization,$snapshotJsonTests,$arbitrary,$audit)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP412 file'
}
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
if ($coreFiles.Count -ne 15) { throw 'CP412 exact fifteen-file bounded core set drift' }
$coreText = ($coreFiles | ForEach-Object {
    Assert-LineLimit -Path $_.FullName -Limit 500 -Description 'bounded CP412 core file'
    Read-RepoText -Path $_.FullName
}) -join "`n"
$coreTests = @(Get-ChildItem -LiteralPath "$root\tests" -File -Filter '*.rs')
if ($coreTests.Count -ne 4) { throw 'CP412 requires exactly four bounded split core test files' }
$testText = (Read-RepoText -Path $tests) + "`n" + (($coreTests | ForEach-Object { Read-RepoText -Path $_.FullName }) -join "`n")
$registrations = @(
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\calc.rs'; Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs'; Pattern="mod\s+$stem" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs'; Pattern="$($stem)_tests" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs'; Pattern="mod\s+$($stem)_validation" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs'; Pattern='test_coupled_runtime_cp412\.rs' },
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
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP412 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP412 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2313].Trim() -cne 'SupplyHumRatSat = PsyWFnTdbRhPb(state, PurchAir.SupplyTemp, 1.0, state.dataEnvrn->OutBaroPress, RoutineName);' -or
    $sourceLines[2314].Trim() -cne 'if (SupplyHumRatSat < SupplyHumRatOrig) {' -or
    $sourceLines[2315].Trim() -cne 'PurchAir.SupplyTemp = PsyTsatFnHPb(state, SupplyEnthalpy, state.dataEnvrn->OutBaroPress, RoutineName);') {
    throw 'CP412 source/first-excluded/continuation boundary drift'
}

$moduleText = Read-RepoText -Path $module
Assert-Cp412Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2314' -Description 'source constant'
Assert-Cp412Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2315' -Description 'first excluded constant'
$orderMatch = [regex]::Match($moduleText, '(?s)SATURATION_ASSIGNMENT_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP412 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSites = @(
    'read-purchased-air-supply-temperature-for-saturation-humidity-ratio',
    'read-environment-outdoor-barometric-pressure-for-saturation-humidity-ratio',
    'evaluate-psy-w-fn-tdb-rh-pb-at-unity-relative-humidity',
    'assign-local-saturation-supply-humidity-ratio'
)
if (($sites -join '|') -cne ($expectedSites -join '|')) { throw 'CP412 four-site source order drift' }

$snapshotStruct = Get-Cp412BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$expectedFields = @(
    'source','first_excluded_source','source_order','system','parent_call_ordinal','controlled_zone','unit_off_skipped','non_cooling_skipped','positive_guard_false_fallthrough_skipped','heating_availability_guard_false_fallthrough','humidification_control_guard_false_fallthrough','dehumidification_control_humidistat_maximum_assignment_executed','dehumidification_control_none_maximum_assignment_executed','dehumidification_control_guard_false_fallthrough',
    'predecessor_capacity_limit_guard_evaluated','predecessor_capacity_limit_body_entered','predecessor_active_capacity_limit_guard_false_fallthrough','predecessor_dehumidification_guard_evaluated','predecessor_dehumidification_body_entered','predecessor_dehumidification_guard_false_fallthrough','predecessor_dehumidification_total_output_assignment_executed','predecessor_dehumidification_total_output_capacity_guard_evaluated','predecessor_dehumidification_total_output_capacity_adjustment_body_entered','predecessor_dehumidification_total_output_capacity_guard_false_fallthrough','dehumidification_total_output_capacity_guard_false_fallthrough','dehumidification_total_output_maximum_capacity_assignment_executed','predecessor_supply_enthalpy_assignment_executed','predecessor_dehumidification_control_type_read','predecessor_dehumidification_control_type','predecessor_dehumidification_control_switch_dispatched','predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered','predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break','predecessor_dehumidification_control_humidistat_case_entered','predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed','predecessor_dehumidification_control_humidistat_case_exited_via_break','predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered','predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough','predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed','predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break',
    'predecessor_cp409_resulting_supply_humidity_ratio','predecessor_cp409_resulting_supply_enthalpy_j_per_kg','predecessor_cp409_resulting_supply_temperature_c','predecessor_dehumidification_control_default_case_exited_via_break','predecessor_cp410_resulting_supply_humidity_ratio','predecessor_cp410_resulting_supply_enthalpy_j_per_kg','predecessor_cp410_resulting_supply_temperature_c','post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed','cp410_retained_supply_humidity_ratio_state_owned','cp410_retained_supply_enthalpy_state_owned','cp410_retained_supply_temperature_state_owned','cp410_retained_supply_humidity_ratio_owned_read','purchased_air_supply_humidity_ratio_read','purchased_air_supply_humidity_ratio_before_saturation_check','local_supply_humidity_ratio_original_assignment_performed','assigned_supply_humidity_ratio_original','resulting_supply_humidity_ratio_original','predecessor_cp411_resulting_supply_humidity_ratio','predecessor_cp411_resulting_supply_enthalpy_j_per_kg','predecessor_cp411_resulting_supply_temperature_c',
    'post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed','cp411_retained_supply_humidity_ratio_state_owned','cp411_retained_supply_enthalpy_state_owned','cp411_retained_supply_temperature_state_owned','cp411_retained_supply_temperature_owned_read','purchased_air_supply_temperature_for_saturation_humidity_ratio_read','supply_temperature_for_saturation_humidity_ratio_c','environment_outdoor_barometric_pressure_owned_read','environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read','outdoor_barometric_pressure_pa','psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated','saturation_supply_humidity_ratio','local_saturation_supply_humidity_ratio_assignment_performed','assigned_saturation_supply_humidity_ratio','resulting_saturation_supply_humidity_ratio','resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
if ($fields.Count -ne 77 -or $expectedFields.Count -ne 77 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP412 snapshot must expose exactly 77 canonical fields' }
$expectedNumeric = @(
    'predecessor_cp409_resulting_supply_humidity_ratio','predecessor_cp409_resulting_supply_enthalpy_j_per_kg','predecessor_cp409_resulting_supply_temperature_c','predecessor_cp410_resulting_supply_humidity_ratio','predecessor_cp410_resulting_supply_enthalpy_j_per_kg','predecessor_cp410_resulting_supply_temperature_c','purchased_air_supply_humidity_ratio_before_saturation_check','assigned_supply_humidity_ratio_original','resulting_supply_humidity_ratio_original','predecessor_cp411_resulting_supply_humidity_ratio','predecessor_cp411_resulting_supply_enthalpy_j_per_kg','predecessor_cp411_resulting_supply_temperature_c','supply_temperature_for_saturation_humidity_ratio_c','outdoor_barometric_pressure_pa','saturation_supply_humidity_ratio','assigned_saturation_supply_humidity_ratio','resulting_saturation_supply_humidity_ratio','resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
if (($numericFields -join '|') -cne ($expectedNumeric -join '|')) { throw 'CP412 twenty numeric carrier fields drift' }
if ([regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP412 optional control enum drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*30\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @('predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts','predecessor_maximum_capacity_assignment_route_counts','predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts','supply_humidity_ratio_saturation_assignment_route_counts')
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP412 five width-30 route arrays drift' }
foreach ($counter in @('transition_count','inactive_transition_count','predecessor_guard_false_fallthrough_count','predecessor_maximum_capacity_assignment_count','predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count','supply_humidity_ratio_saturation_assignment_count','source_site_execution_count','cp411_supply_humidity_ratio_state_owner_count','unchanged_supply_humidity_ratio_preservation_count','cp411_supply_enthalpy_state_owner_count','unchanged_supply_enthalpy_preservation_count','cp411_supply_temperature_state_owner_count','unchanged_supply_temperature_preservation_count','cp411_retained_supply_temperature_owned_read_count','purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count','environment_outdoor_barometric_pressure_owner_count','environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count','psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count','local_saturation_supply_humidity_ratio_assignment_write_count')) {
    Assert-Cp412Text -Text $stateText -Pattern ("pub\s+" + [regex]::Escape($counter) + "\s*:\s*usize") -Description 'state counter'
}

$transitionText = Read-RepoText -Path $transition
$transitionBlock = Get-Cp412BraceBlock -Text $transitionText -AnchorPattern "fn\s+advance_$($stem)_state\s*\(" -Description 'transition'
$inputBlock = Get-Cp412BraceBlock -Text $transitionText -AnchorPattern "struct\s+$($typeStem)ActiveInput\s*" -Description 'active input'
$inputFields = @([regex]::Matches($inputBlock, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
if (($inputFields -join '|') -cne 'outdoor_barometric_pressure_pa') { throw 'CP412 ActiveInput must carry pressure only' }
Assert-Cp412Text -Text $transitionText -Pattern 'PreSaturationOriginalAssignmentSnapshot as Predecessor' -Description 'sole CP411 predecessor type'
foreach ($pattern in @('let\s+active\s*=\s*route_is_active\(route\)','let\s+temperature\s*=\s*predecessor\.resulting_supply_temperature_c\?','let\s+pressure\s*=\s*input\.outdoor_barometric_pressure_pa','energyplus_psy_w_fn_tdb_rh_pb\(temperature,\s*1\.0,\s*pressure\)','post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed:\s*active','supply_temperature_for_saturation_humidity_ratio_c:\s*temperature','outdoor_barometric_pressure_pa:\s*pressure','saturation_supply_humidity_ratio:\s*evaluated','assigned_saturation_supply_humidity_ratio:\s*evaluated','resulting_saturation_supply_humidity_ratio:\s*evaluated','resulting_supply_humidity_ratio:\s*predecessor\.resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg:\s*predecessor\.resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c:\s*predecessor\.resulting_supply_temperature_c')) {
    Assert-Cp412Text -Text $transitionBlock -Pattern $pattern -Description 'canonical evaluation and carrier preservation'
}
foreach ($field in @('cp411_retained_supply_temperature_owned_read','purchased_air_supply_temperature_for_saturation_humidity_ratio_read','environment_outdoor_barometric_pressure_owned_read','environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read','psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated','local_saturation_supply_humidity_ratio_assignment_performed')) { Assert-Cp412Text -Text $transitionBlock -Pattern ([regex]::Escape($field) + ':\s*active') -Description 'active-only local site flag' }
if ([regex]::Matches($transitionBlock,'energyplus_psy_w_fn_tdb_rh_pb\(').Count -ne 1) { throw 'CP412 transition must call the canonical psychrometric helper exactly once' }
Assert-Cp412Text -Text $transitionBlock -Pattern '(?s)else\s*\{\s*if\s+input\.is_some\(\).*?\(None,\s*None,\s*None\)' -Description 'inactive zero-site shape'
foreach ($forbidden in @('energyplus_psychrometric_humidity_ratio_from_rh','energyplus_psy_tsat','SupplyHumRatOrig','supply_hum_rat_orig','PsyTsatFnHPb','is_finite\s*\(','\s<\s','f64::min','f64::max','\.min\s*\(','\.max\s*\(','clamp\s*\(','RoutineName','DirectZonePurchasedAirCouplingInput')) {
    Assert-Cp412Text -Text $transitionBlock -Pattern "^(?![\s\S]*$forbidden)[\s\S]*$" -Description 'excluded wrapper/comparison/clamp/numerical behavior'
}
$psychrometricsText = Read-RepoText -Path 'crates\ep_runtime\src\psychrometrics.rs'
$psyBlock = Get-Cp412BraceBlock -Text $psychrometricsText -AnchorPattern 'pub\s+fn\s+energyplus_psy_w_fn_tdb_rh_pb\s*\(' -Description 'canonical psychrometric helper'
Assert-Cp412Text -Text $psyBlock -Pattern '(?s)energyplus_psy_psat_fn_temp_default_numerical_projection\(dry_bulb_c\).*?atmospheric_pressure_pa\s*-\s*dew_pressure_pa.*?pressure_difference_pa\s*<\s*1000\.0.*?humidity_ratio\s*<\s*ENERGYPLUS_MIN_HUMIDITY_RATIO' -Description 'unconditional projection and ordered denominator/humidity floors'
Assert-Contains -Path $routes -Pattern 'active:\s*matches!\(route\.predecessor_index,\s*18\.\.=29\)' -Description 'underlying active routes 18 through 29'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)' -Description 'six split predecessor indices'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description 'thirteen-route public reconstruction'
Assert-Contains -Path $routes -Pattern '(?s)predecessor_has_supply_humidity_ratio.*?route_is_active\(route\)' -Description 'W-presence equals active set'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)' -Description 'H-presence mapping'
Assert-Contains -Path $routes -Pattern '(?s)predecessor_has_supply_temperature.*?index\s*>=\s*3' -Description 'T-presence mapping'
foreach ($counter in @('cp411_retained_supply_temperature_owned_read_count','purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count','environment_outdoor_barometric_pressure_owner_count','environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count','psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count','local_saturation_supply_humidity_ratio_assignment_write_count')) {
    Assert-Contains -Path $accounting -Pattern ([regex]::Escape($counter) + '\s*\+=\s*1') -Description 'active source counter increment'
}
Assert-Contains -Path $accounting -Pattern 'source_site_execution_count\s*\+=\s*ORDER\.len\(\)' -Description 'four-site increment'

$releaseText = Read-RepoText -Path $release
$publicRelease = Get-Cp412BraceBlock -Text $releaseText -AnchorPattern "pub\s+fn\s+advance_direct_no_oa_calc_$stem\s*\(" -Description 'public release'
Assert-Cp412Text -Text $publicRelease -Pattern 'predecessor_cp411:\s*Predecessor,\s*barometric_pressure_pa:\s*f64' -Description 'CP411 plus pressure signature'
foreach ($pattern in @('predecessor_cp411\s*\.resulting_supply_temperature_c','temperature\.is_finite\(\)','barometric_pressure_pa\.is_finite\(\)','barometric_pressure_pa\s*<=\s*0\.0','energyplus_psy_w_fn_tdb_rh_pb\(temperature,\s*1\.0,\s*barometric_pressure_pa\)','result\.is_finite\(\)','if\s+route_is_active\(route\).*?Some\(ActiveInput','else\s*\{\s*None\s*\}')) { Assert-Cp412Text -Text $publicRelease -Pattern "(?s)$pattern" -Description 'public finite admission/inactive pressure ignore' }
Assert-Cp412Text -Text $publicRelease -Pattern '(?s)let\s+input\s*=\s*if\s+route_is_active\(route\)\s*\{.*?temperature\.is_finite\(\).*?barometric_pressure_pa\.is_finite\(\).*?barometric_pressure_pa\s*<=\s*0\.0.*?energyplus_psy_w_fn_tdb_rh_pb\(temperature,\s*1\.0,\s*barometric_pressure_pa\).*?result\.is_finite\(\).*?Some\(ActiveInput\s*\{.*?outdoor_barometric_pressure_pa:\s*barometric_pressure_pa.*?\}\)\s*\}\s*else\s*\{\s*None\s*\}' -Description 'active-scoped finite gates and inactive pressure bypass'
foreach ($file in @($transition,$accounting,$routes,$release,$runtimeValidation,$snapshotValidation,$adapter,$coupled,$coupledLineage,$pipelineValidation,$pipelineLineage)) { Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic' }
foreach ($pattern in @('transition_count,\s*36','inactive_transition_count,\s*18','supply_humidity_ratio_saturation_assignment_count,\s*18','source_site_execution_count,\s*72','predecessor_guard_false_fallthrough_count,\s*6','predecessor_maximum_capacity_assignment_count,\s*6','public_active,\s*4','private_active,\s*14','to_bits','-0\.0','from_bits\(1\)','INFINITY','NAN','counter_overflow.*transactional|overflow_is_transactional')) { Assert-Cp412Text -Text $testText -Pattern "(?is)$pattern" -Description 'exhaustive/IEEE/overflow characterization' }
foreach ($pattern in @('inactive_total.*?index\s*<\s*18','matches!\(index,\s*18\.\.=29\)','checked_mul\(ORDER\.len\(\)\)','cp411_supply_humidity_ratio_state_owner_count.*?humidity_total','cp411_supply_enthalpy_state_owner_count.*?enthalpy_total','cp411_supply_temperature_state_owner_count.*?temperature_total','supply_humidity_ratio_saturation_assignment_count.*?predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count')) { Assert-Contains -Path $runtimeValidation -Pattern "(?s)$pattern" -Description 'exact CP412 runtime accounting' }
Assert-Contains -Path $snapshotValidation -Pattern 'to_bits\(\)' -Description 'raw IEEE snapshot equality'
Assert-Contains -Path $privateCharacterization -Pattern 'ActiveInput' -Description 'private raw-input characterization'

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=","let\s+calculation_$successorStem\s*=",'let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP411-to-CP412-to-CP413-to-CP414-to-CP415 binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:","pub\s+calculation_$successorStem\s*:",'pub\s+coupling\s*:') -Description 'CP411-to-CP412-to-CP413 scheduled output order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:","pub\s+calculation_$successorStem\s*:",'pub\s+coupling\s*:') -Description 'CP411-to-CP412-to-CP413-to-CP414 scheduled output order'
$bindingText = Read-RepoText -Path $binding
if ([regex]::Matches($bindingText,"\bcalculation_$predecessorStem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$stem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$successorStem\b").Count -ne 3) { throw 'CP412/CP413/CP414 binding evidence occurrence drift' }
Assert-Contains -Path $binding -Pattern 'input\.barometric_pressure_pa' -Description 'existing current-timestep pressure read'
Assert-Cp412Text -Text $bindingText -Pattern ("(?s)let\s+calculation_" + [regex]::Escape($stem) + "\s*=\s*advance_" + [regex]::Escape($stem) + "\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_" + [regex]::Escape($predecessorStem) + ",\s*input\.barometric_pressure_pa,\s*\)\?") -Description 'CP412-specific pressure wiring'
$dto = Get-Cp412BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp411|cp412|cp413|pre_saturation|saturation_supply_humidity_ratio|saturation_guard') { throw 'CP411/CP412/CP413 evidence entered numerical DTO' }
Assert-Contains -Path $adapterTests -Pattern 'binding_cp412_inactive_u_n_and_p_ignore_invalid_unused_pressure' -Description 'inactive invalid-pressure binding regression'
Assert-Contains -Path $adapterTests -Pattern 'binding_cp412_active_invalid_pressure_is_transactional_and_fail_closed' -Description 'active invalid-pressure fail-closed binding regression'
Assert-Contains -Path $adapterTests -Pattern 'BarometricPressureOutsideDirectSubset' -Description 'active invalid-pressure public error assertion'
Assert-Contains -Path $coupledLineage -Pattern 'option_bits_equal|to_bits' -Description 'bit-exact coupled lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $coupledTests -Pattern 'cp412_evidence_does_not_feed_numerical_result' -Description 'coupled numerical nonfeed regression'
Assert-Contains -Path $witness -Pattern ("set_" + $stem + "_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle","$successorStem::\s*validate_direct_lifecycle") -Description 'pipeline CP411-to-CP412-to-CP413 order'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle","$successorStem::\s*validate_direct_lifecycle") -Description 'pipeline CP411-to-CP412-to-CP413-to-CP414 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp441_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp411\s*:\s*Option<&PredecessorLifecycle>' -Description 'sole pipeline predecessor'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp412_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp412_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp412_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-Contains -Path $arbitrary -Pattern 'mod\s+cp413_assertions' -Description 'CP412 arbitrary successor module'
Assert-Contains -Path $arbitrary -Pattern 'cp413_assertions::assert_direct\(runtime,\s*results\)' -Description 'CP412 direct arbitrary successor delegation'
Assert-Contains -Path $arbitrary -Pattern 'cp413_assertions::assert_non_direct\(runtime\)' -Description 'CP412 non-direct arbitrary successor delegation'
Assert-Contains -Path $arbitrary -Pattern 'Some\(97\)' -Description 'arbitrary 97-key schema'
Assert-Contains -Path $arbitrary -Pattern 'Some\(20\)' -Description 'arbitrary twenty-sidecar schema'

$serializationText = Read-RepoText -Path $serialization
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$numericSet = @{}; foreach ($field in $expectedNumeric) { $numericSet[$field] = $true }
$expectedJson = @(); foreach ($field in $expectedFields) { $expectedJson += $field; if ($numericSet.ContainsKey($field)) { $expectedJson += ($field + '_ieee_bits') } }
if ($jsonKeys.Count -ne 97 -or $expectedJson.Count -ne 97 -or ($jsonKeys -join '|') -cne ($expectedJson -join '|')) { throw 'CP412 JSON must expose 97 canonical keys' }
foreach ($field in $expectedNumeric) { $escaped = [regex]::Escape($field); Assert-Cp412Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar' }
Assert-Contains -Path $snapshotJsonTests -Pattern '97.*(?:key|field)|(?:key|field).*97' -Description '97-key JSON regression'
Assert-Contains -Path $snapshotJsonTests -Pattern 'twenty_sidecar_schema' -Description 'twenty-sidecar JSON regression'
Assert-Contains -Path $snapshotJsonTests -Pattern 'Some\(20\)' -Description 'twenty-sidecar JSON count'

$heading = 'CP412 post-saturation saturation supply-humidity-ratio assignment'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP412 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @('line 2314 exactly','four exact dependency-ordered source sites','line\s+2315.*?first excluded','CP413 candidate','strict comparison','routes 18 through 35 are active','routes 0 through 17 are\s+inactive','13/23 public/private','20, 21, 26, and 27','fourteen active routes','T412=T411=36','A412=A411=18','I412=18','source_site_execution_count=4\*A412=72','Five width-30 arrays','underlying routes 18 through 29','20, 21,\s*24, 25, 27, and 29','CP411.*?sole immediate route','resulting_supply_temperature_c.*?sole\s+temperature operand owner','current-timestep existing\s+scheduled-coupling input','18/23/33','energyplus_psy_w_fn_tdb_rh_pb\(temperature, 1\.0, pressure\)','1000 Pa denominator\s+floor','1\.0e-5','finite predecessor temperature','finite strictly-positive pressure','inactive routes.*?do not validate\s+the unused pressure','raw IEEE|exact input and\s+result IEEE bits','exactly 77 base fields','twenty `Option<f64>`','97 unique keys','twenty.*?IEEE sidecar','CP411-to-CP412-to-unchanged-numerical','32 algorithms, 293\s+routines','58 `state_mapped`, 235 `source_mapped`','350 total, 240 public, 110 internal','238 development commands')) { Assert-Cp412Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
}
$specAddenda = @([PSCustomObject]@{ Path='specs\algorithm_ledger.toml'; Anchor='CP412 supersedes only CP411' },[PSCustomObject]@{ Path='specs\capabilities.toml'; Anchor='CP412 additionally requires' })
foreach ($specAddendum in $specAddenda) {
    $specText = Read-RepoText -Path $specAddendum.Path
    $matches = [regex]::Matches($specText, '(?m)^\s*"(?<body>' + [regex]::Escape($specAddendum.Anchor) + '.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP412 expected one bounded addendum in $($specAddendum.Path), found $($matches.Count)" }
    $body = $matches[0].Groups['body'].Value
    foreach ($claim in @('line[- ]2314|physical executable line 2314','line 2315.*?CP413','36/18/18/72|T412=T411=36','18 through 35.*?active','20, 21, 26, and 27','77 base fields','twenty `Option<f64>`|twenty numeric optionals','97 JSON keys','CP411-to-CP412-to-unchanged-numerical','350 total, 240 public, 110 internal')) { Assert-Cp412Text -Text $body -Pattern "(?is)$claim" -Description "bounded addendum claim in $($specAddendum.Path)" }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP412 supersedes only CP411' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP412 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP412\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP412\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP412 algorithm/routine ledger counts drift' }
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern '(?s)routine\.psy_w_fn_tdb_rh_pb.*?completion_status\s*=\s*"state_mapped"' -Description 'psychrometric routine status preservation'

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 411) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp441_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 411) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 379' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 411) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 139' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 411) { Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 379 \|')) -Description 'historical generated total'; Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 139 \|')) -Description 'historical generated internal total' }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP412 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminal = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 412)) })
if ($terminal.Count -ne 36) { throw 'CP413 terminal propagation set drift' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP412-to-CP413' -Description 'historical terminal interval' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP413-to-CP414' -Description 'historical terminal interval' }
$cp345 = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp411Call\s*=','\$cp412Call\s*=','\$cp413Call\s*=','\$cp414Call\s*=','CP410-to-CP411','CP411-to-CP412','CP412-to-CP413','CP413-to-CP414','CP414-to-CP415')) { Assert-Contains -Path $cp345 -Pattern $pattern -Description 'CP345 terminal chain' }
Assert-LineLimit -Path $cp345 -Limit 1201 -Description 'CP345 fixed structural cap'
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405|406|407|408|409|410|411|412)-' })) { Assert-Contains -Path $file.FullName -Pattern "calculation_$successorStem" -Description 'recent CP413 binding order' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406|407|408|409|410|411|412)-' })) { Assert-Contains -Path $file.FullName -Pattern '\$cp413Call' -Description 'recent CP413 terminal capture' }

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp411-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-pre-saturation-original-assignment.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP412 registration order drift' }

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 379','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp412Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 139) { throw 'CP412 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp412-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-saturation-assignment\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 379 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 139 \|' -Description 'generated internal total'

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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host 'CP412 post-saturation saturation humidity-ratio assignment structure audit passed.'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp437Call' -Description 'CP437 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-CP437' -Description 'CP436-to-CP437 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP436-to-' + 'numerical') -Description 'stale CP436 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp438Call' -Description 'CP438 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-CP438' -Description 'CP437-to-CP438 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP437-to-' + 'numerical') -Description 'stale CP437 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp439Call' -Description 'CP439 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP438-to-CP439' -Description 'CP438-to-CP439 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP438-to-' + 'numerical') -Description 'stale CP438 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp440Call' -Description 'CP440 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-CP440' -Description 'CP439-to-CP440 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP439-to-' + 'numerical') -Description 'stale CP439 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp441Call' -Description 'CP441 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP440-to-CP441' -Description 'CP440-to-CP441 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP440-to-' + 'numerical') -Description 'stale CP440 numerical interval'
