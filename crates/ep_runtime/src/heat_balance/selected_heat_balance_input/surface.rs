//! Selected raw GetSurfaceData rows and retained opaque numerical prefix.
//! Bounded final copy, array constructors, floor areas and retained geometry are
//! consumed; later getter/enclosure/report lifecycle remains incomplete.
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

mod derived_arrays;
mod empty_subsurfaces;
mod final_mapping;
mod final_zero_suffix;
mod floor_areas;
mod foundation;
mod geometry_rules;
mod lists;
mod opaque;
mod parallel_arrays;
mod raw;
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
    Invalid,
}

/// Only the actual SurfaceData members written/read by the pre-vertex slice.
/// No full Native SurfaceData storage/copy or geometry-success claim follows.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SurfaceTmpInput {
    pub(crate) name: String,
    pub(crate) class: SurfaceClass,
    pub(crate) construction: i32,
    pub(crate) construction_stored_input: i32,
    pub(crate) heat_transfer_surface: bool,
    pub(crate) base_surface: i32,
    pub(crate) base_surface_name: String,
    pub(crate) zone_name: String,
    pub(crate) zone: i32,
    pub(crate) space: i32,
    pub(crate) exterior_boundary_name: String,
    pub(crate) exterior_boundary: i32,
    pub(crate) exterior_solar: bool,
    pub(crate) exterior_wind: bool,
    /// Genuine SurfaceData constructor0; positive window getter remains excluded.
    pub(crate) frame_divider: i32,
    /// SurfaceData constructor false; positive mirror/air-boundary getters stay excluded.
    pub(crate) mirrored_surface: bool,
    pub(crate) is_air_boundary_surface: bool,
    pub(crate) is_shadow_possible_obstruction: bool,
    /// Actual SurfaceData constructor841 and default std::vector795.
    pub(crate) has_shade_control: bool,
    pub(crate) window_shading_control_list: Vec<i32>,
    pub(crate) view_factor_ground: f64,
    pub(crate) sides: i32,
    // Objexx Vector's default elements are not initialized numeric triples.
    pub(crate) vertex: Option<Vec<Option<[f64; 3]>>>,
    pub(crate) new_vertex: Option<Vec<Option<[f64; 3]>>>,
    pub(crate) vertex_progress: vertices::VertexProgress,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SurfaceGeometryInputState {
    pub(crate) entered_once: bool,
    pub(crate) warning_displayed: bool,
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
    pub(crate) osc: Option<Vec<std::convert::Infallible>>,
    pub(crate) oscm: Option<Vec<std::convert::Infallible>>,
    pub(crate) foundation: foundation::FoundationInput,
    pub(crate) coordinate: vertices::CoordinateOwner,
    pub(crate) final_surfaces: Option<Vec<final_mapping::FinalSurface>>,
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
