# CP376 maps only PurchasedAirManager.cc physical executable line 2258's
# bit-exact pre-saturation original humidity-ratio assignment.
& {
$cp376Stem = "cooling_supply_humidity_ratio_pre_saturation_original_assignment"
$cp375StemForCp376 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment"
$cp377StemForCp376 = "cooling_supply_humidity_ratio_saturation_assignment"
$cp376PipelineStem = "purchased_air_$cp376Stem"
$cp376Lifecycle = "purchased_air_calc_${cp376Stem}_lifecycle"
$cp376SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp376SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp376Sites = @(
    "read-purchased-air-supply-humidity-ratio-before-saturation-limit",
    "assign-local-original-supply-humidity-ratio-before-saturation-limit"
)
$cp376Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp376Module = "crates\ep_runtime\src\ideal_loads\calc\$cp376Stem.rs"
$cp376Root = "crates\ep_runtime\src\ideal_loads\calc\$cp376Stem"
$cp376State = "$cp376Root\state.rs"
$cp376Transition = "$cp376Root\transition.rs"
$cp376Release = "$cp376Root\release.rs"
$cp376Prefix = "$cp376Root\release\prefix_validation.rs"
$cp376Runtime = "$cp376Root\release\runtime_validation.rs"
$cp376Snapshot = "$cp376Root\release\snapshot_validation.rs"
$cp376Private = "$cp376Root\release\private_characterization.rs"
$cp376CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp376Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp376BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp376Stem.rs"
$cp376BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp376Stem}_tests.rs"
$cp376BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp376ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp376InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp376InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp376InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp376InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp376Stem.rs"
$cp376CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp376Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp376Stem}_validation.rs"
$cp376CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp376CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp376.rs"
$cp376FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp376Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp376Stem}_fixture.rs"
$cp376PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp376Pipeline = "crates\ep_run\src\pipeline\$cp376PipelineStem.rs"
$cp376PipelineValidation = "crates\ep_run\src\pipeline\$cp376PipelineStem\validation.rs"
$cp376PipelineValidationTests = "crates\ep_run\src\pipeline\$cp376PipelineStem\validation\tests.rs"
$cp376Serialization = "crates\ep_run\src\pipeline\$cp376PipelineStem\serialization.rs"
$cp376SnapshotSerialization = "crates\ep_run\src\pipeline\$cp376PipelineStem\serialization\snapshot.rs"
$cp376ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp375_assertions.rs"
$cp376ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp376_assertions.rs"
$cp377ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp377_assertions.rs"
$cp376Audit = "scripts\quality\ideal-loads-structure-audit\cp376-cooling-supply-humidity-ratio-pre-saturation-original-assignment.ps1"

function Assert-Cp376TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP376 $Description missing" }
}

function Assert-Cp376TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP376 $Description unexpectedly present" }
}

function Get-Cp376RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP376 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP376 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP376 $Description closing brace missing"
}

