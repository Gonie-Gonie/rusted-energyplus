# CP395 maps PurchasedAirManager.cc physical executable line 2288 and stops before 2289.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignment"
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
$scheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$bindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupledRootFile = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation"
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp395.rs"
$fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipelineStem = "purchased_air_$stem"
$pipelineDir = "crates\ep_run\src\pipeline\$pipelineStem"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "$pipelineDir\validation.rs"
$pipelineValidationTests = "$pipelineDir\validation\tests.rs"
$pipelineSerialization = "$pipelineDir\serialization.rs"
$snapshotJson = "$pipelineDir\serialization\snapshot.rs"
$snapshotJsonTests = "$pipelineDir\serialization\snapshot\tests.rs"
$arbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp394Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp394_assertions.rs"
$cp395Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp395_assertions.rs"
$psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp395-cooling-post-saturation-capacity-limit-dehumidification-control-humidistat-supply-humidity-ratio-assignment.ps1"
$sites = @(
    "read-purchased-air-supply-temperature-for-humidistat-humidity-ratio-inversion",
    "read-local-supply-enthalpy-for-humidistat-humidity-ratio-inversion",
    "evaluate-psy-w-fn-tdb-h-for-humidistat-capacity-limit",
    "assign-purchased-air-supply-humidity-ratio-for-humidistat-capacity-limit"
)
$numericFields = @(
    "predecessor_cp393_resulting_supply_humidity_ratio",
    "predecessor_cp393_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp393_resulting_supply_temperature_c",
    "predecessor_cp394_resulting_supply_humidity_ratio",
    "predecessor_cp394_resulting_supply_enthalpy_j_per_kg",
    "predecessor_cp394_resulting_supply_temperature_c",
    "supply_temperature_c",
    "supply_enthalpy_j_per_kg",
    "psychrometric_supply_humidity_ratio",
    "assigned_supply_humidity_ratio",
    "resulting_supply_humidity_ratio",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_temperature_c"
)
$localBools = @(
    "dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed",
    "cp394_retained_supply_humidity_ratio_state_owned",
    "cp394_retained_supply_temperature_state_owned",
    "cp394_retained_supply_enthalpy_state_owned",
    "cp394_retained_supply_temperature_owned_read",
    "supply_temperature_for_humidity_ratio_inversion_read",
    "cp394_retained_supply_enthalpy_owned_read",
    "supply_enthalpy_for_humidity_ratio_inversion_read",
    "psychrometric_supply_humidity_ratio_evaluated",
    "supply_humidity_ratio_assignment_performed"
)
$stateFields = @(
    "transition_count", "inactive_transition_count",
    "dehumidification_control_humidistat_supply_humidity_ratio_assignment_count",
    "predecessor_route_counts", "source_site_execution_count",
    "cp394_supply_humidity_ratio_state_owner_count", "unchanged_supply_humidity_ratio_preservation_count",
    "cp394_supply_temperature_state_owner_count", "unchanged_supply_temperature_preservation_count",
    "cp394_supply_enthalpy_state_owner_count", "unchanged_supply_enthalpy_preservation_count",
    "supply_temperature_owned_read_count", "supply_temperature_for_humidity_ratio_inversion_read_count",
    "supply_enthalpy_owned_read_count", "supply_enthalpy_for_humidity_ratio_inversion_read_count",
    "psychrometric_supply_humidity_ratio_evaluation_count", "supply_humidity_ratio_assignment_write_count", "latest"
)

function Assert-Cp395Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP395 $Description missing '$Pattern'" }
}

function Get-Cp395BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP395 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "CP395 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP395 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $owners, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, $privateCharacterization, $tests,
    $adapter, $bindingTests, $coupled, $coupledTests, $fixture, $witness,
    $pipeline, $pipelineValidation, $pipelineValidationTests, $pipelineSerialization, $snapshotJson,
    $snapshotJsonTests, $cp394Assertions, $cp395Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP395 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP395 bounded file"
}
Assert-FileExists -Path $arbitraryRoot -Description "CP395 arbitrary-run test root"
Assert-FileExists -Path $psychrometrics -Description "CP395 canonical psychrometrics owner"
foreach ($directory in @($root, $coupledRoot, $pipelineDir)) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP395 bounded recursive file"
    }
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) { throw "CP395 PurchasedAirManager.cc SHA-256 drift" }
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2287].Trim() -cne 'PurchAir.SupplyHumRat = PsyWFnTdbH(state, PurchAir.SupplyTemp, SupplyEnthalpy, RoutineName);' -or
    $lines[2288].Trim() -cne '} break;' -or
    $lines[2312].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP395 source slice, CP396 boundary, or dynamic continuation drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2288' -Description "mapped assignment"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2289' -Description "first excluded break"
