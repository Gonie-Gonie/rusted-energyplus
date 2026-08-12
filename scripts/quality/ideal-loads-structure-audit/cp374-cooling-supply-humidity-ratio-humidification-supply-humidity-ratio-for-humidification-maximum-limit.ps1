# CP374 maps PurchasedAirManager.cc line 2250's local humidification
# supply-humidity-ratio maximum limit and no part of line 2251.
& {
$cp374Stem = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit"
$cp373StemForCp374 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment"
$cp375StemForCp374 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment"
$cp374PipelineStem = "purchased_air_$cp374Stem"
$cp374Lifecycle = "purchased_air_calc_${cp374Stem}_lifecycle"
$cp374SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp374SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp374Sites = @(
    "read-local-supply-humidity-ratio-for-humidification-for-maximum-limit-minimum",
    "read-purchased-air-maximum-heating-supply-air-humidity-ratio-for-humidification-maximum-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-humidification-maximum-limit",
    "assign-local-supply-humidity-ratio-for-humidification-for-maximum-limit"
)
$cp374Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp374Module = "crates\ep_runtime\src\ideal_loads\calc\$cp374Stem.rs"
$cp374Root = "crates\ep_runtime\src\ideal_loads\calc\$cp374Stem"
$cp374State = "$cp374Root\state.rs"
$cp374Transition = "$cp374Root\transition.rs"
$cp374Predecessor = "$cp374Root\transition\predecessor.rs"
$cp374Release = "$cp374Root\release.rs"
$cp374Operand = "$cp374Root\release\operand_validation.rs"
$cp374Prefix = "$cp374Root\release\prefix_validation.rs"
$cp374Private = "$cp374Root\release\private_counterfactual.rs"
$cp374Runtime = "$cp374Root\release\runtime_validation.rs"
$cp374Snapshot = "$cp374Root\release\snapshot_validation.rs"
$cp374SnapshotRoute = "$cp374Root\release\snapshot_validation\route.rs"
$cp374MinimumHelper = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\transition.rs"
$cp374CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp374Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp374BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp374Stem.rs"
$cp374BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp374Stem}_tests.rs"
$cp374BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp374ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp374InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp374InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp374InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp374InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp374Stem.rs"
$cp374CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp374Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp374Stem}_validation.rs"
$cp374CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp374CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp374.rs"
$cp374FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp374Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp374Stem}_fixture.rs"
$cp374PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp374Pipeline = "crates\ep_run\src\pipeline\$cp374PipelineStem.rs"
$cp374PipelineValidation = "crates\ep_run\src\pipeline\$cp374PipelineStem\validation.rs"
$cp374PipelineTests = "crates\ep_run\src\pipeline\$cp374PipelineStem\validation\tests.rs"
$cp374Serialization = "crates\ep_run\src\pipeline\$cp374PipelineStem\serialization.rs"
$cp374SnapshotSerialization = "crates\ep_run\src\pipeline\$cp374PipelineStem\serialization\snapshot.rs"
$cp374SnapshotSerializationTests = "crates\ep_run\src\pipeline\$cp374PipelineStem\serialization\snapshot\tests.rs"
$cp374ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp373_assertions.rs"
$cp374ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp374_assertions.rs"
$cp375ArbitraryAssertionsForCp374 = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp375_assertions.rs"
$cp374Audit = "scripts\quality\ideal-loads-structure-audit\cp374-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-maximum-limit.ps1"

function Assert-Cp374TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP374 $Description missing" }
}

function Assert-Cp374TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP374 $Description unexpectedly present" }
}

function Get-Cp374RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP374 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP374 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP374 $Description closing brace missing"
}

