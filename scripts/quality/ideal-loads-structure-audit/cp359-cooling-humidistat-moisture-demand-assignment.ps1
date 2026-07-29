# CP359 maps only PurchasedAirManager.cc line 2229 moisture-demand read/assignment.
$cp359Stem = "cooling_humidistat_moisture_demand_assignment"
$cp358StemForCp359 = "cooling_humidistat_case_entry"
$cp359PipelineStem = "purchased_air_$cp359Stem"
$cp359TypeStem = "PurchasedAirCalcCoolingHumidistatMoistureDemandAssignment"
$cp359Lifecycle = "purchased_air_calc_${cp359Stem}_lifecycle"
$cp359SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp359Sites = @(
    "read-zone-dehumidifying-setpoint-moisture-demand",
    "assign-local-zone-dehumidifying-setpoint-moisture-demand"
)

$cp359Module = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem.rs"
$cp359State = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\state.rs"
$cp359Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\transition.rs"
$cp359Release = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\release.rs"
$cp359Prefix = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\release\prefix_validation.rs"
$cp359Private = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\release\private_counterfactual.rs"
$cp359Runtime = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\release\runtime_validation.rs"
$cp359Snapshot = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\release\snapshot_validation.rs"
$cp359TestsRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\tests\mod.rs"
$cp359TestsRoutes = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\tests\routes.rs"
$cp359TestsIeee = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\tests\ieee.rs"
$cp359TestsOverflow = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\tests\overflow.rs"
$cp359TestsRelease = "crates\ep_runtime\src\ideal_loads\calc\$cp359Stem\tests\release.rs"
$cp359CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp359Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp359Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363 binding order"
$cp359BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp359Stem.rs"
$cp359BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp359Stem}_tests.rs"
$cp359BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp359ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp359InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp359InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp359InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp359InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp359Stem.rs"
$cp359CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp359Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp359Stem}_validation.rs"
$cp359CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp359.rs"
$cp359FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp359Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp359Stem}_fixture.rs"
$cp359PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp359Pipeline = "crates\ep_run\src\pipeline\$cp359PipelineStem.rs"
$cp359PipelineValidation = "crates\ep_run\src\pipeline\$cp359PipelineStem\validation.rs"
$cp359PipelineTests = "crates\ep_run\src\pipeline\$cp359PipelineStem\validation\tests.rs"
$cp359Serialization = "crates\ep_run\src\pipeline\$cp359PipelineStem\serialization.rs"
$cp359SnapshotSerialization = "crates\ep_run\src\pipeline\$cp359PipelineStem\serialization\snapshot.rs"
$cp359ArbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp359ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp358_assertions.rs"
$cp359ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp359_assertions.rs"
$cp359Audit = "scripts\quality\ideal-loads-structure-audit\cp359-cooling-humidistat-moisture-demand-assignment.ps1"

function Get-Cp359RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) {
        throw "$Description expected one anchor, found $($anchors.Count)"
    }
    $open = $Text.IndexOf("{", $anchors[0].Index)
    if ($open -lt 0) {
        throw "$Description has no opening brace"
    }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") {
            $depth += 1
        } elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring($anchors[0].Index, $index - $anchors[0].Index + 1)
            }
        }
    }
    throw "$Description has no complete brace block"
}

function Assert-Cp359BindingContract {
    param([string]$Text)
    $cp358 = $Text.IndexOf("let calculation_$cp358StemForCp359 =")
    $cp359 = $Text.IndexOf("let calculation_$cp359Stem =")
    $numerical = $Text.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
    if ($cp358 -lt 0 -or $cp359 -le $cp358 -or $numerical -le $cp359) {
        throw "Binding must execute CP358 then CP359 before numerical coupling"
    }
    if ([regex]::Matches(
            $Text,
            '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;'
        ).Count -ne 1) {
        throw "Binding must execute the exact CP359 release call once"
    }
    $dto = Get-Cp359RustBraceBlock -Text $Text.Substring($numerical) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP359 numerical DTO"
    if ($dto -match '(?i)cp359|humidistat_moisture_demand_assignment|zone_dehumidifying_setpoint') {
        throw "CP359 evidence must not enter DirectZonePurchasedAirCouplingInput"
    }
}

