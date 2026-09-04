# CP382 maps only PurchasedAirManager.cc executable line 2267's grouped local
# cooling-total-output assignment and stops before line 2268's capacity guard.
& {
$cp382Stem = "cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment"
$cp381StemForCp382 = "cooling_post_saturation_capacity_limit_dehumidification_guard"
$cp382PipelineStem = "purchased_air_$cp382Stem"
$cp382Lifecycle = "purchased_air_calc_$($cp382Stem)_lifecycle"
$cp382SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp382SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp382Sites = @(
    "read-retained-supply-mass-flow-rate-for-post-saturation-dehumidification-total-output-product",
    "read-retained-mixed-air-enthalpy-for-post-saturation-dehumidification-total-output-difference",
    "read-retained-supply-enthalpy-for-post-saturation-dehumidification-total-output-difference",
    "calculate-mixed-air-enthalpy-minus-supply-enthalpy-for-post-saturation-dehumidification-total-output",
    "calculate-supply-mass-flow-rate-times-enthalpy-difference-for-post-saturation-dehumidification-total-output",
    "assign-local-cooling-total-output-for-post-saturation-dehumidification"
)
$cp382Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp382Module = "crates\ep_runtime\src\ideal_loads\calc\$cp382Stem.rs"
$cp382Root = "crates\ep_runtime\src\ideal_loads\calc\$cp382Stem"
$cp382State = "$cp382Root\state.rs"
$cp382Transition = "$cp382Root\transition.rs"
$cp382Release = "$cp382Root\release.rs"
$cp382PrefixValidation = "$cp382Root\release\prefix_validation.rs"
$cp382CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp382Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp382Adapter = "crates\ep_runtime\src\ideal_loads\binding\$cp382Stem.rs"
$cp382BindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($cp382Stem)_tests.rs"
$cp382ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp382InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp382InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp382WitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp382Witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp382Stem.rs"
$cp382CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp382Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp382Stem)_validation.rs"
$cp382CoupledLifecycle = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp382Stem)_validation\lifecycle.rs"
$cp382CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp382Stem)_validation\snapshot.rs"
$cp382FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp382Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($cp382Stem)_fixture.rs"
$cp382PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp382Pipeline = "crates\ep_run\src\pipeline\$cp382PipelineStem.rs"
$cp382PipelineValidation = "crates\ep_run\src\pipeline\$cp382PipelineStem\validation.rs"
$cp382PipelineCounts = "crates\ep_run\src\pipeline\$cp382PipelineStem\validation\counts.rs"
$cp382PipelineSnapshotValidation = "crates\ep_run\src\pipeline\$cp382PipelineStem\validation\snapshot.rs"
$cp382Serialization = "crates\ep_run\src\pipeline\$cp382PipelineStem\serialization.rs"
$cp382SnapshotSerialization = "crates\ep_run\src\pipeline\$cp382PipelineStem\serialization\snapshot.rs"
$cp381Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp381_assertions.rs"
$cp382Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs"
$cp383Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs"
$cp382Audit = "scripts\quality\ideal-loads-structure-audit\cp382-cooling-post-saturation-capacity-limit-dehumidification-total-output-assignment.ps1"

function Assert-Cp382TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP382 $Description missing" }
}

function Assert-Cp382TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP382 $Description unexpectedly present" }
}

function Get-Cp382RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP382 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP382 $Description opening brace missing" }
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
    throw "CP382 $Description closing brace missing"
}

$cp382Required = @(
    $cp382Module, $cp382State, $cp382Transition, $cp382Release, $cp382PrefixValidation, $cp382Adapter,
    $cp382BindingTests, $cp382Witness, $cp382Coupled, $cp382CoupledLifecycle,
    $cp382CoupledSnapshot, $cp382Fixture, $cp382Pipeline, $cp382PipelineValidation,
    $cp382PipelineCounts, $cp382PipelineSnapshotValidation, $cp382Serialization,
    $cp382SnapshotSerialization, $cp381Assertions, $cp382Assertions,
    $cp383Assertions, $cp382Audit
)
foreach ($file in $cp382Required) {
    Assert-FileExists -Path $file -Description "CP382 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP382 bounded file"
}
$cp382CoreFiles = @(Get-ChildItem -LiteralPath $cp382Root -Recurse -File -Filter "*.rs")
$cp382CoreText = ($cp382CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP382 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine

# Exact line-2267 boundary and six read/arithmetic/assignment sites.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp382Source).Hash -cne $cp382SourceHash) {
    throw "CP382 PurchasedAirManager.cc SHA-256 drift"
}
$cp382Lines = Get-Content -Encoding UTF8 -LiteralPath $cp382Source
if ($cp382Lines[2266].Trim() -cne 'CoolTotOutput = SupplyMassFlowRate * (MixedAirEnthalpy - SupplyEnthalpy);' -or
    $cp382Lines[2267].Trim() -cne 'if ((CoolTotOutput) > PurchAir.MaxCoolTotCap) {') {
    throw "CP382 line 2267 through first-excluded executable 2268 source boundary drift"
}
Assert-Contains -Path $cp382Module -Pattern 'PurchasedAirManager\.cc:2267' -Description "mapped source"
Assert-Contains -Path $cp382Module -Pattern 'PurchasedAirManager\.cc:2268' -Description "first excluded source"
Assert-ExactStringArray -Path $cp382Module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER' -Expected $cp382Sites -Description "six-site source order"

