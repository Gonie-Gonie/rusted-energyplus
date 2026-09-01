# CP389 maps PurchasedAirManager.cc physical executable line 2279 and stops before 2281.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignment"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$commit = "6f2e40d10250a105b49966baa24d843711e61048"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$routes = "$root\transition\routes.rs"
$owners = "$root\transition\owners.rs"
$positiveGuardRoot = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard.rs"
$positiveGuardRelease = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\release.rs"
$positiveGuardPrefix = "crates\ep_runtime\src\ideal_loads\calc\cooling_supply_mass_flow_positive_guard\release\prefix_validation.rs"
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
$coupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp389.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$pipelineValidation = "crates\ep_run\src\pipeline\$pipelineStem\validation.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization.rs"
$snapshotJson = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$cp388Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp388_assertions.rs"
$cp389Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp389_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment.ps1"
$sites = @(
    "read-purchased-air-mixed-air-temperature-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-difference-minuend",
    "read-local-cooling-sensible-output-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-quotient-numerator",
    "read-local-cp-air-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-denominator-first-factor",
    "read-retained-supply-mass-flow-rate-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-denominator-second-factor",
    "calculate-cp-air-times-supply-mass-flow-rate-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-denominator",
    "calculate-cooling-sensible-output-divided-by-air-capacity-rate-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature-drop",
    "calculate-mixed-air-temperature-minus-sensible-temperature-drop-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-supply-temperature",
    "assign-purchased-air-supply-temperature-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-case"
)

function Assert-Cp389Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP389 $Description missing '$Pattern'" }
}

