# CP373 maps PurchasedAirManager.cc line 2249's local humidification
# supply-humidity-ratio arithmetic assignment and no part of line 2250.
& {
$cp373Stem = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment"
$cp372StemForCp373 = "cooling_supply_humidity_ratio_humidification_moisture_demand_assignment"
$cp374StemForCp373 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit"
$cp375StemForCp373 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment"
$cp373PipelineStem = "purchased_air_$cp373Stem"
$cp373Lifecycle = "purchased_air_calc_${cp373Stem}_lifecycle"
$cp373SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp373SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp373Sites = @(
    "read-local-zone-humidifying-setpoint-moisture-demand-for-supply-humidity-ratio-division",
    "read-retained-supply-mass-flow-rate-for-supply-humidity-ratio-division",
    "calculate-zone-humidifying-setpoint-moisture-demand-divided-by-supply-mass-flow-rate",
    "read-zone-node-humidity-ratio-for-humidification-supply-humidity-ratio",
    "add-zone-node-humidity-ratio-to-moisture-demand-derived-supply-humidity-ratio",
    "assign-local-supply-humidity-ratio-for-humidification"
)
$cp373Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp373Module = "crates\ep_runtime\src\ideal_loads\calc\$cp373Stem.rs"
$cp373Root = "crates\ep_runtime\src\ideal_loads\calc\$cp373Stem"
$cp373State = "$cp373Root\state.rs"
$cp373Transition = "$cp373Root\transition.rs"
$cp373Predecessor = "$cp373Root\transition\predecessor.rs"
$cp373Release = "$cp373Root\release.rs"
$cp373Operand = "$cp373Root\release\operand_validation.rs"
$cp373Prefix = "$cp373Root\release\prefix_validation.rs"
$cp373Private = "$cp373Root\release\private_counterfactual.rs"
$cp373Runtime = "$cp373Root\release\runtime_validation.rs"
$cp373Snapshot = "$cp373Root\release\snapshot_validation.rs"
$cp373SnapshotRoute = "$cp373Root\release\snapshot_validation\route.rs"
$cp373CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp373Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp373BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp373Stem.rs"
$cp373BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp373Stem}_tests.rs"
$cp373BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp373ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp373InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp373InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp373InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp373InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp373Stem.rs"
$cp373CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp373Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp373Stem}_validation.rs"
$cp373CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp373CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp373.rs"
$cp373FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp373Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp373Stem}_fixture.rs"
$cp373PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp373Pipeline = "crates\ep_run\src\pipeline\$cp373PipelineStem.rs"
$cp373PipelineValidation = "crates\ep_run\src\pipeline\$cp373PipelineStem\validation.rs"
$cp373PipelineTests = "crates\ep_run\src\pipeline\$cp373PipelineStem\validation\tests.rs"
$cp373Serialization = "crates\ep_run\src\pipeline\$cp373PipelineStem\serialization.rs"
$cp373SnapshotSerialization = "crates\ep_run\src\pipeline\$cp373PipelineStem\serialization\snapshot.rs"
$cp373ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp372_assertions.rs"
$cp373ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp373_assertions.rs"
$cp374ArbitraryAssertionsForCp373 = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp374_assertions.rs"
$cp375ArbitraryAssertionsForCp373 = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp375_assertions.rs"
$cp373Audit = "scripts\quality\ideal-loads-structure-audit\cp373-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-assignment.ps1"

function Assert-Cp373TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP373 $Description missing" }
}

function Assert-Cp373TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP373 $Description unexpectedly present" }
}

function Get-Cp373RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP373 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP373 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP373 $Description closing brace missing"
}

