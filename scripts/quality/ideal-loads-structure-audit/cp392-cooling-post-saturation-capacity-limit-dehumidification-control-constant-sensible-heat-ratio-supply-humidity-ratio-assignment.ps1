# CP392 maps PurchasedAirManager.cc physical executable line 2284 and stops before 2285.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignment"
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
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$bindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp392.rs"
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
$cp391Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp391_assertions.rs"
$cp392Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp392_assertions.rs"
$psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1"
$sites = @(
    "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-constant-sensible-heat-ratio-overdrying-limit",
    "assign-purchased-air-supply-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit"
)
$numericFields = @(
    "predecessor_mixed_air_humidity_ratio", "predecessor_psychrometric_cp_air_result_j_per_kg_k",
    "predecessor_cp_air_j_per_kg_k", "predecessor_cooling_total_output_w",
    "predecessor_cooling_sensible_heat_ratio", "predecessor_calculated_cooling_sensible_output_w",
    "predecessor_cooling_sensible_output_w", "predecessor_resulting_supply_enthalpy_j_per_kg",
    "predecessor_preexisting_supply_temperature_c", "predecessor_mixed_air_temperature_c",
    "predecessor_cp389_cooling_sensible_output_w", "predecessor_cp389_cp_air_j_per_kg_k",
    "predecessor_supply_mass_flow_rate_kg_per_s", "predecessor_cp_air_times_supply_mass_flow_rate_w_per_k",
    "predecessor_cooling_sensible_output_over_air_capacity_rate_k", "predecessor_calculated_supply_temperature_c",
    "predecessor_assigned_supply_temperature_c", "predecessor_resulting_supply_temperature_c",
    "predecessor_cp390_resulting_supply_enthalpy_j_per_kg", "preexisting_supply_temperature_c",
    "supply_temperature_before_mixed_air_limit_c", "mixed_air_temperature_c", "minimum_supply_temperature_c",
    "assigned_supply_temperature_c", "predecessor_cp390_resulting_supply_temperature_c",
    "preexisting_supply_enthalpy_j_per_kg", "supply_enthalpy_before_overdrying_limit_j_per_kg",
    "predecessor_cp391_supply_temperature_c", "psychrometric_minimum_supply_enthalpy_j_per_kg",
    "maximum_supply_enthalpy_j_per_kg", "assigned_supply_enthalpy_j_per_kg",
    "predecessor_cp391_resulting_supply_enthalpy_j_per_kg", "predecessor_cp391_resulting_supply_temperature_c",
    "supply_temperature_c", "supply_enthalpy_j_per_kg", "psychrometric_supply_humidity_ratio",
    "assigned_supply_humidity_ratio", "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg", "resulting_supply_temperature_c"
)
$localBools = @(
    "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed",
    "cp391_retained_supply_temperature_state_owned", "cp391_retained_supply_enthalpy_state_owned",
    "cp391_retained_supply_temperature_owned_read", "supply_temperature_for_humidity_ratio_inversion_read",
    "cp391_retained_supply_enthalpy_owned_read", "supply_enthalpy_for_humidity_ratio_inversion_read",
    "psychrometric_supply_humidity_ratio_evaluated", "supply_humidity_ratio_assignment_performed"
)
$stateFields = @(
    "transition_count", "inactive_transition_count",
    "dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count",
    "predecessor_route_counts", "source_site_execution_count",
    "cp391_supply_temperature_state_owner_count", "unchanged_supply_temperature_preservation_count",
    "cp391_supply_enthalpy_state_owner_count", "unchanged_supply_enthalpy_preservation_count",
    "supply_temperature_owned_read_count", "supply_temperature_for_humidity_ratio_inversion_read_count",
    "supply_enthalpy_owned_read_count", "supply_enthalpy_for_humidity_ratio_inversion_read_count",
    "psychrometric_supply_humidity_ratio_evaluation_count", "supply_humidity_ratio_assignment_write_count", "latest"
)

function Assert-Cp392Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP392 $Description missing '$Pattern'" }
}

