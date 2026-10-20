---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-10-20
---

# Heat Balance Source Map

Reference version: EnergyPlus 26.1.0

Reference source root:

```text
.reference/energyplus-src/26.1.0/
```

Regenerable local source audit cache:

```text
.runtime/energyplus-source/v26.1.0/src/EnergyPlus/
```

Purpose: record the EnergyPlus source files, routines, data structures, and
call order needed for source-faithful heat-balance porting. This map is a planning guard, not a conformance
claim.

## Primary Source Files

| Area | EnergyPlus source | Rust target |
|---|---|---|
| heat-balance orchestration | `src/EnergyPlus/HeatBalanceManager.cc` | `ep_runtime::heat_balance` |
| heat-balance declarations | `src/EnergyPlus/HeatBalanceManager.hh` | `ep_runtime::heat_balance` |
| Energy Management System dispatch | `src/EnergyPlus/EMSManager.cc::ManageEMS` | deferred; no bounded Rust target |
| global heat-balance data | `src/EnergyPlus/DataHeatBalance.hh` | `ep_model`, `ep_runtime::HeatBalanceState` |
| global heat-balance report registration | `src/EnergyPlus/DataHeatBalance.cc` | `ep_runtime::ResultStore`, conformance report metadata |
| surface heat balance | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ep_runtime::surface_balance` |
| surface heat-balance declarations | `src/EnergyPlus/HeatBalanceSurfaceManager.hh` | `ep_runtime::surface_balance` |
| surface/material selection state | `src/EnergyPlus/DataSurfaces.cc::GetVariableAbsorptanceSurfaceList` | `ep_model::VariableAbsorptanceSurfaceBinding`, `ep_compiler` |
| per-Surface computed geometry | `src/EnergyPlus/DataSurfaces.cc::SurfaceData::set_computed_geometry` | `ep_model::SurfaceComputedGeometry`, `ep_compiler` |
| incident-solar multiplier request front end | `src/EnergyPlus/HeatBalanceManager.cc::GetIncidentSolarMultiplier` | `ep_model::SurfaceIncidentSolarMultiplierRequest`, `ep_compiler` |
| scheduled inside-solar input first phase | `src/EnergyPlus/HeatBalanceManager.cc::GetScheduledSurfaceGains` | `ep_model::SurfaceSolarIncident`, `ep_compiler` |
| scheduled complex-fenestration layer input second phase | `src/EnergyPlus/HeatBalanceManager.cc::GetScheduledSurfaceGains` | `ep_model::FenestrationSolarAbsorbedRequest`, `ep_compiler` |
| zone air predictor/corrector | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ep_runtime::zone_air` |
| zone air declarations | `src/EnergyPlus/ZoneTempPredictorCorrector.hh` | `ep_runtime::zone_air` |
| internal gains input summaries | `src/EnergyPlus/HeatBalanceInternalHeatGains.cc` | `ep_compiler`, `ep_runtime::internal_gains` |
| internal gains runtime sums | `src/EnergyPlus/InternalHeatGains.cc` | `ep_runtime::internal_gains` |
| output variable registration | `src/EnergyPlus/OutputProcessor.cc` | `ep_conformance`, `ep_runtime::ResultStore` |

## Required Routine Map

Routine status below mirrors `specs/algorithm_ledger.toml`. Source mapping
records the reviewed symbol; it does not establish implementation, numerical
parity, or full-domain readiness. Full boundary and addenda remain in the
[canonical algorithm ledger](../../../specs/algorithm_ledger.toml).

| Routine ID | EnergyPlus file | Source routine | Completion | Required |
|---|---|---|---|---|
| `manage_heat_balance` | `src/EnergyPlus/HeatBalanceManager.cc` | `ManageHeatBalance` | source_mapped | yes |
| `manage_ems` | `src/EnergyPlus/EMSManager.cc` | `ManageEMS` | source_mapped | no |
| `init_heat_balance` | `src/EnergyPlus/HeatBalanceManager.cc` | `InitHeatBalance` | source_mapped | yes |
| `rec_keep_heat_balance` | `src/EnergyPlus/HeatBalanceManager.cc` | `RecKeepHeatBalance` | source_mapped | yes |
| `update_window_face_temps_non_bsdf_win` | `src/EnergyPlus/HeatBalanceManager.cc` | `UpdateWindowFaceTempsNonBSDFWin` | source_mapped | yes |
| `open_shading_file` | `src/EnergyPlus/HeatBalanceManager.cc` | `OpenShadingFile` | source_mapped | no |
| `set_storm_window_control` | `src/EnergyPlus/HeatBalanceManager.cc` | `SetStormWindowControl` | source_mapped | no |
| `report_heat_balance` | `src/EnergyPlus/HeatBalanceManager.cc` | `ReportHeatBalance` | source_mapped | yes |
| `update_ems_trend_variables` | `src/EnergyPlus/EMSManager.cc` | `UpdateEMSTrendVariables` | source_mapped | no |
| `update_plugin_values` | `src/EnergyPlus/PluginManager.cc` | `updatePluginValues` | source_mapped | no |
| `check_warmup_convergence` | `src/EnergyPlus/HeatBalanceManager.cc` | `CheckWarmupConvergence` | source_mapped | yes |
| `report_warmup_convergence` | `src/EnergyPlus/HeatBalanceManager.cc` | `ReportWarmupConvergence` | source_mapped | yes |
| `set_pre_construction_input_parameters` | `src/EnergyPlus/HeatBalanceManager.cc` | `SetPreConstructionInputParameters` | source_mapped | yes |
| `get_site_atmosphere_data` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetSiteAtmosphereData` | source_mapped | yes |
| `allocate_zone_heat_bal_arrays` | `src/EnergyPlus/HeatBalanceManager.cc` | `AllocateZoneHeatBalArrays` | source_mapped | yes |
| `allocate_heat_bal_arrays` | `src/EnergyPlus/HeatBalanceManager.cc` | `AllocateHeatBalArrays` | source_mapped | yes |
| `init_conduction_transfer_functions` | `src/EnergyPlus/HeatBalanceManager.cc` | `InitConductionTransferFunctions` | source_mapped | yes |
| `get_frame_and_divider_data` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetFrameAndDividerData` | state_mapped | no |
| `get_construct_data` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetConstructData` | source_mapped | yes |
| `search_window5_data_file` | `src/EnergyPlus/HeatBalanceManager.cc` | `SearchWindow5DataFile` | source_mapped | no |
| `create_fc_factor_constructions` | `src/EnergyPlus/HeatBalanceManager.cc` | `CreateFCfactorConstructions` | state_mapped | no |
| `create_air_boundary_constructions` | `src/EnergyPlus/HeatBalanceManager.cc` | `CreateAirBoundaryConstructions` | state_mapped | no |
| `setup_complex_fenestration_state_input` | `src/EnergyPlus/HeatBalanceManager.cc` | `SetupComplexFenestrationStateInput` | state_mapped | no |
| `get_building_data` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetBuildingData` | source_mapped | yes |
| `get_incident_solar_multiplier` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetIncidentSolarMultiplier` | source_mapped | no |
| `get_scheduled_surface_gains` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetScheduledSurfaceGains` | source_mapped | no |
| `check_scheduled_surface_gains` | `src/EnergyPlus/HeatBalanceManager.cc` | `CheckScheduledSurfaceGains` | state_mapped | no |
| `create_tc_constructions` | `src/EnergyPlus/HeatBalanceManager.cc` | `CreateTCConstructions` | state_mapped | no |
| `check_valid_simulation_objects` | `src/EnergyPlus/HeatBalanceManager.cc` | `CheckValidSimulationObjects` | state_mapped | no |
| `check_used_constructions` | `src/EnergyPlus/HeatBalanceManager.cc` | `CheckUsedConstructions` | state_mapped | no |
| `init_solar_view_factors` | `src/EnergyPlus/HeatBalanceIntRadExchange.cc` | `InitSolarViewFactors` | source_mapped | yes |
| `manage_internal_heat_gains` | `src/EnergyPlus/InternalHeatGains.cc` | `ManageInternalHeatGains` | source_mapped | yes |
| `get_internal_heat_gains_input` | `src/EnergyPlus/InternalHeatGains.cc` | `GetInternalHeatGainsInput` | state_mapped | no |
| `setup_kiva_instances` | `src/EnergyPlus/HeatBalanceKivaManager.cc` | `setupKivaInstances` | source_mapped | no |
| `surface_octree_init` | `src/EnergyPlus/SurfaceOctree.cc` | `init` | source_mapped | no |
| `set_computed_geometry` | `src/EnergyPlus/DataSurfaces.cc` | `set_computed_geometry` | state_mapped | no |
| `get_zone_data` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetZoneData` | source_mapped | yes |
| `get_zone_local_env_data` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetZoneLocalEnvData` | state_mapped | no |
| `get_space_data` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetSpaceData` | state_mapped | no |
| `get_general_space_type_num` | `src/EnergyPlus/HeatBalanceManager.cc` | `GetGeneralSpaceTypeNum` | state_mapped | no |
| `process_zone_data` | `src/EnergyPlus/HeatBalanceManager.cc` | `ProcessZoneData` | state_mapped | no |
| `manage_surface_heat_balance` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ManageSurfaceHeatBalance` | source_mapped | yes |
| `init_surface_heat_balance` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `InitSurfaceHeatBalance` | source_mapped | yes |
| `allocate_surface_heat_bal_arrays` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `AllocateSurfaceHeatBalArrays` | source_mapped | yes |
| `init_thermal_and_flux_histories` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `InitThermalAndFluxHistories` | source_mapped | yes |
| `eval_outside_movable_insulation` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `EvalOutsideMovableInsulation` | source_mapped | no |
| `eval_inside_movable_insulation` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `EvalInsideMovableInsulation` | source_mapped | no |
| `init_solar_heat_gains` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `InitSolarHeatGains` | source_mapped | yes |
| `init_int_solar_distribution` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `InitIntSolarDistribution` | source_mapped | yes |
| `compute_int_thermal_absorp_factors` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ComputeIntThermalAbsorpFactors` | source_mapped | yes |
| `compute_int_sw_absorp_factors` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ComputeIntSWAbsorpFactors` | source_mapped | yes |
| `compute_dif_sol_exc_zones_wiz_windows` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ComputeDifSolExcZonesWIZWindows` | source_mapped | yes |
| `init_ems_controlled_surface_properties` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `InitEMSControlledSurfaceProperties` | source_mapped | no |
| `init_ems_controlled_constructions` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `InitEMSControlledConstructions` | source_mapped | no |
| `update_intermediate_surface_heat_balance_results` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `UpdateIntermediateSurfaceHeatBalanceResults` | source_mapped | yes |
| `update_non_representative_surface_results` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `UpdateNonRepresentativeSurfaceResults` | source_mapped | no |
| `gather_for_predefined_report` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `GatherForPredefinedReport` | source_mapped | no |
| `calc_heat_balance_outside_surf` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `CalcHeatBalanceOutsideSurf` | source_mapped | yes |
| `get_qdot_conv_out_per_area` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `GetQdotConvOutPerArea` | source_mapped | yes |
| `calc_heat_balance_inside_surf` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `CalcHeatBalanceInsideSurf` | source_mapped | yes |
| `calc_heat_balance_inside_surf_2` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `CalcHeatBalanceInsideSurf2` | source_mapped | yes |
| `calc_heat_balance_inside_surf_2_ctf_only` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `CalcHeatBalanceInsideSurf2CTFOnly` | source_mapped | yes |
| `sum_surf_qdot_rad_hvac` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `sumSurfQdotRadHVAC` | source_mapped | yes |
| `test_surf_temp_calc_heat_balance_inside_surf` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `TestSurfTempCalcHeatBalanceInsideSurf` | source_mapped | yes |
| `calc_outside_surf_temp` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `CalcOutsideSurfTemp` | source_mapped | yes |
| `calc_exterior_vented_cavity` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `CalcExteriorVentedCavity` | source_mapped | no |
| `gather_component_loads_surf_abs_fact` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `GatherComponentLoadsSurfAbsFact` | source_mapped | no |
| `get_surf_incident_solar_multiplier` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `GetSurfIncidentSolarMultiplier` | source_mapped | no |
| `init_surface_property_view_factors` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `InitSurfacePropertyViewFactors` | source_mapped | no |
| `get_ground_surfaces_temperature_average` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `GetGroundSurfacesTemperatureAverage` | source_mapped | no |
| `get_ground_surfaces_reflectance_average` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `GetGroundSurfacesReflectanceAverage` | source_mapped | no |
| `re_set_ground_surfaces_view_factor` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ReSetGroundSurfacesViewFactor` | source_mapped | no |
| `get_surrounding_surfaces_temperature_average` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `GetSurroundingSurfacesTemperatureAverage` | source_mapped | no |
| `update_final_surface_heat_balance` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `UpdateFinalSurfaceHeatBalance` | source_mapped | yes |
| `update_thermal_histories` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `UpdateThermalHistories` | source_mapped | yes |
| `calculate_zone_mrt` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `CalculateZoneMRT` | source_mapped | yes |
| `surface_data_fd_update_moisture_balance` | `src/EnergyPlus/HeatBalFiniteDiffManager.hh` | `UpdateMoistureBalance` | source_mapped | no |
| `manage_thermal_comfort` | `src/EnergyPlus/ThermalComfort.cc` | `ManageThermalComfort` | source_mapped | no |
| `report_surface_heat_balance` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ReportSurfaceHeatBalance` | source_mapped | yes |
| `report_non_representative_surface_results` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ReportNonRepresentativeSurfaceResults` | source_mapped | no |
| `report_int_mov_ins_inside_surf_temp` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ReportIntMovInsInsideSurfTemp` | source_mapped | no |
| `gather_component_loads_surface` | `src/EnergyPlus/OutputReportTabular.cc` | `GatherComponentLoadsSurface` | source_mapped | no |
| `calc_thermal_resilience` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `CalcThermalResilience` | source_mapped | no |
| `report_thermal_resilience` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ReportThermalResilience` | source_mapped | no |
| `report_co2_resilience` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ReportCO2Resilience` | source_mapped | no |
| `report_visual_resilience` | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | `ReportVisualResilience` | source_mapped | no |
| `manage_air_heat_balance` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `ManageAirHeatBalance` | source_mapped | yes |
| `get_air_heat_balance_input` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `GetAirHeatBalanceInput` | source_mapped | yes |
| `get_air_flow_flag` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `GetAirFlowFlag` | source_mapped | yes |
| `set_zone_mass_conservation_flag` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `SetZoneMassConservationFlag` | source_mapped | yes |
| `get_simple_air_model_inputs` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `GetSimpleAirModelInputs` | source_mapped | yes |
| `get_room_air_model_parameters` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `GetRoomAirModelParameters` | source_mapped | yes |
| `init_air_heat_balance` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `InitAirHeatBalance` | source_mapped | yes |
| `init_simple_mixing_convective_heat_gains` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `InitSimpleMixingConvectiveHeatGains` | source_mapped | yes |
| `calc_heat_balance_air` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `CalcHeatBalanceAir` | source_mapped | yes |
| `initialize_for_external_hvac_manager` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `initializeForExternalHVACManager` | source_mapped | no |
| `report_zone_mean_air_temp` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `ReportZoneMeanAirTemp` | source_mapped | yes |
| `calc_mean_air_temps` | `src/EnergyPlus/HeatBalanceAirManager.cc` | `calcMeanAirTemps` | source_mapped | yes |

### Source stage references

| Algorithm ID | EnergyPlus file | Routine | Source stage | Owner |
|---|---|---|---|---|
| `official_dynamic_heat_balance_compat_candidate` | `src/EnergyPlus/HeatBalanceManager.cc` | `ManageHeatBalance` | `GetHeatBalanceInput` | `GetHeatBalanceInput` |
| `official_dynamic_heat_balance_compat_candidate` | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ManageZoneAirUpdates` | `ManageZoneAirUpdates` | `ManageZoneAirUpdates` |
| `heat_balance_manager_source_order` | `src/EnergyPlus/HeatBalanceManager.cc` | `ManageHeatBalance` | `GetHeatBalanceInput` | `GetHeatBalanceInput` |

## Call Order Boundary

The manager preserves the EnergyPlus caller sequence below. Definition-order
inventory is separate from runtime call order; BeginSim, BeginEnvrn, warmup,
EMS, and conditional child calls retain their source-owned cadence. A mapped
barrier does not prove a complete Rust implementation of its children.

1. `ManageHeatBalance`
2. input acquisition through project controls, materials, frame-and-divider properties, constructions, then `GetBuildingData` in its `GetShadowingInput` -> `GetZoneData` -> `SetupZoneGeometry` order, followed by `DataSurfaces::GetVariableAbsorptanceSurfaceList`, `GetIncidentSolarMultiplier`, `GetScheduledSurfaceGains`, the inline representative-surface EIO assignment barrier, `CreateTCConstructions`, the inline no-Zone validity gate with `CheckValidSimulationObjects`, `CheckUsedConstructions`, the immediate inline fatal barrier, `HeatBalanceIntRadExchange::InitSolarViewFactors` at line 316, `ManageInternalHeatGains(state, true)` at line 320, and conditional Kiva setup at lines 322-325; after `GetHeatBalanceInput` returns, the caller conditionally applies the sizing Space heat-balance mode at lines 169-171, conditionally initializes the Surface octree at lines 173-180, then visits the complete Surface array at lines 182-184 for `set_computed_geometry` before clearing `ManageHeatBalanceGetInputFlag` at line 186.
3. unconditional `ManageEMS(state, EMSCallFrom::BeginZoneTimestepBeforeInitHeatBalance, anyRan, absent)`
4. `InitHeatBalance`
5. unconditional `ManageEMS(state, EMSCallFrom::BeginZoneTimestepAfterInitHeatBalance, anyRan, absent)`
6. unconditional `ManageSurfaceHeatBalance(state)`; its surface-parent order is detailed below.
7. unconditional `ManageEMS(state, EMSCallFrom::EndZoneTimestepBeforeZoneReporting, anyRan, absent)` at `HeatBalanceManager.cc` line 210
8. unconditional `RecKeepHeatBalance(state)` at line 211, declared at `HeatBalanceManager.hh` line 134 and implemented at `HeatBalanceManager.cc` lines 2971-3057
9. unconditional `ReportHeatBalance(state)` at line 217, declared at `HeatBalanceManager.hh` line 142 and implemented at `HeatBalanceManager.cc` lines 3321-3418
10. unconditional `ManageEMS(state, EMSCallFrom::EndZoneTimestepAfterZoneReporting, anyRan, absent)` at line 219
11. unconditional `UpdateEMSTrendVariables(state)` at line 221, declared at `EMSManager.hh` line 122 and implemented at `EMSManager.cc` lines 1444-1479
12. unconditional `PluginManagement::PluginManager::updatePluginValues(state)` at line 222, declared at `PluginManager.hh` line 198 and implemented at `PluginManager.cc` lines 1458-1467
13. the `WarmupFlag && EndDayFlag` outer block at lines 224-226 and its `CheckWarmupConvergence(state)` call, declared at `HeatBalanceManager.hh` line 136 and implemented at `HeatBalanceManager.cc` lines 3059-3226
14. the inner `!WarmupFlag` branch at lines 227-229, assigning `DayOfSim = 0` and then `DayOfSimChr = "0"`
15. `ManageEMS(state, EMSCallFrom::BeginNewEnvironmentAfterWarmUp, anyRan, absent)` at line 231, still inside that branch
16. the lines 235-237 `!WarmupFlag && EndDayFlag && DayOfSim == 1 && !DoingSizing`-guarded `ReportWarmupConvergence(state)` call, declared at `HeatBalanceManager.hh` line 138 and implemented at `HeatBalanceManager.cc` lines 3228-3301

