# CP383 maps only PurchasedAirManager.cc executable line 2268's strict
# total-output capacity guard and stops before line 2269's cap assignment.
& {
$cp383Stem = "cooling_post_saturation_capacity_limit_dehumidification_total_output_guard"
$cp382StemForCp383 = "cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment"
$cp383PipelineStem = "purchased_air_$cp383Stem"
$cp383Lifecycle = "purchased_air_calc_$($cp383Stem)_lifecycle"
$cp383SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp383SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp383Sites = @(
    "read-retained-cooling-total-output-for-post-saturation-dehumidification-maximum-capacity-comparison",
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-dehumidification-total-output-comparison",
    "compare-post-saturation-dehumidification-cooling-total-output-strictly-greater-than-maximum-total-cooling-capacity",
    "enter-post-saturation-dehumidification-total-output-capacity-adjustment-body-if-comparison-satisfied"
)
$cp383Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp383Module = "crates\ep_runtime\src\ideal_loads\calc\$cp383Stem.rs"
$cp383Root = "crates\ep_runtime\src\ideal_loads\calc\$cp383Stem"
$cp383State = "$cp383Root\state.rs"
$cp383Transition = "$cp383Root\transition.rs"
$cp383Release = "$cp383Root\release.rs"
$cp383CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp383Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp383Adapter = "crates\ep_runtime\src\ideal_loads\binding\$cp383Stem.rs"
$cp383BindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($cp383Stem)_tests.rs"
$cp383ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp383InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp383InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp383WitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp383Witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp383Stem.rs"
$cp383CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp383Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp383Stem)_validation.rs"
$cp383CoupledLifecycle = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp383Stem)_validation\lifecycle.rs"
$cp383CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp383Stem)_validation\snapshot.rs"
$cp383FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp383Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($cp383Stem)_fixture.rs"
$cp383PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp383Pipeline = "crates\ep_run\src\pipeline\$cp383PipelineStem.rs"
$cp383PipelineValidation = "crates\ep_run\src\pipeline\$cp383PipelineStem\validation.rs"
$cp383PipelineCounts = "crates\ep_run\src\pipeline\$cp383PipelineStem\validation\counts.rs"
$cp383PipelineSnapshotValidation = "crates\ep_run\src\pipeline\$cp383PipelineStem\validation\snapshot.rs"
$cp383Serialization = "crates\ep_run\src\pipeline\$cp383PipelineStem\serialization.rs"
$cp383SnapshotSerialization = "crates\ep_run\src\pipeline\$cp383PipelineStem\serialization\snapshot.rs"
$cp382Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs"
$cp383Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs"
$cp383Audit = "scripts\quality\ideal-loads-structure-audit\cp383-cooling-post-saturation-capacity-limit-dehumidification-total-output-guard.ps1"

function Assert-Cp383TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP383 $Description missing" }
}

function Assert-Cp383TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP383 $Description unexpectedly present" }
}

function Get-Cp383RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP383 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP383 $Description opening brace missing" }
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
    throw "CP383 $Description closing brace missing"
}

$cp383Required = @(
    $cp383Module, $cp383State, $cp383Transition, $cp383Release, $cp383Adapter,
    $cp383BindingTests, $cp383Witness, $cp383Coupled, $cp383CoupledLifecycle,
    $cp383CoupledSnapshot, $cp383Fixture, $cp383Pipeline, $cp383PipelineValidation,
    $cp383PipelineCounts, $cp383PipelineSnapshotValidation, $cp383Serialization,
    $cp383SnapshotSerialization, $cp382Assertions, $cp383Assertions, $cp383Audit
)
foreach ($file in $cp383Required) {
    Assert-FileExists -Path $file -Description "CP383 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP383 bounded file"
}
$cp383CoreFiles = @(Get-ChildItem -LiteralPath $cp383Root -Recurse -File -Filter "*.rs")
$cp383CoreText = ($cp383CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP383 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine

# Exact line-2268 boundary and four read/comparison/body-entry sites.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp383Source).Hash -cne $cp383SourceHash) {
    throw "CP383 PurchasedAirManager.cc SHA-256 drift"
}
$cp383Lines = Get-Content -Encoding UTF8 -LiteralPath $cp383Source
if ($cp383Lines[2267].Trim() -cne 'if ((CoolTotOutput) > PurchAir.MaxCoolTotCap) {' -or
    $cp383Lines[2268].Trim() -cne 'CoolTotOutput = PurchAir.MaxCoolTotCap;') {
    throw "CP383 line 2268 through first-excluded executable 2269 source boundary drift"
}
Assert-Contains -Path $cp383Module -Pattern 'PurchasedAirManager\.cc:2268' -Description "mapped source"
Assert-Contains -Path $cp383Module -Pattern 'PurchasedAirManager\.cc:2269' -Description "first excluded source"
Assert-ExactStringArray -Path $cp383Module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER' -Expected $cp383Sites -Description "four-site source order"