function Assert-Cp359PipelineContract {
    param([string]$Text)
    $boundary = [regex]::Match($Text, '(?m)^\s*#\[cfg\(test\)\]\s*$')
    if (-not $boundary.Success) {
        throw "Pipeline production/test boundary is missing"
    }
    $production = $Text.Substring(0, $boundary.Index)
    $execute = Get-Cp359RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn execute_rust_runtime\s*\(' -Description "pipeline runtime"
    $none = [regex]::Matches($execute, [regex]::Escape($cp359Lifecycle) + '\s*:\s*None').Count
    $some = [regex]::Matches($execute, 'let\s+' + [regex]::Escape($cp359Lifecycle) + '\s*=\s*Some\s*\(').Count
    $shorthand = [regex]::Matches($execute, '(?m)^\s*' + [regex]::Escape($cp359Lifecycle) + '\s*,\s*$').Count
    if ($none -ne 3 -or $some -ne 1 -or $shorthand -ne 1) {
        throw "Pipeline must expose one direct CP359 Some/shorthand and three non-direct None constructors"
    }
    $firewall = Get-Cp359RustBraceBlock -Text $production -AnchorPattern '(?m)^\s*fn validate_runtime_demand_provenance\s*\(' -Description "pipeline firewall"
    if ([regex]::Matches($firewall, 'result\s*\.\s*' + [regex]::Escape($cp359Lifecycle) + '\s*\.\s*is_some').Count -ne 1) {
        throw "Non-direct firewall must reject CP359 evidence exactly once"
    }
    $cp358Validator = $production.IndexOf("purchased_air_cooling_humidistat_case_entry::validate_direct_lifecycle(")
    $cp359Validator = $production.IndexOf("purchased_air_cooling_humidistat_moisture_demand_assignment::validate_direct_lifecycle(")
    if ($cp358Validator -lt 0 -or $cp359Validator -le $cp358Validator) {
        throw "Pipeline must validate CP359 after its CP358 predecessor"
    }
}

function Assert-Cp359ProductionHasNoPanics {
    param([string]$Path)
    $text = Read-RepoText -Path $Path
    $tests = [regex]::Match($text, '(?m)^\s*#\[cfg\(test\)\]\s*\r?\n\s*mod\s+tests\b')
    $production = if ($tests.Success) { $text.Substring(0, $tests.Index) } else { $text }
    if ($production -match '(?m)\b(?:expect|unwrap)\s*\(|panic!\s*\(') {
        throw "CP359 production path contains expect/unwrap/panic: $Path"
    }
}

foreach ($required in @(
        $cp359Module, $cp359State, $cp359Transition, $cp359Release, $cp359Prefix,
        $cp359Private, $cp359Runtime, $cp359Snapshot, $cp359TestsRoot,
        $cp359TestsRoutes, $cp359TestsIeee, $cp359TestsOverflow, $cp359TestsRelease,
        $cp359BindingAdapter, $cp359BindingTests, $cp359InitWitness, $cp359Coupled,
        $cp359CoupledTests, $cp359Fixture, $cp359Pipeline, $cp359PipelineValidation,
        $cp359PipelineTests, $cp359Serialization, $cp359SnapshotSerialization,
        $cp359ParentAssertions, $cp359ArbitraryAssertions, $cp359Audit
    )) {
    Assert-FileExists -Path $required -Description "CP359 structure"
}
foreach ($limited in @(
        $cp359Transition, $cp359Release, $cp359Prefix, $cp359Private, $cp359Runtime,
        $cp359Snapshot, $cp359TestsRoutes, $cp359TestsIeee, $cp359TestsOverflow,
        $cp359TestsRelease, $cp359Coupled, $cp359PipelineValidation,
        $cp359PipelineTests, $cp359Serialization, $cp359SnapshotSerialization,
        $cp359ArbitraryAssertions, $cp359Audit
    )) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP359 bounded structure"
}
Assert-LineLimit -Path $cp359ArbitraryRoot -Limit 1200 -Description "arbitrary-run integration"
foreach ($production in @(
        $cp359State, $cp359Transition, $cp359Release, $cp359Prefix, $cp359Private,
        $cp359Runtime, $cp359Snapshot, $cp359BindingAdapter, $cp359Coupled,
        $cp359Pipeline, $cp359PipelineValidation, $cp359Serialization,
        $cp359SnapshotSerialization
    )) {
    Assert-Cp359ProductionHasNoPanics -Path $production
}

