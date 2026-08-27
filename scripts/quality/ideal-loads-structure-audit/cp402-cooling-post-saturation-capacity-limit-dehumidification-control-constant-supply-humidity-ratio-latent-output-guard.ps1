# CP402 maps PurchasedAirManager.cc physical executable line 2297 only and
# stops before line 2298's mixed-air supply-temperature assignment.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuard"
$pipelineStem = "purchased_air_$stem"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$routes = "$root\transition\routes.rs"
$accounting = "$root\transition\accounting.rs"
$tests = "$root\tests\mod.rs"
$testFixtures = "$root\tests\fixtures.rs"
$ieeeTests = "$root\tests\ieee.rs"
$overflowTests = "$root\tests\overflow.rs"
$routeTests = "$root\tests\routes.rs"
$release = "$root\release.rs"
$prefix = "$root\release\prefix_validation.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$counterValidation = "$root\release\runtime_validation\counters.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$error = "$root\release\error.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation\snapshot.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp402.rs"
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
$arbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp402_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp402-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-guard.ps1"
$sites = @(
    "read-retained-cooling-latent-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-comparison",
    "compare-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cooling-latent-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
    "enter-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-body-if-comparison-satisfied"
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
    "cooling_latent_output_w",
    "maximum_total_cooling_capacity_w",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)

function Assert-Cp402Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP402 $Description missing '$Pattern'" }
}

function Get-Cp402BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP402 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP402 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP402 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $accounting, $tests, $testFixtures, $ieeeTests,
    $overflowTests, $routeTests,
    $release, $prefix,
    $runtimeValidation, $counterValidation, $snapshotValidation, $privateCharacterization,
    $error, $adapter, $adapterTests, $coupled, $coupledSnapshot, $coupledTests,
    $coupledFixture, $witness,
    $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineLineage,
    $pipelineSerialization, $snapshotJson, $snapshotJsonTests, $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP402 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP402 bounded file"
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP402 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2296].Trim() -cne 'if (CoolLatOutput >= PurchAir.MaxCoolTotCap) {' -or
    $lines[2297].Trim() -cne 'PurchAir.SupplyTemp = PurchAir.MixedAirTemp;') {
    throw "CP402 source boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2297' -Description "mapped executable"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2298' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER' `
    -Expected $sites -Description "exact four source sites"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp402BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 35) { throw "CP402 snapshot must expose exactly thirty-five Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP402 numeric field order drift at $index" }
}
Assert-PatternsInOrder -Path $module -Patterns @(
    'pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated\s*:\s*bool',
    'pub\s+cp401_retained_cooling_latent_output_owned_read\s*:\s*bool',
    'pub\s+cooling_latent_output_read\s*:\s*bool',
    'pub\s+cp321_maximum_total_cooling_capacity_owned_read\s*:\s*bool',
    'pub\s+cp340_same_call_maximum_total_cooling_capacity_bit_corroborated\s*:\s*bool',
    'pub\s+maximum_total_cooling_capacity_read\s*:\s*bool',
    'pub\s+cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated\s*:\s*bool',
    'pub\s+cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity\s*:\s*Option<bool>',
    'pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered\s*:\s*bool',
    'pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough\s*:\s*bool'
) -Description "owner and four-site guard schema"

$stateText = Read-RepoText -Path $state
foreach ($pattern in @(
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]',
        'guard_false_fallthrough_route_counts\s*:\s*\[usize;\s*30\]',
        'adjustment_body_entry_route_counts\s*:\s*\[usize;\s*30\]',
        'dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count',
        'source_site_execution_count',
        'cp401_cooling_latent_output_owned_read_count', 'cooling_latent_output_read_count',
        'cp321_maximum_total_cooling_capacity_owned_read_count',
        'cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_latent_output_maximum_total_cooling_capacity_comparison_count',
        'cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count',
        'latent_output_capacity_adjustment_body_entry_count',
        'latent_output_capacity_guard_false_fallthrough_count',
        'pub\(super\)\s+latest_route', 'pub\(super\)\s+latest_transition_ordinal'
    )) { Assert-Cp402Text -Text $stateText -Pattern $pattern -Description "state contract" }

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'source_greater_than_or_equal\(\s*input\.cooling_latent_output_w\s*,\s*input\.maximum_total_cooling_capacity_w\s*,?\s*\)',
        'fn\s+source_greater_than_or_equal\(left:\s*f64,\s*right:\s*f64\)\s*->\s*bool\s*\{\s*left\s*>=\s*right\s*\}',
        'source_site_execution_count\s*\+=\s*3\s*\+\s*usize::from\(route\.body_entered\)',
        'false_count\.checked_add\(body_count\)\s*==\s*Some\(state\.predecessor_route_counts\[index\]\)',
        'state\.transition_count\s*==\s*predecessor\.transition_count'
    )) { Assert-Cp402Text -Text $core -Pattern $pattern -Description "route/comparison/accounting contract" }
