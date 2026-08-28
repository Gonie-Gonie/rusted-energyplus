# CP343 maps only PurchasedAirManager.cc physical executable line 2201:
# PurchAir.SupplyTemp = PsyTdbFnHW(SupplyEnthalpy, PurchAir.SupplyHumRat);
# Physical line 2202 is commentary and line 2203 is the first excluded executable.

$cp343Stem = "cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment"
$cp343PipelineStem = "purchased_air_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment"
$cp343TypeStem = "PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignment"
$cp343Module = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem.rs"
$cp343State = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem\state.rs"
$cp343Transition = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem\transition.rs"
$cp343Release = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem\release.rs"
$cp343PrefixValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem\release\prefix_validation.rs"
$cp343RuntimeValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem\release\runtime_validation.rs"
$cp343SnapshotValidation = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem\release\snapshot_validation.rs"
$cp343Tests = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem\tests\mod.rs"
$cp343PublicReleaseTests = "crates\ep_runtime\src\ideal_loads\calc\$cp343Stem\tests\public_release.rs"
$cp343CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp343Psychrometrics = "crates\ep_runtime\src\psychrometrics.rs"
$cp343PsychrometricsTests = "crates\ep_runtime\src\psychrometrics_inverse_density_tests.rs"
$cp343Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
Assert-Contains -Path $cp343Binding -Pattern '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =.*?let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =.*?let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =.*?let calculation_cooling_humidistat_case_break =.*?let calculation_cooling_constant_supply_humidity_ratio_case_entry =.*?let calculation_cooling_constant_supply_humidity_ratio_assignment =.*?let calculation_cooling_constant_supply_humidity_ratio_case_break =.*?let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =.*?let calculation_cooling_default_supply_humidity_ratio_case_break =.*?let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =.*?let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =.*?let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =.*?let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_assignment =.*?let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =.*?let calculation_cooling_supply_enthalpy_post_saturation_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =.*?let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =.*?let coupling = complete_direct_zone_purchased_air_coupling\(' -Description "historical CP359-to-CP360-to-CP361-to-CP362-to-CP363-to-CP364 binding order"
$cp343ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp343BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp343Stem.rs"
$cp343BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp343BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp343Stem}_tests.rs"
$cp343InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp343InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp343InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp343InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp343Stem.rs"
$cp343CoupledRuntime = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp343CoupledRuntimeTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp343CoupledValidation = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp343Stem}_validation.rs"
$cp343CoupledFixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp343CoupledFixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp343Stem}_fixture.rs"
$cp343PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp343Pipeline = "crates\ep_run\src\pipeline\$cp343PipelineStem.rs"
$cp343PipelineValidation = "crates\ep_run\src\pipeline\$cp343PipelineStem\validation.rs"
$cp343PipelineSerialization = "crates\ep_run\src\pipeline\$cp343PipelineStem\serialization.rs"
$cp343PipelineSnapshotSerialization = "crates\ep_run\src\pipeline\$cp343PipelineStem\serialization\snapshot.rs"
$cp343Cp342Audit = "scripts\quality\ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1"

foreach ($cp343RequiredFile in @(
        $cp343Module,
        $cp343State,
        $cp343Transition,
        $cp343Release,
        $cp343PrefixValidation,
        $cp343RuntimeValidation,
        $cp343SnapshotValidation,
        $cp343Tests,
        $cp343PublicReleaseTests,
        $cp343Psychrometrics,
        $cp343PsychrometricsTests,
        $cp343BindingAdapter,
        $cp343BindingTests,
        $cp343InitWitness,
        $cp343CoupledValidation,
        $cp343CoupledFixture,
        $cp343Pipeline,
        $cp343PipelineValidation,
        $cp343PipelineSerialization,
        $cp343PipelineSnapshotSerialization,
        $cp343CoupledRuntimeTests
    )) {
    Assert-FileExists -Path $cp343RequiredFile -Description "CP343 supply-temperature assignment structure"
}
Assert-LineLimit -Path $cp343Release -Limit 800 -Description "CP343 release module"
Assert-LineLimit -Path $cp343CoupledValidation -Limit 800 -Description "CP343 coupled validation module"
Assert-LineLimit -Path $cp343Pipeline -Limit 800 -Description "CP343 pipeline module"

$cp343SourceStatementPattern =
    'PurchAir\.SupplyTemp\s*=\s*PsyTdbFnHW\(\s*SupplyEnthalpy\s*,\s*PurchAir\.SupplyHumRat\s*\)\s*;'
$cp343FirstExcludedStatementPattern =
    'PurchAir\.SupplyTemp\s*=\s*min\(\s*PurchAir\.SupplyTemp\s*,\s*PurchAir\.MixedAirTemp\s*\)\s*;'
$cp343OrderedSourceSitesPattern = '(?s)' +
    'read-local-supply-enthalpy-for-dry-bulb-inversion.*?' +
    'read-purchased-air-supply-humidity-ratio-for-dry-bulb-inversion.*?' +
    'evaluate-psy-tdb-fn-h-w.*?' +
    'assign-purchased-air-supply-temperature'
