//! ManageHVAC451-560 actual report order, with unresolved positive inline subtrees explicit.
use super::*;
impl SourceHvacManager {
    pub(super) fn report_source_system_step(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
        children: &mut impl SourceHvacChildren,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        use SourceHvacChild as Child;
        if !o.clock.warmup_flag {
            if o.clock.do_output_reporting && !o.globals.zone_sizing_calculation {
                children.run(Child::CalculateMoreNodeInfo, self, o, eio)?;
                children.run(Child::CalculatePollution, self, o, eio)?;
                children.run(Child::InitializeEnergyReports, self, o, eio)?;
                children.run(Child::ReportSystemEnergyUse, self, o, eio)?;
            }
            if o.clock.do_output_reporting
                || (o.globals.zone_sizing_calculation && o.globals.component_load_report_requested)
            {
                children.run(Child::ReportAirHeatBalance, self, o, eio)?;
                if o.globals.zone_sizing_calculation {
                    children.run(Child::GatherComponentLoadsHvac, self, o, eio)?;
                }
            }
            if o.clock.do_output_reporting {
                children.run(Child::ReportVentilationLoads, self, o, eio)?;
                children.run(Child::UpdateDataAndReportSystem, self, o, eio)?;
                if o.clock.kind_of_sim == 4 || o.clock.kind_of_sim == 5 {
                    return Err(Error::Unavailable(
                        "actual optional HVAC sizing simulation manager pointer/log write473",
                    ));
                }
                children.run(Child::UpdateTabularReportsSystem, self, o, eio)?;
            }
            if o.globals.zone_sizing_calculation {
                children.run(Child::UpdateZoneSizingDuringDay, self, o, eio)?;
                children.run(Child::UpdateFacilitySizingDuringDay, self, o, eio)?;
            }
            children.run(
                Child::CheckConcurrentEirPlantLoopHeatPumpOperation,
                self,
                o,
                eio,
            )?;
        } else if !o.globals.kickoff_simulation
            && o.clock.do_output_reporting
            && o.project.system.report_during_warmup
        {
            return Err(Error::Unavailable(
                "actual inline HVAC warmup environment-stamp/report subtree488",
            ));
        } else if o
            .project
            .system
            .update_data_during_warmup_external_interface
        {
            return Err(Error::Unavailable(
                "actual inline HVAC external-interface warmup stamp/report subtree533",
            ));
        }
        Ok(())
    }
}
