# CP404 maps PurchasedAirManager.cc physical executable line 2299 only and
# stops before line 2300's cooling-latent-output maximum-capacity overwrite.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment"
$successorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignment"
$pipelineStem = "purchased_air_$stem"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation\snapshot.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp404.rs"
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$psychrometricTests = "crates\ep_runtime\src\psychrometrics_humidity_ratio_tests.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$pipelineValidationTests = "crates\ep_run\src\pipeline\$pipelineStem\validation\tests.rs"
$pipelineSerialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp404_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp404-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-humidity-ratio-assignment.ps1"
$sites = @(
    "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment",
    "assign-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment"
)

function Assert-Cp404Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP404 $Description missing '$Pattern'" }
}

function Get-Cp404BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP404 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP404 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP404 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $accounting, $tests, $release,
    $adapter, $adapterTests, $coupled, $coupledSnapshot, $coupledTests,
    $coupledFixture, $witness, $pipeline, $pipelineValidation, $pipelineLineage,
    $pipelineValidationTests, $pipelineSerialization, $snapshotJson,
    $snapshotJsonTests, $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP404 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP404 bounded file"
}
foreach ($file in @($psychrometrics, $psychrometricTests)) {
    Assert-FileExists -Path $file -Description "CP404 canonical psychrometric evidence file"
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP404 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2298].Trim() -cne 'PurchAir.SupplyHumRat = PsyWFnTdbH(state, PurchAir.SupplyTemp, SupplyEnthalpy, RoutineName);' -or
    $lines[2299].Trim() -cne 'CoolLatOutput = PurchAir.MaxCoolTotCap;') {
    throw "CP404 source boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2299' -Description "mapped executable"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2300' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites -Description "exact four source sites"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp404BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$numericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($numericFields.Count -ne 47) { throw "CP404 snapshot must expose exactly forty-seven Option<f64> fields" }
$expectedSuffix = @(
    'supply_temperature_c', 'supply_enthalpy_j_per_kg',
    'psychrometric_supply_humidity_ratio', 'assigned_supply_humidity_ratio',
    'resulting_supply_humidity_ratio', 'resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c'
)
for ($index = 0; $index -lt $expectedSuffix.Count; $index += 1) {
    if ($numericFields[40 + $index] -cne $expectedSuffix[$index]) {
        throw "CP404 numeric suffix order drift at $index"
    }
}
Assert-PatternsInOrder -Path $module -Patterns @(
    'pub\s+predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed\s*:\s*bool',
    'pub\s+predecessor_cp403_cp329_retained_mixed_air_temperature_owned_read\s*:\s*bool',
    'pub\s+predecessor_cp402_same_call_mixed_air_temperature_bit_corroborated\s*:\s*bool',
    'pub\s+predecessor_cp403_mixed_air_temperature_read\s*:\s*bool',
    'pub\s+predecessor_cp403_mixed_air_temperature_c\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+predecessor_supply_temperature_assigned\s*:\s*bool',
    'pub\s+predecessor_cp403_assigned_supply_temperature_c\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+predecessor_cp403_resulting_supply_humidity_ratio\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+predecessor_cp403_resulting_supply_enthalpy_j_per_kg\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+predecessor_cp403_resulting_supply_temperature_c\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+cp403_retained_supply_temperature_owned_read\s*:\s*bool',
    'pub\s+supply_temperature_for_humidity_ratio_inversion_read\s*:\s*bool',
    'pub\s+supply_temperature_c\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+cp403_retained_supply_enthalpy_owned_read\s*:\s*bool',
    'pub\s+supply_enthalpy_for_humidity_ratio_inversion_read\s*:\s*bool',
    'pub\s+supply_enthalpy_j_per_kg\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+psychrometric_supply_humidity_ratio_evaluated\s*:\s*bool',
    'pub\s+psychrometric_supply_humidity_ratio\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+supply_humidity_ratio_assignment_performed\s*:\s*bool',
    'pub\s+assigned_supply_humidity_ratio\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+resulting_supply_humidity_ratio\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+resulting_supply_enthalpy_j_per_kg\s*:\s*Option\s*<\s*f64\s*>',
    'pub\s+resulting_supply_temperature_c\s*:\s*Option\s*<\s*f64\s*>'
) -Description "owner, psychrometric, assignment, and result schema"
Assert-NotContains -Path $module -Pattern 'preexisting_supply_humidity' -Description "forbidden local preexisting humidity field"

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'energyplus_psy_w_fn_tdb_h\(supply_temperature_c,\s*supply_enthalpy_j_per_kg\)',
        'supply_humidity_ratio_assignment_count',
        'source_site_execution_count\s*\+=\s*SOURCE_ORDER\.len\(\)',
        'supply_temperature_for_humidity_ratio_inversion_read_count',
        'cp385_same_call_supply_enthalpy_bit_corroboration_count',
        'supply_enthalpy_for_humidity_ratio_inversion_read_count',
        'psychrometric_supply_humidity_ratio_evaluation_count',
        'supply_humidity_ratio_assignment_write_count'
    )) { Assert-Cp404Text -Text $core -Pattern $pattern -Description "route, psychrometric, or accounting contract" }
