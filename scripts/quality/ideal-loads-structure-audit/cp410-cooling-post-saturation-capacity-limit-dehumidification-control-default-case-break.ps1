# CP410 maps PurchasedAirManager.cc physical executable line 2308's untyped-default break.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreak'
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
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp410.rs'
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineValidationTests = "crates\ep_run\src\pipeline\$pipelineStem\validation\tests.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp410_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp409_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp410-cooling-post-saturation-capacity-limit-dehumidification-control-default-case-break.ps1'

function Assert-Cp410Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP410 $Description missing '$Pattern'" }
}

function Get-Cp410BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP410 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP410 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP410 $Description closing brace missing"
}

$required = @(
    $source,$module,$state,$transition,$accounting,$routes,$tests,$release,$error,
    $prefixValidation,$runtimeValidation,$snapshotValidation,$privateCharacterization,
    $binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledLineage,$coupledTests,
    $coupledFixture,$witness,$pipelineRoot,$pipeline,$pipelineValidation,
    $pipelineValidationTests,$pipelineLineage,$serialization,$snapshotJsonTests,
    $arbitrary,$arbitraryPredecessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP410 implementation/audit file' }
foreach ($file in @(
    $module,$state,$transition,$accounting,$routes,$tests,$release,$error,$prefixValidation,
    $runtimeValidation,$snapshotValidation,$privateCharacterization,$adapter,$adapterTests,
    $coupled,$coupledLineage,$coupledTests,$coupledFixture,$witness,$pipeline,
    $pipelineValidation,$pipelineValidationTests,$pipelineLineage,$serialization,
    $snapshotJsonTests,$arbitrary,$audit
)) { Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP410 file' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) {
    throw 'CP410 PurchasedAirManager.cc SHA-256 drift'
}
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2305].Trim() -cne '} break;' -or
    $sourceLines[2306].Trim() -cne 'default:' -or
    $sourceLines[2307].Trim() -cne 'break;' -or
    $sourceLines[2308].Trim() -cne '}' -or
    $sourceLines[2309].Trim() -cne '// Limit supply humidity ratio to saturation at supply outlet temp' -or
    $sourceLines[2310].Trim() -cne '// If saturation exceeded, then honor capacity limit and set to dew point at supply enthalpy' -or
    $sourceLines[2311].Trim() -cne '' -or
    $sourceLines[2312].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw 'CP410 source/default/switch-closure/continuation boundary drift'
}

$moduleText = Read-RepoText -Path $module
Assert-Cp410Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2308' -Description 'source constant'
Assert-Cp410Text -Text $moduleText -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2313' -Description 'first excluded constant'
$orderMatch = [regex]::Match($moduleText, '(?s)DEFAULT_CASE_BREAK_SOURCE_ORDER:\s*&\[&str\]\s*=\s*&\[(?<body>.*?)\];')
if (-not $orderMatch.Success) { throw 'CP410 source-order array missing' }
$sites = @([regex]::Matches($orderMatch.Groups['body'].Value, '"(?<site>[^"]+)"') | ForEach-Object { $_.Groups['site'].Value })
$expectedSite = 'exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-default-case-via-break'
if ($sites.Count -ne 1 -or $sites[0] -cne $expectedSite) { throw 'CP410 sole source site drift' }

$snapshotStruct = Get-Cp410BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
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
    'dehumidification_control_default_case_exited_via_break',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
