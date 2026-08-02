# CP378 maps only PurchasedAirManager.cc physical executable line 2260's
# purchased-air saturation-limit assignment and reconciles without feeding.
& {
$cp378Stem = "cooling_supply_humidity_ratio_saturation_limit_assignment"
$cp377StemForCp378 = "cooling_supply_humidity_ratio_saturation_assignment"
$cp378PipelineStem = "purchased_air_$cp378Stem"
$cp378Lifecycle = "purchased_air_calc_${cp378Stem}_lifecycle"
$cp378SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp378SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp378Sites = @(
    "read-local-original-supply-humidity-ratio-for-saturation-limit-minimum",
    "read-local-saturation-supply-humidity-ratio-for-saturation-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-saturation-limit",
    "assign-purchased-air-supply-humidity-ratio-for-saturation-limit"
)
$cp378Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp378Module = "crates\ep_runtime\src\ideal_loads\calc\$cp378Stem.rs"
$cp378Root = "crates\ep_runtime\src\ideal_loads\calc\$cp378Stem"
$cp378State = "$cp378Root\state.rs"
$cp378Transition = "$cp378Root\transition.rs"
$cp378Release = "$cp378Root\release.rs"
$cp378Prefix = "$cp378Root\release\prefix_validation.rs"
$cp378Runtime = "$cp378Root\release\runtime_validation.rs"
$cp378Snapshot = "$cp378Root\release\snapshot_validation.rs"
$cp378Private = "$cp378Root\release\private_characterization.rs"
$cp378CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp378Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp378Adapter = "crates\ep_runtime\src\ideal_loads\binding\$cp378Stem.rs"
$cp378BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp378Stem}_tests.rs"
$cp378BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp378ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp378InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp378InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp378WitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp378Witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp378Stem.rs"
$cp378CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp378Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp378Stem}_validation.rs"
$cp378CoupledLifecycle = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp378Stem}_validation\lifecycle.rs"
$cp378CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp378Stem}_validation\snapshot.rs"
$cp378CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp378CoupledTests = "crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp378.rs"
$cp378FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp378Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp378Stem}_fixture.rs"
$cp378PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp378Pipeline = "crates\ep_run\src\pipeline\$cp378PipelineStem.rs"
$cp378PipelineValidation = "crates\ep_run\src\pipeline\$cp378PipelineStem\validation.rs"
$cp378PipelineCounts = "crates\ep_run\src\pipeline\$cp378PipelineStem\validation\counts.rs"
$cp378PipelineSnapshot = "crates\ep_run\src\pipeline\$cp378PipelineStem\validation\snapshot.rs"
$cp378Serialization = "crates\ep_run\src\pipeline\$cp378PipelineStem\serialization.rs"
$cp378SnapshotSerialization = "crates\ep_run\src\pipeline\$cp378PipelineStem\serialization\snapshot.rs"
$cp378ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp377_assertions.rs"
$cp378Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp378_assertions.rs"; $cp379Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp379_assertions.rs"; $cp380Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs"; $cp381Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp381_assertions.rs"
$cp378Audit = "scripts\quality\ideal-loads-structure-audit\cp378-cooling-supply-humidity-ratio-saturation-limit-assignment.ps1"

function Assert-Cp378TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP378 $Description missing" }
}

function Assert-Cp378TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP378 $Description unexpectedly present" }
}

function Get-Cp378RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP378 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP378 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP378 $Description closing brace missing"
}

