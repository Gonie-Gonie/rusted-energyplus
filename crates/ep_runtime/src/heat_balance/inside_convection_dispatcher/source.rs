//! Genuine Tmp / iteration SurfTempIn entry, borrowing the sole Source owners.
//! Static scalar views are temporary; no mutable Surface convection metadata,
//! temperature, Space air, equipment flag, or HConv mirror is retained here.
mod ceiling;
mod cold_external;
use super::{
    ConvectionEntry, InsideConvectionCoefficients, InsideConvectionError, InsideConvectionLayout,
    InsideConvectionModel, InsideConvectionState, InsideConvectionSurface,
    InsideConvectionSurfaceMetadata, InsideConvectionUnavailable as Unavailable,
    InsideConvectionZone, InsideSurfaceTemperatureOwner, InsideSurfaceTemperatureValues,
    NoUserConvectionInput, initialize_inside_convection_prefix,
    initialize_inside_convection_surfaces, initialize_user_convection_metadata,
    validate_static_geometry,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::inside_convection_layout::{
    InsideConvectionAssociation, InsideConvectionGroup,
};
use crate::heat_balance::ordinary_air_heat_balance::{SourceAirHeatBalance, SourceSpaceAirView};
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::WeatherGlobalState;
use ep_model::{ReportInputs, SurfaceId, SurfaceType};

/// Live property/diagnostic roots borrowed by both genuine convection callers.
/// Every phase, Node, psychrometric cache and recurrence cell stays in its owner.
pub(crate) struct SourceInsideConvectionPropertyOwners<'a> {
    pub(crate) environment: &'a crate::weather::day::WeatherEnvironmentState,
    pub(crate) hvac: &'a crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
    pub(crate) psychrometrics: &'a mut crate::psychrometrics::SourcePsychrometricAccess,
    pub(crate) recurring:
        &'a mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
    pub(crate) developer: &'a bool,
}

fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("positive Source convection ordinal"))
}
fn translate(error: InsideConvectionError) -> Error {
    match error {
        InsideConvectionError::Backend(value) => Error::Backend(value),
        InsideConvectionError::Scan(_) => {
            Error::Unavailable("actual convection report scan arguments")
        }
        InsideConvectionError::Unavailable(value) => match value {
            Unavailable::Association => Error::Unavailable("actual convection owner association"),
            Unavailable::Geometry => {
                Error::Unavailable("actual convection finite static geometry domain")
            }
            Unavailable::UserConvectionInput => {
                Error::Unavailable("positive actual user convection getter")
            }
            Unavailable::AlgorithmOrSurface => {
                Error::Unavailable("unsupported inside/window/EMS convection branch")
            }
            Unavailable::SurfaceDetailsReport => {
                Error::Unavailable("actual convection Details report writer")
            }
            Unavailable::TemperatureOwner => {
                Error::Unavailable("actual convection full temperature owner")
            }
        },
    }
}

