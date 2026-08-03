# CP409 maps PurchasedAirManager.cc physical line 2306's break only.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreak'
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
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp409.rs'
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineValidationTests = "crates\ep_run\src\pipeline\$pipelineStem\validation\tests.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp409_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp408_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp409-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-case-break.ps1'

function Assert-Cp409Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP409 $Description missing '$Pattern'" }
}

function Get-Cp409BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP409 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP409 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP409 $Description closing brace missing"
}

$required = @(
    $source, $module, $state, $transition, $accounting, $routes, $tests, $release,
    $error, $prefixValidation, $runtimeValidation, $snapshotValidation,
    $privateCharacterization, $binding, $scheduledOutput, $adapter, $adapterTests,
    $coupled, $coupledLineage, $coupledTests, $coupledFixture, $witness,
    $pipelineRoot, $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineLineage,
    $serialization, $snapshotJsonTests, $arbitrary, $arbitraryPredecessor, $audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP409 implementation/audit file' }
foreach ($file in @(
    $module, $state, $transition, $accounting, $routes, $tests, $release, $error,
    $prefixValidation, $runtimeValidation, $snapshotValidation,
    $privateCharacterization, $adapter, $adapterTests, $coupled, $coupledLineage,
    $coupledTests, $coupledFixture, $witness, $pipeline, $pipelineValidation, $pipelineValidationTests,
    $pipelineLineage, $serialization, $snapshotJsonTests, $arbitrary, $audit
)) { Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP409 file' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) {
    throw 'CP409 PurchasedAirManager.cc SHA-256 drift'
}
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2304].Trim() -cne '}' -or
    $sourceLines[2305].Trim() -cne '} break;' -or
    $sourceLines[2306].Trim() -cne 'default:' -or
    $sourceLines[2307].Trim() -cne 'break;' -or
    $sourceLines[2312].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw 'CP409 source/closing-brace/default/continuation boundary drift'
}

