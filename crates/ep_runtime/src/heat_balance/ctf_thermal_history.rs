//! Selected ordinary opaque CTF/EMPD thermal-history owners, without activation.
//!
//! Ports only the no-internal-source history suffix of InitThermalAndFluxHistories
//! and the ordinary UpdateThermalHistories branches. All allocated owners and
//! actual caller operands are supplied; this is not the whole surface lifecycle.

use super::surface_manager::ctf_all_resistive::CTF_COEFFICIENT_SLOTS;
use super::surface_manager::ctf_public_storage::CtfPublicCoefficientStorage;
use ep_model::{ConstructionId, OutsideBoundaryCondition, SurfaceId};
use std::collections::{BTreeMap, BTreeSet};

/// Native DataHeatBalance::SurfInitialTemp, separate from a user air fixture.
pub const CTF_SURFACE_INITIAL_TEMPERATURE: f64 = 23.0;

/// Actual allocated normal or master history arrays, term-major and fixed19.
/// Slot zero here is Native history term1, not public coefficient index zero.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfThermalHistoryArrays {
    /// SurfInsideTempHist or SurfInsideTempHistMaster.
    pub inside_temperature: [Vec<f64>; CTF_COEFFICIENT_SLOTS],
    /// SurfOutsideTempHist or SurfOutsideTempHistMaster.
    pub outside_temperature: [Vec<f64>; CTF_COEFFICIENT_SLOTS],
    /// SurfInsideFluxHist or SurfInsideFluxHistMaster; outside-to-inside sign.
    pub inside_flux: [Vec<f64>; CTF_COEFFICIENT_SLOTS],
    /// SurfOutsideFluxHist or SurfOutsideFluxHistMaster; before report negation.
    pub outside_flux: [Vec<f64>; CTF_COEFFICIENT_SLOTS],
}

/// Actual optionally allocated master arrays and per-surface block counters.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfThermalHistoryMasters {
    /// Four actual allocated master history owners.
    pub arrays: CtfThermalHistoryArrays,
    /// Actual SurfCurrNumHist; never inferred from elapsed time or CTF timestep.
    pub current_num_histories: Vec<i32>,
}

/// Actual UpdateThermalHistories first-block values and elapsed-time scratch.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfThermalHistoryScratch {
    /// QExt1.
    pub outside_flux_first: Vec<f64>,
    /// QInt1.
    pub inside_flux_first: Vec<f64>,
    /// TempInt1, distinct from the caller's current SurfTempIn.
    pub inside_temperature_first: Vec<f64>,
    /// TempExt1.
    pub outside_temperature_first: Vec<f64>,
    /// SumTime; assigned from the actual incremented counter on general visits.
    pub sum_time: Vec<f64>,
}

/// Existing allocated ordinary flux report owners mutated by the selected code.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfThermalHistoryFluxReports {
    /// SurfOpaqInsFaceCond.
    pub inside_conduction: Vec<f64>,
    /// SurfOpaqInsFaceCondFlux.
    pub inside_flux: Vec<f64>,
    /// SurfOpaqOutFaceCondFlux, with source unary negation.
    pub outside_flux: Vec<f64>,
    /// SurfOpaqOutFaceCond.
    pub outside_conduction: Vec<f64>,
}

/// Supplied allocated owners; no constructor fabricates an initialized state.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfThermalHistoryState {
    /// Full actual allocated surface layout, including surfaces outside HT loops.
    pub surface_ids: Vec<SurfaceId>,
    /// All four ordinary arrays, each of whose19 rows has TotSurfaces elements.
    pub normal: CtfThermalHistoryArrays,
    /// Actual optional master allocation, separate from scratch allocation.
    pub masters: Option<CtfThermalHistoryMasters>,
    /// Actual optional Update scratch allocation.
    pub scratch: Option<CtfThermalHistoryScratch>,
    /// Actual UpdateThermalHistoriesFirstTimeFlag before this call.
    pub update_first_time: bool,
    /// Actual report owners; their presence does not certify report completion.
    pub flux_reports: CtfThermalHistoryFluxReports,
}

