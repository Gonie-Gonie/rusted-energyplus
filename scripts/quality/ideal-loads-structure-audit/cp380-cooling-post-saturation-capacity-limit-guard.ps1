# CP380 maps only PurchasedAirManager.cc executable line 2264's short-circuited
# post-saturation capacity-limit guard, without entering its line-2266 body.
& {
$cp380Stem = "cooling_post_saturation_capacity_limit_guard"
$cp379StemForCp380 = "cooling_supply_enthalpy_post_saturation_assignment"
$cp380PipelineStem = "purchased_air_$cp380Stem"
$cp380Lifecycle = "purchased_air_calc_$($cp380Stem)_lifecycle"
$cp380SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp380SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp380Sites = @(
    "read-cooling-limit-for-post-saturation-capacity-comparison",
    "compare-cooling-limit-equal-to-capacity-for-post-saturation-capacity-guard",
    "read-cooling-limit-for-post-saturation-flow-rate-and-capacity-comparison-after-first-false",
    "compare-cooling-limit-equal-to-flow-rate-and-capacity-for-post-saturation-capacity-guard",
    "enter-post-saturation-capacity-limit-body-if-compound-condition-satisfied"
)
$cp380Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp380Module = "crates\ep_runtime\src\ideal_loads\calc\$cp380Stem.rs"
$cp380Root = "crates\ep_runtime\src\ideal_loads\calc\$cp380Stem"
$cp380State = "$cp380Root\state.rs"
$cp380Transition = "$cp380Root\transition.rs"
$cp380Release = "$cp380Root\release.rs"
$cp380CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp380Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp380Adapter = "crates\ep_runtime\src\ideal_loads\binding\$cp380Stem.rs"
$cp380BindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($cp380Stem)_tests.rs"
$cp380ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp380InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp380InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp380WitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp380Witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp380Stem.rs"
$cp380CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp380Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp380Stem)_validation.rs"
$cp380CoupledLifecycle = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp380Stem)_validation\lifecycle.rs"
$cp380CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp380Stem)_validation\snapshot.rs"
$cp380FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp380Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($cp380Stem)_fixture.rs"
$cp380PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp380Pipeline = "crates\ep_run\src\pipeline\$cp380PipelineStem.rs"
$cp380PipelineValidation = "crates\ep_run\src\pipeline\$cp380PipelineStem\validation.rs"
$cp380PipelineCounts = "crates\ep_run\src\pipeline\$cp380PipelineStem\validation\counts.rs"
$cp380PipelineSnapshotValidation = "crates\ep_run\src\pipeline\$cp380PipelineStem\validation\snapshot.rs"
$cp380Serialization = "crates\ep_run\src\pipeline\$cp380PipelineStem\serialization.rs"
$cp380SnapshotSerialization = "crates\ep_run\src\pipeline\$cp380PipelineStem\serialization\snapshot.rs"
$cp379Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp379_assertions.rs"
$cp380Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs"; $cp381Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp381_assertions.rs"
$cp380Audit = "scripts\quality\ideal-loads-structure-audit\cp380-cooling-post-saturation-capacity-limit-guard.ps1"

function Assert-Cp380TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP380 $Description missing" }
}

function Assert-Cp380TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP380 $Description unexpectedly present" }
}

function Get-Cp380RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP380 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP380 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP380 $Description closing brace missing"
}

