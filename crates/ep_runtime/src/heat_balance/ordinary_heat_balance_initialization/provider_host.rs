//! Real HB2622 CTF and HB2631/2708 Solar children over disjoint live owners.
use super::ctf::{SourceCtfInitialization, SourceCtfInitializationOwners};
use super::{
    Error, SourceContaminantArrays, SourceHeatBalanceManager, SourceInitHeatBalanceOwners,
    SourceInitHeatBalanceProviders,
};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::radiation::ordinary::shading_report::SourceShadingStream;
use crate::heat_balance::radiation::ordinary::source_children::{
    SourceSolarChildOwners, SourceSolarReportFlags,
};
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
};
use ep_model::TypedModel;
use std::{io::Write, path::Path};

/// Only the additional live loans needed by Native InitHeatBalance2754.
/// Allocation-only hosts do not carry these; no Native state/readiness is stored.
pub(crate) struct SourceInitHeatBalanceOutdoorAirOwners<'a> {
    pub(crate) equipment: &'a mut crate::heat_balance::ordinary_equipment::OrdinaryEquipment,
    pub(crate) schedules:
        &'a mut crate::heat_balance::selected_schedule_manager::SelectedScheduleManager,
    pub(crate) outdoor_air_nodes:
        &'a mut crate::heat_balance::ordinary_hvac_manager::SourceOutdoorAirNodes,
    pub(crate) branch: &'a mut crate::heat_balance::startup_integrity::SourceBranchNodeConnections,
    pub(crate) source_is_epjson: &'a bool,
    pub(crate) psychrometrics: &'a mut crate::psychrometrics::SourcePsychrometricAccess,
    pub(crate) recurring:
        &'a mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
    pub(crate) hvac: &'a crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
}

