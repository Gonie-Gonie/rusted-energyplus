//! Actual CloseOutputFiles1764-1955 on the one retained physical file owner.
use super::{SourceInitializationFiles, SourceOutputStream};
use crate::heat_balance::ordinary_output_control::SourceOutputControl;
use crate::heat_balance::ordinary_tabular_reports::SourceTabularReports;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::radiation::ordinary::shading_report::SourceShadingStream;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::selected_project_data::ProjectSystemVariables;
use crate::heat_balance::selected_simulation_startup::SelectedRuntimeLanguage;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::SourceOutputRegistration;
use crate::heat_balance::startup_integrity::SourceBranchNodeConnections;
use std::io::{self, Write};

/// Each member is borrowed from the same actual session/reports. Record counts
/// are the i64 cells incremented by the real OP writers, not parsed artifacts.
pub(crate) struct SourceCloseOutputFilesOwners<'a> {
    pub(crate) registration: &'a SourceOutputRegistration,
    pub(crate) tabular: &'a SourceTabularReports,
    pub(crate) solar: &'a OrdinarySolarRadiation,
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) system: &'a ProjectSystemVariables,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) runtime_language: &'a SelectedRuntimeLanguage,
    pub(crate) connections: &'a SourceBranchNodeConnections,
    pub(crate) heat_balance: &'a SelectedHeatBalanceInputState,
    pub(crate) standard_output_record_count: &'a i64,
    pub(crate) standard_meter_record_count: &'a i64,
    pub(crate) shading_stream: &'a mut SourceShadingStream,
    pub(crate) output_control: &'a SourceOutputControl,
}

impl SourceOutputStream {
    pub(super) fn close_source_output(&mut self) {
        // Actual InputOutputFile.close234-237 resets os, not its path.
        self.file = None;
    }
    pub(super) fn delete_source_output(&mut self) -> io::Result<()> {
        // Actual del239-245 acts only when the os pointer exists, closes it
        // before removeFile, and leaves this same retained pathname in place.
        if self.file.take().is_some() {
            match std::fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(e),
            }
            self.deleted_empty = true;
        }
        Ok(())
    }
}