$cp376Required = @(
    $cp376Module, $cp376State, $cp376Transition, $cp376Release, $cp376Prefix,
    $cp376Runtime, $cp376Snapshot, $cp376Private, $cp376BindingAdapter,
    $cp376BindingTests, $cp376InitWitness, $cp376Coupled, $cp376CoupledTests,
    $cp376Fixture, $cp376Pipeline, $cp376PipelineValidation,
    $cp376PipelineValidationTests, $cp376Serialization,
    $cp376SnapshotSerialization, $cp376ArbitraryAssertions, $cp377ArbitraryAssertions, $cp376Audit
)
foreach ($file in $cp376Required) {
    Assert-FileExists -Path $file -Description "CP376 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP376 bounded file"
}
$cp376CoreFiles = @(Get-ChildItem -LiteralPath $cp376Root -Recurse -File -Filter "*.rs")
$cp376CoreText = ($cp376CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP376 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join "`n"
$cp376CoreTests = @(Get-ChildItem -LiteralPath "$cp376Root\tests" -File -Filter "*.rs")
if ($cp376CoreTests.Count -lt 4) { throw "CP376 requires bounded route/release/IEEE/overflow tests" }
$cp376TestText = ($cp376CoreTests | ForEach-Object { Read-RepoText -Path $_.FullName }) -join "`n"
foreach ($pattern in @('direct', 'private', 'overflow', 'to_bits', 'signed_zero', 'infinity', 'NaN', 'Cp347', 'Cp356', 'Cp362', 'Cp365')) {
    Assert-Cp376TextContains -Text $cp376TestText -Pattern $pattern -Description "core test matrix '$pattern'"
}

# Locked single statement and first excluded psychrometric statement.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp376Source).Hash -cne $cp376SourceHash) {
    throw "CP376 PurchasedAirManager.cc SHA-256 drift"
}
$cp376Lines = Get-Content -Encoding UTF8 -LiteralPath $cp376Source
if ($cp376Lines[2257].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;' -or
    $cp376Lines[2258].Trim() -cne 'SupplyHumRatSat = PsyWFnTdbRhPb(state, PurchAir.SupplyTemp, 1.0, state.dataEnvrn->OutBaroPress, RoutineName);') {
    throw "CP376 line 2258/2259 source boundary drift"
}
Assert-Contains -Path $cp376Module -Pattern 'PurchasedAirManager\.cc:2258' -Description "mapped source"
Assert-Contains -Path $cp376Module -Pattern 'PurchasedAirManager\.cc:2259' -Description "first excluded source"
Assert-ExactStringArray -Path $cp376Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER' -Expected $cp376Sites -Description "two-site source order"

# Eight inherited routes; U/N/P skip and the other five perform the two-site copy.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthrough', 'HumidificationControlGuardFalseFallthrough',
        'DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp376State -Pattern $route -Description "retained route $route"
}
$cp376TransitionText = Read-RepoText -Path $cp376Transition
$cp376ActiveBlock = Get-Cp376RustBraceBlock -Text $cp376TransitionText -AnchorPattern '(?m)^pub\(in crate::ideal_loads::calc\) fn route_is_active\s*\(' -Description "active-route selector"
foreach ($route in @('UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough')) {
    Assert-Cp376TextContains -Text $cp376ActiveBlock -Pattern $route -Description "inactive route $route"
}
foreach ($counter in @(
        'source_site_execution_count',
        'purchased_air_supply_humidity_ratio_before_saturation_limit_read_count',
        'local_original_supply_humidity_ratio_before_saturation_limit_assignment_count',
        'cp375_maximum_assignment_owner_count', 'cp347_none_case_owner_count',
        'cp356_constant_shr_owner_count', 'cp362_humidistat_owner_count',
        'cp365_constant_supply_humidity_ratio_owner_count'
    )) {
    Assert-Contains -Path $cp376State -Pattern "pub $counter\s*:\s*usize" -Description "state counter $counter"
}
foreach ($field in @(
        'cp375_maximum_assignment_owned_read', 'cp347_none_case_owned_read',
        'cp356_constant_shr_owned_read', 'cp362_humidistat_owned_read',
        'cp365_constant_supply_humidity_ratio_owned_read',
        'purchased_air_supply_humidity_ratio_before_saturation_check',
        'assigned_supply_humidity_ratio_original', 'resulting_supply_humidity_ratio_original'
    )) {
    Assert-Contains -Path $cp376Module -Pattern "pub $field\s*:" -Description "snapshot field $field"
}
Assert-Contains -Path $cp376Transition -Pattern 'SOURCE_ORDER\.len\(\)' -Description "two-site increment"
foreach ($field in @('purchased_air_supply_humidity_ratio_before_saturation_check', 'assigned_supply_humidity_ratio_original', 'resulting_supply_humidity_ratio_original')) {
    Assert-Contains -Path $cp376Transition -Pattern "$field\s*:\s*value" -Description "raw-copy field $field"
}
Assert-NotContains -Path $cp376Transition -Pattern 'PsyWFn|SupplyHumRatSat|OutBaroPress|relative_humidity|\.clamp\(|\.max\(|\.min\(|f64::max|f64::min|\.is_finite\(|DirectZonePurchasedAirCouplingInput' -Description "psychrometric/saturation/arithmetic/service behavior"