$cp378Required = @(
    $cp378Module, $cp378State, $cp378Transition, $cp378Release, $cp378Prefix,
    $cp378Runtime, $cp378Snapshot, $cp378Private, $cp378Adapter,
    $cp378BindingTests, $cp378Witness, $cp378Coupled, $cp378CoupledLifecycle,
    $cp378CoupledSnapshot, $cp378CoupledTests, $cp378Fixture, $cp378Pipeline,
    $cp378PipelineValidation, $cp378PipelineCounts, $cp378PipelineSnapshot,
    $cp378Serialization, $cp378SnapshotSerialization,
    $cp378Assertions, $cp379Assertions, $cp378Audit
)
foreach ($file in $cp378Required) {
    Assert-FileExists -Path $file -Description "CP378 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP378 bounded file"
}
$cp378CoreFiles = @(Get-ChildItem -LiteralPath $cp378Root -Recurse -File -Filter "*.rs")
$cp378CoreText = ($cp378CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP378 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join "`n"
$cp378CoreTests = @(Get-ChildItem -LiteralPath "$cp378Root\tests" -File -Filter "*.rs")
if ($cp378CoreTests.Count -lt 4) { throw "CP378 requires bounded route/release/IEEE/overflow tests" }
$cp378TestText = ($cp378CoreTests | ForEach-Object { Read-RepoText -Path $_.FullName }) -join "`n"
foreach ($pattern in @('direct', 'private', 'overflow', 'to_bits', '-0\.0', 'INFINITY', 'NAN', 'right')) {
    Assert-Cp378TextContains -Text $cp378TestText -Pattern $pattern -Description "core test matrix '$pattern'"
}

# Exact line 2260 boundary and four source sites.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp378Source).Hash -cne $cp378SourceHash) {
    throw "CP378 PurchasedAirManager.cc SHA-256 drift"
}
$cp378Lines = Get-Content -Encoding UTF8 -LiteralPath $cp378Source
if ($cp378Lines[2259].Trim() -cne 'PurchAir.SupplyHumRat = min(SupplyHumRatOrig, SupplyHumRatSat);' -or
    $cp378Lines[2260].Trim() -cne 'SupplyEnthalpy = PsyHFnTdbW(PurchAir.SupplyTemp, PurchAir.SupplyHumRat);') {
    throw "CP378 line 2260/2261 source boundary drift"
}
Assert-Contains -Path $cp378Module -Pattern 'PurchasedAirManager\.cc:2260' -Description "mapped source"
Assert-Contains -Path $cp378Module -Pattern 'PurchasedAirManager\.cc:2261' -Description "first excluded source"
Assert-ExactStringArray -Path $cp378Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER' -Expected $cp378Sites -Description "four-site source order"

