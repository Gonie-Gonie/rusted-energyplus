# CP401 maps PurchasedAirManager.cc physical executable line 2296 only.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignment"
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
$error = "$root\release\error.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp401.rs"
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
$arbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp401_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp401-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-assignment.ps1"
$sites = @(
    "read-retained-cooling-total-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-difference",
    "read-local-cooling-sensible-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-difference",
    "calculate-cooling-total-output-minus-cooling-sensible-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output",
    "assign-local-cooling-latent-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-case"
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
    "cooling_total_output_w",
    "cooling_sensible_output_w",
    "calculated_cooling_latent_output_w",
    "cooling_latent_output_w",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)

function Assert-Cp401Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP401 $Description missing '$Pattern'" }
}

function Get-Cp401BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP401 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP401 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP401 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $owners, $accounting, $tests, $release,
    $prefix, $runtimeValidation, $snapshotValidation, $privateCharacterization, $error,
    $adapter, $adapterTests, $coupled, $coupledTests, $coupledFixture, $witness,
    $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineLineage,
    $pipelineSerialization, $snapshotJson, $snapshotJsonTests, $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP401 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP401 bounded file"
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP401 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2295].Trim() -cne 'CoolLatOutput = CoolTotOutput - CoolSensOutput;' -or
    $lines[2296].Trim() -cne 'if (CoolLatOutput >= PurchAir.MaxCoolTotCap) {') {
    throw "CP401 source boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2296' -Description "mapped executable"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2297' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites -Description "exact four source sites"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp401BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 30) { throw "CP401 snapshot must expose exactly thirty Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP401 numeric field order drift at $index" }
}
Assert-PatternsInOrder -Path $module -Patterns @(
    'pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed\s*:\s*bool',
    'pub\s+cp384_retained_cooling_total_output_owned_read\s*:\s*bool',
    'pub\s+cp385_cooling_total_output_bit_corroborated\s*:\s*bool',
    'pub\s+cooling_total_output_read\s*:\s*bool',
    'pub\s+cp400_retained_cooling_sensible_output_owned_read\s*:\s*bool',
    'pub\s+cooling_sensible_output_read\s*:\s*bool',
    'pub\s+cooling_latent_output_calculated\s*:\s*bool',
    'pub\s+cooling_latent_output_assigned\s*:\s*bool'
) -Description "owner and four-site operation schema"

$stateText = Read-RepoText -Path $state
foreach ($pattern in @(
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]',
        'dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count',
        'source_site_execution_count',
        'cp400_supply_humidity_ratio_state_owner_count',
        'cp400_supply_enthalpy_state_owner_count',
        'cp400_supply_temperature_state_owner_count',
        'cooling_total_output_owned_read_count',
        'cooling_total_output_bit_corroboration_count',
        'cooling_total_output_read_count',
        'cooling_sensible_output_owned_read_count',
        'cooling_sensible_output_read_count',
        'cooling_latent_output_calculation_count',
        'cooling_latent_output_assignment_write_count',
        'pub\(super\)\s+latest_route', 'pub\(super\)\s+latest_transition_ordinal'
    )) { Assert-Cp401Text -Text $stateText -Pattern $pattern -Description "state contract" }

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'let\s+total\s*=\s*active\.cooling_total_output_w\s*;\s*let\s+sensible\s*=\s*active\.cooling_sensible_output_w\s*;\s*let\s+output\s*=\s*total\s*-\s*sensible',
        'cooling_total_output_from_exact_owner',
        'state\.predecessor_route_counts\s*==\s*predecessor\.predecessor_route_counts'
    )) { Assert-Cp401Text -Text $core -Pattern $pattern -Description "route/arithmetic/accounting contract" }
Assert-Contains -Path $accounting -Pattern 'source_site_execution_count\s*\+=\s*PURCHASED_AIR_.*_SOURCE_ORDER\.len\(\)' -Description "four-site accounting"
foreach ($forbidden in @('mul_add\s*\(', '\.max\s*\(', '\.abs\s*\(', 'DirectZonePurchasedAirCouplingInput')) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "forbidden arithmetic/numerical coupling"
}
foreach ($expectation in @(
        @('state\.transition_count\s*,\s*30', 'thirty exhaustive routes'),
        @('state\.inactive_transition_count\s*,\s*24', 'twenty-four inactive routes'),
        @('latent_output_assignment_count\s*,\s*6', 'six active routes'),
        @('state\.source_site_execution_count\s*,\s*24', 'twenty-four exhaustive site executions')
    )) { Assert-Contains -Path $tests -Pattern $expectation[0] -Description $expectation[1] }