$cp374Required = @(
    $cp374Module, $cp374State, $cp374Transition, $cp374Predecessor,
    $cp374Release, $cp374Operand, $cp374Prefix, $cp374Private, $cp374Runtime,
    $cp374Snapshot, $cp374SnapshotRoute, $cp374BindingAdapter,
    $cp374BindingTests, $cp374InitWitness, $cp374Coupled, $cp374CoupledTests,
    $cp374Fixture, $cp374Pipeline, $cp374PipelineValidation,
    $cp374PipelineTests, $cp374Serialization, $cp374SnapshotSerialization,
    $cp374SnapshotSerializationTests, $cp374ArbitraryAssertions, $cp374Audit
)
foreach ($file in $cp374Required) {
    Assert-FileExists -Path $file -Description "CP374 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP374 bounded file"
}
$cp374CoreTests = @(Get-ChildItem -LiteralPath "$cp374Root\tests" -Recurse -File -Filter "*.rs")
if ($cp374CoreTests.Count -lt 4) { throw "CP374 requires bounded route/release/IEEE/overflow tests" }
$cp374CoreTestText = ($cp374CoreTests | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP374 bounded core test"
        Read-RepoText -Path $_.FullName
    }) -join "`n"
foreach ($pattern in @('source_order', 'direct', 'private', 'Humidistat', 'None', 'overflow', 'to_bits|ieee', 'transaction', 'signed_zero|signed zero|NaN')) {
    Assert-Cp374TextContains -Text $cp374CoreTestText -Pattern $pattern -Description "core test matrix '$pattern'"
}

# Locked source, exact minimum line, excluded result-store update, and dynamic continuation.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp374Source).Hash -cne $cp374SourceHash) {
    throw "CP374 PurchasedAirManager.cc SHA-256 drift"
}
$cp374Lines = Get-Content -Encoding UTF8 -LiteralPath $cp374Source
if ($cp374Lines[2249].Trim() -cne 'SupplyHumRatForHumid = min(SupplyHumRatForHumid, PurchAir.MaxHeatSuppAirHumRat);' -or
    $cp374Lines[2250].Trim() -cne 'PurchAir.SupplyHumRat = max(PurchAir.SupplyHumRat, SupplyHumRatForHumid);' -or
    $cp374Lines[2257].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP374 line 2250/2251/2258 source boundary drift"
}
Assert-Contains -Path $cp374Module -Pattern 'PurchasedAirManager\.cc:2250' -Description "CP374 mapped source"
Assert-Contains -Path $cp374Module -Pattern 'PurchasedAirManager\.cc:2251' -Description "CP374 first excluded source"
Assert-ExactStringArray -Path $cp374Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER' -Expected $cp374Sites -Description "CP374 four-site source order"

# CP373 is the sole immediate predecessor and the selected typed system owns the right operand.
foreach ($pattern in @(
        'PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot',
        'predecessor_route', 'resulting_supply_humidity_ratio_for_humidification'
    )) {
    Assert-Contains -Path $cp374Predecessor -Pattern $pattern -Description "CP373 predecessor '$pattern'"
}
foreach ($pattern in @(
        'maximum_heating_supply_air_humidity_ratio_from_selected_typed_owner',
        'system\.id != predecessor\.system', 'unit\.system != system\.id',
        'system\.maximum_heating_supply_air_humidity_ratio', '\.is_finite\(\)'
    )) {
    Assert-Contains -Path $cp374Operand -Pattern $pattern -Description "selected typed right owner '$pattern'"
}
Assert-Contains -Path $cp374Private -Pattern 'private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release' -Description "canonical CP373 private bridge"
Assert-Contains -Path $cp374Private -Pattern 'active_operands_from_selected_typed_owner' -Description "selected typed right operand"
Assert-Contains -Path $cp374Private -Pattern 'source_shaped_two_argument_minimum' -Description "canonical minimum replay"
Assert-NotContains -Path $cp374Private -Pattern 'DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|dataLoopNodes|ZoneSysEnergyDemand' -Description "private service/feed firewall"