$cp343PurchasedAirSourceHash =
    '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005'
$cp343PsychrometricsSourceHash =
    '30C9575BC5A8E73D33D111E0D54A4DA8916AF4534175E9B95071ACA2513AEF45'

# Exact source boundary, public surface, routes, and four-site inventory.
Assert-Contains -Path $cp343Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2201' -Description "CP343 exact physical source boundary"
Assert-Contains -Path $cp343Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2203' -Description "CP343 first excluded physical executable"
Assert-Contains -Path $cp343Module -Pattern 'Exact four textual source sites represented by CP343' -Description "CP343 source-site count"
Assert-ExactStringArray -Path $cp343Module -Name "PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER" -Expected @(
    "read-local-supply-enthalpy-for-dry-bulb-inversion",
    "read-purchased-air-supply-humidity-ratio-for-dry-bulb-inversion",
    "evaluate-psy-tdb-fn-h-w",
    "assign-purchased-air-supply-temperature"
) -Description "CP343 deterministic source witness"
Assert-Contains -Path $cp343Module -Pattern ('pub struct ' + $cp343TypeStem + 'Snapshot') -Description "CP343 public snapshot"
Assert-Contains -Path $cp343State -Pattern ('pub struct ' + $cp343TypeStem + 'RuntimeState') -Description "CP343 persistent state"
Assert-Contains -Path $cp343Module -Pattern ('pub struct ' + $cp343TypeStem + 'LifecycleSummary') -Description "CP343 lifecycle summary"
Assert-Contains -Path $cp343Module -Pattern 'pub fn purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle_summary\s*\(' -Description "CP343 lifecycle accessor"
Assert-Contains -Path $cp343Release -Pattern 'pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\s*\(' -Description "CP343 exact-direct release"
Assert-Contains -Path $cp343Transition -Pattern 'advance_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_state\s*\(' -Description "CP343 pure transition"
Assert-Contains -Path $cp343CalcRoot -Pattern 'mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;' -Description "CP343 Calc module declaration"

foreach ($cp343Route in @(
        "UnitOff",
        "NonCooling",
        "PositiveGuardFalseFallthrough",
        "ActiveCapacityLimitGuardFalseFallthrough",
        "CapacityLimitSensibleOutputGuardFalseFallthrough",
        "CapacityLimitSensibleOutputSupplyTemperatureAssigned"
    )) {
    Assert-Contains -Path $cp343State -Pattern $cp343Route -Description "CP343 retained route '$cp343Route'"
}
foreach ($cp343Counter in @(
        "transition_count",
        "unit_off_skip_count",
        "non_cooling_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "capacity_limit_guard_false_fallthrough_skip_count",
        "capacity_limit_sensible_output_guard_false_fallthrough_count",
        "capacity_limit_sensible_output_supply_temperature_assignment_count",
        "source_site_execution_count",
        "supply_enthalpy_for_dry_bulb_inversion_read_count",
        "supply_humidity_ratio_for_dry_bulb_inversion_read_count",
        "psychrometric_supply_temperature_evaluation_count",
        "supply_temperature_assignment_write_count"
    )) {
    Assert-Contains -Path $cp343State -Pattern ('pub ' + $cp343Counter + ':\s*usize') -Description "CP343 state counter '$cp343Counter'"
}

# The transition must call the canonical source-shaped inverse exactly once.
Assert-PatternsInOrder -Path $cp343Transition -Patterns @(
    'energyplus_psy_tdb_fn_h_w\(\s*operands\.supply_enthalpy_j_per_kg,\s*operands\.supply_humidity_ratio,\s*\)',
    'assigned_supply_temperature_c\s*=\s*[\r\n\s]*psychrometric_supply_temperature_result_c',
    'resulting_supply_temperature_c\s*=\s*if assignment_executed'
) -Description "CP343 helper evaluation, assignment, and result order"
Assert-NotContains -Path $cp343Transition -Pattern '2\.500_?94e6|1\.004_?84e3|1\.858_?95e3|mul_add|total_cmp|partial_cmp|\.clamp\(|is_finite|\.max\(|\.min\(' -Description "CP343 transition delegates formula without coercion"
Assert-Contains -Path $cp343Psychrometrics -Pattern '(?s)pub fn energyplus_psy_tdb_fn_h_w\(enthalpy_j_per_kg: f64, humidity_ratio: f64\) -> f64 \{\s*let humidity_ratio = energyplus_humidity_ratio_floor\(humidity_ratio\);\s*\(enthalpy_j_per_kg - 2\.500_94e6 \* humidity_ratio\) / \(1\.004_84e3 \+ 1\.858_95e3 \* humidity_ratio\)\s*\}' -Description "canonical PsyTdbFnHW exact grouping"
Assert-Contains -Path $cp343Psychrometrics -Pattern '(?s)fn energyplus_humidity_ratio_floor\(humidity_ratio: f64\) -> f64 \{.*?if humidity_ratio < ENERGYPLUS_MIN_HUMIDITY_RATIO \{\s*ENERGYPLUS_MIN_HUMIDITY_RATIO\s*\} else \{\s*humidity_ratio\s*\}' -Description "canonical first-argument NaN and floor semantics"
Assert-Contains -Path $cp343PsychrometricsTests -Pattern 'psy_tdb_matches_pinned_source_formula_vectors_bitwise' -Description "PsyTdbFnHW pinned vectors"
Assert-Contains -Path $cp343PsychrometricsTests -Pattern 'psy_tdb_applies_the_source_humidity_floor_and_nan_semantics' -Description "PsyTdbFnHW floor and IEEE regression"