Assert-Contains -Path $release -Pattern 'SensibleOutputAssignmentSnapshot as Predecessor' -Description "exact CP400 predecessor"
Assert-Contains -Path $prefix -Pattern 'calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment;' -Description "CP400 immediate predecessor state"
Assert-Contains -Path $prefix -Pattern 'let\s+Some\(latest\)\s*=\s*state\.latest' -Description "CP400 latest evidence"
Assert-Contains -Path $prefix -Pattern 'completed_direct_.*sensible_output_assignment_is_consistent' -Description "recursive CP400 completion"
Assert-Contains -Path $owners -Pattern 'TotalOutputMaximumCapacityAssignmentSnapshot as Owner' -Description "CP384 authoritative owner"
Assert-Contains -Path $owners -Pattern 'TotalOutputSupplyEnthalpyAssignmentSnapshot as Corroborator' -Description "CP385 corroborator"
Assert-Contains -Path $owners -Pattern 'owner\.resulting_cooling_total_output_w' -Description "CP384 retained total output"
Assert-Contains -Path $owners -Pattern 'corroborator\.cooling_total_output_w' -Description "CP385 total-output bit corroboration"
Assert-Contains -Path $transition -Pattern 'predecessor_cp_air_j_per_kg_k\s*:\s*predecessor\s*\.predecessor_cp_air_j_per_kg_k' -Description "inherited CP399 CpAir lineage slot"
Assert-Contains -Path $transition -Pattern 'predecessor_cp400_cp_air_j_per_kg_k\s*:\s*predecessor\.cp_air_j_per_kg_k' -Description "CP400 local CpAir lineage slot"
Assert-Contains -Path $snapshotValidation -Pattern 'matches!\(route\.predecessor_index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description "exact public direct routes"
Assert-Contains -Path $snapshotValidation -Pattern 'route\.active\s*==\s*matches!\(route\.predecessor_index,\s*20\s*\|\s*24\)' -Description "exact public active routes"
Assert-Contains -Path $privateCharacterization -Pattern 'private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_characterization' -Description "private route characterization"

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\s*=',
    'let\s+unit_available\s*=',
    'let\s+coupling\s*='
) -Description "CP400-to-CP401-to-numerical binding order"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\s*:',
    'pub\s+coupling\s*:'
) -Description "scheduled output order"
$bindingText = Read-RepoText -Path $binding
$bindingEvidenceName = 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment'
if ([regex]::Matches($bindingText, "\b$bindingEvidenceName\b").Count -ne 2) {
    throw "CP401 binding evidence must be produced once and stored once without feeding numerical coupling"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}

$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 30 -or $ieeeSidecars.Count -ne 30) {
    throw "CP401 JSON snapshot must expose exactly thirty numeric/IEEE pairs"
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
        throw "CP401 JSON numeric/IEEE sidecar order drift at $field"
    }
}
foreach ($pattern in @(
        'predecessor_cp400\s*:\s*Option<&PredecessorLifecycle>',
        'total_output_owner_cp384\s*:\s*Option<&OwnerLifecycle>',
        'total_output_corroborator_cp385\s*:\s*Option<&CorroboratorLifecycle>',
        'CP400 latest evidence is missing',
        'CP384 latest owner is missing',
        'CP385 latest corroborator is missing',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)'
    )) { Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline predecessor/owner/public-route contract" }
Assert-Contains -Path $pipelineLineage -Pattern 'let\s+latent\s*=\s*total\s*-\s*sensible' -Description "pipeline exact subtraction"
Assert-Contains -Path $pipelineLineage -Pattern 'owner\.resulting_cooling_total_output_w' -Description "pipeline CP384 total-output owner"
Assert-Contains -Path $pipelineLineage -Pattern 'corroborator\.cooling_total_output_w' -Description "pipeline CP385 bit corroboration"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp401_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle"
) -Description "pipeline CP400-to-CP401 validation order"
Assert-Contains -Path $coupledTests -Pattern 'cp401' -Description "coupled regressions"
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp401_after_cp400_before_unchanged_numerical_coupling' -Description "binding execution/nonfeed regression"
Assert-Contains -Path $coupledFixture -Pattern 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment' -Description "coupled output fixture"
Assert-Contains -Path $witness -Pattern 'set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_latest_witness' -Description "runtime witness setter"
Assert-Contains -Path $pipelineValidationTests -Pattern 'public_cp401_validator_requires_cp400_cp384_and_cp385' -Description "pipeline owner regressions"
Assert-Contains -Path $snapshotJsonTests -Pattern 'ieee' -Description "snapshot IEEE regressions"
Assert-Contains -Path $arbitraryAssertions -Pattern 'route\(20\)\s*\+\s*route\(24\)' -Description "public active-route regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'ends_with\("_ieee_bits"\)' -Description "exact IEEE-sidecar regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'non-direct runtime must not publish CP401 evidence' -Description "non-direct regression"

foreach ($doc in @(
        'docs\src\current\current-status.md', 'docs\src\current\project-contract.md',
        'docs\src\porting-map\heat-balance-source-map.md',
        'docs\src\porting-map\ideal-loads-source-map.md',
        'docs\src\porting-map\zone-air-update-map.md'
    )) { Assert-Contains -Path $doc -Pattern 'CP401 post-saturation shared-case latent-output assignment' -Description "CP401 documentation" }
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP401 supersedes only CP400' -Description "algorithm claim"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP401 additionally requires' -Description "capability claim"

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$cp400Index = $master.IndexOf('cp400-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-sensible-output-assignment.ps1')
$cp401Index = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp400Index -lt 0 -or $cp401Index -le $cp400Index -or $completionIndex -le $cp401Index) {
    throw "Master CP401 registration order drift"
}
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 339', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp401Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 99) {
    throw "CP401 inventory classification drift"
}
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| executable script records \| 339 \|' -Description "generated script total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| internal scripts \| 99 \|' -Description "generated internal total"

Write-Host "CP401 post-saturation shared-case latent-output-assignment structure audit passed."
}