$cp380Required = @(
    $cp380Module, $cp380State, $cp380Transition, $cp380Release, $cp380Adapter,
    $cp380BindingTests, $cp380Witness, $cp380Coupled, $cp380CoupledLifecycle,
    $cp380CoupledSnapshot, $cp380Fixture, $cp380Pipeline, $cp380PipelineValidation,
    $cp380PipelineCounts, $cp380PipelineSnapshotValidation, $cp380Serialization,
    $cp380SnapshotSerialization,
    $cp379Assertions, $cp380Assertions, $cp381Assertions, $cp380Audit
)
foreach ($file in $cp380Required) {
    Assert-FileExists -Path $file -Description "CP380 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP380 bounded file"
}
$cp380CoreFiles = @(Get-ChildItem -LiteralPath $cp380Root -Recurse -File -Filter "*.rs")
$cp380CoreText = ($cp380CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP380 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine

# Exact line-2264 boundary and five unique short-circuited source sites.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp380Source).Hash -cne $cp380SourceHash) {
    throw "CP380 PurchasedAirManager.cc SHA-256 drift"
}
$cp380Lines = Get-Content -Encoding UTF8 -LiteralPath $cp380Source
if ($cp380Lines[2263].Trim() -cne 'if ((PurchAir.CoolingLimit == LimitType::Capacity) || (PurchAir.CoolingLimit == LimitType::FlowRateAndCapacity)) {' -or
    $cp380Lines[2264].Trim() -cne '// If dehumidifying, compare total cooling to the limit' -or
    $cp380Lines[2265].Trim() -cne 'if (PurchAir.SupplyHumRat < PurchAir.MixedAirHumRat) { // Dehumidifying') {
    throw "CP380 line 2264 through first-excluded executable 2266 source boundary drift"
}
Assert-Contains -Path $cp380Module -Pattern 'PurchasedAirManager\.cc:2264' -Description "mapped source"
Assert-Contains -Path $cp380Module -Pattern 'PurchasedAirManager\.cc:2266' -Description "first excluded source"
Assert-ExactStringArray -Path $cp380Module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER' -Expected $cp380Sites -Description "five-site source order"

# Three complete-skip routes and two outcomes for every one of five active routes.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughBodyEntered',
        'HeatingAvailabilityGuardFalseFallthroughGuardFalseFallthrough',
        'HumidificationControlGuardFalseFallthroughBodyEntered',
        'HumidificationControlGuardFalseFallthroughGuardFalseFallthrough',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedGuardFalseFallthrough',
        'DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered',
        'DehumidificationControlNoneMaximumAssignmentExecutedGuardFalseFallthrough',
        'DehumidificationControlGuardFalseFallthroughBodyEntered',
        'DehumidificationControlGuardFalseFallthroughGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp380State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'capacity_limit_guard_evaluation_count', 'source_site_execution_count',
        'configured_cooling_limit_owned_read_count',
        'cp337_same_call_selector_lineage_corroboration_count',
        'first_cooling_limit_read_count', 'cooling_limit_capacity_comparison_count',
        'cooling_limit_capacity_match_count', 'second_cooling_limit_read_count',
        'cooling_limit_flow_rate_and_capacity_comparison_count',
        'cooling_limit_flow_rate_and_capacity_match_count', 'cooling_limit_rejected_count',
        'capacity_limit_body_entry_count', 'active_guard_false_fallthrough_count'
    )) {
    Assert-Contains -Path $cp380State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed',
        'capacity_limit_guard_evaluated', 'configured_cooling_limit_owned_read',
        'cp337_same_call_selector_lineage_corroborated', 'first_cooling_limit_read',
        'first_cooling_limit', 'cooling_limit_capacity_comparison_evaluated',
        'cooling_limit_capacity', 'second_cooling_limit_read', 'second_cooling_limit',
        'cooling_limit_flow_rate_and_capacity_comparison_evaluated',
        'cooling_limit_flow_rate_and_capacity', 'cooling_limit_condition_satisfied',
        'cooling_limit_rejected', 'capacity_limit_body_entered', 'active_guard_false_fallthrough'
    )) {
    Assert-Contains -Path $cp380Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}

