# CP377 maps only PurchasedAirManager.cc physical executable line 2259's
# saturation supply-humidity-ratio assignment.
& {
$cp377Stem = "cooling_supply_humidity_ratio_saturation_assignment"
$cp376StemForCp377 = "cooling_supply_humidity_ratio_pre_saturation_original_assignment"
$cp377PipelineStem = "purchased_air_$cp377Stem"
$cp377Lifecycle = "purchased_air_calc_${cp377Stem}_lifecycle"
$cp377SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp377SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp377Sites = @(
    "read-purchased-air-supply-temperature-for-saturation-humidity-ratio",
    "read-environment-outdoor-barometric-pressure-for-saturation-humidity-ratio",
    "evaluate-psy-w-fn-tdb-rh-pb-at-unity-relative-humidity",
    "assign-local-saturation-supply-humidity-ratio"
)
$cp377Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp377Module = "crates\ep_runtime\src\ideal_loads\calc\$cp377Stem.rs"
$cp377Root = "crates\ep_runtime\src\ideal_loads\calc\$cp377Stem"
$cp377State = "$cp377Root\state.rs"
$cp377Transition = "$cp377Root\transition.rs"
$cp377Release = "$cp377Root\release.rs"
$cp377Prefix = "$cp377Root\release\prefix_validation.rs"
$cp377Runtime = "$cp377Root\release\runtime_validation.rs"
$cp377Snapshot = "$cp377Root\release\snapshot_validation.rs"
$cp377Private = "$cp377Root\release\private_characterization.rs"
$cp377CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp377Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp377BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp377Stem.rs"
$cp377BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp377Stem}_tests.rs"
$cp377BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp377ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp377InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp377InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp377InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp377InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp377Stem.rs"
$cp377CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp377Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp377Stem}_validation.rs"
$cp377CoupledLifecycle = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp377Stem}_validation\lifecycle.rs"
$cp377CoupledSnapshot = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp377Stem}_validation\snapshot.rs"
$cp377CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp377CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp377.rs"
$cp377FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp377Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp377Stem}_fixture.rs"
$cp377PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp377Pipeline = "crates\ep_run\src\pipeline\$cp377PipelineStem.rs"
$cp377PipelineValidation = "crates\ep_run\src\pipeline\$cp377PipelineStem\validation.rs"
$cp377PipelineCounts = "crates\ep_run\src\pipeline\$cp377PipelineStem\validation\counts.rs"
$cp377PipelineSnapshot = "crates\ep_run\src\pipeline\$cp377PipelineStem\validation\snapshot.rs"
$cp377PipelineValidationTests = "crates\ep_run\src\pipeline\$cp377PipelineStem\validation\tests.rs"
$cp377Serialization = "crates\ep_run\src\pipeline\$cp377PipelineStem\serialization.rs"
$cp377SnapshotSerialization = "crates\ep_run\src\pipeline\$cp377PipelineStem\serialization\snapshot.rs"
$cp377ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp376_assertions.rs"
$cp377ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp377_assertions.rs"
$cp377Audit = "scripts\quality\ideal-loads-structure-audit\cp377-cooling-supply-humidity-ratio-saturation-assignment.ps1"

function Assert-Cp377TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP377 $Description missing" }
}

function Assert-Cp377TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP377 $Description unexpectedly present" }
}

function Get-Cp377RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP377 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP377 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP377 $Description closing brace missing"
}