impl SourceInitializationFiles {
    pub(crate) fn close_source_output_files(
        &mut self,
        o: &mut SourceCloseOutputFilesOwners<'_>,
    ) -> Result<(), Error> {
        let audit = self.ensure_source_reporting_audit(o.output_control)?;
        writeln!(
            audit,
            " NumOfRVariable={:12}",
            o.registration.num_real_variables_setup
        )?;
        writeln!(
            audit,
            " NumOfRVariable(Total)={:12}",
            o.registration.num_total_real_variables
        )?;
        writeln!(
            audit,
            " NumOfRVariable(Actual)={:12}",
            o.registration.num_real_variables
        )?;
        writeln!(
            audit,
            " NumOfRVariable(Summed)={:12}",
            o.registration.num_real_variables_sum
        )?;
        writeln!(
            audit,
            " NumOfRVariable(Meter)={:12}",
            o.registration.num_real_variables_meter
        )?;
        writeln!(
            audit,
            " NumOfIVariable={:12}",
            o.registration.num_integer_variables_setup
        )?;
        writeln!(
            audit,
            " NumOfIVariable(Total)={:12}",
            o.registration.num_total_integer_variables
        )?;
        writeln!(
            audit,
            " NumOfIVariable(Actual)={:12}",
            o.registration.num_integer_variables
        )?;
        writeln!(
            audit,
            " NumOfIVariable(Summed)={:12}",
            o.registration.num_integer_variables_sum
        )?;
        writeln!(
            audit,
            " NumEnergyMeters={:12}",
            o.registration.meters.meters.len()
        )?;
        writeln!(
            audit,
            " maxUniqueKeyCount={:12}",
            o.tabular.source_max_unique_key_count()
        )?;
        writeln!(
            audit,
            " maxNumberOfFigures={:12}",
            o.solar.solar_timestep.max_number_of_figures
        )?;
        writeln!(
            audit,
            " MAXHCArrayBounds={:12}",
            o.solar.solar_timestep.max_hc_array_bounds
        )?;
        writeln!(
            audit,
            " MaxVerticesPerSurface={:12}",
            o.geometry.coordinate.max_vertices_per_surface
        )?;
        if o.system.shadowing_clipping.sutherland_hodgman {
            if o.system.shadowing_clipping.slater_barsky {
                writeln!(audit, " ClippingAlgorithm=SlaterBarskyandSutherlandHodgman")?;
            } else {
                writeln!(audit, " ClippingAlgorithm=SutherlandHodgman")?;
            }
        } else {
            // Original1825 deliberately has no leading blank in this branch.
            writeln!(audit, "ClippingAlgorithm=ConvexWeilerAtherton")?;
        }
        writeln!(
            audit,
            " MonthlyFieldSetInputCount={:12}",
            o.tabular.source_monthly_field_set_input_count()
        )?;
        writeln!(
            audit,
            " NumConsideredOutputVariables={:12}",
            o.prescan.num_considered_output_variables
        )?;
        writeln!(
            audit,
            " MaxConsideredOutputVariables={:12}",
            o.prescan.max_considered_output_variables
        )?;
        writeln!(
            audit,
            " numActuatorsUsed={:12}",
            o.runtime_language.number_of_actuators_used
        )?;
        writeln!(
            audit,
            " numEMSActuatorsAvailable={:12}",
            o.runtime_language.number_ems_actuators_available
        )?;
        writeln!(
            audit,
            " maxEMSActuatorsAvailable={:12}",
            o.runtime_language.max_ems_actuators_available
        )?;
        writeln!(
            audit,
            " numInternalVariablesUsed={:12}",
            o.runtime_language.number_internal_variables_used
        )?;
        writeln!(
            audit,
            " numEMSInternalVarsAvailable={:12}",
            o.runtime_language.number_ems_internal_variables_available
        )?;
        writeln!(
            audit,
            " maxEMSInternalVarsAvailable={:12}",
            o.runtime_language.max_ems_internal_variables_available
        )?;
        writeln!(
            audit,
            " NumOfNodeConnections={:12}",
            o.connections.source_number_node_connections()
        )?;
        writeln!(
            audit,
            " MaxNumOfNodeConnections={:12}",
            o.connections.source_max_number_node_connections()
        )?;
        // Native EP_Count_Calls conditional body is absent in the selected
        // ordinary build. No instrumentation counters are invented here.

        self.outputs[0].write_all(b"End of Data\n")?;
        if *o.standard_output_record_count > 0 {
            writeln!(
                self.outputs[0],
                " Number of Records Written={:12}",
                o.standard_output_record_count
            )?;
            self.outputs[0].close_source_output();
        } else {
            self.outputs[0].delete_source_output()?;
        }

        if o.heat_balance.any_cond_fd {
            self.outputs[1].write_all(b"! <ConductionFiniteDifference Numerical Parameters>, Starting Relaxation Factor, Final Relaxation Factor\n")?;
            return Err(Error::Unavailable(
                "actual CloseOutputFiles CondFDRelaxFactorInput/CondFDRelaxFactor1873 read",
            ));
        }
        self.outputs[1].write_all(b"! <Program Control Information:Threads/Parallel Sims>, Threading Supported,Maximum Number of Threads, Env Set Threads (OMP_NUM_THREADS), EP Env Set Threads (EP_OMP_NUM_THREADS), IDF Set Threads, Number of Threads Used (Interior Radiant Exchange), Number Nominal Surfaces, Number Parallel Sims\n")?;
        let t = &o.system.threading;
        if t.enabled {
            let env = if t.environment_set_threads == 0 {
                "Not Set".to_owned()
            } else {
                t.environment_set_threads.to_string()
            };
            let ep_env = if t.energyplus_environment_set_threads == 0 {
                "Not Set".to_owned()
            } else {
                t.energyplus_environment_set_threads.to_string()
            };
            let idf = if t.idf_set_threads == 0 {
                "Not Set".to_owned()
            } else {
                t.idf_set_threads.to_string()
            };
            if t.number_active_simulations_set {
                writeln!(
                    self.outputs[1],
                    "Program Control Information:Threads/Parallel Sims, Yes,{}, {}, {}, {}, {}, {}, {}",
                    t.max_number_of_threads,
                    env,
                    ep_env,
                    idf,
                    t.number_interior_radiant_threads,
                    t.nominal_total_surfaces,
                    t.number_active_simulations
                )?;
            } else {
                writeln!(
                    self.outputs[1],
                    "Program Control Information:Threads/Parallel Sims, Yes,{}, {}, {}, {}, {}, {}, N/A",
                    t.max_number_of_threads,
                    env,
                    ep_env,
                    idf,
                    t.number_interior_radiant_threads,
                    t.nominal_total_surfaces
                )?;
            }
        } else if t.number_active_simulations_set {
            writeln!(
                self.outputs[1],
                "Program Control Information:Threads/Parallel Sims, No,{}, N/A, N/A, N/A, N/A, N/A, {}",
                t.max_number_of_threads, t.number_active_simulations
            )?;
        } else {
            writeln!(
                self.outputs[1],
                "Program Control Information:Threads/Parallel Sims, No,{}, N/A, N/A, N/A, N/A, N/A, N/A",
                t.max_number_of_threads
            )?;
        }
        self.outputs[1].write_all(b"End of Data\n")?;
        self.outputs[1].close_source_output();
        self.outputs[2].write_all(b"End of Data\n")?;
        writeln!(
            self.outputs[2],
            " Number of Records Written={:12}",
            o.standard_meter_record_count
        )?;
        if *o.standard_meter_record_count > 0 {
            self.outputs[2].close_source_output();
        } else {
            self.outputs[2].delete_source_output()?;
        }
        // This is the sole real SHD stream already used/reset by SolarShading;
        // InputOutputFile.close is the same unique_ptr reset234-237.
        o.shading_stream.reset();
        Ok(())
    }
}
