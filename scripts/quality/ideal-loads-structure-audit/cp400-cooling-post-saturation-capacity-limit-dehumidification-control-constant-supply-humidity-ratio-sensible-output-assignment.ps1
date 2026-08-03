# CP400 maps PurchasedAirManager.cc physical executable line 2295 only.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignment"
$pipelineStem = "purchased_air_$stem"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$routes = "$root\transition\routes.rs"
$owners = "$root\transition\owners.rs"
$accounting = "$root\transition\accounting.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$prefix = "$root\release\prefix_validation.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp400.rs"
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
$arbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp400_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp400-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-sensible-output-assignment.ps1"
$sites = @(
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output-first-product",
    "read-local-cp-air-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output-first-product",
    "calculate-supply-mass-flow-rate-times-cp-air-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output",
    "read-purchased-air-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output-difference",
    "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output-difference",
    "calculate-mixed-air-temperature-minus-supply-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output",
    "calculate-mass-flow-cp-air-product-times-temperature-difference-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-sensible-output",
    "assign-local-cooling-sensible-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-case"
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
    "supply_mass_flow_rate_kg_per_s",
    "cp_air_j_per_kg_k",
    "supply_mass_flow_rate_times_cp_air_w_per_k",
    "mixed_air_temperature_c",
    "supply_temperature_c",
    "mixed_air_minus_supply_temperature_k",
    "calculated_cooling_sensible_output_w",
    "cooling_sensible_output_w",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)

function Assert-Cp400Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP400 $Description missing '$Pattern'" }
}

function Get-Cp400BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP400 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP400 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP400 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $owners, $accounting, $tests, $release,
    $prefix, $runtimeValidation, $snapshotValidation, $privateCharacterization,
    $adapter, $adapterTests, $coupled, $coupledTests, $coupledFixture, $witness,
    $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineLineage,
    $pipelineSerialization, $snapshotJson, $snapshotJsonTests, $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP400 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP400 bounded file"
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP400 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2294].Trim() -cne 'CoolSensOutput = SupplyMassFlowRate * CpAir * (PurchAir.MixedAirTemp - PurchAir.SupplyTemp);' -or
    $lines[2295].Trim() -cne 'CoolLatOutput = CoolTotOutput - CoolSensOutput;') {
    throw "CP400 source boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2295' -Description "mapped executable"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2296' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites -Description "exact eight source sites"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp400BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 23) { throw "CP400 snapshot must expose exactly twenty-three Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP400 numeric field order drift at $index" }
}
Assert-PatternsInOrder -Path $module -Patterns @(
    'pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed\s*:\s*bool',
    'pub\s+supply_mass_flow_rate_read\s*:\s*bool',
    'pub\s+cp_air_read\s*:\s*bool',
    'pub\s+supply_mass_flow_rate_times_cp_air_calculated\s*:\s*bool',
    'pub\s+mixed_air_temperature_read\s*:\s*bool',
    'pub\s+supply_temperature_read\s*:\s*bool',
    'pub\s+mixed_air_minus_supply_temperature_calculated\s*:\s*bool',
    'pub\s+cooling_sensible_output_calculated\s*:\s*bool',
    'pub\s+cooling_sensible_output_assigned\s*:\s*bool'
) -Description "eight-site operation schema"

$stateText = Read-RepoText -Path $state
foreach ($pattern in @(
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]',
        'dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count',
        'source_site_execution_count', 'supply_mass_flow_rate_read_count',
        'cp_air_read_count', 'supply_mass_flow_rate_times_cp_air_calculation_count',
        'mixed_air_temperature_read_count', 'supply_temperature_read_count',
        'mixed_air_minus_supply_temperature_calculation_count',
        'cooling_sensible_output_calculation_count',
        'cooling_sensible_output_assignment_write_count',
        'pub\(super\)\s+latest_route', 'pub\(super\)\s+latest_transition_ordinal'
    )) { Assert-Cp400Text -Text $stateText -Pattern $pattern -Description "state contract" }

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'let\s+first_product\s*=\s*active\.supply_mass_flow_rate_kg_per_s\s*\*\s*active\.cp_air_j_per_kg_k',
        'let\s+difference\s*=\s*active\.mixed_air_temperature_c\s*-\s*active\.supply_temperature_c',
        'let\s+output\s*=\s*first_product\s*\*\s*difference',
        'positive_guard_links_to_mixed_air_call',
        'state\.predecessor_route_counts\s*==\s*predecessor\.predecessor_route_counts'
    )) { Assert-Cp400Text -Text $core -Pattern $pattern -Description "route/arithmetic/accounting contract" }
Assert-Contains -Path $accounting -Pattern 'source_site_execution_count\s*\+=\s*PURCHASED_AIR_.*_SOURCE_ORDER\.len\(\)' -Description "eight-site accounting"
foreach ($forbidden in @('mul_add\s*\(', 'DirectZonePurchasedAirCouplingInput')) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "forbidden arithmetic/numerical coupling"
}
foreach ($expectation in @(
        @('state\.transition_count\s*,\s*30', 'thirty exhaustive routes'),
        @('state\.inactive_transition_count\s*,\s*24', 'twenty-four inactive routes'),
        @('sensible_output_assignment_count\s*,\s*6', 'six active routes'),
        @('state\.source_site_execution_count\s*,\s*48', 'forty-eight exhaustive site executions')
    )) { Assert-Contains -Path $tests -Pattern $expectation[0] -Description $expectation[1] }