Assert-Contains -Path $module -Pattern 'line 2313' -Description "dynamic continuation note"
Assert-ExactStringArray -Path $module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER' -Expected $sites -Description "exact four sites"

$snapshotStruct = Get-Cp395BraceBlock -Text (Read-RepoText -Path $module) -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description "snapshot"
[string[]]$actualNumericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($actualNumericFields.Count -ne 13) { throw "CP395 snapshot must expose exactly thirteen Option<f64> fields" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    if ($actualNumericFields[$index] -cne $numericFields[$index]) { throw "CP395 numeric field order drift at $index" }
}
$localStart = $snapshotStruct.IndexOf("pub $($localBools[0]):")
if ($localStart -lt 0) { throw "CP395 local field boundary missing" }
[string[]]$actualLocalBools = @([regex]::Matches($snapshotStruct.Substring($localStart), 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*bool') | ForEach-Object { $_.Groups['field'].Value })
if ($actualLocalBools.Count -ne $localBools.Count) { throw "CP395 local boolean count drift" }
for ($index = 0; $index -lt $localBools.Count; $index += 1) {
    if ($actualLocalBools[$index] -cne $localBools[$index]) { throw "CP395 local boolean order drift at $index" }
}

$localSnapshot = $snapshotStruct.Substring($localStart)
if ($localSnapshot -match '(?i)humidistat_case_(?:break|exit)|(?:break|case_exit|saturation_continuation)_(?:executed|performed|count)|line_?2289') {
    throw "CP396 break or line-2313 continuation behavior leaked into CP395 local schema"
}

$stateStruct = Get-Cp395BraceBlock -Text (Read-RepoText -Path $state) -AnchorPattern "pub\s+struct\s+$($typeStem)RuntimeState\s*" -Description "runtime state"
[string[]]$actualStateFields = @([regex]::Matches($stateStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:') | ForEach-Object { $_.Groups['field'].Value })
[string[]]$expectedStateFields = @("system") + $stateFields
if ($actualStateFields.Count -ne $expectedStateFields.Count) { throw "CP395 runtime state public field count drift" }
for ($index = 0; $index -lt $expectedStateFields.Count; $index += 1) {
    if ($actualStateFields[$index] -cne $expectedStateFields[$index]) { throw "CP395 state field order drift at $index" }
}
Assert-Contains -Path $state -Pattern 'predecessor_route_counts\s*:\s*\[usize;\s*30\]' -Description "thirty route counters"

$coreFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter "*.rs")
$core = ($coreFiles | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine
foreach ($pattern in @(
        'humidistat_case_entry_snapshot_route',
        'matches!\(index,\s*19\s*\|\s*23\s*\|\s*26\)',
        'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'matches!\(index,\s*18\s*\|\s*19\s*\|\s*22\s*\|\s*23\s*\|\s*26\s*\|\s*28\)',
        'matches!\(index,\s*5\s*\|\s*8\s*\|\s*11\s*\|\s*14\s*\|\s*17\.\.=29\)',
        'index\s*>=\s*3', 'predecessor\.resulting_supply_humidity_ratio',
        'predecessor\.resulting_supply_enthalpy_j_per_kg', 'predecessor\.resulting_supply_temperature_c',
        'active_total\s*!=\s*Some\(assignments\)',
        'humidity_total\s*!=\s*Some\(state\.cp394_supply_humidity_ratio_state_owner_count\)',
        'humidity_total\s*!=\s*Some\(state\.unchanged_supply_humidity_ratio_preservation_count\)',
        'temperature_total\s*!=\s*Some\(state\.cp394_supply_temperature_state_owner_count\)',
        'temperature_total\s*!=\s*Some\(state\.unchanged_supply_temperature_preservation_count\)',
        'enthalpy_total\s*!=\s*Some\(state\.cp394_supply_enthalpy_state_owner_count\)',
        'enthalpy_total\s*!=\s*Some\(state\.unchanged_supply_enthalpy_preservation_count\)',
        'expected_sites\s*!=\s*Some\(state\.source_site_execution_count\)',
        'active_counts\.into_iter\(\)\.any\(\|count\| count != assignments\)'
    )) { Assert-Cp395Text -Text $core -Pattern $pattern -Description "route, owner, or algebra contract" }

Assert-Contains -Path $release -Pattern 'HumidistatCaseEntrySnapshot as Predecessor' -Description "exact CP394 predecessor type"
Assert-Contains -Path $release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp394:\s*Predecessor,\s*\)' -Description "public predecessor-only arguments"
Assert-Contains -Path $release -Pattern 'humidistat_case_entry_snapshot_is_exact_direct_release\(predecessor_cp394\)' -Description "recursive CP394 exact-direct evidence"
Assert-Contains -Path $prefix -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_committed_latest_snapshot_is_consistent\s*\(' -Description "bounded CP394 committed predecessor proof"
Assert-NotContains -Path $prefix -Pattern 'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_is_consistent\s*\(' -Description "recursive CP394 predecessor completion"
Assert-NotContains -Path $release -Pattern 'DirectZonePurchasedAirCouplingInput|ActiveInput|latest_numerical|numerical_supply|final_supply|predecessor_cp393\s*:|cp392\s*:|cp385\s*:|cp379\s*:|cp329\s*:|supply_temperature_c:\s*f64|supply_enthalpy_j_per_kg:\s*f64' -Description "older owner, scalar, active-input, or numerical substitution"

Assert-Contains -Path $transition -Pattern 'use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;' -Description "canonical PsyW import"
Assert-Contains -Path $transition -Pattern '(?s)prepared\.active\.map\(\|active\|\s*\{?\s*energyplus_psy_w_fn_tdb_h\(active\.supply_temperature_c,\s*active\.supply_enthalpy_j_per_kg\)' -Description "canonical PsyW call"
Assert-Contains -Path $transition -Pattern 'resulting_supply_humidity_ratio\s*=\s*if route\.active\s*\{\s*psychrometric_supply_humidity_ratio\s*\}\s*else\s*\{\s*prepared\.predecessor_supply_humidity_ratio' -Description "active assignment and inactive humidity preservation"
Assert-Contains -Path $transition -Pattern 'resulting_supply_enthalpy_j_per_kg:\s*prepared\.predecessor_supply_enthalpy_j_per_kg' -Description "unchanged enthalpy carrier"
Assert-Contains -Path $transition -Pattern 'resulting_supply_temperature_c:\s*prepared\.predecessor_supply_temperature_c' -Description "unchanged temperature carrier"
Assert-NotContains -Path $transition -Pattern 'fn\s+energyplus_psy_w_fn_tdb_h|PsyWFnTdbH|DehumidificationControlType|HumControl|is_finite|is_nan|f64::(?:min|max)|\.min\s*\(|\.max\s*\(|\.clamp\s*\(|mul_add|total_cmp|partial_cmp|normalize|coerce|cache|EP_psych|RoutineName|CalledFrom|SuppressWarnings' -Description "duplicate, selector-reread, gated, regrouped, or stateful PsyW path"
$psyW = Get-Cp395BraceBlock -Text (Read-RepoText -Path $psychrometrics) -AnchorPattern 'pub\s+fn\s+energyplus_psy_w_fn_tdb_h\s*\(' -Description "canonical PsyW helper"
Assert-Cp395Text -Text $psyW -Pattern '(?s)let humidity_ratio\s*=\s*\(enthalpy_j_per_kg\s*-\s*1\.004_84e3\s*\*\s*dry_bulb_c\)\s*/\s*\(2\.500_94e6\s*\+\s*1\.858_95e3\s*\*\s*dry_bulb_c\);\s*if humidity_ratio < 0\.0\s*\{\s*ENERGYPLUS_MIN_HUMIDITY_RATIO\s*\}\s*else\s*\{\s*humidity_ratio\s*\}' -Description "canonical PsyW grouping and strict floor"
if ($psyW -match 'is_finite|is_nan|\.clamp|\.min|\.max|mul_add|total_cmp|partial_cmp') { throw "CP395 canonical PsyW helper gained alternate arithmetic" }
Assert-Contains -Path $psychrometrics -Pattern 'const ENERGYPLUS_MIN_HUMIDITY_RATIO:\s*f64\s*=\s*1\.0e-5;' -Description "canonical floor literal"
Assert-Contains -Path $snapshotValidation -Pattern '(?s)energyplus_psy_w_fn_tdb_h\(temperature,\s*enthalpy\).*?temperature\.to_bits\(\).*?predecessor_cp394_resulting_supply_temperature_c\?.*?to_bits\(\).*?enthalpy\.to_bits\(\).*?predecessor_cp394_resulting_supply_enthalpy_j_per_kg\?.*?to_bits\(\).*?psychrometric\.to_bits\(\) != expected\.to_bits\(\).*?assigned\.to_bits\(\) != psychrometric\.to_bits\(\).*?resulting\.to_bits\(\) != assigned\.to_bits\(\)' -Description "active IEEE exactness"

$coreProductionFiles = @($transition, $routes, $owners, $accounting, $release, $prefix, $runtimeValidation, $snapshotValidation, $privateCharacterization)
foreach ($path in $coreProductionFiles) {
    Assert-NotContains -Path $path -Pattern '(?i)dehumidification_control_humidistat_case_(?:break|exit)|(?:break|case_exit|saturation_continuation)_(?:site|flag|counter|executed|performed|count)|line_?2289' -Description "CP396 break and line-2313 behavior firewall"
}
foreach ($path in @($transition, $release, $adapter, $pipelineValidation, $coupled) + @(Get-ChildItem -LiteralPath $coupledRoot -Recurse -File -Filter "*.rs" | ForEach-Object { $_.FullName })) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine + ((Get-ChildItem -LiteralPath "$root\tests" -Recurse -File -Filter "*.rs" | ForEach-Object { Read-RepoText -Path $_.FullName }) -join [Environment]::NewLine)
foreach ($pattern in @(
        'cp395_boundary_and_four_assignment_sites_are_exact',
        'cp395_preserves_thirty_routes_and_assigns_exactly_three_humidistat_routes',
        'assert_eq!\(snapshots\.len\(\),\s*30\)', 'assert_eq!\(state\.transition_count,\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*27\)',
        '(?s)dehumidification_control_humidistat_supply_humidity_ratio_assignment_count,\s*3',
        'assert_eq!\(state\.predecessor_route_counts,\s*\[1;\s*30\]\)',
        'assert_eq!\(state\.source_site_execution_count,\s*12\)',
        '(?s)cp394_supply_humidity_ratio_state_owner_count,\s*3',
        '(?s)unchanged_supply_humidity_ratio_preservation_count,\s*3',
        '(?s)cp394_supply_enthalpy_state_owner_count,\s*17',
        '(?s)unchanged_supply_enthalpy_preservation_count,\s*17',
        '(?s)cp394_supply_temperature_state_owner_count,\s*27',
        '(?s)unchanged_supply_temperature_preservation_count,\s*27',
        'matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)',
        'matches!\(index,\s*19\s*\|\s*23\s*\|\s*26\)',
        'usize::MAX', '0x3ee4_f8b5_88e3_68f1', 'f64::NEG_INFINITY', 'f64::INFINITY',
        '-0\.0', 'positive_subfloor|below_floor_but_positive', 'nan|NaN|from_bits', 'pole', 'to_bits\(\)'
    )) { Assert-Cp395Text -Text $testText -Pattern $pattern -Description "route, count, overflow, or IEEE test" }

$bindingText = Read-RepoText -Path $binding
$cp394BindingName = "calculation_$predecessorStem"
$cp395BindingName = "calculation_$stem"
$cp394Index = $bindingText.IndexOf("let $cp394BindingName =")
$cp395Index = $bindingText.IndexOf("let $cp395BindingName =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp394Index -lt 0 -or $cp395Index -le $cp394Index -or $numericalIndex -le $cp395Index) { throw "Binding must execute CP394, CP395, then unchanged numerical coupling" }
$cp395Call = [regex]::Match($bindingText, "(?s)let $([regex]::Escape($cp395BindingName)) =\s*advance_$([regex]::Escape($stem))\((?<args>.*?)\)\?;")
if (-not $cp395Call.Success -or [regex]::Matches($cp395Call.Groups["args"].Value, [regex]::Escape($cp394BindingName)).Count -ne 1) { throw "CP395 must consume CP394 exactly once as sole immediate predecessor" }
$cp394Matches = [regex]::Matches($bindingText, [regex]::Escape($cp394BindingName))
$cp395Matches = [regex]::Matches($bindingText, [regex]::Escape($cp395BindingName))
if ($cp394Matches.Count -ne 3 -or $cp395Matches.Count -ne 3 -or
    $cp394Matches[1].Index -lt $cp395Index -or $cp394Matches[1].Index -ge ($cp395Call.Index + $cp395Call.Length) -or
    $cp395Matches[1].Index -le ($cp395Call.Index + $cp395Call.Length) -or $cp395Matches[1].Index -ge $numericalIndex -or
    $cp394Matches[2].Index -le $numericalIndex -or $cp395Matches[2].Index -le $numericalIndex) {
    throw "Binding references must be CP394=3 and CP395=3 with sole-predecessor consumption, CP396 handoff, and post-numerical storage"
}
$dto = Get-Cp395BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match [regex]::Escape($stem) -or $dto -match '\bcp395\b') { throw "CP395 evidence unexpectedly feeds the numerical DTO" }
Assert-NotContains -Path $adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node|prediction|feedback|\breport\b|ResultStore|numerical|supply_temperature_c\s*:|supply_enthalpy_j_per_kg\s*:' -Description "adapter numerical or scalar feed"
Assert-Contains -Path $bindingTests -Pattern 'binding_places_cp395_after_cp394_before_unchanged_numerical_coupling' -Description "binding order and numerical-nonfeed test"

$coupledFiles = @($coupled) + @(Get-ChildItem -LiteralPath $coupledRoot -Recurse -File -Filter "*.rs" | ForEach-Object { $_.FullName })
$coupledText = ($coupledFiles | ForEach-Object { Read-RepoText -Path $_ }) -join [Environment]::NewLine
foreach ($pattern in @(
        "output\.$([regex]::Escape($cp394BindingName))",
        "output\.$([regex]::Escape($cp395BindingName))",
        'predecessor_cp394:\s*&PredecessorLifecycle',
        'HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER',
        'predecessor_route_counts\[19\]', 'predecessor_route_counts\[23\]', 'predecessor_route_counts\[26\]',
        'cp394_supply_humidity_ratio_state_owner_count', 'cp394_supply_enthalpy_state_owner_count',
        'cp394_supply_temperature_state_owner_count', 'source_site_execution_count'
    )) { Assert-Cp395Text -Text $coupledText -Pattern $pattern -Description "coupled lifecycle and lineage evidence" }
foreach ($path in $coupledFiles) {
    Assert-NotContains -Path $path -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|\.coupling\b|\.prediction\b|\.feedback\b|supply_node|\bload\b|\breport\b|ResultStore|reconcil|latest_numerical' -Description "coupled numerical feed"
}

foreach ($registration in @(
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\calc.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $binding; Pattern = "mod $stem;" },
        [PSCustomObject]@{ Path = $scheduledOutput; Pattern = "pub $($cp395BindingName):" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"; Pattern = "$($stem)_tests.rs" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $coupledRootFile; Pattern = "mod $($stem)_validation;" },
        [PSCustomObject]@{ Path = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"; Pattern = $stem },
        [PSCustomObject]@{ Path = $pipelineRoot; Pattern = $pipelineStem }
    )) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration" }
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    "pub\s+$([regex]::Escape($cp394BindingName))\s*:",
    "pub\s+$([regex]::Escape($cp395BindingName))\s*:",
    "pub\s+coupling\s*:"
) -Description "scheduled output CP394-to-CP396-to-numerical order"
Assert-PatternsInOrder -Path $coupledRootFile -Patterns @(
    "let\s+calc_$([regex]::Escape($predecessorStem))_lifecycle\s*=",
    "let\s+calc_$([regex]::Escape($stem))_lifecycle\s*="
) -Description "coupled lifecycle order"
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$($predecessorStem)::\s*validate_direct_lifecycle", "$($stem)::\s*validate_direct_lifecycle") -Description "pipeline validation order"

$lifecycleField = "purchased_air_calc_$($stem)_lifecycle"
$pipelineText = Read-RepoText -Path $pipelineRoot
$testBoundary = [regex]::Match($pipelineText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
if (-not $testBoundary.Success) { throw "CP395 pipeline production/test boundary missing" }
$production = $pipelineText.Substring(0, $testBoundary.Index)
$execute = Get-Cp395BraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
if ([regex]::Matches($execute, [regex]::Escape($lifecycleField) + '\s*:\s*None').Count -ne 3 -or
    [regex]::Matches($execute, 'let\s+' + [regex]::Escape($lifecycleField) + '\s*=\s*Some\s*\(').Count -ne 1) {
    throw "Pipeline must expose one direct CP395 Some and three non-direct None constructors"
}
$firewall = Get-Cp395BraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($lifecycleField) + '\s*\.\s*is_some').Count -ne 1) { throw "Non-direct firewall must reject CP395 evidence exactly once" }
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp433_lifecycle_evidence' -Description "cumulative non-direct firewall test"
Assert-Contains -Path $pipelineValidation -Pattern 'CP395 CP394 evidence is missing|missing CP394' -Description "pipeline missing-CP394 fail closed"
Assert-Contains -Path "$pipelineDir\validation\lineage.rs" -Pattern 'HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER' -Description "pipeline exact CP394 provenance"
foreach ($pattern in @(
        'transition_count', 'inactive_transition_count', 'source_site_execution_count', 'predecessor_route_counts',
        'cp394_supply_humidity_ratio_state_owner_count', 'cp394_supply_temperature_state_owner_count',
        'cp394_supply_enthalpy_state_owner_count', 'supply_humidity_ratio_assignment_write_count'
    )) { Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline count $pattern" }

$snapshotJsonText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$ieeeSidecars = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 13 -or $ieeeSidecars.Count -ne 13) { throw "CP395 serialization must expose exactly thirteen numeric projections and IEEE sidecars" }
for ($index = 0; $index -lt $numericFields.Count; $index += 1) {
    $expected = $numericFields[$index]
    if ($jsonNumbers[$index].Groups["field"].Value -cne $expected -or
        $jsonNumbers[$index].Groups["value"].Value -cne $expected -or
        $ieeeSidecars[$index].Groups["field"].Value -cne $expected -or
        $ieeeSidecars[$index].Groups["value"].Value -cne $expected) {
        throw "CP395 numeric/IEEE serialization order drift at '$expected'"
    }
}
$directJsonFields = [regex]::Matches($snapshotJsonText, '(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*,?\s*$')
$localJsonBools = @($directJsonFields | Where-Object { $localBools -contains $_.Groups["field"].Value })
if ($localJsonBools.Count -ne 10) { throw "CP395 serialization must expose exactly ten local booleans" }
for ($index = 0; $index -lt $localBools.Count; $index += 1) {
    $expected = $localBools[$index]
    if ($localJsonBools[$index].Groups["field"].Value -cne $expected -or $localJsonBools[$index].Groups["value"].Value -cne $expected) {
        throw "CP395 local JSON boolean order drift at '$expected'"
    }
}
Assert-Contains -Path $snapshotJson -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "finite-only numeric projection"
Assert-Contains -Path $snapshotJson -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "authoritative IEEE bits"

foreach ($pattern in @(
        'cp395_preserves_cp394_recursive_carriers_and_skips_assignment_on_direct_routes',
        'cp395_rejects_cp394_recursive_carrier_bit_drift_and_route_drift'
    )) { Assert-Contains -Path $coupledTests -Pattern $pattern -Description "coupled regression" }
foreach ($pattern in @(
        'public_cp395_validator_depends_only_on_cp394_and_requires_active_sites_inactive',
        'ep_run_cp395_rejects_missing_cp394_predecessor_evidence',
        'ep_run_cp395_links_recursive_cp393_and_terminal_cp394_carriers'
    )) { Assert-Contains -Path $pipelineValidationTests -Pattern $pattern -Description "pipeline validation regression" }
foreach ($pattern in @(
        'thirteen_compact_values_serialize_with_adjacent_ieee_sidecars',
        'thirteen_nonfinite_values_project_null_and_preserve_nan_payload_bits'
    )) { Assert-Contains -Path $snapshotJsonTests -Pattern $pattern -Description "JSON binary64 regression" }
Assert-Contains -Path $cp394Assertions -Pattern '#\[path\s*=\s*"cp395_assertions\.rs"\]' -Description "arbitrary assertion chain"
Assert-Contains -Path $cp394Assertions -Pattern 'cp395_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp394Assertions -Pattern 'cp395_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
foreach ($pattern in @('CP395_KEY', 'PurchasedAirManager\.cc:2288', 'PurchasedAirManager\.cc:2289', $sites[0], $sites[3], 'source_site_execution_count', 'predecessor_cp393_resulting_supply_', 'predecessor_cp394_resulting_supply_', '_ieee_bits', 'CP395 lifecycle must remain outside numerical result state', 'assert_non_direct')) {
    Assert-Contains -Path $cp395Assertions -Pattern $pattern -Description "arbitrary CP395 assertion"
}

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"; $capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP395 supersedes only CP394[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP395 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) { throw "CP395 must add exactly two algorithm and two capability claims" }
$algorithmBlocks = [regex]::Matches($algorithmText, '(?ms)^\[\[algorithm\]\]\s*.*?(?=^\[\[algorithm\]\]|\z)')
foreach ($parentId in @("zone_temp_predictor_corrector_source_order", "ideal_loads_zone_equipment_purchased_air_source_order")) {
    $parent = @($algorithmBlocks | Where-Object { $_.Value -match ('(?m)^id = "' + [regex]::Escape($parentId) + '"$') })
    if ($parent.Count -ne 1 -or $parent[0].Value.IndexOf("CP395 supersedes only CP394") -le $parent[0].Value.IndexOf("CP394 supersedes only CP393")) { throw "CP395 algorithm addendum placement drift in $parentId" }
}
$capabilityBlocks = [regex]::Matches($capabilityText, '(?ms)^\[\[capability\]\]\s*.*?(?=^\[\[capability\]\]|\z)')
foreach ($parentId in @("ideal_loads_no_oa_sensible", "ideal_loads_finite_limits")) {
    $parent = @($capabilityBlocks | Where-Object { $_.Value -match ('(?m)^id = "' + [regex]::Escape($parentId) + '"$') })
    if ($parent.Count -ne 1 -or $parent[0].Value.IndexOf("CP395 additionally requires") -le $parent[0].Value.IndexOf("CP394 additionally requires")) { throw "CP395 capability addendum placement drift in $parentId" }
}
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, 'physical executable line 2288', 'PurchAir\.SupplyHumRat\s*=\s*PsyWFnTdbH\(state,\s*PurchAir\.SupplyTemp,\s*SupplyEnthalpy,\s*RoutineName\);',
            'line 2289', '\}\s*break;', 'CP396', 'line 2313', $sites[0], $sites[1], $sites[2], $sites[3],
            'thirty', '19, 23, and 26', 'twenty-seven', 'eleven public', 'nineteen routes remain private',
            'T395=T394', 'A395=H394=R\[19\]\+R\[23\]\+R\[26\]', '4\*A395', '30/27/3/12',
            'sole immediate predecessor', '3/17/27', 'energyplus_psy_w_fn_tdb_h', '0x3ee4f8b588e368f1',
            'positive sub-floor', 'negative zero', 'NaN', 'positive infinity', 'denominator-pole', 'negative infinity',
            'thirteen Option<f64>', 'thirteen authoritative IEEE sidecars', 'ten-field', $numericFields[0], $numericFields[12], $localBools[0], $localBools[9],
            'CP394-to-CP395-to-unchanged-numerical', 'DirectZonePurchasedAirCouplingInput', '32 algorithms', '293 routines',
            '58 state-mapped', '235 source-mapped', '170 required', '333 total', '240 public', '93 internal', 'zero unused', 'zero unreachable', '238 development commands', 'Roadmap'
        )) { if ($claim.Value -notmatch $pattern) { throw "CP395 spec addendum missing '$pattern'" } }
}

