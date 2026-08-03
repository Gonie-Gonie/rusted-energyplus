# CP399 maps PurchasedAirManager.cc physical executable line 2294 only.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignment"
$pipelineStem = "purchased_air_$stem"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$routes = "$root\transition\routes.rs"
$accounting = "$root\transition\accounting.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$prefix = "$root\release\prefix_validation.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp399.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$pipelineSerialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$arbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp399_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp399-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-cp-air-assignment.ps1"
$siteRead = "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cp-air"
$siteEvaluate = "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cp-air"
$siteAssign = "assign-local-cp-air-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-case"
$numericFields = @(
    "predecessor_cp397_resulting_supply_humidity_ratio",
    "predecessor_cp397_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp397_resulting_supply_temperature_c",
    "predecessor_cp398_resulting_supply_humidity_ratio",
    "predecessor_cp398_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp398_resulting_supply_temperature_c",
    "mixed_air_humidity_ratio",
    "psychrometric_cp_air_result_j_per_kg_k",
    "cp_air_j_per_kg_k",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)
$localBools = @(
    "dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed",
    "mixed_air_humidity_ratio_read",
    "psychrometric_cp_air_evaluated",
    "cp_air_assigned"
)

function Assert-Cp399Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP399 $Description missing '$Pattern'" }
}

function Get-Cp399BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP399 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP399 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP399 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $accounting, $tests, $release, $prefix,
    $runtimeValidation, $snapshotValidation, $adapter, $coupled, $coupledTests,
    $pipeline, $pipelineValidation, $pipelineLineage, $pipelineSerialization, $snapshotJson,
    $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP399 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP399 bounded file"
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP399 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2290].Trim() -cne 'case HumControl::ConstantSupplyHumidityRatio: {' -or
    $lines[2293].Trim() -cne 'CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat);' -or
    $lines[2294].Trim() -cne 'CoolSensOutput = SupplyMassFlowRate * CpAir * (PurchAir.MixedAirTemp - PurchAir.SupplyTemp);') {
    throw "CP399 source boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2294' -Description "mapped executable"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2295' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER' `
    -Expected @($siteRead, $siteEvaluate, $siteAssign) -Description "exact three source sites"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp399BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 12) { throw "CP399 snapshot must expose exactly twelve Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP399 numeric field order drift at $index" }
}
Assert-PatternsInOrder -Path $module -Patterns @(
    'pub\s+predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered\s*:\s*bool',
    'pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed\s*:\s*bool',
    'pub\s+mixed_air_humidity_ratio_read\s*:\s*bool',
    'pub\s+mixed_air_humidity_ratio\s*:',
    'pub\s+psychrometric_cp_air_evaluated\s*:\s*bool',
    'pub\s+psychrometric_cp_air_result_j_per_kg_k\s*:',
    'pub\s+cp_air_assigned\s*:\s*bool',
    'pub\s+cp_air_j_per_kg_k\s*:'
) -Description "retained entry and local operation schema"
$localSchemaStart = $snapshotStruct.IndexOf('pub predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: bool')
if ($localSchemaStart -lt 0) { throw "CP399 retained shared-case marker missing from snapshot" }
[string[]]$actualLocalBools = @(
    [regex]::Matches($snapshotStruct.Substring($localSchemaStart), 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*bool') |
        Select-Object -Skip 1 |
        ForEach-Object { $_.Groups['field'].Value }
)
if ($actualLocalBools.Count -ne $localBools.Count) { throw "CP399 snapshot must add exactly four local booleans" }
for ($index = 0; $index -lt $localBools.Count; $index += 1) {
    if ($actualLocalBools[$index] -cne $localBools[$index]) { throw "CP399 local boolean order drift at $index" }
}

$stateText = Read-RepoText -Path $state
foreach ($pattern in @(
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]',
        'dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count',
        'source_site_execution_count', 'mixed_air_humidity_ratio_read_count',
        'psychrometric_cp_air_evaluation_count', 'cp_air_assignment_write_count',
        'pub\(super\)\s+latest_route', 'pub\(super\)\s+latest_transition_ordinal'
    )) { Assert-Cp399Text -Text $stateText -Pattern $pattern -Description "state contract" }

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'energyplus_psy_cp_air_fn_w',
        'mixed_air_humidity_ratio\.is_finite\(\)',
        'mixed_air_humidity_ratio\s*<\s*0\.0',
        'checked_mul\(',
        'state\.predecessor_route_counts\s*==\s*predecessor\.predecessor_route_counts'
    )) { Assert-Cp399Text -Text $core -Pattern $pattern -Description "route/numeric/accounting contract" }
Assert-Contains -Path $accounting -Pattern 'source_site_execution_count\s*\+=\s*PURCHASED_AIR_.*_SOURCE_ORDER\.len\(\)' -Description "three-site accounting"
foreach ($expectation in @(
        @('state\.transition_count\s*,\s*30', 'thirty exhaustive routes'),
        @('state\.inactive_transition_count\s*,\s*24', 'twenty-four inactive routes'),
        @('cp_air_assignment_count\s*,\s*6', 'six active routes'),
        @('state\.source_site_execution_count\s*,\s*18', 'eighteen exhaustive site executions')
    )) { Assert-Contains -Path $tests -Pattern $expectation[0] -Description $expectation[1] }