$cp377Required = @(
    $cp377Module, $cp377State, $cp377Transition, $cp377Release, $cp377Prefix,
    $cp377Runtime, $cp377Snapshot, $cp377Private, $cp377BindingAdapter,
    $cp377BindingTests, $cp377InitWitness, $cp377Coupled, $cp377CoupledLifecycle,
    $cp377CoupledSnapshot, $cp377CoupledTests, $cp377Fixture, $cp377Pipeline,
    $cp377PipelineValidation, $cp377PipelineCounts, $cp377PipelineSnapshot,
    $cp377PipelineValidationTests, $cp377Serialization,
    $cp377SnapshotSerialization, $cp377ArbitraryAssertions, $cp377Audit
)
foreach ($file in $cp377Required) {
    Assert-FileExists -Path $file -Description "CP377 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP377 bounded file"
}
$cp377CoreFiles = @(Get-ChildItem -LiteralPath $cp377Root -Recurse -File -Filter "*.rs")
$cp377CoreText = ($cp377CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP377 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join "`n"
$cp377CoreTests = @(Get-ChildItem -LiteralPath "$cp377Root\tests" -File -Filter "*.rs")
if ($cp377CoreTests.Count -lt 4) { throw "CP377 requires bounded route/release/IEEE/overflow tests" }
$cp377TestText = ($cp377CoreTests | ForEach-Object { Read-RepoText -Path $_.FullName }) -join "`n"
foreach ($pattern in @('direct', 'private', 'overflow', 'to_bits', '-0\.0', 'INFINITY', 'NAN', 'Cp334', 'Cp344', 'barometric')) {
    Assert-Cp377TextContains -Text $cp377TestText -Pattern $pattern -Description "core test matrix '$pattern'"
}

# Locked single statement and CP378 clamp boundary.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp377Source).Hash -cne $cp377SourceHash) {
    throw "CP377 PurchasedAirManager.cc SHA-256 drift"
}
$cp377Lines = Get-Content -Encoding UTF8 -LiteralPath $cp377Source
if ($cp377Lines[2258].Trim() -cne 'SupplyHumRatSat = PsyWFnTdbRhPb(state, PurchAir.SupplyTemp, 1.0, state.dataEnvrn->OutBaroPress, RoutineName);' -or
    $cp377Lines[2259].Trim() -cne 'PurchAir.SupplyHumRat = min(SupplyHumRatOrig, SupplyHumRatSat);') {
    throw "CP377 line 2259/2260 source boundary drift"
}
Assert-Contains -Path $cp377Module -Pattern 'PurchasedAirManager\.cc:2259' -Description "mapped source"
Assert-Contains -Path $cp377Module -Pattern 'PurchasedAirManager\.cc:2260' -Description "first excluded source"
Assert-ExactStringArray -Path $cp377Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER' -Expected $cp377Sites -Description "four-site source order"

