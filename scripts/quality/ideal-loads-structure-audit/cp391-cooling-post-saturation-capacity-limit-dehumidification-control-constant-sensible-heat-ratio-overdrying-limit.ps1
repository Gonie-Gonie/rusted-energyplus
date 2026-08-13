# CP391 maps PurchasedAirManager.cc physical executable line 2283 and stops before 2284.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit"
$successorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimit"
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
$privateCharacterization = "$root\release\private_characterization.rs"
$tests = "$root\tests.rs"
$routeTests = "$root\tests\routes.rs"
$ieeeTests = "$root\tests\ieee.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$bindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation"
$coupledLineage = "$coupledRoot\lineage.rs"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp391.rs"
$fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineValidationTests = "crates\ep_run\src\pipeline\$pipelineStem\validation\tests.rs"
$pipelineSerialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$arbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp390Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp390_assertions.rs"
$cp391Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp391_assertions.rs"
$psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$maximumHelper = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\transition.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1"
$sites = @(
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit-maximum",
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-overdrying-limit-enthalpy",
    "evaluate-psy-h-fn-tdb-w-at-minimum-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
    "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit"
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
    "predecessor_cp390_resulting_supply_enthalpy_j_per_kg",
    "preexisting_supply_temperature_c",
    "supply_temperature_before_mixed_air_limit_c",
    "mixed_air_temperature_c",
    "minimum_supply_temperature_c",
    "assigned_supply_temperature_c",
    "predecessor_cp390_resulting_supply_temperature_c",
    "preexisting_supply_enthalpy_j_per_kg",
    "supply_enthalpy_before_overdrying_limit_j_per_kg",
    "supply_temperature_c",
    "psychrometric_minimum_supply_enthalpy_j_per_kg",
    "maximum_supply_enthalpy_j_per_kg",
    "assigned_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)
$localBools = @(
    "dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed",
    "cp390_retained_supply_enthalpy_state_owned",
    "cp390_retained_supply_enthalpy_owned_read",
    "supply_enthalpy_for_overdrying_limit_maximum_read",
    "cp390_retained_supply_temperature_owned_read",
    "supply_temperature_for_minimum_humidity_ratio_enthalpy_read",
    "psychrometric_minimum_supply_enthalpy_evaluated",
    "source_shaped_two_argument_maximum_evaluated",
    "supply_enthalpy_assignment_performed"
)

function Assert-Cp391Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP391 $Description missing '$Pattern'" }
}

