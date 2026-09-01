# CP385 maps only PurchasedAirManager.cc executable line 2270 and stops at 2272.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment"
$pipelineStem = "purchased_air_$stem"
$lifecycle = "purchased_air_calc_$($stem)_lifecycle"
$commit = "6f2e40d10250a105b49966baa24d843711e61048"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$sites = @(
    "read-retained-mixed-air-enthalpy-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy-difference",
    "read-retained-cooling-total-output-for-post-saturation-capacity-limited-dehumidification-specific-cooling-output-division",
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limited-dehumidification-specific-cooling-output-division",
    "calculate-cooling-total-output-divided-by-supply-mass-flow-rate-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy",
    "calculate-mixed-air-enthalpy-minus-specific-cooling-output-for-post-saturation-capacity-limited-dehumidification-supply-enthalpy",
    "assign-local-supply-enthalpy-after-post-saturation-capacity-limited-dehumidification-total-output-adjustment"
)
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$release = "$root\release.rs"
$prefix = "$root\release\prefix_validation.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$releaseTests = "$root\tests\release.rs"
$cp384Release = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem\release.rs"
$cp384SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem\release\snapshot_validation.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipelineCounts = "crates\ep_run\src\pipeline\$pipelineStem\validation\counts.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$cp384Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp384_assertions.rs"
$cp385Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp385-cooling-post-saturation-capacity-limit-dehumidification-total-output-supply-enthalpy-assignment.ps1"

function Assert-Cp385Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP385 $Description missing" }
}

function Get-Cp385BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP385 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP385 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring($anchor.Index, $index - $anchor.Index + 1)
            }
        }
    }
    throw "CP385 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, "$root\transition\accounting.rs", $release,
    $prefix, $snapshotValidation, $releaseTests, $cp384Release, $cp384SnapshotValidation,
    $adapter, "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs",
    "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs",
    "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs",
    "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs",
    "crates\ep_run\src\pipeline\$pipelineStem.rs", $pipelineCounts, $snapshotJson,
    $cp384Assertions, $cp385Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP385 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP385 bounded file"
}
foreach ($directory in @($root, "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation", "crates\ep_run\src\pipeline\$pipelineStem")) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP385 bounded recursive file"
    }
}
$core = (Get-ChildItem -LiteralPath $root -Recurse -File -Filter "*.rs" | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP385 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2269].Trim() -cne 'SupplyEnthalpy = MixedAirEnthalpy - CoolTotOutput / SupplyMassFlowRate;' -or
    $lines[2270].Trim() -cne '// Adjust output based on dehumidification control type' -or
    $lines[2271].Trim() -cne 'switch (PurchAir.DehumidCtrlType) {') {
    throw "CP385 source boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2270' -Description "mapped source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2272' -Description "first excluded source"
Assert-ExactStringArray -Path $module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER' -Expected $sites -Description "six-site source order"

$stateText = Read-RepoText -Path $state
$routeBlock = Get-Cp385BraceBlock -Text $stateText -AnchorPattern 'enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedRoute' -Description "route enum"
if ([regex]::Matches($routeBlock, '(?m)^\s{4}[A-Z][A-Za-z0-9]+,\s*$').Count -ne 23) {
    throw "CP385 retained route enum must contain exactly twenty-three variants"
}
foreach ($counter in @(
        'transition_count',
        'dehumidification_total_output_capacity_guard_evaluation_count',
        'dehumidification_total_output_capacity_guard_false_fallthrough_count',
        'dehumidification_total_output_maximum_capacity_assignment_count',
        'post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count',
        'source_site_execution_count',
        'cp379_retained_supply_enthalpy_owned_read_count',
        'cp329_retained_mixed_air_enthalpy_owned_read_count',
        'mixed_air_enthalpy_read_count',
        'cp384_retained_cooling_total_output_owned_read_count',
        'cooling_total_output_read_count',
        'cp330_retained_supply_mass_flow_rate_owned_read_count',
        'supply_mass_flow_rate_read_count',
        'specific_cooling_output_calculation_count',
        'supply_enthalpy_difference_calculation_count',
        'supply_enthalpy_assignment_write_count'
    )) {
    Assert-Contains -Path $state -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'supply_enthalpy_assignment_executed',
        'preexisting_supply_enthalpy_j_per_kg',
        'cp379_retained_supply_enthalpy_owned_read',
        'cp329_retained_mixed_air_enthalpy_owned_read',
        'mixed_air_enthalpy_j_per_kg',
        'cp384_retained_cooling_total_output_owned_read',
        'cooling_total_output_w',
        'cp330_retained_supply_mass_flow_rate_owned_read',
        'supply_mass_flow_rate_kg_per_s',
        'specific_cooling_output_j_per_kg',
        'calculated_supply_enthalpy_j_per_kg',
        'assigned_supply_enthalpy_j_per_kg',
        'resulting_supply_enthalpy_j_per_kg'
    )) {
    Assert-Contains -Path $module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
foreach ($pattern in @(
        'predecessor_cp384',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_is_consistent',
        'direct_predecessor_is_retained_and_complete',
        'retained_cp382_lineage_is_exact',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent',
        'cp382\.mixed_air_enthalpy_j_per_kg',
        'cp382\.supply_mass_flow_rate_kg_per_s',
        'cp382\.supply_enthalpy_j_per_kg',
        'predecessor\.resulting_cooling_total_output_w',
        'source_site_execution_count \+= 6',
        'cooling_total_output_w / operands\.supply_mass_flow_rate_kg_per_s',
        'mixed_air_enthalpy_j_per_kg - specific_cooling_output'
    )) {
    Assert-Cp385Text -Text $core -Pattern $pattern -Description "predecessor/operand/arithmetic $pattern"
}
foreach ($forbidden in @(
        '\bmul_add\b', '\btotal_cmp\b', '\bf64::max\b', '\bf64::min\b',
        '\bepsilon\b', '\bclamp\b', '\bpartial_cmp\b', '\brecip\b',
        'DirectZonePurchasedAirCouplingInput', 'reconcile_', 'PsyHFn|PsyWFn'
    )) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "forbidden transform/feed $forbidden"
}
foreach ($path in @($transition, $release, $adapter, "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs")) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node_update|report' -Description "adapter numerical reconciliation"