# Checked lifecycle algebra is T=U+N+P+G+F+D, A=F+D, and
# D=CP342 H=CP341 M=CP340 body entries, with four source sites per D.
Assert-Contains -Path $cp343RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_supply_temperature_assignment_count\s*\.checked_mul\(4\)' -Description "CP343 checked 4*D source formula"
Assert-Contains -Path $cp343RuntimeValidation -Pattern '(?s)capacity_limit_sensible_output_supply_temperature_assignment_count\s*==\s*predecessor\s*\.capacity_limit_sensible_output_supply_enthalpy_assignment_count' -Description "CP343 D equals CP342 H"
Assert-Contains -Path $cp343RuntimeValidation -Pattern '(?s)route_partition == state\.transition_count.*?source_site_execution_count == expected_source_sites.*?supply_enthalpy_for_dry_bulb_inversion_read_count == assigned.*?supply_humidity_ratio_for_dry_bulb_inversion_read_count == assigned.*?psychrometric_supply_temperature_evaluation_count == assigned.*?supply_temperature_assignment_write_count == assigned' -Description "CP343 route/source/site identities"
Assert-Contains -Path $cp343Cp342Audit -Pattern 'H equals CP341 M' -Description "cumulative CP343 D/H/M identity"
Assert-Contains -Path $cp343Cp342Audit -Pattern 'H equals CP340 body entries' -Description "cumulative CP343 D/H/CP340 identity"
foreach ($cp343PreflightCounter in @(
        "capacity_limit_sensible_output_supply_temperature_assignment_count",
        "source_site_execution_count",
        "supply_enthalpy_for_dry_bulb_inversion_read_count",
        "supply_humidity_ratio_for_dry_bulb_inversion_read_count",
        "psychrometric_supply_temperature_evaluation_count",
        "supply_temperature_assignment_write_count",
        "witnessed_capacity_limit_sensible_output_supply_temperature_assignment_count"
    )) {
    Assert-Contains -Path $cp343RuntimeValidation -Pattern ($cp343PreflightCounter + '[\r\n\s.]+checked_add\(') -Description "CP343 checked preflight '$cp343PreflightCounter'"
}

# Snapshot shapes preserve CP334 temperature on CP340 false, expose no values
# on inherited skips, and apply no new finite gate to CP342 H or the result.
Assert-Contains -Path $cp343SnapshotValidation -Pattern '(?s)fn false_fallthrough_snapshot_is_exact\(.*?preexisting\.is_finite\(\).*?preexisting\.to_bits\(\) == resulting\.to_bits\(\).*?source_values_are_none' -Description "CP343 false route preserves finite CP334 temperature"
Assert-Contains -Path $cp343SnapshotValidation -Pattern '(?s)fn assigned_snapshot_is_exact\(.*?expected = energyplus_psy_tdb_fn_h_w\(enthalpy, humidity\).*?humidity\.is_finite\(\).*?humidity >= 0\.0.*?psychrometric\.to_bits\(\) == expected\.to_bits\(\).*?assigned\.to_bits\(\) == psychrometric\.to_bits\(\).*?resulting\.to_bits\(\) == assigned\.to_bits\(\)' -Description "CP343 exact helper and assignment bits"
Assert-NotContains -Path $cp343SnapshotValidation -Pattern 'enthalpy\.is_finite\(\)|psychrometric\.is_finite\(\)|assigned\.is_finite\(\)|resulting\.is_finite\(\)' -Description "CP343 adds no derived finite gate"
Assert-Contains -Path $cp343SnapshotValidation -Pattern '(?s)fn skipped_snapshot_is_exact\(.*?preexisting_supply_temperature_c\.is_none\(\).*?resulting_supply_temperature_c\.is_none\(\).*?source_values_are_none' -Description "CP343 inherited complete-null firewall"

