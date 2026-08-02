# CP372 maps PurchasedAirManager.cc line 2248's humidifying-setpoint
# moisture-demand read and local assignment, and no part of line 2249.
$cp372Stem = "cooling_supply_humidity_ratio_humidification_moisture_demand_assignment"
$cp371StemForCp372 = "cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard"
$cp373StemForCp372 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment"
$cp374StemForCp372 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit"
$cp375StemForCp372 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment"
$cp372PipelineStem = "purchased_air_$cp372Stem"
$cp372TypeStem = "PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignment"
$cp372Lifecycle = "purchased_air_calc_${cp372Stem}_lifecycle"
$cp372SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp372SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp372Sites = @(
    "read-zone-humidifying-setpoint-moisture-demand",
    "assign-local-zone-humidifying-setpoint-moisture-demand"
)
$cp372Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp372Module = "crates\ep_runtime\src\ideal_loads\calc\$cp372Stem.rs"
$cp372Root = "crates\ep_runtime\src\ideal_loads\calc\$cp372Stem"
$cp372State = "$cp372Root\state.rs"
$cp372Transition = "$cp372Root\transition.rs"
$cp372Release = "$cp372Root\release.rs"
$cp372Prefix = "$cp372Root\release\prefix_validation.rs"
$cp372Private = "$cp372Root\release\private_counterfactual.rs"
$cp372Runtime = "$cp372Root\release\runtime_validation.rs"
$cp372Snapshot = "$cp372Root\release\snapshot_validation.rs"
$cp372CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp372Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp372BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp372Stem.rs"
$cp372BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp372BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp372Stem}_tests.rs"
$cp372ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp372InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp372InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp372InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp372InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp372Stem.rs"
$cp372CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp372Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp372Stem}_validation.rs"
$cp372CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp372CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp372.rs"
$cp372FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp372Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp372Stem}_fixture.rs"
$cp372PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp372Pipeline = "crates\ep_run\src\pipeline\$cp372PipelineStem.rs"
$cp372PipelineValidation = "crates\ep_run\src\pipeline\$cp372PipelineStem\validation.rs"
$cp372PipelineTests = "crates\ep_run\src\pipeline\$cp372PipelineStem\validation\tests.rs"
$cp372Serialization = "crates\ep_run\src\pipeline\$cp372PipelineStem\serialization.rs"
$cp372SnapshotSerialization = "crates\ep_run\src\pipeline\$cp372PipelineStem\serialization\snapshot.rs"
$cp372ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp371_assertions.rs"
$cp372ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp372_assertions.rs"
$cp372Cp320 = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow.rs"
$cp372Audit = "scripts\quality\ideal-loads-structure-audit\cp372-cooling-supply-humidity-ratio-humidification-moisture-demand-assignment.ps1"

function Assert-Cp372TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP372 $Description missing" }
}

function Assert-Cp372TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP372 $Description unexpectedly present" }
}

function Get-Cp372RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP372 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP372 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP372 $Description closing brace missing"
}

$cp372Required = @(
    $cp372Module, $cp372State, $cp372Transition, $cp372Release, $cp372Prefix,
    $cp372Private, $cp372Runtime, $cp372Snapshot, $cp372BindingAdapter,
    $cp372BindingTests, $cp372InitWitness, $cp372Coupled, $cp372CoupledTests,
    $cp372Fixture, $cp372Pipeline, $cp372PipelineValidation, $cp372PipelineTests,
    $cp372Serialization, $cp372SnapshotSerialization, $cp372ArbitraryAssertions,
    $cp372Audit
)
foreach ($file in $cp372Required) {
    Assert-FileExists -Path $file -Description "CP372 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP372 bounded file"
}
$cp372CoreTestFiles = @(Get-ChildItem -LiteralPath "$cp372Root\tests" -Recurse -File -Filter "*.rs")
if ($cp372CoreTestFiles.Count -lt 2) { throw "CP372 requires bounded core and release tests" }
$cp372CoreTestText = ($cp372CoreTestFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP372 bounded core test"
        Read-RepoText -Path $_.FullName
    }) -join "`n"
foreach ($pattern in @('source_order', 'direct', 'private', 'Humidistat', 'None', 'overflow', 'to_bits|ieee', 'transaction')) {
    Assert-Cp372TextContains -Text $cp372CoreTestText -Pattern $pattern -Description "core test matrix '$pattern'"
}
foreach ($testName in @(
        'rejected_nested_controls_skip_the_assignment',
        'heating_availability_guard_false_skips_the_assignment'
    )) {
    Assert-Cp372TextContains -Text $cp372CoreTestText -Pattern $testName -Description "complete skip-route regression '$testName'"
}