foreach ($forbidden in @('mul_add\s*\(', '\.max\s*\(', '\.abs\s*\(', 'epsilon|tolerance|approx', 'DirectZonePurchasedAirCouplingInput')) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "forbidden guard arithmetic/numerical coupling"
}
$activeIndices = @(20, 21, 24, 25, 27, 29)
$publicPredecessors = @(0..8) + @(20, 24)
if ((30 - $activeIndices.Count) -ne 24 -or (30 + $activeIndices.Count) -ne 36 -or
    ($publicPredecessors.Count + 2) -ne 13 -or (36 - 13) -ne 23 -or
    (3 * 12 + 6) -ne 42) { throw "CP402 logical route/accounting constants drift" }
foreach ($expectation in @(
        @('public_successors\s*,\s*13', 'thirteen public successor routes'),
        @('state\.transition_count\s*,\s*36', 'thirty-six exhaustive successor routes'),
        @('state\.inactive_transition_count\s*,\s*24', 'twenty-four inactive routes'),
        @('latent_output_capacity_guard_evaluation_count\s*,\s*12', 'twelve guard evaluations'),
        @('latent_output_capacity_guard_false_fallthrough_count\s*,\s*6', 'six false fallthroughs'),
        @('latent_output_capacity_adjustment_body_entry_count\s*,\s*6', 'six body entries'),
        @('state\.source_site_execution_count\s*,\s*42', 'forty-two source-site executions')
    )) { Assert-Cp402Text -Text $core -Pattern $expectation[0] -Description $expectation[1] }
foreach ($pattern in @('f64::NAN', 'f64::INFINITY', 'f64::NEG_INFINITY', 'to_bits\(\)')) {
    Assert-Cp402Text -Text $core -Pattern $pattern -Description "raw IEEE comparison characterization"
}

Assert-Contains -Path $release -Pattern 'LatentOutputAssignmentSnapshot as Predecessor' -Description "exact CP401 predecessor"
Assert-Contains -Path $prefix -Pattern 'completed_direct_.*latent_output_assignment_is_consistent' -Description "recursive CP401 completion"
Assert-Contains -Path $prefix -Pattern 'calc_cooling_capacity_zero_flow_reset\.latest' -Description "CP321 maximum-capacity owner"
Assert-Contains -Path $prefix -Pattern 'calc_cooling_positive_supply_capacity_limit_sensible_output_guard\s*\.latest' -Description "CP340 same-call corroborator"
Assert-Contains -Path $prefix -Pattern 'maximum_total_cooling_capacity_w\.is_finite\(\)' -Description "finite nonnegative owner"
Assert-Contains -Path $prefix -Pattern 'value\.to_bits\(\)\s*==\s*maximum_total_cooling_capacity_w\.to_bits\(\)' -Description "CP340 bit corroboration"
Assert-Contains -Path $snapshotValidation -Pattern 'predecessor_index_is_public\(route\.predecessor_index\)' -Description "public predecessor routes"
Assert-Contains -Path $privateCharacterization -Pattern 'private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization' -Description "private route characterization"

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*=', 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*=',
    'let\s+unit_available\s*=',
    'let\s+coupling\s*='
) -Description "CP401-to-CP402-to-CP403-to-CP404-to-CP405-to-CP406-to-CP407-to-CP408 binding order"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\s*:',
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
$bindingEvidenceName = 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard'
if ([regex]::Matches($bindingText, "\b$bindingEvidenceName\b").Count -ne 3) {
    throw "CP402 binding evidence must be produced once, consumed once by CP403, and stored once without feeding numerical coupling"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}