# Eight CP376 routes; U/N/P skip and the five active routes execute four sites.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthrough', 'HumidificationControlGuardFalseFallthrough',
        'DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp377State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'source_site_execution_count',
        'purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count',
        'environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count',
        'psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count',
        'local_saturation_supply_humidity_ratio_assignment_count',
        'cp334_supply_temperature_mixed_air_limit_owner_count',
        'cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count',
        'environment_outdoor_barometric_pressure_owner_count'
    )) {
    Assert-Contains -Path $cp377State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'cp334_supply_temperature_mixed_air_limit_owned_read',
        'cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read',
        'environment_outdoor_barometric_pressure_owned_read',
        'supply_temperature_for_saturation_humidity_ratio_c',
        'outdoor_barometric_pressure_pa', 'saturation_supply_humidity_ratio',
        'assigned_saturation_supply_humidity_ratio', 'resulting_saturation_supply_humidity_ratio'
    )) {
    Assert-Contains -Path $cp377Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
Assert-Contains -Path $cp377Transition -Pattern 'SOURCE_ORDER\s*\.len\(\)' -Description "four-site increment"
Assert-Contains -Path $cp377Transition -Pattern 'energyplus_psy_w_fn_tdb_rh_pb\(' -Description "canonical psychrometric evaluation"
Assert-Contains -Path $cp377Transition -Pattern 'input\.supply_temperature_c,\s*1\.0,\s*input\.outdoor_barometric_pressure_pa' -Description "temperature/RH/pressure operand order"
Assert-NotContains -Path $cp377Transition -Pattern 'energyplus_psychrometric_humidity_ratio_from_rh|\.clamp\(|f64::min|f64::max' -Description "guarded wrapper or line-2260 clamp"

# CP376 predecessor, route-specific temperature owner, and exact pressure owner.
foreach ($pattern in @(
        'direct_predecessor_is_retained_and_complete', 'cp376_snapshots_match_bit_exact',
        'completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent',
        'direct_temperature_owner', 'Cp334MixedAirLimit', 'Cp344CapacityMixedAirLimit',
        'capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed'
    )) {
    Assert-Contains -Path $cp377Prefix -Pattern $pattern -Description "predecessor/temperature-owner proof $pattern"
}
$cp377ReleaseText = Read-RepoText -Path $cp377Release
$cp377PublicRelease = Get-Cp377RustBraceBlock -Text $cp377ReleaseText -AnchorPattern "(?m)^pub fn advance_direct_no_oa_calc_${cp377Stem}\s*\(" -Description "public direct release"
foreach ($pattern in @('barometric_pressure_pa', 'supply_temperature_c\.is_finite\(\)', 'barometric_pressure_pa\.is_finite\(\)', 'barometric_pressure_pa <= 0\.0', 'saturation_humidity_ratio\.is_finite\(\)', 'energyplus_psy_w_fn_tdb_rh_pb\(')) {
    Assert-Cp377TextContains -Text $cp377PublicRelease -Pattern $pattern -Description "public finite admission $pattern"
}
Assert-Cp377TextNotContains -Text $cp377PublicRelease -Pattern 'energyplus_psychrometric_humidity_ratio_from_rh|SupplyHumRatOrig|\.min\(|f64::min|DirectZonePurchasedAirCouplingInput' -Description "wrapper/clamp/numerical feed"
foreach ($pattern in @('partition == state\.transition_count', 'source_site_execution_count == source_sites', 'temperature_owner_total == active', 'environment_outdoor_barometric_pressure_owner_count == active')) {
    Assert-Contains -Path $cp377Runtime -Pattern $pattern -Description "checked route/owner algebra $pattern"
}
foreach ($pattern in @('transition_partition', 'temperature_owner_partition', 'source_site_execution_count', 'outdoor_barometric_pressure_owner_count', 'checked_sum')) {
    Assert-Contains -Path $cp377CoupledLifecycle -Pattern $pattern -Description "coupled route/owner validation $pattern"
    Assert-Contains -Path $cp377PipelineCounts -Pattern $pattern -Description "pipeline route/owner validation $pattern"
}

# Registration, CP376 -> CP377 -> unchanged numerical order, and no feed.
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp377CalcRoot; Pattern = $cp377Stem },
        [PSCustomObject]@{ Path = $cp377ScheduledOutput; Pattern = "pub calculation_${cp377Stem}:" },
        [PSCustomObject]@{ Path = $cp377BindingTestsRoot; Pattern = $cp377Stem },
        [PSCustomObject]@{ Path = $cp377InitState; Pattern = $cp377Stem },
        [PSCustomObject]@{ Path = $cp377InitUnit; Pattern = $cp377Stem },
        [PSCustomObject]@{ Path = $cp377InitWitnessRoot; Pattern = $cp377Stem },
        [PSCustomObject]@{ Path = $cp377CoupledRoot; Pattern = "mod ${cp377Stem}_validation;" },
        [PSCustomObject]@{ Path = $cp377CoupledTestsRoot; Pattern = 'coupled_runtime_tests_cp377' },
        [PSCustomObject]@{ Path = $cp377FixtureRoot; Pattern = $cp377Stem },
        [PSCustomObject]@{ Path = $cp377PipelineRoot; Pattern = $cp377PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}
