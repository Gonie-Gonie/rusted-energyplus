# CP394 maps PurchasedAirManager.cc physical control line 2286 and stops before 2288.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntry"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$commit = "6f2e40d10250a105b49966baa24d843711e61048"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$routes = "$root\transition\routes.rs"
$accounting = "$root\transition\accounting.rs"
$release = "$root\release.rs"
$prefix = "$root\release\prefix_validation.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$tests = "$root\tests.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$bindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupledRootFile = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledDir = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp394.rs"
$coupledOutputRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineDir = "crates\ep_run\src\pipeline\$pipelineStem"
$pipelineValidation = "$pipelineDir\validation.rs"
$pipelineValidationTests = "$pipelineDir\validation\tests.rs"
$pipelineSerialization = "$pipelineDir\serialization.rs"
$snapshotJson = "$pipelineDir\serialization\snapshot.rs"
$snapshotJsonTests = "$pipelineDir\serialization\snapshot\tests.rs"
$cp393Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp393_assertions.rs"
$cp394Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp394_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp394-cooling-post-saturation-capacity-limit-dehumidification-control-humidistat-case-entry.ps1"
$site = "enter-purchased-air-post-saturation-capacity-limit-dehumidification-control-humidistat-case"
$numericFields = @(
    "predecessor_cp393_resulting_supply_humidity_ratio",
    "predecessor_cp393_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp393_resulting_supply_temperature_c",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)
$localBools = @(
    "predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break",
    "dehumidification_control_humidistat_case_entered"
)
$stateFields = @(
    "system", "transition_count", "inactive_transition_count",
    "dehumidification_control_humidistat_case_entry_count",
    "predecessor_route_counts", "source_site_execution_count", "latest"
)

function Assert-Cp394Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP394 $Description missing '$Pattern'" }
}

function Get-Cp394BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP394 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP394 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP394 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, $privateCharacterization, $tests,
    $adapter, $bindingTests, $coupled, $coupledTests, $fixture, $witness,
    $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineSerialization,
    $snapshotJson, $snapshotJsonTests, $cp393Assertions, $cp394Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP394 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP394 bounded file"
}
foreach ($directory in @($root, $coupledDir, $pipelineDir)) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP394 bounded recursive file"
    }
}
Assert-LineLimit -Path "crates\ep_run\tests\arbitrary_run_ideal_loads.rs" -Limit 1200 -Description "CP394 arbitrary-run root cap"

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) { throw "CP394 PurchasedAirManager.cc SHA-256 drift" }
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2285].Trim() -cne 'case HumControl::Humidistat: {' -or
    $lines[2286].Trim() -cne '// Keep supply temp and adjust humidity ratio to reduce load' -or
    $lines[2287].Trim() -cne 'PurchAir.SupplyHumRat = PsyWFnTdbH(state, PurchAir.SupplyTemp, SupplyEnthalpy, RoutineName);' -or
    $lines[2288].Trim() -cne '} break;' -or
    $lines[2312].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP394 source slice, exclusion, or dynamic continuation drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2286' -Description "mapped label"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2288' -Description "first excluded executable"
Assert-Contains -Path $module -Pattern 'line 2288.*CP395' -Description "CP395 boundary note"
Assert-Contains -Path $module -Pattern 'line 2313' -Description "dynamic continuation note"
Assert-ExactStringArray -Path $module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER' -Expected @($site) -Description "sole Humidistat entry site"