# Public release accepts only runtime/system/CP342. CP342 owns enthalpy, CP334
# owns preexisting temperature, CP335 owns humidity, and CP336 corroborates.
Assert-Contains -Path $cp343Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor_cp342:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,\s*\)' -Description "CP343 exact public arguments"
Assert-Contains -Path $cp343Release -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witness\s*\(' -Description "CP343 CP342 private witness"
Assert-Contains -Path $cp343Release -Pattern 'completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_is_consistent\s*\(' -Description "CP343 recursive CP342 proof"
Assert-Contains -Path $cp343Release -Pattern 'cooling_positive_supply_temperature_mixed_air_limit_latest_witness\s*\(' -Description "CP343 CP334 private owner witness"
Assert-Contains -Path $cp343Release -Pattern 'cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness\s*\(' -Description "CP343 CP335 private owner witness"
Assert-Contains -Path $cp343Release -Pattern 'cooling_positive_supply_enthalpy_assignment_latest_witness\s*\(' -Description "CP343 CP336 corroborating witness"
Assert-Contains -Path $cp343PrefixValidation -Pattern 'retained_source_owner_lineage_is_exact\s*\(' -Description "CP343 retained owner validation"
Assert-Contains -Path $cp343PrefixValidation -Pattern '(?s)predecessor\.resulting_supply_enthalpy_j_per_kg.*?assignment\.supply_enthalpy_j_per_kg' -Description "CP343 CP342 enthalpy ownership"
Assert-Contains -Path $cp343PrefixValidation -Pattern '(?s)cp334\.assigned_supply_temperature_c.*?assignment\.preexisting_supply_temperature_c' -Description "CP343 CP334 temperature ownership"
Assert-Contains -Path $cp343PrefixValidation -Pattern '(?s)cp335\.assigned_supply_humidity_ratio.*?assignment\.supply_humidity_ratio' -Description "CP343 CP335 humidity ownership"
Assert-Contains -Path $cp343PrefixValidation -Pattern '(?s)cp336\.supply_temperature_c.*?cp336\.supply_humidity_ratio.*?owner_temperature\.to_bits\(\) == read_temperature\.to_bits\(\).*?owner_humidity\.to_bits\(\) == read_humidity\.to_bits\(\)' -Description "CP343 CP336 corroborating lineage"
Assert-NotContains -Path $cp343Release -Pattern '(?s)pub fn advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^)]*(supply_enthalpy|supply_humidity_ratio|supply_temperature)\s*:' -Description "CP343 duplicate caller operands"
Assert-NotContains -Path $cp343Release -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|DirectZonePurchasedAirCouplingInput|ZoneHeatBalanceState|zone_state|psychrometric_service|cache|diagnostic' -Description "CP343 service/numerical firewall"

# Runtime-root state and latest witness stay private and system-rooted.
Assert-Contains -Path $cp343InitState -Pattern ('(?s)' + $cp343Stem + '_latest_witnesses:\s*BTreeMap<\s*IdealLoadsAirSystemId,\s*' + $cp343TypeStem + 'Snapshot') -Description "runtime-root private CP343 witness map"
Assert-NotContains -Path $cp343InitState -Pattern ('(?m)^\s*pub(?:\([^)]*\))?\s+' + $cp343Stem + '_latest_witnesses:') -Description "CP343 witness map remains private"
Assert-Contains -Path $cp343InitWitnessRoot -Pattern ('mod ' + $cp343Stem + ';') -Description "CP343 witness module"
Assert-Contains -Path $cp343InitWitness -Pattern ($cp343Stem + '_latest_witness\s*\(') -Description "CP343 private witness getter"
Assert-Contains -Path $cp343InitWitness -Pattern ('set_' + $cp343Stem + '_latest_witness\s*\(') -Description "CP343 private witness setter"
Assert-Contains -Path $cp343InitState -Pattern ('pub calc_' + $cp343Stem + ':') -Description "per-unit CP343 state"
Assert-Contains -Path $cp343InitUnit -Pattern ('(?s)calc_' + $cp343Stem + ':\s*' + $cp343TypeStem + 'RuntimeState::new\(\s*system') -Description "per-unit CP343 initialization"

# Binding order is exactly CP342 -> CP343 -> unchanged numerical DTO.
$cp343BindingText = Read-RepoText -Path $cp343Binding
$cp342BindingCallForCp343 = [regex]::Match(
    $cp343BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment\([^;]+?\)\?;'
)
$cp343BindingCall = [regex]::Match(
    $cp343BindingText,
    '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\([^;]+?\)\?;'
)
$cp343NumericalIndex = $cp343BindingText.IndexOf('let coupling = complete_direct_zone_purchased_air_coupling(')
if (
    -not $cp342BindingCallForCp343.Success -or
    -not $cp343BindingCall.Success -or
    $cp343BindingCall.Index -lt ($cp342BindingCallForCp343.Index + $cp342BindingCallForCp343.Length) -or
    $cp343NumericalIndex -lt ($cp343BindingCall.Index + $cp343BindingCall.Length)
) {
    throw "Binding must complete CP342 then CP343 before unchanged numerical coupling"
}
foreach ($cp343BindingInterval in @(
        [PSCustomObject]@{
            Start = $cp342BindingCallForCp343.Index + $cp342BindingCallForCp343.Length
            End = $cp343BindingCall.Index
            Description = "after CP342 and before CP343"
        },
        [PSCustomObject]@{
            Start = $cp343BindingCall.Index + $cp343BindingCall.Length
            End = $cp343NumericalIndex
            Description = "after CP343 and before numerical coupling"
        }
    )) {
    $cp343BindingIntervalText = $cp343BindingText.Substring(
        $cp343BindingInterval.Start,
        $cp343BindingInterval.End - $cp343BindingInterval.Start
    )
    $cp343BindingIntervalCode =
        [regex]::Replace($cp343BindingIntervalText, '(?m)//.*$', '')
    $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit =\s*advance_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;',
        ''
    )
    $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment =\s*advance_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;',
        ''
    )
    $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;',
        ''
    )
    $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_none_case\([^;]+?\)\?;',
        ''
    )
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_constant_shr_case_break =\s*advance_cooling_constant_shr_case_break\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_case_entry =\s*advance_cooling_humidistat_case_entry\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_moisture_demand_assignment =\s*advance_cooling_humidistat_moisture_demand_assignment\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)let calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit =\s*advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit\([^;]+?\)\?;',
    ''
)
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)(?:let calculation_cooling_humidistat_case_break =\s*advance_cooling_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_assignment =\s*advance_cooling_constant_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_constant_supply_humidity_ratio_case_break =\s*advance_cooling_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_default_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_default_supply_humidity_ratio_case_break =\s*advance_cooling_default_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =\s*advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =\s*advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =\s*advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment =\s*advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment =\s*advance_cooling_supply_humidity_ratio_saturation_limit_assignment\([^;]+?\)\?;|let calculation_cooling_supply_enthalpy_post_saturation_assignment =\s*advance_cooling_supply_enthalpy_post_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_guard =\s*advance_cooling_post_saturation_capacity_limit_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment\([^;]+?\)\?;)',
    ''
)
    $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment\([^;]+?\)\?;',
        ''
    )
        $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment\([^;]+?\)\?;',
        ''
    )
        $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry\([^;]+?\)\?;',
        ''
    )
        $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment\([^;]+?\)\?;',
        ''
    )
        $cp343BindingIntervalCode = [regex]::Replace(
        $cp343BindingIntervalCode,
        '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry\([^;]+?\)\?;)',
        ''
    )