# Typed owner, explicit short circuit, exact 2G+2S+B accounting, and no numerical/body feed.
foreach ($pattern in @(
        'let first = input\.cooling_limit;',
        'let capacity = first == IdealLoadsLimit::LimitCapacity;',
        'let second = \(!capacity\)\.then_some\(input\.cooling_limit\);',
        'limit == IdealLoadsLimit::LimitFlowRateAndCapacity',
        'Some\(capacity \|\| combined == Some\(true\)\)',
        'let source_sites = 2 \+ 2 \* usize::from\(second\) \+ usize::from\(body\);'
    )) {
    Assert-Contains -Path $cp380Transition -Pattern $pattern -Description "transition contract $pattern"
}
Assert-Contains -Path $cp380Transition -Pattern 'UnitOff[\s\S]+NonCooling[\s\S]+PositiveGuardFalseFallthrough' -Description "U/N/P inactive route set"
foreach ($path in @($cp380Transition, $cp380Release, $cp380Adapter, $cp380Coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
Assert-NotContains -Path $cp380Transition -Pattern '\bf64\b|DirectZonePurchasedAirCouplingInput|MaxCoolTotCap|SupplyMassFlowRate|MixedAirHumRat|CoolTotOutput|reconcile_' -Description "numerical/body/DTO feed"
foreach ($pattern in @(
        'predecessor_cp379', 'system\.cooling_limit',
        'cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release',
        'direct_predecessor_is_retained_and_complete',
        'direct_selector_lineage_is_retained_and_complete',
        'cp337_same_call_selector_lineage_corroborated:\s*true',
        'completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent'
    )) {
    Assert-Cp380TextContains -Text $cp380CoreText -Pattern $pattern -Description "release predecessor/owner proof $pattern"
}
Assert-NotContains -Path $cp380Adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node_update|report|MaxCoolTotCap' -Description "adapter numerical/body reconciliation"

# CP379 -> CP380 -> unchanged numerical placement, with no CP380 DTO field.
$cp380BindingText = Read-RepoText -Path $cp380Binding
$cp379BindingIndexForCp380 = $cp380BindingText.IndexOf("let calculation_$cp379StemForCp380 =")
$cp380BindingIndex = $cp380BindingText.IndexOf("let calculation_$cp380Stem ="); $cp381BindingIndex = $cp380BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndex = $cp380BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndex = $cp380BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp380 = $cp380BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp380 = $cp380BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp380NumericalIndex = $cp380BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp379BindingIndexForCp380 -lt 0 -or $cp380BindingIndex -le $cp379BindingIndexForCp380 -or
    $cp381BindingIndex -le $cp380BindingIndex -or $cp382BindingIndex -le $cp381BindingIndex -or $cp383BindingIndex -le $cp382BindingIndex -or $cp384BindingIndexForCp380 -le $cp383BindingIndex -or $cp385BindingIndexForCp380 -le $cp384BindingIndexForCp380 -or $cp380NumericalIndex -le $cp385BindingIndexForCp380) {
    throw "Binding must execute CP379, CP380, CP381, then unchanged numerical coupling"
}
$cp380Dto = Get-Cp380RustBraceBlock -Text $cp380BindingText.Substring($cp380NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp380TextNotContains -Text $cp380Dto -Pattern 'cp380|post_saturation_capacity|capacity_limit_guard' -Description "numerical DTO feed"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp380CalcRoot; Pattern = $cp380Stem },
        [PSCustomObject]@{ Path = $cp380ScheduledOutput; Pattern = "pub calculation_$($cp380Stem):" },
        [PSCustomObject]@{ Path = $cp380InitState; Pattern = $cp380Stem },
        [PSCustomObject]@{ Path = $cp380InitUnit; Pattern = $cp380Stem },
        [PSCustomObject]@{ Path = $cp380WitnessRoot; Pattern = $cp380Stem },
        [PSCustomObject]@{ Path = $cp380CoupledRoot; Pattern = "mod $($cp380Stem)_validation;" },
        [PSCustomObject]@{ Path = $cp380FixtureRoot; Pattern = $cp380Stem },
        [PSCustomObject]@{ Path = $cp380PipelineRoot; Pattern = $cp380PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}

# Direct-only lifecycle/serialization and final arbitrary numerical-nonfeed firewall.
Assert-Contains -Path $cp380PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp441_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp380PipelineRoot -Pattern $cp380Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @('source_site_execution_count', 'configured_cooling_limit_owned_read_count', 'cooling_limit_capacity_match_count', 'second_cooling_limit_read_count', 'capacity_limit_body_entry_count', 'active_guard_false_fallthrough_count')) {
    Assert-Contains -Path $cp380PipelineCounts -Pattern $pattern -Description "serialized checked count $pattern"
}
foreach ($pattern in @('source_order', 'first_cooling_limit', 'second_cooling_limit', 'cooling_limit_condition_satisfied', 'capacity_limit_body_entered', 'active_guard_false_fallthrough')) {
    Assert-Contains -Path $cp380SnapshotSerialization -Pattern $pattern -Description "serialized guard evidence $pattern"
}
Assert-Contains -Path $cp379Assertions -Pattern 'mod cp380_assertions;' -Description "arbitrary CP380 module"
Assert-Contains -Path $cp379Assertions -Pattern 'cp380_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP380 direct delegation"
Assert-Contains -Path $cp379Assertions -Pattern 'cp380_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP380 non-direct delegation"
Assert-Contains -Path $cp380Assertions -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP380 terminal numerical nonfeed"; Assert-Contains -Path $cp380Assertions -Pattern 'mod cp381_assertions;' -Description "arbitrary CP381 module"; Assert-Contains -Path $cp380Assertions -Pattern 'cp381_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP381 direct delegation"; Assert-Contains -Path $cp380Assertions -Pattern 'cp381_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP381 non-direct delegation"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed firewall"
Assert-NotContains -Path $cp381Assertions -Pattern 'MaxCoolTotCap|CoolTotOutput|SupplyMassFlowRate|MixedAirEnthalpy|SupplyEnthalpy|(?:latest|cp381|results)\["(?:supply_node|report|capacity_w)' -Description "CP381 body/numerical assertion"

