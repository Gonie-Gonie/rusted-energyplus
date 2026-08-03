# CP379 maps only PurchasedAirManager.cc physical executable line 2261's
# post-saturation local supply-enthalpy assignment, without numerical reconciliation.
& {
$cp379Stem = "cooling_supply_enthalpy_post_saturation_assignment"
$cp378StemForCp379 = "cooling_supply_humidity_ratio_saturation_limit_assignment"
$cp379PipelineStem = "purchased_air_$cp379Stem"
$cp379Lifecycle = "purchased_air_calc_$($cp379Stem)_lifecycle"
$cp379SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp379SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp379Sites = @(
    "read-purchased-air-supply-temperature-for-post-saturation-enthalpy",
    "read-purchased-air-supply-humidity-ratio-for-post-saturation-enthalpy",
    "evaluate-psy-h-fn-tdb-w-for-post-saturation-enthalpy",
    "assign-local-supply-enthalpy-after-saturation-limit"
)
$cp379Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp379Module = "crates\ep_runtime\src\ideal_loads\calc\$cp379Stem.rs"
$cp379Root = "crates\ep_runtime\src\ideal_loads\calc\$cp379Stem"
$cp379State = "$cp379Root\state.rs"
$cp379Transition = "$cp379Root\transition.rs"
$cp379Release = "$cp379Root\release.rs"
$cp379CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp379Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp379Adapter = "crates\ep_runtime\src\ideal_loads\binding\$cp379Stem.rs"
$cp379BindingTests = "crates\ep_runtime\src\ideal_loads\binding\$($cp379Stem)_tests.rs"
$cp379ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp379InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp379InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp379WitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp379Witness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp379Stem.rs"
$cp379CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp379Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp379Stem)_validation.rs"
$cp379CoupledLifecycle = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp379Stem)_validation\lifecycle.rs"
$cp379CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($cp379Stem)_validation\snapshot.rs"
$cp379FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp379Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\$($cp379Stem)_fixture.rs"
$cp379PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp379Pipeline = "crates\ep_run\src\pipeline\$cp379PipelineStem.rs"
$cp379PipelineValidation = "crates\ep_run\src\pipeline\$cp379PipelineStem\validation.rs"
$cp379PipelineCounts = "crates\ep_run\src\pipeline\$cp379PipelineStem\validation\counts.rs"
$cp379Serialization = "crates\ep_run\src\pipeline\$cp379PipelineStem\serialization.rs"
$cp379SnapshotSerialization = "crates\ep_run\src\pipeline\$cp379PipelineStem\serialization\snapshot.rs"
$cp378Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp378_assertions.rs"
$cp379Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp379_assertions.rs"; $cp380Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs"; $cp381Assertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp381_assertions.rs"
$cp379Audit = "scripts\quality\ideal-loads-structure-audit\cp379-cooling-supply-enthalpy-post-saturation-assignment.ps1"

function Assert-Cp379TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP379 $Description missing" }
}

function Assert-Cp379TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP379 $Description unexpectedly present" }
}

function Get-Cp379RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP379 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP379 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP379 $Description closing brace missing"
}

