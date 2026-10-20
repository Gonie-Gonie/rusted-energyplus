//! Selected raw GetSurfaceData rows and retained opaque numerical prefix.
//! Bounded final copy, array constructors, floor areas and retained geometry are
//! consumed; the admitted body return is distinct from SetupZoneGeometry/HB readiness.
use super::super::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use super::super::selected_project_data::ProjectInputObject;
use super::zone::SelectedZoneInputOwners;
use super::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInput, SelectedHeatBalanceInputState,
    SourceErrorBackend,
};
use std::collections::BTreeMap;
use std::io::Write;

mod boundary_inputs;
pub(crate) mod computed_geometry;
mod convexity;
mod derived_arrays;
mod diagnostics;
mod empty_subsurfaces;
mod final_mapping;
mod final_zero_suffix;
mod floor_areas;
mod foundation;
mod geometry_rules;
mod ground;
mod lists;
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
mod native_gap_cohort;
mod opaque;
mod parallel_arrays;
mod raw;
mod return_suffix;
mod setup_area;
mod setup_continuation;
mod setup_height;
mod setup_process;
mod setup_volume;
mod setup_zone_bounds;
mod vertices;
mod volume_faces;
mod window_arrays;

/// Raw operands from the same admitted model; not another InputProcessor.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectedSurfaceInput {
    /// Actual per-family getter-order GlobalGeometryRules objects.
    pub geometry_rules: Vec<ProjectInputObject>,
    /// Actual BuildingSurface getter order, before world-coordinate conversion.
    pub building_surfaces: Vec<OpaqueSurfaceInput>,
}
/// Actual JSON scalar operands plus the raw extensible coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct OpaqueSurfaceInput {
    /// Original name/fields and actual IDF maximum-field metadata.
    pub object: ProjectInputObject,
    /// Original parser extensible-field count; mandatory for IDF.
    pub idf_max_extensible_fields: Option<usize>,
    /// Complete admitted raw coordinate rows, before transformation.
    pub vertices: Vec<[f64; 3]>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum SurfaceClass {
    #[default]
    None,
    Wall,
    Floor,
    Roof,
    #[allow(dead_code)] // Native predicate only; positive getter remains excluded.
    Window,
    #[allow(dead_code)] // Native predicate only; positive getter remains excluded.
    Door,
    #[allow(dead_code)] // Genuine Setup count predicate; positive getter remains excluded.
    GlassDoor,
    #[allow(dead_code)] // Genuine Setup count predicate; positive getter remains excluded.
    TddDome,
    #[allow(dead_code)] // Genuine Setup count predicate; positive getter remains excluded.
    TddDiffuser,
    #[allow(dead_code)] // Genuine Setup predicate; positive getter remains excluded.
    Shading,
    #[allow(dead_code)] // Genuine Setup predicate; positive getter remains excluded.
    DetachedFixed,
    #[allow(dead_code)] // Genuine Setup predicate; positive getter remains excluded.
    DetachedBuilding,
    #[allow(dead_code)] // Genuine Setup skip predicate; positive getter remains excluded.
    IntMass,
    Invalid,
}

