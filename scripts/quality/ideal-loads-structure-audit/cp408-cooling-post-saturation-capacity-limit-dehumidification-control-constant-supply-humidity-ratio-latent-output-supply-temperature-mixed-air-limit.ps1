# CP408 maps PurchasedAirManager.cc physical executable line 2304 only.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimit'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$owners = "$root\transition\owners.rs"
$routes = "$root\transition\routes.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$ownerValidation = "$root\release\owner_validation.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp408.rs'
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp408_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp407_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp408-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-mixed-air-limit.ps1'

function Assert-Cp408Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP408 $Description missing '$Pattern'" }
}

function Get-Cp408BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP408 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP408 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP408 $Description closing brace missing"
}

$required = @(
    $source, $module, $state, $transition, $accounting, $owners, $routes, $tests,
    $release, $ownerValidation, $runtimeValidation, $snapshotValidation,
    $privateCharacterization, $binding, $scheduledOutput, $adapter, $adapterTests,
    $coupled, $coupledTests, $coupledFixture, $witness, $pipelineRoot, $pipeline,
    $pipelineValidation, $pipelineLineage, $serialization, $snapshotJsonTests,
    $arbitrary, $arbitraryPredecessor, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description 'CP408 implementation/audit file'
}
foreach ($file in @(
    $module, $state, $transition, $accounting, $owners, $routes, $tests, $release,
    $ownerValidation, $runtimeValidation, $snapshotValidation,
    $privateCharacterization, $adapter, $adapterTests, $coupled, $coupledTests,
    $coupledFixture, $witness, $pipeline, $pipelineValidation, $pipelineLineage,
    $serialization, $snapshotJsonTests, $arbitrary, $audit
)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP408 file'
}

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) {
    throw 'CP408 PurchasedAirManager.cc SHA-256 drift'
}
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2303].Trim() -cne 'PurchAir.SupplyTemp = min(PurchAir.SupplyTemp, PurchAir.MixedAirTemp);' -or
    $sourceLines[2304].Trim() -cne '}' -or
    $sourceLines[2305].Trim() -cne '} break;') {
    throw 'CP408 source/closing-brace/first-excluded boundary drift'
}

$moduleText = Read-RepoText -Path $module
Assert-Cp408Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2304' -Description 'source constant'
Assert-Cp408Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2306' -Description 'first excluded constant'
$orderMatch = [regex]::Match(
    $moduleText,
    '(?s)SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER:\s*&\[\&str\]\s*=\s*&\[(?<body>.*?)\];'
)
if (-not $orderMatch.Success) { throw 'CP408 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSites = @(
    'read-purchased-air-supply-temperature-for-minimum',
    'read-purchased-air-mixed-air-temperature-for-minimum',
    'apply-source-shaped-two-argument-minimum',
    'assign-purchased-air-supply-temperature'
)
if ($sites.Count -ne 4) { throw 'CP408 source-order array must contain exactly four sites' }
for ($index = 0; $index -lt 4; $index += 1) {
    if ($sites[$index] -cne $expectedSites[$index]) { throw "CP408 source-site order drift at $index" }
}