# Eighteen retained routes: thirteen skips and five assignment-completed routes.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputAssigned',
        'HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
        'HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned',
        'HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputAssigned',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough',
        'DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputAssigned',
        'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough',
        'DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputAssigned',
        'DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp382State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'dehumidification_total_output_assignment_count', 'source_site_execution_count',
        'cp330_supply_mass_flow_rate_owned_read_count',
        'cp329_same_call_supply_mass_flow_rate_bit_corroboration_count',
        'cp339_same_call_supply_mass_flow_rate_bit_corroboration_count',
        'supply_mass_flow_rate_read_count',
        'cp329_mixed_air_enthalpy_owned_read_count',
        'cp329_same_call_recirculation_enthalpy_bit_corroboration_count',
        'cp339_same_call_mixed_air_enthalpy_bit_corroboration_count',
        'mixed_air_enthalpy_read_count',
        'cp379_post_saturation_supply_enthalpy_owned_read_count',
        'cp379_same_call_supply_enthalpy_bits_corroboration_count',
        'supply_enthalpy_read_count', 'enthalpy_difference_calculation_count',
        'cooling_total_output_calculation_count',
        'cooling_total_output_assignment_write_count'
    )) {
    Assert-Contains -Path $cp382State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_dehumidification_body_entered',
        'predecessor_dehumidification_guard_false_fallthrough',
        'dehumidification_total_output_assignment_executed',
        'cp330_supply_mass_flow_rate_owned_read',
        'cp329_same_call_supply_mass_flow_rate_bit_corroborated',
        'cp339_same_call_supply_mass_flow_rate_bit_corroborated',
        'supply_mass_flow_rate_read', 'supply_mass_flow_rate_kg_per_s',
        'cp329_mixed_air_enthalpy_owned_read',
        'cp329_same_call_recirculation_enthalpy_bit_corroborated',
        'cp339_same_call_mixed_air_enthalpy_bit_corroborated',
        'mixed_air_enthalpy_read', 'mixed_air_enthalpy_j_per_kg',
        'cp379_post_saturation_supply_enthalpy_owned_read',
        'cp379_same_call_supply_enthalpy_bits_corroborated',
        'supply_enthalpy_read', 'supply_enthalpy_j_per_kg',
        'enthalpy_difference_calculated', 'mixed_air_minus_supply_enthalpy_j_per_kg',
        'cooling_total_output_calculated', 'calculated_cooling_total_output_w',
        'cooling_total_output_assigned', 'cooling_total_output_w'
    )) {
    Assert-Contains -Path $cp382Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}