# Raw source pin, exact line, exact two sites, and both excluded continuations.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp372Source).Hash -cne $cp372SourceHash) {
    throw "CP372 PurchasedAirManager.cc SHA-256 drift"
}
$cp372Lines = Get-Content -Encoding UTF8 -LiteralPath $cp372Source
if ($cp372Lines[2247].Trim() -cne 'MdotZnHumidSP = state.dataZoneEnergyDemand->ZoneSysMoistureDemand(ControlledZoneNum).RemainingOutputReqToHumidSP;' -or
    $cp372Lines[2248].Trim() -cne 'SupplyHumRatForHumid = MdotZnHumidSP / SupplyMassFlowRate + state.dataLoopNodes->Node(ZoneNodeNum).HumRat;' -or
    $cp372Lines[2257].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP372 line 2248/2249/2258 source boundary drift"
}
Assert-Contains -Path $cp372Module -Pattern 'PurchasedAirManager\.cc:2248' -Description "mapped source"
Assert-Contains -Path $cp372Module -Pattern 'PurchasedAirManager\.cc:2249' -Description "first excluded source"
Assert-ExactStringArray -Path $cp372Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER' -Expected $cp372Sites -Description "CP372 source order"
Assert-PatternsInOrder -Path $cp372Cp320 -Patterns @($cp372Sites | ForEach-Object { [regex]::Escape('"' + $_ + '"') }) -Description "CP320 structural slice"

# CP371 is the sole predecessor; active data is an explicit pre-sampled scalar.
foreach ($pattern in @(
        $cp371StemForCp372,
        'cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_route',
        'DehumidificationControlHumidistatBodyEntered',
        'DehumidificationControlNoneBodyEntered',
        'ActiveInput', 'zone_humidifying_setpoint_moisture_demand_kg_per_s'
    )) {
    Assert-Contains -Path $cp372Transition -Pattern $pattern -Description "predecessor/active-input '$pattern'"
}
Assert-Contains -Path $cp372Private -Pattern 'private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_from_direct_release' -Description "canonical private CP371 bridge"
Assert-Contains -Path $cp372Private -Pattern '(?s)counterfactual_from_direct_release.*?pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s:\s*f64' -Description "explicit private pre-sampled scalar"
Assert-Contains -Path $cp372Private -Pattern 'source_site_execution_count == 2' -Description "canonical private two-site count"
Assert-NotContains -Path $cp372Private -Pattern 'ZoneSysEnergyDemand|selected_typed_moisture_demand_owner|DirectZonePurchasedAirCouplingInput' -Description "private retained/live owner firewall"
Assert-NotContains -Path $cp372Prefix -Pattern 'ZoneSysEnergyDemand|selected_typed_moisture_demand_owner' -Description "prefix retained/live owner firewall"
Assert-Contains -Path $cp372Release -Pattern "advance_direct_no_oa_calc_$cp372Stem" -Description "public direct release"
Assert-Contains -Path $cp372Release -Pattern 'None\s*,?\s*\)?' -Description "direct release accepts no active scalar"

# Eight routes, checked preflight, exact A/read/assign/2A accounting, and bit copies.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthrough', 'HumidificationControlGuardFalseFallthrough',
        'DehumidificationControlHumidistatMoistureDemandAssignmentExecuted',
        'DehumidificationControlNoneMoistureDemandAssignmentExecuted',
        'DehumidificationControlGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp372State -Pattern $route -Description "retained route $route"
}
foreach ($pattern in @(
        'humidification_moisture_demand_assignment_count',
        'zone_humidifying_setpoint_moisture_demand_read_count',
        'zone_humidifying_setpoint_moisture_demand_assignment_count',
        'source_site_execution_count', 'checked_add', 'SOURCE_ORDER\.len\(\)'
    )) {
    Assert-Contains -Path $cp372Transition -Pattern $pattern -Description "counter/preflight '$pattern'"
}
foreach ($field in @(
        'zone_humidifying_setpoint_moisture_demand_kg_per_s',
        'assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s',
        'resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s'
    )) {
    Assert-Contains -Path $cp372Module -Pattern $field -Description "authoritative scalar field $field"
    Assert-Contains -Path $cp372SnapshotSerialization -Pattern "${field}_ieee_bits" -Description "IEEE sidecar $field"
}
Assert-Contains -Path $cp372SnapshotSerialization -Pattern 'is_finite' -Description "nonfinite JSON projection"
Assert-Contains -Path $cp372SnapshotSerialization -Pattern 'to_bits' -Description "authoritative IEEE bits"

