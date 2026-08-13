# CP344 maps only PurchasedAirManager.cc physical executable line 2203:
# PurchAir.SupplyTemp = min(PurchAir.SupplyTemp, PurchAir.MixedAirTemp);
# Physical lines 2204-2207 are non-executable; line 2208 is the first
# excluded lexical executable.

$cp344Stem = "cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit"
$cp344PipelineStem = "purchased_air_$cp344Stem"
$cp344TypeStem = "PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimit"
$cp344Module = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem.rs"
$cp344State = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\state.rs"
$cp344Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\transition.rs"
$cp344Release = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\release.rs"
$cp344PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\release\prefix_validation.rs"
$cp344RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\release\runtime_validation.rs"
$cp344SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\release\snapshot_validation.rs"
$cp344Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\tests\mod.rs"
$cp344PublicReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\tests\public_release.rs"
$cp344ReleaseCorruptionTests = "crates\ep_runtime\src\ideal_loads\calc\$cp344Stem\tests\release_corruption.rs"
$cp344CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp344MinimumHelper = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\transition.rs"
$cp344Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp344Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp344ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp344BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp344Stem.rs"
$cp344BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp344BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp344Stem}_tests.rs"
$cp344InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp344InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp344InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp344InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp344Stem.rs"
$cp344CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp344CoupledRuntimeTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp344CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp344Stem}_validation.rs"
$cp344CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp344CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp344Stem}_fixture.rs"
$cp344PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp344Pipeline = "crates\ep_run\src\pipeline\$cp344PipelineStem.rs"
$cp344PipelineValidation = "crates\ep_run\src\pipeline\$cp344PipelineStem\validation.rs"
$cp344PipelineSerialization = "crates\ep_run\src\pipeline\$cp344PipelineStem\serialization.rs"
$cp344PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\$cp344PipelineStem\serialization\snapshot.rs"

function Get-Cp344RustBraceBlock {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$AnchorPattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $anchors = [regex]::Matches($Text, $AnchorPattern)
    if ($anchors.Count -ne 1) {
        throw "$Description expected exactly one anchor, found $($anchors.Count)"
    }
    $openingBrace = $Text.IndexOf("{", $anchors[0].Index)
    if ($openingBrace -lt 0) {
        throw "$Description opening brace is missing"
    }

    $depth = 0
    for ($index = $openingBrace; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") {
            $depth += 1
        } elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) {
                return $Text.Substring(
                    $anchors[0].Index,
                    $index - $anchors[0].Index + 1
                )
            }
        }
    }

    throw "$Description closing brace is missing"
}

function Get-Cp344RustFunctionRegion {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$StartPattern,
        [Parameter(Mandatory = $true)][string]$NextFunctionPattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $starts = [regex]::Matches($Text, $StartPattern)
    if ($starts.Count -ne 1) {
        throw "$Description expected exactly one function start, found $($starts.Count)"
    }
    $remaining = $Text.Substring($starts[0].Index + $starts[0].Length)
    $nextFunction = [regex]::Match($remaining, $NextFunctionPattern)
    if (-not $nextFunction.Success) {
        throw "$Description next top-level function boundary is missing"
    }

    return $Text.Substring(
        $starts[0].Index,
        $starts[0].Length + $nextFunction.Index
    )
}

function Assert-Cp344PatternsInText {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string[]]$Patterns,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $cursor = 0
    for ($index = 0; $index -lt $Patterns.Count; $index += 1) {
        $match = [regex]::Match($Text.Substring($cursor), $Patterns[$index])
        if (-not $match.Success) {
            throw "$Description pattern $($index + 1) is missing or out of order"
        }
        $cursor += $match.Index + $match.Length
    }
}

foreach ($cp344RequiredFile in @(
        $cp344Module,
        $cp344State,
        $cp344Transition,
        $cp344Release,
        $cp344PrefixValidation,
        $cp344RuntimeValidation,
        $cp344SnapshotValidation,
        $cp344Tests,
        $cp344PublicReleaseTests,
        $cp344ReleaseCorruptionTests,
        $cp344BindingAdapter,
        $cp344BindingTests,
        $cp344InitWitness,
        $cp344CoupledValidation,
        $cp344CoupledFixture,
        $cp344Pipeline,
        $cp344PipelineValidation,
        $cp344PipelineSerialization,
        $cp344PipelineSnapshotSerialization,
        $cp344CoupledRuntimeTests
    )) {
    Assert-FileExists -Path $cp344RequiredFile -Description "CP344 supply-temperature mixed-air-limit structure"
}
Assert-LineLimit -Path $cp344Release -Limit 800 -Description "CP344 release module"
Assert-LineLimit -Path $cp344CoupledValidation -Limit 800 -Description "CP344 coupled validation module"
Assert-LineLimit -Path $cp344Pipeline -Limit 800 -Description "CP344 pipeline module"

$cp344SourceStatementPattern =
    'PurchAir\.SupplyTemp\s*=\s*min\(\s*PurchAir\.SupplyTemp\s*,\s*PurchAir\.MixedAirTemp\s*\)\s*;'
$cp344FirstExcludedStatementPattern =
    'PurchAir\.SupplyHumRat\s*=\s*PurchAir\.MixedAirHumRat\s*;'
$cp344OrderedSourceSitesPattern = '(?s)' +
    'read-purchased-air-supply-temperature-for-minimum.*?' +
    'read-purchased-air-mixed-air-temperature-for-minimum.*?' +
    'apply-source-shaped-two-argument-minimum.*?' +
    'assign-purchased-air-supply-temperature'
$cp344PurchasedAirSourceHash =
    '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'

