# CP366 maps only PurchasedAirManager.cc physical executable line 2236's
# ConstantSupplyHumidityRatio case break.
$cp366Stem = "cooling_constant_supply_humidity_ratio_case_break"
$cp365StemForCp366 = "cooling_constant_supply_humidity_ratio_assignment"
$cp366PipelineStem = "purchased_air_$cp366Stem"
$cp366TypeStem = "PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreak"
$cp366Lifecycle = "purchased_air_calc_${cp366Stem}_lifecycle"
$cp366SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp366SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp366Sites = @(
    "exit-purchased-air-dehumidification-control-constant-supply-humidity-ratio-case-via-break"
)
$cp366Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp366Module = "crates\ep_runtime\src\ideal_loads\calc\$cp366Stem.rs"
$cp366ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp366Stem"
$cp366State = "$cp366ModuleRoot\state.rs"
$cp366Transition = "$cp366ModuleRoot\transition.rs"
$cp366Release = "$cp366ModuleRoot\release.rs"
$cp366Prefix = "$cp366ModuleRoot\release\prefix_validation.rs"
$cp366Private = "$cp366ModuleRoot\release\private_counterfactual.rs"
$cp366Runtime = "$cp366ModuleRoot\release\runtime_validation.rs"
$cp366Snapshot = "$cp366ModuleRoot\release\snapshot_validation.rs"
$cp366CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp366Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp366BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp366Stem.rs"
$cp366BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp366BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp366Stem}_tests.rs"
$cp366ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp366InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp366InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp366InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp366InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp366Stem.rs"
$cp366CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp366Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp366Stem}_validation.rs"
$cp366CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp366CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp366.rs"
$cp366FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp366Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp366Stem}_fixture.rs"
$cp366PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp366Pipeline = "crates\ep_run\src\pipeline\$cp366PipelineStem.rs"
$cp366PipelineValidation = "crates\ep_run\src\pipeline\$cp366PipelineStem\validation.rs"
$cp366PipelineTests = "crates\ep_run\src\pipeline\$cp366PipelineStem\validation\tests.rs"
$cp366Serialization = "crates\ep_run\src\pipeline\$cp366PipelineStem\serialization.rs"
$cp366SnapshotSerialization = "crates\ep_run\src\pipeline\$cp366PipelineStem\serialization\snapshot.rs"
$cp366ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp365_assertions.rs"
$cp366ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp366_assertions.rs"
$cp366Audit = "scripts\quality\ideal-loads-structure-audit\cp366-cooling-constant-supply-humidity-ratio-case-break.ps1"

function Assert-Cp366TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP366 $Description missing" }
}

function Assert-Cp366TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP366 $Description unexpectedly present" }
}

function Get-Cp366RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) { throw "$Description expected one anchor, found $($anchors.Count)" }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) { throw "$Description has no opening brace" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1)
            }
        }
    }
    throw "$Description has no complete brace block"
}

foreach ($required in @(
        $cp366Source, $cp366Module, $cp366State, $cp366Transition, $cp366Release,
        $cp366Prefix, $cp366Private, $cp366Runtime, $cp366Snapshot,
        $cp366BindingAdapter, $cp366BindingTests, $cp366InitWitness, $cp366Coupled,
        $cp366CoupledTests, $cp366Fixture, $cp366Pipeline, $cp366PipelineValidation,
        $cp366PipelineTests, $cp366Serialization, $cp366SnapshotSerialization,
        $cp366ParentAssertions, $cp366ArbitraryAssertions, $cp366Audit
    )) {
    Assert-FileExists -Path $required -Description "CP366 structure"
}
foreach ($bounded in @(
        $cp366Module, $cp366State, $cp366Transition, $cp366Release, $cp366Prefix,
        $cp366Private, $cp366Runtime, $cp366Snapshot, $cp366BindingAdapter,
        $cp366BindingTests, $cp366InitWitness, $cp366Coupled, $cp366CoupledTests,
        $cp366Fixture, $cp366Pipeline, $cp366PipelineValidation, $cp366PipelineTests,
        $cp366Serialization, $cp366SnapshotSerialization, $cp366ArbitraryAssertions,
        $cp366Audit
    )) {
    Assert-LineLimit -Path $bounded -Limit 500 -Description "CP366 bounded structure"
}