# CP375 is the route predecessor; CP347 is the direct RHS last writer.
foreach ($pattern in @(
        'direct_predecessor_is_retained_and_complete', 'cp375_snapshots_match_bit_exact',
        'cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release',
        'completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent'
    )) {
    Assert-Contains -Path $cp376Prefix -Pattern $pattern -Description "CP375 predecessor proof $pattern"
}
foreach ($pattern in @(
        'direct_cp347_owner', 'cp347_snapshots_match_bit_exact',
        'cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_snapshot_is_exact_direct_release',
        'completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent',
        'owner\.resulting_supply_humidity_ratio'
    )) {
    Assert-Contains -Path $cp376Prefix -Pattern $pattern -Description "CP347 owner proof $pattern"
}
$cp376ReleaseText = Read-RepoText -Path $cp376Release
$cp376PublicRelease = Get-Cp376RustBraceBlock -Text $cp376ReleaseText -AnchorPattern "(?m)^pub fn advance_direct_no_oa_calc_${cp376Stem}\s*\(" -Description "public direct release"
foreach ($pattern in @('predecessor_cp375', 'direct_cp347_owner', 'Owner::Cp347NoneCase')) {
    Assert-Cp376TextContains -Text $cp376PublicRelease -Pattern $pattern -Description "public direct proof $pattern"
}
Assert-Cp376TextNotContains -Text $cp376PublicRelease -Pattern 'Cp375MaximumAssignment|Cp356ConstantShr|Cp362Humidistat|Cp365ConstantSupplyHumidityRatio|DirectZonePurchasedAirCouplingInput|PsyWFn' -Description "public alternate owner/service/psychrometric read"
foreach ($pattern in @('partition == state\.transition_count', 'owner_total == active', 'source_site_execution_count == source_sites', 'cp375_owned == Some\(state\.cp375_maximum_assignment_owner_count\)')) {
    Assert-Contains -Path $cp376Runtime -Pattern $pattern -Description "checked route/owner algebra $pattern"
}
foreach ($pattern in @('validate_route_partition', 'validate_source_and_owner_counters', 'owner_read_partition', 'checked_sum')) {
    Assert-Contains -Path $cp376Coupled -Pattern $pattern -Description "coupled route/owner validation $pattern"
}

