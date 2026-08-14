# CP403 maps PurchasedAirManager.cc physical executable line 2298 only and
# stops before line 2299's PsyWFnTdbH supply-humidity-ratio assignment.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignment"
$pipelineStem = "purchased_air_$stem"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$routes = "$root\transition\routes.rs"
$accounting = "$root\transition\accounting.rs"
$snapshotBuilder = "$root\transition\snapshot.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation\snapshot.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp403.rs"
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
$arbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp403_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp403-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-mixed-air-assignment.ps1"
$sites = @(
    "read-retained-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-supply-temperature-assignment",
    "assign-purchased-air-supply-temperature-from-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment"
)
$numericFields = @(
    "predecessor_cp397_resulting_supply_humidity_ratio",
    "predecessor_cp397_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp397_resulting_supply_temperature_c",
    "predecessor_cp398_resulting_supply_humidity_ratio",
    "predecessor_cp398_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp398_resulting_supply_temperature_c",
    "predecessor_mixed_air_humidity_ratio",
    "predecessor_psychrometric_cp_air_result_j_per_kg_k",
    "predecessor_cp_air_j_per_kg_k",
    "predecessor_cp399_resulting_supply_humidity_ratio",
    "predecessor_cp399_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp399_resulting_supply_temperature_c",
    "predecessor_supply_mass_flow_rate_kg_per_s",
    "predecessor_cp400_cp_air_j_per_kg_k",
    "predecessor_supply_mass_flow_rate_times_cp_air_w_per_k",
    "predecessor_mixed_air_temperature_c",
    "predecessor_supply_temperature_c",
    "predecessor_mixed_air_minus_supply_temperature_k",
    "predecessor_calculated_cooling_sensible_output_w",
    "predecessor_cooling_sensible_output_w",
    "predecessor_cp400_resulting_supply_humidity_ratio",
    "predecessor_cp400_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp400_resulting_supply_temperature_c",
    "predecessor_cooling_total_output_w",
    "predecessor_cp401_cooling_sensible_output_w",
    "predecessor_calculated_cooling_latent_output_w",
    "predecessor_cooling_latent_output_w",
    "predecessor_cp401_resulting_supply_humidity_ratio",
    "predecessor_cp401_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp401_resulting_supply_temperature_c",
    "predecessor_cp402_cooling_latent_output_w",
    "predecessor_maximum_total_cooling_capacity_w",
    "predecessor_cp402_resulting_supply_humidity_ratio",
    "predecessor_cp402_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp402_resulting_supply_temperature_c",
    "mixed_air_temperature_c",
    "assigned_supply_temperature_c",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)

function Assert-Cp403Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP403 $Description missing '$Pattern'" }
}

function Get-Cp403BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP403 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP403 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP403 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $accounting, $snapshotBuilder, $tests,
    $release, $runtimeValidation, $snapshotValidation, $privateCharacterization,
    $adapter, $adapterTests, $coupled, $coupledSnapshot, $coupledTests, $coupledFixture,
    $witness, $pipeline, $pipelineValidation, $pipelineLineage, $pipelineValidationTests,
    $pipelineSerialization, $snapshotJson, $snapshotJsonTests, $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP403 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP403 bounded file"
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP403 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2297].Trim() -cne 'PurchAir.SupplyTemp = PurchAir.MixedAirTemp;' -or
    $lines[2298].Trim() -cne 'PurchAir.SupplyHumRat = PsyWFnTdbH(state, PurchAir.SupplyTemp, SupplyEnthalpy, RoutineName);') {
    throw "CP403 source boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2298' -Description "mapped executable"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2299' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites -Description "exact two source sites"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp403BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 40) { throw "CP403 snapshot must expose exactly forty Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP403 numeric field order drift at $index" }
}
Assert-PatternsInOrder -Path $module -Patterns @(
    'pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed\s*:\s*bool',
    'pub\s+cp329_retained_mixed_air_temperature_owned_read\s*:\s*bool',
    'pub\s+cp402_same_call_mixed_air_temperature_bit_corroborated\s*:\s*bool',
    'pub\s+mixed_air_temperature_read\s*:\s*bool',
    'pub\s+mixed_air_temperature_c\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+supply_temperature_assigned\s*:\s*bool',
    'pub\s+assigned_supply_temperature_c\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+cp402_retained_supply_humidity_ratio_state_owned\s*:\s*bool',
    'pub\s+cp402_retained_supply_enthalpy_state_owned\s*:\s*bool',
    'pub\s+cp402_retained_supply_temperature_state_owned\s*:\s*bool',
    'pub\s+resulting_supply_humidity_ratio\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+resulting_supply_enthalpy_j_per_kg\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+resulting_supply_temperature_c\s*:\s*Option\s*<\s*f64\s*>'
) -Description "owner, assignment, and unchanged-result schema"
Assert-NotContains -Path $module -Pattern 'preexisting_supply_temperature' -Description "forbidden local preexisting-temperature field"