impl InsideConvectionSurfaceMetadata for SurfaceGeometryInputState {
    fn metadata_count(&self) -> Result<usize, Unavailable> {
        let rows = self
            .final_surfaces
            .as_ref()
            .ok_or(Unavailable::Association)?;
        let arrays = self
            .parallel_arrays
            .as_ref()
            .ok_or(Unavailable::Association)?;
        if arrays.interior_convection.len() != rows.len()
            || arrays.exterior_convection.len() != rows.len()
        {
            return Err(Unavailable::Association);
        }
        Ok(rows.len())
    }
    fn metadata_id(&self, index: usize) -> Result<SurfaceId, Unavailable> {
        self.final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(index))
            .and_then(|row| row.typed_id)
            .ok_or(Unavailable::Association)
    }
    fn model(&self, index: usize) -> Result<InsideConvectionModel, Unavailable> {
        match self
            .parallel_arrays
            .as_ref()
            .and_then(|a| a.interior_convection.get(index))
            .ok_or(Unavailable::Association)?
            .model
        {
            2 => Ok(InsideConvectionModel::SetByZone),
            3 => Ok(InsideConvectionModel::Simple),
            4 => Ok(InsideConvectionModel::Tarp),
            _ => Err(Unavailable::AlgorithmOrSurface),
        }
    }
    fn has_active_element(&self, index: usize) -> Result<bool, Unavailable> {
        Ok(self
            .parallel_arrays
            .as_ref()
            .and_then(|a| a.interior_convection.get(index))
            .ok_or(Unavailable::Association)?
            .has_active_in_it)
    }
    fn set_interior_geometry(&mut self, index: usize, values: [f64; 4]) -> Result<(), Unavailable> {
        let row = self
            .parallel_arrays
            .as_mut()
            .and_then(|a| a.interior_convection.get_mut(index))
            .ok_or(Unavailable::Association)?;
        row.zone_wall_height = values[0];
        row.zone_perimeter = values[1];
        row.zone_horizontal_hydraulic_diameter = values[2];
        row.window_wall_ratio = values[3];
        Ok(())
    }
    fn set_exterior_geometry(&mut self, index: usize, values: [f64; 3]) -> Result<(), Unavailable> {
        let row = self
            .parallel_arrays
            .as_mut()
            .and_then(|a| a.exterior_convection.get_mut(index))
            .ok_or(Unavailable::Association)?;
        row.face_area = values[0];
        row.face_perimeter = values[1];
        row.face_height = values[2];
        Ok(())
    }
}

struct SourceTemperatures<'a> {
    owner: InsideSurfaceTemperatureOwner,
    layout: &'a InsideConvectionLayout,
    values: &'a [f64],
}
impl InsideSurfaceTemperatureValues for SourceTemperatures<'_> {
    fn temperature_owner(&self) -> InsideSurfaceTemperatureOwner {
        self.owner
    }
    fn temperature_count(&self) -> usize {
        self.values.len()
    }
    fn temperature_id(&self, index: usize) -> SurfaceId {
        self.layout.surfaces()[index].surface_id
    }
    fn borrow_temperature(&self, index: usize) -> Result<&f64, Unavailable> {
        self.values.get(index).ok_or(Unavailable::TemperatureOwner)
    }
}
struct SourceCoefficients<'a> {
    layout: &'a InsideConvectionLayout,
    values: &'a mut [f64],
}
impl InsideConvectionCoefficients for SourceCoefficients<'_> {
    fn surface_count(&self) -> usize {
        self.values.len()
    }
    fn surface_id(&self, index: usize) -> SurfaceId {
        self.layout.surfaces()[index].surface_id
    }
    fn set_coefficient(&mut self, index: usize, value: f64) {
        self.values[index] = value;
    }
    fn borrow_coefficient(&self, index: usize) -> Result<&f64, Unavailable> {
        self.values.get(index).ok_or(Unavailable::Association)
    }
}