impl CtfThermalHistoryState {
    /// Genuine empty DataHeatBalSurface arrays and HeatBalSurfMgr first-time flag.
    /// IDs are caller metadata from the completed actual GetSurfaceData order.
    /// Numerical dimensions/values are assigned ONLY by the real Source allocator.
    pub(crate) fn new_source_unallocated(surface_ids: Vec<SurfaceId>) -> Self {
        Self {
            surface_ids,
            normal: CtfThermalHistoryArrays {
                inside_temperature: std::array::from_fn(|_| Vec::new()),
                outside_temperature: std::array::from_fn(|_| Vec::new()),
                inside_flux: std::array::from_fn(|_| Vec::new()),
                outside_flux: std::array::from_fn(|_| Vec::new()),
            },
            masters: None,
            scratch: None,
            update_first_time: true,
            flux_reports: CtfThermalHistoryFluxReports {
                inside_conduction: Vec::new(),
                inside_flux: Vec::new(),
                outside_flux: Vec::new(),
                outside_conduction: Vec::new(),
            },
        }
    }
}

/// Actual selected surface heat-transfer algorithm, not coefficient inference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfThermalHistoryAlgorithm {
    /// Native CTF.
    Ctf,
    /// Native EMPD ordinary opaque conduction uses this same history code.
    Empd,
    /// A caller-owned other algorithm, unsupported by this bounded interface.
    Other,
}

/// Actual surface/construction association and current caller-owned operands.
#[derive(Clone, Copy, Debug)]
pub struct CtfThermalHistorySurface<'a> {
    /// Actual surface ID, looked up in the allocated layout without sorting.
    pub surface_id: SurfaceId,
    /// Actual current surface.Construction identity.
    pub construction_id: ConstructionId,
    /// Identity of the borrowed returned public construction owner.
    pub public_owner_construction_id: ConstructionId,
    /// All twelve actual fixed19 coefficient arrays and counts/dt/U.
    pub coefficients: &'a CtfPublicCoefficientStorage,
    /// Actual algorithm selector.
    pub algorithm: CtfThermalHistoryAlgorithm,
    /// Actual opaque/IntMass loop scope, not guessed from coefficients.
    pub ordinary_opaque: bool,
    /// Actual construction SourceSinkPresent, excluded from this proposal.
    pub source_sink_present: bool,
    /// Actual outside boundary selector; only Outdoors/Adiabatic are selected.
    pub outside_boundary: OutsideBoundaryCondition,
    /// Actual resolved SurfOutDryBulbTemp, used only by Outdoors initialization.
    pub outside_dry_bulb: f64,
    /// Actual surface.Area.
    pub area: f64,
    /// Actual SurfTempIn, separate from inside-temperature history term1.
    pub inside_surface_temperature: f64,
    /// Actual SurfCTFConstInPart, already produced by the true surface caller.
    pub inside_constant: f64,
    /// Actual SurfCTFConstOutPart, already produced by the true surface caller.
    pub outside_constant: f64,
}

/// Actual global caller inputs and loop orders; no flags/clock are inferred.
#[derive(Clone, Copy, Debug)]
pub struct CtfThermalHistoryContext<'a> {
    /// Actual SimpleCTFOnly.
    pub simple_ctf_only: bool,
    /// Actual AnyEnergyManagementSystemInModel, used by initialization.
    pub any_ems_in_model: bool,
    /// Actual AnyConstrOverridesInModel, used by update dispatch.
    pub any_construction_overrides: bool,
    /// Actual AnyInternalHeatSourceInInput; selected false only.
    pub any_internal_heat_source: bool,
    /// Actual global MaxCTFTerms, independent of individual construction counts.
    pub global_max_ctf_terms: i32,
    /// Actual TimeStepZone in hours, not reconstructed from CTFTimeStep.
    pub time_step_zone_hours: f64,
    /// Actual Zone/Space HT first..last traversal, each group in source order.
    pub heat_transfer_groups: &'a [Vec<SurfaceId>],
    /// Actual Zone/Space opaque/IntMass traversal, each group in source order.
    pub opaque_groups: &'a [Vec<SurfaceId>],
    /// Actual AllHTSurfaceList order, kept separately from Zone/Space traversal.
    pub all_heat_transfer_surfaces: &'a [SurfaceId],
}

