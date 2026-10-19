//! Persistent Source InitInteriorRadExchange/CalcInteriorRadExchange owners.
//! The real InitSurface caller owns its shared first-time flag and supplies
//! actual Source-indexed temperature/absorption/output arrays and caller flags.

mod absorption;
mod initialization;

use super::OrdinarySolarRadiation;
use super::{OrdinaryRadiationError as Error, OrdinaryRadiationUnavailable as Unavailable};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use ep_model::ReportInputs;
use std::io::Write;

/// Actual HeatBalanceIntRadExchgData constructor and persistent scratch.
#[derive(Clone, Debug, PartialEq)]
pub struct OrdinaryRadiantExchangeState {
    /// Actual MaxNumOfRadEnclosureSurfs, reset by each InitInterior invocation.
    pub max_num_enclosure_surfaces: i32,
    /// Actual CalcInteriorRadExchangefirstTime; cleared after scratch allocation.
    pub first_exchange_call: bool,
    /// Actual HeatBalanceIntRadExchgData LargestSurf, initialized0 and retained.
    pub largest_surface: i32,
    /// Same ViewFactorReport field written by Solar and radiant report scans.
    pub view_factor_report: bool,
    /// None: unallocated; Some(None): allocated, not written by a producer.
    pub surface_temp_rad: Option<Vec<Option<f64>>>,
    /// Actual ordered square-then-square temperature scratch, same write mask.
    pub surface_temp_in_kelvin_fourth: Option<Vec<Option<f64>>>,
    /// Actual Construction inside emissivity scratch, distinct from enclosure EMISS.
    pub surface_emissivity: Option<Vec<Option<f64>>>,
}

impl Default for OrdinaryRadiantExchangeState {
    fn default() -> Self {
        Self {
            max_num_enclosure_surfaces: 0,
            first_exchange_call: true,
            largest_surface: 0,
            view_factor_report: false,
            surface_temp_rad: None,
            surface_temp_in_kelvin_fourth: None,
            surface_emissivity: None,
        }
    }
}

/// Borrow the sole live Source metadata; no typed-ID arithmetic or copied F seed.
pub(crate) struct RadiantExchangeSource<'a> {
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) constructions: &'a SelectedHeatBalanceInputState,
    /// Actual retained SelectedProjectData CarrollMethod owner.
    pub(crate) carroll_method: bool,
    /// Actual DataGlobals AnyInsideShelf owner supplied by its genuine caller.
    pub(crate) any_inside_shelf: bool,
}

/// Disjoint borrows of the actual Scan/IP and output/error owners.
pub(crate) struct RadiantInitializationOutput<'a, 'b> {
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) backend: &'a mut SourceErrorBackend<'b>,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) advanced_report_variables: bool,
}

/// Actual function argument and Source global controls, never inferred from a mode.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RadiantExchangeCall {
    pub(crate) surface_iterations: i32,
    pub(crate) begin_environment: bool,
    pub(crate) kickoff_simulation: bool,
    pub(crate) kickoff_sizing: bool,
    pub(crate) any_movable_insulation: bool,
    pub(crate) carroll_method: bool,
}

fn index(value: i32) -> Result<usize, Error> {
    value
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or_else(|| Unavailable::StorageDomain.into())
}

fn value(values: &[f64], source_surface: i32) -> Result<f64, Error> {
    values
        .get(index(source_surface)?)
        .copied()
        .ok_or_else(|| Unavailable::StorageDomain.into())
}

fn backend_error(error: crate::heat_balance::error_backend::SourceErrorBackendFailure) -> Error {
    Error::Output(crate::error::WarmupOutputError::Backend(error))
}

fn script_f(
    area: &[f64],
    f: &[f64],
    emissivity: &mut [f64],
    backend: &mut SourceErrorBackend<'_>,
) -> Result<Vec<f64>, Error> {
    super::super::energyplus_scriptf_from_view_factors_with_warning(
        area, f, emissivity,
        || backend.show_warning_error("A thermal emissivity above 0.99999 was detected. This is not allowed. Value was reset to 0.99999").map_err(backend_error),
    )?.ok_or_else(|| Unavailable::NumericDomain.into())
}