function Get-Cp392BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP392 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP392 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP392 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $owners, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, $privateCharacterization, $tests, $adapter, $bindingTests,
    $coupled, $coupledTests, $fixture, $witness, $pipeline, $pipelineValidation,
    $pipelineValidationTests, $pipelineSerialization, $snapshotJson, $snapshotJsonTests,
    $cp391Assertions, $cp392Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP392 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP392 bounded file"
}
Assert-FileExists -Path $arbitraryRoot -Description "CP392 arbitrary-run test root"
Assert-FileExists -Path $psychrometrics -Description "CP392 canonical psychrometrics owner"
foreach ($directory in @($root, $coupledRoot, $pipelineDir)) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP392 bounded recursive file"
    }
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) { throw "CP392 PurchasedAirManager.cc SHA-256 drift" }
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2283].Trim() -cne 'PurchAir.SupplyHumRat = PsyWFnTdbH(state, PurchAir.SupplyTemp, SupplyEnthalpy, RoutineName);' -or
    $lines[2284].Trim() -cne '} break;') { throw "CP392 source slice or CP393 first-excluded break drift" }
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2284' -Description "mapped source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2285' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites -Description "exact four-site source order"

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp392BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot declaration"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 40) { throw "CP392 snapshot must expose exactly 40 Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP392 numeric field $($index + 1) expected '$($numericFields[$index])', found '$($actualNumericFields[$index])'" }
}
$localStart = $snapshotStruct.IndexOf("pub $($localBools[0]):")
if ($localStart -lt 0) { throw "CP392 local field boundary missing" }
[string[]]$actualLocalBools = @([regex]::Matches($snapshotStruct.Substring($localStart), 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*bool') | ForEach-Object { $_.Groups['field'].Value })
if ($actualLocalBools.Count -ne 9) { throw "CP392 snapshot must expose exactly nine CP392-local bool fields" }
for ($index = 0; $index -lt $localBools.Count; $index += 1) {
    if ($actualLocalBools[$index] -cne $localBools[$index]) { throw "CP392 local bool $($index + 1) expected '$($localBools[$index])', found '$($actualLocalBools[$index])'" }
}
if ($snapshotStruct.Substring($localStart) -match '(?i)case_(?:break|exit)|break_(?:executed|performed)|humidistat.*(?:executed|performed)') { throw "CP393 break/case-exit or Humidistat behavior leaked into CP392 local schema" }

$stateStruct = Get-Cp392BraceBlock -Text (Read-RepoText -Path $state) -AnchorPattern "pub\s+struct\s+$($typeStem)RuntimeState\s*" -Description "runtime state declaration"
[string[]]$actualStateFields = @([regex]::Matches($stateStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:') | ForEach-Object { $_.Groups['field'].Value })
[string[]]$expectedStateFields = @('system') + $stateFields
if ($actualStateFields.Count -ne $expectedStateFields.Count) { throw "CP392 runtime state public field count drift" }
for ($index = 0; $index -lt $expectedStateFields.Count; $index += 1) {
    if ($actualStateFields[$index] -cne $expectedStateFields[$index]) { throw "CP392 state field $($index + 1) expected '$($expectedStateFields[$index])'" }
}
Assert-Contains -Path $state -Pattern 'predecessor_route_counts\s*:\s*\[usize;\s*30\]' -Description "thirty route counters"

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'overdrying_limit_snapshot_route', 'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)', 'index\s*>=\s*3',
        'predecessor\.resulting_supply_temperature_c', 'predecessor\.resulting_supply_enthalpy_j_per_kg',
        'active_total\s*!=\s*Some\(assignments\)', 'temperature_total\s*!=\s*Some\(state\.cp391_supply_temperature_state_owner_count\)',
        'temperature_total\s*!=\s*Some\(state\.unchanged_supply_temperature_preservation_count\)',
        'enthalpy_total\s*!=\s*Some\(state\.cp391_supply_enthalpy_state_owner_count\)',
        'enthalpy_total\s*!=\s*Some\(state\.unchanged_supply_enthalpy_preservation_count\)',
        'expected_sites\s*!=\s*Some\(state\.source_site_execution_count\)',
        'active_counts\.into_iter\(\)\.any\(\|count\| count != assignments\)'
    )) { Assert-Cp392Text -Text $core -Pattern $pattern -Description "route/owner/algebra contract" }
Assert-Contains -Path $release -Pattern 'OverdryingLimitSnapshot as Predecessor' -Description "exact CP391 predecessor type"
Assert-Contains -Path $release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp391:\s*Predecessor,\s*\)' -Description "exact public predecessor-only arguments"
Assert-Contains -Path $release -Pattern 'overdrying_limit_snapshot_is_exact_direct_release\(predecessor_cp391\)' -Description "recursive CP391 direct release"
Assert-Contains -Path $prefix -Pattern 'overdrying_limit_is_consistent' -Description "recursive CP391 completion evidence"
Assert-NotContains -Path $release -Pattern 'DirectZonePurchasedAirCouplingInput|latest_numerical|numerical_supply|final_supply|predecessor_cp39[0]|cp38[59]\s*:|cp379\s*:|cp329\s*:|supply_temperature_c:\s*f64|supply_enthalpy_j_per_kg:\s*f64' -Description "older owner, scalar, or numerical substitution"

Assert-Contains -Path $transition -Pattern 'use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;' -Description "canonical PsyW import"
Assert-Contains -Path $transition -Pattern '(?s)prepared\.active\.map\(\|active\|\s*\{?\s*energyplus_psy_w_fn_tdb_h\(active\.supply_temperature_c,\s*active\.supply_enthalpy_j_per_kg\)' -Description "canonical PsyW call"
Assert-Contains -Path $transition -Pattern 'resulting_supply_enthalpy_j_per_kg:\s*prepared\.resulting_supply_enthalpy_j_per_kg' -Description "unchanged enthalpy carrier"
Assert-Contains -Path $transition -Pattern 'resulting_supply_temperature_c:\s*prepared\.resulting_supply_temperature_c' -Description "unchanged temperature carrier"
Assert-NotContains -Path $transition -Pattern 'fn\s+energyplus_psy_w_fn_tdb_h|PsyWFnTdbH|is_finite|is_nan|f64::(?:min|max)|\.min\s*\(|\.max\s*\(|\.clamp\s*\(|mul_add|total_cmp|partial_cmp|normalize|coerce|cache|EP_psych|RoutineName|CalledFrom|SuppressWarnings' -Description "duplicate, gated, regrouped, or stateful PsyW path"
$psychText = Read-RepoText -Path $psychrometrics
$psyW = Get-Cp392BraceBlock -Text $psychText -AnchorPattern 'pub\s+fn\s+energyplus_psy_w_fn_tdb_h\s*\(' -Description "canonical PsyW helper"
Assert-Cp392Text -Text $psyW -Pattern '(?s)let humidity_ratio\s*=\s*\(enthalpy_j_per_kg\s*-\s*1\.004_84e3\s*\*\s*dry_bulb_c\)\s*/\s*\(2\.500_94e6\s*\+\s*1\.858_95e3\s*\*\s*dry_bulb_c\);\s*if humidity_ratio < 0\.0\s*\{\s*ENERGYPLUS_MIN_HUMIDITY_RATIO\s*\}\s*else\s*\{\s*humidity_ratio\s*\}' -Description "canonical PsyW grouping and strict floor"
if ($psyW -match 'is_finite|is_nan|\.clamp|\.min|\.max|mul_add|total_cmp|partial_cmp') { throw "Canonical PsyW helper gained a finite gate or alternate arithmetic" }
Assert-Contains -Path $psychrometrics -Pattern 'const ENERGYPLUS_MIN_HUMIDITY_RATIO:\s*f64\s*=\s*1\.0e-5;' -Description "canonical floor literal"
Assert-Contains -Path $snapshotValidation -Pattern '(?s)energyplus_psy_w_fn_tdb_h\(temperature,\s*enthalpy\).*?temperature\.to_bits\(\) != predecessor\.resulting_supply_temperature_c\?\.to_bits\(\).*?enthalpy\.to_bits\(\) != predecessor\.resulting_supply_enthalpy_j_per_kg\?\.to_bits\(\).*?psychrometric\.to_bits\(\) != expected\.to_bits\(\).*?assigned\.to_bits\(\) != psychrometric\.to_bits\(\).*?resulting\.to_bits\(\) != assigned\.to_bits\(\)' -Description "active IEEE exactness"

$coreProductionFiles = @($module) + @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs' | Where-Object { $_.FullName -notmatch '[\\/]tests\.rs$' -and $_.FullName -notmatch '[\\/]tests[\\/]' } | ForEach-Object { $_.FullName })
$integrationProductionFiles = @($adapter, $coupled, $pipeline) + @(Get-ChildItem -LiteralPath $coupledRoot -Recurse -File -Filter '*.rs' | Where-Object { $_.FullName -notmatch '[\\/]tests\.rs$' -and $_.FullName -notmatch '[\\/]tests[\\/]' } | ForEach-Object { $_.FullName }) + @(Get-ChildItem -LiteralPath $pipelineDir -Recurse -File -Filter '*.rs' | Where-Object { $_.FullName -notmatch '[\\/]tests\.rs$' -and $_.FullName -notmatch '[\\/]tests[\\/]' } | ForEach-Object { $_.FullName })
$cp393BehaviorPattern = '(?i)\bCP393\b|PurchasedAirManager\.cc:2288|line[-_ ]?2288|case_(?:break|exit)|(?:break|case_exit)_(?:site|flag|counter|executed|performed|count)|\bbreak\s*;|(?:source_order|source_site)[A-Za-z0-9_\s:]*break'
foreach ($path in @($coreProductionFiles + $integrationProductionFiles | Select-Object -Unique)) { Assert-NotContains -Path $path -Pattern $cp393BehaviorPattern -Description "recursive CP393 break/case-exit firewall" }
$humidistatBehaviorPattern = '(?i)(?:DehumidificationControlType|HumControl)::Humidistat\s*(?:=>|==|!=|\{)|case\s+HumControl::Humidistat'
$behaviorIntegrationFiles = @($adapter, $coupled, $pipeline, $pipelineValidation) + @(Get-ChildItem -LiteralPath $coupledRoot -Recurse -File -Filter '*.rs' | Where-Object { $_.FullName -notmatch '[\\/]tests\.rs$' -and $_.FullName -notmatch '[\\/]tests[\\/]' } | ForEach-Object { $_.FullName }) + @(Get-ChildItem -LiteralPath "$pipelineDir\validation" -Recurse -File -Filter '*.rs' | Where-Object { $_.FullName -notmatch '[\\/]tests\.rs$' -and $_.FullName -notmatch '[\\/]tests[\\/]' } | ForEach-Object { $_.FullName })
foreach ($path in @($coreProductionFiles + $behaviorIntegrationFiles | Select-Object -Unique)) { Assert-NotContains -Path $path -Pattern $humidistatBehaviorPattern -Description "CP392 Humidistat case-entry/line-2288 behavior firewall" }
foreach ($path in @($transition, $release, $adapter, $pipelineValidation, $coupled) + @(Get-ChildItem -LiteralPath $coupledRoot -Recurse -File -Filter '*.rs' | ForEach-Object { $_.FullName })) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine + ((Get-ChildItem -LiteralPath "$root\tests" -Recurse -File -Filter '*.rs' | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine)
foreach ($pattern in @(
        'cp392_boundaries_and_physical_four_site_order_are_exact', 'cp392_preserves_thirty_routes_and_assigns_exactly_three',
        'canonical_inverse_uses_strict_negative_floor_without_extra_normalization',
        'assert_eq!\(snapshots\.len\(\),\s*30\)', 'assert_eq!\(state\.transition_count,\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*27\)', 'assert_eq!\(state\.assignment_count\(\),\s*3\)',
        'assert_eq!\(state\.predecessor_route_counts,\s*\[1;\s*30\]\)',
        'assert_eq!\(state\.source_site_execution_count,\s*12\)',
        '(?s)cp391_supply_temperature_state_owner_count,\s*27', '(?s)unchanged_supply_temperature_preservation_count,\s*27',
        '(?s)cp391_supply_enthalpy_state_owner_count,\s*17', '(?s)unchanged_supply_enthalpy_preservation_count,\s*17',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)', 'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'usize::MAX', '0x3ee4_f8b5_88e3_68f1', 'f64::NEG_INFINITY', 'f64::INFINITY', '-0\.0',
        'positive_subfloor|below_floor_but_positive', 'nan|NaN', 'pole', 'to_bits\(\)'
    )) { Assert-Cp392Text -Text $testText -Pattern $pattern -Description "route/count/overflow/IEEE test" }

$bindingText = Read-RepoText -Path $binding
$cp391BindingName = "calculation_$predecessorStem"
$cp392BindingName = "calculation_$stem"
$cp391Index = $bindingText.IndexOf("let $cp391BindingName =")
$cp392Index = $bindingText.IndexOf("let $cp392BindingName =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp391Index -lt 0 -or $cp392Index -le $cp391Index -or $numericalIndex -le $cp392Index) { throw "Binding must execute CP391, CP392, then unchanged numerical coupling" }
$cp392Call = [regex]::Match($bindingText, "(?s)let $([regex]::Escape($cp392BindingName)) =\s*advance_$([regex]::Escape($stem))\((?<args>.*?)\)\?;")
if (-not $cp392Call.Success -or [regex]::Matches($cp392Call.Groups['args'].Value, [regex]::Escape($cp391BindingName)).Count -ne 1) { throw "CP392 must consume CP391 exactly once as sole immediate predecessor" }
$cp391Matches = [regex]::Matches($bindingText, [regex]::Escape($cp391BindingName))
$cp392Matches = [regex]::Matches($bindingText, [regex]::Escape($cp392BindingName))
if ($cp391Matches.Count -ne 3 -or $cp392Matches.Count -ne 3 -or $cp391Matches[1].Index -lt $cp392Index -or $cp391Matches[1].Index -ge ($cp392Call.Index + $cp392Call.Length) -or $cp391Matches[2].Index -le $numericalIndex -or $cp392Matches[1].Index -le $cp392Call.Index -or $cp392Matches[1].Index -ge $numericalIndex -or $cp392Matches[2].Index -le $numericalIndex) { throw "Binding references must be CP391=3 and CP392=3 with CP393 consumption and post-numerical storage" }
if ([regex]::Matches($bindingText.Substring($cp391Index, $cp392Index - $cp391Index), [regex]::Escape($cp391BindingName)).Count -ne 1 -or
    [regex]::Matches($bindingText.Substring($cp392Index, $numericalIndex - $cp392Index), [regex]::Escape($cp392BindingName)).Count -ne 2) { throw "Binding CP391-to-CP392-to-CP393-to-CP396-to-numerical intervals are not exact" }
$dto = Get-Cp392BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match [regex]::Escape($stem) -or $dto -match 'cp392|supply_humidity_ratio|psychrometric_supply_humidity') { throw "CP392 evidence unexpectedly feeds the numerical DTO" }
Assert-NotContains -Path $adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node|prediction|feedback|\breport\b|ResultStore|numerical|supply_temperature_c\s*:|supply_enthalpy_j_per_kg\s*:' -Description "adapter numerical/scalar feed"

$coupledFiles = @($coupled) + @(Get-ChildItem -LiteralPath $coupledRoot -Recurse -File -Filter '*.rs' | ForEach-Object { $_.FullName })
$coupledText = ($coupledFiles | ForEach-Object { Read-RepoText -Path $_ }) -join [Environment]::NewLine
foreach ($pattern in @("output\.$([regex]::Escape($cp391BindingName))", "output\.$([regex]::Escape($cp392BindingName))", 'predecessor_cp391:\s*&PredecessorLifecycle', 'OVERDRYING_LIMIT_SOURCE_ORDER')) { Assert-Cp392Text -Text $coupledText -Pattern $pattern -Description "coupled CP391/CP392 evidence" }
foreach ($path in $coupledFiles) { Assert-NotContains -Path $path -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|\.coupling\b|\.prediction\b|\.feedback\b|supply_node|\bload\b|\breport\b|ResultStore|reconcil|latest_numerical' -Description "coupled numerical feed" }
Assert-Contains -Path $bindingTests -Pattern 'binding_releases_cp392_after_cp391_without_mutating_direct_numerical_state' -Description "binding order/nonfeed test"
foreach ($pattern in @('cp392_preserves_cp391_enthalpy_temperature_and_nulls_source_locals_on_direct_routes', 'cp392_rejects_one_bit_drift_in_cp391_retained_result', 'cp392_rejects_cp391_source_order_corruption', 'cp392_rejects_route_drift_and_remains_evidence_only')) { Assert-Contains -Path $coupledTests -Pattern $pattern -Description "coupled regression" }

foreach ($registration in @(
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\calc.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $binding; Pattern = "mod $stem;" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"; Pattern = "pub $($cp392BindingName):" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"; Pattern = "$($stem)_tests.rs" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"; Pattern = "mod $($stem)_validation;" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $pipelineRoot; Pattern = $pipelineStem }
    )) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration" }

$lifecycleField = "purchased_air_calc_$($stem)_lifecycle"
$pipelineText = Read-RepoText -Path $pipelineRoot
$testBoundary = [regex]::Match($pipelineText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
if (-not $testBoundary.Success) { throw "CP392 pipeline production/test boundary missing" }
$production = $pipelineText.Substring(0, $testBoundary.Index)
$execute = Get-Cp392BraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
if ([regex]::Matches($execute, [regex]::Escape($lifecycleField) + '\s*:\s*None').Count -ne 3 -or [regex]::Matches($execute, 'let\s+' + [regex]::Escape($lifecycleField) + '\s*=\s*Some\s*\(').Count -ne 1) { throw "Pipeline must expose one direct CP392 Some and three non-direct None constructors" }
$firewall = Get-Cp392BraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($lifecycleField) + '\s*\.\s*is_some').Count -ne 1) { throw "Non-direct firewall must reject CP392 evidence exactly once" }
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp407_lifecycle_evidence' -Description "cumulative non-direct firewall test"
Assert-Contains -Path $pipelineValidation -Pattern 'CP392 CP391 evidence is missing|missing CP391' -Description "pipeline missing-CP391 fail closed"
Assert-Contains -Path "$pipelineDir\validation\lineage.rs" -Pattern 'OVERDRYING_LIMIT_SOURCE_ORDER' -Description "pipeline exact CP391 provenance"
foreach ($pattern in @('transition_count', 'inactive_transition_count', 'source_site_execution_count', 'predecessor_route_counts', 'cp391_supply_temperature_state_owner_count', 'cp391_supply_enthalpy_state_owner_count')) { Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline count $pattern" }
foreach ($pattern in @('ep_run_cp392_rejects_missing_cp391_predecessor_evidence', 'ep_run_cp392_links_exactly_to_cp391_and_rejects_corruption', 'source_order', 'to_bits|bit')) { Assert-Contains -Path $pipelineValidationTests -Pattern $pattern -Description "pipeline validation regression" }

$snapshotJsonText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 40 -or $ieeeSidecars.Count -ne 40) { throw "CP392 serialization must expose exactly 40 numeric projections and 40 IEEE sidecars" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $expected = $numericFields[$index]
    if ($jsonNumbers[$index].Groups['field'].Value -cne $expected -or $jsonNumbers[$index].Groups['value'].Value -cne $expected -or $ieeeSidecars[$index].Groups['field'].Value -cne $expected -or $ieeeSidecars[$index].Groups['value'].Value -cne $expected) { throw "CP392 numeric/IEEE serialization field $($index + 1) must be exact '$expected'" }
}
$directJsonFields = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*,?\s*$')
$localJsonBools = @($directJsonFields | Where-Object { $localBools -contains $_.Groups['field'].Value })
if ($localJsonBools.Count -ne 9) { throw "CP392 serialization must expose exactly nine local booleans" }
for ($index = 0; $index -lt $localBools.Count; $index += 1) {
    $expected = $localBools[$index]
    if ($localJsonBools[$index].Groups['field'].Value -cne $expected -or $localJsonBools[$index].Groups['value'].Value -cne $expected) { throw "CP392 local JSON boolean $($index + 1) must be exact '$expected'" }
}
Assert-Contains -Path $snapshotJson -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "finite-only numeric projection"
Assert-Contains -Path $snapshotJson -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "authoritative IEEE bits"
Assert-Contains -Path $snapshotJsonTests -Pattern 'all_forty_nonfinite_fields_serialize_as_null_with_exact_ieee_sidecars' -Description "forty-field nonfinite serialization regression"
Assert-Contains -Path $snapshotJsonTests -Pattern 'direct_skip_retains_signed_zero_enthalpy_and_temperature' -Description "direct signed-zero/local-null regression"
Assert-Contains -Path $cp391Assertions -Pattern 'mod cp392_assertions;' -Description "arbitrary CP392 module"
Assert-Contains -Path $cp391Assertions -Pattern 'cp392_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp391Assertions -Pattern 'cp392_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-Contains -Path $cp392Assertions -Pattern 'CP392 lifecycle must remain outside numerical result state' -Description "terminal numerical nonfeed"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP392 supersedes only CP391[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP392 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) { throw "CP392 must have exactly two algorithm and two capability addenda" }
$algorithmBlocks = [regex]::Matches($algorithmText, '(?ms)^\[\[algorithm\]\]\s*.*?(?=^\[\[algorithm\]\]|\z)')
foreach ($parentId in @('zone_temp_predictor_corrector_source_order', 'ideal_loads_zone_equipment_purchased_air_source_order')) {
    $parent = @($algorithmBlocks | Where-Object { $_.Value -match ('(?m)^id = "' + [regex]::Escape($parentId) + '"$') })
    if ($parent.Count -ne 1 -or [regex]::Matches($parent[0].Value, '(?m)^\s*"CP391 supersedes only CP390[^"\r\n]+",\s*$').Count -ne 1 -or [regex]::Matches($parent[0].Value, '(?m)^\s*"CP392 supersedes only CP391[^"\r\n]+",\s*$').Count -ne 1) { throw "CP392 algorithm addendum must occur once beside CP391 in '$parentId'" }
}
$capabilityBlocks = [regex]::Matches($capabilityText, '(?ms)^\[\[capability\]\]\s*.*?(?=^\[\[capability\]\]|\z)')
foreach ($parentId in @('ideal_loads_no_oa_sensible', 'ideal_loads_finite_limits')) {
    $parent = @($capabilityBlocks | Where-Object { $_.Value -match ('(?m)^id = "' + [regex]::Escape($parentId) + '"$') })
    if ($parent.Count -ne 1 -or [regex]::Matches($parent[0].Value, '(?m)^\s*"CP391 additionally requires[^"\r\n]+",\s*$').Count -ne 1 -or [regex]::Matches($parent[0].Value, '(?m)^\s*"CP392 additionally requires[^"\r\n]+",\s*$').Count -ne 1) { throw "CP392 capability addendum must occur once beside CP391 in '$parentId'" }
}
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, 'physical executable line 2284', 'PurchAir\.SupplyHumRat\s*=\s*PsyWFnTdbH\(state,\s*PurchAir\.SupplyTemp,\s*SupplyEnthalpy,\s*RoutineName\);',
            'line 2285', '\}\s*break;', $sites[0], $sites[1], $sites[2], $sites[3], 'T392=T391', '4\*A392',
            'thirty', 'three', 'twenty-seven', '17', '27', '12', '18', '22', '28', 'sole immediate predecessor', 'CP391',
            'energyplus_psy_w_fn_tdb_h', '0x3ee4f8b588e368f1', 'positive sub-floor', 'negative zero', 'NaN', 'positive infinity', 'pole', 'negative infinity',
            '40', 'nine', 'CP391-to-CP392-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput', 'CP393',
            '330 total', '240 public', '90 internal', 'zero unused', 'zero unreachable', '238 development commands', 'Roadmap'
        )) { if ($claim.Value -notmatch $pattern) { throw "CP392 spec addendum missing '$pattern'" } }
}

