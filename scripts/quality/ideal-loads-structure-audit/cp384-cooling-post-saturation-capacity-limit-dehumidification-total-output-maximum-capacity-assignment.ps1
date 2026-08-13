# CP384 maps only PurchasedAirManager.cc executable line 2269's retained
# maximum-capacity assignment and stops before line 2270's enthalpy arithmetic.
& {
$cp384Stem = "cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment"
$cp383StemForCp384 = "cooling_post_saturation_capacity_limit_dehumidification_total_output_guard"
$cp384PipelineStem = "purchased_air_$cp384Stem"
$cp384Lifecycle = "purchased_air_calc_$($cp384Stem)_lifecycle"
$cp384SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp384SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp384Sites = @(
    "read-retained-maximum-total-cooling-capacity-for-post-saturation-dehumidification-total-output-assignment",
    "assign-local-cooling-total-output-from-maximum-total-cooling-capacity"
)
$cp384Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp384Module = "crates\ep_runtime\src\ideal_loads\calc\$cp384Stem.rs"
$cp384Root = "crates\ep_runtime\src\ideal_loads\calc\$cp384Stem"
$cp384State = "$cp384Root\state.rs"
$cp384Transition = "$cp384Root\transition.rs"
$cp384Accounting = "$cp384Root\transition\accounting.rs"
$cp384Release = "$cp384Root\release.rs"
$cp384PrefixValidation = "$cp384Root\release\prefix_validation.rs"
$cp384CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp384Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp384Adapter = "crates\ep_runtime\src\ideal_loads\binding\$cp384Stem.rs"
$cp384BindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($cp384Stem)_tests.rs"
$cp384ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp384InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp384InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp384WitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp384Witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp384Stem.rs"
$cp384CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp384Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp384Stem)_validation.rs"
$cp384CoupledDir = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp384Stem)_validation"
$cp384FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp384Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($cp384Stem)_fixture.rs"
$cp384PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp384Pipeline = "crates\ep_run\src\pipeline\$cp384PipelineStem.rs"
$cp384PipelineValidation = "crates\ep_run\src\pipeline\$cp384PipelineStem\validation.rs"
$cp384PipelineCounts = "crates\ep_run\src\pipeline\$cp384PipelineStem\validation\counts.rs"
$cp384PipelineSnapshotValidation = "crates\ep_run\src\pipeline\$cp384PipelineStem\validation\snapshot.rs"
$cp384Serialization = "crates\ep_run\src\pipeline\$cp384PipelineStem\serialization.rs"
$cp384SnapshotSerialization = "crates\ep_run\src\pipeline\$cp384PipelineStem\serialization\snapshot.rs"
$cp383Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs"
$cp384Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp384_assertions.rs"
$cp385Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs"
$cp386Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp386_assertions.rs"
$cp384Audit = "scripts\quality\ideal-loads-structure-audit\cp384-cooling-post-saturation-capacity-limit-dehumidification-total-output-maximum-capacity-assignment.ps1"

function Assert-Cp384TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP384 $Description missing" }
}

function Assert-Cp384TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP384 $Description unexpectedly present" }
}

function Get-Cp384RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP384 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP384 $Description opening brace missing" }
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
    throw "CP384 $Description closing brace missing"
}