$moduleText = Read-RepoText -Path $module
Assert-Cp409Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2306' -Description 'source constant'
Assert-Cp409Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2308' -Description 'first excluded executable constant'
$orderMatch = [regex]::Match($moduleText, '(?s)CASE_BREAK_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP409 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSite = 'exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-none-or-constant-supply-humidity-ratio-shared-case-via-break'
if ($sites.Count -ne 1 -or $sites[0] -cne $expectedSite) { throw 'CP409 sole source site drift' }

$snapshotStruct = Get-Cp409BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
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
    'predecessor_cp408_resulting_supply_humidity_ratio','predecessor_cp408_resulting_supply_enthalpy_j_per_kg','predecessor_cp408_resulting_supply_temperature_c',
    'dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
if ($fields.Count -ne 45 -or $expectedFields.Count -ne 45) { throw 'CP409 snapshot must expose exactly 45 fields' }
for ($index = 0; $index -lt 45; $index += 1) {
    if ($fields[$index] -cne $expectedFields[$index]) { throw "CP409 field order drift at $index" }
}
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$expectedNumeric = @(
    'predecessor_cp408_resulting_supply_humidity_ratio','predecessor_cp408_resulting_supply_enthalpy_j_per_kg','predecessor_cp408_resulting_supply_temperature_c',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
if ($numericFields.Count -ne 6) { throw 'CP409 snapshot must expose six Option<f64> fields' }
for ($index = 0; $index -lt 6; $index += 1) {
    if ($numericFields[$index] -cne $expectedNumeric[$index]) { throw "CP409 numeric field order drift at $index" }
}
if ([regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) {
    throw 'CP409 snapshot must expose one optional dehumidification-control enum'
}

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*30\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedRouteArrays = @('predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts','predecessor_maximum_capacity_assignment_route_counts')
if ($routeArrays.Count -ne 3) { throw 'CP409 state must expose three width-30 route-lineage arrays' }
for ($index = 0; $index -lt 3; $index += 1) {
    if ($routeArrays[$index] -cne $expectedRouteArrays[$index]) { throw "CP409 route-array order drift at $index" }
}
foreach ($counter in @(
    'transition_count','inactive_transition_count','predecessor_guard_false_fallthrough_count',
    'predecessor_maximum_capacity_assignment_count','dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_break_count','source_site_execution_count'
)) { Assert-Cp409Text -Text $stateText -Pattern ("pub\s+" + [regex]::Escape($counter) + "\s*:\s*usize") -Description 'state counter' }

$transitionText = Read-RepoText -Path $transition
$transitionBlock = Get-Cp409BraceBlock -Text $transitionText -AnchorPattern "fn\s+advance_$($stem)_state\s*\(" -Description 'transition function'
Assert-Cp409Text -Text $transitionText -Pattern 'LatentOutputSupplyTemperatureMixedAirLimitSnapshot as Predecessor' -Description 'sole CP408 predecessor type'
Assert-Cp409Text -Text $transitionBlock -Pattern 'dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break:\s*route\.active' -Description 'active-route break execution'
foreach ($carrier in @('humidity_ratio','enthalpy_j_per_kg','temperature_c')) {
    Assert-Cp409Text -Text $transitionBlock -Pattern ("predecessor_cp408_resulting_supply_$carrier") -Description 'CP408 carrier capture'
    Assert-Cp409Text -Text $transitionBlock -Pattern ("resulting_supply_${carrier}:\s*predecessor_cp408_resulting_supply_$carrier") -Description 'bit-preserving carrier result'
}
foreach ($forbidden in @('DirectZonePurchasedAirCouplingInput','ZoneHeatBalanceState','energyplus_psy','Psy[A-Z]','f64::min','\.min\s*\(','is_finite\s*\(','clamp\s*\(','mul_add\s*\(')) {
    Assert-Cp409Text -Text $transitionBlock -Pattern "^(?![\s\S]*$forbidden)[\s\S]*$" -Description 'numeric-free transition'
}
Assert-Contains -Path $accounting -Pattern 'source_site_execution_count\s*\+=\s*ORDER\.len\(\)' -Description 'sole-site execution accounting'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)' -Description 'six active predecessor indices'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description 'eleven public predecessor indices'

Assert-Contains -Path $release -Pattern 'LatentOutputSupplyTemperatureMixedAirLimitSnapshot as Predecessor' -Description 'exact CP408 public predecessor'
Assert-Contains -Path $prefixValidation -Pattern 'completed_direct_.*latent_output_supply_temperature_mixed_air_limit_is_consistent' -Description 'recursive CP408 completion'
Assert-Contains -Path $privateCharacterization -Pattern "private_$stem" -Description 'restricted pure characterization'
$releaseText = Read-RepoText -Path $release
Assert-Cp409Text -Text $releaseText -Pattern "(?s)pub fn advance_direct_no_oa_calc_$stem\s*\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor_cp408:\s*Predecessor,\s*\)" -Description 'operand-free public release signature'
foreach ($file in @($transition,$accounting,$routes,$release,$prefixValidation,$runtimeValidation,$snapshotValidation,$adapter,$coupled,$coupledLineage,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}

$activeIndices = @(20,21,22,23,26,27,28,29,31,32,34,35)
$activePublic = @(20,21,26,27)
if ($activeIndices.Count -ne 12 -or (36 - $activeIndices.Count) -ne 24 -or
    $activePublic.Count -ne 4 -or (13 + 23) -ne 36) { throw 'CP409 logical route constants drift' }
foreach ($pattern in @(
    'assert_eq!\(routes\.len\(\),\s*36\)','\[20,\s*21,\s*22,\s*23,\s*26,\s*27,\s*28,\s*29,\s*31,\s*32,\s*34,\s*35\]',
    '\.count\(\),\s*13','\[20,\s*21,\s*26,\s*27\]','state\.transition_count,\s*36','state\.inactive_transition_count,\s*24',
    'state\.predecessor_guard_false_fallthrough_count,\s*6','state\.predecessor_maximum_capacity_assignment_count,\s*6',
    'shared_case_break_count,\s*12','state\.source_site_execution_count,\s*12','0x7ff8_0000_0000_0409'
)) { Assert-Contains -Path $tests -Pattern $pattern -Description '36/24/12/12 route, accounting, and IEEE characterization' }
Assert-Contains -Path $runtimeValidation -Pattern '(?s)predecessor_guard_false_fallthrough_count.*?checked_add\(state\.predecessor_maximum_capacity_assignment_count\).*?Some\(breaks\)' -Description 'B409 equals L408 plus M405'
Assert-Contains -Path $runtimeValidation -Pattern '(?s)inactive_transition_count\.checked_add\(breaks\).*?Some\(state\.transition_count\)' -Description 'T409 equals I409 plus B409'

Assert-PatternsInOrder -Path $binding -Patterns @(
    "let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+unit_available\s*=','let\s+coupling\s*='
) -Description 'CP408-to-CP411-to-numerical binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    "pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+coupling\s*:'
) -Description 'CP408-to-CP409 scheduled output order'
$bindingText = Read-RepoText -Path $binding
if ([regex]::Matches($bindingText,"\bcalculation_$predecessorStem\b").Count -ne 3 -or
    [regex]::Matches($bindingText,"\bcalculation_$stem\b").Count -ne 3) { throw 'CP409 binding evidence occurrence drift' }
$dto = Get-Cp409BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp408|cp409|constant_supply_humidity_ratio_case_break|latent_output_supply_temperature_mixed_air_limit') {
    throw 'CP408/CP409 evidence must not feed DirectZonePurchasedAirCouplingInput'
}
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp409_after_cp408_before_unchanged_numerical_coupling' -Description 'binding regression'
Assert-Contains -Path $coupledTests -Pattern 'cp409_evidence_does_not_feed_or_replace_numerical_coupling_dto' -Description 'coupled numerical firewall'
Assert-Contains -Path $coupledLineage -Pattern 'option_bits_equal' -Description 'bit-exact CP408 lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $witness -Pattern "set_${stem}_latest_witness" -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle"
) -Description 'pipeline CP408-to-CP409 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp411_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp408\s*:\s*Option<&PredecessorLifecycle>' -Description 'sole pipeline predecessor'
Assert-Contains -Path $pipelineValidationTests -Pattern 'public_cp409_validator_depends_only_on_cp408' -Description 'pipeline predecessor regression'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp409_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp409_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp409_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-Contains -Path $arbitrary -Pattern 'Some\(51\)' -Description 'arbitrary 51-key schema'
Assert-Contains -Path $arbitrary -Pattern 'Some\(6\)' -Description 'arbitrary six-sidecar schema'