/// Rust scope/allocated-owner representability, never a Native fatal outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfThermalHistoryUnavailable {
    /// Internal-source histories, other algorithms/classes or boundaries omitted.
    UnsupportedScope,
    /// Duplicate/missing IDs, wrong construction identity or mismatched loop set.
    Association,
    /// Actual allocated rows/vectors do not match the supplied surface layout.
    AllocatedShape,
    /// Required master/counter owners are absent; no default is substituted.
    MastersUnallocated,
    /// FirstTimeFlag is false but actual required scratch is absent.
    ScratchUnallocated,
    /// A supplied count would index beyond the fixed19 history owner.
    HistoryCapacity,
    /// Nonempty global max0 moves the current row in the simple Native branch;
    /// this fixed fully allocated owner declines that unrepresented moved state.
    SimpleCurrentRowMove,
    /// Native i32 increment has no representable successor; not a saturation.
    CounterSuccessor,
}

/// Actual selected update branch; does not certify the whole HBSM procedure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfThermalHistoryUpdateRoute {
    /// Global pointer shift and hard copy, followed by the literal early return.
    SimpleShift,
    /// Actual per-surface counter/completed-master/interpolated-master branch.
    MasterHistories,
}

struct Orders {
    heat_transfer: Vec<Vec<usize>>,
    opaque: Vec<Vec<usize>>,
    all: Vec<usize>,
    input: BTreeMap<usize, usize>,
    terms: BTreeMap<usize, usize>,
    global_last: usize,
}

fn arrays_match(arrays: &CtfThermalHistoryArrays, count: usize) -> bool {
    [
        &arrays.inside_temperature,
        &arrays.outside_temperature,
        &arrays.inside_flux,
        &arrays.outside_flux,
    ]
    .iter()
    .all(|value| value.iter().all(|row| row.len() == count))
}

fn prepare(
    state: &CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
) -> Result<Orders, CtfThermalHistoryUnavailable> {
    prepare_with_scope(state, surfaces, context, true)
}

fn prepare_with_scope(
    state: &CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
    selected_scope: bool,
) -> Result<Orders, CtfThermalHistoryUnavailable> {
    use CtfThermalHistoryUnavailable as Error;
    if selected_scope && context.any_internal_heat_source {
        return Err(Error::UnsupportedScope);
    }
    let global_last =
        usize::try_from(context.global_max_ctf_terms).map_err(|_| Error::HistoryCapacity)?;
    if global_last >= CTF_COEFFICIENT_SLOTS {
        return Err(Error::HistoryCapacity);
    }
    let count = state.surface_ids.len();
    let positions: BTreeMap<_, _> = state
        .surface_ids
        .iter()
        .copied()
        .enumerate()
        .map(|(index, id)| (id, index))
        .collect();
    if positions.len() != count {
        return Err(Error::Association);
    }
    if !arrays_match(&state.normal, count)
        || [
            &state.flux_reports.inside_conduction,
            &state.flux_reports.inside_flux,
            &state.flux_reports.outside_flux,
            &state.flux_reports.outside_conduction,
        ]
        .iter()
        .any(|value| value.len() != count)
    {
        return Err(Error::AllocatedShape);
    }
    let mut input = BTreeMap::new();
    let mut term_counts = BTreeMap::new();
    for (row, surface) in surfaces.iter().enumerate() {
        let index = *positions
            .get(&surface.surface_id)
            .ok_or(Error::Association)?;
        if input.insert(index, row).is_some()
            || surface.construction_id != surface.public_owner_construction_id
        {
            return Err(Error::Association);
        }
        if selected_scope
            && (!surface.ordinary_opaque
                || surface.source_sink_present
                || surface.algorithm == CtfThermalHistoryAlgorithm::Other
                || !matches!(
                    surface.outside_boundary,
                    OutsideBoundaryCondition::Outdoors | OutsideBoundaryCondition::Adiabatic
                ))
        {
            return Err(Error::UnsupportedScope);
        }
        let terms = usize::try_from(surface.coefficients.num_ctf_terms)
            .map_err(|_| Error::HistoryCapacity)?;
        if terms >= CTF_COEFFICIENT_SLOTS {
            return Err(Error::HistoryCapacity);
        }
        term_counts.insert(index, terms);
    }
    let map_groups = |groups: &[Vec<SurfaceId>]| -> Result<Vec<Vec<usize>>, Error> {
        groups
            .iter()
            .map(|group| {
                group
                    .iter()
                    .map(|id| positions.get(id).copied().ok_or(Error::Association))
                    .collect()
            })
            .collect()
    };
    let heat_transfer = map_groups(context.heat_transfer_groups)?;
    let opaque = map_groups(context.opaque_groups)?;
    let all: Vec<_> = context
        .all_heat_transfer_surfaces
        .iter()
        .map(|id| positions.get(id).copied().ok_or(Error::Association))
        .collect::<Result<_, _>>()?;
    let unique = |values: &[usize]| -> Result<BTreeSet<usize>, Error> {
        let set: BTreeSet<_> = values.iter().copied().collect();
        if set.len() != values.len() {
            return Err(Error::Association);
        }
        Ok(set)
    };
    let ht_flat: Vec<_> = heat_transfer.iter().flatten().copied().collect();
    let opaque_flat: Vec<_> = opaque.iter().flatten().copied().collect();
    let expected: BTreeSet<_> = input.keys().copied().collect();
    if unique(&ht_flat)? != expected
        || (selected_scope && unique(&opaque_flat)? != expected)
        || unique(&all)? != expected
    {
        return Err(Error::Association);
    }
    Ok(Orders {
        heat_transfer,
        opaque,
        all,
        input,
        terms: term_counts,
        global_last,
    })
}

