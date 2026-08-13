# CP371 maps PurchasedAirManager.cc line 2247's short-circuit
# dehumidification-control Humidistat-or-None guard and its five textual sites.
$cp371Stem = "cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard"
$cp370StemForCp371 = "cooling_supply_humidity_ratio_humidification_control_humidistat_guard"
$cp371PipelineStem = "purchased_air_$cp371Stem"
$cp371TypeStem = "PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuard"
$cp371Lifecycle = "purchased_air_calc_${cp371Stem}_lifecycle"
$cp371SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp371SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp371Sites = @(
    "read-dehumidification-control-type-for-humidistat-comparison",
    "compare-dehumidification-control-type-equal-to-humidistat",
    "read-dehumidification-control-type-for-none-comparison-after-first-false",
    "compare-dehumidification-control-type-equal-to-none",
    "enter-admitted-humidification-body-if-control-condition-satisfied"
)
$cp371Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp371Module = "crates\ep_runtime\src\ideal_loads\calc\$cp371Stem.rs"
$cp371Root = "crates\ep_runtime\src\ideal_loads\calc\$cp371Stem"
$cp371State = "$cp371Root\state.rs"
$cp371Transition = "$cp371Root\transition.rs"
$cp371Release = "$cp371Root\release.rs"
$cp371Prefix = "$cp371Root\release\prefix_validation.rs"
$cp371Private = "$cp371Root\release\private_counterfactual.rs"
$cp371Runtime = "$cp371Root\release\runtime_validation.rs"
$cp371Snapshot = "$cp371Root\release\snapshot_validation.rs"
$cp371CoreTests = "$cp371Root\tests\mod.rs"
$cp371ReleaseTests = "$cp371Root\tests\release.rs"
$cp371CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp371Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp371BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp371Stem.rs"
$cp371BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp371BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp371Stem}_tests.rs"
$cp371ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp371InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp371InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp371InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp371InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp371Stem.rs"
$cp371CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp371Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp371Stem}_validation.rs"
$cp371CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp371CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp371.rs"
$cp371FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp371Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp371Stem}_fixture.rs"
$cp371PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp371Pipeline = "crates\ep_run\src\pipeline\$cp371PipelineStem.rs"
$cp371PipelineValidation = "crates\ep_run\src\pipeline\$cp371PipelineStem\validation.rs"
$cp371PipelineTests = "crates\ep_run\src\pipeline\$cp371PipelineStem\validation\tests.rs"
$cp371Serialization = "crates\ep_run\src\pipeline\$cp371PipelineStem\serialization.rs"
$cp371SnapshotSerialization = "crates\ep_run\src\pipeline\$cp371PipelineStem\serialization\snapshot.rs"
$cp371ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp370_assertions.rs"
$cp371ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp371_assertions.rs"
$cp371Cp320 = "crates\ep_runtime\src\ideal_loads\calc\cooling_humidification_flow.rs"
$cp371Audit = "scripts\quality\ideal-loads-structure-audit\cp371-cooling-supply-humidity-ratio-humidification-dehumidification-control-humidistat-or-none-guard.ps1"

function Assert-Cp371TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP371 $Description missing" }
}
function Assert-Cp371TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP371 $Description unexpectedly present" }
}
function Get-Cp371RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP371 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP371 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP371 $Description closing brace missing"
}

$cp371Required = @(
    $cp371Module, $cp371State, $cp371Transition, $cp371Release, $cp371Prefix,
    $cp371Private, $cp371Runtime, $cp371Snapshot, $cp371CoreTests,
    $cp371ReleaseTests, $cp371BindingAdapter, $cp371BindingTests,
    $cp371InitWitness, $cp371Coupled, $cp371CoupledTests, $cp371Fixture,
    $cp371Pipeline, $cp371PipelineValidation, $cp371PipelineTests,
    $cp371Serialization, $cp371SnapshotSerialization, $cp371ArbitraryAssertions,
    $cp371Audit
)
foreach ($file in $cp371Required) {
    Assert-FileExists -Path $file -Description "CP371 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP371 bounded file"
}