# Twenty-three retained routes: thirteen skips and two outcomes for five assignments.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered',
        'HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
        'HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough',
        'HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered',
        'DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough',
        'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough',
        'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityAdjustmentBodyEntered',
        'DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
        'DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough',
        'DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityAdjustmentBodyEntered'
    )) {
    Assert-Contains -Path $cp383State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'dehumidification_total_output_capacity_guard_evaluation_count',
        'source_site_execution_count',
        'cp382_cooling_total_output_owned_read_count',
        'cooling_total_output_read_count',
        'cp321_maximum_total_cooling_capacity_owned_read_count',
        'cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_total_output_maximum_total_cooling_capacity_comparison_count',
        'cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count',
        'dehumidification_total_output_capacity_adjustment_body_entry_count',
        'dehumidification_total_output_capacity_guard_false_fallthrough_count'
    )) {
    Assert-Contains -Path $cp383State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_dehumidification_total_output_assignment_executed',
        'dehumidification_total_output_capacity_guard_evaluated',
        'cp382_cooling_total_output_owned_read',
        'cooling_total_output_read', 'cooling_total_output_w',
        'cp321_maximum_total_cooling_capacity_owned_read',
        'cp340_same_call_maximum_total_cooling_capacity_bit_corroborated',
        'maximum_total_cooling_capacity_read', 'maximum_total_cooling_capacity_w',
        'cooling_total_output_maximum_total_cooling_capacity_comparison_evaluated',
        'cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity',
        'dehumidification_total_output_capacity_adjustment_body_entered',
        'dehumidification_total_output_capacity_guard_false_fallthrough'
    )) {
    Assert-Contains -Path $cp383Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}