$cp379Required = @(
    $cp379Module, $cp379State, $cp379Transition, $cp379Release, $cp379Adapter,
    $cp379BindingTests, $cp379Witness, $cp379Coupled, $cp379CoupledLifecycle,
    $cp379CoupledSnapshot, $cp379Fixture, $cp379Pipeline, $cp379PipelineValidation,
    $cp379PipelineCounts, $cp379Serialization, $cp379SnapshotSerialization,
    $cp378Assertions, $cp379Assertions, $cp379Audit
)
foreach ($file in $cp379Required) {
    Assert-FileExists -Path $file -Description "CP379 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP379 bounded file"
}
$cp379CoreFiles = @(Get-ChildItem -LiteralPath $cp379Root -Recurse -File -Filter "*.rs")
$cp379CoreText = ($cp379CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP379 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join [Environment]::NewLine

# Exact line-2261 boundary and unique four-site source order.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp379Source).Hash -cne $cp379SourceHash) {
    throw "CP379 PurchasedAirManager.cc SHA-256 drift"
}
$cp379Lines = Get-Content -Encoding UTF8 -LiteralPath $cp379Source
if ($cp379Lines[2260].Trim() -cne 'SupplyEnthalpy = PsyHFnTdbW(PurchAir.SupplyTemp, PurchAir.SupplyHumRat);' -or
    $cp379Lines[2261].Trim() -cne '' -or
    $cp379Lines[2262].Trim() -cne '// Check max total Cooling capacity, if specified' -or
    $cp379Lines[2263].Trim() -cne 'if ((PurchAir.CoolingLimit == LimitType::Capacity) || (PurchAir.CoolingLimit == LimitType::FlowRateAndCapacity)) {') {
    throw "CP379 line 2261 through first-excluded executable 2264 source boundary drift"
}
Assert-Contains -Path $cp379Module -Pattern 'PurchasedAirManager\.cc:2261' -Description "mapped source"
Assert-Contains -Path $cp379Module -Pattern 'PurchasedAirManager\.cc:2264' -Description "first excluded source"
Assert-ExactStringArray -Path $cp379Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER' -Expected $cp379Sites -Description "four-site source order"

# Eight inherited routes and exact active-site/owner surface.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthrough', 'HumidificationControlGuardFalseFallthrough',
        'DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp379State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'source_site_execution_count',
        'purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count',
        'purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count',
        'psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count',
        'local_supply_enthalpy_after_saturation_limit_assignment_count',
        'cp334_supply_temperature_mixed_air_limit_owner_count',
        'cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count',
        'cp378_supply_humidity_ratio_saturation_limit_owner_count'
    )) {
    Assert-Contains -Path $cp379State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'predecessor_resulting_supply_humidity_ratio',
        'cp377_supply_temperature_owned_read',
        'cp334_supply_temperature_mixed_air_limit_owned_read',
        'cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read',
        'cp378_supply_humidity_ratio_saturation_limit_owned_read',
        'purchased_air_supply_temperature_for_post_saturation_enthalpy_read',
        'supply_temperature_c',
        'purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read',
        'supply_humidity_ratio',
        'psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated',
        'psychrometric_supply_enthalpy_j_per_kg',
        'local_supply_enthalpy_after_saturation_limit_assignment_performed',
        'assigned_supply_enthalpy_j_per_kg', 'resulting_supply_enthalpy_j_per_kg'
    )) {
    Assert-Contains -Path $cp379Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}

# Canonical psychrometric evaluation and CP378/CP377 retained ownership only.
foreach ($pattern in @(
        'energyplus_psy_h_fn_tdb_w',
        'predecessor\.resulting_supply_humidity_ratio',
        'supply_temperature_c',
        'SOURCE_ORDER\s*\.len\(\)',
        'cp378_supply_humidity_ratio_saturation_limit_owner_count'
    )) {
    Assert-Contains -Path $cp379Transition -Pattern $pattern -Description "transition contract $pattern"
}
Assert-NotContains -Path $cp379Transition -Pattern 'moist_air_enthalpy_j_per_kg|energyplus_psy_h_fn_tdb_w_(?:fast|raw)|mul_add|DirectZonePurchasedAirCouplingInput|f64::max' -Description "legacy/alternate psychrometrics or DTO feed"
foreach ($path in @($cp379Transition, $cp379Release, $cp379Adapter, $cp379Coupled)) {
    Assert-NotContains -Path $path -Pattern '\.unwrap\s*\(|\.expect\s*\(|\bpanic!\s*\(' -Description "production source-quality violation"
}
foreach ($pattern in @(
        'completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent',
        'predecessor_cp378',
        'snapshots_match_bit_exact',
        'resulting_supply_humidity_ratio',
        'is_finite'
    )) {
    Assert-Cp379TextContains -Text $cp379CoreText -Pattern $pattern -Description "release predecessor/admission proof $pattern"
}
Assert-NotContains -Path $cp379Adapter -Pattern 'DirectZonePurchasedAirCouplingInput|reconcile_|supply_node_update|report' -Description "adapter numerical/node/report reconciliation"