if ($fields.Count -ne 46 -or $expectedFields.Count -ne 46 -or ($fields -join '|') -cne ($expectedFields -join '|')) {
    throw 'CP410 snapshot must expose exactly 46 canonical fields'
}
$expectedNumeric = @(
    'predecessor_cp409_resulting_supply_humidity_ratio','predecessor_cp409_resulting_supply_enthalpy_j_per_kg','predecessor_cp409_resulting_supply_temperature_c',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
$numericFields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
if (($numericFields -join '|') -cne ($expectedNumeric -join '|')) { throw 'CP410 six numeric carrier fields drift' }
if ([regex]::Matches($snapshotStruct, 'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP410 optional control enum drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*30\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @('predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts','predecessor_maximum_capacity_assignment_route_counts')
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP410 three width-30 route arrays drift' }
foreach ($counter in @('transition_count','inactive_transition_count','predecessor_guard_false_fallthrough_count','predecessor_maximum_capacity_assignment_count','dehumidification_control_default_case_break_count','source_site_execution_count')) {
    Assert-Cp410Text -Text $stateText -Pattern ("pub\s+" + [regex]::Escape($counter) + "\s*:\s*usize") -Description 'state counter'
}

$transitionText = Read-RepoText -Path $transition
$transitionBlock = Get-Cp410BraceBlock -Text $transitionText -AnchorPattern "fn\s+advance_$($stem)_state\s*\(" -Description 'transition'
Assert-Cp410Text -Text $transitionText -Pattern 'ConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor' -Description 'sole CP409 predecessor type'
Assert-Cp410Text -Text $transitionBlock -Pattern 'dehumidification_control_default_case_exited_via_break:\s*false' -Description 'universal typed default skip'
foreach ($carrier in @('humidity_ratio','enthalpy_j_per_kg','temperature_c')) {
    Assert-Cp410Text -Text $transitionBlock -Pattern "predecessor_cp409_resulting_supply_$carrier" -Description 'CP409 terminal carrier capture'
    Assert-Cp410Text -Text $transitionBlock -Pattern "resulting_supply_${carrier}:\s*predecessor_cp409_resulting_supply_$carrier" -Description 'bit-preserving result carrier'
}
foreach ($forbidden in @('DirectZonePurchasedAirCouplingInput','ZoneHeatBalanceState','energyplus_psy','Psy[A-Z]','f64::min','\.min\s*\(','is_finite\s*\(','clamp\s*\(','mul_add\s*\(')) {
    Assert-Cp410Text -Text $transitionBlock -Pattern "^(?![\s\S]*$forbidden)[\s\S]*$" -Description 'operation-free transition'
}
Assert-Contains -Path $accounting -Pattern 'dehumidification_control_default_case_break_count\s*!=\s*0' -Description 'default counter zero gate'
Assert-Contains -Path $accounting -Pattern 'source_site_execution_count\s*!=\s*0' -Description 'source counter zero gate'
Assert-NotContains -Path $accounting -Pattern '(?:default_case_break_count|source_site_execution_count)\s*\+=' -Description 'unreachable source counter increment'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)' -Description 'six split predecessor indices'
Assert-Contains -Path $routes -Pattern 'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description 'eleven public predecessor indices'
Assert-NotContains -Path $routes -Pattern '(?i)invalid|unknown|thirty.?seventh|default.*RetainedRoute' -Description 'invented default route'

Assert-Contains -Path $release -Pattern 'ConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor' -Description 'exact CP409 public predecessor'
Assert-Contains -Path $prefixValidation -Pattern 'completed_direct_.*constant_supply_humidity_ratio_case_break_is_consistent' -Description 'recursive CP409 completion'
$releaseText = Read-RepoText -Path $release
Assert-Cp410Text -Text $releaseText -Pattern "(?s)pub fn advance_direct_no_oa_calc_$stem\s*\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor_cp409:\s*Predecessor,\s*\)" -Description 'operand-free public signature'
foreach ($file in @($transition,$accounting,$routes,$release,$prefixValidation,$runtimeValidation,$snapshotValidation,$adapter,$coupled,$coupledLineage,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}
foreach ($pattern in @('routes\.len\(\),\s*36','transition_count,\s*36','inactive_transition_count,\s*36','default_case_break_count,\s*0','source_site_execution_count,\s*0','\.count\(\),\s*13')) {
    Assert-Contains -Path $tests -Pattern $pattern -Description '36/36/0/0 exhaustive characterization'
}
Assert-Contains -Path $runtimeValidation -Pattern '(?s)inactive_transition_count\s*==\s*state\.transition_count' -Description 'all transitions inactive identity'
Assert-Contains -Path $runtimeValidation -Pattern '(?s)predecessor_guard_false_fallthrough_count.*?checked_add\(state\.predecessor_maximum_capacity_assignment_count\)' -Description 'CP409 shared-break reconstruction'

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP409-to-CP410-to-numerical binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+coupling\s*:') -Description 'CP409-to-CP410 scheduled output order'
$bindingText = Read-RepoText -Path $binding
if ([regex]::Matches($bindingText,"\bcalculation_$predecessorStem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$stem\b").Count -ne 2) { throw 'CP410 binding evidence occurrence drift' }
$dto = Get-Cp410BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp409|cp410|default_case_break|constant_supply_humidity_ratio_case_break') { throw 'CP409/CP410 evidence entered numerical DTO' }
Assert-Contains -Path $coupledLineage -Pattern 'option_bits_equal|to_bits' -Description 'bit-exact coupled lineage'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $witness -Pattern "set_${stem}_latest_witness" -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle") -Description 'pipeline CP409-to-CP410 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp410_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp409\s*:\s*Option<&PredecessorLifecycle>' -Description 'sole pipeline predecessor'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp410_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp410_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp410_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-Contains -Path $arbitrary -Pattern 'Some\(52\)' -Description 'arbitrary 52-key schema'
Assert-Contains -Path $arbitrary -Pattern 'Some\(6\)' -Description 'arbitrary six-sidecar schema'

$serializationText = Read-RepoText -Path $serialization
$jsonKeys = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$numericSet = @{}; foreach ($field in $expectedNumeric) { $numericSet[$field] = $true }
$expectedJson = @(); foreach ($field in $expectedFields) { $expectedJson += $field; if ($numericSet.ContainsKey($field)) { $expectedJson += ($field + '_ieee_bits') } }
if ($jsonKeys.Count -ne 52 -or $expectedJson.Count -ne 52 -or ($jsonKeys -join '|') -cne ($expectedJson -join '|')) { throw 'CP410 JSON must expose 52 canonical keys' }
foreach ($field in $expectedNumeric) {
    $escaped = [regex]::Escape($field)
    Assert-Cp410Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar'
}
Assert-Contains -Path $snapshotJsonTests -Pattern '52.*(?:key|field)|(?:key|field).*52' -Description '52-key JSON regression'

$heading = 'CP410 post-saturation dehumidification-control default case break'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
$successorReachabilityPatterns = @(
    'flattened\s+CP410 routes 18 through 35\s*\(18 routes total\)',
    'split 4 public and 14 private',
    'flattened routes 0 through 17 do not reach'
)
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP410 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @(
        'line 2308:\s*`break;`','line 2307.*?non-executable `default:`','sole CP410\s+source site','line 2313.*?first excluded','CP411 candidate',
        '(?:36|thirty-six) logical routes','13/23 public/private','no active public or private','T410=T409=36','I410=T410=36','B410=0','source_site_execution_count=B410=0',
        'Three width-30 arrays','B409=6\+6=12','CP409.*?sole immediate route','exactly 46 base fields','six\s*`Option<f64>`','52 unique keys','CP409-to-CP410-to-unchanged-numerical',
        '32 algorithms, 293 routines','58\s*`state_mapped`, 235\s*`source_mapped`','170 required','348 total, 240 public, 108 internal','238 development commands'
    )) { Assert-Cp410Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
    foreach ($pattern in $successorReachabilityPatterns) {
        Assert-Cp410Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded CP411 reachability claim'
    }
}
$specAddenda = @(
    [PSCustomObject]@{ Path = 'specs\algorithm_ledger.toml'; Anchor = 'CP410 supersedes only CP409' },
    [PSCustomObject]@{ Path = 'specs\capabilities.toml'; Anchor = 'CP410 additionally requires' }
)
foreach ($specAddendum in $specAddenda) {
    $specText = Read-RepoText -Path $specAddendum.Path
    $addendumPattern = '(?m)^\s*"(?<body>' + [regex]::Escape($specAddendum.Anchor) + '.*)",\r?$'
    $addendumMatches = [regex]::Matches($specText, $addendumPattern)
    if ($addendumMatches.Count -ne 1) {
        throw "CP410 expected one bounded addendum in $($specAddendum.Path), found $($addendumMatches.Count)"
    }
    foreach ($pattern in $successorReachabilityPatterns) {
        Assert-Cp410Text -Text $addendumMatches[0].Groups['body'].Value -Pattern $pattern -Description "bounded CP411 reachability addendum in $($specAddendum.Path)"
    }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP410 supersedes only CP409' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP410 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP410\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP410\b' -Description 'psychrometrics-map non-promotion'

$ledgerText = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledgerText,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.source_file\s*=\s*').Count -ne 293 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or
    [regex]::Matches($ledgerText,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP410 algorithm/routine ledger counts drift' }

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 409) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp410_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 409) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 348' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 409) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 108' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 409) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 348 \|')) -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 108 \|')) -Description 'historical generated internal total'
    }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP410 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminal = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 409)) })