$cp343BindingIntervalCode = [regex]::Replace(
    $cp343BindingIntervalCode,
    '(?s)(?:let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment\([^;]+?\)\?;|let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment =\s*advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment\([^;]+?\)\?;|let calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry =\s*advance_cooling_supply_mass_flow_positive_guard_else_branch_entry\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment =\s*advance_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment =\s*advance_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment\([^;]+?\)\?;|let calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment =\s*advance_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment\([^;]+?\)\?;|let calculation_heating_or_no_load_case_entry =\s*advance_heating_or_no_load_case_entry\([^;]+?\)\?;|let calculation_heating_mode_guard =\s*advance_heating_mode_guard\([^;]+?\)\?;|let calculation_heating_operating_mode_heat_assignment =\s*advance_heating_operating_mode_heat_assignment\([^;]+?\)\?;)',
    ''
)
    if ($cp343BindingIntervalCode -match '(?<![A-Za-z0-9_])(?:\b[A-Za-z_][A-Za-z0-9_:]*|\.[A-Za-z_][A-Za-z0-9_]*)!?\s*\(') {
        throw "No intermediary helper call may execute $($cp343BindingInterval.Description)"
    }
}
Assert-Contains -Path $cp343Binding -Pattern '(?s)advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\(\s*input\.purchased_air_runtime_state,\s*binding\.system,\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment,\s*\)\?;' -Description "binding exact CP342-to-CP343 call"
Assert-Contains -Path $cp343BindingAdapter -Pattern '(?s)pub\(super\) fn advance_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\(\s*runtime: &mut PurchasedAirRuntimeState,\s*system: &IdealLoadsAirSystem,\s*predecessor:\s*PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,\s*\)' -Description "CP343 binding adapter arguments"
Assert-NotContains -Path $cp343BindingAdapter -Pattern 'supply_enthalpy\s*:|supply_humidity_ratio\s*:|supply_temperature\s*:|latest_numerical|complete_direct_zone_purchased_air_coupling' -Description "CP343 binding excludes duplicate operands"
Assert-Contains -Path $cp343ScheduledOutput -Pattern 'pub calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment:' -Description "CP343 scheduled output"
Assert-Contains -Path $cp343BindingTestsRoot -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_tests\.rs' -Description "CP343 binding test module"

