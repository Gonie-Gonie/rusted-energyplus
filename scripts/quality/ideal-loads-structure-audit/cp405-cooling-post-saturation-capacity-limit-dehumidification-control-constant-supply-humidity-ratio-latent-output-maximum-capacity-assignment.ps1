# CP405 maps PurchasedAirManager.cc physical executable line 2300 only and
# stops before line 2301's else boundary and line 2302's next executable.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignment"
$pipelineStem = "purchased_air_$stem"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$predecessorModule = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$owners = "$root\transition\owners.rs"
$routes = "$root\transition\routes.rs"
$transitionSnapshot = "$root\transition\snapshot.rs"
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
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp405.rs"
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
$arbitraryPredecessor = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp404_assertions.rs"
$arbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp405_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp405-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-maximum-capacity-assignment.ps1"
$sites = @(
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-assignment",
    "assign-local-cooling-latent-output-from-maximum-total-cooling-capacity"
)

function Assert-Cp405Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP405 $Description missing '$Pattern'" }
}

function Get-Cp405BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP405 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP405 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP405 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $accounting, $owners, $routes,
    $transitionSnapshot, $tests, $release, $privateCharacterization,
    $runtimeValidation, $snapshotValidation, $adapter, $adapterTests, $coupled,
    $coupledSnapshot, $coupledTests, $coupledFixture, $witness, $pipeline,
    $pipelineValidation, $pipelineLineage, $pipelineValidationTests,
    $pipelineSerialization, $snapshotJson, $snapshotJsonTests,
    $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP405 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP405 bounded file"
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP405 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2299].Trim() -cne 'CoolLatOutput = PurchAir.MaxCoolTotCap;' -or
    $lines[2300].Trim() -cne '} else {' -or
    $lines[2301].Trim() -cne 'PurchAir.SupplyTemp = PsyTdbFnHW(SupplyEnthalpy, PurchAir.SupplyHumRat);') {
    throw "CP405 source and first-exclusion boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2300' -Description "mapped executable"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2302' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites -Description "exact two source sites"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp405BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$fields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:') | ForEach-Object { $_.Groups['field'].Value })
[string[]]$numericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($fields.Count -ne 161) { throw "CP405 snapshot must expose exactly 161 fields" }
if ($numericFields.Count -ne 54) { throw "CP405 snapshot must expose exactly fifty-four Option<f64> fields" }
$predecessorText = Read-RepoText -Path $predecessorModule
$predecessorSnapshot = Get-Cp405BraceBlock -Text $predecessorText `
    -AnchorPattern 'pub\s+struct\s+PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot\s*' `
    -Description "CP404 predecessor snapshot"
if ([regex]::Matches($predecessorSnapshot, 'pub\s+[A-Za-z0-9_]+\s*:').Count -ne 147) {
    throw "CP404 predecessor snapshot must remain exactly 147 fields"
}
$expectedLocalSuffix = @(
    'dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed',
    'cp404_retained_supply_humidity_ratio_state_owned',
    'cp404_retained_supply_temperature_state_owned',
    'cp404_retained_supply_enthalpy_state_owned',
    'preexisting_cooling_latent_output_w',
    'cp404_retained_maximum_total_cooling_capacity_owned_read',
    'maximum_total_cooling_capacity_read',
    'maximum_total_cooling_capacity_w',
    'cooling_latent_output_assigned',
    'assigned_cooling_latent_output_w',
    'resulting_cooling_latent_output_w',
    'resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c'
)
for ($index = 0; $index -lt $expectedLocalSuffix.Count; $index += 1) {
    if ($fields[147 + $index] -cne $expectedLocalSuffix[$index]) {
        throw "CP405 local field suffix order drift at $index"
    }
}
$expectedNumericSuffix = @(
    'preexisting_cooling_latent_output_w', 'maximum_total_cooling_capacity_w',
    'assigned_cooling_latent_output_w', 'resulting_cooling_latent_output_w',
    'resulting_supply_humidity_ratio', 'resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c'
)
for ($index = 0; $index -lt $expectedNumericSuffix.Count; $index += 1) {
    if ($numericFields[47 + $index] -cne $expectedNumericSuffix[$index]) {
        throw "CP405 numeric suffix order drift at $index"
    }
}

Assert-PatternsInOrder -Path $state -Patterns @(
    'pub\s+transition_count\s*:\s*usize',
    'pub\s+inactive_transition_count\s*:\s*usize',
    'pub\s+predecessor_guard_false_fallthrough_count\s*:\s*usize',
    'pub\s+cooling_latent_output_maximum_capacity_assignment_count\s*:\s*usize',
    'pub\s+predecessor_route_counts\s*:\s*\[usize;\s*30\]',
    'pub\s+predecessor_guard_false_fallthrough_route_counts\s*:\s*\[usize;\s*30\]',
    'pub\s+cooling_latent_output_maximum_capacity_assignment_route_counts\s*:\s*\[usize;\s*30\]',
    'pub\s+source_site_execution_count\s*:\s*usize',
    'pub\s+cp404_supply_humidity_ratio_state_owner_count\s*:\s*usize',
    'pub\s+unchanged_supply_humidity_ratio_preservation_count\s*:\s*usize',
    'pub\s+cp404_supply_enthalpy_state_owner_count\s*:\s*usize',
    'pub\s+unchanged_supply_enthalpy_preservation_count\s*:\s*usize',
    'pub\s+cp404_supply_temperature_state_owner_count\s*:\s*usize',
    'pub\s+unchanged_supply_temperature_preservation_count\s*:\s*usize',
    'pub\s+cp404_retained_maximum_total_cooling_capacity_owned_read_count\s*:\s*usize',
    'pub\s+maximum_total_cooling_capacity_read_count\s*:\s*usize',
    'pub\s+cooling_latent_output_assignment_write_count\s*:\s*usize',
    'pub\s+latest\s*:', 'latest_route\s*:', 'latest_transition_ordinal\s*:'
) -Description "persistent accounting schema"
$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'state\.source_site_execution_count\s*\+=\s*SOURCE_ORDER\.len\(\)',
        'predecessor_cp321_maximum_total_cooling_capacity_owned_read',
        'predecessor_cp340_same_call_maximum_total_cooling_capacity_bit_corroborated',
        'predecessor_maximum_total_cooling_capacity_w\?',
        'assigned\.to_bits\(\)\s*==\s*super::source_assignment\(maximum\)\.to_bits\(\)',
        'resulting\.to_bits\(\)\s*==\s*assigned\.to_bits\(\)',
        'option_bits_match\(\s*snapshot\.resulting_supply_humidity_ratio',
        'option_bits_match\(\s*snapshot\.resulting_supply_enthalpy_j_per_kg',
        'option_bits_match\(\s*snapshot\.resulting_supply_temperature_c'
    )) { Assert-Cp405Text -Text $core -Pattern $pattern -Description "route, provenance, raw-copy, or W/H/T contract" }
