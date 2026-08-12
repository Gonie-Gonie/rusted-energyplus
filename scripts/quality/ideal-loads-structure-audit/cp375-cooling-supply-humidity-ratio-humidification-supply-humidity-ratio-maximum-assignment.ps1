# CP375 maps PurchasedAirManager.cc line 2251's humidification
# supply-humidity-ratio result-store maximum assignment and no part of line 2258.
& {
$cp375Stem = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment"
$cp374StemForCp375 = "cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit"
$cp375PipelineStem = "purchased_air_$cp375Stem"
$cp375Lifecycle = "purchased_air_calc_${cp375Stem}_lifecycle"
$cp375SourceCommit = "6f2e40d10250a105b49966baa24d843711e61048"
$cp375SourceHash = "54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005"
$cp375Sites = @(
    "read-purchased-air-supply-humidity-ratio-for-humidification-supply-maximum",
    "read-local-supply-humidity-ratio-for-humidification-for-supply-maximum",
    "apply-source-shaped-two-argument-maximum-for-humidification-supply-maximum",
    "assign-purchased-air-supply-humidity-ratio-for-humidification-supply-maximum"
)
$cp375Source = ".reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc"
$cp375Module = "crates\ep_runtime\src\ideal_loads\calc\$cp375Stem.rs"
$cp375Root = "crates\ep_runtime\src\ideal_loads\calc\$cp375Stem"
$cp375State = "$cp375Root\state.rs"
$cp375Transition = "$cp375Root\transition.rs"
$cp375Predecessor = "$cp375Root\transition\predecessor.rs"
$cp375Release = "$cp375Root\release.rs"
$cp375Prefix = "$cp375Root\release\prefix_validation.rs"
$cp375Snapshot = "$cp375Root\release\snapshot_validation.rs"
$cp375SnapshotRoute = "$cp375Root\release\snapshot_validation\route.rs"
$cp375MaximumHelper = "crates\ep_runtime\src\ideal_loads\calc\cooling_positive_supply_temperature_minimum_limit\transition.rs"
$cp375CalcRoot = "crates\ep_runtime\src\ideal_loads\calc.rs"
$cp375Binding = "crates\ep_runtime\src\ideal_loads\binding.rs"
$cp375BindingAdapter = "crates\ep_runtime\src\ideal_loads\binding\$cp375Stem.rs"
$cp375BindingTests = "crates\ep_runtime\src\ideal_loads\binding\${cp375Stem}_tests.rs"
$cp375BindingTestsRoot = "crates\ep_runtime\src\ideal_loads\binding_tests.rs"
$cp375ScheduledOutput = "crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs"
$cp375InitState = "crates\ep_runtime\src\ideal_loads\init\state.rs"
$cp375InitUnit = "crates\ep_runtime\src\ideal_loads\init\state\unit.rs"
$cp375InitWitnessRoot = "crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs"
$cp375InitWitness = "crates\ep_runtime\src\ideal_loads\init\state\witnesses\$cp375Stem.rs"
$cp375CoupledRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime.rs"
$cp375Coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\${cp375Stem}_validation.rs"
$cp375CoupledTestsRoot = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests.rs"
$cp375CoupledTests = "crates\ep_runtime\src\ideal_loads\coupled_runtime_tests_cp375.rs"
$cp375FixtureRoot = "crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs"
$cp375Fixture = "crates\ep_runtime\src\ideal_loads\coupled_output_tests\${cp375Stem}_fixture.rs"
$cp375PipelineRoot = "crates\ep_run\src\pipeline.rs"
$cp375Pipeline = "crates\ep_run\src\pipeline\$cp375PipelineStem.rs"
$cp375PipelineValidation = "crates\ep_run\src\pipeline\$cp375PipelineStem\validation.rs"
$cp375PipelineTests = "crates\ep_run\src\pipeline\$cp375PipelineStem\validation\tests.rs"
$cp375Serialization = "crates\ep_run\src\pipeline\$cp375PipelineStem\serialization.rs"
$cp375SnapshotSerialization = "crates\ep_run\src\pipeline\$cp375PipelineStem\serialization\snapshot.rs"
$cp375SnapshotSerializationTests = "crates\ep_run\src\pipeline\$cp375PipelineStem\serialization\snapshot\tests.rs"
$cp375ParentAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp374_assertions.rs"
$cp375ArbitraryAssertions = "crates\ep_run\tests\arbitrary_run_ideal_loads\cp375_assertions.rs"
$cp375Audit = "scripts\quality\ideal-loads-structure-audit\cp375-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-maximum-assignment.ps1"

function Assert-Cp375TextContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch $Pattern) { throw "CP375 $Description missing" }
}