if ($terminal.Count -ne 33) { throw 'CP410 terminal propagation set drift' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP409-to-CP410' -Description 'historical terminal interval' }
$cp345 = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp410Call\s*=','CP409-to-CP410','CP410-to-numerical')) { Assert-Contains -Path $cp345 -Pattern $pattern -Description 'CP345 terminal chain' }
Assert-LineLimit -Path $cp345 -Limit 1200 -Description 'CP345 fixed structural cap'
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405|406|407|408|409)-' })) { Assert-Contains -Path $file.FullName -Pattern 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break' -Description 'recent CP410 binding order' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406|407|408|409)-' })) { Assert-Contains -Path $file.FullName -Pattern '\$cp410Call' -Description 'recent CP410 terminal capture' }
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp409-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-case-break.ps1' -Pattern 'calculation_\$stem\\b"\)\.Count -ne 3' -Description 'CP409 successor-consumption binding count'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 94' -Description 'calc-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1' -Pattern 'Assert-LineLimit -Path \$cp339CalcRoot -Limit 94' -Description 'historical calc-root structural cap'

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp409-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-case-break.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP410 registration order drift' }

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 348','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp410Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 108) { throw 'CP410 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp410-cooling-post-saturation-capacity-limit-dehumidification-control-default-case-break\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 348 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 108 \|' -Description 'generated internal total'

Write-Host 'CP410 post-saturation default-case break structure audit passed.'
}
