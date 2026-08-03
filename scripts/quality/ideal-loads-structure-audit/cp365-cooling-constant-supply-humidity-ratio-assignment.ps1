# CP365 maps only PurchasedAirManager.cc line 2235's raw assignment.
$cp365Stem = "cooling_constant_supply_humidity_ratio_assignment"
$cp364StemForCp365 = "cooling_constant_supply_humidity_ratio_case_entry"
$cp365PipelineStem = "purchased_air_$cp365Stem"
$cp365TypeStem = "PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignment"
$cp365Lifecycle = "purchased_air_calc_${cp365Stem}_lifecycle"
$cp365SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp365SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp365Sites = @(
    "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-constant-supply-humidity-ratio-assignment",
    "assign-purchased-air-supply-humidity-ratio-for-constant-supply-humidity-ratio-case"
)
$cp365Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp365Module = "crates\ep_runtime\src\ideal_loads\calc\$cp365Stem.rs"
$cp365ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp365Stem"
$cp365State = "$cp365ModuleRoot\state.rs"
$cp365Transition = "$cp365ModuleRoot\transition.rs"
$cp365Predecessor = "$cp365ModuleRoot\transition\predecessor.rs"
$cp365Release = "$cp365ModuleRoot\release.rs"
$cp365Prefix = "$cp365ModuleRoot\release\prefix_validation.rs"
$cp365Private = "$cp365ModuleRoot\release\private_counterfactual.rs"
$cp365Runtime = "$cp365ModuleRoot\release\runtime_validation.rs"
$cp365Snapshot = "$cp365ModuleRoot\release\snapshot_validation.rs"
$cp365Tests = "$cp365ModuleRoot\tests\mod.rs"
$cp365CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp365Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp365BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp365Stem.rs"
$cp365BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp365BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp365Stem}_tests.rs"
$cp365ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp365InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp365InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp365InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp365InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp365Stem.rs"
$cp365CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp365Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp365Stem}_validation.rs"
$cp365CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp365CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp365.rs"
$cp365FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp365Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp365Stem}_fixture.rs"
$cp365PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp365Pipeline = "crates\ep_run\src\pipeline\$cp365PipelineStem.rs"
$cp365PipelineValidation = "crates\ep_run\src\pipeline\$cp365PipelineStem\validation.rs"
$cp365PipelineTests = "crates\ep_run\src\pipeline\$cp365PipelineStem\validation\tests.rs"
$cp365Serialization = "crates\ep_run\src\pipeline\$cp365PipelineStem\serialization.rs"
$cp365SnapshotSerialization = "crates\ep_run\src\pipeline\$cp365PipelineStem\serialization\snapshot.rs"
$cp365ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp364_assertions.rs"
$cp365ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp365_assertions.rs"
$cp365ArbitraryRoot = "crates\ep_run\tests\arbitrary_run_ideal_loads.rs"
$cp365Audit = "scripts\quality\ideal-loads-structure-audit\cp365-cooling-constant-supply-humidity-ratio-assignment.ps1"

function Assert-Cp365TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP365 $Description missing" }
}

function Assert-Cp365TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP365 $Description unexpectedly present" }
}

