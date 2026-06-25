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
$surfaceBalance = "crates\ep_runtime\src\heat_balance\surface_balance.rs"
$surfaceBoundary = "crates\ep_runtime\src\heat_balance\surface_boundary.rs"
$airManager = "crates\ep_runtime\src\heat_balance\air_manager.rs"
$zonePredictorCorrector = "crates\ep_runtime\src\heat_balance\zone_predictor_corrector.rs"
$zoneAirCorrection = "crates\ep_runtime\src\heat_balance\zone_air_correction.rs"
$ctf = "crates\ep_runtime\src\heat_balance\ctf.rs"
$insideConvection = "crates\ep_runtime\src\heat_balance\inside_convection.rs"
$convection = "crates\ep_runtime\src\heat_balance\convection.rs"
$longwave = "crates\ep_runtime\src\heat_balance\longwave.rs"
$radiation = "crates\ep_runtime\src\heat_balance\radiation.rs"
$reports = "crates\ep_runtime\src\heat_balance\reports.rs"
$surfaceWeather = "crates\ep_runtime\src\heat_balance\surface_weather.rs"
$diagnosticProbe = "crates\ep_runtime\src\diagnostic_probes\heat_balance.rs"
$executionPlan = "crates\ep_runtime\src\execution_plan.rs"
$runtime = "crates\ep_runtime\src\runtime.rs"

foreach ($entry in @(
        @($heatBalanceMod, "heat-balance module facade"),
        @($algorithm, "heat-balance algorithm selector module"),
        @($manager, "HeatBalanceManager source-order module"),
        @($surfaceManager, "HeatBalanceSurfaceManager source-order module"),
        @($surfaceBalance, "surface balance ownership module"),
        @($surfaceBoundary, "surface boundary ownership module"),
        @($airManager, "HeatBalanceAirManager source-order module"),
        @($zonePredictorCorrector, "ZoneTempPredictorCorrector source-order module"),
        @($zoneAirCorrection, "zone-air correction ownership module"),
        @($ctf, "CTF ownership module"),
        @($insideConvection, "inside convection ownership module"),
        @($convection, "convection ownership module"),
        @($longwave, "exterior longwave ownership module"),
        @($radiation, "radiation ownership module"),
        @($reports, "report ownership module"),
        @($surfaceWeather, "surface weather ownership module"),
        @($diagnosticProbe, "diagnostic probe selector module"),
        @($executionPlan, "execution plan module"),
        @($runtime, "legacy runtime root")
    )) {
    Assert-FileExists -Path $entry[0] -Description $entry[1]
}

Assert-LineLimit -Path $manager -Limit 180 -Description "HeatBalanceManager source-order module"
Assert-LineLimit -Path $surfaceManager -Limit 240 -Description "HeatBalanceSurfaceManager source-order module"
Assert-LineLimit -Path $surfaceBalance -Limit 720 -Description "surface balance ownership module"
Assert-LineLimit -Path $surfaceBoundary -Limit 280 -Description "surface boundary ownership module"
Assert-LineLimit -Path $airManager -Limit 260 -Description "HeatBalanceAirManager source-order module"
Assert-LineLimit -Path $zonePredictorCorrector -Limit 240 -Description "ZoneTempPredictorCorrector source-order module"
Assert-LineLimit -Path $zoneAirCorrection -Limit 520 -Description "zone-air correction ownership module"
Assert-LineLimit -Path $ctf -Limit 800 -Description "CTF ownership module"
Assert-LineLimit -Path $insideConvection -Limit 360 -Description "inside convection ownership module"
Assert-LineLimit -Path $convection -Limit 420 -Description "convection ownership module"
Assert-LineLimit -Path $longwave -Limit 180 -Description "exterior longwave ownership module"
Assert-LineLimit -Path $radiation -Limit 1200 -Description "radiation ownership module"
Assert-LineLimit -Path $reports -Limit 160 -Description "report ownership module"
Assert-LineLimit -Path $surfaceWeather -Limit 180 -Description "surface weather ownership module"

Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod manager;' -Description "HeatBalanceManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_manager;' -Description "HeatBalanceSurfaceManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_balance;' -Description "surface balance module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_boundary;' -Description "surface boundary module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod air_manager;' -Description "HeatBalanceAirManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod zone_predictor_corrector;' -Description "ZoneTempPredictorCorrector module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod zone_air_correction;' -Description "zone-air correction module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod ctf;' -Description "CTF module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod inside_convection;' -Description "inside convection module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod convection;' -Description "convection module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod longwave;' -Description "longwave module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod radiation;' -Description "radiation module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod reports;' -Description "reports module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_weather;' -Description "surface weather module declaration"

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

