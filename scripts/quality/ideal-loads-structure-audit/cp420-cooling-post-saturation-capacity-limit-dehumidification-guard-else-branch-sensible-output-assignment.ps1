# CP420 maps PurchasedAirManager.cc physical executable line 2331's not-dehumidifying sensible-output assignment.
& {
$stem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment'
$predecessorStem = 'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment'
$typeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignment'
$predecessorTypeStem = 'PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignment'
$pipelineStem = "purchased_air_$stem"
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$sourceHash = '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$predecessorModule = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$release = "$root\release.rs"
$releaseError = "$root\release\error.rs"
$runtimeValidation = "$root\release\runtime_validation.rs"
$releaseSnapshot = "$root\release\snapshot.rs"
$tests = "$root\tests.rs"
$schemaRouteTests = "$root\tests\schema_routes.rs"
$overflowTests = "$root\tests\overflow.rs"
$predecessorCommitted = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem\release\committed.rs"
$cp329Module = 'crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call.rs'
$cp329Release = 'crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\release.rs'
$cp329Committed = 'crates\ep_runtime\src\ideal_loads\calc\cooling_mixed_air_call\release\committed.rs'
$cp330Module = 'crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard.rs'
$cp330Release = 'crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\release.rs'
$cp330Committed = 'crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\release\committed.rs'
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'
$scheduledOutput = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp420.rs'
$coupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs"
$witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs"
$pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$pipelineLineage = "crates\ep_run\src\pipeline\$pipelineStem\validation\lineage.rs"
$serializationRoot = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$serializationTests = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot\tests.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp420_assertions.rs'
$arbitraryPredecessor = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp419_assertions.rs'
$audit = 'scripts\quality\ideal-loads-structure-audit\cp420-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-assignment.ps1'

function Assert-Cp420Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP420 $Description missing '$Pattern'" }
}

function Get-Cp420BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "CP420 $Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf('{', $anchors[0].Index)
    if ($open -lt 0) { throw "CP420 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq '{') { $depth += 1 }
        elseif ($Text[$index] -eq '}') {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1) }
        }
    }
    throw "CP420 $Description closing brace missing"
}

$required = @(
    $source,$module,$predecessorModule,$state,$transition,$accounting,$release,$releaseError,$runtimeValidation,$releaseSnapshot,$tests,$schemaRouteTests,$overflowTests,
    $predecessorCommitted,$cp329Module,$cp329Release,$cp329Committed,$cp330Module,$cp330Release,$cp330Committed,
    $binding,$scheduledOutput,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipelineRoot,$pipeline,$pipelineValidation,$pipelineLineage,$serializationRoot,$serialization,$serializationTests,
    $arbitrary,$arbitraryPredecessor,$audit
)
foreach ($file in $required) { Assert-FileExists -Path $file -Description 'CP420 implementation/audit file' }
foreach ($file in @($module,$state,$transition,$accounting,$release,$releaseError,$runtimeValidation,$releaseSnapshot,$tests,$schemaRouteTests,$overflowTests,
    $predecessorCommitted,$cp329Committed,$cp330Committed,$adapter,$adapterTests,$coupled,$coupledTests,$coupledFixture,$witness,
    $pipeline,$pipelineValidation,$pipelineLineage,$serializationRoot,$serialization,$serializationTests,$arbitrary,$audit)) {
    Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP420/committed-evidence file'
}
foreach ($file in @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')) {
    Assert-LineLimit -Path $file.FullName -Limit 500 -Description 'bounded CP420 core subtree file'
}
if (@(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs').Count -ne 13) { throw 'CP420 exact thirteen-file bounded core subtree drift' }
Assert-Contains -Path $tests -Pattern 'cp420_boundary_and_eight_sites_are_exact' -Description 'CP420 exact source-boundary/site test'
Assert-Contains -Path $tests -Pattern 'exhaustive_54_outcomes_49_inactive_five_assignments_and_ten_arrays_are_exact' -Description 'CP420 exhaustive route/count test'
Assert-Contains -Path $tests -Pattern 'release_hot_path_uses_only_committed_owners_and_validated_route' -Description 'CP420 public hot-path recursion-zero regression'
Assert-Contains -Path $schemaRouteTests -Pattern 'snapshot_schema_is_exact_202_fields_71_optional_and_cp419_base_prefixed' -Description 'CP420 split schema-prefix test'
Assert-Contains -Path $overflowTests -Pattern 'every_mutable_scalar_and_all_ten_route_arrays_overflow_transactionally' -Description 'CP420 split transactional-overflow test'

$registrations = @(
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\calc.rs'; Pattern="(?s)mod\s+$stem;.*?pub\s+use\s+$stem::\*;" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\unit.rs'; Pattern="calc_$stem\s*:\s*$($typeStem)RuntimeState::new\(system\)" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs'; Pattern="mod\s+$stem" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\binding_tests.rs'; Pattern="$($stem)_tests" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime.rs'; Pattern="mod\s+$($stem)_validation" },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs'; Pattern='test_coupled_runtime_cp420\.rs' },
    [PSCustomObject]@{ Path='crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs'; Pattern="$($stem)_fixture" },
    [PSCustomObject]@{ Path=$module; Pattern='(?s)mod\s+release;.*?mod\s+state;.*?mod\s+transition;' },
    [PSCustomObject]@{ Path=$release; Pattern='(?s)mod\s+error;.*?mod\s+runtime_validation;.*?mod\s+snapshot;' },
    [PSCustomObject]@{ Path=$transition; Pattern='mod\s+accounting;' },
    [PSCustomObject]@{ Path=$pipelineRoot; Pattern="mod\s+$pipelineStem;" },
    [PSCustomObject]@{ Path=$pipeline; Pattern='(?s)mod\s+serialization;.*?mod\s+validation;' },
    [PSCustomObject]@{ Path=$pipelineValidation; Pattern='mod\s+lineage;' },
    [PSCustomObject]@{ Path=$serializationRoot; Pattern='mod\s+snapshot;' },
    [PSCustomObject]@{ Path=$serialization; Pattern='mod\s+tests;' }
)
foreach ($registration in $registrations) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'CP420 module/test registration' }