## Current Blocker Ledger

`scripts/compare/official-dynamic-heat-balance-diagnostic.ps1` now requires the
diagnostic JSON and Markdown report to expose `top_blocker`,
`current_blockers`, `warmup_end_state_deltas`, and
`first_divergence_by_variable`. Resolved items stay in this source map with
`closed` status rather than disappearing from the trail.

| Blocker id | Status | Source-map note |
|---|---|---|
| `floor-storage-mismatch` | active | candidate top blocker row; keeps the floor storage delta separate from generic bottleneck ranking |
| `floor-face-temperature-current-inside-mismatch` | open | separate floor current-inside face-temperature row under the storage blocker |
| `ctf-current-term-delta` | open | per-surface CTF current-term row derived from inside/outside current RMSE |
| `ctf-history-temperature-term-delta` | open | per-surface CTF history temperature-term row |
| `ctf-history-flux-term-delta` | open | per-surface CTF history flux-term row |
| `longwave-radiation-source-delta` | open | per-surface inside longwave/radiation source row |
| `hconv-source-timing-delta` | open | per-surface hconv/reference-air source timing row |
| `warmup-end-state-mat-delta` | open | warmup final MAT delta row |
| `warmup-end-state-surface-temperature-delta` | open | first run-period surface-temperature delta used as warmup end-state evidence |
| `warmup-end-state-ctf-history-delta` | open | first run-period CTF history delta used as warmup end-state evidence |
| `ctf-coefficient-eio-seeding` | closed | EIO coefficient availability is resolved as diagnostic isolation; current blocker is source/history handoff, not coefficient presence |
| `source-order-wrapper-boundary` | closed | heat-balance source-order wrappers now exist on the runtime path; remaining rows track algorithm/state deltas behind those wrappers |
| `diagnostic-probe-conformance-aliasing` | closed | diagnostic probe metadata, selector matching, and compatibility source-order execution are separated; diagnostic selectors resolve to a probe-agnostic runtime config before compatibility code runs, so probe output cannot be promoted as conformance evidence |

<a id="diagnostic-probe-lifecycle-ledger"></a>

## Diagnostic Probe Lifecycle Ledger

The blocker ledger above records observed output and state deltas. Its active
and open rows are not, by themselves, source-state hypotheses and therefore do
not determine the number of active probes. A hypothesis in this registry must
name one scalar EnergyPlus state, one source ownership boundary, and one
falsifiable expected observation.

The active probe unit is a full executable lane: its selector, CTF and warmup
policies, iteration and convection settings, report settings, wrapper command,
and expected observation travel together. An enum variant by itself is not an
active probe. Historical probe selectors and wrappers are closed replay
artifacts; they are excluded from the default active suite. They remain
reproducible through the suite's explicit `-IncludeClosed` path and retained
direct wrapper commands. The concise diagnostic boundary below records their
closure; the machine-readable ledger retains each replay identity, while Git
history retains the per-probe results. This does not reopen a closed hypothesis
or make its lane active.

### Active Source-State Hypothesis Registry

<a id="diagnostic-probe-hypothesis:warmup-surftempin-first-sample-state-mismatch"></a>

#### `warmup-surftempin-first-sample-state-mismatch`

| Field | Contract |
|---|---|
| Status | `unresolved` |
| Algorithm / owner routine | `heat_balance_surface_manager_source_order` / `calc_heat_balance_inside_surf_2_ctf_only` |
| EnergyPlus ownership | `src/EnergyPlus/HeatBalanceSurfaceManager.cc::CalcHeatBalanceInsideSurf2CTFOnly` under the `ManageSurfaceHeatBalance` stage order |
| Scalar source state | `SurfTempIn` |
| Hypothesis | After repeated-day warmup, at least one first-run-period CTF surface has different EnergyPlus `SurfTempIn` and Rust inside-face state values at the reporting boundary. This is a direct state-mismatch hypothesis, not a claim that the mismatch's upstream cause has already been identified. |
| Expected observation | With forcing and the compatibility configuration held fixed, the observation-only lane finds the `ZN001:FLR001` first-run-period CTF row whose finite `oracle_inside_face_temperature_c` and `rust_inside_face_temperature_c` values differ by more than `1.0e-9 C`, with their absolute difference equal to `inside_face_temperature_delta_c`. The EnergyPlus `Surface Inside Face Temperature` report exposes `SurfTempIn`; the warmup day-end zone-air trace establishes the repeated-day context. If that direct state mismatch is absent, this hypothesis is rejected rather than converted into a compatibility branch. |
| Latest observation | 2026-07-14 active-lane run: 20 warmup day-end rows; `ZN001:FLR001` sample-`0` EnergyPlus `SurfTempIn` `-0.115727652883 C`, Rust inside-face state `-0.116329096525 C`, absolute delta `0.000601443642 C`. The direct mismatch remains present, so the hypothesis stays unresolved. |
| Active lane | `official-dynamic-heat-balance-warmup-surftempin-first-sample-probe` |

The locked source establishes the ownership boundary: the CTF-only inside solve
calculates `SurfTempInTmp` and copies it to the directly reported `SurfTempIn`,
while its caller continues through the air balance, final surface pass, and
`UpdateThermalHistories`. The hypothesis records only the directly observed
`SurfTempIn` mismatch at that boundary; it does not attribute the mismatch to
`SurfTempInTmp` or assert that the upstream defect has already been located.

## Source Ownership Boundary

The official `1ZoneUncontrolled` dynamic promotion lane retains the following
EnergyPlus 26.1.0 ownership boundaries. `ManageHeatBalance` calls initialization,
surface work, before-reporting EMS, `RecKeepHeatBalance`, `ReportHeatBalance`,
after-reporting EMS, trend/plugin updates, and guarded warmup convergence in
source order. Warmup convergence is checked only at end-of-day.

- `HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance` calls
  `InitSurfaceHeatBalance`, `CalcHeatBalanceOutsideSurf`,
  `CalcHeatBalanceInsideSurf`, `HeatBalanceAirManager::ManageAirHeatBalance`,
  `UpdateFinalSurfaceHeatBalance`, conditionally `UpdateThermalHistories`,
  conditionally updates CondFD moisture over the filtered complete Surface
  array, then calls `ManageThermalComfort`, `ReportSurfaceHeatBalance`,
  conditionally gathers sizing component loads, calculates thermal resilience,
  conditionally emits the three resilience reports, and clears its first-time
  flag. Four first-time progress displays precede initialization, outside,
  inside, and air work respectively.
- `CalcHeatBalanceInsideSurf2CTFOnly` freezes each surface's
  `SurfTempOutHist` from `SurfOutsideTempHist(1)`, stores
  `Surface::getInsideAirTemperature` in `RefAirTemp` and
  `SurfTempEffBulkAir`, builds `SurfTempTerm`, and only re-evaluates
  `Convect::InitIntConvCoeff` on the `ItersReevalConvCoeff` cadence.
- The CTF-only inside solve updates `SurfTempInTmp`, then copies it to
  `SurfTempIn`. For reporting, `SurfTempOut` is set from
  `SurfOutsideTempHist(1)`. Interzone surfaces are then synchronized by copying
  the paired inside history into `SurfOutsideTempHist(1)` and
  `SurfTempOutHist`; a direct current-inside adiabatic report sync is not the
  generic EnergyPlus reporting path.
- `UpdateThermalHistories` computes current inside flux from
  `SurfOutsideTempHist(1)`, current `SurfTempIn`, and
  `SurfCTFConstInPart`; computes current outside flux from the same current
  outside history slot and current `SurfTempIn`; writes report conduction
  variables; then shifts thermal histories. In `SimpleCTFOnly`, slot `1` is
  hard-copied to slot `2` after shifting.
- `ZoneTempPredictorCorrector.cc::calcSumHAT` builds solver coefficients from
  `SurfHConvInt * Area * SurfTempInTmp` plus `SurfTAirRef`-dependent
  `SumHA`/`SumHATref`. This is separate from the zone-air heat-balance report
  row.
- `ZoneTempPredictorCorrector.cc::CalcZoneComponentLoadSums` reports
  `SumHADTsurfs` by re-walking heat-transfer surfaces with
  `Surface::getInsideAirTemperature`, `SurfHConvInt`, and `SurfTempInTmp`.
  Therefore the zone `Surface Convection Rate` row is not required to equal the
  signed sum of individual surface inside convection report rows.
- `DataHeatBalance.cc::AirReportVars::setUpOutputVars` registers `Zone Mean Air
  Temperature` as `Zone/Average`, but registers `Zone Air Heat Balance Internal
  Convective Heat Gain Rate`, `Zone Air Heat Balance Surface Convection Rate`,
  and `Zone Air Heat Balance Air Energy Storage Rate` as `System/Average`.
  Rust keeps the default hourly average report mode and exposes a diagnostic
  `zone_air_report_sampling=last-system-state` probe only to isolate timing
  differences.
- `HeatBalanceSurfaceManager.cc::CalcHeatBalanceInsideSurf*` registers the
  advanced `Surface Inside Face Heat Balance Calculation Iteration Count` row
  with key `Simulation`, `TimeStepType::Zone`, and `StoreType::Sum`. It reports
  `InsideSurfIterations`, which is reset before the inside-surface convergence
  loop, increments after each pass, and triggers `InitIntConvCoeff` whenever
  `InsideSurfIterations % ItersReevalConvCoeff == 0`.
- The same source order keeps zone-air correction outside the
  inside-surface convergence loop: `CalcHeatBalanceInsideSurf*` converges
  `SurfTempIn`/`SurfTempInTmp` first, then the caller proceeds to air heat
  balance/reporting and final history update work. Both the broad
  `energyplus-heat-balance-compat-candidate` diagnostic wrapper and the
  promoted official compatibility candidate now pin
  `surface_loop_zone_air_correction=after-surface-loop` for this source-order
  parity point; `each-surface-iteration` remains available only as an explicit
  comparison probe.

## Bounded Window Frame And Divider Input Notes

EnergyPlus 26.1.0 calls `GetFrameAndDividerData` after
`Material::GetHysteresisData` and before `GetConstructData`. Rust instead
places `WindowProperty:FrameAndDivider` after base `parse_materials` and before
`parse_constructions`; its existing separate
`parse_material_phase_change_hystereses` pass remains later. Therefore this
checkpoint claims only the base-material/frame/construction relative order,
not Hysteresis-relative or complete `GetHeatBalanceInput` pass-order parity,
downstream surface binding, or any window calculation.

The bounded input owns an independent normalized name map and immutable frame,
divider, reveal, and NFRC descriptors. It accepts every source/schema numeric
field and both enums, applies only the source-effective corrections recorded
below, and validates every definition even when no surface references it. All
typed definitions are explicitly rejected at the runtime support boundary.

### `GetFrameAndDividerData` state contract

<!-- routine-state-contract:v1 begin get_frame_and_divider_data -->
GetFrameAndDividerData

read_state:
- EnergyPlus source places `GetFrameAndDividerData` after `Material::GetHysteresisData` and before `GetConstructData`; Rust deterministically reads `WindowProperty:FrameAndDivider` after base `parse_materials` and before `parse_constructions`, while its separate `parse_material_phase_change_hystereses` pass remains later; every definition is read eagerly, including an all-default, reveal-only, or unused record
- one required nonblank outer-key name in an independent normalized namespace; Divider Type defaults to `DividedLite` and accepts `DividedLite` or `Suspended`, while NFRC Product Type defaults to `CurtainWall` and accepts `CasementDouble`, `CasementSingle`, `DualAction`, `Fixed`, `Garage`, `Greenhouse`, `HingedEscape`, `HorizontalSlider`, `Jal`, `Pivoted`, `ProjectingSingle`, `ProjectingDual`, `DoorSidelite`, `Skylight`, `SlidingPatioDoor`, `CurtainWall`, `SpandrelPanel`, `SideHingedDoor`, `DoorTransom`, `TropicalAwning`, `TubularDaylightingDevice`, or `VerticalSlider`
- all 23 numeric fields as finite values with source-effective defaults and bounds: frame width [0,1] default 0, two frame projections [0,0.5] default 0, nonnegative frame conductance default 0 without upper bound, frame edge/center ratio (0,4] default 1, frame solar/visible absorptance [0,1] default 0.7, frame emissivity greater than 0 without upper bound default 0.9; divider width [0,0.5] default 0, two counts default 0, two divider projections [0,0.5] default 0, nonnegative divider conductance default 0 without upper bound, divider edge/center ratio (0,4] default 1, divider solar/visible absorptance [0,1] default 0, divider emissivity (0,1) default 0.9; three reveal absorptances [0,1] default 0 and two reveal depths [0,2] default 0
- horizontal and vertical divider counts accept finite values in [0,2147483648), truncate toward zero into nonnegative integer state, and have no cross-field validity requirement; no material, construction, surface, schedule, geometry, or usage dependency is read

write_state:
- a separate deterministic `WindowFrameAndDivider` arena and normalized name map; each record owns a typed ID, normalized name, nested `WindowFrameProperties`, `WindowDividerProperties`, and `WindowRevealProperties`, and one `WindowNfrcProductType`
- source correction order first clears both frame projections when frame width is zero, then clears both divider projections when divider width is zero or type is `Suspended`, then warns and resets a still-positive divider width when both effective counts are zero without revisiting the prior projection result, and finally warns and raises an inside sill shallower than the inside reveal to the reveal depth
- source-fixed frame and divider edge widths of 0.06355 m; no WINDOW 5 mullion orientation or synthesized frame/divider state is invented
- two nonblocking typed diagnostics identify the positive-divider-width/zero-count reset and the inside-sill-depth reset without claiming EnergyPlus warning text, severity, order, or multiplicity
- compile failure before typed ID or normalized-name reservation for a blank name, malformed or out-of-range numeric, count outside the finite [0,2147483648) boundary, invalid enum, or case-insensitive duplicate name; normalized duplicate rejection and invalid-enum rejection deliberately fail closed relative to source case-collision lookup and invalid-token fallback behavior

history_state_ownership:
- TypedModel owns immutable frame/divider/reveal descriptors; this checkpoint allocates no mutable window history

unsupported_state:
- fenestration-surface and legacy Window or GlazedDoor reference binding, first-match case-collision behavior, geometry and projected area state, frame/divider edge areas, reveal and sill geometry, and surface-local frame/divider pointers
- WINDOW 5 synthesized records and mullion orientation, between-glass shading mutation of shared divider width, window optical and thermal state, NFRC assembly calculations, reports, output variables, and runtime numerical state

inactive_branches:
- zero frame width silently leaves both frame projections at zero; zero divider width or `Suspended` silently leaves both divider projections at zero; a positive divider width with zero effective counts resets only the width after projection correction, so positive divided-lite projections intentionally remain possible beside the recovered zero width
- all-default, reveal-only, unused, non-fenestration-referenced, and names colliding with Material, Construction, or BuildingSurface namespaces remain valid typed records because this input has no cross-object dependency

unsupported_active_branches:
- every valid `WindowProperty:FrameAndDivider` definition is typed but reported as `UnsupportedSurfaceBoundary` and `RunBlocked`, including all-default and unused definitions; no partial run is allowed
- valid frame emissivity above 1, fractional divider counts truncated toward zero, source-recovered positive-width/zero-count input, and source-recovered shallow-sill input remain typed but do not activate a runtime or reporting consumer

not_claimed_branches:
- Hysteresis-relative Rust compiler pass order and complete `GetHeatBalanceInput` order parity, native-epJSON canonical enum casing, source case-colliding name lookup and declaration ordering, invalid-enum fallback, exact diagnostic text/severity/order/multiplicity, fenestration binding, geometry, WINDOW 5 synthesis, between-glass shading mutation, optics, thermal execution, NFRC assembly calculations, EIO/SQLite or other reporting, runtime numerics, and conformance
<!-- routine-state-contract:v1 end get_frame_and_divider_data -->

## Bounded Ordinary Construction Input Notes

EnergyPlus 26.1.0 calls `GetConstructData` after `GetFrameAndDividerData` and
before `GetBuildingData`. The routine reads all ordinary `Construction`
objects first, then calls the separately state-mapped F-factor/C-factor
generator, AirBoundary reader, and `SetupComplexFenestrationStateInput` before
entering its inline internal heat source overlay, equivalent-layer construction
family, and final WindowDataFile request branch. The parent routine therefore
remains `source_mapped`: the ordinary path and bounded F/C/Air/CFS/
InternalHeatSource/WindowEquivalentLayer/WindowDataFile request readers do not
imply completion of `SearchWindow5DataFile` or any downstream CTF/QTF, surface,
window, or source/sink consumer.

The bounded ordinary input retains its required construction name and ordered
material prefix. The bounded compiler retains source `GlassTCParent` handling:
every typed
thermochromic parent is replaced in the effective stack by its first glazing
state, while the final parent encountered owns the source-shaped zero-based
master metadata. Effective model-graph edges follow the substituted glazing
IDs. Checked direct-index construction/material and opaque-cache lookups are
structural hardening only; the existing thermochromic parent capability keeps
all such definitions runtime-blocked, and no window execution is promoted. The
later `CreateTCConstructions` slice derives separate immutable series and
child snapshots from this metadata without changing this parent routine's
ConstructionIds, names, graph, or runtime boundary.

The bounded compiler accepts the intended whole-system SimpleGlazing form only when it is
the sole ordinary-`Construction` layer. The original SimpleGlazing material ID
is retained as the outside layer, sole ordered layer, and graph target, and the
construction is classified as fenestration. Multi-layer source-validation
holes and every window runtime consumer remain fail-closed.

