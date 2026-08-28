# CP407 maps PurchasedAirManager.cc physical executable line 2302 only.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignment'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$psychrometricsSource = '.reference\energyplus-src\26.1.0\src\EnergyPlus\Psychrometrics.hh'
$psychrometricsHash = '30C9575BC5A8E73D33D111E0D54A4DA8916AF4534175E9B95071ACA2513AEF45'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$owners = "$root\transition\owners.rs"
$routes = "$root\transition\routes.rs"
$tests = "$root\tests.rs"
$release = "$root\release.rs"
$ownerValidation = "$root\release\owner_validation.rs"
$privateCharacterization = "$root\release\private_characterization.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$psychrometrics = 'crates\ep_runtime\src\psychrometrics.rs'
$psychrometricsTests = 'crates\ep_runtime\src\psychrometrics_inverse_density_tests.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\${stem}_tests.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${stem}_validation.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp407.rs'
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${stem}_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineValidationTests = "crates\ep_run\src\pipeline\$pipelineStem\validation\tests.rs"
$pipelineSerialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$snapshotJsonTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp406_assertions.rs'
$arbitraryAssertions = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp407_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp407-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-assignment.ps1'

function Assert-Cp407Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP407 $Description missing '$Pattern'" }
}

function Get-Cp407BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP407 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP407 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP407 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $accounting, $owners, $routes, $tests, $release,
    $ownerValidation, $privateCharacterization, $runtimeValidation, $snapshotValidation,
    $adapter, $adapterTests, $coupled,
    $coupledTests, $coupledFixture, $witness, $pipeline, $pipelineValidation,
    $pipelineValidationTests, $pipelineSerialization, $snapshotJson, $snapshotJsonTests,
    $arbitraryAssertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description 'CP407 implementation/audit file'
    Assert-LineLimit -Path $file -Limit 500 -Description 'CP407 bounded file'
}
Assert-FileExists -Path $psychrometrics -Description 'canonical psychrometrics implementation'
Assert-FileExists -Path $psychrometricsTests -Description 'canonical psychrometrics tests'

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $sourceHash) {
    throw 'CP407 PurchasedAirManager.cc SHA-256 drift'
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $psychrometricsSource).Hash -cne $psychrometricsHash) {
    throw 'CP407 Psychrometrics.hh SHA-256 drift'
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2301].Trim() -cne 'PurchAir.SupplyTemp = PsyTdbFnHW(SupplyEnthalpy, PurchAir.SupplyHumRat);' -or
    -not $lines[2302].Trim().StartsWith('//') -or
    $lines[2303].Trim() -cne 'PurchAir.SupplyTemp = min(PurchAir.SupplyTemp, PurchAir.MixedAirTemp);') {
    throw 'CP407 source/comment/first-exclusion boundary drift'
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2302' -Description 'mapped executable'
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2304' -Description 'first excluded executable'
$sites = @(
    'read-cp385-retained-supply-enthalpy-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature-dry-bulb-inversion',
    'read-cp378-retained-supply-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature-dry-bulb-inversion',
    'evaluate-psy-tdb-fn-h-w-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-supply-temperature',
    'assign-purchased-air-supply-temperature-after-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-guard-else-branch'
)
Assert-ExactStringArray -Path $module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER' -Expected $sites -Description 'exact four source sites'

$moduleText = Read-RepoText -Path $module
$snapshotStruct = Get-Cp407BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
[string[]]$fields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:') | ForEach-Object { $_.Groups['field'].Value })
$expectedFields = @(
    'source','first_excluded_source','source_order','system','parent_call_ordinal','controlled_zone',
    'unit_off_skipped','non_cooling_skipped','positive_guard_false_fallthrough_skipped',
    'heating_availability_guard_false_fallthrough','humidification_control_guard_false_fallthrough',
    'dehumidification_control_humidistat_maximum_assignment_executed','dehumidification_control_none_maximum_assignment_executed',
    'dehumidification_control_guard_false_fallthrough','predecessor_capacity_limit_guard_evaluated',
    'predecessor_capacity_limit_body_entered','predecessor_active_capacity_limit_guard_false_fallthrough',
    'predecessor_dehumidification_guard_evaluated','predecessor_dehumidification_body_entered',
    'predecessor_dehumidification_guard_false_fallthrough','predecessor_dehumidification_total_output_assignment_executed',
    'predecessor_dehumidification_total_output_capacity_guard_evaluated',
    'predecessor_dehumidification_total_output_capacity_adjustment_body_entered',
    'predecessor_dehumidification_total_output_capacity_guard_false_fallthrough',
    'dehumidification_total_output_capacity_guard_false_fallthrough',
    'dehumidification_total_output_maximum_capacity_assignment_executed','predecessor_supply_enthalpy_assignment_executed',
    'predecessor_dehumidification_control_type_read','predecessor_dehumidification_control_type',
    'predecessor_dehumidification_control_switch_dispatched',
    'predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered',
    'predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break',
    'predecessor_dehumidification_control_humidistat_case_entered',
    'predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed',
    'predecessor_dehumidification_control_humidistat_case_exited_via_break',
    'predecessor_dehumidification_control_none_case_entered',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed',
    'predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered',
    'predecessor_cp406_resulting_supply_humidity_ratio','predecessor_cp406_resulting_supply_enthalpy_j_per_kg',
    'predecessor_cp406_resulting_supply_temperature_c',
    'dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed',
    'cp385_retained_supply_enthalpy_owned_read','cp406_same_call_supply_enthalpy_bit_corroborated',
    'supply_enthalpy_for_dry_bulb_inversion_read','supply_enthalpy_j_per_kg',
    'cp378_retained_supply_humidity_ratio_owned_read','supply_humidity_ratio_for_dry_bulb_inversion_read',
    'supply_humidity_ratio','cp406_retained_supply_temperature_state_owned','preexisting_supply_temperature_c',
    'psychrometric_supply_temperature_evaluated','psychrometric_supply_temperature_result_c',
    'supply_temperature_assigned','assigned_supply_temperature_c','resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
if ($fields.Count -ne 60 -or $expectedFields.Count -ne 60) { throw 'CP407 snapshot must expose exactly 60 fields' }
for ($index = 0; $index -lt 60; $index += 1) {
    if ($fields[$index] -cne $expectedFields[$index]) { throw "CP407 field order drift at $index" }
}
$expectedNumeric = @(
    'predecessor_cp406_resulting_supply_humidity_ratio','predecessor_cp406_resulting_supply_enthalpy_j_per_kg',
    'predecessor_cp406_resulting_supply_temperature_c','supply_enthalpy_j_per_kg','supply_humidity_ratio',
    'preexisting_supply_temperature_c','psychrometric_supply_temperature_result_c','assigned_supply_temperature_c',
    'resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c'
)
[string[]]$numericFields = @([regex]::Matches($snapshotStruct, 'pub\s+(?<field>[A-Za-z0-9_]+)\s*:\s*Option\s*<\s*f64\s*>') | ForEach-Object { $_.Groups['field'].Value })
if ($numericFields.Count -ne 11) { throw 'CP407 snapshot must expose eleven Option<f64> fields' }
for ($index = 0; $index -lt 11; $index += 1) {
    if ($numericFields[$index] -cne $expectedNumeric[$index]) { throw "CP407 numeric order drift at $index" }
}
if ([regex]::Matches($snapshotStruct, 'Option\s*<\s*DehumidificationControlType\s*>').Count -ne 1) {
    throw 'CP407 snapshot must expose one optional dehumidification-control enum'
}

Assert-PatternsInOrder -Path $state -Patterns @(
    'pub\s+transition_count\s*:\s*usize','pub\s+inactive_transition_count\s*:\s*usize',
    'pub\s+predecessor_guard_false_fallthrough_count\s*:\s*usize','pub\s+predecessor_maximum_capacity_assignment_count\s*:\s*usize',
    'pub\s+predecessor_else_branch_entry_count\s*:\s*usize','pub\s+dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count\s*:',
    'pub\s+predecessor_route_counts\s*:\s*\[usize;\s*30\]','pub\s+predecessor_else_branch_entry_route_counts\s*:\s*\[usize;\s*30\]',
    'pub\s+supply_temperature_assignment_route_counts\s*:\s*\[usize;\s*30\]','pub\s+source_site_execution_count\s*:\s*usize',
    'pub\s+cp385_retained_supply_enthalpy_owned_read_count\s*:\s*usize','pub\s+cp406_same_call_supply_enthalpy_bit_corroboration_count\s*:\s*usize',
    'pub\s+cp378_retained_supply_humidity_ratio_owned_read_count\s*:\s*usize','pub\s+psychrometric_supply_temperature_evaluation_count\s*:\s*usize',
    'pub\s+supply_temperature_assignment_write_count\s*:\s*usize','pub\s+latest\s*:'
) -Description 'persistent CP407 accounting schema'

$transitionText = Read-RepoText -Path $transition
if ([regex]::Matches($transitionText, 'energyplus_psy_tdb_fn_h_w\(').Count -ne 1) {
    throw 'CP407 transition must delegate exactly once to canonical PsyTdbFnHW'
}
Assert-Contains -Path $transition -Pattern 'energyplus_psy_tdb_fn_h_w\(supply_enthalpy_j_per_kg,\s*supply_humidity_ratio\)' -Description 'canonical helper call'
Assert-NotContains -Path $transition -Pattern '2\.500_?94e6|1\.004_?84e3|1\.858_?95e3|mul_add|total_cmp|partial_cmp|\.clamp\(|is_finite|\.max\(|\.min\(' -Description 'no duplicate formula or coercion'
Assert-Contains -Path $psychrometrics -Pattern '(?s)pub fn energyplus_psy_tdb_fn_h_w\(enthalpy_j_per_kg: f64, humidity_ratio: f64\) -> f64 \{\s*let humidity_ratio = energyplus_humidity_ratio_floor\(humidity_ratio\);\s*\(enthalpy_j_per_kg - 2\.500_94e6 \* humidity_ratio\) / \(1\.004_84e3 \+ 1\.858_95e3 \* humidity_ratio\)\s*\}' -Description 'canonical formula grouping'
Assert-Contains -Path $psychrometrics -Pattern '(?s)fn energyplus_humidity_ratio_floor\(humidity_ratio: f64\) -> f64 \{.*?if humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO \{\s*ENERGYPLUS_MIN_HUMIDITY_RATIO\s*\} else \{\s*humidity_ratio\s*\}' -Description 'source-first humidity floor'
Assert-Contains -Path $psychrometricsTests -Pattern 'psy_tdb_matches_pinned_source_formula_vectors_bitwise' -Description 'canonical formula vectors'
Assert-Contains -Path $psychrometricsTests -Pattern 'psy_tdb_applies_the_source_humidity_floor_and_nan_semantics' -Description 'canonical IEEE vectors'

$core = (@($accounting,$owners,$routes,$ownerValidation,$runtimeValidation,$snapshotValidation) | ForEach-Object { Read-RepoText -Path $_ }) -join [Environment]::NewLine
foreach ($pattern in @(
    'matches!\(index,\s*20\s*\|\s*21\s*\|\s*24\s*\|\s*25\s*\|\s*27\s*\|\s*29\)',
    'supply_temperature_assignment_route_counts\s*==\s*state\.predecessor_else_branch_entry_route_counts',
    'inactive_transition_count\.checked_add\(assigned\)','assigned\.checked_mul\([^)]*SOURCE_ORDER\.len\(\)',
    'cp385_retained_supply_enthalpy_owned_read_count','cp378_retained_supply_humidity_ratio_owned_read_count',
    'predecessor\.resulting_supply_humidity_ratio\.is_none\(\)','left\.to_bits\(\)\s*==\s*right\.to_bits\(\)'
)) { Assert-Cp407Text -Text $core -Pattern $pattern -Description 'route/accounting/owner contract' }
$testsText = Read-RepoText -Path $tests
foreach ($pattern in @('routes\.len\(\),\s*36','\[20,\s*22,\s*26,\s*28,\s*31,\s*34\]','\[20,\s*26\]','inactive_transition_count,\s*30','assignment_count,\s*6','source_site_execution_count,\s*24')) {
    Assert-Cp407Text -Text $testsText -Pattern $pattern -Description '36/30/6/24 characterization'
}

Assert-Contains -Path $release -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?_supply_temperature_assignment\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor_cp406:\s*Predecessor,\s*\)' -Description 'exact public arguments'
Assert-Contains -Path $release -Pattern 'direct_predecessor_is_retained_and_complete' -Description 'sole CP406 immediate authority'
Assert-Contains -Path $ownerValidation -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_committed_latest_snapshot_is_consistent\s*\(' -Description 'bounded CP406 committed predecessor proof'
Assert-NotContains -Path $ownerValidation -Pattern 'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_is_consistent\s*\(' -Description 'recursive CP406 predecessor completion'
Assert-Contains -Path $release -Pattern 'active_owners_from_retained_runtime' -Description 'active-only owner acquisition'
Assert-Contains -Path $ownerValidation -Pattern 'calc_cooling_supply_humidity_ratio_saturation_limit_assignment\s*\.latest' -Description 'CP378 humidity owner'
Assert-Contains -Path $ownerValidation -Pattern 'cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness' -Description 'CP378 private witness'
Assert-Contains -Path $ownerValidation -Pattern 'calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\s*\.latest' -Description 'CP385 enthalpy owner'
Assert-Contains -Path $ownerValidation -Pattern 'total_output_supply_enthalpy_assignment_latest_witness' -Description 'CP385 private witness'
Assert-Contains -Path $ownerValidation -Pattern '(?s)predecessor\.resulting_supply_humidity_ratio\.is_none\(\).*?predecessor\.resulting_supply_enthalpy_j_per_kg.*?enthalpy\.resulting_supply_enthalpy_j_per_kg' -Description 'CP406 W rejection and H corroboration'
Assert-NotContains -Path $release -Pattern '(?s)pub fn advance_direct_no_oa_calc_[^(]+\([^)]*(supply_enthalpy|supply_humidity_ratio|supply_temperature)\s*:' -Description 'no caller operand substitutes'
Assert-NotContains -Path $release -Pattern 'DirectZonePurchasedAirCouplingInput|ZoneHeatBalanceState|psychrometric_service|latest_numerical|numerical_supply|cache|diagnostic' -Description 'service/numerical firewall'
foreach ($path in @($transition,$accounting,$owners,$routes,$release,$ownerValidation,$runtimeValidation,$snapshotValidation,$adapter,$coupled,$pipelineValidation)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}

Assert-PatternsInOrder -Path $binding -Patterns @(
    "let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*=','let\s+unit_available\s*=','let\s+coupling\s*='
) -Description 'CP406-to-CP407-to-CP408-to-CP414-to-CP415 binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @(
    "pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\s*:','pub\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\s*:','pub\s+coupling\s*:'
) -Description 'CP406-to-CP407-to-CP408-to-CP409 scheduled output order'
$bindingText = Read-RepoText -Path $binding
if ([regex]::Matches($bindingText,"\bcalculation_$predecessorStem\b").Count -ne 3 -or [regex]::Matches($bindingText,"\bcalculation_$stem\b").Count -ne 3) {
    throw 'CP407 binding evidence occurrence drift'
}
$dto = Get-Cp407BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp407|latent_output_supply_temperature_assignment') { throw 'CP407 must not feed numerical DTO' }
Assert-Contains -Path $adapterTests -Pattern 'cp407' -Description 'binding regression'
Assert-Contains -Path $coupledTests -Pattern 'cp407' -Description 'coupled regression'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$stem" -Description 'coupled output fixture'
Assert-Contains -Path $witness -Pattern "set_${stem}_latest_witness" -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle",'cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit::\s*validate_direct_lifecycle','cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break::\s*validate_direct_lifecycle') -Description 'pipeline CP406-to-CP407-to-CP408-to-CP409 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp430_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp407_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp407_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp407_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
foreach ($pattern in @('PurchasedAirManager\.cc:2302','PurchasedAirManager\.cc:2304','matches!\(index,\s*0\.\.=8\s*\|\s*20\s*\|\s*24\)','non-direct runtime must not publish CP407 evidence','ends_with\("_ieee_bits"\)')) {
    Assert-Contains -Path $arbitraryAssertions -Pattern $pattern -Description 'arbitrary runtime contract'
}

Assert-PatternsInOrder -Path $pipelineSerialization -Patterns @(
    '"transition_count"\s*:','"inactive_transition_count"\s*:','"predecessor_else_branch_entry_count"\s*:',
    '"dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count"\s*:',
    '"supply_temperature_assignment_route_counts"\s*:','"source_site_execution_count"\s*:',
    '"cp385_retained_supply_enthalpy_owned_read_count"\s*:','"cp378_retained_supply_humidity_ratio_owned_read_count"\s*:',
    '"psychrometric_supply_temperature_evaluation_count"\s*:','"supply_temperature_assignment_write_count"\s*:','"latest"\s*:'
) -Description 'serialized lifecycle state'
$serializationText = Read-RepoText -Path $snapshotJson
$jsonNumbers = [regex]::Matches($serializationText,'(?m)^\s*"(?<field>[A-Za-z0-9_]+)"\s*:\s*json_number\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
$sidecars = [regex]::Matches($serializationText,'(?m)^\s*"(?<field>[A-Za-z0-9_]+)_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.(?<value>[A-Za-z0-9_]+)\s*\)')
if ($jsonNumbers.Count -ne 11 -or $sidecars.Count -ne 11) { throw 'CP407 JSON must expose eleven numeric/IEEE pairs' }
for ($index = 0; $index -lt 11; $index += 1) {
    $field = $expectedNumeric[$index]; $escaped = [regex]::Escape($field)
    $adjacent = '(?m)^\s*"'+$escaped+'"\s*:\s*json_number\s*\(\s*snapshot\.'+$escaped+'\s*\)\s*,\s*\r?\n\s*"'+$escaped+'_ieee_bits"\s*:\s*ieee_bits\s*\(\s*snapshot\.'+$escaped+'\s*\)'
    if ($jsonNumbers[$index].Groups['field'].Value -cne $field -or $sidecars[$index].Groups['field'].Value -cne $field -or $serializationText -notmatch $adjacent) { throw "CP407 JSON sidecar order drift at $field" }
}
Assert-Contains -Path $snapshotJsonTests -Pattern 'object\.len\(\),\s*71' -Description '60-field plus eleven-sidecar JSON shape'
Assert-Contains -Path $snapshotJsonTests -Pattern '(?s)ends_with\("_ieee_bits"\).*?count\(\),\s*11' -Description 'eleven sidecars'

$heading = 'CP407 post-saturation shared-case latent-output supply-temperature assignment'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP407 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## |\z)").Groups['body'].Value
    foreach ($pattern in @('line 2302.*?PsyTdbFnHW','line 2304.*?first excluded','20,\s*22,\s*26,\s*28,\s*31,\s*and 34','36/30/6/24','exactly 60 fields','11 `Option<f64>`','71 unique keys','enthalpy operand.*?CP385','humidity operand.*?CP378','CP406-to-CP407-to-unchanged-numerical','345 total,\s*240 public,\s*105 internal')) {
        Assert-Cp407Text -Text $section -Pattern "(?is)$pattern" -Description 'bounded documentation claim'
    }
}
Assert-Contains -Path 'specs\algorithm_ledger.toml' -Pattern 'CP407 supersedes only CP406' -Description 'algorithm addendum'
Assert-Contains -Path 'specs\capabilities.toml' -Pattern 'CP407 additionally requires' -Description 'capability addendum'
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP407 supersedes only CP406' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP407 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern 'CP407' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP407\b' -Description 'psychrometrics-map non-promotion'

$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 406) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp430_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 406) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 368' -Description 'historical script count' }
    if ($number -ge 335 -and $number -le 406) {
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 368 \|')) -Description 'historical generated total'
        Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 128 \|')) -Description 'historical generated internal total'
    }
    if ($number -ge 367 -and $number -le 406) {
        Assert-Contains -Path $file.FullName -Pattern 'Count -ne 128' -Description 'historical classification count'
        Assert-Contains -Path $file.FullName -Pattern '240 public and 122 internal' -Description 'historical classification phrase'
    }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP407 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminal = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 406)) })
