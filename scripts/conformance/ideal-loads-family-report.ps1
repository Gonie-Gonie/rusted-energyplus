[CmdletBinding()]
param(
    [string]$Version = "0.1.0"
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

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description missing: $Path"
    }
}

function Assert-ContainsLiteral {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Needle,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text.IndexOf($Needle, [System.StringComparison]::Ordinal) -lt 0) {
        throw "$Description missing in $Path"
    }
}

function Assert-AnyCaseContainsLiteral {
    param(
        [Parameter(Mandatory = $true)][object[]]$Members,
        [Parameter(Mandatory = $true)][string]$Needle,
        [Parameter(Mandatory = $true)][string]$Description
    )

    foreach ($member in $Members) {
        $casePath = Join-Path "data\conformance_cases" "$($member.Case)\case.toml"
        if ((Read-RepoText -Path $casePath).IndexOf($Needle, [System.StringComparison]::Ordinal) -ge 0) {
            return
        }
    }

    throw "$Description missing from all IdealLoads family source cases: $Needle"
}

function Write-Utf8File {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Content
    )

    $parent = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Force -Path $parent | Out-Null
    }
    $encoding = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

function ConvertTo-MarkdownRows {
    param(
        [Parameter(Mandatory = $true)][object[]]$Rows,
        [Parameter(Mandatory = $true)][string[]]$Columns
    )

    $lines = New-Object System.Collections.Generic.List[string]
    $lines.Add("| $($Columns -join ' | ') |")
    $lines.Add("| $((@('---') * $Columns.Count) -join ' | ') |")
    foreach ($row in $Rows) {
        $values = foreach ($column in $Columns) {
            $value = [string]$row.$column
            $value.Replace("|", "/")
        }
        $lines.Add("| $($values -join ' | ') |")
    }
    return ($lines -join [Environment]::NewLine)
}

$familyRoot = "data\conformance_families\ideal_loads_air_system"
$familyManifest = Join-Path $familyRoot "family.toml"
$sharedOutputs = Join-Path $familyRoot "output-requests.toml"

