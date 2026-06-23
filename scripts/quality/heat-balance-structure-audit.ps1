[CmdletBinding()]
param()

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

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -notmatch $Pattern) {
        throw "$Description missing in $Path"
    }
}

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -match $Pattern) {
        throw "$Description unexpectedly present in $Path"
    }
}

function Assert-LineLimit {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][int]$Limit,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $lineCount = (Get-Content -Encoding UTF8 -LiteralPath $Path | Measure-Object -Line).Lines
    if ($lineCount -gt $Limit) {
        throw "$Description exceeds $Limit LOC: $Path has $lineCount LOC"
    }
}

$heatBalanceMod = "crates\ep_runtime\src\heat_balance\mod.rs"
$algorithm = "crates\ep_runtime\src\heat_balance\algorithm.rs"
$manager = "crates\ep_runtime\src\heat_balance\manager.rs"
$surfaceManager = "crates\ep_runtime\src\heat_balance\surface_manager.rs"
$airManager = "crates\ep_runtime\src\heat_balance\air_manager.rs"
$zonePredictorCorrector = "crates\ep_runtime\src\heat_balance\zone_predictor_corrector.rs"
$ctf = "crates\ep_runtime\src\heat_balance\ctf.rs"
$convection = "crates\ep_runtime\src\heat_balance\convection.rs"
$radiation = "crates\ep_runtime\src\heat_balance\radiation.rs"
$reports = "crates\ep_runtime\src\heat_balance\reports.rs"
$diagnosticProbe = "crates\ep_runtime\src\diagnostic_probes\heat_balance.rs"
$executionPlan = "crates\ep_runtime\src\execution_plan.rs"
$runtime = "crates\ep_runtime\src\runtime.rs"

foreach ($entry in @(
        @($heatBalanceMod, "heat-balance module facade"),
        @($algorithm, "heat-balance algorithm selector module"),
        @($manager, "HeatBalanceManager source-order module"),
        @($surfaceManager, "HeatBalanceSurfaceManager source-order module"),
        @($airManager, "HeatBalanceAirManager source-order module"),
        @($zonePredictorCorrector, "ZoneTempPredictorCorrector source-order module"),
        @($ctf, "CTF ownership module"),
        @($convection, "convection ownership module"),
        @($radiation, "radiation ownership module"),
        @($reports, "report ownership module"),
        @($diagnosticProbe, "diagnostic probe selector module"),
        @($executionPlan, "execution plan module"),
        @($runtime, "legacy runtime root")
    )) {
    Assert-FileExists -Path $entry[0] -Description $entry[1]
}

Assert-LineLimit -Path $manager -Limit 180 -Description "HeatBalanceManager source-order module"
Assert-LineLimit -Path $surfaceManager -Limit 240 -Description "HeatBalanceSurfaceManager source-order module"
Assert-LineLimit -Path $airManager -Limit 60 -Description "HeatBalanceAirManager source-order module"
Assert-LineLimit -Path $zonePredictorCorrector -Limit 240 -Description "ZoneTempPredictorCorrector source-order module"
Assert-LineLimit -Path $ctf -Limit 800 -Description "CTF ownership module"
Assert-LineLimit -Path $convection -Limit 260 -Description "convection ownership module"
Assert-LineLimit -Path $radiation -Limit 1200 -Description "radiation ownership module"
Assert-LineLimit -Path $reports -Limit 60 -Description "report ownership module"

Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod manager;' -Description "HeatBalanceManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_manager;' -Description "HeatBalanceSurfaceManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod air_manager;' -Description "HeatBalanceAirManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod zone_predictor_corrector;' -Description "ZoneTempPredictorCorrector module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod ctf;' -Description "CTF module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod convection;' -Description "convection module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod radiation;' -Description "radiation module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod reports;' -Description "reports module declaration"

