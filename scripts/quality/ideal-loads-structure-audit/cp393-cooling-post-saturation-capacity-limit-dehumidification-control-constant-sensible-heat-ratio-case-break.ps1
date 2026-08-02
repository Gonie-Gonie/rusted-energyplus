# CP393 maps PurchasedAirManager.cc physical executable line 2285 and stops before 2288.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreak"
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
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp393.rs"
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
$arbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp392Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp392_assertions.rs"
$cp393Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp393_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp393-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-break.ps1"
$site = "exit-purchased-air-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case-via-break"
$numericFields = @(
    "predecessor_cp392_resulting_supply_humidity_ratio",
    "predecessor_cp392_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp392_resulting_supply_temperature_c",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)
$localBools = @(
    "predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed",
    "dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break"
)
$stateFields = @(
    "system", "transition_count", "inactive_transition_count",
    "dehumidification_control_constant_sensible_heat_ratio_case_break_count",
    "predecessor_route_counts", "source_site_execution_count", "latest"
)

function Assert-Cp393Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP393 $Description missing '$Pattern'" }
}

function Get-Cp393BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP393 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP393 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP393 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, $privateCharacterization, $tests,
    $adapter, $bindingTests, $coupled, $coupledTests, $fixture, $witness,
    $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineSerialization,
    $snapshotJson, $snapshotJsonTests, $cp392Assertions, $cp393Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP393 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP393 bounded file"
}
foreach ($directory in @($root, $coupledRoot, $pipelineDir)) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP393 bounded recursive file"
    }
}
Assert-FileExists -Path $arbitraryRoot -Description "CP393 arbitrary-run test root"
Assert-LineLimit -Path $arbitraryRoot -Limit 1200 -Description "CP393 arbitrary-run root cap"

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) { throw "CP393 PurchasedAirManager.cc SHA-256 drift" }
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2284].Trim() -cne '} break;' -or
    $lines[2285].Trim() -cne 'case HumControl::Humidistat: {' -or
    $lines[2287].Trim() -cne 'PurchAir.SupplyHumRat = PsyWFnTdbH(state, PurchAir.SupplyTemp, SupplyEnthalpy, RoutineName);' -or
    $lines[2312].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP393 source slice, lexical exclusion, or dynamic continuation drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2285' -Description "mapped break source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2288' -Description "first excluded executable"