# Exactly two algorithm/capability addenda and five ordered handwritten sections.
$cp380AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp380CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp380AlgorithmAddenda = [regex]::Matches($cp380AlgorithmText, '(?m)^\s*"CP380 supersedes only CP379[^"\r\n]+",\s*$')
$cp380CapabilityAddenda = [regex]::Matches($cp380CapabilityText, '(?m)^\s*"CP380 additionally requires[^"\r\n]+",\s*$')
if ($cp380AlgorithmAddenda.Count -ne 2 -or $cp380CapabilityAddenda.Count -ne 2) {
    throw "CP380 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp380AlgorithmAddenda + $cp380CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp380SourceCommit, $cp380SourceHash, '2264', '2266', 'CP381',
            $cp380Sites[0], $cp380Sites[1], $cp380Sites[2], $cp380Sites[3], $cp380Sites[4],
            'U/N/P|UnitOff `U`.+non-cooling `N`.+positive-guard-false `P`',
            'H/Hu/DH/DN/DG|heating-availability false `H`.+humidification-control false `Hu`',
            '2\*G\+2\*S\+B', 'CP379', 'sole immediate predecessor',
            'IdealLoadsAirSystem\.cooling_limit|typed system.*cooling_limit', 'CP337',
            'never enters line 2266|Line 2266', 'DirectZonePurchasedAirCouplingInput',
            'no routine or psychrometrics-map row|adds no routine or psychrometrics-map row|counts remain 32 algorithms',
            '318 total', '240 public', '78 internal', 'zero unused', 'zero unreachable',
            '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP380 spec addendum missing '$pattern'" }
    }
}
$cp380Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP380 Cooling Post-Saturation Capacity-Limit Guard' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP380 Source-Ordered Cooling Post-Saturation Capacity-Limit Guard' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP380 Cooling Post-Saturation Capacity-Limit Guard' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP380 Post-Saturation Capacity-Limit Guard in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP380 Post-Saturation Capacity-Limit Guard Placement' }
)
foreach ($doc in $cp380Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP380 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..380) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP380 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @($cp380SourceCommit, $cp380SourceHash, '2264', '2266', 'CP381', $cp380Sites[0], $cp380Sites[1], $cp380Sites[2], $cp380Sites[3], $cp380Sites[4], 'CP379', 'CP337', 'cooling_limit', '2\*G\+2\*S\+B', '318\s+total', '78\s+internal')) {
        if ($sections[0].Value -notmatch $required) { throw "CP380 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP380\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP380 supersedes only CP379' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP380 additionally requires' -Description "generated capability addendum"

# Historical current-state propagation while CP379's 317/77 checkpoint stays historical.
foreach ($historical in 334..379) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp441_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..379) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 379 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 139 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..379) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 379' -Description "historical inventory total"
}
foreach ($historical in 367..379) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 139' -Description "historical internal classification count"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 136 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp380Stem" -Description "historical CP380 compact binding order"
}
foreach ($historical in 360..379) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp380BindingIndex' -Description "historical CP380 binding index"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp380Stem" -Description "historical CP380 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP388-to-CP389' -Description "CP345 CP389 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP389-to-CP390' -Description "CP345 CP390 predecessor interval"
Assert-LineLimit -Path $cp345Audit -Limit 1201 -Description "CP345 historical audit"
foreach ($historical in 364, 373, 374, 376, 377, 378, 379) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp380_assertions\.rs' -Description "historical CP380 arbitrary terminal"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp382_assertions\.rs' -Description "historical CP382 arbitrary terminal"
}
$cp379HistoricalAlgorithm = [regex]::Matches($cp380AlgorithmText, '(?m)^\s*"CP379 supersedes only CP378[^"\r\n]+",\s*$')
$cp379HistoricalCapability = [regex]::Matches($cp380CapabilityText, '(?m)^\s*"CP379 additionally requires[^"\r\n]+",\s*$')
if ($cp379HistoricalAlgorithm.Count -ne 2 -or $cp379HistoricalCapability.Count -ne 2) { throw "CP379 historical addenda count drift" }
foreach ($claim in @($cp379HistoricalAlgorithm + $cp379HistoricalCapability)) {
    if ($claim.Value -notmatch '317 total' -or $claim.Value -notmatch '77 internal') { throw "CP379 historical addendum inventory numbers must remain 317/77" }
}

