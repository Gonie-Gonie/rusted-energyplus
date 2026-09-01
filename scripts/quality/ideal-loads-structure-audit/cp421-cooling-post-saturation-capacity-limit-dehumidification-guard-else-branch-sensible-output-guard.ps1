# CP421 maps PurchasedAirManager.cc physical line 2332's sensible-output maximum-capacity guard.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuard'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$predecessorModule = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$snapshot = "$root\transition\snapshot.rs"
$release = "$root\release.rs"
$releaseError = "$root\release\error.rs"
$releasePrefix = "$root\release\prefix.rs"
$releaseRuntime = "$root\release\runtime_validation.rs"
$releaseSnapshot = "$root\release\snapshot_validation.rs"
$tests = "$root\tests.rs"
$schemaIeeeTests = "$root\tests\schema_ieee.rs"
$overflowTests = "$root\tests\overflow.rs"
$cp321Committed = 'crates\ep_runtime\src\ideal_loads\calc\cooling_capacity_zero_flow_reset\release\committed.rs'
$cp340Committed = 'crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_capacity_limit_sensible_output_guard\release\committed.rs'
$cp420Committed = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem\release\committed.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp421.rs'
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serializationRoot = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp421_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp420_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp421-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-guard.ps1'

function Assert-Cp421Text {
    param([string]$Text,[string]$Pattern,[string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP421 $Description missing '$Pattern'" }
}

function Get-Cp421BraceBlock {
    param([string]$Text,[string]$AnchorPattern,[string]$Description)
    $anchors = [regex]::Matches($Text,$AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP421 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{',$anchors[0].Index)
    if ($open -lt 0) { throw "CP421 $Description opening brace missing" }
    $depth = 0
    for ($index=$open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') { $depth -= 1; if ($depth -eq 0) { return $Text.Substring($anchors[0].Index,$index-$anchors[0].Index+1) } }
    }
    throw "CP421 $Description closing brace missing"
}

$required = @($source,$module,$predecessorModule,$state,$transition,$accounting,$snapshot,$release,$releaseError,$releasePrefix,$releaseRuntime,$releaseSnapshot,$tests,$schemaIeeeTests,$overflowTests,
    $cp321Committed,$cp340Committed,$cp420Committed,$binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledTests,$witness,
    $pipelineRoot,$pipeline,$pipelineValidation,$pipelineLineage,$serializationRoot,$serialization,$arbitrary,$arbitraryPredecessor,$audit)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP421 implementation/audit file' }
foreach ($file in @($module,$state,$transition,$accounting,$snapshot,$release,$releaseError,$releasePrefix,$releaseRuntime,$releaseSnapshot,$tests,$schemaIeeeTests,$overflowTests,
    $cp321Committed,$cp340Committed,$cp420Committed,$adapter,$adapterTests,$coupled,$coupledTests,$witness,$pipeline,$pipelineValidation,$pipelineLineage,$serializationRoot,$serialization,$arbitrary,$audit)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP421/hot-seal file'
}
foreach ($file in @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')) { Assert-LineLimit -Path $file.FullName -Limit 500 -Description 'bounded CP421 core subtree file' }
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
if ($coreFiles.Count -ne 13 -or @(Get-ChildItem -LiteralPath "$root\tests" -File -Filter '*.rs').Count -ne 2) { throw 'CP421 exact thirteen-file core/two-file split-test topology drift' }

$registrations = @(
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\calc.rs';Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\init\state.rs';Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs';Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs';Pattern="mod\s+$stem"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs';Pattern="$($stem)_tests"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs';Pattern="mod\s+$($stem)_validation"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs';Pattern='test_coupled_runtime_cp421\.rs'},
    [PSCustomObject]@{Path=$module;Pattern='(?s)mod\s+release;.*?mod\s+state;.*?mod\s+tests;.*?mod\s+transition;'},
    [PSCustomObject]@{Path=$transition;Pattern='(?s)mod\s+accounting;.*?mod\s+snapshot;'},
    [PSCustomObject]@{Path=$pipelineRoot;Pattern="mod\s+$pipelineStem;"},
    [PSCustomObject]@{Path=$pipeline;Pattern='(?s)mod\s+serialization;.*?mod\s+validation;'},
    [PSCustomObject]@{Path=$pipelineValidation;Pattern='mod\s+lineage;'},
    [PSCustomObject]@{Path=$serializationRoot;Pattern='mod\s+snapshot;'}
)
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP421 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP421 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2331].Trim() -cne 'if (CoolSensOutput >= PurchAir.MaxCoolTotCap) {' -or $sourceLines[2332].Trim() -cne 'CoolSensOutput = PurchAir.MaxCoolTotCap;') { throw 'CP421 exact source/first-excluded boundary drift' }
$sites = @(
    'read-retained-cooling-sensible-output-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-maximum-capacity-comparison',
    'read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-comparison',
    'compare-post-saturation-capacity-limit-dehumidification-guard-else-branch-cooling-sensible-output-greater-than-or-equal-to-maximum-total-cooling-capacity',
    'enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-capacity-adjustment-body-if-comparison-satisfied'
)
Assert-Contains -Path $module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2332' -Description 'source constant'
Assert-Contains -Path $module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2333' -Description 'first excluded constant'
Assert-ExactStringArray -Path $module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER' -Expected $sites -Description 'exact four source sites'
foreach ($test in @('cp421_boundary_and_four_sites_are_exact','exhaustive_54_predecessors_refine_to_59_successors_with_exact_accounting','inactive_route_is_owner_lazy_and_rejects_supplied_operands_transactionally','hot_release_and_pending_validation_have_no_recursive_lineage_calls')) { Assert-Contains -Path $tests -Pattern $test -Description 'CP421 core contract regression' }
foreach ($test in @('snapshot_schema_is_exact_220_76_2_1_with_cp420_first_199_and_unique_tail','raw_ieee_greater_equal_truth_table_and_payload_bits_are_preserved','route_derived_owner_and_preservation_pairs_reject_equal_counter_forgeries')) { Assert-Contains -Path $schemaIeeeTests -Pattern $test -Description 'CP421 schema/IEEE regression' }
Assert-Contains -Path $overflowTests -Pattern 'every_mutable_scalar_and_all_three_route_arrays_overflow_transactionally' -Description 'CP421 transactional-overflow regression'

$moduleText = Read-RepoText -Path $module
$predecessorText = Read-RepoText -Path $predecessorModule
$snapshotStruct = Get-Cp421BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$predecessorStruct = Get-Cp421BraceBlock -Text $predecessorText -AnchorPattern 'pub\s+struct\s+PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot\s*' -Description 'CP420 snapshot'
$fields = @([regex]::Matches($snapshotStruct,'(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object {$_.Groups['name'].Value})
$predecessorFields = @([regex]::Matches($predecessorStruct,'(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object {$_.Groups['name'].Value})
$predecessorTerminal = @('predecessor_cp420_resulting_supply_humidity_ratio','predecessor_cp420_resulting_supply_enthalpy_j_per_kg','predecessor_cp420_resulting_supply_temperature_c')
$localFields = @(
    'post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated',
    'cp420_retained_cooling_sensible_output_owned_read','cooling_sensible_output_read','cp420_cooling_sensible_output_for_capacity_guard_w',
    'cp321_maximum_total_cooling_capacity_owned_read','cp340_same_call_maximum_total_cooling_capacity_bit_corroborated','maximum_total_cooling_capacity_read','maximum_total_cooling_capacity_w',
    'cooling_sensible_output_maximum_total_cooling_capacity_comparison_evaluated','cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity',
    'post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered',
    'post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_false_fallthrough',
    'cp420_retained_supply_humidity_ratio_state_owned','cp420_retained_supply_enthalpy_state_owned','cp420_retained_supply_temperature_state_owned'
)
$terminal = @('resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c')
$expectedFields = @($predecessorFields[0..198]) + $predecessorTerminal + $localFields + $terminal
if ($predecessorFields.Count -ne 202 -or $fields.Count -ne 220 -or @($fields|Sort-Object -Unique).Count -ne 220 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP421 exact CP420 first-199 plus twenty-one-field tail schema drift' }
$numeric = @([regex]::Matches($snapshotStruct,'(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object {$_.Groups['name'].Value})
if ($numeric.Count -ne 76 -or [regex]::Matches($snapshotStruct,'Option<bool>').Count -ne 2 -or [regex]::Matches($snapshotStruct,'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP421 76 numeric/two comparison/one control carrier drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText,'(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object {$_.Groups['name'].Value})
if (($routeArrays -join '|') -cne 'predecessor_route_counts|guard_false_fallthrough_route_counts|adjustment_body_entry_route_counts') { throw 'CP421 exact three width-36 route arrays drift' }
foreach ($counter in @('transition_count','inactive_transition_count','source_site_execution_count','cp420_cooling_sensible_output_owned_read_count','cooling_sensible_output_read_count','cp321_maximum_total_cooling_capacity_owned_read_count','cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count','maximum_total_cooling_capacity_read_count','cooling_sensible_output_maximum_total_cooling_capacity_comparison_count','cooling_sensible_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count')) { Assert-Cp421Text -Text $stateText -Pattern ("pub\s+"+[regex]::Escape($counter)+"\s*:\s*usize") -Description 'state counter' }

$transitionText = Read-RepoText -Path $transition
foreach ($pattern in @('matches!\(route\.logical_index,\s*4\s*\|\s*7\s*\|\s*10\s*\|\s*13\s*\|\s*16\)','source_greater_than_or_equal\(\s*input\.cooling_sensible_output_w,\s*input\.maximum_total_cooling_capacity_w','fn\s+source_greater_than_or_equal\(left:\s*f64,\s*right:\s*f64\)\s*->\s*bool\s*\{\s*left\s*>=\s*right\s*\}')) { Assert-Cp421Text -Text $transitionText -Pattern "(?s)$pattern" -Description 'raw >= guard transition' }
foreach ($pattern in @('total_cmp','partial_cmp','epsilon|tolerance|approx','clamp\s*\(','normalize','DirectZonePurchasedAirCouplingInput')) { Assert-NotContains -Path $transition -Pattern $pattern -Description 'guard forbidden operation/feed' }
foreach ($file in @($transition,$accounting,$snapshot,$release,$releaseError,$releasePrefix,$releaseRuntime,$releaseSnapshot,$adapter,$coupled,$pipelineValidation,$pipelineLineage)) { Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic' }

foreach ($file in @($cp321Committed,$cp340Committed,$cp420Committed)) {
    foreach ($pattern in @('completed_','snapshot_is_exact','(?<![A-Za-z0-9_])predecessor_route\s*\(')) { Assert-NotContains -Path $file -Pattern $pattern -Description 'sealed hot capability recursion' }
}
Assert-Contains -Path $cp321Committed -Pattern 'cooling_capacity_zero_flow_reset_committed_latest_maximum_total_cooling_capacity' -Description 'sealed CP321 maximum-capacity owner'
Assert-Contains -Path $cp340Committed -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity' -Description 'sealed CP340 bit corroboration'
Assert-Contains -Path $cp340Committed -Pattern 'cooling_capacity_zero_flow_reset_committed_latest_maximum_total_cooling_capacity\s*\(' -Description 'CP340 combined seal calls CP321 owner seal'
Assert-Contains -Path $cp420Committed -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_latest_route_and_cooling_sensible_output' -Description 'sealed CP420 route/output owner'
Assert-Contains -Path $module -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot' -Description 'bounded CP420 reconstruction export'
$releaseText = Read-RepoText -Path $release
$hot = Get-Cp421BraceBlock -Text $releaseText -AnchorPattern "pub\s+fn\s+advance_direct_no_oa_calc_$stem\s*\(" -Description 'public hot release'
foreach ($pattern in @('cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_latest_route_and_cooling_sensible_output\s*\(','cooling_positive_supply_capacity_limit_sensible_output_guard_committed_latest_maximum_total_cooling_capacity\s*\(','cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_route_from_committed_predecessor\s*\(','advance_with_route\s*\(')) { Assert-Cp421Text -Text $hot -Pattern $pattern -Description 'sealed bounded hot release' }
foreach ($pattern in @('completed_','snapshot_is_exact','(?<![A-Za-z0-9_])predecessor_route\s*\(')) { if ($hot -match $pattern) { throw "CP421 public hot release recursively validates through '$pattern'" } }

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\s*=','let\s+calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry\s*=','let\s+calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment\s*=','let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP420-to-CP421-to-CP422-to-CP423-to-CP424-to-CP425-to-CP426 binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+coupling\s*:') -Description 'CP420-to-CP421 scheduled output order'
Assert-Contains -Path $adapterTests -Pattern 'cp421_is_preserved_before_cp422_in_the_current_127_snapshot_binding' -Description 'current 123-snapshot binding regression'
$bindingText = Read-RepoText -Path $binding
$dto = Get-Cp421BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp421|sensible_output_guard') { throw 'CP421 evidence entered numerical DTO' }
Assert-Contains -Path $witness -Pattern ("set_"+$stem+"_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle") -Description 'pipeline CP420-to-CP421 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp436_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp421_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp421_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp421_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
foreach ($pattern in @('\(59usize,\s*49usize,\s*5usize,\s*5usize,\s*19usize,\s*40usize\)','assert_eq!\(conceptual,\s*inactive \+ false_paths \+ body_paths\)','assert_eq!\(conceptual,\s*public \+ private\)','object\.len\(\),\s*296','76')) { Assert-Contains -Path $arbitrary -Pattern $pattern -Description 'arbitrary exact route/schema count' }

$serializationText = Read-RepoText -Path $serialization
$predecessorSerialization = "crates\ep_run\src\pipeline\purchased_air_$predecessorStem\serialization\snapshot.rs"
$predecessorJson = @([regex]::Matches((Read-RepoText -Path $predecessorSerialization),'(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object {$_.Groups['key'].Value})
$jsonTail = @(
    'predecessor_cp420_resulting_supply_humidity_ratio','predecessor_cp420_resulting_supply_humidity_ratio_ieee_bits','predecessor_cp420_resulting_supply_enthalpy_j_per_kg','predecessor_cp420_resulting_supply_enthalpy_j_per_kg_ieee_bits','predecessor_cp420_resulting_supply_temperature_c','predecessor_cp420_resulting_supply_temperature_c_ieee_bits',
    $localFields[0],$localFields[1],$localFields[2],'cp420_cooling_sensible_output_for_capacity_guard_w','cp420_cooling_sensible_output_for_capacity_guard_w_ieee_bits',$localFields[4],$localFields[5],$localFields[6],'maximum_total_cooling_capacity_w','maximum_total_cooling_capacity_w_ieee_bits',$localFields[8],$localFields[9],$localFields[10],$localFields[11],$localFields[12],$localFields[13],$localFields[14],
    'resulting_supply_humidity_ratio','resulting_supply_humidity_ratio_ieee_bits','resulting_supply_enthalpy_j_per_kg','resulting_supply_enthalpy_j_per_kg_ieee_bits','resulting_supply_temperature_c','resulting_supply_temperature_c_ieee_bits'
)
$serializedTail = @([regex]::Matches($serializationText,'(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object {$_.Groups['key'].Value})
if ($predecessorJson.Count -ne 273 -or $jsonTail.Count -ne 29 -or ($serializedTail -join '|') -cne ($jsonTail -join '|')) { throw 'CP421 exact CP420 first-267 plus 29-key JSON tail drift' }
foreach ($key in $jsonTail) { Assert-Cp421Text -Text $serializationText -Pattern ('"'+[regex]::Escape($key)+'"') -Description 'exact JSON tail key' }
Assert-Contains -Path $serialization -Pattern 'cp420_snapshot_json\(predecessor\)' -Description 'CP420 JSON reconstruction'
Assert-Contains -Path $serialization -Pattern 'static_schema_is_first_267_exact_then_29_unique_entries_with_76_sidecars' -Description 'exact JSON prefix/tail/sidecar regression'
$serializationProduction = [regex]::Split($serializationText,'(?m)^#\[cfg\(test\)\]\r?$',2)[0]
if ($serializationProduction -match 'DirectZonePurchasedAirCouplingInput|numerical_dto|prediction|feedback|nodes|loads|reports') { throw 'serializer numerical feed unexpectedly present in production source' }

$heading = 'CP421 post-saturation capacity-limit dehumidification-guard else-branch sensible-output maximum-capacity guard'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
$canonical = $null
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    if ([regex]::Matches($text,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP421 heading count drift in $doc" }
    $section = [regex]::Match($text,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## CP422\b)").Groups['body'].Value.TrimEnd([char[]]"`r`n")
    if ($null -eq $canonical) {$canonical=$section} elseif ($section -cne $canonical) { throw "CP421 manual section drift in $doc" }
}
foreach ($pattern in @('physical executable/control line 2332 exactly','line\s+2333.*?first excluded.*?CP422','exact four dependency-ordered sites','T421=59','I421=49','Q421=10','F421=5','B421=5','S421=3\*Q421\+B421=35','19/40','Three width-36','36/41/56','220 base fields','seventy-six\s+`Option<f64>`','two optional comparison bools','296\s+unique JSON keys','exact first 267 keys','29-key tail','CP420-to-CP421-to-unchanged-numerical','111 to 112','359 total','240 public','119 internal')) { Assert-Cp421Text -Text $canonical -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
foreach ($spec in @([PSCustomObject]@{Path='specs\algorithm_ledger.toml';Anchor='CP421 supersedes only CP420'},[PSCustomObject]@{Path='specs\capabilities.toml';Anchor='CP421 additionally requires'})) {
    $matches=[regex]::Matches((Read-RepoText -Path $spec.Path),'(?m)^\s*"(?<body>'+[regex]::Escape($spec.Anchor)+'.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP421 expected one bounded addendum in $($spec.Path)" }
    foreach ($pattern in @('2332','2333.*?CP422','59.*?49.*?10.*?5.*?5.*?35','19/40','three width-36','220 base','seventy-six','296','267','29-key','111 to 112','359.*?240.*?119')) { Assert-Cp421Text -Text $matches[0].Groups['body'].Value -Pattern "(?is)$pattern" -Description 'bounded spec claim' }
}
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP421\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP421\b' -Description 'psychrometrics-map non-promotion'

$ledger=Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledger,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.source_file\s*=').Count -ne 293 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP421 ledger counts drift' }
$auditRoot='scripts\quality\ideal-loads-structure-audit'; $audits=@(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) { if ($file.BaseName -notmatch '^cp(?<number>\d+)-') {continue}; $number=[int]$Matches['number']; if ($number -ge 334 -and $number -le 421) {Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp436_lifecycle_evidence' -Description 'historical non-direct firewall'}; if ($number -ge 337 -and $number -le 421) {Assert-Contains -Path $file.FullName -Pattern 'script_count = 374' -Description 'historical script count'}; if ($number -ge 367 -and $number -le 421) {Assert-Contains -Path $file.FullName -Pattern 'Count -ne 134' -Description 'historical classification count'}; if ($number -ge 335 -and $number -le 421) {Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 374 \|')) -Description 'historical generated total';Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 134 \|')) -Description 'historical generated internal total'} }
$cleanup=@((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File|Where-Object {$_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344})
if ($cleanup.Count -ne 17) {throw 'CP421 helper-cleanup propagation set drift'}; foreach ($file in $cleanup) {Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist'}
$terminal=@((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File);$audits|Where-Object {$_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 420))})
if ($terminal.Count -ne 44) {throw 'CP421 terminal propagation set drift'}; foreach ($file in $terminal) {Assert-Contains -Path $file.FullName -Pattern '\$cp421Call' -Description 'CP421 terminal capture';Assert-Contains -Path $file.FullName -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval';Assert-Contains -Path $file.FullName -Pattern '\$cp422Call' -Description 'CP422 terminal capture';Assert-Contains -Path $file.FullName -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval';Assert-Contains -Path $file.FullName -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 interval'}
$master=Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'; $previous=$master.IndexOf('cp420-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-assignment.ps1');$current=$master.IndexOf((Split-Path -Leaf $audit));$completion=$master.IndexOf('Write-Host "IdealLoads structure audit complete."');if ($previous -lt 0 -or $current -le $previous -or $completion -le $current -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) {throw 'Master CP420-to-CP421 registration order drift'}
if ((Get-Content -LiteralPath 'scripts\quality\ideal-loads-structure-audit.ps1').Count -gt 4200) {throw 'IdealLoads master audit line cap exceeded after CP421 registration'}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 99' -Description 'CP421 calc-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$idealLoadsInitWitnesses -Limit 272' -Description 'CP421 witness-root structural cap'
if ((Get-Content -LiteralPath 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1').Count -gt 1200) {throw 'CP345 line cap exceeded after CP421 terminal propagation'};Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture';Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval';Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal-to-numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
$inventory=Read-RepoText -Path 'specs\script_inventory.toml';foreach ($pattern in @('script_count = 374','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) {Assert-Cp421Text -Text $inventory -Pattern $pattern -Description 'inventory count'};if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 134) {throw 'CP421 inventory classification drift'}
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp421-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-guard\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP421 supersedes only CP420' -Description 'generated algorithm addendum';Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP421 additionally requires' -Description 'generated capability addendum';Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 374 \|' -Description 'generated script total';Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 134 \|' -Description 'generated internal total'
Write-Host 'CP421 post-saturation dehumidification-guard else-branch sensible-output maximum-capacity guard structure audit passed.'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment' -Description 'CP425 binding successor registration'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment' -Description 'CP426 binding successor registration'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment' -Description 'CP427 recent binding propagation'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment' -Description 'CP428 recent binding propagation'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment' -Description 'CP429 recent binding propagation'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_heating_or_no_load_case_entry' -Description 'CP430 recent binding propagation'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'