Assert-Contains -Path $module -Pattern 'line 2286.*CP394' -Description "lexical control-boundary note"
Assert-Contains -Path $module -Pattern 'line 2313' -Description "dynamic continuation note"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_BREAK_SOURCE_ORDER' `
    -Expected @($site) -Description "exact sole break site"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp393BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot declaration"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne $numericFields.Count) { throw "CP393 snapshot must expose exactly six Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP393 numeric field $($index + 1) expected '$($numericFields[$index])'" }
}
$localStart = $snapshotStruct.IndexOf("pub $($localBools[0]):")
if ($localStart -lt 0) { throw "CP393 local field boundary missing" }
[string[]]$actualLocalBools = @([regex]::Matches($snapshotStruct.Substring($localStart), 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*bool') | ForEach-Object { $_.Groups['field'].Value })
if ($actualLocalBools.Count -ne 2 -or $actualLocalBools[0] -cne $localBools[0] -or $actualLocalBools[1] -cne $localBools[1]) { throw "CP393 local boolean subsequence drift" }
Assert-PatternsInOrder -Path $module -Patterns @(
    "pub\s+$($localBools[0])\s*:\s*bool",
    'pub\s+predecessor_cp392_resulting_supply_humidity_ratio\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+predecessor_cp392_resulting_supply_enthalpy_j_per_kg\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+predecessor_cp392_resulting_supply_temperature_c\s*:\s*Option\s*<\s*f64\s*>',
    "pub\s+$($localBools[1])\s*:\s*bool",
    'pub\s+resulting_supply_humidity_ratio\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+resulting_supply_enthalpy_j_per_kg\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+resulting_supply_temperature_c\s*:\s*Option\s*<\s*f64\s*>'
) -Description "CP393 schema interleaving with independent numeric and boolean subsequences"
foreach ($forbidden in @(
        'pub dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed:',
        'cp391_retained_supply_temperature_state_owned', 'cp391_retained_supply_enthalpy_state_owned',
        'cp391_retained_supply_temperature_owned_read', 'supply_temperature_for_humidity_ratio_inversion_read',
        'cp391_retained_supply_enthalpy_owned_read', 'supply_enthalpy_for_humidity_ratio_inversion_read',
        'psychrometric_supply_humidity_ratio_evaluated', 'supply_humidity_ratio_assignment_performed'
    )) { if ($snapshotStruct.Contains($forbidden)) { throw "CP392 local operation evidence leaked into CP393: $forbidden" } }

$stateStruct = Get-Cp393BraceBlock -Text (Read-RepoText -Path $state) -AnchorPattern "pub\s+struct\s+$($typeStem)RuntimeState\s*" -Description "runtime state declaration"
[string[]]$actualStateFields = @([regex]::Matches($stateStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:') | ForEach-Object { $_.Groups['field'].Value })
if ($actualStateFields.Count -ne $stateFields.Count) { throw "CP393 runtime state public field count drift" }
for ($index = 0; $index -lt $stateFields.Count; $index += 1) {
    if ($actualStateFields[$index] -cne $stateFields[$index]) { throw "CP393 state field $($index + 1) expected '$($stateFields[$index])'" }
}
Assert-Contains -Path $state -Pattern 'predecessor_route_counts\s*:\s*\[usize;\s*30\]' -Description "thirty route counters"
Assert-Contains -Path $state -Pattern 'pub\(super\)\s+latest_route' -Description "private latest route"
Assert-Contains -Path $state -Pattern 'pub\(super\)\s+latest_transition_ordinal' -Description "private latest ordinal"
Assert-NotContains -Path $state -Pattern '(?i)(?:humidity|enthalpy|temperature).*?(?:owner|read|evaluation|write)_count' -Description "numeric owner/read/write counter firewall"

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        [regex]::Escape("$($predecessorStem)_snapshot_route"),
        'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'matches!\(route\.predecessor_index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)\s*&&\s*!route\.active',
        'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)',
        'index\s*>=\s*3',
        'state\.inactive_transition_count\.checked_add\(breaks\)\s*!=\s*Some\(state\.transition_count\)',
        'active_total\s*!=\s*Some\(breaks\)',
        'expected_sites\s*!=\s*Some\(state\.source_site_execution_count\)',
        'state\.predecessor_route_counts\s*==\s*predecessor\.predecessor_route_counts',
        'case_break_count\s*==\s*predecessor\.dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count'
    )) { Assert-Cp393Text -Text $core -Pattern $pattern -Description "route/algebra contract" }
Assert-Contains -Path $accounting -Pattern '(?s)if\s+!route\.active.*?inactive_transition_count\s*\+=\s*1;.*?return;.*?case_break_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=' -Description "inactive versus active accounting"
Assert-Contains -Path $transition -Pattern 'case_exited_via_break:\s*route\.active' -Description "route-owned break flag"
foreach ($field in $numericFields[0..2]) { Assert-Contains -Path $transition -Pattern ([regex]::Escape("predecessor.$($field.Replace('predecessor_cp392_',''))")) -Description "CP392 terminal carrier input" }
foreach ($pair in @(
        @('predecessor_cp392_resulting_supply_humidity_ratio', 'resulting_supply_humidity_ratio'),
        @('predecessor_cp392_resulting_supply_enthalpy_j_per_kg', 'resulting_supply_enthalpy_j_per_kg'),
        @('predecessor_cp392_resulting_supply_temperature_c', 'resulting_supply_temperature_c')
    )) { Assert-Contains -Path $transition -Pattern ("$($pair[1]):\s*$($pair[0])") -Description "unchanged terminal carrier" }
Assert-NotContains -Path $transition -Pattern 'ActiveInput|DirectZonePurchasedAirCouplingInput|energyplus_psy|PsyWFn|PsyHFn|is_finite|is_nan|f64::|\.clamp\s*\(|\.min\s*\(|\.max\s*\(|mul_add|total_cmp|partial_cmp' -Description "numeric work and scalar-input firewall"
Assert-NotContains -Path $transition -Pattern '(?:supply_humidity_ratio|supply_enthalpy|supply_temperature)[^\r\n]*(?:\+|\*|/|\s-\s)' -Description "terminal-carrier arithmetic firewall"

Assert-Contains -Path $release -Pattern 'SupplyHumidityRatioAssignmentSnapshot as Predecessor' -Description "exact CP392 predecessor type"
Assert-Contains -Path $release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp392:\s*Predecessor,\s*\)' -Description "predecessor-only public arguments"
Assert-Contains -Path $release -Pattern 'supply_humidity_ratio_assignment_snapshot_is_exact_direct_release\(predecessor_cp392\)' -Description "recursive CP392 direct release"
Assert-Contains -Path $prefix -Pattern 'completed_direct_.*?supply_humidity_ratio_assignment_is_consistent' -Description "recursive complete CP392 prefix"
Assert-NotContains -Path $release -Pattern 'predecessor_cp391|OverdryingLimitSnapshot as Predecessor|ActiveInput|numerical_supply|latest_numerical|supply_temperature_c:\s*f64|supply_enthalpy_j_per_kg:\s*f64' -Description "older or scalar predecessor substitution"

$productionFiles = @($transition, $accounting, $release, $prefix, $runtimeValidation, $snapshotValidation, $privateCharacterization, $adapter, $coupled, $pipelineValidation) +
    @(Get-ChildItem -LiteralPath $coupledRoot -Recurse -File -Filter '*.rs' | Where-Object { $_.FullName -notmatch '[\\/]tests\.rs$' } | ForEach-Object { $_.FullName }) +
    @(Get-ChildItem -LiteralPath $pipelineDir -Recurse -File -Filter '*.rs' | Where-Object { $_.FullName -notmatch '[\\/]tests\.rs$' } | ForEach-Object { $_.FullName })
$cp394Pattern = '(?i)\bCP394\b|PurchasedAirManager\.cc:(?:2286|2288|2313)|line[-_ ]?(?:2286|2288|2313)|humidistat.*case[_ -]?(?:entry|entered)|saturation[_ -]?continuation'
foreach ($path in @($productionFiles | Select-Object -Unique)) {
    Assert-NotContains -Path $path -Pattern $cp394Pattern -Description "CP394/Humidistat/saturation-continuation firewall"
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern 'DirectZonePurchasedAirCouplingInput|\.coupling\b|\.prediction\b|\.feedback\b|ResultStore|reconcil|latest_numerical|supply_node_update|numerical_dto|\breport\b|\bloads?\b' -Description "numerical DTO/result/node/load/report firewall"
}

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine + (($coreFiles | Where-Object { $_.FullName -match '[\\/]tests(?:\.rs|[\\/])' } | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine)
foreach ($pattern in @(
        'cp393_boundary_and_single_break_site_are_exact',
        'cp393_preserves_thirty_routes_and_breaks_exactly_three',
        'assert_eq!\(snapshots\.len\(\),\s*30\)',
        'assert_eq!\(state\.transition_count,\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*27\)',
        'assert_eq!\(state\.predecessor_route_counts,\s*\[1;\s*30\]\)',
        'assert_eq!\(state\.source_site_execution_count,\s*3\)',
        'compressed_snapshot_preserves_arbitrary_carrier_bits_without_numeric_gates',
        'binary64_snapshot_comparison_distinguishes_nan_payloads',
        'malformed_or_wrong_identity_cp392_predecessor_rejects_without_mutation',
        'every_active_counter_overflow_rejects_before_mutation',
        'inactive_counter_overflow_rejects_before_mutation',
        'direct_release_skips_break_and_retains_lifecycle_metadata'
    )) { Assert-Cp393Text -Text $testText -Pattern $pattern -Description "non-vacuous core test" }

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\s*=',
    'let\s+unit_available\s*='
) -Description "CP392-to-CP393-to-unchanged-numerical binding order"
$bindingText = Read-RepoText -Path $binding
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($numericalIndex -lt 0) { throw "CP393 unchanged numerical coupling anchor missing" }
$dto = Get-Cp393BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match [regex]::Escape($stem) -or $dto -match 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break' -or $dto -match 'predecessor_cp392_resulting_supply_(?:humidity_ratio|enthalpy|temperature)|dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break') {
    throw "CP393 evidence unexpectedly feeds DirectZonePurchasedAirCouplingInput"
}
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break:',
    'pub\s+coupling:'
) -Description "CP392-to-CP393-to-CP396-to-numerical output order"
Assert-PatternsInOrder -Path $coupledRootFile -Patterns @(
    'let\s+calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_lifecycle\s*=',
    'let\s+calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_lifecycle\s*='
) -Description "coupled lifecycle order"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle"
) -Description "pipeline CP392-to-CP393 validation order"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp399_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineValidationTests -Pattern 'public_cp393_validator_depends_only_on_cp392_and_requires_all_routes_inactive' -Description "public direct inactive validation"
Assert-Contains -Path $pipelineValidationTests -Pattern 'ep_run_cp393_rejects_missing_cp392_predecessor_evidence' -Description "missing CP392 rejection"
Assert-Contains -Path $pipelineValidationTests -Pattern 'ep_run_cp393_links_exactly_to_cp392_and_rejects_corruption' -Description "CP392 corruption rejection"
foreach ($pattern in @('cp393_preserves_cp392_terminal_carriers_and_skips_break_on_direct_routes', 'cp393_rejects_cp392_carrier_bit_drift_and_route_drift', 'cp393_validation_remains_independent_of_numerical_output_state')) {
    Assert-Contains -Path $coupledTests -Pattern $pattern -Description "coupled CP393 test"
}
Assert-Contains -Path $bindingTests -Pattern 'binding_places_cp393_after_cp392_before_unchanged_numerical_coupling' -Description "binding order and numerical preservation test"

