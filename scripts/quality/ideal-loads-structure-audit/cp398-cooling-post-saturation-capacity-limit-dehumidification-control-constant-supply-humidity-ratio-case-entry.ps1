# CP398 maps PurchasedAirManager.cc physical source line 2291 only.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntry"
$pipelineStem = "purchased_air_$stem"
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
$coupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp398.rs"
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
$arbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp397Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp397_assertions.rs"
$cp398Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp398_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp398-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-case-entry.ps1"
$site = "enter-purchased-air-post-saturation-capacity-limit-dehumidification-control-none-or-constant-supply-humidity-ratio-shared-case"
$numericFields = @(
    "predecessor_cp397_resulting_supply_humidity_ratio",
    "predecessor_cp397_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp397_resulting_supply_temperature_c",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)
$localBool = "dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered"
$stateFields = @(
    "system", "transition_count", "inactive_transition_count",
    "dehumidification_control_constant_supply_humidity_ratio_case_entry_count", "predecessor_route_counts",
    "source_site_execution_count", "latest"
)

function Assert-Cp398Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP398 $Description missing '$Pattern'" }
}

function Get-Cp398BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP398 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP398 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP398 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, $privateCharacterization, $tests,
    $adapter, $bindingTests, $coupled, $coupledTests, $fixture, $witness,
    $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineSerialization,
    $snapshotJson, $snapshotJsonTests, $cp397Assertions, $cp398Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP398 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP398 bounded file"
}
foreach ($directory in @($root, $coupledRoot, $pipelineDir)) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP398 bounded recursive file"
    }
}
Assert-FileExists -Path $arbitraryRoot -Description "CP398 arbitrary-run test root"
Assert-LineLimit -Path $arbitraryRoot -Limit 1200 -Description "CP398 arbitrary-run root cap"

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) { throw "CP398 PurchasedAirManager.cc SHA-256 drift" }
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2289].Trim() -cne 'case HumControl::None:' -or
    $lines[2290].Trim() -cne 'case HumControl::ConstantSupplyHumidityRatio: {' -or
    $lines[2291].Trim() -cne '// Keep humidity ratio and adjust supply temp' -or
    $lines[2292].Trim() -cne '// Check if latent output exceeds capacity' -or
    $lines[2293].Trim() -cne 'CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat);' -or
    $lines[2312].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP398 predecessor label, mapped shared label, excluded comments/body, or dynamic continuation drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2291' -Description "mapped shared case label"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2294' -Description "first excluded executable"
Assert-Contains -Path $module -Pattern 'Comment-only lines 2292-2293' -Description "excluded comments"
Assert-Contains -Path $module -Pattern 'line 2313' -Description "dynamic continuation exclusion"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER' `
    -Expected @($site) -Description "exact sole shared case-entry site"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp398BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot declaration"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 6) { throw "CP398 snapshot must expose exactly six Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP398 numeric field order drift at $index" }
}
Assert-PatternsInOrder -Path $module -Patterns @(
    'pub\s+predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered\s*:\s*bool',
    'pub\s+predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break\s*:\s*bool',
    'pub\s+predecessor_dehumidification_control_humidistat_case_entered\s*:\s*bool',
    'pub\s+predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed\s*:\s*bool',
    'pub\s+predecessor_dehumidification_control_humidistat_case_exited_via_break\s*:\s*bool',
    'pub\s+predecessor_cp397_resulting_supply_humidity_ratio\s*:',
    'pub\s+predecessor_cp397_resulting_supply_enthalpy_j_per_kg\s*:',
    'pub\s+predecessor_cp397_resulting_supply_temperature_c\s*:',
    'pub\s+predecessor_dehumidification_control_none_case_entered\s*:\s*bool',
    "pub\s+$localBool\s*:\s*bool",
    'pub\s+resulting_supply_humidity_ratio\s*:',
    'pub\s+resulting_supply_enthalpy_j_per_kg\s*:',
    'pub\s+resulting_supply_temperature_c\s*:'
) -Description "retained-control, six-carrier, and local-entry schema order"
if ([regex]::Matches($snapshotStruct, 'pub\s+predecessor_dehumidification_control_none_case_entered\s*:\s*bool').Count -ne 1 -or
    [regex]::Matches($snapshotStruct, "pub\s+$localBool\s*:\s*bool").Count -ne 1) {
    throw "CP398 must retain exactly one predecessor None marker and add exactly one local shared-case marker"
}