# Raw source grouping, exact 6*A accounting, and owner/provenance enforcement.
foreach ($pattern in @(
        'input\.mixed_air_enthalpy_j_per_kg - input\.supply_enthalpy_j_per_kg',
        'input\.supply_mass_flow_rate_kg_per_s \* enthalpy_difference_j_per_kg',
        'SOURCE_ORDER\.len\(\)',
        'predecessor_route_is_assignment',
        'cp330_supply_mass_flow_rate_owned_read',
        'cp329_same_call_supply_mass_flow_rate_bit_corroborated',
        'cp339_same_call_supply_mass_flow_rate_bit_corroborated',
        'cp329_mixed_air_enthalpy_owned_read',
        'cp329_same_call_recirculation_enthalpy_bit_corroborated',
        'cp339_same_call_mixed_air_enthalpy_bit_corroborated',
        'cp379_post_saturation_supply_enthalpy_owned_read',
        'cp379_same_call_supply_enthalpy_bits_corroborated',
        'active_counters_mut'
    )) {
    Assert-Cp382TextContains -Text $cp382CoreText -Pattern $pattern -Description "core arithmetic/provenance $pattern"
}
foreach ($pattern in @(
        'predecessor_cp381',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent'
    )) {
    Assert-Cp382TextContains -Text $cp382CoreText -Pattern $pattern -Description "release predecessor proof $pattern"
}
Assert-Contains -Path $cp382PrefixValidation -Pattern 'cooling_post_saturation_capacity_limit_dehumidification_guard_committed_latest_snapshot_is_consistent\s*\(' -Description "bounded CP381 committed predecessor proof"
Assert-NotContains -Path $cp382PrefixValidation -Pattern 'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_is_consistent\s*\(' -Description "recursive CP381 predecessor completion"
foreach ($pattern in @(
        'cooling_mixed_air_call_committed_latest_sensible_output_inputs\s*\(',
        'cooling_mixed_air_call_committed_latest_mixed_air_enthalpy\s*\(',
        'cooling_supply_mass_flow_positive_guard_committed_latest_snapshot_is_consistent\s*\(',
        'cooling_positive_supply_capacity_limit_sensible_output_assignment_committed_latest_snapshot_is_consistent\s*\(',
        'cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent\s*\('
    )) { Assert-Contains -Path $cp382PrefixValidation -Pattern $pattern -Description "bounded committed active-owner proof $pattern" }
foreach ($pattern in @(
        'completed_direct_cooling_mixed_air_call_is_consistent\s*\(',
        'completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent\s*\(',
        'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_assignment_is_consistent\s*\(',
        'completed_direct_cooling_supply_enthalpy_post_saturation_assignment_is_consistent\s*\('
    )) { Assert-NotContains -Path $cp382PrefixValidation -Pattern $pattern -Description "recursive active-owner completion $pattern" }
foreach ($forbidden in @(
        '\bmul_add\b', '\btotal_cmp\b', '\bf64::max\b', '\bf64::min\b',
        '\bepsilon\b', '\bclamp\b', 'MaxCoolTotCap',
        'DirectZonePurchasedAirCouplingInput', 'reconcile_'
    )) {
    Assert-NotContains -Path $cp382Transition -Pattern $forbidden -Description "forbidden arithmetic/feed $forbidden"
}
foreach ($path in @($cp382Transition, $cp382Release, $cp382Adapter, $cp382Coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $cp382Adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node_update|report|MaxCoolTotCap' -Description "adapter numerical/capacity reconciliation"

# CP381 -> CP382 -> CP383 -> CP384 -> unchanged numerical placement, with no CP382 DTO field.
$cp382BindingText = Read-RepoText -Path $cp382Binding
$cp381BindingIndexForCp382 = $cp382BindingText.IndexOf("let calculation_$cp381StemForCp382 =")
$cp382BindingIndex = $cp382BindingText.IndexOf("let calculation_$cp382Stem ="); $cp383BindingIndex = $cp382BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp382 = $cp382BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp382 = $cp382BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp382NumericalIndex = $cp382BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp381BindingIndexForCp382 -lt 0 -or $cp382BindingIndex -le $cp381BindingIndexForCp382 -or
    $cp383BindingIndex -le $cp382BindingIndex -or $cp384BindingIndexForCp382 -le $cp383BindingIndex -or $cp385BindingIndexForCp382 -le $cp384BindingIndexForCp382 -or $cp382NumericalIndex -le $cp385BindingIndexForCp382) {
    throw "Binding must execute CP381, CP382, CP383, CP384, then unchanged numerical coupling"
}
$cp382Dto = Get-Cp382RustBraceBlock -Text $cp382BindingText.Substring($cp382NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp382TextNotContains -Text $cp382Dto -Pattern 'cp382|post_saturation_capacity_limit_dehumidification_total_output_assignment' -Description "numerical DTO feed"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp382CalcRoot; Pattern = $cp382Stem },
        [PSCustomObject]@{ Path = $cp382ScheduledOutput; Pattern = "pub calculation_$($cp382Stem):" },
        [PSCustomObject]@{ Path = $cp382InitState; Pattern = $cp382Stem },
        [PSCustomObject]@{ Path = $cp382InitUnit; Pattern = $cp382Stem },
        [PSCustomObject]@{ Path = $cp382WitnessRoot; Pattern = $cp382Stem },
        [PSCustomObject]@{ Path = $cp382CoupledRoot; Pattern = "mod $($cp382Stem)_validation;" },
        [PSCustomObject]@{ Path = $cp382FixtureRoot; Pattern = $cp382Stem },
        [PSCustomObject]@{ Path = $cp382PipelineRoot; Pattern = $cp382PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}

# Direct-only lifecycle, IEEE JSON sidecars, and terminal nonfeed firewall.
Assert-Contains -Path $cp382PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp441_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp382PipelineRoot -Pattern $cp382Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @(
        'dehumidification_total_output_assignment_count', 'source_site_execution_count',
        'supply_mass_flow_rate_read_count', 'mixed_air_enthalpy_read_count',
        'supply_enthalpy_read_count', 'enthalpy_difference_calculation_count',
        'cooling_total_output_calculation_count',
        'cooling_total_output_assignment_write_count'
    )) {
    Assert-Contains -Path $cp382PipelineCounts -Pattern $pattern -Description "serialized checked count $pattern"
}
foreach ($pattern in @(
        'json_number', 'ieee_bits', 'supply_mass_flow_rate_kg_per_s',
        'mixed_air_enthalpy_j_per_kg', 'supply_enthalpy_j_per_kg',
        'mixed_air_minus_supply_enthalpy_j_per_kg',
        'calculated_cooling_total_output_w', 'cooling_total_output_w'
    )) {
    Assert-Contains -Path $cp382SnapshotSerialization -Pattern $pattern -Description "finite JSON/IEEE assignment evidence $pattern"
}
Assert-Contains -Path $cp381Assertions -Pattern 'mod cp382_assertions;' -Description "arbitrary CP382 module"
Assert-Contains -Path $cp381Assertions -Pattern 'cp382_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP382 direct delegation"
Assert-Contains -Path $cp381Assertions -Pattern 'cp382_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP382 non-direct delegation"
Assert-Contains -Path $cp382Assertions -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed"
Assert-NotContains -Path $cp382Assertions -Pattern 'MaxCoolTotCap|(?:latest|cp382|results)\["(?:supply_node|report|capacity_w)' -Description "CP382 excluded capacity/node/report assertion"
Assert-Contains -Path $cp382Assertions -Pattern 'mod cp383_assertions;' -Description "arbitrary CP383 module"
Assert-Contains -Path $cp382Assertions -Pattern 'cp383_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP383 direct delegation"
Assert-Contains -Path $cp382Assertions -Pattern 'cp383_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP383 non-direct delegation"
Assert-Contains -Path $cp383Assertions -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed"