$stateText = Read-RepoText -Path $state
foreach ($pattern in @(
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]',
        'predecessor_guard_false_fallthrough_route_counts\s*:\s*\[usize;\s*30\]',
        'supply_temperature_mixed_air_assignment_route_counts\s*:\s*\[usize;\s*30\]',
        'inactive_transition_count', 'predecessor_guard_false_fallthrough_count',
        'supply_temperature_mixed_air_assignment_count', 'source_site_execution_count',
        'cp402_supply_humidity_ratio_state_owner_count', 'unchanged_supply_humidity_ratio_preservation_count',
        'cp402_supply_enthalpy_state_owner_count', 'unchanged_supply_enthalpy_preservation_count',
        'cp402_supply_temperature_state_owner_count', 'unchanged_supply_temperature_preservation_count',
        'cp329_mixed_air_temperature_owned_read_count',
        'cp402_same_call_mixed_air_temperature_bit_corroboration_count',
        'mixed_air_temperature_read_count', 'supply_temperature_assignment_write_count',
        'pub\(super\)\s+latest_route', 'pub\(super\)\s+latest_transition_ordinal'
    )) { Assert-Cp403Text -Text $stateText -Pattern $pattern -Description "state contract" }

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'predecessor\.predecessor_mixed_air_temperature_c\?',
        'carried\.to_bits\(\)\s*==\s*read\.to_bits\(\)',
        'read\.to_bits\(\)\s*==\s*assigned\.to_bits\(\)',
        'assigned\.to_bits\(\)\s*==\s*resulting\.to_bits\(\)',
        'state\.source_site_execution_count\s*\+=\s*SOURCE_ORDER\.len\(\)',
        'state\.transition_count\s*==\s*predecessor\.transition_count',
        'state\.supply_temperature_mixed_air_assignment_count',
        'state\.predecessor_guard_false_fallthrough_count'
    )) { Assert-Cp403Text -Text $core -Pattern $pattern -Description "route/raw-copy/accounting contract" }
foreach ($forbidden in @('mul_add\s*\(', '\.max\s*\(', '\.min\s*\(', '\.abs\s*\(', 'epsilon|tolerance|approx', 'is_finite\s*\(', 'DirectZonePurchasedAirCouplingInput')) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "forbidden assignment arithmetic, finite gate, or numerical coupling"
}
$activeIndices = @(20, 21, 24, 25, 27, 29)
$publicPredecessors = @(0..8) + @(20, 24)
if ((36 - $activeIndices.Count) -ne 30 -or (30 - $activeIndices.Count) -ne 24 -or
    ($publicPredecessors.Count + 2) -ne 13 -or (36 - 13) -ne 23 -or
    (2 * $activeIndices.Count) -ne 12) { throw "CP403 logical route/accounting constants drift" }
foreach ($pattern in @(
        'routes\.len\(\)\s*,\s*36',
        '\[20,\s*21,\s*24,\s*25,\s*27,\s*29\]',
        '\.count\(\)\s*,\s*13',
        '\[21,\s*27\]'
    )) { Assert-Contains -Path $tests -Pattern $pattern -Description "exact exhaustive/public route characterization" }
foreach ($pattern in @('0x7ff8_0000_0000_0403', 'f64::INFINITY', 'f64::NEG_INFINITY', 'to_bits\(\)')) {
    Assert-Cp403Text -Text $core -Pattern $pattern -Description "raw IEEE assignment characterization"
}

