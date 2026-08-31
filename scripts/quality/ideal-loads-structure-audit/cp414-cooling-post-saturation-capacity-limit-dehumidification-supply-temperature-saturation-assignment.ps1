# CP414 maps PurchasedAirManager.cc physical executable line 2316's saturation-temperature assignment.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard'
$successorStem = 'cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignment'
$predecessorTypeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuard'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$psychrometricsSource = '.reference\energyplus-src\26.1.0\src\EnergyPlus\Psychrometrics.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$predecessorModule = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$releaseError = "$root\release\error.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$psychrometrics = 'crates\ep_runtime\src\psychrometrics.rs'
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp414.rs'
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
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp414_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp413_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp414-cooling-post-saturation-capacity-limit-dehumidification-supply-temperature-saturation-assignment.ps1'

function Assert-Cp414Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP414 $Description missing '$Pattern'" }
}

function Get-Cp414BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP414 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP414 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP414 $Description closing brace missing"
}

$required = @(
    $source,$psychrometricsSource,$module,$predecessorModule,$state,$transition,$tests,$release,$releaseError,$runtimeValidation,
    $psychrometrics,$binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledTests,
    $coupledFixture,$witness,$pipelineRoot,$pipeline,$pipelineValidation,$pipelineValidationTests,
    $pipelineLineage,$serializationRoot,$serialization,$snapshotJsonTests,$arbitrary,
    $arbitraryPredecessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP414 implementation/audit file' }
foreach ($file in @($module,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipeline,$pipelineValidation,$pipelineValidationTests,$pipelineLineage,$serializationRoot,
    $serialization,$snapshotJsonTests,$arbitrary,$audit)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP414 file'
}
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
if ($coreFiles.Count -ne 6) { throw 'CP414 exact six-file bounded core subtree drift' }
$coreText = ($coreFiles | ForEach-Object {
    Assert-LineLimit -Path $_.FullName -Limit 500 -Description 'bounded CP414 core file'
    Read-RepoText -Path $_.FullName
}) -join [Environment]::NewLine
$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine +
    (Read-RepoText -Path $adapterTests) + [Environment]::NewLine +
    (Read-RepoText -Path $coupledTests) + [Environment]::NewLine +
    (Read-RepoText -Path $pipelineValidationTests) + [Environment]::NewLine +
    (Read-RepoText -Path $snapshotJsonTests) + [Environment]::NewLine +
    (Read-RepoText -Path $arbitrary)

$registrations = @(
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\calc.rs'; Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs'; Pattern="mod\s+$stem" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs'; Pattern="$($stem)_tests" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs'; Pattern="mod\s+$($stem)_validation" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs'; Pattern='test_coupled_runtime_cp414\.rs' },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs'; Pattern="$($stem)_fixture" },
    [PSCustomObject]@{ Path=$module; Pattern='(?s)mod\s+release;.*?mod\s+state;.*?mod\s+tests;.*?mod\s+transition;' },
    [PSCustomObject]@{ Path=$release; Pattern='(?s)mod\s+error;.*?mod\s+runtime_validation;' },
    [PSCustomObject]@{ Path=$pipelineRoot; Pattern="mod\s+$pipelineStem;" },
    [PSCustomObject]@{ Path=$pipeline; Pattern='(?s)mod\s+serialization;.*?mod\s+validation;' },
    [PSCustomObject]@{ Path=$pipelineValidation; Pattern='(?s)mod\s+lineage;.*?mod\s+tests;' },
    [PSCustomObject]@{ Path=$serializationRoot; Pattern='mod\s+snapshot;' },
    [PSCustomObject]@{ Path=$serialization; Pattern='mod\s+tests;' }
)
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP414 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP414 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2315].Trim() -cne 'PurchAir.SupplyTemp = PsyTsatFnHPb(state, SupplyEnthalpy, state.dataEnvrn->OutBaroPress, RoutineName);' -or
    $sourceLines[2316].Trim() -cne '' -or
    $sourceLines[2317].Trim() -cne '// This is the cooling mode, so SupplyTemp can''t be more than MixedAirTemp' -or
    $sourceLines[2318].Trim() -cne 'PurchAir.SupplyTemp = min(PurchAir.SupplyTemp, PurchAir.MixedAirTemp);') {
    throw 'CP414 source/blank/comment/first-excluded boundary drift'
}