# Exact boundary, routes, two-site H assignment, and private parametric semantics.
Assert-Contains -Path $cp359Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2229' -Description "CP359 source line"
Assert-Contains -Path $cp359Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2230' -Description "CP359 first excluded executable"
Assert-ExactStringArray -Path $cp359Module -Name "PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER" -Expected $cp359Sites -Description "CP359 two-site source order"
Assert-Contains -Path $cp359State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,\s*DehumidificationControlHumidistatMoistureDemandAssignmentExecuted,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "CP359 seven routes"
foreach ($counter in @(
        "dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_humidistat_moisture_demand_assignment_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
        "source_site_execution_count",
        "zone_dehumidifying_setpoint_moisture_demand_read_count",
        "zone_dehumidifying_setpoint_moisture_demand_assignment_count"
    )) {
    Assert-Contains -Path $cp359State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP359 counter '$counter'"
}
Assert-Contains -Path $cp359Transition -Pattern '(?s)Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted\s*=>.*?moisture_demand_assignment_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=.*?SOURCE_ORDER\.len\(\).*?moisture_demand_read_count\s*\+=\s*1;.*?moisture_demand_assignment_count\s*\+=\s*1;' -Description "CP359 H two-site counters"
Assert-Contains -Path $cp359Transition -Pattern '(?s)fn prepare_values.*?let value = operands\?\.zone_dehumidifying_setpoint_moisture_demand_kg_per_s;.*?raw: Some\(value\).*?assigned_value: Some\(value\).*?resulting: Some\(value\)' -Description "CP359 exact scalar pass-through"
Assert-Contains -Path $cp359Transition -Pattern 'next_transition_fits' -Description "CP359 overflow proof"
Assert-NotContains -Path $cp359Transition -Pattern 'Psy[A-Za-z_]*|DirectZonePurchasedAirCouplingInput|\.is_finite\(\)|\.clamp\(|f64::min|f64::max' -Description "CP359 arithmetic/service firewall"
Assert-Contains -Path $cp359Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "CP359 route partition"
Assert-Contains -Path $cp359Runtime -Pattern '(?s)h\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "CP359 checked 2H"
Assert-Contains -Path $cp359Runtime -Pattern '(?s)source_site_execution_count\s*==\s*expected_source_sites.*?moisture_demand_read_count\s*==\s*h.*?moisture_demand_assignment_count\s*==\s*h' -Description "CP359 H site equalities"
Assert-Contains -Path $cp359Prefix -Pattern 'PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot as Predecessor' -Description "CP359 exact CP358 predecessor"
Assert-Contains -Path $cp359Prefix -Pattern 'cp358_private_humidistat_counterfactual_links_to_direct_release' -Description "CP359 recursive CP358 owner"
Assert-Contains -Path $cp359Private -Pattern '(?s)private_humidistat_counterfactual_from_direct_release.*?pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s:\s*f64' -Description "CP359 explicit private scalar"
Assert-Contains -Path $cp359Private -Pattern '(?s)cp358_private_humidistat_counterfactual_from_direct_release.*?Operands\s*\{.*?pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s.*?to_bits\(\)' -Description "CP359 canonical parametric bridge"
Assert-NotContains -Path $cp359Private -Pattern '\.is_finite\(\)|\.clamp\(|f64::min|f64::max|DirectZonePurchasedAirCouplingInput' -Description "CP359 private scalar noncoercion"