# Eight inherited routes and exact active-site/owner surface.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthrough', 'HumidificationControlGuardFalseFallthrough',
        'DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp378State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'source_site_execution_count',
        'local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count',
        'local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count',
        'source_shaped_two_argument_minimum_evaluation_count',
        'purchased_air_supply_humidity_ratio_saturation_limit_assignment_count',
        'cp376_original_supply_humidity_ratio_owner_count',
        'cp377_saturation_supply_humidity_ratio_owner_count'
    )) {
    Assert-Contains -Path $cp378State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_resulting_supply_humidity_ratio_original',
        'predecessor_resulting_saturation_supply_humidity_ratio',
        'cp376_original_supply_humidity_ratio_owned_read',
        'cp377_saturation_supply_humidity_ratio_owned_read',
        'original_supply_humidity_ratio_before_saturation_limit',
        'saturation_supply_humidity_ratio_for_limit',
        'source_shaped_two_argument_minimum_evaluated',
        'minimum_supply_humidity_ratio_after_saturation_limit',
        'purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed',
        'assigned_supply_humidity_ratio', 'resulting_supply_humidity_ratio'
    )) {
    Assert-Contains -Path $cp378Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
Assert-Contains -Path $cp378Transition -Pattern 'source_shaped_two_argument_minimum\(original, saturation\)' -Description "canonical minimum"
Assert-Contains -Path $cp378Transition -Pattern 'SOURCE_ORDER\s*\.len\(\)' -Description "four-site increment"
Assert-Contains -Path $cp378Transition -Pattern 'predecessor_resulting_supply_humidity_ratio_original' -Description "CP376-through-CP377 left operand"
Assert-Contains -Path $cp378Transition -Pattern 'resulting_saturation_supply_humidity_ratio' -Description "CP377 right operand"
Assert-NotContains -Path $cp378Transition -Pattern 'f64::min|\.min\(|energyplus_psychrometric|energyplus_psy_w_fn|PsyHFnTdbW|enthalpy' -Description "alternate minimum, psychrometrics, or line-2261 work"
foreach ($path in @($cp378Transition, $cp378Release, $cp378Prefix, $cp378Runtime, $cp378Snapshot)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}

# CP377 is the sole predecessor; CP378 adds no finite-left admission gate.
foreach ($pattern in @(
        'direct_predecessor_is_retained_and_complete',
        'direct_original_owner_is_retained_and_complete',
        'cp377_snapshots_match_bit_exact',
        'completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent',
        'assignment_links_to_predecessor'
    )) {
    Assert-Contains -Path $cp378Release -Pattern $pattern -Description "predecessor proof $pattern"
}
$cp378ReleaseText = Read-RepoText -Path $cp378Release
$cp378PublicRelease = Get-Cp378RustBraceBlock -Text $cp378ReleaseText -AnchorPattern "(?m)^pub fn advance_direct_no_oa_calc_${cp378Stem}\s*\(" -Description "public direct release"
Assert-Cp378TextContains -Text $cp378PublicRelease -Pattern 'predecessor_cp377' -Description "CP377 argument"
Assert-Cp378TextNotContains -Text $cp378PublicRelease -Pattern '\.is_finite\(|f64::min|\.min\(|energyplus_psychrometric|DirectZonePurchasedAirCouplingInput' -Description "new finite gate, alternate helper, or feed"
foreach ($pattern in @('transition_count', 'source_site_execution_count', 'owner_count', 'checked_add')) {
    Assert-Contains -Path $cp378Runtime -Pattern $pattern -Description "checked state algebra $pattern"
}
foreach ($pattern in @('transition_partition', 'source_site_execution_count', 'purchased_air_supply_humidity_ratio_saturation_limit_assignment_count', 'checked_sum')) {
    Assert-Contains -Path $cp378CoupledLifecycle -Pattern $pattern -Description "coupled count validation $pattern"
    Assert-Contains -Path $cp378PipelineCounts -Pattern $pattern -Description "pipeline count validation $pattern"
}

# Registration and CP377 -> CP378 -> numerical -> reconciliation placement.
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp378CalcRoot; Pattern = $cp378Stem },
        [PSCustomObject]@{ Path = $cp378ScheduledOutput; Pattern = "pub calculation_${cp378Stem}:" },
        [PSCustomObject]@{ Path = $cp378BindingTestsRoot; Pattern = $cp378Stem },
        [PSCustomObject]@{ Path = $cp378InitState; Pattern = $cp378Stem },
        [PSCustomObject]@{ Path = $cp378InitUnit; Pattern = $cp378Stem },
        [PSCustomObject]@{ Path = $cp378WitnessRoot; Pattern = $cp378Stem },
        [PSCustomObject]@{ Path = $cp378CoupledRoot; Pattern = "mod ${cp378Stem}_validation;" },
        [PSCustomObject]@{ Path = $cp378CoupledTestsRoot; Pattern = 'coupled_runtime_tests_cp378' },
        [PSCustomObject]@{ Path = $cp378FixtureRoot; Pattern = $cp378Stem },
        [PSCustomObject]@{ Path = $cp378PipelineRoot; Pattern = $cp378PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}