$snapshotStruct = Get-Cp408BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$expectedFields = @(
    'source',
    'first_excluded_source',
    'source_order',
    'system',
    'parent_call_ordinal',
    'controlled_zone',
    'unit_off_skipped',
    'non_cooling_skipped',
    'positive_guard_false_fallthrough_skipped',
    'heating_availability_guard_false_fallthrough',
    'humidification_control_guard_false_fallthrough',
    'dehumidification_control_humidistat_maximum_assignment_executed',
    'dehumidification_control_none_maximum_assignment_executed',
    'dehumidification_control_guard_false_fallthrough',
    'predecessor_capacity_limit_guard_evaluated',
    'predecessor_capacity_limit_body_entered',
    'predecessor_active_capacity_limit_guard_false_fallthrough',
    'predecessor_dehumidification_guard_evaluated',
    'predecessor_dehumidification_body_entered',
    'predecessor_dehumidification_guard_false_fallthrough',
    'predecessor_dehumidification_total_output_assignment_executed',
    'predecessor_dehumidification_total_output_capacity_guard_evaluated',
    'predecessor_dehumidification_total_output_capacity_adjustment_body_entered',
    'predecessor_dehumidification_total_output_capacity_guard_false_fallthrough',
    'dehumidification_total_output_capacity_guard_false_fallthrough',
    'dehumidification_total_output_maximum_capacity_assignment_executed',
    'predecessor_supply_enthalpy_assignment_executed',
    'predecessor_dehumidification_control_type_read',
    'predecessor_dehumidification_control_type',
    'predecessor_dehumidification_control_switch_dispatched',
    'predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered',
    'predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break',
    'predecessor_dehumidification_control_humidistat_case_entered',
    'predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed',
    'predecessor_dehumidification_control_humidistat_case_exited_via_break',
    'predecessor_dehumidification_control_none_case_entered',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered',
    'predecessor_cp406_resulting_supply_humidity_ratio',
    'predecessor_cp406_resulting_supply_enthalpy_j_per_kg',
    'predecessor_cp406_resulting_supply_temperature_c',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed',
    'predecessor_cp385_retained_supply_enthalpy_owned_read',
    'predecessor_cp406_same_call_supply_enthalpy_bit_corroborated',
    'predecessor_supply_enthalpy_for_dry_bulb_inversion_read',
    'predecessor_supply_enthalpy_j_per_kg',
    'predecessor_cp378_retained_supply_humidity_ratio_owned_read',
    'predecessor_supply_humidity_ratio_for_dry_bulb_inversion_read',
    'predecessor_supply_humidity_ratio',
    'predecessor_cp406_retained_supply_temperature_state_owned',
    'predecessor_preexisting_supply_temperature_c',
    'predecessor_psychrometric_supply_temperature_evaluated',
    'predecessor_psychrometric_supply_temperature_result_c',
    'predecessor_supply_temperature_assigned',
    'predecessor_assigned_supply_temperature_c',
    'predecessor_resulting_supply_humidity_ratio',
    'predecessor_resulting_supply_enthalpy_j_per_kg',
    'predecessor_resulting_supply_temperature_c',
    'dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed',
    'cp407_retained_supply_temperature_state_owned',
    'preexisting_supply_temperature_c',
    'cp407_retained_supply_temperature_owned_read',
    'supply_temperature_for_minimum_read',
    'supply_temperature_before_mixed_air_limit_c',
    'cp329_retained_mixed_air_temperature_owned_read',
    'mixed_air_temperature_for_minimum_read',
    'mixed_air_temperature_c',
    'source_shaped_two_argument_minimum_evaluated',
    'minimum_supply_temperature_c',
    'supply_temperature_assignment_performed',
    'assigned_supply_temperature_c',
    'resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c'
)
if ($fields.Count -ne 76 -or $expectedFields.Count -ne 76) { throw 'CP408 snapshot must expose exactly 76 fields' }
for ($index = 0; $index -lt 76; $index += 1) {
    if ($fields[$index] -cne $expectedFields[$index]) { throw "CP408 field order drift at $index" }
}
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$expectedNumeric = @(
    'predecessor_cp406_resulting_supply_humidity_ratio',
    'predecessor_cp406_resulting_supply_enthalpy_j_per_kg',
    'predecessor_cp406_resulting_supply_temperature_c',
    'predecessor_supply_enthalpy_j_per_kg',
    'predecessor_supply_humidity_ratio',
    'predecessor_preexisting_supply_temperature_c',
    'predecessor_psychrometric_supply_temperature_result_c',
    'predecessor_assigned_supply_temperature_c',
    'predecessor_resulting_supply_humidity_ratio',
    'predecessor_resulting_supply_enthalpy_j_per_kg',
    'predecessor_resulting_supply_temperature_c',
    'preexisting_supply_temperature_c',
    'supply_temperature_before_mixed_air_limit_c',
    'mixed_air_temperature_c',
    'minimum_supply_temperature_c',
    'assigned_supply_temperature_c',
    'resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c'
)
if ($numericFields.Count -ne 19) { throw 'CP408 snapshot must expose nineteen Option<f64> fields' }
for ($index = 0; $index -lt 19; $index += 1) {
    if ($numericFields[$index] -cne $expectedNumeric[$index]) { throw "CP408 numeric field order drift at $index" }
}
if ([regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) {
    throw 'CP408 snapshot must expose one optional dehumidification-control enum'
}

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*30\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedRouteArrays = @(
    'predecessor_route_counts',
    'predecessor_guard_false_fallthrough_route_counts',
    'predecessor_maximum_capacity_assignment_route_counts',
    'predecessor_else_branch_entry_route_counts',
    'predecessor_supply_temperature_assignment_route_counts',
    'supply_temperature_mixed_air_limit_route_counts'
)
if ($routeArrays.Count -ne 6) { throw 'CP408 state must expose six width-30 route arrays' }
for ($index = 0; $index -lt 6; $index += 1) {
    if ($routeArrays[$index] -cne $expectedRouteArrays[$index]) { throw "CP408 route-array order drift at $index" }
}
foreach ($counter in @(
    'transition_count', 'inactive_transition_count',
    'predecessor_guard_false_fallthrough_count',
    'predecessor_maximum_capacity_assignment_count',
    'predecessor_else_branch_entry_count',
    'predecessor_supply_temperature_assignment_count',
    'dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count',
    'source_site_execution_count', 'cp407_supply_temperature_state_owner_count',
    'unchanged_supply_humidity_ratio_preservation_count',
    'unchanged_supply_enthalpy_preservation_count',
    'unchanged_supply_temperature_preservation_count',
    'cp407_retained_supply_temperature_owned_read_count',
    'supply_temperature_for_minimum_read_count',
    'cp329_retained_mixed_air_temperature_owned_read_count',
    'mixed_air_temperature_for_minimum_read_count',
    'source_shaped_two_argument_minimum_evaluation_count',
    'supply_temperature_assignment_write_count'
)) {
    Assert-Cp408Text -Text $stateText -Pattern ("pub\s+" + [regex]::Escape($counter) + "\s*:\s*usize") -Description 'state counter'
}

Assert-Contains -Path $transition -Pattern 'cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum' -Description 'canonical minimum reuse'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\transition.rs' -Pattern '(?s)if left < right \{ left \} else \{ right \}' -Description 'source-shaped strict minimum'
Assert-Contains -Path $transition -Pattern '(?s)fn source_minimum\(\s*left: f64,\s*right: f64\s*\) -> f64 \{\s*source_shaped_two_argument_minimum\(left, right\)\s*\}' -Description 'single canonical minimum delegate'
Assert-NotContains -Path $transition -Pattern 'f64::min|\.min\s*\(|is_finite\s*\(|clamp\s*\(|mul_add\s*\(' -Description 'minimum semantic substitution'
Assert-Contains -Path $owners -Pattern 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor' -Description 'sole CP407 predecessor'
Assert-Contains -Path $owners -Pattern 'PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner' -Description 'sole CP329 mixed-air owner'
Assert-Contains -Path $owners -Pattern 'supply_temperature_c:\s*predecessor\.resulting_supply_temperature_c\?' -Description 'CP407 retained temperature operand'
Assert-Contains -Path $owners -Pattern 'mixed_air_temperature_c:\s*owner\.mixed_air_temperature_c\?' -Description 'CP329 retained mixed-air operand'
Assert-Contains -Path $ownerValidation -Pattern 'supply_temperature_assignment_latest_witness' -Description 'CP407 private latest witness'
Assert-Contains -Path $ownerValidation -Pattern 'cooling_mixed_air_call_latest_witness' -Description 'CP329 private latest witness'
Assert-NotContains -Path $release -Pattern '(?s)pub fn advance_direct_no_oa_calc_[^(]+\([^)]*(supply_temperature_c|mixed_air_temperature_c)\s*:' -Description 'no caller operand substitutes'
foreach ($file in @($transition, $accounting, $owners, $routes, $release, $ownerValidation, $runtimeValidation, $snapshotValidation, $adapter, $coupled, $pipelineValidation, $pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}

foreach ($pattern in @(
    'assert_eq!\(routes\.len\(\),\s*36\)',
    '\[20,\s*22,\s*26,\s*28,\s*31,\s*34\]',
    '\[20,\s*26\]',
    'state\.transition_count,\s*36',
    'state\.inactive_transition_count,\s*30',
    'state\.source_site_execution_count,\s*24',
    'assert_eq!\(count,\s*6\)',
    'if left < right \{ left \} else \{ right \}'
)) {
    Assert-Contains -Path $tests -Pattern $pattern -Description '36/30/6/24 route and IEEE characterization'
}
Assert-Contains -Path $runtimeValidation -Pattern '(?s)state\.inactive_transition_count\.checked_add\(assigned\).*?state\.transition_count' -Description 'T408 equals Z408 plus L408'
Assert-Contains -Path $runtimeValidation -Pattern '(?s)assigned\.checked_mul\(.*?SOURCE_ORDER\.len\(\)\).*?source_site_execution_count' -Description 'four-site accounting'
Assert-Contains -Path $runtimeValidation -Pattern '(?s)predecessor_supply_temperature_assignment_count\s*==\s*assigned' -Description 'CP407 assignment parity'

Assert-PatternsInOrder -Path $binding -Patterns @(
    "let\s+calculation_$predecessorStem\s*=",
    "let\s+calculation_$stem\s*=",
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*=',
    'let\s+unit_available\s*=',
    'let\s+coupling\s*='
) -Description 'CP407-to-CP408-to-CP411-to-numerical binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    "pub\s+calculation_$predecessorStem\s*:",
    "pub\s+calculation_$stem\s*:",
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*:',
    'pub\s+coupling\s*:'
) -Description 'CP407-to-CP408-to-CP409 scheduled output order'
$bindingText = Read-RepoText -Path $binding
if ([regex]::Matches($bindingText, "\bcalculation_$predecessorStem\b").Count -ne 3 -or
    [regex]::Matches($bindingText, "\bcalculation_$stem\b").Count -ne 3) {
    throw 'CP408 binding evidence occurrence drift'
}
$dto = Get-Cp408BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp407|cp408|cp409|latent_output_supply_temperature_(assignment|mixed_air_limit)|constant_supply_humidity_ratio_case_break') {
    throw 'CP407/CP408/CP409 evidence must not feed DirectZonePurchasedAirCouplingInput'
}
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp408_after_cp407_before_unchanged_numerical_coupling' -Description 'binding regression'
Assert-Contains -Path $coupledTests -Pattern 'cp408_evidence_does_not_feed_or_replace_numerical_coupling_dto' -Description 'coupled numerical firewall'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$predecessorStem::\s*validate_direct_lifecycle",
    "$stem::\s*validate_direct_lifecycle",
    'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break::\s*validate_direct_lifecycle'
) -Description 'pipeline CP407-to-CP408-to-CP409 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp411_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp408_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp408_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp408_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-Contains -Path $arbitrary -Pattern 'CP408' -Description 'arbitrary CP408 assertions'