$stateStruct = Get-Cp398BraceBlock -Text (Read-RepoText -Path $state) -AnchorPattern "pub\s+struct\s+$($typeStem)RuntimeState\s*" -Description "runtime state declaration"
[string[]]$actualStateFields = @([regex]::Matches($stateStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:') | ForEach-Object { $_.Groups['field'].Value })
if ($actualStateFields.Count -ne $stateFields.Count) { throw "CP398 runtime state public field count drift" }
for ($index = 0; $index -lt $stateFields.Count; $index += 1) {
    if ($actualStateFields[$index] -cne $stateFields[$index]) { throw "CP398 runtime state field order drift at $index" }
}
Assert-Contains -Path $state -Pattern 'predecessor_route_counts\s*:\s*\[usize;\s*30\]' -Description "thirty route counters"
Assert-Contains -Path $state -Pattern 'pub\(super\)\s+latest_route' -Description "private latest route"
Assert-Contains -Path $state -Pattern 'pub\(super\)\s+latest_transition_ordinal' -Description "private latest ordinal"
Assert-NotContains -Path $state -Pattern '(?i)(?:humidity|enthalpy|temperature).*?(?:owner|read|evaluation|assignment|write)_count' -Description "numeric counter firewall"

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        [regex]::Escape("$($predecessorStem)_snapshot_route"),
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'matches!\(index,\s*20\s*\|\s*24\s*\|\s*27\)',
        'matches!\(index,\s*18\s*\|\s*19\s*\|\s*22\s*\|\s*23\s*\|\s*26\s*\|\s*28\)',
        'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)',
        'index\s*>=\s*3',
        'checked_selected_sum\(\s*&predecessor\.predecessor_route_counts,\s*&\[20,\s*24,\s*27\]\s*\)\s*==\s*Some\(predecessor\.dehumidification_control_none_case_entry_count\)',
        'checked_selected_sum\(\s*&predecessor\.predecessor_route_counts,\s*&\[20,\s*21,\s*24,\s*25,\s*27,\s*29\],?\s*\)\s*==\s*Some\(state\.dehumidification_control_constant_supply_humidity_ratio_case_entry_count\)',
        'inactive_transition_count\.checked_add\(entries\)\s*!=\s*Some\(state\.transition_count\)',
        'active_total\s*!=\s*Some\(entries\)',
        'expected_sites\s*!=\s*Some\(state\.source_site_execution_count\)',
        'state\.predecessor_route_counts\s*==\s*predecessor\.predecessor_route_counts',
        'matches!\(route\.predecessor_index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'route\.active\s*==\s*matches!\(route\.predecessor_index,\s*20\s*\|\s*24\)'
    )) { Assert-Cp398Text -Text $core -Pattern $pattern -Description "route/algebra contract" }
Assert-Contains -Path $accounting -Pattern '(?s)if\s+!route\.active.*?inactive_transition_count\s*\+=\s*1;.*?return;.*?dehumidification_control_constant_supply_humidity_ratio_case_entry_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=' -Description "inactive versus active accounting"
Assert-Contains -Path $transition -Pattern "$localBool\s*:\s*route\.active" -Description "route-owned shared case-entry flag"
foreach ($pair in @(
        @('predecessor_cp397_resulting_supply_humidity_ratio', 'resulting_supply_humidity_ratio'),
        @('predecessor_cp397_resulting_supply_enthalpy_j_per_kg', 'resulting_supply_enthalpy_j_per_kg'),
        @('predecessor_cp397_resulting_supply_temperature_c', 'resulting_supply_temperature_c')
    )) { Assert-Contains -Path $transition -Pattern ("$($pair[1]):\s*$($pair[0])") -Description "bit-exact carrier preservation" }