fn source_layout(
    geometry: &SurfaceGeometryInputState,
    owners: &SelectedZoneInputOwners,
) -> Result<InsideConvectionLayout, Error> {
    let zones = owners
        .zones
        .as_ref()
        .ok_or(Error::Unavailable("actual convection Zones"))?;
    let spaces = owners
        .spaces
        .as_ref()
        .ok_or(Error::Unavailable("actual convection Spaces"))?;
    let surfaces = geometry
        .final_surfaces
        .as_ref()
        .ok_or(Error::Unavailable("actual convection Surface"))?;
    let mut associations = Vec::with_capacity(surfaces.len());
    for surface in surfaces {
        associations.push(InsideConvectionAssociation {
            surface_id: surface
                .typed_id
                .ok_or(Error::Unavailable("actual convection SurfaceId"))?,
            zone_id: zones
                .get(index(surface.row.zone)?)
                .and_then(|z| z.typed_zone_id)
                .ok_or(Error::Unavailable("actual convection Surface Zone"))?,
            space_id: spaces
                .get(index(surface.row.space)?)
                .and_then(|s| s.typed_space_id)
                .ok_or(Error::Unavailable("actual convection Surface Space"))?,
        });
    }
    let mut groups = Vec::new();
    for (zone_index, zone) in zones.iter().enumerate() {
        let zone_id = zone
            .typed_zone_id
            .ok_or(Error::Unavailable("actual convection ZoneId"))?;
        for &number in &zone.space_indexes {
            let space = spaces
                .get(index(number)?)
                .ok_or(Error::Unavailable("actual convection Zone.spaceIndexes"))?;
            if index(space.zone_num)? != zone_index {
                return Err(Error::Unavailable(
                    "actual convection independent Space Zone",
                ));
            }
            let mut ids = Vec::new();
            for surface_number in space.ht_surface_first..=space.ht_surface_last {
                ids.push(
                    surfaces
                        .get(index(surface_number)?)
                        .and_then(|s| s.typed_id)
                        .ok_or(Error::Unavailable("actual convection HTSurfaceFirst..Last"))?,
                );
            }
            if space.window_first <= space.window_last {
                return Err(Error::Unavailable(
                    "actual convection positive Window range",
                ));
            }
            groups.push(InsideConvectionGroup {
                zone_id,
                space_id: space
                    .typed_space_id
                    .ok_or(Error::Unavailable("actual convection SpaceId"))?,
                surfaces: ids,
            });
        }
    }
    InsideConvectionLayout::new(
        zones
            .iter()
            .map(|z| {
                z.typed_zone_id
                    .ok_or(Error::Unavailable("actual convection ordered Zones"))
            })
            .collect::<Result<_, _>>()?,
        spaces
            .iter()
            .map(|s| {
                Ok((
                    s.typed_space_id
                        .ok_or(Error::Unavailable("actual convection ordered Spaces"))?,
                    zones
                        .get(index(s.zone_num)?)
                        .and_then(|z| z.typed_zone_id)
                        .ok_or(Error::Unavailable("actual convection Space ZoneId"))?,
                ))
            })
            .collect::<Result<_, Error>>()?,
        associations,
        groups,
    )
    .map_err(|_| Error::Unavailable("actual full Source convection traversal"))
}

fn static_zone_views(owners: &SelectedZoneInputOwners) -> Result<Vec<InsideConvectionZone>, Error> {
    owners
        .zones
        .as_ref()
        .ok_or(Error::Unavailable("actual convection static Zones"))?
        .iter()
        .map(|zone| {
            Ok(InsideConvectionZone {
                zone_id: zone
                    .typed_zone_id
                    .ok_or(Error::Unavailable("actual convection static ZoneId"))?,
                inside_algorithm: zone.inside_algorithm,
                ceiling_height: zone.ceiling_height,
                volume: zone.volume,
                floor_area: zone.floor_area,
                xy_bounds: [
                    zone.bounds_m[0],
                    zone.bounds_m[1],
                    zone.bounds_m[2],
                    zone.bounds_m[3],
                ],
                exterior_gross_wall_area: zone.exterior_gross_wall_area,
                exterior_window_area: zone.exterior_window_area,
                multiplier: zone.multiplier,
                list_multiplier: zone.list_multiplier,
            })
        })
        .collect::<Result<Vec<_>, Error>>()
}