function Assert-Cp375TextNotContains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -match $Pattern) { throw "CP375 $Description unexpectedly present" }
}

function Get-Cp375RustBraceBlock {
    param([string]$Text, [string]$AnchorPattern, [string]$Description)
    $anchor = [regex]::Match($Text, $AnchorPattern)
    if (-not $anchor.Success) { throw "CP375 $Description anchor missing" }
    $open = $Text.IndexOf("{", $anchor.Index)
    if ($open -lt 0) { throw "CP375 $Description opening brace missing" }
    $depth = 0
    for ($index = $open; $index -lt $Text.Length; $index += 1) {
        if ($Text[$index] -eq "{") { $depth += 1 }
        elseif ($Text[$index] -eq "}") {
            $depth -= 1
            if ($depth -eq 0) { return $Text.Substring($anchor.Index, $index - $anchor.Index + 1) }
        }
    }
    throw "CP375 $Description closing brace missing"
}

$cp375Required = @(
    $cp375Module, $cp375State, $cp375Transition, $cp375Predecessor,
    $cp375Release, $cp375Prefix, $cp375Snapshot, $cp375SnapshotRoute, $cp375BindingAdapter,
    $cp375BindingTests, $cp375InitWitness, $cp375Coupled, $cp375CoupledTests,
    $cp375Fixture, $cp375Pipeline, $cp375PipelineValidation,
    $cp375PipelineTests, $cp375Serialization, $cp375SnapshotSerialization,
    $cp375SnapshotSerializationTests, $cp375ArbitraryAssertions, $cp375Audit
)
foreach ($file in $cp375Required) {
    Assert-FileExists -Path $file -Description "CP375 implementation/audit file"
    Assert-LineLimit -Path $file -Limit 500 -Description "CP375 bounded file"
}
$cp375CoreFiles = @(Get-ChildItem -LiteralPath $cp375Root -Recurse -File -Filter "*.rs")
$cp375CoreText = ($cp375CoreFiles | ForEach-Object {
        Assert-LineLimit -Path $_.FullName -Limit 500 -Description "CP375 bounded core file"
        Read-RepoText -Path $_.FullName
    }) -join "`n"
$cp375CoreTests = @(Get-ChildItem -LiteralPath "$cp375Root\tests" -Recurse -File -Filter "*.rs")
if ($cp375CoreTests.Count -lt 4) { throw "CP375 requires bounded direct/private/IEEE/overflow tests" }
$cp375CoreTestText = ($cp375CoreTests | ForEach-Object { Read-RepoText -Path $_.FullName }) -join "`n"
foreach ($pattern in @('direct', 'private', 'Humidistat', 'None', 'overflow', 'to_bits|ieee', 'signed_zero|signed zero', 'NaN', 'CP345', 'CP362')) {
    Assert-Cp375TextContains -Text $cp375CoreTestText -Pattern $pattern -Description "core test matrix '$pattern'"
}

