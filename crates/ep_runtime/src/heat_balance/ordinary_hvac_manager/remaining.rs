//! Stateless actual HVAC child routing; all values are borrowed for this call.
use super::provider_host::SourceHvacRemainingChildren;
use super::simulation::output_registration::SourceHvacOutputContext;
use super::{Error, SourceHvacChild, SourceHvacManager, SourceHvacOwners};
use crate::heat_balance::{
    convection::surface_environment::source_rows::SourceSurfaceOutdoorState,
    ctf_thermal_history::CtfThermalHistoryState,
    facility_electric_service::{
        FacilityElectricService, SourceFacilityElectricOwners, SourceFacilityOutputContext,
    },
    ordinary_air_heat_balance::{
        SourceAirHeatBalance,
        environment::SourceZoneFanHeatBalanceArrays,
        output_registration::SourceAirOutputOwners,
        predictor_corrector::{
            SourceZoneAirPredictorCorrector,
            oscillation::{SourceOscillationOutputContext, SourceOscillationOwners},
            simple_air::SourceAirReportOutputOwners,
        },
    },
    ordinary_exterior_energy_use::SourceExteriorEnergyUse,
    ordinary_equipment::OrdinaryEquipment,
    ordinary_pollution::SourcePollutionModule,
    ordinary_surface_initialization::{
        SourceSurfaceHeatBalanceArrays, output::SourceSurfaceOutputOwners,
    },
    ordinary_surface_management::SourceResilienceOutputOwners,
    ordinary_system_reports::SourceSystemReports,
    ordinary_thermal_comfort::SourceThermalComfort,
    radiation::ordinary::{
        OrdinaryRadiationError, OrdinaryRadiationUnavailable, OrdinarySolarRadiation,
        solar_output_registration::SourceSolarOutputOwners,
    },
    selected_heat_balance_input::{
        SelectedHeatBalanceInputState, surface::SurfaceGeometryInputState,
        zone::SelectedZoneInputOwners,
    },
    selected_schedule_manager::SelectedScheduleManager,
    source_output_registration::{
        SourceHvacRealOutputField, SourceOutputDictionaryHost, SourceOutputFileHost,
        SourceRealOutputOwners, SourceRealOutputTarget,
    },
};
use std::io::Write;

mod output_update;
mod tabular_update;
pub(crate) struct SourceOrdinaryHvacRemaining;

/// These two live fields do not require borrowing the whole mutable manager.
struct HvacLossFields<'a>(&'a SelectedHeatBalanceInputState);
impl SourceRealOutputOwners for HvacLossFields<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        match target {
            SourceRealOutputTarget::Hvac {
                field: SourceHvacRealOutputField::SystemReliefHeatLoss,
            } => Ok(&self.0.sys_total_hvac_relief_heat_loss),
            SourceRealOutputTarget::Hvac {
                field: SourceHvacRealOutputField::SystemRejectHeatLoss,
            } => Ok(&self.0.sys_total_hvac_reject_heat_loss),
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}

/// Facility composes its own Which references within each read. Pollution's
/// sixteen energy-meter reads exclude its own carbon outputs. A forbidden
/// partial-view read is an explicit error, never a fabricated numeric zero.
struct LiveReadView<'a> {
    nodes: &'a OrdinaryEquipment,
    weather_outputs: Option<&'a dyn SourceRealOutputOwners>,
    schedules: &'a SelectedScheduleManager,
    surface: &'a dyn SourceRealOutputOwners,
    solar: &'a dyn SourceRealOutputOwners,
    air: &'a dyn SourceRealOutputOwners,
    comfort: &'a dyn SourceRealOutputOwners,
    hvac: &'a dyn SourceRealOutputOwners,
    oscillation: &'a dyn SourceRealOutputOwners,
    exterior: &'a dyn SourceRealOutputOwners,
    facility: Option<&'a FacilityElectricService>,
    pollution: Option<&'a SourcePollutionModule>,
    system_reports: &'a SourceSystemReports,
}
impl SourceRealOutputOwners for LiveReadView<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        use SourceRealOutputTarget as T;
        match target {
            T::Node { .. } | T::PurchasedAir { .. } => self.nodes.real_output(target),
            T::Weather { .. } => self
                .weather_outputs
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
                .real_output(target),
            T::Schedule { .. } => self.schedules.real_output(target),
            T::SurfaceHeatBalance { .. } => self.surface.real_output(target),
            T::Solar { .. } => self.solar.real_output(target),
            T::Air { .. } => self.air.real_output(target),
            T::Comfort { .. } => self.comfort.real_output(target),
            T::Hvac { .. } => self.hvac.real_output(target),
            T::Oscillation { .. } => self.oscillation.real_output(target),
            T::ExteriorLight { .. } => self.exterior.real_output(target),
            T::SystemReports { .. } => self.system_reports.real_output(target),
            T::Facility { .. } => self
                .facility
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
                .real_output(target),
            T::Pollution { .. } => self
                .pollution
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
                .real_output(target),
        }
    }
}

