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

// Native DataEnvironment71-163 point getter, distinct from the surface setters.
fn source_outdoor_air_bulb_temperature_at(
    global_bulb: f64,
    height: f64,
    parameters: &crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentParameters,
    headline: &'static str,
    backend: &mut SourceErrorBackend<'_>,
    flags: Option<SourceDiagnosticFlags<'_>>,
) -> Result<f64, Error> {
    let base = global_bulb + parameters.weather_file_temperature_modifier_c;
    let value = if parameters.site_temperature_gradient_k_per_m == 0.0 {
        global_bulb
    } else if height <= 0.0 {
        base
    } else {
        base - parameters.site_temperature_gradient_k_per_m * 6_356_000.0 * height
            / (6_356_000.0 + height)
    };
    if value < -100.0 {
        let flags = flags.ok_or(Error::Unavailable(
            "actual outdoor-air point-temperature live diagnostic owners",
        ))?;
        backend.show_source_severe_error(headline, &flags)?;
        let height = crate::heat_balance::source_round_format::round(height, 0)
            .map_err(|error| Error::Format(format!("outdoor-air point height .0R: {error:?}")))?;
        backend.show_continue_error(&format!("...check heights, this height=[{height}]."))?;
        return match backend
            .show_source_fatal_error("Program terminates due to preceding condition(s).", &flags)
        {
            Ok(never) => match never {},
            Err(error) => Err(error),
        };
    }
    Ok(value)
}