$serializationText = Read-RepoText -Path $serialization
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$numericSet = @{}; foreach ($field in $expectedNumeric) { $numericSet[$field] = $true }
$expectedJson = @(); foreach ($field in $expectedFields) {
    $expectedJson += $field
    if ($numericSet.ContainsKey($field)) { $expectedJson += ($field + '_ieee_bits') }
}
if ($jsonKeys.Count -ne 51 -or $expectedJson.Count -ne 51 -or @($jsonKeys | Sort-Object -Unique).Count -ne 51) {
    throw 'CP409 JSON must expose 51 unique keys'
}
for ($index = 0; $index -lt 51; $index += 1) {
    if ($jsonKeys[$index] -cne $expectedJson[$index]) { throw "CP409 JSON key order drift at $index" }
}
foreach ($field in $expectedNumeric) {
    $escaped = [regex]::Escape($field)
    $adjacent = '(?m)^\s*"'+$escaped+'"\s*:\s*json_number\s*\(\s*snapshot\.'+$escaped+'\s*\)\s*,\s*\r?\n\s*"'+$escaped+'_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.'+$escaped+'\s*\)'
    Assert-Cp409Text -Text $serializationText -Pattern $adjacent -Description 'adjacent IEEE sidecar'
}
Assert-Contains -Path $snapshotJsonTests -Pattern 'compact_snapshot_has_exact_51_key_and_six_sidecar_schema' -Description '51-key/six-sidecar regression'
Assert-Contains -Path $snapshotJsonTests -Pattern '0x7ff8_0000_0000_0409' -Description 'NaN payload regression'