/// Persistent numerical state stays in its original owner. These references
/// exist only for the enclosing genuine InitHeatBalance call.
pub(crate) struct SourceOrdinaryInitHeatBalanceHost<'a, H, D> {
    pub(crate) model: &'a TypedModel,
    pub(crate) outdoor_air: Option<SourceInitHeatBalanceOutdoorAirOwners<'a>>,
    pub(crate) ctf: &'a mut SourceCtfInitialization,
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) developer: &'a bool,
    pub(crate) output_host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) shading_stream: &'a mut SourceShadingStream,
    pub(crate) shading_path: &'a Path,
    pub(crate) report_flags: &'a mut SourceSolarReportFlags,
}
macro_rules! unavailable_child {
    ($method:ident,$reason:literal) => {
        fn $method(
            &mut self,
            _owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
        ) -> Result<(), Error> {
            Err(Error::Unavailable($reason))
        }
    };
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost> SourceInitHeatBalanceProviders
    for SourceOrdinaryInitHeatBalanceHost<'_, H, D>
{
    fn allocate_internal_gains(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        let zones = usize::try_from(owners.globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual AllocateIntGains NumOfZones"))?;
        let spaces = usize::try_from(owners.globals.num_spaces)
            .map_err(|_| Error::Unavailable("actual AllocateIntGains numSpaces"))?;
        owners
            .gains
            .allocate_source_internal_gains_if_unallocated(zones, spaces);
        Ok(())
    }
    fn initialize_conduction_transfer_functions(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.ctf
            .initialize_source_conduction_transfer_functions(&mut SourceCtfInitializationOwners {
                model: self.model,
                control: owners.control,
                materials: owners.materials,
                clock: owners.clock,
                input: owners.input,
                shortcut: owners.shortcut,
                report_inputs: owners.report_inputs,
                registry: self.registry,
                table: self.table,
                scan: owners.scan,
                backend: owners.backend,
                eio: self.eio,
            })
            .map_err(Error::from)
    }
    fn initialize_solar_calculations(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        // Build this child inside the actual call. Keeping it in the outer host
        // would alias the same mutable IP/Dayltg owners needed by earlier children.
        let mut children = SourceSolarChildOwners {
            shading_stream: self.shading_stream,
            shading_path: self.shading_path,
            report_flags: self.report_flags,
            globals: owners.globals,
            clock: owners.clock,
            zones: owners.zones,
            shadowing: owners.shadowing,
            daylighting: owners.daylighting,
            gain_modules: owners.gain_modules,
            input: owners.input,
            shortcut: owners.shortcut,
            report_inputs: owners.report_inputs,
            minimum_report_frequency: self.minimum_report_frequency,
            developer: self.developer,
            registration: self.registration,
            prescan: self.prescan,
            output_host: self.output_host,
            dictionary_host: self.dictionary_host,
            eio: self.eio,
        };
        owners.radiation.source_init_solar_calculations(
            owners.geometry,
            owners.zones,
            owners.control,
            owners.clock,
            owners.environment,
            owners.shadowing,
            &mut owners.surface.arrays,
            &mut children,
            owners.backend,
        )
    }
    fn perform_solar_calculations(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut children = SourceSolarChildOwners {
            shading_stream: self.shading_stream,
            shading_path: self.shading_path,
            report_flags: self.report_flags,
            globals: owners.globals,
            clock: owners.clock,
            zones: owners.zones,
            shadowing: owners.shadowing,
            daylighting: owners.daylighting,
            gain_modules: owners.gain_modules,
            input: owners.input,
            shortcut: owners.shortcut,
            report_inputs: owners.report_inputs,
            minimum_report_frequency: self.minimum_report_frequency,
            developer: self.developer,
            registration: self.registration,
            prescan: self.prescan,
            output_host: self.output_host,
            dictionary_host: self.dictionary_host,
            eio: self.eio,
        };
        let diagnostic_flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
            kickoff: &owners.globals.kickoff_simulation,
            developer: &owners.project.system.developer_flag,
            warmup: &owners.clock.warmup_flag,
            doing_sizing: &owners.globals.doing_sizing,
        };
        owners.radiation.source_perform_solar_calculations(
            owners.geometry,
            owners.control,
            owners.clock,
            owners.environment,
            owners.shadowing,
            &owners.project.system.shadowing_clipping,
            owners.globals,
            owners.globals.kickoff_sizing,
            owners.globals.kickoff_simulation,
            &mut children,
            owners.backend,
            &diagnostic_flags,
        )
    }
    fn initialize_positive_surface_property_view_factors(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        crate::heat_balance::ordinary_surface_initialization::initialize_source_surface_property_view_factors(
            owners.geometry, owners.globals, owners.surface.first_time,
        )
    }
    unavailable_child!(
        initialize_positive_equivalent_layer_windows,
        "InitHeatBalance2629 positive equivalent-layer windows"
    );
    fn allocate_carbon_dioxide(
        &mut self,
        _arrays: &mut SourceContaminantArrays,
        _owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "AllocateHeatBalArrays positive CO2 allocation",
        ))
    }
    fn allocate_generic_contaminant(
        &mut self,
        _arrays: &mut SourceContaminantArrays,
        _owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "AllocateHeatBalArrays positive generic-contaminant allocation",
        ))
    }
    unavailable_child!(
        initialize_ems_constructions,
        "InitHeatBalance2664 positive EMS construction overrides"
    );
    unavailable_child!(
        initialize_ems_surface_properties,
        "InitHeatBalance2665 positive EMS surface overrides"
    );
    fn initialize_storm_windows(
        &mut self,
        _manager: &mut SourceHeatBalanceManager,
        _owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        Err(Error::Unavailable(
            "InitHeatBalance2675 positive storm-window child",
        ))
    }
    unavailable_child!(
        open_shading_file,
        "InitHeatBalance2689 external-sunlit input stream"
    );
    unavailable_child!(
        report_external_sunlit_fractions,
        "InitHeatBalance2715 external-sunlit fraction reporting"
    );
    fn apply_zone_local_environments(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error> {
        let outdoor = self.outdoor_air.as_mut().ok_or(Error::Unavailable(
            "actual InitHeatBalance2754 outdoor-air interface loans",
        ))?;
        outdoor
            .outdoor_air_nodes
            .apply_source_zone_outdoor_environment(
            owners.input,
            outdoor.equipment,
            owners.control,
            outdoor.branch,
            *outdoor.source_is_epjson,
            outdoor.schedules,
            owners.zones,
            &mut crate::heat_balance::ordinary_hvac_manager::SourceOutdoorAirNodeRuntimeOwners {
                globals: owners.globals,
                clock: owners.clock,
                environment: owners.environment,
                hvac: outdoor.hvac,
                developer: &owners.project.system.developer_flag,
                psychrometrics: outdoor.psychrometrics,
                recurring: outdoor.recurring,
                backend: owners.backend,
            },
        )
    }
    unavailable_child!(
        apply_zone_ems_overrides,
        "InitHeatBalance2774 positive Zone EMS overrides"
    );
}