# Raw source pin and exact short-circuit sites.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp371Source).Hash -cne $cp371SourceHash) {
    throw "CP371 PurchasedAirManager.cc SHA-256 drift"
}
$cp371Lines = Get-Content -Encoding UTF8 -LiteralPath $cp371Source
if ($cp371Lines[2246].Trim() -cne 'if ((PurchAir.DehumidCtrlType == HumControl::Humidistat) || (PurchAir.DehumidCtrlType == HumControl::None)) {' -or
    $cp371Lines[2247].Trim() -cne 'MdotZnHumidSP = state.dataZoneEnergyDemand->ZoneSysMoistureDemand(ControlledZoneNum).RemainingOutputReqToHumidSP;' -or
    $cp371Lines[2257].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP371 line 2247/2248/2258 source boundary drift"
}
Assert-Contains -Path $cp371Module -Pattern 'PurchasedAirManager\.cc:2247' -Description "CP371 mapped source"
Assert-Contains -Path $cp371Module -Pattern 'PurchasedAirManager\.cc:2248' -Description "CP371 first excluded source"
Assert-ExactStringArray -Path $cp371Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_OR_NONE_GUARD_SOURCE_ORDER' -Expected $cp371Sites -Description "CP371 source order"
Assert-PatternsInOrder -Path $cp371Cp320 -Patterns @($cp371Sites | ForEach-Object { [regex]::Escape('"' + $_ + '"') }) -Description "CP320 structural short-circuit slice"
Assert-Contains -Path $cp371Prefix -Pattern 'PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER\[6\.\.11\]' -Description "CP320 structural slice equality"
Assert-NotContains -Path $cp371Prefix -Pattern 'cooling_humidification_flow_latest_witness' -Description "CP320 direct value provenance"

# CP370 is sole immediate predecessor; selected immutable dehumidification control owns the operand.
foreach ($pattern in @(
        'cooling_supply_humidity_ratio_humidification_control_humidistat_guard',
        'let owner = system\.dehumidification_control_type',
        'DehumidificationControlType::None',
        'guard_links_to_predecessor'
    )) {
    Assert-Contains -Path $cp371Prefix -Pattern $pattern -Description "CP371 predecessor/owner '$pattern'"
}
Assert-Contains -Path $cp371Release -Pattern 'advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard' -Description "CP371 public release"

# Exact left-to-right OR routes: direct skip 0; private None 5, Humidistat 3, other 4.
$cp371TransitionText = Read-RepoText -Path $cp371Transition
foreach ($pattern in @(
        'first_is_humidistat', 'second_control', 'first_is_humidistat == Some\(false\)',
        'second_is_none', 'body_entered', 'source_sites = if !evaluate',
        'first_is_humidistat == Some\(true\)\s*\{\s*3',
        'second_is_none == Some\(true\)\s*\{\s*5',
        '\}\s*else\s*\{\s*4\s*\}'
    )) {
    Assert-Cp371TextContains -Text $cp371TransitionText -Pattern $pattern -Description "short-circuit contract '$pattern'"
}
$cp371StateText = Read-RepoText -Path $cp371State
foreach ($field in @(
        'dehumidification_control_type_first_read_count',
        'dehumidification_control_type_humidistat_comparison_count',
        'dehumidification_control_type_humidistat_match_count',
        'dehumidification_control_type_second_read_count',
        'dehumidification_control_type_none_comparison_count',
        'dehumidification_control_type_none_match_count',
        'dehumidification_control_body_entry_count',
        'dehumidification_control_guard_false_fallthrough_count',
        'source_site_execution_count'
    )) {
    Assert-Cp371TextContains -Text $cp371StateText -Pattern $field -Description "state counter '$field'"
}