function Get-Cp391BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP391 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP391 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP391 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $owners, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, $privateCharacterization, $tests, $routeTests,
    $ieeeTests, $adapter, $bindingTests, $coupled, $coupledLineage, $coupledTests, $fixture, $witness,
    $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineSerialization, $snapshotJson,
    $cp390Assertions, $cp391Assertions, $maximumHelper, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP391 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP391 bounded file"
}
Assert-FileExists -Path $arbitraryRoot -Description "CP391 arbitrary-run test root"
Assert-FileExists -Path $psychrometrics -Description "CP391 canonical psychrometrics owner"
foreach ($directory in @($root, $coupledRoot, "crates\ep_run\src\pipeline\$pipelineStem")) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP391 bounded recursive file"
    }
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP391 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2282].Trim() -cne 'SupplyEnthalpy = max(SupplyEnthalpy, PsyHFnTdbW(PurchAir.SupplyTemp, 0.00001));' -or
    $lines[2283].Trim() -cne 'PurchAir.SupplyHumRat = PsyWFnTdbH(state, PurchAir.SupplyTemp, SupplyEnthalpy, RoutineName);') {
    throw "CP391 source slice or full CP392 first-excluded statement drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2283' -Description "mapped source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2284' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER' `
    -Expected $sites -Description "exact five-site source order shared with CP353"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp391BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot declaration"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 33) { throw "CP391 snapshot must expose exactly 33 Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP391 numeric field $($index + 1) expected '$($numericFields[$index])', found '$($actualNumericFields[$index])'" }
}
$localStart = $snapshotStruct.IndexOf("pub $($localBools[0]):")
if ($localStart -lt 0) { throw "CP391 local field boundary missing" }
[string[]]$actualLocalBools = @([regex]::Matches($snapshotStruct.Substring($localStart), 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*bool') | ForEach-Object { $_.Groups['field'].Value })
if ($actualLocalBools.Count -ne 9) { throw "CP391 snapshot must expose exactly nine CP391-local bool fields" }
for ($index = 0; $index -lt $localBools.Count; $index += 1) {
    if ($actualLocalBools[$index] -cne $localBools[$index]) { throw "CP391 local bool $($index + 1) expected '$($localBools[$index])', found '$($actualLocalBools[$index])'" }
}

foreach ($counter in @(
        'transition_count', 'inactive_transition_count',
        'dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count',
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]', 'source_site_execution_count',
        'cp390_supply_enthalpy_state_owner_count', 'unchanged_supply_enthalpy_preservation_count',
        'supply_enthalpy_owned_read_count', 'supply_enthalpy_for_overdrying_limit_maximum_read_count',
        'supply_temperature_owned_read_count', 'supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count',
        'psychrometric_minimum_supply_enthalpy_evaluation_count',
        'source_shaped_two_argument_maximum_evaluation_count', 'supply_enthalpy_assignment_write_count'
    )) {
    Assert-Contains -Path $state -Pattern "pub $counter" -Description "state counter $counter"
}
$core = (Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs' | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'supply_temperature_mixed_air_limit_snapshot_route', 'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)', 'index\s*>=\s*3',
        'predecessor\.resulting_supply_enthalpy_j_per_kg', 'predecessor\.resulting_supply_temperature_c',
        'predecessor_total\s*!=\s*state\.transition_count', 'route_total\s*!=\s*Some\(state\.transition_count\)',
        'active_total\s*!=\s*Some\(limits\)', 'expected_sites\s*!=\s*Some\(state\.source_site_execution_count\)',
        'count\.checked_sub\(limits\)', 'active_counts\.into_iter\(\)\.any\(\|count\| count != limits\)'
    )) {
    Assert-Cp391Text -Text $core -Pattern $pattern -Description "route/owner/algebra contract"
}
Assert-Contains -Path $release -Pattern 'SupplyTemperatureMixedAirLimitSnapshot as Predecessor' -Description "exact CP390 predecessor type"
Assert-Contains -Path $release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp390:\s*Predecessor,\s*\)' -Description "exact public predecessor-only arguments"
foreach ($pattern in @(
        'completed_direct_.*?_supply_temperature_mixed_air_limit_is_consistent',
        'supply_temperature_mixed_air_limit_latest_witness'
    )) {
    Assert-Contains -Path $prefix -Pattern $pattern -Description "recursive CP390 completion evidence"
}
Assert-Contains -Path $release -Pattern 'supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release\(predecessor_cp390\)' -Description "recursive CP390 direct-release evidence"
Assert-NotContains -Path $release -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|cp38[59]\s*:|cp379\s*:|cp329\s*:|supply_enthalpy_before_overdrying_limit_j_per_kg\s*:|supply_temperature_c\s*:' -Description "older owner, scalar, or numerical substitution"

Assert-Contains -Path $transition -Pattern 'cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum' -Description "canonical maximum import"
Assert-Contains -Path $transition -Pattern 'crate::psychrometrics::energyplus_psy_h_fn_tdb_w' -Description "canonical PsyH import"
Assert-Contains -Path $transition -Pattern '(?s)energyplus_psy_h_fn_tdb_w\(active\.supply_temperature_c,\s*1\.0e-5\)' -Description "canonical psychrometric call"
Assert-Contains -Path $transition -Pattern '(?s)source_shaped_two_argument_maximum\(\s*active\.supply_enthalpy_before_overdrying_limit_j_per_kg,\s*minimum,\s*\)' -Description "source-shaped maximum call"
Assert-Contains -Path $transition -Pattern 'maximum_supply_enthalpy_j_per_kg\.or\(prepared\.preexisting_supply_enthalpy_j_per_kg\)' -Description "inactive exact enthalpy preservation"
Assert-Contains -Path $transition -Pattern 'resulting_supply_temperature_c:\s*prepared\.resulting_supply_temperature_c' -Description "unchanged temperature carrier"
Assert-NotContains -Path $transition -Pattern 'fn\s+source_shaped_two_argument_maximum|f64::max|\.max\s*\(|total_cmp|partial_cmp|mul_add|\.clamp\s*\(|normalize|is_finite|is_nan|epsilon|tolerance|moist_air_enthalpy_j_per_kg' -Description "duplicate/broadened maximum or alternate enthalpy path"
Assert-Contains -Path $maximumHelper -Pattern '(?s)fn source_shaped_two_argument_maximum\(.*?\)\s*->\s*f64\s*\{\s*if left < right \{ right \} else \{ left \}\s*\}' -Description "canonical strict maximum body"
Assert-Contains -Path $psychrometrics -Pattern 'const ENERGYPLUS_MIN_HUMIDITY_RATIO:\s*f64\s*=\s*1\.0e-5;' -Description "canonical humidity floor literal"
Assert-Contains -Path $psychrometrics -Pattern '(?s)fn energyplus_psy_h_fn_tdb_w_raw\(.*?\)\s*->\s*f64\s*\{\s*1\.004_84e3 \* dry_bulb_c \+ humidity_ratio \* \(2\.500_94e6 \+ 1\.858_95e3 \* dry_bulb_c\)\s*\}' -Description "canonical PsyH grouping"
Assert-Contains -Path $psychrometrics -Pattern '(?s)pub fn energyplus_psy_h_fn_tdb_w\(.*?\)\s*->\s*f64\s*\{\s*energyplus_psy_h_fn_tdb_w_raw\(dry_bulb_c,\s*energyplus_humidity_ratio_floor\(humidity_ratio\)\)\s*\}' -Description "canonical PsyH floor path"
Assert-Contains -Path $snapshotValidation -Pattern '(?s)energyplus_psy_h_fn_tdb_w\(temperature,\s*1\.0e-5\).*?source_shaped_two_argument_maximum\(left,\s*expected_psychrometric\).*?psychrometric\.to_bits\(\) != expected_psychrometric\.to_bits\(\).*?maximum\.to_bits\(\) != expected_maximum\.to_bits\(\).*?assigned\.to_bits\(\) != maximum\.to_bits\(\).*?resulting\.to_bits\(\) != assigned\.to_bits\(\)' -Description "active IEEE exactness"
Assert-Contains -Path $snapshotValidation -Pattern '(?s)active_values\.into_iter\(\)\.any\(\|value\| value\.is_some\(\)\).*?resulting_supply_enthalpy_j_per_kg,\s*snapshot\.preexisting_supply_enthalpy_j_per_kg' -Description "inactive local nulls and bit preservation"

$coupledFiles = @($coupled) + @(Get-ChildItem -LiteralPath $coupledRoot -Recurse -File -Filter '*.rs' | ForEach-Object { $_.FullName })
$productionFiles = @($module, $adapter, $pipeline, $pipelineValidation, $pipelineSerialization, $snapshotJson) + $coupledFiles
$productionFiles += @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs' | Where-Object { $_.FullName -notmatch '[\\/]tests\.rs$' -and $_.FullName -notmatch '[\\/]tests[\\/]' } | ForEach-Object { $_.FullName })
$humidityFirewallPattern = 'PsyWFnTdbH|energyplus_psy_w_fn_tdb_h|psy_w_fn_tdb_h|\bRoutineName\b|(?:\bSupplyHumRat\b|\b[A-Za-z0-9_]*supply_(?:hum(?:idity)?_ratio|hum_rat)\b)\s*(?:=|:)'
$integrationHumidityWritePattern = 'PsyWFnTdbH|energyplus_psy_w_fn_tdb_h|psy_w_fn_tdb_h|\bRoutineName\b|(?:\bSupplyHumRat\b|\b[A-Za-z0-9_]*supply_(?:hum(?:idity)?_ratio|hum_rat)\b)\s*='
foreach ($productionFile in $productionFiles | Select-Object -Unique) {
    Assert-NotContains -Path $productionFile -Pattern $humidityFirewallPattern -Description "CP392 humidity-ratio-assignment firewall"
}
foreach ($integrationPath in @($binding, "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs", "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs", $pipelineRoot)) {
    $integrationText = Read-RepoText -Path $integrationPath
    $anchors = [regex]::Matches($integrationText, [regex]::Escape($stem))
    if ($anchors.Count -eq 0) { throw "CP391 integration anchor missing in '$integrationPath'" }
    foreach ($anchor in $anchors) {
        $start = [Math]::Max(0, $anchor.Index - 4096)
        $end = [Math]::Min($integrationText.Length, $anchor.Index + $anchor.Length + 4096)
        if ($integrationText.Substring($start, $end - $start) -match $integrationHumidityWritePattern) {
            throw "CP391 CP392 humidity-ratio-assignment firewall violated near integration anchor in '$integrationPath'"
        }
    }
}
foreach ($path in @($transition, $release, $adapter, $pipelineValidation) + $coupledFiles) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine + ((Get-ChildItem -LiteralPath "$root\tests" -Recurse -File -Filter '*.rs' | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine)
foreach ($pattern in @(
        'assert_eq!\(snapshots\.len\(\),\s*30\)', 'assert_eq!\(state\.transition_count,\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*27\)', '(?s)overdrying_limit_count,\s*3',
        'assert_eq!\(state\.predecessor_route_counts,\s*\[1;\s*30\]\)',
        'assert_eq!\(state\.source_site_execution_count,\s*15\)',
        'assert_eq!\(state\.cp390_supply_enthalpy_state_owner_count,\s*17\)',
        'assert_eq!\(state\.unchanged_supply_enthalpy_preservation_count,\s*14\)',
        '(?s)snapshot_is_exact_direct_release\(.*?\.count\(\),\s*11',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)',
        'usize::MAX', 'from_bits\(0x7ff8_', 'f64::INFINITY', '-0\.0', 'to_bits\(\)', 'ties_and_unordered'
    )) {
    Assert-Cp391Text -Text $testText -Pattern $pattern -Description "route/count/overflow/IEEE test"
}

$bindingText = Read-RepoText -Path $binding
$cp390BindingName = "calculation_$predecessorStem"
$cp391BindingName = "calculation_$stem"
$cp392BindingName = "calculation_$successorStem"
$cp390Index = $bindingText.IndexOf("let $cp390BindingName =")
$cp391Index = $bindingText.IndexOf("let $cp391BindingName =")
$cp392Index = $bindingText.IndexOf("let $cp392BindingName =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp390Index -lt 0 -or $cp391Index -le $cp390Index -or $cp392Index -le $cp391Index -or $numericalIndex -le $cp392Index) { throw "Binding must execute CP390, CP391, CP392, then unchanged numerical coupling" }
$cp391Call = [regex]::Match($bindingText, "(?s)let $([regex]::Escape($cp391BindingName)) =\s*advance_$([regex]::Escape($stem))\((?<args>.*?)\)\?;")
if (-not $cp391Call.Success -or [regex]::Matches($cp391Call.Groups['args'].Value, [regex]::Escape($cp390BindingName)).Count -ne 1) { throw "CP391 must consume CP390 exactly once as sole immediate predecessor" }
$cp392Call = [regex]::Match($bindingText, "(?s)let $([regex]::Escape($cp392BindingName)) =\s*advance_$([regex]::Escape($successorStem))\((?<args>.*?)\)\?;")
if (-not $cp392Call.Success -or [regex]::Matches($cp392Call.Groups['args'].Value, [regex]::Escape($cp391BindingName)).Count -ne 1) { throw "CP392 must consume CP391 exactly once as sole immediate predecessor" }
$cp390Matches = [regex]::Matches($bindingText, [regex]::Escape($cp390BindingName))
$cp391Matches = [regex]::Matches($bindingText, [regex]::Escape($cp391BindingName))
$cp392Matches = [regex]::Matches($bindingText, [regex]::Escape($cp392BindingName))
if ($cp390Matches.Count -ne 3 -or $cp391Matches.Count -ne 3 -or $cp392Matches.Count -ne 3 -or $cp390Matches[1].Index -lt $cp391Index -or $cp390Matches[1].Index -ge ($cp391Call.Index + $cp391Call.Length) -or $cp391Matches[1].Index -lt $cp392Index -or $cp391Matches[1].Index -ge ($cp392Call.Index + $cp392Call.Length) -or $cp390Matches[2].Index -le $numericalIndex -or $cp391Matches[2].Index -le $numericalIndex -or $cp392Matches[1].Index -le $cp392Call.Index -or $cp392Matches[1].Index -ge $numericalIndex -or $cp392Matches[2].Index -le $numericalIndex) {
    throw "Binding evidence references must be CP390=3, CP391=3, and CP392=3 with CP393 consumption and post-numerical storage"
}
if ([regex]::Matches($bindingText.Substring($cp390Index, $cp391Index - $cp390Index), [regex]::Escape($cp390BindingName)).Count -ne 1 -or
    [regex]::Matches($bindingText.Substring($cp391Index, $cp392Index - $cp391Index), [regex]::Escape($cp391BindingName)).Count -ne 1 -or
    [regex]::Matches($bindingText.Substring($cp392Index, $numericalIndex - $cp392Index), [regex]::Escape($cp392BindingName)).Count -ne 2) {
    throw "Binding CP390-to-CP391-to-CP392-to-CP393-to-CP396-to-numerical intervals are not exact"
}
$dto = Get-Cp391BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match [regex]::Escape($predecessorStem) -or $dto -match [regex]::Escape($stem) -or $dto -match [regex]::Escape($successorStem) -or $dto -match 'cp39[012]|preexisting_supply_enthalpy|maximum_supply_enthalpy|assigned_supply_enthalpy|resulting_supply_enthalpy|resulting_supply_temperature|supply_humidity_ratio') {
    throw "CP390/CP391/CP392 evidence unexpectedly feeds the numerical DTO"
}
Assert-NotContains -Path $adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node|prediction|feedback|\breport\b|ResultStore|numerical|supply_enthalpy_before_overdrying_limit_j_per_kg\s*:|supply_temperature_c\s*:' -Description "adapter numerical/scalar feed"
$coupledText = ($coupledFiles | ForEach-Object { Read-RepoText -Path $_ }) -join [Environment]::NewLine
foreach ($pattern in @("output\.$([regex]::Escape($cp390BindingName))", "output\.$([regex]::Escape($cp391BindingName))", 'predecessor_cp390:\s*&PredecessorLifecycle', 'predecessor\.source_order', 'OVERDRYING_LIMIT_SOURCE_ORDER') ) {
    Assert-Cp391Text -Text $coupledText -Pattern $pattern -Description "coupled CP390/CP391 evidence"
}
foreach ($path in $coupledFiles) {
    Assert-NotContains -Path $path -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|\.coupling\b|\.prediction\b|\.feedback\b|supply_node|\bload\b|\breport\b|ResultStore|reconcil|latest_numerical' -Description "coupled numerical feed"
}

foreach ($registration in @(
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\calc.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $binding; Pattern = "mod $stem;" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"; Pattern = "pub $($cp391BindingName):" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"; Pattern = "$($stem)_tests.rs" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"; Pattern = "mod $($stem)_validation;" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $pipelineRoot; Pattern = $pipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}
Assert-Contains -Path $bindingTests -Pattern 'binding_releases_cp391_after_cp390_without_mutating_direct_numerical_state' -Description "binding order/nonfeed test"
foreach ($pattern in @(
        'cp391_preserves_cp390_enthalpy_and_temperature_on_representative_direct_routes',
        'cp391_rejects_one_bit_drift_in_cp390_retained_result', 'cp391_rejects_cp390_source_order(?:_and_route)?_corruption',
        'cp391_rejects_(?:cp390_source_order_and_route_corruption|route_drift_and_remains_evidence_only)'
    )) {
    Assert-Contains -Path $coupledTests -Pattern $pattern -Description "coupled regression"
}

$lifecycleField = "purchased_air_calc_$($stem)_lifecycle"
$pipelineText = Read-RepoText -Path $pipelineRoot
$testBoundary = [regex]::Match($pipelineText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
if (-not $testBoundary.Success) { throw "CP391 pipeline production/test boundary missing" }
$production = $pipelineText.Substring(0, $testBoundary.Index)
$execute = Get-Cp391BraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
if ([regex]::Matches($execute, [regex]::Escape($lifecycleField) + '\s*:\s*None').Count -ne 3 -or
    [regex]::Matches($execute, 'let\s+' + [regex]::Escape($lifecycleField) + '\s*=\s*Some\s*\(').Count -ne 1) {
    throw "Pipeline must expose one direct CP391 Some and three non-direct None constructors"
}
$firewall = Get-Cp391BraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($lifecycleField) + '\s*\.\s*is_some').Count -ne 1) { throw "Non-direct firewall must reject CP391 evidence exactly once" }
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp422_lifecycle_evidence' -Description "cumulative non-direct firewall test"
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor\.source_order\s*(?:==|!=)\s*PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER' -Description "pipeline CP390 source-order provenance"
Assert-Cp391Text -Text $coupledText -Pattern 'predecessor\.source_order\s*(?:==|!=)\s*PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER' -Description "coupled CP390 source-order provenance"
foreach ($path in @($pipelineValidation) + $coupledFiles) {
    Assert-NotContains -Path $path -Pattern 'SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER|TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER|purchased_air_calc_.*(?:cp389|cp385|cp379|cp329).*_lifecycle' -Description "older direct lifecycle substitution"
}
foreach ($pattern in @('transition_count', 'inactive_transition_count', 'source_site_execution_count', 'predecessor_route_counts', 'cp390_supply_enthalpy_state_owner_count', 'unchanged_supply_enthalpy_preservation_count')) {
    Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline count $pattern"
}
foreach ($pattern in @('missing.*CP390|CP390.*missing', 'source_order|source order', 'route', 'to_bits|bit')) {
    Assert-Contains -Path $pipelineValidationTests -Pattern $pattern -Description "pipeline validation regression"
}

$snapshotJsonText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 33 -or $ieeeSidecars.Count -ne 33) { throw "CP391 serialization must expose exactly 33 numeric projections and 33 IEEE sidecars" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $expected = $numericFields[$index]
    if ($jsonNumbers[$index].Groups['field'].Value -cne $expected -or $jsonNumbers[$index].Groups['value'].Value -cne $expected -or
        $ieeeSidecars[$index].Groups['field'].Value -cne $expected -or $ieeeSidecars[$index].Groups['value'].Value -cne $expected) {
        throw "CP391 numeric/IEEE serialization field $($index + 1) must be exact '$expected'"
    }
}
Assert-Contains -Path $snapshotJson -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "finite-only numeric projection"
Assert-Contains -Path $snapshotJson -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "authoritative IEEE bits"
Assert-Contains -Path $cp390Assertions -Pattern 'mod cp391_assertions;' -Description "arbitrary CP391 module"
Assert-Contains -Path $cp390Assertions -Pattern 'cp391_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp390Assertions -Pattern 'cp391_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-Contains -Path $cp391Assertions -Pattern 'CP391 lifecycle must remain outside numerical result state' -Description "terminal numerical nonfeed"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP391 supersedes only CP390[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP391 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) { throw "CP391 must have exactly two algorithm and two capability addenda" }
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, 'physical executable line 2283', 'SupplyEnthalpy\s*=\s*max\(SupplyEnthalpy,\s*PsyHFnTdbW\(PurchAir\.SupplyTemp,\s*0\.00001\)\);',
            'physical executable line 2284', 'PurchAir\.SupplyHumRat\s*=\s*PsyWFnTdbH\(state,\s*PurchAir\.SupplyTemp,\s*SupplyEnthalpy,\s*RoutineName\);',
            $sites[0], $sites[1], $sites[2], $sites[3], $sites[4], 'T391=T390', 'O391=L390', '5\*O391',
            'thirty', 'three', 'twenty-seven', 'Seventeen', 'fourteen', 'thirteen', 'Eleven', 'nineteen', '18', '22', '28',
            'sole immediate predecessor', 'CP390', 'CP385/CP379', 'CP389/CP329', 'energyplus_psy_h_fn_tdb_w',
            '0x3ee4f8b588e368f1', 'if left < right \{ right \} else \{ left \}', 'f64::max', 'left NaN', 'right NaN',
            '33', 'CP390-to-CP391-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput',
            '329 total', '240 public', '89 internal', 'zero unused', 'zero unreachable', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP391 spec addendum missing '$pattern'" }
    }
}

$docs = @(
    "docs\src\current\current-status.md", "docs\src\current\project-contract.md",
    "docs\src\porting-map\ideal-loads-source-map.md", "docs\src\porting-map\heat-balance-source-map.md",
    "docs\src\porting-map\zone-air-update-map.md"
)
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    $sections = [regex]::Matches($text, '(?ms)^## CP391\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP391 documentation expected one section in $doc" }
    foreach ($pattern in @(
            $commit, $hash, '2283', 'SupplyEnthalpy\s*=\s*max\(SupplyEnthalpy,\s*PsyHFnTdbW\(PurchAir\.SupplyTemp,\s*0\.00001\)\);',
            '2284', 'PurchAir\.SupplyHumRat\s*=\s*PsyWFnTdbH\(state,\s*PurchAir\.SupplyTemp,\s*SupplyEnthalpy,\s*RoutineName\);',
            $sites[0], $sites[1], $sites[2], $sites[3], $sites[4], '30|thirty', '3|three', '27|twenty-seven',
            '17|seventeen', '14|fourteen', '13|thirteen', '11|eleven', '19|nineteen', '18', '22', '28',
            'T391\s*=\s*T390', 'O391\s*=\s*L390', '5\s*\*\s*O391', 'CP390', 'CP385', 'CP379', 'CP389', 'CP329',
            'energyplus_psy_h_fn_tdb_w', '0x3ee4f8b588e368f1', 'if left < right', 'f64::max',
            '33|thirty-three', 'CP390-to-CP391-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput',
            '329\s+total', '240\s+public', '89\s+internal', '238\s+development commands', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP391 documentation in $doc missing '$pattern'" }
    }
    $cp390DocIndex = $text.LastIndexOf("## CP390 ")
    $cp391DocIndex = $text.LastIndexOf("## CP391 ")
    if ($cp390DocIndex -lt 0 -or $cp391DocIndex -le $cp390DocIndex) { throw "CP390-to-CP391 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP391\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP391 supersedes only CP390' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP391 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..390) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp422_lifecycle_evidence' -Description "historical cumulative firewall"
}
foreach ($historical in 335..390) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 360 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 120 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..390) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 360' -Description "historical inventory total"
}
foreach ($historical in 367..390) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 120' -Description "historical internal classification"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 120 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP391 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp390Call\s*=', '\$cp391Call\s*=', '\$cp392Call\s*=', 'CP390-to-CP391', 'CP391-to-CP392', 'CP396-to-CP397', 'CP397-to-CP398', 'CP400-to-CP401', 'CP401-to-CP402', 'CP402-to-CP403', 'CP403-to-CP404')) { Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "strict CP345 terminal ordering" }
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 fixed structural cap"
foreach ($historical in 377..390) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP390-to-CP391' -Description "historical predecessor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP391-to-CP392' -Description "historical successor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP396-to-CP397' -Description "historical CP396-to-CP397 interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP397-to-CP398' -Description "historical CP397-to-CP398 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP400-to-CP401' -Description "historical CP400-to-CP401 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP401-to-CP402' -Description "historical CP401-to-CP402 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP402-to-CP403' -Description "historical CP402 terminal interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP403-to-CP404' -Description "historical CP402 terminal interval"
}
foreach ($historical in 385..390) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp391Index\s*=' -Description "historical binding CP391 index"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp391AuditIndex\s*=' -Description "historical master CP391 index"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp392Index\s*=' -Description "historical binding CP392 index"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp392AuditIndex\s*=' -Description "historical master CP392 index"
}

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp390AuditIndex = $master.IndexOf("cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1")
$cp391AuditIndex = $master.IndexOf("cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp390AuditIndex -lt 0 -or $cp391AuditIndex -le $cp390AuditIndex -or $cp392AuditIndex -le $cp391AuditIndex -or $completionIndex -le $cp392AuditIndex) { throw "Master audit must dot-source CP390, CP391, then CP392 before completion" }
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 360', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) { Assert-Cp391Text -Text $inventory -Pattern $pattern -Description "inventory" }
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 120) { throw "CP391 inventory must be exactly 240 public and 120 internal scripts" }
Assert-Cp391Text -Text $inventory -Pattern ('path = "scripts/quality/ideal-loads-structure-audit/' + [regex]::Escape((Split-Path -Leaf $audit)) + '"') -Description "inventory record"
foreach ($pattern in @('\| 360 \|', '\| public scripts \| 240 \|', '\| 120 \|', '\| scripts without callers \| 0 \|')) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory"
}

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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-numerical' -Description 'CP422 terminal interval'
Write-Host "CP391 post-saturation constant-SHR supply-enthalpy overdrying-limit structure audit passed."
}