$snapshotStruct = Get-Cp394BraceBlock -Text (Read-RepoText -Path $module) -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 6) { throw "CP394 snapshot must expose exactly six Option<f64> fields" }
for ($index = 0; $index -lt 6; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP394 numeric field order drift at $index" }
}
$localStart = $snapshotStruct.IndexOf("pub $($localBools[0]):")
if ($localStart -lt 0) { throw "CP394 local field boundary missing" }
[string[]]$actualLocalBools = @([regex]::Matches($snapshotStruct.Substring($localStart), 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*bool') | ForEach-Object { $_.Groups['field'].Value })
if ($actualLocalBools.Count -ne 2 -or $actualLocalBools[0] -cne $localBools[0] -or $actualLocalBools[1] -cne $localBools[1]) { throw "CP394 local boolean subsequence drift" }
Assert-PatternsInOrder -Path $module -Patterns @(
    "pub\s+$($localBools[0])\s*:\s*bool",
    'pub\s+predecessor_cp393_resulting_supply_humidity_ratio\s*:',
    'pub\s+predecessor_cp393_resulting_supply_enthalpy_j_per_kg\s*:',
    'pub\s+predecessor_cp393_resulting_supply_temperature_c\s*:',
    "pub\s+$($localBools[1])\s*:\s*bool",
    'pub\s+resulting_supply_humidity_ratio\s*:',
    'pub\s+resulting_supply_enthalpy_j_per_kg\s*:',
    'pub\s+resulting_supply_temperature_c\s*:'
) -Description "exact snapshot interleaving"
foreach ($forbidden in @('predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed', 'predecessor_cp392_', 'psychrometric_supply_humidity_ratio', 'supply_temperature_for_humidity_ratio_inversion_read')) {
    if ($snapshotStruct.Contains($forbidden)) { throw "CP393 terminal operation leaked into CP394 snapshot: $forbidden" }
}

$stateStruct = Get-Cp394BraceBlock -Text (Read-RepoText -Path $state) -AnchorPattern "pub\s+struct\s+$($typeStem)RuntimeState\s*" -Description "runtime state"
[string[]]$actualStateFields = @([regex]::Matches($stateStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:') | ForEach-Object { $_.Groups['field'].Value })
if ($actualStateFields.Count -ne $stateFields.Count) { throw "CP394 public state field count drift" }
for ($index = 0; $index -lt $stateFields.Count; $index += 1) {
    if ($actualStateFields[$index] -cne $stateFields[$index]) { throw "CP394 public state field order drift at $index" }
}
Assert-Contains -Path $state -Pattern 'predecessor_route_counts\s*:\s*\[usize;\s*30\]' -Description "thirty routes"
Assert-Contains -Path $state -Pattern 'pub\(super\)\s+latest_route' -Description "private latest route"
Assert-Contains -Path $state -Pattern 'pub\(super\)\s+latest_transition_ordinal' -Description "private ordinal"
Assert-NotContains -Path $state -Pattern '(?i)(?:humidity|enthalpy|temperature).*?(?:owner|read|evaluation|assignment|write)_count' -Description "numeric counters"

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        [regex]::Escape("$($predecessorStem)_snapshot_route"),
        'matches!\(index,\s*19\s*\|\s*23\s*\|\s*26\)',
        'matches!\(route\.predecessor_index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)\s*&&\s*!route\.active',
        'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)',
        'index\s*>=\s*3',
        'checked_selected_sum\(&state\.predecessor_route_counts,\s*&\[19,\s*23,\s*26\]\)',
        'predecessor_total\s*!=\s*Some\(state\.transition_count\)',
        'inactive_transition_count\.checked_add\(entries\)\s*!=\s*Some\(state\.transition_count\)',
        'active_total\s*!=\s*Some\(entries\)',
        'expected_sites\s*!=\s*Some\(state\.source_site_execution_count\)',
        'state\.predecessor_route_counts\s*==\s*predecessor\.predecessor_route_counts'
    )) { Assert-Cp394Text -Text $core -Pattern $pattern -Description "route/algebra contract" }
Assert-Contains -Path $accounting -Pattern '(?s)if\s+!route\.active.*?inactive_transition_count\s*\+=\s*1;.*?return;.*?case_entry_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=' -Description "inactive/active accounting"
Assert-Contains -Path $transition -Pattern 'dehumidification_control_humidistat_case_entered:\s*route\.active' -Description "route-owned entry"
Assert-Contains -Path $transition -Pattern 'case_exited_via_break:\s*predecessor\.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break' -Description "predecessor break witness"
foreach ($pair in @(
        @('predecessor_cp393_resulting_supply_humidity_ratio', 'resulting_supply_humidity_ratio'),
        @('predecessor_cp393_resulting_supply_enthalpy_j_per_kg', 'resulting_supply_enthalpy_j_per_kg'),
        @('predecessor_cp393_resulting_supply_temperature_c', 'resulting_supply_temperature_c')
    )) { Assert-Contains -Path $transition -Pattern ("$($pair[1]):\s*$($pair[0])") -Description "bit-carrier preservation" }
Assert-NotContains -Path $transition -Pattern 'ActiveInput|DirectZonePurchasedAirCouplingInput|energyplus_psy|PsyWFn|PsyHFn|is_finite|is_nan|\.clamp\s*\(|\.min\s*\(|\.max\s*\(|mul_add|total_cmp|partial_cmp' -Description "selector and numerical firewall"

Assert-Contains -Path $release -Pattern 'ConstantSensibleHeatRatioCaseBreakSnapshot as Predecessor' -Description "exact CP393 predecessor"
Assert-Contains -Path $release -Pattern 'predecessor_cp393:\s*Predecessor' -Description "CP393-only argument"
Assert-Contains -Path $release -Pattern 'constant_sensible_heat_ratio_case_break_snapshot_is_exact_direct_release\(predecessor_cp393\)' -Description "exact CP393 direct-release evidence"
Assert-Contains -Path $prefix -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_committed_latest_snapshot_is_consistent\s*\(' -Description "bounded CP393 committed predecessor proof"
Assert-NotContains -Path $prefix -Pattern 'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_is_consistent\s*\(' -Description "recursive CP393 predecessor completion"
Assert-NotContains -Path $release -Pattern 'predecessor_cp392|SupplyHumidityRatioAssignmentSnapshot as Predecessor|ActiveInput|latest_numerical' -Description "older/scalar predecessor substitution"
$productionFiles = @($transition, $accounting, $release, $prefix, $runtimeValidation, $snapshotValidation, $privateCharacterization, $adapter, $coupled, $pipelineValidation)
foreach ($path in $productionFiles) {
    Assert-NotContains -Path $path -Pattern 'PsyWFnTdbH|energyplus_psy_w_fn_tdb_h|supply_temperature_for_humidity_ratio_inversion_read|supply_enthalpy_for_humidity_ratio_inversion_read|psychrometric_supply_humidity_ratio|assigned_supply_humidity_ratio|humidity_ratio_assignment_performed|line[-_ ]?2289' -Description "CP395 behavior firewall"
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern 'DirectZonePurchasedAirCouplingInput|\.prediction\b|\.feedback\b|ResultStore|reconcil|latest_numerical|supply_node_update|numerical_dto|\breport\b|\bloads?\b' -Description "numerical DTO/result firewall"
}

$testText = ($coreFiles | Where-Object { $_.FullName -match '[\\/]tests(?:\.rs|[\\/])' } | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'cp394_boundary_and_single_entry_site_are_exact',
        'cp394_preserves_thirty_routes_and_enters_exactly_three_humidistat_routes',
        'assert_eq!\(snapshots\.len\(\),\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*27\)',
        'assert_eq!\(state\.source_site_execution_count,\s*3\)',
        'compressed_snapshot_preserves_arbitrary_carrier_bits_without_numeric_gates',
        'binary64_snapshot_comparison_distinguishes_nan_payloads',
        'malformed_or_wrong_identity_cp393_predecessor_rejects_without_mutation',
        'constant_shr_break_never_falls_through_the_humidistat_label',
        'every_active_counter_overflow_rejects_before_mutation',
        'inactive_counter_overflow_rejects_before_mutation',
        'direct_release_skips_humidistat_entry_and_retains_lifecycle_metadata'
    )) { Assert-Cp394Text -Text $testText -Pattern $pattern -Description "non-vacuous core test" }

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\s*=',
    'let\s+unit_available\s*=',
    'let\s+coupling\s*='
) -Description "CP393-to-CP396-to-numerical binding order"
$bindingText = Read-RepoText -Path $binding
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($numericalIndex -lt 0) { throw "CP394 numerical anchor missing" }
$dto = Get-Cp394BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match [regex]::Escape($stem) -or $dto -match 'predecessor_cp393_resulting_supply_|dehumidification_control_humidistat_case_entered') { throw "CP394 evidence feeds numerical DTO" }
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry:',
    'pub\s+coupling:'
) -Description "scheduled output order"
Assert-PatternsInOrder -Path $coupledRootFile -Patterns @(
    'let\s+calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_lifecycle\s*=',
    'let\s+calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_lifecycle\s*='
) -Description "coupled lifecycle order"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$($predecessorStem)::\s*validate_direct_lifecycle", "$($stem)::\s*validate_direct_lifecycle") -Description "pipeline validation order"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp439_lifecycle_evidence' -Description "non-direct firewall"
foreach ($pattern in @('public_cp394_validator_depends_only_on_cp393_and_requires_all_routes_inactive', 'ep_run_cp394_rejects_missing_cp393_predecessor_evidence', 'ep_run_cp394_links_exactly_to_cp393_and_rejects_corruption')) {
    Assert-Contains -Path $pipelineValidationTests -Pattern $pattern -Description "pipeline validation test"
}
foreach ($pattern in @('cp394_preserves_cp393_terminal_carriers_and_skips_humidistat_entry_on_direct_routes', 'cp394_rejects_cp393_carrier_bit_drift_and_route_drift', 'cp394_validation_remains_independent_of_numerical_output_state')) {
    Assert-Contains -Path $coupledTests -Pattern $pattern -Description "coupled CP394 test"
}
Assert-Contains -Path $bindingTests -Pattern 'binding_places_cp394_after_cp393_before_unchanged_numerical_coupling' -Description "binding preservation test"

