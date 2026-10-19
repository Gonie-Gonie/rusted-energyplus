//! Actual System UpdateData callback with temporary views of the same live owners.
use super::*;
use crate::heat_balance::ordinary_hvac_manager::simulation::air_loops::output::SourceAirLoopAndHvacIntegerOwners;
use crate::heat_balance::ordinary_hvac_manager::simulation::output_registration::SourceHvacOutputOwners;
use crate::heat_balance::source_output_registration::{
    SourceAllIntegerOutputOwners, SourceIntegerOutputOwners, SourceIntegerOutputTarget,
    SourceOutputReportStreams, SourceOutputUpdateEnvironment, SourceOutputUpdateOwners,
    TimeStepKind,
};
struct SourceInsideIteration<'a>(&'a i32);
impl SourceIntegerOutputOwners for SourceInsideIteration<'_> {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError> {
        match target {
            SourceIntegerOutputTarget::SurfaceInsideIterationCount => Ok(self.0),
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
impl SourceOrdinaryHvacRemaining {
    pub(crate) fn update_source_ordinary_output<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost + SourceOutputReportStreams,
    >(
        kind: TimeStepKind,
        manager: &SourceHvacManager,
        o: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        let weather = crate::weather::day::output_bindings::SourceWeatherOutputOwners {
            current_weather: &o.environment.current_weather,
            weather: o.weather,
            ground_temp: &o.environment.ground_temp,
            water_mains_temperature: &o.environment.water_mains_temperature,
            rainfall_current_rate: &manager.ancillary.water.data.rainfall.current_rate,
            rainfall_current_amount: &manager.ancillary.water.data.rainfall.current_amount,
            dst_indicator: &o.environment.dst_indicator,
            exterior_horizontal_sky_illuminance: &o.environment.exterior_horizontal_sky_illuminance,
            exterior_horizontal_beam_illuminance: &o
                .environment
                .exterior_horizontal_beam_illuminance,
            exterior_beam_normal_illuminance: &o.environment.exterior_beam_normal_illuminance,
            diffuse_solar_luminous_efficacy: &o.environment.diffuse_solar_luminous_efficacy,
            direct_solar_luminous_efficacy: &o.environment.direct_solar_luminous_efficacy,
            daylighting_sky_clearness: &o.environment.daylighting_sky_clearness,
            daylighting_sky_brightness: &o.environment.daylighting_sky_brightness,
        };
        let fields = LiveReadFields {
            nodes: &*o.equipment,
            gains: &*o.gains,
            weather_outputs: Some(&weather),
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
            pollution: Some(&manager.ancillary.pollution),
            system_reports: &manager.ancillary.system_reports,
        };
        let report = SourceAirReportOutputOwners {
            controller: o.controller,
        };
        let resilience = SourceResilienceOutputOwners { zones: o.zones };
        let air = SourceAirOutputOwners {
            air: o.air,
            demand: &o.controller.demand,
            fan: o.fan,
            report: &report,
            resilience: &resilience,
        };
        let hvac = SourceHvacOutputOwners {
            manager,
            control: o.control,
        };
        let air_loop = SourceAirLoopAndHvacIntegerOwners {
            air_loop: &manager.simulation.air_loop,
            hvac: &hvac,
        };
        let surface = SourceInsideIteration(o.inside_surface_iterations);
        let integers = SourceAllIntegerOutputOwners {
            equipment: &*o.equipment,
            weather: &weather,
            air: &air,
            surface: &surface,
            hvac: &air_loop,
        };
        fields.with_owners(|real| {
            output.registration.update_source_data_and_report(
                kind,
                &mut SourceOutputUpdateOwners {
                    clock: o.clock,
                    environment: SourceOutputUpdateEnvironment {
                        month: &o.environment.month,
                        day_of_month: &o.environment.day_of_month,
                        day_of_week: &o.environment.day_of_week,
                        holiday_index: &o.environment.holiday_index,
                        dst_indicator: &o.environment.dst_indicator,
                        end_month_flag: &mut o.environment.end_month_flag,
                        end_year_flag: &o.environment.end_year_flag,
                    },
                    hvac: o.hvac,
                    project: o.project,
                    results: output.results,
                    standard_output_record_count: output.standard_output_record_count,
                    standard_meter_record_count: output.standard_meter_record_count,
                    real,
                    integers: &integers,
                },
                output.dictionary_host,
            )
        })
    }
}