$strictCp384Shape = 'cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_control_flow_shape_is_exact'
Assert-Contains -Path $cp384Release -Pattern $strictCp384Shape -Description "shared strict CP384 control-flow predicate export"
Assert-Contains -Path $cp384SnapshotValidation -Pattern ("pub\(in crate::ideal_loads::calc\) fn " + $strictCp384Shape) -Description "strict CP384 control-flow predicate owner"
Assert-Contains -Path $cp384SnapshotValidation -Pattern ("snapshot_route[\s\S]+" + $strictCp384Shape) -Description "CP384 exact validator strict control-flow gate"
Assert-Contains -Path $snapshotValidation -Pattern ("predecessor_shape\(snapshot\)[\s\S]+" + $strictCp384Shape + "\(predecessor\)[\s\S]+predecessor_route\(predecessor\)") -Description "CP385 standalone strict predecessor gate order"
foreach ($flag in @(
        'predecessor_capacity_limit_guard_evaluated',
        'predecessor_capacity_limit_body_entered',
        'predecessor_active_capacity_limit_guard_false_fallthrough',
        'predecessor_dehumidification_guard_evaluated',
        'predecessor_dehumidification_body_entered',
        'predecessor_dehumidification_guard_false_fallthrough',
        'predecessor_dehumidification_total_output_assignment_executed',
        'predecessor_dehumidification_total_output_capacity_guard_evaluated',
        'predecessor_dehumidification_total_output_capacity_adjustment_body_entered',
        'predecessor_dehumidification_total_output_capacity_guard_false_fallthrough'
    )) {
    Assert-Contains -Path $releaseTests -Pattern ("!snapshot\s*\.\s*" + $flag) -Description "single-bit inherited control-flow drift $flag"
}
foreach ($route in @(
        'snapshot_for_route\(0, 0, false\)',
        'snapshot_for_route\(3, 0, false\)',
        'snapshot_for_route\(3, 2, false\)',
        'snapshot_for_route\(3, 1, false\)',
        'snapshot_for_route\(3, 1, true\)'
    )) {
    Assert-Contains -Path $releaseTests -Pattern $route -Description "U/K/X/F/M control-flow drift route"
}
Assert-Contains -Path $releaseTests -Pattern 'for flag_index in 0\.\.10' -Description "ten inherited control-flow single-bit drifts per route"

