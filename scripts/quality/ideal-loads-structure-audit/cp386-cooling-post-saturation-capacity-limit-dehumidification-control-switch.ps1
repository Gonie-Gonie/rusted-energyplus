# CP386 maps PurchasedAirManager.cc physical line 2272 only:
# switch (PurchAir.DehumidCtrlType) {
# Line 2273 is the first excluded lexical construct and line 2277 is the
# first excluded executable statement.
& {
$stem = "cooling_post_saturation_capacity_limit_dehumidification_control_switch"
$successorStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment"
$terminalStem = "cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment"
$predecessorStem = "cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment"
$pipelineStem = "purchased_air_$stem"
$typeStem = "PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitch"
$source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$commit = "6f2e40d10250a105b49966baa24d843711e61048"
$hash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"
$transition = "$root\transition.rs"
$accounting = "$root\transition\accounting.rs"
$release = "$root\release.rs"
$snapshotValidation = "$root\release\snapshot_validation.rs"
$tests = "$root\tests.rs"
$binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"
$pipelineRoot = "crates\ep_run\src\pipeline.rs"
$pipeline = "crates\ep_run\src\pipeline\$pipelineStem.rs"
$serialization = "crates\ep_run\src\pipeline\$pipelineStem\serialization\snapshot.rs"
$cp385Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs"
$cp386Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp386_assertions.rs"
$cp387Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp387_assertions.rs"
$cp388Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp388_assertions.rs"
$audit = "scripts\quality\ideal-loads-structure-audit\cp386-cooling-post-saturation-capacity-limit-dehumidification-control-switch.ps1"
$sites = @(
    "read-purchased-air-dehumidification-control-type",
    "dispatch-dehumidification-control-switch"
)

function Assert-Cp386Text {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP386 $Description missing '$Pattern'" }
}

function Get-Cp386BraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP386 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP386 $Description opening brace missing" }
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
    throw "CP386 $Description closing brace missing"
}

$required = @(
    $module, $state, $transition, $accounting, "$root\release.rs", "$root\release\prefix_validation.rs",
    "$root\release\runtime_validation.rs", $snapshotValidation,
    $tests, $adapter,
    "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs",
    "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$stem.rs",
    $coupled, "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($stem)_fixture.rs",
    $pipeline, "crates\ep_run\src\pipeline\$pipelineStem\validation.rs",
    $serialization, $cp385Assertions, $cp386Assertions, $cp387Assertions, $cp388Assertions, $audit
)
foreach ($file in $required) {
    Assert-FileExists -Path $file -Description "CP386 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP386 bounded file"
}
foreach ($directory in @(
        $root,
        "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation",
        "crates\ep_run\src\pipeline\$pipelineStem"
    )) {
    if (Test-Path -LiteralPath $directory -PathType Container) {
        foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
            Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP386 bounded recursive file"
        }
    }
}

