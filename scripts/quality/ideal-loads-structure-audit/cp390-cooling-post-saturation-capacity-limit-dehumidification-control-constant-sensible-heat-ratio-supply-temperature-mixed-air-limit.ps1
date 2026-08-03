# CP390 maps PurchasedAirManager.cc physical executable line 2281 and stops before 2283.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment"
$successorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit"
$terminalStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimit"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$commit = "6f2e40d10250a105b49966baa24d843711e61048"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$routes = "$root\transition\routes.rs"
$owners = "$root\transition\owners.rs"
$accounting = "$root\transition\accounting.rs"
$release = "$root\release.rs"
$prefix = "$root\release\prefix_validation.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$tests = "$root\tests.rs"
$routeTests = "$root\tests\routes.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp390.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineSerialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$cp389Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp389_assertions.rs"
$cp390Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp390_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1"
$sites = @(
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature"
)
$numericFields = @(
    "predecessor_mixed_air_humidity_ratio",
    "predecessor_psychrometric_cp_air_result_j_per_kg_k",
    "predecessor_cp_air_j_per_kg_k",
    "predecessor_cooling_total_output_w",
    "predecessor_cooling_sensible_heat_ratio",
    "predecessor_calculated_cooling_sensible_output_w",
    "predecessor_cooling_sensible_output_w",
    "predecessor_resulting_supply_enthalpy_j_per_kg",
    "predecessor_preexisting_supply_temperature_c",
    "predecessor_mixed_air_temperature_c",
    "predecessor_cp389_cooling_sensible_output_w",
    "predecessor_cp389_cp_air_j_per_kg_k",
    "predecessor_supply_mass_flow_rate_kg_per_s",
    "predecessor_cp_air_times_supply_mass_flow_rate_w_per_k",
    "predecessor_cooling_sensible_output_over_air_capacity_rate_k",
    "predecessor_calculated_supply_temperature_c",
    "predecessor_assigned_supply_temperature_c",
    "predecessor_resulting_supply_temperature_c",
    "resulting_supply_enthalpy_j_per_kg",
    "preexisting_supply_temperature_c",
    "supply_temperature_before_mixed_air_limit_c",
    "mixed_air_temperature_c",
    "minimum_supply_temperature_c",
    "assigned_supply_temperature_c",
    "resulting_supply_temperature_c"
)

function Assert-Cp390Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP390 $Description missing '$Pattern'" }
}