Assert-Contains -Path $surfaceBalance -Pattern 'CalcHeatBalanceOutsideSurf' -Description "surface balance source owner"
Assert-Contains -Path $surfaceBalance -Pattern 'pub\(crate\) struct QuickOutsideConductionContext' -Description "quick outside conduction context owner"
Assert-Contains -Path $surfaceBalance -Pattern 'heat_balance_surface_boundary_balance' -Description "surface boundary balance owner"
Assert-Contains -Path $surfaceBalance -Pattern 'exterior_surface_boundary_balance' -Description "exterior surface boundary balance owner"
Assert-Contains -Path $surfaceBalance -Pattern 'reported_surface_outside_face_temperature_c' -Description "reported outside face temperature owner"
Assert-Contains -Path $surfaceBalance -Pattern 'surface_exterior_report_terms' -Description "surface exterior report terms owner"
Assert-Contains -Path $surfaceBalance -Pattern 'surface_inside_ctf_source_terms_w_per_m2' -Description "inside CTF source term owner"
Assert-Contains -Path $surfaceBalance -Pattern 'exterior_surface_energy_balance' -Description "exterior surface energy balance owner"
Assert-NotContains -Path $runtime -Pattern 'struct QuickOutsideConductionContext' -Description "runtime-owned quick outside conduction context"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_surface_boundary_balance\s*\(' -Description "runtime-owned surface boundary balance"
Assert-NotContains -Path $runtime -Pattern 'fn exterior_surface_boundary_balance\s*\(' -Description "runtime-owned exterior surface boundary balance"
Assert-NotContains -Path $runtime -Pattern 'fn reported_surface_outside_face_temperature_c\s*\(' -Description "runtime-owned reported outside face temperature"
Assert-NotContains -Path $runtime -Pattern 'fn surface_exterior_report_terms\s*\(' -Description "runtime-owned surface exterior report terms"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_ctf_source_terms_w_per_m2\s*\(' -Description "runtime-owned inside CTF source term"
Assert-NotContains -Path $runtime -Pattern 'fn exterior_surface_energy_balance\s*\(' -Description "runtime-owned exterior surface energy balance"
Assert-Contains -Path $surfaceBoundary -Pattern 'pub\(crate\) struct SurfaceBoundaryTarget' -Description "surface boundary target owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'resolve_surface_boundary_target' -Description "surface boundary target resolver owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'seed_initial_surface_ctf_boundary_histories' -Description "initial CTF boundary seeding owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'seed_energyplus_initial_surface_ctf_histories' -Description "EnergyPlus initial CTF boundary seeding owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'surface_boundary_temperature_c' -Description "surface boundary temperature owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'surface_steady_u_value_w_per_m2_k' -Description "surface steady U-value owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'sync_adiabatic_outside_faces_to_inside_faces' -Description "adiabatic outside-face sync owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'inside_ctf_outside_temperature_history_commit_override_c' -Description "inside CTF outside history commit override owner"
Assert-NotContains -Path $runtime -Pattern 'fn sync_adiabatic_outside_faces_to_inside_faces\s*\(' -Description "runtime-owned adiabatic outside-face sync"
Assert-NotContains -Path $runtime -Pattern 'fn inside_ctf_outside_temperature_history_commit_override_c\s*\(' -Description "runtime-owned inside CTF outside history commit override"
Assert-NotContains -Path $runtime -Pattern 'struct SurfaceBoundaryTarget' -Description "runtime-owned surface boundary target"
Assert-NotContains -Path $runtime -Pattern 'fn resolve_surface_boundary_target\s*\(' -Description "runtime-owned surface boundary target resolver"
Assert-NotContains -Path $runtime -Pattern 'fn seed_initial_surface_ctf_boundary_histories\s*\(' -Description "runtime-owned initial CTF boundary seeding"
Assert-NotContains -Path $runtime -Pattern 'fn seed_energyplus_initial_surface_ctf_histories\s*\(' -Description "runtime-owned EnergyPlus initial CTF boundary seeding"
Assert-NotContains -Path $runtime -Pattern 'fn surface_boundary_temperature_c\s*\(' -Description "runtime-owned surface boundary temperature"
Assert-NotContains -Path $runtime -Pattern 'fn surface_steady_u_value_w_per_m2_k\s*\(' -Description "runtime-owned surface steady U-value"
Assert-Contains -Path $airManager -Pattern 'ManageAirHeatBalance' -Description "HeatBalanceAirManager routine"
Assert-Contains -Path $airManager -Pattern 'weather_context_zone_air_heat_capacity_j_per_k' -Description "weather-driven zone-air heat capacity owner"
Assert-Contains -Path $airManager -Pattern 'update_zone_air_heat_capacities_from_weather_context' -Description "zone-air weather capacity updater owner"
Assert-Contains -Path $airManager -Pattern 'seed_zone_air_humidity_ratios_from_weather_records' -Description "zone-air weather humidity seeding owner"
Assert-Contains -Path $airManager -Pattern 'zone_air_heat_balance_air_storage_rate_w' -Description "zone-air storage report owner"
Assert-NotContains -Path $runtime -Pattern 'fn weather_context_zone_air_heat_capacity_j_per_k\s*\(' -Description "runtime-owned zone-air weather capacity implementation"
Assert-NotContains -Path $runtime -Pattern 'fn update_zone_air_heat_capacities_from_weather_context\s*\(' -Description "runtime-owned zone-air weather capacity updater"
Assert-NotContains -Path $runtime -Pattern 'fn seed_zone_air_humidity_ratios_from_weather_records\s*\(' -Description "runtime-owned weather humidity seeding"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_air_storage_rate_w\s*\(' -Description "runtime-owned zone-air storage report implementation"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'ManageZoneAirUpdates' -Description "ZoneTempPredictorCorrector routine"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_zone_air_temperature_coefficients' -Description "ZoneTempPredictorCorrector coefficient owner"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_third_order_zone_air_temperature_c' -Description "ZoneTempPredictorCorrector third-order solver owner"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_analytical_zone_air_temperature_c' -Description "ZoneTempPredictorCorrector analytical solver owner"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_zone_air_temperature_coefficients\s*\(' -Description "runtime-owned zone-air coefficient implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_third_order_zone_air_temperature_c\s*\(' -Description "runtime-owned third-order zone-air implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_analytical_zone_air_temperature_c\s*\(' -Description "runtime-owned analytical zone-air implementation"
Assert-Contains -Path $zoneAirCorrection -Pattern 'ManageZoneAirUpdates' -Description "zone-air correction source owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'correct_zone_air_temperatures_from_current_surfaces' -Description "zone-air temperature correction owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'correct_zone_air_humidity_ratios_from_current_state' -Description "zone-air humidity correction owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'apply_energyplus_adaptive_system_timestep_zone_air_correction' -Description "adaptive system timestep correction owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'energyplus_down_interpolate_three_history_values' -Description "system timestep history interpolation owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'zone_air_system_timestep_storage_report_rate_w' -Description "system timestep air storage report owner"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_zone_temperature_map\s*\(' -Description "runtime-owned zone temperature map"
Assert-NotContains -Path $runtime -Pattern 'fn correct_zone_air_temperatures_from_current_surfaces\s*\(' -Description "runtime-owned zone-air temperature correction"
Assert-NotContains -Path $runtime -Pattern 'fn correct_zone_air_humidity_ratios_from_current_state\s*\(' -Description "runtime-owned zone-air humidity correction"
Assert-NotContains -Path $runtime -Pattern 'fn apply_energyplus_adaptive_system_timestep_zone_air_correction\s*\(' -Description "runtime-owned adaptive system timestep correction"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_system_timestep_storage_report_rate_w\s*\(' -Description "runtime-owned system timestep air storage report"
Assert-NotContains -Path $runtime -Pattern 'fn correct_single_zone_air_temperature_from_current_surfaces\s*\(' -Description "runtime-owned single-zone temperature correction"
Assert-NotContains -Path $runtime -Pattern 'fn correct_single_zone_air_humidity_ratio_from_history\s*\(' -Description "runtime-owned single-zone humidity correction"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_down_interpolate_three_history_values\s*\(' -Description "runtime-owned down interpolation"
Assert-Contains -Path $insideConvection -Pattern 'CalcHeatBalanceInsideSurf' -Description "inside convection source owner"
Assert-Contains -Path $insideConvection -Pattern 'heat_balance_inside_convection_coefficients' -Description "inside convection coefficient owner"
Assert-Contains -Path $insideConvection -Pattern 'heat_balance_inside_convection_coefficient_inputs' -Description "inside convection coefficient input owner"
Assert-Contains -Path $insideConvection -Pattern 'zone_surface_convection_sums' -Description "zone surface convection sum owner"
Assert-Contains -Path $insideConvection -Pattern 'surface_inside_convection_report_coefficient_w_per_m2_k' -Description "inside convection report coefficient owner"
Assert-Contains -Path $insideConvection -Pattern 'surface_inside_convection_heat_gain_rate_per_area_w_per_m2' -Description "inside convection heat gain report owner"
Assert-Contains -Path $insideConvection -Pattern 'zone_air_heat_balance_surface_convection_rate_w' -Description "zone-air surface convection report owner"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_inside_convection_coefficients\s*\(' -Description "runtime-owned inside convection coefficients"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_inside_convection_coefficient_inputs\s*\(' -Description "runtime-owned inside convection coefficient inputs"
Assert-NotContains -Path $runtime -Pattern 'fn zone_surface_convection_sums\s*\(' -Description "runtime-owned zone surface convection sums"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_convection_reference_air_temperature_c\s*\(' -Description "runtime-owned inside convection reference air report"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_convection_report_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned inside convection report coefficient"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_convection_heat_gain_rate_per_area_w_per_m2\s*\(' -Description "runtime-owned inside convection heat gain report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w\s*\(' -Description "runtime-owned surface reference air convection report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_from_final_inside_hconv_report_w\s*\(' -Description "runtime-owned final hconv convection report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_w\s*\(' -Description "runtime-owned zone-air surface convection report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_at_air_temperature_w\s*\(' -Description "runtime-owned zone-air convection at air temperature report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_from_balance_w\s*\(' -Description "runtime-owned balance surface convection report"
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
Assert-Contains -Path $convection -Pattern 'pub\(crate\) struct ExteriorConvectionTerms' -Description "exterior convection terms owner"
Assert-Contains -Path $convection -Pattern 'energyplus_exterior_convection_terms' -Description "exterior convection term calculation owner"
Assert-Contains -Path $convection -Pattern 'energyplus_surface_outside_wind_speed_m_per_s' -Description "exterior wind speed profile owner"
Assert-Contains -Path $convection -Pattern 'energyplus_surface_outdoor_air_temperature_c' -Description "exterior air temperature profile owner"
Assert-Contains -Path $convection -Pattern 'heat_balance_uses_doe2_outside_convection' -Description "DOE-2 outside convection selector owner"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_tarp_inside_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned TARP inside convection implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_ashrae_tarp_natural_convection_w_per_m2_k\s*\(' -Description "runtime-owned ASHRAE TARP helper implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_doe2_outside_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned DOE-2 outside convection implementation"
Assert-NotContains -Path $runtime -Pattern 'fn exterior_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned fallback exterior convection implementation"
Assert-NotContains -Path $runtime -Pattern 'struct ExteriorConvectionTerms' -Description "runtime-owned exterior convection terms"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_convection_terms\s*\(' -Description "runtime-owned exterior convection term calculation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_surface_outside_wind_speed_m_per_s\s*\(' -Description "runtime-owned exterior wind speed profile"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_surface_outdoor_air_temperature_c\s*\(' -Description "runtime-owned exterior air temperature profile"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_doe2_outside_convection\s*\(' -Description "runtime-owned DOE-2 outside convection selector"
Assert-Contains -Path $longwave -Pattern 'CalcHeatBalanceOutsideSurf' -Description "exterior longwave source owner"
Assert-Contains -Path $longwave -Pattern 'pub\(crate\) struct ExteriorLongwaveTerms' -Description "exterior longwave terms owner"
Assert-Contains -Path $longwave -Pattern 'energyplus_exterior_longwave_terms' -Description "exterior longwave term calculation owner"
Assert-Contains -Path $longwave -Pattern 'energyplus_linearized_radiation_coefficient_w_per_m2_k' -Description "linearized exterior radiation coefficient owner"
Assert-Contains -Path $longwave -Pattern 'horizontal_infrared_sky_temperature_c' -Description "horizontal infrared sky temperature owner"
Assert-NotContains -Path $runtime -Pattern 'struct ExteriorLongwaveTerms' -Description "runtime-owned exterior longwave terms"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_longwave_terms\s*\(' -Description "runtime-owned exterior longwave implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_linearized_radiation_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned linearized exterior radiation coefficient"
Assert-NotContains -Path $runtime -Pattern 'fn horizontal_infrared_sky_temperature_c\s*\(' -Description "runtime-owned horizontal infrared sky temperature"
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
Assert-Contains -Path $surfaceWeather -Pattern 'CalcHeatBalanceOutsideSurf' -Description "surface weather source owner"
Assert-Contains -Path $surfaceWeather -Pattern 'energyplus_exterior_wet_timestep_fraction' -Description "exterior wet timestep fraction owner"
Assert-Contains -Path $surfaceWeather -Pattern 'energyplus_exterior_wet_context_fraction' -Description "exterior wet context fraction owner"
Assert-Contains -Path $surfaceWeather -Pattern 'energyplus_weather_record_is_rain_at_timestep' -Description "rain interpolation owner"
Assert-Contains -Path $surfaceWeather -Pattern 'energyplus_exterior_wet_reference_temperature_c' -Description "wet exterior reference temperature owner"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_wet_timestep_fraction\s*\(' -Description "runtime-owned exterior wet timestep fraction"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_wet_context_fraction\s*\(' -Description "runtime-owned exterior wet context fraction"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_weather_record_is_rain_at_timestep\s*\(' -Description "runtime-owned rain interpolation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_wet_reference_temperature_c\s*\(' -Description "runtime-owned wet exterior reference temperature"
Assert-Contains -Path $reports -Pattern 'zone_surface_report_conduction_rates_w' -Description "zone surface conduction report owner"
Assert-Contains -Path $reports -Pattern 'heat_gain_rate_w' -Description "positive heat-gain report helper owner"
Assert-Contains -Path $reports -Pattern 'heat_loss_rate_w' -Description "positive heat-loss report helper owner"
Assert-NotContains -Path $runtime -Pattern 'fn zone_surface_report_conduction_rates_w\s*\(' -Description "runtime-owned zone surface conduction report"
Assert-NotContains -Path $runtime -Pattern 'fn heat_gain_rate_w\s*\(' -Description "runtime-owned heat gain report helper"
Assert-NotContains -Path $runtime -Pattern 'fn heat_loss_rate_w\s*\(' -Description "runtime-owned heat loss report helper"
Assert-Contains -Path $reports -Pattern 'ReportSurfaceHeatBalance' -Description "surface report owner"

Assert-Contains -Path $executionPlan -Pattern 'ManageZoneAirUpdates' -Description "ManageZoneAirUpdates execution stage kind"
Assert-Contains -Path $executionPlan -Pattern 'manage_heat_balance_source_order_stages' -Description "execution plan consumes heat-balance source-order module"
Assert-Contains -Path $executionPlan -Pattern 'ExecutionStageKind::ManageZoneAirUpdates' -Description "zone-air steps bind to ManageZoneAirUpdates"
Assert-Contains -Path $runtime -Pattern 'ExecutionStageKind::ManageZoneAirUpdates' -Description "runtime tests assert ManageZoneAirUpdates barrier"

Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_third_order_zone_air_correction' -Description "third-order zone-air flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_preserves_surface_inside_temperature_for_first_longwave' -Description "first-longwave inside-temperature preservation flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_weather_air_storage_report' -Description "weather air-storage report flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_balance_surface_convection_report' -Description "balance surface convection report flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_surface_reference_air_surface_convection_report' -Description "surface reference-air convection report flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_final_inside_convection_report' -Description "final inside convection report flag owner"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_third_order_zone_air_correction\s*\(' -Description "runtime-owned third-order zone-air flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_preserves_surface_inside_temperature_for_first_longwave\s*\(' -Description "runtime-owned first-longwave preservation flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_weather_air_storage_report\s*\(' -Description "runtime-owned weather air-storage report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_previous_mat_surface_convection_report\s*\(' -Description "runtime-owned previous-MAT convection report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_balance_surface_convection_report\s*\(' -Description "runtime-owned balance convection report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_surface_reference_air_convection_report\s*\(' -Description "runtime-owned surface reference-air report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_surface_reference_air_surface_convection_report\s*\(' -Description "runtime-owned surface reference-air convection report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_final_inside_convection_report\s*\(' -Description "runtime-owned final inside convection report flag"
Assert-Contains -Path $algorithm -Pattern 'pub enum CompatibilityHeatBalanceAlgorithm' -Description "compatibility algorithm enum"
Assert-Contains -Path $algorithm -Pattern 'pub enum HeatBalanceZoneAirSelection' -Description "typed heat-balance selection enum"
Assert-Contains -Path $algorithm -Pattern 'EnergyPlusSourceOrder1ZoneOpaqueCompatibility' -Description "explicit source-order selector"
Assert-NotContains -Path $algorithm -Pattern 'CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat =>\s*\{\s*HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate' -Description "compatibility selector mapped to legacy candidate alias"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub enum DiagnosticHeatBalanceProbe' -Description "diagnostic probe enum"
Assert-Contains -Path $diagnosticProbe -Pattern 'HeatBalanceZoneAirSelection::Diagnostic' -Description "diagnostic selectors remain diagnostic"

Write-Host "Heat-balance structure audit complete."