fn masters_match(state: &CtfThermalHistoryState) -> Result<(), CtfThermalHistoryUnavailable> {
    use CtfThermalHistoryUnavailable as Error;
    let masters = state.masters.as_ref().ok_or(Error::MastersUnallocated)?;
    if !arrays_match(&masters.arrays, state.surface_ids.len())
        || masters.current_num_histories.len() != state.surface_ids.len()
    {
        return Err(Error::AllocatedShape);
    }
    Ok(())
}

/// Literal selected history suffix2323–2435, with Native23 and actual boundary
/// inputs. Caller must establish the genuine environment/lifecycle entry; this
/// function neither initializes zone air/convection nor claims whole Init return.
pub fn initialize_selected_ctf_thermal_histories(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
) -> Result<(), CtfThermalHistoryUnavailable> {
    let orders = prepare(state, surfaces, context)?;
    let master_selected = !context.simple_ctf_only || context.any_ems_in_model;
    if master_selected {
        masters_match(state)?;
    }
    initialize_history_bulk_from_orders(state, &orders, master_selected);
    initialize_history_boundary_flux_from_orders(state, surfaces, &orders, None)?;
    initialize_history_master_boundary_from_orders(
        state,
        surfaces,
        &orders,
        master_selected,
        None,
    )?;
    Ok(())
}

fn initialize_history_bulk_from_orders(
    state: &mut CtfThermalHistoryState,
    orders: &Orders,
    master_selected: bool,
) {
    for group in &orders.heat_transfer {
        for term in 0..CTF_COEFFICIENT_SLOTS {
            for &surface in group {
                state.normal.inside_temperature[term][surface] = CTF_SURFACE_INITIAL_TEMPERATURE;
                state.normal.outside_temperature[term][surface] = CTF_SURFACE_INITIAL_TEMPERATURE;
                state.normal.inside_flux[term][surface] = 0.0;
                state.normal.outside_flux[term][surface] = 0.0;
            }
        }
    }
    if master_selected {
        if let Some(masters) = state.masters.as_mut() {
            for group in &orders.heat_transfer {
                for &surface in group {
                    masters.current_num_histories[surface] = 0;
                }
                for term in 0..CTF_COEFFICIENT_SLOTS {
                    for &surface in group {
                        masters.arrays.inside_temperature[term][surface] =
                            CTF_SURFACE_INITIAL_TEMPERATURE;
                        masters.arrays.outside_temperature[term][surface] =
                            CTF_SURFACE_INITIAL_TEMPERATURE;
                        masters.arrays.inside_flux[term][surface] = 0.0;
                        masters.arrays.outside_flux[term][surface] = 0.0;
                    }
                }
            }
        }
    }
}