# Locked source, exact assignment line, non-executable gap, and first executable exclusion.
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $cp375Source).Hash -cne $cp375SourceHash) {
    throw "CP375 PurchasedAirManager.cc SHA-256 drift"
}
$cp375Lines = Get-Content -Encoding UTF8 -LiteralPath $cp375Source
if ($cp375Lines[2250].Trim() -cne 'PurchAir.SupplyHumRat = max(PurchAir.SupplyHumRat, SupplyHumRatForHumid);' -or
    $cp375Lines[2251].Trim() -cne '}' -or $cp375Lines[2252].Trim() -cne '}' -or
    $cp375Lines[2253].Trim() -cne '}' -or $cp375Lines[2254].Trim() -cne '' -or
    $cp375Lines[2255].Trim() -cne '//   Limit supply humidity ratio to saturation at supply outlet temp' -or
    $cp375Lines[2256].Trim() -cne '' -or
    $cp375Lines[2257].Trim() -cne 'SupplyHumRatOrig = PurchAir.SupplyHumRat;') {
    throw "CP375 line 2251 and 2252-2258 source boundary drift"
}
Assert-Contains -Path $cp375Module -Pattern 'PurchasedAirManager\.cc:2251' -Description "CP375 mapped source"
Assert-Contains -Path $cp375Module -Pattern 'PurchasedAirManager\.cc:2258' -Description "CP375 first excluded executable source"
Assert-ExactStringArray -Path $cp375Module -Name 'PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER' -Expected $cp375Sites -Description "CP375 four-site source order"
Assert-Contains -Path $cp375Module -Pattern 'do not claim C\+\+ operand evaluation order' -Description "operand-order nonclaim"

# CP374 is the sole immediate predecessor and supplies the active right operand.
foreach ($pattern in @(
        'PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot',
        'cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_route',
        'Cp374Route', 'predecessor_route'
    )) {
    Assert-Contains -Path $cp375Predecessor -Pattern $pattern -Description "CP374 predecessor '$pattern'"
}
Assert-Contains -Path $cp375Transition -Pattern 'predecessor\.resulting_supply_humidity_ratio_for_humidification\?' -Description "CP374 right operand"
Assert-Cp375TextContains -Text $cp375CoreText -Pattern 'private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release' -Description "private active bridge"
foreach ($pattern in @(
        'active_none_operands_from_retained_cp345',
        'active_humidistat_operands_from_cp362_counterfactual',
        'owner\.assigned_supply_humidity_ratio\?',
        'private\.resulting_supply_humidity_ratio\?'
    )) {
    Assert-Contains -Path $cp375Prefix -Pattern $pattern -Description "branch-specific left owner '$pattern'"
}
foreach ($pattern in @(
        'owner\.parent_call_ordinal != predecessor\.parent_call_ordinal',
        'direct\.parent_call_ordinal != predecessor\.parent_call_ordinal',
        'cp345_snapshots_match_bit_exact',
        'cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact',
        'cp362_private_counterfactual', 'cp362_private_links'
    )) {
    Assert-Contains -Path $cp375Prefix -Pattern $pattern -Description "same-call owner proof '$pattern'"
}
Assert-Cp375TextContains -Text $cp375CoreTestText -Pattern 'cp375_direct_is_owner_free_and_private_none_uses_exact_cp345_owner' -Description "public/CP345 owner regression"
Assert-Cp375TextContains -Text $cp375CoreTestText -Pattern 'cp375_humidistat_left_is_the_validated_same_call_cp362_result' -Description "CP362 owner regression"