$docs = @("docs\src\current\current-status.md", "docs\src\current\project-contract.md", "docs\src\porting-map\ideal-loads-source-map.md", "docs\src\porting-map\heat-balance-source-map.md", "docs\src\porting-map\zone-air-update-map.md")
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc; $sections = [regex]::Matches($text, '(?ms)^## CP395\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1 -or $text.LastIndexOf("## CP395 ") -le $text.LastIndexOf("## CP394 ")) { throw "CP395 documentation count/order drift in $doc" }
    $section = $sections[0].Value
    foreach ($pattern in @($commit, $hash, '2288', '2289', 'CP396', '2313', $sites[0], $sites[3], '19, 23, and 26', 'T395\s*=\s*T394', '4\s*\*\s*A395', '30/27/3/12', '3/17/27', '0x3ee4f8b588e368f1', 'thirteen', 'CP394-to-CP395-to-unchanged-numerical', '333\s+total', '240\s+public', '93\s+internal', 'Roadmap')) {
        if ($section -notmatch $pattern) { throw "CP395 documentation in $doc missing '$pattern'" }
    }
    $cursor = 0; foreach ($field in @($numericFields + $localBools)) { $next = $section.IndexOf($field, $cursor); if ($next -lt 0) { throw "CP395 documentation schema order missing '$field' in $doc" }; $cursor = $next + $field.Length }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP395\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP395 supersedes only CP394' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP395 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..394) { $file = (Get-ChildItem "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp433_lifecycle_evidence' -Description "historical firewall" }