# Raw IEEE comparison, exact 3*A+E accounting, and owner/provenance enforcement.
foreach ($pattern in @(
        'input\.cooling_total_output_w > input\.maximum_total_cooling_capacity_w',
        'source_site_execution_count \+= 3 \+ usize::from\(body\)',
        'predecessor_route_is_active',
        'cp382_cooling_total_output_owned_read',
        'cp321_maximum_total_cooling_capacity_owned_read',
        'cp340_same_call_maximum_total_cooling_capacity_bit_corroborated',
        'maximum_total_cooling_capacity_w\.is_finite\(\)',
        'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent'
    )) {
    Assert-Cp383TextContains -Text $cp383CoreText -Pattern $pattern -Description "core comparison/provenance $pattern"
}
foreach ($pattern in @(
        'predecessor_cp382',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent'
    )) {
    Assert-Cp383TextContains -Text $cp383CoreText -Pattern $pattern -Description "release predecessor proof $pattern"
}
foreach ($forbidden in @(
        '\bmul_add\b', '\btotal_cmp\b', '\bf64::max\b', '\bf64::min\b',
        '\bepsilon\b', '\bclamp\b', '\bpartial_cmp\b',
        'input\.cooling_total_output_w\s*=',
        'DirectZonePurchasedAirCouplingInput', 'reconcile_'
    )) {
    Assert-NotContains -Path $cp383Transition -Pattern $forbidden -Description "forbidden arithmetic/feed $forbidden"
}
foreach ($path in @($cp383Transition, $cp383Release, $cp383Adapter, $cp383Coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $cp383Adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node_update|report' -Description "adapter numerical reconciliation"

# CP382 -> CP383 -> CP384 -> unchanged numerical placement, with no CP383 DTO field.
$cp383BindingText = Read-RepoText -Path $cp383Binding
$cp382BindingIndexForCp383 = $cp383BindingText.IndexOf("let calculation_$cp382StemForCp383 =")
$cp383BindingIndex = $cp383BindingText.IndexOf("let calculation_$cp383Stem ="); $cp384BindingIndexForCp383 = $cp383BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp383 = $cp383BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp383NumericalIndex = $cp383BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp382BindingIndexForCp383 -lt 0 -or $cp383BindingIndex -le $cp382BindingIndexForCp383 -or
    $cp384BindingIndexForCp383 -le $cp383BindingIndex -or $cp385BindingIndexForCp383 -le $cp384BindingIndexForCp383 -or $cp383NumericalIndex -le $cp385BindingIndexForCp383) {
    throw "Binding must execute CP382, CP383, CP384, then unchanged numerical coupling"
}
$cp383Dto = Get-Cp383RustBraceBlock -Text $cp383BindingText.Substring($cp383NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp383TextNotContains -Text $cp383Dto -Pattern 'cp383|post_saturation_capacity_limit_dehumidification_total_output_guard' -Description "numerical DTO feed"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp383CalcRoot; Pattern = $cp383Stem },
        [PSCustomObject]@{ Path = $cp383ScheduledOutput; Pattern = "pub calculation_$($cp383Stem):" },
        [PSCustomObject]@{ Path = $cp383InitState; Pattern = $cp383Stem },
        [PSCustomObject]@{ Path = $cp383InitUnit; Pattern = $cp383Stem },
        [PSCustomObject]@{ Path = $cp383WitnessRoot; Pattern = $cp383Stem },
        [PSCustomObject]@{ Path = $cp383CoupledRoot; Pattern = "mod $($cp383Stem)_validation;" },
        [PSCustomObject]@{ Path = $cp383FixtureRoot; Pattern = $cp383Stem },
        [PSCustomObject]@{ Path = $cp383PipelineRoot; Pattern = $cp383PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}

# Direct-only lifecycle, IEEE JSON sidecars, and terminal nonfeed firewall.
Assert-Contains -Path $cp383PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp420_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp383PipelineRoot -Pattern $cp383Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @(
        'dehumidification_total_output_capacity_guard_evaluation_count',
        'source_site_execution_count',
        'cp382_cooling_total_output_owned_read_count',
        'cooling_total_output_read_count',
        'cp321_maximum_total_cooling_capacity_owned_read_count',
        'cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_total_output_maximum_total_cooling_capacity_comparison_count',
        'cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity_count',
        'dehumidification_total_output_capacity_adjustment_body_entry_count',
        'dehumidification_total_output_capacity_guard_false_fallthrough_count'
    )) {
    Assert-Contains -Path $cp383PipelineCounts -Pattern $pattern -Description "serialized checked count $pattern"
}
foreach ($pattern in @(
        'json_number', 'ieee_bits', 'cooling_total_output_w',
        'maximum_total_cooling_capacity_w',
        'cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity',
        'dehumidification_total_output_capacity_adjustment_body_entered',
        'dehumidification_total_output_capacity_guard_false_fallthrough'
    )) {
    Assert-Contains -Path $cp383SnapshotSerialization -Pattern $pattern -Description "finite JSON/IEEE assignment evidence $pattern"
}
Assert-Contains -Path $cp382Assertions -Pattern 'mod cp383_assertions;' -Description "arbitrary CP383 module"
Assert-Contains -Path $cp382Assertions -Pattern 'cp383_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP383 direct delegation"
Assert-Contains -Path $cp382Assertions -Pattern 'cp383_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP383 non-direct delegation"
Assert-Contains -Path $cp382Assertions -Pattern 'fn assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 retained local-output numerical nonfeed"
Assert-Contains -Path $cp383Assertions -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 retained guard numerical nonfeed"
Assert-Contains -Path $cp383Assertions -Pattern 'mod cp384_assertions;' -Description "arbitrary CP384 module"
Assert-Contains -Path $cp383Assertions -Pattern 'cp384_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP384 direct delegation"
Assert-Contains -Path $cp383Assertions -Pattern 'cp384_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP384 non-direct delegation"
Assert-NotContains -Path $cp383Assertions -Pattern 'CoolTotOutput\s*=|(?:latest|cp383|results)\["(?:supply_node|report)' -Description "CP383 excluded cap/node/report assertion"