$snapshotJsonText = Read-RepoText -Path $snapshotJson
$jsonCursor = 0
foreach ($field in $numericFields) {
    foreach ($key in @($field, "$( $field )_ieee_bits")) {
        $needle = '"' + $key + '"'
        if ([regex]::Matches($snapshotJsonText, [regex]::Escape($needle)).Count -ne 1) { throw "CP393 JSON key '$key' must occur exactly once" }
        $next = $snapshotJsonText.IndexOf($needle, $jsonCursor)
        if ($next -lt 0) { throw "CP393 JSON carrier order drift at '$key'" }
        $jsonCursor = $next + $needle.Length
    }
}
if ([regex]::Matches($snapshotJsonText, '_ieee_bits"').Count -ne 6) { throw "CP393 JSON must expose exactly six IEEE sidecars" }
foreach ($field in $localBools) {
    if ([regex]::Matches($snapshotJsonText, '"' + [regex]::Escape($field) + '"').Count -ne 1) { throw "CP393 JSON boolean key '$field' must occur exactly once" }
}
Assert-PatternsInOrder -Path $snapshotJson -Patterns @(
    ('"' + [regex]::Escape($localBools[0]) + '"'),
    '"predecessor_cp392_resulting_supply_humidity_ratio"',
    '"predecessor_cp392_resulting_supply_enthalpy_j_per_kg"',
    '"predecessor_cp392_resulting_supply_temperature_c"',
    ('"' + [regex]::Escape($localBools[1]) + '"'),
    '"resulting_supply_humidity_ratio"',
    '"resulting_supply_enthalpy_j_per_kg"',
    '"resulting_supply_temperature_c"'
) -Description "CP393 JSON schema interleaving and boolean subsequence"
Assert-Contains -Path $snapshotJson -Pattern '(?s)value\s*\.filter\(\|value\| value\.is_finite\(\)\).*?Value::Null' -Description "finite-only JSON projection"
Assert-Contains -Path $snapshotJson -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "authoritative IEEE sidecar"
Assert-Contains -Path $snapshotJsonTests -Pattern 'six_compact_carriers_serialize_with_adjacent_ieee_sidecars' -Description "six carrier serialization test"
Assert-Contains -Path $snapshotJsonTests -Pattern 'six_nonfinite_carriers_project_null_and_preserve_nan_payload_bits' -Description "nonfinite sidecar test"
foreach ($path in @($module, $transition, $snapshotJson)) {
    Assert-NotContains -Path $path -Pattern 'predecessor_mixed_air_humidity_ratio|predecessor_psychrometric_cp_air|predecessor_cp391_|psychrometric_supply_humidity_ratio|assigned_supply_humidity_ratio|supply_temperature_for_humidity_ratio_inversion_read' -Description "CP392 intermediate numeric/operation leakage"
}