$cp384Required = @(
    $cp384Module, $cp384State, $cp384Transition, $cp384Accounting, $cp384Release,
    $cp384PrefixValidation, $cp384Adapter, $cp384BindingTests, $cp384Witness,
    $cp384Coupled, $cp384Fixture, $cp384Pipeline, $cp384PipelineValidation,
    $cp384PipelineCounts, $cp384PipelineSnapshotValidation, $cp384Serialization,
    $cp384SnapshotSerialization, $cp383Assertions, $cp384Assertions, $cp385Assertions,
    $cp386Assertions, $cp384Audit
)
foreach ($file in $cp384Required) {
    Assert-FileExists -Path $file -Description "CP384 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP384 bounded file"
}
foreach ($directory in @($cp384Root, $cp384CoupledDir, "crates\ep_run\src\pipeline\$cp384PipelineStem")) {
    foreach ($file in Get-ChildItem -LiteralPath $directory -Recurse -File -Filter "*.rs") {
        Assert-LineLimit -Path $file.FullName -Limit 500 -Description "CP384 bounded recursive file"
    }
}
$cp384CoreText = (Get-ChildItem -LiteralPath $cp384Root -Recurse -File -Filter "*.rs" | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine

# Exact line-2269 boundary and two dependency-ordered assignment sites.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp384Source).Hash -cne $cp384SourceHash) {
    throw "CP384 PurchasedAirManager.cc SHA-256 drift"
}
$cp384Lines = Get-Content -Encoding UTF8 -LiteralPath $cp384Source
if ($cp384Lines[2268].Trim() -cne 'CoolTotOutput = PurchAir.MaxCoolTotCap;' -or
    $cp384Lines[2269].Trim() -cne 'SupplyEnthalpy = MixedAirEnthalpy - CoolTotOutput / SupplyMassFlowRate;') {
    throw "CP384 line 2269 through first-excluded executable 2270 source boundary drift"
}
Assert-Contains -Path $cp384Module -Pattern 'PurchasedAirManager\.cc:2269' -Description "mapped source"
Assert-Contains -Path $cp384Module -Pattern 'PurchasedAirManager\.cc:2270' -Description "first excluded source"
Assert-ExactStringArray -Path $cp384Module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER' -Expected $cp384Sites -Description "two-site source order"

# Twenty-three retained routes: thirteen inherited skips, five guard-false
# preservations, and five maximum-capacity assignments.
$cp384Routes = @(
    'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
    'HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
    'HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
    'HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough',
    'HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned',
    'HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
    'HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
    'HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough',
    'HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned',
    'DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough',
    'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough',
    'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough',
    'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned',
    'DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough',
    'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough',
    'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough',
    'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned',
    'DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
    'DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
    'DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough',
    'DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned'
)
foreach ($route in $cp384Routes) {
    Assert-Contains -Path $cp384State -Pattern $route -Description "retained route $route"
}
$cp384StateText = Read-RepoText -Path $cp384State
$cp384RouteBlock = Get-Cp384RustBraceBlock -Text $cp384StateText -AnchorPattern 'enum PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRetainedRoute' -Description "route enum"
if ([regex]::Matches($cp384RouteBlock, '(?m)^\s{4}[A-Z][A-Za-z0-9]+,\s*$').Count -ne 23) {
    throw "CP384 retained route enum must contain exactly twenty-three variants"
}
foreach ($counter in @(
        'dehumidification_total_output_capacity_guard_evaluation_count',
        'dehumidification_total_output_capacity_guard_false_fallthrough_count',
        'dehumidification_total_output_maximum_capacity_assignment_count',
        'source_site_execution_count',
        'cp383_retained_maximum_total_cooling_capacity_owned_read_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_total_output_assignment_write_count'
    )) {
    Assert-Contains -Path $cp384State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_dehumidification_total_output_capacity_guard_evaluated',
        'predecessor_dehumidification_total_output_capacity_adjustment_body_entered',
        'predecessor_dehumidification_total_output_capacity_guard_false_fallthrough',
        'dehumidification_total_output_capacity_guard_false_fallthrough',
        'dehumidification_total_output_maximum_capacity_assignment_executed',
        'preexisting_cooling_total_output_w',
        'cp383_retained_maximum_total_cooling_capacity_owned_read',
        'maximum_total_cooling_capacity_read', 'maximum_total_cooling_capacity_w',
        'cooling_total_output_assigned', 'assigned_cooling_total_output_w',
        'resulting_cooling_total_output_w'
    )) {
    Assert-Contains -Path $cp384Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}