# Eight inherited routes, two private active writes, and four exact site counters.
foreach ($route in @(
        'UnitOff', 'NonCooling', 'PositiveGuardFalseFallthrough',
        'HeatingAvailabilityGuardFalseFallthrough', 'HumidificationControlGuardFalseFallthrough',
        'DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted',
        'DehumidificationControlGuardFalseFallthrough'
    )) {
    Assert-Contains -Path $cp375State -Pattern $route -Description "retained route $route"
}
foreach ($counter in @(
        'source_site_execution_count',
        'purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count',
        'supply_humidity_ratio_for_humidification_for_supply_maximum_read_count',
        'source_shaped_two_argument_maximum_evaluation_count',
        'purchased_air_supply_humidity_ratio_assignment_count'
    )) {
    Assert-Contains -Path $cp375State -Pattern "pub $counter\s*:\s*usize" -Description "site counter $counter"
}
Assert-Contains -Path $cp375Transition -Pattern 'SOURCE_ORDER\.len\(\)' -Description "four-site increment"
Assert-Contains -Path $cp375Transition -Pattern 'source_shaped_two_argument_maximum\(left, right\)' -Description "source-shaped maximum call"
Assert-Contains -Path $cp375MaximumHelper -Pattern 'if left < right \{ right \} else \{ left \}' -Description "CP333 strict less-than left-biased helper"
Assert-NotContains -Path $cp375Transition -Pattern 'f64::max|f64::min|\.max\(|\.min\(|if left > right|\.is_finite\(\)|\.clamp\(|Psy[A-Za-z_]*|DirectZonePurchasedAirCouplingInput' -Description "maximum/gate/service firewall"

# Public direct HumidificationControl None is complete-null and reads no numeric owner.
$cp375ReleaseText = Read-RepoText -Path $cp375Release
$cp375PublicRelease = Get-Cp375RustBraceBlock -Text $cp375ReleaseText -AnchorPattern "(?m)^pub fn advance_direct_no_oa_calc_${cp375Stem}\s*\(" -Description "public direct release"
Assert-Cp375TextContains -Text $cp375PublicRelease -Pattern 'predecessor_cp374' -Description "public CP374 predecessor"
Assert-Cp375TextContains -Text $cp375PublicRelease -Pattern '(?s)advance_.*?None' -Description "public complete-null transition"
Assert-Cp375TextNotContains -Text $cp375PublicRelease -Pattern 'f64|CP345|CP362|assigned_supply_humidity_ratio|resulting_supply_humidity_ratio\s*:' -Description "public numeric owner read"