$docs = @(
    "docs\src\current\current-status.md", "docs\src\current\project-contract.md",
    "docs\src\porting-map\ideal-loads-source-map.md", "docs\src\porting-map\heat-balance-source-map.md",
    "docs\src\porting-map\zone-air-update-map.md"
)
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    $sections = [regex]::Matches($text, '(?ms)^## CP392\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP392 documentation expected one section in $doc" }
    foreach ($pattern in @(
            $commit, $hash, '2284', 'PurchAir\.SupplyHumRat\s*=\s*PsyWFnTdbH\(state,\s*PurchAir\.SupplyTemp,\s*SupplyEnthalpy,\s*RoutineName\);',
            '2285', '\}\s*break;', $sites[0], $sites[1], $sites[2], $sites[3], '30|thirty', '3|three', '27|twenty-seven',
            '17|seventeen', '12', '18', '22', '28', 'T392\s*=\s*T391', '4\s*\*\s*A392', 'CP391',
            'energyplus_psy_w_fn_tdb_h', '0x3ee4f8b588e368f1', 'positive sub-floor', 'negative zero', 'NaN', 'infinity', 'pole',
            '40|forty', 'nine|9', 'CP391-to-CP392-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput', 'CP393',
            '330\s+total', '240\s+public', '90\s+internal', '238\s+development\s+commands', 'Roadmap'
        )) { if ($sections[0].Value -notmatch $pattern) { throw "CP392 documentation in $doc missing '$pattern'" } }
    $cp391DocIndex = $text.LastIndexOf("## CP391 ")
    $cp392DocIndex = $text.LastIndexOf("## CP392 ")
    if ($cp391DocIndex -lt 0 -or $cp392DocIndex -le $cp391DocIndex) { throw "CP391-to-CP392 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP392\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP392 supersedes only CP391' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP392 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..391) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp407_lifecycle_evidence' -Description "historical cumulative firewall"
}
foreach ($historical in 335..391) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 345 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 105 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..391) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 345' -Description "historical inventory total"
}
foreach ($historical in 367..391) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 105' -Description "historical internal classification"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 105 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP392 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp391Call\s*=', '\$cp392Call\s*=', 'CP391-to-CP392', 'CP396-to-CP397', 'CP397-to-CP398', 'CP400-to-CP401', 'CP401-to-CP402', 'CP402-to-CP403', 'CP403-to-CP404')) { Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "strict CP345 terminal ordering" }
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 fixed structural cap"
foreach ($historical in 377..391) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP391-to-CP392' -Description "historical successor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP396-to-CP397' -Description "historical CP396-to-CP397 interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP397-to-CP398' -Description "historical CP397-to-CP398 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP400-to-CP401' -Description "historical CP400-to-CP401 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP401-to-CP402' -Description "historical CP401-to-CP402 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP402-to-CP403' -Description "historical CP402 terminal interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP403-to-CP404' -Description "historical CP402 terminal interval"
}
foreach ($historical in 385..391) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp392Index\s*=' -Description "historical binding CP392 index"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp392AuditIndex\s*=' -Description "historical master CP392 index"
}

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp391AuditIndex = $master.IndexOf("cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp391AuditIndex -lt 0 -or $cp392AuditIndex -le $cp391AuditIndex -or $completionIndex -le $cp392AuditIndex) { throw "Master audit must dot-source CP392 after CP391 before completion" }
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 345', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) { Assert-Cp392Text -Text $inventory -Pattern $pattern -Description "inventory" }
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 105) { throw "CP392 inventory must be exactly 240 public and 105 internal scripts" }
Assert-Cp392Text -Text $inventory -Pattern ('path = "scripts/quality/ideal-loads-structure-audit/' + [regex]::Escape((Split-Path -Leaf $audit)) + '"') -Description "inventory record"
foreach ($pattern in @('\| 345 \|', '\| public scripts \| 240 \|', '\| 105 \|', '\| scripts without callers \| 0 \|')) { Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory" }

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP404-to-CP405' -Description "CP345 CP404-to-CP405 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP405-to-CP406' -Description "CP345 CP405-to-CP406 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP406-to-CP407' -Description "CP345 CP406-to-CP407 interval"
Write-Host "CP392 post-saturation constant-SHR supply-humidity-ratio assignment structure audit passed."
}