function Get-Cp390BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP390 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP390 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP390 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $owners, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, "$root\release\private_characterization.rs",
    $tests, $routeTests, $adapter, "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs",
    $coupled, $coupledTests,
    "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs",
    "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs",
    $pipeline, $pipelineValidation, $pipelineSerialization, $snapshotJson, $snapshotJsonTests,
    $cp389Assertions, $cp390Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP390 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP390 bounded file"
}
foreach ($directory in @($root, "crates\ep_run\src\pipeline\$pipelineStem")) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP390 bounded recursive file"
    }
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP390 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2280].Trim() -cne 'PurchAir.SupplyTemp = min(PurchAir.SupplyTemp, PurchAir.MixedAirTemp);' -or
    -not $lines[2281].Trim().StartsWith('//') -or
    $lines[2282].Trim() -cne 'SupplyEnthalpy = max(SupplyEnthalpy, PsyHFnTdbW(PurchAir.SupplyTemp, 0.00001));') {
    throw "CP390 source slice, comment-only line 2282, or CP391 boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2281' -Description "mapped source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2283' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER' `
    -Expected $sites `
    -Description "exact four-site source order"

$core = (Get-ChildItem -LiteralPath $root -Recurse -File -Filter "*.rs" | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine
foreach ($counter in @(
        'transition_count', 'inactive_transition_count',
        'dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count',
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]', 'source_site_execution_count',
        'cp389_supply_temperature_state_owner_count', 'unchanged_supply_temperature_preservation_count',
        'supply_temperature_owned_read_count', 'supply_temperature_for_minimum_read_count',
        'mixed_air_temperature_owned_read_count', 'mixed_air_temperature_bit_corroboration_count',
        'mixed_air_temperature_for_minimum_read_count',
        'source_shaped_two_argument_minimum_evaluation_count', 'supply_temperature_assignment_write_count'
    )) {
    Assert-Contains -Path $state -Pattern "pub $counter" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed',
        'predecessor_resulting_supply_temperature_c', 'predecessor_resulting_supply_enthalpy_j_per_kg',
        'dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed',
        'cp389_retained_supply_temperature_state_owned', 'preexisting_supply_temperature_c',
        'cp389_retained_supply_temperature_owned_read', 'supply_temperature_for_minimum_read',
        'supply_temperature_before_mixed_air_limit_c',
        'cp329_retained_mixed_air_temperature_owned_read', 'cp389_mixed_air_temperature_bit_corroborated',
        'mixed_air_temperature_for_minimum_read', 'mixed_air_temperature_c',
        'source_shaped_two_argument_minimum_evaluated', 'minimum_supply_temperature_c',
        'supply_temperature_assignment_performed', 'assigned_supply_temperature_c',
        'resulting_supply_temperature_c', 'resulting_supply_enthalpy_j_per_kg'
    )) {
    Assert-Contains -Path $module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp390BraceBlock -Text $moduleText `
    -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" `
    -Description "snapshot declaration"
[string[]]$actualNumericFields = @(
    [regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') |
        ForEach-Object { $_.Groups['field'].Value }
)
if ($actualNumericFields.Count -ne $numericFields.Count) {
    throw "CP390 snapshot must expose exactly $($numericFields.Count) Option<f64> fields; found $($actualNumericFields.Count)"
}
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) {
        throw "CP390 numeric field $($index + 1) expected '$($numericFields[$index])', found '$($actualNumericFields[$index])'"
    }
}
foreach ($pattern in @(
        'cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_snapshot_route',
        'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'predecessor\.resulting_supply_temperature_c',
        'predecessor\.mixed_air_temperature_c',
        'owner\.mixed_air_temperature_c', 'to_bits\(\)',
        'source_shaped_two_argument_minimum\(',
        'minimum_supply_temperature_c\.or\(prepared\.preexisting_supply_temperature_c\)',
        'cooling_mixed_air_call_snapshot_is_exact_direct_release'
    )) {
    Assert-Cp390Text -Text $core -Pattern $pattern -Description "route/formula/owner contract"
}
Assert-Contains -Path $transition -Pattern 'cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum' -Description "canonical ObjexxFCL minimum reuse"
Assert-Contains -Path $transition -Pattern 'predecessor_resulting_supply_enthalpy_j_per_kg\s*:\s*predecessor\.resulting_supply_enthalpy_j_per_kg' -Description "distinct predecessor enthalpy lineage"
Assert-Contains -Path $transition -Pattern '(?m)^\s*resulting_supply_enthalpy_j_per_kg\s*:\s*predecessor\.resulting_supply_enthalpy_j_per_kg' -Description "unchanged local enthalpy carry"
Assert-Contains -Path $snapshotValidation -Pattern '(?s)snapshot\.resulting_supply_enthalpy_j_per_kg.*?predecessor\.resulting_supply_enthalpy_j_per_kg' -Description "local-to-predecessor enthalpy bit corroboration"
Assert-Contains -Path "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\transition.rs" -Pattern '(?s)if left < right \{ left \} else \{ right \}' -Description "source-shaped strict minimum"
foreach ($path in @($transition, $release, $adapter, $coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $transition -Pattern 'f64::min|\.min\s*\(|total_cmp|partial_cmp|\.clamp\s*\(|normalize|is_finite|epsilon|tolerance|DirectZonePurchasedAirCouplingInput' -Description "broadened minimum, finite gate, or DTO use"
$productionFiles = @($module, $adapter, $coupled, $pipeline, $pipelineValidation, $pipelineSerialization, $snapshotJson)
$productionFiles += @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs' | Where-Object {
        $_.FullName -notmatch '[\\/]tests\.rs$' -and $_.FullName -notmatch '[\\/]tests[\\/]'
    } | ForEach-Object { $_.FullName })