The internal heat-source reader types only the source-declaration overlay on an already materialized
ordinary opaque construction with at least two effective layers. It preserves
the direct epJSON map's lexical instance-key order, treats the required outer
name as a nonsemantic normalized diagnostic snapshot, and validates both
one-based layer interfaces plus the bounded dimension, spacing, and optional
perpendicular-position fields before attaching immutable metadata. The target
keeps the same identity, kind, layer stack, and material graph. Source recovery
quirks, broader construction-family targets, global source flags, and all
CTF/QTF and runtime consumers remain fail-closed.

The following reader types `Construction:WindowEquivalentLayer` declarations.
Staged IDF preserves family-local declaration order through the `getObjectItem`
order overlay, while native epJSON retains lexical instance-key order. A
required outside layer and optional layers 2 through 11 form a contiguous pack;
every reference must resolve to one of the six already typed equivalent-layer
material variants. GetConstructData itself does not impose solid/gap
alternation, so the bounded declaration retains one to eleven family-valid
layers without inventing downstream topology repair or ASHWAT behavior. Every
definition remains run-blocked.

The final reader types only the `Construction:WindowDataFile` request surface. Its
required WINDOW5 entry name is normalized for matching, a missing or blank file
field retains the current-working-folder `Window5DataFile.dat` selector, and an
explicit file name remains retain-case text. Staged IDF uses the family order
overlay and native epJSON stays lexical. No file is opened, and the request
creates no construction name/ID, material, frame/divider, graph, or optical
state; all requests are blocked before I/O. The source's external-file expansion
remains mapped under `SearchWindow5DataFile`.

### `GetConstructData` bounded ordinary state contract

<!-- routine-state-contract:v1 begin get_construct_data -->
GetConstructData

read_state:
- EnergyPlus calls `GetConstructData` after `GetFrameAndDividerData` and before `GetBuildingData`; within it, every ordinary `Construction` is read before the separately state-mapped F-factor then C-factor generation pass, AirBoundary pass, and SetupComplexFenestrationStateInput pass, followed by the bounded InternalHeatSource overlay, bounded WindowEquivalentLayer family, and bounded request-only WindowDataFile front end whose external-file expansion remains deferred
- one required nonblank outer-key name in an independent normalized construction namespace, one required outside-layer material name, and optional layers 2 through 10 that must form a contiguous populated prefix; every populated layer resolves through the typed material namespace
- every typed `WindowMaterial:GlazingGroup:Thermochromic` parent encountered in layer order contributes its first ordered glazing state to the effective layer stack; the existing typed parent contract guarantees at least one state
- a `WindowMaterial:SimpleGlazingSystem` material is accepted only when it is the sole ordinary-Construction layer, matching the intended whole-system input while excluding EnergyPlus 26.1's multi-layer validation holes
- all layer references, contiguous-prefix rules, bounded opaque/fenestration family and topology checks, unsupported-consumer checks, and normalized duplicate checks complete before a `ConstructionId` or name is reserved
- the `ConstructionProperty:InternalHeatSource` epJSON instance map is read directly in case-sensitive lexical outer-key order and deliberately remains outside staged IDF declaration-order recovery; its required nonblank outer key is a normalized diagnostic snapshot without a typed ID or name map, so case variants may coexist when they target different constructions
- each internal-source record has a required nonblank case-insensitive shared-Construction reference restricted by the bounded contract to an ordinary opaque construction with at least two effective layers; both required one-based layer interfaces are positive integers in `1..TotLayers-1`, need not match, and have no ordering relationship
- internal-source CTF dimensions are exactly integer 1 or 2, required finite tube spacing is in `[0.01,1.0]` m and derives a retained perpendicular half-spacing, and the optional finite perpendicular temperature position defaults to 0 in `[0,1]` and remains retained but inactive for a 1-D declaration
- each `Construction:WindowEquivalentLayer` has a required nonblank name in the shared case-insensitive Construction namespace, one required outside layer, and optional layers 2 through 11 forming a contiguous one-to-eleven-layer prefix; every reference resolves case-insensitively to an already typed `MaterialFamily::EquivalentLayer` glazing, shade, drape, blind, screen, or gap definition
- EnergyPlus reads equivalent-layer constructions through `getObjectItem`; staged IDF therefore preserves family-local declaration order through the explicit order overlay, while native epJSON without that overlay retains case-sensitive lexical instance-key order
- each `Construction:WindowDataFile` request has a required nonblank WINDOW5 entry name and optional retain-case `file_name`; missing or blank input selects the explicit current-working-folder default `Window5DataFile.dat`, while nonblank text is retained without path resolution or file access
- EnergyPlus also reads WindowDataFile requests through `getObjectItem`; staged IDF preserves family-local declaration order through its explicit order overlay, while native epJSON without that overlay retains case-sensitive lexical instance-key order

write_state:
- the deterministic `Construction` arena and independent normalized name map retain each valid ID, normalized name, bounded opaque-or-fenestration kind, effective outside layer, and ordered effective outside-to-inside material stack
- every thermochromic parent layer is replaced by its first-state glazing `MaterialId`; the final parent encountered overwrites the source-shaped zero-based parent-material-layer and glazing-layer metadata slots while earlier parent substitutions remain in the effective stack
- a sole-layer SimpleGlazingSystem construction is retained as Fenestration with the original material ID in outside-layer, ordered-layer, and construction/material graph state; no detailed-glazing child is synthesized
- construction/material model-graph edges are emitted from the effective material stack, so thermochromic edges target first-state glazing materials rather than parent group descriptors
- runtime construction/material resolution and opaque construction thermal-cache building use checked direct-index lookups; these structural paths do not enable thermochromic or other window execution
- one immutable internal-source descriptor on each valid unique target retains its normalized diagnostic source name, one-based source and requested-temperature interfaces, 1-D or 2-D selector, authored tube spacing, derived perpendicular half-spacing, and perpendicular temperature position without changing Construction identity, kind, layers, other construction-family metadata, material state, or graph edges
- all internal-source fields and target constraints are validated before attachment; a malformed or missing earlier lexical record reserves no target, the first valid record attaches, and a later valid record for the same `ConstructionId` fails closed without overwriting the first descriptor
- one dedicated `ConstructionKind::WindowEquivalentLayer` record per valid equivalent-layer definition after the InternalHeatSource overlay, retaining an explicit outside material, ordered material stack and construction/material graph edges, plus a zero-based family source index corresponding to EnergyPlus's one-based `EQLConsPtr` ordinal
- all equivalent-layer fields, references, material families, contiguity, and shared-name duplication are validated before identity reservation; GetConstructData's family-only acceptance is preserved without inventing alternating solid/gap topology or downstream ASHWAT semantics
- one immutable `ConstructionWindowDataFileRequest` record per valid final-family input retains its normalized search-name snapshot, default-or-explicit retain-case file selector, and zero-based source ordinal in a separate arena
- request publication creates no `ConstructionId`, shared construction-name entry, material, frame/divider, graph edge, or optical state; name collisions remain request data until the deferred file parser can validate and materialize source identities

history_state_ownership:
- TypedModel owns immutable construction layer stacks, optional thermochromic master metadata, optional internal-source declaration metadata, equivalent-layer declaration ordinals, and WINDOW5 request selectors; this checkpoint creates no thermochromic child constructions, global source-presence flag, CTF/QTF or source-node history, equivalent-layer ASHWAT state, WINDOW5 synthesized state, active-state history, or mutable window/surface state

unsupported_state:
- multi-layer or shaded ordinary `Construction` consumption of `WindowMaterial:SimpleGlazingSystem`
- InternalHeatSource lookup or overlay of F-factor, C-factor, AirBoundary, window, complex-fenestration, later equivalent-layer, or WindowDataFile constructions; source warning recoveries for invalid layer interfaces, dimensions, spacing, or perpendicular position; and thermochromic child propagation
- `SetPreConstructionInputParameters` maximum-layer projection, `CheckAndFixCFSLayer`, `FinalizeCFS`, `SetEquivalentLayerWindowProperties`, ASHWAT optical and thermal calculations, nominal equivalent-layer resistance, U-factor/SHGC/rating state, and every equivalent-layer surface or window consumer
- `SearchWindow5DataFile` path lookup and fatal behavior, Unicode/header/EOF and entry matching, one- or two-glazing-system parsing, W5 glass/gas material and construction generation, nominal resistance and U state, angular optical arrays and polynomial fits, frame/divider and mullion synthesis, source collision behavior, and every generated surface, reporting, or runtime consumer
- the global `AnyInternalHeatSourceInInput` and `SimpleCTFOnly` flags, resistance-layer merging and source-node remapping, CTF/QTF generation and histories, source/sink heat fluxes and interior temperatures, radiant-system, ventilated-slab, surface-ground-heat-exchanger, representative-surface, and other downstream consumers
- `CreateTCConstructions` global child allocation and deep copying, ConstructionId/name/count/graph integration, surface-active construction switching, temperature-driven state selection, fenestration binding, optics, thermal calculations, daylighting, shading, nominal-U adjustment, EIO or other construction reporting, and runtime or conformance behavior; CP103's separate immutable projection is covered by its later dedicated contract

inactive_branches:
- ordinary constructions without a thermochromic parent retain their existing effective material stacks and bounded opaque/fenestration classification; for these records the input and effective stacks are identical
- a sole-layer SimpleGlazingSystem construction retains its one source material ID and has no thermochromic metadata; no multi-layer source quirk is materialized
- when more than one thermochromic parent is present, every parent is first-state substituted but only the final parent owns the zero-based master metadata; the later CP103 projection varies only that final retained parent while earlier occurrences stay at their first states
- when no valid `ConstructionProperty:InternalHeatSource` targets an ordinary opaque construction, every construction retains absent internal-source metadata; a retained nonzero perpendicular position on a 1-D declaration has no active consumer
- when no `Construction:WindowEquivalentLayer` definition exists, the construction arena and graph gain no equivalent-layer state; when definitions exist, any one-to-eleven-layer contiguous family-only pack remains declaration data without topology repair or an executable window consumer
- when no `Construction:WindowDataFile` request exists, the request arena is empty; missing or blank file names retain the default selector, and explicit names remain inert retain-case text even when the referenced file does not exist

unsupported_active_branches:
- every typed thermochromic parent remains an all-definition runtime blocker through its existing parent-material capability rule, including an unused parent and a parent consumed by a valid ordinary `Construction`; direct-index structural lookup and the later immutable child projection do not weaken that block
- every typed SimpleGlazingSystem definition remains an all-definition runtime blocker, including an unused definition and one consumed by a valid sole-layer ordinary `Construction`
- valid bounded fenestration constructions remain typed graph state only and do not enter the opaque runtime thermal cache or acquire window execution
- every valid `ConstructionProperty:InternalHeatSource` definition, including one attached only to an otherwise unused construction, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None`; a `BuildingSurface:Detailed` may retain the still-ordinary opaque target identity, but no partial or compatibility runtime is admitted
- every valid `Construction:WindowEquivalentLayer` definition, including every unused definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None`; supported equivalent-layer materials and graph edges do not admit surface binding, reporting, or partial window runtime
- every valid `Construction:WindowDataFile` request, including every unused, defaulted, or missing-file request, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None` before file I/O; no synthesized construction identity or partial runtime is admitted

not_claimed_branches:
- complete `GetConstructData` parity, source case-collision and exact duplicate-key behavior, invalid-object and mark-used side effects, exact diagnostics/order/multiplicity, multi-layer or shaded SimpleGlazingSystem source quirks, thermochromic global child ConstructionId/name/count/graph integration and active-state selection beyond CP103's private projection, `SearchWindow5DataFile` external-file expansion and generated state, broad InternalHeatSource target and recovery quirks, equivalent-layer topology repair and ASHWAT consumers, global flags, CTF/QTF calculations, nominal-U, EIO/SQLite and other reporting, window, air-boundary, ground, radiant, or source/sink physics, runtime numerics, and conformance
<!-- routine-state-contract:v1 end get_construct_data -->

## Bounded F/C-Factor Construction Generation Notes

EnergyPlus allocates the common concrete and per-object fictitious insulation
materials from raw F/C counts during material input, then
`CreateFCfactorConstructions` creates every F-factor construction before every
C-factor construction. Rust preserves that family and staged-IDF declaration
order, keeps generated material names private, and retains the exact bounded
input formulas and graph. Surface pairing and ground runtime remain blocked.

### `CreateFCfactorConstructions` state contract

<!-- routine-state-contract:v1 begin create_fc_factor_constructions -->
CreateFCfactorConstructions

read_state:
- raw `Construction:FfactorGroundFloor` and `Construction:CfactorUndergroundWall` counts allocate source-ordered private material slots: public `Material`, common `~FC_Concrete`, public `Material:NoMass`, then one `~FC_Insulation_n` for every raw F object followed by every raw C object; staged IDF declaration-order overlays preserve ordinary, F-family, and C-family ordinals independently
- each F-factor record has a required nonblank shared Construction name plus required finite `f_factor` and `area` greater than zero and exact epJSON `perimeterexposed` greater than or equal to zero; positive perimeter derives `Reff = area/(perimeter*f_factor)-0.135-0.03`, while zero perimeter derives `Reff = 177`
- each C-factor record has a required nonblank shared Construction name plus required finite `c_factor` and `height` greater than zero; equivalent soil resistance is 0.12 at height less than or equal to 0.25 m, 0.92 at height greater than or equal to 2.5 m, and `0.0607+0.3479*height` between, with `Reff = 1/c_factor+soil resistance`
- the common MediumRough concrete has thickness 0.15 m, conductivity 1.95 W/m-K, density 2240 kg/m3, specific heat 900 J/kg-K, and default opaque absorptances; each MediumRough resistance-only insulation has all three absorptances zero and derives `Rfic = Reff-(0.15/1.95)`, which must be finite and greater than zero together with finite `Reff`

write_state:
- one typed opaque `Construction` per valid record after every ordinary Construction and in all-F-then-all-C order, with a distinct F-factor or C-factor metadata discriminant retaining source inputs, derived effective resistance, generated insulation resistance, and C-factor soil resistance where applicable
- each generated construction owns the outside-to-inside layer stack `[private insulation, private concrete]`; the construction/material graph emits both ordered edges and the raw-ordinal insulation slot receives the derived resistance
- internal materials live in the typed material arena but are deliberately absent from public `material_names` and every public attachment-target lookup; exact normalized generated-name collisions from user material families fail closed, while nonidentical names such as `~FC_Insulation_01` remain public
- invalid fields, nonpositive or nonfinite derived insulation resistance, and duplicate Construction names reserve no Construction ID or name; raw-count internal slots and later-family ordinals remain stable, while exact EnergyPlus invalid-object and collision side effects are not reproduced

history_state_ownership:
- TypedModel owns immutable generated ground-factor construction metadata and layer stacks; no surface-local ground-temperature, CTF, CondFD, HAMT, or mutable heat-balance history is allocated

unsupported_state:
- `GroundFCfactorMethod` surface-type pairing, detailed-surface and rectangular-surface boundary conversion, F area/perimeter and C height geometry checks, `Site:GroundTemperature:FCfactorMethod`, EPW fallback, monthly ground state, CTF/CondFD/HAMT calculation, and ground heat-balance execution
- public attachment targeting of private internal materials, EMS mutation, bounded-out `ConstructionProperty:InternalHeatSource` targeting of generated F/C constructions, nominal-U adjustment, generic material/construction EIO timing, envelope/SQLite reporting, exact diagnostics/order/multiplicity, and conformance evidence

inactive_branches:
- when no raw F-factor or C-factor definition exists, no internal concrete or insulation material is injected and ordinary material/construction behavior is unchanged
- F-factor exposed perimeter exactly zero uses the source 177 m2-K/W effective-resistance branch; C-factor heights exactly 0.25 m and 2.5 m use the lower and upper constant soil-resistance branches respectively

unsupported_active_branches:
- every valid F-factor or C-factor definition, including every unused definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked`; no partial or compatibility runtime is admitted
- `BuildingSurface:Detailed` deliberately rejects a generated F/C-factor construction until the downstream `GroundFCfactorMethod` pairing and validation checkpoint is ported; reporting and runtime consumers likewise filter generated stacks from ordinary opaque constructions

not_claimed_branches:
- surface binding and geometry validation, ground-temperature and CTF/runtime behavior, internal-material public attachment behavior, source collision and invalid-object side effects, no-thermal-mass flags, nominal-U and reporting parity, exact diagnostics/order/multiplicity, runtime numerics, EIO/SQLite, and conformance
<!-- routine-state-contract:v1 end create_fc_factor_constructions -->

## Bounded Air-Boundary Construction Input Notes

EnergyPlus calls `CreateAirBoundaryConstructions` after ordinary construction
input and F/C-factor generation. Unlike those preceding families, it iterates
the epJSON object map directly, so the bounded Rust pass deliberately retains
case-sensitive lexical instance-key order instead of adding an IDF declaration
order overlay. Rust waits until the typed top-level schedule namespace is
complete before materializing these records, while still appending them after
all ordinary, F-factor, and C-factor constructions. This compiler pass-order
deviation is explicit and does not claim complete `GetConstructData` parity.

The checkpoint owns declaration state only. Official AirBoundary EIO evidence
depends on deferred surface conversion, enclosure grouping, and generated
cross-mixing, so no report comparator or numerical conformance claim is added.

### `CreateAirBoundaryConstructions` state contract

<!-- routine-state-contract:v1 begin create_air_boundary_constructions -->
CreateAirBoundaryConstructions

read_state:
- the `Construction:AirBoundary` epJSON object map after every ordinary, F-factor, and C-factor construction; EnergyPlus iterates that map directly, so records materialize in case-sensitive lexical key order rather than staged source-IDF declaration order, and Rust preserves the same family-local order
- one required nonblank outer-key name in the shared case-insensitive Construction namespace; `air_exchange_method` missing or blank defaults to `None` and otherwise accepts case-insensitive `None` or `SimpleMixing` within the Rust parser convention
- a supplied nonblank `simple_mixing_air_changes_per_hour` must be finite and nonnegative without an upper bound; `SimpleMixing` applies the source default 0.5 when it is missing or blank, while `None` retains source-effective zero mixing state
- only `SimpleMixing` consumes `simple_mixing_schedule_name`: missing or Rust-normalized blank input selects an explicit always-on sentinel, while a nonblank name resolves case-insensitively through the existing typed top-level schedule namespace after Rust schedule materialization; `None` ignores the schedule field and creates no schedule dependency

write_state:
- one dedicated zero-layer air-boundary construction descriptor per valid record, appended after every C-factor construction with a shared `ConstructionId` and normalized Construction name but no outside material, material layer, thermochromic or ground-factor metadata, or construction/material graph edge
- `None` retains false simple-mixing state, zero air changes per hour, and no schedule selector; inactive supplied mixing fields do not create executable state
- `SimpleMixing` retains true mixing state, the effective nonnegative air-changes-per-hour value, and either a resolved user `ScheduleId` or an explicit always-on selector without synthesizing another typed Schedule object
- blank names, malformed or out-of-range numeric input, invalid enums, unresolved active schedule references, and normalized duplicate Construction names fail before Rust ID or name reservation; this deliberately does not reproduce source partial-slot and name-reservation side effects after an active schedule lookup failure