/// Selected actual SurfaceData members reached by the bounded raw/final prefix.
/// No full Native SurfaceData storage/copy or geometry-success claim follows.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceTmpInput {
    pub(crate) name: String,
    pub(crate) class: SurfaceClass,
    pub(crate) construction: i32,
    pub(crate) construction_stored_input: i32,
    pub(crate) heat_transfer_surface: bool,
    /// Actual SurfaceData constructor Invalid, overwritten at reached override8453.
    pub(crate) heat_transfer_algorithm: super::SelectedHeatTransferModel,
    /// Actual SurfaceData constructor0, written by InitSolar887-889.
    pub(crate) solar_encl_surface_index: i32,
    pub(crate) solar_encl_index: i32,
    pub(crate) rad_encl_index: i32,
    pub(crate) base_surface: i32,
    pub(crate) base_surface_name: String,
    pub(crate) zone_name: String,
    pub(crate) zone: i32,
    pub(crate) space: i32,
    pub(crate) exterior_boundary_name: String,
    pub(crate) exterior_boundary: i32,
    pub(crate) exterior_solar: bool,
    /// Actual SurfaceData.hh837 constructor true; CheckConvexity only writes false.
    pub(crate) is_convex: bool,
    pub(crate) exterior_wind: bool,
    /// Genuine SurfaceData constructor0; positive window getter remains excluded.
    pub(crate) frame_divider: i32,
    /// Actual SurfaceData.hh834 constructor0; positive window Reveal writes stay excluded.
    pub(crate) reveal: f64,
    /// SurfaceData constructor false; positive mirror/air-boundary getters stay excluded.
    pub(crate) mirrored_surface: bool,
    pub(crate) is_air_boundary_surface: bool,
    pub(crate) is_shadow_possible_obstruction: bool,
    pub(crate) is_shadowing: bool,
    pub(crate) shape: crate::geometry::SourceProcessShape,
    pub(crate) vertices_processed: bool,
    pub(crate) x_shift: f64,
    pub(crate) y_shift: f64,
    /// Actual SurfaceData constructor841 and default std::vector795.
    pub(crate) has_shade_control: bool,
    pub(crate) window_shading_control_list: Vec<i32>,
    /// Actual DataSurfaces.hh712–714 mutable areas, ctor0; GetVertices9387–9389 writes each.
    pub(crate) area: f64,
    pub(crate) gross_area: f64,
    pub(crate) net_area_shadow_calc: f64,
    pub(crate) view_factor_ground: f64,
    pub(crate) sides: i32,
    // Objexx Vector's default elements are not initialized numeric triples.
    pub(crate) vertex: Option<Vec<Option<[f64; 3]>>>,
    pub(crate) new_vertex: Option<Vec<Option<[f64; 3]>>>,
    pub(crate) vertex_progress: vertices::VertexProgress,
}
impl Default for SurfaceTmpInput {
    fn default() -> Self {
        Self {
            name: Default::default(),
            class: Default::default(),
            construction: Default::default(),
            construction_stored_input: Default::default(),
            heat_transfer_surface: Default::default(),
            heat_transfer_algorithm: Default::default(),
            solar_encl_surface_index: Default::default(),
            solar_encl_index: Default::default(),
            rad_encl_index: Default::default(),
            base_surface: Default::default(),
            base_surface_name: Default::default(),
            zone_name: Default::default(),
            zone: Default::default(),
            space: Default::default(),
            exterior_boundary_name: Default::default(),
            exterior_boundary: Default::default(),
            exterior_solar: Default::default(),
            is_convex: true,
            exterior_wind: Default::default(),
            frame_divider: Default::default(),
            reveal: 0.0,
            mirrored_surface: Default::default(),
            is_air_boundary_surface: Default::default(),
            is_shadow_possible_obstruction: Default::default(),
            is_shadowing: Default::default(),
            shape: Default::default(),
            vertices_processed: Default::default(),
            x_shift: Default::default(),
            y_shift: Default::default(),
            has_shade_control: Default::default(),
            window_shading_control_list: Default::default(),
            area: 0.0,
            gross_area: 0.0,
            net_area_shadow_calc: 0.0,
            view_factor_ground: Default::default(),
            sides: Default::default(),
            vertex: Default::default(),
            new_vertex: Default::default(),
            vertex_progress: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SurfaceGeometryInputState {
    pub(crate) entered_once: bool,
    pub(crate) warning_displayed: bool,
    /// Actual SurfaceGeometryData NoGroundTempObjWarning constructortrue;
    /// first Ground getter clears it after any actual diagnostic effects.
    pub(crate) no_ground_temp_warning: ground::GroundTemperatureWarning,
    /// SurfaceGeometryData ErrCount constructor0; Setup514 reads it without incrementing.
    pub(crate) err_count: i32,
    /// SurfaceGeometryData ErrCount5 constructor0; volume mismatches increment it.
    pub(crate) err_count5: i32,
    /// Actual SurfaceGeometry constructor counters; positive Window5/6 writers stay excluded.
    pub(crate) warning1_count: i32,
    pub(crate) warning2_count: i32,
    pub(crate) warning3_count: i32,
    pub(crate) corner: i32,
    pub(crate) counterclockwise: bool,
    pub(crate) world_coordinates: bool,
    pub(crate) daylight_world_coordinates: bool,
    pub(crate) rectangle_world_coordinates: bool,
    pub(crate) surface_tmp: Option<Vec<SurfaceTmpInput>>,
    pub(crate) unique_names: BTreeMap<String, String>,
    pub(crate) total_surfaces: i32,
    /// Actual DataSurface constructor0, used by list reserve2632-2635.
    pub(crate) total_windows: i32,
    pub(crate) surface_lists: lists::SurfaceLists,
    /// Actual GetSurfaceData1104-1133 locals, retained without repeat queries.
    pub(crate) source_surface_counts: [i32; 30],
    pub(crate) added_subsurfaces: i32,
    /// Actual local operands are unwritten until the respective getter resets.
    pub(crate) need_to_add_surfaces: Option<i32>,
    pub(crate) need_to_add_subsurfaces: Option<i32>,
    pub(crate) total_window_shading_control: i32,
    pub(crate) total_osc: i32,
    pub(crate) total_oscm: i32,
    /// DataSurface constructor0; actual parent GetIncidentSolarMultiplier count.
    pub(crate) total_incident_solar_multipliers: i32,
    /// DataSurface constructor0; actual GetScheduledSurfaceGains count owners.
    pub(crate) total_incident_solar_scheduled_gains: i32,
    pub(crate) total_fenestration_absorbed_solar_scheduled_gains: i32,
    /// Actual TotExtVentCav constructor0 and reached count; only allocate(0) is represented.
    pub(crate) total_ext_vented_cavities: i32,
    /// Actual DataSurface TotStormWin constructor0; positive writer stays excluded.
    pub(crate) total_storm_windows: i32,
    pub(crate) storm_windows: Option<Vec<std::convert::Infallible>>,
    /// Genuine DataSurface constructor0 and Ground8253/Local7972 count owners.
    pub(crate) total_surface_ground_properties: i32,
    pub(crate) total_surface_local_environments: i32,
    /// EPVector default unallocated; the selected zero loops do not allocate(0).
    pub(crate) ground_surface_properties: Option<Vec<std::convert::Infallible>>,
    pub(crate) surrounding_surface_properties: Option<Vec<std::convert::Infallible>>,
    pub(crate) surface_local_environments: Option<Vec<std::convert::Infallible>>,
    /// Diagnostic boundary only for this admitted GetSurfaceData body return.
    /// It does not complete SetupZoneGeometry or HeatBalanceInput readiness.
    pub(crate) selected_get_surface_data_returned: bool,
    pub(crate) ext_vented_cavities: Option<Vec<std::convert::Infallible>>,
    pub(crate) osc: Option<Vec<std::convert::Infallible>>,
    pub(crate) oscm: Option<Vec<std::convert::Infallible>>,
    pub(crate) foundation: foundation::FoundationInput,
    pub(crate) coordinate: vertices::CoordinateOwner,
    pub(crate) check_convexity: convexity::SourceConvexityState,
    pub(crate) process_vertices_scratch: setup_process::ProcessVerticesScratch,
    pub(crate) setup_shading: setup_process::SetupShadingCounters,
    pub(crate) final_surfaces: Option<Vec<final_mapping::FinalSurface>>,
    /// DataSurface SurfAdjacentZone constructor-unallocated; dimensioned at Setup607.
    pub(crate) adjacent_zone: Option<Vec<i32>>,
    pub(crate) surface_windows: Option<Vec<derived_arrays::SurfaceWindowCalculation>>,
    pub(crate) surface_shades: Option<Vec<derived_arrays::SurfaceShade>>,
    pub(crate) parallel_arrays: Option<parallel_arrays::SurfaceParallelArrays>,
    pub(crate) window_arrays: Option<window_arrays::SurfaceWindowArrays>,
    pub(crate) class_moved: Vec<bool>,
    /// Retained actual local remapping, not a second geometry input owner.
    pub(crate) old_to_new: Vec<i32>,
    pub(crate) moved_surfaces: usize,
    pub(crate) all_surface_list_report_order: Vec<i32>,
    pub(crate) centroid_scratch: crate::geometry::SourceCentroidScratch,
    pub(crate) use_representative_surface_calculations: bool,
    pub(crate) any_kiva: bool,
    /// DataSurfaces.hh1504/1507/1508 constructor flags, read by genuine InitSurface.
    pub(crate) airflow_windows: bool,
    pub(crate) any_movable_insulation: bool,
    pub(crate) any_movable_slat: bool,
    /// Actual DataSurface.hh1505 constructorfalse; no positive selected shading writer.
    pub(crate) shading_transmittance_varies: bool,
    pub(crate) surface_error: bool,
    pub(crate) surfaces_read: usize,
    pub(crate) next_object: usize,
    pub(crate) pending_vertices: Option<usize>,
}
pub(crate) struct SurfaceGetterOwners<'a> {
    pub(crate) model: &'a ep_model::TypedModel,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) control: &'a mut SelectedHeatBalanceInputState,
    pub(crate) globals: &'a super::super::selected_heat_balance_globals::SelectedHeatBalanceGlobals,
    pub(crate) environment: &'a crate::weather::day::WeatherEnvironmentState,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceInputPrefix {
    AlreadyEntered,
    OpaqueRows { count: usize },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceRowPrefix {
    Continue,
    BeforeVertices { slot: usize, sides: usize },
}

// Original1104-1133, in literal query order. Weight is used only in the
// Original1139-1144 total formula; selected positive families remain unavailable.
const FAMILIES: [(&str, i32); 30] = [
    ("Shading:Site:Detailed", 2),
    ("Shading:Building:Detailed", 2),
    ("Shading:Site", 2),
    ("Shading:Building", 2),
    ("BuildingSurface:Detailed", 1),
    ("Wall:Detailed", 1),
    ("RoofCeiling:Detailed", 1),
    ("Floor:Detailed", 1),
    ("FenestrationSurface:Detailed", 1),
    ("Shading:Zone:Detailed", 2),
    ("Shading:Overhang", 2),
    ("Shading:Overhang:Projection", 2),
    ("Shading:Fin", 4),
    ("Shading:Fin:Projection", 4),
    ("Window", 1),
    ("Door", 1),
    ("GlazedDoor", 1),
    ("Window:Interzone", 1),
    ("Door:Interzone", 1),
    ("GlazedDoor:Interzone", 1),
    ("Wall:Exterior", 1),
    ("Wall:Adiabatic", 1),
    ("Wall:Interzone", 1),
    ("Wall:Underground", 1),
    ("Roof", 1),
    ("Ceiling:Adiabatic", 1),
    ("Ceiling:Interzone", 1),
    ("Floor:GroundContact", 1),
    ("Floor:Adiabatic", 1),
    ("Floor:Interzone", 1),
];
fn signed(value: usize) -> Result<i32, Error> {
    i32::try_from(value).map_err(|_| Error::Unavailable("Original signed surface count"))
}
fn definition(input: &mut SelectedInputProcessor, family: &str) -> Result<usize, Error> {
    input
        .get_object_def_max_args(family)
        .map(|counts| counts.alphas)
        .map_err(|_| Error::Unavailable("actual surface object-definition closure"))
}
impl SurfaceGeometryInputState {
    /// End only at the first BuildingSurface definition query, not getter return.
    pub(crate) fn begin_input(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors: &mut bool,
    ) -> Result<SurfaceInputPrefix, Error> {
        if self.entered_once {
            return Ok(SurfaceInputPrefix::AlreadyEntered);
        }
        self.entered_once = true; // Actual1056, retained on every later failure.
        self.get_geometry_parameters(raw, owners, backend, eio, errors)?;
        let zones = owners
            .zones
            .zones
            .as_ref()
            .ok_or(Error::Unavailable("actual Zone input allocation"))?;
        if signed(zones.len())? != owners.globals.num_of_zones {
            return Err(Error::Unavailable("actual NumOfZones association"));
        }
        if self.world_coordinates {
            let relative = owners.control.building_azimuth != 0.0
                || zones.iter().any(|z| z.relative_north != 0.0);
            if relative && !self.warning_displayed {
                self.ignored_relative_warning(backend)?;
            }
            if zones.iter().any(|z| z.origin_m.iter().any(|v| *v != 0.0)) && !self.warning_displayed
            {
                self.ignored_relative_warning(backend)?;
            }
        }
        let mut counts = [0_i32; 30];
        for (slot, &(family, _)) in FAMILIES.iter().enumerate() {
            counts[slot] = signed(owners.input.get_num_objects_found(family))?;
        }
        self.source_surface_counts = counts;
        self.total_osc = 0;
        owners.shortcut.current_module_object = "InternalMass".into(); // GetNumIntMassSurfaces7219
        if owners.input.get_num_objects_found("InternalMass") != 0 {
            return Err(Error::Unavailable("positive InternalMass getter"));
        }
        for (index, &(family, _)) in FAMILIES.iter().enumerate() {
            if index != 4 && counts[index] != 0 {
                return Err(Error::Unavailable(family));
            }
        }
        if raw.surface.building_surfaces.len()
            != usize::try_from(counts[4])
                .map_err(|_| Error::Unavailable("actual surface raw count"))?
        {
            return Err(Error::Unavailable(
                "actual surface raw/IP count association",
            ));
        }
        // All other terms of the actual weighted total are genuinely zero.
        self.total_surfaces = counts[4];
        let total = usize::try_from(self.total_surfaces)
            .map_err(|_| Error::Unavailable("surface allocation count"))?;
        self.surface_tmp = Some(vec![SurfaceTmpInput::default(); total]);
        self.added_subsurfaces = 0; // actual local1151, after SurfaceTmp allocation
        backend.state.tracking.ask_for_surfaces_report = true; // Actual1152, same ErrorTracking owner.
        // GetDetShd3436 and GetRectDetShd3617 return before any scratch write.
        owners.shortcut.current_module_object = "SurfaceProperty:OtherSideCoefficients".into();
        self.total_osc = signed(
            owners
                .input
                .get_num_objects_found(&owners.shortcut.current_module_object),
        )?;
        if self.total_osc != 0 {
            return Err(Error::Unavailable("positive OtherSideCoefficients"));
        }
        self.osc = Some(Vec::new());
        owners.shortcut.current_module_object = "SurfaceProperty:OtherSideConditionsModel".into();
        self.total_oscm = signed(
            owners
                .input
                .get_num_objects_found(&owners.shortcut.current_module_object),
        )?;
        if self.total_oscm != 0 {
            return Err(Error::Unavailable("positive OtherSideConditionsModel"));
        }
        self.oscm = Some(Vec::new());
        self.foundation.get_zero_input(
            owners.input,
            owners.shortcut,
            owners.environment.elevation,
        )?;
        self.need_to_add_surfaces = Some(0); // GetHTSurfaceData3851, after real Foundation
        owners.shortcut.current_module_object = "BuildingSurface:Detailed".into();
        let alphas = definition(owners.input, &owners.shortcut.current_module_object)?;
        if alphas != 9 {
            backend.show_severe_error(&format!(
                "{}: Object Definition indicates not = 9 Alpha Objects, Number Indicated={alphas}",
                owners.shortcut.current_module_object
            ))?;
            *errors = true;
        }
        Ok(SurfaceInputPrefix::OpaqueRows { count: total })
    }
    fn ignored_relative_warning(
        &mut self,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        backend.show_warning_error("GetSurfaceData: World Coordinate System selected.  Any non-zero Building/Zone North Axes or non-zero Zone Origins are ignored.")?;
        backend.show_continue_error("These may be used in daylighting reference point coordinate calculations but not in normal geometry inputs.")?;
        self.warning_displayed = true;
        Ok(())
    }
    /// Caller invokes only after every actual BuildingSurface row's vertex work.
    /// These are the remaining three definition queries in the real Item loop.
    pub(crate) fn finish_empty_detailed_types(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        if self.pending_vertices.is_some() || signed(self.next_object)? != self.total_surfaces {
            return Err(Error::Unavailable(
                "actual complete BuildingSurface per-row call order",
            ));
        }
        for family in ["Wall:Detailed", "Floor:Detailed", "RoofCeiling:Detailed"] {
            shortcut.current_module_object = family.into();
            let alphas = definition(input, family)?;
            if alphas != 8 {
                backend.show_severe_error(&format!("{family}: Object Definition indicates not = 8 Alpha Objects, Number Indicated={alphas}"))?;
                *errors = true;
            }
        }
        Ok(()) // End only of selected GetHT input loop; not GetSurfaceData.
    }
}

// Test-only function-entry restoration, not a production geometry-getter execution claim.
#[cfg(test)]
impl SurfaceGeometryInputState {
    #[allow(
        dead_code,
        reason = "Captured-entry restoration is used only by the feature-gated Native cohort; ordinary test targets intentionally do not invoke this test-only physical surface owner method."
    )]
    pub(crate) fn restore_native_cohort_opaque_surfaces(
        &mut self,
        rows: &[(f64, SurfaceClass, i32)],
    ) {
        self.final_surfaces = Some(vec![final_mapping::FinalSurface::default(); rows.len()]);
        self.parallel_arrays = Some(parallel_arrays::SurfaceParallelArrays::allocate(rows.len()));
        if let (Some(surfaces), Some(parallel)) =
            (&mut self.final_surfaces, &mut self.parallel_arrays)
        {
            for (at, (area, class, reference)) in rows.iter().enumerate() {
                surfaces[at].row.area = *area;
                surfaces[at].row.class = *class;
                parallel.surf_t_air_ref[at] = *reference;
            }
        }
    }
}

#[cfg(test)]
mod source_surface_report_tests {
    use super::super::zone::{SelectedZoneData, SelectedZoneInputOwners};
    use super::*;
    use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
    use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
    use crate::heat_balance::selected_heat_balance_input::{
        SelectedConstructionUsage, SelectedHeatTransferModel,
    };
    use crate::heat_balance::source_initialization::primary_startup_surfaces::write_source_surface_details;
    use ep_model::{
        InsideSurfaceConvectionAlgorithm as Inside, OutsideSurfaceConvectionAlgorithm as Outside,
        Point3,
    };