foreach ($historical in 335..394) { $file = (Get-ChildItem "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 371 \|')) -Description "historical generated total"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 131 \|')) -Description "historical generated internal" }
foreach ($historical in 337..394) { $file = (Get-ChildItem "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 371' -Description "historical inventory total" }
foreach ($historical in 367..394) { $file = (Get-ChildItem "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 131' -Description "historical internal count"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 122 internal' -Description "historical classification" }
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(329..344 | ForEach-Object { (Get-ChildItem "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) { Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical helper whitelist" }
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp395Call\s*=', 'CP394-to-CP395', 'CP396-to-CP397', 'CP397-to-CP398', 'CP400-to-CP401', 'CP401-to-CP402', 'CP402-to-CP403', 'CP403-to-CP404')) { Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "CP345 terminal chain" }; Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 structural cap"

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"; $cp394AuditIndex = $master.IndexOf("cp394-cooling-post-saturation-capacity-limit-dehumidification-control-humidistat-case-entry.ps1"); $cp395AuditIndex = $master.IndexOf((Split-Path -Leaf $audit)); $completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp394AuditIndex -lt 0 -or $cp395AuditIndex -le $cp394AuditIndex -or $completionIndex -le $cp395AuditIndex) { throw "Master CP395 registration order drift" }
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 371', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) { Assert-Cp395Text -Text $inventory -Pattern $pattern -Description "inventory" }
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 131) { throw "CP395 inventory classification drift" }
Assert-Cp395Text -Text $inventory -Pattern ('(?s)path = "scripts/quality/ideal-loads-structure-audit/' + [regex]::Escape((Split-Path -Leaf $audit)) + '".*?callers = \["scripts/quality/ideal-loads-structure-audit\.ps1"\]') -Description "inventory record and caller"
foreach ($pattern in @('\| 371 \|', '\| public scripts \| 240 \|', '\| 131 \|', '\| scripts without callers \| 0 \|')) { Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory" }

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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP395 post-saturation Humidistat supply-humidity-ratio assignment structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-numerical' -Description 'CP433-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'