foreach ($test in @(
        [PSCustomObject]@{ Path = $cp359TestsRoutes; Pattern = 'source_boundary_and_seven_route_algebra_are_exact' },
        [PSCustomObject]@{ Path = $cp359TestsRoutes; Pattern = 'active_operand_contract_is_transactional_and_route_exact' },
        [PSCustomObject]@{ Path = $cp359TestsIeee; Pattern = 'humidistat_assignment_preserves_every_sampled_ieee_bit' },
        [PSCustomObject]@{ Path = $cp359TestsIeee; Pattern = 'snapshot_matcher_distinguishes_signed_zero_and_nan_payloads' },
        [PSCustomObject]@{ Path = $cp359TestsOverflow; Pattern = 'every_counter_overflow_rejects_without_mutation' },
        [PSCustomObject]@{ Path = $cp359TestsOverflow; Pattern = 'two_site_increment_preflight_rejects_max_minus_one' },
        [PSCustomObject]@{ Path = $cp359TestsRelease; Pattern = 'public_direct_routes_are_complete_null_and_private_h_is_parametric' },
        [PSCustomObject]@{ Path = $cp359TestsRelease; Pattern = 'corruption_replay_and_witness_redistribution_reject_without_mutation' },
        [PSCustomObject]@{ Path = $cp359BindingTests; Pattern = 'scheduled_binding_places_cp359_after_cp358_without_reading_moisture_demand' },
        [PSCustomObject]@{ Path = $cp359BindingTests; Pattern = 'scheduled_binding_preserves_u_n_p_skips_and_rejects_private_case_routes' },
        [PSCustomObject]@{ Path = $cp359CoupledTests; Pattern = 'cp359_coupled_direct_none_is_exact_skip_and_cp345_remains_numerical_owner' },
        [PSCustomObject]@{ Path = $cp359Coupled; Pattern = 'partition_overflow_and_source_corruption_fail_closed' },
        [PSCustomObject]@{ Path = $cp359SnapshotSerialization; Pattern = 'direct_none_release_serializes_null_moisture_demand_values_and_bits' },
        [PSCustomObject]@{ Path = $cp359SnapshotSerialization; Pattern = 'finite_and_nonfinite_parametric_characterization_preserves_exact_bits' },
        [PSCustomObject]@{ Path = $cp359PipelineTests; Pattern = 'route_partition_overflow_fails_closed' },
        [PSCustomObject]@{ Path = $cp359PipelineTests; Pattern = 'source_and_read_assignment_counter_mismatches_fail_closed' },
        [PSCustomObject]@{ Path = $cp359PipelineTests; Pattern = 'inherited_u_n_p_and_c0_routes_validate' },
        [PSCustomObject]@{ Path = $cp359PipelineTests; Pattern = 'self_consistent_q_h_and_csh_routes_are_rejected' },
        [PSCustomObject]@{ Path = $cp359PipelineTests; Pattern = 'snapshot_identity_route_and_numeric_corruptions_fail_closed' }
    )) {
    Assert-Contains -Path $test.Path -Pattern $test.Pattern -Description "CP359 regression '$($test.Pattern)'"
}