history_state_ownership:
- TypedModel owns immutable zero-layer air-boundary and optional simple-mixing descriptors; no surface pair, enclosure, cross-mixing object, schedule current value, or mutable heat-balance history is allocated

unsupported_state:
- base-surface or fenestration-surface binding, `Surface` or `Zone` outside-boundary validation, paired nonadiabatic interzone checks, child-fenestration exclusion, `AirBoundaryNoHT` surface conversion, and surface heat-transfer suppression
- solar, radiant, and daylighting enclosure grouping, `AnyAirBoundary`, view-factor and enclosure remapping, and every geometry-dependent surface mutation
- space-pair deduplication, minimum-space-volume times air-changes-per-hour divided by 3600 flow derivation, AirflowNetwork suppression, generated `ZoneCrossMixing`, schedule current-value consumption, zone mixing calculations, and mixing output variables
- nominal-U and CTF skip behavior beyond the typed zero-layer flag, construction or surface EIO/SQLite reporting, exact diagnostics, and conformance evidence

inactive_branches:
- when no `Construction:AirBoundary` definition exists, ordinary, F-factor, and C-factor construction behavior is unchanged
- `None` leaves simple mixing disabled with zero air changes and no schedule selector even when inactive mixing fields are supplied; their schedule name is not resolved
- `SimpleMixing` with no effective schedule name retains only the explicit always-on selector; no schedule sampling or downstream mixing state is activated