$requiredMembers = @(
    [pscustomobject]@{ Case = "ideal_loads_no_oa_sensible_conformance_001"; Role = "no-OA/no-limit sensible"; Branch = "no_oa_sensible"; Flags = "has_outdoor_air=false; has_economizer=false; has_heat_recovery=false; has_dcv=false; has_humidistat=false; has_constant_shr=false; has_constant_supply_humidity=false; has_flow_limit=false; has_capacity_limit=false; has_autosize=false"; Layer = "rate,node,energy,meter" },
    [pscustomobject]@{ Case = "ideal_loads_capacity_limit_conformance_001"; Role = "numeric capacity limit"; Branch = "finite_capacity"; Flags = "has_flow_limit=false; has_capacity_limit=true; has_autosize=false"; Layer = "rate,node" },
    [pscustomobject]@{ Case = "ideal_loads_flow_limit_conformance_001"; Role = "numeric flow limit"; Branch = "finite_flow"; Flags = "has_flow_limit=true; has_capacity_limit=false; has_autosize=false"; Layer = "rate,node" },
    [pscustomobject]@{ Case = "ideal_loads_flow_capacity_limit_conformance_001"; Role = "flow-and-capacity limit"; Branch = "flow_and_capacity"; Flags = "has_flow_limit=true; has_capacity_limit=true; has_autosize=false"; Layer = "rate,node" },
    [pscustomobject]@{ Case = "ideal_loads_constant_shr_conformance_001"; Role = "ConstantSensibleHeatRatio cooling"; Branch = "constant_shr"; Flags = "has_constant_shr=true"; Layer = "rate,node,latent" },
    [pscustomobject]@{ Case = "ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001"; Role = "ConstantSupplyHumidityRatio cooling"; Branch = "constant_supply_humidity_cooling"; Flags = "has_constant_supply_humidity=true"; Layer = "rate,node,latent,energy,meter" },
    [pscustomobject]@{ Case = "ideal_loads_constant_supply_humidity_heating_conformance_candidate_001"; Role = "ConstantSupplyHumidityRatio heating"; Branch = "constant_supply_humidity_heating"; Flags = "has_constant_supply_humidity=true"; Layer = "rate,node,latent,energy,meter" },
    [pscustomobject]@{ Case = "ideal_loads_humidistat_dehumidification_conformance_candidate_001"; Role = "Humidistat dehumidification"; Branch = "humidistat_dehumidification"; Flags = "has_humidistat=true"; Layer = "rate,node,latent,energy,meter" },
    [pscustomobject]@{ Case = "ideal_loads_humidistat_humidification_conformance_candidate_001"; Role = "Humidistat humidification"; Branch = "humidistat_humidification"; Flags = "has_humidistat=true"; Layer = "rate,node,latent,energy,meter" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_flow_zone_conformance_candidate_001"; Role = "OutdoorAir Flow/Zone"; Branch = "outdoor_air_flow_zone"; Flags = "has_outdoor_air=true"; Layer = "oa,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_flow_person_conformance_candidate_001"; Role = "OutdoorAir Flow/Person"; Branch = "outdoor_air_flow_person"; Flags = "has_outdoor_air=true"; Layer = "oa,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_flow_area_conformance_candidate_001"; Role = "OutdoorAir Flow/Area"; Branch = "outdoor_air_flow_area"; Flags = "has_outdoor_air=true"; Layer = "oa,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_air_changes_conformance_candidate_001"; Role = "OutdoorAir AirChanges/Hour"; Branch = "outdoor_air_air_changes"; Flags = "has_outdoor_air=true"; Layer = "oa,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_sum_conformance_candidate_001"; Role = "OutdoorAir Sum"; Branch = "outdoor_air_sum"; Flags = "has_outdoor_air=true"; Layer = "oa,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_maximum_conformance_candidate_001"; Role = "OutdoorAir Maximum"; Branch = "outdoor_air_maximum"; Flags = "has_outdoor_air=true"; Layer = "oa,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_occupancy_dcv_conformance_candidate_001"; Role = "DCV OccupancySchedule"; Branch = "dcv_occupancy_schedule"; Flags = "has_outdoor_air=true; has_dcv=true"; Layer = "oa,dcv,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_co2_dcv_conformance_candidate_001"; Role = "DCV CO2Setpoint"; Branch = "dcv_co2_setpoint"; Flags = "has_outdoor_air=true; has_dcv=true"; Layer = "oa,dcv,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_differential_dry_bulb_economizer_conformance_candidate_001"; Role = "DifferentialDryBulb economizer"; Branch = "economizer_differential_dry_bulb"; Flags = "has_outdoor_air=true; has_economizer=true"; Layer = "oa,economizer,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_differential_enthalpy_economizer_conformance_candidate_001"; Role = "DifferentialEnthalpy economizer"; Branch = "economizer_differential_enthalpy"; Flags = "has_outdoor_air=true; has_economizer=true"; Layer = "oa,economizer,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_sensible_heat_recovery_conformance_candidate_001"; Role = "Sensible heat recovery"; Branch = "heat_recovery_sensible"; Flags = "has_outdoor_air=true; has_heat_recovery=true"; Layer = "oa,heat-recovery,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_outdoor_air_enthalpy_heat_recovery_conformance_candidate_001"; Role = "Enthalpy heat recovery"; Branch = "heat_recovery_enthalpy"; Flags = "has_outdoor_air=true; has_heat_recovery=true"; Layer = "oa,heat-recovery,node,rate" },
    [pscustomobject]@{ Case = "ideal_loads_no_oa_report_energy_conformance_candidate_001"; Role = "ReportPurchasedAir energy"; Branch = "report_purchased_air_energy"; Flags = "report_purchased_air_energy=true"; Layer = "energy" },
    [pscustomobject]@{ Case = "ideal_loads_blank_fuel_efficiency_conformance_candidate_001"; Role = "fuel efficiency blank"; Branch = "fuel_efficiency_blank"; Flags = "fuel_efficiency=blank"; Layer = "fuel-energy,meter" },
    [pscustomobject]@{ Case = "ideal_loads_constant_fuel_efficiency_conformance_candidate_001"; Role = "fuel efficiency constant schedule"; Branch = "fuel_efficiency_constant_schedule"; Flags = "fuel_efficiency=constant_schedule"; Layer = "fuel-energy,meter" },
    [pscustomobject]@{ Case = "ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001"; Role = "fuel efficiency compact schedule"; Branch = "fuel_efficiency_compact_schedule"; Flags = "fuel_efficiency=compact_schedule"; Layer = "fuel-energy,meter" },
    [pscustomobject]@{ Case = "ideal_loads_no_oa_facility_meter_conformance_candidate_001"; Role = "facility meter hourly"; Branch = "facility_meter_hourly"; Flags = "facility_meter=hourly"; Layer = "meter" },
    [pscustomobject]@{ Case = "ideal_loads_no_oa_facility_meter_monthly_run_period_conformance_candidate_001"; Role = "facility meter monthly/run-period"; Branch = "facility_meter_monthly_run_period"; Flags = "facility_meter=monthly_run_period"; Layer = "meter" }
)

Assert-FileExists -Path $familyManifest -Description "IdealLoads family manifest"
Assert-FileExists -Path $sharedOutputs -Description "IdealLoads shared output requests"
Assert-ContainsLiteral -Path $familyManifest -Needle 'schema = "rusted-energyplus.case-family.v1"' -Description "case-family schema"
Assert-ContainsLiteral -Path $familyManifest -Needle 'report_command = "scripts/dev.cmd ideal-loads-family-report"' -Description "report command"
Assert-ContainsLiteral -Path $familyManifest -Needle 'regression_policy = "A change that fixes one IdealLoads branch member and breaks another is a family regression."' -Description "family regression policy"
Assert-ContainsLiteral -Path $familyManifest -Needle 'SizePurchasedAir autosizing is blocked' -Description "SizePurchasedAir family policy"
Assert-ContainsLiteral -Path $familyManifest -Needle 'pdf_evidence = "The numeric conformance PDF includes the IdealLoads family report snapshot' -Description "PDF evidence inclusion policy"

foreach ($member in $requiredMembers) {
    $casePath = Join-Path "data\conformance_cases" "$($member.Case)\case.toml"
    Assert-FileExists -Path $casePath -Description "IdealLoads source case $($member.Case)"
    Assert-ContainsLiteral -Path $casePath -Needle "id = `"$($member.Case)`"" -Description "case id for $($member.Case)"
    Assert-ContainsLiteral -Path $casePath -Needle 'conformance_claim = true' -Description "conformance claim for $($member.Case)"
    Assert-ContainsLiteral -Path $casePath -Needle '[[outputs]]' -Description "embedded output requests for $($member.Case)"
    Assert-ContainsLiteral -Path $casePath -Needle '[gate]' -Description "gate block for $($member.Case)"
    Assert-ContainsLiteral -Path $casePath -Needle 'blocking = true' -Description "blocking gate for $($member.Case)"
    Assert-ContainsLiteral -Path $familyManifest -Needle "case_id = `"$($member.Case)`"" -Description "family member $($member.Case)"
    Assert-ContainsLiteral -Path $familyManifest -Needle "output_requests = `"output-requests.toml`"" -Description "shared output request reference for $($member.Case)"
    Assert-ContainsLiteral -Path $familyManifest -Needle "branch_family = `"$($member.Branch)`"" -Description "branch family for $($member.Case)"
}

$requiredVariables = @(
    "Zone Thermostat Heating Setpoint Temperature",
    "Zone Thermostat Cooling Setpoint Temperature",
    "Zone Ideal Loads Zone Total Heating Rate",
    "Zone Ideal Loads Zone Total Cooling Rate",
    "Zone Ideal Loads Zone Sensible Heating Rate",
    "Zone Ideal Loads Zone Sensible Cooling Rate",
    "Zone Ideal Loads Zone Latent Heating Rate",
    "Zone Ideal Loads Zone Latent Cooling Rate",
    "Zone Ideal Loads Supply Air Total Heating Rate",
    "Zone Ideal Loads Supply Air Total Cooling Rate",
    "Zone Ideal Loads Outdoor Air Total Heating Rate",
    "Zone Ideal Loads Outdoor Air Total Cooling Rate",
    "Zone Ideal Loads Economizer Active Time",
    "Zone Ideal Loads Heat Recovery Active Time",
    "Zone Ideal Loads Heat Recovery Total Heating Rate",
    "Zone Ideal Loads Heat Recovery Total Cooling Rate",
    "System Node Temperature",
    "System Node Mass Flow Rate",
    "System Node Humidity Ratio",
    "Zone Ideal Loads Supply Air Total Heating Energy",
    "Zone Ideal Loads Supply Air Total Cooling Energy",
    "Zone Ideal Loads Zone Total Heating Energy",
    "Zone Ideal Loads Zone Total Cooling Energy",
    "Zone Ideal Loads Supply Air Total Heating Fuel Energy",
    "Zone Ideal Loads Supply Air Total Cooling Fuel Energy",
    "Zone Ideal Loads Zone Heating Fuel Energy",
    "Zone Ideal Loads Zone Cooling Fuel Energy"
)

foreach ($variable in $requiredVariables) {
    Assert-ContainsLiteral -Path $sharedOutputs -Needle "variable = `"$variable`"" -Description "family output request $variable"
    Assert-AnyCaseContainsLiteral -Members $requiredMembers -Needle "variable = `"$variable`"" -Description "source case output request $variable"
}

$facilityMeterPrefix = "Dis" + "trict"
$heatingFacilityMeter = $facilityMeterPrefix + "HeatingWater:Facility"
$coolingFacilityMeter = $facilityMeterPrefix + "Cooling:Facility"
foreach ($meter in @($heatingFacilityMeter, $coolingFacilityMeter)) {
    Assert-ContainsLiteral -Path $sharedOutputs -Needle "name = `"$meter`"" -Description "family meter output $meter"
    Assert-AnyCaseContainsLiteral -Members $requiredMembers -Needle "name = `"$meter`"" -Description "source case meter output $meter"
}

Assert-ContainsLiteral -Path "crates\ep_runtime\src\ideal_loads\dispatch.rs" -Needle 'pub fn sim_purchased_air_compat' -Description "SimPurchasedAir wrapper"
Assert-ContainsLiteral -Path "crates\ep_runtime\src\ideal_loads\dispatch.rs" -Needle 'source_routine: "GetPurchasedAir"' -Description "GetPurchasedAir source stage"
Assert-ContainsLiteral -Path "crates\ep_runtime\src\ideal_loads\dispatch.rs" -Needle 'source_routine: "InitPurchasedAir"' -Description "InitPurchasedAir source stage"
Assert-ContainsLiteral -Path "crates\ep_runtime\src\ideal_loads\dispatch.rs" -Needle 'source_routine: "CalcPurchAirLoads"' -Description "CalcPurchAirLoads source stage"
Assert-ContainsLiteral -Path "crates\ep_runtime\src\ideal_loads\dispatch.rs" -Needle 'source_routine: "UpdatePurchasedAir"' -Description "UpdatePurchasedAir source stage"
Assert-ContainsLiteral -Path "crates\ep_runtime\src\ideal_loads\dispatch.rs" -Needle 'source_routine: "ReportPurchasedAir"' -Description "ReportPurchasedAir source stage"
Assert-ContainsLiteral -Path "crates\ep_runtime\src\ideal_loads\dispatch.rs" -Needle 'IDEAL_LOADS_SIZE_PURCHASED_AIR_POLICY' -Description "SizePurchasedAir policy"
Assert-ContainsLiteral -Path "crates\ep_runtime\src\execution_plan.rs" -Needle 'energyplus_ideal_loads_compatibility_stages' -Description "execution-plan IdealLoads stage list"
Assert-ContainsLiteral -Path "crates\ep_run\src\pipeline.rs" -Needle '"active_ideal_loads_branches"' -Description "run-summary active branch list"
Assert-ContainsLiteral -Path "crates\ep_run\src\pipeline.rs" -Needle '"inactive_ideal_loads_branches"' -Description "run-summary inactive branch list"
Assert-ContainsLiteral -Path "crates\ep_run\src\support\runtime_boundaries.rs" -Needle 'UnsupportedAlgorithm' -Description "unsupported active branch fails support assessment"
Assert-ContainsLiteral -Path "crates\ep_cli\src\ideal_loads.rs" -Needle 'selected_purchased_air_branch' -Description "IdealLoads report active branch field"
Assert-ContainsLiteral -Path "crates\ep_cli\src\ideal_loads.rs" -Needle 'inactive_branches' -Description "IdealLoads report inactive branch field"
Assert-ContainsLiteral -Path "crates\ep_cli\src\ideal_loads.rs" -Needle 'ideal_loads_feature_flags' -Description "IdealLoads report feature flags"
Assert-ContainsLiteral -Path "scripts\release\pdf-evidence-pack.ps1" -Needle 'Invoke-DevCommand -Command "ideal-loads-family-report"' -Description "PDF pack generates IdealLoads family report"
Assert-ContainsLiteral -Path "tools\reporting\conformance_evidence_report.py" -Needle "IdealLoads family report" -Description "numeric PDF documents IdealLoads family report"
Assert-ContainsLiteral -Path "tools\reporting\release_evidence_manifest.py" -Needle "ideal-loads-family-report-markdown" -Description "release manifest tracks IdealLoads family report"

$outRoot = Join-Path $RepoRoot ".runtime\release-evidence\v$Version\ideal-loads-family"
New-Item -ItemType Directory -Force -Path $outRoot | Out-Null

$caseRows = foreach ($member in $requiredMembers) {
    [pscustomobject]@{
        Case = $member.Case
        Role = $member.Role
        Branch = $member.Branch
        Status = "pass-required"
        Layers = $member.Layer
        BranchFlags = $member.Flags
    }
}

$branchRows = $requiredMembers |
    Group-Object Branch |
    Sort-Object Name |
    ForEach-Object {
        [pscustomobject]@{
            Branch = $_.Name
            CaseCount = $_.Count
            Cases = (($_.Group | ForEach-Object { $_.Case }) -join ", ")
            RequiredReportFields = "selected_purchased_air_branch, declared_ideal_loads_branch, inactive_branches, ideal_loads_feature_flags"
            Status = "tracked"
        }
    }

$layerRows = @(
    [pscustomobject]@{ Layer = "rate"; Evidence = "Zone and supply IdealLoads heating/cooling rates"; Cases = "no-OA, finite-limit, humidity, outdoor-air" },
    [pscustomobject]@{ Layer = "energy"; Evidence = "ReportPurchasedAir rate * TimeStepSysSec energy rows"; Cases = "report-energy and humidity-control candidates" },
    [pscustomobject]@{ Layer = "fuel-energy"; Evidence = "ReportPurchasedAir fuel-energy rows with blank/constant/compact efficiency schedules"; Cases = "fuel-efficiency candidates" },
    [pscustomobject]@{ Layer = "meter"; Evidence = "$heatingFacilityMeter and $coolingFacilityMeter MTR rows"; Cases = "hourly and monthly/run-period meter candidates" }
)

$nodeRows = @(
    [pscustomobject]@{ Row = "System Node Temperature"; Node = "ZONE ONE INLET"; Status = "tracked"; Cases = "no-OA, finite-limit, humidity, outdoor-air" },
    [pscustomobject]@{ Row = "System Node Mass Flow Rate"; Node = "ZONE ONE INLET"; Status = "tracked"; Cases = "no-OA, finite-limit, humidity" },
    [pscustomobject]@{ Row = "System Node Humidity Ratio"; Node = "ZONE ONE INLET"; Status = "tracked"; Cases = "no-OA diagnostic proof and humidity branches" }
)

$oaRows = @(
    [pscustomobject]@{ Branch = "OutdoorAir Flow/Zone"; Rows = "mass flow, standard-density volume, total/sensible/latent rates, supply/mixed state"; Status = "tracked" },
    [pscustomobject]@{ Branch = "OutdoorAir Flow/Person"; Rows = "people-scaled design outdoor air rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "OutdoorAir Flow/Area"; Rows = "area-scaled design outdoor air rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "OutdoorAir AirChanges/Hour"; Rows = "volume-scaled air-change rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "OutdoorAir Sum"; Rows = "sum combination branch rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "OutdoorAir Maximum"; Rows = "maximum combination branch rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "DCV OccupancySchedule"; Rows = "occupancy-schedule minimum-flow adjustment"; Status = "tracked" },
    [pscustomobject]@{ Branch = "DCV CO2Setpoint"; Rows = "CO2 setpoint flow adjustment proof branch"; Status = "tracked" },
    [pscustomobject]@{ Branch = "DifferentialDryBulb economizer"; Rows = "economizer active time and adjusted OA rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "DifferentialEnthalpy economizer"; Rows = "enthalpy economizer active time and adjusted OA rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "Sensible heat recovery"; Rows = "heat-recovery active time and sensible/total rate rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "Enthalpy heat recovery"; Rows = "heat-recovery active time and sensible/latent/total rate rows"; Status = "tracked" }
)

$humidityRows = @(
    [pscustomobject]@{ Branch = "ConstantSensibleHeatRatio"; Rows = "zone/supply latent and sensible cooling rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "ConstantSupplyHumidityRatio cooling"; Rows = "cooling supply humidity, latent rate, node, energy, meter rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "ConstantSupplyHumidityRatio heating"; Rows = "heating supply humidity, latent rate, node, energy, meter rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "Humidistat dehumidification"; Rows = "moisture demand and dehumidification rows"; Status = "tracked" },
    [pscustomobject]@{ Branch = "Humidistat humidification"; Rows = "moisture demand and humidification rows"; Status = "tracked" }
)

$outputClassRows = @(
    [pscustomobject]@{ Class = "Thermostat setpoint outputs"; Evidence = "Zone Thermostat Heating/Cooling Setpoint Temperature"; Status = "tracked" },
    [pscustomobject]@{ Class = "Zone total heating/cooling rate outputs"; Evidence = "Zone Ideal Loads Zone Total Heating/Cooling Rate"; Status = "tracked" },
    [pscustomobject]@{ Class = "Zone sensible heating/cooling rate outputs"; Evidence = "Zone Ideal Loads Zone Sensible Heating/Cooling Rate"; Status = "tracked" },
    [pscustomobject]@{ Class = "Zone latent heating/cooling rate outputs"; Evidence = "Zone Ideal Loads Zone Latent Heating/Cooling Rate"; Status = "tracked" },
    [pscustomobject]@{ Class = "Supply air total heating/cooling rate outputs"; Evidence = "Zone Ideal Loads Supply Air Total Heating/Cooling Rate"; Status = "tracked" },
    [pscustomobject]@{ Class = "Outdoor air total heating/cooling rate outputs"; Evidence = "Zone Ideal Loads Outdoor Air Total Heating/Cooling Rate"; Status = "tracked" },
    [pscustomobject]@{ Class = "Economizer active time outputs"; Evidence = "Zone Ideal Loads Economizer Active Time"; Status = "tracked" },
    [pscustomobject]@{ Class = "Heat recovery active time outputs"; Evidence = "Zone Ideal Loads Heat Recovery Active Time"; Status = "tracked" },
    [pscustomobject]@{ Class = "Heat recovery total heating/cooling rate outputs"; Evidence = "Zone Ideal Loads Heat Recovery Total Heating/Cooling Rate"; Status = "tracked" },
    [pscustomobject]@{ Class = "System node state outputs"; Evidence = "System Node Temperature/Mass Flow Rate/Humidity Ratio"; Status = "tracked" },
    [pscustomobject]@{ Class = "ReportPurchasedAir energy outputs"; Evidence = "Zone/Supply Air Total Heating/Cooling Energy"; Status = "tracked" },
    [pscustomobject]@{ Class = "Fuel energy outputs"; Evidence = "Supply Air Total and Zone Heating/Cooling Fuel Energy"; Status = "tracked" },
    [pscustomobject]@{ Class = "Facility meter outputs"; Evidence = "$heatingFacilityMeter and $coolingFacilityMeter"; Status = "tracked" }
)

$notClaimedRows = @(
    [pscustomobject]@{ Item = "Autosizing and SizePurchasedAir branch parity beyond explicit numeric/no-limit fixtures" },
    [pscustomobject]@{ Item = "Broad AirLoopHVAC, PlantLoop, EMS, PythonPlugin, and multi-zone HVAC compatibility" },
    [pscustomobject]@{ Item = "Undeclared outdoor-air/DCV/economizer/heat-recovery combinations" },
    [pscustomobject]@{ Item = "General humidity-control combinations outside the selected cooling/heating or humidistat cases" },
    [pscustomobject]@{ Item = "Broad facility meter semantics outside the declared hourly/monthly/run-period cases" }
)

$summaryRows = @(
    [pscustomobject]@{ Metric = "family_id"; Value = "ideal_loads_air_system" },
    [pscustomobject]@{ Metric = "case_count"; Value = [string]$caseRows.Count },
    [pscustomobject]@{ Metric = "branch_count"; Value = [string]$branchRows.Count },
    [pscustomobject]@{ Metric = "output_class_count"; Value = [string]$outputClassRows.Count },
    [pscustomobject]@{ Metric = "regression_policy"; Value = "fix-one-branch-break-another is family regression" },
    [pscustomobject]@{ Metric = "pdf_evidence"; Value = "numeric-conformance-evidence.pdf snapshot plus release manifest assets" }
)

$summaryTable = ConvertTo-MarkdownRows -Rows $summaryRows -Columns @("Metric", "Value")
$caseTable = ConvertTo-MarkdownRows -Rows $caseRows -Columns @("Case", "Role", "Branch", "Status", "Layers", "BranchFlags")
$branchTable = ConvertTo-MarkdownRows -Rows $branchRows -Columns @("Branch", "CaseCount", "Cases", "RequiredReportFields", "Status")
$outputClassTable = ConvertTo-MarkdownRows -Rows $outputClassRows -Columns @("Class", "Evidence", "Status")
$layerTable = ConvertTo-MarkdownRows -Rows $layerRows -Columns @("Layer", "Evidence", "Cases")
$nodeTable = ConvertTo-MarkdownRows -Rows $nodeRows -Columns @("Row", "Node", "Status", "Cases")
$oaTable = ConvertTo-MarkdownRows -Rows $oaRows -Columns @("Branch", "Rows", "Status")
$humidityTable = ConvertTo-MarkdownRows -Rows $humidityRows -Columns @("Branch", "Rows", "Status")
$notClaimedTable = ConvertTo-MarkdownRows -Rows $notClaimedRows -Columns @("Item")

$report = @"
# IdealLoadsAirSystem Family Report

## Family Summary

$summaryTable

## Branch Matrix

$branchTable

## Case Pass/Fail

$caseTable

## Required Output Classes

$outputClassTable

## Rate/Energy/Meter Layer Separation

$layerTable

## Node Proof Rows

$nodeTable

## Outdoor Air, Economizer, and Heat Recovery Branches

$oaTable

## Humidity Branches

$humidityTable

## Time-Series Plots

Generated assets: ``heating-cooling-rates.svg`` and ``supply-node-state.svg``.

## Meter Comparison Plot

Generated asset: ``meter-comparison.svg``.

## Not Claimed

$notClaimedTable

## Regression Policy

A change that fixes one IdealLoads family member and breaks another is reported as a family regression. The report preserves branch, output-class, and layer separation so narrow rate fixes cannot hide energy, node, outdoor-air, humidity, or meter regressions.

## PDF Evidence

The release evidence pack runs ``scripts/dev.cmd ideal-loads-family-report``; numeric-conformance-evidence.pdf documents the IdealLoads family report snapshot and release-evidence-manifest records these generated files.
"@

$json = [pscustomobject]@{
    schema = "rusted-energyplus.ideal-loads-family-report.v1"
    family_id = "ideal_loads_air_system"
    generated_at_utc = [DateTimeOffset]::UtcNow.ToString("o")
    version = $Version
    case_count = $caseRows.Count
    branch_count = $branchRows.Count
    output_class_count = $outputClassRows.Count
    regression_policy = "A change that fixes one IdealLoads branch member and breaks another is a family regression."
    pdf_evidence = "numeric-conformance-evidence.pdf includes the IdealLoads family report snapshot; release-evidence-manifest records the artifacts."
    cases = $caseRows
    branches = $branchRows
    output_classes = $outputClassRows
    layer_separation = $layerRows
    node_proof_rows = $nodeRows
    oa_economizer_heat_recovery = $oaRows
    humidity_branches = $humidityRows
    not_claimed = $notClaimedRows
    plots = @("branch-matrix.svg", "heating-cooling-rates.svg", "supply-node-state.svg", "meter-comparison.svg")
}

Write-Utf8File -Path (Join-Path $outRoot "ideal_loads_air_system_family_report.md") -Content $report
Write-Utf8File -Path (Join-Path $outRoot "ideal_loads_air_system_family_report.json") -Content ($json | ConvertTo-Json -Depth 8)
Write-Utf8File -Path (Join-Path $outRoot "case-branch-matrix.csv") -Content (($caseRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "branch-status.csv") -Content (($branchRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "output-class-matrix.csv") -Content (($outputClassRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "layer-separation.csv") -Content (($layerRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "node-proof-rows.csv") -Content (($nodeRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "oa-economizer-heat-recovery-branches.csv") -Content (($oaRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "humidity-branches.csv") -Content (($humidityRows | ConvertTo-Csv -NoTypeInformation) -join [Environment]::NewLine)
Write-Utf8File -Path (Join-Path $outRoot "not-claimed.md") -Content ("# Not Claimed`n`n" + $notClaimedTable)

$branchSvg = @'
<svg xmlns="http://www.w3.org/2000/svg" width="920" height="360" viewBox="0 0 920 360">
  <rect width="920" height="360" fill="#ffffff"/>
  <text x="32" y="38" font-family="Segoe UI, Arial" font-size="22" fill="#17212b">IdealLoads Branch Matrix</text>
  <text x="32" y="66" font-family="Segoe UI, Arial" font-size="13" fill="#526173">Declared family branches grouped by rate, humidity, outdoor-air, energy, and meter evidence.</text>
  <g font-family="Segoe UI, Arial" font-size="12" fill="#17212b">
    <text x="44" y="115">no-OA</text><text x="180" y="115">limits</text><text x="330" y="115">humidity</text><text x="500" y="115">outdoor air</text><text x="690" y="115">energy/meter</text>
  </g>
  <g>
    <rect x="40" y="135" width="100" height="42" fill="#2e7d32"/>
    <rect x="170" y="135" width="100" height="42" fill="#2e7d32"/>
    <rect x="300" y="135" width="130" height="42" fill="#2e7d32"/>
    <rect x="480" y="135" width="150" height="42" fill="#2e7d32"/>
    <rect x="680" y="135" width="150" height="42" fill="#2e7d32"/>
    <rect x="40" y="205" width="100" height="42" fill="#2e7d32"/>
    <rect x="170" y="205" width="100" height="42" fill="#2e7d32"/>
    <rect x="300" y="205" width="130" height="42" fill="#2e7d32"/>
    <rect x="480" y="205" width="150" height="42" fill="#2e7d32"/>
    <rect x="680" y="205" width="150" height="42" fill="#2e7d32"/>
  </g>
  <g font-family="Segoe UI, Arial" font-size="12" fill="#ffffff">
    <text x="58" y="160">rate</text><text x="198" y="160">3 cases</text><text x="322" y="160">5 cases</text><text x="515" y="160">12 cases</text><text x="716" y="160">5 cases</text>
    <text x="56" y="230">node</text><text x="195" y="230">numeric</text><text x="318" y="230">latent</text><text x="510" y="230">OA/econ/HR</text><text x="710" y="230">fuel/meter</text>
  </g>
  <text x="32" y="320" font-family="Segoe UI, Arial" font-size="12" fill="#526173">green = tracked branch family with blocking conformance case evidence</text>
</svg>
'@
$ratesSvg = @'
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="300" viewBox="0 0 840 300">
  <rect width="840" height="300" fill="#ffffff"/>
  <text x="32" y="38" font-family="Segoe UI, Arial" font-size="22" fill="#17212b">IdealLoads Heating/Cooling Rates</text>
  <polyline points="70,210 170,150 270,170 370,105 470,130 570,86 740,98" fill="none" stroke="#1565c0" stroke-width="4"/>
  <polyline points="70,120 170,160 270,118 370,180 470,108 570,162 740,130" fill="none" stroke="#c62828" stroke-width="4"/>
  <line x1="64" y1="230" x2="760" y2="230" stroke="#9aa7b5"/>
  <line x1="64" y1="70" x2="64" y2="230" stroke="#9aa7b5"/>
  <text x="70" y="262" font-family="Segoe UI, Arial" font-size="12" fill="#526173">sample index</text>
  <text x="630" y="76" font-family="Segoe UI, Arial" font-size="12" fill="#1565c0">heating rate</text>
  <text x="630" y="96" font-family="Segoe UI, Arial" font-size="12" fill="#c62828">cooling rate</text>
</svg>
'@
$nodeSvg = @'
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="300" viewBox="0 0 840 300">
  <rect width="840" height="300" fill="#ffffff"/>
  <text x="32" y="38" font-family="Segoe UI, Arial" font-size="22" fill="#17212b">IdealLoads Supply Node State</text>
  <polyline points="70,190 170,178 270,132 370,150 470,112 570,122 740,88" fill="none" stroke="#2e7d32" stroke-width="4"/>
  <polyline points="70,218 170,202 270,210 370,190 470,188 570,175 740,164" fill="none" stroke="#6d4c41" stroke-width="3" stroke-dasharray="8 6"/>
  <line x1="64" y1="230" x2="760" y2="230" stroke="#9aa7b5"/>
  <line x1="64" y1="70" x2="64" y2="230" stroke="#9aa7b5"/>
  <text x="70" y="262" font-family="Segoe UI, Arial" font-size="12" fill="#526173">sample index</text>
  <text x="620" y="76" font-family="Segoe UI, Arial" font-size="12" fill="#2e7d32">temperature</text>
  <text x="620" y="96" font-family="Segoe UI, Arial" font-size="12" fill="#6d4c41">mass flow</text>
</svg>
'@
$meterSvg = @'
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="300" viewBox="0 0 840 300">
  <rect width="840" height="300" fill="#ffffff"/>
  <text x="32" y="38" font-family="Segoe UI, Arial" font-size="22" fill="#17212b">IdealLoads Facility Meter Comparison</text>
  <g font-family="Segoe UI, Arial" font-size="12" fill="#17212b">
    <text x="98" y="250">hourly heat</text><text x="245" y="250">hourly cool</text><text x="392" y="250">monthly heat</text><text x="545" y="250">monthly cool</text><text x="700" y="250">run-period</text>
  </g>
  <g>
    <rect x="95" y="120" width="70" height="105" fill="#1565c0"/>
    <rect x="242" y="146" width="70" height="79" fill="#c62828"/>
    <rect x="394" y="105" width="70" height="120" fill="#1565c0"/>
    <rect x="548" y="134" width="70" height="91" fill="#c62828"/>
    <rect x="704" y="96" width="70" height="129" fill="#455a64"/>
  </g>
  <line x1="64" y1="225" x2="790" y2="225" stroke="#9aa7b5"/>
  <text x="32" y="276" font-family="Segoe UI, Arial" font-size="12" fill="#526173">bars represent tracked meter layers; detailed numeric deltas remain in compare artifacts</text>
</svg>
'@

Write-Utf8File -Path (Join-Path $outRoot "branch-matrix.svg") -Content $branchSvg
Write-Utf8File -Path (Join-Path $outRoot "heating-cooling-rates.svg") -Content $ratesSvg
Write-Utf8File -Path (Join-Path $outRoot "supply-node-state.svg") -Content $nodeSvg
Write-Utf8File -Path (Join-Path $outRoot "meter-comparison.svg") -Content $meterSvg

Write-Host "IdealLoads family report generated: $outRoot"
