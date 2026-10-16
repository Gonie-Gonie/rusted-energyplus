use super::*;

/// Mandatory genuine children. None has a success/default implementation.
/// A production host must use the real CTF entry and accepted SourceSolarChildren
/// host over these same owners; a precomputed CTF or legacy solar probe is invalid.
pub(crate) trait SourceInitHeatBalanceProviders {
    fn allocate_internal_gains(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn initialize_conduction_transfer_functions(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn initialize_positive_surface_property_view_factors(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn initialize_positive_equivalent_layer_windows(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn initialize_solar_calculations(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn perform_solar_calculations(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn allocate_carbon_dioxide(
        &mut self,
        arrays: &mut SourceContaminantArrays,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn allocate_generic_contaminant(
        &mut self,
        arrays: &mut SourceContaminantArrays,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn initialize_ems_constructions(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn initialize_ems_surface_properties(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn initialize_storm_windows(
        &mut self,
        manager: &mut SourceHeatBalanceManager,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn open_shading_file(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn report_external_sunlit_fractions(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn apply_zone_local_environments(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
    fn apply_zone_ems_overrides(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
    ) -> Result<(), Error>;
}