unsupported_active_branches:
- every valid `Construction:AirBoundary` definition, including every unused `None` or `SimpleMixing` definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked`; no partial or compatibility runtime is admitted
- `BuildingSurface:Detailed` deliberately rejects an air-boundary construction until interzone pairing, enclosure grouping, no-heat-transfer surface conversion, and optional simple-mixing consumers are ported; a supported referenced schedule does not weaken this block

not_claimed_branches:
- complete `GetConstructData` or Rust compiler pass-order parity, source case-collision lookup and invalid-after-reservation side effects, explicit-empty native-epJSON behavior, canonical enum casing, exact diagnostics/order/multiplicity, surface and enclosure behavior, schedule sampling, AirflowNetwork interaction, zone mixing, reporting, runtime numerics, EIO/SQLite, and conformance
<!-- routine-state-contract:v1 end create_air_boundary_constructions -->

## Bounded Complex-Fenestration State Input Notes

EnergyPlus calls `SetupComplexFenestrationStateInput` after AirBoundary input
and before `ConstructionProperty:InternalHeatSource`. The source uses
`getObjectItem` for complex states, so staged IDF input retains family-local
declaration order; native epJSON without the overlay follows its object-map
order. `ConvertInputFormat` can reorder that map, so broad IDF/native-epJSON
order parity is not claimed.

This checkpoint owns only the bounded LBNLWINDOW/None declaration graph. A CFS
presence activates all-definition validation of raw `WindowThermalModel:Params`
helpers. `Matrix:TwoDimension` remains lazy until at least one nonblank CFS
name survives the shared Construction-name collision gate; the first surviving
candidate then validates every matrix definition, including unused ones.
Thermal and matrix identities resolve through normalized case-insensitive keys,
while retained matrix snapshots preserve their original spelling. Neither raw
helper family enters public typed object coverage.

The only EnergyPlus EIO-observable complex-window rows require deferred
fenestration-surface, BSDF optical, or complex thermal consumers. No manifest,
comparator, proof variable, numerical runtime, or conformance claim is added.

### `SetupComplexFenestrationStateInput` state contract

<!-- routine-state-contract:v1 begin setup_complex_fenestration_state_input -->
SetupComplexFenestrationStateInput

read_state:
- the `Construction:ComplexFenestrationState` family after every ordinary, F-factor, C-factor, and AirBoundary construction; staged IDF input preserves family-local declaration order, while native epJSON without that overlay retains lexical object-map order
- one required nonblank name in the shared case-insensitive Construction namespace; missing or blank basis fields default to `LBNLWINDOW` and `None`, and only that combination is admitted while `UserDefined` and `Axisymmetric` fail closed
- when at least one complex state exists, every raw-only `WindowThermalModel:Params` definition is validated before state inspection; `Matrix:TwoDimension` loading remains lazy until one nonblank state name passes the shared Construction-name collision gate, then every matrix definition including unused entries is validated before any surviving state identity is materialized
- positive finite matrix shapes and effective row-major prefixes drive a one- or two-column LBNL basis, four square global optical matrices, and one-row front/back absorptance matrices per solid; layers form a contiguous alternating one-to-five-solid and zero-to-four-gap pack ending in a solid, accepting only SpectralAverage glazing or ComplexShade at solid positions and WindowMaterial:Gap at gap positions with reserved gap matrix fields blank

write_state:
- one `ConstructionKind::ComplexFenestration` record per valid definition after every AirBoundary record, with shared `ConstructionId`, normalized name, outside layer, ordered material stack, no thermochromic, ground-factor, or air-boundary metadata, and staged-IDF family order when available
- immutable `ConstructionComplexFenestrationState` metadata retains LBNL/None selectors, the resolved thermal-model snapshot, basis matrix and derived basis length, four global optical matrix snapshots, and ordered solid-layer front/back absorptance snapshots; thermal and matrix lookup plus duplicate detection are normalized case-insensitively, retained matrix snapshots preserve original spelling, and raw helper families remain outside public typed object coverage
- ordered construction/material graph edges cover every solid and gap layer, and material-family validation prevents ordinary, equivalent-layer, simple-glazing, thermochromic, or unsupported BSDF glazing variants from entering the pack
- all helper catalogs, references, matrix dimensions, layer topology, reserved fields, and shared-name duplication are validated before identity reservation; an invalid earlier state does not consume the shared name needed by a later valid case-variant definition

history_state_ownership:
- TypedModel owns immutable declaration, helper snapshots, and ordered layer graph state; no BSDF basis cache, surface-local optical/thermal state, deflection history, TARCOG/WCE state, or mutable window history is allocated

unsupported_state:
- `UserDefined` bases, `Axisymmetric` symmetry, regular-glazing `BSDF`, Spectral and SpectralAndAngle glazing consumers, nonblank reserved gap matrix fields, unsupported solid/gap material families, and any layer topology outside the bounded alternating pack
- fenestration-surface binding, complex-window initialization, basis expansion beyond retained input snapshots, BSDF optical calculations, deflection and support-pillar algorithms, TARCOG/WCE thermal calculations, shading, ratings, daylighting, nominal-U or CTF state, output variables, and EIO/SQLite or other reporting

inactive_branches:
- when no `Construction:ComplexFenestrationState` definition exists, raw thermal-model and matrix helpers are not parsed by this pass; when definitions exist but every nonblank name collides with the shared Construction namespace, thermal helpers are still validated but the matrix catalog remains untouched
- a one-column basis uses its row count directly; a two-column basis uses the EnergyPlus count projection retained by the bounded LBNL branch, while extra finite matrix values beyond rows times columns are ignored
- a single solid needs no gap, while each higher populated position must preserve solid-gap alternation and every omitted trailing layer leaves no typed or graph state

unsupported_active_branches:
- every valid `Construction:ComplexFenestrationState` definition, including every unused definition, is reported as `UnsupportedSurfaceBoundary` and `RunBlocked` with `RuntimeClass::None`; no partial or compatibility runtime is admitted
- `BuildingSurface:Detailed` deliberately rejects a complex-fenestration construction until the dedicated fenestration-surface, BSDF optical, and complex-window thermal consumers are ported; supported material and helper declarations do not weaken that block

not_claimed_branches:
- complete `GetConstructData` parity, source partial-slot allocation and invalid-object side effects, broad IDF/native-epJSON order parity, source case-collision behavior, exact diagnostics/order/multiplicity, custom or axisymmetric bases, broad matrix semantics, surface and window behavior, optics, thermal calculations, ratings, daylighting, reporting, runtime numerics, EIO/SQLite, and conformance
<!-- routine-state-contract:v1 end setup_complex_fenestration_state_input -->

## Bounded Internal Heat Source Overlay Notes

EnergyPlus reads the `ConstructionProperty:InternalHeatSource` epJSON object
map directly after complex-fenestration states and before the following typed
equivalent-layer family and deferred WindowDataFile branch. The bounded Rust
overlay therefore uses case-sensitive lexical outer-key order without an IDF
declaration-order overlay. Its required outer key is retained only as a
normalized diagnostic source-name snapshot, not as a typed identity or lookup
namespace; case-colliding source names may consequently attach to different
targets.

The admitted target is an already materialized ordinary opaque construction
with at least two effective layers. Both one-based layer interfaces must be in
`1..TotLayers-1`; CTF dimensions must be exactly 1 or 2, tube spacing must be
finite in `[0.01,1.0]` m, and the optional finite perpendicular temperature
position defaults to 0 and must remain in `[0,1]`. Every field and target
constraint is validated before attachment, so an invalid earlier lexical
record does not reserve the target, the first valid attachment wins, and a
later valid record for the same construction fails closed without replacement.
The overlay changes no Construction identity, kind, layer stack, material
state, or graph edge.

Every valid definition remains an exact `UnsupportedSurfaceBoundary` /
`RunBlocked` declaration with `RuntimeClass::None`, including an unused target.
No manifest, comparator, proof variable, runtime, reporting, or conformance
claim is added. Broader F/C-factor, AirBoundary, window, complex-fenestration,
later equivalent-layer, and WindowDataFile targets; source warning recoveries;
thermochromic-child propagation; global source-presence flags; resistance-layer
remapping; CTF/QTF generation and histories; and every surface, radiant, or
source/sink consumer remain deferred.

## Bounded Equivalent-Layer Window Construction Notes

EnergyPlus reads `Construction:WindowEquivalentLayer` after the internal heat
source overlay through `getObjectItem`, so staged IDF declaration order is
semantic for the family. Rust adds that object to the existing order overlay;
native epJSON without the overlay continues to follow its case-sensitive
lexical instance-key order. The required name shares the normalized
Construction namespace. The required outside layer and optional layers 2
through 11 must form a contiguous prefix and resolve case-insensitively to the
typed EquivalentLayer material family.

Each definition becomes a dedicated `ConstructionKind::WindowEquivalentLayer`
with its ordered layer stack, construction/material graph edges, and zero-based
family ordinal corresponding to EnergyPlus's one-based `EQLConsPtr`. All
fallible validation precedes identity reservation. The source input routine
accepts family-valid packs without enforcing solid/gap alternation, and Rust
preserves that declaration behavior rather than projecting rules from later
window initialization.

Every equivalent-layer construction, including an unused definition, is an
exact `UnsupportedSurfaceBoundary` / `RunBlocked` declaration with
`RuntimeClass::None`. `SetPreConstructionInputParameters` maximum-layer state,
`CheckAndFixCFSLayer`, `FinalizeCFS`, `SetEquivalentLayerWindowProperties`,
ASHWAT optical/thermal calculations, ratings, surfaces, runtime execution,
reporting, and conformance remain deferred. The following WindowDataFile request
is typed separately, but its external-file expansion remains unported, so
`GetConstructData` stays source-mapped.

## Bounded WINDOW5 Data-File Request Notes

EnergyPlus reads `Construction:WindowDataFile` through `getObjectItem` after
every equivalent-layer construction. Rust therefore adds it to the staged-IDF
order overlay; native epJSON without the overlay retains lexical instance-key
order. The required object name is a normalized WINDOW5 entry search snapshot.
The optional retain-case file field becomes either an explicit
`DefaultWorkingDirectory` selector for `Window5DataFile.dat` or an exact
nonblank string. The IDD's 100-character text is a note, not a schema/source
validation rule, so the bounded compiler does not invent a length limit.

Requests live in a separate `ConstructionWindowDataFileRequest` arena with a
zero-based source ordinal. They deliberately reserve no `ConstructionId` or
shared name, and produce no generated material, construction/material edge,
frame/divider, or optical state. Consequently request case collisions and
collisions with existing construction names remain inert request data until the
file-expansion routine is ported; a surface cannot resolve a request as a
construction.

Every request, including a defaulted, unused, or nonexistent-file request, is
an exact `UnsupportedSurfaceBoundary` / `RunBlocked` declaration with
`RuntimeClass::None` before file access. `SearchWindow5DataFile` remains only
source-mapped: path search/fatal behavior, Unicode/header/EOF handling, entry
matching, one- or two-system expansion, W5 glass/gas material generation,
nominal resistance/U state, angular optics and fits, `:2` construction naming,
frame/divider and mullion synthesis, surfaces, reporting, runtime, and
conformance are deferred.

## Bounded Zone Declaration Notes

EnergyPlus calls `GetBuildingData` immediately after `GetConstructData` and
orders `GetShadowingInput`, `GetZoneData`, then `SetupZoneGeometry`.
`GetZoneData` sizes the Zone, ZoneDaylight, and resilience arrays before reading
every `Zone` through `getObjectItem` and delegating its public fields to
`ProcessZoneData`. After that loop it derives nominal equipment-control flags,
processes ZoneList and ZoneGroup, reads local-environment data, allocates
ZonePreDefRep, and calls `GetSpaceData`. Rust keeps both wrappers source-mapped,
retains the complete immutable `ProcessZoneData` declaration state, and now
continues the bounded `GetZoneData` state through nominal-control marking,
ZoneList, ZoneGroup, `GetZoneLocalEnvData`, and `GetSpaceData` while explicitly
omitting the intervening ZonePreDefRep reporting allocation.

The staged-IDF order overlay includes `Zone`, `ZoneList`, `ZoneGroup`, and
`ZoneProperty:LocalEnvironment`, so those dense arenas follow family-local IDF
declaration order. Native epJSON without that overlay uses lexical instance-key
order. `Space` and `SpaceList` deliberately do not use the overlay: EnergyPlus
directly iterates their JSON object maps, so both staged IDF and native epJSON
use lexical outer-key order. All 12 Zone public fields plus the required
object-key name are validated before the dense ID and normalized name are
published. A positive authored floor area is preferred by current area
consumers; an authored local convection selector is retained but blocks
arbitrary execution until the zone-local coefficient path consumes it.

After the Zone arena is complete, the bounded equipment-connection scan sets
each Zone's nominal-control flag solely from case-insensitive raw `zone_name`
presence. ZoneList then retains nonempty, ordered, resolved Zone membership and
the longest authored member name. ZoneGroup retains its resolved list and
positive multiplier, rejects repeated or overlapping grouped lists, and writes
the multiplier plus the source-shaped ZoneList identity to every member Zone.
Every ZoneList and ZoneGroup definition remains fail-closed because current
Zone-or-ZoneList consumers and comprehensive list-multiplier runtime paths are
not yet wired.

`GetZoneLocalEnvData` retains each resolved Zone and its optional generic node.
A nonblank node can name a one-member NodeList alias or a direct node that is
registered or reused. Ordered nonblank definitions overwrite the linked node
on their Zone, while a later blank node does not clear an earlier link. The
EnergyPlus 26.1 source checks outdoor-node membership only behind a
short-circuited zero-node condition, so this bounded declaration phase does not
require a direct node to have an `OutdoorAir:Node` definition. NodeList parsing
therefore moves immediately before this phase, matching the source routine's
lazy single-node lookup dependency without otherwise changing node order.
Every local-environment definition remains fail-closed until its mutable local
weather state and downstream consumers are wired.

`GetSpaceData` next retains authored Spaces in lexical order, including their
resolved Zone, three numeric-or-Autocalculate geometry selectors, first-seen
case-insensitive Space type identity, and ordered tags. It then retains lexical
SpaceLists whose member arrays resolve authored Spaces in authored array order;
missing and empty arrays are valid. Finally, it visits Zones in typed order and
adds one General whole-zone default to every Zone without an authored Space.
Those late defaults are unavailable to the preceding SpaceList lookup but do
remain in EnergyPlus's shared Space array for later surface lookup. The bounded
`GetHTSurfaceData` cross-section resolves optional typed
`BuildingSurface:Detailed.space_name` values through that full pre-remainder
arena and validates same-Zone membership while the full routine remains
`source_mapped`. State-mapped `CreateMissingSpaces` then consumes those
preclassified assignments and gives every retained detailed opaque surface a
final SpaceId. A Zone mixing explicit and implicit assignments receives one
General `AutoZoneRemainder`; an all-implicit Zone uses its existing last Space
and an all-explicit Zone creates no remainder. Authored Spaces, all SpaceLists,
and generated remainders fail closed, while a sole whole-zone default remains
inactive when no remainder is needed, including when every valid surface
explicitly references it.
Other surface families, opposite-surface generation, reordering, Space surface
lists, and all derived geometry remain with later checkpoints.

### `GetZoneData` bounded collection state contract

<!-- routine-state-contract:v1 begin get_zone_data -->
GetZoneData

read_state:
- after all `ProcessZoneData` calls complete, EnergyPlus derives nominal control for every Zone, processes every ZoneList and ZoneGroup, calls `GetZoneLocalEnvData`, allocates ZonePreDefRep, then calls `GetSpaceData`; bounded Rust preserves those declaration phases while explicitly omitting the intervening reporting allocation
- nominal control is a case-insensitive field-value existence scan independent of full equipment-connection field parsing; every Zone is explicitly retained as true for a matching raw zone name or false otherwise
- each ZoneList has one required nonblank outer-key name and a nonempty `zones` array whose ordered entries each require one case-insensitively resolved Zone name; staged IDF lists use recovered declaration order while native epJSON lists use lexical outer-key order
- each ZoneGroup has one required nonblank outer-key name, one required case-insensitively resolved ZoneList name, and an integer multiplier defaulting to one in the source-compatible positive signed-integer range; staged IDF groups use recovered declaration order while native epJSON groups use lexical outer-key order
- each ZoneProperty:LocalEnvironment has one required nonblank outer-key name, one required case-insensitively resolved nonblank Zone name, and an optional outdoor-air node name; bounded Rust initializes NodeList declarations immediately before this phase because EnergyPlus `GetOnlySingleNode` lazily initializes them
- GetSpaceData reads authored Space then SpaceList outer keys in lexical order for both staged IDF and native epJSON, preserves each tag/member array order, and finally visits Zones in typed Zone order to create whole-zone defaults

write_state:
- each typed Zone retains `is_nominal_controlled`, default false and set true solely by the bounded raw equipment-connection scan without claiming full connection validity
- a deterministic dense ZoneList arena and independent normalized name map retain each validated `ZoneListId`, normalized name, ordered ZoneId members, and maximum authored member-name length; empty, malformed, unresolved, or duplicate members fail before identity publication, while a Zone-name collision emits a nonblocking warning
- a deterministic dense ZoneGroup arena and independent normalized name map retain each validated `ZoneGroupId`, normalized name, ZoneListId, and multiplier; repeated list use or grouped-list Zone overlap fails before publication, while every valid member Zone receives the multiplier and the source-shaped ZoneList identity in `list_group`
- a deterministic dense ZoneLocalEnvironment arena and independent normalized name map retain each validated declaration, its ZoneId, and optional generic NodeId; each nonblank node resolves a one-member NodeList alias or registers/reuses a direct node, and ordered nonblank links overwrite the Zone link while a later blank node does not clear it
- deterministic Space, SpaceList, and first-seen SpaceType state retain validated authored declarations, ordered Zone/member links, then one whole-zone default for each Zone still without a Space
- every valid ZoneList or ZoneGroup is reported as `UnsupportedZoneGrouping`, every valid ZoneProperty:LocalEnvironment as `UnsupportedZoneLocalEnvironment`, and every authored Space or SpaceList as `UnsupportedSpacePartitioning`; the later CreateMissingSpaces contract applies that partition boundary to generated remainders while leaving sole whole-zone defaults inactive

history_state_ownership:
- TypedModel owns immutable nominal-control, ZoneList, ZoneGroup, ZoneLocalEnvironment, Space, SpaceList, SpaceType, and per-Zone list/local-node/space declaration state only; this checkpoint allocates no mutable local weather, geometry, sizing, reporting, equipment, surface, space-air, or zone-air history

unsupported_state:
- the pre-Zone-loop ZoneDaylight and resilience allocations and the post-`GetZoneLocalEnvData` ZonePreDefRep allocation
- ZoneList expansion for People, gains, thermostat, sizing, and every other Zone-or-ZoneList consumer, plus comprehensive `Zone.Multiplier * Zone.ListMultiplier` consumption across geometry, loads, HVAC flows, sizing, and reports
- OutdoorAir:Node condition inputs and node-connection metadata, Space-or-SpaceList target expansion, SetupZoneGeometry behavior beyond the separately bounded CP97 GetHTSurfaceData/CreateMissingSpaces surface-membership composite, local weather and EMS state, space heat balance, reporting, runtime numerical behavior, and conformance evidence

inactive_branches:
- when no raw equipment connection names a Zone, its nominal-control flag remains false without a diagnostic; unmatched connection names mark no Zone in this bounded phase
- when no ZoneList, ZoneGroup, or ZoneProperty:LocalEnvironment exists, every Zone retains list multiplier one, no list-group identity, and no linked outdoor-air node, adding no boundary from those absent families
- a ZoneGroup multiplier omitted from valid input retains one but still records group membership exactly like an authored multiplier and remains inside the fail-closed grouping boundary
- a blank or missing local-environment node remains an explicit no-node declaration and does not clear an earlier nonblank link for the same Zone
- when no authored Space exists, GetSpaceData still creates one General whole-zone default per Zone; that default adds no space-partition runtime boundary, including when every valid typed detailed surface explicitly references the sole default and no remainder is needed

unsupported_active_branches:
- every valid ZoneList, including an otherwise unused definition, is typed but blocks arbitrary runtime execution until all Zone-or-ZoneList consumers expand it
- every valid ZoneGroup, including multiplier one, is typed but blocks arbitrary runtime execution until list multiplier semantics are comprehensively consumed
- every valid ZoneProperty:LocalEnvironment, including an otherwise unused definition or one with a blank node, is typed but blocks arbitrary runtime execution until local weather consumers are wired
- every authored Space and every SpaceList, including an otherwise unused or empty list, is typed but blocks arbitrary runtime execution until space partitioning and all downstream consumers are wired; generated remainder Spaces are blocked by the separate CreateMissingSpaces contract
- typed BuildingSurface:Detailed lookup and same-Zone validation belong to the bounded GetHTSurfaceData cross-section, while final fallback assignment and mixed-assignment remainder generation belong to the state-mapped CreateMissingSpaces contract; the composite does not promote full GetHTSurfaceData or this parent routine
- the nominal-control scan does not validate or execute ZoneHVAC equipment; the existing later typed equipment-connection and IdealLoads boundaries remain authoritative

not_claimed_branches:
- complete GetZoneData parity, broad compiler pass-order parity, source partial-allocation and invalid-input recovery side effects, exact diagnostics/text/order/multiplicity, whitespace-preserving or case-colliding names, complete shared Zone/Space/ZoneList/SpaceList namespace behavior, full OutdoorAir:Node/NodeList and local-weather state, ZonePreDefRep, SetupZoneGeometry beyond the separate bounded surface-membership slice, sizing, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_zone_data -->

### `GetZoneLocalEnvData` state contract

<!-- routine-state-contract:v1 begin get_zone_local_env_data -->
GetZoneLocalEnvData

read_state:
- EnergyPlus calls `GetZoneLocalEnvData` after ZoneGroup processing and before ZonePreDefRep allocation and `GetSpaceData`; bounded Rust moves existing NodeList parsing immediately before this phase because `GetOnlySingleNode` lazily initializes NodeLists
- each ZoneProperty:LocalEnvironment has a required nonblank outer-key name and case-insensitively resolved nonblank `zone_name`; `outdoor_air_node_name` is optional, and blank or missing input retains no node; staged IDF instances use recovered declaration order while native epJSON instances use lexical outer-key order
- a nonblank node first resolves a typed NodeList alias, accepting exactly one member and rejecting multiple members; otherwise an existing generic NodeId is reused or a new generic NodeId is registered
- EnergyPlus 26.1 checks OutdoorAir:Node membership only inside a short-circuited `NodeNum == 0` condition, so bounded declaration state likewise does not require a nonblank direct node to have an OutdoorAir:Node definition
- ordered declarations for the same Zone are allowed; each nonblank resolved node overwrites that Zone's link while a later blank node does not clear the previous link, so the last nonblank node wins

write_state:
- a deterministic dense ZoneLocalEnvironment arena and independent normalized name map retain each fully validated ZoneLocalEnvironmentId, normalized name, ZoneId, and optional generic NodeId; validation completes before identity, node, or Zone-link publication, so an invalid earlier record consumes none of those bounded identities or side effects
- each Zone retains an optional linked outdoor-air NodeId; a one-member NodeList stores its member node, a direct name registers or reuses that generic node, and blank input stores no node
- every valid ZoneProperty:LocalEnvironment definition is reported as `UnsupportedZoneLocalEnvironment` and `RunBlocked` before arbitrary runtime execution because mutable local-weather conditions and their consumers are not wired

history_state_ownership:
- TypedModel owns immutable local-environment declaration and per-Zone optional generic-node linkage only; this checkpoint allocates no mutable dry-bulb, wet-bulb, wind, psychrometric, EMS, surface, load, or zone-air history

unsupported_state:
- complete OutdoorAir:Node condition input including height, dry-bulb/wet-bulb/wind schedules, wind-pressure-coefficient curve and symmetry/angle controls, plus NodeConnection metadata
- AnyLocalEnvironmentsInModel and mutable local dry-bulb, wet-bulb, humidity, pressure, wind, psychrometric, EMS, weather, surface, infiltration, load, and zone-air consumer state
- source-sized partial records and invalid-input side effects and the post-routine ZonePreDefRep allocation; GetSpaceData declaration/default state belongs to a separate bounded contract

inactive_branches:
- when no ZoneProperty:LocalEnvironment exists, the arena stays empty, every Zone node link stays absent, and no local-environment runtime boundary is added
- a blank or missing node is a valid no-node declaration and does not clear an earlier nonblank link for the same Zone
- multiple declarations may reference one Zone; only ordered declarations with a nonblank resolved node overwrite its link

unsupported_active_branches:
- every valid definition, including an otherwise unused record or a record with no node, blocks arbitrary runtime execution until local-weather consumers are wired
- a nonblank direct generic NodeId does not claim an OutdoorAir:Node declaration or any local environmental conditions

not_claimed_branches:
- complete GetZoneLocalEnvData parity, source preallocation and partial invalid-record/node/link side effects, the source multi-member NodeList first-node side effect, exact diagnostics/text/order/multiplicity, whitespace-preserving or case-colliding names, OutdoorAir:Node condition state, local-weather consumers, ZonePreDefRep, geometry, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_zone_local_env_data -->

### `GetSpaceData` state contract

<!-- routine-state-contract:v1 begin get_space_data -->
GetSpaceData

read_state:
- EnergyPlus calls `GetSpaceData` after `GetZoneLocalEnvData` and ZonePreDefRep allocation and before `SetupZoneGeometry`; bounded Rust calls its declaration/default phase immediately after local-environment parsing while leaving that intervening reporting allocation deferred
- authored Space outer keys are always lexical for both staged IDF and native epJSON because the source directly iterates the JSON object map; each Space requires a nonblank name and resolved nonblank Zone, while ceiling height, volume, and floor area preserve every finite number or default missing, blank, and Autocalculate input to AutoCalculate
- space type defaults to General for missing or blank input and joins a case-insensitive first-seen lexical-order registry; tag objects preserve array order, normalized text, duplicates, and empty or missing tag strings
- SpaceList outer keys are likewise always lexical while member arrays preserve authored order; missing or empty member arrays are valid, each present member resolves an authored Space, duplicate resolved members fail, and list names colliding with a Zone or authored Space fail
- after all lists are processed, Zones are visited in typed order and each Zone without an authored Space receives one appended whole-zone default named after the Zone with AutoCalculate geometry, no tags, and the General space type; its late position keeps it unavailable to the preceding SpaceList lookup while the later bounded GetHTSurfaceData cross-section can resolve it through the full pre-remainder Space arena

write_state:
- a deterministic dense Space arena retains each fully validated authored declaration followed by generated whole-zone defaults; authored names alone enter the reference map, validation completes before SpaceId, SpaceTypeId, or Zone-link publication, and each Space retains its ZoneId, three AutoOrNumber selectors, normalized type name/id, ordered tags, and origin
- each Zone retains ordered SpaceIds and exits the routine with at least one Space; generated defaults reuse or append General through the separately bounded `GetGeneralSpaceTypeNum` helper and remain outside the authored-name map used by the preceding SpaceList phase
- a deterministic dense SpaceList arena and normalized name map retain each validated SpaceListId, lexical name, ordered authored SpaceIds, and maximum authored member-name length, including valid zero-member lists
- every authored Space and SpaceList is reported as `UnsupportedSpacePartitioning` and `RunBlocked` before arbitrary runtime execution; the separate CreateMissingSpaces contract applies the same boundary to generated remainder Spaces, while sole generated whole-zone defaults remain inactive even when valid detailed surfaces explicitly reference them

history_state_ownership:
- TypedModel owns immutable authored/default Space, SpaceList, SpaceType, and per-Zone SpaceId declaration topology only; this checkpoint allocates no mutable geometry, surface, space-air, zone-air, gain, HVAC, sizing, or reporting history

unsupported_state:
- the preceding ZonePreDefRep allocation and all predefined-report mutation
- SetupZoneGeometry behavior beyond the separately bounded CP97 composite of GetHTSurfaceData typed detailed-surface lookup/same-Zone validation and CreateMissingSpaces remainder/fallback finalization: other surface families, opposite-surface generation, reordering, Space surface lists, calculated height/volume/floor area, floor/volume fractions, enclosure and surface ranges, and geometry correction/warnings
- Space-or-SpaceList target expansion for internal gains, infiltration, mixing, ventilation, sizing, outdoor air, HVAC, outputs, and every space heat-balance/reporting/runtime consumer

inactive_branches:
- with no authored Space, every Zone receives one General whole-zone default and no UnsupportedSpacePartitioning boundary is added when detailed surfaces are all implicit or all validly explicit to that sole default
- a Zone with one or more authored Spaces receives no whole-zone default; if every Zone is covered and all authored types are non-General, the registry contains no General entry
- a SpaceList with a missing or empty member array is retained as a valid zero-member list but remains inside the all-definition runtime boundary

unsupported_active_branches:
- every authored Space, including one using only defaults and not referenced by a surface or load, blocks arbitrary runtime execution until space partition consumers are wired
- every SpaceList, including an empty or otherwise unused definition, blocks arbitrary runtime execution until all Space-or-SpaceList consumers expand it
- typed BuildingSurface:Detailed lookup and same-Zone validation belong to the bounded GetHTSurfaceData cross-section, while final fallback assignment and mixed-assignment remainder generation belong to the state-mapped CreateMissingSpaces contract; this declaration routine does not claim either part

not_claimed_branches:
- complete GetSpaceData parity, source allocation sizes and partial invalid Space/SpaceType/Zone/list side effects, exact diagnostics/text/order/multiplicity, source whitespace preservation or case-colliding same-family names, complete shared Zone/ZoneList/Space/SpaceList namespace behavior, SetupZoneGeometry beyond the separately bounded typed detailed-surface membership slice, loads, space heat balance, HVAC, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_space_data -->

### `GetGeneralSpaceTypeNum` state contract

<!-- routine-state-contract:v1 begin get_general_space_type_num -->
GetGeneralSpaceTypeNum

read_state:
- GetSpaceData calls `GetGeneralSpaceTypeNum` while generating a whole-zone default and CreateMissingSpaces calls it while generating a mixed-assignment remainder; the helper searches the ordered type registry built from validated authored Spaces and any earlier generated default using case-insensitive General matching

write_state:
- the helper reuses the first existing General SpaceTypeId or appends one General entry at the end and returns that dense identity; every generated whole-zone default and remainder in the same model reuses it

history_state_ownership:
- TypedModel owns the immutable ordered normalized SpaceType name registry and dense IDs only; the helper allocates no mutable simulation history

unsupported_state:
- EnergyPlus allocation capacity and global integer numSpaceTypes storage beyond the equivalent dense typed registry
- all downstream type-based grouping, gains, reporting, geometry, HVAC, and space heat-balance consumers

inactive_branches:
- when every Zone already has an authored Space and no mixed explicit/blank surface assignment requires a remainder, the helper is not called, so a non-General-only authored registry remains unchanged
- when General already exists from an authored Space or earlier generated default, later default or remainder creation reuses that identity without appending another type

unsupported_active_branches:
- generated whole-zone default creation and General identity alone add no runtime boundary; authored Space and SpaceList definitions are blocked by the parent GetSpaceData contract, while generated remainders are blocked by the separate CreateMissingSpaces contract

not_claimed_branches:
- complete helper parity, source one-based numeric identity, allocation/counter side effects, whitespace-preserving labels, exact diagnostics, downstream type consumers, numerical parity, and conformance
<!-- routine-state-contract:v1 end get_general_space_type_num -->

### `CheckScheduledSurfaceGains` state contract

<!-- routine-state-contract:v1 begin check_scheduled_surface_gains -->
CheckScheduledSurfaceGains

read_state:
- the internal tail helper called by `GetScheduledSurfaceGains` for every EnergyPlus Zone only when either scheduled-gain input family has at least one source record; the enclosing routine remains source-mapped, and its next top-level sibling is the representative-surface EIO assignment block
- only the error-free retained typed `BuildingSurface:Detailed` subset after final Space assignment: deterministic `Zone::spaces`, each retained surface's final SpaceId and current resolved ConstructionId, and the immutable `SurfaceSolarIncident` exact SurfaceId/ConstructionId pairs
- for each retained typed opaque surface, a pair is scheduled only when its current construction exactly matches one typed first-family record; complex-fenestration requests remain unresolved and cannot match this subset

write_state:
- no model state is written; a Zone receives one nonblocking compile warning only when its retained typed opaque subset already contains both at least one exact-pair match and at least one miss
- the bounded diagnostic identifies the Zone and lists only the known retained typed surface names whose current SurfaceId/ConstructionId pair is unscheduled; it does not invent names or outcomes for deferred surfaces
- the warning predicate is monotonic under completion of the full EnergyPlus heat-transfer surface set: once a retained subset contains both a match and a miss, adding omitted surfaces cannot make the full set uniformly scheduled or uniformly unscheduled
- any relevant typed surface-input or scheduled-surface-gain input error suppresses the entire check so diagnostics are not inferred from partial compiler arenas

history_state_ownership:
- the compiler owns no new persistent state for this diagnostic-only pass; it allocates only local match/miss observations and warning text, with no schedule values, surface flags, lookup cache, or runtime history

unsupported_state:
- the complete EnergyPlus heat-transfer surface population: windows and other fenestration, legacy detailed surface families, InternalMass, generated opposite surfaces, source surface reordering, and representative/constituent-surface mutation
- EnergyPlus `WindowScheduledSolarAbs` matching, complex-fenestration current or active construction selection, `SurfaceScheduledSolarInc` and `WindowScheduledSolarAbs` runtime consumers, schedule sampling, solar replacement, BSDF absorption, and reporting

inactive_branches:
- an empty retained subset, an all-matched retained subset, or an all-unmatched retained subset emits no warning because omitted full-domain surfaces could change those classifications; this also avoids claiming the source empty-Zone flag quirk
- a Zone with no typed detailed opaque surfaces emits no warning, and unresolved CP101 fenestration requests cannot create a typed-subset match
- surface or scheduled-input errors suppress the pass rather than emitting warnings from partially published state

unsupported_active_branches:
- the bounded warning is diagnostic-only and nonblocking; it does not weaken the all-definition `UnsupportedSurfaceBoundary` and `RunBlocked` boundary for either scheduled-gain object family
- Zones whose full source surface set is mixed but whose retained typed subset is empty, all matched, or all unmatched intentionally receive no warning until full completeness is available

not_claimed_branches:
- complete Zone/Space heat-transfer surface membership, windows, legacy/InternalMass/generated/representative surfaces, active-construction semantics, full scheduled-gain completeness, or the EnergyPlus empty-Zone behavior
- exact EnergyPlus warning severity text, continuation text, surface text, order, punctuation, or multiplicity; pair-lookup routines, schedule evaluation, runtime numerics, reporting, graph edges, manifests, comparators, proof variables, and conformance
<!-- routine-state-contract:v1 end check_scheduled_surface_gains -->

### `CreateTCConstructions` state contract

<!-- routine-state-contract:v1 begin create_tc_constructions -->
CreateTCConstructions

read_state:
- EnergyPlus calls this routine immediately after the inline representative-surface EIO assignment block; bounded Rust enters after the CP102 scheduled-gain check and reads only immutable typed-model state, while the intervening output block remains mapped and deferred
- every existing typed Construction is visited once in dense ConstructionId order; only records with CP85 `ConstructionThermochromicMaster` metadata are selected, and that descriptor's parent MaterialId plus zero-based layer index address the final thermochromic parent and its slot in the already first-state-substituted effective stack
- each selected parent resolves to a typed `WindowGlazingThermochromicGroupMaterial` and its nonempty contiguous state-arena slice; every ordered state, including the first, supplies its retained optical-data temperature and resolved glazing MaterialId without sorting, deduplication, or duplicate rejection

write_state:
- one immutable `ConstructionThermochromicSeries` per selected master retains the master ConstructionId, the first state's initial specification temperature, and a contiguous first-child/count descriptor
- one immutable `ConstructionThermochromicChild` per ordered state retains a dense projection-only child ID, master ConstructionId, zero-based state index, source-shaped `{:.0R}` normalized name with the pinned 40, -999.5, signed-zero, 0.05, and finite-limit boundaries, specification temperature, outside layer, and cloned effective layer stack with only the final master's retained layer slot replaced by that state's glazing MaterialId
- output order is master ConstructionId then input state; earlier thermochromic parents in a multiple-parent construction remain fixed at their first states, layer-zero replacement also updates the child outside layer, duplicate temperatures/materials/names are preserved, and a shared group produces a separate series for each master
- both projection arenas are built transactionally and published together only after every selected parent descriptor, state range, effective first-state layer, finite temperature, layer index, and dense child range validates; prior compiler errors suppress the pass rather than publishing projections from partial upstream arenas
- the main Construction arena and name map, ConstructionIds, object counts, material arena, model-graph edges, surfaces, and runtime state are unchanged

history_state_ownership:
- TypedModel owns only immutable thermochromic series and child projection arenas; no master mutation, active-state pointer, temperature history, surface state, optical/thermal cache, or mutable simulation history is allocated

unsupported_state:
- appending deep-copied children to the global Construction arena, assigning source-global ConstructionIds after any WINDOW5-generated constructions, updating `TotConstructs`, and extending construction names, object counts, or construction/material graph edges
- full `ConstructionProps` deep copy, master `TCChildConstrs` allocation, child-to-master global construction links, and nominal R/U, pre-adjusted U, or coefficient-adjustment arrays
- source linear lookup and first-match behavior for generated-name collisions, including collisions between rounded temperatures, different masters, existing constructions, or deferred WINDOW5 children
- active surface-construction mutation, nearest-temperature state selection and history, fenestration binding, window optics and thermal calculations, daylighting, shading, ratings, EIO/SQLite or other reporting, and runtime behavior

inactive_branches:
- with no retained thermochromic master construction, both projection arenas remain empty; an otherwise valid but unused thermochromic group produces no series or child
- reusing one thermochromic group from multiple masters produces an independent series and child range for each master without sharing projection identities
- the first state always produces a child even though its layers equal the master's already first-state-substituted effective stack; duplicate temperatures, glazing references, and formatted names remain distinct children in input order

unsupported_active_branches:
- every typed thermochromic parent remains `UnsupportedSurfaceBoundary` and `RunBlocked`, including an unused parent and one used by one or more projected master constructions; projection materialization never admits partial runtime
- series and child identities are private derived state, not public ConstructionIds or object definitions, and do not change object, unsupported-object, capability, manifest, comparator, proof-variable, or conformance counts

not_claimed_branches:
- complete source ordering relative to WINDOW5-synthesized constructions, source-global child identity and name lookup, exhaustive fmt 8.0.1 versus Rust generated-name equivalence for arbitrary finite `f64` values beyond the pinned boundaries, invalid-upstream partial allocation or assertion behavior, exact `ErrorsFound` side effects, and full deep-copy field parity
- representative-surface assignment output, construction/surface mutation, thermochromic state selection, optics, thermal calculations, daylighting, shading, ratings, EIO/SQLite or other reporting, runtime numerics, exact diagnostics/order/multiplicity, and conformance
<!-- routine-state-contract:v1 end create_tc_constructions -->

### `GetInternalHeatGainsInput` state contract

<!-- routine-state-contract:v1 begin get_internal_heat_gains_input -->
GetInternalHeatGainsInput

read_state:
- the line-320 `ManageInternalHeatGains(state, true)` parent call after `InitSolarViewFactors`; bounded Rust invokes `Compiler::parse_bounded_internal_heat_gains_input` after the CP105 construction-use collector, returns before either family when an Error already exists on entry, otherwise scans People before OtherEquipment, and continues into OtherEquipment even when People adds an Error
- every `People` definition whose required `zone_or_zonelist_or_space_or_spacelist_name` resolves directly to an existing typed Zone; an optional number-of-people schedule resolves through the existing typed schedule namespace, the calculation method defaults to People and accepts People, People/Area or PeoplePerArea, and Area/Person or AreaPerPerson, and the three finite sizing scalars default to zero and must be nonnegative
- every `OtherEquipment` definition whose same target field resolves directly to an existing typed Zone; an optional schedule resolves through the existing typed schedule namespace, fuel type defaults to normalized None, design-level method defaults to EquipmentLevel and accepts the bounded Watts/Area, Power/Area, WattsPerZoneFloorArea, Watts/Person, Power/Person, or WattsPerPerson aliases, and all existing finite numeric/default/range validation remains in force
- OtherEquipment retains finite design level and carbon-dioxide generation rate with zero defaults, nonnegative power-per-floor-area and power-per-person with zero defaults, and latent, radiant, and lost fractions each in [0,1] with zero defaults; a sum above one emits the existing blocking typed diagnostic without suppressing later definitions

write_state:
- the existing deterministic People arena and normalized people name map, with dense InternalGainId, normalized name, direct ZoneId, optional ScheduleId, calculation-method enum, number of people, people per floor area, and floor area per person
- the existing deterministic OtherEquipment arena and normalized other-equipment name map, with dense InternalGainId, normalized name and fuel type, direct ZoneId, optional ScheduleId, design-level-method enum, design level, power-per-floor-area, power-per-person, latent/radiant/lost fractions, and carbon-dioxide generation rate
- family-level diagnostic projection ordered People before OtherEquipment for an error-free entry; validation remains definition-local, so a malformed People object does not prevent the bounded OtherEquipment family from being scanned in the same wrapper invocation

history_state_ownership:
- TypedModel owns immutable bounded People and OtherEquipment declaration state already used by downstream Rust paths; CP107 adds no mutable EnergyPlus internal-gain history, one-time input flag, runtime rate, schedule sample, meter, output, or EMS state

unsupported_state:
- Lights, ElectricEquipment, GasEquipment, HotWaterEquipment, SteamEquipment, ElectricEquipment:ITE:AirCooled, ZoneBaseboard:OutdoorTemperatureControlled, and ZoneContaminantSourceAndSink:CarbonDioxide source-family input and all of their dependent state
- ZoneList, Space, and SpaceList target expansion; source-generated gain instances and declaration order; Space and Zone occupant totals; `setupIHGZonesAndSpaces`, `setDesignLevel`, floor-area/person-derived design-level materialization, schedule-minimum checks, nominal min/max gain arrays, and Space heat-balance topology
- ManageInternalHeatGains runtime branches and recurring summation/update behavior, daylighting order beyond the mapped parent call, reporting variables, EIO, meters, contaminant coupling, EMS actuators/internal variables, component-load reporting, and all numerical runtime state

inactive_branches:
- any compile Error present on wrapper entry suppresses both bounded family scans and publishes no additional People or OtherEquipment state from this pass
- when neither bounded family is present, the wrapper is an empty no-op and adds no identity, count, capability, runtime state, or diagnostic
- a missing optional schedule reference retains no ScheduleId; absent optional/defaulted numeric, fuel, and method fields retain their existing typed defaults without creating schedule or derived design-level state

unsupported_active_branches:
- a People Error created after entry does not suppress the OtherEquipment scan, but no claim follows for exact source recovery state, complete diagnostic ordering, or arbitrary compiler passes after this bounded wrapper
- a valid direct-Zone People or OtherEquipment definition retains its pre-existing typed support and downstream bounded uses only; CP107 does not promote full internal-gain runtime execution or any omitted family

not_claimed_branches:
- complete `ManageInternalHeatGains` or `GetInternalHeatGainsInput` parity, source `GetInputFlag` persistence and re-entry behavior, `InitOnly` control beyond this compile-time projection, exact object/declaration ordering within either family, partial-allocation recovery, exact diagnostics/text/severity/order/multiplicity, or exact global compiler short-circuit behavior after Errors created inside the pass
- ZoneList/Space/SpaceList expansion, generated instances, Space/Zone occupant totals, every omitted internal-gain family, setup and design-level derivation, schedule minima, reporting, meters, EMS, runtime gains, numerical parity, capability expansion, object-count or graph expansion, and conformance
<!-- routine-state-contract:v1 end get_internal_heat_gains_input -->

### `set_computed_geometry` state contract

<!-- routine-state-contract:v1 begin set_computed_geometry -->
set_computed_geometry

read_state:
- the complete source loop at `ManageHeatBalance` lines 182-184, after sizing-mode and octree work but before the line-186 one-time flag clear; bounded Rust invokes `Compiler::set_bounded_surface_computed_geometry` after the CP107 executable slice while CP108-CP110 remain mapped or deferred, and returns without mutation when any compile Error already exists
- every retained typed `BuildingSurface:Detailed` Surface in dense typed order, reading only its already canonicalized world-coordinate `Point3` vertices; source `SurfaceGeometry.cc::GetVertices` world-coordinate assignment plus `ProcessSurfaceVertices` shape assignment and `isRectangle` are dependencies rather than promoted routines
- a positive-only candidate gate accepting exactly three or four vertices, all finite; a finite nonzero source-ordered Newell normal and finite plane; finite nonzero projected twice-signed area; and conservative coplanarity whose point-to-plane distance is at most `1.0e-9 * max(1, max(abs(world coordinate)))`
- for four-vertex candidates only, positive finite length for every wrap edge, source diagonal lengths 1-3 and 2-4 differing by strictly less than 0.020 m, and the absolute dot product of normalized source edges 3-2 and 2-1 no greater than `cos(89 degrees)`; three-vertex candidates map directly to `SurfaceShapeCategory::Triangular`, while admitted four-vertex candidates map to `Rectangular`
- source-ordered Newell accumulation without normal normalization; projection-axis selection from the largest absolute plane-normal component with X-over-all and then Y-over-Z tie precedence; X-to-(y,z), Y-to-(x,z), or Z-to-(x,y) projection; projected bounds; strict negative-area reversal of vertices 2 through N only; then wrap-edge construction and Rectangle-only projected edge-1/edge-4 squared lengths

write_state:
- each admitted Surface receives `Some(SurfaceComputedGeometry)` containing its `SurfaceShapeCategory`, source-shaped four-element plane, `SurfaceProjectionAxis`, ordered `projected_vertices`, `projected_lower_bound`, `projected_upper_bound`, ordered `projected_edges`, and `rectangle_side_1_squared_m2` plus `rectangle_side_3_squared_m2`
- a negative projected signed area reverses only the copied projected tail before edges are written and never mutates the source 3D vertices or plane orientation; a Triangle stores exact zero for both rectangle side-square fields
- every excluded retained Surface receives `computed_geometry = None` without a diagnostic, identity, name-map entry, object-count increment, graph edge, support/capability change, manifest, comparator, proof variable, or runtime state; exclusion of one Surface does not suppress eligible later entries

history_state_ownership:
- `TypedModel::surfaces` owns immutable compile-time derived `Surface::computed_geometry` attachments only; CP111 allocates no mutable source input flag, octree pointer, intersection cache, Surface history, timestep state, reporting state, or runtime geometry consumer

unsupported_state:
- the complete EnergyPlus Surface array and its source order, all generated/opposite surfaces, FenestrationSurface:Detailed, shading, InternalMass, doors/windows, overhangs/fins, and every other omitted Surface family or source shape
- source `Shape`, `IsConvex`, Convex and Nonconvex categories, nonrectangular quadrilaterals, polygons, `Surface2DSlab`, unique slab-Y state, inverse edge slopes, and the convex `n >= 20` slab switch
- complete `GetVertices` canonicalization and recovery, `CheckConvexity`, coincident/collinear deletion, planarity checking and diagnostics, `ProcessSurfaceVertices`, full `isRectangle` lifecycle, source partial writes/default objects, one-time flag ownership and clearing, octree/PierceSurface/intersection consumers, runtime, reporting, performance, and numerical conformance

inactive_branches:
- any compile Error present on pass entry preserves the pre-pass computed-geometry state for every retained Surface and emits no CP111 diagnostic
- an empty Surface arena is a no-op; a retained Surface with a vertex count other than three or four, any nonfinite or overflow-derived value, a zero Newell normal or signed projected area, conservative noncoplanarity, or a rejected four-vertex rectangle remains `None` and does not affect another Surface
- strict source boundaries are retained for rectangle diagonal difference below 0.020 m, the inclusive cosine threshold, X-then-Y projection ties, and signed-area reversal only below zero; no tolerance is added to those branch comparisons beyond the separately declared conservative coplanarity gate

unsupported_active_branches:
- valid computed geometry remains metadata only and grants no new object support, runtime admission, daylighting, shading, solar, octree, ray-intersection, or heat-balance behavior
- a four-vertex input that source local `isRectangle` could accept only through zero-vector normalization is deliberately rejected, as are finite source inputs whose derived state overflows or whose missing upstream validation cannot be conservatively established
- typed Surface order is preserved for the bounded pass, but this does not claim complete global Surface count/order, generated-family placement, source reordering, or exact loop side effects outside the retained arena

not_claimed_branches:
- complete `SurfaceData::set_computed_geometry`, `computed_shapeCat`, `computed_plane`, `computed_surface2d`, `Surface2D` construction, `GetVertices`, `ProcessSurfaceVertices`, `CheckConvexity`, or `isRectangle` parity; source invalid-input recovery, partial mutation/default retention, exact diagnostic text/severity/order/multiplicity, or one-time flag clearing and re-entry
- Convex/Nonconvex and slab state, complete Surface families/order, octree/PierceSurface and every computed-geometry consumer, exact cross-language floating-point behavior, runtime numerical behavior or performance, output/report serialization, capability/support expansion, conformance cases, or numerical conformance
<!-- routine-state-contract:v1 end set_computed_geometry -->

## Data Structure Map

| EnergyPlus data | Rust target | Boundary |
|---|---|---|
| `DataHeatBalance::ZoneData` declaration and post-list/local-environment/space fields | `ep_model::Zone`, `ep_model::ZoneConvectionAlgorithm` | dense ID/name, north/origin, standard type, direct multiplier, raw auto geometry selectors, effective inherited-or-local convection algorithms, total-floor-area membership, nominal-control presence, list multiplier, source-shaped list identity, optional linked generic outdoor-air node, and ordered authored/default/remainder SpaceIds are retained; positive authored floor area is consumed, while local convection overrides, Zone grouping, local-environment declarations, authored partitions, and generated remainders run-block before deferred geometry/report/runtime state |
| `DataHeatBalance::ZoneListData` and `ZoneGroupData` | `ep_model::ZoneList`, `ep_model::ZoneGroup`, and per-Zone `list_multiplier`/`list_group` | staged-IDF or native-epJSON ordered dense collections retain resolved membership, longest member name, list reference, positive multiplier, repeated-list and grouped-overlap validation, and member-Zone propagation; every definition run-blocks until list-target expansion and comprehensive list-multiplier consumption are wired |
| `DataHeatBalance::ZoneLocalEnvironmentData` and `ZoneData::LinkedOutAirNode` | `ep_model::ZoneLocalEnvironment` and `Zone::linked_outdoor_air_node` | staged-IDF or native-epJSON ordered dense declarations retain resolved ZoneId and optional one-member-NodeList or direct generic NodeId; last nonblank link wins and a later blank does not clear it; every definition run-blocks until OutdoorAir:Node condition state and local-weather consumers are wired |
| `DataHeatBalance::SpaceData`, `SpaceListData`, `spaceTypes`, and `ZoneData::spaceIndexes` | `ep_model::Space`, `ep_model::SpaceList`, `ep_model::SpaceOrigin`, `SpaceTypeId`, typed name maps, and `Zone::spaces` | lexical dense authored Spaces retain resolved Zone, numeric-or-Autocalculate geometry selectors, first-seen normalized type identity, ordered tags, and authored origin; lexical SpaceLists retain ordered authored-Space membership including valid empty lists; Zone-order General defaults ensure every Zone has a Space but remain outside the preceding SpaceList name map. A later bounded pass appends General `AutoZoneRemainder` entries for mixed detailed-surface assignments. Authored Spaces, every SpaceList, and remainders run-block; sole whole-zone defaults do not, while Space surface lists/geometry/runtime consumers remain deferred |
| `DataSurface::SurfaceData::{Zone, spaceNum}` | `ep_model::Surface::{zone, space}` and `ep_model::SpaceOrigin` | the typed detailed opaque subset resolves optional Space names through the full pre-remainder arena with same-Zone validation, retains explicit targets, assigns all-implicit Zones to their existing last Space, and redirects mixed implicit surfaces to a Zone-order remainder; other surface families, opposite-surface generation, reordering, per-Space surface lists, geometry, and runtime consumers remain deferred |
| `DataSurface::AllVaryAbsOpaqSurfaceList` selected by `GetVariableAbsorptanceSurfaceList` | `TypedModel::variable_absorptance_surface_bindings` and `ep_model::VariableAbsorptanceSurfaceBinding` | after the retained detailed-surface pass, dense-order Outdoors surfaces whose construction outside layer owns a typed overlay retain a SurfaceId-to-MaterialVariableAbsorptanceId binding; non-Outdoors outside-layer uses and each typed inside-layer occurrence produce bounded warnings only. Full `AllHTSurfaceList` membership/reorder parity, other surface families, runtime updates, exact warning text/order/multiplicity, numerics, and conformance remain deferred, and every overlay still run-blocks |
| `SurfaceProperty:IncidentSolarMultiplier` request fields before `Surface::hasIncSolMultiplier` and `SurfIncSolMultiplier` mutation | `TypedModel::surface_incident_solar_multiplier_requests`, `ep_model::SurfaceIncidentSolarMultiplierRequestId`, and `ep_model::SurfaceIncidentSolarMultiplierRequest` | a dense request arena retains the nonsemantic normalized declaration key, unresolved normalized window target, inclusive-[0,1] default-1 multiplier, and optional resolved ScheduleId without creating or mutating a SurfaceId. Duplicate targets and missing schedules fail closed; source order, fenestration lookup and eligibility, construction/shade checks, per-surface overwrite state, schedule evaluation, runtime, reporting, and conformance remain deferred, and every request run-blocks |
| `SurfaceProperty:SolarIncidentInside` first-phase `SurfaceSolarIncident::{Name, SurfPtr, ConstrPtr, sched}` state | `TypedModel::surface_solar_incidents`, `ep_model::SurfaceSolarIncidentId`, and `ep_model::SurfaceSolarIncident` | each dense record retains a normalized semantic name without a name map plus one typed detailed-opaque SurfaceId, any typed ConstructionId, and required ScheduleId. Duplicate names and construction-mismatched surface pairs remain valid, repeated resolved pairs fail closed, and no source order is claimed. separately types the following complex-fenestration request family, while representative-surface mutation, full completeness, pair lookup, schedule sampling, solar replacement, runtime, reporting, and conformance remain deferred. only emits a nonblocking monotonic warning when the retained typed opaque subset is already mixed by exact current-construction pair matching; every definition run-blocks and no graph edge is added |
| `ComplexFenestrationProperty:SolarAbsorbedLayers` second-phase `FenestrationSolarAbsorbed::{Name, SurfPtr, ConstrPtr, NumOfSched, scheds}` request state | `TypedModel::fenestration_solar_absorbed_requests`, `ep_model::FenestrationSolarAbsorbedRequestId`, and `ep_model::FenestrationSolarAbsorbedRequest` | each dense request retains a normalized semantic name without a name map, unresolved normalized fenestration target, typed complex-fenestration ConstructionId, and outside-to-inside ScheduleIds whose count exactly matches `complex_fenestration.optical_layers`. Duplicate names remain valid and duplicate target/construction pairs fail closed because no source order is claimed. Source `NumAlpha`/trailing-blank positional parity, source indexing defects, fenestration binding, full completeness, pair lookup, schedule sampling and value/type limits, BSDF layer absorption, runtime, reporting, and conformance remain deferred; every request run-blocks and no graph edge is added |
| `CheckScheduledSurfaceGains` per-Zone `ZoneScheduled`/`ZoneUnscheduled` diagnostic state | `Compiler::check_scheduled_surface_gains_typed_subset` and compile warnings only | after error-free final Space assignment, a Zone warns nonblockingly only when retained typed detailed opaque surfaces already include both an exact current SurfaceId/ConstructionId pair match and a miss; only known misses are named. Empty, all-matched, and all-unmatched subsets are silent. No model state is added; full surface completeness, windows, other surface families, active constructions, pair lookups, exact diagnostics, and runtime remain deferred |
| inline representative-surface EIO header and assignment rows | mapped/deferred barrier only | the exact condition, unconditional-when-enabled header, complete global Surface order, non-self assignment filter, and representative-name payload are documented without adding a synthetic routine or Rust output state; the project flag, full Surface population/order, representative/constituent mutation, and EIO writer remain absent |
| `CreateTCConstructions` master `specTemp`/`TCChildConstrs` and child layer/name/temperature projection | `TypedModel::construction_thermochromic_series`, `TypedModel::construction_thermochromic_children`, `ep_model::ConstructionThermochromicSeries`, `ep_model::ConstructionThermochromicChild`, and `ep_model::ThermochromicConstructionChildId` | dense master-ConstructionId then ordered-state projection retains the initial first-state temperature and one child per state, including the first; each child clones the effective stack, replaces only the final retained TC layer, derives the outside layer, and uses source-shaped `{:.0R}` naming with pinned boundary examples. Projection IDs are not ConstructionIds; arbitrary-finite exact formatter equivalence, global append/count/name/graph state, WINDOW5-relative ordering, deep-copy fields, collision lookup, switching, reporting, and runtime remain deferred |
| inline no-Zone gate and `CheckValidSimulationObjects` return consumed as caller diagnostics | `Compiler::check_valid_simulation_objects_bounded` and compile diagnostics only | after the projection and absent prior Errors, an empty typed Zone arena plus raw detailed Site/Building shading presence is a positive Surface witness; absence of all eight raw allowed collector/generator families emits one blocking `InvalidSimulationWithoutZones` Error. No model state is added. Full `TotSurfaces` parity, other surface families, exact severe/fatal behavior, allowed-family semantics, runtime, and conformance remain deferred; raw allowed presence is not a typing or support claim |
| `CheckUsedConstructions` retained-surface plus six-family reference scan before `ConstructionProps::{IsUsed, IsUsedCTF}` and unused warnings | `TypedModel::known_used_constructions`, `TypedModel::known_ctf_used_constructions`, and `Compiler::collect_known_construction_use_evidence` | after the validity check and before the fatal barrier, error-free input produces sorted/deduplicated positive-only ConstructionId vectors. Known used is the union of retained typed surfaces and resolved Pipe Indoor/Outdoor/Underground, GroundHeatExchanger Surface, DaylightingDevice Tubular, and EMS ConstructionIndexVariable references in source-family order. Known CTF use contains only Opaque, including F/C, or AirBoundary IDs reached from the GroundHeatExchanger/EMS pair; all three window kinds are excluded. It mirrors only the source mark and does not assert AirBoundary CTF coefficient state. Missing/blank/wrong/unresolved fields and prior-error input publish nothing. Absence is unknown: false flags, unused count/names/warnings, `DisplayExtraWarnings`, complete generated/global identity, CTF/CondFD selection, runtime, support promotion, object counts, and graph edges remain deferred |
| inline post-input `ErrorsFound` fatal barrier | final `CompileResult::model = None` on any Error, as a coarse fail-closed analogue only | EnergyPlus stops immediately after `CheckUsedConstructions` at lines 311-313, before `InitSolarViewFactors`; Rust does not model that exact checkpoint, source diagnostic ordering, fatal message, or side effects, and this inline barrier has no synthetic routine entry |
| `ViewFactorInformation`, Solar enclosure `spaceNums`/`NumOfSurfaces`/`F`/`Area`/`SolAbsorptance`/`Azimuth`/`Tilt`/`SurfacePtr`/`SurfaceReportNums`, Surface Solar/Radiant enclosure back-pointers, and Zone `EnforcedReciprocity` | source-mapped `InitSolarViewFactors`; isolated `ep_runtime::heat_balance::radiation::{energyplus_approximate_view_factors, fix_energyplus_approximate_view_factors}` diagnostic helpers and the `approximate_view_factors_match_energyplus_1zone_eio` evidence test | the parent call is `HeatBalanceManager.cc` line 316. Full support requires report-option and EIO/debug behavior, user-factor alignment, complete merged Solar enclosure/Space/surface topology with AirBoundary exclusion, pointer/global-report ordering, geometric and inside-absorptance inputs, zero/one/at-most-three-surface branches, approximate/user matrices, InternalMass-aware fixing, `EnforcedReciprocity`, 401st-iteration and fatal paths, and all report side effects. Existing helpers and one fixture prove only bounded calculations and do not promote the routine |
| `DataSurfaces::FrameDividerProperties` | `ep_model::WindowFrameAndDivider`, `ep_model::WindowFrameProperties`, `ep_model::WindowDividerProperties`, `ep_model::WindowRevealProperties` | complete bounded immutable user-input descriptors and an independent normalized namespace are typed; fenestration binding, geometry, WINDOW 5 synthesis, shading mutation, window physics, NFRC calculations, reporting, and runtime remain blocked |
| `Construction::ConstructionProps::{Name, TotLayers, LayerPoint, isTCWindow, isTCMaster, TCMasterMatNum, TCLayerNum, TCGlassNum}` and construction/material CTF data | `ep_model::Construction`, optional immutable thermochromic master metadata, separate immutable thermochromic series/child projections, `ep_model::ModelGraph::construction_materials`, checked runtime direct-index construction/material lookup, and `ep_runtime::SurfaceCtfState` | ordinary input layers resolve into a bounded opaque/fenestration construction; every thermochromic parent contributes its first glazing state to the effective stack and only the final parent owns zero-based master metadata, while a sole SimpleGlazingSystem layer retains its original material identity and Fenestration kind. derives private ordered child snapshots without mutating the global construction/name/graph arenas or granting child ConstructionIds. Existing graph edges follow the effective or retained master IDs. The opaque runtime cache, static Regular/AirGap/IRT EIO evidence, diagnostic steady/no-mass coefficient seeding, and CTF histories do not enable thermochromic/window execution, multi-layer SimpleGlazing quirks, global child construction integration, mass-material coefficient generation, or broad face-temperature solving |
| F/C-factor construction flags, source dimensions/factors, `NominalR`, and generated material layer points | `ep_model::ConstructionGroundFactor`, private generated entries in `TypedModel::materials`, and `ModelGraph::construction_materials` | exact bounded generation formulas, ordinary-then-F-then-C ordering, private names, raw ordinals, and two graph edges are retained; surface pairing, ground temperatures, CTF/runtime, reporting, and public attachment targeting remain blocked |
| `ConstructionProps::{TypeIsAirBoundary, TypeIsAirBoundaryMixing, AirBoundaryACH, AirBoundaryMixingSched}` with zero `TotLayers` | `ep_model::ConstructionKind::AirBoundary`, `ConstructionAirBoundary`, `AirBoundaryAirExchange`, and `AirBoundaryMixingSchedule` | lexical-order zero-layer descriptors retain `None` or `SimpleMixing` input state with an optional typed schedule identity or explicit always-on selector and emit no construction/material edge; surface pairing, enclosure remapping, generated cross-mixing, schedule sampling, reporting, and runtime remain blocked |
| `ConstructionProps::BSDFInput`, complex-state layer points, `WindowThermalModel:Params`, and `Matrix:TwoDimension` snapshots | `ep_model::ConstructionKind::ComplexFenestration`, `ConstructionComplexFenestrationState`, `WindowThermalModelParameters`, `ComplexFenestrationMatrix`, `ComplexFenestrationOpticalLayer`, and `ModelGraph::construction_materials` | the bounded LBNLWINDOW/None declaration state retains normalized helper identities, original matrix spelling, derived basis length, dimension-checked global/solid optical snapshots, an alternating SpectralAverage-or-ComplexShade/Gap layer pack, and every ordered graph edge; helper families remain raw-only, while surfaces, BSDF/TARCOG/WCE calculations, reporting, runtime, and conformance remain blocked |
| `ConstructionProps::{SourceSinkPresent, SourceAfterLayer, TempAfterLayer, SolutionDimensions, ThicknessPerpend, userTemperatureLocationPerpendicular}` | optional `ep_model::ConstructionInternalHeatSource` metadata on `ep_model::Construction` | lexical epJSON overlay state is validated before attaching to one ordinary opaque construction with at least two layers; it retains the normalized diagnostic source name, strict one-based interfaces, dimension selector, authored spacing, derived half-spacing, and perpendicular temperature position without changing construction/material identity, layers, or graph edges. Source recovery, broader targets, global flags, CTF/QTF state, runtime consumers, reporting, and conformance remain blocked |
| equivalent-layer `ConstructionProps::{WindowTypeEQL, EQLConsPtr, TotLayers, LayerPoint}` | `ep_model::ConstructionKind::WindowEquivalentLayer`, `ConstructionWindowEquivalentLayer`, and `ModelGraph::construction_materials` | a staged-IDF-ordered or native-epJSON-lexical declaration retains one to eleven contiguous EquivalentLayer-family material identities, every graph edge, and a zero-based source ordinal; downstream topology repair, ASHWAT state, surfaces, reporting, runtime, ratings, and conformance remain blocked |
| `Construction:WindowDataFile` request name/file selector before `SearchWindow5DataFile` | `ep_model::ConstructionWindowDataFileRequest` and `WindowDataFileSource` | staged-IDF or native-epJSON order, normalized search name, default-versus-explicit retain-case file source, and zero-based request ordinal are retained without creating a construction/material/frame/graph identity; all external parsing, synthesized state, surfaces, reporting, runtime, and conformance remain blocked |
| zone predictor histories, sums, and coefficients such as `MAT`, `XMAT`, `DSXMAT`, `SumHA`, `SumHATsurf`, `SumHATref`, `TempDepCoef`, `TempIndCoef`, `AirPowerCap`, and `TempHistoryTerm` | `ep_runtime::ZoneHeatBalanceState`, `ep_runtime::ZoneAirTemperatureCoefficients`, `predict_direct_zone_dual_setpoint_third_order_demand`, and `advance_heat_balance_state_one_timestep_with_direct_zone_purchased_air` | release-wires the fixed nonadaptive ThirdOrder predictor and same-step system-air correction only for the exact one-Zone topology. General predictor/corrector algorithms, four-slot/Space/room-air histories, adaptive iteration, warmup, and lagged-system-load ownership remain unported |
| internal gain sums such as `SumIntGain` | `simulate_zone_internal_convective_gains` and future state fields | convective trace conformance only for declared v0.26 case |

## CTF Porting Notes

EnergyPlus 26.1.0 anchors for opaque conduction:

- `Construction.hh` defines `MaxLayersInConstruct`, `ConstructionProps::TotLayers`,
  `LayerPoint`, and CTF arrays `CTFOutside`, `CTFCross`, `CTFInside`, and
  `CTFFlux`.
- `Construction.cc::ConstructionProps::calculateTransferFunction` consumes the
  material layer physical properties, handles all-resistive, reversed, and
  state-space paths, and emits the EIO `Construction CTF` rows.
- In those EIO rows, `#CTFs` is EnergyPlus `NumCTFTerms`, the number of
  transfer-function coefficient terms. It is not the per-surface history
  cadence; cadence is governed by the construction `CTFTimeStep` relative to
  `TimeStepZone` and EnergyPlus `NumHistories`. The official 1Zone FLOOR row
  reports `#CTFs=5` and `Time Step {hours}=0.250`, which matches the 15-minute
  zone timestep and therefore does not by itself imply a multi-zone-timestep
  master-history interpolation path.