# Pinned source, one exact site, seven routes, and one-site CSH counter.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp366Source).Hash -cne $cp366SourceHash) {
    throw "CP366 pinned PurchasedAirManager.cc hash drifted"
}
$cp366SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp366Source
if ($cp366SourceLines[2234].Trim() -cne 'PurchAir.SupplyHumRat = PurchAir.MinCoolSuppAirHumRat;' -or
    $cp366SourceLines[2235].Trim() -cne '} break;' -or
    $cp366SourceLines[2236].Trim() -cne 'default: {' -or
    $cp366SourceLines[2237].Trim() -cne 'PurchAir.SupplyHumRat = PurchAir.MixedAirHumRat;' -or
    $cp366SourceLines[2244].Trim() -cne 'if (HeatOn) {') {
    throw "CP366 physical lines 2235 through 2238 or 2245 drifted"
}
Assert-Contains -Path $cp366Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2236' -Description "CP366 source line"
Assert-Contains -Path $cp366Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2238' -Description "CP366 first excluded executable"
Assert-ExactStringArray -Path $cp366Module -Name "PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER" -Expected $cp366Sites -Description "CP366 one-site source order"
Assert-Contains -Path $cp366State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,\s*DehumidificationControlHumidistatCaseCompletedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioCaseBreak' -Description "CP366 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_supply_humidity_ratio_case_break_count",
        "source_site_execution_count"
    )) {
    Assert-Contains -Path $cp366State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP366 counter '$counter'"
}
foreach ($field in @(
        "predecessor_dehumidification_control_constant_supply_humidity_ratio_assignment_executed",
        "dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break"
    )) {
    Assert-Contains -Path $cp366Module -Pattern ('pub ' + $field + ':\s*bool') -Description "CP366 snapshot field '$field'"
}
Assert-Contains -Path $cp366Transition -Pattern '(?s)DehumidificationControlConstantSupplyHumidityRatioCaseBreak\s*=>\s*\{.*?case_break_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=.*?SOURCE_ORDER.*?\.len\(\);' -Description "CP366 CSH one-site counter"
Assert-Contains -Path $cp366Runtime -Pattern '(?s)csh\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER\.len\(\)' -Description "CP366 checked one-site count"
Assert-Contains -Path $cp366Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "CP366 route partition"

