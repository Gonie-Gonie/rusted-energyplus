# CP381 maps only PurchasedAirManager.cc executable line 2266's humidity-ratio
# comparison guard, without entering line 2267's total-cooling calculation.
& {
$cp381Stem = "cooling_post_saturation_capacity_limit_dehumidification_guard"
$cp380StemForCp381 = "cooling_post_saturation_capacity_limit_guard"
$cp381PipelineStem = "purchased_air_$cp381Stem"
$cp381Lifecycle = "purchased_air_calc_$($cp381Stem)_lifecycle"
$cp381SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp381SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp381Sites = @(
    "read-retained-purchased-air-supply-humidity-ratio-for-post-saturation-dehumidification-comparison",
    "read-retained-purchased-air-mixed-air-humidity-ratio-for-post-saturation-dehumidification-comparison",
    "compare-purchased-air-supply-humidity-ratio-strictly-less-than-mixed-air-humidity-ratio-for-post-saturation-dehumidification-guard",
    "enter-post-saturation-capacity-limit-dehumidification-body-if-comparison-satisfied"
)
$cp381Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp381Module = "crates\ep_runtime\src\ideal_loads\calc\$cp381Stem.rs"
$cp381Root = "crates\ep_runtime\src\ideal_loads\calc\$cp381Stem"
$cp381State = "$cp381Root\state.rs"
$cp381Transition = "$cp381Root\transition.rs"
$cp381Release = "$cp381Root\release.rs"
$cp381CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp381Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp381Adapter = "crates\ep_runtime\src\ideal_loads\binding\$cp381Stem.rs"
$cp381BindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($cp381Stem)_tests.rs"
$cp381ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp381InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp381InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp381WitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp381Witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp381Stem.rs"
$cp381CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp381Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp381Stem)_validation.rs"
$cp381CoupledLifecycle = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp381Stem)_validation\lifecycle.rs"
$cp381CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp381Stem)_validation\snapshot.rs"
$cp381FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp381Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($cp381Stem)_fixture.rs"
$cp381PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp381Pipeline = "crates\ep_run\src\pipeline\$cp381PipelineStem.rs"
$cp381PipelineValidation = "crates\ep_run\src\pipeline\$cp381PipelineStem\validation.rs"
$cp381PipelineCounts = "crates\ep_run\src\pipeline\$cp381PipelineStem\validation\counts.rs"
$cp381PipelineSnapshotValidation = "crates\ep_run\src\pipeline\$cp381PipelineStem\validation\snapshot.rs"
$cp381Serialization = "crates\ep_run\src\pipeline\$cp381PipelineStem\serialization.rs"
$cp381SnapshotSerialization = "crates\ep_run\src\pipeline\$cp381PipelineStem\serialization\snapshot.rs"
$cp380Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs"
$cp381Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp381_assertions.rs"
$cp381Audit = "scripts\quality\ideal-loads-structure-audit\cp381-cooling-post-saturation-capacity-limit-dehumidification-guard.ps1"

function Assert-Cp381TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP381 $Description missing" }
}

function Assert-Cp381TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP381 $Description unexpectedly present" }
}

function Get-Cp381RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP381 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP381 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP381 $Description closing brace missing"
}

$cp381Required = @(
    $cp381Module, $cp381State, $cp381Transition, $cp381Release, $cp381Adapter,
    $cp381BindingTests, $cp381Witness, $cp381Coupled, $cp381CoupledLifecycle,
    $cp381CoupledSnapshot, $cp381Fixture, $cp381Pipeline, $cp381PipelineValidation,
    $cp381PipelineCounts, $cp381PipelineSnapshotValidation, $cp381Serialization,
    $cp381SnapshotSerialization, $cp380Assertions, $cp381Assertions, $cp381Audit
)
foreach ($file in $cp381Required) {
    Assert-FileExists -Path $file -Description "CP381 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP381 bounded file"
}
$cp381CoreFiles = @(Get-ChildItem -LiteralPath $cp381Root -Recurse -File -Filter "*.rs")
$cp381CoreText = ($cp381CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP381 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine

# Exact line-2266 boundary and four textual operand/comparison/body sites.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp381Source).Hash -cne $cp381SourceHash) {
    throw "CP381 PurchasedAirManager.cc SHA-256 drift"
}
$cp381Lines = Get-Content -Encoding UTF8 -LiteralPath $cp381Source
if ($cp381Lines[2265].Trim() -cne 'if (PurchAir.SupplyHumRat < PurchAir.MixedAirHumRat) { // Dehumidifying' -or
    $cp381Lines[2266].Trim() -cne 'CoolTotOutput = SupplyMassFlowRate * (MixedAirEnthalpy - SupplyEnthalpy);') {
    throw "CP381 line 2266 through first-excluded executable 2267 source boundary drift"
}
Assert-Contains -Path $cp381Module -Pattern 'PurchasedAirManager\.cc:2266' -Description "mapped source"
Assert-Contains -Path $cp381Module -Pattern 'PurchasedAirManager\.cc:2267' -Description "first excluded source"
Assert-ExactStringArray -Path $cp381Module -Name 'PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER' -Expected $cp381Sites -Description "four-site source order"