- `Construction.cc::ConstructionProps::printReport` emits CTF coefficient
  rows in descending array-index order (`NumCTFTerms` down to `0`), but the row
  index is still the EnergyPlus CTF array index. `HeatBalanceSurfaceManager.cc`
  consumes history terms by looping `Term = 1..NumCTFTerms`, so Rust intentionally
  sorts parsed EIO rows by `time_index` before storing runtime history vectors.
- `HeatBalanceSurfaceManager.cc` builds `SurfCTFConstInPart` and
  `SurfCTFConstOutPart` from temperature and flux histories before calculating
  current inside/outside conduction fluxes and face temperatures.
- `CalcHeatBalanceInsideSurf2CTFOnly` uses `IterDampConst = 5.0`, subtracts
  `CTFCross[0]` from the inside denominator for adiabatic surfaces, and uses
  `CTFCross[0] * SurfTempOutHist(1)` for standard opaque surfaces.
- `CalcHeatBalanceInsideSurf2CTFOnly` builds `SurfTempTerm` from
  `SurfCTFConstInPart`, `SurfQdotRadIntGainsInPerArea`, `SurfOpaqQRadSWInAbs`,
  `SurfQAdditionalHeatSourceInside`, `HConvInt * RefAirTemp`, and
  `SurfQdotRadHVACInPerArea`, then adds `SurfQdotRadNetLWInPerArea` in the
  standard no-pool branch. Rust now feeds the `SurfQdotRadIntGainsInPerArea`
  slot from `OtherEquipment` radiant fractions using the EnergyPlus
  inside-layer area-absorptance normalization while retaining outside-layer
  absorptance for exterior solar and longwave forcing. The other inside
  shortwave, additional heat source, HVAC radiant, and net longwave slots
  remain explicit per-surface source-map fields so future solar/radiation
  wiring can be isolated without changing the CTF face solver API again.