$cp373Required = @(
    $cp373Module, $cp373State, $cp373Transition, $cp373Predecessor,
    $cp373Release, $cp373Operand, $cp373Prefix, $cp373Private, $cp373Runtime,
    $cp373Snapshot, $cp373SnapshotRoute, $cp373BindingAdapter,
    $cp373BindingTests, $cp373InitWitness, $cp373Coupled, $cp373CoupledTests,
    $cp373Fixture, $cp373Pipeline, $cp373PipelineValidation,
    $cp373PipelineTests, $cp373Serialization, $cp373SnapshotSerialization,
    $cp373ArbitraryAssertions, $cp373Audit
)
foreach ($file in $cp373Required) {
    Assert-FileExists -Path $file -Description "CP373 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP373 bounded file"
}
$cp373CoreTests = @(Get-ChildItem -LiteralPath "$cp373Root\tests" -Recurse -File -Filter "*.rs")
if ($cp373CoreTests.Count -lt 4) { throw "CP373 requires bounded route/release/IEEE/overflow tests" }
$cp373CoreTestText = ($cp373CoreTests | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP373 bounded core test"
        Read-RepoText -Path $_.FullName
    }) -join "`n"
foreach ($pattern in @('source_order', 'direct', 'private', 'Humidistat', 'None', 'overflow', 'to_bits|ieee', 'transaction')) {
    Assert-Cp373TextContains -Text $cp373CoreTestText -Pattern $pattern -Description "core test matrix '$pattern'"
}

# Locked source, exact arithmetic line, excluded clamp, and dynamic continuation.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp373Source).Hash -cne $cp373SourceHash) {
    throw "CP373 PurchasedAirManager.cc SHA-256 drift"
}
$cp373Lines = Get-Content -Encoding UTF8 -LiteralPath $cp373Source
if ($cp373Lines[2248].Trim() -cne 'SupplyHumRatForHumid = MdotZnHumidSP / SupplyMassFlowRate + state.dataLoopNodes->Node(ZoneNodeNum).HumRat;' -or
    $cp373Lines[2249].Trim() -cne 'SupplyHumRatForHumid = min(SupplyHumRatForHumid, PurchAir.MaxHeatSuppAirHumRat);' -or
    $cp373Lines[2257].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP373 line 2249/2250/2258 source boundary drift"
}
Assert-Contains -Path $cp373Module -Pattern 'PurchasedAirManager\.cc:2249' -Description "CP373 mapped source"
Assert-Contains -Path $cp373Module -Pattern 'PurchasedAirManager\.cc:2250' -Description "CP373 first excluded source"
Assert-ExactStringArray -Path $cp373Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER' -Expected $cp373Sites -Description "CP373 six-site source order"

# CP372 is the only predecessor; CP330 owns the denominator; Zone humidity is pre-sampled.
foreach ($pattern in @(
        'PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot',
        'predecessor_route', 'resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s'
    )) {
    Assert-Contains -Path $cp373Predecessor -Pattern $pattern -Description "CP372 predecessor '$pattern'"
}
foreach ($pattern in @(
        'cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release',
        'completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent',
        'partial_cmp', 'Ordering::Greater'
    )) {
    Assert-Contains -Path $cp373Operand -Pattern $pattern -Description "CP330 denominator owner '$pattern'"
}
Assert-Contains -Path $cp373Private -Pattern 'private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release' -Description "canonical CP372 private bridge"
Assert-Contains -Path $cp373Private -Pattern 'pre_sampled_zone_node_humidity_ratio:\s*f64' -Description "pre-sampled Zone-node humidity"
Assert-Contains -Path $cp373Private -Pattern 'supply_mass_flow_rate_from_retained_owner' -Description "retained CP330 flow use"
Assert-NotContains -Path $cp373Private -Pattern 'DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|selected_typed.*humidity|dataLoopNodes|ZoneSysEnergyDemand' -Description "private service/feed firewall"