# CP378 -> CP379 -> CP380 -> unchanged numerical placement, with no CP379 DTO field.
$cp379BindingText = Read-RepoText -Path $cp379Binding
$cp378BindingIndexForCp379 = $cp379BindingText.IndexOf("let calculation_$cp378StemForCp379 =")
$cp379BindingIndex = $cp379BindingText.IndexOf("let calculation_$cp379Stem ="); $cp380BindingIndexForCp379 = $cp379BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp379 = $cp379BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp379 = $cp379BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp379 = $cp379BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp379 = $cp379BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp379 = $cp379BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp379NumericalIndex = $cp379BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp378BindingIndexForCp379 -lt 0 -or $cp379BindingIndex -le $cp378BindingIndexForCp379 -or $cp380BindingIndexForCp379 -le $cp379BindingIndex -or
    $cp381BindingIndexForCp379 -le $cp380BindingIndexForCp379 -or $cp382BindingIndexForCp379 -le $cp381BindingIndexForCp379 -or $cp383BindingIndexForCp379 -le $cp382BindingIndexForCp379 -or $cp384BindingIndexForCp379 -le $cp383BindingIndexForCp379 -or $cp385BindingIndexForCp379 -le $cp384BindingIndexForCp379 -or $cp379NumericalIndex -le $cp385BindingIndexForCp379) {
    throw "Binding must execute CP378, CP379, CP380, then unchanged numerical coupling"
}
$cp379Dto = Get-Cp379RustBraceBlock -Text $cp379BindingText.Substring($cp379NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp379TextNotContains -Text $cp379Dto -Pattern 'cp379|post_saturation|psychrometric_supply_enthalpy|assigned_supply_enthalpy' -Description "numerical DTO feed"
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp379CalcRoot; Pattern = $cp379Stem },
        [PSCustomObject]@{ Path = $cp379ScheduledOutput; Pattern = "pub calculation_$($cp379Stem):" },
        [PSCustomObject]@{ Path = $cp379InitState; Pattern = $cp379Stem },
        [PSCustomObject]@{ Path = $cp379InitUnit; Pattern = $cp379Stem },
        [PSCustomObject]@{ Path = $cp379WitnessRoot; Pattern = $cp379Stem },
        [PSCustomObject]@{ Path = $cp379CoupledRoot; Pattern = "mod $($cp379Stem)_validation;" },
        [PSCustomObject]@{ Path = $cp379FixtureRoot; Pattern = $cp379Stem },
        [PSCustomObject]@{ Path = $cp379PipelineRoot; Pattern = $cp379PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}