$cp378BindingText = Read-RepoText -Path $cp378Binding
$cp377BindingIndexForCp378 = $cp378BindingText.IndexOf("let calculation_${cp377StemForCp378} =")
$cp378BindingIndex = $cp378BindingText.IndexOf("let calculation_${cp378Stem} ="); $cp379BindingIndex = $cp378BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndex = $cp378BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndex = $cp378BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndex = $cp378BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndex = $cp378BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp378 = $cp378BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp378 = $cp378BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp378NumericalIndex = $cp378BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
$cp378ReconcileIndex = $cp378BindingText.IndexOf("reconcile_${cp378Stem}(")
if ($cp377BindingIndexForCp378 -lt 0 -or $cp378BindingIndex -le $cp377BindingIndexForCp378 -or
    $cp379BindingIndex -le $cp378BindingIndex -or $cp380BindingIndex -le $cp379BindingIndex -or $cp381BindingIndex -le $cp380BindingIndex -or $cp382BindingIndex -le $cp381BindingIndex -or $cp383BindingIndex -le $cp382BindingIndex -or $cp384BindingIndexForCp378 -le $cp383BindingIndex -or $cp385BindingIndexForCp378 -le $cp384BindingIndexForCp378 -or $cp378NumericalIndex -le $cp385BindingIndexForCp378 -or $cp378ReconcileIndex -le $cp378NumericalIndex) {
    throw "Binding must execute CP377, CP378, CP379, CP380, unchanged numerical coupling, then CP378 humidity reconciliation"
}
$cp378Dto = Get-Cp378RustBraceBlock -Text $cp378BindingText.Substring($cp378NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp378TextNotContains -Text $cp378Dto -Pattern 'cp378|saturation_limit_assignment|minimum_supply_humidity_ratio_after_saturation_limit' -Description "numerical DTO feed"
foreach ($pattern in @('reconcile_cooling_supply_humidity_ratio_saturation_limit_assignment', 'calculation\.supply_humidity_ratio', 'supply_node_update\.humidity_ratio', 'report\.supply_humidity_ratio', 'to_bits')) {
    Assert-Contains -Path $cp378Adapter -Pattern $pattern -Description "exact-bit reconciliation $pattern"
}
Assert-NotContains -Path $cp378Adapter -Pattern 'supply_humidity_ratio\s*=|humidity_ratio\s*=' -Description "numerical overwrite"
Assert-Contains -Path $cp378BindingTests -Pattern 'reconciles_without_feeding_the_calculation' -Description "binding no-feed reconciliation regression"
Assert-Contains -Path $cp378BindingTests -Pattern 'rejects_each_corrupted_projection_without_overwriting_it' -Description "binding fail-closed reconciliation regression"
Assert-Contains -Path $cp378CoupledTests -Pattern 'reconciles_all_humidity_projections' -Description "coupled exact-bit reconciliation"

# Direct-only pipeline, IEEE serialization, and terminal arbitrary source owner.
Assert-Contains -Path $cp378PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp402_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp378PipelineRoot -Pattern $cp378Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @('json_number', 'ieee_bits', 'original_supply_humidity_ratio_before_saturation_limit_ieee_bits', 'saturation_supply_humidity_ratio_for_limit_ieee_bits', 'resulting_supply_humidity_ratio_ieee_bits')) {
    Assert-Contains -Path $cp378SnapshotSerialization -Pattern $pattern -Description "finite JSON/IEEE sidecar $pattern"
}
Assert-Contains -Path $cp378ParentAssertions -Pattern 'mod cp378_assertions;' -Description "arbitrary CP378 module"
Assert-Contains -Path $cp378ParentAssertions -Pattern 'cp378_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp378ParentAssertions -Pattern 'cp378_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp378ParentAssertions -Pattern 'assert_numerical_(?:nonfeed|reconciliation)\(' -Description "CP377 terminal ownership relinquishment"
Assert-Contains -Path $cp378Assertions -Pattern 'assert_numerical_nonfeed_and_exact_reconciliation\(' -Description "CP378 terminal no-feed reconciliation"
Assert-Contains -Path $cp378Assertions -Pattern 'mod cp379_assertions;' -Description "arbitrary CP379 module"; Assert-Contains -Path $cp378Assertions -Pattern 'cp379_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP379 direct delegation"; Assert-Contains -Path $cp378Assertions -Pattern 'cp379_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP379 non-direct delegation"; Assert-Contains -Path $cp379Assertions -Pattern 'mod cp380_assertions;' -Description "arbitrary CP380 module"; Assert-Contains -Path $cp379Assertions -Pattern 'cp380_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP380 direct delegation"; Assert-Contains -Path $cp379Assertions -Pattern 'cp380_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP380 non-direct delegation"; Assert-Contains -Path $cp380Assertions -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP380 numerical nonfeed firewall"; Assert-Contains -Path $cp380Assertions -Pattern 'mod cp381_assertions;' -Description "arbitrary CP381 module"; Assert-Contains -Path $cp380Assertions -Pattern 'cp381_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP381 direct delegation"; Assert-Contains -Path $cp380Assertions -Pattern 'cp381_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP381 non-direct delegation"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed firewall"