function Get-Cp365RustBraceBlock {
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

function Assert-Cp365ProductionHasNoPanics {
    param([string]$Path)
    if ((Read-RepoText -Path $Path) -match '(?m)\b(?:expect|unwrap)\s*\(|panic!\s*\(') {
        throw "CP365 production path contains expect/unwrap/panic: $Path"
    }
}

foreach ($required in @(
        $cp365Source, $cp365Module, $cp365State, $cp365Transition, $cp365Predecessor,
        $cp365Release, $cp365Prefix, $cp365Private, $cp365Runtime, $cp365Snapshot,
        $cp365Tests, $cp365BindingAdapter, $cp365BindingTestsRoot, $cp365BindingTests,
        $cp365InitWitness, $cp365Coupled, $cp365CoupledTestsRoot, $cp365CoupledTests,
        $cp365Fixture, $cp365Pipeline, $cp365PipelineValidation, $cp365PipelineTests,
        $cp365Serialization, $cp365SnapshotSerialization, $cp365ParentAssertions,
        $cp365ArbitraryAssertions, $cp365Audit
    )) {
    Assert-FileExists -Path $required -Description "CP365 structure"
}
$cp365CalcFiles = @($cp365Module) + @(
    Get-ChildItem -LiteralPath $cp365ModuleRoot -Recurse -File -Filter "*.rs" |
        ForEach-Object { $_.FullName }
)
$cp365TestFiles = @($cp365CalcFiles | Where-Object {
        $_ -match '(?:_tests\.rs$|[\\/]tests[\\/])'
    })
if ($cp365TestFiles.Count -eq 0) { throw "CP365 calc regression tests missing" }
$cp365ProductionFiles = @($cp365CalcFiles | Where-Object { $cp365TestFiles -notcontains $_ })
$cp365Limited = @($cp365CalcFiles) + @(
    $cp365BindingAdapter, $cp365InitWitness, $cp365Coupled, $cp365CoupledTests,
    $cp365Pipeline, $cp365PipelineValidation, $cp365PipelineTests,
    $cp365Serialization, $cp365SnapshotSerialization, $cp365ArbitraryAssertions,
    $cp365Audit
)
foreach ($limited in $cp365Limited | Select-Object -Unique) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP365 bounded structure"
}
Assert-LineLimit -Path $cp365ArbitraryRoot -Limit 1200 -Description "arbitrary-run integration"
foreach ($production in @($cp365ProductionFiles) + @(
        $cp365BindingAdapter, $cp365InitWitness, $cp365Coupled, $cp365Pipeline,
        $cp365PipelineValidation, $cp365Serialization, $cp365SnapshotSerialization
    ) | Select-Object -Unique) {
    Assert-Cp365ProductionHasNoPanics -Path $production
}