# Public direct admission, binding/pipeline placement, JSON, and numerical nonfeed.
$cp359ReleaseText = Read-RepoText -Path $cp359Release
$cp359PublicRelease = Get-Cp359RustBraceBlock -Text $cp359ReleaseText -AnchorPattern '(?m)^pub fn advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment\s*\(' -Description "CP359 public release"
if ($cp359PublicRelease -notmatch '(?s)runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*predecessor_cp358:\s*Predecessor,\s*\)' -or $cp359PublicRelease -match 'f64|moisture_demand_kg_per_s:\s*') {
    throw "CP359 public direct release must accept CP358 only and no moisture-demand operand"
}
if ($cp359PublicRelease -notmatch 'system\.dehumidification_control_type\s*!=\s*DehumidificationControlType::None' -or $cp359PublicRelease -notmatch '(?s)advance_cooling_humidistat_moisture_demand_assignment_state\(.*?None') {
    throw "CP359 public direct release must be a complete-null None-selector skip"
}
if ($cp359PublicRelease -match 'PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply') {
    throw "CP359 public release admits a forbidden numerical/demand feed"
}
$cp359BindingText = Read-RepoText -Path $cp359Binding
$cp359PipelineRootText = Read-RepoText -Path $cp359PipelineRoot
Assert-Cp359BindingContract -Text $cp359BindingText
Assert-Cp359PipelineContract -Text $cp359PipelineRootText
Assert-Contains -Path $cp359CalcRoot -Pattern ('mod ' + [regex]::Escape($cp359Stem) + ';') -Description "CP359 calc module"
Assert-Contains -Path $cp359BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + [regex]::Escape($cp359Stem)) -Description "CP359 binding adapter"
Assert-Contains -Path $cp359ScheduledOutput -Pattern ('pub calculation_' + [regex]::Escape($cp359Stem) + ':') -Description "CP359 scheduled output"
Assert-Contains -Path $cp359BindingTestsRoot -Pattern ([regex]::Escape("${cp359Stem}_tests.rs")) -Description "CP359 binding tests"
Assert-Contains -Path $cp359InitState -Pattern $cp359Stem -Description "CP359 init state"
Assert-Contains -Path $cp359InitUnit -Pattern $cp359Stem -Description "CP359 unit state"
Assert-Contains -Path $cp359InitWitnessRoot -Pattern $cp359Stem -Description "CP359 witness module"
Assert-Contains -Path $cp359CoupledRoot -Pattern ('mod ' + [regex]::Escape($cp359Stem) + '_validation;') -Description "CP359 coupled validator"
Assert-Contains -Path $cp359Coupled -Pattern 'PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState' -Description "coupled CP358 predecessor"
Assert-Contains -Path $cp359Coupled -Pattern '(?s)assignments\s*\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER\.len\(\)' -Description "coupled checked 2H source count"
Assert-NotContains -Path $cp359Coupled -Pattern 'complete_direct_zone_purchased_air_coupling|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply' -Description "coupled CP359 numerical firewall"
Assert-Contains -Path $cp359FixtureRoot -Pattern $cp359Stem -Description "CP359 fixture registration"
Assert-Contains -Path $cp359Fixture -Pattern ('calculation_' + [regex]::Escape($cp359Stem) + '_snapshot') -Description "CP359 output fixture"
Assert-Contains -Path $cp359PipelineRoot -Pattern ('mod ' + [regex]::Escape($cp359PipelineStem) + ';') -Description "CP359 pipeline module"
Assert-Contains -Path $cp359PipelineRoot -Pattern ('"' + $cp359Lifecycle + '":\s*result\s*\.' + $cp359Lifecycle) -Description "CP359 lifecycle JSON"
Assert-Contains -Path $cp359PipelineValidation -Pattern 'humidistat_case_entry_cp358' -Description "pipeline CP358 predecessor"
Assert-Contains -Path $cp359PipelineValidation -Pattern 'zone_dehumidifying_setpoint_moisture_demand_read_count' -Description "pipeline read counter"
Assert-Contains -Path $cp359PipelineValidation -Pattern 'zone_dehumidifying_setpoint_moisture_demand_assignment_count' -Description "pipeline assignment counter"
Assert-Contains -Path $cp359PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp363_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp359ParentAssertions -Pattern 'mod cp359_assertions;' -Description "arbitrary CP359 module delegation"
Assert-Contains -Path $cp359ParentAssertions -Pattern 'cp359_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP359 direct delegation"
Assert-Contains -Path $cp359ParentAssertions -Pattern 'cp359_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP359 non-direct delegation"
Assert-Contains -Path $cp359ArbitraryAssertions -Pattern 'CP359_KEY' -Description "arbitrary CP359 lifecycle"
Assert-Contains -Path $cp359ArbitraryAssertions -Pattern 'cp360_assertions::assert_direct\(runtime, results\)' -Description "CP360 numerical nonfeed delegation"
Assert-Contains -Path $cp359SnapshotSerialization -Pattern 'json_number' -Description "CP359 finite JSON projection"
Assert-Contains -Path $cp359SnapshotSerialization -Pattern '_ieee_bits' -Description "CP359 authoritative IEEE sidecars"
Assert-Contains -Path $cp359SnapshotSerialization -Pattern 'to_bits' -Description "CP359 exact numeric JSON evidence"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$cp359AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp359CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp359AlgorithmAddenda = [regex]::Matches($cp359AlgorithmText, '(?m)^\s*"CP359 supersedes only CP358[^"\r\n]+",\s*$')
$cp359CapabilityAddenda = [regex]::Matches($cp359CapabilityText, '(?m)^\s*"CP359 additionally requires[^"\r\n]+",\s*$')
if ($cp359AlgorithmAddenda.Count -ne 2 -or $cp359CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP359 addenda"
}
foreach ($claim in @($cp359AlgorithmAddenda) + @($cp359CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp359SourceHash, 'physical executable line 2229', 'MdotZnDehumidSP',
            'physical executable line 2230', 'line 2245', $cp359Sites[0],
            $cp359Sites[1], 'T=U\+N\+P\+C0\+Q\+H\+CSH',
            'S=C0\+Q\+H\+CSH=R=G\+F\+L', 'A=F\+L',
            'source_site_execution_count=2H', 'C0=S', 'Q=H=CSH=0', 'CP358',
            'sole predecessor owner', 'explicit pre-sampled', 'no retained authoritative owner',
            'no live Zone moisture-demand service', 'CP319', 'not a predecessor',
            'CP358-to-CP359-to-unchanged-numerical', $cp359Lifecycle,
            'first/last supply-humidity bits remain unchanged', 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '297 total', '240 public', '57 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP359 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp359Stem/release\.rs::advance_direct_no_oa_calc_$cp359Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp359Stem\.rs::purchased_air_calc_${cp359Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp359Stem\.rs::${cp359TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp359Stem\.rs::${cp359TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp359AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP359 target count failed for '$($target.Pattern)'"
    }
}
$cp359Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^CP359 now maps only.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP359 Source-Ordered Cooling Humidistat Moisture-Demand Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP359 Humidistat Moisture-Demand Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP359 Humidistat Moisture-Demand Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP359 Humidistat Moisture-Demand Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp359Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP359 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp359SourceHash, '2229', 'MdotZnDehumidSP', '2230', 'first excluded',
            '2245', $cp359Sites[0], $cp359Sites[1], 'U/N/P/C0/Q/H/CSH',
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L', 'source_site_execution_count\s*=\s*2H',
            'C0\s*=\s*S', 'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP358',
            '(?s)(?:sole|solely).*?predecessor', 'explicit pre-sampled',
            'no retained\s+authoritative', 'no live\s+Zone moisture-demand service',
            'CP319', '(?s)not.{0,80}(?:predecessor|owner|feed)',
            'CP358-to-CP359-to-unchanged-numerical', $cp359Lifecycle,
            'first/last', 'CP345', '32\s+algorithms', '293\s+routines',
            '58\s+[^,\r\n]*state[_-]mapped', '235\s+[^,\r\n]*source[_-]mapped',
            '170\s+required', '297\s+total', '240\s+public', '57\s+internal',
            'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP359 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP359\b' -Description "CP359 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP359 supersedes only CP358' -Description "generated CP359 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP359 additionally requires' -Description "generated CP359 capability addendum"