# The CP371 DTOs and production transitions are control-only.
$cp371ModuleText = Read-RepoText -Path $cp371Module
$cp371SnapshotDto = Get-Cp371RustBraceBlock -Text $cp371ModuleText -AnchorPattern "pub struct ${cp371TypeStem}Snapshot\s*\{" -Description "snapshot DTO"
$cp371StateDto = Get-Cp371RustBraceBlock -Text $cp371StateText -AnchorPattern "pub struct ${cp371TypeStem}RuntimeState\s*\{" -Description "runtime-state DTO"
$cp371LifecycleDto = Get-Cp371RustBraceBlock -Text $cp371ModuleText -AnchorPattern "pub struct ${cp371TypeStem}LifecycleSummary\s*\{" -Description "lifecycle DTO"
foreach ($dto in @($cp371SnapshotDto, $cp371StateDto, $cp371LifecycleDto)) {
    Assert-Cp371TextNotContains -Text $dto -Pattern 'f64|ieee|bits|humidity_ratio\s*:' -Description "numeric DTO payload"
}
foreach ($file in @($cp371Transition, $cp371Release, $cp371Prefix, $cp371Private)) {
    Assert-NotContains -Path $file -Pattern 'to_bits|from_bits|\.is_finite\(\)|f64::|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|SupplyHumRatOrig' -Description "CP371 numerical/psychrometric firewall"
}
Assert-NotContains -Path $cp371Serialization -Pattern '_ieee_bits|json_number|to_bits|from_bits|\bf64\b|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio' -Description "CP371 control-only lifecycle JSON"