$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 35 -or $ieeeSidecars.Count -ne 35) {
    throw "CP402 JSON snapshot must expose exactly thirty-five numeric/IEEE pairs"
}
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $field = $numericFields[$index]
    $escapedField = [regex]::Escape($field)
    $adjacentPair = '(?m)^\s*"' + $escapedField + '"\s*:\s*json_number\s*\(\s*snapshot\.' + $escapedField + '\s*\)\s*,\s*\r?\n\s*"' + $escapedField + '_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.' + $escapedField + '\s*\)'
    if ($jsonNumbers[$index].Groups['field'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['field'].Value -cne $field -or
        $serializationText -notmatch $adjacentPair) { throw "CP402 JSON numeric/IEEE sidecar order drift at $field" }
}
foreach ($pattern in @(
        'predecessor_cp401\s*:\s*Option<&PredecessorLifecycle>',
        'capacity_owner_cp321\s*:\s*Option<&OwnerLifecycle>',
        'capacity_corroborator_cp340\s*:\s*Option<&CorroboratorLifecycle>',
        'CP401 latest evidence is missing', 'CP321 latest owner is missing',
        'CP340 latest corroborator is missing'
    )) { Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline owner contract" }
Assert-Contains -Path $pipelineLineage -Pattern 'cooling_latent_output_w\s*>=\s*maximum_total_cooling_capacity_w' -Description "pipeline raw comparison"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp428_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle"
) -Description "pipeline CP401-to-CP402 validation order"
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp402_after_cp401_before_unchanged_numerical_coupling' -Description "binding execution/nonfeed regression"
Assert-Contains -Path $coupledTests -Pattern 'cp402_executes_public_routes_with_raw_greater_than_or_equal_comparison' -Description "coupled raw-comparison regression"
Assert-Contains -Path $coupledTests -Pattern 'cp402_rejects_cp401_owner_cp321_owner_and_cp340_corroborator_drift' -Description "coupled owner regression"
Assert-Contains -Path $coupledFixture -Pattern $bindingEvidenceName -Description "coupled output fixture"
Assert-Contains -Path $witness -Pattern 'set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_latest_witness' -Description "runtime witness setter"
Assert-Contains -Path $snapshotJsonTests -Pattern 'ieee' -Description "snapshot IEEE regressions"
Assert-Contains -Path $arbitraryAssertions -Pattern 'route\(20\).*route\(24\)' -Description "public active-predecessor regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'ends_with\("_ieee_bits"\)' -Description "exact IEEE-sidecar regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'non-direct runtime must not publish CP402 evidence' -Description "non-direct regression"

foreach ($doc in @(
        'docs\src\current\current-status.md', 'docs\src\current\project-contract.md',
        'docs\src\porting-map\heat-balance-source-map.md',
        'docs\src\porting-map\ideal-loads-source-map.md',
        'docs\src\porting-map\zone-air-update-map.md'
    )) { Assert-Contains -Path $doc -Pattern 'CP402 post-saturation shared-case latent-output maximum-capacity guard' -Description "CP402 documentation" }
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP402 supersedes only CP401' -Description "algorithm claim"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP402 additionally requires' -Description "capability claim"
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern 'CP402' -Description "Roadmap non-promotion"

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
foreach ($file in @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 401) {
        Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp428_lifecycle_evidence' -Description "historical CP402 non-direct firewall"
    }
    if ($number -ge 337 -and $number -le 401) {
        Assert-Contains -Path $file.FullName -Pattern 'script_count = 366' -Description "historical current script count"
    }
    if ($number -ge 335 -and $number -le 401) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 366 \|')) -Description "historical generated script count"
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 126 \|')) -Description "historical generated internal count"
    }
    if ($number -ge 367 -and $number -le 401) {
        Assert-Contains -Path $file.FullName -Pattern 'Count -ne 126' -Description "historical internal classification count"
    }
}
$cp345Audit = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp402Call\s*=', 'CP401-to-CP402', 'CP402-to-CP403', 'CP403-to-CP404')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "CP345 terminal CP402 order"
}

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$cp401Index = $master.IndexOf('cp401-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-assignment.ps1')
$cp402Index = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp401Index -lt 0 -or $cp402Index -le $cp401Index -or $completionIndex -le $cp402Index) {
    throw "Master CP402 registration order drift"
}
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 366', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp402Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 126) {
    throw "CP402 inventory classification drift; expected 240 public and 122 internal"
}
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp402-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-guard\.ps1' -Description "inventory record"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 366 \|' -Description "generated script total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 126 \|' -Description "generated internal total"

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP404-to-CP405' -Description "CP345 CP404-to-CP405 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP405-to-CP406' -Description "CP345 CP405-to-CP406 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP406-to-CP407' -Description "CP345 CP406-to-CP407 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP407-to-CP408' -Description "CP345 CP407-to-CP408 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP408-to-CP409' -Description "CP345 CP408-to-CP409 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP410-to-CP411' -Description "CP345 CP410-to-CP411 interval"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\s*=' -Description "CP410 historical binding order"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\s*=' -Description "CP411 historical binding order"
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP411-to-CP412' -Description 'CP345 CP411-to-CP412 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\s*=' -Description 'CP412 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP412-to-CP413' -Description 'CP345 CP412-to-CP413 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP413-to-CP414' -Description 'CP345 CP413-to-CP414 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\s*=' -Description 'CP413 historical binding order'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\s*=' -Description 'CP414 historical binding order'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\s*=' -Description 'CP415 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp415Call' -Description 'CP415 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP414-to-CP415' -Description 'CP414-to-CP415 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\s*=' -Description 'CP416 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp416Call' -Description 'CP416 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP415-to-CP416' -Description 'CP415-to-CP416 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp417Call' -Description 'CP417 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP416-to-CP417' -Description 'CP416-to-CP417 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP402 post-saturation shared-case latent-output maximum-capacity guard structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-numerical' -Description 'CP428-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-numerical' -Description 'CP428-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-numerical' -Description 'CP428-to-numerical terminal interval'