/// Temporary actual Source selector view and the same live Weather GroundTemp.
/// Only IDs/boundary codes are projected; no ground temperature is copied or retained.
#[derive(Clone, Copy)]
pub(crate) struct SourceCtfGroundTemperatures<'a> {
    pub(crate) surface_boundary_codes: &'a [(SurfaceId, i32)],
    pub(crate) ground_temp: &'a [f64; 4],
}
impl SourceCtfGroundTemperatures<'_> {
    fn borrow_temperature(&self, surface: SurfaceId) -> Result<&f64, CtfThermalHistoryUnavailable> {
        let code = self
            .surface_boundary_codes
            .iter()
            .find(|(id, _)| *id == surface)
            .map(|(_, code)| *code)
            .ok_or(CtfThermalHistoryUnavailable::Association)?;
        match code {
            -1 => Ok(&self.ground_temp[0]), // DataEnvironment::GroundTempType::BuildingSurface
            -5 => Ok(&self.ground_temp[3]), // DataEnvironment::GroundTempType::FCFactorMethod
            _ => Err(CtfThermalHistoryUnavailable::UnsupportedScope),
        }
    }
}

fn initialize_history_boundary_flux_from_orders(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    orders: &Orders,
    source_ground: Option<SourceCtfGroundTemperatures<'_>>,
) -> Result<(), CtfThermalHistoryUnavailable> {
    for term in 0..=orders.global_last {
        for &surface in &orders.all {
            let input = &surfaces[orders.input[&surface]];
            if input.outside_boundary == OutsideBoundaryCondition::Outdoors {
                state.normal.outside_temperature[term][surface] = input.outside_dry_bulb;
            } else if input.outside_boundary == OutsideBoundaryCondition::Ground {
                state.normal.outside_temperature[term][surface] = *source_ground
                    .ok_or(CtfThermalHistoryUnavailable::UnsupportedScope)?
                    .borrow_temperature(input.surface_id)?;
            }
            state.normal.outside_flux[term][surface] = input.coefficients.u_value
                * (state.normal.outside_temperature[0][surface]
                    - state.normal.inside_temperature[0][surface]);
            state.normal.inside_flux[term][surface] = state.normal.outside_flux[1][surface];
        }
    }
    Ok(())
}

fn initialize_history_master_boundary_from_orders(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    orders: &Orders,
    master_selected: bool,
    source_ground: Option<SourceCtfGroundTemperatures<'_>>,
) -> Result<(), CtfThermalHistoryUnavailable> {
    if master_selected {
        if let Some(masters) = state.masters.as_mut() {
            for &surface in &orders.all {
                let input = &surfaces[orders.input[&surface]];
                if input.outside_boundary == OutsideBoundaryCondition::Outdoors {
                    for term in 0..CTF_COEFFICIENT_SLOTS {
                        masters.arrays.outside_temperature[term][surface] = input.outside_dry_bulb;
                    }
                } else if input.outside_boundary == OutsideBoundaryCondition::Ground {
                    for term in 0..CTF_COEFFICIENT_SLOTS {
                        masters.arrays.outside_temperature[term][surface] = *source_ground
                            .ok_or(CtfThermalHistoryUnavailable::UnsupportedScope)?
                            .borrow_temperature(input.surface_id)?;
                    }
                }
                for term in 1..=orders.terms[&surface] {
                    masters.arrays.outside_flux[term][surface] =
                        state.normal.outside_flux[1][surface];
                    masters.arrays.inside_flux[term][surface] =
                        state.normal.outside_flux[1][surface];
                }
            }
        }
    }
    Ok(())
}

/// Genuine Source initialization bulk stage; caller supplies real intervening operations.
pub(crate) fn initialize_source_ctf_history_bulk(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
) -> Result<(), CtfThermalHistoryUnavailable> {
    let orders = prepare_with_scope(state, surfaces, context, false)?;
    let master_selected = !context.simple_ctf_only || context.any_ems_in_model;
    if master_selected {
        masters_match(state)?;
    }
    initialize_history_bulk_from_orders(state, &orders, master_selected);
    Ok(())
}

/// Genuine Source initialization boundary_flux stage; caller supplies real intervening operations.
pub(crate) fn initialize_source_ctf_history_boundary_flux(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
    source_ground: SourceCtfGroundTemperatures<'_>,
) -> Result<(), CtfThermalHistoryUnavailable> {
    let orders = prepare_with_scope(state, surfaces, context, false)?;
    initialize_history_boundary_flux_from_orders(state, surfaces, &orders, Some(source_ground))
}