# Eighteen retained routes: eight skips and two successors per five active CP380 routes.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthroughDehumidificationBodyEntered',
        'HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
        'HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'HumidificationControlGuardFalseFallthroughDehumidificationBodyEntered',
        'HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationBodyEntered',
        'DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough',
        'DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationBodyEntered',
        'DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough',
        'DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough',
        'DehumidificationControlGuardFalseFallthroughDehumidificationBodyEntered',
        'DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp381State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'dehumidification_guard_evaluation_count', 'source_site_execution_count',
        'cp378_supply_humidity_ratio_saturation_limit_owned_read_count',
        'cp379_same_call_supply_humidity_ratio_bit_corroboration_count',
        'purchased_air_supply_humidity_ratio_read_count',
        'cp329_mixed_air_humidity_ratio_owned_read_count',
        'purchased_air_mixed_air_humidity_ratio_read_count',
        'supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count',
        'supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count',
        'dehumidification_body_entry_count', 'dehumidification_guard_false_fallthrough_count'
    )) {
    Assert-Contains -Path $cp381State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_capacity_limit_guard_evaluated', 'predecessor_capacity_limit_body_entered',
        'predecessor_active_capacity_limit_guard_false_fallthrough',
        'dehumidification_guard_evaluated',
        'cp378_supply_humidity_ratio_saturation_limit_owned_read',
        'cp379_same_call_supply_humidity_ratio_bit_corroborated',
        'purchased_air_supply_humidity_ratio_read', 'supply_humidity_ratio',
        'cp329_mixed_air_humidity_ratio_owned_read',
        'purchased_air_mixed_air_humidity_ratio_read', 'mixed_air_humidity_ratio',
        'supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated',
        'supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio',
        'dehumidification_body_entered', 'dehumidification_guard_false_fallthrough'
    )) {
    Assert-Contains -Path $cp381Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}