# Coupled runtime and pipeline carry direct-only evidence without DTO input.
Assert-Contains -Path $cp343CoupledRuntime -Pattern ('mod ' + $cp343Stem + '_validation;') -Description "coupled CP343 validator"
Assert-Contains -Path $cp343CoupledRuntime -Pattern ('pub calc_' + $cp343Stem + '_lifecycle:') -Description "coupled CP343 lifecycle"
Assert-Contains -Path $cp343CoupledRuntime -Pattern ($cp343Stem + '_validation::snapshot_matches_release') -Description "coupled CP343 snapshot validation"
Assert-Contains -Path $cp343CoupledRuntime -Pattern ($cp343Stem + '_validation::validate_lifecycle') -Description "coupled CP343 final validation"
Assert-Contains -Path $cp343CoupledValidation -Pattern 'output\s*\.\s*calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment' -Description "coupled CP342 predecessor"
Assert-Contains -Path $cp343CoupledValidation -Pattern 'output\s*\.\s*calculation_cooling_positive_supply_temperature_mixed_air_limit' -Description "coupled CP334 temperature owner"
Assert-Contains -Path $cp343CoupledValidation -Pattern 'output\s*\.\s*calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment' -Description "coupled CP335 humidity owner"
Assert-Contains -Path $cp343CoupledValidation -Pattern 'output\s*\.\s*calculation_cooling_positive_supply_enthalpy_assignment' -Description "coupled CP336 corroboration"
Assert-Contains -Path $cp343CoupledValidation -Pattern '(?s)let\s+assignments\s*=\s*state\.capacity_limit_sensible_output_supply_temperature_assignment_count\s*;.*?checked_mul\(\s*assignments,\s*4,' -Description "coupled CP343 checked 4*D"
Assert-NotContains -Path $cp343CoupledValidation -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|psychrometric_service|cache' -Description "coupled CP343 numerical/service firewall"
Assert-Contains -Path $cp343CoupledFixtureRoot -Pattern 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_fixture\.rs' -Description "CP343 coupled fixture module"
Assert-Contains -Path $cp343PipelineRoot -Pattern ('mod ' + $cp343PipelineStem + ';') -Description "pipeline CP343 module"
Assert-Contains -Path $cp343PipelineRoot -Pattern 'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle' -Description "pipeline CP343 lifecycle and JSON key"
Assert-Contains -Path $cp343Pipeline -Pattern 'predecessor_cp342' -Description "pipeline CP342 predecessor"
Assert-Contains -Path $cp343PipelineValidation -Pattern '(?s)let\s+assignments\s*=\s*state\.capacity_limit_sensible_output_supply_temperature_assignment_count\s*;.*?assignments\s*\.checked_mul\(\s*PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline CP343 checked 4*D"
Assert-NotContains -Path $cp343Pipeline -Pattern 'latest_numerical|numerical_supply|final_supply|complete_direct_zone_purchased_air_coupling|psychrometric_service|cache' -Description "pipeline CP343 numerical/service firewall"
foreach ($cp343JsonField in @(
        "preexisting_supply_temperature_c",
        "supply_enthalpy_j_per_kg",
        "supply_humidity_ratio",
        "psychrometric_supply_temperature_result_c",
        "assigned_supply_temperature_c",
        "resulting_supply_temperature_c"
    )) {
    Assert-Contains -Path $cp343PipelineSnapshotSerialization -Pattern ('"' + $cp343JsonField + '"') -Description "pipeline CP343 JSON field '$cp343JsonField'"
}
Assert-Contains -Path $cp343PipelineSnapshotSerialization -Pattern '(?s)fn json_number\(value: Option<f64>\) -> Value.*?filter\(\|value\| value\.is_finite\(\)\).*?map_or\(Value::Null' -Description "CP343 nonfinite numeric null projection"
Assert-Contains -Path $cp343PipelineSnapshotSerialization -Pattern 'value\.map\(\|value\| format!\("0x\{:016x\}", value\.to_bits\(\)\)\)' -Description "CP343 authoritative IEEE bits"
Assert-Contains -Path $cp343PipelineRoot -Pattern '(?s)purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle:\s*None' -Description "non-direct CP343 null evidence"
Assert-Contains -Path $cp343PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp432_lifecycle_evidence' -Description "non-direct CP343 through CP363 evidence rejection"

