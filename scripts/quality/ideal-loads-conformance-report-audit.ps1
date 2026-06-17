[CmdletBinding()]
param(
    [string]$CaseId = ""
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Read-RepoText {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
}

function Get-TomlString {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $pattern = '(?m)^\s*' + [regex]::Escape($Key) + '\s*=\s*"(?<value>[^"]+)"\s*$'
    $match = [regex]::Match($Text, $pattern)
    if (-not $match.Success) {
        throw "$Description missing TOML string key: $Key"
    }
    return $match.Groups["value"].Value
}

function Get-TomlBool {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $pattern = '(?m)^\s*' + [regex]::Escape($Key) + '\s*=\s*(?<value>true|false)\s*$'
    $match = [regex]::Match($Text, $pattern)
    if (-not $match.Success) {
        throw "$Description missing TOML bool key: $Key"
    }
    return ($match.Groups["value"].Value -eq "true")
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
}

function Assert-JsonPropertyExists {
    param(
        [Parameter(Mandatory = $true)]$Object,
        [Parameter(Mandatory = $true)][string]$PropertyName,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not ($Object.PSObject.Properties.Name -contains $PropertyName)) {
        throw "$Description missing $PropertyName"
    }
}

function Assert-JsonPropertyEquals {
    param(
        [Parameter(Mandatory = $true)]$Object,
        [Parameter(Mandatory = $true)][string]$PropertyName,
        [Parameter(Mandatory = $true)]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-JsonPropertyExists -Object $Object -PropertyName $PropertyName -Description $Description
    $actual = $Object.$PropertyName
    if ($actual -ne $Expected) {
        throw "$Description $PropertyName mismatch: expected $Expected, got $actual"
    }
}

function Assert-TextContains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Needle,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not $Text.Contains($Needle)) {
        throw "$Description missing text: $Needle"
    }
}

function Assert-ReportFieldEquals {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Field,
        [Parameter(Mandatory = $true)][string]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-TextContains -Text $Text -Needle "${Field}: $Expected" -Description $Description
}