# Raw source-shaped comparison, exact 3E+L accounting, and no line-2267 arithmetic.
foreach ($pattern in @(
        'strictly_less:\s*Some\(input\.supply_humidity_ratio < input\.mixed_air_humidity_ratio\)',
        'source_site_execution_count \+= 3 \+ usize::from\(body_entered\)',
        '\.checked_add\(3 \+ usize::from\(body\)\)',
        'cp378_supply_humidity_ratio_saturation_limit_owned_read',
        'cp379_same_call_supply_humidity_ratio_bit_corroborated',
        'cp329_mixed_air_humidity_ratio_owned_read'
    )) {
    Assert-Contains -Path $cp381Transition -Pattern $pattern -Description "transition contract $pattern"
}
Assert-Contains -Path $cp381Transition -Pattern 'HeatingAvailabilityGuardFalseFallthroughBodyEntered[\s\S]+HumidificationControlGuardFalseFallthroughBodyEntered[\s\S]+DehumidificationControlHumidistatMaximumAssignmentExecutedBodyEntered[\s\S]+DehumidificationControlNoneMaximumAssignmentExecutedBodyEntered[\s\S]+DehumidificationControlGuardFalseFallthroughBodyEntered' -Description "five active CP380 predecessor routes"
Assert-NotContains -Path $cp381Transition -Pattern 'CoolTotOutput|SupplyMassFlowRate|MixedAirEnthalpy|SupplyEnthalpy|DirectZonePurchasedAirCouplingInput|reconcile_|total_cmp|epsilon|mul_add' -Description "line-2267 arithmetic or DTO feed"
foreach ($path in @($cp381Transition, $cp381Release, $cp381Adapter, $cp381Coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
foreach ($pattern in @(
        'predecessor_cp380',
        'completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent',
        'cp378_supply_humidity_ratio_saturation_limit_owned_read',
        'cp379_same_call_supply_humidity_ratio_bit_corroborated',
        'cp329_mixed_air_humidity_ratio_owned_read',
        'completed_direct_cooling_post_saturation_capacity_limit_dehumidification_guard_is_consistent'
    )) {
    Assert-Cp381TextContains -Text $cp381CoreText -Pattern $pattern -Description "release predecessor/owner proof $pattern"
}
Assert-NotContains -Path $cp381Adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node_update|report|CoolTotOutput' -Description "adapter numerical/body reconciliation"

# CP380 -> CP381 -> unchanged numerical placement, with no CP381 DTO field.
$cp381BindingText = Read-RepoText -Path $cp381Binding
$cp380BindingIndexForCp381 = $cp381BindingText.IndexOf("let calculation_$cp380StemForCp381 =")
$cp381BindingIndex = $cp381BindingText.IndexOf("let calculation_$cp381Stem =")
$cp381NumericalIndex = $cp381BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp380BindingIndexForCp381 -lt 0 -or $cp381BindingIndex -le $cp380BindingIndexForCp381 -or
    $cp381NumericalIndex -le $cp381BindingIndex) {
    throw "Binding must execute CP380, CP381, then unchanged numerical coupling"
}
$cp381Dto = Get-Cp381RustBraceBlock -Text $cp381BindingText.Substring($cp381NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp381TextNotContains -Text $cp381Dto -Pattern 'cp381|post_saturation_capacity_limit_dehumidification_guard' -Description "numerical DTO feed"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp381CalcRoot; Pattern = $cp381Stem },
        [PSCustomObject]@{ Path = $cp381ScheduledOutput; Pattern = "pub calculation_$($cp381Stem):" },
        [PSCustomObject]@{ Path = $cp381InitState; Pattern = $cp381Stem },
        [PSCustomObject]@{ Path = $cp381InitUnit; Pattern = $cp381Stem },
        [PSCustomObject]@{ Path = $cp381WitnessRoot; Pattern = $cp381Stem },
        [PSCustomObject]@{ Path = $cp381CoupledRoot; Pattern = "mod $($cp381Stem)_validation;" },
        [PSCustomObject]@{ Path = $cp381FixtureRoot; Pattern = $cp381Stem },
        [PSCustomObject]@{ Path = $cp381PipelineRoot; Pattern = $cp381PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}

# Direct-only lifecycle, finite/IEEE JSON sidecars, and final arbitrary nonfeed firewall.
Assert-Contains -Path $cp381PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp381_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp381PipelineRoot -Pattern $cp381Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @('dehumidification_guard_evaluation_count', 'source_site_execution_count', 'purchased_air_supply_humidity_ratio_read_count', 'purchased_air_mixed_air_humidity_ratio_read_count', 'dehumidification_body_entry_count', 'dehumidification_guard_false_fallthrough_count')) {
    Assert-Contains -Path $cp381PipelineCounts -Pattern $pattern -Description "serialized checked count $pattern"
}
foreach ($pattern in @('json_number', 'ieee_bits', 'supply_humidity_ratio_ieee_bits', 'mixed_air_humidity_ratio_ieee_bits', 'supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio', 'dehumidification_body_entered')) {
    Assert-Contains -Path $cp381SnapshotSerialization -Pattern $pattern -Description "finite JSON/IEEE guard evidence $pattern"
}
Assert-Contains -Path $cp380Assertions -Pattern 'mod cp381_assertions;' -Description "arbitrary CP381 module"
Assert-Contains -Path $cp380Assertions -Pattern 'cp381_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP381 direct delegation"
Assert-Contains -Path $cp380Assertions -Pattern 'cp381_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP381 non-direct delegation"
Assert-Contains -Path $cp381Assertions -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP381 terminal numerical nonfeed"
Assert-NotContains -Path $cp381Assertions -Pattern 'MaxCoolTotCap|CoolTotOutput|SupplyMassFlowRate|MixedAirEnthalpy|SupplyEnthalpy|(?:latest|cp381|results)\["(?:supply_node|report|capacity_w)' -Description "CP381 body/numerical assertion"