$snapshotJsonText = Read-RepoText -Path $snapshotJson
$jsonCursor = 0
foreach ($field in $numericFields) {
    foreach ($key in @($field, "$($field)_ieee_bits")) {
        $needle = '"' + $key + '"'
        if ([regex]::Matches($snapshotJsonText, [regex]::Escape($needle)).Count -ne 1) { throw "CP394 JSON key '$key' count drift" }
        $next = $snapshotJsonText.IndexOf($needle, $jsonCursor)
        if ($next -lt 0) { throw "CP394 JSON carrier order drift at '$key'" }
        $jsonCursor = $next + $needle.Length
    }
}
if ([regex]::Matches($snapshotJsonText, '_ieee_bits"').Count -ne 6) { throw "CP394 JSON sidecar count drift" }
foreach ($field in $localBools) {
    if ([regex]::Matches($snapshotJsonText, '"' + [regex]::Escape($field) + '"').Count -ne 1) { throw "CP394 JSON boolean '$field' count drift" }
}
Assert-PatternsInOrder -Path $snapshotJson -Patterns @(
    ('"' + [regex]::Escape($localBools[0]) + '"'),
    '"predecessor_cp393_resulting_supply_humidity_ratio"',
    '"predecessor_cp393_resulting_supply_enthalpy_j_per_kg"',
    '"predecessor_cp393_resulting_supply_temperature_c"',
    ('"' + [regex]::Escape($localBools[1]) + '"'),
    '"resulting_supply_humidity_ratio"',
    '"resulting_supply_enthalpy_j_per_kg"',
    '"resulting_supply_temperature_c"'
) -Description "JSON schema interleaving"
Assert-Contains -Path $snapshotJson -Pattern '(?s)value\s*\.filter\(\|value\| value\.is_finite\(\)\)' -Description "finite-only JSON projection"
Assert-Contains -Path $snapshotJson -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "IEEE sidecar"
Assert-Contains -Path $snapshotJsonTests -Pattern 'six_compact_carriers_serialize_with_adjacent_ieee_sidecars' -Description "six-carrier JSON test"
Assert-Contains -Path $snapshotJsonTests -Pattern 'six_nonfinite_carriers_project_null_and_preserve_nan_payload_bits' -Description "nonfinite JSON test"