$cp377BindingText = Read-RepoText -Path $cp377Binding
$cp376BindingIndexForCp377 = $cp377BindingText.IndexOf("let calculation_${cp376StemForCp377} =")
$cp377BindingIndex = $cp377BindingText.IndexOf("let calculation_${cp377Stem} ="); $cp378BindingIndex = $cp377BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndex = $cp377BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndex = $cp377BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndex = $cp377BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndex = $cp377BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndex = $cp377BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp377 = $cp377BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp377 = $cp377BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp377NumericalIndex = $cp377BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp376BindingIndexForCp377 -lt 0 -or $cp377BindingIndex -le $cp376BindingIndexForCp377 -or
    $cp378BindingIndex -le $cp377BindingIndex -or $cp379BindingIndex -le $cp378BindingIndex -or $cp380BindingIndex -le $cp379BindingIndex -or $cp381BindingIndex -le $cp380BindingIndex -or $cp382BindingIndex -le $cp381BindingIndex -or $cp383BindingIndex -le $cp382BindingIndex -or $cp384BindingIndexForCp377 -le $cp383BindingIndex -or $cp385BindingIndexForCp377 -le $cp384BindingIndexForCp377 -or $cp377NumericalIndex -le $cp385BindingIndexForCp377) {
    throw "Binding must execute CP376 then CP377 before unchanged numerical coupling"
}
$cp377Dto = Get-Cp377RustBraceBlock -Text $cp377BindingText.Substring($cp377NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp377TextNotContains -Text $cp377Dto -Pattern 'cp377|saturation_assignment|saturation_supply_humidity_ratio' -Description "numerical DTO feed"
Assert-Contains -Path $cp377Binding -Pattern 'input\.barometric_pressure_pa' -Description "current-timestep scheduled-coupling pressure read"
Assert-Contains -Path $cp377BindingTests -Pattern 'orders_cp376_then_cp377_and_keeps_the_numerical_owner_unchanged' -Description "binding nonfeed regression"
Assert-Contains -Path $cp377CoupledTests -Pattern 'does_not_feed_numerical_result' -Description "coupled nonfeed regression"

# Direct-only ep_run lifecycle, finite JSON, and terminal arbitrary ownership.
Assert-Contains -Path $cp377PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp441_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp377PipelineRoot -Pattern $cp377Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @('json_number', 'ieee_bits', 'supply_temperature_for_saturation_humidity_ratio_c_ieee_bits', 'outdoor_barometric_pressure_pa_ieee_bits', 'saturation_supply_humidity_ratio_ieee_bits')) {
    Assert-Contains -Path $cp377SnapshotSerialization -Pattern $pattern -Description "finite JSON/IEEE sidecar $pattern"
}
Assert-Contains -Path $cp377ParentAssertions -Pattern 'mod cp377_assertions;' -Description "arbitrary CP377 module"
Assert-Contains -Path $cp377ParentAssertions -Pattern 'cp377_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp377ParentAssertions -Pattern 'cp377_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp377ParentAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP376 terminal nonfeed relinquishment"
Assert-Contains -Path $cp377ArbitraryAssertions -Pattern 'mod cp378_assertions;' -Description "arbitrary CP378 module"; Assert-Contains -Path $cp377ArbitraryAssertions -Pattern 'cp378_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP378 direct delegation"; Assert-Contains -Path $cp377ArbitraryAssertions -Pattern 'cp378_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP378 non-direct delegation"; Assert-NotContains -Path $cp377ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP377 terminal numerical evidence relinquishment"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp378_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_exact_reconciliation\(' -Description "CP378 terminal reconciliation"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp379_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP379 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP380 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed firewall"

# Exactly two spec addenda and five ordered handwritten sections.
$cp377AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp377CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp377AlgorithmAddenda = [regex]::Matches($cp377AlgorithmText, '(?m)^\s*"CP377 supersedes only CP376[^"\r\n]+",\s*$')
$cp377CapabilityAddenda = [regex]::Matches($cp377CapabilityText, '(?m)^\s*"CP377 additionally requires[^"\r\n]+",\s*$')
if ($cp377AlgorithmAddenda.Count -ne 2 -or $cp377CapabilityAddenda.Count -ne 2) {
    throw "CP377 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp377AlgorithmAddenda + $cp377CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp377SourceCommit, $cp377SourceHash, '2259', '2260',
            $cp377Sites[0], $cp377Sites[1], $cp377Sites[2], $cp377Sites[3],
            'eight', 'UnitOff', 'five', '4\*S', 'CP376', 'sole immediate route predecessor',
            'CP334', 'CP344', 'scheduled-coupling', 'Site-density', 'WeatherManager',
            'energyplus_psy_w_fn_tdb_rh_pb', 'finite strictly-positive pressure',
            'pure/private', 'guarded', 'cache', 'diagnostics', 'CP378',
            'CP376-to-CP377-to-unchanged-numerical', $cp377Lifecycle,
            '32 algorithms', '293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', '315 total', '240 public', '75 internal',
            'zero unused', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP377 spec addendum missing '$pattern'" }
    }
}
$cp377Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP377 Cooling Supply-Humidity-Ratio Saturation Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP377 Source-Ordered Cooling Supply-Humidity-Ratio Saturation Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP377 Supply-Humidity-Ratio Saturation Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP377 Saturation Humidity Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP377 Saturation Humidity-Assignment Placement' }
)
foreach ($doc in $cp377Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP377 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..377) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP377 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @($cp377SourceCommit, $cp377SourceHash, '2259', '2260', $cp377Sites[0], $cp377Sites[1], $cp377Sites[2], $cp377Sites[3], 'eight|8', 'CP376', 'CP334', 'CP344', '315\s+total', '75\s+internal')) {
        if ($sections[0].Value -notmatch $required) { throw "CP377 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP377\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP377 supersedes only CP376' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP377 additionally requires' -Description "generated capability addendum"

# Historical terminal expectations, line caps, master order, and generated inventory.
foreach ($historical in 334..376) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp441_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..376) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 379 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 139 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..376) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 379' -Description "historical inventory total"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp377Stem" -Description "historical CP377 binding order"
}
foreach ($historical in 360..376) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'cp377BindingIndex' -Description "historical CP377 binding index"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp377Stem" -Description "historical CP377 helper whitelist"
}
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP376-to-CP377' -Description "CP345 predecessor interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP388-to-CP389' -Description "CP345 CP389 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP386-to-CP387' -Description "CP345 CP387 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP379-to-CP380' -Description "CP345 CP380 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP385-to-CP386' -Description "CP345 CP386 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP387-to-CP388' -Description "CP345 CP388 predecessor interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP389-to-CP390' -Description "CP345 CP390 predecessor interval"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Limit 1201 -Description "CP345 historical audit"
Assert-LineLimit -Path "scripts\quality\ideal-loads-structure-audit\cp362-cooling-humidistat-supply-humidity-ratio-mixed-air-limit.ps1" -Limit 500 -Description "CP362 historical audit"