foreach ($productionFile in $productionFiles | Select-Object -Unique) {
    Assert-NotContains -Path $productionFile `
        -Pattern 'PsyHFnTdbW|0\.00001|source_shaped_two_argument_maximum|f64::max|\.max\s*\(|energyplus_psy_h_fn_tdb_w|moist_air_enthalpy_j_per_kg|SupplyEnthalpy' `
        -Description "CP391 psychrometric maximum/enthalpy-assignment leakage"
}
Assert-Contains -Path $snapshotValidation -Pattern 'to_bits\(\)' -Description "bit-exact minimum and retained results"

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine +
    ((Get-ChildItem -LiteralPath "$root\tests" -Recurse -File -Filter "*.rs" | ForEach-Object {
            Read-RepoText -Path $_.FullName
        }) -join [Environment]::NewLine)
foreach ($pattern in @(
        'assert_eq!\(snapshots\.len\(\),\s*30\)',
        'assert_eq!\(state\.transition_count,\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*27\)',
        '(?s)dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count,\s*3',
        'assert_eq!\(state\.predecessor_route_counts,\s*\[1;\s*30\]\)',
        'assert_eq!\(state\.source_site_execution_count,\s*12\)',
        'assert_eq!\(state\.cp389_supply_temperature_state_owner_count,\s*27\)',
        'assert_eq!\(state\.unchanged_supply_temperature_preservation_count,\s*24\)',
        '(?s)snapshot_is_exact_direct_release\(.*?\.count\(\),\s*11',
        '-0\.0', 'f64::INFINITY', 'from_bits\(0x7ff8_', 'to_bits\(\)', 'unordered|tie'
    )) {
    Assert-Cp390Text -Text $testText -Pattern $pattern -Description "route/retention/IEEE test"
}

$bindingText = Read-RepoText -Path $binding
$cp389Index = $bindingText.IndexOf("let calculation_$predecessorStem =")
$cp390Index = $bindingText.IndexOf("let calculation_$stem =")
$cp391Index = $bindingText.IndexOf("let calculation_$successorStem =")
$cp392Index = $bindingText.IndexOf("let calculation_$terminalStem =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp389Index -lt 0 -or $cp390Index -le $cp389Index -or $cp391Index -le $cp390Index -or $cp392Index -le $cp391Index -or $numericalIndex -le $cp392Index) {
    throw "Binding must execute CP389, CP390, CP391, CP392, then unchanged numerical coupling"
}
$cp390BindingName = "calculation_$stem"
$cp390BindingMatches = [regex]::Matches($bindingText, [regex]::Escape($cp390BindingName))
$cp391Call = [regex]::Match($bindingText, "(?s)let calculation_$([regex]::Escape($successorStem)) =\s*advance_$([regex]::Escape($successorStem))\((?<args>[^;]+?)\)\?;")
if (-not $cp391Call.Success -or [regex]::Matches($cp391Call.Groups['args'].Value, [regex]::Escape($cp390BindingName)).Count -ne 1) {
    throw "CP391 must consume CP390 exactly once as its immediate predecessor"
}
if ($cp390BindingMatches.Count -ne 3 -or
    $cp390BindingMatches[0].Index -lt $cp390Index -or $cp390BindingMatches[0].Index -ge $cp391Index -or
    $cp390BindingMatches[1].Index -le $cp391Index -or $cp390BindingMatches[1].Index -ge ($cp391Call.Index + $cp391Call.Length) -or
    $cp390BindingMatches[2].Index -le $numericalIndex) {
    throw "CP390 binding evidence must be declared, consumed once by CP391, then stored once after numerical coupling"
}
$cp390ToCp391Interval = $bindingText.Substring($cp390Index, $cp391Index - $cp390Index)
if ([regex]::Matches($cp390ToCp391Interval, [regex]::Escape($cp390BindingName)).Count -ne 1) {
    throw "CP390 evidence must remain unconsumed until the exact CP391 call"
}
$dto = Get-Cp390BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match [regex]::Escape($stem) -or $dto -match [regex]::Escape($typeStem) -or
    $dto -match [regex]::Escape($successorStem) -or $dto -match [regex]::Escape($terminalStem) -or
    $dto -match 'cp39[012]|minimum_supply_temperature_c|assigned_supply_temperature_c|resulting_supply_temperature_c|preexisting_supply_temperature_c|resulting_supply_enthalpy_j_per_kg|maximum_supply_enthalpy_j_per_kg|resulting_supply_humidity_ratio') {
    throw "CP390/CP391/CP392 evidence unexpectedly feeds the numerical DTO"
}
Assert-NotContains -Path $adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node|prediction|feedback|\breport\b|ResultStore|numerical' -Description "adapter numerical feed"
$coupledText = Read-RepoText -Path $coupled
$coupledSnapshotValidation = Get-Cp390BraceBlock -Text $coupledText `
    -AnchorPattern 'fn\s+snapshot_matches_release\s*\(' `
    -Description "coupled snapshot validation"