# Specs contain exactly two addenda and the 2+4 target distribution.
$cp343AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp343AlgorithmAddenda = [regex]::Matches(
    $cp343AlgorithmText,
    '(?m)^\s*"CP343 supersedes only CP342[^"\r\n]+",\s*$'
)
if ($cp343AlgorithmAddenda.Count -ne 2) {
    throw "Algorithm ledger must contain exactly two CP343 claim addenda"
}
$cp343TargetCounts = @(
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment/release\.rs::advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\.rs::purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle_summary'
        Expected = 2
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState'
        Expected = 1
    },
    [PSCustomObject]@{
        Pattern = 'cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment\.rs::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary'
        Expected = 1
    }
)
foreach ($cp343Target in $cp343TargetCounts) {
    $cp343TargetCount = [regex]::Matches($cp343AlgorithmText, $cp343Target.Pattern).Count
    if ($cp343TargetCount -ne $cp343Target.Expected) {
        throw "CP343 target '$($cp343Target.Pattern)' expected $($cp343Target.Expected), found $cp343TargetCount"
    }
}
$cp343CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp343CapabilityAddenda = [regex]::Matches(
    $cp343CapabilityText,
    '(?m)^\s*"CP343 additionally requires[^"\r\n]+",\s*$'
)
if ($cp343CapabilityAddenda.Count -ne 2) {
    throw "Capability registry must contain exactly two CP343 claim addenda"
}
foreach ($cp343Claim in @($cp343AlgorithmAddenda) + @($cp343CapabilityAddenda)) {
    foreach ($cp343Pattern in @(
            '6f2e40d10250a105b49966baa24d843711e61048',
            $cp343PurchasedAirSourceHash,
            $cp343PsychrometricsSourceHash,
            $cp343SourceStatementPattern,
            $cp343OrderedSourceSitesPattern,
            'Psychrometrics\.hh.*?743-762',
            'W=max\(dW,1\.0e-5\)',
            '\(H - 2\.50094e6 \* W\) / \(1\.00484e3 \+ 1\.85895e3 \* W\)',
            'CapacityLimitSensibleOutputSupplyEnthalpyAssigned',
            'CapacityLimitSensibleOutputSupplyTemperatureAssigned',
            'T=U\+N\+P\+G\+F\+D',
            'A=F\+D',
            'D=H=M=CP340 adjustment-body entries',
            'source_site_execution_count=4\*D',
            'supplied/latest/private CP342',
            'resulting_supply_enthalpy_j_per_kg',
            'CP335 `assigned_supply_humidity_ratio` owns humidity',
            'CP334 `assigned_supply_temperature_c` owns preexisting temperature',
            'CP336 latest/private.*?corroborates',
            'no new finite gate',
            'defensive characterization',
            'not a new full-public-chain reachability claim',
            'CP342-to-CP343-to-unchanged-numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle',
            'line 2202 is commentary',
            $cp343FirstExcludedStatementPattern,
            'Roadmap promotion'
        )) {
        if ($cp343Claim.Value -notmatch $cp343Pattern) {
            throw "CP343 spec addendum missing '$cp343Pattern'"
        }
    }
}
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'routine\.psy_tdb_fn_h_w\.completion_status = "state_mapped"' -Description "PsyTdbFnHW remains state_mapped"
Assert-Contains -Path "specs\algorithm_ledger.toml" -Pattern 'broader downstream IdealLoads inverse replacement beyond bounded CP343 physical line 2201' -Description "PsyTdbFnHW broader downstream nonclaim"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP343 supersedes only CP342' -Description "generated CP343 algorithm ledger"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP343 additionally requires' -Description "generated CP343 capability index"