if ((Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash -cne $hash) {
    throw "CP386 PurchasedAirManager.cc SHA-256 drift"
}
$lines = Get-Content -Encoding UTF8 -LiteralPath $source
if ($lines[2271].Trim() -cne 'switch (PurchAir.DehumidCtrlType) {' -or
    $lines[2272].Trim() -cne 'case HumControl::ConstantSensibleHeatRatio: {' -or
    $lines[2276].Trim() -cne 'CpAir = PsyCpAirFnW(PurchAir.MixedAirHumRat);') {
    throw "CP386 source, lexical boundary, or executable boundary drift"
}
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2272' -Description "mapped source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2273' -Description "first excluded lexical source"
Assert-Contains -Path $module -Pattern 'PurchasedAirManager\.cc:2277' -Description "first excluded executable source"
Assert-ExactStringArray -Path $module `
    -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE_ORDER' `
    -Expected $sites `
    -Description "exact two-site source order"

$core = (Get-ChildItem -LiteralPath $root -Recurse -File -Filter "*.rs" | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine
foreach ($counter in @(
        'transition_count', 'dehumidification_control_switch_count', 'source_site_execution_count',
        'dehumidification_control_type_read_count', 'dehumidification_control_switch_dispatch_count',
        'dehumidification_control_constant_sensible_heat_ratio_case_selection_count',
        'dehumidification_control_humidistat_case_selection_count',
        'dehumidification_control_none_case_selection_count',
        'dehumidification_control_constant_supply_humidity_ratio_case_selection_count'
    )) {
    Assert-Contains -Path $state -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'dehumidification_control_type_read', 'dehumidification_control_type',
        'dehumidification_control_switch_dispatched', 'resulting_supply_enthalpy_j_per_kg'
    )) {
    Assert-Contains -Path $module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
Assert-PatternsInOrder -Path $accounting `
    -Patterns @(
        'DehumidificationControlType::ConstantSensibleHeatRatio',
        'DehumidificationControlType::Humidistat',
        'DehumidificationControlType::None',
        'DehumidificationControlType::ConstantSupplyHumidityRatio'
    ) `
    -Description "named C++ case-order dispatch"
foreach ($pattern in @(
        'predecessor.*supply_enthalpy_assignment_executed',
        'source_site_execution_count', 'checked_mul|checked_add',
        'eighteen|18', 'twelve|12', 'thirty|30',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent',
        'DehumidificationControlType::None'
    )) {
    Assert-Cp386Text -Text $core -Pattern $pattern -Description "contract $pattern"
}

# Lock the complete route characterization to executable assertions instead of
# accepting documentation/comments that merely mention the expected totals.
foreach ($expectation in @(
        [PSCustomObject]@{ Pattern = 'fn\s+cp386_has_eighteen_inactive_and_twelve_lineage_constrained_active_routes\s*\('; Description = '30-route characterization test' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(snapshots\.len\(\),\s*30\)'; Description = '30 snapshots' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.transition_count,\s*30\)'; Description = '30 transitions' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.inactive_transition_count,\s*18\)'; Description = '18 inactive routes' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.dehumidification_control_switch_count,\s*12\)'; Description = '12 active switch routes' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.source_site_execution_count,\s*24\)'; Description = 'two sites per active route' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.dehumidification_control_type_read_count,\s*12\)'; Description = '12 selector reads' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.dehumidification_control_switch_dispatch_count,\s*12\)'; Description = '12 selector dispatches' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.predecessor_route_counts\.iter\(\)\.sum::<usize>\(\),\s*30\)'; Description = '30 predecessor-route counts' },
        [PSCustomObject]@{ Pattern = '(?s)assert_eq!\(\s*state\.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,\s*3,?\s*\)'; Description = 'three constant-SHR selections' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.dehumidification_control_humidistat_case_selection_count,\s*3\)'; Description = 'three humidistat selections' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.dehumidification_control_none_case_selection_count,\s*3\)'; Description = 'three none selections' },
        [PSCustomObject]@{ Pattern = '(?s)assert_eq!\(\s*state\.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,\s*3,?\s*\)'; Description = 'three constant-supply-HR selections' },
        [PSCustomObject]@{ Pattern = '(?s)cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release\(.*?\)\s*\}\)\s*\.count\(\),\s*11,?\s*\)'; Description = '11 exact-direct routes' }
    )) {
    Assert-Contains -Path $tests -Pattern $expectation.Pattern -Description "CP386 $($expectation.Description)"
}
Assert-Contains `
    -Path "$root\transition\routes.rs" `
    -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_control_flow_shape_is_exact\s*\(' `
    -Description "CP386 shared strict CP385 predecessor control-flow validation"
foreach ($expectation in @(
        [PSCustomObject]@{ Pattern = 'fn\s+all_ten_inherited_control_flags_are_revalidated_across_u_k_x_f_m_routes\s*\('; Description = 'inherited control-flow inversion test' },
        [PSCustomObject]@{ Pattern = 'for\s+flag\s+in\s+0\.\.10\s*\{'; Description = 'all ten inherited flags' },
        [PSCustomObject]@{ Pattern = '(?s)let\s+routes\s*=\s*\[\s*predecessor\(0,\s*0,\s*false,\s*1\),\s*predecessor\(3,\s*0,\s*false,\s*1\),\s*predecessor\(3,\s*2,\s*false,\s*1\),\s*predecessor\(3,\s*1,\s*false,\s*1\),\s*predecessor\(3,\s*1,\s*true,\s*1\),\s*\]'; Description = 'U/K/X/F/M inversion routes' },
        [PSCustomObject]@{ Pattern = 'assert_eq!\(state\.transition_count,\s*0\)'; Description = 'inversion rejection before mutation' }
    )) {
    Assert-Contains -Path $tests -Pattern $expectation.Pattern -Description "CP386 $($expectation.Description)"
}
foreach ($path in @($transition, $release, $adapter, $coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
foreach ($forbidden in @(
        'dehumidification_control_type\s+as\s+(?:usize|isize|u\d+|i\d+)',
        'discriminant\s*\(', '\bPsy(?:H|W|Cp)Fn', '\bmul_add\b',
        'DirectZonePurchasedAirCouplingInput', 'complete_direct_zone_purchased_air_coupling'
    )) {
    Assert-NotContains -Path $transition -Pattern $forbidden -Description "transition forbidden feed/transform"
}
Assert-Contains `
    -Path $snapshotValidation `
    -Pattern 'predecessor\.to_bits\(\)\s*==\s*resulting\.to_bits\(\)' `
    -Description "CP386 production bit-exact CP385 enthalpy preservation"
Assert-Contains `
    -Path $tests `
    -Pattern '(?s)predecessor_resulting_supply_enthalpy_j_per_kg\s*\.map\(f64::to_bits\),\s*snapshot\.resulting_supply_enthalpy_j_per_kg\.map\(f64::to_bits\)' `
    -Description "CP386 route characterization bit-exact enthalpy regression"

$bindingText = Read-RepoText -Path $binding
$cp385Index = $bindingText.IndexOf("let calculation_$predecessorStem =")
$cp386Index = $bindingText.IndexOf("let calculation_$stem =")
$cp387Index = $bindingText.IndexOf("let calculation_$successorStem =")
$cp388Index = $bindingText.IndexOf("let calculation_$terminalStem =")
$cp389Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =")
$cp390Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =")
$cp391Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =")
$cp392Index = $bindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =")
$numericalIndex = $bindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp385Index -lt 0 -or $cp386Index -le $cp385Index -or $cp387Index -le $cp386Index -or $cp388Index -le $cp387Index -or $cp389Index -le $cp388Index -or $cp390Index -le $cp389Index -or $cp391Index -le $cp390Index -or $cp392Index -le $cp391Index -or $numericalIndex -le $cp392Index) {
    throw "Binding must execute CP385, CP386, CP387, CP388, CP389, CP390, CP391, then unchanged numerical coupling"
}
$dto = Get-Cp386BraceBlock `
    -Text $bindingText.Substring($numericalIndex) `
    -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' `
    -Description "numerical DTO"
if ($dto -match 'cp386|cp387|cp388|dehumidification_control_switch|constant_sensible_heat_ratio_(?:cp_air|sensible_output)_assignment') {
    throw "CP386-CP388 evidence unexpectedly feeds the numerical DTO"
}
Assert-NotContains -Path $adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node_update|report' -Description "adapter numerical feed"

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
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp423_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $pipelineRoot -Pattern "purchased_air_calc_$($stem)_lifecycle" -Description "pipeline lifecycle key"
$pipelineText = Read-RepoText -Path $pipelineRoot
$nonDirectValidation = Get-Cp386BraceBlock `
    -Text $pipelineText `
    -AnchorPattern 'fn\s+validate_runtime_demand_provenance\s*\(' `
    -Description "non-direct production firewall"
Assert-Cp386Text `
    -Text $nonDirectValidation `
    -Pattern "(?s)\.$([regex]::Escape("purchased_air_calc_$($stem)_lifecycle"))\s*\.is_some\s*\(\s*\)" `
    -Description "production lifecycle Some rejection"
$nonDirectTest = Get-Cp386BraceBlock `
    -Text $pipelineText `
    -AnchorPattern 'fn\s+non_direct_runtime_rejects_cp316_through_cp423_lifecycle_evidence\s*\(' `
    -Description "cumulative non-direct regression"
$lifecycleField = [regex]::Escape("purchased_air_calc_$($stem)_lifecycle")
Assert-Cp386Text `
    -Text $nonDirectTest `
    -Pattern "(?s)\.$lifecycleField\s*=\s*Some\(\s*ep_runtime::$($typeStem)LifecycleSummary\s*\{.*?validate_runtime_demand_provenance\(.*?Err\(.*?persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime.*?\.$lifecycleField\s*=\s*None;.*?\.$lifecycleField\s*\.is_none\s*\(\s*\)" `
    -Description "concrete Some-to-Err-to-None non-direct regression"
foreach ($mapping in @(
        'DehumidificationControlType::ConstantSensibleHeatRatio\s*=>\s*"ConstantSensibleHeatRatio"',
        'DehumidificationControlType::Humidistat\s*=>\s*"Humidistat"',
        'DehumidificationControlType::None\s*=>\s*"None"',
        'DehumidificationControlType::ConstantSupplyHumidityRatio\s*=>\s*"ConstantSupplyHumidityRatio"'
    )) {
    Assert-Contains -Path $serialization -Pattern $mapping -Description "symbolic selector JSON"
}
Assert-NotContains -Path $serialization -Pattern '(?i)dehumidification_control_type_(?:ordinal|discriminant|ieee_bits)|dehumidification_control_type\s+as\s+(?:usize|isize|u\d+|i\d+)' -Description "selector ordinal JSON"
Assert-Contains -Path $cp385Assertions -Pattern 'mod cp386_assertions;' -Description "arbitrary assertion delegation"
Assert-Contains -Path $cp385Assertions -Pattern 'cp386_assertions::assert_direct\(runtime, results\)' -Description "direct arbitrary delegation"
Assert-Contains -Path $cp385Assertions -Pattern 'cp386_assertions::assert_non_direct\(runtime\)' -Description "non-direct arbitrary delegation"
Assert-Contains -Path $cp386Assertions -Pattern 'mod cp387_assertions;' -Description "CP387 arbitrary module"
Assert-Contains -Path $cp386Assertions -Pattern 'cp387_assertions::assert_direct\(runtime, results\)' -Description "CP387 direct arbitrary delegation"
Assert-Contains -Path $cp386Assertions -Pattern 'cp387_assertions::assert_non_direct\(runtime\)' -Description "CP387 non-direct arbitrary delegation"
Assert-Contains -Path $cp387Assertions -Pattern 'mod cp388_assertions;' -Description "CP388 arbitrary module"
Assert-Contains -Path $cp387Assertions -Pattern 'cp388_assertions::assert_direct\(runtime, results\)' -Description "CP388 direct arbitrary delegation"
Assert-Contains -Path $cp387Assertions -Pattern 'cp388_assertions::assert_non_direct\(runtime\)' -Description "CP388 non-direct arbitrary delegation"
Assert-Contains -Path $cp388Assertions -Pattern 'CP388 lifecycle must remain outside numerical result state' -Description "CP388 terminal numerical nonfeed"
Assert-Contains -Path $cp386Assertions -Pattern 'supply_node.*report.*reconciled.*numerical_dto' -Description "node/load/report nonfeed set"

$algorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$capabilityText = Read-RepoText -Path "specs\capabilities.toml"
$algorithmClaims = [regex]::Matches($algorithmText, '(?m)^\s*"CP386 supersedes only [^"\r\n]+",\s*$')
$capabilityClaims = [regex]::Matches($capabilityText, '(?m)^\s*"CP386 additionally requires[^"\r\n]+",\s*$')
if ($algorithmClaims.Count -ne 2 -or $capabilityClaims.Count -ne 2) {
    throw "CP386 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($algorithmClaims + $capabilityClaims)) {
    foreach ($pattern in @(
            $commit, $hash, '2272', '2273', '2277', 'two', 'thirty',
            'eighteen', 'twelve', 'CP385', 'sole (?:immediate )?predecessor',
            'None', 'symbolic|named variants', 'DirectZonePurchasedAirCouplingInput',
            '324 total', '240 public', '84 internal', 'zero unused',
            'zero unreachable', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP386 spec addendum missing '$pattern'" }
    }
}

$docs = @(
    "docs\src\current\current-status.md",
    "docs\src\current\project-contract.md",
    "docs\src\porting-map\ideal-loads-source-map.md",
    "docs\src\porting-map\heat-balance-source-map.md",
    "docs\src\porting-map\zone-air-update-map.md"
)
foreach ($doc in $docs) {
    $text = Read-RepoText -Path $doc
    $sections = [regex]::Matches($text, '(?ms)^## CP386\b.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP386 documentation expected one section in $doc" }
    foreach ($pattern in @($commit, $hash, '2272', '2273', '2277', 'thirty|30', 'eighteen|18', 'twelve|12', 'CP385', '324\s+total', '84\s+internal')) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP386 documentation in $doc missing '$pattern'" }
    }
    $cp385DocIndex = $text.LastIndexOf("## CP385 ")
    $cp386DocIndex = $text.LastIndexOf("## CP386 ")
    if ($cp385DocIndex -lt 0 -or $cp386DocIndex -le $cp385DocIndex) { throw "CP385-to-CP386 documentation order drift in $doc" }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP386\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP386 supersedes only' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP386 additionally requires' -Description "generated capability addendum"

foreach ($historical in 334..385) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp423_lifecycle_evidence' -Description "historical cumulative firewall"
}
foreach ($historical in 335..385) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 361 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 121 \|')) -Description "historical generated internal"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$stem" -Description "historical CP386 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
foreach ($pattern in @('\$cp385Call\s*=', '\$cp386Call\s*=', '\$cp387Call\s*=', '\$cp388Call\s*=', '\$cp389Call\s*=', 'CP385-to-CP386', 'CP386-to-CP387', 'CP387-to-CP388', 'CP388-to-CP389', 'CP389-to-CP390')) {
    Assert-Contains -Path $cp345Audit -Pattern $pattern -Description "strict CP345 terminal ordering"
}
Assert-NotContains -Path $cp345Audit -Pattern '\$cp385Call\s*=.*?total_output_supply_enthalpy_assignment[^\r\n]+\|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch' -Description "combined CP385/CP386 matcher"

$master = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp385AuditIndex = $master.IndexOf("cp385-cooling-post-saturation-capacity-limit-dehumidification-total-output-supply-enthalpy-assignment.ps1")
$cp386AuditIndex = $master.IndexOf("cp386-cooling-post-saturation-capacity-limit-dehumidification-control-switch.ps1")
$cp387AuditIndex = $master.IndexOf("cp387-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1")
$cp388AuditIndex = $master.IndexOf("cp388-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-sensible-output-assignment.ps1")
$cp389AuditIndex = $master.IndexOf("cp389-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-assignment.ps1")
$cp390AuditIndex = $master.IndexOf("cp390-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-temperature-mixed-air-limit.ps1")
$cp391AuditIndex = $master.IndexOf("cp391-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-overdrying-limit.ps1")
$cp392AuditIndex = $master.IndexOf("cp392-cooling-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-supply-humidity-ratio-assignment.ps1")
$completionIndex = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp385AuditIndex -lt 0 -or $cp386AuditIndex -le $cp385AuditIndex -or $cp387AuditIndex -le $cp386AuditIndex -or $cp388AuditIndex -le $cp387AuditIndex -or $cp389AuditIndex -le $cp388AuditIndex -or $cp390AuditIndex -le $cp389AuditIndex -or $cp391AuditIndex -le $cp390AuditIndex -or $cp392AuditIndex -le $cp391AuditIndex -or $completionIndex -le $cp392AuditIndex) {
    throw "Master audit must dot-source CP386 through CP391 after CP385 before completion"
}
$inventory = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 361', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp386Text -Text $inventory -Pattern $pattern -Description "inventory"
}
if ([regex]::Matches($inventory, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($inventory, '(?m)^classification = "internal"$').Count -ne 121) {
throw "CP386 inventory must be exactly 240 public and 121 internal scripts"
}
Assert-Cp386Text -Text $inventory -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp386-cooling-post-saturation-capacity-limit-dehumidification-control-switch\.ps1"' -Description "inventory record"
foreach ($pattern in @(
        '\| 361 \|', '\| public scripts \| 240 \|',
        '\| 121 \|', '\| scripts without callers \| 0 \|'
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-numerical' -Description 'CP423 terminal-to-numerical interval'
Write-Host "CP386 post-saturation dehumidification-control switch structure audit passed."
}
