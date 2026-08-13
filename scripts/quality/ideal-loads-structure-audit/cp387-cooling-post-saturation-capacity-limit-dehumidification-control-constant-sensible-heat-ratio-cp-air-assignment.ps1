# CP387 maps PurchasedAirManager.cc physical lines 2273-2277 and stops at 2278.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_switch"
$successorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignment"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$commit = "6f2e40d10250a105b49966baa24d843711e61048"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$routes = "$root\transition\routes.rs"
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
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp387.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$cp386Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp386_assertions.rs"
$cp387Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp387_assertions.rs"
$cp388Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp388_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp387-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1"
$sites = @(
    "enter-purchased-air-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case",
    "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-cp-air",
    "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-cp-air",
    "assign-local-cp-air-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-case"
)

function Assert-Cp387Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP387 $Description missing '$Pattern'" }
}

function Get-Cp387BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP387 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP387 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP387 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, "$root\release\private_characterization.rs", $tests, $routeTests,
    $adapter, "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs", $coupled, $coupledTests,
    "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs",
    "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs",
    $pipeline, $pipelineValidation, $serialization, $snapshotJson,
    $cp386Assertions, $cp387Assertions, $cp388Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP387 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP387 bounded file"
}
foreach ($directory in @($root, "crates\ep_run\src\pipeline\$pipelineStem")) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP387 bounded recursive file"
    }
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP387 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2272].Trim() -cne 'case HumControl::ConstantSensibleHeatRatio: {' -or
    $lines[2276].Trim() -cne 'CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat);' -or
    $lines[2277].Trim() -cne 'CoolSensOutput = CoolTotOutput * PurchAir.CoolSHR;') {
    throw "CP387 source slice or CP388 boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2273-2277' -Description "mapped source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2278' -Description "first excluded source"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites `
    -Description "exact four-site source order"

$core = (Get-ChildItem -LiteralPath $root -Recurse -File -Filter "*.rs" | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine
foreach ($counter in @(
        'transition_count', 'inactive_transition_count',
        'dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count',
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]', 'source_site_execution_count',
        'dehumidification_control_constant_sensible_heat_ratio_case_entry_count',
        'mixed_air_humidity_ratio_read_count', 'psychrometric_cp_air_evaluation_count',
        'cp_air_assignment_write_count'
    )) {
    Assert-Contains -Path $state -Pattern "pub $counter" -Description "state counter $counter"
}
foreach ($field in @(
        'dehumidification_control_constant_sensible_heat_ratio_case_entered',
        'dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed',
        'mixed_air_humidity_ratio_read', 'mixed_air_humidity_ratio',
        'psychrometric_cp_air_evaluated', 'psychrometric_cp_air_result_j_per_kg_k',
        'cp_air_assigned', 'cp_air_j_per_kg_k', 'resulting_supply_enthalpy_j_per_kg'
    )) {
    Assert-Contains -Path $module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
foreach ($pattern in @(
        'cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact',
        'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        '(?:DehumidificationControlType|D)::ConstantSensibleHeatRatio',
        'energyplus_psy_cp_air_fn_w\(mixed_air_humidity_ratio\)',
        '!mixed_air_humidity_ratio\.is_finite\(\)\s*\|\|\s*mixed_air_humidity_ratio\s*<\s*0\.0',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_switch_is_consistent',
        'predecessor_resulting_supply_enthalpy_j_per_kg'
    )) {
    Assert-Cp387Text -Text $core -Pattern $pattern -Description "core contract"
}
foreach ($expectation in @(
        [PSCustomObject]@{ Pattern = 'fn\s+cp387_has_twenty_seven_inactive_and_three_constant_shr_assignment_routes\s*\('; Description = '30-route test' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(snapshots\.len\(\),\s*30\)'; Description = '30 snapshots' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.transition_count,\s*30\)'; Description = '30 transitions' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.inactive_transition_count,\s*27\)'; Description = '27 inactive routes' },
        [PSCustomObject]@{ Pattern = '(?s)dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count,\s*3'; Description = 'three active assignments' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.predecessor_route_counts,\s*\[1;\s*30\]\)'; Description = 'exact route parity' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.source_site_execution_count,\s*12\)'; Description = 'four sites times three routes' },
        [PSCustomObject]@{ Pattern = '(?s)snapshot_is_exact_direct_release\(.*?\)\s*\}\)\s*\.count\(\),\s*11'; Description = '11 public exact-direct routes' },
        [PSCustomObject]@{ Pattern = 'fn\s+canonical_cp_air_preserves_signed_zero_operands_without_clamping\s*\('; Description = 'signed-zero canonical helper test' },
        [PSCustomObject]@{ Pattern = 'fn\s+cp386_metadata_selector_and_enthalpy_corruption_are_rejected_atomically\s*\('; Description = 'predecessor corruption test' },
        [PSCustomObject]@{ Pattern = 'fn\s+all_ten_inherited_control_flags_are_revalidated_across_u_k_x_f_m_routes\s*\('; Description = 'control-flow corruption test' }
    )) {
    $testText = (Read-RepoText -Path $tests) + [Environment]::NewLine + (Read-RepoText -Path $routeTests)
    Assert-Cp387Text -Text $testText -Pattern $expectation.Pattern -Description "$($expectation.Description)"
}
foreach ($path in @($transition, $release, $adapter, $coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $transition -Pattern 'energyplus_moist_air_specific_heat|energyplus_psy_cp_air_fn_w_fast|dwSave|cpaSave|-100\.0|Mutex|OnceLock|thread_local|CoolSensOutput|CoolSHR|DirectZonePurchasedAirCouplingInput' -Description "cache, substitute helper, CP388, or numerical feed"
Assert-Contains -Path $prefix -Pattern 'PurchasedAirCalcCoolingMixedAirCallSnapshot' -Description "CP329 retained operand owner type"
Assert-Contains -Path $prefix -Pattern 'cooling_mixed_air_call_latest_witness' -Description "CP329 private witness owner"
Assert-Contains -Path $prefix -Pattern 'cooling_mixed_air_call_snapshots_match_bit_exact' -Description "CP329 retained/witness bit parity"
Assert-Contains -Path $prefix -Pattern 'owner\.parent_call_ordinal\s*==\s*predecessor\.parent_call_ordinal' -Description "same-call CP329 owner"
Assert-Contains -Path $prefix -Pattern 'mixed_air_humidity_ratio:\s*owner\.mixed_air_humidity_ratio\?' -Description "CP329 operand extraction"
Assert-Contains -Path $tests -Pattern 'active_input_is_derived_only_from_same_call_bit_exact_cp329_owner_evidence' -Description "CP329 same-call owner test"
Assert-Contains -Path $tests -Pattern '(?s)wrong_bits.*?active_input_from_owner_for_test.*?is_none' -Description "CP329 owner bit-drift rejection"
Assert-Contains -Path $tests -Pattern 'private_active_characterization_requires_retained_cp329_latest_witness_and_completion' -Description "retained CP329 completion test"
Assert-Contains -Path $snapshotValidation -Pattern 'energyplus_psy_cp_air_fn_w\(humidity\)' -Description "canonical result revalidation"
Assert-Contains -Path $snapshotValidation -Pattern 'to_bits\(\)' -Description "bit-exact scalar and enthalpy validation"

$bindingText = Read-RepoText -Path $binding
$cp386Index = $bindingText.IndexOf("let calculation_$predecessorStem =")
$cp387Index = $bindingText.IndexOf("let calculation_$stem =")
$cp388Index = $bindingText.IndexOf("let calculation_$successorStem =")
$cp389Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =")
$cp390Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =")
$cp391Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
$cp392Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp386Index -lt 0 -or $cp387Index -le $cp386Index -or $cp388Index -le $cp387Index -or $cp389Index -le $cp388Index -or $cp390Index -le $cp389Index -or $cp391Index -le $cp390Index -or $cp392Index -le $cp391Index -or $numericalIndex -le $cp392Index) {
    throw "Binding must execute CP386, CP387, CP388, CP389, CP390, CP391, then unchanged numerical coupling"
}
$dto = Get-Cp387BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match 'cp387|cp388|constant_sensible_heat_ratio_(?:cp_air|sensible_output)_assignment|cp_air_j_per_kg_k|cooling_sensible_output_w') {
    throw "CP387/CP388 evidence unexpectedly feeds the numerical DTO"
}
Assert-NotContains -Path $adapter -Pattern 'DirectZonePurchasedAirCouplingInput|mixed_air_humidity_ratio|reconcile_|supply_node|report' -Description "adapter owner bypass or numerical feed"

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
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp421_lifecycle_evidence' -Description "cumulative non-direct firewall"
$lifecycleField = "purchased_air_calc_$($stem)_lifecycle"
Assert-Contains -Path $pipelineRoot -Pattern $lifecycleField -Description "pipeline lifecycle key"
$pipelineText = Read-RepoText -Path $pipelineRoot
$nonDirectValidation = Get-Cp387BraceBlock -Text $pipelineText -AnchorPattern 'fn\s+validate_runtime_demand_provenance\s*\(' -Description "non-direct production firewall"
Assert-Cp387Text -Text $nonDirectValidation -Pattern "(?s)\.$([regex]::Escape($lifecycleField))\s*\.is_some\s*\(\s*\)" -Description "production lifecycle Some rejection"
$nonDirectTest = Get-Cp387BraceBlock -Text $pipelineText -AnchorPattern 'fn\s+non_direct_runtime_rejects_cp316_through_cp421_lifecycle_evidence\s*\(' -Description "cumulative non-direct regression"
$escapedField = [regex]::Escape($lifecycleField)
Assert-Cp387Text -Text $nonDirectTest -Pattern "(?s)\.$escapedField\s*=\s*Some\(\s*ep_runtime::$($typeStem)LifecycleSummary\s*\{.*?validate_runtime_demand_provenance\(.*?Err\(.*?persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime.*?\.$escapedField\s*=\s*None;.*?\.$escapedField\s*\.is_none\s*\(\s*\)" -Description "concrete Some-to-Err-to-None regression"
foreach ($pattern in @('inactive_transition_count', 'cp_air_assignment_count', 'source_site_execution_count', 'predecessor_route_counts', 'cp_air_j_per_kg_k_ieee_bits')) {
    $path = $pipelineValidation
    if ($pattern -eq 'cp_air_j_per_kg_k_ieee_bits') { $path = $snapshotJson }
    Assert-Contains -Path $path -Pattern $pattern -Description "pipeline count/serialization $pattern"
}
Assert-Contains -Path $cp386Assertions -Pattern 'mod cp387_assertions;' -Description "arbitrary CP387 module"
Assert-Contains -Path $cp386Assertions -Pattern 'cp387_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp386Assertions -Pattern 'cp387_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-Contains -Path $cp387Assertions -Pattern 'mod cp388_assertions;' -Description "arbitrary CP388 module"
Assert-Contains -Path $cp387Assertions -Pattern 'cp388_assertions::assert_direct\(runtime, results\)' -Description "CP388 direct arbitrary delegation"
Assert-Contains -Path $cp387Assertions -Pattern 'cp388_assertions::assert_non_direct\(runtime\)' -Description "CP388 non-direct arbitrary delegation"
Assert-Contains -Path $cp388Assertions -Pattern 'CP388 lifecycle must remain outside numerical result state' -Description "terminal numerical nonfeed"
Assert-Contains -Path $cp387Assertions -Pattern '(?s)supply_node.*load.*report.*reconciled.*numerical_dto' -Description "node/load/report nonfeed set"
Assert-Contains -Path $coupledTests -Pattern '(?s)prediction.*predicted_loads.*total_output_required_w' -Description "coupled predicted-load nonfeed regression"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP387 supersedes CP386[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP387 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) {
    throw "CP387 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, '2273', '2277', '2278', 'four', 'thirty',
            'three', 'twenty-seven', 'CP386', 'sole (?:immediate )?predecessor',
            'CP329', 'energyplus_psy_cp_air_fn_w', 'dwSave.*cpaSave',
            'DirectZonePurchasedAirCouplingInput', '325 total', '240 public',
            '85 internal', 'zero unused', 'zero unreachable', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP387 spec addendum missing '$pattern'" }
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
    $sections = [regex]::Matches($text, '(?ms)^## CP387\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP387 documentation expected one section in $doc" }
    foreach ($pattern in @($commit, $hash, '2273', '2277', '2278', 'thirty|30', 'twenty-seven|27', 'three|3', 'CP329', '325\s+total', '85\s+internal')) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP387 documentation in $doc missing '$pattern'" }
    }
    $cp386Index = $text.LastIndexOf("## CP386 ")
    $cp387Index = $text.LastIndexOf("## CP387 ")
    if ($cp386Index -lt 0 -or $cp387Index -le $cp386Index) { throw "CP386-to-CP387 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP387\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP387 supersedes CP386' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP387 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..386) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp421_lifecycle_evidence' -Description "historical cumulative firewall"
}
foreach ($historical in 335..386) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 359 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 119 \|')) -Description "historical generated internal"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP387 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp386Call\s*=', '\$cp387Call\s*=', '\$cp388Call\s*=', '\$cp389Call\s*=', 'CP386-to-CP387', 'CP387-to-CP388', 'CP388-to-CP389', 'CP389-to-CP390')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "strict CP345 terminal ordering"
}

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp386AuditIndex = $master.IndexOf("cp386-cooling-post-saturation-capacity-limit-dehumidification-control-switch.ps1")
$cp387AuditIndex = $master.IndexOf("cp387-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1")
$cp388AuditIndex = $master.IndexOf("cp388-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
$cp389AuditIndex = $master.IndexOf("cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment.ps1")
$cp390AuditIndex = $master.IndexOf("cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1")
$cp391AuditIndex = $master.IndexOf("cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp386AuditIndex -lt 0 -or $cp387AuditIndex -le $cp386AuditIndex -or $cp388AuditIndex -le $cp387AuditIndex -or $cp389AuditIndex -le $cp388AuditIndex -or $cp390AuditIndex -le $cp389AuditIndex -or $cp391AuditIndex -le $cp390AuditIndex -or $cp392AuditIndex -le $cp391AuditIndex -or $completionIndex -le $cp392AuditIndex) {
    throw "Master audit must dot-source CP387 through CP391 after CP386 before completion"
}
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 359', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp387Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 119) {
throw "CP387 inventory must be exactly 240 public and 119 internal scripts"
}
Assert-Cp387Text -Text $inventory -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp387-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment\.ps1"' -Description "inventory record"
foreach ($pattern in @(
        '\| 359 \|', '\| public scripts \| 240 \|',
        '\| 119 \|', '\| scripts without callers \| 0 \|'
    )) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory"
}

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP390-to-CP391' -Description "CP345 CP390-to-CP391 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP391-to-CP392' -Description "CP345 CP391-to-CP392 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP396-to-CP397' -Description "CP345 CP396-to-CP397 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP397-to-CP398' -Description "CP345 CP397-to-CP398 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP400-to-CP401' -Description "CP345 CP400-to-CP401 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP401-to-CP402' -Description "CP345 CP401-to-CP402 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP402-to-CP403' -Description "CP345 CP402 terminal interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP403-to-CP404' -Description "CP345 CP402 terminal interval"
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-numerical' -Description 'CP421 terminal interval'
Write-Host "CP387 post-saturation constant-SHR CpAir-assignment structure audit passed."
}