Assert-Contains -Path $cp392Assertions -Pattern '#\[path\s*=\s*"cp393_assertions\.rs"\]' -Description "arbitrary-run CP392-to-CP393 chain"
foreach ($pattern in @('CP393_KEY', 'PurchasedAirManager\.cc:2285', 'PurchasedAirManager\.cc:2288', $site, 'dehumidification_control_constant_sensible_heat_ratio_case_break_count', 'source_site_execution_count', 'predecessor_cp392_resulting_supply_humidity_ratio_ieee_bits', 'resulting_supply_temperature_c_ieee_bits', 'assert_non_direct')) {
    Assert-Contains -Path $cp393Assertions -Pattern $pattern -Description "arbitrary-run CP393 assertion"
}
Assert-Contains -Path $coupledOutputRoot -Pattern ([regex]::Escape("$($stem)_fixture.rs")) -Description "coupled-output fixture wiring"
Assert-Contains -Path $fixture -Pattern 'predecessor_cp392_resulting_supply_humidity_ratio' -Description "coupled-output compressed carrier fixture"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP393 supersedes only CP392[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP393 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) { throw "CP393 must add exactly two algorithm and two capability claims" }
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, 'physical executable line 2285', '\}\s*break;', $site,
            'line 2286', 'CP394', 'line 2288', 'line 2313', 'thirty', '18', '22', '28',
            'twenty-seven', 'eleven', 'nineteen', 'T393=T392', 'B393=A392', 'sole immediate predecessor', 'CP392',
            'exactly six', 'six authoritative IEEE sidecars', 'routes 5, 8, 11, 14', 'routes 3 through 29', 'boolean subsequence', 'schema field order',
            'CP392-to-CP393-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput',
            '32 algorithms', '293 routines', '58 state-mapped', '235 source-mapped', '170 required',
            '331 total', '240 public', '91 internal', 'zero unused', 'zero unreachable', '238 development commands', 'Roadmap'
        )) { if ($claim.Value -notmatch $pattern) { throw "CP393 spec addendum missing '$pattern'" } }
}