if ($terminal.Count -ne 30) { throw 'CP407 terminal propagation set drift' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP406-to-CP407' -Description 'historical terminal interval' }
foreach ($file in $terminal) { Assert-Contains -Path $file.FullName -Pattern 'CP407-to-CP408' -Description 'historical terminal interval' }
$cp345 = "$auditRoot\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp407Call\s*=','\$cp408Call\s*=','\$cp409Call\s*=','CP406-to-CP407','CP407-to-CP408','CP408-to-CP409','CP414-to-CP415')) { Assert-Contains -Path $cp345 -Pattern $pattern -Description 'CP345 terminal chain' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>399|400|401|402|403|404|405|406|407)-' })) { Assert-Contains -Path $file.FullName -Pattern 'latent_output_supply_temperature_mixed_air_limit\\s\*' -Description 'recent CP408 binding order' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406)-' })) { Assert-Contains -Path $file.FullName -Pattern '\$cp407Call' -Description 'recent CP407 terminal capture' }
foreach ($file in @($audits | Where-Object { $_.BaseName -match '^cp(?<number>403|404|405|406|407)-' })) { Assert-Contains -Path $file.FullName -Pattern '\$cp408Call' -Description 'recent CP408 terminal capture' }

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp406-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-capacity-guard-else-branch-entry.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex) { throw 'Master CP407 registration order drift' }
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 368','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp407Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"$').Count -ne 128) { throw 'CP407 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp407-cooling-post-saturation-capacity-limit-dehumidification-control-constant-supply-humidity-ratio-latent-output-supply-temperature-assignment\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 368 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 128 \|' -Description 'generated internal total'

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
Write-Host 'CP407 post-saturation shared-case latent-output supply-temperature assignment structure audit passed.'
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-numerical' -Description 'CP430-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-numerical' -Description 'CP430-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-numerical' -Description 'CP430-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-numerical' -Description 'CP430-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-numerical' -Description 'CP430-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP42[9]-to-numerical' -Description 'stale CP429 numerical interval'