# Direct-only lifecycle/serialization and arbitrary nonfeed firewall.
Assert-Contains -Path $cp379PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp410_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp379PipelineRoot -Pattern $cp379Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @('json_number', 'ieee_bits', 'supply_temperature_c_ieee_bits', 'supply_humidity_ratio_ieee_bits', 'resulting_supply_enthalpy_j_per_kg_ieee_bits')) {
    Assert-Contains -Path $cp379SnapshotSerialization -Pattern $pattern -Description "finite JSON/IEEE sidecar $pattern"
}
Assert-Contains -Path $cp378Assertions -Pattern 'mod cp379_assertions;' -Description "arbitrary CP379 module"
Assert-Contains -Path $cp378Assertions -Pattern 'cp379_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP379 direct delegation"
Assert-Contains -Path $cp378Assertions -Pattern 'cp379_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP379 non-direct delegation"
Assert-Contains -Path $cp378Assertions -Pattern 'assert_numerical_nonfeed_and_exact_reconciliation\(' -Description "CP378 humidity reconciliation retention"
Assert-Contains -Path $cp379Assertions -Pattern 'mod cp380_assertions;' -Description "arbitrary CP380 module"; Assert-Contains -Path $cp379Assertions -Pattern 'cp380_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP380 direct delegation"; Assert-Contains -Path $cp379Assertions -Pattern 'cp380_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP380 non-direct delegation"; Assert-Contains -Path $cp380Assertions -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP380 numerical nonfeed firewall"; Assert-Contains -Path $cp380Assertions -Pattern 'mod cp381_assertions;' -Description "arbitrary CP381 module"; Assert-Contains -Path $cp380Assertions -Pattern 'cp381_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP381 direct delegation"; Assert-Contains -Path $cp380Assertions -Pattern 'cp381_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP381 non-direct delegation"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed firewall"
Assert-NotContains -Path $cp379Assertions -Pattern 'System Node Enthalpy|supply_node.*enthalpy|report.*supply_enthalpy|calculation.*supply_enthalpy' -Description "CP379 numerical/node/report enthalpy comparison"

# Exactly two algorithm/capability addenda and five ordered handwritten sections.
$cp379AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp379CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp379AlgorithmAddenda = [regex]::Matches($cp379AlgorithmText, '(?m)^\s*"CP379 supersedes only CP378[^"\r\n]+",\s*$')
$cp379CapabilityAddenda = [regex]::Matches($cp379CapabilityText, '(?m)^\s*"CP379 additionally requires[^"\r\n]+",\s*$')
if ($cp379AlgorithmAddenda.Count -ne 2 -or $cp379CapabilityAddenda.Count -ne 2) {
    throw "CP379 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp379AlgorithmAddenda + $cp379CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp379SourceCommit, $cp379SourceHash, '2261', '2264', 'CP380',
            $cp379Sites[0], $cp379Sites[1], $cp379Sites[2], $cp379Sites[3],
            'eight', 'UnitOff', 'five', '4\*E', 'CP378', 'sole immediate predecessor',
            'energyplus_psy_h_fn_tdb_w', 'max\(W, 1\.0e-5\)', 'not a terminal',
            'no bit-exact equivalence', 'never enters, consumes, feeds, reconciles with, overwrites, or replaces',
            'routine\.psy_h_fn_tdb_w', 'state_mapped', '32 algorithms', '293 routines',
            '58 state-mapped plus 235 source-mapped', '170 required', '317 total',
            '240 public', '77 internal', 'zero unused', 'zero unreachable',
            '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP379 spec addendum missing '$pattern'" }
    }
}
$cp379Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP379 Cooling Supply-Enthalpy Post-Saturation Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP379 Source-Ordered Cooling Supply-Enthalpy Post-Saturation Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP379 Supply-Enthalpy Post-Saturation Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP379 Post-Saturation Supply-Enthalpy Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP379 Post-Saturation Supply-Enthalpy Assignment Placement' }
)
foreach ($doc in $cp379Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP379 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..379) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP379 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @($cp379SourceCommit, $cp379SourceHash, '2261', '2264', 'CP380', $cp379Sites[0], $cp379Sites[1], $cp379Sites[2], $cp379Sites[3], 'CP378', 'routine\.psy_h_fn_tdb_w', 'neither\s+terminal|not\s+(?:a\s+)?terminal|not\s+final', 'reconcil', '317\s+total', '77\s+internal')) {
        if ($sections[0].Value -notmatch $required) { throw "CP379 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP379\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP379 supersedes only CP378' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP379 additionally requires' -Description "generated capability addendum"

# Historical current-state propagation while CP378 checkpoint numbers stay historical.
foreach ($historical in 334..378) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp410_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..378) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 348 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 108 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..378) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 348' -Description "historical inventory total"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp379Stem" -Description "historical CP379 binding order"
}
foreach ($historical in 360..378) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp379BindingIndex' -Description "historical CP379 binding index"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp379Stem" -Description "historical CP379 helper whitelist"
}
$cp345Audit = "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1"
Assert-Contains -Path $cp345Audit -Pattern 'CP378-to-CP379' -Description "CP345 predecessor interval"
Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP388-to-CP389' -Description "CP345 CP389 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path $cp345Audit -Pattern 'CP389-to-CP390' -Description "CP345 CP390 predecessor interval"
Assert-LineLimit -Path $cp345Audit -Limit 1200 -Description "CP345 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1" -Limit 500 -Description "CP362 historical audit"
foreach ($historical in 364, 373, 374, 376, 377, 378) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp380_assertions\.rs' -Description "historical CP380 arbitrary terminal"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp382_assertions\.rs' -Description "historical CP382 arbitrary terminal"
}
$cp378HistoricalAlgorithm = [regex]::Matches($cp379AlgorithmText, '(?m)^\s*"CP378 supersedes only CP377[^"\r\n]+",\s*$')
$cp378HistoricalCapability = [regex]::Matches($cp379CapabilityText, '(?m)^\s*"CP378 additionally requires[^"\r\n]+",\s*$')
if ($cp378HistoricalAlgorithm.Count -ne 2 -or $cp378HistoricalCapability.Count -ne 2) {
    throw "CP378 historical addenda count drift"
}
foreach ($claim in @($cp378HistoricalAlgorithm + $cp378HistoricalCapability)) {
    if ($claim.Value -notmatch '316 total' -or $claim.Value -notmatch '76 internal') {
        throw "CP378 historical addendum inventory numbers must remain 316/76"
    }
}