Assert-NotContains -Path $transition -Pattern 'ActiveInput|DirectZonePurchasedAirCouplingInput|energyplus_psy|PsyWFn|PsyHFn|is_finite|is_nan|f64::|\.clamp\s*\(|\.min\s*\(|\.max\s*\(|mul_add|total_cmp|partial_cmp|system\.dehumidification_control_type' -Description "numeric work, scalar input, and selector-reread firewall"
Assert-NotContains -Path $transition -Pattern '(?:supply_humidity_ratio|supply_enthalpy|supply_temperature)[^\r\n]*(?:\+|\*|/|\s-\s)' -Description "carrier arithmetic firewall"

Assert-Contains -Path $release -Pattern 'ControlNoneCaseEntrySnapshot as Predecessor' -Description "exact CP397 predecessor type"
Assert-Contains -Path $release -Pattern 'predecessor_cp397:\s*Predecessor' -Description "predecessor-only public argument"
Assert-Contains -Path $release -Pattern 'none_case_entry_snapshot_is_exact_direct_release\(predecessor_cp397\)' -Description "exact CP397 direct release"
$predecessorRegex = [regex]::Escape($predecessorStem)
foreach ($pattern in @(
        "(?s)let\s+state\s*=\s*&unit\s*\.calc_$predecessorRegex;\s*let\s+Some\(latest\)\s*=\s*state\.latest",
        "\.$($predecessorRegex)_latest_witness\(system\.id\)",
        'classify_no_oa_sensible_subset\(system\)\.is_supported\(\)',
        'system\.dehumidification_control_type\s*==\s*DehumidificationControlType::None',
        'system\.humidification_control_type\s*==\s*HumidificationControlType::None',
        'system\.id\s*==\s*predecessor\.system',
        'unit\.controlled_zone\s*==\s*Some\(predecessor\.controlled_zone\)',
        "$($predecessorRegex)_latest_metadata_is_consistent\(",
        "$($predecessorRegex)_snapshot_is_exact_direct_release\(predecessor\)",
        "(?s)$($predecessorRegex)_snapshots_match_bit_exact\(\s*latest,\s*predecessor,\s*\)",
        "(?s)$($predecessorRegex)_snapshots_match_bit_exact\(\s*witness,\s*predecessor,\s*\)"
    )) { Assert-Contains -Path $prefix -Pattern $pattern -Description "bounded exact CP397 prefix admission" }
Assert-NotContains -Path $release -Pattern 'predecessor_cp396|HumidistatCaseBreakSnapshot as Predecessor|ActiveInput|latest_numerical|supply_temperature_c:\s*f64|supply_enthalpy_j_per_kg:\s*f64' -Description "older or scalar predecessor substitution"

$productionFiles = @($transition, $accounting, $release, $prefix, $runtimeValidation, $snapshotValidation, $privateCharacterization, $adapter, $coupled, $pipelineValidation)
foreach ($path in $productionFiles) {
    Assert-NotContains -Path $path -Pattern 'PsyWFnTdbH|energyplus_psy_w_fn_tdb_h|supply_temperature_for_humidity_ratio_inversion_read|supply_enthalpy_for_humidity_ratio_inversion_read|psychrometric_supply_humidity_ratio|assigned_supply_humidity_ratio|humidity_ratio_assignment_performed' -Description "numeric-operation leak"
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern 'DirectZonePurchasedAirCouplingInput|\.coupling\b|\.prediction\b|\.feedback\b|ResultStore|reconcil|latest_numerical|supply_node_update|numerical_dto|\breport\b|\bloads?\b' -Description "numerical DTO/result/node/load/report firewall"
}