# Exactly two algorithm/capability addenda and five ordered handwritten sections.
$cp378AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp378CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp378AlgorithmAddenda = [regex]::Matches($cp378AlgorithmText, '(?m)^\s*"CP378 supersedes only CP377[^"\r\n]+",\s*$')
$cp378CapabilityAddenda = [regex]::Matches($cp378CapabilityText, '(?m)^\s*"CP378 additionally requires[^"\r\n]+",\s*$')
if ($cp378AlgorithmAddenda.Count -ne 2 -or $cp378CapabilityAddenda.Count -ne 2) {
    throw "CP378 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp378AlgorithmAddenda + $cp378CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp378SourceCommit, $cp378SourceHash, '2260', '2261', 'CP379',
            $cp378Sites[0], $cp378Sites[1], $cp378Sites[2], $cp378Sites[3],
            'eight', 'UnitOff', 'five', '4\*M', 'CP377', 'sole immediate predecessor',
            'source-shaped', 'right-biased', 'no finite-left gate', 'at least 1e-5',
            'terminal source owner', 'reconciles', 'never enters, feeds, overwrites, or replaces',
            'CP345', 'numerical implementation owner', 'pure/private', 'PsyHFnTdbW',
            '32 algorithms', '293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', '316 total', '240 public', '76 internal',
            'zero unused', 'zero unreachable', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP378 spec addendum missing '$pattern'" }
    }
}
$cp378Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP378 Cooling Supply-Humidity-Ratio Saturation-Limit Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP378 Source-Ordered Cooling Supply-Humidity-Ratio Saturation-Limit Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP378 Supply-Humidity-Ratio Saturation-Limit Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP378 Saturation-Limit Humidity Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP378 Saturation-Limit Humidity-Assignment Placement' }
)
foreach ($doc in $cp378Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP378 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..378) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP378 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @($cp378SourceCommit, $cp378SourceHash, '2260', '2261', $cp378Sites[0], $cp378Sites[1], $cp378Sites[2], $cp378Sites[3], 'CP377', 'CP345', 'terminal', 'reconcil', '316\s+total', '76\s+internal')) {
        if ($sections[0].Value -notmatch $required) { throw "CP378 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP378\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP378 supersedes only CP377' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP378 additionally requires' -Description "generated capability addendum"

# Historical current-state propagation, tight caps, master order, and inventory.
foreach ($historical in 334..377) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp402_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..377) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 340 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 100 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..377) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 340' -Description "historical inventory total"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_supply_enthalpy_post_saturation_assignment' -Description "historical CP379 binding order"
}
foreach ($historical in 360..377) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp379BindingIndex' -Description "historical CP379 binding index"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_supply_enthalpy_post_saturation_assignment' -Description "historical CP379 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern 'CP377-to-CP378' -Description "CP345 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP378-to-CP379' -Description "CP345 CP379 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP378-to-CP379' -Description "CP345 CP379 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP388-to-CP389' -Description "CP345 CP389 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP378-to-CP379' -Description "CP345 CP379 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP378-to-CP379' -Description "CP345 CP379 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP389-to-CP390' -Description "CP345 CP390 predecessor interval"
Assert-LineLimit -Path $cp345Audit -Limit 1200 -Description "CP345 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1" -Limit 500 -Description "CP362 historical audit"
foreach ($historical in 364, 373, 374, 376, 377) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp380_assertions\.rs' -Description "historical CP380 terminal source owner"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp382_assertions\.rs' -Description "historical CP382 arbitrary terminal"
}

$cp378MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp377AuditIndexForCp378 = $cp378MainAuditText.IndexOf("cp377-cooling-supply-humidity-ratio-saturation-assignment.ps1")
$cp378AuditIndex = $cp378MainAuditText.IndexOf("cp378-cooling-supply-humidity-ratio-saturation-limit-assignment.ps1")
$cp378CompletionIndex = $cp378MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp377AuditIndexForCp378 -lt 0 -or $cp378AuditIndex -le $cp377AuditIndexForCp378 -or $cp378CompletionIndex -le $cp378AuditIndex) {
    throw "Master audit must dot-source CP378 after CP377 before completion"
}
$cp378InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 340', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp378TextContains -Text $cp378InventoryText -Pattern $pattern -Description "inventory $pattern"
}
if ([regex]::Matches($cp378InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp378InventoryText, '(?m)^classification = "internal"$').Count -ne 100) {
throw "CP378 inventory must be exactly 240 public and 100 internal scripts"
}
Assert-Cp378TextContains -Text $cp378InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp378-' -Description "inventory record"
Assert-Cp378TextContains -Text $cp378InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 340 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 100 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated unused"

Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed firewall"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP390-to-CP391' -Description "CP345 CP390-to-CP391 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP391-to-CP392' -Description "CP345 CP391-to-CP392 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP396-to-CP397' -Description "CP345 CP396-to-CP397 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP397-to-CP398' -Description "CP345 CP397-to-CP398 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP400-to-CP401' -Description "CP345 CP400-to-CP401 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP401-to-CP402' -Description "CP345 CP401-to-CP402 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP402-to-numerical' -Description "CP345 CP402 terminal interval"
Write-Host "CP378 saturation-limit humidity-ratio assignment structure audit passed."
}
