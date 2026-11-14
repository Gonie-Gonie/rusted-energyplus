//! Genuine OutAirNode CheckAndAdd registration and global-weather Node writes.
use super::{OrdinaryEquipment, source_nodes::SourceNodeFluidType};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::startup_integrity::{
    SourceBranchNodeConnections, SourceCompFluidStream, SourceNodeConnectionType,
    SourceNodeObjectType,
};
use crate::weather::day::WeatherEnvironmentState;

impl OrdinaryEquipment {
    #[allow(
        dead_code,
        reason = "Called by the genuine CheckAndAdd owner prerequisite, ahead of the separate positive-OA getter port."
    )]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn register_source_outdoor_air_node(
        &mut self,
        input: &mut SelectedInputProcessor,
        branch: &mut SourceBranchNodeConnections,
        node_number: i32,
        count: usize,
        temporary_numbers: &mut [usize],
        backend: &mut SourceErrorBackend<'_>,
        flags: Option<SourceDiagnosticFlags<'_>>,
    ) -> Result<(), Error> {
        let ordinal = usize::try_from(node_number)
            .map_err(|_| Error::Unavailable("actual CheckAndAdd NodeID ordinal"))?;
        let name = self.source_node_id(ordinal)?.to_owned();
        let mut dummy_number = 0;
        let mut errors_found = false;
        let stream = SourceCompFluidStream::source_positive(
            i32::try_from(count)
                .map_err(|_| Error::Unavailable("actual CheckAndAdd fluid-stream int cast"))?,
        )?;
        self.nodes
            .get_source_node_nums(
                input,
                backend,
                flags,
                branch,
                &name,
                &mut dummy_number,
                temporary_numbers,
                &mut errors_found,
                SourceNodeFluidType::Air,
                SourceNodeObjectType::OutdoorAirNode,
                "OutdoorAir:Node",
                SourceNodeConnectionType::OutsideAir,
                stream,
                false,
                true,
                "",
            )
            .map_err(|_| Error::Unavailable("actual CheckAndAdd GetNodeNums registration511"))?;
        Ok(())
    }

    pub(crate) fn set_source_outdoor_air_node_values(
        &mut self,
        node_number: i32,
        init_call: bool,
        environment: &WeatherEnvironmentState,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        let ordinal = node_number
            .checked_sub(1)
            .and_then(|n| usize::try_from(n).ok())
            .ok_or(Error::Unavailable(
                "actual SetOANodeValues positive Node ordinal",
            ))?;
        let node = self
            .nodes
            .physical
            .as_mut()
            .and_then(|v| v.get_mut(ordinal))
            .ok_or(Error::Unavailable("actual SetOANodeValues physical Node"))?;
        if node.height < 0.0 {
            node.outdoor_air_dry_bulb = environment.current_weather.out_dry_bulb_temp;
            node.outdoor_air_wet_bulb = environment.current_weather.out_wet_bulb_temp;
            if init_call {
                node.outdoor_air_wind_speed = environment.current_weather.wind_speed;
            }
        } else {
            return Err(Error::Unavailable(
                "actual positive OutDryBulbTempAt height548",
            ));
        }
        if !init_call {
            node.outdoor_air_wind_speed = environment.current_weather.wind_speed;
        }
        node.outdoor_air_wind_direction = environment.current_weather.wind_dir;
        if init_call {
            if node.outdoor_air_dry_bulb_schedule.is_some() {
                return Err(Error::Unavailable("actual local OA dry-bulb schedule559"));
            }
            if node.outdoor_air_wet_bulb_schedule.is_some() {
                return Err(Error::Unavailable("actual local OA wet-bulb schedule562"));
            }
            if node.outdoor_air_wind_speed_schedule.is_some() {
                return Err(Error::Unavailable("actual local OA wind-speed schedule565"));
            }
            if node.outdoor_air_wind_direction_schedule.is_some() {
                return Err(Error::Unavailable(
                    "actual local OA wind-direction schedule568",
                ));
            }
            if node.ems_override_outdoor_air_dry_bulb {
                return Err(Error::Unavailable("actual local OA EMS dry-bulb574"));
            }
            if node.ems_override_outdoor_air_wet_bulb {
                return Err(Error::Unavailable("actual local OA EMS wet-bulb577"));
            }
            if node.ems_override_outdoor_air_wind_speed {
                return Err(Error::Unavailable("actual local OA EMS wind-speed580"));
            }
            if node.ems_override_outdoor_air_wind_direction {
                return Err(Error::Unavailable("actual local OA EMS wind-direction583"));
            }
        }
        node.temperature = node.outdoor_air_dry_bulb;
        if node.is_local_node {
            if init_call && node.outdoor_air_wet_bulb > node.outdoor_air_dry_bulb {
                node.outdoor_air_wet_bulb = node.outdoor_air_dry_bulb;
            }
            return Err(Error::Unavailable(
                "actual local OA Twb/W property owner592",
            ));
        } else {
            node.humidity_ratio = environment.current_weather.out_hum_rat;
        }
        node.enthalpy = crate::psychrometrics::energyplus_psy_h_fn_tdb_w(
            node.outdoor_air_dry_bulb,
            node.humidity_ratio,
        );
        node.pressure = environment.current_weather.out_baro_press;
        node.quality = 0.0;
        if control.carbon_dioxide_simulation {
            return Err(Error::Unavailable(
                "actual positive outdoor CO2 scalar owner621",
            ));
        }
        if control.generic_contaminant_simulation {
            return Err(Error::Unavailable(
                "actual positive outdoor generic-contaminant scalar625",
            ));
        }
        Ok(())
    }
}