# Two active routes execute raw division then addition and all six sites.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthrough', 'HumidificationControlGuardFalseFallthrough',
        'DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted',
        'DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted',
        'DehumidificationControlGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp373State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'source_site_execution_count', 'zone_humidifying_setpoint_moisture_demand_read_count',
        'supply_mass_flow_rate_read_count', 'moisture_demand_derived_supply_humidity_ratio_calculation_count',
        'zone_node_humidity_ratio_read_count', 'supply_humidity_ratio_for_humidification_calculation_count',
        'supply_humidity_ratio_for_humidification_assignment_count'
    )) {
    Assert-Contains -Path $cp373State -Pattern "pub $counter\s*:\s*usize" -Description "site counter $counter"
}
Assert-Contains -Path $cp373Transition -Pattern 'let quotient = demand / operands\.supply_mass_flow_rate_kg_per_s;' -Description "raw division"
Assert-Contains -Path $cp373Transition -Pattern 'let calculated = quotient \+ operands\.zone_node_humidity_ratio;' -Description "ordered addition"
Assert-Contains -Path $cp373Transition -Pattern 'SOURCE_ORDER\.len\(\)' -Description "six-site increment"
Assert-NotContains -Path $cp373Transition -Pattern 'mul_add|recip|\.is_finite\(\)|\.clamp\(|f64::min|f64::max|Psy[A-Za-z_]*|DirectZonePurchasedAirCouplingInput' -Description "arithmetic/service firewall"

# Public direct release accepts CP372 only and keeps the current zero-site route.
$cp373ReleaseText = Read-RepoText -Path $cp373Release
$cp373PublicRelease = Get-Cp373RustBraceBlock -Text $cp373ReleaseText -AnchorPattern "(?m)^pub fn advance_direct_no_oa_calc_${cp373Stem}\s*\(" -Description "public direct release"
Assert-Cp373TextContains -Text $cp373PublicRelease -Pattern 'predecessor_cp372' -Description "public CP372 predecessor"
Assert-Cp373TextNotContains -Text $cp373PublicRelease -Pattern 'pre_sampled|zone_node_humidity_ratio:\s*f64|supply_mass_flow_rate_kg_per_s:\s*f64' -Description "public numeric operands"
Assert-Cp373TextContains -Text $cp373PublicRelease -Pattern '(?s)advance_.*?None' -Description "public zero-site transition"