# CP383 is the only direct predecessor and retained operand source. Assignment
# is a raw bit copy with exact M/M/M and 2*M accounting.
foreach ($pattern in @(
        'predecessor_cp383',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_is_consistent',
        'direct_predecessor_is_retained_and_complete',
        'retained_operand_is_admissible',
        'cp383_snapshots_match_bit_exact',
        'assigned:\s*Some\(maximum\)',
        'resulting:\s*Some\(maximum\)',
        'resulting:\s*Some\(preexisting\)',
        'source_site_execution_count \+= 2',
        'cp383_retained_maximum_total_cooling_capacity_owned_read_count \+= 1',
        'maximum_total_cooling_capacity_read_count \+= 1',
        'cooling_total_output_assignment_write_count \+= 1'
    )) {
    Assert-Cp384TextContains -Text $cp384CoreText -Pattern $pattern -Description "core predecessor/assignment $pattern"
}
foreach ($forbidden in @(
        '\bmul_add\b', '\btotal_cmp\b', '\bf64::max\b', '\bf64::min\b',
        '\bepsilon\b', '\bclamp\b', '\bpartial_cmp\b',
        'DirectZonePurchasedAirCouplingInput', 'reconcile_',
        'MixedAirEnthalpy', 'SupplyMassFlowRate', 'SupplyEnthalpy\s*='
    )) {
    Assert-NotContains -Path $cp384Transition -Pattern $forbidden -Description "forbidden arithmetic/feed $forbidden"
}
foreach ($path in @($cp384Transition, $cp384Release, $cp384Adapter, $cp384Coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $cp384Adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node_update|report' -Description "adapter numerical reconciliation"
Assert-NotContains -Path $cp384Release -Pattern 'calc_cooling_capacity_zero_flow_reset\.|calc_cooling_positive_supply_capacity_limit_sensible_output_guard\.' -Description "direct CP321/CP340 owner reach-through"

# CP383 -> CP384 -> unchanged numerical placement with no CP384 DTO field.
$cp384BindingText = Read-RepoText -Path $cp384Binding
$cp383BindingIndexForCp384 = $cp384BindingText.IndexOf("let calculation_$cp383StemForCp384 =")
$cp384BindingIndex = $cp384BindingText.IndexOf("let calculation_$cp384Stem =")
$cp385BindingIndexForCp384 = $cp384BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp384NumericalIndex = $cp384BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp383BindingIndexForCp384 -lt 0 -or $cp384BindingIndex -le $cp383BindingIndexForCp384 -or
    $cp385BindingIndexForCp384 -le $cp384BindingIndex -or $cp384NumericalIndex -le $cp385BindingIndexForCp384) {
    throw "Binding must execute CP383, CP384, CP385, then unchanged numerical coupling"
}
$cp384Dto = Get-Cp384RustBraceBlock -Text $cp384BindingText.Substring($cp384NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp384TextNotContains -Text $cp384Dto -Pattern 'cp384|maximum_capacity_assignment' -Description "numerical DTO feed"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp384CalcRoot; Pattern = $cp384Stem },
        [PSCustomObject]@{ Path = $cp384ScheduledOutput; Pattern = "pub calculation_$($cp384Stem):" },
        [PSCustomObject]@{ Path = $cp384InitState; Pattern = $cp384Stem },
        [PSCustomObject]@{ Path = $cp384InitUnit; Pattern = $cp384Stem },
        [PSCustomObject]@{ Path = $cp384WitnessRoot; Pattern = $cp384Stem },
        [PSCustomObject]@{ Path = $cp384CoupledRoot; Pattern = "mod $($cp384Stem)_validation;" },
        [PSCustomObject]@{ Path = $cp384FixtureRoot; Pattern = $cp384Stem },
        [PSCustomObject]@{ Path = $cp384PipelineRoot; Pattern = $cp384PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}

# Direct-only pipeline, exact counters, three snapshot shapes, and terminal
# numerical nonfeed assertion delegation.
Assert-Contains -Path $cp384PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp422_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp384PipelineRoot -Pattern $cp384Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @(
        'dehumidification_total_output_capacity_guard_evaluation_count',
        'dehumidification_total_output_capacity_guard_false_fallthrough_count',
        'dehumidification_total_output_maximum_capacity_assignment_count',
        'source_site_execution_count',
        'cp383_retained_maximum_total_cooling_capacity_owned_read_count',
        'maximum_total_cooling_capacity_read_count',
        'cooling_total_output_assignment_write_count',
        'checked_mul\(assignments, 2'
    )) {
    Assert-Contains -Path $cp384PipelineCounts -Pattern $pattern -Description "serialized checked count $pattern"
}
foreach ($pattern in @(
        'json_number', 'ieee_bits', 'preexisting_cooling_total_output_w',
        'maximum_total_cooling_capacity_w', 'assigned_cooling_total_output_w',
        'resulting_cooling_total_output_w',
        'dehumidification_total_output_maximum_capacity_assignment_executed'
    )) {
    Assert-Contains -Path $cp384SnapshotSerialization -Pattern $pattern -Description "finite JSON/IEEE assignment evidence $pattern"
}
Assert-Contains -Path $cp383Assertions -Pattern 'mod cp384_assertions;' -Description "arbitrary CP384 module"
Assert-Contains -Path $cp383Assertions -Pattern 'cp384_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP384 direct delegation"
Assert-Contains -Path $cp383Assertions -Pattern 'cp384_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP384 non-direct delegation"
Assert-Contains -Path $cp384Assertions -Pattern 'mod cp385_assertions;' -Description "arbitrary CP385 module"
Assert-Contains -Path $cp384Assertions -Pattern 'cp385_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP385 direct delegation"
Assert-Contains -Path $cp384Assertions -Pattern 'cp385_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP385 non-direct delegation"
Assert-Contains -Path $cp385Assertions -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed"
Assert-Contains -Path $cp385Assertions -Pattern 'mod cp386_assertions;' -Description "arbitrary CP386 module"
Assert-Contains -Path $cp385Assertions -Pattern 'cp386_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP386 direct delegation"
Assert-Contains -Path $cp385Assertions -Pattern 'cp386_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP386 non-direct delegation"
Assert-NotContains -Path $cp385Assertions -Pattern '(?:latest|cp385|results)\["(?:supply_node|report)' -Description "CP385 excluded node/report assertion"