Assert-Cp390Text -Text $coupledSnapshotValidation `
    -Pattern "output\.$([regex]::Escape($cp390BindingName))" `
    -Description "coupled CP390 evidence read"
if ($coupledSnapshotValidation -match '\.coupling\b|\.prediction\b|\.purchased_air\b|\.feedback\b|\.report\b|supply_node|\bload\b|reconcil|DirectZonePurchasedAirCouplingInput|ResultStore') {
    throw "CP390 coupled validation unexpectedly consumes numerical, node, load, report, or reconciliation state"
}

foreach ($registration in @(
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\calc.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"; Pattern = "pub calculation_$($stem):" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"; Pattern = "mod $($stem)_validation;" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $pipelineRoot; Pattern = $pipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}

$lifecycleField = "purchased_air_calc_$($stem)_lifecycle"
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp412_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineRoot -Pattern $lifecycleField -Description "pipeline lifecycle key"
$pipelineText = Read-RepoText -Path $pipelineRoot
$nonDirectValidation = Get-Cp390BraceBlock -Text $pipelineText -AnchorPattern 'fn\s+validate_runtime_demand_provenance\s*\(' -Description "non-direct production firewall"
Assert-Cp390Text -Text $nonDirectValidation -Pattern "(?s)\.$([regex]::Escape($lifecycleField))\s*\.is_some\s*\(\s*\)" -Description "production lifecycle Some rejection"
Assert-NotContains -Path $pipelineValidation -Pattern 'MixedAirLifecycle|mixed_air_cp329|PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL|mixed_air_state|mixed_air_latest' -Description "direct CP329 dependency on public inactive routes"
$pipelineCall = [regex]::Match(
    $pipelineText,
    "(?s)$([regex]::Escape($pipelineStem))::\s*validate_direct_lifecycle\s*\((?<args>.*?)\)\s*\?;"
)
if (-not $pipelineCall.Success -or
    $pipelineCall.Groups['args'].Value -match 'purchased_air_calc_cooling_mixed_air_call_lifecycle') {
    throw "CP390 pipeline validation must depend directly on CP389, not CP329"
}
foreach ($path in @($pipelineValidation, $coupled)) {
    Assert-Contains -Path $path `
        -Pattern 'predecessor\.source_order\s*==\s*PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER' `
        -Description "CP389 source-order provenance"
}
foreach ($pattern in @('inactive_transition_count', 'mixed_air_limit_count', 'source_site_execution_count', 'predecessor_route_counts')) {
    Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline count $pattern"
}
$snapshotJsonText = Read-RepoText -Path $snapshotJson
$jsonNumberMatches = [regex]::Matches(
    $snapshotJsonText,
    '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)'
)
$ieeeSidecarMatches = [regex]::Matches(
    $snapshotJsonText,
    '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)'
)
if ($jsonNumberMatches.Count -ne $numericFields.Count -or $ieeeSidecarMatches.Count -ne $numericFields.Count) {
    throw "CP390 serialization must expose exactly 25 numeric projections and 25 IEEE sidecars"
}
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $expected = $numericFields[$index]
    if ($jsonNumberMatches[$index].Groups['field'].Value -cne $expected -or
        $jsonNumberMatches[$index].Groups['value'].Value -cne $expected -or
        $ieeeSidecarMatches[$index].Groups['field'].Value -cne $expected -or
        $ieeeSidecarMatches[$index].Groups['value'].Value -cne $expected) {
        throw "CP390 numeric/IEEE serialization field $($index + 1) must be exact '$expected'"
    }
}
Assert-Cp390Text -Text $snapshotJsonText -Pattern '"predecessor_resulting_supply_enthalpy_j_per_kg_ieee_bits"' -Description "predecessor enthalpy IEEE sidecar"
Assert-Cp390Text -Text $snapshotJsonText -Pattern '(?m)^\s*"resulting_supply_enthalpy_j_per_kg_ieee_bits"' -Description "local enthalpy IEEE sidecar"
Assert-Contains -Path $snapshotJsonTests -Pattern 'fn\s+numeric_fields\s*\(\s*\)\s*->\s*\[&''static\s+str;\s*25\]' -Description "twenty-five-field serialization regression test"
Assert-Contains -Path $cp389Assertions -Pattern 'mod cp390_assertions;' -Description "arbitrary CP390 module"
Assert-Contains -Path $cp389Assertions -Pattern 'cp390_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp389Assertions -Pattern 'cp390_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-Contains -Path $cp390Assertions -Pattern 'CP390 lifecycle must remain outside numerical result state' -Description "terminal numerical nonfeed"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP390 supersedes only CP389[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP390 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) {
    throw "CP390 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, '2281', '2282', '2283', 'four', 'thirty', '18', '22', '28',
            'twenty-seven', 'eleven', 'nineteen', '12', 'twenty-four',
            'T390=T389', 'L390=A389', 'inactive_transition_count=T390-L390',
            'sole immediate predecessor', 'CP389', 'CP329', 'CP379', 'CP334', 'CP344',
            'CP385', 'a < b \? a : b', 'if left < right \{ left \} else \{ right \}',
            'ties and unordered', 'f64::min', 'IEEE sidecars',
            'DirectZonePurchasedAirCouplingInput', '328 total', '240 public', '88 internal',
            'zero unused', 'zero unreachable', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP390 spec addendum missing '$pattern'" }
    }
}