function Get-Cp389BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP389 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP389 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP389 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $routes, $owners, $accounting, $release, $prefix,
    $positiveGuardRoot, $positiveGuardRelease, $positiveGuardPrefix,
    $runtimeValidation, $snapshotValidation, "$root\release\private_characterization.rs",
    $tests, $routeTests, $adapter, "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs",
    $coupled, $coupledTests,
    "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs",
    "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs",
    $pipeline, $pipelineValidation, $serialization, $snapshotJson,
    $cp388Assertions, $cp389Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP389 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP389 bounded file"
}
foreach ($directory in @($root, "crates\ep_run\src\pipeline\$pipelineStem")) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP389 bounded recursive file"
    }
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP389 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2278].Trim() -cne 'PurchAir.SupplyTemp = PurchAir.MixedAirTemp - CoolSensOutput / (CpAir * SupplyMassFlowRate);' -or
    -not $lines[2279].Trim().StartsWith('//') -or
    $lines[2280].Trim() -cne 'PurchAir.SupplyTemp = min(PurchAir.SupplyTemp, PurchAir.MixedAirTemp);') {
    throw "CP389 source slice, comment-only line 2280, or CP390 boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2279' -Description "mapped source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2281' -Description "first excluded executable"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER' `
    -Expected $sites `
    -Description "exact eight-site source order"

$core = (Get-ChildItem -LiteralPath $root -Recurse -File -Filter "*.rs" | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine
foreach ($counter in @(
        'transition_count', 'inactive_transition_count',
        'dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count',
        'predecessor_route_counts\s*:\s*\[usize;\s*30\]', 'source_site_execution_count',
        'cp379_supply_temperature_state_owner_count', 'unchanged_supply_temperature_preservation_count',
        'mixed_air_temperature_owned_read_count', 'cooling_sensible_output_owned_read_count',
        'cp_air_owned_read_count', 'supply_mass_flow_rate_owned_read_count',
        'supply_mass_flow_rate_bit_corroboration_count', 'air_capacity_rate_calculation_count',
        'sensible_temperature_drop_calculation_count', 'supply_temperature_calculation_count',
        'supply_temperature_assignment_write_count'
    )) {
    Assert-Contains -Path $state -Pattern "pub $counter" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed',
        'predecessor_cooling_sensible_output_w', 'resulting_supply_enthalpy_j_per_kg',
        'dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed',
        'cp379_retained_supply_temperature_state_owned', 'preexisting_supply_temperature_c',
        'cp329_retained_mixed_air_temperature_owned_read', 'mixed_air_temperature_c',
        'cp388_retained_cooling_sensible_output_owned_read', 'cooling_sensible_output_w',
        'cp387_retained_cp_air_owned_read', 'cp_air_j_per_kg_k',
        'cp330_retained_supply_mass_flow_rate_owned_read', 'cp329_supply_mass_flow_rate_bit_corroborated',
        'supply_mass_flow_rate_kg_per_s', 'cp_air_times_supply_mass_flow_rate_w_per_k',
        'cooling_sensible_output_over_air_capacity_rate_k', 'calculated_supply_temperature_c',
        'assigned_supply_temperature_c', 'resulting_supply_temperature_c'
    )) {
    Assert-Contains -Path $module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
foreach ($pattern in @(
        'cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact',
        'matches!\(index,\s*18\s*\|\s*22\s*\|\s*28\)',
        'active\.cp_air_j_per_kg_k\s*\*\s*active\.supply_mass_flow_rate_kg_per_s',
        'active\.cooling_sensible_output_w\s*/\s*denominator',
        'active\.mixed_air_temperature_c\s*-\s*drop',
        'calculated\.or\(prepared\.preexisting_supply_temperature_c\)',
        'cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact',
        '(?s)predecessor\.predecessor_dehumidification_control_type_read\s*&&\s*predecessor\.predecessor_dehumidification_control_type\s*!=\s*owner\.predecessor_dehumidification_control_type',
        'cp334_supply_temperature_mixed_air_limit_owned_read',
        'cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read',
        'cooling_mixed_air_call_snapshot_is_exact_direct_release',
        'cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release',
        '!positive_guard_links_to_mixed_air_call\(flow,\s*mixed\)',
        'predecessor\.cooling_sensible_output_w', 'cp_air_matches_predecessor', 'to_bits\(\)'
    )) {
    Assert-Cp389Text -Text $core -Pattern $pattern -Description "core formula/owner contract"
}
Assert-Contains -Path $prefix -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_committed_latest_snapshot_is_consistent\s*\(' -Description "bounded CP388 committed predecessor proof"
Assert-NotContains -Path $prefix -Pattern 'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_is_consistent\s*\(' -Description "recursive CP388 predecessor completion"
foreach ($pattern in @(
        'cooling_supply_enthalpy_post_saturation_assignment_latest_witness\s*\(system\.id\)',
        '(?s)cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact\s*\(\s*retained_owner,\s*owner,\s*\)',
        '(?s)cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact\s*\(\s*owner_witness,\s*owner,\s*\)',
        'cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent\s*\('
    )) { Assert-Contains -Path $prefix -Pattern $pattern -Description "bounded CP379 retained/runtime owner proof $pattern" }
Assert-NotContains -Path $prefix -Pattern 'completed_direct_cooling_supply_enthalpy_post_saturation_assignment_is_consistent\s*\(' -Description "recursive CP379 side-owner completion"
foreach ($path in @($transition, $release, $adapter, $coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $transition -Pattern 'DirectZonePurchasedAirCouplingInput|\bmin\s*\(|mul_add|\.clamp\s*\(|\.recip\s*\(|is_finite|epsilon|tolerance' -Description "CP390, DTO, reassociation, or finite gate"
Assert-Contains -Path $owners -Pattern 'parent_call_ordinal' -Description "same-call owner validation"
Assert-Contains -Path $owners -Pattern 'transitive_owner_count' -Description "CP334-or-CP344 transitive owner validation"
foreach ($path in @($positiveGuardRoot, $positiveGuardRelease, $positiveGuardPrefix)) {
    Assert-Contains -Path $path -Pattern 'pub\(in crate::ideal_loads::calc\).*positive_guard_links_to_mixed_air_call' -Description "narrow CP329-to-CP330 lineage helper visibility"
}
Assert-Contains -Path $snapshotValidation -Pattern 'to_bits\(\)' -Description "bit-exact result and retained fields"

$testText = (Read-RepoText -Path $tests) + [Environment]::NewLine +
    ((Get-ChildItem -LiteralPath "$root\tests" -Recurse -File -Filter "*.rs" | ForEach-Object {
            Read-RepoText -Path $_.FullName
        }) -join [Environment]::NewLine)
foreach ($pattern in @(
        'assert_eq!\(snapshots\.len\(\),\s*30\)',
        'assert_eq!\(state\.transition_count,\s*30\)',
        'assert_eq!\(state\.inactive_transition_count,\s*27\)',
        '(?s)dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count,\s*3',
        'assert_eq!\(state\.predecessor_route_counts,\s*\[1;\s*30\]\)',
        'assert_eq!\(state\.source_site_execution_count,\s*24\)',
        'assert_eq!\(state\.cp379_supply_temperature_state_owner_count,\s*27\)',
        'assert_eq!\(state\.unchanged_supply_temperature_preservation_count,\s*24\)',
        '(?s)snapshot_is_exact_direct_release\(.*?\.count\(\),\s*11',
        'individually_exact_cp379_with_a_different_selector_is_rejected_atomically',
        'assert!\(chain\.cp388\.predecessor_dehumidification_control_type_read\)',
        '(?s)assert_eq!\(\s*chain\.cp388\.predecessor_dehumidification_control_type,\s*Some\(D::ConstantSupplyHumidityRatio\)',
        'sensible_output_assignment_snapshot_is_exact\(chain\.cp388\)',
        'individually_exact_cp329_and_cp330_from_different_branches_are_rejected_atomically',
        '-0\.0', 'f64::INFINITY', 'from_bits\(0x7ff8_', 'to_bits\(\)', 'overflow'
    )) {
    Assert-Cp389Text -Text $testText -Pattern $pattern -Description "route/retention/IEEE/corruption test"
}

$bindingText = Read-RepoText -Path $binding
$cp388Index = $bindingText.IndexOf("let calculation_$predecessorStem =")
$cp389Index = $bindingText.IndexOf("let calculation_$stem =")
$cp390Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =")
$cp391Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
$cp392Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp388Index -lt 0 -or $cp389Index -le $cp388Index -or $cp390Index -le $cp389Index -or $cp391Index -le $cp390Index -or $cp392Index -le $cp391Index -or $numericalIndex -le $cp392Index) {
    throw "Binding must execute CP388, CP389, CP390, CP391, then unchanged numerical coupling"
}
$dto = Get-Cp389BraceBlock -Text $bindingText.Substring($numericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
if ($dto -match 'cp389|assigned_supply_temperature_c|resulting_supply_temperature_c') {
    throw "CP389 evidence unexpectedly feeds the numerical DTO"
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
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp437_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineRoot -Pattern $lifecycleField -Description "pipeline lifecycle key"
$pipelineText = Read-RepoText -Path $pipelineRoot
$nonDirectValidation = Get-Cp389BraceBlock -Text $pipelineText -AnchorPattern 'fn\s+validate_runtime_demand_provenance\s*\(' -Description "non-direct production firewall"
Assert-Cp389Text -Text $nonDirectValidation -Pattern "(?s)\.$([regex]::Escape($lifecycleField))\s*\.is_some\s*\(\s*\)" -Description "production lifecycle Some rejection"
$nonDirectTest = Get-Cp389BraceBlock -Text $pipelineText -AnchorPattern 'fn\s+non_direct_runtime_rejects_cp316_through_cp437_lifecycle_evidence\s*\(' -Description "cumulative non-direct regression"
$escapedField = [regex]::Escape($lifecycleField)
Assert-Cp389Text -Text $nonDirectTest -Pattern "(?s)\.$escapedField\s*=\s*Some\(\s*ep_runtime::$($typeStem)LifecycleSummary\s*\{.*?validate_runtime_demand_provenance\(.*?Err\(.*?persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime.*?\.$escapedField\s*=\s*None;.*?\.$escapedField\s*\.is_none\s*\(\s*\)" -Description "concrete Some-to-Err-to-None regression"
foreach ($pattern in @('inactive_transition_count', 'supply_temperature_assignment_count', 'source_site_execution_count', 'predecessor_route_counts')) {
    Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description "pipeline count $pattern"
}
foreach ($pattern in @(
        'preexisting_supply_temperature_c_ieee_bits', 'mixed_air_temperature_c_ieee_bits',
        'cooling_sensible_output_w_ieee_bits', 'cp_air_j_per_kg_k_ieee_bits',
        'supply_mass_flow_rate_kg_per_s_ieee_bits', 'cp_air_times_supply_mass_flow_rate_w_per_k_ieee_bits',
        'cooling_sensible_output_over_air_capacity_rate_k_ieee_bits',
        'calculated_supply_temperature_c_ieee_bits', 'assigned_supply_temperature_c_ieee_bits',
        'resulting_supply_temperature_c_ieee_bits'
    )) {
    Assert-Contains -Path $snapshotJson -Pattern $pattern -Description "authoritative IEEE sidecar $pattern"
}
Assert-Contains -Path $cp388Assertions -Pattern 'mod cp389_assertions;' -Description "arbitrary CP389 module"
Assert-Contains -Path $cp388Assertions -Pattern 'cp389_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp388Assertions -Pattern 'cp389_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-Contains -Path $cp389Assertions -Pattern 'CP389 lifecycle must remain outside numerical result state' -Description "terminal numerical nonfeed"
Assert-Contains -Path $cp389Assertions -Pattern '(?s)supply_node.*load.*report.*reconciled.*numerical_dto' -Description "node/load/report nonfeed set"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP389 supersedes only CP388[^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP389 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) {
    throw "CP389 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, '2279', '2280', '2281', 'eight', 'thirty', '18', '22', '28',
            'twenty-seven', 'eleven', 'nineteen', '24', 'T389=T388', 'A389',
            'sensible-output assignment count', 'inactive_transition_count=T389-A389',
            'sole immediate predecessor', 'CP329', 'CP330', 'CP379', 'CP334', 'CP344',
            'CP385', 'product', 'division', 'subtraction', 'signed zero', 'overflow',
            'NaN', 'IEEE sidecars', 'DirectZonePurchasedAirCouplingInput',
            '327 total', '240 public', '87 internal', 'zero unused', 'zero unreachable',
            '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP389 spec addendum missing '$pattern'" }
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
    $sections = [regex]::Matches($text, '(?ms)^## CP389\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP389 documentation expected one section in $doc" }
    foreach ($pattern in @($commit, $hash, '2279', '2280', '2281', 'thirty|30', 'twenty-seven|27', 'three|3', '18', '22', '28', 'CP379', 'CP334', 'CP344', 'CP385', 'T389=T388', 'A389', '327\s+total', '87\s+internal')) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP389 documentation in $doc missing '$pattern'" }
    }
    $cp388Index = $text.LastIndexOf("## CP388 ")
    $cp389Index = $text.LastIndexOf("## CP389 ")
    if ($cp388Index -lt 0 -or $cp389Index -le $cp388Index) { throw "CP388-to-CP389 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP389\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP389 supersedes only CP388' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP389 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp437_lifecycle_evidence' -Description "historical cumulative firewall"
}
foreach ($historical in 335..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 375 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 135 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 375' -Description "historical current inventory total"
}
foreach ($historical in 367..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 135' -Description "historical current internal classification"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 135 internal' -Description "historical current classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP389 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp388Call\s*=', '\$cp389Call\s*=', '\$cp390Call\s*=', '\$cp391Call\s*=', '\$cp392Call\s*=', 'CP388-to-CP389', 'CP389-to-CP390', 'CP390-to-CP391', 'CP391-to-CP392', 'CP396-to-CP397', 'CP397-to-CP398', 'CP400-to-CP401', 'CP401-to-CP402', 'CP402-to-CP403', 'CP403-to-CP404')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "strict CP345 terminal ordering"
}
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 fixed structural cap"
foreach ($historical in 377..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP388-to-CP389' -Description "historical CP389 predecessor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP389-to-CP390' -Description "historical CP390 predecessor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP390-to-CP391' -Description "historical CP391 predecessor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP391-to-CP392' -Description "historical CP392 predecessor interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP396-to-CP397' -Description "historical CP396-to-CP397 interval"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP397-to-CP398' -Description "historical CP397-to-CP398 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP400-to-CP401' -Description "historical CP400-to-CP401 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP401-to-CP402' -Description "historical CP401-to-CP402 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP402-to-CP403' -Description "historical CP402 terminal interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'CP403-to-CP404' -Description "historical CP402 terminal interval"
}
foreach ($historical in 385..389) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp389Index\s*=' -Description "historical binding CP389 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp389AuditIndex\s*=' -Description "historical master CP389 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp390Index\s*=' -Description "historical binding CP390 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp390AuditIndex\s*=' -Description "historical master CP390 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp391Index\s*=' -Description "historical binding CP391 successor"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '\$cp391AuditIndex\s*=' -Description "historical master CP391 successor"
}

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp388AuditIndex = $master.IndexOf("cp388-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
$cp389AuditIndex = $master.IndexOf("cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment.ps1")
$cp390AuditIndex = $master.IndexOf("cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1")
$cp391AuditIndex = $master.IndexOf("cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp388AuditIndex -lt 0 -or $cp389AuditIndex -le $cp388AuditIndex -or $cp390AuditIndex -le $cp389AuditIndex -or $cp391AuditIndex -le $cp390AuditIndex -or $cp392AuditIndex -le $cp391AuditIndex -or $completionIndex -le $cp392AuditIndex) {
    throw "Master audit must dot-source CP389, CP390, and CP391 after CP388 before completion"
}
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 375', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp389Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 135) {
    throw "CP389 inventory must be exactly 240 public and 135 internal scripts"
}
Assert-Cp389Text -Text $inventory -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment\.ps1"' -Description "inventory record"
foreach ($pattern in @(
        '\| 375 \|', '\| public scripts \| 240 \|',
        '\| 135 \|', '\| scripts without callers \| 0 \|'
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP389 post-saturation constant-SHR supply-temperature-assignment structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-numerical' -Description 'CP437-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-numerical' -Description 'CP437-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-numerical' -Description 'CP437-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-numerical' -Description 'CP437-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-numerical' -Description 'CP437-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-numerical' -Description 'CP437-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-numerical' -Description 'CP437-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-numerical' -Description 'CP437-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp437Call' -Description 'CP437 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-CP437' -Description 'CP436-to-CP437 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP436-to-' + 'numerical') -Description 'stale CP436 numerical interval'