Assert-Contains -Path $release -Pattern 'LatentOutputGuardSnapshot as Predecessor' -Description "exact CP402 predecessor"
Assert-Contains -Path $release -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_committed_latest_snapshot_is_consistent\s*\(' -Description "bounded CP402 committed predecessor proof"
Assert-NotContains -Path $release -Pattern 'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_is_consistent\s*\(' -Description "recursive CP402 predecessor completion"
Assert-Contains -Path $snapshotValidation -Pattern 'predecessor_index_is_public\(route\.predecessor_index\)' -Description "public predecessor routes"
Assert-Contains -Path $privateCharacterization -Pattern 'private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_characterization' -Description "private route characterization"

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*=', 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*=',
    'let\s+unit_available\s*=',
    'let\s+coupling\s*='
) -Description "CP402-to-CP403-to-CP404-to-CP405-to-CP406-to-CP407-to-CP408 binding order"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*:', 'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*:',
    'pub\s+coupling\s*:'
) -Description "scheduled output order"
$bindingText = Read-RepoText -Path $binding
$bindingEvidenceName = 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment'
if ([regex]::Matches($bindingText, "\b$bindingEvidenceName\b").Count -ne 3) {
    throw "CP403 binding evidence must be produced once, consumed once by CP404, and stored once without feeding numerical coupling"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}

$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 40 -or $ieeeSidecars.Count -ne 40) {
    throw "CP403 JSON snapshot must expose exactly forty numeric/IEEE pairs"
}
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $field = $numericFields[$index]
    $escapedField = [regex]::Escape($field)
    $adjacentPair = '(?m)^\s*"' + $escapedField + '"\s*:\s*json_number\s*\(\s*snapshot\.' + $escapedField + '\s*\)\s*,\s*\r?\n\s*"' + $escapedField + '_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.' + $escapedField + '\s*\)'
    if ($jsonNumbers[$index].Groups['field'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['field'].Value -cne $field -or
        $serializationText -notmatch $adjacentPair) { throw "CP403 JSON numeric/IEEE sidecar order drift at $field" }
}
foreach ($pattern in @(
        'predecessor_cp402\s*:\s*Option<&PredecessorLifecycle>',
        'mixed_air_owner_cp329\s*:\s*Option<&OwnerLifecycle>',
        'CP402 latest evidence is missing', 'CP329 owner is missing'
    )) { Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline owner contract" }
Assert-Contains -Path $pipelineLineage -Pattern 'mixed_air_temperature_c' -Description "pipeline raw-copy lineage"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp425_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle",
    'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment::\s*validate_direct_lifecycle',
    'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment::\s*validate_direct_lifecycle'
) -Description "pipeline CP402-to-CP403-to-CP404-to-CP405 validation order"
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp403_after_cp402_before_unchanged_numerical_coupling' -Description "binding execution/nonfeed regression"
Assert-Contains -Path $coupledTests -Pattern 'cp403' -Description "coupled CP403 regression"
Assert-Contains -Path $coupledFixture -Pattern $bindingEvidenceName -Description "coupled output fixture"
Assert-Contains -Path $witness -Pattern 'set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_latest_witness' -Description "runtime witness setter"
Assert-Contains -Path $snapshotJsonTests -Pattern 'ieee' -Description "snapshot IEEE regressions"
Assert-Contains -Path $arbitraryAssertions -Pattern 'route\(20\).*route\(24\)' -Description "public active-predecessor regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'ends_with\("_ieee_bits"\)' -Description "exact IEEE-sidecar regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'non-direct runtime must not publish CP403 evidence' -Description "non-direct regression"