function Get-RepoPath {
    param([Parameter(Mandatory = $true)][string]$Path)

    if ([System.IO.Path]::IsPathRooted($Path)) {
        return $Path
    }

    return Join-Path $RepoRoot ($Path -replace '/', '\')
}

$caseFiles = @(
    Get-ChildItem -LiteralPath "data\conformance_cases" -Directory |
        Where-Object { $_.Name -like "ideal_loads*" } |
        ForEach-Object { Join-Path $_.FullName "case.toml" } |
        Where-Object { Test-Path -LiteralPath $_ -PathType Leaf } |
        Sort-Object
)

$promotedCases = @()
foreach ($caseFile in $caseFiles) {
    $text = Read-RepoText -Path $caseFile
    $id = Get-TomlString -Text $text -Key "id" -Description $caseFile
    if ($CaseId -ne "" -and $id -ne $CaseId) {
        continue
    }

    if (-not (Get-TomlBool -Text $text -Key "conformance_claim" -Description $id)) {
        continue
    }

    $comparisonClass = Get-TomlString -Text $text -Key "comparison_class" -Description $id
    if ($comparisonClass -ne "conformance") {
        throw "$id sets conformance_claim=true without comparison_class=conformance"
    }

    $gateScript = Get-TomlString -Text $text -Key "script" -Description "$id gate"
    if ($gateScript -notmatch '^scripts[\\/]dev\.cmd\s+(?<command>.+)$') {
        throw "$id gate script must use scripts/dev.cmd <command>"
    }
    $gateCommand = $Matches["command"]

    $blocking = Get-TomlBool -Text $text -Key "blocking" -Description "$id gate"
    if (-not $blocking) {
        throw "$id conformance gate must be blocking"
    }

    $reportPath = Get-TomlString -Text $text -Key "path" -Description "$id report"
    if ($reportPath -notmatch 'compare-report\.md$') {
        throw "$id report path must point to compare-report.md"
    }

    $promotedCases += [pscustomobject]@{
        Id = $id
        Command = $gateCommand
        ReportPath = $reportPath
    }
}

if ($CaseId -ne "" -and $promotedCases.Count -eq 0) {
    throw "No promoted IdealLoads conformance case matched CaseId: $CaseId"
}

if ($promotedCases.Count -eq 0) {
    throw "No promoted IdealLoads conformance cases found."
}

$devCmd = Join-Path $RepoRoot "scripts\dev.cmd"
$caseIndex = 0
foreach ($case in $promotedCases) {
    $caseIndex += 1
    Write-Host "[$caseIndex/$($promotedCases.Count)] Running $($case.Id): scripts/dev.cmd $($case.Command)"
    $commandArgs = @($case.Command -split '\s+' | Where-Object { $_ -ne "" })
    & $devCmd @commandArgs
    if ($LASTEXITCODE -ne 0) {
        throw "$($case.Id) gate failed with exit code $LASTEXITCODE"
    }

    $reportPath = Get-RepoPath -Path $case.ReportPath
    $compareRoot = Split-Path -Parent $reportPath
    $summaryPath = Join-Path $compareRoot "compare-summary.json"
    $stageSummaryPath = Join-Path $compareRoot "stage-summary.json"
    $toleranceFailuresPath = Join-Path $compareRoot "tolerance-failures.csv"

    Assert-FileExists -Path $reportPath -Description "$($case.Id) compare report"
    Assert-FileExists -Path $summaryPath -Description "$($case.Id) compare summary"
    Assert-FileExists -Path $stageSummaryPath -Description "$($case.Id) stage summary"
    Assert-FileExists -Path $toleranceFailuresPath -Description "$($case.Id) tolerance failures CSV"

    $reportText = Get-Content -Encoding UTF8 -Raw -LiteralPath $reportPath
    $summary = Get-Content -Encoding UTF8 -Raw -LiteralPath $summaryPath | ConvertFrom-Json
    if ($summary.case_id -ne $case.Id) {
        throw "$($case.Id) summary case_id mismatch: $($summary.case_id)"
    }
    if ($summary.comparison_class -ne "conformance") {
        throw "$($case.Id) summary comparison_class mismatch: $($summary.comparison_class)"
    }
    if ($summary.conformance_claim -ne $true) {
        throw "$($case.Id) summary must set conformance_claim=true"
    }
    if ($summary.status -ne "pass") {
        throw "$($case.Id) summary status must be pass, got $($summary.status)"
    }
    if ($summary.tolerance_failures -ne 0) {
        throw "$($case.Id) summary tolerance_failures must be 0, got $($summary.tolerance_failures)"
    }
    if (($summary.PSObject.Properties.Name -contains "meter_tolerance_failures") -and $summary.meter_tolerance_failures -ne 0) {
        throw "$($case.Id) summary meter_tolerance_failures must be 0, got $($summary.meter_tolerance_failures)"
    }
    $outputRows = @($summary.series)
    if (($summary.PSObject.Properties.Name -contains "series_count") -and $summary.series_count -ne $outputRows.Count) {
        throw "$($case.Id) summary series_count mismatch: expected $($summary.series_count), got $($outputRows.Count)"
    }
    foreach ($row in $outputRows) {
        if ($row.level -notin @("conformance", "diagnostic")) {
            throw "$($case.Id) output row has unexpected level '$($row.level)': $($row.key) / $($row.variable)"
        }
        if ($row.status -ne "pass") {
            throw "$($case.Id) output row did not pass: $($row.key) / $($row.variable) = $($row.status)"
        }
    }
    $meterRows = @()
    if ($summary.PSObject.Properties.Name -contains "meter_series") {
        $meterRows = @($summary.meter_series)
        if (($summary.PSObject.Properties.Name -contains "meter_series_count") -and $summary.meter_series_count -ne $meterRows.Count) {
            throw "$($case.Id) summary meter_series_count mismatch: expected $($summary.meter_series_count), got $($meterRows.Count)"
        }
        foreach ($meterRow in $meterRows) {
            if ($meterRow.level -notin @("conformance", "diagnostic")) {
                throw "$($case.Id) meter row has unexpected level '$($meterRow.level)': $($meterRow.name)"
            }
            if ($meterRow.status -ne "pass") {
                throw "$($case.Id) meter row did not pass: $($meterRow.name) = $($meterRow.status)"
            }
        }
    }
    $conformanceOutputRows = @($outputRows | Where-Object { $_.level -eq "conformance" })
    $diagnosticOutputRows = @($outputRows | Where-Object { $_.level -eq "diagnostic" })
    $conformanceMeterRows = @($meterRows | Where-Object { $_.level -eq "conformance" })
    $diagnosticMeterRows = @($meterRows | Where-Object { $_.level -eq "diagnostic" })
    if (($conformanceOutputRows.Count + $conformanceMeterRows.Count) -eq 0) {
        throw "$($case.Id) summary must include at least one conformance output or meter row"
    }
    if (($diagnosticOutputRows.Count + $diagnosticMeterRows.Count) -eq 0) {
        throw "$($case.Id) summary must include diagnostic rows separated from conformance rows"
    }
    foreach ($propertyName in @(
        "selected_purchased_air_branch",
        "declared_ideal_loads_branch",
        "inactive_branches",
        "source_map_anchor",
        "node_output_timestamp_alignment",
        "node_output_store_type",
        "node_output_state_struct",
        "node_output_update_source",
        "node_output_report_source",
        "zone_demand_source",
        "zone_demand_struct_source",
        "zone_demand_heating_field",
        "zone_demand_heating_sign_convention",
        "zone_demand_cooling_field",
        "zone_demand_cooling_sign_convention",
        "zone_demand_mismatch_classification",
        "zone_demand_fixture_mode",
        "zone_equipment_dispatch_path",
        "zone_equipment_dispatch_validation",
        "zone_equipment_conformance_candidate",
        "zone_equipment_scope",
        "zone_equipment_dispatch_issues",
        "zone_equipment_dispatch_warnings"
    )) {
        Assert-JsonPropertyExists -Object $summary -PropertyName $propertyName -Description "$($case.Id) compare summary"
    }
    Assert-JsonPropertyEquals -Object $summary -PropertyName "source_map_anchor" -Expected "docs/src/porting-map/ideal-loads-source-map.md" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "node_output_timestamp_alignment" -Expected "timestamp" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "node_output_store_type" -Expected "ep_runtime::ResultStore" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "node_output_state_struct" -Expected "ep_runtime::node::IdealLoadsSupplyNodeUpdate" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "node_output_update_source" -Expected "UpdatePurchasedAir" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "node_output_report_source" -Expected "ReportPurchasedAir" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_demand_source" -Expected "EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_demand_struct_source" -Expected "src/EnergyPlus/DataZoneEnergyDemands.hh::ZoneSysEnergyDemand" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_demand_heating_field" -Expected "RemainingOutputReqToHeatSP" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_demand_heating_sign_convention" -Expected "positive W requests heating; non-positive means no active heating request" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_demand_cooling_field" -Expected "RemainingOutputReqToCoolSP" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_demand_cooling_sign_convention" -Expected "negative W requests cooling; non-negative means no active cooling request" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_demand_mismatch_classification" -Expected "upstream_zone_heat_balance_input" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_demand_fixture_mode" -Expected "source-order-oracle-demand-input" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_equipment_dispatch_path" -Expected "ZoneEquipmentManager::ManageZoneEquipment -> SimZoneEquipment -> ZoneEquipType::PurchasedAir -> PurchasedAirManager::SimPurchasedAir" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_equipment_dispatch_validation" -Expected "pass" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_equipment_conformance_candidate" -Expected "pass" -Description "$($case.Id) compare summary"
    Assert-JsonPropertyEquals -Object $summary -PropertyName "zone_equipment_scope" -Expected "single-zone-single-equipment" -Description "$($case.Id) compare summary"
    if (@($summary.zone_equipment_dispatch_issues).Count -ne 0) {
        throw "$($case.Id) compare summary zone_equipment_dispatch_issues must be empty"
    }
    if (@($summary.zone_equipment_dispatch_warnings).Count -ne 0) {
        throw "$($case.Id) compare summary zone_equipment_dispatch_warnings must be empty"
    }
    $expectedSourceOrderWrapper = if ($summary.selected_purchased_air_branch -eq "outdoor_air") {
        "ep_runtime::ideal_loads::sim_purchased_air_outdoor_air_compat"
    }
    else {
        "ep_runtime::ideal_loads::sim_purchased_air_compat"
    }
    $reportFieldExpectations = [ordered]@{
        "case_id" = $case.Id
        "comparison_class" = "conformance"
        "conformance_claim" = "true"
        "status" = "pass"
        "source_order_wrapper" = $expectedSourceOrderWrapper
        "zone_equipment_dispatch_path" = "ZoneEquipmentManager::ManageZoneEquipment -> SimZoneEquipment -> ZoneEquipType::PurchasedAir -> PurchasedAirManager::SimPurchasedAir"
        "zone_equipment_dispatch_validation" = "pass"
        "zone_equipment_conformance_candidate" = "pass"
        "zone_equipment_scope" = "single-zone-single-equipment"
        "zone_equipment_dispatch_issues" = "none"
        "zone_equipment_dispatch_warnings" = "none"
        "selected_purchased_air_branch" = [string]$summary.selected_purchased_air_branch
        "declared_ideal_loads_branch" = [string]$summary.declared_ideal_loads_branch
        "inactive_branches" = (@($summary.inactive_branches) -join ", ")
        "source_map_anchor" = "docs/src/porting-map/ideal-loads-source-map.md"
        "node_output_timestamp_alignment" = "timestamp"
        "node_output_store_type" = "ep_runtime::ResultStore"
        "node_output_state_struct" = "ep_runtime::node::IdealLoadsSupplyNodeUpdate"
        "node_output_update_source" = "UpdatePurchasedAir"
        "node_output_report_source" = "ReportPurchasedAir"
        "purchased_air_source_order" = "GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir"
        "zone_demand_source" = "EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs"
        "zone_demand_struct_source" = "src/EnergyPlus/DataZoneEnergyDemands.hh::ZoneSysEnergyDemand"
        "zone_demand_heating_field" = "RemainingOutputReqToHeatSP"
        "zone_demand_heating_sign_convention" = "positive W requests heating; non-positive means no active heating request"
        "zone_demand_cooling_field" = "RemainingOutputReqToCoolSP"
        "zone_demand_cooling_sign_convention" = "negative W requests cooling; non-negative means no active cooling request"
        "zone_demand_mismatch_classification" = "upstream_zone_heat_balance_input"
        "zone_demand_fixture_mode" = "source-order-oracle-demand-input"
    }
    foreach ($reportField in $reportFieldExpectations.GetEnumerator()) {
        Assert-ReportFieldEquals -Text $reportText -Field $reportField.Key -Expected $reportField.Value -Description "$($case.Id) compare report"
    }

    $stageSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath $stageSummaryPath | ConvertFrom-Json
    foreach ($propertyName in @(
        "stages",
        "purchased_air_stages",
        "selected_purchased_air_branch",
        "declared_ideal_loads_branch",
        "inactive_branches",
        "source_map_anchor",
        "node_output_timestamp_alignment",
        "node_output_store_type",
        "node_output_state_struct",
        "node_output_update_source",
        "node_output_report_source",
        "zone_demand_source",
        "zone_demand_struct_source",
        "zone_demand_heating_field",
        "zone_demand_heating_sign_convention",
        "zone_demand_cooling_field",
        "zone_demand_cooling_sign_convention",
        "zone_demand_mismatch_classification",
        "zone_demand_fixture_mode",
        "zone_equipment_dispatch_path",
        "zone_equipment_dispatch_validation",
        "zone_equipment_conformance_candidate",
        "zone_equipment_scope",
        "zone_equipment_dispatch_issues",
        "zone_equipment_dispatch_warnings"
    )) {
        Assert-JsonPropertyExists -Object $stageSummary -PropertyName $propertyName -Description "$($case.Id) stage summary"
    }

    $expectedZoneEquipmentRoutines = @(
        "ManageZoneEquipment",
        "SimZoneEquipment",
        "SimPurchasedAir"
    )
    $zoneEquipmentStages = @($stageSummary.stages)
    if ($zoneEquipmentStages.Count -ne $expectedZoneEquipmentRoutines.Count) {
        throw "$($case.Id) expected $($expectedZoneEquipmentRoutines.Count) ZoneEquipment stages, got $($zoneEquipmentStages.Count)"
    }
    for ($stageIndex = 0; $stageIndex -lt $expectedZoneEquipmentRoutines.Count; $stageIndex++) {
        $actualRoutine = $zoneEquipmentStages[$stageIndex].source_routine
        if ($actualRoutine -ne $expectedZoneEquipmentRoutines[$stageIndex]) {
            throw "$($case.Id) ZoneEquipment source-order mismatch at ${stageIndex}: expected $($expectedZoneEquipmentRoutines[$stageIndex]), got $actualRoutine"
        }
    }

    $expectedPurchasedAirRoutines = @(
        "GetPurchasedAir",
        "InitPurchasedAir",
        "CalcPurchAirLoads",
        "UpdatePurchasedAir",
        "ReportPurchasedAir"
    )
    $purchasedAirStages = @($stageSummary.purchased_air_stages)
    if ($purchasedAirStages.Count -ne $expectedPurchasedAirRoutines.Count) {
        throw "$($case.Id) expected $($expectedPurchasedAirRoutines.Count) PurchasedAir stages, got $($purchasedAirStages.Count)"
    }
    for ($stageIndex = 0; $stageIndex -lt $expectedPurchasedAirRoutines.Count; $stageIndex++) {
        $actualRoutine = $purchasedAirStages[$stageIndex].source_routine
        if ($actualRoutine -ne $expectedPurchasedAirRoutines[$stageIndex]) {
            throw "$($case.Id) PurchasedAir source-order mismatch at ${stageIndex}: expected $($expectedPurchasedAirRoutines[$stageIndex]), got $actualRoutine"
        }
    }
    if ($stageSummary.source_map_anchor -ne "docs/src/porting-map/ideal-loads-source-map.md") {
        throw "$($case.Id) stage summary source_map_anchor mismatch: $($stageSummary.source_map_anchor)"
    }
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "node_output_timestamp_alignment" -Expected "timestamp" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "node_output_store_type" -Expected "ep_runtime::ResultStore" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "node_output_state_struct" -Expected "ep_runtime::node::IdealLoadsSupplyNodeUpdate" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "node_output_update_source" -Expected "UpdatePurchasedAir" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "node_output_report_source" -Expected "ReportPurchasedAir" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_demand_source" -Expected "EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_demand_struct_source" -Expected "src/EnergyPlus/DataZoneEnergyDemands.hh::ZoneSysEnergyDemand" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_demand_heating_field" -Expected "RemainingOutputReqToHeatSP" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_demand_heating_sign_convention" -Expected "positive W requests heating; non-positive means no active heating request" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_demand_cooling_field" -Expected "RemainingOutputReqToCoolSP" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_demand_cooling_sign_convention" -Expected "negative W requests cooling; non-negative means no active cooling request" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_demand_mismatch_classification" -Expected "upstream_zone_heat_balance_input" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_demand_fixture_mode" -Expected "source-order-oracle-demand-input" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_equipment_dispatch_path" -Expected "ZoneEquipmentManager::ManageZoneEquipment -> SimZoneEquipment -> ZoneEquipType::PurchasedAir -> PurchasedAirManager::SimPurchasedAir" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_equipment_dispatch_validation" -Expected "pass" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_equipment_conformance_candidate" -Expected "pass" -Description "$($case.Id) stage summary"
    Assert-JsonPropertyEquals -Object $stageSummary -PropertyName "zone_equipment_scope" -Expected "single-zone-single-equipment" -Description "$($case.Id) stage summary"
    if (@($stageSummary.zone_equipment_dispatch_issues).Count -ne 0) {
        throw "$($case.Id) stage summary zone_equipment_dispatch_issues must be empty"
    }
    if (@($stageSummary.zone_equipment_dispatch_warnings).Count -ne 0) {
        throw "$($case.Id) stage summary zone_equipment_dispatch_warnings must be empty"
    }
}

Write-Host "IdealLoads conformance report audit complete."
Write-Host "  promoted_ideal_loads_cases_run: $($promotedCases.Count)"