Assert-Contains -Path $transition -Pattern 'predecessor_cp398_resulting_supply_humidity_ratio:\s*predecessor\.resulting_supply_humidity_ratio' -Description "CP398 carrier rename"
Assert-Contains -Path $transition -Pattern 'resulting_supply_temperature_c:\s*predecessor\.resulting_supply_temperature_c' -Description "terminal carrier preservation"
Assert-Contains -Path $transition -Pattern 'let\s+cp_air_j_per_kg_k\s*=\s*energyplus_psy_cp_air_fn_w\(mixed_air_humidity_ratio\)' -Description "canonical stateless CpAir evaluation"

Assert-Contains -Path $release -Pattern 'ConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor' -Description "exact CP398 predecessor"
Assert-Contains -Path $prefix -Pattern 'calc_cooling_mixed_air_call\.latest' -Description "CP329 authoritative owner"
Assert-Contains -Path $prefix -Pattern 'cooling_mixed_air_call_latest_witness' -Description "CP329 owner witness"
Assert-Contains -Path $prefix -Pattern 'completed_direct_cooling_mixed_air_call_is_consistent' -Description "recursive CP329 completion"
Assert-NotContains -Path $release -Pattern 'DirectZonePurchasedAirCouplingInput|supply_humidity_ratio_c:\s*f64|zone_humidity|CP331|CP338|CP387' -Description "forbidden scalar substitutes"
Assert-Contains -Path $snapshotValidation -Pattern 'matches!\(route\.predecessor_index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description "exact public direct routes"
Assert-Contains -Path $snapshotValidation -Pattern 'route\.active\s*==\s*matches!\(route\.predecessor_index,\s*20\s*\|\s*24\)' -Description "exact public active routes"

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\s*=',
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
) -Description "CP398-to-CP399-to-CP400-to-CP401-to-CP402-to-CP403-to-CP404-to-CP405-to-CP406-to-CP407-to-CP408 binding order"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\s*:',
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
$bindingEvidenceName = 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment'
if ([regex]::Matches($bindingText, "\b$bindingEvidenceName\b").Count -ne 3) {
    throw "CP399 binding evidence must be produced once, consumed by CP400 once, and stored once without feeding numerical coupling"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}

$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 12 -or $ieeeSidecars.Count -ne 12) {
    throw "CP399 JSON snapshot must expose exactly twelve IEEE sidecars"
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
        throw "CP399 JSON numeric/IEEE sidecar order drift at $field"
    }
}
foreach ($pattern in @(
        'predecessor_cp398\s*:\s*Option<&PredecessorLifecycle>',
        'owner_cp329\s*:\s*Option<&OwnerLifecycle>',
        'CP398 latest evidence is missing',
        'CP329 latest owner is missing',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)'
    )) { Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline predecessor/owner/public-route contract" }
Assert-Contains -Path $pipelineLineage -Pattern 'energyplus_psy_cp_air_fn_w\(humidity_ratio\)' -Description "pipeline canonical CpAir corroboration"
Assert-Contains -Path $pipelineLineage -Pattern 'owner\.mixed_air_humidity_ratio' -Description "pipeline CP329 operand ownership"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp410_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle"
) -Description "pipeline CP398-to-CP399 validation order"
Assert-Contains -Path $coupledTests -Pattern 'cp399' -Description "coupled regressions"
Assert-Contains -Path $arbitraryAssertions -Pattern 'route\(20\)\s*\+\s*route\(24\)' -Description "public active-route regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'CP399 must read the CP329-owned mixed-air humidity ratio' -Description "CP329 owner-bit regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'ends_with\("_ieee_bits"\)' -Description "exact IEEE-sidecar regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'non-direct runtime must not publish CP399 evidence' -Description "non-direct regression"

foreach ($doc in @(
        'docs\src\current\current-status.md', 'docs\src\current\project-contract.md',
        'docs\src\porting-map\heat-balance-source-map.md',
        'docs\src\porting-map\ideal-loads-source-map.md',
        'docs\src\porting-map\zone-air-update-map.md'
    )) { Assert-Contains -Path $doc -Pattern 'CP399 post-saturation shared-case' -Description "CP399 documentation" }
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP399 supersedes only CP398' -Description "algorithm claim"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP399 additionally requires' -Description "capability claim"

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$cp398Index = $master.IndexOf('cp398-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-case-entry.ps1')
$cp399Index = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp398Index -lt 0 -or $cp399Index -le $cp398Index -or $completionIndex -le $cp399Index) {
    throw "Master CP399 registration order drift"
}
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 348', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp399Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 108) {
    throw "CP399 inventory classification drift; expected 240 public and 106 internal"
}
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 348 \|' -Description "generated script total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 108 \|' -Description "generated internal total"

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP404-to-CP405' -Description "CP345 CP404-to-CP405 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP405-to-CP406' -Description "CP345 CP405-to-CP406 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP406-to-CP407' -Description "CP345 CP406-to-CP407 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP407-to-CP408' -Description "CP345 CP407-to-CP408 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP408-to-CP409' -Description "CP345 CP408-to-CP409 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\binding.rs" -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\s*=' -Description "CP410 historical binding order"
Write-Host "CP399 post-saturation shared-case CpAir-assignment structure audit passed."
}