$moduleText = Read-RepoText -Path $module
Assert-Cp414Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2316' -Description 'source constant'
Assert-Cp414Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2319' -Description 'first excluded constant'
$orderMatch = [regex]::Match($moduleText, '(?s)SATURATION_ASSIGNMENT_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP414 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSites = @(
    'read-cp413-retained-supply-enthalpy-for-saturation-temperature',
    'read-environment-outdoor-barometric-pressure-for-saturation-temperature',
    'evaluate-psy-tsat-fn-h-pb',
    'assign-purchased-air-supply-temperature-to-saturation-temperature'
)
if (($sites -join '|') -cne ($expectedSites -join '|')) { throw 'CP414 four-site source order drift' }

$snapshotStruct = Get-Cp414BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$predecessorText = Read-RepoText -Path $predecessorModule
$predecessorStruct = Get-Cp414BraceBlock -Text $predecessorText -AnchorPattern "pub\s+struct\s+$($predecessorTypeStem)Snapshot\s*" -Description 'CP413 snapshot'
$predecessorFields = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$cp413Terminal = @('resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c')
if ($predecessorFields.Count -ne 95 -or (($predecessorFields[92..94]) -join '|') -cne ($cp413Terminal -join '|')) { throw 'CP414 CP413 predecessor shape drift' }
$predecessorTriple = @('predecessor_cp413_resulting_supply_humidity_ratio','predecessor_cp413_resulting_supply_enthalpy_j_per_kg','predecessor_cp413_resulting_supply_temperature_c')
$suffix = @(
    'post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed',
    'cp413_retained_supply_humidity_ratio_state_owned','cp413_retained_supply_enthalpy_state_owned','cp413_retained_supply_temperature_state_owned',
    'cp413_retained_supply_enthalpy_owned_read','supply_enthalpy_for_saturation_temperature_read','supply_enthalpy_for_saturation_temperature_j_per_kg',
    'environment_outdoor_barometric_pressure_for_saturation_temperature_owned_read','environment_outdoor_barometric_pressure_for_saturation_temperature_read','outdoor_barometric_pressure_for_saturation_temperature_pa',
    'psy_tsat_fn_h_pb_evaluated','psychrometric_saturation_supply_temperature_result_c',
    'purchased_air_supply_temperature_saturation_assignment_performed','assigned_saturation_supply_temperature_c',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
$expectedFields = @($predecessorFields[0..91]) + $predecessorTriple + $suffix
if ($fields.Count -ne 112 -or $expectedFields.Count -ne 112 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP414 snapshot must expose exactly 112 canonical fields' }
$predecessorNumeric = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
if ($predecessorNumeric.Count -ne 25 -or (($predecessorNumeric[22..24]) -join '|') -cne ($cp413Terminal -join '|')) { throw 'CP414 CP413 numeric predecessor shape drift' }
$localNumeric = @('supply_enthalpy_for_saturation_temperature_j_per_kg','outdoor_barometric_pressure_for_saturation_temperature_pa','psychrometric_saturation_supply_temperature_result_c','assigned_saturation_supply_temperature_c')
$expectedNumeric = @($predecessorNumeric[0..21]) + $predecessorTriple + $localNumeric + $cp413Terminal
if ($numericFields.Count -ne 32 -or ($numericFields -join '|') -cne ($expectedNumeric -join '|')) { throw 'CP414 thirty-two numeric carrier fields drift' }
if ([regex]::Matches($snapshotStruct, 'Option<bool>').Count -ne 1 -or [regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP414 optional comparison/control carrier drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @('predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts','predecessor_guard_body_entry_route_counts','supply_temperature_saturation_assignment_route_counts')
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP414 four width-36 route arrays drift' }
$expectedCounters = @(
    'transition_count','inactive_transition_count','saturation_supply_temperature_assignment_count','source_site_execution_count',
    'cp413_supply_humidity_ratio_state_owner_count','unchanged_supply_humidity_ratio_preservation_count',
    'cp413_supply_enthalpy_state_owner_count','unchanged_supply_enthalpy_preservation_count',
    'cp413_supply_temperature_state_owner_count','unchanged_supply_temperature_preservation_count',
    'cp414_saturation_supply_temperature_state_owner_count','cp413_retained_supply_enthalpy_owned_read_count',
    'supply_enthalpy_for_saturation_temperature_read_count','environment_outdoor_barometric_pressure_for_saturation_temperature_owner_count',
    'environment_outdoor_barometric_pressure_for_saturation_temperature_read_count','psy_tsat_fn_h_pb_evaluation_count',
    'purchased_air_supply_temperature_saturation_assignment_write_count'
)
$counterFields = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*usize') | ForEach-Object { $_.Groups['name'].Value })
if (($counterFields -join '|') -cne ($expectedCounters -join '|')) { throw 'CP414 exact runtime counter set drift' }

$transitionText = Read-RepoText -Path $transition
Assert-Cp414Text -Text $transitionText -Pattern 'SupplyHumidityRatioSaturationGuardSnapshot as Predecessor' -Description 'sole CP413 predecessor type'
foreach ($pattern in @(
    'assignment_executed:\s*route\.body_entered','if\s+route\.assignment_executed',
    'predecessor\.resulting_supply_enthalpy_j_per_kg\?','energyplus_psy_tsat_fn_h_pb_raw\(enthalpy,\s*barometric_pressure_pa\)',
    'resulting_supply_humidity_ratio:\s*predecessor\.resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg:\s*predecessor\.resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c:\s*resulting_temperature',
    'source_site_execution_count\s*\+=\s*4','supply_temperature_saturation_assignment_route_counts\[index\]\s*\+=\s*1'
)) { Assert-Cp414Text -Text $transitionText -Pattern "(?s)$pattern" -Description 'assignment transition contract' }
Assert-NotContains -Path $transition -Pattern 'energyplus_psy_tsat_fn_h_pb_raw\([^,]+,\s*predecessor\.' -Description 'CP412 pressure substitute'
foreach ($file in @($transition,$release,$releaseError,$runtimeValidation,$adapter,$coupled,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}

$releaseText = Read-RepoText -Path $release
$publicRelease = Get-Cp414BraceBlock -Text $releaseText -AnchorPattern "pub\s+fn\s+advance_direct_no_oa_calc_$stem\s*\(" -Description 'public release'
foreach ($pattern in @(
    'predecessor_cp413:\s*Predecessor\s*,\s*barometric_pressure_pa:\s*f64',
    'if\s+route\.assignment_executed','resulting_supply_enthalpy_j_per_kg',
    'enthalpy\.is_finite\(\)','barometric_pressure_pa\.is_finite\(\)','barometric_pressure_pa\s*<=\s*0\.0',
    'resulting_supply_temperature_c\s*\.is_some_and\(f64::is_finite\)',
    'SupplyEnthalpyOutsideDirectSubset','BarometricPressureOutsideDirectSubset','PsychrometricSaturationTemperatureOutsideDirectSubset'
)) { Assert-Cp414Text -Text $publicRelease -Pattern "(?s)$pattern" -Description 'active-scoped public finite gate' }
$runtimeValidationText = Read-RepoText -Path $runtimeValidation
Assert-Cp414Text -Text $runtimeValidationText -Pattern 'supply_temperature_saturation_assignment_route_counts\[index\]\s*!=\s*state\.predecessor_guard_body_entry_route_counts\[index\]' -Description 'assignment/body route identity'
Assert-Cp414Text -Text $runtimeValidationText -Pattern 'let\s+Some\(source_sites\)\s*=\s*assignments\.checked_mul\(4\)\s*else\s*\{\s*return\s+false' -Description 'four-site overflow rejection'
Assert-Cp414Text -Text $runtimeValidationText -Pattern 'source_site_execution_count\s*==\s*source_sites' -Description 'four-site checked accounting'

$psychrometricsText = Read-RepoText -Path $psychrometrics
$helper = Get-Cp414BraceBlock -Text $psychrometricsText -AnchorPattern 'pub\s+fn\s+energyplus_psy_tsat_fn_h_pb_raw\s*\(' -Description 'canonical raw PsyTsatFnHPb helper'
foreach ($pattern in @(
    'const\s+CASE_RANGE:\s*\[f64;\s*10\]','-4\.24e4.*?-2\.2138e4.*?-6\.7012e2.*?2\.7297e4.*?7\.5222e4.*?1\.8379e5.*?4\.7577e5.*?1\.5445e6.*?3\.8353e6.*?4\.5866e7',
    'enthalpy_j_per_kg\s*\+\s*1\.78637e4','enthalpy_j_per_kg\s*>=\s*0\.0','1\.0e-5','-1\.0e-5',
    'while\s+begin\s*\+\s*1\s*<\s*end','let\s+case_index\s*=\s*begin\s*\+\s*1',
    'shifted_enthalpy\s*=\s*-4\.24e4','shifted_enthalpy\s*=\s*4\.5866e7',
    '\(barometric_pressure_pa\s*-\s*1\.0133e5\)\.abs\(\)\s*/\s*1\.0133e5\s*>\s*0\.01',
    'energyplus_psy_h_fn_tdb_w','energyplus_psy_w_fn_tdb_twb_pb','first_temperature_c\s*\*\s*0\.9',
    'while\s+iteration_count\s*<=\s*30','second_error\s*/\s*\(second_error\s*-\s*first_error\)'
)) { Assert-Cp414Text -Text $helper -Pattern "(?s)$pattern" -Description 'canonical default numerical-miss projection' }
if ([regex]::Matches($helper, 'energyplus_f6\(').Count -ne 8 -or [regex]::Matches($helper, 'energyplus_f7\(').Count -ne 1) { throw 'CP414 exact eight-F6/one-F7 seed branch drift' }
foreach ($forbidden in @('PsychCache','NumTimesCalled','ShowWarning','ShowSevere','CalledFrom','WarmupFlag','FlagError')) {
    Assert-Cp414Text -Text $helper -Pattern "^(?![\s\S]*$forbidden)[\s\S]*$" -Description 'isolated helper exclusion'
}

foreach ($pattern in @(
    'cp414_boundary_and_four_sites_are_exact','exhaustive_54_outcome_transition_and_four_route_partitions_are_exact',
    'active_raw_pressure_is_retained_but_inactive_pressure_is_not_read','ieee_enthalpy_bits_are_preserved_and_nonfinite_public_evidence_is_rejected',
    'every_incremented_counter_overflow_is_transactional','cp414_conceptual_contract_has_54_outcomes_72_sites_and_expected_carrier_ownership',
    'cp414_new_state_has_zeroed_lossless_route_partitions',
    'public_cp414_validator_depends_only_on_cp413','ep_run_cp414_rejects_missing_cp413_predecessor_evidence',
    'route_evidence_requires_assignment_to_equal_cp413_body_entry','public_route_firewall_rejects_private_base_routes',
    'conceptual_contract_retains_54_outcomes_and_four_public_body_assignments','overflow_helpers_fail_closed',
    'cp414_snapshot_serializer_declares_144_unique_json_entries_and_32_sidecars','nonfinite_json_projection_retains_authoritative_bits'
)) { Assert-Cp414Text -Text $testText -Pattern $pattern -Description 'CP414 regression coverage' }

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=","let\s+calculation_$successorStem\s*=",'let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP413-to-CP414-to-CP415-to-numerical binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:","pub\s+calculation_$successorStem\s*:",'pub\s+coupling\s*:') -Description 'CP413-to-CP414-to-CP415 scheduled output order'
$bindingText = Read-RepoText -Path $binding
if ([regex]::Matches($bindingText,"\bcalculation_$predecessorStem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$stem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$successorStem\b").Count -ne 3) { throw 'CP413/CP414/CP415 binding evidence occurrence drift' }
Assert-Contains -Path $binding -Pattern "(?s)calculation_$stem\s*=.*?input\.barometric_pressure_pa" -Description 'existing current pressure reuse'
$dto = Get-Cp414BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp413|cp414|cp415|saturation_assignment|saturation_mixed_air_limit|psychrometric_saturation|supply_enthalpy_for_saturation') { throw 'CP413/CP414/CP415 evidence entered numerical DTO' }
Assert-Contains -Path $adapterTests -Pattern 'cp414_binding_contract_is_source_ordered_after_cp413' -Description 'binding source-order regression'
Assert-Contains -Path $coupled -Pattern 'predecessor_cp413:\s*&PredecessorLifecycle' -Description 'coupled sole predecessor'
Assert-Contains -Path $coupled -Pattern 'snapshots_match_bit_exact' -Description 'coupled bit-exact lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $coupledTests -Pattern 'cp414_conceptual_contract_has_54_outcomes_72_sites_and_expected_carrier_ownership' -Description 'coupled conceptual accounting regression'
Assert-Contains -Path $coupledTests -Pattern 'cp414_new_state_has_zeroed_lossless_route_partitions' -Description 'coupled zero-state regression'
Assert-Contains -Path $witness -Pattern ("set_" + $stem + "_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle","$successorStem::\s*validate_direct_lifecycle") -Description 'pipeline CP413-to-CP414-to-CP415 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineRoot -Pattern 'arbitrary_run_cp414_registers_json_and_delegates_validation_to_cp413' -Description 'pipeline CP414 arbitrary JSON/delegation regression'
Assert-Contains -Path $pipelineRoot -Pattern 'arbitrary_run_cp414_lifecycle_evidence_does_not_feed_numerical_results' -Description 'pipeline CP414 arbitrary numerical nonfeed regression'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp413:\s*Option<&PredecessorLifecycle>' -Description 'sole pipeline predecessor'
Assert-Contains -Path $pipelineLineage -Pattern 'option_bits_equal|to_bits' -Description 'bit-exact pipeline lineage'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp414_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp414_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp414_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-Contains -Path $arbitrary -Pattern 'mod\s+cp415_assertions' -Description 'CP415 arbitrary successor module'
Assert-Contains -Path $arbitrary -Pattern 'cp415_assertions::assert_direct\(runtime,\s*results\)' -Description 'CP415 direct arbitrary successor delegation'
Assert-Contains -Path $arbitrary -Pattern 'cp415_assertions::assert_non_direct\(runtime\)' -Description 'CP415 non-direct arbitrary successor delegation'

$serializationText = Read-RepoText -Path $serialization
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$numericSet = @{}; foreach ($field in $expectedNumeric) { $numericSet[$field] = $true }
$expectedJson = @(); foreach ($field in $expectedFields) { $expectedJson += $field; if ($numericSet.ContainsKey($field)) { $expectedJson += ($field + '_ieee_bits') } }
if ($jsonKeys.Count -ne 144 -or $expectedJson.Count -ne 144 -or ($jsonKeys -join '|') -cne ($expectedJson -join '|')) { throw 'CP414 JSON must expose 144 canonical keys' }
foreach ($field in $expectedNumeric) {
    $escaped = [regex]::Escape($field)
    Assert-Cp414Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar'
}
Assert-Contains -Path $snapshotJsonTests -Pattern '144.*32|32.*144' -Description '144-key/thirty-two-sidecar JSON regression'

$heading = 'CP414 post-saturation saturation supply-temperature assignment'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP414 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'line 2316 exactly','four exact.*?source sites','line 2317 is blank','line 2318 is comment-only','line 2319.*?first excluded','CP415 candidate',
        'fifty-four flattened conceptual outcomes','thirty-six.*?no CP414\s+site','eighteen.*?all four','T414=54','Z414=36','A414=18','S414=4\*A414=72',
        '17/37','0 through 8, 22 through 25, and 34 through 37','23, 25, 35, and 37','Four width-36 arrays',
        'CP413.*?sole immediate','resulting_supply_enthalpy_j_per_kg.*?solely owns','current-timestep scheduled.*?pressure.*?sole pressure owner','CP412.*?neither bit corroboration.*?substitute','36/41/51',
        'energyplus_psy_tsat_fn_h_pb_raw','HH=H\+1\.78637e4','ten CaseRange','nine\s+F6/F7','one-percent pressure-band','thirty-one loop iterations',
        'finite\s+enthalpy','finite\s+strictly-positive pressure','finite\s+projected\s+temperature','Zero-site routes ignore','full.*?routine parity remain excluded',
        'first ninety-two fields','exactly\s+seventeen','112 base fields','thirty-two `Option<f64>`','one\s+`Option<bool>`','144 unique keys','thirty-two adjacent',
        'CP413-to-CP414-to-unchanged-numerical','adds no numerical or coupling-input DTO field','never feeds','32 algorithms, 293','58 `state_mapped`','235 `source_mapped`','352 total','240 public','112 internal','238\s+development\s+commands'
    )) { Assert-Cp414Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
}
$specAddenda = @(
    [PSCustomObject]@{ Path='specs\algorithm_ledger.toml'; Anchor='CP414 supersedes only CP413' },
    [PSCustomObject]@{ Path='specs\capabilities.toml'; Anchor='CP414 additionally requires' }
)
foreach ($specAddendum in $specAddenda) {
    $specText = Read-RepoText -Path $specAddendum.Path
    $matches = [regex]::Matches($specText, '(?m)^\s*"(?<body>' + [regex]::Escape($specAddendum.Anchor) + '.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP414 expected one bounded addendum in $($specAddendum.Path), found $($matches.Count)" }
    $body = $matches[0].Groups['body'].Value
    foreach ($claim in @(
        'line[- ]2316|physical executable line 2316','line 2319.*?CP415','54.*?36.*?18.*?72','17/37','23, 25, 35, and 37','four width-36',
        'CP413.*?sole','current.*?pressure.*?sole','no CP412|neither corroboration','36/41/51','HH=H\+1\.78637e4','nine.*?F6/F7','31 loop iterations|secant iterations',
        '112 base fields','thirty-two.*?numeric|thirty-two `Option<f64>`','144 JSON keys','CP413-to-CP414-to-unchanged-numerical','352 total, 240 public, 112 internal'
    )) { Assert-Cp414Text -Text $body -Pattern "(?is)$claim" -Description "bounded addendum claim in $($specAddendum.Path)" }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP414 supersedes only CP413' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP414 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP414\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP414\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP414 algorithm/routine ledger counts drift' }
foreach ($routine in @('psy_tsat_fn_h_pb_raw','psy_tsat_fn_h_pb')) {
    Assert-Cp414Text -Text $ledgerText -Pattern ("(?s)routine\." + $routine + "\.source_file.*?routine\." + $routine + "\.completion_status\s*=\s*`"source_mapped`"") -Description 'PsyTsatFnHPb routine status preservation'
}

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 414) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 414) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 373' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 414) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 133' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 414) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 373 \|')) -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 133 \|')) -Description 'historical generated internal total'
    }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP414 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminal = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 414)) })
if ($terminal.Count -ne 38) { throw 'CP414 terminal propagation set drift' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP413-to-CP414' -Description 'historical terminal interval' }
$recentTerminal = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 403 -and [int]$Matches['number'] -le 414 })
if ($recentTerminal.Count -ne 13) { throw 'CP414 recent terminal propagation set drift' }
foreach ($file in $recentTerminal) {
    Assert-Contains -Path $file.FullName -Pattern '\$cp414Call' -Description 'CP414 terminal capture'
    Assert-Contains -Path $file.FullName -Pattern 'CP414-to-CP415' -Description 'CP414 terminal-to-numerical interval'
}

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp413-cooling-post-saturation-capacity-limit-dehumidification-supply-humidity-ratio-saturation-guard.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$successorIndex = $master.IndexOf('cp415-cooling-post-saturation-capacity-limit-dehumidification-supply-temperature-saturation-mixed-air-limit.ps1')
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $successorIndex -le $currentIndex -or $completionIndex -le $successorIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP413-to-CP414-to-CP415 registration order drift' }

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 373','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp414Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 133) { throw 'CP414 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp414-cooling-post-saturation-capacity-limit-dehumidification-supply-temperature-saturation-assignment\.ps1' -Description 'inventory record'
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp415-cooling-post-saturation-capacity-limit-dehumidification-supply-temperature-saturation-mixed-air-limit\.ps1' -Description 'CP415 inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 373 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 133 \|' -Description 'generated internal total'

Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\s*=' -Description 'CP415 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp415Call' -Description 'CP415 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP414-to-CP415' -Description 'CP414-to-CP415 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\s*=' -Description 'CP416 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp416Call' -Description 'CP416 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP415-to-CP416' -Description 'CP415-to-CP416 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp417Call' -Description 'CP417 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP416-to-CP417' -Description 'CP416-to-CP417 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host 'CP414 post-saturation saturation supply-temperature assignment structure audit passed.'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'
