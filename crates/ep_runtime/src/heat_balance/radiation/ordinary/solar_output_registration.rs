//! Genuine Solar dictionary declarations with live semantic Which targets.
use super::{
    OrdinaryRadiationError as OutputError, OrdinaryRadiationUnavailable as Unavailable,
    OrdinarySolarRadiation,
};
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, surface::SurfaceGeometryInputState,
};
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceIntegerOutputDeclaration, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputRegistration, SourceOutputStoreType as Store, SourceOutputUnits as Units,
    SourceRealOutputBinding, SourceRealOutputDeclaration, SourceRealOutputTarget,
    SourceReportVariableInput, SourceSolarOutputField as Field,
};
use std::io::Write;
mod bindings;
mod debug;
mod module;
pub(crate) use debug::register_source_sky_diffuse_debug_outputs;
pub(crate) use module::register_source_solar_module_outputs;

pub(crate) struct SourceSolarOutputOwners<'a> {
    pub(crate) radiation: &'a OrdinarySolarRadiation,
    pub(crate) geometry: &'a SurfaceGeometryInputState,
}
pub(crate) struct SourceSolarOutputRegistrationContext<'a, 'request, 'backend, H, D> {
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) input: &'a mut SourceReportVariableInput<'request, 'backend>,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) host: &'a mut H,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) dictionary_host: &'a mut D,
}
#[derive(Clone, Copy)]
enum Target {
    Real(Field),
    IntegerBsdfDirection,
}
#[derive(Clone, Copy)]
struct Declaration {
    name: &'static str,
    units: Units,
    store: Store,
    target: Target,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("positive actual Solar output ordinal"))
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>
    SourceSolarOutputRegistrationContext<'_, '_, '_, H, D>
{
    fn declare(
        &mut self,
        owners: &SourceSolarOutputOwners<'_>,
        number: i32,
        key: &str,
        declaration: Declaration,
    ) -> Result<(), Error> {
        match declaration.target {
            Target::Real(field) => {
                let binding = SourceRealOutputBinding::new(
                    SourceRealOutputTarget::Solar {
                        field,
                        index: number,
                    },
                    owners,
                )
                .map_err(Error::Output)?;
                self.registration
                    .setup_bound_unmetered_zone_real(
                        self.input,
                        self.prescan,
                        SourceRealOutputDeclaration {
                            name: declaration.name,
                            key,
                            units: declaration.units,
                            store_type: declaration.store,
                        },
                        self.minutes_in_time_step,
                        self.host,
                        self.eio,
                        binding,
                        self.dictionary_host,
                    )
                    .map_err(Error::Output)
            }
            Target::IntegerBsdfDirection => {
                let _actual: &i32 = owners
                    .radiation
                    .solar_timestep
                    .heat_balance
                    .surf_win_bsdf_beam_direction_rep
                    .read_ref([usize::try_from(number)
                        .map_err(|_| Error::Unavailable("actual BSDF report ordinal"))?])?;
                self.registration
                    .setup_unmetered_zone_integer(
                        self.input,
                        self.prescan,
                        SourceIntegerOutputDeclaration {
                            name: declaration.name,
                            key,
                            units: declaration.units,
                            store_type: declaration.store,
                        },
                        self.minutes_in_time_step,
                        self.host,
                        self.eio,
                    )
                    .map_err(Error::Output)
            }
        }
    }
}