$cp377MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp376AuditIndexForCp377 = $cp377MainAuditText.IndexOf("cp376-cooling-supply-humidity-ratio-pre-saturation-original-assignment.ps1")
$cp377AuditIndex = $cp377MainAuditText.IndexOf("cp377-cooling-supply-humidity-ratio-saturation-assignment.ps1")
$cp377CompletionIndex = $cp377MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp376AuditIndexForCp377 -lt 0 -or $cp377AuditIndex -le $cp376AuditIndexForCp377 -or $cp377CompletionIndex -le $cp377AuditIndex) {
    throw "Master audit must dot-source CP377 after CP376 before completion"
}
$cp377InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp377TextContains -Text $cp377InventoryText -Pattern 'script_count = 379' -Description "script total"
Assert-Cp377TextContains -Text $cp377InventoryText -Pattern 'dev_command_count = 238' -Description "development-command total"
Assert-Cp377TextContains -Text $cp377InventoryText -Pattern 'unused_script_count = 0' -Description "zero unused"
if ([regex]::Matches($cp377InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp377InventoryText, '(?m)^classification = "internal"$').Count -ne 139) {
throw "CP377 inventory must be exactly 240 public and 136 internal scripts"
}
Assert-Cp377TextContains -Text $cp377InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp377-' -Description "inventory record"
Assert-Cp377TextContains -Text $cp377InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 379 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 139 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated unused"

Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed firewall"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP390-to-CP391' -Description "CP345 CP390-to-CP391 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP391-to-CP392' -Description "CP345 CP391-to-CP392 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP396-to-CP397' -Description "CP345 CP396-to-CP397 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP397-to-CP398' -Description "CP345 CP397-to-CP398 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP400-to-CP401' -Description "CP345 CP400-to-CP401 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP401-to-CP402' -Description "CP345 CP401-to-CP402 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP402-to-CP403' -Description "CP345 CP402 terminal interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP403-to-CP404' -Description "CP345 CP402 terminal interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP404-to-CP405' -Description "CP345 CP404-to-CP405 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP405-to-CP406' -Description "CP345 CP405-to-CP406 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP406-to-CP407' -Description "CP345 CP406-to-CP407 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP407-to-CP408' -Description "CP345 CP407-to-CP408 interval"; Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP408-to-CP409' -Description "CP345 CP408-to-CP409 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP409-to-CP410' -Description "CP345 CP409-to-CP410 interval"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1" -Pattern 'CP410-to-CP411' -Description "CP345 CP410-to-CP411 interval"
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP411-to-CP412' -Description 'CP345 CP411-to-CP412 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP412-to-CP413' -Description 'CP345 CP412-to-CP413 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP413-to-CP414' -Description 'CP345 CP413-to-CP414 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit\s*=' -Description 'CP415 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp415Call' -Description 'CP415 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP414-to-CP415' -Description 'CP414-to-CP415 interval'
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'let\s+calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment\s*=' -Description 'CP416 historical binding order'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp416Call' -Description 'CP416 terminal capture'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP415-to-CP416' -Description 'CP415-to-CP416 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp417Call' -Description 'CP417 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP416-to-CP417' -Description 'CP416-to-CP417 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp418Call' -Description 'CP418 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP417-to-CP418' -Description 'CP417-to-CP418 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp419Call' -Description 'CP419 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP418-to-CP419' -Description 'CP418-to-CP419 interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp420Call' -Description 'CP420 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP419-to-CP420' -Description 'CP419-to-CP420 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp421Call' -Description 'CP421 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP420-to-CP421' -Description 'CP420-to-CP421 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp422Call' -Description 'CP422 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP421-to-CP422' -Description 'CP421-to-CP422 interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp423Call' -Description 'CP423 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP422-to-CP423' -Description 'CP422-to-CP423 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP423-to-CP424' -Description 'CP423-to-CP424 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp424Call' -Description 'CP424 terminal capture'; foreach ($currentTerminalPattern in @('\$cp425Call', 'CP424-to-CP425', 'CP425-to-CP426')) { Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern $currentTerminalPattern -Description 'current CP425 terminal chain' }
Write-Host "CP377 saturation humidity-ratio assignment structure audit passed."
}

Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp425Call' -Description 'CP425 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP424-to-CP425' -Description 'CP424-to-CP425 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP425-to-CP426' -Description 'CP425-to-CP426 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp426Call' -Description 'CP426 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp427Call' -Description 'CP427 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP426-to-CP427' -Description 'CP426-to-CP427 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp428Call' -Description 'CP428 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP427-to-CP428' -Description 'CP427-to-CP428 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp429Call' -Description 'CP429 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP428-to-CP429' -Description 'CP428-to-CP429 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp430Call' -Description 'CP430 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP429-to-CP430' -Description 'CP429-to-CP430 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp431Call' -Description 'CP431 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP430-to-CP431' -Description 'CP430-to-CP431 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP430 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp432Call' -Description 'CP432 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP431-to-CP432' -Description 'CP431-to-CP432 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP431 numerical interval'
Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp433Call' -Description 'CP433 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP432-to-CP433' -Description 'CP432-to-CP433 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP43[2]-to-numerical' -Description 'stale CP432 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp434Call' -Description 'CP434 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP433-to-CP434' -Description 'CP433-to-CP434 terminal interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP441-to-numerical' -Description 'CP441-to-numerical terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical') -Description 'stale CP433 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp437Call' -Description 'CP437 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-CP437' -Description 'CP436-to-CP437 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP436-to-' + 'numerical') -Description 'stale CP436 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp438Call' -Description 'CP438 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP437-to-CP438' -Description 'CP437-to-CP438 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP437-to-' + 'numerical') -Description 'stale CP437 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp439Call' -Description 'CP439 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP438-to-CP439' -Description 'CP438-to-CP439 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP438-to-' + 'numerical') -Description 'stale CP438 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp440Call' -Description 'CP440 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP439-to-CP440' -Description 'CP439-to-CP440 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP439-to-' + 'numerical') -Description 'stale CP439 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp441Call' -Description 'CP441 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP440-to-CP441' -Description 'CP440-to-CP441 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP440-to-' + 'numerical') -Description 'stale CP440 numerical interval'