Assert-Contains -Path $transition -Pattern 'const\s+fn\s+source_assignment\s*\(maximum_total_cooling_capacity_w:\s*f64\)\s*->\s*f64\s*\{\s*maximum_total_cooling_capacity_w\s*\}' -Description "raw assignment identity"
foreach ($forbidden in @('Psy|psychrometric', 'mul_add\s*\(', '\.max\s*\(', '\.min\s*\(', 'clamp\s*\(', 'is_finite\s*\(', 'DirectZonePurchasedAirCouplingInput')) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "forbidden arithmetic, psychrometrics, finite gate, or numerical coupling"
}
$activeIndices = @(20, 21, 24, 25, 27, 29)
$publicPredecessors = @(0..8) + @(20, 24)
if ((36 - $activeIndices.Count) -ne 30 -or (30 - $activeIndices.Count) -ne 24 -or
    ($publicPredecessors.Count + 2) -ne 13 -or (36 - 13) -ne 23 -or
    (2 * $activeIndices.Count) -ne 12) { throw "CP405 logical route/accounting constants drift" }
foreach ($pattern in @('routes\.len\(\)\s*,\s*36', '\[20,\s*21,\s*24,\s*25,\s*27,\s*29\]', '\.count\(\)\s*,\s*13')) {
    Assert-Contains -Path $tests -Pattern $pattern -Description "exhaustive/public route characterization"
}
Assert-Contains -Path $transition -Pattern 'SupplyHumidityRatioAssignmentSnapshot\s+as\s+Predecessor' -Description "exact CP404 predecessor"
Assert-Contains -Path $release -Pattern 'completed_direct_.*latent_output_supply_humidity_ratio_assignment_is_consistent' -Description "recursive CP404 completion"
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp404\s*:\s*Option<&PredecessorLifecycle>' -Description "pipeline sole predecessor"
Assert-Contains -Path $privateCharacterization -Pattern "private_$stem`_characterization" -Description "private route characterization"

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*=', 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*=',
    'let\s+unit_available\s*=', 'let\s+coupling\s*='
) -Description "CP404-to-CP405-to-CP406-to-CP407-to-CP408-to-CP414-to-CP415 binding order"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\s*:',
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*:', 'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*:',
    'pub\s+coupling\s*:'
) -Description "scheduled output order"
$bindingText = Read-RepoText -Path $binding
$predecessorEvidence = "calculation_$predecessorStem"
$bindingEvidence = "calculation_$stem"
if ([regex]::Matches($bindingText, "\b$predecessorEvidence\b").Count -ne 3 -or
    [regex]::Matches($bindingText, "\b$bindingEvidence\b").Count -ne 3) {
    throw "CP405 binding must consume CP404 once, feed CP406 once, publish CP405 once, and keep both outside numerical coupling"
}
$dto = Get-Cp405BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description "numerical coupling DTO"
if ($dto -match '(?i)cp405|latent_output_maximum_capacity_assignment') {
    throw "CP405 evidence must not feed DirectZonePurchasedAirCouplingInput"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp405_after_cp404_before_unchanged_numerical_coupling' -Description "binding execution/nonfeed regression"
Assert-Contains -Path $coupledTests -Pattern 'cp405' -Description "coupled CP405 regression"
Assert-Contains -Path $coupledFixture -Pattern $bindingEvidence -Description "coupled output fixture"
Assert-Contains -Path $witness -Pattern "set_$stem`_latest_witness" -Description "runtime witness setter"

$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 54 -or $ieeeSidecars.Count -ne 54) {
    throw "CP405 JSON snapshot must expose exactly fifty-four numeric/IEEE pairs"
}
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $field = $numericFields[$index]
    $escaped = [regex]::Escape($field)
    $adjacent = '(?m)^\s*"' + $escaped + '"\s*:\s*json_number\s*\(\s*snapshot\.' + $escaped + '\s*\)\s*,\s*\r?\n\s*"' + $escaped + '_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.' + $escaped + '\s*\)'
    if ($jsonNumbers[$index].Groups['field'].Value -cne $field -or
        $jsonNumbers[$index].Groups['value'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['field'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['value'].Value -cne $field -or
        $serializationText -notmatch $adjacent) { throw "CP405 JSON numeric/IEEE sidecar order drift at $field" }
}
Assert-Contains -Path $snapshotJsonTests -Pattern 'object\.len\(\),\s*215' -Description "161-field plus 54-sidecar JSON shape"
Assert-Contains -Path $snapshotJsonTests -Pattern 'ends_with\("_ieee_bits"\).*54|\)\s*,\s*54' -Description "exact IEEE sidecar count regression"

Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle"
) -Description "pipeline CP404-to-CP405 validation order"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp432_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineLineage -Pattern 'predecessor_cp321_maximum_total_cooling_capacity_owned_read' -Description "recursive CP321 capacity ownership"
Assert-Contains -Path $pipelineLineage -Pattern 'predecessor_cp340_same_call_maximum_total_cooling_capacity_bit_corroborated' -Description "recursive CP340 capacity corroboration"
Assert-Contains -Path $pipelineLineage -Pattern 'assigned_cooling_latent_output_w,\s*Some\(maximum\)' -Description "raw assigned maximum payload"
Assert-Contains -Path $pipelineLineage -Pattern 'resulting_cooling_latent_output_w,\s*Some\(maximum\)' -Description "raw resulting maximum payload"
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp405_assertions' -Description "CP404-to-CP405 arbitrary delegation module"
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp405_assertions::assert_direct\(runtime,\s*results\)' -Description "CP405 direct arbitrary delegation"
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp405_assertions::assert_non_direct\(runtime\)' -Description "CP405 non-direct arbitrary delegation"
foreach ($pattern in @(
        'PurchasedAirManager\.cc:2300', 'PurchasedAirManager\.cc:2302',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'non-direct runtime must not publish CP405 evidence',
        'ends_with\("_ieee_bits"\)'
    )) { Assert-Contains -Path $arbitraryAssertions -Pattern $pattern -Description "arbitrary runtime contract" }