$docs = @(
    "docs\src\current\current-status.md", "docs\src\current\project-contract.md",
    "docs\src\porting-map\ideal-loads-source-map.md", "docs\src\porting-map\heat-balance-source-map.md",
    "docs\src\porting-map\zone-air-update-map.md"
)
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    $sections = [regex]::Matches($text, '(?ms)^## CP393\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP393 documentation expected one section in $doc" }
    foreach ($pattern in @(
            $commit, $hash, '2285', '\}\s*break;', $site, '2286', 'CP394', '2288', '2313',
            '30|thirty', '27|twenty-seven', '18', '22', '28', 'eleven', 'nineteen',
            'T393\s*=\s*T392', 'B393\s*=\s*A392', 'CP392', 'exactly six', 'IEEE sidecars', 'boolean subsequence', 'Schema order',
            'CP392-to-CP393-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput',
            '32\s+algorithms', '293\s+routines', '58\s+`?state[_-]mapped`?', '235\s+`?source[_-]mapped`?', '170\s+required',
            '331\s+total', '240\s+public', '91\s+internal', '238\s+development\s+commands', 'Roadmap'
        )) { if ($sections[0].Value -notmatch $pattern) { throw "CP393 documentation in $doc missing '$pattern'" } }
    $cp392Index = $text.LastIndexOf("## CP392 ")
    $cp393Index = $text.LastIndexOf("## CP393 ")
    if ($cp392Index -lt 0 -or $cp393Index -le $cp392Index) { throw "CP392-to-CP393 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP393\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP393 supersedes only CP392' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP393 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..392) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp399_lifecycle_evidence' -Description "historical cumulative firewall"
}
foreach ($historical in 335..392) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 337 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 97 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..392) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 337' -Description "historical inventory total"
}
foreach ($historical in 367..392) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 97' -Description "historical internal classification"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 97 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP393 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-LineLimit -Path $cp345Audit -Limit 1200 -Description "CP345 fixed structural cap"

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$cp393AuditIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp392AuditIndex -lt 0 -or $cp393AuditIndex -le $cp392AuditIndex -or $completionIndex -le $cp393AuditIndex) { throw "Master audit must dot-source CP393 after CP392 before completion" }
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 337', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) { Assert-Cp393Text -Text $inventory -Pattern $pattern -Description "inventory" }
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 97) { throw "CP393 inventory must be exactly 240 public and 97 internal scripts" }
Assert-Cp393Text -Text $inventory -Pattern ('path = "scripts/quality/ideal-loads-structure-audit/' + [regex]::Escape((Split-Path -Leaf $audit)) + '"') -Description "inventory record"
foreach ($pattern in @('\| executable script records \| 337 \|', '\| public scripts \| 240 \|', '\| internal scripts \| 97 \|', '\| scripts without callers \| 0 \|')) { Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory" }

Write-Host "CP393 post-saturation constant-SHR case-break structure audit passed."
}