/// Genuine Source initialization master_boundary stage; caller supplies real intervening operations.
pub(crate) fn initialize_source_ctf_history_master_boundary(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
    source_ground: SourceCtfGroundTemperatures<'_>,
) -> Result<(), CtfThermalHistoryUnavailable> {
    let orders = prepare_with_scope(state, surfaces, context, false)?;
    let master_selected = !context.simple_ctf_only || context.any_ems_in_model;
    if master_selected {
        masters_match(state)?;
    }
    initialize_history_master_boundary_from_orders(
        state,
        surfaces,
        &orders,
        master_selected,
        Some(source_ground),
    )
}

/// Literal selected no-source Update5245–5496. Current term1 temperatures and
/// true surface balance constants must already exist. No clock advance, history
/// term/count inference, boundary solve or report-finalization is performed here.
pub fn update_selected_ctf_thermal_histories(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
) -> Result<CtfThermalHistoryUpdateRoute, CtfThermalHistoryUnavailable> {
    let orders = prepare(state, surfaces, context)?;
    update_ctf_thermal_histories_from_orders(state, surfaces, context, orders)
}

/// Genuine Source Update does not read outside-boundary or outdoor operands.
/// Retain the same no-source/opaque/algorithm/shape/association scope while
/// allowing the actual Ground/FC/interzone selector to remain truthful.
pub(crate) fn update_source_ctf_thermal_histories(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
) -> Result<CtfThermalHistoryUpdateRoute, CtfThermalHistoryUnavailable> {
    use CtfThermalHistoryUnavailable as Error;
    let orders = prepare_with_scope(state, surfaces, context, false)?;
    if context.any_internal_heat_source
        || surfaces.iter().any(|surface| {
            !surface.ordinary_opaque
                || surface.source_sink_present
                || surface.algorithm == CtfThermalHistoryAlgorithm::Other
        })
    {
        return Err(Error::UnsupportedScope);
    }
    let opaque: Vec<_> = orders.opaque.iter().flatten().copied().collect();
    let unique: BTreeSet<_> = opaque.iter().copied().collect();
    let expected: BTreeSet<_> = orders.input.keys().copied().collect();
    if unique.len() != opaque.len() || unique != expected {
        return Err(Error::Association);
    }
    update_ctf_thermal_histories_from_orders(state, surfaces, context, orders)
}

