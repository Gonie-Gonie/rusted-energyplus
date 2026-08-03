# CP362 maps PurchasedAirManager.cc line 2232 Humidistat mixed-air limit.
$cp362Stem = "cooling_humidistat_supply_humidity_ratio_mixed_air_limit"
$cp361StemForCp362 = "cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit"
$cp362PipelineStem = "purchased_air_$cp362Stem"
$cp362TypeStem = "PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimit"
$cp362Lifecycle = "purchased_air_calc_${cp362Stem}_lifecycle"
$cp362SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp362SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp362Sites = @(
    "read-purchased-air-mixed-air-humidity-ratio-for-humidistat-mixed-air-limit-minimum",
    "read-local-supply-humidity-ratio-for-dehumidification-for-humidistat-mixed-air-limit-minimum",
    "apply-source-shaped-two-argument-minimum-for-humidistat-mixed-air-limit",
    "assign-purchased-air-supply-humidity-ratio-for-humidistat-mixed-air-limit"
)
$cp362Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp362Module = "crates\ep_runtime\src\ideal_loads\calc\$cp362Stem.rs"
$cp362ModuleRoot = "crates\ep_runtime\src\ideal_loads\calc\$cp362Stem"
$cp362Release = "$cp362ModuleRoot\release.rs"
$cp362RuntimeValidation = "$cp362ModuleRoot\release\runtime_validation.rs"
$cp362LegacyTests = "crates\ep_runtime\src\ideal_loads\calc\${cp362Stem}_tests.rs"
$cp362SharedMinimum = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_mixed_air_limit\transition.rs"
$cp362CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp362Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp362BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp362Stem.rs"
$cp362BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp362ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp362InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp362InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp362InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp362Stem.rs"
$cp362CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp362Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp362Stem}_validation.rs"
$cp362CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp362FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp362Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp362Stem}_fixture.rs"
$cp362PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp362Pipeline = "crates\ep_run\src\pipeline\$cp362PipelineStem.rs"
$cp362PipelineValidation = "crates\ep_run\src\pipeline\$cp362PipelineStem\validation.rs"
$cp362PipelineTests = "crates\ep_run\src\pipeline\$cp362PipelineStem\validation\tests.rs"
$cp362Serialization = "crates\ep_run\src\pipeline\$cp362PipelineStem\serialization.rs"
$cp362SnapshotSerialization = "crates\ep_run\src\pipeline\$cp362PipelineStem\serialization\snapshot.rs"
$cp362ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp361_assertions.rs"
$cp362ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp362_assertions.rs"
$cp362Audit = "scripts\quality\ideal-loads-structure-audit\cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1"
function Assert-Cp362TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) {
        throw "CP362 $Description missing"
    }
}
function Assert-Cp362TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) {
        throw "CP362 $Description unexpectedly present"
    }
}
function Get-Cp362RustBraceBlock {
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
function Assert-Cp362ProductionHasNoPanics {
    param([string]$Path)
    $text = Read-RepoText -Path $Path
    if ($text -match '(?m)\b(?:expect|unwrap)\s*\(|panic!\s*\(') {
        throw "CP362 production path contains expect/unwrap/panic: $Path"
    }
}
foreach ($required in @(
        $cp362Source, $cp362Module, $cp362Release, $cp362RuntimeValidation,
        $cp362SharedMinimum,
        $cp362BindingAdapter, $cp362BindingTestsRoot, $cp362InitWitness,
        $cp362Coupled, $cp362CoupledTestsRoot, $cp362Fixture,
        $cp362Pipeline, $cp362PipelineValidation, $cp362PipelineTests,
        $cp362Serialization, $cp362SnapshotSerialization,
        $cp362ParentAssertions, $cp362ArbitraryAssertions, $cp362Audit
    )) {
    Assert-FileExists -Path $required -Description "CP362 structure"
}
$cp362CalcFiles = @($cp362Module) + @(
    Get-ChildItem -LiteralPath $cp362ModuleRoot -Recurse -File -Filter "*.rs" |
        ForEach-Object { $_.FullName }
)
$cp362TestFiles = @(
    (@($cp362LegacyTests) + @(
            $cp362CalcFiles | Where-Object {
                $_ -match '(?:_tests\.rs$|[\\/]tests[\\/])'
            }
        )) |
        Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } |
        Select-Object -Unique
)
if ($cp362TestFiles.Count -eq 0) {
    throw "CP362 calc regression tests missing"
}
$cp362ProductionFiles = @(
    $cp362CalcFiles | Where-Object { $cp362TestFiles -notcontains $_ }
)
foreach ($limited in @(
        $cp362CalcFiles + $cp362TestFiles + @(
            $cp362BindingAdapter, $cp362InitWitness, $cp362Coupled,
            $cp362Pipeline, $cp362PipelineValidation, $cp362PipelineTests,
            $cp362Serialization, $cp362SnapshotSerialization,
            $cp362ArbitraryAssertions, $cp362Audit
        )
    ) | Select-Object -Unique) {
    Assert-LineLimit -Path $limited -Limit 500 -Description "CP362 bounded structure"
}
foreach ($production in @(
        $cp362ProductionFiles + @(
            $cp362BindingAdapter, $cp362InitWitness, $cp362Coupled,
            $cp362Pipeline, $cp362PipelineValidation,
            $cp362Serialization, $cp362SnapshotSerialization
        )
    ) | Select-Object -Unique) {
    Assert-Cp362ProductionHasNoPanics -Path $production
}
$cp362CalcText = ($cp362CalcFiles | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
$cp362TestsText = ($cp362TestFiles | ForEach-Object { Read-RepoText -Path $_ }) -join "`n"
$cp362SemanticTestFiles = @(Get-ChildItem -LiteralPath "crates\ep_runtime\src\ideal_loads" -Recurse -File -Filter "*.rs" | Where-Object {
        $candidate = Read-RepoText -Path $_.FullName
        $candidate -match '#\[test\]' -and $candidate -match "(?:$cp362Stem|cp362)"
    })
$cp362SemanticTestsText = ($cp362SemanticTestFiles | ForEach-Object {
        Read-RepoText -Path $_.FullName
    }) -join "`n"
# Pinned source, exact boundary, routes, sites, fields, and source arithmetic.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp362Source).Hash -cne $cp362SourceHash) {
    throw "CP362 pinned PurchasedAirManager.cc hash drifted"
}
$cp362SourceLines = Get-Content -Encoding UTF8 -LiteralPath $cp362Source
if ($cp362SourceLines[2231].Trim() -cne 'PurchAir.SupplyHumRat = min(PurchAir.MixedAirHumRat, SupplyHumRatForDehum);' -or
    $cp362SourceLines[2232].Trim() -cne '} break;' -or
    $cp362SourceLines[2233].Trim() -cne 'case HumControl::ConstantSupplyHumidityRatio: {' -or
    $cp362SourceLines[2234].Trim() -cne 'PurchAir.SupplyHumRat = PurchAir.MinCoolSuppAirHumRat;') {
    throw "CP362 physical lines 2232 through 2235 drifted"
}
Assert-Contains -Path $cp362Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2232' -Description "CP362 source line"
Assert-Contains -Path $cp362Module -Pattern 'EnergyPlus 26\.1 PurchasedAirManager\.cc:2233' -Description "CP362 first excluded break"
Assert-ExactStringArray -Path $cp362Module -Name "PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER" -Expected $cp362Sites -Description "CP362 four-site source order"
Assert-Cp362TextContains -Text $cp362CalcText -Pattern '(?s)UnitOff,\s*NonCooling,\s*PositiveGuardFalseFallthrough,\s*DehumidificationControlNoneCaseCompletedSkip,\s*DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,\s*DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted,\s*DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip' -Description "seven retained routes"
foreach ($counter in @(
        "dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count",
        "source_site_execution_count",
        "mixed_air_humidity_ratio_for_minimum_read_count",
        "supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_assignment_count"
    )) {
    Assert-Cp362TextContains -Text $cp362CalcText -Pattern ('pub ' + $counter + ':\s*usize') -Description "counter '$counter'"
}
$cp362WitnessPairs = @(
    @(
        "witnessed_positive_guard_false_fallthrough_skip_count",
        "positive_guard_false_fallthrough_skip_count",
        "state\s*\.\s*positive_guard_false_fallthrough_skip_count"
    ),
    @(
        "witnessed_dehumidification_control_none_case_completed_skip_count",
        "dehumidification_control_none_case_completed_skip_count",
        "state\s*\.\s*dehumidification_control_none_case_completed_skip_count"
    ),
    @(
        "witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count",
        "state\s*\.\s*dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count"
    ),
    @(
        "witnessed_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count",
        "dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count",
        "h"
    ),
    @(
        "witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
        "dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count",
        "state\s*\.\s*dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count"
    )
)
foreach ($pair in $cp362WitnessPairs) {
    $witness = $pair[0]
    Assert-Cp362TextContains -Text $cp362CalcText -Pattern ('pub\(super\)\s+' + $witness + ':\s*usize') -Description "private witness counter '$witness'"
    Assert-Contains -Path $cp362RuntimeValidation -Pattern (
        'state\s*\.\s*' + $witness + '\s*!=\s*' + $pair[2]
    ) -Description "internal witness parity '$witness'"
}
foreach ($field in @(
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification",
        "mixed_air_humidity_ratio_for_minimum_read",
        "mixed_air_humidity_ratio",
        "supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read",
        "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit",
        "source_shaped_two_argument_minimum_evaluated",
        "minimum_supply_humidity_ratio",
        "supply_humidity_ratio_assignment_performed",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio"
    )) {
    Assert-Cp362TextContains -Text $cp362CalcText -Pattern ('pub ' + $field + ':') -Description "snapshot field '$field'"
}
Assert-Cp362TextContains -Text $cp362CalcText -Pattern 'source_shaped_two_argument_minimum\(mixed(?:_air_humidity_ratio)?,\s*(?:local|supply_humidity_ratio_for_dehumidification)\)' -Description "left-then-right minimum call"
Assert-Cp362TextNotContains -Text $cp362CalcText -Pattern 'source_shaped_two_argument_minimum\((?:local|supply_humidity_ratio_for_dehumidification),\s*mixed|f64::min|\.min\s*\(|total_cmp|partial_cmp|\.clamp\(' -Description "reversed/substitute minimum or local clamp"
Assert-Contains -Path $cp362SharedMinimum -Pattern '(?s)fn source_shaped_two_argument_minimum\(.*?left:\s*f64,.*?right:\s*f64,.*?\)\s*->\s*f64\s*\{\s*if left < right \{ left \} else \{ right \}\s*\}' -Description "shared CP334 strict right-biased minimum"
$cp362MinimumDefinitionCount = 0
foreach ($rust in Get-ChildItem -LiteralPath "crates\ep_runtime\src\ideal_loads\calc" -Recurse -File -Filter "*.rs") {
    $cp362MinimumDefinitionCount += [regex]::Matches(
        (Read-RepoText -Path $rust.FullName),
        '(?m)^\s*pub\(in crate::ideal_loads::calc\)\s+fn source_shaped_two_argument_minimum\s*\('
    ).Count
}
if ($cp362MinimumDefinitionCount -ne 1) {
    throw "CP362 must reuse the sole CP334 minimum helper; found $cp362MinimumDefinitionCount definitions"
}
# Exact CP361 predecessor, recursive CP329/CP361 ownership, checked 4H, and C0 skip.
Assert-Cp362TextContains -Text $cp362CalcText -Pattern 'PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Predecessor' -Description "exact CP361 predecessor"
foreach ($pattern in @(
        'completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent',
        'cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release',
        'cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness',
        'private_humidistat_counterfactual_from_direct_release as predecessor_private_counterfactual',
        'private_humidistat_counterfactual_links_to_direct_release as predecessor_private_links',
        'completed_direct_cooling_mixed_air_call_is_consistent',
        'cooling_mixed_air_call_snapshot_is_exact_direct_release',
        'cooling_mixed_air_call_snapshots_match_bit_exact',
        'cooling_mixed_air_call_latest_witness',
        'mixed_air\.mixed_air_humidity_ratio\?'
    )) {
    Assert-Cp362TextContains -Text $cp362CalcText -Pattern $pattern -Description "recursive owner '$pattern'"
}
Assert-Cp362TextContains -Text $cp362CalcText -Pattern '(?s)pub fn advance_direct_no_oa_calc_.*?\(\s*runtime:\s*&mut PurchasedAirRuntimeState,\s*system:\s*&IdealLoadsAirSystem,\s*(?:predecessor|predecessor_cp361):\s*Predecessor,\s*\)' -Description "public release arguments"
Assert-Cp362TextContains -Text $cp362CalcText -Pattern '(?s)advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_state\(.*?(?:predecessor|retained_predecessor),\s*None,\s*\)' -Description "direct C0 supplies no operands"
Assert-Cp362TextContains -Text $cp362CalcText -Pattern '(?s)\[\s*snapshot\.predecessor_resulting_supply_humidity_ratio_for_dehumidification,.*?snapshot\.mixed_air_humidity_ratio,.*?snapshot\.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,.*?snapshot\.minimum_supply_humidity_ratio,.*?snapshot\.assigned_supply_humidity_ratio,.*?snapshot\.resulting_supply_humidity_ratio,?\s*\].*?all\(\|value\| value\.is_none\(\)\)' -Description "six-field complete-null direct skip"
Assert-Cp362TextNotContains -Text $cp362CalcText -Pattern 'PurchasedAirSizedLimits|DirectZonePurchasedAirCouplingInput|prediction\.zone_demand|latest_numerical|numerical_supply|final_supply|system\.mixed_air_humidity_ratio|zone.*mixed_air_humidity_ratio' -Description "alternate owner or numerical DTO"
Assert-Contains -Path $cp362PipelineValidation -Pattern '(?s)checked_mul\(\s*PURCHASED_AIR_CALC_.*?_MIXED_AIR_LIMIT_SOURCE_ORDER\s*\.len\(\)' -Description "pipeline checked 4H"
Assert-Contains -Path $cp362PipelineValidation -Pattern '(?s)humidistat_mixed_air_limit_count.*?dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count' -Description "CP362 H inherits CP361 H"
foreach ($counter in @(
        "mixed_air_humidity_ratio_for_minimum_read_count",
        "supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count",
        "source_shaped_two_argument_minimum_evaluation_count",
        "supply_humidity_ratio_assignment_count"
    )) {
    Assert-Contains -Path $cp362PipelineValidation -Pattern ('(?s)"' + $counter + '".*?executed.*?state\.' + $counter) -Description "pipeline site count '$counter'=H"
}
# Semantic regressions cover boundary, right bias, overflow, provenance, and corruption.
foreach ($pattern in @(
        'seven_routes_preserve_exact_cp361_lineage_and_execute_only_h',
        'public_direct_u_n_p_c0_routes_are_complete_null',
        'source_minimum_is_right_biased_for_ties_and_unordered_values',
        'malformed_cp361_source_prefix_selector_and_numeric_lineage_are_transactional',
        'every_counter_overflow_rejects_without_partial_mutation',
        'owner_predecessor_and_witness_corruption_is_rejected_transactionally',
        'replay_is_rejected_transactionally_without_redistributing_counts',
        'private_h_bridge_reads_only_cp329_owner_and_cp361_local_bits',
        'cp329_owner_latest_and_witness_corruption_rejects_private_bridge'
    )) {
    Assert-Cp362TextContains -Text $cp362TestsText -Pattern $pattern -Description "semantic regression '$pattern'"
}
foreach ($pair in $cp362WitnessPairs) {
    Assert-Cp362TextContains -Text $cp362TestsText -Pattern (
        '(?s)reject_route_pair!\(\s*[^,]+,\s*' + $pair[1] + ',\s*' + $pair[0] + '\s*\)'
    ) -Description "transactional overflow regression for '$($pair[0])'"
}
# Final regressions must be compiled CP362 tests, regardless of their eventual split.
$cp362FinalTestNames = @("scheduled_binding_places_cp362_after_cp361_and_keeps_cp345_owner", "scheduled_binding_preserves_cp362_u_n_p_as_complete_null_operand_nonreads", "coupled_route_source_and_latest_corruption_fail_closed", "coordinated_owner_predecessor_and_witness_forgeries_reject_transactionally", "transition_preserves_right_biased_ieee_bits_without_numeric_gate", "public_wrapper_u_n_p_c0_routes_are_complete_null_and_do_not_read_operands")
foreach ($name in $cp362FinalTestNames) {
    Assert-Cp362TextContains -Text $cp362SemanticTestsText -Pattern ('(?m)fn\s+' + $name + '\s*\(') -Description "final semantic regression '$name'"
}
$cp362BindingTestFiles = @($cp362SemanticTestFiles | Where-Object {
        (Read-RepoText -Path $_.FullName) -match 'fn\s+scheduled_binding_places_cp362_after_cp361_and_keeps_cp345_owner\s*\('
    })