// Native DataEnvironment166-190: zero height precedes the zero exponent branch.
fn source_outdoor_air_wind_speed_at(
    wind: f64,
    height: f64,
    parameters: &crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentParameters,
) -> f64 {
    if height <= 0.0 {
        return 0.0;
    }
    if parameters.site_wind_exponent == 0.0 {
        return wind;
    }
    wind * parameters.weather_file_wind_modifier
        * (height / parameters.site_wind_boundary_layer_height_m)
            .powf(parameters.site_wind_exponent)
}

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

    /// Native GetOutAirNodesInput270-282 registers the authored A1 name on
    /// the same physical NodeOwner used by the later PurchasedAir consumer.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_source_authored_outdoor_air_node_nums(
        &mut self,
        input: &mut SelectedInputProcessor,
        branch: &mut SourceBranchNodeConnections,
        name: &str,
        field_name: &str,
        stream: i32,
        number_nodes: &mut usize,
        node_numbers: &mut [usize],
        errors_found: &mut bool,
        backend: &mut SourceErrorBackend<'_>,
        flags: Option<SourceDiagnosticFlags<'_>>,
    ) -> Result<(), Error> {
        self.nodes
            .get_source_node_nums(
                input,
                backend,
                flags,
                branch,
                name,
                number_nodes,
                node_numbers,
                errors_found,
                SourceNodeFluidType::Air,
                SourceNodeObjectType::OutdoorAirNode,
                "OutdoorAir:Node",
                SourceNodeConnectionType::OutsideAir,
                SourceCompFluidStream::source_positive(stream)?,
                false,
                true,
                field_name,
            )
            .map_err(|_| Error::Unavailable("actual authored OutdoorAir GetNodeNums270"))?;
        Ok(())
    }

    /// Native313: only a physically counted N1 writes Height, after the real
    /// registration/membership checks. Omission retains the Node constructor.
    pub(crate) fn set_source_authored_outdoor_air_node_height(
        &mut self,
        node_number: usize,
        height: f64,
    ) -> Result<(), Error> {
        let index = node_number.checked_sub(1).ok_or(Error::Unavailable(
            "actual authored outdoor-air positive physical Node ordinal",
        ))?;
        self.nodes
            .physical
            .as_mut()
            .and_then(|rows| rows.get_mut(index))
            .ok_or(Error::Unavailable(
                "actual authored outdoor-air Height physical Node",
            ))?
            .height = height;
        Ok(())
    }

    pub(crate) fn set_source_authored_outdoor_air_wet_bulb_schedule(
        &mut self,
        node_number: usize,
        schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    ) -> Result<(), Error> {
        let index = node_number
            .checked_sub(1)
            .ok_or(Error::Unavailable("actual OA wet-bulb Node ordinal"))?;
        self.nodes
            .physical
            .as_mut()
            .and_then(|rows| rows.get_mut(index))
            .ok_or(Error::Unavailable("actual OA wet-bulb physical Node"))?
            .outdoor_air_wet_bulb_schedule = schedule;
        Ok(())
    }

    pub(crate) fn set_source_authored_outdoor_air_wind_speed_schedule(
        &mut self,
        node_number: usize,
        schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    ) -> Result<(), Error> {
        let index = node_number
            .checked_sub(1)
            .ok_or(Error::Unavailable("actual OA wind-speed Node ordinal"))?;
        self.nodes
            .physical
            .as_mut()
            .and_then(|rows| rows.get_mut(index))
            .ok_or(Error::Unavailable("actual OA wind-speed physical Node"))?
            .outdoor_air_wind_speed_schedule = schedule;
        Ok(())
    }

    pub(crate) fn set_source_authored_outdoor_air_wind_direction_schedule(
        &mut self,
        node_number: usize,
        schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    ) -> Result<(), Error> {
        let index = node_number
            .checked_sub(1)
            .ok_or(Error::Unavailable("actual OA wind-direction Node ordinal"))?;
        self.nodes
            .physical
            .as_mut()
            .and_then(|rows| rows.get_mut(index))
            .ok_or(Error::Unavailable("actual OA wind-direction physical Node"))?
            .outdoor_air_wind_direction_schedule = schedule;
        Ok(())
    }

    /// Native post-lookup conditional write; raw presence never fabricates local state.
    pub(crate) fn set_source_authored_outdoor_air_local_if_temperature_schedule(
        &mut self,
        node_number: usize,
    ) -> Result<(), Error> {
        let at = node_number
            .checked_sub(1)
            .ok_or(Error::Unavailable("actual local OA positive Node ordinal"))?;
        let node = self
            .nodes
            .physical
            .as_mut()
            .and_then(|rows| rows.get_mut(at))
            .ok_or(Error::Unavailable("actual local OA physical Node"))?;
        if node.outdoor_air_dry_bulb_schedule.is_some()
            || node.outdoor_air_wet_bulb_schedule.is_some()
        {
            node.is_local_node = true;
        }
        Ok(())
    }

    /// Read the actual pointers for cross-module owner tests without exposing the arena.
    #[cfg(test)]
    pub(crate) fn source_outdoor_air_wind_schedule_pointers(
        &self,
        node_number: i32,
    ) -> Result<
        (
            Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
            Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
        ),
        Error,
    > {
        let at = node_number
            .checked_sub(1)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or(Error::Unavailable(
                "actual test outdoor-air positive Node ordinal",
            ))?;
        let node = self
            .nodes
            .physical
            .as_deref()
            .and_then(|rows| rows.get(at))
            .ok_or(Error::Unavailable("actual test outdoor-air physical Node"))?;
        Ok((
            node.outdoor_air_wind_speed_schedule,
            node.outdoor_air_wind_direction_schedule,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn set_source_outdoor_air_node_values(
        &mut self,
        node_number: i32,
        init_call: bool,
        control: &SelectedHeatBalanceInputState,
        schedules: &crate::heat_balance::selected_schedule_manager::SelectedScheduleManager,
        o: &mut crate::heat_balance::ordinary_hvac_manager::SourceOutdoorAirNodeRuntimeOwners<
            '_,
            '_,
        >,
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
            node.outdoor_air_dry_bulb = o.environment.current_weather.out_dry_bulb_temp;
            node.outdoor_air_wet_bulb = o.environment.current_weather.out_wet_bulb_temp;
            if init_call {
                node.outdoor_air_wind_speed = o.environment.current_weather.wind_speed;
            }
        } else {
            node.outdoor_air_dry_bulb = source_outdoor_air_bulb_temperature_at(
                o.environment.current_weather.out_dry_bulb_temp,
                node.height,
                &o.environment.outdoor_environment_parameters,
                "OutDryBulbTempAt: outdoor drybulb temperature < -100 C",
                o.backend,
                Some(crate::heat_balance::error_backend::SourceDiagnosticFlags {
                    kickoff: &o.globals.kickoff_simulation,
                    developer: o.developer,
                    warmup: &o.clock.warmup_flag,
                    doing_sizing: &o.globals.doing_sizing,
                }),
            )?;
            node.outdoor_air_wet_bulb = source_outdoor_air_bulb_temperature_at(
                o.environment.current_weather.out_wet_bulb_temp,
                node.height,
                &o.environment.outdoor_environment_parameters,
                "OutWetBulbTempAt: outdoor wetbulb temperature < -100 C",
                o.backend,
                Some(crate::heat_balance::error_backend::SourceDiagnosticFlags {
                    kickoff: &o.globals.kickoff_simulation,
                    developer: o.developer,
                    warmup: &o.clock.warmup_flag,
                    doing_sizing: &o.globals.doing_sizing,
                }),
            )?;
            if init_call {
                node.outdoor_air_wind_speed = source_outdoor_air_wind_speed_at(
                    o.environment.current_weather.wind_speed,
                    node.height,
                    &o.environment.outdoor_environment_parameters,
                );
            }
        }
        if !init_call {
            node.outdoor_air_wind_speed = o.environment.current_weather.wind_speed;
        }
        node.outdoor_air_wind_direction = o.environment.current_weather.wind_dir;
        if init_call {
            if node.outdoor_air_dry_bulb_schedule.is_some() {
                return Err(Error::Unavailable("actual local OA dry-bulb schedule559"));
            }
            if let Some(schedule) = node.outdoor_air_wet_bulb_schedule {
                node.outdoor_air_wet_bulb = schedules.source_current_schedule_value(schedule)?;
            }
            if let Some(schedule) = node.outdoor_air_wind_speed_schedule {
                node.outdoor_air_wind_speed = schedules.source_current_schedule_value(schedule)?;
            }
            if let Some(schedule) = node.outdoor_air_wind_direction_schedule {
                node.outdoor_air_wind_direction =
                    schedules.source_current_schedule_value(schedule)?;
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
            if init_call
                && node.outdoor_air_wet_bulb_schedule.is_none()
                && !node.ems_override_outdoor_air_wet_bulb
                && (node.ems_override_outdoor_air_dry_bulb
                    || node.outdoor_air_dry_bulb_schedule.is_some())
            {
                node.humidity_ratio = o.environment.current_weather.out_hum_rat;
                return Err(Error::Unavailable(
                    "actual local OA DB-only cached Twb property owner598",
                ));
            }
            node.humidity_ratio = o.psychrometrics.source_humidity_ratio_from_wet_bulb(
                node.outdoor_air_dry_bulb,
                node.outdoor_air_wet_bulb,
                o.environment.current_weather.out_baro_press,
                "",
                &mut crate::psychrometrics::SourcePsychrometricOwners {
                    clock: o.clock,
                    globals: &*o.globals,
                    environment: o.environment,
                    hvac: o.hvac,
                    recurring: &mut *o.recurring,
                    backend: &mut *o.backend,
                },
            )?;
        } else {
            node.humidity_ratio = o.environment.current_weather.out_hum_rat;
        }
        node.enthalpy = crate::psychrometrics::energyplus_psy_h_fn_tdb_w(
            node.outdoor_air_dry_bulb,
            node.humidity_ratio,
        );
        node.pressure = o.environment.current_weather.out_baro_press;
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
