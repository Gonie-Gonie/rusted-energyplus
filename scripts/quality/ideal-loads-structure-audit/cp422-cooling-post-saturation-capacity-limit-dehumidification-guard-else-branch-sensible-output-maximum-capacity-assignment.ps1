# CP422 maps PurchasedAirManager.cc physical line 2333's sensible-output maximum-capacity assignment.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignment'
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
$schemaTests = "$root\tests\schema_ieee.rs"
$overflowTests = "$root\tests\overflow.rs"
$cp421Committed = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem\release\committed.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp422.rs'
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serializationRoot = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp422_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp421_assertions.rs'
$audit = "scripts\quality\ideal-loads-structure-audit\cp422-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-maximum-capacity-assignment.ps1"

function Assert-Cp422Text {
    param([string]$Text,[string]$Pattern,[string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP422 $Description missing '$Pattern'" }
}

function Get-Cp422BraceBlock {
    param([string]$Text,[string]$AnchorPattern,[string]$Description)
    $anchors = [regex]::Matches($Text,$AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP422 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{',$anchors[0].Index)
    if ($open -lt 0) { throw "CP422 $Description opening brace missing" }
    $depth=0
    for ($index=$open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') {$depth += 1}
        elseif ($Text[$index] -eq '}') {$depth -= 1; if ($depth -eq 0) {return $Text.Substring($open + 1,$index - $open - 1)}}
    }
    throw "CP422 $Description closing brace missing"
}

$required = @($source,$module,$predecessorModule,$state,$transition,$accounting,$snapshot,$release,$releaseError,$releasePrefix,$releaseRuntime,$releaseSnapshot,$tests,$schemaTests,$overflowTests,$cp421Committed,$binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledTests,$witness,$pipelineRoot,$pipeline,$pipelineValidation,$pipelineLineage,$serializationRoot,$serialization,$arbitrary,$arbitraryPredecessor,$audit)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP422 implementation/audit file' }
foreach ($file in @($module,$state,$transition,$accounting,$snapshot,$release,$releaseError,$releasePrefix,$releaseRuntime,$releaseSnapshot,$tests,$schemaTests,$overflowTests,$cp421Committed,$adapter,$adapterTests,$coupled,$coupledTests,$witness,$pipeline,$pipelineValidation,$pipelineLineage,$serializationRoot,$serialization,$arbitrary,$audit)) { Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP422 file' }
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
if ($coreFiles.Count -ne 13 -or @(Get-ChildItem -LiteralPath "$root\tests" -File -Filter '*.rs').Count -ne 2) { throw 'CP422 exact thirteen-file core/two-file split-test topology drift' }

$registrations = @(
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\calc.rs';Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\init\state.rs';Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs';Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs';Pattern="$($stem)_tests"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\binding.rs';Pattern="mod\s+$stem;"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs';Pattern="calculation_$stem"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs';Pattern="mod\s+$($stem)_validation"},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs';Pattern='test_coupled_runtime_cp422\.rs'},
    [PSCustomObject]@{Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs';Pattern="$stem"},
    [PSCustomObject]@{Path=$module;Pattern='(?s)mod\s+release;.*?mod\s+state;.*?mod\s+tests;.*?mod\s+transition;'},
    [PSCustomObject]@{Path=$transition;Pattern='(?s)mod\s+accounting;.*?mod\s+snapshot;'},
    [PSCustomObject]@{Path='crates\ep_run\src\pipeline.rs';Pattern="mod\s+$pipelineStem;"},
    [PSCustomObject]@{Path=$pipeline;Pattern='(?s)mod\s+serialization;.*?mod\s+validation;'},
    [PSCustomObject]@{Path=$pipelineValidation;Pattern='mod\s+lineage;'},
    [PSCustomObject]@{Path=$serializationRoot;Pattern='mod\s+snapshot;'}
)
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP422 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP422 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2332].Trim() -cne 'CoolSensOutput = PurchAir.MaxCoolTotCap;' -or $sourceLines[2333].Trim() -cne 'PurchAir.SupplyTemp = PurchAir.MixedAirTemp - CoolSensOutput / (SupplyMassFlowRate * CpAir);') { throw 'CP422 exact source/first-excluded boundary drift' }
$sites = @(
    'read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-assignment',
    'assign-local-cooling-sensible-output-from-maximum-total-cooling-capacity'
)
Assert-ExactStringArray -Path $module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER' -Expected $sites -Description 'exact two assignment sites'
Assert-Contains -Path $module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2333' -Description 'source constant'
Assert-Contains -Path $module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2334' -Description 'first excluded constant'
foreach ($test in @('cp422_boundary_and_two_sites_are_exact','exhaustive_59_routes_have_exact_assignment_and_owner_accounting','inactive_route_is_owner_lazy_and_rejects_supplied_values_transactionally','validated_route_and_owner_value_forgeries_are_transactional','hot_release_and_cp421_committed_owner_are_bounded')) { Assert-Contains -Path $tests -Pattern $test -Description 'CP422 core contract regression' }
foreach ($test in @('snapshot_schema_is_exact_234_83_2_1_with_cp421_first_217_and_unique_tail','predecessor_reconstruction_and_cold_validated_paths_are_bit_exact','source_assignment_copies_all_non_nan_ieee_classes_bit_exact','nan_guard_false_preserves_preexisting_payload_and_bit_comparator_detects_change')) { Assert-Contains -Path $schemaTests -Pattern $test -Description 'CP422 schema/IEEE regression' }
foreach ($test in @('common_transition_route_and_owner_overflows_are_transactional','inactive_counter_overflow_is_transactional','guard_false_counter_overflows_are_transactional','assignment_counter_and_site_overflows_are_transactional')) { Assert-Contains -Path $overflowTests -Pattern $test -Description 'CP422 transactional-overflow regression' }

$moduleText = Read-RepoText -Path $module
$predecessorText = Read-RepoText -Path $predecessorModule
$stateText = Read-RepoText -Path $state
$transitionText = Read-RepoText -Path $transition
$releaseText = Read-RepoText -Path $release
$snapshotStruct = Get-Cp422BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$predecessorStruct = Get-Cp422BraceBlock -Text $predecessorText -AnchorPattern 'pub\s+struct\s+PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot\s*' -Description 'CP421 snapshot'
$fields = @([regex]::Matches($snapshotStruct,'(?m)^\s*pub\s+(?<name>[A-Za-z_][A-Za-z0-9_]*)\s*:') | ForEach-Object {$_.Groups['name'].Value})
$predecessorFields = @([regex]::Matches($predecessorStruct,'(?m)^\s*pub\s+(?<name>[A-Za-z_][A-Za-z0-9_]*)\s*:') | ForEach-Object {$_.Groups['name'].Value})
$predecessorTerminal = @('predecessor_cp421_resulting_supply_humidity_ratio','predecessor_cp421_resulting_supply_enthalpy_j_per_kg','predecessor_cp421_resulting_supply_temperature_c')
$localFields = @(
    'post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed',
    'cp421_retained_supply_humidity_ratio_state_owned','cp421_retained_supply_enthalpy_state_owned','cp421_retained_supply_temperature_state_owned',
    'preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w','cp421_retained_maximum_total_cooling_capacity_owned_read',
    'maximum_total_cooling_capacity_for_sensible_output_assignment_read','maximum_total_cooling_capacity_for_sensible_output_assignment_w',
    'cooling_sensible_output_maximum_capacity_assignment_performed','assigned_cooling_sensible_output_from_maximum_capacity_w',
    'resulting_cooling_sensible_output_after_maximum_capacity_assignment_w'
)
$terminal = @('resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c')
$expectedFields = @($predecessorFields[0..216] + $predecessorTerminal + $localFields + $terminal)
if ($predecessorFields.Count -ne 220 -or $fields.Count -ne 234 -or @($fields|Sort-Object -Unique).Count -ne 234 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP422 exact CP421 first-217 plus seventeen-field tail schema drift' }
if ([regex]::Matches($snapshotStruct,'Option<f64>').Count -ne 83 -or [regex]::Matches($snapshotStruct,'Option<bool>').Count -ne 2 -or [regex]::Matches($snapshotStruct,'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP422 83 numeric/two comparison/one control carrier drift' }
$routeArrays = @([regex]::Matches($stateText,'(?m)^\s*pub\s+(?<name>[A-Za-z_][A-Za-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object {$_.Groups['name'].Value})
if (($routeArrays -join '|') -cne 'predecessor_route_counts|predecessor_guard_false_fallthrough_route_counts|cooling_sensible_output_maximum_capacity_assignment_route_counts') { throw 'CP422 exact three width-36 route arrays drift' }
foreach ($counter in @('transition_count','inactive_transition_count','predecessor_guard_false_fallthrough_count','cooling_sensible_output_maximum_capacity_assignment_count','source_site_execution_count','cp421_supply_humidity_ratio_state_owner_count','unchanged_supply_humidity_ratio_preservation_count','cp421_supply_enthalpy_state_owner_count','unchanged_supply_enthalpy_preservation_count','cp421_supply_temperature_state_owner_count','unchanged_supply_temperature_preservation_count','cp421_retained_maximum_total_cooling_capacity_owned_read_count','maximum_total_cooling_capacity_for_sensible_output_assignment_read_count','cooling_sensible_output_maximum_capacity_assignment_write_count')) { Assert-Cp422Text -Text $stateText -Pattern ("pub\s+"+[regex]::Escape($counter)+"\s*:\s*usize") -Description 'state counter' }
foreach ($pattern in @('matches!\(predecessor_route\.logical_index,\s*4\s*\|\s*7\s*\|\s*10\s*\|\s*13\s*\|\s*16\)','assignment_executed\s*=\s*active\s*&&\s*predecessor_route\.body_entered','result\s*=\s*if\s+route\.assignment_executed\s*\{\s*prepared\.maximum_total_cooling_capacity_w\s*\}\s*else\s*\{\s*prepared\.preexisting_cooling_sensible_output_w','maximum_total_cooling_capacity_w:\s*route\s*\.assignment_executed\s*\.then_some')) { Assert-Cp422Text -Text $transitionText -Pattern "(?s)$pattern" -Description 'raw bit-copy assignment transition' }
foreach ($forbidden in @('Psy','mul_add','total_cmp','partial_cmp')) { Assert-NotContains -Path $transition -Pattern $forbidden -Description 'excluded CP422 arithmetic/psychrometrics' }
foreach ($file in @($transition,$accounting,$snapshot,$release,$releaseError,$releasePrefix,$releaseRuntime,$releaseSnapshot,$adapter,$coupled,$pipelineValidation,$pipelineLineage)) { Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic' }

foreach ($pattern in @('completed_','snapshot_is_exact','(?<![A-Za-z0-9_])predecessor_route\s*\(')) { Assert-NotContains -Path $cp421Committed -Pattern $pattern -Description 'sealed CP421 hot capability recursion' }
Assert-Contains -Path $cp421Committed -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_committed_latest_route_and_assignment_values' -Description 'sealed CP421 route/operand owner'
$hot = Get-Cp422BraceBlock -Text $releaseText -AnchorPattern "pub\s+fn\s+advance_direct_no_oa_calc_$stem\s*\(" -Description 'public hot release'
foreach ($pattern in @('cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_committed_latest_route_and_assignment_values\s*\(','cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_route_from_committed_predecessor\s*\(','advance_with_route\s*\(')) { Assert-Cp422Text -Text $hot -Pattern $pattern -Description 'sealed bounded hot release' }
foreach ($pattern in @('completed_','snapshot_is_exact','(?<![A-Za-z0-9_])predecessor_route\s*\(')) { if ($hot -match $pattern) { throw "CP422 public hot release recursively validates through '$pattern'" } }

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\s*=','let\s+calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry\s*=','let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP421-to-CP422-to-CP423-to-CP424-to-numerical binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\s*:','pub\s+coupling\s*:') -Description 'CP421-to-CP422-to-CP423 scheduled output order'
$bindingText = Read-RepoText -Path $binding
$dto = Get-Cp422BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp422|maximum_capacity_assignment') { throw 'CP422 evidence entered numerical DTO' }
Assert-Contains -Path $witness -Pattern ("set_"+$stem+"_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle",'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment::\s*validate_direct_lifecycle') -Description 'pipeline CP421-to-CP422-to-CP423 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp424_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp422_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp422_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp422_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
foreach ($pattern in @('object\.len\(\),\s*317','ends_with\("_ieee_bits"\)\)\s*\.count\(\),\s*83')) { Assert-Contains -Path $arbitrary -Pattern $pattern -Description 'arbitrary exact schema count' }
foreach ($pattern in @('\(59usize,\s*49usize,\s*5usize,\s*5usize\)','assert_eq!\(conceptual,\s*inactive \+ false_paths \+ assignment_paths\)')) { Assert-Contains -Path $arbitrary -Pattern $pattern -Description 'arbitrary exact route count' }
foreach ($test in @('scheduled_binding_advances_cp422_after_cp421_before_unchanged_coupling','cp422_adapter_accepts_only_the_cp421_snapshot','cp422_remains_in_the_extended_114_snapshot_binding')) { Assert-Contains -Path $adapterTests -Pattern $test -Description 'binding regression' }
foreach ($test in @('cp422_conceptual_contract_has_59_outcomes_49_inactive_5_false_5_assignments_and_two_sites','cp422_new_state_has_three_zeroed_lossless_route_partitions','cp422_binding_and_pipeline_keep_numerical_dto_unchanged')) { Assert-Contains -Path $coupledTests -Pattern $test -Description 'coupled regression' }
Assert-Contains -Path $serialization -Pattern 'static_schema_is_first_290_exact_then_27_unique_entries_with_83_sidecars' -Description 'exact JSON prefix/tail/sidecar regression'
$serializationText = Read-RepoText -Path $serialization
$jsonTail = @(
    'predecessor_cp421_resulting_supply_humidity_ratio','predecessor_cp421_resulting_supply_humidity_ratio_ieee_bits','predecessor_cp421_resulting_supply_enthalpy_j_per_kg','predecessor_cp421_resulting_supply_enthalpy_j_per_kg_ieee_bits','predecessor_cp421_resulting_supply_temperature_c','predecessor_cp421_resulting_supply_temperature_c_ieee_bits',
    $localFields[0],$localFields[1],$localFields[2],$localFields[3],$localFields[4],'preexisting_cooling_sensible_output_for_maximum_capacity_assignment_w_ieee_bits',$localFields[5],$localFields[6],$localFields[7],'maximum_total_cooling_capacity_for_sensible_output_assignment_w_ieee_bits',$localFields[8],$localFields[9],'assigned_cooling_sensible_output_from_maximum_capacity_w_ieee_bits',$localFields[10],'resulting_cooling_sensible_output_after_maximum_capacity_assignment_w_ieee_bits',
    'resulting_supply_humidity_ratio','resulting_supply_humidity_ratio_ieee_bits','resulting_supply_enthalpy_j_per_kg','resulting_supply_enthalpy_j_per_kg_ieee_bits','resulting_supply_temperature_c','resulting_supply_temperature_c_ieee_bits'
)
$serializedTail = @([regex]::Matches($serializationText,'(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object {$_.Groups['key'].Value})
if ($jsonTail.Count -ne 27 -or ($serializedTail -join '|') -cne ($jsonTail -join '|')) { throw 'CP422 exact 27-key JSON tail drift' }
Assert-Contains -Path $serialization -Pattern 'cp421_snapshot_json\(predecessor\)' -Description 'CP421 JSON reconstruction'
$serializationProduction = [regex]::Split($serializationText,'(?m)^#\[cfg\(test\)\]\r?$',2)[0]
if ($serializationProduction -match 'DirectZonePurchasedAirCouplingInput|numerical_dto|prediction|feedback|nodes|loads|reports') { throw 'CP422 serializer numerical feed unexpectedly present' }

$heading = 'CP422 post-saturation capacity-limit dehumidification-guard else-branch sensible-output maximum-capacity assignment'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
$canonical = $null
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    if ([regex]::Matches($text,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP422 heading count drift in $doc" }
    $section = [regex]::Match($text,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## CP423\b)").Groups['body'].Value.TrimEnd([char[]]"`r`n")
    if ($null -eq $canonical) {$canonical=$section} elseif ($section -cne $canonical) { throw "CP422 manual section drift in $doc" }
}
foreach ($pattern in @('physical executable line 2333 exactly','line\s+2334.*?first excluded.*?CP423','exact two dependency-ordered sites','T422=59','I422=49','F421=5','Z422=54','M422=5','S422=2\*M422=10','19/40','three\s+width-36','36/41/56','234 base fields','eighty-three\s+`Option<f64>`','two optional comparison bools','317\s+unique JSON keys','exact first 290 keys','27-key tail','CP421-to-CP422-to-unchanged-numerical','112 to 113','360 total','240 public','120 internal')) { Assert-Cp422Text -Text $canonical -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }
foreach ($spec in @([PSCustomObject]@{Path='specs\algorithm_ledger.toml';Anchor='CP422 supersedes only CP421'},[PSCustomObject]@{Path='specs\capabilities.toml';Anchor='CP422 additionally requires'})) {
    $matches=[regex]::Matches((Read-RepoText -Path $spec.Path),'(?m)^\s*"(?<body>'+[regex]::Escape($spec.Anchor)+'.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP422 expected one bounded addendum in $($spec.Path)" }
    foreach ($pattern in @('2333','2334.*?CP423','59.*?49.*?5.*?54.*?5.*?10','19/40','three width-36','234 base','eighty-three','317','290','27-key','112 to 113','360.*?240.*?120')) { Assert-Cp422Text -Text $matches[0].Groups['body'].Value -Pattern "(?is)$pattern" -Description 'bounded spec claim' }
}
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP422\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP422\b' -Description 'psychrometrics-map non-promotion'

$ledger=Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledger,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.source_file\s*=').Count -ne 293 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP422 ledger counts drift' }
$auditRoot='scripts\quality\ideal-loads-structure-audit'; $audits=@(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) { if ($file.BaseName -notmatch '^cp(?<number>\d+)-') {continue}; $number=[int]$Matches['number']; if ($number -ge 334 -and $number -le 422) {Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp424_lifecycle_evidence' -Description 'historical non-direct firewall'}; if ($number -ge 337 -and $number -le 422) {Assert-Contains -Path $file.FullName -Pattern 'script_count = 362' -Description 'historical script count'}; if ($number -ge 367 -and $number -le 422) {Assert-Contains -Path $file.FullName -Pattern 'Count -ne 122' -Description 'historical classification count'}; if ($number -ge 335 -and $number -le 422) {Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 362 \|')) -Description 'historical generated total';Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 122 \|')) -Description 'historical generated internal total'} }
$cleanup=@((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File|Where-Object {$_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344})
if ($cleanup.Count -ne 17) {throw 'CP422 helper-cleanup propagation set drift'}; foreach ($file in $cleanup) {Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist'}
$terminal=@((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File);$audits|Where-Object {$_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 421))})
if ($terminal.Count -ne 45) {throw 'CP422 terminal propagation set drift'}; foreach ($file in $terminal) {Assert-Contains -Path $file.FullName -Pattern '\$cp422Call' -Description 'CP422 terminal capture';Assert-Contains -Path $file.FullName -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval';Assert-Contains -Path $file.FullName -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 interval'}; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-numerical' -Description 'CP424 terminal-to-numerical interval'
$master=Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'; $previous=$master.IndexOf('cp421-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-guard.ps1');$current=$master.IndexOf((Split-Path -Leaf $audit));$completion=$master.IndexOf('Write-Host "IdealLoads structure audit complete."');if ($previous -lt 0 -or $current -le $previous -or $completion -le $current -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) {throw 'Master CP421-to-CP422 registration order drift'}
if ((Get-Content -LiteralPath 'scripts\quality\ideal-loads-structure-audit.ps1').Count -gt 4200) {throw 'IdealLoads master audit line cap exceeded after CP422 registration'}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 99' -Description 'CP422 calc-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$idealLoadsInitWitnesses -Limit 272' -Description 'CP422 witness-root structural cap'
if ((Get-Content -LiteralPath 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1').Count -gt 1200) {throw 'CP345 line cap exceeded after CP422 terminal propagation'}
$inventory=Read-RepoText -Path 'specs\script_inventory.toml';foreach ($pattern in @('script_count = 362','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) {Assert-Cp422Text -Text $inventory -Pattern $pattern -Description 'inventory count'};if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 122) {throw 'CP422 inventory classification drift'}
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp422-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-maximum-capacity-assignment\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP422 supersedes only CP421' -Description 'generated algorithm addendum';Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP422 additionally requires' -Description 'generated capability addendum';Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 362 \|' -Description 'generated script total';Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 122 \|' -Description 'generated internal total'
Write-Host 'CP422 post-saturation sensible-output maximum-capacity assignment structure audit passed.'
}