# Registrations, CP374 -> CP375 -> numerical order, JSON bits, and no numerical feed.
foreach ($registration in @(
        [PSCustomObject]@{ Path = $cp375CalcRoot; Pattern = $cp375Stem },
        [PSCustomObject]@{ Path = $cp375ScheduledOutput; Pattern = "pub calculation_${cp375Stem}:" },
        [PSCustomObject]@{ Path = $cp375BindingTestsRoot; Pattern = $cp375Stem },
        [PSCustomObject]@{ Path = $cp375InitState; Pattern = $cp375Stem },
        [PSCustomObject]@{ Path = $cp375InitUnit; Pattern = $cp375Stem },
        [PSCustomObject]@{ Path = $cp375InitWitnessRoot; Pattern = $cp375Stem },
        [PSCustomObject]@{ Path = $cp375CoupledRoot; Pattern = "mod ${cp375Stem}_validation;" },
        [PSCustomObject]@{ Path = $cp375CoupledTestsRoot; Pattern = 'coupled_runtime_tests_cp375' },
        [PSCustomObject]@{ Path = $cp375FixtureRoot; Pattern = $cp375Stem },
        [PSCustomObject]@{ Path = $cp375PipelineRoot; Pattern = $cp375PipelineStem }
    )) {
    Assert-Contains -Path $registration.Path -Pattern $registration.Pattern -Description "CP375 registration"
}
$cp375BindingText = Read-RepoText -Path $cp375Binding
$cp374BindingIndexForCp375 = $cp375BindingText.IndexOf("let calculation_${cp374StemForCp375} =")
$cp375BindingIndex = $cp375BindingText.IndexOf("let calculation_${cp375Stem} =")
$cp376BindingIndex = $cp375BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment ="); $cp377BindingIndex = $cp375BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_assignment ="); $cp378BindingIndex = $cp375BindingText.IndexOf("let calculation_cooling_supply_humidity_ratio_saturation_limit_assignment ="); $cp379BindingIndex = $cp375BindingText.IndexOf("let calculation_cooling_supply_enthalpy_post_saturation_assignment ="); $cp380BindingIndex = $cp375BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_guard ="); $cp381BindingIndex = $cp375BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard ="); $cp382BindingIndex = $cp375BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment ="); $cp383BindingIndex = $cp375BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard ="); $cp384BindingIndexForCp375 = $cp375BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment ="); $cp385BindingIndexForCp375 = $cp375BindingText.IndexOf("let calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment =")
$cp375NumericalIndex = $cp375BindingText.IndexOf("let coupling = complete_direct_zone_purchased_air_coupling(")
if ($cp374BindingIndexForCp375 -lt 0 -or $cp375BindingIndex -le $cp374BindingIndexForCp375 -or
    $cp376BindingIndex -le $cp375BindingIndex -or $cp377BindingIndex -le $cp376BindingIndex -or $cp378BindingIndex -le $cp377BindingIndex -or $cp379BindingIndex -le $cp378BindingIndex -or $cp380BindingIndex -le $cp379BindingIndex -or $cp381BindingIndex -le $cp380BindingIndex -or $cp382BindingIndex -le $cp381BindingIndex -or $cp383BindingIndex -le $cp382BindingIndex -or $cp384BindingIndexForCp375 -le $cp383BindingIndex -or $cp385BindingIndexForCp375 -le $cp384BindingIndexForCp375 -or $cp375NumericalIndex -le $cp385BindingIndexForCp375) {
    throw "Binding must execute CP374 then CP375 then CP376 before unchanged numerical coupling"
}
$cp375Dto = Get-Cp375RustBraceBlock -Text $cp375BindingText.Substring($cp375NumericalIndex) -AnchorPattern 'DirectZonePurchasedAirCouplingInput\s*\{' -Description "numerical DTO"
Assert-Cp375TextNotContains -Text $cp375Dto -Pattern 'cp375|maximum_assignment|supply_humidity_ratio_for_humidification' -Description "numerical DTO feed"
Assert-Contains -Path $cp375CoupledTests -Pattern 'does_not_feed_numerical_result' -Description "supported direct numerical nonfeed regression"
Assert-Contains -Path $cp375ParentAssertions -Pattern 'mod cp375_assertions;' -Description "arbitrary CP375 module"
Assert-Contains -Path $cp375ParentAssertions -Pattern 'cp375_assertions::assert_direct\(runtime, results\)' -Description "arbitrary direct delegation"
Assert-Contains -Path $cp375ParentAssertions -Pattern 'cp375_assertions::assert_non_direct\(runtime\)' -Description "arbitrary non-direct delegation"
Assert-NotContains -Path $cp375ParentAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP374 relinquishes terminal nonfeed"
Assert-Contains -Path $cp375ArbitraryAssertions -Pattern 'mod cp376_assertions;' -Description "CP376 arbitrary delegation module"
Assert-Contains -Path $cp375ArbitraryAssertions -Pattern 'cp376_assertions::assert_direct\(runtime, results\)' -Description "CP376 arbitrary direct delegation"
Assert-Contains -Path $cp375ArbitraryAssertions -Pattern 'cp376_assertions::assert_non_direct\(runtime\)' -Description "CP376 arbitrary non-direct delegation"
Assert-NotContains -Path $cp375ArbitraryAssertions -Pattern 'assert_numerical_nonfeed\(' -Description "CP375 relinquishes terminal nonfeed"
Assert-Contains -Path $cp375PipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp417_lifecycle_evidence' -Description "cumulative non-direct firewall"
Assert-Contains -Path $cp375PipelineRoot -Pattern $cp375Lifecycle -Description "pipeline lifecycle key"
Assert-Contains -Path $cp375SnapshotSerialization -Pattern 'json_number|is_finite' -Description "finite JSON projection"
Assert-Contains -Path $cp375SnapshotSerialization -Pattern '_ieee_bits' -Description "authoritative IEEE sidecars"