# Registrations, CP372 -> CP373 -> numerical order, and no numerical feed.
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp373CalcRoot; Pattern = $cp373Stem },
        [PSCustomObject]@{ Path = $cp373ScheduledOutput; Pattern = "pub calculation_${cp373Stem}:" },
        [PSCustomObject]@{ Path = $cp373BindingTestsRoot; Pattern = $cp373Stem },
        [PSCustomObject]@{ Path = $cp373InitState; Pattern = $cp373Stem },
        [PSCustomObject]@{ Path = $cp373InitUnit; Pattern = $cp373Stem },
        [PSCustomObject]@{ Path = $cp373InitWitnessRoot; Pattern = $cp373Stem },
        [PSCustomObject]@{ Path = $cp373CoupledRoot; Pattern = "mod ${cp373Stem}_validation;" },
        [PSCustomObject]@{ Path = $cp373CoupledTestsRoot; Pattern = 'coupled_runtime_tests_cp373' },
        [PSCustomObject]@{ Path = $cp373FixtureRoot; Pattern = $cp373Stem },
        [PSCustomObject]@{ Path = $cp373PipelineRoot; Pattern = $cp373PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP373 registration"
}
$cp373BindingText = Read-RepoText -Path $cp373Binding
$cp372BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_${cp372StemForCp373} =")
$cp373BindingIndex = $cp373BindingText.IndexOf("let calculation_${cp373Stem} =")
$cp374BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_${cp374StemForCp373} =")
$cp375BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_${cp375StemForCp373} =")
$cp376BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp373 = $cp373BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp373NumericalIndex = $cp373BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp372BindingIndexForCp373 -lt 0 -or $cp373BindingIndex -le $cp372BindingIndexForCp373 -or
    $cp374BindingIndexForCp373 -le $cp373BindingIndex -or
    $cp375BindingIndexForCp373 -le $cp374BindingIndexForCp373 -or
    $cp376BindingIndexForCp373 -le $cp375BindingIndexForCp373 -or $cp377BindingIndexForCp373 -le $cp376BindingIndexForCp373 -or $cp378BindingIndexForCp373 -le $cp377BindingIndexForCp373 -or $cp379BindingIndexForCp373 -le $cp378BindingIndexForCp373 -or $cp380BindingIndexForCp373 -le $cp379BindingIndexForCp373 -or $cp381BindingIndexForCp373 -le $cp380BindingIndexForCp373 -or $cp382BindingIndexForCp373 -le $cp381BindingIndexForCp373 -or $cp383BindingIndexForCp373 -le $cp382BindingIndexForCp373 -or $cp384BindingIndexForCp373 -le $cp383BindingIndexForCp373 -or $cp385BindingIndexForCp373 -le $cp384BindingIndexForCp373 -or $cp373NumericalIndex -le $cp385BindingIndexForCp373) {
    throw "Binding must execute CP372 then CP373 then CP374 then CP375 before unchanged numerical coupling"
}
$cp373Dto = Get-Cp373RustBraceBlock -Text $cp373BindingText.Substring($cp373NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp373TextNotContains -Text $cp373Dto -Pattern 'cp373|cp374|supply_humidity_ratio_for_humidification|zone_node_humidity_ratio' -Description "numerical DTO feed"
Assert-Contains -Path $cp373ParentAssertions -Pattern 'mod cp373_assertions;' -Description "arbitrary CP373 module"
Assert-Contains -Path $cp373ParentAssertions -Pattern 'cp373_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp373ParentAssertions -Pattern 'cp373_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-Contains -Path $cp373ArbitraryAssertions -Pattern 'mod cp374_assertions;' -Description "CP374 arbitrary delegation module"
Assert-Contains -Path $cp373ArbitraryAssertions -Pattern 'cp374_assertions::assert_direct\(runtime, results\)' -Description "CP374 arbitrary direct delegation"
Assert-Contains -Path $cp373ArbitraryAssertions -Pattern 'cp374_assertions::assert_non_direct\(runtime\)' -Description "CP374 arbitrary non-direct delegation"
Assert-Cp373TextNotContains -Text (Read-RepoText -Path $cp373ArbitraryAssertions) -Pattern 'assert_numerical_nonfeed\(' -Description "CP373 terminal nonfeed"
Assert-Contains -Path $cp374ArbitraryAssertionsForCp373 -Pattern 'mod cp375_assertions;' -Description "CP375 arbitrary delegation module"
Assert-Contains -Path $cp374ArbitraryAssertionsForCp373 -Pattern 'cp375_assertions::assert_direct\(runtime, results\)' -Description "CP375 arbitrary direct delegation"
Assert-Contains -Path $cp374ArbitraryAssertionsForCp373 -Pattern 'cp375_assertions::assert_non_direct\(runtime\)' -Description "CP375 arbitrary non-direct delegation"
Assert-NotContains -Path $cp374ArbitraryAssertionsForCp373 -Pattern 'assert_numerical_nonfeed\(' -Description "CP374 relinquishes terminal nonfeed"
Assert-NotContains -Path $cp375ArbitraryAssertionsForCp373 -Pattern 'assert_numerical_nonfeed\(' -Description "CP375 relinquishes terminal nonfeed"
Assert-NotContains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp377_assertions.rs" -Pattern 'assert_numerical_nonfeed\(' -Description "CP377 relinquishes terminal numerical evidence"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp378_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_exact_reconciliation\(' -Description "CP378 terminal reconciliation"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp379_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP379 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP380 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed firewall"
Assert-Contains -Path $cp373PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp428_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp373PipelineRoot -Pattern $cp373Lifecycle -Description "pipeline lifecycle key"
Assert-Contains -Path $cp373SnapshotSerialization -Pattern 'json_number|is_finite' -Description "finite JSON projection"
Assert-Contains -Path $cp373SnapshotSerialization -Pattern '_ieee_bits' -Description "authoritative IEEE sidecars"

# Exactly two stable spec addenda and five hand-written documentation sections.
$cp373AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp373CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp373AlgorithmAddenda = [regex]::Matches($cp373AlgorithmText, '(?m)^\s*"CP373 supersedes only CP372[^"\r\n]+",\s*$')
$cp373CapabilityAddenda = [regex]::Matches($cp373CapabilityText, '(?m)^\s*"CP373 additionally requires[^"\r\n]+",\s*$')
if ($cp373AlgorithmAddenda.Count -ne 2 -or $cp373CapabilityAddenda.Count -ne 2) {
    throw "CP373 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp373AlgorithmAddenda + $cp373CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp373SourceCommit, $cp373SourceHash, '2249', '2250', '2258',
            'six', 'CP372', 'sole predecessor|sole immediate', 'CP330', 'CP360', 'CP329',
            'positive infinity', 'raw binary64 division', 'CP372-to-CP373-to-unchanged-numerical',
            $cp373Lifecycle, 'CP345', '32 algorithms', '293 routines',
            '58 state-mapped plus 235 source-mapped', '170 required',
            '311 total', '240 public', '71 internal', 'zero unused', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP373 spec addendum missing '$pattern'" }
    }
}
$cp373Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP373 Cooling Supply-Humidity-Ratio Humidification Supply-Humidity-Ratio Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP373 Source-Ordered Cooling Humidification Supply-Humidity-Ratio Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP373 Humidification Supply-Humidity-Ratio Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP373 Humidification Supply-Humidity-Ratio Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP373 Humidification Supply-Humidity-Ratio Assignment Placement' }
)
foreach ($doc in $cp373Docs) {
    $pattern = '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)'
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $pattern)
    if ($sections.Count -ne 1) { throw "CP373 documentation expected one section in $($doc.Path)" }
    foreach ($required in @(
            $cp373SourceCommit, $cp373SourceHash, '2249', '2250', 'CP372', 'CP330',
            'six', 'division', 'addition', 'CP345', '311\s+total', '71\s+internal'
        )) {
        if ($sections[0].Value -notmatch $required) { throw "CP373 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP373\b' -Description "CP373 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP373 supersedes only CP372' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP373 additionally requires' -Description "generated capability addendum"

# Historical latest-stage expectations and current generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..371 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp373Stem" -Description "historical CP373 binding order"
}
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp372-cooling-supply-humidity-ratio-humidification-moisture-demand-assignment.ps1" -Pattern 'cp373StemForCp372' -Description "CP372 variable binding order"
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp373Stem" -Description "historical CP373 helper whitelist"
}
foreach ($historical in 334..372) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp428_lifecycle_evidence' -Description "historical CP373 firewall"
}
foreach ($historical in 335..372) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 366 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 126 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..372) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 366' -Description "historical inventory total"
}
$cp373MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp372AuditIndexForCp373 = $cp373MainAuditText.IndexOf("cp372-cooling-supply-humidity-ratio-humidification-moisture-demand-assignment.ps1")
$cp373AuditIndex = $cp373MainAuditText.IndexOf("cp373-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-assignment.ps1")
$cp374AuditIndexForCp373 = $cp373MainAuditText.IndexOf("cp374-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-maximum-limit.ps1")
$cp375AuditIndexForCp373 = $cp373MainAuditText.IndexOf("cp375-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-maximum-assignment.ps1")
$cp373CompletionIndex = $cp373MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp372AuditIndexForCp373 -lt 0 -or $cp373AuditIndex -le $cp372AuditIndexForCp373 -or
    $cp374AuditIndexForCp373 -le $cp373AuditIndex -or
    $cp375AuditIndexForCp373 -le $cp374AuditIndexForCp373 -or
    $cp373CompletionIndex -le $cp375AuditIndexForCp373) {
    throw "Master audit must dot-source CP373 then CP374 then CP375 after CP372 before completion"
}
$cp373InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp373TextContains -Text $cp373InventoryText -Pattern 'script_count = 366' -Description "script total"
Assert-Cp373TextContains -Text $cp373InventoryText -Pattern 'dev_command_count = 238' -Description "stable dev-command total"
Assert-Cp373TextContains -Text $cp373InventoryText -Pattern 'unused_script_count = 0' -Description "zero unused"
if ([regex]::Matches($cp373InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp373InventoryText, '(?m)^classification = "internal"$').Count -ne 126) {
throw "CP374 inventory must be exactly 240 public and 122 internal scripts"
}
Assert-Cp373TextContains -Text $cp373InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp373-' -Description "inventory record"
Assert-Cp373TextContains -Text $cp373InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 366 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 126 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated unused"

Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed firewall"
Write-Host "CP373 Cooling humidification supply-humidity-ratio assignment structure audit passed."
}