Assert-Contains -Path $manager -Pattern 'pub fn manage_heat_balance_source_order_stages\s*\(' -Description "HeatBalanceManager source-order list"
foreach ($routine in @(
        "GetHeatBalanceInput",
        "InitHeatBalance",
        "RecKeepHeatBalance",
        "ReportHeatBalance",
        "CheckWarmupConvergence"
    )) {
    Assert-Contains -Path $manager -Pattern $routine -Description "HeatBalanceManager routine $routine"
}

foreach ($routine in @(
        "ManageSurfaceHeatBalance",
        "InitSurfaceHeatBalance",
        "CalcHeatBalanceOutsideSurf",
        "CalcHeatBalanceInsideSurf",
        "UpdateFinalSurfaceHeatBalance",
        "UpdateThermalHistories",
        "ReportSurfaceHeatBalance"
    )) {
    Assert-Contains -Path $surfaceManager -Pattern $routine -Description "HeatBalanceSurfaceManager routine $routine"
}

Assert-Contains -Path $airManager -Pattern 'ManageAirHeatBalance' -Description "HeatBalanceAirManager routine"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'ManageZoneAirUpdates' -Description "ZoneTempPredictorCorrector routine"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_zone_air_temperature_coefficients' -Description "ZoneTempPredictorCorrector coefficient owner"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_third_order_zone_air_temperature_c' -Description "ZoneTempPredictorCorrector third-order solver owner"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_analytical_zone_air_temperature_c' -Description "ZoneTempPredictorCorrector analytical solver owner"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_zone_air_temperature_coefficients\s*\(' -Description "runtime-owned zone-air coefficient implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_third_order_zone_air_temperature_c\s*\(' -Description "runtime-owned third-order zone-air implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_analytical_zone_air_temperature_c\s*\(' -Description "runtime-owned analytical zone-air implementation"
Assert-Contains -Path $ctf -Pattern 'UpdateThermalHistories' -Description "CTF history owner stage"
Assert-Contains -Path $ctf -Pattern 'surface_inside_conduction_rate_w_for_report' -Description "CTF inside conduction report owner"
Assert-Contains -Path $ctf -Pattern 'surface_outside_conduction_rate_w_for_report' -Description "CTF outside conduction report owner"
Assert-Contains -Path $ctf -Pattern 'surface_ctf_history_slot_samples' -Description "CTF history slot sampling owner"
Assert-Contains -Path $ctf -Pattern 'advance_surface_ctf_histories' -Description "CTF history advancement owner"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_conduction_rate_w' -Description "runtime-owned inside conduction implementation"
Assert-NotContains -Path $runtime -Pattern 'fn surface_outside_conduction_rate_w' -Description "runtime-owned outside conduction implementation"
Assert-NotContains -Path $runtime -Pattern 'fn update_surface_ctf_history_constants' -Description "runtime-owned CTF history constants implementation"
Assert-NotContains -Path $runtime -Pattern 'fn advance_surface_ctf_histories' -Description "runtime-owned CTF history advancement implementation"
Assert-Contains -Path $convection -Pattern 'CalcHeatBalanceInsideSurf' -Description "inside convection source owner"
Assert-Contains -Path $convection -Pattern 'CalcHeatBalanceOutsideSurf' -Description "outside convection source owner"
Assert-Contains -Path $convection -Pattern 'energyplus_tarp_inside_convection_coefficient_w_per_m2_k' -Description "inside TARP convection owner"
Assert-Contains -Path $convection -Pattern 'energyplus_doe2_outside_convection_coefficient_w_per_m2_k' -Description "DOE-2 outside convection owner"
Assert-Contains -Path $convection -Pattern 'exterior_convection_coefficient_w_per_m2_k' -Description "fallback exterior convection owner"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_tarp_inside_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned TARP inside convection implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_ashrae_tarp_natural_convection_w_per_m2_k\s*\(' -Description "runtime-owned ASHRAE TARP helper implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_doe2_outside_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned DOE-2 outside convection implementation"
Assert-NotContains -Path $runtime -Pattern 'fn exterior_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned fallback exterior convection implementation"
Assert-Contains -Path $radiation -Pattern 'CalcHeatBalanceOutsideSurf' -Description "exterior radiation source owner"
Assert-Contains -Path $radiation -Pattern 'CalcHeatBalanceInsideSurf' -Description "interior radiation source owner"
Assert-Contains -Path $radiation -Pattern 'append_surface_incident_solar_radiation_series' -Description "surface incident solar diagnostic owner"
Assert-Contains -Path $radiation -Pattern 'surface_incident_solar_components_hourly_average_w_per_m2' -Description "surface incident solar component owner"
Assert-Contains -Path $radiation -Pattern 'pub\(crate\) enum InteriorLongwaveExchangeProbe' -Description "interior longwave probe selector owner"
Assert-Contains -Path $radiation -Pattern 'update_surface_inside_longwave_exchange_probe' -Description "grey interior longwave probe owner"
Assert-Contains -Path $radiation -Pattern 'update_surface_inside_scriptf_longwave_exchange_probe' -Description "ScriptF interior longwave probe owner"
Assert-Contains -Path $radiation -Pattern 'energyplus_scriptf_from_view_factors' -Description "ScriptF matrix owner"
Assert-Contains -Path $radiation -Pattern 'energyplus_approximate_view_factors' -Description "interior approximate view-factor owner"
Assert-NotContains -Path $runtime -Pattern 'fn append_surface_incident_solar_radiation_series' -Description "runtime-owned surface incident solar diagnostic"
Assert-NotContains -Path $runtime -Pattern 'fn surface_incident_solar_components_hourly_average_w_per_m2' -Description "runtime-owned surface incident solar components"
Assert-NotContains -Path $runtime -Pattern 'fn update_surface_inside_longwave_exchange_probe\s*\(' -Description "runtime-owned grey interior longwave implementation"
Assert-NotContains -Path $runtime -Pattern 'fn update_surface_inside_scriptf_longwave_exchange_probe\s*\(' -Description "runtime-owned ScriptF interior longwave implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_scriptf_from_view_factors\s*\(' -Description "runtime-owned ScriptF matrix implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_approximate_view_factors\s*\(' -Description "runtime-owned approximate view-factor implementation"
Assert-Contains -Path $reports -Pattern 'ReportHeatBalance' -Description "zone report owner"
Assert-Contains -Path $reports -Pattern 'ReportSurfaceHeatBalance' -Description "surface report owner"

Assert-Contains -Path $executionPlan -Pattern 'ManageZoneAirUpdates' -Description "ManageZoneAirUpdates execution stage kind"
Assert-Contains -Path $executionPlan -Pattern 'manage_heat_balance_source_order_stages' -Description "execution plan consumes heat-balance source-order module"
Assert-Contains -Path $executionPlan -Pattern 'ExecutionStageKind::ManageZoneAirUpdates' -Description "zone-air steps bind to ManageZoneAirUpdates"
Assert-Contains -Path $runtime -Pattern 'ExecutionStageKind::ManageZoneAirUpdates' -Description "runtime tests assert ManageZoneAirUpdates barrier"

Assert-Contains -Path $algorithm -Pattern 'pub enum CompatibilityHeatBalanceAlgorithm' -Description "compatibility algorithm enum"
Assert-Contains -Path $algorithm -Pattern 'pub enum HeatBalanceZoneAirSelection' -Description "typed heat-balance selection enum"
Assert-Contains -Path $algorithm -Pattern 'EnergyPlusSourceOrder1ZoneOpaqueCompatibility' -Description "explicit source-order selector"
Assert-NotContains -Path $algorithm -Pattern 'CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat =>\s*\{\s*HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate' -Description "compatibility selector mapped to legacy candidate alias"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub enum DiagnosticHeatBalanceProbe' -Description "diagnostic probe enum"
Assert-Contains -Path $diagnosticProbe -Pattern 'HeatBalanceZoneAirSelection::Diagnostic' -Description "diagnostic selectors remain diagnostic"

Write-Host "Heat-balance structure audit complete."