# Exactly two algorithm/capability addenda and five ordered handwritten sections.
$cp384AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp384CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp384AlgorithmAddenda = [regex]::Matches($cp384AlgorithmText, '(?m)^\s*"CP384 supersedes only [^"\r\n]+",\s*$')
$cp384CapabilityAddenda = [regex]::Matches($cp384CapabilityText, '(?m)^\s*"CP384 additionally requires[^"\r\n]+",\s*$')
if ($cp384AlgorithmAddenda.Count -ne 2 -or $cp384CapabilityAddenda.Count -ne 2) {
    throw "CP384 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp384AlgorithmAddenda + $cp384CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp384SourceCommit, $cp384SourceHash, '2269', '2270', 'CP385',
            $cp384Sites[0], $cp384Sites[1], 'twenty-three', 'thirteen',
            '2\*M', 'CP383', 'sole (?:immediate )?predecessor', 'CP321', 'CP340',
            'binary64|raw binary64', 'sidecar', 'DirectZonePurchasedAirCouplingInput',
            '322 total', '240 public', '82 internal', 'zero unused',
            'zero unreachable', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP384 spec addendum missing '$pattern'" }
    }
}
$cp384Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP384 Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Maximum-Capacity Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP384 Source-Ordered Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Maximum-Capacity Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP384 Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Maximum-Capacity Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP384 Post-Saturation Capacity-Limit Dehumidification Total-Output Maximum-Capacity Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP384 Post-Saturation Capacity-Limit Dehumidification Total-Output Maximum-Capacity Assignment Placement' }
)
foreach ($doc in $cp384Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP384 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..384) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP384 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @(
            $cp384SourceCommit, $cp384SourceHash, '2269', '2270', 'CP385',
            $cp384Sites[0], $cp384Sites[1], 'twenty-three', '2\*M',
            'CP383', '322\s+total', '82\s+internal'
        )) {
        if ($sections[0].Value -notmatch $required) {
            throw "CP384 documentation in $($doc.Path) missing '$required'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP384\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP384 supersedes only' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP384 additionally requires' -Description "generated capability addendum"

# Current-state propagation while CP383's 321/81 checkpoint remains historical.
foreach ($historical in 334..383) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp422_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..383) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 360 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 120 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..383) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 360' -Description "historical inventory total"
}
foreach ($historical in 367..383) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 120' -Description "historical internal classification count"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 120 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp384Stem" -Description "historical CP384 compact binding order"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp384Stem" -Description "historical CP384 helper whitelist"
}
foreach ($historical in 360..383) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp384BindingIndex' -Description "historical CP384 binding index"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern 'CP383-to-CP384' -Description "CP345 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP384-to-CP385' -Description "CP345 CP385 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP388-to-CP389' -Description "CP345 CP389 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP389-to-CP390' -Description "CP345 CP390 predecessor interval"
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 historical audit"
foreach ($historical in 364, 373, 374, 376, 377, 378, 379, 380, 381, 382, 383) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp385_assertions\.rs' -Description "historical CP385 arbitrary terminal"
}
$cp383HistoricalAlgorithm = [regex]::Matches($cp384AlgorithmText, '(?m)^\s*"CP383 supersedes only [^"\r\n]+",\s*$')
$cp383HistoricalCapability = [regex]::Matches($cp384CapabilityText, '(?m)^\s*"CP383 additionally requires[^"\r\n]+",\s*$')
if ($cp383HistoricalAlgorithm.Count -ne 2 -or $cp383HistoricalCapability.Count -ne 2) {
    throw "CP383 historical addenda count drift"
}
foreach ($claim in @($cp383HistoricalAlgorithm + $cp383HistoricalCapability)) {
    if ($claim.Value -notmatch '321 total' -or $claim.Value -notmatch '81 internal') {
        throw "CP383 historical addendum inventory numbers must remain 321/81"
    }
}