$heading = 'CP403 post-saturation shared-case latent-output body supply-temperature mixed-air assignment'
$docs = @(
    'docs\src\current\current-status.md', 'docs\src\current\project-contract.md',
    'docs\src\porting-map\heat-balance-source-map.md',
    'docs\src\porting-map\ideal-loads-source-map.md',
    'docs\src\porting-map\zone-air-update-map.md'
)
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText, "(?m)^## $([regex]::Escape($heading))$").Count -ne 1) {
        throw "CP403 documentation heading must appear exactly once in $doc"
    }
}
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP403 supersedes only CP402' -Description "algorithm claim"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP403 additionally requires' -Description "capability claim"
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP403 supersedes only CP402' -Description "generated algorithm claim"
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP403 additionally requires' -Description "generated capability claim"
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern 'CP403' -Description "Roadmap non-promotion"
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP402 supersedes only CP401.*Script inventory becomes 340 total, 240 public, 100 internal' -Description "frozen CP402 algorithm inventory prose"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP402 additionally requires.*Script inventory becomes 340 total, 240 public, 100 internal' -Description "frozen CP402 capability inventory prose"

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
foreach ($file in @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 402) {
        Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp425_lifecycle_evidence' -Description "historical CP403 non-direct firewall"
    }
    if ($number -ge 337 -and $number -le 402) {
        Assert-Contains -Path $file.FullName -Pattern 'script_count = 363' -Description "historical current script count"
    }
    if ($number -ge 335 -and $number -le 402) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 363 \|')) -Description "historical generated script count"
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 123 \|')) -Description "historical generated internal count"
    }
    if ($number -ge 367 -and $number -le 402) {
        Assert-Contains -Path $file.FullName -Pattern 'Count -ne 123' -Description "historical internal classification count"
    }
}
$cleanupAudits = @(
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp326-*.ps1' -File
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 }
)
if ($cleanupAudits.Count -ne 17) { throw "CP403 cleanup propagation file set drift" }
foreach ($file in $cleanupAudits) {
    Assert-Contains -Path $file.FullName -Pattern 'latent_output_supply_temperature_mixed_air_assignment =\\s\*advance_.*latent_output_supply_temperature_mixed_air_assignment' -Description "historical cleanup whitelist"
}
$terminalAudits = @(
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp345-*.ps1' -File
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File | Where-Object {
        $_.BaseName -match '^cp(?<number>\d+)-' -and
        (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or
         ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 402))
    }
)
if ($terminalAudits.Count -ne 26) { throw "CP403 terminal-order propagation file set drift" }
foreach ($file in $terminalAudits) {
    Assert-Contains -Path $file.FullName -Pattern 'CP402-to-CP403' -Description "historical CP402-to-CP403 terminal interval"
    Assert-Contains -Path $file.FullName -Pattern 'CP403-to-CP404' -Description "historical CP403-to-CP404 terminal interval"
    Assert-Contains -Path $file.FullName -Pattern 'CP404-to-CP405' -Description "historical CP404-to-CP405 terminal interval"
}
$cp345Audit = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern '\$cp403Call\s*=' -Description "CP345 CP403 call capture"
Assert-Contains -Path $cp345Audit -Pattern '\$cp404Call\s*=' -Description "CP345 CP404 call capture"
Assert-Contains -Path $cp345Audit -Pattern '\$cp405Call\s*=' -Description "CP345 CP405 call capture"
Assert-Contains -Path $cp345Audit -Pattern 'CP404-to-CP405' -Description "CP345 CP404-to-CP405 interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP408-to-CP409' -Description "CP345 CP408-to-CP409 interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP414-to-CP415' -Description "CP345 CP413 terminal interval"
$cp402Audit = "$auditRoot\cp402-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-guard.ps1"
Assert-Contains -Path $cp402Audit -Pattern 'Count -ne 3' -Description "CP402 evidence consumed by CP403"
foreach ($file in @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402)-' })) {
    Assert-Contains -Path $file.FullName -Pattern 'latent_output_supply_temperature_mixed_air_assignment\\s\*' -Description "recent binding/scheduled CP403 order"
    Assert-Contains -Path $file.FullName -Pattern 'latent_output_supply_humidity_ratio_assignment\\s\*' -Description "recent binding/scheduled CP404 order"
    Assert-Contains -Path $file.FullName -Pattern 'latent_output_maximum_capacity_assignment\\s\*' -Description "recent binding/scheduled CP405 order"
}

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$cp402Index = $master.IndexOf('cp402-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-guard.ps1')
$cp403Index = $master.IndexOf((Split-Path -Leaf $audit))
$cp404Index = $master.IndexOf('cp404-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-humidity-ratio-assignment.ps1')
$cp405Index = $master.IndexOf('cp405-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-maximum-capacity-assignment.ps1')
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp402Index -lt 0 -or $cp403Index -le $cp402Index -or $cp404Index -le $cp403Index -or
    $cp405Index -le $cp404Index -or $completionIndex -le $cp405Index) {
    throw "Master CP403 registration order drift"
}
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 363', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp403Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 123) {
    throw "CP403 inventory classification drift; expected 240 public and 122 internal"
}
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp403-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-mixed-air-assignment\.ps1' -Description "inventory record"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 363 \|' -Description "generated script total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description "generated public total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 123 \|' -Description "generated internal total"

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP405-to-CP406' -Description "CP345 CP405-to-CP406 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern '\$cp406Call\s*=' -Description "CP345 CP406 call capture"
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-numerical')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP403 post-saturation shared-case latent-output body supply-temperature mixed-air assignment structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-numerical' -Description 'CP425-to-numerical terminal interval'