# Exactly two stable spec addenda and five source-ordered hand-written sections.
$cp375AlgorithmText = Read-RepoText -Path "specs\algorithm_ledger.toml"
$cp375CapabilityText = Read-RepoText -Path "specs\capabilities.toml"
$cp375AlgorithmAddenda = [regex]::Matches($cp375AlgorithmText, '(?m)^\s*"CP375 supersedes only CP374[^"\r\n]+",\s*$')
$cp375CapabilityAddenda = [regex]::Matches($cp375CapabilityText, '(?m)^\s*"CP375 additionally requires[^"\r\n]+",\s*$')
if ($cp375AlgorithmAddenda.Count -ne 2 -or $cp375CapabilityAddenda.Count -ne 2) {
    throw "CP375 must have exactly two algorithm and two capability addenda"
}
foreach ($claim in @($cp375AlgorithmAddenda + $cp375CapabilityAddenda)) {
    foreach ($pattern in @(
            $cp375SourceCommit, $cp375SourceHash, '2251', '2252-2257', '2258',
            'four', 'CP374', 'sole immediate predecessor', 'CP345', 'CP362',
            'no intervening', 'CP333', 'if left < right', 'left bias',
            'signed zero', 'NaN', 'no `f64::max`',
            'CP374-to-CP375-to-unchanged-numerical', $cp375Lifecycle,
            '32 algorithms', '293 routines', '58 state-mapped plus 235 source-mapped',
            '170 required', '313 total', '240 public', '73 internal',
            'zero unused', '238 development commands', 'Roadmap'
        )) {
        if ($claim.Value -notmatch $pattern) { throw "CP375 spec addendum missing '$pattern'" }
    }
}
$cp375Docs = @(
    [PSCustomObject]@{ Path = "docs\src\current\current-status.md"; Heading = 'CP375 Cooling Humidification Supply-Humidity-Ratio Maximum Assignment' },
    [PSCustomObject]@{ Path = "docs\src\current\project-contract.md"; Heading = 'CP375 Source-Ordered Cooling Humidification Supply-Humidity-Ratio Maximum Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\ideal-loads-source-map.md"; Heading = 'CP375 Humidification Supply-Humidity-Ratio Maximum Assignment' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\heat-balance-source-map.md"; Heading = 'CP375 Humidification Supply-Humidity-Ratio Maximum Assignment in the Heat-Balance Loop' },
    [PSCustomObject]@{ Path = "docs\src\porting-map\zone-air-update-map.md"; Heading = 'CP375 Humidification Supply-Humidity-Ratio Maximum-Assignment Placement' }
)
foreach ($doc in $cp375Docs) {
    $text = Read-RepoText -Path $doc.Path
    $pattern = '(?ms)^## ' + [regex]::Escape($doc.Heading) + '\r?\n.*?(?=^## |\z)'
    $sections = [regex]::Matches($text, $pattern)
    if ($sections.Count -ne 1) { throw "CP375 documentation expected one section in $($doc.Path)" }
    $previous = -1
    foreach ($checkpoint in 370..375) {
        $index = $text.LastIndexOf("## CP$checkpoint ")
        if ($index -le $previous) { throw "CP370 through CP375 documentation order drift in $($doc.Path)" }
        $previous = $index
    }
    foreach ($required in @(
            $cp375SourceCommit, $cp375SourceHash, '2251', '2252-2257|2252-2254',
            '2258', 'CP374', 'CP345', 'CP362', 'CP333', 'left',
            '313\s+total', '73\s+internal'
        )) {
        if ($sections[0].Value -notmatch $required) { throw "CP375 documentation in $($doc.Path) missing '$required'" }
    }
}
Assert-NotContains -Path "docs\src\porting-map\psychrometrics-source-map.md" -Pattern '(?m)^## CP375\b' -Description "CP375 psychrometrics non-promotion"
Assert-Contains -Path "docs\src\generated\algorithm-ledger.md" -Pattern 'CP375 supersedes only CP374' -Description "generated algorithm addendum"
Assert-Contains -Path "docs\src\generated\capability-index.md" -Pattern 'CP375 additionally requires' -Description "generated capability addendum"

# Historical terminal expectations and current generated inventory.
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..371 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "calculation_$cp375Stem" -Description "historical CP375 binding order"
}
foreach ($historical in 372..374) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern "cp375StemForCp$historical" -Description "historical CP375 binding order"
}
foreach ($historical in @("cp326-cooling-supply-mass-flow-limit-body.ps1") + @(
        329..344 | ForEach-Object { (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$($_)-*.ps1").Name }
    )) {
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$historical" -Pattern "advance_$cp375Stem" -Description "historical CP375 helper whitelist"
}
foreach ($historical in 334..374) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'non_direct_runtime_rejects_cp316_through_cp417_lifecycle_evidence' -Description "historical CP375 firewall"
}
foreach ($historical in 335..374) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 355 \|')) -Description "historical generated total"
Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern ([regex]::Escape('\| 115 \|')) -Description "historical generated internal"
}
foreach ($historical in 337..374) {
    $file = (Get-ChildItem -LiteralPath "scripts\quality\ideal-loads-structure-audit" -Filter "cp$historical-*.ps1").Name
    Assert-Contains -Path "scripts\quality\ideal-loads-structure-audit\$file" -Pattern 'script_count = 355' -Description "historical inventory total"
}
$cp375MainAuditText = Read-RepoText -Path "scripts\quality\ideal-loads-structure-audit.ps1"
$cp374AuditIndexForCp375 = $cp375MainAuditText.IndexOf("cp374-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-for-humidification-maximum-limit.ps1")
$cp375AuditIndex = $cp375MainAuditText.IndexOf("cp375-cooling-supply-humidity-ratio-humidification-supply-humidity-ratio-maximum-assignment.ps1")
$cp375CompletionIndex = $cp375MainAuditText.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($cp374AuditIndexForCp375 -lt 0 -or $cp375AuditIndex -le $cp374AuditIndexForCp375 -or
    $cp375CompletionIndex -le $cp375AuditIndex) {
    throw "Master audit must dot-source CP375 after CP374 before completion"
}
$cp375InventoryText = Read-RepoText -Path "specs\script_inventory.toml"
Assert-Cp375TextContains -Text $cp375InventoryText -Pattern 'script_count = 355' -Description "script total"
Assert-Cp375TextContains -Text $cp375InventoryText -Pattern 'dev_command_count = 238' -Description "stable dev-command total"
Assert-Cp375TextContains -Text $cp375InventoryText -Pattern 'unused_script_count = 0' -Description "zero unused"
if ([regex]::Matches($cp375InventoryText, '(?m)^classification = "public"$').Count -ne 240 -or
[regex]::Matches($cp375InventoryText, '(?m)^classification = "internal"$').Count -ne 115) {
throw "CP376 inventory must be exactly 240 public and 113 internal scripts"
}
Assert-Cp375TextContains -Text $cp375InventoryText -Pattern 'path = "scripts/quality/ideal-loads-structure-audit/cp375-' -Description "inventory record"
Assert-Cp375TextContains -Text $cp375InventoryText -Pattern 'ideal-loads-structure-audit\.ps1::dot_sources' -Description "caller evidence"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 355 \|' -Description "generated total"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| public scripts \| 240 \|' -Description "generated public"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| 115 \|' -Description "generated internal"
Assert-Contains -Path "docs\src\generated\script-index.md" -Pattern '\| scripts without callers \| 0 \|' -Description "generated unused"

Write-Host "CP375 Cooling humidification supply-humidity-ratio maximum-assignment structure audit passed."
}