# Five hand-authored contracts and the psychrometrics source map carry the
# source, owner, IEEE, lifecycle, exclusion, and no-promotion boundaries.
$cp343DocumentationSections = @(
    [PSCustomObject]@{
        Path = "docs\src\current\current-status.md"
        Pattern = '(?ms)^CP343 now maps only.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\current\project-contract.md"
        Pattern = '(?ms)^## CP343 Source-Ordered Cooling Positive-Supply Capacity-Limit Supply-Temperature Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\ideal-loads-source-map.md"
        Pattern = '(?ms)^## CP343 Cooling Positive-Supply Capacity-Limit Supply-Temperature Assignment\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\heat-balance-source-map.md"
        Pattern = '(?ms)^## CP343 Positive-Supply Capacity-Limit Supply-Temperature Assignment in the Heat-Balance Loop\r?\n.*?(?=^## |\z)'
    },
    [PSCustomObject]@{
        Path = "docs\src\porting-map\zone-air-update-map.md"
        Pattern = '(?ms)^## CP343 Cooling Positive-Supply Capacity-Limit Supply-Temperature Assignment Placement\r?\n.*?(?=^## |\z)'
    }
)
foreach ($cp343Documentation in $cp343DocumentationSections) {
    $cp343DocumentText = Read-RepoText -Path $cp343Documentation.Path
    $cp343Matches = [regex]::Matches($cp343DocumentText, $cp343Documentation.Pattern)
    if ($cp343Matches.Count -ne 1) {
        throw "CP343 documentation expected one scoped section in $($cp343Documentation.Path), found $($cp343Matches.Count)"
    }
    $cp343Section = $cp343Matches[0].Value
    foreach ($cp343Pattern in @(
            $cp343PurchasedAirSourceHash,
            $cp343PsychrometricsSourceHash,
            $cp343SourceStatementPattern,
            $cp343OrderedSourceSitesPattern,
            'Psychrometrics\.hh',
            '743-762',
            'max\(dW,\s*1\.0e-5\)',
            'first-argument\s+NaN',
            'CapacityLimitSensibleOutputSupplyEnthalpyAssigned',
            'CapacityLimitSensibleOutputSupplyTemperatureAssigned',
            'T\s*=\s*U\+N\+P\+G\+F\+D',
            'A\s*=\s*F\+D',
            'D\s*=\s*H\s*=\s*M\s*=\s*CP340 adjustment-body entries',
            '4\*D',
            '(?s)supplied.*?(?:latest|retained-latest).*?private CP342',
            'resulting_supply_enthalpy_j_per_kg',
            '(?s)(?:CP335.*?(?:owns|owned).*?humidity|humidity.*?owned.*?CP335)',
            '(?s)(?:CP334.*?(?:owns|owned).*?preexisting temperature|preexisting temperature.*?owned.*?CP334)',
            '(?s)CP336.*?corroborat',
            'no\s+new\s+finite\s+gate',
            '(?s)(?:pure|defensive).*?characterization.*?(?:not|rather than).*?(?:full-public|public)',
            'IEEE',
            'CP342-to-CP343-to-(?:unchanged-)?numerical',
            'purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle',
            'line\s+2202\s+is\s+commentary',
            $cp343FirstExcludedStatementPattern,
            '(?i)numerical[- ]DTO',
            'state_mapped',
            'source_mapped',
            'Roadmap'
        )) {
        if ($cp343Section -notmatch $cp343Pattern) {
            throw "CP343 documentation in $($cp343Documentation.Path) missing '$cp343Pattern'"
        }
    }
    if ($cp343Section -match '(?is)public(?:ly)? reachable.{0,100}(?:-infinity|NaN)|(?:-infinity|NaN).{0,100}public(?:ly)? reachable') {
        throw "CP343 documentation must not promote unproved nonfinite full-public reachability in $($cp343Documentation.Path)"
    }
}
Assert-Contains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern $cp343PsychrometricsSourceHash -Description "psychrometrics locked raw source"
Assert-Contains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern 'bounded `PurchasedAirManager\.cc` physical-line-2201 direct lifecycle' -Description "bounded CP343 PsyTdbFnHW integration"
Assert-Contains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern 'broader downstream IdealLoads inverse replacement beyond bounded CP343 physical line 2201' -Description "psychrometrics broader downstream nonclaim"

# Historical audits must explicitly admit only the CP343 binding call and
# carry the renamed non-direct firewall and cumulative inventory.
foreach ($cp343HistoricalBindingAudit in @(
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
        "scripts\quality\ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1"
    )) {
    Assert-Contains -Path $cp343HistoricalBindingAudit -Pattern 'calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment' -Description "historical binding interval admits only CP343"
}
foreach ($cp343HistoricalFirewallAudit in @(
        "scripts\quality\ideal-loads-structure-audit\cp334-cooling-positive-supply-temperature-mixed-air-limit.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp335-cooling-positive-supply-humidity-ratio-mixed-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp336-cooling-positive-supply-enthalpy-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp337-cooling-positive-supply-capacity-limit-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp338-cooling-positive-supply-capacity-limit-cp-air-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp339-cooling-positive-supply-capacity-limit-sensible-output-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp340-cooling-positive-supply-capacity-limit-sensible-output-guard.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp341-cooling-positive-supply-capacity-limit-sensible-output-maximum-capacity-assignment.ps1",
        "scripts\quality\ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1"
    )) {
    Assert-Contains -Path $cp343HistoricalFirewallAudit -Pattern 'non_direct_runtime_rejects_cp316_through_cp432_lifecycle_evidence' -Description "historical non-direct firewall reaches CP362"
}

# Root reachability and generated inventory add one internal script:
# 284 executable records, 240 public, 44 internal, and zero uncalled.
$cp343MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp342DotSourceIndexForCp343 = $cp343MainAuditText.IndexOf('ideal-loads-structure-audit\cp342-cooling-positive-supply-capacity-limit-sensible-output-supply-enthalpy-assignment.ps1')
$cp343DotSourceIndex = $cp343MainAuditText.IndexOf('ideal-loads-structure-audit\cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment.ps1')
$cp343AuditCompletionIndex = $cp343MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if (
    $cp342DotSourceIndexForCp343 -lt 0 -or
    $cp343DotSourceIndex -le $cp342DotSourceIndexForCp343 -or
    $cp343AuditCompletionIndex -le $cp343DotSourceIndex
) {
    throw "Main IdealLoads audit must dot-source CP343 after CP342 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 370' -Description "CP343 cumulative inventory total through CP403"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment\.ps1"' -Description "CP343 internal script inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'scripts/quality/ideal-loads-structure-audit/cp343-cooling-positive-supply-capacity-limit-sensible-output-supply-temperature-assignment\.ps1::dot_sources' -Description "CP343 main-audit callee evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 370 \|' -Description "CP343 generated script count through CP403"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "CP343 generated public script count"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 130 \|' -Description "CP343 generated internal script count through CP403"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "CP343 generated uncalled script count"