Assert-Contains -Path $cp393Assertions -Pattern '#\[path\s*=\s*"cp394_assertions\.rs"\]' -Description "arbitrary assertion chain"
foreach ($pattern in @('CP394_KEY', 'PurchasedAirManager\.cc:2286', 'PurchasedAirManager\.cc:2288', $site, 'dehumidification_control_humidistat_case_entry_count', 'source_site_execution_count', 'predecessor_cp393_resulting_supply_humidity_ratio_ieee_bits', 'resulting_supply_temperature_c_ieee_bits', 'assert_non_direct')) {
    Assert-Contains -Path $cp394Assertions -Pattern $pattern -Description "arbitrary CP394 assertion"
}
Assert-Contains -Path $coupledOutputRoot -Pattern ([regex]::Escape("$($stem)_fixture.rs")) -Description "fixture wiring"
Assert-Contains -Path $fixture -Pattern 'predecessor_cp393_resulting_supply_humidity_ratio' -Description "compressed fixture"

$algorithmClaims = [regex]::Matches((Read-RepoText -Path "specs\algorithm_ledger.toml"), '(?m)^\s*"CP394 supersedes only CP393[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches((Read-RepoText -Path "specs\capabilities.toml"), '(?m)^\s*"CP394 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) { throw "CP394 must add exactly two algorithm and two capability claims" }
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, 'physical line 2286', 'case HumControl::Humidistat', $site,
            'line 2287', 'line 2288', 'CP395', 'line-2289', 'line 2313', 'thirty', '19', '23', '26',
            'twenty-seven', 'eleven', 'T394=T393', 'R\[19\]\+R\[23\]\+R\[26\]',
            'sole immediate predecessor', 'exactly six', 'six authoritative IEEE sidecars',
            'CP393-to-CP394-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput',
            '32 algorithms', '293 routines', '58 state-mapped', '235 source-mapped', '170 required',
            '332 total', '240 public', '92 internal', 'zero unused', 'zero unreachable', '238 development commands', 'Roadmap'
        )) { if ($claim.Value -notmatch $pattern) { throw "CP394 spec addendum missing '$pattern'" } }
}