# Both active routes execute the canonical right-biased ObjexxFCL minimum and all four sites.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthrough', 'HumidificationControlGuardFalseFallthrough',
        'DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted',
        'DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted',
        'DehumidificationControlGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp374State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'source_site_execution_count',
        'supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read_count',
        'maximum_heating_supply_air_humidity_ratio_for_minimum_read_count',
        'source_shaped_two_argument_minimum_evaluation_count',
        'supply_humidity_ratio_for_humidification_assignment_count'
    )) {
    Assert-Contains -Path $cp374State -Pattern "pub $counter\s*:\s*usize" -Description "site counter $counter"
}
Assert-Contains -Path $cp374Transition -Pattern 'cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum' -Description "CP334 canonical minimum helper"
Assert-Contains -Path $cp374Transition -Pattern 'let minimum = source_shaped_two_argument_minimum\(left, right\);' -Description "source-shaped minimum call"
Assert-Contains -Path $cp374Transition -Pattern 'SOURCE_ORDER\.len\(\)' -Description "four-site increment"
Assert-Contains -Path $cp374MinimumHelper -Pattern 'if left < right \{ left \} else \{ right \}' -Description "strict less-than right-biased helper"
Assert-NotContains -Path $cp374Transition -Pattern 'f64::min|f64::max|\.min\(|\.max\(|if left > right|\.clamp\(|Psy[A-Za-z_]*|DirectZonePurchasedAirCouplingInput' -Description "minimum/service firewall"

# Public direct release accepts CP373 only and remains a complete-null zero-site route.
$cp374ReleaseText = Read-RepoText -Path $cp374Release
$cp374PublicRelease = Get-Cp374RustBraceBlock -Text $cp374ReleaseText -AnchorPattern "(?m)^pub fn advance_direct_no_oa_calc_${cp374Stem}\s*\(" -Description "public direct release"
Assert-Cp374TextContains -Text $cp374PublicRelease -Pattern 'predecessor_cp373' -Description "public CP373 predecessor"
Assert-Cp374TextNotContains -Text $cp374PublicRelease -Pattern 'maximum_heating_supply_air_humidity_ratio:\s*f64|pre_sampled' -Description "public numeric operands"
Assert-Cp374TextContains -Text $cp374PublicRelease -Pattern '(?s)advance_.*?None' -Description "public zero-site transition"