    fn owners() -> CtfInitializationReports {
        // Boundary test using genuine Source constructors/allocators, not an active simulation claim.
        let mut reports = CtfInitializationReports {
            selected_heat_balance_globals: Some(SelectedHeatBalanceGlobals {
                num_of_zones: 1,
                ..Default::default()
            }),
            selected_zone_input: Some(SelectedZoneInputOwners {
                zones: Some(vec![SelectedZoneData {
                    name: "ACTUAL ZONE".into(),
                    all_surface_first: 1,
                    all_surface_last: 3,
                    inside_algorithm: Inside::CeilingDiffuser,
                    outside_algorithm: Outside::MoWitt,
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        };
        reports.heat_balance_input_state.total_constructions = 1;
        reports.heat_balance_input_state.construction_usage = vec![SelectedConstructionUsage {
            name: "LIVE CONSTRUCTION".into(),
            ..Default::default()
        }];
        reports.heat_balance_input_state.nominal_u = Some(vec![2.0]);
        reports.heat_balance_input_state.inside_convection_algorithm = Inside::Simple;
        reports
            .heat_balance_input_state
            .outside_convection_algorithm = Outside::Doe2;
        let geometry = &mut reports.surface_geometry_input;
        geometry.total_surfaces = 3;
        geometry.final_surfaces = Some(vec![final_mapping::FinalSurface::default(); 3]);
        geometry.parallel_arrays = Some(parallel_arrays::SurfaceParallelArrays::allocate(3));
        geometry.all_surface_list_report_order = vec![1, 3, 2];
        let points = [
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 3.0,
            },
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 2.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 2.0,
                y_m: 0.0,
                z_m: 3.0,
            },
        ];
        let normal = crate::geometry::source_normal_area(&points);
        let orientation = crate::geometry::orientation_from_normal(&points, normal.newell_normal);
        let properties = crate::geometry::finish_get_vertices_properties(normal, orientation);
        for (index, (name, class, boundary)) in [
            ("WALL", SurfaceClass::Wall, 0),
            ("FLOOR", SurfaceClass::Floor, -1),
            ("ROOF", SurfaceClass::Roof, 3),
        ]
        .into_iter()
        .enumerate()
        {
            let row = &mut geometry.final_surfaces.as_mut().unwrap()[index].row;
            row.name = name.into();
            row.class = class;
            row.zone = 1;
            row.base_surface = i32::try_from(index + 1).unwrap();
            row.heat_transfer_algorithm = SelectedHeatTransferModel::Ctf;
            row.construction = 1;
            row.exterior_boundary = boundary;
            row.exterior_boundary_name = name.into();
            row.exterior_wind = true;
            row.area = 5.0;
            row.gross_area = 6.0;
            row.net_area_shadow_calc = 6.0;
            row.sides = 4;
            row.vertex = Some(points.iter().map(|p| Some([p.x_m, p.y_m, p.z_m])).collect());
            row.vertex_progress.properties = Some(properties);
            row.vertex_progress.width = 2.0;
            row.vertex_progress.height = 3.0;
            row.view_factor_ground = 0.5;
            row.vertex_progress.view_factor_sky = 0.5;
            row.vertex_progress.view_factor_ground_ir = 0.25;
            row.vertex_progress.view_factor_sky_ir = 0.75;
        }
        reports
    }
    #[test]
    fn cost_info_uses_actual_ordinals_unique_boundaries_names_and_independent_areas_with_io_prefix()
    {
        use crate::heat_balance::source_initialization::primary_startup_surfaces::cost_info::{
            unique_cost_surfaces, write_cost_info_body,
        };
        let mut reports = owners();
        let header = "           3           3\n data for surfaces useful for cost information\n Number, Name, Construction, class, area, grossarea\n";
        let unique = unique_cost_surfaces(&reports).unwrap().unwrap();
        let mut bytes = Vec::new();
        write_cost_info_body(&reports, &unique, &mut bytes).unwrap();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.starts_with(header));
        assert_eq!(
            text.lines().skip(3).collect::<Vec<_>>(),
            [
                "    1,WALL,LIVE CONSTRUCTION,Wall,       5.00000,       6.00000",
                "    3,ROOF,LIVE CONSTRUCTION,Roof,       5.00000,       6.00000",
                "    2,FLOOR,LIVE CONSTRUCTION,Floor,       5.00000,       6.00000",
            ]
        );
        // A later physical ordinal paired to an earlier one is excluded;
        // construction0 is independently excluded. Self-pair3 above stays.
        let rows = reports
            .surface_geometry_input
            .final_surfaces
            .as_mut()
            .unwrap();
        rows[2].row.exterior_boundary = 1;
        rows[1].row.construction = 0;
        let unique = unique_cost_surfaces(&reports).unwrap().unwrap();
        assert_eq!(unique, [true, false, false]);
        bytes = Vec::new();
        write_cost_info_body(&reports, &unique, &mut bytes).unwrap();
        assert_eq!(String::from_utf8(bytes).unwrap().lines().count(), 4);
        struct HeaderOnly {
            bytes: Vec<u8>,
            limit: usize,
        }
        impl std::io::Write for HeaderOnly {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                let remaining = self.limit - self.bytes.len();
                if remaining == 0 {
                    return Err(std::io::Error::other("real unit writer failure"));
                }
                let count = remaining.min(bytes.len());
                self.bytes.extend_from_slice(&bytes[..count]);
                Ok(count)
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let reports = owners();
        let unique = unique_cost_surfaces(&reports).unwrap().unwrap();
        let mut writer = HeaderOnly {
            bytes: Vec::new(),
            limit: header.len(),
        };
        assert!(matches!(
            write_cost_info_body(&reports, &unique, &mut writer),
            Err(Error::Io(_))
        ));
        assert_eq!(writer.bytes, header.as_bytes());
        let mut reports = reports;
        reports.surface_geometry_input.final_surfaces = None;
        assert_eq!(unique_cost_surfaces(&reports).unwrap(), None);
    }

    #[test]
    fn details_report_types_preserve_real_order_owner_fields_class_films_and_headers() {
        for (kind, columns, vertices) in [(1, 18, true), (10, 27, false), (11, 39, true)] {
            let reports = owners();
            let mut bytes = Vec::new();
            write_source_surface_details(&reports, kind, &mut bytes).unwrap();
            let text = String::from_utf8(bytes).unwrap();
            assert!(text.contains("Zone Surfaces,ACTUAL ZONE,3\n"));
            assert_eq!(text.contains("Vertex 1 X {m}"), vertices);
            if vertices {
                assert!(text.contains("Vertex 4 X {m},Vertex 4 Z {m},Vertex 4 Z {m}"));
            }
            let rows: Vec<Vec<&str>> = text
                .lines()
                .filter(|line| line.starts_with("HeatTransfer Surface,"))
                .map(|line| line.split(',').collect())
                .collect();
            assert_eq!(
                rows.iter().map(|r| r[1]).collect::<Vec<_>>(),
                ["WALL", "ROOF", "FLOOR"]
            );
            assert!(rows.iter().all(|r| r.len() == columns
                && r[3].is_empty()
                && r[4] == "CTF - ConductionTransferFunction"));
            if kind == 1 {
                assert_eq!(
                    &rows[0][5..],
                    [
                        "4", "0.00", "0.00", "3.00", "0.00", "0.00", "0.00", "2.00", "0.00",
                        "0.00", "2.00", "0.00", "3.00"
                    ]
                );
            } else {
                assert_eq!(
                    rows.iter().map(|r| r[7]).collect::<Vec<_>>(),
                    ["1.539", "1.453", "1.346"]
                );
                assert_eq!(
                    &rows[0][5..12],
                    [
                        "LIVE CONSTRUCTION",
                        "2.000",
                        "1.539",
                        "",
                        "5.00",
                        "6.00",
                        "6.00"
                    ]
                );
                assert_eq!(&rows[0][18..20], ["MoWitt", "CeilingDiffuser"]);
                assert_eq!(
                    &rows[2][17..20],
                    ["Ground", "N/A-Ground", "CeilingDiffuser"]
                );
                assert_eq!(
                    &rows[1][17..20],
                    ["ROOF", "Other/Same Surface Int Conv", "CeilingDiffuser"]
                );
                assert_eq!(&rows[0][22..27], ["0.50", "0.50", "0.25", "0.75", "4"]);
                assert_eq!(rows[0][16], "0.00");
            }
        }
    }
    #[test]
    fn details_report_reads_surface_override_and_invalid_nominal_owners_without_substituting_geometry()
     {
        let mut reports = owners();
        reports
            .surface_geometry_input
            .parallel_arrays
            .as_mut()
            .unwrap()
            .interior_convection[0]
            .model = 3;
        reports.heat_balance_input_state.nominal_u.as_mut().unwrap()[0] = 0.0;
        let mut bytes = Vec::new();
        write_source_surface_details(&reports, 10, &mut bytes).unwrap();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains("LIVE CONSTRUCTION,0.000,[invalid],"));
        assert!(text.contains("ExternalEnvironment,MoWitt,ASHRAESimple,"));
        reports
            .surface_geometry_input
            .final_surfaces
            .as_mut()
            .unwrap()[0]
            .row
            .construction = 0;
        bytes = Vec::new();
        write_source_surface_details(&reports, 10, &mut bytes).unwrap();
        assert!(
            String::from_utf8(bytes)
                .unwrap()
                .contains("**invalid**,**,**,")
        );
    }
    #[test]
    fn details_report_buffers_until_complete_and_propagates_one_actual_eio_failure() {
        let mut reports = owners();
        reports
            .surface_geometry_input
            .parallel_arrays
            .as_mut()
            .unwrap()
            .interior_convection[1]
            .user_model_num = 1;
        let mut bytes = b"existing EIO prefix\n".to_vec();
        assert!(matches!(
            write_source_surface_details(&reports, 11, &mut bytes),
            Err(Error::Unavailable(_))
        ));
        assert_eq!(bytes, b"existing EIO prefix\n");
        reports
            .surface_geometry_input
            .parallel_arrays
            .as_mut()
            .unwrap()
            .interior_convection[1]
            .user_model_num = 0;
        struct FailedEio(usize);
        impl std::io::Write for FailedEio {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                self.0 += 1;
                Err(std::io::Error::other("actual test writer failure"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut writer = FailedEio(0);
        assert!(matches!(
            write_source_surface_details(&reports, 10, &mut writer),
            Err(Error::Io(_))
        ));
        assert_eq!(writer.0, 1);
        reports.surface_geometry_input.final_surfaces = None;
        bytes = b"existing EIO prefix\n".to_vec();
        write_source_surface_details(&reports, 11, &mut bytes).unwrap();
        assert_eq!(bytes, b"existing EIO prefix\n");
    }
}
