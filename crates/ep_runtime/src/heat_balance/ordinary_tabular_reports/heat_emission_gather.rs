//! Genuine GatherHeatEmissionReport4227-4258; second actual System Calc call.
use super::*;
use crate::heat_balance::ordinary_hvac_manager::heat_emission::SourceHeatEmissionOwners;
impl SourceTabularReports {
    pub(super) fn gather_source_heat_emission_report(
        &mut self,
        time_step: TimeStepKind,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        // These two writes precede the report predicate in Native4227.
        o.control.sys_total_hvac_relief_heat_loss = 0.0;
        o.control.sys_total_hvac_reject_heat_loss = 0.0;
        if !self.display_heat_emissions_summary {
            return Ok(());
        }
        if time_step == TimeStepKind::Zone {
            o.zones.building_predefined.emi_envelop_conv += *o.surface_heat_emission * 1.0E-9;
            return Ok(());
        }
        // Native4248 executes Calc again on the same real losses, even after
        // the earlier HVAC442 call. Do not reuse its earlier numeric result.
        o.heat_emission
            .calculate_source_heat_emission_report(&mut SourceHeatEmissionOwners {
                hvac: o.hvac,
                environment: o.environment,
                control: o.control,
                equipment: o.equipment,
                gain_modules: o.gain_modules,
            })?;
        o.zones.building_predefined.emi_zone_exfiltration +=
            o.controller.simple_air.zone_total_exfiltration_heat_loss * 1.0E-9;
        o.zones.building_predefined.emi_zone_exhaust +=
            o.controller.simple_air.zone_total_exhaust_heat_loss * 1.0E-9;
        o.zones.building_predefined.emi_hvac_relief +=
            o.control.sys_total_hvac_relief_heat_loss * 1.0E-9;
        o.zones.building_predefined.emi_hvac_reject +=
            o.control.sys_total_hvac_reject_heat_loss * 1.0E-9;
        o.zones.building_predefined.emi_tot_heat = o.zones.building_predefined.emi_envelop_conv
            + o.zones.building_predefined.emi_zone_exfiltration
            + o.zones.building_predefined.emi_zone_exhaust
            + o.zones.building_predefined.emi_hvac_relief
            + o.zones.building_predefined.emi_hvac_reject;
        Ok(())
    }
}