# Exactly two algorithm/capability addenda and five ordered handwritten sections.
$cp382AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp382CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp382AlgorithmAddenda = [regex]::Matches($cp382AlgorithmText, '(?m)^\s*"CP382 supersedes only [^"\r\n]+",\s*$')
$cp382CapabilityAddenda = [regex]::Matches($cp382CapabilityText, '(?m)^\s*"CP382 additionally requires[^"\r\n]+",\s*$')
if ($cp382AlgorithmAddenda.Count -ne 2 -or $cp382CapabilityAddenda.Count -ne 2) {
    throw "CP382 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp382AlgorithmAddenda + $cp382CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp382SourceCommit, $cp382SourceHash, '2267', '2268', 'CP383',
            $cp382Sites[0], $cp382Sites[1], $cp382Sites[2],
            $cp382Sites[3], $cp382Sites[4], $cp382Sites[5],
            'eighteen', 'thirteen', '6\*A', 'CP381',
            'sole immediate predecessor', 'CP330', 'CP329', 'CP339', 'CP379',
            'IEEE binary64|Raw IEEE', 'NaN', 'infinity', 'sidecar',
            'DirectZonePurchasedAirCouplingInput', '320 total', '240 public',
            '80 internal', 'zero unused', 'zero unreachable',
            '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP382 spec addendum missing '$pattern'" }
    }
}
$cp382Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP382 Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP382 Source-Ordered Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP382 Cooling Post-Saturation Capacity-Limit Dehumidification Total-Output Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP382 Post-Saturation Capacity-Limit Dehumidification Total-Output Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP382 Post-Saturation Capacity-Limit Dehumidification Total-Output Assignment Placement' }
)
foreach ($doc in $cp382Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP382 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..382) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP382 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @(
            $cp382SourceCommit, $cp382SourceHash, '2267', '2268', 'CP383',
            $cp382Sites[0], $cp382Sites[1], $cp382Sites[2],
            $cp382Sites[3], $cp382Sites[4], $cp382Sites[5],
            'eighteen', '6\*A', 'CP381', 'CP330', 'CP329', 'CP379',
            '320\s+total', '80\s+internal'
        )) {
        if ($sections[0].Value -notmatch $required) {
            throw "CP382 documentation in $($doc.Path) missing '$required'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP382\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP382 supersedes only' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP382 additionally requires' -Description "generated capability addendum"

# Current-state propagation while the CP381 319/79 checkpoint stays historical.
foreach ($historical in 334..381) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp441_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..381) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 379 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 139 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..381) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 379' -Description "historical inventory total"
}
foreach ($historical in 367..381) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 139' -Description "historical internal classification count"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 136 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp382Stem" -Description "historical CP382 compact binding order"
}
foreach ($historical in 360..381) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp382BindingIndex' -Description "historical CP382 binding index"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp382Stem" -Description "historical CP382 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern 'CP381-to-CP382' -Description "CP345 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP382-to-CP383' -Description "CP345 successor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP388-to-CP389' -Description "CP345 CP389 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP389-to-CP390' -Description "CP345 CP390 predecessor interval"
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1" -Limit 600 -Description "CP347 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp349-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1" -Limit 500 -Description "CP349 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1" -Limit 500 -Description "CP362 historical audit"
foreach ($historical in 364, 373, 374, 376, 377, 378, 379, 380, 381) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp382_assertions\.rs' -Description "historical CP382 arbitrary terminal"
}
$cp381HistoricalAlgorithm = [regex]::Matches($cp382AlgorithmText, '(?m)^\s*"CP381 supersedes only CP380[^"\r\n]+",\s*$')
$cp381HistoricalCapability = [regex]::Matches($cp382CapabilityText, '(?m)^\s*"CP381 additionally requires[^"\r\n]+",\s*$')
if ($cp381HistoricalAlgorithm.Count -ne 2 -or $cp381HistoricalCapability.Count -ne 2) {
    throw "CP381 historical addenda count drift"
}
foreach ($claim in @($cp381HistoricalAlgorithm + $cp381HistoricalCapability)) {
    if ($claim.Value -notmatch '319 total' -or $claim.Value -notmatch '79 internal') {
        throw "CP381 historical addendum inventory numbers must remain 319/79"
    }
}