# Exactly two algorithm/capability addenda and five ordered handwritten sections.
$cp381AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp381CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp381AlgorithmAddenda = [regex]::Matches($cp381AlgorithmText, '(?m)^\s*"CP381 supersedes only CP380[^"\r\n]+",\s*$')
$cp381CapabilityAddenda = [regex]::Matches($cp381CapabilityText, '(?m)^\s*"CP381 additionally requires[^"\r\n]+",\s*$')
if ($cp381AlgorithmAddenda.Count -ne 2 -or $cp381CapabilityAddenda.Count -ne 2) {
    throw "CP381 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp381AlgorithmAddenda + $cp381CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp381SourceCommit, $cp381SourceHash, '2266', '2267', 'CP382',
            $cp381Sites[0], $cp381Sites[1], $cp381Sites[2], $cp381Sites[3],
            'thirteen routes into eighteen|thirteen routes.*eighteen', '3\*E\+L',
            'CP380', 'sole immediate predecessor', 'CP378', 'CP379', 'CP329',
            'IEEE binary64|Raw IEEE', 'signed-zero', 'NaN',
            'DirectZonePurchasedAirCouplingInput', '319 total', '240 public',
            '79 internal', 'zero unused', 'zero unreachable', '238 development commands',
            'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP381 spec addendum missing '$pattern'" }
    }
}
$cp381Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP381 Cooling Post-Saturation Capacity-Limit Dehumidification Guard' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP381 Source-Ordered Cooling Post-Saturation Capacity-Limit Dehumidification Guard' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP381 Cooling Post-Saturation Capacity-Limit Dehumidification Guard' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP381 Post-Saturation Capacity-Limit Dehumidification Guard in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP381 Post-Saturation Capacity-Limit Dehumidification Guard Placement' }
)
foreach ($doc in $cp381Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP381 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..381) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP381 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @($cp381SourceCommit, $cp381SourceHash, '2266', '2267', 'CP382', $cp381Sites[0], $cp381Sites[1], $cp381Sites[2], $cp381Sites[3], 'eighteen', '3\*E\+L', 'CP380', 'CP378', 'CP379', 'CP329', '319\s+total', '79\s+internal')) {
        if ($sections[0].Value -notmatch $required) { throw "CP381 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP381\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP381 supersedes only CP380' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP381 additionally requires' -Description "generated capability addendum"

# Historical current-state propagation while CP380's 318/78 checkpoint stays historical.
foreach ($historical in 334..380) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp381_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..380) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 319 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 79 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..380) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 319' -Description "historical inventory total"
}
foreach ($historical in 367..380) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'Count -ne 79' -Description "historical internal classification count"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern '240 public and 79 internal' -Description "historical classification diagnostic"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp381Stem" -Description "historical CP381 compact binding order"
}
foreach ($historical in 360..380) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp381BindingIndex' -Description "historical CP381 binding index"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..344 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp381Stem" -Description "historical CP381 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern 'CP380-to-CP381' -Description "CP345 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP381-to-numerical' -Description "CP345 terminal interval"
Assert-LineLimit -Path $cp345Audit -Limit 1200 -Description "CP345 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp347-cooling-positive-supply-post-capacity-limit-dehumidification-control-none-case.ps1" -Limit 600 -Description "CP347 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp349-cooling-positive-supply-post-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-cp-air-assignment.ps1" -Limit 500 -Description "CP349 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1" -Limit 500 -Description "CP362 historical audit"
foreach ($historical in 364, 373, 374, 376, 377, 378, 379, 380) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp381_assertions\.rs' -Description "historical CP381 arbitrary terminal"
}
$cp380HistoricalAlgorithm = [regex]::Matches($cp381AlgorithmText, '(?m)^\s*"CP380 supersedes only CP379[^"\r\n]+",\s*$')
$cp380HistoricalCapability = [regex]::Matches($cp381CapabilityText, '(?m)^\s*"CP380 additionally requires[^"\r\n]+",\s*$')
if ($cp380HistoricalAlgorithm.Count -ne 2 -or $cp380HistoricalCapability.Count -ne 2) { throw "CP380 historical addenda count drift" }
foreach ($claim in @($cp380HistoricalAlgorithm + $cp380HistoricalCapability)) {
    if ($claim.Value -notmatch '318 total' -or $claim.Value -notmatch '78 internal') { throw "CP380 historical addendum inventory numbers must remain 318/78" }
}

# Master order and current generated inventory.
$cp381MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp380AuditIndexForCp381 = $cp381MainAuditText.IndexOf("cp380-cooling-post-saturation-capacity-limit-guard.ps1")
$cp381AuditIndex = $cp381MainAuditText.IndexOf("cp381-cooling-post-saturation-capacity-limit-dehumidification-guard.ps1")
$cp381CompletionIndex = $cp381MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp380AuditIndexForCp381 -lt 0 -or $cp381AuditIndex -le $cp380AuditIndexForCp381 -or $cp381CompletionIndex -le $cp381AuditIndex) {
    throw "Master audit must dot-source CP381 after CP380 before completion"
}
$cp381InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 319', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp381TextContains -Text $cp381InventoryText -Pattern $pattern -Description "inventory $pattern"
}
if ([regex]::Matches($cp381InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
    [regex]::Matches($cp381InventoryText, '(?m)^classification = "internal"$').Count -ne 79) {
    throw "CP381 inventory must be exactly 240 public and 79 internal scripts"
}
Assert-Cp381TextContains -Text $cp381InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp381-cooling-post-saturation-capacity-limit-dehumidification-guard\.ps1"' -Description "inventory record"
Assert-Cp381TextContains -Text $cp381InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
foreach ($pattern in @('\| executable script records \| 319 \|', '\| public scripts \| 240 \|', '\| internal scripts \| 79 \|', '\| scripts without callers \| 0 \|')) {
    Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern $pattern -Description "generated script inventory $pattern"
}

Write-Host "CP381 post-saturation capacity-limit dehumidification guard structure audit passed."
}