# Registrations, source order, and numerical/coupling firewall.
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp372CalcRoot; Pattern = $cp372Stem },
        [PSCustomObject]@{ Path = $cp372ScheduledOutput; Pattern = "pub calculation_${cp372Stem}:" },
        [PSCustomObject]@{ Path = $cp372BindingTestsRoot; Pattern = $cp372Stem },
        [PSCustomObject]@{ Path = $cp372InitState; Pattern = $cp372Stem },
        [PSCustomObject]@{ Path = $cp372InitUnit; Pattern = $cp372Stem },
        [PSCustomObject]@{ Path = $cp372InitWitnessRoot; Pattern = $cp372Stem },
        [PSCustomObject]@{ Path = $cp372CoupledRoot; Pattern = "mod ${cp372Stem}_validation;" },
        [PSCustomObject]@{ Path = $cp372CoupledTestsRoot; Pattern = 'coupled_runtime_tests_cp372' },
        [PSCustomObject]@{ Path = $cp372FixtureRoot; Pattern = $cp372Stem },
        [PSCustomObject]@{ Path = $cp372PipelineRoot; Pattern = $cp372PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP372 registration"
}
$cp372BindingText = Read-RepoText -Path $cp372Binding
$cp371BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_${cp371StemForCp372} =")
$cp372BindingIndex = $cp372BindingText.IndexOf("let calculation_${cp372Stem} =")
$cp373BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_${cp373StemForCp372} =")
$cp374BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_${cp374StemForCp372} =")
$cp375BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_${cp375StemForCp372} =")
$cp376BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp372 = $cp372BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp372NumericalIndex = $cp372BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp371BindingIndexForCp372 -lt 0 -or $cp372BindingIndex -le $cp371BindingIndexForCp372 -or
    $cp373BindingIndexForCp372 -le $cp372BindingIndex -or
    $cp374BindingIndexForCp372 -le $cp373BindingIndexForCp372 -or
    $cp375BindingIndexForCp372 -le $cp374BindingIndexForCp372 -or
    $cp376BindingIndexForCp372 -le $cp375BindingIndexForCp372 -or $cp377BindingIndexForCp372 -le $cp376BindingIndexForCp372 -or $cp378BindingIndexForCp372 -le $cp377BindingIndexForCp372 -or $cp379BindingIndexForCp372 -le $cp378BindingIndexForCp372 -or $cp380BindingIndexForCp372 -le $cp379BindingIndexForCp372 -or $cp381BindingIndexForCp372 -le $cp380BindingIndexForCp372 -or $cp382BindingIndexForCp372 -le $cp381BindingIndexForCp372 -or $cp383BindingIndexForCp372 -le $cp382BindingIndexForCp372 -or $cp384BindingIndexForCp372 -le $cp383BindingIndexForCp372 -or $cp385BindingIndexForCp372 -le $cp384BindingIndexForCp372 -or $cp372NumericalIndex -le $cp385BindingIndexForCp372) {
    throw "Binding must execute CP371 then CP372 then CP373 then CP374 then CP375 before unchanged numerical coupling"
}
$cp372Dto = Get-Cp372RustBraceBlock -Text $cp372BindingText.Substring($cp372NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp372TextNotContains -Text $cp372Dto -Pattern 'cp372|humidification_moisture_demand|zone_humidifying' -Description "numerical DTO feed"
Assert-Contains -Path $cp372ParentAssertions -Pattern 'mod cp372_assertions;' -Description "arbitrary delegation module"
Assert-Contains -Path $cp372ParentAssertions -Pattern 'cp372_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp372ParentAssertions -Pattern 'cp372_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp372ParentAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP371 relinquishes terminal nonfeed"
Assert-Contains -Path $cp372ArbitraryAssertions -Pattern 'mod cp373_assertions;' -Description "CP373 arbitrary delegation module"
Assert-Contains -Path $cp372ArbitraryAssertions -Pattern 'cp373_assertions::assert_direct\(runtime, results\)' -Description "CP373 arbitrary direct delegation"
Assert-Contains -Path $cp372ArbitraryAssertions -Pattern 'cp373_assertions::assert_non_direct\(runtime\)' -Description "CP373 arbitrary non-direct delegation"
Assert-NotContains -Path $cp372ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP372 relinquishes terminal nonfeed"
Assert-Contains -Path $cp372PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp393_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp372PipelineRoot -Pattern $cp372Lifecycle -Description "pipeline lifecycle key"

# Exactly two algorithm/capability addenda, no promotion, and five hand docs.
$cp372AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp372CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp372AlgorithmAddenda = [regex]::Matches($cp372AlgorithmText, '(?m)^\s*"CP372 supersedes only CP371[^"\r\n]+",\s*$')
$cp372CapabilityAddenda = [regex]::Matches($cp372CapabilityText, '(?m)^\s*"CP372 additionally requires[^"\r\n]+",\s*$')
if ($cp372AlgorithmAddenda.Count -ne 2 -or $cp372CapabilityAddenda.Count -ne 2) {
    throw "CP372 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp372AlgorithmAddenda + $cp372CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp372SourceCommit, $cp372SourceHash, '2248', '2249', '2258',
            $cp372Sites[0], $cp372Sites[1], 'CP371', 'sole immediate source-order predecessor',
            'CP320', 'pre-sampled', 'no retained authoritative owner', 'two sites|both sites',
            'CP371-to-CP372-to-unchanged-numerical', $cp372Lifecycle, 'CP345',
            '32 algorithms', '293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', '310 total', '240 public', '70 internal', 'zero unused', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP372 spec addendum missing '$pattern'" }
    }
}
$cp372Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP372 Cooling Supply-Humidity-Ratio Humidification Moisture-Demand Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP372 Source-Ordered Cooling Supply-Humidity-Ratio Humidification Moisture-Demand Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP372 Cooling Supply-Humidity-Ratio Humidification Moisture-Demand Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP372 Cooling Supply-Humidity-Ratio Humidification Moisture-Demand Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP372 Cooling Supply-Humidity-Ratio Humidification Moisture-Demand Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp372Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP372 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp372SourceCommit, $cp372SourceHash, '2248', '2249', '2258',
            $cp372Sites[0], $cp372Sites[1], 'CP371', 'sole immediate source-order predecessor',
            'CP320', 'pre-sampled', 'no retained authoritative owner', 'direct.*zero',
            'selected-`None`.*both|selected `None`.*both', 'Humidistat.*both',
            'CP371-to-CP372-to-unchanged-?\s*numerical', $cp372Lifecycle, 'CP345',
            '32\s+algorithms', '293\s+routines', '58\s+`?state_mapped`?',
            '235\s+`?source_mapped`?', '170\s+required', '310\s+total',
            '240\s+public', '70\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP372 documentation in $($doc.Path) missing '$pattern'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP372\b' -Description "CP372 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP372 supersedes only CP371' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP372 additionally requires' -Description "generated capability addendum"