# Master order and current generated inventory.
$cp380MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp379AuditIndexForCp380 = $cp380MainAuditText.IndexOf("cp379-cooling-supply-enthalpy-post-saturation-assignment.ps1")
$cp380AuditIndex = $cp380MainAuditText.IndexOf("cp380-cooling-post-saturation-capacity-limit-guard.ps1")
$cp380CompletionIndex = $cp380MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp379AuditIndexForCp380 -lt 0 -or $cp380AuditIndex -le $cp379AuditIndexForCp380 -or $cp380CompletionIndex -le $cp380AuditIndex) {
    throw "Master audit must dot-source CP380 after CP379 before completion"
}
$cp380InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 379', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp380TextContains -Text $cp380InventoryText -Pattern $pattern -Description "inventory $pattern"
}
if ([regex]::Matches($cp380InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($cp380InventoryText, '(?m)^classification = "internal"$').Count -ne 139) {
    throw "CP380 inventory must be exactly 240 public and 136 internal scripts"
}
Assert-Cp380TextContains -Text $cp380InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp380-cooling-post-saturation-capacity-limit-guard\.ps1"' -Description "inventory record"
Assert-Cp380TextContains -Text $cp380InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
foreach ($pattern in @('\| 379 \|', '\| public scripts \| 240 \|', '\| 139 \|', '\| scripts without callers \| 0 \|')) {
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
Write-Host "CP380 post-saturation capacity-limit guard structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp437Call' -Description 'CP437 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-CP437' -Description 'CP436-to-CP437 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP436-to-' + 'numerical') -Description 'stale CP436 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp438Call' -Description 'CP438 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-CP438' -Description 'CP437-to-CP438 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP437-to-' + 'numerical') -Description 'stale CP437 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp439Call' -Description 'CP439 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP438-to-CP439' -Description 'CP438-to-CP439 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP438-to-' + 'numerical') -Description 'stale CP438 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp440Call' -Description 'CP440 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-CP440' -Description 'CP439-to-CP440 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP439-to-' + 'numerical') -Description 'stale CP439 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp441Call' -Description 'CP441 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP440-to-CP441' -Description 'CP440-to-CP441 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP440-to-' + 'numerical') -Description 'stale CP440 numerical interval'
