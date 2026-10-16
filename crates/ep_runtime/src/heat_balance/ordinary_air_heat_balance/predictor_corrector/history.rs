//! Actual Source air history shifts. Surface CTF histories are a distinct owner.
use super::*;
impl SourceZoneSpaceHeatBalanceData {
    fn push_source_zone_history(
        &mut self,
        number: i32,
        space: bool,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        let model = controller.subsystems.room_air.model(number)?; //4193 before any shifts
        for i in (1..=3).rev() {
            self.xmat[i] = self.xmat[i - 1];
            self.w_prev_zone_ts[i] = self.w_prev_zone_ts[i - 1];
        }
        self.xmat[0] = self.ztav;
        self.xmpt = self.zt;
        self.w_prev_zone_ts[0] = self.air_hum_rat_avg;
        self.air_hum_rat = self.air_hum_rat_temp;
        self.w_time_minus_p = self.air_hum_rat_temp;
        self.air_rel_hum = 100.0
            * relative_humidity(
                owners.psychrometrics,
                self.zt,
                self.air_hum_rat,
                owners.environment.current_weather.out_baro_press,
            )?;
        if !space && matches!(model, 3 | 5 | 6) {
            return Err(Error::Unavailable("positive nonmixing zone history4212"));
        }
        if !space && model == 7 {
            return Err(Error::Unavailable("positive RoomAFN zone history4233"));
        }
        if owners.control.zone_air_solution != SelectedZoneAirSolution::ThirdOrder {
            return Err(Error::Unavailable("non-ThirdOrder history TM2/TMX4248"));
        }
        Ok(())
    }
    fn push_source_system_history(
        &mut self,
        number: i32,
        space: bool,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        for i in (1..=3).rev() {
            self.dsxmat[i] = self.dsxmat[i - 1];
            self.dsw_prev_zone_ts[i] = self.dsw_prev_zone_ts[i - 1];
        }
        self.dsxmat[0] = self.mat;
        self.dsw_prev_zone_ts[0] = self.air_hum_rat;
        if !space && controller.subsystems.room_air.any_non_mixing {
            return Err(Error::Unavailable("positive nonmixing system history4309"));
        }
        if owners.control.zone_air_solution != SelectedZoneAirSolution::ThirdOrder {
            return Err(Error::Unavailable("non-ThirdOrder system histories4341"));
        }
        let _ = number; // No Source RoomAir row is read on mixed/Space branch4308.
        Ok(())
    }
    fn revert_source_zone_history(
        &mut self,
        number: i32,
        space: bool,
        controller: &SourceZoneAirPredictorCorrector,
    ) -> Result<(), Error> {
        for i in 0..=2 {
            self.xmat[i] = self.xmat[i + 1];
            self.w_prev_zone_ts[i] = self.w_prev_zone_ts[i + 1];
        }
        if !space {
            let model = controller.subsystems.room_air.model(number)?;
            if matches!(model, 3 | 5 | 6) {
                return Err(Error::Unavailable("positive nonmixing history revert4406"));
            }
            if model == 7 {
                return Err(Error::Unavailable("positive RoomAFN history revert4420"));
            }
        }
        Ok(())
    }
}
impl SourceAirHeatBalance {
    pub(crate) fn push_source_zone_timestep_histories(
        &mut self,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        for number in 1..=owners.globals.num_of_zones {
            self.zones
                .as_mut()
                .and_then(|v| v.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual Zone push history4178"))?
                .air
                .push_source_zone_history(number, false, controller, owners)?;
            if owners.space_controls.current {
                let spaces = zone(owners, number)?.space_indexes.clone(); // Source ordinal metadata only
                for space in spaces {
                    self.spaces
                        .as_mut()
                        .and_then(|v| v.get_mut(index(space).ok()?))
                        .ok_or(Error::Unavailable("actual Space push history4181"))?
                        .air
                        .push_source_zone_history(number, true, controller, owners)?;
                }
            }
        }
        Ok(())
    }
    pub(crate) fn push_source_system_timestep_histories(
        &mut self,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        for number in 1..=owners.globals.num_of_zones {
            self.zones
                .as_mut()
                .and_then(|v| v.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual Zone system history4288"))?
                .air
                .push_source_system_history(number, false, controller, owners)?;
            if owners.space_controls.current {
                for &space in &zone(owners, number)?.space_indexes {
                    self.spaces
                        .as_mut()
                        .and_then(|v| v.get_mut(index(space).ok()?))
                        .ok_or(Error::Unavailable("actual Space system history4291"))?
                        .air
                        .push_source_system_history(number, true, controller, owners)?;
                }
            }
        }
        Ok(())
    }
    pub(crate) fn revert_source_zone_timestep_histories(
        &mut self,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        for number in 1..=owners.globals.num_of_zones {
            self.zones
                .as_mut()
                .and_then(|v| v.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual Zone revert4382"))?
                .air
                .revert_source_zone_history(number, false, controller)?;
            if owners.space_controls.current {
                for &space in &zone(owners, number)?.space_indexes {
                    self.spaces
                        .as_mut()
                        .and_then(|v| v.get_mut(index(space).ok()?))
                        .ok_or(Error::Unavailable("actual Space revert4385"))?
                        .air
                        .revert_source_zone_history(number, true, controller)?;
                }
            }
        }
        Ok(())
    }
}
