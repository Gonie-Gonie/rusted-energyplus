//! ManageHVAC451-560 actual report order on the same retained reporting owners.
use super::*;
use crate::heat_balance::ordinary_heat_balance_initialization::report_warmup::warmup_environment_stamp;
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::heat_balance::source_output_registration::SourceOutputReportStreams;
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
            children.run(Child::WriteWarmupEnvironmentStamp, self, o, eio)?;
            if !o.globals.doing_sizing {
                children.run(Child::CalculateMoreNodeInfo, self, o, eio)?;
            }
            children.run(Child::UpdateDataAndReportSystem, self, o, eio)?;
            if o.clock.kind_of_sim == 4 || o.clock.kind_of_sim == 5 {
                return Err(Error::Unavailable(
                    "actual optional HVAC sizing simulation manager pointer/log write522",
                ));
            }
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

    /// Native480-517 on the same clock/environment/report flags/physical streams.
    /// HVAC PrintedWarmup guards differ from the genuine Zone stamp writer.
    pub(super) fn write_source_hvac_warmup_environment_stamp(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
        report_flags: &mut SourceSolarReportFlags,
        streams: &mut impl SourceOutputReportStreams,
    ) -> Result<(), Error> {
        if o.clock.begin_day_flag && !o.environment.print_environment_stamp_warmup_printed {
            o.environment.print_environment_stamp_warmup = true;
            o.environment.print_environment_stamp_warmup_printed = true;
        }
        if !o.clock.begin_day_flag {
            o.environment.print_environment_stamp_warmup_printed = false;
        }
        if o.environment.print_environment_stamp_warmup {
            if report_flags.print_end_data_dictionary
                && o.clock.do_output_reporting
                && !self.printed_warmup
            {
                streams.write_eso(b"End of Data Dictionary\n")?;
                streams.write_mtr(b"End of Data Dictionary\n")?;
                report_flags.print_end_data_dictionary = false;
            }
            if o.clock.do_output_reporting && !self.printed_warmup {
                streams
                    .write_eso(warmup_environment_stamp(o.environment, report_flags).as_bytes())?;
                streams
                    .write_mtr(warmup_environment_stamp(o.environment, report_flags).as_bytes())?;
                o.environment.print_environment_stamp_warmup = false;
            }
            self.printed_warmup = true;
        }
        Ok(())
    }
}