# Master order and current generated inventory.
$cp382MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp381AuditIndexForCp382 = $cp382MainAuditText.IndexOf("cp381-cooling-post-saturation-capacity-limit-dehumidification-guard.ps1")
$cp382AuditIndex = $cp382MainAuditText.IndexOf("cp382-cooling-post-saturation-capacity-limit-dehumidification-total-output-assignment.ps1")
$cp382CompletionIndex = $cp382MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp381AuditIndexForCp382 -lt 0 -or $cp382AuditIndex -le $cp381AuditIndexForCp382 -or
    $cp382CompletionIndex -le $cp382AuditIndex) {
    throw "Master audit must dot-source CP382 after CP381 before completion"
}
$cp382InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @(
        'script_count = 379', 'dev_command_count = 238',
        'unused_script_count = 0', 'unreachable_count = 0'
    )) {
    Assert-Cp382TextContains -Text $cp382InventoryText -Pattern $pattern -Description "inventory $pattern"
}
if ([regex]::Matches($cp382InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($cp382InventoryText, '(?m)^classification = "internal"$').Count -ne 139) {
    throw "CP382 inventory must be exactly 240 public and 136 internal scripts"
}
Assert-Cp382TextContains -Text $cp382InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp382-cooling-post-saturation-capacity-limit-dehumidification-total-output-assignment\.ps1"' -Description "inventory record"
Assert-Cp382TextContains -Text $cp382InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
foreach ($pattern in @(
        '\| 379 \|',
        '\| public scripts \| 240 \|',
        '\| 139 \|',
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
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp417Call' -Description 'CP417 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP416-to-CP417' -Description 'CP416-to-CP417 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP382 post-saturation dehumidification total-output assignment structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp437Call' -Description 'CP437 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-CP437' -Description 'CP436-to-CP437 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP436-to-' + 'numerical') -Description 'stale CP436 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp438Call' -Description 'CP438 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-CP438' -Description 'CP437-to-CP438 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP437-to-' + 'numerical') -Description 'stale CP437 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp439Call' -Description 'CP439 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP438-to-CP439' -Description 'CP438-to-CP439 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP438-to-' + 'numerical') -Description 'stale CP438 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp440Call' -Description 'CP440 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-CP440' -Description 'CP439-to-CP440 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP439-to-' + 'numerical') -Description 'stale CP439 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp441Call' -Description 'CP441 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP440-to-CP441' -Description 'CP440-to-CP441 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP440-to-' + 'numerical') -Description 'stale CP440 numerical interval'
