# CP406 maps PurchasedAirManager.cc physical control line 2301 only and
# stops before line 2302's first executable statement.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntry"
$pipelineStem = "purchased_air_$stem"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$routes = "$root\transition\routes.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation\snapshot.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp406.rs"
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$pipelineValidationTests = "crates\ep_run\src\pipeline\$pipelineStem\validation\tests.rs"
$pipelineSerialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitraryPredecessor = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp405_assertions.rs"
$arbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp406_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp406-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-capacity-guard-else-branch-entry.ps1"
$site = "enter-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-guard-else-branch-after-guard-false-fallthrough"

function Assert-Cp406Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP406 $Description missing '$Pattern'" }
}

function Get-Cp406BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP406 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP406 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1)
            }
        }
    }
    throw "CP406 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $accounting, $routes, $tests, $release,
    $privateCharacterization, $runtimeValidation, $snapshotValidation, $adapter,
    $adapterTests, $coupled, $coupledSnapshot, $coupledTests, $coupledFixture,
    $witness, $pipeline, $pipelineValidation, $pipelineLineage,
    $pipelineValidationTests, $pipelineSerialization, $snapshotJson,
    $snapshotJsonTests, $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP406 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP406 bounded file"
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP406 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2300].Trim() -cne '} else {' -or
    $lines[2301].Trim() -cne 'PurchAir.SupplyTemp = PsyTdbFnHW(SupplyEnthalpy, PurchAir.SupplyHumRat);') {
    throw "CP406 source and first-exclusion boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2301' -Description "mapped control boundary"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2302' -Description "first excluded executable"
Assert-ExactStringArray -Path $module -Name "PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER" -Expected @($site) -Description "sole source site"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp406BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "compressed snapshot"
[string[]]$fields = @(
    [regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:') |
        ForEach-Object { $_.Groups['field'].Value }
)
[string[]]$numericFields = @(
    [regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') |
        ForEach-Object { $_.Groups['field'].Value }
)
if ($fields.Count -ne 46) { throw "CP406 compressed snapshot must expose exactly 46 fields" }
$expectedSuffix = @(
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed',
    'predecessor_cp405_resulting_supply_humidity_ratio',
    'predecessor_cp405_resulting_supply_enthalpy_j_per_kg',
    'predecessor_cp405_resulting_supply_temperature_c',
    'dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered',
    'resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c'
)
for ($index = 0; $index -lt $expectedSuffix.Count; $index += 1) {
    if ($fields[37 + $index] -cne $expectedSuffix[$index]) {
        throw "CP406 terminal field order drift at $index"
    }
}
$expectedNumeric = @(
    'predecessor_cp405_resulting_supply_humidity_ratio',
    'predecessor_cp405_resulting_supply_enthalpy_j_per_kg',
    'predecessor_cp405_resulting_supply_temperature_c',
    'resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c'
)
if ($numericFields.Count -ne 6) { throw "CP406 snapshot must expose exactly six Option<f64> carriers" }
for ($index = 0; $index -lt $expectedNumeric.Count; $index += 1) {
    if ($numericFields[$index] -cne $expectedNumeric[$index]) {
        throw "CP406 numeric field order drift at $index"
    }
}
foreach ($forbidden in @(
        'preexisting_cooling_latent_output_w',
        'assigned_cooling_latent_output_w',
        'resulting_cooling_latent_output_w',
        'maximum_total_cooling_capacity_(?:read|w)',
        'predecessor_cp40[0-4]_',
        'latent_output_capacity_guard_(?:evaluated|body_entered)'
    )) {
    if ($snapshotStruct -match $forbidden) {
        throw "CP406 compressed snapshot unexpectedly republishes '$forbidden'"
    }
}

Assert-PatternsInOrder -Path $state -Patterns @(
    'pub\s+transition_count\s*:\s*usize',
    'pub\s+inactive_transition_count\s*:\s*usize',
    'pub\s+predecessor_guard_false_fallthrough_count\s*:\s*usize',
    'pub\s+predecessor_maximum_capacity_assignment_count\s*:\s*usize',
    'pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count\s*:\s*usize',
    'pub\s+predecessor_route_counts\s*:\s*\[usize;\s*30\]',
    'pub\s+predecessor_guard_false_fallthrough_route_counts\s*:\s*\[usize;\s*30\]',
    'pub\s+predecessor_maximum_capacity_assignment_route_counts\s*:\s*\[usize;\s*30\]',
    'pub\s+else_branch_entry_route_counts\s*:\s*\[usize;\s*30\]',
    'pub\s+source_site_execution_count\s*:\s*usize',
    'pub\s+latest\s*:', 'latest_route\s*:', 'latest_transition_ordinal\s*:'
) -Description "persistent compressed accounting schema"

$core = (@($transition, $accounting, $routes, $runtimeValidation, $snapshotValidation) |
    ForEach-Object { Read-RepoText -Path $_ }) -join [Environment]::NewLine
foreach ($pattern in @(
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'route\.predecessor_index\s*\+\s*extra\s*\+\s*if\s+route\.assignment_executed\s*\{\s*1\s*\}\s*else\s*\{\s*0\s*\}',
        'let\s+else_entered\s*=\s*route\.guard_evaluated\s*&&\s*!route\.assignment_executed',
        'else\s+if\s+guard_false\s*\|\|\s*assignment\s*\|\|\s*else_entered',
        'state\.else_branch_entry_route_counts\s*==\s*state\.predecessor_guard_false_fallthrough_route_counts',
        'inactive_transition_count\s*\.checked_add',
        'checked_mul\([^)]*SOURCE_ORDER\.len\(\)',
        'option_bits_match\(predecessor,\s*resulting\)',
        'left\.to_bits\(\)\s*==\s*right\.to_bits\(\)'
    )) {
    Assert-Cp406Text -Text $core -Pattern $pattern -Description "route, algebra, inactive-route, or bit-exact contract"
}
foreach ($pair in @(
        @('predecessor_cp405_resulting_supply_humidity_ratio', 'resulting_supply_humidity_ratio'),
        @('predecessor_cp405_resulting_supply_enthalpy_j_per_kg', 'resulting_supply_enthalpy_j_per_kg'),
        @('predecessor_cp405_resulting_supply_temperature_c', 'resulting_supply_temperature_c')
    )) {
    Assert-Contains -Path $transition -Pattern "$($pair[1])\s*:\s*$($pair[0])" -Description "bit-preserving $($pair[1]) copy"
}
$elseIndices = @(20, 22, 26, 28, 31, 34)
$assignmentIndices = @(21, 23, 27, 29, 32, 35)
if ($elseIndices.Count -ne 6 -or $assignmentIndices.Count -ne 6 -or
    (36 - $elseIndices.Count) -ne 30 -or (13 + 23) -ne 36 -or
    (@($elseIndices | Where-Object { $_ -in @(20, 26) }).Count) -ne 2) {
    throw "CP406 logical route constants drift"
}
$testsText = Read-RepoText -Path $tests
foreach ($pattern in @(
        'routes\.len\(\),\s*36',
        '\[20,\s*22,\s*26,\s*28,\s*31,\s*34\]',
        '\[20,\s*26\]',
        '\.count\(\),\s*13',
        'inactive_transition_count,\s*30',
        'else_branch_entry_count,\s*6',
        'source_site_execution_count,\s*6'
    )) {
    Assert-Cp406Text -Text $testsText -Pattern $pattern -Description "exhaustive 36/30/6/6 route characterization"
}

Assert-Contains -Path $transition -Pattern 'LatentOutputMaximumCapacityAssignmentSnapshot\s+as\s+Predecessor' -Description "exact CP405 predecessor"
Assert-Contains -Path $release -Pattern 'completed_direct_.*latent_output_maximum_capacity_assignment_is_consistent' -Description "recursive CP405 completion"
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp405\s*:\s*Option<&PredecessorLifecycle>' -Description "pipeline sole predecessor"
Assert-Contains -Path $privateCharacterization -Pattern ("private_{0}_characterization" -f $stem) -Description "private route characterization"
foreach ($path in @($transition, $accounting, $routes, $release, $adapter, $coupled, $pipelineValidation, $pipelineLineage)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}
foreach ($forbidden in @(
        'PsyTdbFnHW', 'PsyWFnTdbH', 'CoolLatOutput', 'MaxCoolTotCap',
        'maximum_total_cooling_capacity', 'cooling_latent_output_[a-z_]*w',
        'DirectZonePurchasedAirCouplingInput', '\.min\s*\(', '\.max\s*\(',
        'clamp\s*\(', 'mul_add\s*\(', 'is_finite\s*\('
    )) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "no-read/arithmetic/psychrometric/numerical-feed transition"
}

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*=', 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*=',
    'let\s+unit_available\s*=', 'let\s+coupling\s*='
) -Description "CP405-to-CP406-to-CP407-to-CP408-to-CP414-to-CP415 binding order"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*:', 'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*:',
    'pub\s+coupling\s*:'
) -Description "CP405-to-CP406-to-CP407-to-CP408 scheduled output order"
$bindingText = Read-RepoText -Path $binding
$predecessorEvidence = "calculation_$predecessorStem"
$bindingEvidence = "calculation_$stem"
if ([regex]::Matches($bindingText, "\b$predecessorEvidence\b").Count -ne 3 -or
    [regex]::Matches($bindingText, "\b$bindingEvidence\b").Count -ne 3) {
    throw "CP406 binding must consume CP405 once, publish CP406 once, feed CP407 once, retain CP406 once, and keep all evidence outside numerical coupling"
}
$dto = Get-Cp406BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description "numerical coupling DTO"
if ($dto -match '(?i)cp406|capacity_guard_else_branch_entry') {
    throw "CP406 evidence must not feed DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $adapterTests -Pattern 'cp406' -Description "binding CP406 regression"
Assert-Contains -Path $adapterTests -Pattern 'unchanged_numerical' -Description "binding numerical nonfeed regression"
Assert-Contains -Path $coupledTests -Pattern 'cp406' -Description "coupled CP406 regression"
Assert-Contains -Path $coupledFixture -Pattern $bindingEvidence -Description "coupled output fixture"
Assert-Contains -Path $witness -Pattern ("set_{0}_latest_witness" -f $stem) -Description "runtime witness setter"

Assert-PatternsInOrder -Path $pipelineSerialization -Patterns @(
    '"transition_count"\s*:', '"inactive_transition_count"\s*:',
    '"predecessor_guard_false_fallthrough_count"\s*:',
    '"predecessor_maximum_capacity_assignment_count"\s*:',
    '"dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_count"\s*:',
    '"predecessor_route_counts"\s*:', '"predecessor_guard_false_fallthrough_route_counts"\s*:',
    '"predecessor_maximum_capacity_assignment_route_counts"\s*:',
    '"else_branch_entry_route_counts"\s*:', '"source_site_execution_count"\s*:', '"latest"\s*:'
) -Description "serialized public lifecycle state"
$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 6 -or $ieeeSidecars.Count -ne 6) {
    throw "CP406 JSON snapshot must expose exactly six numeric/IEEE pairs"
}
for ($index = 0; $index -lt $expectedNumeric.Count; $index += 1) {
    $field = $expectedNumeric[$index]
    $escaped = [regex]::Escape($field)
    $adjacent = '(?m)^\s*"' + $escaped + '"\s*:\s*json_number\s*\(\s*snapshot\.' + $escaped + '\s*\)\s*,\s*\r?\n\s*"' + $escaped + '_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.' + $escaped + '\s*\)'
    if ($jsonNumbers[$index].Groups['field'].Value -cne $field -or
        $jsonNumbers[$index].Groups['value'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['field'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['value'].Value -cne $field -or
        $serializationText -notmatch $adjacent) {
        throw "CP406 JSON numeric/IEEE sidecar order drift at $field"
    }
}
Assert-Contains -Path $snapshotJsonTests -Pattern 'object\.len\(\),\s*52' -Description "46-field plus six-sidecar JSON shape"
Assert-Contains -Path $snapshotJsonTests -Pattern 'ends_with\("_ieee_bits"\)[\s\S]*?\.count\(\)' -Description "IEEE sidecar count regression"
Assert-Contains -Path $snapshotJsonTests -Pattern '\.count\(\),\s*6' -Description "exact six IEEE sidecars"

Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle"
) -Description "pipeline CP405-to-CP406 validation order"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp439_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineLineage -Pattern 'else_branch_entered\s*==\s*guard_false' -Description "guard-false else-entry lineage"
Assert-Contains -Path $pipelineLineage -Pattern '!\(guard_false\s*&&\s*maximum_assignment\)' -Description "CP405/CP406 mutual exclusion"
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp406_assertions' -Description "CP405-to-CP406 arbitrary delegation module"
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp406_assertions::assert_direct\(runtime,\s*results\)' -Description "CP406 direct arbitrary delegation"
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp406_assertions::assert_non_direct\(runtime\)' -Description "CP406 non-direct arbitrary delegation"
foreach ($pattern in @(
        'PurchasedAirManager\.cc:2301', 'PurchasedAirManager\.cc:2302',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'non-direct runtime must not publish CP406 evidence',
        'ends_with\("_ieee_bits"\)'
    )) {
    Assert-Contains -Path $arbitraryAssertions -Pattern $pattern -Description "arbitrary runtime contract"
}

$heading = 'CP406 post-saturation shared-case latent-output capacity-guard else-branch entry'
$docs = @(
    'docs\src\current\current-status.md', 'docs\src\current\project-contract.md',
    'docs\src\porting-map\heat-balance-source-map.md',
    'docs\src\porting-map\ideal-loads-source-map.md',
    'docs\src\porting-map\zone-air-update-map.md'
)
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    $headingPattern = "(?m)^## $([regex]::Escape($heading))$"
    if ([regex]::Matches($docText, $headingPattern).Count -ne 1) {
        throw "CP406 documentation heading must appear exactly once in $doc"
    }
    $sectionMatch = [regex]::Match($docText, "(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)")
    if (-not $sectionMatch.Success) { throw "CP406 documentation section missing in $doc" }
    $section = $sectionMatch.Groups['body'].Value
    foreach ($pattern in @(
            'line[- ]2301.*?\} else \{',
            'line 2302.*?first excluded executable.*?CP407 candidate',
            '20,\s*22,\s*26,\s*28,\s*31,\s*and 34',
            '21,\s*23,\s*27,\s*29,\s*32,\s*and 35',
            'T406=T405', 'I406=I402\+A405', '36/30/6/6',
            'exactly 46 fields', 'Exactly six\s+.*?Option<f64>', '52 unique keys',
            'routine\.psy_tdb_fn_h_w.*?state_mapped',
            '344 total,\s*240 public,\s*(?:and\s+)?104\s+internal'
        )) {
        Assert-Cp406Text -Text $section -Pattern "(?is)$pattern" -Description "bounded documentation claim"
    }
}
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP406 supersedes only CP405' -Description "algorithm claim"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP406 additionally requires' -Description "capability claim"
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP406 supersedes only CP405' -Description "generated algorithm claim"
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP406 additionally requires' -Description "generated capability claim"
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern 'CP406' -Description "Roadmap non-promotion"
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP406\b' -Description "psychrometrics-map non-promotion"

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 405) {
        Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp439_lifecycle_evidence' -Description "historical non-direct firewall"
    }
    if ($number -ge 337 -and $number -le 405) {
        Assert-Contains -Path $file.FullName -Pattern 'script_count = 377' -Description "historical current script count"
    }
    if ($number -ge 335 -and $number -le 405) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 377 \|')) -Description "historical generated script count"
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 137 \|')) -Description "historical generated internal count"
    }
    if ($number -ge 367 -and $number -le 405) {
        Assert-Contains -Path $file.FullName -Pattern 'Count -ne 137' -Description "historical internal classification count"
        if ($number -le 398) { Assert-Contains -Path $file.FullName -Pattern '240 public and 136 internal' -Description "historical public/internal classification phrase" } else { Assert-Contains -Path $file.FullName -Pattern '240 public and 137 internal' -Description "historical public/internal classification phrase" }
    }
}
$cleanupAudits = @(
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp326-*.ps1' -File
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp3*.ps1' -File | Where-Object {
        $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344
    }
)
if ($cleanupAudits.Count -ne 17) { throw "CP406 cleanup propagation file set drift" }
foreach ($file in $cleanupAudits) {
    Assert-Contains -Path $file.FullName -Pattern 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry' -Description "historical CP406 helper whitelist"
}
$terminalAudits = @(
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp345-*.ps1' -File
    $audits | Where-Object {
        $_.BaseName -match '^cp(?<number>\d+)-' -and
        (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or
         ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 405))
    }
)
if ($terminalAudits.Count -ne 29) { throw "CP406 terminal-order propagation file set drift" }
foreach ($file in $terminalAudits) {
    Assert-Contains -Path $file.FullName -Pattern 'CP405-to-CP406' -Description "historical CP405-to-CP406 interval"
}
$cp345Audit = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp406Call\s*=', '\$cp409Call\s*=', 'CP405-to-CP406', 'CP408-to-CP409', 'CP414-to-CP415')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "CP345 terminal chain"
}
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405)-' })) {
    Assert-Contains -Path $file.FullName -Pattern 'latent_output_capacity_guard_else_branch_entry\\s\*' -Description "recent binding/scheduled CP406 order"
}
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405)-' })) {
    Assert-Contains -Path $file.FullName -Pattern '\$cp406Call' -Description "recent CP406 terminal capture assertion"
}

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$cp405Index = $master.IndexOf('cp405-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-maximum-capacity-assignment.ps1')
$cp406Index = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp405Index -lt 0 -or $cp406Index -le $cp405Index -or $completionIndex -le $cp406Index) {
    throw "Master CP406 registration order drift"
}
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 377', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp406Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 137) {
    throw "CP406 inventory classification drift"
}
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp406-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-capacity-guard-else-branch-entry\.ps1' -Description "inventory record"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 377 \|' -Description "generated script total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description "generated public total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 137 \|' -Description "generated internal total"

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP406-to-CP407' -Description "CP345 CP406-to-CP407 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP407-to-CP408' -Description "CP345 CP407-to-CP408 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP408-to-CP409' -Description "CP345 CP408-to-CP409 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp407Call\s*=' -Description "CP345 CP407 call capture"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp408Call\s*=' -Description "CP345 CP408 call capture"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp409Call\s*=' -Description "CP345 CP409 call capture"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP410-to-CP411' -Description "CP345 CP410-to-CP411 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp410Call\s*=' -Description "CP345 CP410 call capture"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp411Call\s*=' -Description "CP345 CP411 call capture"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\s*=' -Description "CP410 historical binding order"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\s*=' -Description "CP411 historical binding order"
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP406 post-saturation shared-case latent-output capacity-guard else-branch entry structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp437Call' -Description 'CP437 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-CP437' -Description 'CP436-to-CP437 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP436-to-' + 'numerical') -Description 'stale CP436 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp438Call' -Description 'CP438 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-CP438' -Description 'CP437-to-CP438 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP437-to-' + 'numerical') -Description 'stale CP437 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp439Call' -Description 'CP439 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP438-to-CP439' -Description 'CP438-to-CP439 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP438-to-' + 'numerical') -Description 'stale CP438 numerical interval'