$docs = @(
    "docs\src\current\current-status.md", "docs\src\current\project-contract.md",
    "docs\src\porting-map\ideal-loads-source-map.md", "docs\src\porting-map\heat-balance-source-map.md",
    "docs\src\porting-map\zone-air-update-map.md"
)
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    $sections = [regex]::Matches($text, '(?ms)^## CP394\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP394 documentation count drift in $doc" }
    foreach ($pattern in @($commit, $hash, '2286', $site, '2288', 'CP395', '2289', '2313', 'T394\s*=\s*T393', 'R\[19\]\+R\[23\]\+R\[26\]', 'exactly six', 'IEEE sidecars', 'boolean subsequence', 'CP393-to-CP394-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput', '332\s+total', '240\s+public', '92\s+internal', '238\s+development\s+commands', 'Roadmap')) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP394 documentation in $doc missing '$pattern'" }
    }
    if ($text.LastIndexOf("## CP394 ") -le $text.LastIndexOf("## CP393 ")) { throw "CP393-to-CP394 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP394\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP394 supersedes only CP393' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP394 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..393) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp439_lifecycle_evidence' -Description "historical non-direct token"
}
foreach ($historical in 335..393) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 377 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 137 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..393) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 377' -Description "historical inventory total"
}
foreach ($historical in 367..393) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 137' -Description "historical internal count"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 136 internal' -Description "historical classification"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP394 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 structural cap"
foreach ($pattern in @('\$cp394Call\s*=', 'CP393-to-CP394', 'CP396-to-CP397', 'CP397-to-CP398', 'CP400-to-CP401', 'CP401-to-CP402', 'CP402-to-CP403', 'CP403-to-CP404')) { Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "CP345 terminal chain" }

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp393AuditIndex = $master.IndexOf("cp393-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-break.ps1")
$cp394AuditIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp393AuditIndex -lt 0 -or $cp394AuditIndex -le $cp393AuditIndex -or $completionIndex -le $cp394AuditIndex) { throw "Master CP394 registration order drift" }
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 377', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) { Assert-Cp394Text -Text $inventory -Pattern $pattern -Description "inventory" }
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 137) { throw "CP394 inventory classification drift" }
$auditLeaf = Split-Path -Leaf $audit
Assert-Cp394Text -Text $inventory -Pattern ('(?s)path = "scripts/quality/ideal-loads-structure-audit/' + [regex]::Escape($auditLeaf) + '".*?callers = \["scripts/quality/ideal-loads-structure-audit\.ps1"\]') -Description "inventory record/caller"
foreach ($pattern in @('\| 377 \|', '\| public scripts \| 240 \|', '\| 137 \|', '\| scripts without callers \| 0 \|')) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory"
}
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern ([regex]::Escape("scripts/quality/ideal-loads-structure-audit/$auditLeaf")) -Description "generated CP394 audit record"

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP404-to-CP405' -Description "CP345 CP404-to-CP405 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP405-to-CP406' -Description "CP345 CP405-to-CP406 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP406-to-CP407' -Description "CP345 CP406-to-CP407 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP407-to-CP408' -Description "CP345 CP407-to-CP408 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP408-to-CP409' -Description "CP345 CP408-to-CP409 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP410-to-CP411' -Description "CP345 CP410-to-CP411 interval"
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP411-to-CP412' -Description 'CP345 CP411-to-CP412 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP412-to-CP413' -Description 'CP345 CP412-to-CP413 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP413-to-CP414' -Description 'CP345 CP413-to-CP414 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\s*=' -Description 'CP415 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp415Call' -Description 'CP415 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP414-to-CP415' -Description 'CP414-to-CP415 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\s*=' -Description 'CP416 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp416Call' -Description 'CP416 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP415-to-CP416' -Description 'CP415-to-CP416 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp417Call' -Description 'CP417 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP416-to-CP417' -Description 'CP416-to-CP417 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP394 post-saturation Humidistat case-entry structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-numerical' -Description 'CP439-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp437Call' -Description 'CP437 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-CP437' -Description 'CP436-to-CP437 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP436-to-' + 'numerical') -Description 'stale CP436 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp438Call' -Description 'CP438 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-CP438' -Description 'CP437-to-CP438 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP437-to-' + 'numerical') -Description 'stale CP437 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp439Call' -Description 'CP439 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP438-to-CP439' -Description 'CP438-to-CP439 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP438-to-' + 'numerical') -Description 'stale CP438 numerical interval'