struct LiveReadFields<'a> {
    nodes: &'a OrdinaryEquipment,
    weather_outputs: Option<&'a dyn SourceRealOutputOwners>,
    schedules: &'a SelectedScheduleManager,
    arrays: &'a SourceSurfaceHeatBalanceArrays,
    surface_heat_emission: &'a f64,
    geometry: &'a SurfaceGeometryInputState,
    history: &'a CtfThermalHistoryState,
    physical_surfaces: &'a [SourceSurfaceOutdoorState],
    radiation: &'a OrdinarySolarRadiation,
    air: &'a SourceAirHeatBalance,
    controller: &'a SourceZoneAirPredictorCorrector,
    fan: &'a SourceZoneFanHeatBalanceArrays,
    zones: &'a SelectedZoneInputOwners,
    control: &'a SelectedHeatBalanceInputState,
    comfort: &'a SourceThermalComfort,
    exterior: &'a SourceExteriorEnergyUse,
    facility: Option<&'a FacilityElectricService>,
    pollution: Option<&'a SourcePollutionModule>,
    system_reports: &'a SourceSystemReports,
}
impl LiveReadFields<'_> {
    fn with_owners<T>(
        &self,
        read: impl FnOnce(&dyn SourceRealOutputOwners) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let surface = SourceSurfaceOutputOwners {
            arrays: self.arrays,
            sum_surface_heat_emission: self.surface_heat_emission,
            geometry: self.geometry,
            history: self.history,
            physical_surfaces: self.physical_surfaces,
        };
        let solar = SourceSolarOutputOwners {
            radiation: self.radiation,
            geometry: self.geometry,
        };
        let report = SourceAirReportOutputOwners {
            controller: self.controller,
        };
        let resilience = SourceResilienceOutputOwners { zones: self.zones };
        let air = SourceAirOutputOwners {
            air: self.air,
            demand: &self.controller.demand,
            fan: self.fan,
            report: &report,
            resilience: &resilience,
        };
        let hvac = HvacLossFields(self.control);
        read(&LiveReadView {
            nodes: self.nodes,
            weather_outputs: self.weather_outputs,
            schedules: self.schedules,
            surface: &surface,
            solar: &solar,
            air: &air,
            comfort: self.comfort,
            hvac: &hvac,
            oscillation: &self.controller.oscillation,
            exterior: self.exterior,
            facility: self.facility,
            pollution: self.pollution,
            system_reports: self.system_reports,
        })
    }
}

impl SourceOrdinaryHvacRemaining {
    fn facility<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        first: bool,
        update_only: bool,
        external_simulate: Option<&mut bool>,
        manager: &mut SourceHvacManager,
        o: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let fields = LiveReadFields {
            nodes: &*o.equipment,
            weather_outputs: None,
            schedules: o.schedules,
            arrays: o.surface_arrays,
            surface_heat_emission: o.surface_heat_emission,
            geometry: o.geometry,
            history: o.history,
            physical_surfaces: o.physical_surfaces,
            radiation: o.radiation,
            air: o.air,
            controller: o.controller,
            fan: o.fan,
            zones: o.zones,
            control: o.control,
            comfort: o.comfort,
            exterior: o.exterior,
            facility: None,
            pollution: Some(&manager.ancillary.pollution),
            system_reports: &manager.ancillary.system_reports,
        };
        fields.with_owners(|live| {
            let facility = o.facility.as_deref_mut().ok_or(Error::Unavailable(
                "actual electric facility manager instance",
            ))?;
            let mut owners = SourceFacilityElectricOwners {
                input: o.input,
                shortcut: o.shortcut,
                globals: o.globals,
                clock: o.clock,
                time_step_sys_seconds: &o.hvac.time_step_sys_seconds,
                registration: output.registration,
                predefined_registry: o.predefined_registry,
                predefined_table: o.predefined_table,
                real_output_owners: live,
                backend: o.backend,
            };
            let mut context = SourceFacilityOutputContext {
                prescan: output.prescan,
                reports: output.reports,
                minimum_report_frequency: output.minimum_report_frequency,
                minutes_in_time_step: o.clock.minutes_in_time_step,
                host: output.host,
                dictionary_host: output.dictionary_host,
                eio,
            };
            let simulate = match external_simulate {
                Some(actual) => actual,
                None => &mut o.hvac.sim_electric_circuits,
            };
            facility.manage_source_electric_power_service(
                first,
                simulate,
                update_only,
                &mut owners,
                &mut context,
            )
        })
    }
}
impl<
    H: SourceOutputFileHost,
    D: SourceOutputDictionaryHost
        + crate::heat_balance::source_output_registration::SourceOutputReportStreams,