foreach ($forbidden in @('mul_add\s*\(', 'DirectZonePurchasedAirCouplingInput', 'is_finite\s*\(')) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "forbidden regrouping, numerical coupling, or finite gate"
}
$activeIndices = @(20, 21, 24, 25, 27, 29)
if ((36 - $activeIndices.Count) -ne 30 -or (4 * $activeIndices.Count) -ne 24 -or (36 - 13) -ne 23) {
    throw "CP404 logical route/accounting constants drift"
}
foreach ($pattern in @('routes\.len\(\)\s*,\s*36', '\[20,\s*21,\s*24,\s*25,\s*27,\s*29\]', 'assert_eq!\(public,\s*13\)')) {
    Assert-Contains -Path $tests -Pattern $pattern -Description "exhaustive/public route characterization"
}
Assert-PatternsInOrder -Path $psychrometrics -Patterns @(
    'enthalpy_j_per_kg\s*-\s*1\.004_84e3\s*\*\s*dry_bulb_c',
    '2\.500_94e6\s*\+\s*1\.858_95e3\s*\*\s*dry_bulb_c',
    'if\s+humidity_ratio\s*<\s*0\.0',
    'ENERGYPLUS_MIN_HUMIDITY_RATIO'
) -Description "canonical PsyWFnTdbH grouping and strict-negative floor"
$psychrometricEvidence = (Read-RepoText -Path $psychrometrics) + [Environment]::NewLine +
    (Read-RepoText -Path $psychrometricTests)