fn static_views(
    geometry: &SurfaceGeometryInputState,
    owners: &SelectedZoneInputOwners,
    control: &SelectedHeatBalanceInputState,
) -> Result<(Vec<InsideConvectionZone>, Vec<InsideConvectionSurface>), Error> {
    let zones = static_zone_views(owners)?;
    let source = geometry
        .final_surfaces
        .as_ref()
        .ok_or(Error::Unavailable("actual convection static Surface"))?;
    let parallel = geometry
        .parallel_arrays
        .as_ref()
        .ok_or(Error::Unavailable("actual convection EMS fields"))?;
    if parallel.surf_ems_override_int_conv_coef.len() != source.len() {
        return Err(Error::Unavailable("actual convection EMS cardinality"));
    }
    let mut surfaces = Vec::with_capacity(source.len());
    for (i, surface) in source.iter().enumerate() {
        let row = &surface.row;
        let properties = row
            .vertex_progress
            .properties
            .ok_or(Error::Unavailable("actual produced convection properties"))?;
        let construction = control
            .construction_usage
            .get(index(row.construction)?)
            .ok_or(Error::Unavailable("actual convection Construction"))?;
        let class = match row.class {
            SurfaceClass::Wall => SurfaceType::Wall,
            SurfaceClass::Roof => SurfaceType::Roof,
            SurfaceClass::Floor => SurfaceType::Floor,
            _ => {
                return Err(Error::Unavailable(
                    "actual convection nonopaque Surface class",
                ));
            }
        };
        let vertices = row
            .vertex
            .as_ref()
            .ok_or(Error::Unavailable("actual convection Vertex allocation"))?
            .iter()
            .map(|v| v.ok_or(Error::Unavailable("actual convection written Vertex")))
            .collect::<Result<Vec<_>, _>>()?;
        surfaces.push(InsideConvectionSurface {
            surface_id: surface
                .typed_id
                .ok_or(Error::Unavailable("actual convection static SurfaceId"))?,
            class,
            exterior: row.exterior_boundary == 0,
            heat_transfer: row.heat_transfer_surface,
            construction_is_window: construction.type_is_window,
            kiva: row.exterior_boundary == -6,
            representative_calculations: geometry.use_representative_surface_calculations,
            ems_override: parallel.surf_ems_override_int_conv_coef[i],
            outward_cos_tilt: properties.cos_tilt,
            tilt: properties.tilt_deg,
            azimuth: properties.azimuth_deg,
            area: properties.area_m2,
            gross_area: properties.gross_area_m2,
            width: row.vertex_progress.width,
            perimeter: row.vertex_progress.perimeter,
            vertices,
        });
    }
    Ok((zones, surfaces))
}

