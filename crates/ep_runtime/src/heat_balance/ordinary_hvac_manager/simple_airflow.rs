//! CalcAirFlowSimple5570-6910 genuine selected zero-object route.
use super::*;
use crate::heat_balance::ordinary_air_heat_balance::SourceZoneSpaceHeatBalanceData;
mod state;
mod systems;
pub(super) use state::SourceSimpleAirflowSystems;
fn reset_airflow(row: &mut SourceZoneSpaceHeatBalanceData) {
    row.mcpm = 0.0;
    row.mcptm = 0.0;
    row.mcpti = 0.0;
    row.mcpi = 0.0;
    row.oamfl = 0.0;
    row.mcptv = 0.0;
    row.mcpv = 0.0;
    row.vamfl = 0.0;
    row.m_dot_cpoa = 0.0;
    row.m_dot_oa = 0.0;
    row.mcp_therm_chim = 0.0;
    row.therm_chim_amfl = 0.0;
    row.mcpt_therm_chim = 0.0;
    row.mixing_mass_flow_zone = 0.0;
    row.mixing_mass_flow_x_hum_rat = 0.0;
}
fn mixing_air(row: &mut SourceZoneSpaceHeatBalanceData, system_step: i32) {
    row.mixing_mat = row.mat;
    row.mixing_hum_rat = row.air_hum_rat;
    if system_step == 1 {
        row.mixing_mat = row.xmpt;
        row.mixing_hum_rat = row.w_time_minus_p;
    }
}
fn add_thermal_chimney(row: &mut SourceZoneSpaceHeatBalanceData) {
    row.mcpi += row.mcp_therm_chim;
    row.oamfl += row.therm_chim_amfl;
    row.mcpti += row.mcpt_therm_chim;
}
impl SourceHvacManager {
    /// Native optional SysTimestepLoop default0; both adjustment arguments are
    /// actually absent at the two ManageHVAC calls and use their Source false defaults.
    // Retain the explicit Original absent-system-step branch; no default owner substitute.
    #[allow(clippy::manual_unwrap_or_default, clippy::manual_unwrap_or)]
    pub(crate) fn calculate_source_simple_airflow(
        &mut self,
        system_step: Option<i32>,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        let system_step = match system_step {
            Some(value) => value,
            None => 0,
        };
        for row in o.air.zones.iter_mut().flatten() {
            reset_airflow(&mut row.air);
        }
        if o.space_controls.current {
            for row in o.air.spaces.iter_mut().flatten() {
                reset_airflow(&mut row.air);
            }
        }
        if o.control.carbon_dioxide_simulation {
            let simple = &o.controller.simple_air;
            if simple.total_mixing
                + simple.total_cross_mixing
                + simple.total_refrigeration_door_mixing
                > 0
            {
                return Err(Error::Unavailable(
                    "actual MixingMassFlowCO2 whole reset5636",
                ));
            }
        }
        if o.control.generic_contaminant_simulation {
            let simple = &o.controller.simple_air;
            if simple.total_mixing
                + simple.total_cross_mixing
                + simple.total_refrigeration_door_mixing
                > 0
            {
                return Err(Error::Unavailable(
                    "actual MixingMassFlowGC whole reset5640",
                ));
            }
        }
        if !o.controller.simple_air.air_flow_flag {
            return Ok(());
        }
        let afn = o.controller.subsystems.airflow_network.control_type;
        if !(afn == 0 || afn == 2) {
            return Ok(());
        }
        let systems = &mut self.ancillary.flow_systems;
        systems.earth_tube.manage(o)?;
        systems.cool_tower.manage(o)?;
        systems.thermal_chimney.manage(o)?;
        for row in o.air.zones.iter_mut().flatten() {
            mixing_air(&mut row.air, system_step);
        }
        if o.space_controls.current {
            for row in o.air.spaces.iter_mut().flatten() {
                mixing_air(&mut row.air, system_step);
            }
        }
        if o.controller.simple_air.total_zone_air_balance > 0 {
            for row in o
                .controller
                .simple_air
                .zone_air_balance
                .iter_mut()
                .flatten()
            {
                row.balanced_mass_flow_rate = 0.0;
                row.infiltration_mass_flow_rate = 0.0;
                row.natural_mass_flow_rate = 0.0;
                row.exhaust_mass_flow_rate = 0.0;
                row.intake_mass_flow_rate = 0.0;
                row.erv_mass_flow_rate = 0.0;
            }
        }
        if o.controller.simple_air.total_ventilation > 0 {
            for row in o.controller.reports.zones.iter_mut().flatten() {
                row.ventil_fan_elec = 0.0;
            }
        }
        if o.controller.simple_air.total_ventilation > 0 {
            return Err(Error::Unavailable(
                "actual first Ventilation row5698 before positive physics",
            ));
        }
        if o.controller.simple_air.total_mixing > 0 {
            return Err(Error::Unavailable(
                "actual first Mixing row6052 before positive physics",
            ));
        }
        if o.controller.simple_air.total_cross_mixing > 0 {
            return Err(Error::Unavailable(
                "actual first CrossMixing row6334 before positive physics",
            ));
        }
        if o.controller.simple_air.total_refrigeration_door_mixing > 0 {
            return Err(Error::Unavailable(
                "actual positive RefDoorMixing Zone pair branch6573",
            ));
        }
        if o.controller.simple_air.total_infiltration > 0 {
            return Err(Error::Unavailable(
                "actual first Infiltration row6713 before positive physics",
            ));
        }
        for row in o.air.zones.iter_mut().flatten() {
            add_thermal_chimney(&mut row.air);
        }
        if o.space_controls.current {
            for row in o.air.spaces.iter_mut().flatten() {
                add_thermal_chimney(&mut row.air);
            }
        }
        for row in o.controller.simple_air.zone_air_balance.iter().flatten() {
            if row.balance_method == 1 {
                // Actual DataHeatBalance::AirBalance::Quadrature.
                return Err(Error::Unavailable(
                    "actual positive Quadrature first OneTimeFlag read6876",
                ));
            }
        }
        Ok(())
    }
}