# Exact historical binding/firewall/inventory ranges and master reachability.
$cp359BindingHistory = @(326) + @(329..358)
foreach ($number in $cp359BindingHistory) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'calculation_cooling_humidistat_moisture_demand_assignment' -Description "historical CP359 binding/whitelist"
}
foreach ($number in 334..358) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp363_lifecycle_evidence' -Description "historical CP363 firewall"
}
foreach ($number in 335..358) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| executable script records \| 301 \|')) -Description "historical generated total"
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| internal scripts \| 61 \|')) -Description "historical generated internal"
}
foreach ($number in 337..358) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$number-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 301' -Description "historical script inventory total"
}
$cp359MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp358AuditIndexForCp359 = $cp359MainAuditText.IndexOf("cp358-cooling-humidistat-case-entry.ps1")
$cp359AuditIndex = $cp359MainAuditText.IndexOf("cp359-cooling-humidistat-moisture-demand-assignment.ps1")
$cp359CompletionIndex = $cp359MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp358AuditIndexForCp359 -lt 0 -or $cp359AuditIndex -le $cp358AuditIndexForCp359 -or $cp359CompletionIndex -le $cp359AuditIndex) {
    throw "Master audit must dot-source CP359 after CP358 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 301' -Description "CP359 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP359 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp359-' -Description "CP359 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp359-cooling-humidistat-moisture-demand-assignment\.ps1::dot_sources' -Description "CP359 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| executable script records \| 301 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| internal scripts \| 61 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP359 Humidistat moisture-demand assignment structure audit passed."