$bindingText = Read-RepoText -Path $binding
$cp384Index = $bindingText.IndexOf("let calculation_$predecessorStem =")
$cp385Index = $bindingText.IndexOf("let calculation_$stem =")
$cp386Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =")
$cp387Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =")
$cp388Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =")
$cp389Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =")
$cp390Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =")
$cp391Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
$cp392Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp384Index -lt 0 -or $cp385Index -le $cp384Index -or $cp386Index -le $cp385Index -or $cp387Index -le $cp386Index -or $cp388Index -le $cp387Index -or $cp389Index -le $cp388Index -or $cp390Index -le $cp389Index -or $cp391Index -le $cp390Index -or $cp392Index -le $cp391Index -or $numericalIndex -le $cp392Index) {
    throw "Binding must execute CP384, CP385, CP386, CP387, CP388, CP389, CP390, CP391, then unchanged numerical coupling"
}
$dto = Get-Cp385BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match 'cp385|cp386|cp387|cp388|post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment|dehumidification_control_switch|constant_sensible_heat_ratio_(?:cp_air|sensible_output)_assignment') {
    throw "CP385-CP388 numerical DTO feed unexpectedly present"
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

Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp436_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineRoot -Pattern $lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @(
        'post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count',
        'cp379_retained_supply_enthalpy_owned_read_count',
        'cp329_retained_mixed_air_enthalpy_owned_read_count',
        'cp384_retained_cooling_total_output_owned_read_count',
        'cp330_retained_supply_mass_flow_rate_owned_read_count',
        'specific_cooling_output_calculation_count',
        'supply_enthalpy_difference_calculation_count',
        'supply_enthalpy_assignment_write_count',
        'checked_mul\(assignments, 6'
    )) {
    Assert-Contains -Path $pipelineCounts -Pattern $pattern -Description "checked count $pattern"
}
foreach ($pattern in @(
        'json_number', 'ieee_bits', 'preexisting_supply_enthalpy_j_per_kg',
        'mixed_air_enthalpy_j_per_kg', 'cooling_total_output_w',
        'supply_mass_flow_rate_kg_per_s', 'specific_cooling_output_j_per_kg',
        'calculated_supply_enthalpy_j_per_kg', 'assigned_supply_enthalpy_j_per_kg',
        'resulting_supply_enthalpy_j_per_kg'
    )) {
    Assert-Contains -Path $snapshotJson -Pattern $pattern -Description "JSON/IEEE evidence $pattern"
}
Assert-Contains -Path $cp384Assertions -Pattern 'mod cp385_assertions;' -Description "arbitrary module"
Assert-Contains -Path $cp384Assertions -Pattern 'cp385_assertions::assert_direct\(runtime, results\)' -Description "direct delegation"
Assert-Contains -Path $cp384Assertions -Pattern 'cp385_assertions::assert_non_direct\(runtime\)' -Description "non-direct delegation"
Assert-Contains -Path $cp385Assertions -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 numerical nonfeed"
Assert-Contains -Path $cp385Assertions -Pattern 'mod cp386_assertions;' -Description "CP386 arbitrary module"
Assert-Contains -Path $cp385Assertions -Pattern 'cp386_assertions::assert_direct\(runtime, results\)' -Description "CP386 direct delegation"
Assert-Contains -Path $cp385Assertions -Pattern 'cp386_assertions::assert_non_direct\(runtime\)' -Description "CP386 non-direct delegation"
Assert-NotContains -Path $cp385Assertions -Pattern '(?:latest|cp385|results)\["(?:supply_node|report)' -Description "CP385 node/report nonfeed"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP385 supersedes only [^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP385 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) {
    throw "CP385 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, '2270', '2272', 'CP386', 'six',
            $sites[0], $sites[5], 'twenty-three', '6\*S', 'CP384',
            'sole (?:immediate )?predecessor', 'CP382', 'CP329', 'CP330', 'CP379',
            'division', 'subtraction', 'sidecar', 'DirectZonePurchasedAirCouplingInput',
            '323 total', '240 public', '83 internal', 'zero unused',
            'zero unreachable', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP385 spec addendum missing '$pattern'" }
    }
}

$docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP385 Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Supply-Enthalpy Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP385 Source-Ordered Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Supply-Enthalpy Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP385 Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Supply-Enthalpy Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP385 Post-Saturation Capacity-Limit Dehumidification Total-Output Supply-Enthalpy Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP385 Post-Saturation Capacity-Limit Dehumidification Total-Output Supply-Enthalpy Assignment Placement' }
)
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP385 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..385) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP385 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($pattern in @($commit, $hash, '2270', '2272', 'CP386', 'twenty-three', '6\*S', 'CP384', 'CP382', '323\s+total', '83\s+internal')) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP385 documentation in $($doc.Path) missing '$pattern'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP385\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP385 supersedes only' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP385 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..384) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp436_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..384) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 374 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 134 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..384) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 374' -Description "historical inventory total"
}
foreach ($historical in 367..384) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 134' -Description "historical internal classification count"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 122 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$stem" -Description "historical CP385 compact binding order"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP385 helper whitelist"
}
foreach ($historical in 360..384) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp385BindingIndex' -Description "historical CP385 binding index"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern 'CP384-to-CP385' -Description "CP345 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP388-to-CP389' -Description "CP345 CP389 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP389-to-CP390' -Description "CP345 CP390 predecessor interval"
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 historical audit"
foreach ($historical in 364, 373, 374, 376, 377, 378, 379, 380, 381, 382, 383, 384) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp385_assertions\.rs' -Description "historical CP385 arbitrary terminal"
}
$oldAlgorithm = [regex]::Matches($algorithmText, '(?m)^\s*"CP384 supersedes only [^"\r\n]+",\s*$')
$oldCapability = [regex]::Matches($capabilityText, '(?m)^\s*"CP384 additionally requires[^"\r\n]+",\s*$')
if ($oldAlgorithm.Count -ne 2 -or $oldCapability.Count -ne 2) { throw "CP384 historical addenda count drift" }
foreach ($claim in @($oldAlgorithm + $oldCapability)) {
    if ($claim.Value -notmatch '322 total' -or $claim.Value -notmatch '82 internal') {
        throw "CP384 historical addendum inventory numbers must remain 322/82"
    }
}

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp384AuditIndex = $master.IndexOf("cp384-cooling-post-saturation-capacity-limit-dehumidification-total-output-maximum-capacity-assignment.ps1")
$cp385AuditIndex = $master.IndexOf("cp385-cooling-post-saturation-capacity-limit-dehumidification-total-output-supply-enthalpy-assignment.ps1")
$cp386AuditIndex = $master.IndexOf("cp386-cooling-post-saturation-capacity-limit-dehumidification-control-switch.ps1")
$cp387AuditIndex = $master.IndexOf("cp387-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1")
$cp388AuditIndex = $master.IndexOf("cp388-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
$cp389AuditIndex = $master.IndexOf("cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment.ps1")
$cp390AuditIndex = $master.IndexOf("cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1")
$cp391AuditIndex = $master.IndexOf("cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp384AuditIndex -lt 0 -or $cp385AuditIndex -le $cp384AuditIndex -or $cp386AuditIndex -le $cp385AuditIndex -or $cp387AuditIndex -le $cp386AuditIndex -or $cp388AuditIndex -le $cp387AuditIndex -or $cp389AuditIndex -le $cp388AuditIndex -or $cp390AuditIndex -le $cp389AuditIndex -or $cp391AuditIndex -le $cp390AuditIndex -or $cp392AuditIndex -le $cp391AuditIndex -or $completionIndex -le $cp392AuditIndex) {
    throw "Master audit must dot-source CP385 through CP391 after CP384 before completion"
}
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 374', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp385Text -Text $inventory -Pattern $pattern -Description "inventory $pattern"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 134) {
    throw "CP385 inventory must be exactly 240 public and 122 internal scripts"
}
Assert-Cp385Text -Text $inventory -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp385-cooling-post-saturation-capacity-limit-dehumidification-total-output-supply-enthalpy-assignment\.ps1"' -Description "inventory record"
foreach ($pattern in @(
        '\| 374 \|',
        '\| public scripts \| 240 \|',
        '\| 134 \|',
        '\| scripts without callers \| 0 \|'
    )) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated inventory $pattern"
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP385 post-saturation dehumidification total-output supply-enthalpy assignment structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-numerical' -Description 'CP436-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'