# Registrations, CP373 -> CP374 -> numerical order, and no numerical feed.
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp374CalcRoot; Pattern = $cp374Stem },
        [PSCustomObject]@{ Path = $cp374ScheduledOutput; Pattern = "pub calculation_${cp374Stem}:" },
        [PSCustomObject]@{ Path = $cp374BindingTestsRoot; Pattern = $cp374Stem },
        [PSCustomObject]@{ Path = $cp374InitState; Pattern = $cp374Stem },
        [PSCustomObject]@{ Path = $cp374InitUnit; Pattern = $cp374Stem },
        [PSCustomObject]@{ Path = $cp374InitWitnessRoot; Pattern = $cp374Stem },
        [PSCustomObject]@{ Path = $cp374CoupledRoot; Pattern = "mod ${cp374Stem}_validation;" },
        [PSCustomObject]@{ Path = $cp374CoupledTestsRoot; Pattern = 'coupled_runtime_tests_cp374' },
        [PSCustomObject]@{ Path = $cp374FixtureRoot; Pattern = $cp374Stem },
        [PSCustomObject]@{ Path = $cp374PipelineRoot; Pattern = $cp374PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP374 registration"
}
$cp374BindingText = Read-RepoText -Path $cp374Binding
$cp373BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_${cp373StemForCp374} =")
$cp374BindingIndex = $cp374BindingText.IndexOf("let calculation_${cp374Stem} =")
$cp375BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_${cp375StemForCp374} =")
$cp376BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp374 = $cp374BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp374NumericalIndex = $cp374BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp373BindingIndexForCp374 -lt 0 -or $cp374BindingIndex -le $cp373BindingIndexForCp374 -or
    $cp375BindingIndexForCp374 -le $cp374BindingIndex -or
    $cp376BindingIndexForCp374 -le $cp375BindingIndexForCp374 -or $cp377BindingIndexForCp374 -le $cp376BindingIndexForCp374 -or $cp378BindingIndexForCp374 -le $cp377BindingIndexForCp374 -or $cp379BindingIndexForCp374 -le $cp378BindingIndexForCp374 -or $cp380BindingIndexForCp374 -le $cp379BindingIndexForCp374 -or $cp381BindingIndexForCp374 -le $cp380BindingIndexForCp374 -or $cp382BindingIndexForCp374 -le $cp381BindingIndexForCp374 -or $cp383BindingIndexForCp374 -le $cp382BindingIndexForCp374 -or $cp384BindingIndexForCp374 -le $cp383BindingIndexForCp374 -or $cp385BindingIndexForCp374 -le $cp384BindingIndexForCp374 -or $cp374NumericalIndex -le $cp385BindingIndexForCp374) {
    throw "Binding must execute CP373 then CP374 then CP375 before unchanged numerical coupling"
}
$cp374Dto = Get-Cp374RustBraceBlock -Text $cp374BindingText.Substring($cp374NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp374TextNotContains -Text $cp374Dto -Pattern 'cp374|maximum_limit|supply_humidity_ratio_for_humidification' -Description "numerical DTO feed"
Assert-Contains -Path $cp374ParentAssertions -Pattern 'mod cp374_assertions;' -Description "arbitrary CP374 module"
Assert-Contains -Path $cp374ParentAssertions -Pattern 'cp374_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp374ParentAssertions -Pattern 'cp374_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-Contains -Path $cp374ArbitraryAssertions -Pattern 'mod cp375_assertions;' -Description "CP375 arbitrary delegation module"
Assert-Contains -Path $cp374ArbitraryAssertions -Pattern 'cp375_assertions::assert_direct\(runtime, results\)' -Description "CP375 arbitrary direct delegation"
Assert-Contains -Path $cp374ArbitraryAssertions -Pattern 'cp375_assertions::assert_non_direct\(runtime\)' -Description "CP375 arbitrary non-direct delegation"
Assert-NotContains -Path $cp374ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP374 relinquishes terminal nonfeed"
Assert-NotContains -Path $cp375ArbitraryAssertionsForCp374 -Pattern 'assert_numerical_nonfeed\(' -Description "CP375 relinquishes terminal nonfeed"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp377_assertions.rs" -Pattern 'assert_numerical_nonfeed\(' -Description "CP377 relinquishes terminal numerical evidence"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp378_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_exact_reconciliation\(' -Description "CP378 terminal reconciliation"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp379_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP379 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP380 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed firewall"
Assert-Contains -Path $cp374PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp418_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp374PipelineRoot -Pattern $cp374Lifecycle -Description "pipeline lifecycle key"
Assert-Contains -Path $cp374SnapshotSerialization -Pattern 'json_number|is_finite' -Description "finite JSON projection"
Assert-Contains -Path $cp374SnapshotSerialization -Pattern '_ieee_bits' -Description "authoritative IEEE sidecars"

# Exactly two stable spec addenda and five source-ordered hand-written sections.
$cp374AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp374CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp374AlgorithmAddenda = [regex]::Matches($cp374AlgorithmText, '(?m)^\s*"CP374 supersedes only CP373[^"\r\n]+",\s*$')
$cp374CapabilityAddenda = [regex]::Matches($cp374CapabilityText, '(?m)^\s*"CP374 additionally requires[^"\r\n]+",\s*$')
if ($cp374AlgorithmAddenda.Count -ne 2 -or $cp374CapabilityAddenda.Count -ne 2) {
    throw "CP374 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp374AlgorithmAddenda + $cp374CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp374SourceCommit, $cp374SourceHash, '2250', '2251', '2258',
            'four', 'CP373', 'sole immediate predecessor',
            'maximum_heating_supply_air_humidity_ratio', 'finite', 'CP320',
            'CP334', 'CP354', 'CP356', 'CP362', 'CP361', 'ObjexxFCL',
            'right bias', 'signed zero', 'NaN', 'CP373-to-CP374-to-unchanged-numerical',
            $cp374Lifecycle, 'CP345', '32 algorithms', '293 routines',
            '58 state-mapped plus 235 source-mapped', '170 required',
            '312 total', '240 public', '72 internal', 'zero unused',
            '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP374 spec addendum missing '$pattern'" }
    }
}
$cp374Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP374 Cooling Humidification Supply-Humidity-Ratio Maximum Limit' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP374 Source-Ordered Cooling Humidification Supply-Humidity-Ratio Maximum Limit' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP374 Humidification Supply-Humidity-Ratio Maximum Limit' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP374 Humidification Supply-Humidity-Ratio Maximum Limit in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP374 Humidification Supply-Humidity-Ratio Maximum-Limit Placement' }
)
foreach ($doc in $cp374Docs) {
    $text = Read-RepoText -Path $doc.Path
    $pattern = '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)'
    $sections = [regex]::Matches($text, $pattern)
    if ($sections.Count -ne 1) { throw "CP374 documentation expected one section in $($doc.Path)" }
    $cp373HeadingIndex = $text.LastIndexOf('## CP373 ')
    if ($cp373HeadingIndex -lt 0 -or $sections[0].Index -le $cp373HeadingIndex) {
        throw "CP374 documentation must follow CP373 in $($doc.Path)"
    }
    foreach ($required in @(
            $cp374SourceCommit, $cp374SourceHash, '2250', '2251', '2258',
            'CP373', 'four', 'right', 'CP345', '312\s+total', '72\s+internal'
        )) {
        if ($sections[0].Value -notmatch $required) { throw "CP374 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP374\b' -Description "CP374 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP374 supersedes only CP373' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP374 additionally requires' -Description "generated capability addendum"

# Historical terminal expectations and current generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..371 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp374Stem" -Description "historical CP374 binding order"
}
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp372-cooling-supply-humidity-ratio-humidification-moisture-demand-assignment.ps1" -Pattern 'cp374StemForCp372' -Description "CP372 CP374 binding order"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp373-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-assignment.ps1" -Pattern 'cp374StemForCp373' -Description "CP373 CP374 binding order"
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp374Stem" -Description "historical CP374 helper whitelist"
}
foreach ($historical in 334..373) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp418_lifecycle_evidence' -Description "historical CP374 firewall"
}
foreach ($historical in 335..373) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 356 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 116 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..373) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 356' -Description "historical inventory total"
}
$cp374MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp373AuditIndexForCp374 = $cp374MainAuditText.IndexOf("cp373-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-assignment.ps1")
$cp374AuditIndex = $cp374MainAuditText.IndexOf("cp374-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-maximum-limit.ps1")
$cp375AuditIndexForCp374 = $cp374MainAuditText.IndexOf("cp375-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-maximum-assignment.ps1")
$cp374CompletionIndex = $cp374MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp373AuditIndexForCp374 -lt 0 -or $cp374AuditIndex -le $cp373AuditIndexForCp374 -or
    $cp375AuditIndexForCp374 -le $cp374AuditIndex -or
    $cp374CompletionIndex -le $cp375AuditIndexForCp374) {
    throw "Master audit must dot-source CP375 after CP374 and CP373 before completion"
}
$cp374InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp374TextContains -Text $cp374InventoryText -Pattern 'script_count = 356' -Description "script total"
Assert-Cp374TextContains -Text $cp374InventoryText -Pattern 'dev_command_count = 238' -Description "stable dev-command total"
Assert-Cp374TextContains -Text $cp374InventoryText -Pattern 'unused_script_count = 0' -Description "zero unused"
if ([regex]::Matches($cp374InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp374InventoryText, '(?m)^classification = "internal"$').Count -ne 116) {
throw "CP374 inventory must be exactly 240 public and 116 internal scripts"
}
Assert-Cp374TextContains -Text $cp374InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp374-' -Description "inventory record"
Assert-Cp374TextContains -Text $cp374InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 356 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 116 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated unused"

Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed firewall"
Write-Host "CP374 Cooling humidification supply-humidity-ratio maximum-limit structure audit passed."
}