# Master order and current generated inventory.
$cp379MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp378AuditIndexForCp379 = $cp379MainAuditText.IndexOf("cp378-cooling-supply-humidity-ratio-saturation-limit-assignment.ps1")
$cp379AuditIndex = $cp379MainAuditText.IndexOf("cp379-cooling-supply-enthalpy-post-saturation-assignment.ps1")
$cp379CompletionIndex = $cp379MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp378AuditIndexForCp379 -lt 0 -or $cp379AuditIndex -le $cp378AuditIndexForCp379 -or $cp379CompletionIndex -le $cp379AuditIndex) {
    throw "Master audit must dot-source CP379 after CP378 before completion"
}
$cp379InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
foreach ($pattern in @('script_count = 348', 'dev_command_count = 238', 'unused_script_count = 0', 'unreachable_count = 0')) {
    Assert-Cp379TextContains -Text $cp379InventoryText -Pattern $pattern -Description "inventory $pattern"
}
if ([regex]::Matches($cp379InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp379InventoryText, '(?m)^classification = "internal"$').Count -ne 108) {
throw "CP379 inventory must be exactly 240 public and 106 internal scripts"
}
Assert-Cp379TextContains -Text $cp379InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp379-' -Description "inventory record"
Assert-Cp379TextContains -Text $cp379InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 348 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 108 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated unused"

Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed firewall"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP390-to-CP391' -Description "CP345 CP390-to-CP391 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP391-to-CP392' -Description "CP345 CP391-to-CP392 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP396-to-CP397' -Description "CP345 CP396-to-CP397 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP397-to-CP398' -Description "CP345 CP397-to-CP398 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP400-to-CP401' -Description "CP345 CP400-to-CP401 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP401-to-CP402' -Description "CP345 CP401-to-CP402 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP402-to-CP403' -Description "CP345 CP402 terminal interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP403-to-CP404' -Description "CP345 CP402 terminal interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP404-to-CP405' -Description "CP345 CP404-to-CP405 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP405-to-CP406' -Description "CP345 CP405-to-CP406 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP406-to-CP407' -Description "CP345 CP406-to-CP407 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP407-to-CP408' -Description "CP345 CP407-to-CP408 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP408-to-CP409' -Description "CP345 CP408-to-CP409 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Write-Host "CP379 post-saturation supply-enthalpy assignment structure audit passed."
}