$serializationText = Read-RepoText -Path $serialization
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$numericSet = @{}
foreach ($field in $expectedNumeric) { $numericSet[$field] = $true }
$expectedJson = @()
foreach ($field in $expectedFields) {
    $expectedJson += $field
    if ($numericSet.ContainsKey($field)) { $expectedJson += ($field + '_ieee_bits') }
}
if ($jsonKeys.Count -ne 95 -or $expectedJson.Count -ne 95 -or @($jsonKeys | Sort-Object -Unique).Count -ne 95) {
    throw 'CP408 JSON must expose 95 unique keys'
}
for ($index = 0; $index -lt 95; $index += 1) {
    if ($jsonKeys[$index] -cne $expectedJson[$index]) { throw "CP408 JSON key order drift at $index" }
}
foreach ($field in $expectedNumeric) {
    $escaped = [regex]::Escape($field)
    $adjacent = '(?m)^\s*"'+$escaped+'"\s*:\s*json_number\s*\(\s*snapshot\.'+$escaped+'\s*\)\s*,\s*\r?\n\s*"'+$escaped+'_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.'+$escaped+'\s*\)'
    Assert-Cp408Text -Text $serializationText -Pattern $adjacent -Description 'adjacent IEEE sidecar'
}
Assert-Contains -Path $snapshotJsonTests -Pattern 'object\.len\(\),\s*95' -Description '76-field plus nineteen-sidecar JSON shape'
Assert-Contains -Path $snapshotJsonTests -Pattern 'active_snapshot_has_exact_95_key_and_19_sidecar_schema' -Description 'nineteen-sidecar regression'
Assert-Contains -Path $snapshotJsonTests -Pattern 'source_shaped_tie_keeps_right_operand_ieee_bits' -Description 'right-biased tie serialization'
Assert-Contains -Path $snapshotJsonTests -Pattern '0x7ff8000000002408' -Description 'NaN payload sidecar'