if ($cp362BindingTestFiles.Count -ne 1 -or $cp362BindingTestFiles[0].FullName -eq (Resolve-Path $cp362BindingTestsRoot).Path) {
    throw "CP362 requires one dedicated binding regression file"
}
Assert-Contains -Path $cp362BindingTestsRoot -Pattern ([regex]::Escape($cp362BindingTestFiles[0].Name)) -Description "registered dedicated CP362 binding tests"
$cp362CoupledRegressionFiles = @($cp362SemanticTestFiles | Where-Object {
        (Read-RepoText -Path $_.FullName) -match 'fn\s+coordinated_owner_predecessor_and_witness_forgeries_reject_transactionally\s*\('
    })
if ($cp362CoupledRegressionFiles.Count -ne 1) { throw "CP362 requires one coordinated forgery regression file" }
$cp362CoordinatedRegistration = 'mod\s+' + [regex]::Escape($cp362CoupledRegressionFiles[0].BaseName) + '\s*;|' + [regex]::Escape($cp362CoupledRegressionFiles[0].Name)
if ($cp362TestsText -notmatch $cp362CoordinatedRegistration -and (Read-RepoText -Path $cp362CoupledTestsRoot) -notmatch $cp362CoordinatedRegistration) { throw "CP362 coordinated forgery regression is not registered" }
$cp362BindingTestText = Read-RepoText -Path $cp362BindingTestFiles[0].FullName
foreach ($pattern in @("calculation_$cp361StemForCp362", "calculation_$cp362Stem", 'calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment', 'dehumidification_control_none_case_completed_skip', 'unit_off_skipped', 'non_cooling_skipped', 'positive_guard_false_fallthrough_skipped', 'supply_node_update\s*\.\s*humidity_ratio', 'to_bits')) {
    Assert-Cp362TextContains -Text $cp362BindingTestText -Pattern $pattern -Description "binding regression evidence '$pattern'"
}
$cp362CoupledTestFiles = @($cp362SemanticTestFiles | Where-Object { (Read-RepoText -Path $_.FullName) -match 'fn\s+coupled_route_source_and_latest_corruption_fail_closed\s*\(' })
if ($cp362CoupledTestFiles.Count -ne 1) { throw "CP362 requires one coupled validator regression file" }
Assert-Contains -Path $cp362CoupledTestsRoot -Pattern ([regex]::Escape($cp362CoupledTestFiles[0].Name)) -Description "registered CP362 coupled validator regressions"
$cp362CoupledTestBlock = Get-Cp362RustBraceBlock -Text (Read-RepoText -Path $cp362CoupledTestFiles[0].FullName) -AnchorPattern '(?m)fn\s+coupled_route_source_and_latest_corruption_fail_closed\s*\(' -Description "CP362 coupled validator tests"
Assert-Contains -Path $cp362Coupled -Pattern '(?s)let route_partition = checked_sum\(&\[.*?unit_off_skip_count.*?non_cooling_skip_count.*?positive_guard_false_fallthrough_skip_count.*?dehumidification_control_none_case_completed_skip_count.*?dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count.*?active.*?dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count' -Description "coupled seven-route partition"
foreach ($pattern in @('unit_off_skip_count', 'non_cooling_skip_count', 'dehumidification_control_none_case_completed_skip_count', 'dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count', 'dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count', 'dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count', 'source_site_execution_count', 'mixed_air_humidity_ratio_for_minimum_read_count', 'supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count', 'source_shaped_two_argument_minimum_evaluation_count', 'supply_humidity_ratio_assignment_count', 'latest', 'predecessor', 'usize::MAX|overflow')) {
    Assert-Cp362TextContains -Text $cp362CoupledTestBlock -Pattern $pattern -Description "coupled corruption/overflow evidence '$pattern'"
}
$cp362ForgeryTest = Get-Cp362RustBraceBlock -Text $cp362SemanticTestsText -AnchorPattern '(?m)fn\s+coordinated_owner_predecessor_and_witness_forgeries_reject_transactionally\s*\(' -Description "CP362 coordinated forgery regression"
foreach ($pattern in @('cooling_mixed_air_call_latest_witness', "${cp361StemForCp362}_latest_witness", "calc_$cp362Stem", 'latest', 'assert_eq!\s*\(.*?before')) {
    Assert-Cp362TextContains -Text $cp362ForgeryTest -Pattern $pattern -Description "coordinated forgery evidence '$pattern'"
}
$cp362TransitionTest = Get-Cp362RustBraceBlock -Text $cp362SemanticTestsText -AnchorPattern '(?m)fn\s+transition_preserves_right_biased_ieee_bits_without_numeric_gate\s*\(' -Description "CP362 transition IEEE regression"
foreach ($pattern in @('advance\s*\(', 'from_bits', 'to_bits', 'f64::INFINITY', 'f64::NEG_INFINITY')) {
    Assert-Cp362TextContains -Text $cp362TransitionTest -Pattern $pattern -Description "actual transition IEEE evidence '$pattern'"
}
Assert-Cp362TextNotContains -Text $cp362TransitionTest -Pattern 'source_shaped_two_argument_minimum\s*\(' -Description "helper-only IEEE regression"
foreach ($test in @(
        "missing_direct_lifecycle_fails_closed",
        "route_partition_overflow_fails_closed",
        "four_site_counters_are_exact_and_fail_closed_on_each_mismatch",
        "direct_expected_snapshot_is_complete_null_and_exact_bit_comparison_is_strict",
        "predecessor_numeric_bits_are_preserved_in_expected_snapshot"
    )) {
    Assert-Contains -Path $cp362PipelineTests -Pattern $test -Description "pipeline regression '$test'"
}
# Binding, coupled runtime, pipeline, JSON, and strict numerical nonfeed.
$cp362BindingText = Read-RepoText -Path $cp362Binding
$cp361BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_${cp361StemForCp362} =")
$cp362BindingIndex = $cp362BindingText.IndexOf("let calculation_${cp362Stem} =")
$cp363BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_humidistat_case_break =")
$cp364BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_entry =")
$cp365BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_assignment =")
$cp366BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_constant_supply_humidity_ratio_case_break =")
$cp367BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment =")
$cp368BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_default_supply_humidity_ratio_case_break =")
$cp369BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard =")
$cp370BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard =")
$cp371BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard =")
$cp372BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment =")
$cp373BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment =")
$cp374BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit =")
$cp375BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment ="); $cp376BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp362NumericalIndex = $cp362BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling("); $cp379BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp362 = $cp362BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
if ($cp361BindingIndexForCp362 -lt 0 -or $cp362BindingIndex -le $cp361BindingIndexForCp362 -or $cp363BindingIndexForCp362 -le $cp362BindingIndex -or $cp364BindingIndexForCp362 -le $cp363BindingIndexForCp362 -or $cp365BindingIndexForCp362 -le $cp364BindingIndexForCp362 -or $cp366BindingIndexForCp362 -le $cp365BindingIndexForCp362 -or $cp367BindingIndexForCp362 -le $cp366BindingIndexForCp362 -or $cp368BindingIndexForCp362 -le $cp367BindingIndexForCp362 -or
    $cp362NumericalIndex -le $cp368BindingIndexForCp362 -or
    $cp369BindingIndexForCp362 -le $cp368BindingIndexForCp362 -or
    $cp370BindingIndexForCp362 -le $cp369BindingIndexForCp362 -or
    $cp371BindingIndexForCp362 -le $cp370BindingIndexForCp362 -or
    $cp372BindingIndexForCp362 -le $cp371BindingIndexForCp362 -or
    $cp373BindingIndexForCp362 -le $cp372BindingIndexForCp362 -or
    $cp374BindingIndexForCp362 -le $cp373BindingIndexForCp362 -or $cp375BindingIndexForCp362 -le $cp374BindingIndexForCp362 -or
    $cp376BindingIndexForCp362 -le $cp375BindingIndexForCp362 -or $cp377BindingIndexForCp362 -le $cp376BindingIndexForCp362 -or $cp378BindingIndexForCp362 -le $cp377BindingIndexForCp362 -or $cp379BindingIndexForCp362 -le $cp378BindingIndexForCp362 -or $cp380BindingIndexForCp362 -le $cp379BindingIndexForCp362 -or $cp381BindingIndexForCp362 -le $cp380BindingIndexForCp362 -or $cp382BindingIndexForCp362 -le $cp381BindingIndexForCp362 -or $cp383BindingIndexForCp362 -le $cp382BindingIndexForCp362 -or $cp384BindingIndexForCp362 -le $cp383BindingIndexForCp362 -or $cp385BindingIndexForCp362 -le $cp384BindingIndexForCp362 -or $cp362NumericalIndex -le $cp385BindingIndexForCp362) {
    throw "Binding must execute CP361 through CP370 before numerical coupling"
}
$cp362Dto = Get-Cp362RustBraceBlock -Text $cp362BindingText.Substring($cp362NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "CP362 numerical DTO"
if ($cp362Dto -match '(?i)cp362|humidistat|mixed_air_limit|supply_humidity_ratio_for_dehumidification') {
    throw "CP362 evidence must not enter DirectZonePurchasedAirCouplingInput"
}
Assert-Contains -Path $cp362CalcRoot -Pattern $cp362Stem -Description "CP362 calc registration"
Assert-Contains -Path $cp362BindingAdapter -Pattern ('advance_direct_no_oa_calc_' + $cp362Stem) -Description "CP362 binding adapter"
Assert-Contains -Path $cp362ScheduledOutput -Pattern ('pub calculation_' + $cp362Stem + ':') -Description "CP362 scheduled output"
Assert-Contains -Path $cp362InitState -Pattern $cp362Stem -Description "CP362 runtime state"
Assert-Contains -Path $cp362InitWitnessRoot -Pattern $cp362Stem -Description "CP362 witness registration"
Assert-Contains -Path $cp362CoupledRoot -Pattern ('mod ' + $cp362Stem + '_validation;') -Description "CP362 coupled validator"
Assert-Contains -Path $cp362Coupled -Pattern ('calculation_' + $cp361StemForCp362) -Description "coupled CP361 predecessor"
Assert-NotContains -Path $cp362Coupled -Pattern 'DirectZonePurchasedAirCouplingInput|complete_direct_zone_purchased_air_coupling|latest_numerical|numerical_supply|final_supply' -Description "coupled numerical firewall"
Assert-Contains -Path $cp362FixtureRoot -Pattern $cp362Stem -Description "CP362 fixture registration"
Assert-Contains -Path $cp362Fixture -Pattern ('calculation_' + $cp362Stem + '_snapshot') -Description "CP362 output fixture"
Assert-Contains -Path $cp362PipelineRoot -Pattern ('mod ' + $cp362PipelineStem + ';') -Description "CP362 pipeline module"
Assert-Contains -Path $cp362PipelineRoot -Pattern ('"' + $cp362Lifecycle + '":\s*result\s*\.' + $cp362Lifecycle) -Description "CP362 lifecycle JSON"
Assert-Contains -Path $cp362PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp409_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp362ParentAssertions -Pattern 'mod cp362_assertions;' -Description "arbitrary CP362 delegation"
Assert-Contains -Path $cp362ParentAssertions -Pattern 'cp362_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP362 direct delegation"
Assert-Contains -Path $cp362ParentAssertions -Pattern 'cp362_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP362 non-direct delegation"
Assert-NotContains -Path $cp362ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(runtime, results\)' -Description "CP362 relinquishes terminal numerical nonfeed to CP363"
foreach ($field in @(
        "predecessor_resulting_supply_humidity_ratio_for_dehumidification",
        "mixed_air_humidity_ratio",
        "supply_humidity_ratio_for_dehumidification_before_mixed_air_limit",
        "minimum_supply_humidity_ratio",
        "assigned_supply_humidity_ratio",
        "resulting_supply_humidity_ratio"
    )) {
    Assert-Contains -Path $cp362SnapshotSerialization -Pattern ('"' + $field + '"') -Description "CP362 JSON '$field'"
    Assert-Contains -Path $cp362SnapshotSerialization -Pattern ('"' + $field + '_ieee_bits"') -Description "CP362 JSON bits '$field'"
}
Assert-Contains -Path $cp362SnapshotSerialization -Pattern '(?s)fn json_number.*?is_finite.*?Value::Null' -Description "CP362 nonfinite numeric projection"
Assert-Contains -Path $cp362SnapshotSerialization -Pattern 'format!\("0x\{:016x\}",\s*value\.to_bits\(\)\)' -Description "CP362 authoritative bits"
# Two algorithm/capability addenda, 2+2+1+1 targets, and five hand sections.
$cp362AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp362CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp362AlgorithmAddenda = [regex]::Matches($cp362AlgorithmText, '(?m)^\s*"CP362 supersedes only CP361[^"\r\n]+",\s*$')
$cp362CapabilityAddenda = [regex]::Matches($cp362CapabilityText, '(?m)^\s*"CP362 additionally requires[^"\r\n]+",\s*$')
if ($cp362AlgorithmAddenda.Count -ne 2 -or $cp362CapabilityAddenda.Count -ne 2) {
    throw "Specs must contain exactly two algorithm and two capability CP362 addenda"
}
foreach ($claim in @($cp362AlgorithmAddenda) + @($cp362CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp362SourceCommit, $cp362SourceHash, 'physical executable line 2232',
            'line 2233', 'break', 'CP363', 'line-2234', 'line-2235', 'line-2245',
            $cp362Sites[0], $cp362Sites[1], $cp362Sites[2], $cp362Sites[3],
            'T=U\+N\+P\+C0\+Q\+H\+CSH', 'S=C0\+Q\+H\+CSH=R=G\+F\+L',
            'A=F\+L', 'source_site_execution_count=4H', 'C0=S', 'Q=H=CSH=0',
            'CP329.*?mixed_air_humidity_ratio', 'CP361.*?resulting_supply_humidity_ratio_for_dehumidification',
            'finite no-OA', 'no finite/range gate', 'CP345', 'CP319', 'numerical DTO',
            'if left < right \{ left \} else \{ right \}', 'f64::min',
            'CP361-to-CP362-to-unchanged-numerical', $cp362Lifecycle,
            '32 algorithms and 293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', '300 total', '240 public', '60 internal', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) {
            throw "CP362 spec addendum missing '$pattern'"
        }
    }
}
foreach ($target in @(
        [PSCustomObject]@{ Pattern = "$cp362Stem/release\.rs::advance_direct_no_oa_calc_$cp362Stem"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp362Stem/lifecycle\.rs::purchased_air_calc_${cp362Stem}_lifecycle_summary"; Expected = 2 },
        [PSCustomObject]@{ Pattern = "$cp362Stem\.rs::${cp362TypeStem}RuntimeState"; Expected = 1 },
        [PSCustomObject]@{ Pattern = "$cp362Stem/lifecycle\.rs::${cp362TypeStem}LifecycleSummary"; Expected = 1 }
    )) {
    if ([regex]::Matches($cp362AlgorithmText, $target.Pattern).Count -ne $target.Expected) {
        throw "CP362 target count failed for '$($target.Pattern)'"
    }
}
$cp362Documentation = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Pattern = '(?ms)^## CP362 Cooling Humidistat Supply-Humidity-Ratio Mixed-Air Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Pattern = '(?ms)^## CP362 Source-Ordered Cooling Humidistat Mixed-Air Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Pattern = '(?ms)^## CP362 Humidistat Supply-Humidity-Ratio Mixed-Air Limit\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Pattern = '(?ms)^## CP362 Humidistat Mixed-Air Limit in the Heat-Balance Loop\r?\n.*?(?=^## |\z)' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Pattern = '(?ms)^## CP362 Humidistat Mixed-Air-Limit Placement\r?\n.*?(?=^## |\z)' }
)
foreach ($doc in $cp362Documentation) {
    $sections = [regex]::Matches((Read-RepoText -Path $doc.Path), $doc.Pattern)
    if ($sections.Count -ne 1) {
        throw "CP362 documentation expected one section in $($doc.Path)"
    }
    foreach ($pattern in @(
            $cp362SourceHash, '2232', '2233', 'break', 'CP363', '2234', '2235', '2245',
            $cp362Sites[0], $cp362Sites[1], $cp362Sites[2], $cp362Sites[3],
            'T\s*=\s*U\+N\+P\+C0\+Q\+H\+CSH', '4H', 'C0\s*=\s*S',
            'Q\s*=\s*H\s*=\s*CSH\s*=\s*0', 'CP329', 'mixed_air_humidity_ratio',
            'CP361', 'resulting_supply_humidity_ratio_for_dehumidification',
            'finite no-OA', 'adds?\s+no\s+finite', 'CP345', 'CP319', 'numerical DTO',
            'CP334', 'if left < right \{ left \} else \{ right \}', 'f64::min',
            'CP361-to-CP362-to-unchanged-numerical', $cp362Lifecycle,
            '32\s+algorithms', '293\s+routines', '300\s+total', '240\s+public',
            '60\s+internal', 'zero\s+unused', 'Roadmap'
        )) {
        if ($sections[0].Value -notmatch $pattern) {
            throw "CP362 documentation in $($doc.Path) missing '$pattern'"
        }
    }
}
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP362 supersedes only CP361' -Description "generated CP362 algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP362 additionally requires' -Description "generated CP362 capability addendum"
# Historical order/firewall/count audits, master order, and generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..361 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit' -Description "historical CP362 binding order"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..345 | ForEach-Object {
            (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name
        }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern 'advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit' -Description "historical CP362 helper whitelist"
}
foreach ($historical in 334..361) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp409_lifecycle_evidence' -Description "historical CP363 firewall"
}
foreach ($historical in 335..361) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 347 \|')) -Description "historical current generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 107 \|')) -Description "historical current generated internal"
}
foreach ($historical in 337..361) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 347' -Description "historical current script inventory total"
}
$cp362MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp361AuditIndexForCp362 = $cp362MainAuditText.IndexOf("cp361-cooling-humidistat-supply-humidity-ratio-for-dehumidification-minimum-limit.ps1")
$cp362AuditIndex = $cp362MainAuditText.IndexOf("cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1")
$cp362CompletionIndex = $cp362MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp361AuditIndexForCp362 -lt 0 -or $cp362AuditIndex -le $cp361AuditIndexForCp362 -or $cp362CompletionIndex -le $cp362AuditIndex) {
    throw "Master audit must dot-source CP362 after CP361 before completion"
}
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'script_count = 347' -Description "CP362 script total"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'unused_script_count = 0' -Description "CP362 zero uncalled"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp362-' -Description "CP362 inventory record"
Assert-Contains -Path "specs\script_inventory.toml" -Pattern 'cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit\.ps1::dot_sources' -Description "CP362 caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 347 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 107 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated uncalled"
Write-Host "CP362 Humidistat supply-humidity-ratio mixed-air-limit structure audit passed."