# Exact source boundary, public surface, routes, and four-site inventory.
Assert-Contains -Path $cp344Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2203' -Description "CP344 exact physical source boundary"
Assert-Contains -Path $cp344Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2208' -Description "CP344 first excluded physical executable"
Assert-Contains -Path $cp344Module -Pattern 'Exact four textual source sites represented by CP344' -Description "CP344 source-site count"
Assert-ExactStringArray -Path $cp344Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER" -Expected @(
    "read-purchased-air-supply-temperature-for-minimum",
    "read-purchased-air-mixed-air-temperature-for-minimum",
    "apply-source-shaped-two-argument-minimum",
    "assign-purchased-air-supply-temperature"
) -Description "CP344 deterministic source witness"
Assert-Contains -Path $cp344Module -Pattern ('pub struct ' + $cp344TypeStem + 'Snapshot') -Description "CP344 public snapshot"
Assert-Contains -Path $cp344State -Pattern ('pub struct ' + $cp344TypeStem + 'RuntimeState') -Description "CP344 persistent state"
Assert-Contains -Path $cp344Module -Pattern ('pub struct ' + $cp344TypeStem + 'LifecycleSummary') -Description "CP344 lifecycle summary"
Assert-Contains -Path $cp344Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary\s*\(' -Description "CP344 lifecycle accessor"
Assert-Contains -Path $cp344Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\s*\(' -Description "CP344 exact-direct release"
Assert-Contains -Path $cp344Transition -Pattern 'advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_state\s*\(' -Description "CP344 pure transition"
Assert-Contains -Path $cp344CalcRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit;' -Description "CP344 Calc module declaration"

foreach ($cp344SnapshotField in @(
        "preexisting_supply_temperature_c",
        "supply_temperature_for_minimum_read",
        "supply_temperature_before_mixed_air_limit_c",
        "mixed_air_temperature_for_minimum_read",
        "mixed_air_temperature_c",
        "source_shaped_two_argument_minimum_evaluated",
        "minimum_supply_temperature_c",
        "supply_temperature_assignment_performed",
        "assigned_supply_temperature_c",
        "resulting_supply_temperature_c"
    )) {
    Assert-Contains -Path $cp344Module -Pattern ('pub ' + $cp344SnapshotField + ':') -Description "CP344 snapshot field '$cp344SnapshotField'"
}
foreach ($cp344Route in @(
        "UnitOff",
        "NonCooling",
        "PositiveGuardFalseFallthrough",
        "ActiveCapacityLimitGuardFalseFallthrough",
        "CapacityLimitSensibleOutputGuardFalseFallthrough",
        "CapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitExecuted"
    )) {
    Assert-Contains -Path $cp344State -Pattern $cp344Route -Description "CP344 retained route '$cp344Route'"
}
foreach ($cp344Counter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "capacity_limit_guard_false_fallthrough_skip_count",
        "capacity_limit_sensible_output_guard_false_fallthrough_count",
        "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count",
        "source_site_execution_count",
        "supply_temperature_for_minimum_read_count",
        "mixed_air_temperature_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_temperature_assignment_write_count"
    )) {
    Assert-Contains -Path $cp344State -Pattern ('pub ' + $cp344Counter + ':\s*usize') -Description "CP344 state counter '$cp344Counter'"
}

# Reuse the canonical CP334 source-shaped minimum. Strict true selects left;
# equality and unordered comparisons select right, bit-for-bit.
Assert-Contains -Path $cp344MinimumHelper -Pattern '(?s)fn source_shaped_two_argument_minimum\(\s*left: f64,\s*right: f64,\s*\) -> f64 \{\s*if left < right \{ left \} else \{ right \}\s*\}' -Description "canonical ObjexxFCL two-double minimum semantics"
Assert-Contains -Path $cp344Transition -Pattern 'cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum' -Description "CP344 reuses canonical minimum"
Assert-PatternsInOrder -Path $cp344Transition -Patterns @(
    'preexisting_supply_temperature_c',
    'mixed_air_temperature_c',
    'source_shaped_two_argument_minimum\(',
    'assigned_supply_temperature_c\s*=\s*minimum_supply_temperature_c',
    'resulting_supply_temperature_c\s*=\s*if limit_executed'
) -Description "CP344 reads, minimum, assignment, and result order"
Assert-NotContains -Path $cp344Transition -Pattern 'f64::min|\.min\(|total_cmp|partial_cmp|\.clamp\(|normalize|is_finite|cache|diagnostic' -Description "CP344 transition has no broadened minimum or coercion"

# Checked lifecycle algebra is T=U+N+P+G+F+L, A=F+L, and
# L=D=H=M=CP340 adjustment-body entries, with four sites per L.
Assert-Contains -Path $cp344RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count\s*\.checked_mul\(' -Description "CP344 checked 4*L source formula"
Assert-Contains -Path $cp344RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count\s*==\s*predecessor\s*\.capacity_limit_sensible_output_supply_temperature_assignment_count' -Description "CP344 L equals CP343 D"
Assert-Contains -Path $cp344RuntimeValidation -Pattern '(?s)false_fallthroughs\.checked_add\(limits\).*?capacity_limit_sensible_output_guard_evaluation_count.*?limits.*?capacity_limit_sensible_output_adjustment_body_entry_count.*?limits.*?maximum_capacity_assignment_count.*?limits.*?supply_enthalpy_assignment_count.*?limits.*?supply_temperature_assignment_count' -Description "CP344 cumulative F/L/D/H/M/CP340 algebra"
Assert-Contains -Path $cp344RuntimeValidation -Pattern '(?s)route_partition == state\.transition_count.*?source_site_execution_count == expected_source_sites.*?supply_temperature_for_minimum_read_count == limited.*?mixed_air_temperature_for_minimum_read_count == limited.*?source_shaped_two_argument_minimum_evaluation_count == limited.*?supply_temperature_assignment_write_count == limited' -Description "CP344 route/source/site identities"
foreach ($cp344PreflightCounter in @(
        "capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count",
        "source_site_execution_count",
        "supply_temperature_for_minimum_read_count",
        "mixed_air_temperature_for_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_temperature_assignment_write_count",
        "witnessed_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count"
    )) {
    Assert-Contains -Path $cp344RuntimeValidation -Pattern ($cp344PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP344 checked preflight '$cp344PreflightCounter'"
}

# False fallthrough preserves CP343 bits and inherited skips are complete-null.
# The true route requires only the CP329 right operand to be finite.
Assert-Contains -Path $cp344SnapshotValidation -Pattern '(?s)fn false_fallthrough_snapshot_is_exact\(.*?preexisting\.to_bits\(\) == resulting\.to_bits\(\).*?source_values_are_none' -Description "CP344 false route preserves CP343 result bits"
Assert-Contains -Path $cp344SnapshotValidation -Pattern '(?s)fn limited_snapshot_is_exact\(.*?expected = source_shaped_two_argument_minimum\(left, right\).*?preexisting\.to_bits\(\) == left\.to_bits\(\).*?right\.is_finite\(\).*?minimum\.to_bits\(\) == expected\.to_bits\(\).*?assigned\.to_bits\(\) == minimum\.to_bits\(\).*?resulting\.to_bits\(\) == assigned\.to_bits\(\)' -Description "CP344 exact minimum and assignment bits"
Assert-NotContains -Path $cp344SnapshotValidation -Pattern 'preexisting\.is_finite\(\)|left\.is_finite\(\)|minimum\.is_finite\(\)|assigned\.is_finite\(\)|resulting\.is_finite\(\)' -Description "CP344 adds no left/result finite gate"
Assert-Contains -Path $cp344SnapshotValidation -Pattern '(?s)fn skipped_snapshot_is_exact\(.*?preexisting_supply_temperature_c\.is_none\(\).*?resulting_supply_temperature_c\.is_none\(\).*?source_values_are_none' -Description "CP344 inherited complete-null firewall"

# Public release accepts only runtime/system/CP343. CP343 owns the left
# operand and retained same-call CP329 owns the finite right operand.
Assert-Contains -Path $cp344Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp343:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,\s*\)' -Description "CP344 exact public arguments"
Assert-Contains -Path $cp344Release -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witness\s*\(' -Description "CP344 CP343 private witness"
Assert-Contains -Path $cp344Release -Pattern 'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_is_consistent\s*\(' -Description "CP344 recursive CP343 proof"
Assert-Contains -Path $cp344Release -Pattern '(?s)then_some\(unit\.calc_cooling_mixed_air_call\.latest\).*?cooling_mixed_air_call_latest_witness\s*\(' -Description "CP344 CP329 latest/private owner"
Assert-Contains -Path $cp344Release -Pattern 'completed_direct_cooling_mixed_air_call_is_consistent\s*\(' -Description "CP344 recursive CP329 proof"
Assert-Contains -Path $cp344Release -Pattern '(?s)mixed_air_temperature_c\).*?is_some_and\(\|temperature\| !temperature\.is_finite\(\)\)' -Description "CP344 finite right-operand gate"
Assert-Contains -Path $cp344PrefixValidation -Pattern '(?s)predecessor\.resulting_supply_temperature_c.*?limit\.preexisting_supply_temperature_c' -Description "CP344 CP343 left ownership"
Assert-Contains -Path $cp344PrefixValidation -Pattern '(?s)mixed_air\.mixed_air_temperature_c.*?mixed_air_temperature_c\.is_finite\(\)' -Description "CP344 CP329 right ownership"
Assert-Contains -Path $cp344PrefixValidation -Pattern 'retained_source_owner_lineage_is_exact\s*\(' -Description "CP344 retained owner validation"
Assert-Contains -Path $cp344PrefixValidation -Pattern 'mixed_air_limit_links_to_predecessor\s*\(' -Description "CP344 predecessor linkage"
Assert-NotContains -Path $cp344Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^)]*(mixed_air_temperature|supply_temperature)\s*:' -Description "CP344 duplicate caller operands"
Assert-NotContains -Path $cp344Release -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|DirectZonePurchasedAirCouplingInput|ZoneHeatBalanceState|zone_state|psychrometric_service|cache|diagnostic' -Description "CP344 service/numerical firewall"
$cp344ReleaseText = Read-RepoText -Path $cp344Release
$cp344PublicReleaseText = Get-Cp344RustFunctionRegion `
    -Text $cp344ReleaseText `
    -StartPattern ('(?m)^\s*pub\s+fn\s+advance_direct_no_oa_calc_' + [regex]::Escape($cp344Stem) + '\s*\(') `
    -NextFunctionPattern '(?m)^\s*fn\s+call_order_error\s*\(' `
    -Description "CP344 public release body"
$cp344PublicReleaseOrder = @(
    'retained_source_owner_lineage_is_exact\(',
    'completed_direct_cooling_mixed_air_call_is_consistent\(',
    'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_is_consistent\(',
    '\|\|\s*!owner_complete',
    'retained_input_from_prefix\(',
    'next_supply_temperature_mixed_air_limit_transition_fits\(',
    'advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_state\(',
    'set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness\('
)
Assert-Cp344PatternsInText `
    -Text $cp344PublicReleaseText `
    -Patterns $cp344PublicReleaseOrder `
    -Description "CP344 public release validates owner lineage and completions before mutation"
foreach ($cp344PublicReleaseUniquePattern in $cp344PublicReleaseOrder) {
    $cp344PublicReleaseMatches =
        [regex]::Matches($cp344PublicReleaseText, $cp344PublicReleaseUniquePattern)
    if ($cp344PublicReleaseMatches.Count -ne 1) {
        throw "CP344 public release proof/mutation pattern must occur exactly once: $cp344PublicReleaseUniquePattern"
    }
}
$cp344LastCompletion = [regex]::Match(
    $cp344PublicReleaseText,
    'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_is_consistent\('
)
$cp344FirstMutation = [regex]::Match(
    $cp344PublicReleaseText,
    '(?:runtime\s*\.\s*units\s*\.\s*get_mut\s*\(|advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_state\s*\(|set_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness\s*\()'
)
if (
    -not $cp344LastCompletion.Success -or
    -not $cp344FirstMutation.Success -or
    $cp344FirstMutation.Index -lt ($cp344LastCompletion.Index + $cp344LastCompletion.Length)
) {
    throw "CP344 public release may mutate only after owner-lineage and recursive completion proofs"
}

# Runtime-root state and latest witness stay private and system-rooted.
Assert-Contains -Path $cp344InitState -Pattern ('(?s)' + $cp344Stem + '_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*' + $cp344TypeStem + 'Snapshot') -Description "runtime-root private CP344 witness map"
Assert-NotContains -Path $cp344InitState -Pattern ('(?m)^\s*pub(?:\([^)]*\))?\s+' + $cp344Stem + '_latest_witnesses:') -Description "CP344 witness map remains private"
Assert-Contains -Path $cp344InitWitnessRoot -Pattern ('mod ' + $cp344Stem + ';') -Description "CP344 witness module"
Assert-Contains -Path $cp344InitWitness -Pattern ($cp344Stem + '_latest_witness\s*\(') -Description "CP344 private witness getter"
Assert-Contains -Path $cp344InitWitness -Pattern ('set_' + $cp344Stem + '_latest_witness\s*\(') -Description "CP344 private witness setter"
Assert-Contains -Path $cp344InitState -Pattern ('pub calc_' + $cp344Stem + ':') -Description "per-unit CP344 state"
Assert-Contains -Path $cp344InitUnit -Pattern ('(?s)calc_' + $cp344Stem + ':\s*' + $cp344TypeStem + 'RuntimeState::new\(\s*system') -Description "per-unit CP344 initialization"

# Binding order is exactly CP343 -> CP344 -> unchanged numerical DTO.
$cp344BindingText = Read-RepoText -Path $cp344Binding
$cp344BindingFunctionText = Get-Cp344RustFunctionRegion `
    -Text $cp344BindingText `
    -StartPattern '(?m)^\s*pub\s+fn\s+couple_model_bound_direct_zone_purchased_air\s*\(' `
    -NextFunctionPattern '(?m)^\s*fn\s+validate_runtime_state\s*\(' `
    -Description "direct-zone purchased-air binding function"
$cp343BindingCallForCp344 = [regex]::Match(
    $cp344BindingFunctionText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;'
)
$cp344BindingCall = [regex]::Match(
    $cp344BindingFunctionText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;'
)
$cp344NumericalCalls = [regex]::Matches(
    $cp344BindingFunctionText,
    '(?s)let\s+coupling\s*=\s*complete_direct_zone_purchased_air_coupling\s*\(\s*DirectZonePurchasedAirCouplingInput\s*\{'
)
if (
    -not $cp343BindingCallForCp344.Success -or
    -not $cp344BindingCall.Success -or
    $cp344NumericalCalls.Count -ne 1 -or
    $cp344BindingCall.Index -lt ($cp343BindingCallForCp344.Index + $cp343BindingCallForCp344.Length) -or
    $cp344NumericalCalls[0].Index -lt ($cp344BindingCall.Index + $cp344BindingCall.Length)
) {
    throw "Binding must complete CP343 then CP344 before unchanged numerical coupling"
}
$cp344NumericalIndex = $cp344NumericalCalls[0].Index
foreach ($cp344BindingInterval in @(
        [PSCustomObject]@{
            Start = $cp343BindingCallForCp344.Index + $cp343BindingCallForCp344.Length
            End = $cp344BindingCall.Index
            Description = "after CP343 and before CP344"
        },
        [PSCustomObject]@{
            Start = $cp344BindingCall.Index + $cp344BindingCall.Length
            End = $cp344NumericalIndex
            Description = "after CP344 and before numerical coupling"
        }
    )) {
    $cp344BindingIntervalCode = $cp344BindingFunctionText.Substring(
        $cp344BindingInterval.Start,
        $cp344BindingInterval.End - $cp344BindingInterval.Start
    )
    $cp344BindingIntervalCode =
        [regex]::Replace($cp344BindingIntervalCode, '(?m)//.*$', '')
    $cp344BindingIntervalCode = [regex]::Replace(
        $cp344BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp344BindingIntervalCode = [regex]::Replace(
        $cp344BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp344BindingIntervalCode = [regex]::Replace(
        $cp344BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\([^;]+?\)\?;)',
    ''
)
    $cp344BindingIntervalCode = [regex]::Replace(
        $cp344BindingIntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\([^;]+?\)\?;',
        ''
    )
        $cp344BindingIntervalCode = [regex]::Replace(
        $cp344BindingIntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
        $cp344BindingIntervalCode = [regex]::Replace(
        $cp344BindingIntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\([^;]+?\)\?;',
        ''
    )
        $cp344BindingIntervalCode = [regex]::Replace(
        $cp344BindingIntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
        $cp344BindingIntervalCode = [regex]::Replace(
        $cp344BindingIntervalCode,
        '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\([^;]+?\)\?;)',
        ''
    )
$cp344BindingIntervalCode = [regex]::Replace(
    $cp344BindingIntervalCode,
    '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;)',
    ''
)
    if ($cp344BindingIntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp344BindingInterval.Description)"
    }
}
$cp344NumericalSuffix = $cp344BindingFunctionText.Substring($cp344NumericalIndex)
$cp344NumericalDtoText = Get-Cp344RustBraceBlock `
    -Text $cp344NumericalSuffix `
    -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' `
    -Description "CP344 numerical coupling DTO"
$cp344NumericalDtoCode =
    [regex]::Replace($cp344NumericalDtoText, '(?s)/\*.*?\*/|(?m)//.*$', '')
$cp344NumericalDtoForbiddenPattern = '(?i)(?:\bcp344\b|' +
    [regex]::Escape($cp344Stem) + '|' +
    [regex]::Escape($cp344TypeStem) + ')'
if ($cp344NumericalDtoCode -match $cp344NumericalDtoForbiddenPattern) {
    throw "CP344 evidence must not feed DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp344Binding -Pattern '(?s)advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment,\s*\)\?;' -Description "binding exact CP343-to-CP344 call"
Assert-Contains -Path $cp344BindingAdapter -Pattern '(?s)fn advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,\s*\)' -Description "CP344 binding adapter arguments"
Assert-NotContains -Path $cp344BindingAdapter -Pattern 'mixed_air_temperature\s*:|supply_temperature\s*:|latest_numerical|complete_direct_zone_purchased_air_coupling' -Description "CP344 binding excludes duplicate operands"
Assert-Contains -Path $cp344ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit:' -Description "CP344 scheduled output"
Assert-Contains -Path $cp344BindingTestsRoot -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_tests\.rs' -Description "CP344 binding test module"

# Coupled runtime and pipeline carry direct-only evidence without DTO input.
Assert-Contains -Path $cp344CoupledRuntime -Pattern ('mod ' + $cp344Stem + '_validation;') -Description "coupled CP344 validator"
Assert-Contains -Path $cp344CoupledRuntime -Pattern ('pub calc_' + $cp344Stem + '_lifecycle:') -Description "coupled CP344 lifecycle"
Assert-Contains -Path $cp344CoupledRuntime -Pattern ($cp344Stem + '_validation::snapshot_matches_release') -Description "coupled CP344 snapshot validation"
Assert-Contains -Path $cp344CoupledRuntime -Pattern ($cp344Stem + '_validation::validate_lifecycle') -Description "coupled CP344 final validation"
Assert-Contains -Path $cp344CoupledValidation -Pattern 'output\s*\.\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment' -Description "coupled CP343 predecessor"
Assert-Contains -Path $cp344CoupledValidation -Pattern 'output\.calculation_cooling_mixed_air_call' -Description "coupled CP329 right owner"
Assert-Contains -Path $cp344CoupledValidation -Pattern 'output\.calculation_cooling_positive_supply_temperature_mixed_air_limit' -Description "coupled recursive CP334 corroboration"
Assert-Contains -Path $cp344CoupledValidation -Pattern '(?s)let\s+executions\s*=\s*state\.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count\s*;.*?checked_mul\(\s*executions,' -Description "coupled CP344 checked 4*L"
$cp344CoupledValidationText = Read-RepoText -Path $cp344CoupledValidation
$cp344CoupledSnapshotShapeText = Get-Cp344RustFunctionRegion `
    -Text $cp344CoupledValidationText `
    -StartPattern '(?m)^\s*fn\s+snapshot_shape\s*\(' `
    -NextFunctionPattern '(?m)^\s*fn\s+inherited_shape_matches\s*\(' `
    -Description "coupled CP344 snapshot-shape validator"
$cp344StrictMinimumPattern =
    '(?s)let\s+expected\s*=\s*if\s+preexisting\s*<\s*right\s*\{\s*preexisting\s*\}\s*else\s*\{\s*right\s*\}\s*;'
$cp344StrictMinimumMatches =
    [regex]::Matches($cp344CoupledSnapshotShapeText, $cp344StrictMinimumPattern)
if ($cp344StrictMinimumMatches.Count -ne 1) {
    throw "Coupled CP344 snapshot validation must evaluate the inline source-shaped strict minimum exactly once"
}
if ($cp344CoupledSnapshotShapeText -match '(?:f64\s*::\s*min|\.min\s*\(|total_cmp\s*\(|partial_cmp\s*\(|clamp\s*\()') {
    throw "Coupled CP344 snapshot validation must not replace the inline strict minimum"
}
Assert-NotContains -Path $cp344CoupledValidation -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|psychrometric_service|cache' -Description "coupled CP344 numerical/service firewall"
Assert-Contains -Path $cp344CoupledFixtureRoot -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_fixture\.rs' -Description "CP344 coupled fixture module"
Assert-Contains -Path $cp344PipelineRoot -Pattern ('mod ' + $cp344PipelineStem + ';') -Description "pipeline CP344 module"
Assert-Contains -Path $cp344PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle' -Description "pipeline CP344 lifecycle and JSON key"
Assert-Contains -Path $cp344Pipeline -Pattern 'predecessor_cp343' -Description "pipeline CP343 predecessor"
Assert-Contains -Path $cp344Pipeline -Pattern 'mixed_air_cp329' -Description "pipeline CP329 owner"
Assert-Contains -Path $cp344PipelineValidation -Pattern '(?s)capacity_limit_sensible_output_supply_temperature_mixed_air_limit_count;.*?checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline CP344 checked 4*L"
Assert-NotContains -Path $cp344Pipeline -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|psychrometric_service|cache' -Description "pipeline CP344 numerical/service firewall"
$cp344PipelineSnapshotSerializationText =
    Read-RepoText -Path $cp344PipelineSnapshotSerialization
$cp344SerializerTestBoundary = [regex]::Match(
    $cp344PipelineSnapshotSerializationText,
    '(?m)^\s*#\[cfg\(test\)\]\s*$'
)
if (-not $cp344SerializerTestBoundary.Success) {
    throw "CP344 snapshot serializer test boundary is missing"
}
$cp344PipelineSnapshotSerializationProduction =
    $cp344PipelineSnapshotSerializationText.Substring(0, $cp344SerializerTestBoundary.Index)
$cp344PipelineSnapshotJsonText = Get-Cp344RustFunctionRegion `
    -Text $cp344PipelineSnapshotSerializationProduction `
    -StartPattern '(?m)^\s*pub\(super\)\s+fn\s+snapshot_json\s*\(' `
    -NextFunctionPattern '(?m)^\s*fn\s+json_number\s*\(' `
    -Description "production CP344 snapshot serializer"
foreach ($cp344JsonField in @(
        "preexisting_supply_temperature_c",
        "supply_temperature_before_mixed_air_limit_c",
        "mixed_air_temperature_c",
        "minimum_supply_temperature_c",
        "assigned_supply_temperature_c",
        "resulting_supply_temperature_c"
    )) {
    $cp344JsonValuePattern = '"' + [regex]::Escape($cp344JsonField) +
        '"\s*:\s*json_number\s*\(\s*snapshot\s*\.\s*' +
        [regex]::Escape($cp344JsonField) + '\s*\)'
    $cp344JsonBitsPattern = '"' + [regex]::Escape($cp344JsonField + "_ieee_bits") +
        '"\s*:\s*ieee_bits\s*\(\s*snapshot\s*\.\s*' +
        [regex]::Escape($cp344JsonField) + '\s*\)'
    $cp344JsonValueKeyPattern =
        '"' + [regex]::Escape($cp344JsonField) + '"\s*:'
    $cp344JsonBitsKeyPattern =
        '"' + [regex]::Escape($cp344JsonField + "_ieee_bits") + '"\s*:'
    if (
        [regex]::Matches(
            $cp344PipelineSnapshotJsonText,
            $cp344JsonValuePattern
        ).Count -ne 1 -or
        [regex]::Matches(
            $cp344PipelineSnapshotJsonText,
            $cp344JsonBitsPattern
        ).Count -ne 1 -or
        [regex]::Matches(
            $cp344PipelineSnapshotJsonText,
            $cp344JsonValueKeyPattern
        ).Count -ne 1 -or
        [regex]::Matches(
            $cp344PipelineSnapshotJsonText,
            $cp344JsonBitsKeyPattern
        ).Count -ne 1
    ) {
        throw "Production CP344 JSON must map '$cp344JsonField' and its IEEE bits exactly once"
    }
}
if (
    $cp344PipelineSnapshotSerializationProduction -notmatch
        '(?s)fn\s+json_number\s*\(\s*value:\s*Option<f64>\s*\)\s*->\s*Value\s*\{.*?filter\s*\(\s*\|value\|\s*value\s*\.\s*is_finite\s*\(\s*\)\s*\).*?map_or\s*\(\s*Value::Null' -or
    $cp344PipelineSnapshotSerializationProduction -notmatch
        'value\s*\.\s*map\s*\(\s*\|value\|\s*format!\s*\(\s*"0x\{:016x\}"\s*,\s*value\s*\.\s*to_bits\s*\(\s*\)\s*\)\s*\)'
) {
    throw "Production CP344 JSON helpers must preserve null projection and authoritative IEEE bits"
}

$cp344PipelineRootText = Read-RepoText -Path $cp344PipelineRoot
$cp344PipelineTestBoundary = [regex]::Match(
    $cp344PipelineRootText,
    '(?m)^\s*#\[cfg\(test\)\]\s*$'
)
if (-not $cp344PipelineTestBoundary.Success) {
    throw "Pipeline production/test boundary is missing"
}
$cp344PipelineProductionText =
    $cp344PipelineRootText.Substring(0, $cp344PipelineTestBoundary.Index)
$cp344PipelineExecuteRuntimeText = Get-Cp344RustFunctionRegion `
    -Text $cp344PipelineProductionText `
    -StartPattern '(?m)^\s*fn\s+execute_rust_runtime\s*\(' `
    -NextFunctionPattern '(?m)^\s*fn\s+validate_runtime_demand_provenance\s*\(' `
    -Description "pipeline Rust runtime constructor"
$cp344PipelineProvenanceText = Get-Cp344RustFunctionRegion `
    -Text $cp344PipelineProductionText `
    -StartPattern '(?m)^\s*fn\s+validate_runtime_demand_provenance\s*\(' `
    -NextFunctionPattern '(?m)^\s*fn\s+validate_direct_purchased_air_init_lifecycle\s*\(' `
    -Description "pipeline runtime-demand provenance validator"
$cp344NonDirectError = [regex]::Match(
    $cp344PipelineProvenanceText,
    '"persistent PurchasedAir lifecycle evidence was attached to a non-direct runtime"'
)
if (-not $cp344NonDirectError.Success) {
    throw "Pipeline non-direct lifecycle rejection is missing"
}
$cp344BeforeNonDirectError =
    $cp344PipelineProvenanceText.Substring(0, $cp344NonDirectError.Index)
$cp344ElseIfMatches = [regex]::Matches(
    $cp344BeforeNonDirectError,
    '\belse\s+if\b'
)
if ($cp344ElseIfMatches.Count -eq 0) {
    throw "Pipeline non-direct lifecycle rejection condition is missing"
}
$cp344NonDirectElseIf = $cp344ElseIfMatches[$cp344ElseIfMatches.Count - 1]
$cp344NonDirectConditionOpenBrace = $cp344PipelineProvenanceText.IndexOf(
    "{",
    $cp344NonDirectElseIf.Index + $cp344NonDirectElseIf.Length
)
if (
    $cp344NonDirectConditionOpenBrace -lt 0 -or
    $cp344NonDirectConditionOpenBrace -gt $cp344NonDirectError.Index
) {
    throw "Pipeline non-direct lifecycle rejection condition is malformed"
}
$cp344NonDirectCondition = $cp344PipelineProvenanceText.Substring(
    $cp344NonDirectElseIf.Index + $cp344NonDirectElseIf.Length,
    $cp344NonDirectConditionOpenBrace -
        ($cp344NonDirectElseIf.Index + $cp344NonDirectElseIf.Length)
)
$cp344PipelineLifecycleField =
    'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle'
$cp344NonDirectDisjunctPattern = '(?s)(?:^|\|\|)\s*result\s*\.\s*' +
    [regex]::Escape($cp344PipelineLifecycleField) +
    '\s*\.\s*is_some\s*\(\s*\)(?=\s*(?:\|\||$))'
$cp344NonDirectDisjuncts =
    [regex]::Matches($cp344NonDirectCondition, $cp344NonDirectDisjunctPattern)
if ($cp344NonDirectDisjuncts.Count -ne 1) {
    throw "Pipeline non-direct rejection OR must include CP344 lifecycle is_some() exactly once"
}
$cp344NoneConstructorCount = [regex]::Matches(
    $cp344PipelineExecuteRuntimeText,
    [regex]::Escape($cp344PipelineLifecycleField) + '\s*:\s*None'
).Count
$cp344SomeConstructorCount = [regex]::Matches(
    $cp344PipelineExecuteRuntimeText,
    'let\s+' + [regex]::Escape($cp344PipelineLifecycleField) + '\s*=\s*Some\s*\('
).Count
$cp344RuntimeResultConstructorCount = [regex]::Matches(
    $cp344PipelineExecuteRuntimeText,
    'Ok\s*\(\s*RustRuntimeResult\s*\{'
).Count
$cp344DirectConstructorShorthandCount = [regex]::Matches(
    $cp344PipelineExecuteRuntimeText,
    ('(?m)^\s*' + [regex]::Escape($cp344PipelineLifecycleField) + '\s*,\s*$')
).Count
if (
    $cp344RuntimeResultConstructorCount -ne 4 -or
    $cp344NoneConstructorCount -ne 3 -or
    $cp344SomeConstructorCount -ne 1 -or
    $cp344DirectConstructorShorthandCount -ne 1
) {
    throw "Pipeline must expose CP344 lifecycle through one direct Some/result and all three non-direct None constructors"
}
Assert-Contains -Path $cp344PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp422_lifecycle_evidence' -Description "non-direct CP344 through CP363 evidence rejection"

# Specs contain exactly two addenda and the 2+2+1+1 target distribution.
$cp344AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp344AlgorithmAddenda = [regex]::Matches(
    $cp344AlgorithmText,
    '(?m)^\s*"CP344 supersedes only CP343[^"\r\n]+",\s*$'
)
if ($cp344AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP344 claim addenda"
}
$cp344TargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\.rs::purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp344Target in $cp344TargetCounts) {
    $cp344TargetCount = [regex]::Matches($cp344AlgorithmText, $cp344Target.Pattern).Count
    if ($cp344TargetCount -ne $cp344Target.Expected) {
        throw "CP344 target '$($cp344Target.Pattern)' expected $($cp344Target.Expected), found $cp344TargetCount"
    }
}
$cp344CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp344CapabilityAddenda = [regex]::Matches(
    $cp344CapabilityText,
    '(?m)^\s*"CP344 additionally requires[^"\r\n]+",\s*$'
)
if ($cp344CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP344 claim addenda"
}
foreach ($cp344Claim in @($cp344AlgorithmAddenda) + @($cp344CapabilityAddenda)) {
    foreach ($cp344Pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            $cp344PurchasedAirSourceHash,
            $cp344SourceStatementPattern,
            $cp344OrderedSourceSitesPattern,
            'a < b \? a : b',
            'if left < right \{ left \} else \{ right \}',
            'ties and unordered comparisons select the CP329 right operand bit-for-bit',
            'CapacityLimitSensibleOutputSupplyTemperatureAssigned',
            'CapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitExecuted',
            'T=U\+N\+P\+G\+F\+L',
            'A=F\+L',
            'L=D=H=M=CP340 adjustment-body entries',
            'source_site_execution_count=4\*L',
            'supplied/latest/private CP343',
            'resulting_supply_temperature_c',
            'CP329 latest/private `mixed_air_temperature_c`',
            'no new finite gate',
            'defensive characterization',
            'not a new full-public-chain reachability claim',
            'CP343-to-CP344-to-unchanged-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle',
            '2204-2207 are non-executable',
            $cp344FirstExcludedStatementPattern,
            'Roadmap promotion'
        )) {
        if ($cp344Claim.Value -notmatch $cp344Pattern) {
            throw "CP344 spec addendum missing '$cp344Pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP344 supersedes only CP343' -Description "generated CP344 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP344 additionally requires' -Description "generated CP344 capability index"

# Exactly five hand-authored contract sections carry CP344; the
# psychrometrics source map is intentionally unchanged.
$cp344DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP344 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP344 Source-Ordered Cooling Positive-Supply Capacity-Limit Supply-Temperature Mixed-Air Limit\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP344 Cooling Positive-Supply Capacity-Limit Supply-Temperature Mixed-Air Limit\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP344 Positive-Supply Capacity-Limit Supply-Temperature Mixed-Air Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP344 Cooling Positive-Supply Capacity-Limit Supply-Temperature Mixed-Air Limit Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp344Documentation in $cp344DocumentationSections) {
    $cp344DocumentText = Read-RepoText -Path $cp344Documentation.Path
    $cp344Matches = [regex]::Matches($cp344DocumentText, $cp344Documentation.Pattern)
    if ($cp344Matches.Count -ne 1) {
        throw "CP344 documentation expected one scoped section in $($cp344Documentation.Path), found $($cp344Matches.Count)"
    }
    $cp344Section = $cp344Matches[0].Value
    foreach ($cp344Pattern in @(
            $cp344PurchasedAirSourceHash,
            $cp344SourceStatementPattern,
            $cp344OrderedSourceSitesPattern,
            'a\s*<\s*b\s*\?\s*a\s*:\s*b',
            'if left < right \{ left \} else \{ right \}',
            '(?s)ties.*?unordered.*?CP329.*?right',
            'CapacityLimitSensibleOutputSupplyTemperatureAssigned',
            'CapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitExecuted',
            'T\s*=\s*U\+N\+P\+G\+F\+L',
            'A\s*=\s*F\+L',
            'L\s*=\s*D\s*=\s*H\s*=\s*M\s*=\s*CP340 adjustment-body entries',
            '4\*L',
            '(?s)supplied.*?(?:latest|retained-latest).*?private CP343',
            'resulting_supply_temperature_c',
            '(?s)CP329.*?(?:latest|retained-latest).*?private.*?mixed_air_temperature_c',
            '(?s)CP334.*?CP336.*?(?:lineage|corroborat)',
            'no\s+new\s+finite\s+gate',
            '(?s)(?:pure|defensive).*?characterization.*?(?:not|rather than).*?(?:full-public|public)',
            'IEEE',
            'CP343-to-CP344-to-(?:unchanged-)?numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle',
            '(?s)2204-2207.*?non-executable',
            $cp344FirstExcludedStatementPattern,
            '(?i)numerical[- ]DTO',
            'state_mapped',
            'source_mapped',
            'Roadmap'
        )) {
        if ($cp344Section -notmatch $cp344Pattern) {
            throw "CP344 documentation in $($cp344Documentation.Path) missing '$cp344Pattern'"
        }
    }
    if ($cp344Section -match '(?is)public(?:ly)? reachable.{0,100}(?:infinity|NaN)|(?:infinity|NaN).{0,100}public(?:ly)? reachable') {
        throw "CP344 documentation must not promote unproved nonfinite full-public reachability in $($cp344Documentation.Path)"
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP344\b' -Description "CP344 does not alter the psychrometrics source map"

# Historical audits must explicitly admit the CP344 binding call and carry
# the cumulative non-direct firewall and inventory counts.
foreach ($cp344HistoricalBindingAudit in @(
        "scripts\quality\ideal-loads-structure-audit\cp326-cooling-supply-mass-flow-limit-body.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp329-cooling-mixed-air-call.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp330-cooling-supply-mass-flow-positive-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp331-cooling-positive-supply-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp332-cooling-positive-supply-temperature-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp333-cooling-positive-supply-temperature-minimum-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1"
    )) {
    Assert-Contains -Path $cp344HistoricalBindingAudit -Pattern 'calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit' -Description "historical binding interval admits only CP344"
}
foreach ($cp344HistoricalFirewallAudit in @(
        "scripts\quality\ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1"
    )) {
    Assert-Contains -Path $cp344HistoricalFirewallAudit -Pattern 'non_direct_runtime_rejects_cp316_through_cp422_lifecycle_evidence' -Description "historical non-direct firewall reaches CP362"
}
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1" -Pattern 'cp347_direct_coupled_runtime_completes_none_case_after_g_f_l_and_skips_unit_off' -Description "cumulative coupled CP347 regression"

# Root reachability and generated inventory add one internal script:
# 284 executable records, 240 public, 44 internal, and zero uncalled.
$cp344MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp343DotSourceIndexForCp344 = $cp344MainAuditText.IndexOf('ideal-loads-structure-audit\cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1')
$cp344DotSourceIndex = $cp344MainAuditText.IndexOf('ideal-loads-structure-audit\cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit.ps1')
$cp344AuditCompletionIndex = $cp344MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp343DotSourceIndexForCp344 -lt 0 -or
    $cp344DotSourceIndex -le $cp343DotSourceIndexForCp344 -or
    $cp344AuditCompletionIndex -le $cp344DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP344 after CP343 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 360' -Description "CP344 cumulative inventory total through CP403"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP344 cumulative uncalled inventory"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit\.ps1"' -Description "CP344 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp344-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-mixed-air-limit\.ps1::dot_sources' -Description "CP344 main-audit callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 360 \|' -Description "CP344 generated script count through CP403"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP344 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 120 \|' -Description "CP344 generated internal script count through CP403"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP344 generated uncalled script count"