$testText = ($coreFiles | Where-Object { $_.FullName -match '[\\/]tests(?:\.rs|[\\/])' } | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'cp398_boundary_and_single_shared_case_entry_site_are_exact',
        'cp398_preserves_thirty_routes_and_enters_shared_case_exactly_six_times',
        'assert_eq!\(snapshots\.len\(\),\s*30\)',
        'assert_eq!\(state\.transition_count,\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*24\)',
        'dehumidification_control_constant_supply_humidity_ratio_case_entry_count,\s*6',
        'assert_eq!\(state\.source_site_execution_count,\s*6\)',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'predecessor_dehumidification_control_none_case_entered,\s*none',
        'compressed_snapshot_preserves_arbitrary_carrier_bits_without_numeric_gates',
        'binary64_snapshot_comparison_distinguishes_nan_payloads',
        'malformed_or_wrong_identity_cp397_predecessor_rejects_without_mutation',
        'constant_supply_selector_enters_shared_case_without_none_fallthrough',
        'every_active_counter_overflow_rejects_before_mutation',
        'inactive_counter_overflow_rejects_before_mutation',
        'direct_release_enters_shared_case_via_none_fallthrough_and_retains_metadata',
        'cp398_bounded_prefix_rejects_cp397_local_corruption_transactionally'
    )) { Assert-Cp398Text -Text $testText -Pattern $pattern -Description "non-vacuous core test" }

$cp397BindingName = "calculation_$predecessorStem"
$cp398BindingName = "calculation_$stem"
Assert-PatternsInOrder -Path $binding -Patterns @(
    "let\s+$([regex]::Escape($cp397BindingName))\s*=",
    "let\s+$([regex]::Escape($cp398BindingName))\s*=",
    'let\s+unit_available\s*=',
    'let\s+coupling\s*='
) -Description "CP397-to-CP398-to-unchanged-numerical binding order"
$bindingText = Read-RepoText -Path $binding
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($numericalIndex -lt 0) { throw "CP398 unchanged numerical coupling anchor missing" }
$dto = Get-Cp398BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match [regex]::Escape($stem) -or $dto -match '\bcp398\b' -or $dto -match $localBool) { throw "CP398 evidence unexpectedly feeds DirectZonePurchasedAirCouplingInput" }
Assert-Contains -Path $bindingTests -Pattern 'binding_places_cp398_after_cp397_before_unchanged_numerical_coupling' -Description "binding preservation test"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+$([regex]::Escape($cp397BindingName))\s*:", "pub\s+$([regex]::Escape($cp398BindingName))\s*:", 'pub\s+coupling\s*:') -Description "scheduled output order"
Assert-PatternsInOrder -Path $coupledRootFile -Patterns @("let\s+calc_$([regex]::Escape($predecessorStem))_lifecycle\s*=", "let\s+calc_$([regex]::Escape($stem))_lifecycle\s*=") -Description "coupled lifecycle order"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$($predecessorStem)::\s*validate_direct_lifecycle", "$($stem)::\s*validate_direct_lifecycle") -Description "pipeline CP397-to-CP398 validation order"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description "cumulative non-direct firewall"
foreach ($pattern in @('public_cp398_validator_depends_only_on_cp397_and_accepts_direct_active_routes', 'ep_run_cp398_rejects_missing_cp397_predecessor_evidence', 'ep_run_cp398_links_exactly_to_cp397_and_rejects_corruption')) { Assert-Contains -Path $pipelineValidationTests -Pattern $pattern -Description "pipeline validation regression" }
foreach ($pattern in @('cp398_preserves_cp397_terminal_carriers_and_tracks_direct_none_entry', 'cp398_rejects_cp397_carrier_bit_drift_and_route_drift', 'cp398_validation_remains_independent_of_numerical_output_state')) { Assert-Contains -Path $coupledTests -Pattern $pattern -Description "coupled validation regression" }