fn update_ctf_thermal_histories_from_orders(
    state: &mut CtfThermalHistoryState,
    surfaces: &[CtfThermalHistorySurface<'_>],
    context: CtfThermalHistoryContext<'_>,
    orders: Orders,
) -> Result<CtfThermalHistoryUpdateRoute, CtfThermalHistoryUnavailable> {
    use CtfThermalHistoryUnavailable as Error;
    let simple = context.simple_ctf_only && !context.any_construction_overrides;
    if simple && orders.global_last == 0 && !state.surface_ids.is_empty() {
        return Err(Error::SimpleCurrentRowMove);
    }
    if !state.update_first_time {
        let scratch = state.scratch.as_ref().ok_or(Error::ScratchUnallocated)?;
        if [
            &scratch.outside_flux_first,
            &scratch.inside_flux_first,
            &scratch.inside_temperature_first,
            &scratch.outside_temperature_first,
            &scratch.sum_time,
        ]
        .iter()
        .any(|value| value.len() != state.surface_ids.len())
        {
            return Err(Error::AllocatedShape);
        }
    }
    if !simple {
        masters_match(state)?;
        if let Some(masters) = &state.masters {
            for group in &orders.opaque {
                for &surface in group {
                    masters.current_num_histories[surface]
                        .checked_add(1)
                        .ok_or(Error::CounterSuccessor)?;
                }
            }
        }
    }
    if state.update_first_time {
        let count = state.surface_ids.len();
        state.scratch = Some(CtfThermalHistoryScratch {
            outside_flux_first: vec![0.0; count],
            inside_flux_first: vec![0.0; count],
            inside_temperature_first: vec![0.0; count],
            outside_temperature_first: vec![0.0; count],
            sum_time: vec![0.0; count],
        });
        state.update_first_time = false;
    }
    for group in &orders.opaque {
        for &surface in group {
            let input = &surfaces[orders.input[&surface]];
            let coefficients = input.coefficients;
            if coefficients.num_ctf_terms == 0 {
                continue;
            }
            let outside_temperature = state.normal.outside_temperature[0][surface];
            let inside_flux = outside_temperature * coefficients.cross[0]
                - input.inside_surface_temperature * coefficients.inside[0]
                + input.inside_constant;
            state.flux_reports.inside_conduction[surface] = input.area * inside_flux;
            state.flux_reports.inside_flux[surface] = inside_flux;
            state.normal.inside_flux[0][surface] = inside_flux;
            state.normal.outside_flux[0][surface] = outside_temperature * coefficients.outside[0]
                - input.inside_surface_temperature * coefficients.cross[0]
                + input.outside_constant;
            state.flux_reports.outside_flux[surface] = -state.normal.outside_flux[0][surface];
            state.flux_reports.outside_conduction[surface] =
                input.area * state.flux_reports.outside_flux[surface];
        }
    }
    if simple {
        let last = orders.global_last;
        let inside_temperature = std::mem::take(&mut state.normal.inside_temperature[last]);
        let outside_temperature = std::mem::take(&mut state.normal.outside_temperature[last]);
        let inside_flux = std::mem::take(&mut state.normal.inside_flux[last]);
        let outside_flux = std::mem::take(&mut state.normal.outside_flux[last]);
        for term in (2..=last).rev() {
            state.normal.inside_temperature[term] =
                std::mem::take(&mut state.normal.inside_temperature[term - 1]);
            state.normal.outside_temperature[term] =
                std::mem::take(&mut state.normal.outside_temperature[term - 1]);
            state.normal.inside_flux[term] =
                std::mem::take(&mut state.normal.inside_flux[term - 1]);
            state.normal.outside_flux[term] =
                std::mem::take(&mut state.normal.outside_flux[term - 1]);
        }
        state.normal.inside_temperature[1] = inside_temperature;
        state.normal.outside_temperature[1] = outside_temperature;
        state.normal.inside_flux[1] = inside_flux;
        state.normal.outside_flux[1] = outside_flux;
        let (first, rest) = state.normal.inside_temperature.split_at_mut(1);
        rest[0].clone_from(&first[0]);
        let (first, rest) = state.normal.outside_temperature.split_at_mut(1);
        rest[0].clone_from(&first[0]);
        let (first, rest) = state.normal.inside_flux.split_at_mut(1);
        rest[0].clone_from(&first[0]);
        let (first, rest) = state.normal.outside_flux.split_at_mut(1);
        rest[0].clone_from(&first[0]);
        return Ok(CtfThermalHistoryUpdateRoute::SimpleShift);
    }
    // These owners were checked before mutation; absence is never substituted.
    let masters = state.masters.as_mut().ok_or(Error::MastersUnallocated)?;
    let scratch = state.scratch.as_mut().ok_or(Error::ScratchUnallocated)?;
    for group in &orders.opaque {
        for &surface in group {
            if masters.current_num_histories[surface] == 0 {
                scratch.outside_temperature_first[surface] =
                    state.normal.outside_temperature[0][surface];
                scratch.inside_temperature_first[surface] =
                    state.normal.inside_temperature[0][surface];
                scratch.outside_flux_first[surface] = state.normal.outside_flux[0][surface];
                scratch.inside_flux_first[surface] = state.normal.inside_flux[0][surface];
            }
        }
    }
    for group in &orders.opaque {
        for &surface in group {
            let coefficients = surfaces[orders.input[&surface]].coefficients;
            masters.current_num_histories[surface] += 1;
            scratch.sum_time[surface] =
                f64::from(masters.current_num_histories[surface]) * context.time_step_zone_hours;
            if masters.current_num_histories[surface] == coefficients.num_histories {
                masters.current_num_histories[surface] = 0;
                if coefficients.num_ctf_terms > 1 {
                    for term in (2..=orders.terms[&surface]).rev() {
                        masters.arrays.inside_temperature[term][surface] =
                            masters.arrays.inside_temperature[term - 1][surface];
                        masters.arrays.outside_temperature[term][surface] =
                            masters.arrays.outside_temperature[term - 1][surface];
                        masters.arrays.inside_flux[term][surface] =
                            masters.arrays.inside_flux[term - 1][surface];
                        masters.arrays.outside_flux[term][surface] =
                            masters.arrays.outside_flux[term - 1][surface];
                        state.normal.outside_temperature[term][surface] =
                            masters.arrays.outside_temperature[term - 1][surface];
                        state.normal.inside_temperature[term][surface] =
                            masters.arrays.inside_temperature[term - 1][surface];
                        state.normal.outside_flux[term][surface] =
                            masters.arrays.outside_flux[term - 1][surface];
                        state.normal.inside_flux[term][surface] =
                            masters.arrays.inside_flux[term - 1][surface];
                    }
                }
                masters.arrays.outside_temperature[1][surface] =
                    scratch.outside_temperature_first[surface];
                masters.arrays.inside_temperature[1][surface] =
                    scratch.inside_temperature_first[surface];
                masters.arrays.outside_flux[1][surface] = scratch.outside_flux_first[surface];
                masters.arrays.inside_flux[1][surface] = scratch.inside_flux_first[surface];
                state.normal.outside_temperature[1][surface] =
                    masters.arrays.outside_temperature[1][surface];
                state.normal.inside_temperature[1][surface] =
                    masters.arrays.inside_temperature[1][surface];
                state.normal.outside_flux[1][surface] = masters.arrays.outside_flux[1][surface];
                state.normal.inside_flux[1][surface] = masters.arrays.inside_flux[1][surface];
            } else {
                let sum_steps = scratch.sum_time[surface] / coefficients.ctf_time_step;
                if coefficients.num_ctf_terms > 1 {
                    for term in (2..=orders.terms[&surface]).rev() {
                        let thm_out_1 = masters.arrays.outside_temperature[term][surface];
                        let thm_in_1 = masters.arrays.inside_temperature[term][surface];
                        let thm_out_2 = masters.arrays.outside_temperature[term - 1][surface];
                        let thm_in_2 = masters.arrays.inside_temperature[term - 1][surface];
                        state.normal.outside_temperature[term][surface] =
                            thm_out_1 - (thm_out_1 - thm_out_2) * sum_steps;
                        state.normal.inside_temperature[term][surface] =
                            thm_in_1 - (thm_in_1 - thm_in_2) * sum_steps;
                        let qhm_out_1 = masters.arrays.outside_flux[term][surface];
                        let qhm_in_1 = masters.arrays.inside_flux[term][surface];
                        let qhm_out_2 = masters.arrays.outside_flux[term - 1][surface];
                        let qhm_in_2 = masters.arrays.inside_flux[term - 1][surface];
                        state.normal.outside_flux[term][surface] =
                            qhm_out_1 - (qhm_out_1 - qhm_out_2) * sum_steps;
                        state.normal.inside_flux[term][surface] =
                            qhm_in_1 - (qhm_in_1 - qhm_in_2) * sum_steps;
                    }
                }
                state.normal.outside_temperature[1][surface] = masters.arrays.outside_temperature
                    [1][surface]
                    - (masters.arrays.outside_temperature[1][surface]
                        - scratch.outside_temperature_first[surface])
                        * sum_steps;
                state.normal.inside_temperature[1][surface] = masters.arrays.inside_temperature[1]
                    [surface]
                    - (masters.arrays.inside_temperature[1][surface]
                        - scratch.inside_temperature_first[surface])
                        * sum_steps;
                state.normal.outside_flux[1][surface] = masters.arrays.outside_flux[1][surface]
                    - (masters.arrays.outside_flux[1][surface]
                        - scratch.outside_flux_first[surface])
                        * sum_steps;
                state.normal.inside_flux[1][surface] = masters.arrays.inside_flux[1][surface]
                    - (masters.arrays.inside_flux[1][surface] - scratch.inside_flux_first[surface])
                        * sum_steps;
            }
        }
    }
    Ok(CtfThermalHistoryUpdateRoute::MasterHistories)
}

impl std::fmt::Display for CtfThermalHistoryUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "selected CTF thermal-history scope unavailable: {self:?}"
        )
    }
}
impl std::error::Error for CtfThermalHistoryUnavailable {}

#[cfg(test)]
#[path = "ctf_thermal_history_tests.rs"]
mod tests;