impl OrdinarySolarRadiation {
    /// Source CalcInteriorRadExchange103-425, admitted complete opaque route.
    /// Temperature and absorption slices are complete actual Source ordinal views.
    /// NetLW is the same mutable array consumed immediately by the surface solve.
    #[expect(
        clippy::too_many_arguments,
        reason = "Keep actual Source array owners and caller flags distinct"
    )]
    pub(crate) fn exchange_source_radiant(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        constructions: &SelectedHeatBalanceInputState,
        temperature: &[f64],
        inside_thermal_absorptance: &[f64],
        net_lw: &mut [f64],
        call: RadiantExchangeCall,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.radiant_exchange.first_exchange_call {
            let n = usize::try_from(self.radiant_exchange.max_num_enclosure_surfaces)
                .map_err(|_| Unavailable::StorageDomain)?;
            self.radiant_exchange.surface_temp_rad = Some(vec![None; n]);
            self.radiant_exchange.surface_temp_in_kelvin_fourth = Some(vec![None; n]);
            self.radiant_exchange.surface_emissivity = Some(vec![None; n]);
            self.radiant_exchange.first_exchange_call = false;
            if backend.state.developer_flag {
                backend
                    .display_string(" OMP turned off, HBIRE loop executed in serial")
                    .map_err(backend_error)?;
            }
        }
        // Actual160 precedes every output reset and enclosure read.
        if call.kickoff_simulation || call.kickoff_sizing {
            return Ok(());
        }

        // No ZoneToResimulate is supplied by the admitted complete ordinary caller.
        let total =
            usize::try_from(geometry.total_surfaces).map_err(|_| Unavailable::StorageDomain)?;
        if net_lw.len() != total {
            return Err(Unavailable::StorageDomain.into());
        }
        net_lw.fill(0.0);
        let windows = geometry
            .window_arrays
            .as_mut()
            .ok_or(Unavailable::StorageDomain)?;
        for surface in 0..total {
            *windows
                .surf_win_i_rfrom_parent_zone
                .get_mut(surface)
                .ok_or(Unavailable::StorageDomain)? = 0.0;
        }
        let surfaces = geometry
            .final_surfaces
            .as_ref()
            .ok_or(Unavailable::StorageDomain)?;
        let count =
            usize::try_from(self.num_radiant_enclosures).map_err(|_| Unavailable::StorageDomain)?;
        for number in 0..count {
            let enclosure = self
                .radiant
                .get_mut(number)
                .ok_or(Unavailable::StorageDomain)?;
            let n =
                usize::try_from(enclosure.num_surfaces).map_err(|_| Unavailable::StorageDomain)?;
            let pointer = enclosure
                .source_surface_ptr
                .as_ref()
                .ok_or(Unavailable::StorageDomain)?;
            if call.surface_iterations == 0 {
                if !call.begin_environment {
                    for &surface_number in pointer {
                        let surface = surfaces
                            .get(index(surface_number)?)
                            .ok_or(Unavailable::StorageDomain)?;
                        let construction = constructions
                            .construction_usage
                            .get(index(surface.row.construction)?)
                            .ok_or(Unavailable::StorageDomain)?;
                        if construction.type_is_window || call.any_movable_insulation {
                            // First reached unported shade/movable-insulation change helper.
                            return Err(Unavailable::InputBranch.into());
                        }
                    }
                }
                if call.begin_environment {
                    let emissivity = enclosure
                        .emissivity
                        .as_mut()
                        .ok_or(Unavailable::StorageDomain)?;
                    for position in 0..n {
                        let surface_number =
                            *pointer.get(position).ok_or(Unavailable::StorageDomain)?;
                        *emissivity
                            .get_mut(position)
                            .ok_or(Unavailable::StorageDomain)? =
                            value(inside_thermal_absorptance, surface_number)?;
                        let surface = surfaces
                            .get(index(surface_number)?)
                            .ok_or(Unavailable::StorageDomain)?;
                        let construction = constructions
                            .construction_usage
                            .get(index(surface.row.construction)?)
                            .ok_or(Unavailable::StorageDomain)?;
                        if construction.type_is_window || construction.window_type_eql {
                            return Err(Unavailable::InputBranch.into());
                        }
                    }
                    if call.carroll_method {
                        return Err(Unavailable::InputBranch.into());
                    }
                    let calculated = script_f(
                        enclosure
                            .area
                            .as_deref()
                            .ok_or(Unavailable::StorageDomain)?,
                        enclosure.f.as_deref().ok_or(Unavailable::StorageDomain)?,
                        emissivity,
                        backend,
                    )?;
                    *enclosure
                        .script_f
                        .as_mut()
                        .ok_or(Unavailable::StorageDomain)? = calculated;
                    for entry in enclosure
                        .script_f
                        .as_mut()
                        .ok_or(Unavailable::StorageDomain)?
                    {
                        *entry *= super::super::STEFAN_BOLTZMANN_W_PER_M2_K4;
                    }
                }
            }
            for position in 0..n {
                let surface_number = *pointer.get(position).ok_or(Unavailable::StorageDomain)?;
                let surface = surfaces
                    .get(index(surface_number)?)
                    .ok_or(Unavailable::StorageDomain)?;
                let construction = constructions
                    .construction_usage
                    .get(index(surface.row.construction)?)
                    .ok_or(Unavailable::StorageDomain)?;
                if construction.window_type_eql || construction.type_is_window {
                    return Err(Unavailable::InputBranch.into());
                }
                let t = value(temperature, surface_number)?;
                *self
                    .radiant_exchange
                    .surface_temp_rad
                    .as_mut()
                    .and_then(|v| v.get_mut(position))
                    .ok_or(Unavailable::StorageDomain)? = Some(t);
                *self
                    .radiant_exchange
                    .surface_emissivity
                    .as_mut()
                    .and_then(|v| v.get_mut(position))
                    .ok_or(Unavailable::StorageDomain)? = Some(construction.inside_absorp_thermal);
                let kelvin = t + super::super::KELVIN_OFFSET;
                let squared = kelvin * kelvin;
                *self
                    .radiant_exchange
                    .surface_temp_in_kelvin_fourth
                    .as_mut()
                    .and_then(|v| v.get_mut(position))
                    .ok_or(Unavailable::StorageDomain)? = Some(squared * squared);
                if call.carroll_method {
                    return Err(Unavailable::InputBranch.into());
                }
            }
            let scratch = self
                .radiant_exchange
                .surface_temp_in_kelvin_fourth
                .as_ref()
                .ok_or(Unavailable::StorageDomain)?;
            let matrix = enclosure
                .script_f
                .as_mut()
                .ok_or(Unavailable::StorageDomain)?;
            for receiver in 0..n {
                let receiver_surface = *pointer.get(receiver).ok_or(Unavailable::StorageDomain)?;
                let receiver_t4 = scratch
                    .get(receiver)
                    .copied()
                    .flatten()
                    .ok_or(Unavailable::StorageDomain)?;
                let mut net = 0.0;
                let row = receiver.checked_mul(n).ok_or(Unavailable::StorageDomain)?;
                *matrix
                    .get_mut(row + receiver)
                    .ok_or(Unavailable::StorageDomain)? = 0.0;
                for sender in 0..n {
                    let coefficient =
                        *matrix.get(row + sender).ok_or(Unavailable::StorageDomain)?;
                    let sender_t4 = scratch
                        .get(sender)
                        .copied()
                        .flatten()
                        .ok_or(Unavailable::StorageDomain)?;
                    net += coefficient * (sender_t4 - receiver_t4);
                }
                *net_lw
                    .get_mut(index(receiver_surface)?)
                    .ok_or(Unavailable::StorageDomain)? += net;
            }
        }
        if geometry.use_representative_surface_calculations {
            return Err(Unavailable::InputBranch.into());
        }
        Ok(())
    }
}