$heading = 'CP408 post-saturation shared-case latent-output supply-temperature mixed-air limit'
$docs = @(
    'docs\src\current\current-status.md',
    'docs\src\current\project-contract.md',
    'docs\src\porting-map\heat-balance-source-map.md',
    'docs\src\porting-map\ideal-loads-source-map.md',
    'docs\src\porting-map\zone-air-update-map.md'
)
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText, "(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP408 heading count drift in $doc" }
    $section = [regex]::Match($docText, "(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'line 2304.*?PurchAir\.SupplyTemp = min',
        'line 2306.*?first excluded',
        'read-purchased-air-supply-temperature-for-minimum',
        'read-purchased-air-mixed-air-temperature-for-minimum',
        '20,\s*22,\s*26,\s*28,\s*31,\s*and 34',
        '36/30/6/24',
        'exactly 76 fields',
        '19 .*Option<f64>',
        '95 unique keys',
        'CP407.*?sole immediate route',
        'CP407.*?supply-temperature operand',
        'mixed-air-temperature operand.*?CP329',
        'CP407-to-CP408-to-unchanged-numerical',
        '32\s*algorithms(?:,| and)\s*293 routines',
        '58 .*state_mapped.*235 .*source_mapped.*170',
        '346 total,\s*240 public,\s*106 internal',
        '238 development commands'
    )) {
        Assert-Cp408Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim'
    }
}
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP408 supersedes only CP407' -Description 'algorithm addendum'
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP408 additionally requires' -Description 'capability addendum'
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP408 supersedes only CP407' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP408 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern 'CP408' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP408\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText, '(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText, '(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText, '(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText, '(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText, '(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) {
    throw 'CP408 algorithm/routine ledger counts drift'
}

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 407) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp411_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 407) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 349' -Description 'historical script count' }
    if ($number -ge 335 -and $number -le 407) {
        Assert-Contains -Path $file.FullName -Pattern '349[^\r\n]*generated' -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern '109[^\r\n]*generated' -Description 'historical generated internal total'
    }
    if ($number -ge 367 -and $number -le 407) {
        Assert-Contains -Path $file.FullName -Pattern 'Count -ne 109' -Description 'historical classification count'
        Assert-Contains -Path $file.FullName -Pattern '240 public and 106 internal' -Description 'historical classification phrase'
    }
}
$cleanup = @(
    (Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File)
    Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object {
        $_.BaseName -match '^cp(?<number>\d+)-' -and
        [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344
    }
)
if ($cleanup.Count -ne 17) { throw 'CP408 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }

$terminal = @(
    (Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File)
    $audits | Where-Object {
        $_.BaseName -match '^cp(?<number>\d+)-' -and
        (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or
         ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 407))
    }
)
if ($terminal.Count -ne 31) { throw 'CP408 terminal propagation set drift' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP407-to-CP408' -Description 'historical terminal interval' }
$cp345 = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp408Call\s*=','\$cp409Call\s*=','CP407-to-CP408','CP408-to-CP409','CP411-to-numerical')) {
    Assert-Contains -Path $cp345 -Pattern $pattern -Description 'CP345 terminal chain'
}
Assert-LineLimit -Path $cp345 -Limit 1200 -Description 'CP345 fixed structural cap'
$capAudits = @($audits | Where-Object {
    $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -lt 408 -and
    (Read-RepoText -Path $_.FullName) -match 'Assert-LineLimit[^\r\n]*-Limit\s+1200[^\r\n]*CP345 (?:historical audit|fixed structural cap|structural cap)'
})
if ($capAudits.Count -ne 19) { throw 'CP408 CP345 cap propagation set drift' }
foreach ($file in $capAudits) { Assert-Contains -Path $file.FullName -Pattern 'Limit 1200' -Description 'historical CP345 structural cap' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405|406|407)-' })) {
    Assert-Contains -Path $file.FullName -Pattern 'latent_output_supply_temperature_mixed_air_limit\\s\*' -Description 'recent CP408 binding order'
}
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406|407)-' })) {
    Assert-Contains -Path $file.FullName -Pattern '\$cp408Call' -Description 'recent CP408 terminal capture'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp407-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-assignment.ps1' -Pattern 'calculation_\$stem\\b"\)\.Count -ne 3' -Description 'CP407 successor-consumption binding count'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 94' -Description 'calc-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1' -Pattern 'Assert-LineLimit -Path \$cp339CalcRoot -Limit 94' -Description 'historical calc-root structural cap'

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp407-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-assignment.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or
    [regex]::Matches($master, [regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) {
    throw 'Master CP408 registration order drift'
}

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 349','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) {
    Assert-Cp408Text -Text $inventory -Pattern $pattern -Description 'inventory count'
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"\r?$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"\r?$').Count -ne 109) {
    throw 'CP408 inventory classification drift'
}
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp408-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-mixed-air-limit\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 349 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 109 \|' -Description 'generated internal total'

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP410-to-CP411' -Description "CP345 CP410-to-CP411 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp410Call\s*=' -Description "CP345 CP410 call capture"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp411Call\s*=' -Description "CP345 CP411 call capture"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\s*=' -Description "CP410 historical binding order"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\s*=' -Description "CP411 historical binding order"
Write-Host 'CP408 post-saturation shared-case latent-output supply-temperature mixed-air-limit structure audit passed.'
}