# Historical binding order/helper scope, cumulative firewall, and current inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..371 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp372Stem" -Description "historical CP372 binding order"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp372Stem" -Description "historical CP372 helper whitelist"
}
foreach ($historical in 334..371) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp393_lifecycle_evidence' -Description "historical CP372 firewall"
}
foreach ($historical in 335..371) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 331 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 91 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..371) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 331' -Description "historical inventory total"
}
$cp372MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp371AuditIndexForCp372 = $cp372MainAuditText.IndexOf("cp371-cooling-supply-humidity-ratio-humidification-dehumidification-control-humidistat-or-none-guard.ps1")
$cp372AuditIndex = $cp372MainAuditText.IndexOf("cp372-cooling-supply-humidity-ratio-humidification-moisture-demand-assignment.ps1")
$cp373AuditIndexForCp372 = $cp372MainAuditText.IndexOf("cp373-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-assignment.ps1")
$cp374AuditIndexForCp372 = $cp372MainAuditText.IndexOf("cp374-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-maximum-limit.ps1")
$cp375AuditIndexForCp372 = $cp372MainAuditText.IndexOf("cp375-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-maximum-assignment.ps1")
$cp372CompletionIndex = $cp372MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp371AuditIndexForCp372 -lt 0 -or $cp372AuditIndex -le $cp371AuditIndexForCp372 -or
    $cp373AuditIndexForCp372 -le $cp372AuditIndex -or
    $cp374AuditIndexForCp372 -le $cp373AuditIndexForCp372 -or
    $cp375AuditIndexForCp372 -le $cp374AuditIndexForCp372 -or
    $cp372CompletionIndex -le $cp375AuditIndexForCp372) {
    throw "Master audit must dot-source CP372 then CP373 then CP374 then CP375 before completion"
}
$cp372InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp372TextContains -Text $cp372InventoryText -Pattern 'script_count = 331' -Description "script total"
Assert-Cp372TextContains -Text $cp372InventoryText -Pattern 'dev_command_count = 238' -Description "stable dev-command total"
Assert-Cp372TextContains -Text $cp372InventoryText -Pattern 'unused_script_count = 0' -Description "zero unused"
if ([regex]::Matches($cp372InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp372InventoryText, '(?m)^classification = "internal"$').Count -ne 91) {
throw "CP372 inventory must be exactly 240 public and 91 internal scripts"
}
Assert-Cp372TextContains -Text $cp372InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp372-' -Description "inventory record"
Assert-Cp372TextContains -Text $cp372InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 331 \|' -Description "CP372 generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP372 generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 91 \|' -Description "CP372 generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP372 generated unused"

Write-Host "CP372 Cooling supply-humidity-ratio humidification moisture-demand assignment structure audit passed."