foreach ($pattern in @('-0\.0', 'positive_values_below_the_return_floor', 'NAN|NaN', 'INFINITY', 'denominator_pole', 'ENERGYPLUS_MIN_HUMIDITY_RATIO\.to_bits\(\)')) {
    Assert-Cp404Text -Text $psychrometricEvidence -Pattern $pattern -Description "IEEE psychrometric characterization"
}
foreach ($path in @($transition, $release, $adapter, $coupled, $pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production panic"
}

Assert-PatternsInOrder -Path $binding -Patterns @(
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\s*=',
    'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*=', 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*=',
    'let\s+unit_available\s*=',
    'let\s+coupling\s*='
) -Description "CP403-to-CP404-to-CP405-to-CP406-to-CP407-to-CP408-to-CP414-to-CP415 binding order"
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\s*:',
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
$successorEvidence = "calculation_$successorStem"
if ([regex]::Matches($bindingText, "\b$predecessorEvidence\b").Count -ne 3 -or
    [regex]::Matches($bindingText, "\b$bindingEvidence\b").Count -ne 3 -or
    [regex]::Matches($bindingText, "\b$successorEvidence\b").Count -ne 3) {
    throw "CP404 binding must consume CP403 once, feed CP405 once, publish both snapshots, and preserve CP405 for CP406 without numerical coupling"
}
Assert-Contains -Path $adapterTests -Pattern 'binding_places_cp404_after_cp403_before_unchanged_numerical_coupling' -Description "binding order/nonfeed regression"
Assert-Contains -Path $coupledTests -Pattern 'cp404' -Description "coupled CP404 regression"
Assert-Contains -Path $coupledFixture -Pattern $bindingEvidence -Description "coupled output fixture"
Assert-Contains -Path $witness -Pattern "set_$stem`_latest_witness" -Description "runtime witness setter"

$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($serializationText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 47 -or $ieeeSidecars.Count -ne 47) {
    throw "CP404 JSON snapshot must expose exactly forty-seven numeric/IEEE pairs"
}
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $field = $numericFields[$index]
    if ($jsonNumbers[$index].Groups['field'].Value -cne $field -or
        $jsonNumbers[$index].Groups['value'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['field'].Value -cne $field -or
        $ieeeSidecars[$index].Groups['value'].Value -cne $field) {
        throw "CP404 JSON numeric/IEEE sidecar order drift at $field"
    }
}
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @(
    "$($predecessorStem)::\s*validate_direct_lifecycle",
    "$($stem)::\s*validate_direct_lifecycle",
    "$($successorStem)::\s*validate_direct_lifecycle"
) -Description "pipeline CP403-to-CP404-to-CP405 validation order"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp425_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp403\s*:\s*Option<&PredecessorLifecycle>' -Description "sole immediate predecessor"
Assert-Contains -Path $pipelineValidation -Pattern 'cp385_same_call_supply_enthalpy_bit_corroboration_count' -Description "CP385 corroboration accounting"
Assert-Contains -Path $pipelineLineage -Pattern 'predecessor_supply_enthalpy_assignment_executed' -Description "recursive CP385 enthalpy lineage"
Assert-Contains -Path $pipelineLineage -Pattern 'predecessor_cp403_cp329_retained_mixed_air_temperature_owned_read' -Description "collision-safe CP403 mixed-air owner lineage"
Assert-Contains -Path $pipelineLineage -Pattern 'predecessor_cp403_mixed_air_temperature_read' -Description "collision-safe CP403 mixed-air read lineage"
Assert-Contains -Path $pipelineSerialization -Pattern 'cp385_same_call_supply_enthalpy_bit_corroboration_count' -Description "serialized CP385 corroboration accounting"
Assert-Contains -Path $snapshotJsonTests -Pattern 'forty_seven|47|ieee' -Description "snapshot IEEE regressions"
Assert-Contains -Path $arbitraryAssertions -Pattern 'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)' -Description "public active route regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'non-direct runtime must not publish CP404 evidence' -Description "non-direct regression"
Assert-Contains -Path $arbitraryAssertions -Pattern 'mod\s+cp405_assertions' -Description "CP405 arbitrary assertion delegation module"
Assert-Contains -Path $arbitraryAssertions -Pattern 'cp405_assertions::assert_direct\(runtime,\s*results\)' -Description "CP405 direct arbitrary assertion delegation"
Assert-Contains -Path $arbitraryAssertions -Pattern 'cp405_assertions::assert_non_direct\(runtime\)' -Description "CP405 non-direct arbitrary assertion delegation"

$heading = 'CP404 post-saturation shared-case latent-output body supply-humidity-ratio psychrometric assignment'
$docs = @(
    'docs\src\current\current-status.md', 'docs\src\current\project-contract.md',
    'docs\src\porting-map\heat-balance-source-map.md',
    'docs\src\porting-map\ideal-loads-source-map.md',
    'docs\src\porting-map\zone-air-update-map.md'
)
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText, "(?m)^## $([regex]::Escape($heading))$").Count -ne 1) {
        throw "CP404 documentation heading must appear exactly once in $doc"
    }
}
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP404 supersedes only CP403' -Description "algorithm claim"
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP404 additionally requires' -Description "capability claim"
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP404 supersedes only CP403' -Description "generated algorithm claim"
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP404 additionally requires' -Description "generated capability claim"
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern 'CP404' -Description "Roadmap non-promotion"
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP404\b' -Description "psychrometrics non-promotion"

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
foreach ($file in @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 403) {
        Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp425_lifecycle_evidence' -Description "historical CP404 non-direct firewall"
    }
    if ($number -ge 337 -and $number -le 403) {
        Assert-Contains -Path $file.FullName -Pattern 'script_count = 363' -Description "historical current script count"
    }
    if ($number -ge 335 -and $number -le 403) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 363 \|')) -Description "historical generated script count"
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 123 \|')) -Description "historical generated internal count"
    }
    if ($number -ge 367 -and $number -le 403) {
        Assert-Contains -Path $file.FullName -Pattern 'Count -ne 123' -Description "historical internal classification count"
    }
}
$cp345Audit = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp404Call\s*=', '\$cp405Call\s*=', '\$cp409Call\s*=', 'CP403-to-CP404', 'CP404-to-CP405', 'CP408-to-CP409', 'CP414-to-CP415')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "CP345 terminal chain"
}
$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$cp403Index = $master.IndexOf('cp403-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-mixed-air-assignment.ps1')
$cp404Index = $master.IndexOf((Split-Path -Leaf $audit))
$cp405Index = $master.IndexOf('cp405-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-maximum-capacity-assignment.ps1')
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp403Index -lt 0 -or $cp404Index -le $cp403Index -or $cp405Index -le $cp404Index -or
    $completionIndex -le $cp405Index) {
    throw "Master CP404 registration order drift"
}
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 363', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp404Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 123) {
    throw "CP404 inventory classification drift; expected 240 public and 122 internal"
}
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp404-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-humidity-ratio-assignment\.ps1' -Description "inventory record"
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
Write-Host "CP404 post-saturation shared-case latent-output body supply-humidity-ratio psychrometric assignment structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-numerical' -Description 'CP425-to-numerical terminal interval'