# Registrations, CP370-to-CP371-to-numerical order, and terminal arbitrary-run ownership.
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp371CalcRoot; Pattern = $cp371Stem; Description = "calc registration" },
        [PSCustomObject]@{ Path = $cp371BindingAdapter; Pattern = "advance_direct_no_oa_calc_$cp371Stem"; Description = "binding adapter" },
        [PSCustomObject]@{ Path = $cp371ScheduledOutput; Pattern = "pub calculation_${cp371Stem}:"; Description = "scheduled output" },
        [PSCustomObject]@{ Path = $cp371BindingTestsRoot; Pattern = $cp371Stem; Description = "binding-test registration" },
        [PSCustomObject]@{ Path = $cp371InitState; Pattern = $cp371Stem; Description = "runtime state" },
        [PSCustomObject]@{ Path = $cp371InitUnit; Pattern = $cp371Stem; Description = "unit state" },
        [PSCustomObject]@{ Path = $cp371InitWitnessRoot; Pattern = $cp371Stem; Description = "witness registration" },
        [PSCustomObject]@{ Path = $cp371CoupledRoot; Pattern = "mod ${cp371Stem}_validation;"; Description = "coupled validator" },
        [PSCustomObject]@{ Path = $cp371FixtureRoot; Pattern = $cp371Stem; Description = "fixture registration" },
        [PSCustomObject]@{ Path = $cp371CoupledTestsRoot; Pattern = 'coupled_runtime_tests_cp371'; Description = "coupled-test registration" },
        [PSCustomObject]@{ Path = $cp371PipelineRoot; Pattern = "mod ${cp371PipelineStem};"; Description = "pipeline module" },
        [PSCustomObject]@{ Path = $cp371PipelineRoot; Pattern = "`"$cp371Lifecycle`":\s*result\s*\.$cp371Lifecycle"; Description = "lifecycle JSON" }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP371 $($registration.Description)"
}
$cp371BindingText = Read-RepoText -Path $cp371Binding
$cp370BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_${cp370StemForCp371} =")
$cp371BindingIndex = $cp371BindingText.IndexOf("let calculation_${cp371Stem} =")
$cp372BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp371 = $cp371BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp371NumericalIndex = $cp371BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp370BindingIndexForCp371 -lt 0 -or $cp371BindingIndex -le $cp370BindingIndexForCp371 -or
    $cp372BindingIndexForCp371 -le $cp371BindingIndex -or $cp373BindingIndexForCp371 -le $cp372BindingIndexForCp371 -or
    $cp374BindingIndexForCp371 -le $cp373BindingIndexForCp371 -or
    $cp375BindingIndexForCp371 -le $cp374BindingIndexForCp371 -or
    $cp376BindingIndexForCp371 -le $cp375BindingIndexForCp371 -or $cp377BindingIndexForCp371 -le $cp376BindingIndexForCp371 -or $cp378BindingIndexForCp371 -le $cp377BindingIndexForCp371 -or $cp379BindingIndexForCp371 -le $cp378BindingIndexForCp371 -or $cp380BindingIndexForCp371 -le $cp379BindingIndexForCp371 -or $cp381BindingIndexForCp371 -le $cp380BindingIndexForCp371 -or $cp382BindingIndexForCp371 -le $cp381BindingIndexForCp371 -or $cp383BindingIndexForCp371 -le $cp382BindingIndexForCp371 -or $cp384BindingIndexForCp371 -le $cp383BindingIndexForCp371 -or $cp385BindingIndexForCp371 -le $cp384BindingIndexForCp371 -or $cp371NumericalIndex -le $cp385BindingIndexForCp371) {
    throw "Binding must execute CP370 then CP371 before unchanged numerical coupling"
}
$cp371NumericalDto = Get-Cp371RustBraceBlock -Text $cp371BindingText.Substring($cp371NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp371TextNotContains -Text $cp371NumericalDto -Pattern 'cp371|humidification_dehumidification_control_humidistat_or_none_guard' -Description "numerical DTO feed"
Assert-Contains -Path $cp371ParentAssertions -Pattern 'mod cp371_assertions;' -Description "CP371 arbitrary delegation"
Assert-Contains -Path $cp371ParentAssertions -Pattern 'cp371_assertions::assert_direct\(runtime, results\)' -Description "CP371 arbitrary direct call"
Assert-Contains -Path $cp371ParentAssertions -Pattern 'cp371_assertions::assert_non_direct\(runtime\)' -Description "CP371 arbitrary non-direct call"
Assert-NotContains -Path $cp371ParentAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP370 terminal ownership"
Assert-Contains -Path $cp371ArbitraryAssertions -Pattern 'mod cp372_assertions;' -Description "CP372 arbitrary delegation module"
Assert-Contains -Path $cp371ArbitraryAssertions -Pattern 'cp372_assertions::assert_direct\(runtime, results\)' -Description "CP372 arbitrary direct delegation"
Assert-Contains -Path $cp371ArbitraryAssertions -Pattern 'cp372_assertions::assert_non_direct\(runtime\)' -Description "CP372 arbitrary non-direct delegation"
Assert-NotContains -Path $cp371ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP371 relinquishes terminal nonfeed"
Assert-Contains -Path $cp371PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp423_lifecycle_evidence' -Description "cumulative non-direct firewall"

# Exactly two algorithm/capability addenda and stable targets/counts.
$cp371AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp371CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp371AlgorithmAddenda = [regex]::Matches($cp371AlgorithmText, '(?m)^\s*"CP371 supersedes only CP370[^"\r\n]+",\s*$')
$cp371CapabilityAddenda = [regex]::Matches($cp371CapabilityText, '(?m)^\s*"CP371 additionally requires[^"\r\n]+",\s*$')
if ($cp371AlgorithmAddenda.Count -ne 2 -or $cp371CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP371 addenda"
}
foreach ($claim in @($cp371AlgorithmAddenda) + @($cp371CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp371SourceCommit, $cp371SourceHash, 'physical executable line 2247',
            $cp371Sites[0], $cp371Sites[1], $cp371Sites[2], $cp371Sites[3], $cp371Sites[4],
            'physical executable line 2248', 'physical executable line 2258',
            'CP370', 'sole immediate source-order predecessor', 'CP320', 'structural short-circuit corroboration',
            'dehumidification_control_type', 'direct.*zero', 'None.*five|None.*5',
            'Humidistat.*three|Humidistat.*3', 'other.*four|other.*4',
            'control-only|named-enum', 'CP370-to-CP371-to-unchanged-?\s*numerical',
            $cp371Lifecycle, 'CP345', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', '170 required', 'Roadmap',
            '309 total', '240 public', '69 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP371 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp371Stem/release.rs::advance_direct_no_oa_calc_$cp371Stem"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp371Stem.rs::purchased_air_calc_${cp371Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp371Stem.rs::${cp371TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Value = "crates/ep_runtime/src/ideal_loads/calc/$cp371Stem.rs::${cp371TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp371AlgorithmText, [regex]::Escape($target.Value)).Count -ne $target.Expected) {
        throw "CP371 target count failed for '$($target.Value)'"
    }
}

# Five hand-doc sections; no psychrometrics promotion.
$cp371Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP371 Cooling Supply-Humidity-Ratio Humidification Dehumidification-Control Humidistat-or-None Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP371 Source-Ordered Cooling Supply-Humidity-Ratio Humidification Dehumidification-Control Humidistat-or-None Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP371 Cooling Supply-Humidity-Ratio Humidification Dehumidification-Control Humidistat-or-None Guard\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP371 Cooling Supply-Humidity-Ratio Humidification Dehumidification-Control Humidistat-or-None Guard in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP371 Cooling Supply-Humidity-Ratio Humidification Dehumidification-Control Humidistat-or-None Guard Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp371Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP371 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp371SourceCommit, $cp371SourceHash, '2247', '2248', '2258',
            $cp371Sites[0], $cp371Sites[1], $cp371Sites[2], $cp371Sites[3], $cp371Sites[4],
            'CP370', 'sole immediate source-order predecessor', 'CP320', 'structural short-circuit corroboration',
            'dehumidification_control_type', 'direct.*zero', 'None.*five|None.*5',
            'Humidistat.*three|Humidistat.*3', 'other.*four|other.*4',
            'CP370-to-CP371-to-unchanged-?\s*numerical', $cp371Lifecycle, 'CP345',
            '32\s+algorithms', '293\s+routines', '58\s+`?state_mapped`?',
            '235\s+`?source_mapped`?', '170\s+required', '309\s+total',
            '240\s+public', '69\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) { throw "CP371 documentation in $($doc.Path) missing '$pattern'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP371\b' -Description "CP371 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP371 supersedes only CP370' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP371 additionally requires' -Description "generated capability addendum"

# Historical source order, helper scope, cumulative firewall, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..370 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp371Stem" -Description "historical CP371 binding order"
}
foreach ($historical in 327..328) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern "calculation_$cp371Stem" -Description "out-of-range CP371 binding token"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp371Stem" -Description "historical CP371 helper whitelist"
}
foreach ($historical in @(327, 328) + @(346..370)) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-NotContains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern "advance_$cp371Stem" -Description "out-of-range CP371 helper token"
}
foreach ($historical in 334..370) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp423_lifecycle_evidence' -Description "historical CP371 firewall"
}
foreach ($historical in 335..370) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 361 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 121 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..370) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 361' -Description "historical inventory total"
}
$cp371MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp370AuditIndexForCp371 = $cp371MainAuditText.IndexOf("cp370-cooling-supply-humidity-ratio-humidification-control-humidistat-guard.ps1")
$cp371AuditIndex = $cp371MainAuditText.IndexOf("cp371-cooling-supply-humidity-ratio-humidification-dehumidification-control-humidistat-or-none-guard.ps1")
$cp372AuditIndexForCp371 = $cp371MainAuditText.IndexOf("cp372-cooling-supply-humidity-ratio-humidification-moisture-demand-assignment.ps1")
$cp371CompletionIndex = $cp371MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp370AuditIndexForCp371 -lt 0 -or $cp371AuditIndex -le $cp370AuditIndexForCp371 -or
    $cp372AuditIndexForCp371 -le $cp371AuditIndex -or $cp371CompletionIndex -le $cp372AuditIndexForCp371) {
    throw "Master audit must dot-source CP371 after CP370 before completion"
}
$cp371InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp371TextContains -Text $cp371InventoryText -Pattern 'script_count = 361' -Description "script total"
Assert-Cp371TextContains -Text $cp371InventoryText -Pattern 'dev_command_count = 238' -Description "stable dev-command total"
Assert-Cp371TextContains -Text $cp371InventoryText -Pattern 'unused_script_count = 0' -Description "zero unused"
if ([regex]::Matches($cp371InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp371InventoryText, '(?m)^classification = "internal"$').Count -ne 121) {
throw "CP371 inventory must be exactly 240 public and 121 internal scripts"
}
Assert-Cp371TextContains -Text $cp371InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp371-' -Description "inventory record"
Assert-Cp371TextContains -Text $cp371InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 361 \|' -Description "CP371 generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP371 generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 121 \|' -Description "CP371 generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP371 generated unused"

Write-Host "CP371 Cooling supply-humidity-ratio humidification dehumidification-control Humidistat-or-None guard structure audit passed."
