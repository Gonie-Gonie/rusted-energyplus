# CP434 maps PurchasedAirManager.cc physical executable line 2351's DeadBand assignment.
& {
function Get-Cp434Sha([byte[]]$Bytes) {
    $sha = [Security.Cryptography.SHA256]::Create()
    try { return ([BitConverter]::ToString($sha.ComputeHash($Bytes))).Replace('-', '') } finally { $sha.Dispose() }
}
function Get-Cp434SnapshotBlock([string]$Text, [string]$Type, [string]$EndMarker) {
    $start = $Text.IndexOf("pub struct $Type")
    $end = $Text.IndexOf($EndMarker, $start)
    if ($start -lt 0 -or $end -le $start) { throw "CP434 snapshot block drift: $Type" }
    return $Text.Substring($start, $end - $start)
}
function Get-Cp434BraceBlock([string]$Text, [string]$Anchor, [string]$Description) {
    $hits = [regex]::Matches($Text,$Anchor)
    if ($hits.Count -ne 1) { throw "CP434 $Description anchor count $($hits.Count)" }
    $open = $Text.IndexOf('{',$hits[0].Index); $depth = 0
    for ($i=$open; $i -lt $Text.Length; $i++) {
        if ($Text[$i] -eq '{') { $depth++ } elseif ($Text[$i] -eq '}') { $depth--; if ($depth -eq 0) { $following = $Text.Substring($i,[Math]::Min(80,$Text.Length - $i)); if ($following -notmatch '^\},\s*\r?\n\s*prediction,') { throw "CP434 $Description must end at }, before prediction" }; return $Text.Substring($open + 1,$i - $open - 1) } }
    }
    throw "CP434 $Description brace drift"
}
$stem = 'heating_operating_mode_deadband_assignment'
$predecessorStem = 'heating_mode_guard_else_branch_entry'
$typeStem = 'PurchasedAirCalcHeatingOperatingModeDeadbandAssignment'
$source = '.reference\energyplus-src\26.1.0\src\EnergyPlus\PurchasedAirManager.cc'
$module = "crates\ep_runtime\src\ideal_loads\calc\$stem.rs"
$predecessorModule = "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem.rs"
$root = "crates\ep_runtime\src\ideal_loads\calc\$stem"
$state = "$root\state.rs"; $snapshot = "$root\transition\snapshot.rs"; $tests = "$root\tests.rs"; $schemaTests = "$root\tests\schema_prefix.rs"
$binding = 'crates\ep_runtime\src\ideal_loads\binding.rs'; $scheduled = 'crates\ep_runtime\src\ideal_loads\binding\scheduled_output.rs'
$adapter = "crates\ep_runtime\src\ideal_loads\binding\$stem.rs"; $adapterTests = "crates\ep_runtime\src\ideal_loads\binding\$($stem)_tests.rs"
$coupled = "crates\ep_runtime\src\ideal_loads\coupled_runtime\$($stem)_validation.rs"; $coupledTests = 'crates\ep_runtime\src\ideal_loads\test_coupled_runtime_cp434.rs'
$coupledOutput = 'crates\ep_runtime\src\ideal_loads\coupled_output_tests.rs'; $pipelineRoot = 'crates\ep_run\src\pipeline.rs'
$pipeline = "crates\ep_run\src\pipeline\purchased_air_$stem"; $pipelineModule = "$pipeline.rs"; $serialization = "$pipeline\serialization\snapshot.rs"; $pipelineValidation = "$pipeline\validation.rs"
$arbitrary = 'crates\ep_run\tests\arbitrary_run_ideal_loads\cp434_assertions.rs'; $audit = "scripts\quality\ideal-loads-structure-audit\cp434-$($stem -replace '_','-').ps1"
foreach ($file in @($source,$module,$predecessorModule,$state,$snapshot,$tests,$schemaTests,$binding,$scheduled,$adapter,$adapterTests,$coupled,$coupledTests,$coupledOutput,$pipelineRoot,$pipelineModule,$serialization,$pipelineValidation,$arbitrary,$audit)) { Assert-FileExists -Path $file -Description 'CP434 implementation/audit file' }
$bounded = @($audit,$module,$pipelineModule) + @((Get-ChildItem $root -Recurse -File -Filter '*.rs').FullName) + @((Get-ChildItem $pipeline -Recurse -File -Filter '*.rs').FullName)
foreach ($file in $bounded) { Assert-LineLimit -Path $file -Limit 500 -Description 'bounded CP434 file' }
$sourceBytes = [IO.File]::ReadAllBytes($source)
if ((Get-Cp434Sha $sourceBytes) -cne '54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005') { throw 'CP434 pinned PurchasedAirManager.cc hash drift' }
$sourceLines = Get-Content -LiteralPath $source
if ($sourceLines[2350].Trim() -cne 'OperatingMode = OpMode::DeadBand;' -or $sourceLines[2351].Trim() -cne '}' -or $sourceLines[2360].Trim() -cne 'if (((PurchAir.HeatingLimit == LimitType::FlowRate) || (PurchAir.HeatingLimit == LimitType::FlowRateAndCapacity)) &&') { throw 'CP434 physical 2351/2352/2361 boundary drift' }
foreach ($index in 2352..2359) { $line = $sourceLines[$index].Trim(); if ($line -and -not $line.StartsWith('//')) { throw "CP434 first-executable boundary drift at physical line $($index + 1)" } }
Assert-ExactStringArray -Path $module -Name 'PURCHASED_AIR_CALC_HEATING_OPERATING_MODE_DEADBAND_ASSIGNMENT_SOURCE_ORDER' -Expected @('assign-local-operating-mode-deadband') -Description 'sole CP434 assignment site'
foreach ($pattern in @('EnergyPlus 26\.1 PurchasedAirManager\.cc:2351','EnergyPlus 26\.1 PurchasedAirManager\.cc:2361','heating_operating_mode_deadband_assignment_executed','heating_operating_mode_deadband_assignment_performed','assigned_heating_operating_mode_deadband')) { Assert-Contains -Path $module -Pattern $pattern -Description 'CP434 boundary/assignment contract' }
Assert-Contains -Path $snapshot -Pattern 'assigned_heating_operating_mode_deadband:\s*route\.assignment_executed\.then_some\(IdealLoadsSensibleMode::Deadband\)' -Description 'CP434 authoritative Deadband write'
$core = @(Get-ChildItem -LiteralPath $root -Recurse -File -Filter '*.rs')
$predecessorCore = @(Get-ChildItem -LiteralPath "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem" -Recurse -File -Filter '*.rs')
if ($core.Count -ne 13 -or @(Get-ChildItem "$root\tests" -File -Filter '*.rs').Count -ne 2) { throw 'CP434 exact thirteen-file/two-test topology drift' }
if ($predecessorCore.Count -ne 13 -or @(Get-ChildItem "crates\ep_runtime\src\ideal_loads\calc\$predecessorStem\tests" -File -Filter '*.rs').Count -ne 2) { throw 'CP434 exact CP433 thirteen-file/two-test topology drift' }
$stateText = Read-RepoText -Path $state
$arrays = @([regex]::Matches($stateText, '(?m)^\s*pub\s+(?<name>[A-Za-z_][A-Za-z0-9_]*route_counts)\s*:\s*\[usize;\s*36\]') | ForEach-Object { $_.Groups['name'].Value })
if ($arrays.Count -ne 2 -or ($arrays -join '|') -cne 'predecessor_route_counts|heating_operating_mode_deadband_assignment_route_counts') { throw 'CP434 exact two width-36 arrays drift' }
foreach ($pattern in @('exhaustive_61_routes_have_exact_deadband_partition_and_accounting','\(public,\s*61\s*-\s*public\),\s*\(20,\s*41\)','\(public_assignments,\s*private_assignments\),\s*\(1,\s*1\)','heat_assignments,\s*state\.heating_operating_mode_deadband_assignment_count','\(1,\s*2\)','guard_evaluations','transition_count,\s*61','inactive_transition_count,\s*59','heating_operating_mode_deadband_assignment_count,\s*2','source_site_execution_count,\s*2','expected\[0\]\[1\],\s*3','expected\[1\]\[1\],\s*2','cp433_supply_humidity_ratio_state_owner_count,\s*37','cp433_supply_enthalpy_state_owner_count,\s*42','cp433_supply_temperature_state_owner_count,\s*57')) { Assert-Contains -Path $tests -Pattern $pattern -Description 'CP434 exhaustive routes/counts/index/WHT contract' }
$moduleText = Read-RepoText -Path $module; $predecessorText = Read-RepoText -Path $predecessorModule
$snapshotBlock = Get-Cp434SnapshotBlock $moduleText "$($typeStem)Snapshot" '/// Final selected-unit CP434'
$predecessorBlock = Get-Cp434SnapshotBlock $predecessorText 'PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot' '/// Final selected-unit CP433'
$fields = @([regex]::Matches($snapshotBlock, '(?m)^\s*pub\s+(?<name>[A-Za-z_][A-Za-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
$predecessorFields = @([regex]::Matches($predecessorBlock, '(?m)^\s*pub\s+(?<name>[A-Za-z_][A-Za-z0-9_]*)\s*:') | ForEach-Object { $_.Groups['name'].Value })
if ($predecessorFields.Count -ne 352 -or $fields.Count -ne 361 -or @($fields | Select-Object -Unique).Count -ne 361 -or ($fields[0..347] -join '|') -cne ($predecessorFields[0..347] -join '|')) { throw 'CP434 exact unique 361-field/CP433-first-348 schema drift' }
$tail = @('predecessor_cp433_resulting_supply_humidity_ratio','predecessor_cp433_resulting_supply_enthalpy_j_per_kg','predecessor_cp433_resulting_supply_temperature_c','heating_mode_guard_else_branch_entered','heating_operating_mode_deadband_assignment_executed','cp433_retained_supply_humidity_ratio_state_owned','cp433_retained_supply_enthalpy_state_owned','cp433_retained_supply_temperature_state_owned','heating_operating_mode_deadband_assignment_performed','assigned_heating_operating_mode_deadband','resulting_supply_humidity_ratio','resulting_supply_enthalpy_j_per_kg','resulting_supply_temperature_c')
if (($fields[348..360] -join '|') -cne ($tail -join '|') -or [regex]::Matches($snapshotBlock,'Option<f64>').Count -ne 128 -or [regex]::Matches($snapshotBlock,'Option<bool>').Count -ne 4 -or ([regex]::Matches($snapshotBlock,'Option<').Count - 128 - 4) -ne 4) { throw 'CP434 locked 361/128/4/4 schema/tail drift' }
foreach ($pattern in @('cp434_schema_is_exact_361_128_4_4_with_cp433_first_348_and_locked_tail','predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_61_routes')) { Assert-Contains -Path $schemaTests -Pattern $pattern -Description 'CP434 schema/prefix regression' }
foreach ($pattern in @('serializer_source_preserves_cp433_prefix_and_extends_exact_19_key_tail','cp433_snapshot_json\(predecessor\)','keys\.len\(\),\s*19','keys\[12\],\s*"assigned_heating_operating_mode_deadband"')) { Assert-Contains -Path $serialization -Pattern $pattern -Description 'CP434 CP433-prefix/19-key serializer contract' }
foreach ($pattern in @('actual\.len\(\),\s*489','latest\.len\(\),\s*489','ends_with\("_ieee_bits"\)\)\s*\.count\(\),\s*128','fields\.len\(\),\s*361','snapshot\.matches\("Option<f64>"\)\.count\(\),\s*128','calculation_fields\.len\(\),\s*128','calculation_fields\[123\].*heating_mode_guard_else_branch_entry','calculation_fields\[124\].*heating_operating_mode_deadband_assignment')) { Assert-Contains -Path $arbitrary -Pattern $pattern -Description 'actual CP434 JSON/IEEE/schema/binding contract' }
Assert-PatternsInOrder -Path $binding -Patterns @("let\s+calculation_$predecessorStem\s*=","let\s+calculation_$stem\s*=",'let\s+unit_available\s*=','let\s+coupling\s*=') -Description 'CP433-to-CP434-to-numerical order'
Assert-PatternsInOrder -Path $scheduled -Patterns @("pub\s+calculation_$predecessorStem\s*:","pub\s+calculation_$stem\s*:",'pub\s+coupling\s*:') -Description 'scheduled CP433-to-CP434 order'
$scheduledFields = @([regex]::Matches((Read-RepoText -Path $scheduled), '(?m)^\s*pub\s+(?<name>calculation_[A-Za-z0-9_]+)\s*:') | ForEach-Object { $_.Groups['name'].Value })
if ($scheduledFields.Count -ne 128 -or $scheduledFields[123] -cne 'calculation_heating_mode_guard_else_branch_entry' -or $scheduledFields[124] -cne 'calculation_heating_operating_mode_deadband_assignment') { throw 'CP434 exact binding index 123/124/current 128 drift' }; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_heating_outdoor_air_maximum_flow_guard' -Description 'CP435 recent binding propagation'
foreach ($test in @('scheduled_binding_advances_cp434_after_cp433_before_unchanged_coupling','cp434_adapter_accepts_only_the_cp433_snapshot_and_no_scalar_input','cp434_historical_124_to_125_transition_is_preserved_in_current_128_snapshot_binding')) { Assert-Contains -Path $adapterTests -Pattern $test -Description 'CP434 adapter/current-binding regression' }
$dto = Get-Cp434BraceBlock (Read-RepoText -Path $binding) 'DirectZonePurchasedAirCouplingInput\s*\{\s*zone_state\s*:' 'numerical coupling-input DTO'
if (-not $dto -or $dto -match '(?i)cp434|heating_operating_mode_deadband_assignment|assigned_heating_operating_mode_deadband') { throw 'CP434 entered numerical coupling-input DTO' }
foreach ($pattern in @('numerical_deadband_reconciliation_count','assigned_heating_operating_mode_deadband','calculation\.mode\s*==\s*IdealLoadsSensibleMode::Deadband','assigned_enum_and_numerical_deadband_are_reconciliation_only','production_validator_keeps_characterization_and_coupling_input_out')) { Assert-Contains -Path $coupled -Pattern $pattern -Description 'CP434 post-hoc public Deadband reconciliation/no-feed seal' }
foreach ($pattern in @('cp434_contract_locks_exhaustive_routes_current_schema_and_binding','cp434_is_ordered_after_cp433_and_reconciles_without_feeding_numerics','\[61usize,\s*59,\s*2,\s*2\]','20usize\s*\+\s*41','\.matches\("    pub calculation_"\)\s*\.count\(\),\s*128')) { Assert-Contains -Path $coupledTests -Pattern $pattern -Description 'CP434 coupled count/binding/reconciliation contract' }
Assert-Contains -Path $coupledOutput -Pattern 'cp434_assignment_mutation_cannot_change_the_unchanged_numerical_coupling_output' -Description 'CP434 unchanged numerical output mutation regression'
Assert-Contains -Path $pipelineRoot -Pattern 'non_direct_runtime_rejects_cp316_through_cp437_lifecycle_evidence' -Description 'CP434 non-direct firewall'
foreach ($pattern in @('validate_direct_lifecycle','validate_public_route_contract','heating_operating_mode_deadband_assignment_route_counts','validator_is_structural_and_has_no_numerical_or_dto_feed')) { Assert-Contains -Path $pipelineValidation -Pattern $pattern -Description 'CP434 pipeline structural/no-feed seal' }
$ledger = Read-RepoText -Path 'specs\algorithm_ledger.toml'; $capabilities = Read-RepoText -Path 'specs\capabilities.toml'
if ([regex]::Matches($ledger,[regex]::Escape('CP434 supersedes only CP433')).Count -ne 1 -or [regex]::Matches($capabilities,[regex]::Escape('CP434 additionally requires')).Count -ne 1) { throw 'CP434 bounded spec addendum drift' }
foreach ($text in @($ledger,$capabilities)) { foreach ($claim in @('54D960BCBFDF4F424A84BA73BF62040677424AD93E2F9362584898B0B146C005','physical-line-2351','assign-local-operating-mode-deadband','assigned_heating_operating_mode_deadband','first 348 nonterminal fields','first 470 nonterminal keys','361','489','128','372 total','240 public','132 internal')) { if (-not $text.Contains($claim)) { throw "CP434 spec claim missing $claim" } } }
if ([regex]::Matches($ledger,'(?m)^\[\[algorithm\]\]\r?$').Count -ne 32 -or [regex]::Matches($ledger,'(?m)^routine\.[A-Za-z0-9_]+\.completion_status\s*=\s*"state_mapped"\r?$').Count -ne 58 -or [regex]::Matches($ledger,'(?m)^routine\.[A-Za-z0-9_]+\.completion_status\s*=\s*"source_mapped"\r?$').Count -ne 235 -or [regex]::Matches($ledger,'(?m)^routine\.[A-Za-z0-9_]+\.required_for_full_domain\s*=\s*true\r?$').Count -ne 170) { throw 'CP434 unchanged algorithm/routine count drift' }
Assert-NotContains -Path 'docs\src\current\roadmap.md' -Pattern '(?m)^## CP434\b' -Description 'Roadmap non-promotion'
Assert-NotContains -Path 'docs\src\porting-map\psychrometrics-source-map.md' -Pattern '(?m)^## CP434\b' -Description 'psychrometrics non-promotion'
$auditRoot = 'scripts\quality\ideal-loads-structure-audit'; $audits = @(Get-ChildItem $auditRoot -Filter 'cp*.ps1' -File)
$historical = @($audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and (([int]$Matches.n -eq 326) -or ([int]$Matches.n -ge 329 -and [int]$Matches.n -le 433)) })
if ($historical.Count -ne 106 -or @($historical | Where-Object { $_.BaseName -match '^cp(?:327|328)-' }).Count) { throw 'CP434 exact 106 historical leaf set drift' }
$firewall = @($audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and [int]$Matches.n -ge 334 -and [int]$Matches.n -le 434 })
$script = @($audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and [int]$Matches.n -ge 337 -and [int]$Matches.n -le 434 })
$internal = @($audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and [int]$Matches.n -ge 367 -and [int]$Matches.n -le 434 })
$generated = @($audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and [int]$Matches.n -ge 335 -and [int]$Matches.n -le 434 })
if ($firewall.Count -ne 101 -or $script.Count -ne 98 -or $internal.Count -ne 68 -or $generated.Count -ne 100) { throw 'CP434 propagation range cardinality drift' }
$staleFirewall = 'non_direct_runtime_rejects_cp316_through_cp43' + '3_lifecycle_evidence'; $staleScript = 'script_count = 37' + '1'; $staleGeneratedTotal = '\| 37' + '1 \|'; $staleGeneratedInternal = '\| 13' + '1 \|'
foreach ($file in $firewall) { $text = Read-RepoText -Path $file.FullName; if (-not $text.Contains('non_direct_runtime_rejects_cp316_through_cp437_lifecycle_evidence') -or $text.Contains($staleFirewall)) { throw "CP434 firewall propagation drift $($file.Name)" } }
foreach ($file in $script) { $text = Read-RepoText -Path $file.FullName; if (-not $text.Contains('script_count = 375') -or $text.Contains($staleScript)) { throw "CP434 inventory-total propagation drift $($file.Name)" } }
foreach ($file in $internal) { $text = Read-RepoText -Path $file.FullName; if ($text -notmatch '(?s)classification\s*=\s*"internal".{0,120}\.Count\s*-ne\s*135' -or $text -match '(?s)classification\s*=\s*"internal".{0,120}\.Count\s*-ne\s*131') { throw "CP434 internal propagation drift $($file.Name)" } }
foreach ($file in $generated) { $text = Read-RepoText -Path $file.FullName; if (-not $text.Contains('\| 375 \|') -or -not $text.Contains('\| 135 \|') -or $text.Contains($staleGeneratedTotal) -or $text.Contains($staleGeneratedInternal)) { throw "CP434 generated propagation drift $($file.Name)" } }
$helpers = @((Get-ChildItem $auditRoot -Filter 'cp326-*.ps1' -File); Get-ChildItem $auditRoot -Filter 'cp3*.ps1' -File | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and [int]$Matches.n -ge 329 -and [int]$Matches.n -le 344 })
if ($helpers.Count -ne 17) { throw 'CP434 helper set drift' }
foreach ($file in $helpers) { $text = Read-RepoText -Path $file.FullName; if (-not $text.Contains("calculation_$stem") -or -not $text.Contains("advance_$stem") -or -not $text.Contains('CP340 through CP437')) { throw "CP434 helper strip/whitelist drift $($file.Name)" } }
$terminal = @((Get-ChildItem $auditRoot -Filter 'cp345-*.ps1' -File); $audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and (([int]$Matches.n -ge 377 -and [int]$Matches.n -le 392) -or ([int]$Matches.n -ge 394 -and [int]$Matches.n -le 433)) })
if ($terminal.Count -ne 57) { throw 'CP434 terminal set drift' }
foreach ($file in $terminal) { $text = Read-RepoText -Path $file.FullName; if (-not $text.Contains('$cp434Call') -or -not $text.Contains('CP433-to-CP434') -or -not $text.Contains('CP437-to-numerical') -or -not $text.Contains("-Pattern ('Assert-Contains[^\r\n]+CP433-to-' + 'numerical')")) { throw "CP434 terminal propagation drift $($file.Name)" } }; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp435Call' -Description 'CP435 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP434-to-CP435' -Description 'CP434-to-CP435 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP434-to-' + 'numerical') -Description 'stale CP434 numerical interval'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp436Call' -Description 'CP436 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP435-to-CP436' -Description 'CP435-to-CP436 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP435-to-' + 'numerical') -Description 'stale CP435 numerical interval'
$live = @($audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and [int]$Matches.n -ge 421 -and [int]$Matches.n -le 433 })
if ($live.Count -ne 13) { throw 'CP434 live binding set drift' }
$staleLiveBinding = 'current_12' + '6_snapshot_binding|binding_is_12' + '6|(?:binding|calculation_fields|scheduledFields|scheduled|pub calculation_)[^\r\n]{0,240}(?:Count-ne12' + '6|,\s*\\s\*12' + '6)'
foreach ($file in $live) { $text = Read-RepoText -Path $file.FullName; if ($text -notmatch 'current_128_snapshot_binding|binding_is_128|Count-ne128|,\s*\\s\*128' -or $text -match $staleLiveBinding) { throw "CP434 current-128 propagation drift $($file.Name)" } }
$recent = @($audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and [int]$Matches.n -ge 425 -and [int]$Matches.n -le 433 })
if ($recent.Count -ne 9) { throw 'CP434 recent binding set drift' }
foreach ($file in $recent) { Assert-Contains -Path $file.FullName -Pattern "calculation_$stem" -Description 'CP434 recent binding propagation' }
$cp424 = Get-ChildItem $auditRoot -Filter 'cp424-*.ps1' -File
Assert-Contains -Path $cp424.FullName -Pattern 'calculation_heating_mode_guard_else_branch_entry' -Description 'CP424 retains CP433 registration'; Assert-NotContains -Path $cp424.FullName -Pattern "calculation_$stem" -Description 'CP424 excludes CP434 registration'
$manualAudits = @($audits | Where-Object { $_.BaseName -match '^cp(?<n>\d+)-' -and [int]$Matches.n -ge 424 -and [int]$Matches.n -le 433 })
if ($manualAudits.Count -ne 10) { throw 'CP434 manual cumulative audit set drift' }
foreach ($file in $manualAudits) { if (-not (Read-RepoText -Path $file.FullName).Contains('+2169+3638+3009+3458')) { throw "CP434 manual cumulative propagation drift $($file.Name)" } }
$manuals = @(
    [PSCustomObject]@{Path='docs\src\current\current-status.md';Length=484653;Hash='50F57A74E17F958485F954A65718401A85C98CFA69AEDA551E5A3F1FB43D154F'},
    [PSCustomObject]@{Path='docs\src\current\project-contract.md';Length=1436101;Hash='9036713FD9FA7EF67D2469BEA023FAEB12C6B604E33E6CADEB81AADD2E7F8EEF'},
    [PSCustomObject]@{Path='docs\src\porting-map\heat-balance-source-map.md';Length=3900887;Hash='584A46B456D6173AF2E0B5580A4C991A4CD3190DCC89089938FCDF1A34C7A01C'},
    [PSCustomObject]@{Path='docs\src\porting-map\ideal-loads-source-map.md';Length=1440228;Hash='7137976366F5376982066B59400DC6A0C3C5BD640917272C062382804FE7EF8D'},
    [PSCustomObject]@{Path='docs\src\porting-map\zone-air-update-map.md';Length=1639666;Hash='DEAD645FB5C6C9E237E9EFE8721B5FDFCEB17ABCA588060B03C72536EA0EF2EC'}
)
$canonical = $null
foreach ($manual in $manuals) {
    $bytes = [IO.File]::ReadAllBytes($manual.Path)
    if ($bytes.Length -ne $manual.Length + 2169 + 3638 + 3009 + 3458 -or [Array]::IndexOf($bytes,[byte]13) -ne -1 -or $bytes[-1] -ne 10) { throw "CP434 manual byte shape drift $($manual.Path)" }
    $prefix = New-Object byte[] $manual.Length; [Array]::Copy($bytes,$prefix,$manual.Length)
    $suffix = New-Object byte[] 2169; [Array]::Copy($bytes,$manual.Length,$suffix,0,2169)
    if ((Get-Cp434Sha $prefix) -cne $manual.Hash -or (Get-Cp434Sha $suffix) -cne '4DE4406896C56CD7D7343937F1C498DEA840F9CE821160987B1F0098EAAFEBDF') { throw "CP434 manual prefix/suffix drift $($manual.Path)" }
    $text = [Text.UTF8Encoding]::new($false,$true).GetString($suffix)
    if ([regex]::Matches($text,'(?m)^## CP434 heating operating-mode DeadBand assignment$').Count -ne 1) { throw "CP434 manual heading drift $($manual.Path)" }
    if ($null -eq $canonical) { $canonical = $text } elseif ($text -cne $canonical) { throw "CP434 canonical manual mismatch $($manual.Path)" }
}
foreach ($pattern in @('physical-line-2351','line 2352.*first excluded lexical','line 2361.*first excluded executable/control','assign-local-operating-mode-deadband','T434=61','Z434=59','A434=2','S434=2','20/41','one active route in each visibility class','two width-36','CP432 Heat1 plus CP434 Deadband2','first 348 nonterminal fields','361 unique Rust fields','128 `Option<f64>`','four optional comparison bools','four optional enums','489 keys','128 adjacent IEEE sidecars','37/42/57','CP433-to-CP434-to-unchanged-numerical','index 124','current count 125','post-hoc only','private SingleCool','does not feed or mutate','32 algorithms','293 routines','58 state-mapped','235 source-mapped','170 required','372 total','240 public','132 internal','238 development commands')) { if ($canonical -notmatch "(?is)$pattern") { throw "CP434 documentation claim missing $pattern" } }
$inventory = Read-RepoText -Path 'specs\script_inventory.toml'
foreach ($pattern in @('script_count = 375','dev_command_count = 238','unused_script_count = 0','unreachable_count = 0')) { if (-not $inventory.Contains($pattern)) { throw "CP434 inventory missing $pattern" } }
if ([regex]::Matches($inventory,'(?m)^classification = "public"\r?$').Count -ne 240 -or [regex]::Matches($inventory,'(?m)^classification = "internal"\r?$').Count -ne 135) { throw 'CP434 inventory classification drift' }
Assert-Contains -Path 'specs\script_inventory.toml' -Pattern 'cp434-heating-operating-mode-deadband-assignment\.ps1' -Description 'CP434 inventory record'
Assert-Contains -Path 'docs\src\generated\algorithm-ledger.md' -Pattern 'CP434 supersedes only CP433' -Description 'generated algorithm addendum'; Assert-Contains -Path 'docs\src\generated\capability-index.md' -Pattern 'CP434 additionally requires' -Description 'generated capability addendum'
Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 375 \|' -Description 'generated total'; Assert-Contains -Path 'docs\src\generated\script-index.md' -Pattern '\| 135 \|' -Description 'generated internal'
$master = Read-RepoText -Path 'scripts\quality\ideal-loads-structure-audit.ps1'; $leaf = Split-Path -Leaf $audit
$previous = $master.IndexOf('cp433-heating-mode-guard-else-branch-entry.ps1'); $current = $master.IndexOf($leaf); $done = $master.IndexOf('Write-Host "IdealLoads structure audit complete."')
if ($previous -lt 0 -or $current -le $previous -or $done -le $current -or [regex]::Matches($master,[regex]::Escape($leaf)).Count -ne 1) { throw 'CP434 master order/uniqueness drift' }
if ((Get-Content 'scripts\quality\ideal-loads-structure-audit.ps1').Count -ne 4200 -or (Get-Content 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1').Count -ne 1200 -or (Get-Content 'crates\ep_runtime\src\ideal_loads\calc.rs').Count -ne 99 -or (Get-Content 'crates\ep_runtime\src\ideal_loads\init\state\witnesses.rs').Count -ne 274 -or @(Get-Content 'crates\ep_runtime\src\ideal_loads\init\state.rs' | Where-Object { $_ -match '\S' }).Count -ne 380 -or (Get-Content $scheduled).Count -gt 500) { throw 'CP434 fixed cap drift' }
Write-Host 'CP434 heating operating-mode DeadBand assignment structure audit passed.'
}
Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment' -Description 'CP436 recent binding propagation'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern '\$cp437Call' -Description 'CP437 terminal capture'; Assert-Contains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern 'CP436-to-CP437' -Description 'CP436-to-CP437 terminal interval'; Assert-NotContains -Path 'scripts\quality\ideal-loads-structure-audit\cp345-cooling-positive-supply-post-capacity-limit-humidity-ratio-mixed-air-assignment.ps1' -Pattern ('Assert-Contains[^\r\n]+CP436-to-' + 'numerical') -Description 'stale CP436 numerical interval'; Assert-Contains -Path 'crates\ep_runtime\src\ideal_loads\binding.rs' -Pattern 'calculation_heating_outdoor_air_maximum_flow_first_warning_guard' -Description 'CP437 recent binding propagation'