foreach ($registration in @(
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\calc.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $binding; Pattern = "mod $stem;" },
        [PSCustomObject]@{ Path = $scheduledOutput; Pattern = "pub $($cp398BindingName):" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"; Pattern = "$($stem)_tests.rs" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $coupledRootFile; Pattern = "mod $($stem)_validation;" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $pipelineRoot; Pattern = $pipelineStem }
    )) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration" }

$snapshotJsonText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 6 -or $ieeeSidecars.Count -ne 6) { throw "CP398 serialization must expose exactly six numeric projections and IEEE sidecars" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $expected = $numericFields[$index]
    if ($jsonNumbers[$index].Groups['field'].Value -cne $expected -or $jsonNumbers[$index].Groups['value'].Value -cne $expected -or
        $ieeeSidecars[$index].Groups['field'].Value -cne $expected -or $ieeeSidecars[$index].Groups['value'].Value -cne $expected) { throw "CP398 numeric/IEEE serialization order drift at '$expected'" }
}
Assert-Contains -Path $snapshotJson -Pattern ('"' + $localBool + '"\s*:\s*snapshot\.' + $localBool) -Description "local entry JSON marker"
Assert-Contains -Path $snapshotJson -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "finite-only projection"
Assert-Contains -Path $snapshotJson -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "authoritative IEEE bits"
Assert-Contains -Path $snapshotJsonTests -Pattern 'six_compact_carriers_serialize_with_adjacent_ieee_sidecars' -Description "six-carrier JSON test"
Assert-Contains -Path $snapshotJsonTests -Pattern 'six_nonfinite_carriers_project_null_and_preserve_nan_payload_bits' -Description "nonfinite JSON test"
Assert-Contains -Path $pipelineSerialization -Pattern 'lifecycle_serializes_direct_active_none_entry_and_thirty_route_slots' -Description "lifecycle JSON test"

Assert-Contains -Path $cp397Assertions -Pattern '#\[path\s*=\s*"cp398_assertions\.rs"\]' -Description "arbitrary assertion chain"
foreach ($pattern in @('CP398_KEY', 'PurchasedAirManager\.cc:2291', 'PurchasedAirManager\.cc:2294', $site, 'dehumidification_control_constant_supply_humidity_ratio_case_entry_count', 'source_site_execution_count', 'predecessor_dehumidification_control_none_case_entered', $localBool, 'predecessor_cp397_resulting_supply_', 'resulting_supply_temperature_c', '_ieee_bits', 'assert_non_direct')) { Assert-Contains -Path $cp398Assertions -Pattern $pattern -Description "arbitrary CP398 assertion" }

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP398 supersedes only CP397[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP398 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) { throw "CP398 must add exactly two algorithm and two capability claims" }
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, 'physical.*line 2291', 'case HumControl::ConstantSupplyHumidityRatio:', $site,
            'line 2290', 'case HumControl::None:', 'no body or break', 'fall through',
            'lines 2292-2293', 'line 2294', 'CP399', 'line 2313', 'thirty routes',
            '20, 24, and 27', '21, 25, and 29', 'six routes are active', 'twenty-four',
            'Public routes are exactly', 'nineteen routes remain private',
            'N398=E397=R\[20\]\+R\[24\]\+R\[27\]', 'C398=R\[21\]\+R\[25\]\+R\[29\]',
            'E398=N398\+C398', 'T398=T397', 'I398=T398-E398', '30/24/6/6',
            'sole immediate predecessor', 'exactly six `Option<f64>`', 'six authoritative IEEE sidecars',
            $numericFields[0], $numericFields[5], 'predecessor_dehumidification_control_none_case_entered',
            $localBool, 'dehumidification_control_constant_supply_humidity_ratio_case_entry_count',
            'without a selector reread',
            'CP397-to-CP398-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput',
            '32 algorithms', '293 routines', '58 state-mapped', '235 source-mapped', '170 required',
            '336 total', '240 public', '96 internal', 'zero unused', 'zero unreachable',
            '238 development commands', 'Roadmap checklist promotion'
        )) { if ($claim.Value -notmatch $pattern) { throw "CP398 spec addendum missing '$pattern'" } }
}