$heading = 'CP409 post-saturation shared None/constant-supply-humidity-ratio case break'
$docs = @(
    'docs\src\current\current-status.md','docs\src\current\project-contract.md',
    'docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md',
    'docs\src\porting-map\zone-air-update-map.md'
)
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP409 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'line 2306:\s*`} break;`','leading `}` is non-executable','sole executable source site','line 2308.*?first excluded executable',
        'dynamic continuation.*?line 2313','(?:36|thirty-six) logical routes','13/23 public/private','20, 21, 22, 23, 26, 27, 28','active public indices are 20, 21, 26, and 27',
        'B409=L408\+M405=6\+6=12','I409=T409-B409=24','Three width-30 arrays','CP408.*?sole immediate route',
        'exactly 45 base fields','six\s*`Option<f64>`','51 unique keys','CP408-to-CP409-to-unchanged-numerical',
        '32 algorithms, 293 routines','58\s*`state_mapped`, 235\s*`source_mapped`','170 required','347 total, 240 public, 107 internal','238 development commands'
    )) { Assert-Cp409Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
}
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP409 supersedes only CP408' -Description 'algorithm addendum'
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP409 additionally requires' -Description 'capability addendum'
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP409 supersedes only CP408' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP409 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP409\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP409\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) {
    throw 'CP409 algorithm/routine ledger counts drift'
}

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 408) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp411_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 408) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 349' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 408) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 109' -Description 'historical classification count' }
}
$cleanup = @(
    (Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File)
    Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 }
)
if ($cleanup.Count -ne 17) { throw 'CP409 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminal = @(
    (Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File)
    $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 408)) }
)
if ($terminal.Count -ne 32) { throw 'CP409 terminal propagation set drift' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP408-to-CP409' -Description 'historical terminal interval' }
$cp345 = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp409Call\s*=','CP408-to-CP409','CP411-to-numerical')) { Assert-Contains -Path $cp345 -Pattern $pattern -Description 'CP345 terminal chain' }
Assert-LineLimit -Path $cp345 -Limit 1200 -Description 'CP345 fixed structural cap'
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405|406|407|408)-' })) {
    Assert-Contains -Path $file.FullName -Pattern 'constant_supply_humidity_ratio_case_break\\s\*' -Description 'recent CP409 binding order'
}
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406|407|408)-' })) {
    Assert-Contains -Path $file.FullName -Pattern '\$cp409Call' -Description 'recent CP409 terminal capture'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp408-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-mixed-air-limit.ps1' -Pattern 'calculation_\$stem\\b"\)\.Count -ne 3' -Description 'CP408 successor-consumption binding count'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 94' -Description 'calc-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1' -Pattern 'Assert-LineLimit -Path \$cp339CalcRoot -Limit 94' -Description 'historical calc-root structural cap'

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp408-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-mixed-air-limit.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or
    [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP409 registration order drift' }

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 349','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) {
    Assert-Cp409Text -Text $inventory -Pattern $pattern -Description 'inventory count'
}
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or
    [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 109) { throw 'CP409 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp409-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-case-break\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 349 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 109 \|' -Description 'generated internal total'

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP410-to-CP411' -Description "CP345 CP410-to-CP411 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp410Call\s*=' -Description "CP345 CP410 call capture"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp411Call\s*=' -Description "CP345 CP411 call capture"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\s*=' -Description "CP410 historical binding order"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\s*=' -Description "CP411 historical binding order"
Write-Host 'CP409 post-saturation shared-case break structure audit passed.'
}