$cp365CalcText = ($cp365CalcFiles | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
$cp365SemanticText = (@(
        $cp365Tests, $cp365BindingTests, $cp365CoupledTests, $cp365PipelineTests,
        $cp365SnapshotSerialization, $cp365ArbitraryAssertions
    ) | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"

# Pinned line 2235, first excluded break, two exact sites, and seven routes.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp365Source).Hash -cne $cp365SourceHash) {
    throw "CP365 pinned PurchasedAirManager.cc hash drifted"
}
$cp365SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp365Source
if ($cp365SourceLines[2233].Trim() -cne 'case HumControl::ConstantSupplyHumidityRatio: {' -or
    $cp365SourceLines[2234].Trim() -cne 'PurchAir.SupplyHumRat = PurchAir.MinCoolSuppAirHumRat;' -or
    $cp365SourceLines[2235].Trim() -cne '} break;' -or
    $cp365SourceLines[2236].Trim() -cne 'default: {' -or
    $cp365SourceLines[2244].Trim() -cne 'if (HeatOn) {') {
    throw "CP365 physical lines 2234 through 2237 or 2245 drifted"
}
Assert-Contains -Path $cp365Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2235' -Description "CP365 source line"
Assert-Contains -Path $cp365Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2236' -Description "CP365 first excluded source"
Assert-ExactStringArray -Path $cp365Module -Name "PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER" -Expected $cp365Sites -Description "CP365 two-site source order"
Assert-Contains -Path $cp365State -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,\s*DehumidificationControlHumidistatCaseCompletedSkip,\s*DehumidificationControlConstantSupplyHumidityRatioAssigned' -Description "CP365 seven routes"
foreach ($counter in @(
        "dehumidification_control_constant_supply_humidity_ratio_assignment_count",
        "source_site_execution_count",
        "minimum_cooling_supply_air_humidity_ratio_read_count",
        "supply_humidity_ratio_assignment_count"
    )) {
    Assert-Contains -Path $cp365State -Pattern ('pub ' + $counter + ':\s*usize') -Description "CP365 counter '$counter'"
}
foreach ($field in @(
        "dehumidification_control_constant_supply_humidity_ratio_assignment_executed",
        "minimum_cooling_supply_air_humidity_ratio_read",
        "minimum_cooling_supply_air_humidity_ratio",
        "supply_humidity_ratio_assigned",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio"
    )) {
    Assert-Contains -Path $cp365Module -Pattern ('pub ' + $field + ':') -Description "CP365 snapshot field '$field'"
}
Assert-Contains -Path $cp365Transition -Pattern '(?s)DehumidificationControlConstantSupplyHumidityRatioAssigned\s*=>\s*\{.*?assignment_count\s*\+=\s*1;.*?source_site_execution_count\s*\+=.*?SOURCE_ORDER.*?\.len\(\);.*?minimum_cooling_supply_air_humidity_ratio_read_count\s*\+=\s*1;.*?supply_humidity_ratio_assignment_count\s*\+=\s*1;' -Description "CP365 CSH two-site counters"
Assert-Contains -Path $cp365Runtime -Pattern '(?s)csh\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER\.len\(\)' -Description "CP365 checked 2*CSH"
Assert-Contains -Path $cp365Runtime -Pattern 'route_partition\s*==\s*state\.transition_count' -Description "CP365 route partition"
Assert-Contains -Path $cp365Runtime -Pattern 'selected\s*==\s*recursively_witnessed' -Description "CP365 recursive selected partition"
Assert-Contains -Path $cp365Runtime -Pattern 'state\.minimum_cooling_supply_air_humidity_ratio_read_count\s*==\s*csh' -Description "CP365 RHS counter"
Assert-Contains -Path $cp365Runtime -Pattern 'state\.supply_humidity_ratio_assignment_count\s*==\s*csh' -Description "CP365 LHS counter"

# Exact CP364 predecessor, lazy direct release, selected typed owner, and bit-copy semantics.
Assert-Contains -Path $cp365Transition -Pattern 'PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor' -Description "exact CP364 predecessor type"
Assert-Contains -Path $cp365Predecessor -Pattern 'PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER' -Description "CP364 provenance"
Assert-Contains -Path $cp365Private -Pattern 'private_constant_supply_humidity_ratio_case_entry_counterfactual_from_direct_release as cp364_private_constant_supply_counterfactual_from_direct_release' -Description "canonical CP364 private bridge"
Assert-Contains -Path $cp365Private -Pattern 'private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release' -Description "canonical CP365 private assignment"
Assert-Contains -Path $cp365Private -Pattern 'private_constant_supply_humidity_ratio_assignment_counterfactual_links_to_direct_release' -Description "private CP365 bridge proof"
foreach ($pattern in @(
        'completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent',
        'cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release',
        'cooling_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact',
        'cooling_constant_supply_humidity_ratio_case_entry_latest_witness'
    )) {
    Assert-Cp365TextContains -Text $cp365CalcText -Pattern $pattern -Description "recursive CP364 owner '$pattern'"
}
$cp365ReleaseText = Read-RepoText -Path $cp365Release
$cp365PublicRelease = Get-Cp365RustBraceBlock -Text $cp365ReleaseText -AnchorPattern '(?m)^pub fn advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment\s*\(' -Description "CP365 public release"
Assert-Cp365TextContains -Text $cp365PublicRelease -Pattern 'system:\s*&IdealLoadsAirSystem' -Description "public selected system"
Assert-Cp365TextContains -Text $cp365PublicRelease -Pattern 'predecessor_cp364:\s*Predecessor' -Description "public CP364 predecessor"
Assert-Cp365TextContains -Text $cp365PublicRelease -Pattern '(?s)advance_cooling_constant_supply_humidity_ratio_assignment_state\(.*?retained_predecessor,\s*None,' -Description "public lazy null operand"
Assert-Cp365TextNotContains -Text $cp365PublicRelease -Pattern ':\s*f64|system\.minimum_cooling_supply_air_humidity_ratio|minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner' -Description "public RHS read or numeric argument"
Assert-Contains -Path $cp365Prefix -Pattern 'let minimum\s*=\s*system\.minimum_cooling_supply_air_humidity_ratio;' -Description "selected immutable typed owner"
Assert-Contains -Path $cp365Prefix -Pattern 'minimum\.is_finite\(\)\.then_some\(minimum\)' -Description "private finite-only owner gate"
foreach ($path in @($cp365Prefix, $cp365Private)) {
    Assert-NotContains -Path $path -Pattern 'PurchasedAirSizedLimits|\bsizing\b|CP319|cp319|CP355|cp355|CP361|cp361|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply' -Description "CP365 substitute owner"
}
Assert-Contains -Path $cp365Transition -Pattern '(?s)minimum_cooling_supply_air_humidity_ratio:\s*value\.minimum,.*?assigned_supply_humidity_ratio:\s*value\.minimum,.*?resulting_supply_humidity_ratio:\s*value\.minimum,' -Description "raw RHS-to-LHS bit copy"
Assert-Contains -Path $cp365Transition -Pattern '(?s)fn prepare_value\(route: Route, minimum: Option<f64>\).*?ConstantSupplyHumidityRatioAssigned.*?minimum:\s*Some\(minimum\?\).*?minimum\.is_none\(\)\.then_some' -Description "route-exact lazy operand presence"
Assert-Contains -Path $cp365Snapshot -Pattern '(?s)minimum\.to_bits\(\)\s*==\s*assigned\.to_bits\(\)\s*&&\s*assigned\.to_bits\(\)\s*==\s*resulting\.to_bits\(\)' -Description "bit-exact assignment invariant"
Assert-Contains -Path $cp365Snapshot -Pattern '(?s)pub\(in crate::ideal_loads::calc\) fn snapshots_match_bit_exact.*?option_bits_match' -Description "bit-exact snapshot matcher"
Assert-NotContains -Path $cp365Transition -Pattern 'f64::(?:min|max)|\.min\(|\.max\(|\.clamp\(|mul_add|Psy[A-Za-z_]*|energyplus_psy_|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand' -Description "CP365 source-local arithmetic or numerical feed"

# Semantic regressions, integration registration, JSON, and numerical nonfeed.
foreach ($test in @(
        "source_boundary_two_sites_and_seven_route_algebra_are_exact",
        "raw_assignment_preserves_every_binary64_pattern_bit_exact",
        "bit_exact_matcher_handles_nan_payloads_and_signed_zero",
        "operand_presence_is_route_exact_and_transactional",
        "forged_provenance_selector_prefix_and_one_hot_lineage_are_rejected",
        "every_counter_overflow_rejects_without_mutation",
        "two_site_increment_preflight_rejects_max_minus_one",
        "binding_orders_cp365_after_cp364_as_an_exact_null_c0_skip",
        "binding_cp365_preserves_u_n_p_and_c0_complete_null_routes",
        "cp365_lifecycle_matches_cp364_as_a_complete_null_direct_skip",
        "cp365_route_source_latest_and_predecessor_corruption_are_rejected",
        "cp365_snapshot_matching_rejects_signed_zero_numeric_corruption",
        "missing_direct_lifecycle_fails_closed",
        "checked_partitions_and_source_counts_fail_closed",
        "direct_release_is_complete_null_and_matches_immediate_predecessor",
        "forged_selector_prefix_lineage_and_numeric_payload_fail_closed",
        "direct_none_release_serializes_complete_null_skip",
        "active_assignment_serializes_finite_value_and_authoritative_bits",
        "defensive_nonfinite_characterization_keeps_bits_but_projects_null_number"
    )) {
    Assert-Cp365TextContains -Text $cp365SemanticText -Pattern ('(?m)fn\s+' + $test + '\s*\(') -Description "semantic regression '$test'"
}
Assert-Contains -Path $cp365PipelineTests -Pattern '(?s)checked_partitions_and_source_counts_fail_closed.*?usize::MAX.*?validate_route_partition.*?dehumidification_control_constant_supply_humidity_ratio_assignment_count\s*=\s*1.*?validate_source_counters' -Description "pipeline partition/source corruption"
Assert-Contains -Path $cp365PipelineTests -Pattern '(?s)forged_selector_prefix_lineage_and_numeric_payload_fail_closed.*?predecessor_dehumidification_control_type.*?predecessor_positive_supply_mass_flow_body_entered.*?dehumidification_control_none_case_completed_skip.*?minimum_cooling_supply_air_humidity_ratio\s*=\s*Some' -Description "pipeline selector/prefix/lineage/numeric forge"
Assert-Contains -Path $cp365SnapshotSerialization -Pattern '_ieee_bits' -Description "authoritative IEEE JSON sidecars"
Assert-Contains -Path $cp365SnapshotSerialization -Pattern 'json_number' -Description "finite-only JSON number projection"

$cp365BindingText = Read-RepoText -Path $cp365Binding
$cp364BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_${cp364StemForCp365} =")
$cp365BindingIndex = $cp365BindingText.IndexOf("let calculation_${cp365Stem} =")
$cp366BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_break =")
$cp367BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =")
$cp368BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_case_break =")
$cp369BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =")
$cp376BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp365 = $cp365BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp365NumericalIndex = $cp365BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp364BindingIndexForCp365 -lt 0 -or $cp365BindingIndex -le $cp364BindingIndexForCp365 -or $cp366BindingIndexForCp365 -le $cp365BindingIndex -or $cp367BindingIndexForCp365 -le $cp366BindingIndexForCp365 -or $cp368BindingIndexForCp365 -le $cp367BindingIndexForCp365 -or
    $cp365NumericalIndex -le $cp368BindingIndexForCp365 -or
    $cp369BindingIndexForCp365 -le $cp368BindingIndexForCp365 -or
    $cp370BindingIndexForCp365 -le $cp369BindingIndexForCp365 -or
    $cp371BindingIndexForCp365 -le $cp370BindingIndexForCp365 -or
    $cp372BindingIndexForCp365 -le $cp371BindingIndexForCp365 -or
    $cp373BindingIndexForCp365 -le $cp372BindingIndexForCp365 -or
    $cp374BindingIndexForCp365 -le $cp373BindingIndexForCp365 -or
    $cp375BindingIndexForCp365 -le $cp374BindingIndexForCp365 -or
    $cp376BindingIndexForCp365 -le $cp375BindingIndexForCp365 -or $cp377BindingIndexForCp365 -le $cp376BindingIndexForCp365 -or $cp378BindingIndexForCp365 -le $cp377BindingIndexForCp365 -or $cp379BindingIndexForCp365 -le $cp378BindingIndexForCp365 -or $cp380BindingIndexForCp365 -le $cp379BindingIndexForCp365 -or $cp381BindingIndexForCp365 -le $cp380BindingIndexForCp365 -or $cp382BindingIndexForCp365 -le $cp381BindingIndexForCp365 -or $cp383BindingIndexForCp365 -le $cp382BindingIndexForCp365 -or $cp384BindingIndexForCp365 -le $cp383BindingIndexForCp365 -or $cp385BindingIndexForCp365 -le $cp384BindingIndexForCp365 -or $cp365NumericalIndex -le $cp385BindingIndexForCp365) {
    throw "Binding must execute CP364 then CP365 then CP366 then CP367 then CP368 before numerical coupling"
}
$cp365Dto = Get-Cp365RustBraceBlock -Text $cp365BindingText.Substring($cp365NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP365 numerical DTO"
if ($cp365Dto -match '(?i)cp365|constant_supply_humidity_ratio_assignment') {
    throw "CP365 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp365CalcRoot -Pattern $cp365Stem -Description "calc registration"
Assert-Contains -Path $cp365BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + $cp365Stem) -Description "binding adapter"
Assert-Contains -Path $cp365ScheduledOutput -Pattern ('pub calculation_' + $cp365Stem + ':') -Description "scheduled output"
Assert-Contains -Path $cp365BindingTestsRoot -Pattern $cp365Stem -Description "binding-test registration"
Assert-Contains -Path $cp365InitState -Pattern $cp365Stem -Description "runtime state"
Assert-Contains -Path $cp365InitUnit -Pattern $cp365Stem -Description "unit state"
Assert-Contains -Path $cp365InitWitnessRoot -Pattern $cp365Stem -Description "witness registration"
Assert-Contains -Path $cp365CoupledRoot -Pattern ('mod ' + $cp365Stem + '_validation;') -Description "coupled validator"
Assert-Contains -Path $cp365Coupled -Pattern ('calculation_' + $cp364StemForCp365) -Description "coupled CP364 predecessor"
Assert-Contains -Path $cp365Coupled -Pattern 'snapshots_match_bit_exact' -Description "coupled bit-exact validation"
Assert-Contains -Path $cp365FixtureRoot -Pattern $cp365Stem -Description "fixture registration"
Assert-Contains -Path $cp365Fixture -Pattern ('calculation_' + $cp365Stem + '_snapshot') -Description "output fixture"
Assert-Contains -Path $cp365CoupledTestsRoot -Pattern 'coupled_runtime_tests_cp365' -Description "coupled-test registration"
Assert-Contains -Path $cp365PipelineRoot -Pattern ('mod ' + $cp365PipelineStem + ';') -Description "pipeline module"
Assert-Contains -Path $cp365PipelineRoot -Pattern ('"' + $cp365Lifecycle + '":\s*result\s*\.' + $cp365Lifecycle) -Description "lifecycle JSON"
Assert-Contains -Path $cp365PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp408_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp365ParentAssertions -Pattern 'mod cp365_assertions;' -Description "arbitrary delegation module"
Assert-Contains -Path $cp365ParentAssertions -Pattern 'cp365_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp365ParentAssertions -Pattern 'cp365_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp365ParentAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP364 relinquishes terminal nonfeed"
Assert-Contains -Path $cp365ArbitraryAssertions -Pattern 'mod cp366_assertions;' -Description "CP366 arbitrary delegation module"
Assert-Contains -Path $cp365ArbitraryAssertions -Pattern 'cp366_assertions::assert_direct\(runtime, results\)' -Description "CP366 arbitrary direct delegation"
Assert-Contains -Path $cp365ArbitraryAssertions -Pattern 'cp366_assertions::assert_non_direct\(runtime\)' -Description "CP366 arbitrary non-direct delegation"
Assert-NotContains -Path $cp365ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP365 relinquishes terminal nonfeed to CP366"
Assert-Contains -Path $cp365ArbitraryAssertions -Pattern 'runtime\.contains_key\(CP365_KEY\)' -Description "CP365 non-direct key"
Assert-Contains -Path $cp365ArbitraryAssertions -Pattern 'runtime\[CP365_KEY\]\.is_null\(\)' -Description "CP365 non-direct null"

# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$cp365AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp365CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp365AlgorithmAddenda = [regex]::Matches($cp365AlgorithmText, '(?m)^\s*"CP365 supersedes only CP364[^"\r\n]+",\s*$')
$cp365CapabilityAddenda = [regex]::Matches($cp365CapabilityText, '(?m)^\s*"CP365 additionally requires[^"\r\n]+",\s*$')
if ($cp365AlgorithmAddenda.Count -ne 2 -or $cp365CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP365 addenda"
}
foreach ($claim in @($cp365AlgorithmAddenda) + @($cp365CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp365SourceCommit, $cp365SourceHash, 'physical executable line 2235',
            'PurchAir\.SupplyHumRat = PurchAir\.MinCoolSuppAirHumRat', $cp365Sites[0],
            $cp365Sites[1], 'line 2236', 'CP366', 'line 2237', 'line 2245',
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=2\*CSH', 'C0=S',
            'Q=H=CSH=0', 'does not read or validate', 'complete-null',
            'CP364', 'sole predecessor owner', 'minimum_cooling_supply_air_humidity_ratio',
            '\.is_finite\(\)', 'raw binary64 copy|raw-copies binary64',
            'signed zero', 'NaN/infinity', 'CP364-to-CP365-to-unchanged-numerical',
            $cp365Lifecycle, 'DirectZonePurchasedAirCouplingInput', 'CP345',
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', 'Roadmap', '303 total', '240 public', '63 internal', 'zero unused'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP365 spec addendum missing '$pattern'" }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp365Stem/release\.rs::advance_direct_no_oa_calc_$cp365Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp365Stem\.rs::purchased_air_calc_${cp365Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp365Stem\.rs::${cp365TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp365Stem\.rs::${cp365TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp365AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP365 target count failed for '$($target.Pattern)'"
    }
}
$cp365Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP365 Cooling Constant-Supply-Humidity-Ratio Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP365 Source-Ordered Cooling Constant-Supply-Humidity-Ratio Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP365 Constant-Supply-Humidity-Ratio Assignment\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP365 Constant-Supply-Humidity-Ratio Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP365 Constant-Supply-Humidity-Ratio Assignment Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp365Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) { throw "CP365 documentation expected one section in $($doc.Path)" }
    foreach ($pattern in @(
            $cp365SourceCommit, $cp365SourceHash, '2235', 'SupplyHumRat',
            'MinCoolSuppAirHumRat', $cp365Sites[0], $cp365Sites[1], '2236',
            'CP366', '2237', '2245', 'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH',
            'S\s*=\s*C0\+Q\+H\+CSH\s*=\s*R\s*=\s*G\+F\+L',
            'A\s*=\s*F\+L', 'source_site_execution_count\s*=\s*2\*CSH',
            'C0\s*=\s*S', 'Q\s*=\s*H\s*=\s*CSH\s*=\s*0',
            '(?s)(?:does not read|reads no right operand|no owner read|lazily skips the owner read)',
            'minimum_cooling_supply_air_humidity_ratio', '(?:\.is_finite\(\)|finite-only)',
            '(?s)(?:raw|directly|bit-copies|copies binary64).{0,80}(?:binary64|bits|owner)', 'signed[- ]zero',
            'CP364-to-CP365-to-unchanged-numerical', $cp365Lifecycle,
            '(?:DirectZonePurchasedAirCouplingInput|cannot feed or replace coupling)', 'CP345', '32\s+algorithms',
            '293\s+routines', '303\s+total', '240\s+public', '63\s+internal',
            'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP365 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP365\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP365 supersedes only CP364' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP365 additionally requires' -Description "generated capability addendum"

# Historical propagation, master order, and generated script inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..364 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_constant_supply_humidity_ratio_assignment' -Description "historical CP365 binding order"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_constant_supply_humidity_ratio_assignment' -Description "historical CP365 helper whitelist"
}
foreach ($historical in 334..364) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp408_lifecycle_evidence' -Description "historical CP365 firewall"
}
foreach ($historical in 335..364) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 346 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 106 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..364) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 346' -Description "historical inventory total"
}
$cp365MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp364AuditIndexForCp365 = $cp365MainAuditText.IndexOf("cp364-cooling-constant-supply-humidity-ratio-case-entry.ps1")
$cp365AuditIndex = $cp365MainAuditText.IndexOf("cp365-cooling-constant-supply-humidity-ratio-assignment.ps1")
$cp365CompletionIndex = $cp365MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp364AuditIndexForCp365 -lt 0 -or $cp365AuditIndex -le $cp364AuditIndexForCp365 -or $cp365CompletionIndex -le $cp365AuditIndex) {
    throw "Master audit must dot-source CP365 after CP364 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 346' -Description "script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp365-' -Description "inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp365-cooling-constant-supply-humidity-ratio-assignment\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 346 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 106 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"

Write-Host "CP365 constant-supply-humidity-ratio assignment structure audit passed."