# Exactly two algorithm/capability addenda and five ordered handwritten sections.
$cp383AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp383CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp383AlgorithmAddenda = [regex]::Matches($cp383AlgorithmText, '(?m)^\s*"CP383 supersedes only [^"\r\n]+",\s*$')
$cp383CapabilityAddenda = [regex]::Matches($cp383CapabilityText, '(?m)^\s*"CP383 additionally requires[^"\r\n]+",\s*$')
if ($cp383AlgorithmAddenda.Count -ne 2 -or $cp383CapabilityAddenda.Count -ne 2) {
    throw "CP383 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp383AlgorithmAddenda + $cp383CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp383SourceCommit, $cp383SourceHash, '2268', '2269', 'CP384',
            $cp383Sites[0], $cp383Sites[1], $cp383Sites[2],
            $cp383Sites[3], 'twenty-three', 'thirteen', '3\*A\+E', 'CP382',
            'sole (?:immediate )?predecessor', 'CP321', 'CP340',
            'IEEE binary64|Raw IEEE', 'NaN', 'infinity', 'sidecar',
            'DirectZonePurchasedAirCouplingInput', '321 total', '240 public',
            '81 internal', 'zero unused', 'zero unreachable',
            '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP383 spec addendum missing '$pattern'" }
    }
}
$cp383Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP383 Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Guard' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP383 Source-Ordered Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Guard' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP383 Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Guard' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP383 Post-Saturation Capacity-Limit Dehumidification Total-Output Guard in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP383 Post-Saturation Capacity-Limit Dehumidification Total-Output Guard Placement' }
)
foreach ($doc in $cp383Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP383 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..383) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP383 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @(
            $cp383SourceCommit, $cp383SourceHash, '2268', '2269', 'CP384',
            $cp383Sites[0], $cp383Sites[1], $cp383Sites[2],
            $cp383Sites[3], 'twenty-three', '3\*A\+E',
            'CP382', 'CP321', 'CP340', '321\s+total', '81\s+internal'
        )) {
        if ($sections[0].Value -notmatch $required) {
            throw "CP383 documentation in $($doc.Path) missing '$required'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP383\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP383 supersedes only' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP383 additionally requires' -Description "generated capability addendum"

# Current-state propagation while the CP382 320/80 checkpoint stays historical.
foreach ($historical in 334..382) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp420_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..382) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 358 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 118 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..382) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 358' -Description "historical inventory total"
}
foreach ($historical in 367..382) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 118' -Description "historical internal classification count"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 118 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp383Stem" -Description "historical CP383 compact binding order"
}
foreach ($historical in 360..382) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp383BindingIndex' -Description "historical CP383 binding index"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp383Stem" -Description "historical CP383 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern 'CP382-to-CP383' -Description "CP345 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP388-to-CP389' -Description "CP345 CP389 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP389-to-CP390' -Description "CP345 CP390 predecessor interval"
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1" -Limit 600 -Description "CP347 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp349-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1" -Limit 500 -Description "CP349 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1" -Limit 500 -Description "CP362 historical audit"
foreach ($historical in 364, 373, 374, 376, 377, 378, 379, 380, 381, 382) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp383_assertions\.rs' -Description "historical CP383 arbitrary terminal"
}
$cp382HistoricalAlgorithm = [regex]::Matches($cp383AlgorithmText, '(?m)^\s*"CP382 supersedes only [^"\r\n]+",\s*$')
$cp382HistoricalCapability = [regex]::Matches($cp383CapabilityText, '(?m)^\s*"CP382 additionally requires[^"\r\n]+",\s*$')
if ($cp382HistoricalAlgorithm.Count -ne 2 -or $cp382HistoricalCapability.Count -ne 2) {
    throw "CP382 historical addenda count drift"
}
foreach ($claim in @($cp382HistoricalAlgorithm + $cp382HistoricalCapability)) {
    if ($claim.Value -notmatch '320 total' -or $claim.Value -notmatch '80 internal') {
        throw "CP382 historical addendum inventory numbers must remain 320/80"
    }
}

# Master order and current generated inventory.
$cp383MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp382AuditIndexForCp383 = $cp383MainAuditText.IndexOf("cp382-cooling-post-saturation-capacity-limit-dehumidification-total-output-assignment.ps1")
$cp383AuditIndex = $cp383MainAuditText.IndexOf("cp383-cooling-post-saturation-capacity-limit-dehumidification-total-output-guard.ps1")
$cp383CompletionIndex = $cp383MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp382AuditIndexForCp383 -lt 0 -or $cp383AuditIndex -le $cp382AuditIndexForCp383 -or
    $cp383CompletionIndex -le $cp383AuditIndex) {
    throw "Master audit must dot-source CP383 after CP382 before completion"
}
$cp383InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @(
        'script_count = 358', 'dev_command_count = 238',
        'unused_script_count = 0', 'unreachable_count = 0'
    )) {
    Assert-Cp383TextContains -Text $cp383InventoryText -Pattern $pattern -Description "inventory $pattern"
}
if ([regex]::Matches($cp383InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($cp383InventoryText, '(?m)^classification = "internal"$').Count -ne 118) {
    throw "CP383 inventory must be exactly 240 public and 118 internal scripts"
}
Assert-Cp383TextContains -Text $cp383InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp383-cooling-post-saturation-capacity-limit-dehumidification-total-output-guard\.ps1"' -Description "inventory record"
Assert-Cp383TextContains -Text $cp383InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
foreach ($pattern in @(
        '\| 358 \|',
        '\| public scripts \| 240 \|',
        '\| 118 \|',
        '\| scripts without callers \| 0 \|'
    )) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated script inventory $pattern"
}

Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed firewall"
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp417Call' -Description 'CP417 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP416-to-CP417' -Description 'CP416-to-CP417 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-numerical' -Description 'CP420 terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-numerical' -Description 'CP420 terminal interval'
Write-Host "CP383 post-saturation dehumidification total-output guard structure audit passed."
}