# CP365 is the sole recursive predecessor; CP366 carries no numerical payload.
Assert-Contains -Path $cp366Transition -Pattern 'PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot as Predecessor' -Description "exact CP365 predecessor type"
$cp366CalcText = (@($cp366Module) + @(
        Get-ChildItem -LiteralPath $cp366ModuleRoot -Recurse -File -Filter "*.rs" |
            ForEach-Object { $_.FullName }
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($pattern in @(
        'completed_direct_cooling_constant_supply_humidity_ratio_assignment_is_consistent',
        'cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release',
        'cooling_constant_supply_humidity_ratio_assignment_snapshots_match_bit_exact',
        'cooling_constant_supply_humidity_ratio_assignment_latest_witness',
        'private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release'
    )) {
    Assert-Cp366TextContains -Text $cp366CalcText -Pattern $pattern -Description "recursive CP365 owner '$pattern'"
}
$cp366SnapshotDto = Get-Cp366RustBraceBlock -Text (Read-RepoText -Path $cp366Module) -AnchorPattern 'pub struct PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot\s*\{' -Description "CP366 snapshot DTO"
Assert-Cp366TextNotContains -Text $cp366SnapshotDto -Pattern 'f64|ieee|bits|Option\s*<\s*f64\s*>|humidity_ratio:\s*Option' -Description "snapshot numerical payload"
Assert-NotContains -Path $cp366Transition -Pattern 'minimum_cooling_supply_air_humidity_ratio|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio|to_bits|from_bits|\.is_finite\(\)|f64::|\.clamp\(|Psy[A-Za-z_]*|energyplus_psy_|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand' -Description "CP366 pure numeric firewall"
$cp366PublicRelease = Get-Cp366RustBraceBlock -Text (Read-RepoText -Path $cp366Release) -AnchorPattern '(?m)^pub fn advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break\s*\(' -Description "CP366 public release"
Assert-Cp366TextContains -Text $cp366PublicRelease -Pattern 'predecessor_cp365:\s*Predecessor' -Description "public CP365 predecessor"
Assert-Cp366TextNotContains -Text $cp366PublicRelease -Pattern ':\s*f64|minimum_cooling_supply_air_humidity_ratio|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio' -Description "public numerical operand"
foreach ($path in @($cp366Prefix, $cp366Private)) {
    Assert-NotContains -Path $path -Pattern 'minimum_cooling_supply_air_humidity_ratio|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio|to_bits|from_bits|\.is_finite\(\)|PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply' -Description "CP366 private numeric firewall"
}

# Searchable semantic regression and serialization locks.
$cp366SemanticText = (@(
        Get-ChildItem -LiteralPath $cp366ModuleRoot -Recurse -File -Filter "*.rs" |
            Where-Object { $_.FullName -match '(?:_tests\.rs$|[\\/]tests[\\/])' } |
            ForEach-Object { $_.FullName }
    ) + @(
        $cp366BindingTests, $cp366CoupledTests, $cp366PipelineTests,
        $cp366SnapshotSerialization, $cp366ArbitraryAssertions
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
foreach ($test in @(
        "source_boundary_single_site_and_seven_route_algebra_are_exact",
        "transition_is_evidence_only_and_all_overflow_is_transactional",
        "public_direct_routes_skip_break_and_private_csh_uses_only_cp365_bridge",
        "corruption_identity_replay_and_runtime_forge_reject_without_mutation",
        "binding_orders_cp365_then_cp366_before_numerical_and_does_not_feed_result",
        "binding_cp366_is_complete_skip_for_direct_none",
        "binding_rejects_corrupt_cp365_without_mutation",
        "cp366_lifecycle_matches_outputs_and_cp365",
        "cp366_route_source_latest_and_predecessor_corruption_are_rejected",
        "cp366_expected_snapshot_maps_q_h_and_csh_predecessor_routes",
        "missing_direct_lifecycle_fails_closed",
        "checked_partitions_and_source_counts_fail_closed",
        "expected_snapshot_maps_only_constant_supply_assignment_to_break",
        "direct_release_and_immediate_predecessor_are_strict",
        "direct_none_release_serializes_complete_false_break_skip",
        "active_constant_supply_break_has_no_numeric_payload_or_default_fallthrough"
    )) {
    Assert-Cp366TextContains -Text $cp366SemanticText -Pattern ('(?m)fn\s+' + $test + '\s*\(') -Description "semantic regression '$test'"
}
$cp366SnapshotSerializationText = Read-RepoText -Path $cp366SnapshotSerialization
$cp366SnapshotTestBoundary = [regex]::Match($cp366SnapshotSerializationText, '(?m)^\s*#\[cfg\(test\)\]\s*$')
$cp366SnapshotProduction = if ($cp366SnapshotTestBoundary.Success) {
    $cp366SnapshotSerializationText.Substring(0, $cp366SnapshotTestBoundary.Index)
} else {
    $cp366SnapshotSerializationText
}
Assert-Cp366TextNotContains -Text $cp366SnapshotProduction -Pattern '_ieee_bits|json_number|Value::Null|to_bits|f64' -Description "control-only JSON"

# Binding, coupled runtime, pipeline, arbitrary chain, and numerical nonfeed.
$cp366BindingText = Read-RepoText -Path $cp366Binding
$cp365BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_${cp365StemForCp366} =")
$cp366BindingIndex = $cp366BindingText.IndexOf("let calculation_${cp366Stem} =")
$cp367BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =")
$cp368BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_case_break =")
$cp369BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp366 = $cp366BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp366NumericalIndex = $cp366BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp365BindingIndexForCp366 -lt 0 -or $cp366BindingIndex -le $cp365BindingIndexForCp366 -or $cp367BindingIndexForCp366 -le $cp366BindingIndex -or $cp368BindingIndexForCp366 -le $cp367BindingIndexForCp366 -or
    $cp366NumericalIndex -le $cp368BindingIndexForCp366 -or
    $cp369BindingIndexForCp366 -le $cp368BindingIndexForCp366 -or
    $cp370BindingIndexForCp366 -le $cp369BindingIndexForCp366 -or
    $cp371BindingIndexForCp366 -le $cp370BindingIndexForCp366 -or
    $cp372BindingIndexForCp366 -le $cp371BindingIndexForCp366 -or
    $cp373BindingIndexForCp366 -le $cp372BindingIndexForCp366 -or
    $cp374BindingIndexForCp366 -le $cp373BindingIndexForCp366 -or
    $cp375BindingIndexForCp366 -le $cp374BindingIndexForCp366 -or
    $cp376BindingIndexForCp366 -le $cp375BindingIndexForCp366 -or $cp377BindingIndexForCp366 -le $cp376BindingIndexForCp366 -or $cp378BindingIndexForCp366 -le $cp377BindingIndexForCp366 -or $cp379BindingIndexForCp366 -le $cp378BindingIndexForCp366 -or $cp380BindingIndexForCp366 -le $cp379BindingIndexForCp366 -or $cp381BindingIndexForCp366 -le $cp380BindingIndexForCp366 -or $cp382BindingIndexForCp366 -le $cp381BindingIndexForCp366 -or $cp383BindingIndexForCp366 -le $cp382BindingIndexForCp366 -or $cp384BindingIndexForCp366 -le $cp383BindingIndexForCp366 -or $cp385BindingIndexForCp366 -le $cp384BindingIndexForCp366 -or $cp366NumericalIndex -le $cp385BindingIndexForCp366) {
    throw "Binding must execute CP365 then CP366 then CP367 then CP368 before numerical coupling"
}
$cp366Dto = Get-Cp366RustBraceBlock -Text $cp366BindingText.Substring($cp366NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP366 numerical DTO"
if ($cp366Dto -match '(?i)cp366|constant_supply_humidity_ratio_case_break') {
    throw "CP366 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp366CalcRoot -Pattern $cp366Stem -Description "calc registration"
Assert-Contains -Path $cp366BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + $cp366Stem) -Description "binding adapter"
Assert-Contains -Path $cp366ScheduledOutput -Pattern ('pub calculation_' + $cp366Stem + ':') -Description "scheduled output"
Assert-Contains -Path $cp366BindingTestsRoot -Pattern $cp366Stem -Description "binding-test registration"
Assert-Contains -Path $cp366InitState -Pattern $cp366Stem -Description "runtime state"
Assert-Contains -Path $cp366InitUnit -Pattern $cp366Stem -Description "unit state"
Assert-Contains -Path $cp366InitWitnessRoot -Pattern $cp366Stem -Description "witness registration"
Assert-Contains -Path $cp366CoupledRoot -Pattern ('mod ' + $cp366Stem + '_validation;') -Description "coupled validator"
Assert-Contains -Path $cp366Coupled -Pattern ('calculation_' + $cp365StemForCp366) -Description "coupled CP365 predecessor"
Assert-Contains -Path $cp366FixtureRoot -Pattern $cp366Stem -Description "fixture registration"
Assert-Contains -Path $cp366Fixture -Pattern ('calculation_' + $cp366Stem + '_snapshot') -Description "output fixture"
Assert-Contains -Path $cp366CoupledTestsRoot -Pattern 'coupled_runtime_tests_cp366' -Description "coupled-test registration"
Assert-Contains -Path $cp366PipelineRoot -Pattern ('mod ' + $cp366PipelineStem + ';') -Description "pipeline module"
Assert-Contains -Path $cp366PipelineRoot -Pattern ('"' + $cp366Lifecycle + '":\s*result\s*\.' + $cp366Lifecycle) -Description "lifecycle JSON"
Assert-Contains -Path $cp366PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp395_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp366ParentAssertions -Pattern 'mod cp366_assertions;' -Description "arbitrary delegation module"
Assert-Contains -Path $cp366ParentAssertions -Pattern 'cp366_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp366ParentAssertions -Pattern 'cp366_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp366ParentAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP365 relinquishes terminal nonfeed"
Assert-Contains -Path $cp366ArbitraryAssertions -Pattern 'mod cp367_assertions;' -Description "CP367 arbitrary delegation module"
Assert-Contains -Path $cp366ArbitraryAssertions -Pattern 'cp367_assertions::assert_direct\(runtime, results\)' -Description "CP367 arbitrary direct delegation"
Assert-Contains -Path $cp366ArbitraryAssertions -Pattern 'cp367_assertions::assert_non_direct\(runtime\)' -Description "CP367 arbitrary non-direct delegation"
Assert-NotContains -Path $cp366ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP366 relinquishes terminal nonfeed"
Assert-Contains -Path $cp366ArbitraryAssertions -Pattern 'runtime\.contains_key\(CP366_KEY\)' -Description "CP366 non-direct key"
Assert-Contains -Path $cp366ArbitraryAssertions -Pattern 'runtime\[CP366_KEY\]\.is_null\(\)' -Description "CP366 non-direct null"

# Two algorithm/capability addenda, generated docs, hand docs, and inventory.
$cp366AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp366CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp366AlgorithmAddenda = [regex]::Matches($cp366AlgorithmText, '(?m)^\s*"CP366 supersedes only CP365[^"\r\n]+",\s*$')
$cp366CapabilityAddenda = [regex]::Matches($cp366CapabilityText, '(?m)^\s*"CP366 additionally requires[^"\r\n]+",\s*$')
if ($cp366AlgorithmAddenda.Count -ne 2 -or $cp366CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP366 addenda"
}
foreach ($claim in @($cp366AlgorithmAddenda) + @($cp366CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp366SourceCommit, $cp366SourceHash, 'physical executable line 2236',
            $cp366Sites[0], 'line 2237', 'default', 'physical executable line 2238',
            'line 2245', 'T=U\+N\+P\+C0\+Q\+H\+CSH',
            'S=C0\+Q\+H\+CSH=R=G\+F\+L', 'A=F\+L',
            'source_site_execution_count=constant_supply_humidity_ratio_case_break_count=CSH',
            'C0=S', 'Q=H=CSH=0', 'false break', 'true break', 'cannot fall through',
            'CP365', 'sole predecessor owner', 'no break-local numeric operand',
            'CP365-to-CP366-to-unchanged-numerical', $cp366Lifecycle,
            'first/last supply-humidity bits remain unchanged', '32 algorithms and 293 routines',
            '58 state-mapped plus 235 source-mapped', '170 required', 'Roadmap',
            '304 total', '240 public', '64 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP366 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp366Stem/release\.rs::advance_direct_no_oa_calc_$cp366Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp366Stem\.rs::purchased_air_calc_${cp366Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp366Stem\.rs::${cp366TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp366Stem\.rs::${cp366TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp366AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP366 target count failed for '$($target.Pattern)'"
    }
}
$cp366Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP366 Cooling Constant-Supply-Humidity-Ratio Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP366 Source-Ordered Cooling Constant-Supply-Humidity-Ratio Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP366 Constant-Supply-Humidity-Ratio Case Break\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP366 Constant-Supply-Humidity-Ratio Case Break in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP366 Constant-Supply-Humidity-Ratio Case-Break Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp366Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP366 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            '2236', 'break', '2237', 'default', '2238', '2245',
            '(?:exit-purchased-air-dehumidification-control-constant-supply-humidity-ratio-case-via-break|sole source site|single case-exit source site)',
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            '(?s)(?:source[_ -]site[_ ](?:execution[_ ])?count|sole source[- ]site count).{0,80}(?:equals|=).{0,80}(?:break count|CSH)',
            'C0\s*=\s*S', '(?:Q\s*=\s*H\s*=\s*CSH\s*=\s*0|zero\s+`?Q/H/CSH`?)', 'CP365',
            'CP365-to-CP366-to-unchanged-numerical', $cp366Lifecycle, 'CP345',
            '32\s+algorithms', '293\s+routines', '304\s+total', '240\s+public',
            '64\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP366 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP366\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP366 supersedes only CP365' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP366 additionally requires' -Description "generated capability addendum"

# Historical propagation and master/source order.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..365 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_constant_supply_humidity_ratio_case_break' -Description "historical CP366 binding order"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_constant_supply_humidity_ratio_case_break' -Description "historical CP366 helper whitelist"
}
foreach ($historical in 334..365) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp395_lifecycle_evidence' -Description "historical CP366 firewall"
}
foreach ($historical in 335..365) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 333 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 93 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..365) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 333' -Description "historical inventory total"
}
$cp366MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp365AuditIndexForCp366 = $cp366MainAuditText.IndexOf("cp365-cooling-constant-supply-humidity-ratio-assignment.ps1")
$cp366AuditIndex = $cp366MainAuditText.IndexOf("cp366-cooling-constant-supply-humidity-ratio-case-break.ps1")
$cp366CompletionIndex = $cp366MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp365AuditIndexForCp366 -lt 0 -or $cp366AuditIndex -le $cp365AuditIndexForCp366 -or $cp366CompletionIndex -le $cp366AuditIndex) {
    throw "Master audit must dot-source CP366 after CP365 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 333' -Description "script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp366-' -Description "inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp366-cooling-constant-supply-humidity-ratio-case-break\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 333 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 93 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP366 constant-supply-humidity-ratio case-break structure audit passed."