Assert-Contains -Path $release -Pattern 'CpAirAssignmentSnapshot as Predecessor' -Description "exact CP399 predecessor"
Assert-Contains -Path $prefix -Pattern 'calc_cooling_supply_mass_flow_positive_guard\.latest' -Description "CP330 authoritative owner"
Assert-Contains -Path $prefix -Pattern 'calc_cooling_mixed_air_call\.latest' -Description "CP329 authoritative owner"
Assert-Contains -Path $prefix -Pattern 'completed_direct_.*cp_air_assignment_is_consistent' -Description "recursive CP399 completion"
Assert-Contains -Path $owners -Pattern 'positive_guard_links_to_mixed_air_call' -Description "CP329/CP330 same-call corroboration"
Assert-Contains -Path $snapshotValidation -Pattern 'matches!\(route\.predecessor_index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description "exact public direct routes"
Assert-Contains -Path $snapshotValidation -Pattern 'route\.active\s*==\s*matches!\(route\.predecessor_index,\s*20\s*\|\s*24\)' -Description "exact public active routes"
Assert-Contains -Path $privateCharacterization -Pattern 'private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_characterization' -Description "private route characterization"

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\s*=',
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
) -Description "CP399-to-CP400-to-CP401-to-CP402-to-CP403-to-CP404-to-CP405-to-CP406-to-CP407-to-CP408 binding order"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\s*:',
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
$bindingEvidenceName = 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment'
if ([regex]::Matches($bindingText, "\b$bindingEvidenceName\b").Count -ne 3) {
    throw "CP400 binding evidence must be produced once, consumed by CP401 once, and stored once without feeding numerical coupling"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}

$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 23 -or $ieeeSidecars.Count -ne 23) {
    throw "CP400 JSON snapshot must expose exactly twenty-three numeric/IEEE pairs"
}
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $field = $numericFields[$index]
    $escapedField = [regex]::Escape($field)
    $adjacentPair = '(?m)^\s*"' + $escapedField + '"\s*:\s*json_number\s*\(\s*snapshot\.' + $escapedField + '\s*\)\s*,\s*\r?\n\s*"' + $escapedField + '_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.' + $escapedField + '\s*\)'
    if ($jsonNumbers[$index].Groups['field'].Value -cne $field -or
        $jsonNumbers[$index].Groups['value'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['field'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['value'].Value -cne $field -or
        $serializationText -notmatch $adjacentPair) {
        throw "CP400 JSON numeric/IEEE sidecar order drift at $field"
    }
}
foreach ($pattern in @(
        'predecessor_cp399\s*:\s*Option<&PredecessorLifecycle>',
        'flow_owner_cp330\s*:\s*Option<&FlowOwnerLifecycle>',
        'mixed_owner_cp329\s*:\s*Option<&MixedOwnerLifecycle>',
        'CP399 latest evidence is missing',
        'CP330 latest owner is missing',
        'CP329 latest owner is missing',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)'
    )) { Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline predecessor/owner/public-route contract" }
Assert-Contains -Path $pipelineLineage -Pattern 'supply_mass_flow_rate_times_cp_air_w_per_k' -Description "pipeline first-product evidence"
Assert-Contains -Path $pipelineLineage -Pattern 'mixed_air_minus_supply_temperature_k' -Description "pipeline temperature-difference evidence"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp412_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle"
) -Description "pipeline CP399-to-CP400 validation order"
Assert-Contains -Path $coupledTests -Pattern 'cp400' -Description "coupled regressions"
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp400_after_cp399_before_unchanged_numerical_coupling' -Description "binding execution/nonfeed regression"
Assert-Contains -Path $coupledFixture -Pattern 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment' -Description "coupled output fixture"
Assert-Contains -Path $witness -Pattern 'set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_latest_witness' -Description "runtime witness setter"
Assert-Contains -Path $pipelineValidationTests -Pattern 'public_cp400_validator_requires_cp399_cp330_and_cp329' -Description "pipeline owner regressions"
Assert-Contains -Path $snapshotJsonTests -Pattern 'ieee' -Description "snapshot IEEE regressions"
Assert-Contains -Path $arbitraryAssertions -Pattern 'route\(20\)\s*\+\s*route\(24\)' -Description "public active-route regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'ends_with\("_ieee_bits"\)' -Description "exact IEEE-sidecar regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'non-direct runtime must not publish CP400 evidence' -Description "non-direct regression"

foreach ($doc in @(
        'docs\src\current\current-status.md', 'docs\src\current\project-contract.md',
        'docs\src\porting-map\heat-balance-source-map.md',
        'docs\src\porting-map\ideal-loads-source-map.md',
        'docs\src\porting-map\zone-air-update-map.md'
    )) { Assert-Contains -Path $doc -Pattern 'CP400 post-saturation shared-case sensible-output assignment' -Description "CP400 documentation" }
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP400 supersedes only CP399' -Description "algorithm claim"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP400 additionally requires' -Description "capability claim"

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$cp399Index = $master.IndexOf('cp399-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-cp-air-assignment.ps1')
$cp400Index = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp399Index -lt 0 -or $cp400Index -le $cp399Index -or $completionIndex -le $cp400Index) {
    throw "Master CP400 registration order drift"
}
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 350', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp400Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 110) {
    throw "CP400 inventory classification drift; expected 240 public and 110 internal"
}
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 350 \|' -Description "generated script total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 110 \|' -Description "generated internal total"

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
Write-Host "CP400 post-saturation shared-case sensible-output-assignment structure audit passed."
}