$docs = @(
    "docs\src\current\current-status.md", "docs\src\current\project-contract.md",
    "docs\src\porting-map\ideal-loads-source-map.md",
    "docs\src\porting-map\heat-balance-source-map.md",
    "docs\src\porting-map\zone-air-update-map.md"
)
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    $sections = [regex]::Matches($text, '(?ms)^## CP390\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP390 documentation expected one section in $doc" }
    foreach ($pattern in @(
            $commit, $hash, '2281', '2282', '2283', 'thirty|30', 'twenty-seven|27',
            'three|3', 'twenty-four|24', 'eleven|11', 'nineteen|19', '18', '22', '28',
            'CP389', 'CP329', 'CP379', 'CP334', 'CP344', 'CP385', 'T390=T389',
            'L390=A389', '12', '25|twenty-five',
            'predecessor_resulting_supply_enthalpy_j_per_kg',
            'resulting_supply_enthalpy_j_per_kg', 'IEEE|_ieee_bits',
            'if left < right', 'f64::min', 'CP391', '328\s+total', '88\s+internal'
        )) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP390 documentation in $doc missing '$pattern'" }
    }
    $cp389DocIndex = $text.LastIndexOf("## CP389 ")
    $cp390DocIndex = $text.LastIndexOf("## CP390 ")
    if ($cp389DocIndex -lt 0 -or $cp390DocIndex -le $cp389DocIndex) { throw "CP389-to-CP390 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP390\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP390 supersedes only CP389' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP390 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp412_lifecycle_evidence' -Description "historical cumulative firewall"
}
foreach ($historical in 335..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 350 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 110 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 350' -Description "historical inventory total"
}
foreach ($historical in 367..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 110' -Description "historical internal classification"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 110 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP390 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp389Call\s*=', '\$cp390Call\s*=', '\$cp391Call\s*=', '\$cp392Call\s*=', 'CP389-to-CP390', 'CP390-to-CP391', 'CP391-to-CP392', 'CP396-to-CP397', 'CP397-to-CP398', 'CP400-to-CP401', 'CP401-to-CP402', 'CP402-to-CP403', 'CP403-to-CP404')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "strict CP345 terminal ordering"
}
Assert-LineLimit -Path $cp345Audit -Limit 1200 -Description "CP345 fixed structural cap"
foreach ($historical in 377..390) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP389-to-CP390' -Description "historical CP390 predecessor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP390-to-CP391' -Description "historical CP391 predecessor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP391-to-CP392' -Description "historical CP392 predecessor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP396-to-CP397' -Description "historical CP396-to-CP397 interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP397-to-CP398' -Description "historical CP397-to-CP398 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP400-to-CP401' -Description "historical CP400-to-CP401 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP401-to-CP402' -Description "historical CP401-to-CP402 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP402-to-CP403' -Description "historical CP402 terminal interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP403-to-CP404' -Description "historical CP402 terminal interval"
}
foreach ($historical in 385..390) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp390Index\s*=' -Description "historical binding CP390 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp390AuditIndex\s*=' -Description "historical master CP390 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp391Index\s*=' -Description "historical binding CP391 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp391AuditIndex\s*=' -Description "historical master CP391 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp392Index\s*=' -Description "historical binding CP392 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp392AuditIndex\s*=' -Description "historical master CP392 successor"
}

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp389AuditIndex = $master.IndexOf("cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment.ps1")
$cp390AuditIndex = $master.IndexOf("cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1")
$cp391AuditIndex = $master.IndexOf("cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp389AuditIndex -lt 0 -or $cp390AuditIndex -le $cp389AuditIndex -or $cp391AuditIndex -le $cp390AuditIndex -or $cp392AuditIndex -le $cp391AuditIndex -or $completionIndex -le $cp392AuditIndex) {
    throw "Master audit must dot-source CP390, CP391, then CP392 after CP389 before completion"
}
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 350', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp390Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 110) {
    throw "CP390 inventory must be exactly 240 public and 110 internal scripts"
}
Assert-Cp390Text -Text $inventory -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit\.ps1"' -Description "inventory record"
foreach ($pattern in @(
        '\| 350 \|', '\| public scripts \| 240 \|',
        '\| 110 \|', '\| scripts without callers \| 0 \|'
    )) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory"
}

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP404-to-CP405' -Description "CP345 CP404-to-CP405 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP405-to-CP406' -Description "CP345 CP405-to-CP406 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP406-to-CP407' -Description "CP345 CP406-to-CP407 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP407-to-CP408' -Description "CP345 CP407-to-CP408 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP408-to-CP409' -Description "CP345 CP408-to-CP409 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP410-to-CP411' -Description "CP345 CP410-to-CP411 interval"
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP411-to-CP412' -Description 'CP345 CP411-to-CP412 interval'
Write-Host "CP390 post-saturation constant-SHR supply-temperature mixed-air-limit structure audit passed."
}