- `CalcHeatBalanceInsideSurf2CTFOnly` keeps the previous inside surface
  temperature in `SurfTempInsOld` for the iterative damping term; Rust now
  preserves the previous per-surface inside-face temperature before its
  zone-air predictor pass overwrites the current face estimate.
### Coupled source ownership

- `DataHeatBalance::SurfInitialConvCoeff = 3.076 W/m2-K` initializes inside
  convection before algorithm selection. `LowHConvLimit = 0.1` and
  `HighHConvLimit = 1000` bound calculated coefficients.
  `DataHeatBalSurface.hh::ItersReevalConvCoeff = 30` owns the inside-loop
  re-evaluation cadence; `MaxAllowedDelTemp = 0.002 C` is the mapped
  convergence threshold. Changing cadence to reduce a residual is not a
  substitute for preserving source order.
- `CalcASHRAEDetailedIntConvCoeff` dispatches inside TARP through
  `CalcASHRAETARPNatural(SurfTempIn, RefAirTemp, -CosTilt)` using ASHRAE
  vertical and Walton stable/unstable horizontal-or-tilt correlations.
- `CalcHeatBalanceOutsideSurf` uses `-SurfCTFConstOutPart`, current
  `CTFCross[0] * SurfTempIn`, absorbed sources, and exterior coefficients in
  the no-movable-insulation balance. `InitExtConvCoeff` precedes that solve;
  the coefficient-evaluation temperature, solved outside temperature, inside
  solve's outside-history snapshot, and reporting state are distinct owners.
- Outside DOE-2 combines windward/leeward MoWITT forced terms, TARP natural
  convection, and roughness multipliers. `SetSurfaceWindSpeedAt` uses weather
  station defaults, terrain, and surface centroid height; NoWind keeps local
  wind zero. Timestep wind speed/direction is not raw-hourly weather.
- Exterior longwave splits `SurfHSkyExt`, `SurfHGrdExt`, and `SurfHAirExt`
  using outside absorptance, sky/ground IR view factors, and
  `SurfAirSkyRadSplit = sqrt(0.5 * (1 + CosTilt))`. The balance uses
  `(SurfHConvExt + SurfHAirExt) * TempExt + SurfHSkyExt * TSky +
  SurfHGrdExt * TGround`; reporting reuses the corresponding coefficient
  state rather than recomputing it from the solved face temperature.
- Weather rain uses `IsRainThreshold = 0.8 / TimeStepsInHour`. Exposed wet
  surfaces use `SurfHConvExt = 1000.0` and `SurfOutWetBulbTemp`. Exterior
  forcing consumes timestep-interpolated dry bulb, wind, horizontal IR/sky
  temperature, precipitation, and solar inputs in both warmup and reporting
  periods. Weather day-of-year follows the run calendar and EPW leap policy,
  not the printed TMY source year.
- `AnisoSkyViewFactors` and the surface manager split incident shortwave into
  beam, Perez anisotropic sky diffuse, and ground-reflected terms. Current
  timestep `SOLCOS` and shadowing-period beam/sunlit state are separate inputs;
  the legacy unobstructed 0/1 sunlit proxy does not establish detailed
  obstruction/reflection parity.
- `predictSystemLoad`, `calcZoneOrSpaceSums`, and `calcSumHAT` own
  `TempDepCoef`, `TempIndCoef`, `SumHA`, `SumHATsurf`, and `SumHATref`.
  Moist-air capacity and temperature history must come from the applicable
  state owner. A weather-RH capacity proxy in a diagnostic lane does not
  establish the `ZoneAirHumRat` path.
