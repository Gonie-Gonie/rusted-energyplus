//! Genuine InitZoneAirSetPoints2350-2675 first allocation/output child.
use super::*;
fn extent(number: i32) -> Result<usize, Error> {
    usize::try_from(number).map_err(|_| Error::Unavailable("actual Source air allocation extent"))
}
impl SourceZoneAirPredictorCorrector {
    pub(crate) fn initialize_source_air_setpoints(
        &mut self,
        air: &mut SourceAirHeatBalance,
        owners: &mut SourceZoneAirOwners<'_, '_>,
        outputs: &mut impl SourceZoneAirRegistrationChildren,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        if self.flags.initialize_setpoints_once {
            let n = extent(owners.globals.num_of_zones)?;
            let fan = &mut owners.fan.thermostat;
            // Array1D::allocate retains an equal-size class array in the
            // original non-debug build; changed extents reconstruct its rows.
            if fan.zone_tstat_setpoints.as_ref().map(Vec::len) != Some(n) {
                fan.zone_tstat_setpoints =
                    Some(vec![controls::SourceZoneTstatSetpoint::default(); n]);
            }
            fan.load_correction_factor = Some(vec![0.0; n]);
            fan.temp_control_type = Some(vec![0; n]);
            fan.temp_control_type_report = Some(vec![0; n]);
            if self.controls.number_comfort_zones > 0 {
                return Err(Error::Unavailable(
                    "positive ComfortControlType dimension2388",
                ));
            }
            self.demand.setback = Some(vec![false; n]);
            self.demand.deadband_or_setback = Some(vec![false; n]);
            self.demand.current_deadband_or_setback = Some(vec![false; n]);
            let lists = extent(owners.zones.num_zone_lists)?;
            let groups = extent(owners.zones.num_zone_groups)?;
            let reports = &mut owners.control.zone_air_group_loads;
            reports.list_heat_energy = Some(vec![0.0; lists]);
            reports.list_cool_energy = Some(vec![0.0; lists]);
            reports.list_heat_rate = Some(vec![0.0; lists]);
            reports.list_cool_rate = Some(vec![0.0; lists]);
            reports.group_heat_energy = Some(vec![0.0; groups]);
            reports.group_cool_energy = Some(vec![0.0; groups]);
            reports.group_heat_rate = Some(vec![0.0; groups]);
            reports.group_cool_rate = Some(vec![0.0; groups]);
            fan.previous_measured_temperature_1 = Some(vec![0.0; n]);
            fan.previous_measured_temperature_2 = Some(vec![0.0; n]);
            fan.previous_measured_temperature_3 = Some(vec![0.0; n]);
            fan.previous_measured_humidity_1 = Some(vec![0.0; n]);
            fan.previous_measured_humidity_2 = Some(vec![0.0; n]);
            fan.previous_measured_humidity_3 = Some(vec![0.0; n]);
            self.demand
                .allocate_source_demands(owners.globals, owners.space_controls)?;
            for number in 1..=owners.globals.num_of_zones {
                let mut first = true;
                let mut reference = None;
                let spaces = zone(owners, number)?.space_indexes.clone(); // Source ordinal metadata only
                for space in spaces {
                    let row = owners
                        .zones
                        .spaces
                        .as_ref()
                        .and_then(|rows| rows.get(index(space).ok()?))
                        .ok_or(Error::Unavailable("actual reference-air Space2423"))?;
                    for surface in row.ht_surface_first..=row.ht_surface_last {
                        let actual = *owners
                            .geometry
                            .parallel_arrays
                            .as_ref()
                            .and_then(|a| a.surf_t_air_ref.get(index(surface).ok()?))
                            .ok_or(Error::Unavailable("actual reference-air Surface2428"))?;
                        if first {
                            reference = Some(actual);
                            first = false;
                        }
                        if Some(actual) != reference {
                            let message = format!(
                                "Different reference air temperatures for difference surfaces encountered in zone {}",
                                zone(owners, number)?.name
                            );
                            owners.backend.show_warning_error(&message)?;
                        }
                    }
                }
            }
            for number in 1..=owners.globals.num_of_zones {
                let z = owners
                    .zones
                    .zones
                    .as_ref()
                    .and_then(|rows| rows.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("same actual live Zone output owner"))?;
                outputs.register_air(
                    number,
                    false,
                    &z.name,
                    &air.zones
                        .as_ref()
                        .and_then(|rows| rows.get(index(number).ok()?))
                        .ok_or(Error::Unavailable("actual Zone output2446"))?
                        .air,
                    owners.clock,
                    owners.reporting_diagnostic_flags,
                    owners.input,
                    owners.backend,
                    eio,
                )?;
                if owners.space_controls.simulation {
                    return Err(Error::Unavailable("positive Space Air output2447"));
                }
                let staged = if let Some(rows) = &self.controls.stage_zone_logic {
                    *rows
                        .get(index(number)?)
                        .ok_or(Error::Unavailable("actual StageZoneLogic2458"))?
                } else {
                    false
                };
                let attach_meters = !owners.space_controls.simulation;
                outputs.register_sensible_demand(
                    number,
                    false,
                    &z.name,
                    staged,
                    attach_meters,
                    z.multiplier,
                    z.list_multiplier,
                    self.demand
                        .zones
                        .as_ref()
                        .and_then(|rows| rows.get(index(number).ok()?))
                        .ok_or(Error::Unavailable("actual Zone sensible output2462"))?,
                    owners.clock,
                    owners.reporting_diagnostic_flags,
                    owners.input,
                    owners.backend,
                    eio,
                )?;
                if owners.space_controls.simulation {
                    return Err(Error::Unavailable("positive Space sensible output2464"));
                }
                outputs.register_moisture_demand(
                    number,
                    false,
                    &z.name,
                    owners.control.do_latent_sizing,
                    self.demand
                        .zone_moisture
                        .as_ref()
                        .and_then(|rows| rows.get(index(number).ok()?))
                        .ok_or(Error::Unavailable("actual Zone moisture output2477"))?,
                    owners.clock,
                    owners.reporting_diagnostic_flags,
                    owners.input,
                    owners.backend,
                    eio,
                )?;
                if owners.space_controls.simulation {
                    return Err(Error::Unavailable("positive Space moisture output2478"));
                }
                outputs.register_thermostat(
                    number,
                    &z.name,
                    owners.fan,
                    owners.clock,
                    owners.reporting_diagnostic_flags,
                    owners.input,
                    owners.backend,
                    eio,
                )?;
            }
            if self.controls.number_comfort_zones > 0 {
                return Err(Error::Unavailable("positive comfort-control outputs2530"));
            }
            for number in 1..=owners.zones.num_zone_lists {
                let list = owners
                    .zones
                    .zone_lists
                    .as_deref()
                    .and_then(|rows| index(number).ok().and_then(|i| rows.get(i)))
                    .ok_or(Error::Unavailable("actual ZoneList2554 name/output"))?;
                outputs.register_zone_list(
                    number,
                    &list.name,
                    &owners.control.zone_air_group_loads,
                    owners.clock,
                    owners.reporting_diagnostic_flags,
                    owners.input,
                    owners.backend,
                    eio,
                )?;
            }
            for number in 1..=owners.zones.num_zone_groups {
                let group = owners
                    .zones
                    .zone_groups
                    .as_deref()
                    .and_then(|rows| index(number).ok().and_then(|at| rows.get(at)))
                    .ok_or(Error::Unavailable("actual ZoneGroup2597 name/output"))?;
                outputs.register_zone_group(
                    number,
                    &group.name,
                    &owners.control.zone_air_group_loads,
                    owners.clock,
                    owners.reporting_diagnostic_flags,
                    owners.input,
                    owners.backend,
                    eio,
                )?;
            }
            self.flags.initialize_setpoints_once = false;
        }
        self.initialize_source_air_environment(air, owners)
    }
}