# Master order and current generated inventory.
$cp384MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp383AuditIndexForCp384 = $cp384MainAuditText.IndexOf("cp383-cooling-post-saturation-capacity-limit-dehumidification-total-output-guard.ps1")
$cp384AuditIndex = $cp384MainAuditText.IndexOf("cp384-cooling-post-saturation-capacity-limit-dehumidification-total-output-maximum-capacity-assignment.ps1")
$cp384CompletionIndex = $cp384MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp383AuditIndexForCp384 -lt 0 -or $cp384AuditIndex -le $cp383AuditIndexForCp384 -or
    $cp384CompletionIndex -le $cp384AuditIndex) {
    throw "Master audit must dot-source CP384 after CP383 before completion"
}
$cp384InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @(
        'script_count = 360', 'dev_command_count = 238',
        'unused_script_count = 0', 'unreachable_count = 0'
    )) {
    Assert-Cp384TextContains -Text $cp384InventoryText -Pattern $pattern -Description "inventory $pattern"
}
if ([regex]::Matches($cp384InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($cp384InventoryText, '(?m)^classification = "internal"$').Count -ne 120) {
    throw "CP384 inventory must be exactly 240 public and 120 internal scripts"
}
Assert-Cp384TextContains -Text $cp384InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp384-cooling-post-saturation-capacity-limit-dehumidification-total-output-maximum-capacity-assignment\.ps1"' -Description "inventory record"
Assert-Cp384TextContains -Text $cp384InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
foreach ($pattern in @(
        '\| 360 \|',
        '\| public scripts \| 240 \|',
        '\| 120 \|',
        '\| scripts without callers \| 0 \|'
    )) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated script inventory $pattern"
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-numerical' -Description 'CP422 terminal interval'
Write-Host "CP384 post-saturation dehumidification total-output maximum-capacity assignment structure audit passed."
}