- The relevant timestep order is `ManageSurfaceHeatBalance` with outside
  then inside surface work, followed by `ManageAirHeatBalance`,
  `CalcHeatBalanceAir`, `ManageHVAC`, and zone-air predictor/corrector work.
  `CalcHeatBalanceAir` delegates to `HVACManager::ManageHVAC`;
  `correctAirTemp` selects ThirdOrder, AnalyticalSolution, or EulerMethod
  from freshly formed convection/source coefficients. Radiation, face solve,
  air correction, and history commit form one state transition.
- `UpdateThermalHistories` computes current inside/outside flux slot 1,
  flips the outside flux sign for `SurfOpaqOutFaceCondFlux`, then shifts
  temperature/flux into history slot 2 in the `SimpleCTFOnly` path. The Rust
  retained history vectors represent slot 2 onward, not current slot 1.
  An adiabatic report-only or history-only face replacement cannot be assumed
  equivalent to the coupled transition.
- `ReadWeatherForDay::firstHrInterpUseHr1` and the RunPeriod
  `First Hour Interpolation Starting Values` choose Hour1 or Hour24 of the
  first weather day. The default is Hour24 of that day, not the last EPW
  file record. Source weather initialization and repeated-day warmup are
  distinct from copying the warmup end state into the reporting period.

### Legacy diagnostic boundary

The [probe lifecycle ledger](../../../specs/diagnostic_probe_ledger.toml)
classifies each executable selector and wrapper. Its closed entries remain
closed, with explicit `-IncludeClosed` replay; the unresolved observation
hypothesis and its latest recorded sample remain in the
[registry above](#diagnostic-probe-lifecycle-ledger). Removing repeated RMSE
chronology does not turn rejected probes into passing results.

| Diagnostic family | Durable result and limitation |
|---|---|
| Surface/air coupling, frozen/current convection, reference-air, ScriptF and iteration forks | Local improvements traded against other surface/zone rates. These were isolation probes, not evidence that an equation, cadence, or owner could be replaced solely to reduce error. |
| Adiabatic outside-face, history-commit, and report-only replacements | Rejected or incomplete comparisons keep outside solve, inside history, reporting, and final commit separate; no general replacement was promoted. |
| Report-only storage/convection and weather-capacity forks | Reporting changes can hide a state residual; they do not certify the underlying moist-air or coupled source state. |
| Repeated-day warmup and first reported sample | The retained legacy hypothesis directly observes `SurfTempIn` mismatch. Weather forcing, first-hour averaging, and state-copy checks did not establish its upstream cause. |

The legacy diagnostic implementation retains these interfaces:

- `ep_model::Construction.layers` is outside-to-inside; `outside_layer` is
  the outside-face compatibility field. Material roughness uses EnergyPlus
  categories.
- `ep_compare` associates EIO CTF rows with the preceding construction row;
  diagnostic metadata retains `#CTFs` and CTF timestep separately. EIO seeding
  belongs to diagnostic isolation, not ordinary Rust coefficient generation.
  The simplified CLI diagnostic defaults to steady/no-mass seeds; explicit
  all-EIO replay has its own policy.
- `compare-digest.json` carries summary/bottleneck and CTF metadata without
  full hourly sample rows. `zone_air_first_sample_trace` and
  `surface_first_sample_trace` expose Rust substep coefficients, history,
  weather, and balance inputs. Internal reconstruction or a zero-baseline
  temperature/flux split magnitude is not an independent Native state delta.
- `CompatibilityHeatBalanceAlgorithm`, `DiagnosticHeatBalanceProbe`, and
  `HeatBalanceRuntimeConfig` separate compatibility selection from diagnostic
  flags. The diagnostic module resolves selectors before compatibility code
  runs; a probe selector cannot be relabelled conformance evidence.
- The legacy exterior path uses source-shaped terrain, DOE-2, sky/air/ground
  radiation, wet-surface, and unobstructed solar inputs. Its simplified
  surface/air shell, ground default `18.0 C`, weather-capacity proxy, detailed
  shadowing/reflection, active source/sink terms, and unobserved branches
  remain subject to their declared contracts rather than broad parity claims.
- `ExecutionPlan.stages` and `compatibility_stages` preserve the mapped
  input/warmup/surface/air/history barrier sequence. A matching stage list
  alone does not prove numerical or state-transition equivalence.

The completed finite A/B work is independently documented in the
[porting plan](../../../energyplus_porting_plan/README.md) and its
[accepted Source validation](../../../energyplus_porting_plan/evidence/SRC-06-final-current-source-validation-60/README.md).
That evidence binds the recorded normal Rust executions and explicit card
scope. It does not certify every legacy diagnostic selector, the full
EnergyPlus surface population, general source/sink physics, or whole-file IO
equality. The earlier rejected runs remain in Git history and their retained
evidence; the accepted packet is unchanged by this documentation cleanup.

## Required Cases Before Porting

- `heat_balance_uncontrolled_001`: one-zone uncontrolled, no HVAC, opaque
  surfaces only
- `heat_balance_nomass_001`: `Material:NoMass` variant
- `heat_balance_mass_001`: simple mass material variant
- `material_opaque_variants_001`: nonblocking static grouped-EIO layer evidence
  for the exact Regular/AirGap/IRT construction fixture

The heat-balance cases may remain diagnostic-only until v0.8 declares
tolerances and blocking gates. `material_opaque_variants_001` remains a
nonblocking static smoke and does not supply dynamic heat-balance evidence.

## Stop Rule

No heat-balance algorithm change may be promoted as conformance work unless the
changed behavior has a source-map entry in this document, an output-variable
entry in `output-variable-source-map.md`, an algorithm entry in
`specs/algorithm_ledger.toml`, and the port-ticket fields required by
`docs/src/current/project-contract.md`.


## Additional Bounded State Contracts

### `CheckValidSimulationObjects` state contract

<!-- routine-state-contract:v1 begin check_valid_simulation_objects -->
CheckValidSimulationObjects

read_state:
- the inline `GetHeatBalanceInput` gate immediately after `CreateTCConstructions`: EnergyPlus calls `CheckValidSimulationObjects` only when `TotSurfaces > 0 && NumOfZones == 0`, then calls `CheckUsedConstructions`; the parent routine and exact gate remain source-mapped
- bounded Rust enters immediately after the CP103 projection, suppresses the check after any prior compile Error, requires an empty typed Zone arena, and accepts only raw `Shading:Site:Detailed` or `Shading:Building:Detailed` presence as a monotonic positive witness that at least one source Surface exists
- validity is raw presence only across EnergyPlus's ordered eight-family test: `SolarCollector:FlatPlate:Water`, `Generator:Photovoltaic`, `Generator:InternalCombustionEngine`, `Generator:CombustionTurbine`, `Generator:FuelCell`, `Generator:MicroCHP`, `Generator:MicroTurbine`, or `Generator:WindTurbine`; fields, names, references, typing, and runtime support are not inspected by this check

write_state:
- no model state is written; when the bounded surface witness exists, the typed Zone arena is empty, no prior Error exists, and all eight allowed raw families are absent, the compiler emits one blocking `InvalidSimulationWithoutZones` Error attached to object type `GetHeatBalanceInput` with no object name or field
- a prior Error, any typed Zone, absence of both bounded surface witnesses, or presence of any one allowed raw family suppresses this diagnostic without publishing a flag, identity, graph edge, capability, manifest, comparator, proof variable, or runtime state
- allowed-family presence affects only this bounded validity diagnostic and never promotes that family to typed input support or runtime execution

history_state_ownership:
- the compiler owns no new persistent state for this diagnostic-only check; it reads immutable typed and raw input snapshots and may append one compile Error, with no surface, Zone, generator, collector, or runtime history

unsupported_state:
- the complete EnergyPlus `TotSurfaces` population and order, including legacy rectangular shading, Zone shading, heat-transfer and fenestration surfaces, generated opposite or multiplier surfaces, and every other surface family omitted from the two bounded positive witnesses
- input parsing, field/reference validation, simulation state, electrical or thermal behavior, and runtime support for every allowed collector/generator family; raw count presence is not a support claim
- the caller's mutable `ErrorsFound` flag, later fatal termination, full input-processor recovery behavior, and interaction with diagnostics or state outside the bounded compiler pass

inactive_branches:
- any prior compile Error suppresses the check so a diagnostic is not inferred from partially published typed arenas
- a nonempty typed Zone arena or no raw detailed Site/Building shading witness is silent; the latter is inconclusive rather than proof that EnergyPlus `TotSurfaces` is zero
- presence of any one of the eight raw allowed families is silent regardless of whether that family has typed or executable Rust support

unsupported_active_branches:
- an allowed raw collector/generator witness only suppresses `InvalidSimulationWithoutZones`; its own unsupported-object, malformed-input, or run-blocking treatment remains independent and unchanged
- a detected no-Zone invalid-simulation witness is compile-blocking and adds no partial runtime admission or simulation state

not_claimed_branches:
- exact parity for `TotSurfaces > 0 && NumOfZones == 0`, negative inference from absent detailed Site/Building shading, any other surface family, legacy `Shading:Site` or `Shading:Building`, generated surfaces, or invalid/raw Zone recovery
- exact EnergyPlus severe/fatal text, punctuation, severity mapping, order, multiplicity, `ErrorsFound` timing, or downstream termination; negative construction-use flags and unused warnings beyond CP105 positive metadata, collector/generator semantics, runtime, reporting, and conformance
<!-- routine-state-contract:v1 end check_valid_simulation_objects -->

### `CheckUsedConstructions` state contract

<!-- routine-state-contract:v1 begin check_used_constructions -->
CheckUsedConstructions

read_state:
- the source call immediately after the inline no-Zone validity gate and before the caller's fatal `ErrorsFound` barrier; bounded Rust calls `Compiler::collect_known_construction_use_evidence` after `check_valid_simulation_objects_bounded` in the same relative order
- every retained typed `BuildingSurface:Detailed` contributes its already resolved ConstructionId as monotonic known-used evidence; this neither claims the complete EnergyPlus Surface arena nor interprets an absent typed surface as proof of non-use
- the six source-ordered raw reference scans and exact fields: `Pipe:Indoor.construction_name`, `Pipe:Outdoor.construction_name`, `Pipe:Underground.construction_name`, `GroundHeatExchanger:Surface.construction_name`, `DaylightingDevice:Tubular.construction_name` corresponding to legacy alpha field 4, then `EnergyManagementSystem:ConstructionIndexVariable.construction_object_name`
- raw references contribute only when the field is a nonblank string that case-insensitively resolves to an existing typed ConstructionId; missing fields, blanks, wrong JSON value types, and unresolved names are silent and publish no diagnostic or placeholder
- only resolved GroundHeatExchanger and EMS references are candidates for known-CTF-use evidence, and only when the typed construction kind is Opaque, including ordinary and F/C generated variants, or AirBoundary; Fenestration, ComplexFenestration, and WindowEquivalentLayer are the exact excluded window kinds
- known-CTF-use is only positive metadata mirroring the source non-window `IsUsedCTF` mark; inclusion of an AirBoundary ConstructionId does not assert that the construction owns or requires CTF coefficient state
- any compile Error already present when the collector begins, including one emitted by the preceding CP104 check, makes the collection transactionally inactive and leaves both evidence vectors empty

write_state:
- `TypedModel::known_used_constructions` is a sorted and deduplicated positive-only ConstructionId set containing the union of retained typed-surface references and all resolved references from the six raw families
- `TypedModel::known_ctf_used_constructions` is a separately sorted and deduplicated positive-only ConstructionId subset containing only qualifying non-window GroundHeatExchanger and EMS references; Pipe, tubular daylighting, and retained typed-surface references never imply CTF evidence in this checkpoint
- the two vectors add no public object identity, ConstructionId, construction name, object count, graph edge, support row, capability, manifest, comparator, proof variable, diagnostic, or runtime state

history_state_ownership:
- TypedModel owns two immutable positive-evidence ConstructionId vectors; no mutable `IsUsed`/`IsUsedCTF` flag, surface history, construction history, CTF/CondFD history, or runtime selection state is allocated

unsupported_state:
- the complete EnergyPlus global Construction arena after WINDOW5 and thermochromic child append, generated identities absent from the typed ConstructionId arena, all Surface families beyond the retained typed detailed opaque subset, and any other use-marking path outside the six bounded raw scans
- negative `IsUsed` or `IsUsedCTF` state, the source count of constructions lacking use, summary and per-name unused-construction warnings, the `DisplayExtraWarnings` branch, and exact warning text/severity/order/multiplicity
- downstream CTF, CondFD, HAMT, window, daylighting, pipe, ground-heat-exchanger, EMS, and surface consumers; the raw six-family scan validates no referenced object's own fields, dependencies, or executability

inactive_branches:
- any prior compile Error leaves both evidence vectors empty so partially published typed arenas cannot yield positive metadata
- missing, blank, wrong-typed, or unresolved raw construction references remain silent and add no evidence; they are not reclassified as malformed typed definitions by this routine
- duplicate evidence from multiple surfaces, repeated raw records, or overlap between typed-surface and raw references collapses to one ConstructionId in each independently sorted vector
- a resolved Fenestration, ComplexFenestration, or WindowEquivalentLayer GroundHeatExchanger/EMS reference adds known-used evidence but no known-CTF-used evidence; the same reference from Pipe or tubular daylighting never adds CTF evidence regardless of construction kind

unsupported_active_branches:
- raw Pipe, GroundHeatExchanger, DaylightingDevice, and EMS object presence remains outside typed object coverage and runtime support; a resolved construction-reference string contributes metadata only and never promotes the referring family
- known-used or known-CTF-used evidence does not remove any existing all-definition run blocker, admit partial runtime, choose a heat-transfer algorithm, or prove that another construction is unused

not_claimed_branches:
- complete source `IsUsed`/`IsUsedCTF` mutation parity, false-state parity, complete Construction or Surface coverage/order, source invalid-input recovery, WINDOW5 or thermochromic-child identity, and negative inference from either evidence vector
- unused construction counts or names, `DisplayExtraWarnings`, exact diagnostics/order/multiplicity, CTF/CondFD selection or initialization, pipe/ground-heat-exchanger/daylighting/EMS semantics, runtime, reporting, numerical parity, capability support, and conformance
<!-- routine-state-contract:v1 end check_used_constructions -->

### `ProcessZoneData` state contract

<!-- routine-state-contract:v1 begin process_zone_data -->
ProcessZoneData

read_state:
- EnergyPlus calls `GetBuildingData` after `GetConstructData` and orders `GetShadowingInput`, `GetZoneData`, then `SetupZoneGeometry`; bounded Rust state covers only each Zone declaration processed at the start of `GetZoneData`, while both wrappers remain source-mapped
- one required nonblank outer-key name; finite relative-north and x/y/z origins default to zero without numeric bounds, Type defaults to and must equal integer 1, and Multiplier defaults to integer 1 with source-compatible positive signed-integer range
- Ceiling Height, Volume, and Floor Area retain every finite authored number including zero or negative values, while missing, blank, or `Autocalculate` input retains an explicit `AutoCalculate` selector
- inside overrides accept Simple, TARP, CeilingDiffuser, TrombeWall, AdaptiveConvectionAlgorithm, or ASTMC1340; outside overrides accept SimpleCombined, TARP, MoWiTT, DOE-2, or AdaptiveConvectionAlgorithm; blank or missing values inherit already parsed global selections, defaulting to TARP and DOE-2 when no global object exists
- Part of Total Floor Area defaults to Yes and retains Yes as true or No as false; staged IDF Zone instances use recovered declaration order while native epJSON instances use lexical key order

write_state:
- a deterministic dense `Zone` arena and normalized name map retain each validated `ZoneId`, normalized name, relative-north angle, origin, source-forced standard Type value 1, positive multiplier, three `AutoOrNumber` selectors, and total-floor-area membership flag
- each convection selection retains both its effective enum and whether it was inherited or authored locally; all fields and normalized duplicate checks complete before ID/name publication, so an invalid earlier record does not consume a dense identity
- positive authored Floor Area is preferred by shared runtime geometry, schedule/internal-gain, CLI IdealLoads, and typed IdealLoads outdoor-air consumers; nonpositive or autocalculated values fall through to the available geometry path rather than being normalized during declaration parsing
- any zone-local convection override is reported as `UnsupportedZoneConvectionOverride` and `RunBlocked` before arbitrary runtime execution because the current coefficient paths do not consume zone-local selectors

history_state_ownership:
- TypedModel owns immutable Zone declaration descriptors only; this checkpoint allocates no mutable zone-air, surface, weather, equipment, geometry, reporting, or predictor/corrector history

unsupported_state:
- the remaining `GetZoneData` stages and allocations outside this routine: pre-loop ZoneDaylight and resilience arrays, the separately bounded post-Zone collection, local-environment, and Space declaration/default contracts, plus post-local-environment ZonePreDefRep allocation
- `SetupZoneGeometry` realization of ceiling height, volume, and floor area; five-percent comparison warnings, space-area adjustment, nonpositive-volume recovery, coordinate-system warnings, centroids, bounds, and other surface-derived state
- four zone outdoor-air output registrations, weather and EMS updates, actual inside/outside convection coefficient selection, CeilingDiffuser recovery, building floor-area reporting, EIO/SQLite output, and conformance evidence

inactive_branches:
- missing or blank local convection fields inherit the effective project setting and do not add a runtime boundary; when the project object is absent the retained effective defaults are TARP inside and DOE-2 outside
- missing, blank, or `Autocalculate` geometry selectors remain explicit auto state; authored zero or negative values remain finite declaration values but are treated as geometry selectors requiring downstream realization
- Part of Total Floor Area equal to No is retained but has no current building-total reporting consumer; Type always materializes as the standard value 1

unsupported_active_branches:
- every authored zone-local inside or outside convection override, including one equal to the inherited effective value, is typed but blocks arbitrary runtime execution until the zone-local coefficient consumer is wired
- Space declaration/default topology, nominal control, Zone collections, and local-environment node linkage belong to separate bounded GetSpaceData, GetZoneData, and GetZoneLocalEnvData contracts; typed detailed-surface lookup/same-Zone validation and remainder/fallback assignment belong to the bounded CP97 GetHTSurfaceData/CreateMissingSpaces composite, while full GetHTSurfaceData and broader geometry realization remain deferred

not_claimed_branches:
- complete `GetBuildingData` or `GetZoneData` parity, broad Rust compiler pass-order parity, source control-character restoration, source/native canonical enum-case behavior, whitespace-preserving or case-colliding names, shared Zone/Space/ZoneList/SpaceList namespace uniqueness, invalid-input recovery, exact diagnostics/order/multiplicity, downstream geometry correction and warnings, output registration, runtime convection, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end process_zone_data -->