if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -cne $sourceHash) { throw 'CP420 PurchasedAirManager.cc SHA-256 drift' }
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2330].Trim() -cne 'CoolSensOutput = SupplyMassFlowRate * CpAir * (PurchAir.MixedAirTemp - PurchAir.SupplyTemp);' -or
    $sourceLines[2331].Trim() -cne 'if (CoolSensOutput >= PurchAir.MaxCoolTotCap) {') { throw 'CP420 source boundary drift' }
$sites = @(
    'read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-first-product',
    'read-local-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-first-product',
    'calculate-supply-mass-flow-rate-times-cp-air-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output',
    'read-purchased-air-mixed-air-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-difference',
    'read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-difference',
    'calculate-mixed-air-temperature-minus-supply-temperature-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output',
    'calculate-mass-flow-cp-air-product-times-temperature-difference-for-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output',
    'assign-local-cooling-sensible-output-for-post-saturation-capacity-limit-dehumidification-guard-else-branch'
)
Assert-Contains -Path $module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2331' -Description 'source constant'
Assert-Contains -Path $module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2332' -Description 'first excluded executable constant'
Assert-ExactStringArray -Path $module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER' -Expected $sites -Description 'exact eight source sites'

$moduleText = Read-RepoText -Path $module
$predecessorText = Read-RepoText -Path $predecessorModule
$snapshotStruct = Get-Cp420BraceBlock -Text $moduleText -AnchorPattern "pub\s+struct\s+$($typeStem)Snapshot\s*" -Description 'snapshot'
$predecessorStruct = Get-Cp420BraceBlock -Text $predecessorText -AnchorPattern "pub\s+struct\s+$($predecessorTypeStem)Snapshot\s*" -Description 'CP419 snapshot'
$fields = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$predecessorFields = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$predecessorTerminal = @('predecessor_cp419_resulting_supply_humidity_ratio','predecessor_cp419_resulting_supply_enthalpy_j_per_kg','predecessor_cp419_resulting_supply_temperature_c')
$localFields = @(
    'post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed',
    'cp419_retained_supply_humidity_ratio_state_owned','cp419_retained_supply_enthalpy_state_owned','cp419_retained_supply_temperature_state_owned',
    'cp330_retained_supply_mass_flow_rate_owned_read','cp329_supply_mass_flow_rate_bit_corroborated','supply_mass_flow_rate_read','supply_mass_flow_rate_kg_per_s',
    'cp419_retained_cp_air_owned_read','cp_air_read','cp419_cp_air_for_sensible_output_j_per_kg_k',
    'supply_mass_flow_rate_times_cp_air_calculated','supply_mass_flow_rate_times_cp_air_w_per_k',
    'cp329_retained_mixed_air_temperature_for_sensible_output_owned_read','mixed_air_temperature_read','mixed_air_temperature_for_sensible_output_c',
    'cp419_retained_supply_temperature_owned_read','supply_temperature_read','supply_temperature_for_sensible_output_c',
    'mixed_air_minus_supply_temperature_calculated','mixed_air_minus_supply_temperature_k',
    'cooling_sensible_output_calculated','calculated_cooling_sensible_output_w','cooling_sensible_output_assigned','cooling_sensible_output_w'
)
$terminal = @('resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c')
$expectedFields = @($predecessorFields[0..170]) + $predecessorTerminal + $localFields + $terminal
if ($predecessorFields.Count -ne 174 -or $fields.Count -ne 202 -or ($fields -join '|') -cne ($expectedFields -join '|')) { throw 'CP420 exact 171-field CP419 prefix plus 31-field tail drift' }
$numeric = @([regex]::Matches($snapshotStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$predecessorNumeric = @([regex]::Matches($predecessorStruct, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*)\s*:\s*Option<f64>') | ForEach-Object { $_.Groups['name'].Value })
$localNumeric = @('supply_mass_flow_rate_kg_per_s','cp419_cp_air_for_sensible_output_j_per_kg_k','supply_mass_flow_rate_times_cp_air_w_per_k','mixed_air_temperature_for_sensible_output_c','supply_temperature_for_sensible_output_c','mixed_air_minus_supply_temperature_k','calculated_cooling_sensible_output_w','cooling_sensible_output_w')
$expectedNumeric = @($predecessorNumeric[0..56]) + $predecessorTerminal + $localNumeric + $terminal
if ($predecessorNumeric.Count -ne 60 -or $numeric.Count -ne 71 -or ($numeric -join '|') -cne ($expectedNumeric -join '|')) { throw 'CP420 exact seventy-one numeric-carrier schema drift' }
if ([regex]::Matches($snapshotStruct,'Option<bool>').Count -ne 1 -or [regex]::Matches($snapshotStruct,'Option<DehumidificationControlType>').Count -ne 1) { throw 'CP420 optional comparison/control carrier drift' }

$stateText = Read-RepoText -Path $state
$routeArrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[a-z][a-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object { $_.Groups['name'].Value })
$expectedArrays = @(
    'predecessor_route_counts','predecessor_guard_false_fallthrough_route_counts','predecessor_guard_body_entry_route_counts',
    'predecessor_supply_temperature_saturation_assignment_route_counts','predecessor_supply_temperature_mixed_air_limit_route_counts',
    'predecessor_supply_humidity_ratio_assignment_route_counts','predecessor_supply_enthalpy_assignment_route_counts',
    'predecessor_dehumidification_guard_else_branch_entry_route_counts','predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts',
    'dehumidification_guard_else_branch_sensible_output_assignment_route_counts'
)
if (($routeArrays -join '|') -cne ($expectedArrays -join '|')) { throw 'CP420 exact ten width-36 route arrays drift' }
foreach ($pattern in @(
    'dehumidification_guard_else_branch_sensible_output_assignment_count','source_site_execution_count',
    'cp419_supply_humidity_ratio_state_owner_count','unchanged_supply_humidity_ratio_preservation_count',
    'cp419_supply_enthalpy_state_owner_count','unchanged_supply_enthalpy_preservation_count',
    'cp419_supply_temperature_state_owner_count','unchanged_supply_temperature_preservation_count',
    'supply_mass_flow_rate_owned_read_count','supply_mass_flow_rate_bit_corroboration_count','supply_mass_flow_rate_read_count',
    'cp_air_owned_read_count','cp_air_read_count','supply_mass_flow_rate_times_cp_air_calculation_count',
    'mixed_air_temperature_owned_read_count','mixed_air_temperature_read_count','supply_temperature_owned_read_count','supply_temperature_read_count',
    'mixed_air_minus_supply_temperature_calculation_count','cooling_sensible_output_calculation_count','cooling_sensible_output_assignment_write_count'
)) { Assert-Cp420Text -Text $stateText -Pattern $pattern -Description 'runtime counter schema' }

$transitionText = Read-RepoText -Path $transition
$runtimeValidationText = Read-RepoText -Path $runtimeValidation
foreach ($pattern in @(
    'active:\s*route\.active','post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed:\s*route\.active',
    'let\s+first_product_w_per_k\s*=\s*input\.supply_mass_flow_rate_kg_per_s\s*\*\s*cp_air_j_per_kg_k',
    'let\s+temperature_difference_k\s*=\s*input\.mixed_air_temperature_c\s*-\s*supply_temperature_c',
    'let\s+cooling_sensible_output_w\s*=\s*first_product_w_per_k\s*\*\s*temperature_difference_k',
    'resulting_supply_humidity_ratio:\s*predecessor\.resulting_supply_humidity_ratio',
    'resulting_supply_enthalpy_j_per_kg:\s*predecessor\.resulting_supply_enthalpy_j_per_kg',
    'resulting_supply_temperature_c:\s*predecessor\.resulting_supply_temperature_c'
)) { Assert-Cp420Text -Text $transitionText -Pattern "(?s)$pattern" -Description 'source-shaped transition' }
foreach ($pattern in @('matches!\(index,\s*4\s*\|\s*7\s*\|\s*10\s*\|\s*13\s*\|\s*16\)','assignments\s*\.checked_mul\(8\)','state\.source_site_execution_count\s*==\s*sites')) { Assert-Cp420Text -Text $runtimeValidationText -Pattern $pattern -Description '54/49/5/40 route accounting' }
foreach ($file in @($transition,$accounting,$release,$releaseError,$runtimeValidation,$releaseSnapshot,$adapter,$coupled,$pipelineValidation,$pipelineLineage)) {
    Assert-NotContains -Path $file -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description 'production panic'
}
foreach ($pattern in @('mul_add\s*\(','clamp\s*\(','is_finite\s*\(','DirectZonePurchasedAirCouplingInput')) { Assert-NotContains -Path $transition -Pattern $pattern -Description 'forbidden arithmetic/numerical coupling' }

$predecessorCommittedText = Read-RepoText -Path $predecessorCommitted
$cp329CommittedText = Read-RepoText -Path $cp329Committed
$cp330CommittedText = Read-RepoText -Path $cp330Committed
foreach ($evidence in @(
    [PSCustomObject]@{ Text=$predecessorCommittedText; Anchor='pub\(in crate::ideal_loads::calc\) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route\s*\('; Name='sealed CP419 committed route' },
    [PSCustomObject]@{ Text=$cp329CommittedText; Anchor='pub\(in crate::ideal_loads::calc\) fn cooling_mixed_air_call_committed_latest_sensible_output_inputs\s*\('; Name='sealed CP329 sensible inputs' },
    [PSCustomObject]@{ Text=$cp330CommittedText; Anchor='pub\(in crate::ideal_loads::calc\) fn cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate\s*\('; Name='sealed CP330 positive flow' }
)) {
    $block = Get-Cp420BraceBlock -Text $evidence.Text -AnchorPattern $evidence.Anchor -Description $evidence.Name
    foreach ($forbidden in @('(?<![A-Za-z0-9_])predecessor_route\s*\(','private_[a-z0-9_]*characterization\s*\(','snapshot_is_exact_direct_release\s*\(')) {
        if ($block -match $forbidden) { throw "CP420 $($evidence.Name) recursively derives exact route through '$forbidden'" }
    }
}
foreach ($registration in @(
    [PSCustomObject]@{ Path=$predecessorModule; Pattern='PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentCommittedRoute' },
    [PSCustomObject]@{ Path=$cp329Module; Pattern='PurchasedAirCalcCoolingMixedAirCallCommittedSensibleOutputInputs' },
    [PSCustomObject]@{ Path=$cp330Module; Pattern='cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate' }
)) { Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description 'sealed operand capability re-export' }
Assert-Contains -Path $cp329Committed -Pattern 'cp329_sensible_owner_hot_path_has_no_recursive_exact_validation' -Description 'CP329 sealed-owner recursion-zero regression'
Assert-Contains -Path $cp330Committed -Pattern 'cp330_flow_owner_hot_path_has_no_recursive_exact_validation' -Description 'CP330 sealed-owner recursion-zero regression'
Assert-Contains -Path $predecessorCommitted -Pattern 'cp419_route_hot_path_has_no_recursive_exact_validation' -Description 'CP419 sealed-route recursion-zero regression'
$releaseText = Read-RepoText -Path $release
$hot = Get-Cp420BraceBlock -Text $releaseText -AnchorPattern "pub fn advance_direct_no_oa_calc_$stem\s*\(" -Description 'public hot release'
foreach ($pattern in @(
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_committed_latest_route\s*\(',
    'cooling_mixed_air_call_committed_latest_sensible_output_inputs\s*\(',
    'cooling_supply_mass_flow_positive_guard_committed_latest_supply_mass_flow_rate\s*\(',
    "cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_route_from_validated_predecessor\s*\(",
    'advance_with_validated_route\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_snapshot_has_exact_cp419_prefix_and_local_assignment\s*\('
)) { Assert-Cp420Text -Text $hot -Pattern $pattern -Description 'sealed non-recursive hot release' }
$recursiveForbidden = @(
    '(?<![A-Za-z0-9_])predecessor_route\s*\(','private_[a-z0-9_]*characterization\s*\(',
    'snapshot_is_exact(?:_direct_release)?\s*\(','completed_direct_[a-z0-9_]*_is_consistent\s*\(',
    'cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_route\s*\('
)
foreach ($pattern in $recursiveForbidden) { if ($hot -match $pattern) { throw "CP420 public hot release recursively validates through '$pattern'" } }
Assert-Contains -Path "$root\tests.rs" -Pattern 'release_hot_path_uses_only_committed_owners_and_validated_route' -Description 'CP420 public hot-path recursion-zero regression'

Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\s*=','let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\s*=','let\s+calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry\s*=','let\s+calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment\s*=','let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP419-to-CP420-to-CP421-to-CP422-to-CP423-to-CP424-to-CP425-to-numerical binding order'
Assert-PatternsInOrder -Path $scheduledOutput -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+coupling\s*:') -Description 'CP419-to-CP420 scheduled output order'
$bindingText = Read-RepoText -Path $binding
$dto = Get-Cp420BraceBlock -Text $bindingText -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' -Description 'numerical DTO'
if ($dto -match '(?i)cp420|dehumidification_guard_else_branch_sensible_output_assignment') { throw 'CP420 evidence entered numerical DTO' }
Assert-Contains -Path $coupled -Pattern 'snapshot_has_exact_cp419_prefix_and_local_assignment' -Description 'coupled bounded CP419-prefix/local validation'
foreach ($file in @($coupled,$pipelineValidation,$pipelineLineage)) {
    $text = (Read-RepoText -Path $file) -split '#\[cfg\(test\)\]', 2 | Select-Object -First 1
    foreach ($pattern in $recursiveForbidden) { if ($text -match $pattern) { throw "CP420 coupled/pipeline hot validation recursively validates through '$pattern' in $file" } }
}
Assert-Contains -Path $coupledFixture -Pattern 'fn\s+cp420_owner_input\s*\(' -Description 'coupled output owner-gated fixture'
Assert-Contains -Path $coupledFixture -Pattern "calculation_$predecessorStem" -Description 'coupled fixture CP419 activity gate'
Assert-Contains -Path $witness -Pattern ("set_" + $stem + "_latest_witness") -Description 'private witness setter'
Assert-PatternsInOrder -Path $pipelineRoot -Patterns @("$predecessorStem::\s*validate_direct_lifecycle","$stem::\s*validate_direct_lifecycle") -Description 'pipeline CP419-to-CP420 order'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp425_lifecycle_evidence' -Description 'cumulative non-direct firewall'
Assert-Contains -Path $pipelineValidation -Pattern 'predecessor_cp419\s*:\s*Option<&PredecessorLifecycle>' -Description 'pipeline CP419 predecessor'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'mod\s+cp420_assertions' -Description 'arbitrary delegation module'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp420_assertions::assert_direct\(runtime,\s*results\)' -Description 'direct arbitrary delegation'
Assert-Contains -Path $arbitraryPredecessor -Pattern 'cp420_assertions::assert_non_direct\(runtime\)' -Description 'non-direct arbitrary delegation'
Assert-NotContains -Path $arbitrary -Pattern 'coupling_input.*(?:cp420|sensible_output_assignment)|(?:cp420|sensible_output_assignment).*coupling_input' -Description 'arbitrary numerical DTO feed'

$serializationText = Read-RepoText -Path $serialization
$predecessorSerialization = "crates\ep_run\src\pipeline\purchased_air_$predecessorStem\serialization\snapshot.rs"
$predecessorSerializationText = Read-RepoText -Path $predecessorSerialization
$predecessorJson = @([regex]::Matches($predecessorSerializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$json = @([regex]::Matches($serializationText, '(?m)^\s*"(?<key>[a-z][a-z0-9_]*)"\s*:') | ForEach-Object { $_.Groups['key'].Value })
$jsonTail = @(
    'predecessor_cp419_resulting_supply_humidity_ratio','predecessor_cp419_resulting_supply_humidity_ratio_ieee_bits',
    'predecessor_cp419_resulting_supply_enthalpy_j_per_kg','predecessor_cp419_resulting_supply_enthalpy_j_per_kg_ieee_bits',
    'predecessor_cp419_resulting_supply_temperature_c','predecessor_cp419_resulting_supply_temperature_c_ieee_bits',
    $localFields[0],$localFields[1],$localFields[2],$localFields[3],$localFields[4],$localFields[5],$localFields[6],
    'supply_mass_flow_rate_kg_per_s','supply_mass_flow_rate_kg_per_s_ieee_bits',$localFields[8],$localFields[9],
    'cp419_cp_air_for_sensible_output_j_per_kg_k','cp419_cp_air_for_sensible_output_j_per_kg_k_ieee_bits',$localFields[11],
    'supply_mass_flow_rate_times_cp_air_w_per_k','supply_mass_flow_rate_times_cp_air_w_per_k_ieee_bits',$localFields[13],$localFields[14],
    'mixed_air_temperature_for_sensible_output_c','mixed_air_temperature_for_sensible_output_c_ieee_bits',$localFields[16],$localFields[17],
    'supply_temperature_for_sensible_output_c','supply_temperature_for_sensible_output_c_ieee_bits',$localFields[19],
    'mixed_air_minus_supply_temperature_k','mixed_air_minus_supply_temperature_k_ieee_bits',$localFields[21],
    'calculated_cooling_sensible_output_w','calculated_cooling_sensible_output_w_ieee_bits',$localFields[23],
    'cooling_sensible_output_w','cooling_sensible_output_w_ieee_bits',
    'resulting_supply_humidity_ratio','resulting_supply_humidity_ratio_ieee_bits',
    'resulting_supply_enthalpy_j_per_kg','resulting_supply_enthalpy_j_per_kg_ieee_bits',
    'resulting_supply_temperature_c','resulting_supply_temperature_c_ieee_bits'
)
if ($predecessorJson.Count -ne 234 -or $json.Count -ne 273 -or ($json[0..227] -join '|') -cne ($predecessorJson[0..227] -join '|') -or ($json[228..272] -join '|') -cne ($jsonTail -join '|')) { throw 'CP420 JSON must preserve CP419 keys 0..227 and append the exact forty-five-key tail' }
foreach ($field in $numeric) {
    $escaped = [regex]::Escape($field)
    Assert-Cp420Text -Text $serializationText -Pattern ('(?m)^\s*"'+$escaped+'".*\r?\n\s*"'+$escaped+'_ieee_bits"') -Description 'adjacent IEEE sidecar'
}
foreach ($pattern in @('cp420_preserves_cp419_prefix_and_declares_273_lossless_keys','cp420_tail_is_predecessor_then_eight_site_local_then_terminal','nonfinite_projection_keeps_authoritative_bits')) { Assert-Contains -Path $serializationTests -Pattern $pattern -Description 'serializer regression coverage' }

$heading = 'CP420 post-saturation capacity-limit dehumidification-guard else-branch sensible-output assignment'
$docs = @('docs\src\current\current-status.md','docs\src\current\project-contract.md','docs\src\porting-map\heat-balance-source-map.md','docs\src\porting-map\ideal-loads-source-map.md','docs\src\porting-map\zone-air-update-map.md')
$canonicalSection = $null
foreach ($doc in $docs) {
    $docText = Read-RepoText -Path $doc
    $numbers = @([regex]::Matches($docText,'(?m)^## CP(?<number>40[9]|41[0-9]|420)\b') | ForEach-Object { [int]$_.Groups['number'].Value })
    if (($numbers -join '|') -cne '409|410|411|412|413|414|415|416|417|418|419|420') { throw "CP409-CP420 documentation order drift in $doc" }
    if ([regex]::Matches($docText,"(?m)^## $([regex]::Escape($heading))$").Count -ne 1) { throw "CP420 heading count drift in $doc" }
    $section = [regex]::Match($docText,"(?ms)^## $([regex]::Escape($heading))\r?\n(?<body>.*?)(?=^## CP421\b)").Groups['body'].Value.TrimEnd([char[]]"`r`n")
    if ($null -eq $canonicalSection) { $canonicalSection = $section } elseif ($section -cne $canonicalSection) { throw "CP420 manual documentation section drift in $doc" }
}
foreach ($pattern in @(
    'physical executable line 2331 exactly','line 2332.*?first\s+excluded.*?CP421','exact eight dependency-ordered sites',
    'T420=54','Z420=49','A420=5','S420=8\*A420=40','17/37','active public indices are 4 and\s+7','10, 13, and 16','Ten width-36',
    'sole immediate route\s+predecessor','CP330 solely owns','CP419 solely owns local `CpAir`','CP329 solely owns `MixedAirTemp`',
    'zero generic predecessor-route','202 base fields','seventy-one\s+`Option<f64>`','273 unique JSON keys','exact first 171 fields','exact first 228 keys','45-key tail',
    '36/41/51','owns only five local `CoolSensOutput` values','CP419-to-CP420-to-unchanged-numerical','110 to 111','never feeds',
    '32 algorithms','293 routines','58 `state_mapped`','235 `source_mapped`','358 total','240 public','118 internal','238 development commands'
)) { Assert-Cp420Text -Text $canonicalSection -Pattern "(?is)$pattern" -Description 'bounded documentation claim' }

foreach ($spec in @(
    [PSCustomObject]@{ Path='specs\algorithm_ledger.toml'; Anchor='CP420 supersedes only CP419' },
    [PSCustomObject]@{ Path='specs\capabilities.toml'; Anchor='CP420 additionally requires' }
)) {
    $text = Read-RepoText -Path $spec.Path
    $matches = [regex]::Matches($text,'(?m)^\s*"(?<body>' + [regex]::Escape($spec.Anchor) + '.*)",\r?$')
    if ($matches.Count -ne 1) { throw "CP420 expected one bounded addendum in $($spec.Path)" }
    foreach ($pattern in @('2331','2332.*?CP421','eight','54.*?49.*?(?:5|five).*?(?:40|forty)','17/37','ten width-36','202 base','seventy-one','273 JSON','228','45-key','110 to 111','358/240/118|358 total, 240 public, 118 internal')) { Assert-Cp420Text -Text $matches[0].Groups['body'].Value -Pattern "(?is)$pattern" -Description 'bounded spec claim' }
}
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP420 supersedes only CP419' -Description 'generated algorithm addendum'
Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP420 additionally requires' -Description 'generated capability addendum'
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP420\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP420\b' -Description 'psychrometrics-map non-promotion'

$ledger = Read-RepoText -Path 'specs\algorithm_ledger.toml'
if ([regex]::Matches($ledger,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.source_file\s*=').Count -ne 293 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or [regex]::Matches($ledger,'(?m)^routine\.[^.]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP420 algorithm/routine ledger counts drift' }
$auditRoot = 'scripts\quality\ideal-loads-structure-audit'
$audits = @(Get-ChildItem -LiteralPath $auditRoot -Filter 'cp*.ps1' -File)
foreach ($file in $audits) {
    if ($file.BaseName -notmatch '^cp(?<number>\d+)-') { continue }
    $number = [int]$Matches['number']
    if ($number -ge 334 -and $number -le 420) { Assert-Contains -Path $file.FullName -Pattern 'non_direct_runtime_rejects_cp316_through_cp425_lifecycle_evidence' -Description 'historical non-direct firewall' }
    if ($number -ge 337 -and $number -le 420) { Assert-Contains -Path $file.FullName -Pattern 'script_count = 363' -Description 'historical script count' }
    if ($number -ge 367 -and $number -le 420) { Assert-Contains -Path $file.FullName -Pattern 'Count -ne 123' -Description 'historical classification count' }
    if ($number -ge 335 -and $number -le 420) { Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 363 \|')) -Description 'historical generated total'; Assert-Contains -Path $file.FullName -Pattern ([regex]::Escape('\| 123 \|')) -Description 'historical generated internal total' }
}
$cleanup = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and [int]$Matches['number'] -ge 329 -and [int]$Matches['number'] -le 344 })
if ($cleanup.Count -ne 17) { throw 'CP420 helper-cleanup propagation set drift' }
foreach ($file in $cleanup) { Assert-Contains -Path $file.FullName -Pattern "advance_$stem" -Description 'historical helper whitelist' }
$terminalAudits = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<number>\d+)-' -and (([int]$Matches['number'] -ge 377 -and [int]$Matches['number'] -le 392) -or ([int]$Matches['number'] -ge 394 -and [int]$Matches['number'] -le 419)) })
if ($terminalAudits.Count -ne 43) { throw 'CP420 terminal propagation set drift' }
foreach ($file in $terminalAudits) { Assert-Contains -Path $file.FullName -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path $file.FullName -Pattern 'CP419-to-CP420' -Description 'historical CP419-to-CP420 interval'; Assert-Contains -Path $file.FullName -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path $file.FullName -Pattern 'CP420-to-CP421' -Description 'historical CP420-to-CP421 interval'; Assert-Contains -Path $file.FullName -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path $file.FullName -Pattern 'CP421-to-CP422' -Description 'historical CP421-to-CP422 interval'; Assert-Contains -Path $file.FullName -Pattern 'CP422-to-CP423' -Description 'CP422 terminal-to-numerical interval' }

$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'
$predecessorIndex = $master.IndexOf('cp419-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-cp-air-assignment.ps1')
$currentIndex = $master.IndexOf((Split-Path -Leaf $audit))
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($predecessorIndex -lt 0 -or $currentIndex -le $predecessorIndex -or $completionIndex -le $currentIndex -or [regex]::Matches($master,[regex]::Escape((Split-Path -Leaf $audit))).Count -ne 1) { throw 'Master CP419-to-CP420 registration order drift' }
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$calcRoot -Limit 99' -Description 'CP420 calc-root structural cap'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit.ps1' -Pattern 'Assert-LineLimit -Path \$idealLoadsInitWitnesses -Limit 272' -Description 'CP420 witness-root physical cap (245 nonblank)'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1' -Pattern 'Assert-LineLimit -Path \$cp339CalcRoot -Limit 99' -Description 'historical calc-root structural cap'

$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 363','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { Assert-Cp420Text -Text $inventory -Pattern $pattern -Description 'inventory count' }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 123) { throw 'CP420 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp420-cooling-post-saturation-capacity-limit-dehumidification-guard-else-branch-sensible-output-assignment\.ps1' -Description 'inventory record'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 363 \|' -Description 'generated script total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| public scripts \| 240 \|' -Description 'generated public total'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 123 \|' -Description 'generated internal total'
if ((Get-Content -LiteralPath 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1').Count -gt 1200) { throw 'CP345 line cap exceeded after CP420 terminal propagation' }; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal-to-numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-numerical')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host 'CP420 post-saturation dehumidification-guard else-branch sensible-output assignment structure audit passed.'
}
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-numerical' -Description 'CP425-to-numerical terminal interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment' -Description 'CP425 binding successor registration'