/// Actual module flags plus immutable identity closure. Mutable per-Surface
/// convection records stay exclusively in the existing Source geometry owner.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceInsideConvection {
    state: InsideConvectionState,
    actual_no_user_input: Option<NoUserConvectionInput>,
}
impl SourceInsideConvection {
    pub(crate) fn new(
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
    ) -> Result<Self, Error> {
        let layout = match cold_external::layout(geometry, zones)? {
            Some(layout) => layout,
            None => source_layout(geometry, zones)?,
        };
        Ok(Self {
            state: InsideConvectionState::new_source_control(&layout),
            actual_no_user_input: None,
        })
    }
    /// Actual GetUserConvCoeffs zero-input path. Eight real count calls include
    /// the repeated coefficient/multiple-surface passes, in literal Source order.
    fn get_source_user_input(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<NoUserConvectionInput, Error> {
        let mut counts = [0; 6];
        counts[0] = input.get_num_objects_found(NoUserConvectionInput::FAMILIES[0]);
        if counts[0] != 0 {
            return Err(Error::Unavailable(
                "positive actual interior user-curve allocation",
            ));
        }
        self.state.user_arrays[0] = Some(Vec::new());
        counts[1] = input.get_num_objects_found(NoUserConvectionInput::FAMILIES[1]);
        if counts[1] != 0 {
            return Err(Error::Unavailable(
                "positive actual exterior user-curve allocation",
            ));
        }
        self.state.user_arrays[1] = Some(Vec::new());
        self.state.user_model_counts = [0; 2];
        counts[2] = input.get_num_objects_found(NoUserConvectionInput::FAMILIES[2]);
        if counts[2] != 0 {
            return Err(Error::Unavailable(
                "positive actual MultipleSurface count pass",
            ));
        }
        counts[3] = input.get_num_objects_found(NoUserConvectionInput::FAMILIES[3]);
        if counts[3] != 0 {
            return Err(Error::Unavailable(
                "positive actual single Surface count pass",
            ));
        }
        self.state.user_arrays[2] = Some(Vec::new());
        self.state.user_arrays[3] = Some(Vec::new());
        self.state.user_model_counts = [0; 2];
        if input.get_num_objects_found(NoUserConvectionInput::FAMILIES[3]) != 0 {
            return Err(Error::Unavailable(
                "positive actual single Surface real getter pass",
            ));
        }
        if input.get_num_objects_found(NoUserConvectionInput::FAMILIES[2]) != 0 {
            return Err(Error::Unavailable(
                "positive actual MultipleSurface real getter pass",
            ));
        }
        counts[4] = input.get_num_objects_found(NoUserConvectionInput::FAMILIES[4]);
        if counts[4] != 0 {
            return Err(Error::Unavailable(
                "positive actual inside AdaptiveModelSelections",
            ));
        }
        counts[5] = input.get_num_objects_found(NoUserConvectionInput::FAMILIES[5]);
        if counts[5] != 0 {
            return Err(Error::Unavailable(
                "positive actual outside AdaptiveModelSelections",
            ));
        }
        NoUserConvectionInput::new(counts)
            .map_err(|_| Error::Unavailable("actual convection user count closure"))
    }
    /// Exact shared InitExtConvCoeff first-call child. This performs only the
    /// actual GetUser/static/scan path; no inside coefficient or readiness write.
    /// A later call after Tmp completed the same flag returns without requery.
    pub(crate) fn prepare_external_convection(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        control: &SelectedHeatBalanceInputState,
        input: &SelectedInputProcessor,
        reports: &ReportInputs,
        scan: &mut ReportScanState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if !self.state.get_user_input {
            return Ok(());
        }
        if self.prepare_cold_external_convection(geometry, zones, input, reports, scan, backend)? {
            return Ok(());
        }
        let layout = source_layout(geometry, zones)?;
        if layout != self.state.layout
            || !self.state.surfaces.is_empty()
            || geometry.metadata_count().map_err(|e| translate(e.into()))?
                != layout.surfaces().len()
        {
            return Err(Error::Unavailable(
                "same actual Source external convection control",
            ));
        }
        for (i, row) in layout.surfaces().iter().enumerate() {
            if geometry.metadata_id(i).map_err(|e| translate(e.into()))? != row.surface_id {
                return Err(Error::Unavailable(
                    "actual Source external convection metadata identity",
                ));
            }
        }
        let (zone_views, surface_views) = static_views(geometry, zones, control)?;
        validate_static_geometry(&zone_views, &surface_views).map_err(|e| translate(e.into()))?;
        let actual = self.get_source_user_input(input)?;
        self.actual_no_user_input = Some(actual);
        initialize_user_convection_metadata(
            &layout,
            &zone_views,
            &surface_views,
            &mut self.state,
            geometry,
            reports,
            scan,
            backend,
        )
        .map_err(translate)
    }

    /// The same reached GetUser and real mutable NodeCheck prefix is shared by
    /// Tmp and iteration entries. Static Zone views are rebuilt after fallback.
    #[allow(clippy::too_many_arguments)]
    fn prepare_source_inside_prefix<M: InsideConvectionSurfaceMetadata + ?Sized>(
        &mut self,
        layout: &InsideConvectionLayout,
        zone_views: &[InsideConvectionZone],
        surface_views: &[InsideConvectionSurface],
        metadata: &mut M,
        zones: &mut SelectedZoneInputOwners,
        readiness: super::InsideConvectionReadiness,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        developer: &bool,
        input: &SelectedInputProcessor,
        reports: &ReportInputs,
        scan: &mut ReportScanState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<NoUserConvectionInput, Error> {
        let actual = if self.state.get_user_input {
            let actual = self.get_source_user_input(input)?;
            self.actual_no_user_input = Some(actual);
            initialize_user_convection_metadata(
                layout,
                zone_views,
                surface_views,
                &mut self.state,
                metadata,
                reports,
                scan,
                backend,
            )
            .map_err(translate)?;
            actual
        } else {
            self.actual_no_user_input
                .ok_or(Error::Unavailable("retained real convection getter facts"))?
        };
        ceiling::check_zone_nodes(
            &mut self.state,
            zones,
            readiness,
            globals,
            clock,
            developer,
            backend,
        )?;
        Ok(actual)
    }

    /// One actual Source call, with an explicit choice of the caller's array.
    /// The same object serves the initial Tmp and every-30 SurfTempIn positions.
    pub(crate) fn initialize(
        &mut self,
        temperature_owner: InsideSurfaceTemperatureOwner,
        geometry: &mut SurfaceGeometryInputState,
        arrays: &mut SourceSurfaceHeatBalanceArrays,
        zones: &mut SelectedZoneInputOwners,
        air: &SourceAirHeatBalance,
        control: &SelectedHeatBalanceInputState,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        equipment: &mut OrdinaryEquipment,
        input: &SelectedInputProcessor,
        reports: &ReportInputs,
        scan: &mut ReportScanState,
        mut properties: SourceInsideConvectionPropertyOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let layout = source_layout(geometry, zones)?;
        if layout != self.state.layout || !self.state.surfaces.is_empty() {
            return Err(Error::Unavailable(
                "same sole Source convection controller identity",
            ));
        }
        let (before_node_zones, surface_views) = static_views(geometry, zones, control)?;
        validate_static_geometry(&before_node_zones, &surface_views)
            .map_err(|e| translate(e.into()))?;
        let readiness = equipment.source_convection_readiness(
            clock.begin_envrn_flag,
            globals.system_sizing_calculation,
            globals.zone_sizing_calculation,
        );
        let no_user_input = self.prepare_source_inside_prefix(
            &layout,
            &before_node_zones,
            &surface_views,
            geometry,
            zones,
            readiness,
            globals,
            clock,
            properties.developer,
            input,
            reports,
            scan,
            backend,
        )?;
        // NodeCheck mutates the real Zone; no stale typed or temporary enum can
        // replace this fresh owner read for either prepass or ordinary dispatch.
        let zone_views = static_zone_views(zones)?;
        let values = match temperature_owner {
            InsideSurfaceTemperatureOwner::SurfTempInTmp => arrays.surf_temp_in_tmp.as_deref(),
            InsideSurfaceTemperatureOwner::SurfTempIn => arrays.surf_temp_in.as_deref(),
        }
        .ok_or(Error::Unavailable(
            "actual caller convection temperature allocation",
        ))?;
        let coefficients = arrays
            .surf_h_conv_int
            .as_deref_mut()
            .ok_or(Error::Unavailable("actual caller SurfHConvInt allocation"))?;
        let temperatures = SourceTemperatures {
            owner: temperature_owner,
            layout: &layout,
            values,
        };
        let mut coefficients = SourceCoefficients {
            layout: &layout,
            values: coefficients,
        };
        let space_air = SourceSpaceAirView {
            air,
            metadata: zones,
        };
        {
            // The active-element prefix temporarily borrows only real Config.
            // Release that loan before the whole-Zone child reads physical Nodes.
            let mut config = equipment.source_convection_equipment();
            initialize_inside_convection_prefix(
                &layout,
                &zone_views,
                &surface_views,
                no_user_input,
                &mut self.state,
                geometry,
                &mut config,
                &mut coefficients,
                &space_air,
                &temperatures,
                readiness,
                reports,
                scan,
                backend,
                ConvectionEntry::Source,
            )
            .map_err(translate)?;
        }
        ceiling::initialize(
            geometry,
            zones,
            air,
            &surface_views,
            &temperatures,
            &mut coefficients,
            equipment,
            globals,
            clock,
            &mut properties,
            control.low_hconv_limit,
            backend,
        )?;
        initialize_inside_convection_surfaces(
            &layout,
            &zone_views,
            &surface_views,
            geometry,
            &mut coefficients,
            &space_air,
            &temperatures,
            control.low_hconv_limit,
            ConvectionEntry::Source,
        )
        .map_err(translate)
    }
}

mod exterior;
pub(crate) use exterior::{SourceExternalConvectionCall, SourceExternalConvectionOwners};

#[cfg(all(test, feature = "native-controlled-air-cohort"))]
mod native_gap_cohort;