$docs = @(
    "docs\src\current\current-status.md", "docs\src\current\project-contract.md",
    "docs\src\porting-map\ideal-loads-source-map.md", "docs\src\porting-map\heat-balance-source-map.md",
    "docs\src\porting-map\zone-air-update-map.md"
)
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    $sections = [regex]::Matches($text, '(?ms)^## CP398\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1 -or $text.LastIndexOf("## CP398 ") -le $text.LastIndexOf("## CP397 ")) { throw "CP398 documentation count/order drift in $doc" }
    foreach ($pattern in @(
            $commit, $hash, 'physical source line 2291', 'case HumControl::ConstantSupplyHumidityRatio:',
            '2290', 'case HumControl::None:', 'fall through', '2292-2293', '2294', 'CP399', '2313', $site,
            'pure typed stacked-case-label', '20,\s+24,\s+and\s+27', '21,\s+25,\s+and\s+29',
            'six routes are active', 'twenty-four', 'public exact-direct routes remain 0 through 8, 20, and 24',
            'N398=E397=R\[20\]\+R\[24\]\+R\[27\]', 'C398=R\[21\]\+R\[25\]\+R\[29\]',
            'E398=N398\+C398', 'T398=T397', 'I398=T398-E398', '30/24/6/6',
            'sole immediate predecessor', 'exactly six', 'IEEE sidecars', $numericFields[0], $numericFields[5],
            'predecessor_dehumidification_control_none_case_entered', $localBool,
            'dehumidification_control_constant_supply_humidity_ratio_case_entry_count',
            'CP397-to-CP398-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput',
            '32 algorithms', '293 routines', '336\s+total', '240\s+public', '96\s+internal',
            '238 development', 'Roadmap checklist promotion'
        )) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP398 documentation in $doc missing '$pattern'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP398\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP398 supersedes only CP397' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP398 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..397) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp435_lifecycle_evidence' -Description "historical non-direct firewall"
}
foreach ($historical in 335..397) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 373 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 133 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..397) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 373' -Description "historical inventory total"
}
foreach ($historical in 367..397) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 133' -Description "historical internal count"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 122 internal' -Description "historical classification"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP398 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp398Call\s*=', 'CP397-to-CP398', 'CP400-to-CP401', 'CP401-to-CP402', 'CP402-to-CP403', 'CP403-to-CP404')) { Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "CP345 terminal chain" }
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 structural cap"

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp397AuditIndex = $master.IndexOf("cp397-cooling-post-saturation-capacity-limit-dehumidification-control-none-case-entry.ps1")
$cp398AuditIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp397AuditIndex -lt 0 -or $cp398AuditIndex -le $cp397AuditIndex -or $completionIndex -le $cp398AuditIndex) { throw "Master CP398 registration order drift" }
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 373', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) { Assert-Cp398Text -Text $inventory -Pattern $pattern -Description "inventory" }
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 133) { throw "CP398 inventory classification drift" }
$auditLeaf = Split-Path -Leaf $audit
Assert-Cp398Text -Text $inventory -Pattern ('(?s)path = "scripts/quality/ideal-loads-structure-audit/' + [regex]::Escape($auditLeaf) + '".*?callers = \["scripts/quality/ideal-loads-structure-audit\.ps1"\]') -Description "inventory record/caller"
foreach ($pattern in @('\| 373 \|', '\| public scripts \| 240 \|', '\| 133 \|', '\| scripts without callers \| 0 \|')) { Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory" }
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern ([regex]::Escape("scripts/quality/ideal-loads-structure-audit/$auditLeaf")) -Description "generated CP398 audit record"

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
Write-Host "CP398 post-saturation shared None/constant-supply-humidity-ratio case-entry structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-numerical' -Description 'CP435-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'
