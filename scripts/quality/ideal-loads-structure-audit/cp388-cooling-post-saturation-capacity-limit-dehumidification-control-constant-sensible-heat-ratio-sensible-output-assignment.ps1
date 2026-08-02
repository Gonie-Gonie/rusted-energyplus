# CP388 maps PurchasedAirManager.cc physical executable line 2278 and stops at 2279.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignment"
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
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp388.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$cp387Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp387_assertions.rs"
$cp388Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp388_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp388-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1"
$sites = @(
    "read-retained-cooling-total-output-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output-first-factor",
    "read-purchased-air-cooling-sensible-heat-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output-second-factor",
    "calculate-cooling-total-output-times-cooling-sensible-heat-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output",
    "assign-local-cooling-sensible-output-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-case"
)

function Assert-Cp388Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP388 $Description missing '$Pattern'" }
}

function Get-Cp388BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP388 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP388 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP388 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $owners, $accounting, $release, $prefix,
    $runtimeValidation, $snapshotValidation, "$root\release\private_characterization.rs",
    $tests, $routeTests, $adapter, "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs",
    $coupled, $coupledTests,
    "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs",
    "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs",
    $pipeline, $pipelineValidation, $serialization, $snapshotJson,
    $cp387Assertions, $cp388Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP388 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP388 bounded file"
}
foreach ($directory in @($root, "crates\ep_run\src\pipeline\$pipelineStem")) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP388 bounded recursive file"
    }
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP388 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2277].Trim() -cne 'CoolSensOutput = CoolTotOutput * PurchAir.CoolSHR;' -or
    $lines[2278].Trim() -cne 'PurchAir.SupplyTemp = PurchAir.MixedAirTemp - CoolSensOutput / (CpAir * SupplyMassFlowRate);') {
    throw "CP388 source slice or CP389 boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2278' -Description "mapped source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2279' -Description "first excluded source"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites `
    -Description "exact four-site source order"

$core = (Get-ChildItem -LiteralPath $root -Recurse -File -Filter "*.rs" | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine
foreach ($counter in @(
        'transition_count', 'inactive_transition_count',
        'dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count',
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]', 'source_site_execution_count',
        'cooling_total_output_owned_read_count', 'cooling_total_output_bit_corroboration_count',
        'cooling_sensible_heat_ratio_read_count', 'cooling_sensible_output_calculation_count',
        'cooling_sensible_output_assignment_write_count'
    )) {
    Assert-Contains -Path $state -Pattern "pub $counter" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed',
        'predecessor_cp_air_j_per_kg_k',
        'dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed',
        'cp384_retained_cooling_total_output_owned_read',
        'cp385_cooling_total_output_bit_corroborated', 'cooling_total_output_read',
        'cooling_total_output_w', 'cooling_sensible_heat_ratio_read',
        'cooling_sensible_heat_ratio', 'cooling_sensible_output_calculated',
        'calculated_cooling_sensible_output_w', 'cooling_sensible_output_assigned',
        'cooling_sensible_output_w', 'resulting_supply_enthalpy_j_per_kg'
    )) {
    Assert-Contains -Path $module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
foreach ($pattern in @(
        'cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact',
        'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'cooling_total_output_w\s*\*\s*cooling_sensible_heat_ratio',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent',
        'cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment',
        'cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment',
        'cooling_sensible_heat_ratio', 'to_bits\(\)'
    )) {
    Assert-Cp388Text -Text $core -Pattern $pattern -Description "core contract"
}
foreach ($path in @($transition, $release, $adapter, $coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $transition -Pattern 'SupplyTemp|MixedAirTemp|SupplyMassFlowRate|DirectZonePurchasedAirCouplingInput|mul_add|clamp' -Description "CP389, DTO, or substitute calculation"
Assert-Contains -Path $owners -Pattern 'resulting_cooling_total_output_w' -Description "CP384 resulting-output owner"
Assert-Contains -Path $owners -Pattern 'cooling_total_output_w' -Description "CP385 bridge value"
Assert-Contains -Path $prefix -Pattern 'cooling_sensible_heat_ratio' -Description "selected-system CoolSHR owner"
Assert-Contains -Path $owners -Pattern 'parent_call_ordinal' -Description "same-call owner validation"
Assert-Contains -Path $snapshotValidation -Pattern 'to_bits\(\)' -Description "bit-exact result and retained fields"

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine +
    ((Get-ChildItem -LiteralPath "$root\tests" -Recurse -File -Filter "*.rs" | ForEach-Object {
            Read-RepoText -Path $_.FullName
        }) -join [Environment]::NewLine)
foreach ($pattern in @(
        'assert_eq!\(snapshots\.len\(\),\s*30\)',
        'assert_eq!\(state\.transition_count,\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*27\)',
        '(?s)dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,\s*3',
        'assert_eq!\(state\.predecessor_route_counts,\s*\[1;\s*30\]\)',
        'assert_eq!\(state\.source_site_execution_count,\s*12\)',
        '(?s)snapshot_is_exact_direct_release\(.*?\)\s*\}\)\s*\.count\(\),\s*11',
        '-0\.0', 'f64::INFINITY', 'from_bits\(0x7ff8_', 'to_bits\(\)', 'overflow'
    )) {
    Assert-Cp388Text -Text $testText -Pattern $pattern -Description "route/IEEE/corruption test"
}

$bindingText = Read-RepoText -Path $binding
$cp387Index = $bindingText.IndexOf("let calculation_$predecessorStem =")
$cp388Index = $bindingText.IndexOf("let calculation_$stem =")
$cp389Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =")
$cp390Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =")
$cp391Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
$cp392Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp387Index -lt 0 -or $cp388Index -le $cp387Index -or $cp389Index -le $cp388Index -or $cp390Index -le $cp389Index -or $cp391Index -le $cp390Index -or $cp392Index -le $cp391Index -or $numericalIndex -le $cp392Index) {
    throw "Binding must execute CP387, CP388, CP389, CP390, CP391, then unchanged numerical coupling"
}
$dto = Get-Cp388BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match 'cp388|sensible_output_assignment|cooling_sensible_output_w') {
    throw "CP388 evidence unexpectedly feeds the numerical DTO"
}
Assert-NotContains -Path $adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node|report' -Description "adapter numerical feed"

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
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp394_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineRoot -Pattern $lifecycleField -Description "pipeline lifecycle key"
$pipelineText = Read-RepoText -Path $pipelineRoot
$nonDirectValidation = Get-Cp388BraceBlock -Text $pipelineText -AnchorPattern 'fn\s+validate_runtime_demand_provenance\s*\(' -Description "non-direct production firewall"
Assert-Cp388Text -Text $nonDirectValidation -Pattern "(?s)\.$([regex]::Escape($lifecycleField))\s*\.is_some\s*\(\s*\)" -Description "production lifecycle Some rejection"
$nonDirectTest = Get-Cp388BraceBlock -Text $pipelineText -AnchorPattern 'fn\s+non_direct_runtime_rejects_cp316_through_cp394_lifecycle_evidence\s*\(' -Description "cumulative non-direct regression"
$escapedField = [regex]::Escape($lifecycleField)
Assert-Cp388Text -Text $nonDirectTest -Pattern "(?s)\.$escapedField\s*=\s*Some\(\s*ep_runtime::$($typeStem)LifecycleSummary\s*\{.*?validate_runtime_demand_provenance\(.*?Err\(.*?persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime.*?\.$escapedField\s*=\s*None;.*?\.$escapedField\s*\.is_none\s*\(\s*\)" -Description "concrete Some-to-Err-to-None regression"
foreach ($pattern in @('inactive_transition_count', 'sensible_output_assignment_count', 'source_site_execution_count', 'predecessor_route_counts')) {
    Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline count $pattern"
}
foreach ($pattern in @('cooling_total_output_w_ieee_bits', 'cooling_sensible_heat_ratio_ieee_bits', 'cooling_sensible_output_w_ieee_bits')) {
    Assert-Contains -Path $snapshotJson -Pattern $pattern -Description "authoritative IEEE sidecar $pattern"
}
Assert-Contains -Path $cp387Assertions -Pattern 'mod cp388_assertions;' -Description "arbitrary CP388 module"
Assert-Contains -Path $cp387Assertions -Pattern 'cp388_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp387Assertions -Pattern 'cp388_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-Contains -Path $cp388Assertions -Pattern 'CP388 lifecycle must remain outside numerical result state' -Description "terminal numerical nonfeed"
Assert-Contains -Path $cp388Assertions -Pattern '(?s)supply_node.*load.*report.*reconciled.*numerical_dto' -Description "node/load/report nonfeed set"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP388 supersedes only CP387[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP388 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) {
    throw "CP388 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, '2278', '2279', 'four', 'thirty', 'three',
            'twenty-seven', 'eleven', 'nineteen', 'CP387', 'sole immediate predecessor',
            'CP384', 'CP385', 'cooling_sensible_heat_ratio', 'signed zero',
            'infinities|infinity', 'NaN', 'IEEE sidecars', 'DirectZonePurchasedAirCouplingInput',
            '326 total', '240 public', '86 internal', 'zero unused', 'zero unreachable',
            '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP388 spec addendum missing '$pattern'" }
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
    $sections = [regex]::Matches($text, '(?ms)^## CP388\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP388 documentation expected one section in $doc" }
    foreach ($pattern in @($commit, $hash, '2278', '2279', 'thirty|30', 'twenty-seven|27', 'three|3', 'CP384', 'CP385', 'CP387', '326\s+total', '86\s+internal')) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP388 documentation in $doc missing '$pattern'" }
    }
    $cp387Index = $text.LastIndexOf("## CP387 ")
    $cp388Index = $text.LastIndexOf("## CP388 ")
    if ($cp387Index -lt 0 -or $cp388Index -le $cp387Index) { throw "CP387-to-CP388 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP388\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP388 supersedes only CP387' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP388 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..387) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp394_lifecycle_evidence' -Description "historical cumulative firewall"
}
foreach ($historical in 335..387) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 332 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 92 \|')) -Description "historical generated internal"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP388 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp387Call\s*=', '\$cp388Call\s*=', '\$cp389Call\s*=', 'CP387-to-CP388', 'CP388-to-CP389', 'CP389-to-CP390')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "strict CP345 terminal ordering"
}

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp387AuditIndex = $master.IndexOf("cp387-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1")
$cp388AuditIndex = $master.IndexOf("cp388-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
$cp389AuditIndex = $master.IndexOf("cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment.ps1")
$cp390AuditIndex = $master.IndexOf("cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1")
$cp391AuditIndex = $master.IndexOf("cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp387AuditIndex -lt 0 -or $cp388AuditIndex -le $cp387AuditIndex -or $cp389AuditIndex -le $cp388AuditIndex -or $cp390AuditIndex -le $cp389AuditIndex -or $cp391AuditIndex -le $cp390AuditIndex -or $cp392AuditIndex -le $cp391AuditIndex -or $completionIndex -le $cp392AuditIndex) {
    throw "Master audit must dot-source CP388 and CP389 after CP387 before completion"
}
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 332', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp388Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 92) {
throw "CP388 inventory must be exactly 240 public and 92 internal scripts"
}
Assert-Cp388Text -Text $inventory -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp388-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment\.ps1"' -Description "inventory record"
foreach ($pattern in @(
        '\| executable script records \| 332 \|', '\| public scripts \| 240 \|',
        '\| internal scripts \| 92 \|', '\| scripts without callers \| 0 \|'
    )) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory"
}

Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP390-to-CP391' -Description "CP345 CP390-to-CP391 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP391-to-CP392' -Description "CP345 CP391-to-CP392 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP394-to-numerical' -Description "CP345 CP393 terminal interval"
Write-Host "CP388 post-saturation constant-SHR sensible-output-assignment structure audit passed."
}