# Registrations, CP375 -> CP376 -> unchanged numerical order, and no numerical feed.
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp376CalcRoot; Pattern = $cp376Stem },
        [PSCustomObject]@{ Path = $cp376ScheduledOutput; Pattern = "pub calculation_${cp376Stem}:" },
        [PSCustomObject]@{ Path = $cp376BindingTestsRoot; Pattern = $cp376Stem },
        [PSCustomObject]@{ Path = $cp376InitState; Pattern = $cp376Stem },
        [PSCustomObject]@{ Path = $cp376InitUnit; Pattern = $cp376Stem },
        [PSCustomObject]@{ Path = $cp376InitWitnessRoot; Pattern = $cp376Stem },
        [PSCustomObject]@{ Path = $cp376CoupledRoot; Pattern = "mod ${cp376Stem}_validation;" },
        [PSCustomObject]@{ Path = $cp376CoupledTestsRoot; Pattern = 'coupled_runtime_tests_cp376' },
        [PSCustomObject]@{ Path = $cp376FixtureRoot; Pattern = $cp376Stem },
        [PSCustomObject]@{ Path = $cp376PipelineRoot; Pattern = $cp376PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "registration"
}
$cp376BindingText = Read-RepoText -Path $cp376Binding
$cp375BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_${cp375StemForCp376} =")
$cp376BindingIndex = $cp376BindingText.IndexOf("let calculation_${cp376Stem} =")
$cp377BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_${cp377StemForCp376} ="); $cp378BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp376 = $cp376BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp376NumericalIndex = $cp376BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp375BindingIndexForCp376 -lt 0 -or $cp376BindingIndex -le $cp375BindingIndexForCp376 -or
    $cp377BindingIndexForCp376 -le $cp376BindingIndex -or $cp378BindingIndexForCp376 -le $cp377BindingIndexForCp376 -or $cp379BindingIndexForCp376 -le $cp378BindingIndexForCp376 -or $cp380BindingIndexForCp376 -le $cp379BindingIndexForCp376 -or $cp381BindingIndexForCp376 -le $cp380BindingIndexForCp376 -or $cp382BindingIndexForCp376 -le $cp381BindingIndexForCp376 -or $cp383BindingIndexForCp376 -le $cp382BindingIndexForCp376 -or $cp384BindingIndexForCp376 -le $cp383BindingIndexForCp376 -or $cp385BindingIndexForCp376 -le $cp384BindingIndexForCp376 -or $cp376NumericalIndex -le $cp385BindingIndexForCp376) {
    throw "Binding must execute CP375 then CP376 then CP377 before unchanged numerical coupling"
}
$cp376Dto = Get-Cp376RustBraceBlock -Text $cp376BindingText.Substring($cp376NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp376TextNotContains -Text $cp376Dto -Pattern 'cp376|pre_saturation_original_assignment|supply_humidity_ratio_original' -Description "numerical DTO feed"
Assert-Contains -Path $cp376BindingTests -Pattern 'keeps_the_numerical_owner_unchanged' -Description "binding nonfeed regression"
Assert-Contains -Path $cp376CoupledTests -Pattern 'does_not_feed_numerical_result' -Description "coupled nonfeed regression"

# Fail-closed ep_run validation, finite JSON projection, exact sidecars, and arbitrary lane.
Assert-Contains -Path $cp376PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp411_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp376PipelineRoot -Pattern $cp376Lifecycle -Description "pipeline lifecycle key"
foreach ($pattern in @('validate_counts', 'transition_partition', 'owner_partition', 'source_site_execution_count', 'cp347_none_case_owner_count', 'checked_sum')) {
    Assert-Contains -Path $cp376PipelineValidation -Pattern $pattern -Description "pipeline validation $pattern"
}
Assert-Contains -Path $cp376SnapshotSerialization -Pattern 'json_number|is_finite' -Description "finite JSON projection"
foreach ($field in @('predecessor_resulting_supply_humidity_ratio', 'purchased_air_supply_humidity_ratio_before_saturation_check', 'assigned_supply_humidity_ratio_original', 'resulting_supply_humidity_ratio_original')) {
    Assert-Contains -Path $cp376SnapshotSerialization -Pattern "${field}_ieee_bits" -Description "IEEE sidecar $field"
}
Assert-Contains -Path $cp376ParentAssertions -Pattern 'mod cp376_assertions;' -Description "arbitrary CP376 module"
Assert-Contains -Path $cp376ParentAssertions -Pattern 'cp376_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp376ParentAssertions -Pattern 'cp376_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp376ParentAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP375 terminal nonfeed relinquishment"
Assert-Contains -Path $cp376ArbitraryAssertions -Pattern 'mod cp377_assertions;' -Description "arbitrary CP377 module"
Assert-Contains -Path $cp376ArbitraryAssertions -Pattern 'cp377_assertions::assert_direct\(runtime, results\)' -Description "arbitrary CP377 direct delegation"
Assert-Contains -Path $cp376ArbitraryAssertions -Pattern 'cp377_assertions::assert_non_direct\(runtime\)' -Description "arbitrary CP377 non-direct delegation"
Assert-NotContains -Path $cp376ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP376 terminal nonfeed relinquishment"
Assert-NotContains -Path $cp377ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP377 terminal numerical evidence relinquishment"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp378_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_exact_reconciliation\(' -Description "CP378 terminal reconciliation"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp379_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP379 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp380_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP380 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp382_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP382 terminal numerical nonfeed firewall"; Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp383_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_unchanged_enthalpy\(' -Description "CP383 terminal numerical nonfeed firewall"

# Exactly two stable spec addenda and five source-ordered hand-written sections.
$cp376AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp376CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp376AlgorithmAddenda = [regex]::Matches($cp376AlgorithmText, '(?m)^\s*"CP376 supersedes only CP375[^"\r\n]+",\s*$')
$cp376CapabilityAddenda = [regex]::Matches($cp376CapabilityText, '(?m)^\s*"CP376 additionally requires[^"\r\n]+",\s*$')
if ($cp376AlgorithmAddenda.Count -ne 2 -or $cp376CapabilityAddenda.Count -ne 2) {
    throw "CP376 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp376AlgorithmAddenda + $cp376CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp376SourceCommit, $cp376SourceHash, '2258', '2259', $cp376Sites[0], $cp376Sites[1],
            'eight', 'UnitOff', 'five', 'CP375', 'sole immediate route predecessor',
            'CP347', 'CP329', 'CP345', 'CP346', 'last writer', 'owner_count',
            'signed zero', 'NaN payload', 'finite-only', 'no arithmetic', 'no.*psychrometric',
            'CP375-to-CP376-to-unchanged-numerical', $cp376Lifecycle,
            '32 algorithms', '293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', '314 total', '240 public', '74 internal',
            'zero unused', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP376 spec addendum missing '$pattern'" }
    }
}
$cp376Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP376 Cooling Supply-Humidity-Ratio Pre-Saturation Original Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP376 Source-Ordered Cooling Supply-Humidity-Ratio Pre-Saturation Original Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP376 Supply-Humidity-Ratio Pre-Saturation Original Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP376 Pre-Saturation Original Humidity Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP376 Pre-Saturation Original Humidity-Assignment Placement' }
)
foreach ($doc in $cp376Docs) {
    $text = Read-RepoText -Path $doc.Path
    $sections = [regex]::Matches($text, '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)')
    if ($sections.Count -ne 1) { throw "CP376 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..376) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP376 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @(
            $cp376SourceCommit, $cp376SourceHash, '2258', '2259', $cp376Sites[0], $cp376Sites[1],
            'eight|8', 'CP375', 'CP347', 'CP329', 'CP345', 'CP346',
            '314\s+total', '74\s+internal'
        )) {
        if ($sections[0].Value -notmatch $required) { throw "CP376 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP376\b' -Description "psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP376 supersedes only CP375' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP376 additionally requires' -Description "generated capability addendum"

# Historical terminal expectations, master order, and generated inventory.
foreach ($historical in 334..375) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp411_lifecycle_evidence' -Description "historical firewall"
}
foreach ($historical in 335..375) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 349 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 109 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..375) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 349' -Description "historical inventory total"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..359 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp376Stem" -Description "historical CP376 binding order"
}
foreach ($historical in 360..375) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern "cp376BindingIndex" -Description "historical CP376 binding index"
}
foreach ($historical in @('cp326-cooling-supply-mass-flow-limit-body.ps1') + @(329..345 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name })) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp376Stem" -Description "historical CP376 helper whitelist"
}
$cp376MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp375AuditIndexForCp376 = $cp376MainAuditText.IndexOf("cp375-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-maximum-assignment.ps1")
$cp376AuditIndex = $cp376MainAuditText.IndexOf("cp376-cooling-supply-humidity-ratio-pre-saturation-original-assignment.ps1")
$cp377AuditIndexForCp376 = $cp376MainAuditText.IndexOf("cp377-cooling-supply-humidity-ratio-saturation-assignment.ps1")
$cp376CompletionIndex = $cp376MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp375AuditIndexForCp376 -lt 0 -or $cp376AuditIndex -le $cp375AuditIndexForCp376 -or
    $cp377AuditIndexForCp376 -le $cp376AuditIndex -or $cp376CompletionIndex -le $cp377AuditIndexForCp376) {
    throw "Master audit must dot-source CP377 after CP376 before completion"
}
$cp376InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp376TextContains -Text $cp376InventoryText -Pattern 'script_count = 349' -Description "script total"
Assert-Cp376TextContains -Text $cp376InventoryText -Pattern 'dev_command_count = 238' -Description "development-command total"
Assert-Cp376TextContains -Text $cp376InventoryText -Pattern 'unused_script_count = 0' -Description "zero unused"
if ([regex]::Matches($cp376InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp376InventoryText, '(?m)^classification = "internal"$').Count -ne 109) {
throw "CP376 inventory must be exactly 240 public and 106 internal scripts"
}
Assert-Cp376TextContains -Text $cp376InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp376-' -Description "inventory record"
Assert-Cp376TextContains -Text $cp376InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 349 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 109 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated unused"

Assert-Contains -Path "crates\ep_run\tests\arbitrary_run_ideal_loads\cp385_assertions.rs" -Pattern 'assert_numerical_nonfeed_and_local_enthalpy_only\(' -Description "CP385 terminal numerical nonfeed firewall"
Write-Host "CP376 pre-saturation original humidity-ratio assignment structure audit passed."
}