$heading = 'CP405 post-saturation shared-case latent-output body maximum-capacity assignment'
$docs = @(
    'docs\src\current\current-status.md', 'docs\src\current\project-contract.md',
    'docs\src\porting-map\heat-balance-source-map.md',
    'docs\src\porting-map\ideal-loads-source-map.md',
    'docs\src\porting-map\zone-air-update-map.md'
)
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText, "(?m)^## $([regex]::Escape($heading))$").Count -ne 1) {
        throw "CP405 documentation heading must appear exactly once in $doc"
    }
    Assert-Cp405Text -Text $docText -Pattern '(?s)(?:Physical )?[Ll]ine 2301.*?first excluded lexical/control.*?CP406 candidate' -Description "CP406 lexical/control boundary documentation"
    Assert-Cp405Text -Text $docText -Pattern '(?s)[Ll]ine 2302.*?first excluded executable' -Description "first excluded executable documentation"
}
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP405 supersedes only CP404' -Description "algorithm claim"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP405 additionally requires' -Description "capability claim"
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP405 supersedes only CP404' -Description "generated algorithm claim"
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP405 additionally requires' -Description "generated capability claim"
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern 'CP405' -Description "Roadmap non-promotion"
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP405\b' -Description "psychrometrics-map non-promotion"

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
foreach ($file in @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 404) {
        Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp432_lifecycle_evidence' -Description "historical non-direct firewall"
    }
    if ($number -ge 337 -and $number -le 404) {
        Assert-Contains -Path $file.FullName -Pattern 'script_count = 370' -Description "historical current script count"
    }
    if ($number -ge 335 -and $number -le 404) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 370 \|')) -Description "historical generated script count"
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 130 \|')) -Description "historical generated internal count"
    }
    if ($number -ge 367 -and $number -le 404) {
        Assert-Contains -Path $file.FullName -Pattern 'Count -ne 130' -Description "historical internal classification count"
    }
}
$cleanupAudits = @(
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp326-*.ps1' -File
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp3*.ps1' -File | Where-Object {
        $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344
    }
)
if ($cleanupAudits.Count -ne 17) { throw "CP405 cleanup propagation file set drift" }
foreach ($file in $cleanupAudits) {
    Assert-Contains -Path $file.FullName -Pattern 'calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment' -Description "historical CP405 helper whitelist"
}
$terminalAudits = @(
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp345-*.ps1' -File
    Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File | Where-Object {
        $_.BaseName -match '^cp(?<number>\d+)-' -and
        (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or
         ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 402))
    }
)
if ($terminalAudits.Count -ne 26) { throw "CP405 terminal-order propagation file set drift" }
foreach ($file in $terminalAudits) {
    Assert-Contains -Path $file.FullName -Pattern 'CP404-to-CP405' -Description "historical CP404-to-CP405 interval"
}
$cp345Audit = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp405Call\s*=', '\$cp409Call\s*=', 'CP404-to-CP405', 'CP408-to-CP409', 'CP414-to-CP415')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "CP345 terminal chain"
}
foreach ($file in @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404)-' })) {
    Assert-Contains -Path $file.FullName -Pattern 'latent_output_maximum_capacity_assignment\\s\*' -Description "recent binding/scheduled CP405 order"
}

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$cp404Index = $master.IndexOf('cp404-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-humidity-ratio-assignment.ps1')
$cp405Index = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp404Index -lt 0 -or $cp405Index -le $cp404Index -or $completionIndex -le $cp405Index) {
    throw "Master CP405 registration order drift"
}
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 370', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp405Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 130) {
    throw "CP405 inventory classification drift; expected 240 public and 122 internal"
}
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp405-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-maximum-capacity-assignment\.ps1' -Description "inventory record"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 370 \|' -Description "generated script total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description "generated public total"
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 130 \|' -Description "generated internal total"

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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP405 post-saturation shared-case latent-output body maximum-capacity assignment structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-numerical' -Description 'CP432-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-numerical' -Description 'CP432-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-numerical' -Description 'CP432-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-numerical' -Description 'CP432-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-numerical' -Description 'CP432-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[1]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-numerical' -Description 'CP432-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[1]-to-numerical' -Description 'stale CP431 numerical interval'