> SourceHvacRemainingChildren<H, D> for SourceOrdinaryHvacRemaining
{
    fn manage_electric_facility(
        &mut self,
        first: bool,
        simulate: &mut bool,
        update_only: bool,
        manager: &mut SourceHvacManager,
        o: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        Self::facility(first, update_only, Some(simulate), manager, o, output, eio)
    }
    fn manage_normal_electric_facility(
        &mut self,
        first: bool,
        manager: &mut SourceHvacManager,
        o: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        Self::facility(first, false, None, manager, o, output, eio)
    }
    fn run(
        &mut self,
        child: SourceHvacChild,
        manager: &mut SourceHvacManager,
        o: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        use SourceHvacChild as C;
        match child {
            C::ManageDemand => manager.manage_source_demand(o),
            C::UpdateZoneListAndGroupLoads => manager.update_source_zone_list_and_group_loads(o),
            C::UpdateIceFractions => manager.update_source_ice_fractions(),
            C::UpdatePlantNodeThermalHistory => manager.update_source_plant_node_thermal_history(o),
            C::UpdateDemandManagers => manager.update_source_demand_managers(),
            C::CheckConcurrentEirPlantLoopHeatPumpOperation => {
                manager.check_source_concurrent_eir_heat_pumps()
            }
            C::CalculateMoreNodeInfo => o.equipment.calculate_source_more_node_info(
                o.environment, &*output.registration, &*o.schedules, o.runtime_language,
            ),
            C::DetectOscillatingZoneTemperature => {
                let mut owners = SourceOscillationOwners {
                    globals: o.globals,
                    zones: o.zones,
                    air: o.air,
                    current_deadband_or_setback: &o.controller.demand.current_deadband_or_setback,
                    comfort: o.comfort,
                    clock: o.clock,
                    hvac: o.hvac,
                    input: o.input,
                    backend: o.backend,
                    eio,
                };
                let mut context = SourceOscillationOutputContext {
                    registration: output.registration,
                    prescan: output.prescan,
                    reports: output.reports,
                    minimum_report_frequency: output.minimum_report_frequency,
                    host: output.host,
                    dictionary_host: output.dictionary_host,
                };
                o.controller
                    .oscillation
                    .detect_source_zone_temperature(&mut owners, &mut context)
            }
            C::CalculatePollution => {
                let fields = LiveReadFields {
            nodes: &*o.equipment,
                    weather_outputs: None,
                    schedules: o.schedules,
                    arrays: o.surface_arrays,
                    surface_heat_emission: o.surface_heat_emission,
                    geometry: o.geometry,
                    history: o.history,
                    physical_surfaces: o.physical_surfaces,
                    radiation: o.radiation,
                    air: o.air,
                    controller: o.controller,
                    fan: o.fan,
                    zones: o.zones,
                    control: o.control,
                    comfort: o.comfort,
                    exterior: o.exterior,
                    facility: o.facility.as_deref(),
                    pollution: None,
                    system_reports: &manager.ancillary.system_reports,
                };
                fields.with_owners(|live| {
                    manager.ancillary.pollution.calculate_source_pollution(
                        &o.hvac.fraction_time_step_zone,
                        output.registration,
                        live,
                        o.schedules,
                    )
                })
            }
            C::CalculateHeatEmissionReport => manager.calculate_source_heat_emission_report(o),
            C::InitializeEnergyReports => manager
                .ancillary
                .system_reports
                .initialize_source_energy_reports(
                    o.globals, o.hvac, o.equipment, &manager.simulation.air_loop,
                    o.zones, o.predefined_registry, o.predefined_table, &*output.registration,
                ),
            C::ReportSystemEnergyUse => manager
                .ancillary
                .system_reports
                .report_source_system_energy_use(o.globals, o.hvac, o.equipment, &o.controller.demand),
            C::ReportVentilationLoads => manager
                .ancillary
                .system_reports
                .report_source_ventilation_loads(
                    o.globals, o.hvac, o.equipment, o.zones, &o.controller.demand,
                    &o.controller.reports, o.air, o.environment, o.sizing, o.comfort_summary,
                ),
            C::ReportAirHeatBalance => manager.report_source_air_heat_balance(o),
            C::UpdateDataAndReportSystem => Self::update_source_ordinary_output(
                crate::heat_balance::source_output_registration::TimeStepKind::System,
                manager,
                o,
                output,
            ),
            C::UpdateTabularReportsSystem => Self::update_source_ordinary_tabular(
                crate::heat_balance::source_output_registration::TimeStepKind::System,
                manager,
                o,
                output,
                eio,
            ),
            C::ContaminantGetZoneSetPoints { .. }
            | C::ContaminantPredictStep { .. }
            | C::ContaminantCorrectStep { .. }
            | C::ContaminantPushSystemHistories { .. }
            | C::ContaminantPushZoneHistories { .. } => Err(Error::Unavailable(
                "actual positive contaminant child pending",
            )),
            C::GatherComponentLoadsHvac
            | C::UpdateZoneSizingDuringDay
            | C::UpdateFacilitySizingDuringDay => {
                Err(Error::Unavailable("actual positive sizing child pending"))
            }
            C::CalculateSimpleAirflow { .. }
            | C::SetHeatToReturnAirFlag
            | C::SimulateHvac
            | C::ManageWaterInits
            | C::ManageWater => Err(Error::Unavailable(
                "actual child must be routed by outer SourceHvacProviderHost",
            )),
        }
    }
}
