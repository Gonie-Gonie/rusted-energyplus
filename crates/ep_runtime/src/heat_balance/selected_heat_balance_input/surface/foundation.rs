//! Reached zero-input GetFoundationData and genuine default Kiva input object.
//! No solver/mesh producer is invoked or represented by the empty derived lists.
use super::{Error, SelectedInputProcessor, SourceInputProcessorShortCut};

/// Genuine KivaManager::Settings constructor, not user input or solver state.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Settings {
    pub(crate) soil: Material,
    pub(crate) grade: SurfaceProperties,
    pub(crate) far_field_width: f64,
    pub(crate) automatic_deep_ground_boundary: bool,
    pub(crate) deep_ground_depth: f64,
    pub(crate) autocalculate_deep_ground_depth: bool,
    pub(crate) minimum_cell_dimension: f64,
    pub(crate) maximum_cell_growth: f64,
    pub(crate) hourly_timestep: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            soil: Material {
                conductivity: 0.864,
                density: 1510.0,
                specific_heat: 1260.0,
            },
            grade: SurfaceProperties {
                emissivity: 0.9,
                absorptivity: 0.9,
                roughness: 0.9,
            },
            far_field_width: 40.0,
            automatic_deep_ground_boundary: true,
            deep_ground_depth: 40.0,
            autocalculate_deep_ground_depth: true,
            minimum_cell_dimension: 0.02,
            maximum_cell_growth: 1.5,
            hourly_timestep: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Material {
    pub(crate) conductivity: f64,
    pub(crate) density: f64,
    pub(crate) specific_heat: f64,
}
impl Default for Material {
    fn default() -> Self {
        Self {
            conductivity: 0.0,
            density: 0.0,
            specific_heat: 0.0,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SurfaceProperties {
    pub(crate) emissivity: f64,
    pub(crate) absorptivity: f64,
    pub(crate) roughness: f64,
}
impl Default for SurfaceProperties {
    fn default() -> Self {
        Self {
            emissivity: 0.8,
            absorptivity: 0.8,
            roughness: 0.00208,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Layer {
    pub(crate) material: Material,
    pub(crate) thickness: f64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct InputBlock {
    pub(crate) x: f64,
    pub(crate) z: f64,
    pub(crate) width: f64,
    pub(crate) depth: f64,
    pub(crate) material: Material,
    pub(crate) box_corners: [[f64; 2]; 2],
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Wall {
    pub(crate) interior: SurfaceProperties,
    pub(crate) exterior: SurfaceProperties,
    // Native Wall has no constructor for these two scalars; don't invent zero.
    pub(crate) height_above_grade: Option<f64>,
    pub(crate) depth_below_slab: Option<f64>,
    pub(crate) layers: Vec<Layer>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Slab {
    pub(crate) interior: SurfaceProperties,
    pub(crate) layers: Vec<Layer>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Mesh {
    pub(crate) minimum_cell_dimension: f64,
    pub(crate) maximum_near_growth: f64,
    pub(crate) maximum_depth_growth: f64,
    pub(crate) maximum_interior_growth: f64,
    pub(crate) maximum_exterior_growth: f64,
}
impl Default for Mesh {
    fn default() -> Self {
        Self {
            minimum_cell_dimension: 0.02,
            maximum_near_growth: 1.5,
            maximum_depth_growth: 1.5,
            maximum_interior_growth: 1.5,
            maximum_exterior_growth: 1.5,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct MeshData {
    pub(crate) points: Vec<f64>,
    pub(crate) intervals: Vec<std::convert::Infallible>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Foundation {
    pub(crate) deep_ground_depth: f64,
    pub(crate) far_field_width: f64,
    pub(crate) foundation_depth: f64,
    pub(crate) orientation: f64,
    pub(crate) deep_ground_boundary: DeepGroundBoundary,
    pub(crate) wall_top_interior_temperature: f64,
    pub(crate) wall_top_exterior_temperature: f64,
    pub(crate) wall_top_boundary_zero_flux: bool,
    pub(crate) soil: Material,
    pub(crate) grade: SurfaceProperties,
    pub(crate) coordinate_system_cartesian: bool,
    pub(crate) number_of_dimensions: i32,
    pub(crate) use_symmetry: bool,
    pub(crate) reduction_strategy_boundary: bool,
    pub(crate) two_parameters: bool,
    pub(crate) reduction_length_1: f64,
    pub(crate) reduction_length_2: f64,
    pub(crate) linear_area_multiplier: f64,
    pub(crate) polygon_outer: Vec<[f64; 2]>,
    pub(crate) polygon_inners: Vec<Vec<[f64; 2]>>,
    pub(crate) is_x_symmetric: bool,
    pub(crate) is_y_symmetric: bool,
    pub(crate) exposed_perimeter: Vec<bool>,
    pub(crate) exposed_fraction: f64,
    pub(crate) use_detailed_exposed_perimeter: bool,
    pub(crate) building_height: f64,
    pub(crate) building_surfaces: Vec<std::convert::Infallible>,
    pub(crate) wall: Wall,
    pub(crate) has_wall: bool,
    pub(crate) slab: Slab,
    pub(crate) has_slab: bool,
    pub(crate) input_blocks: Vec<InputBlock>,
    pub(crate) perimeter_surface_width: f64,
    pub(crate) has_perimeter_surface: bool,
    pub(crate) mesh: Mesh,
    pub(crate) numerical_scheme_adi: bool,
    pub(crate) f_adi: f64,
    pub(crate) tolerance: f64,
    pub(crate) maximum_iterations: i32,
    pub(crate) derived_mesh: [MeshData; 3],
    pub(crate) blocks: Vec<std::convert::Infallible>,
    pub(crate) surfaces: Vec<std::convert::Infallible>,
    pub(crate) surface_areas: std::collections::BTreeMap<i32, f64>,
    pub(crate) has_surface: std::collections::BTreeMap<i32, bool>,
    pub(crate) net_area: f64,
    pub(crate) net_perimeter: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DeepGroundBoundary {
    FixedTemperature,
    ZeroFlux,
}
impl Default for Foundation {
    fn default() -> Self {
        Self {
            deep_ground_depth: 40.0,
            far_field_width: 40.0,
            foundation_depth: 0.0,
            orientation: 0.0,
            deep_ground_boundary: DeepGroundBoundary::ZeroFlux,
            wall_top_interior_temperature: 0.0,
            wall_top_exterior_temperature: 0.0,
            wall_top_boundary_zero_flux: true,
            soil: Material {
                conductivity: 1.73,
                density: 1842.0,
                specific_heat: 419.0,
            },
            grade: SurfaceProperties {
                emissivity: 0.9,
                absorptivity: 0.9,
                roughness: 0.03,
            },
            coordinate_system_cartesian: true,
            number_of_dimensions: 2,
            use_symmetry: true,
            reduction_strategy_boundary: true,
            two_parameters: false,
            reduction_length_1: 0.0,
            reduction_length_2: 0.0,
            linear_area_multiplier: 0.0,
            polygon_outer: Vec::new(),
            polygon_inners: Vec::new(),
            is_x_symmetric: false,
            is_y_symmetric: false,
            exposed_perimeter: Vec::new(),
            exposed_fraction: 1.0,
            use_detailed_exposed_perimeter: false,
            building_height: 0.0,
            building_surfaces: Vec::new(),
            wall: Wall::default(),
            has_wall: true,
            slab: Slab::default(),
            has_slab: true,
            input_blocks: Vec::new(),
            perimeter_surface_width: 0.0,
            has_perimeter_surface: false,
            mesh: Mesh::default(),
            numerical_scheme_adi: true,
            f_adi: 0.00001,
            tolerance: 1.0e-6,
            maximum_iterations: 100000,
            derived_mesh: std::array::from_fn(|_| MeshData::default()),
            blocks: Vec::new(),
            surfaces: Vec::new(),
            surface_areas: Default::default(),
            has_surface: Default::default(),
            net_area: 0.0,
            net_perimeter: 0.0,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FoundationKiva {
    pub(crate) foundation: Foundation,
    // intHIns/intVIns/extHIns/extVIns/footing, in their actual member order.
    pub(crate) insulation_and_footing: [InputBlock; 5],
    pub(crate) name: String,
    pub(crate) surfaces: Vec<i32>,
    pub(crate) wall_construction_index: i32,
    pub(crate) assumed_indoor_temperature: Option<f64>,
}
impl Default for FoundationKiva {
    fn default() -> Self {
        Self {
            foundation: Foundation::default(),
            insulation_and_footing: std::array::from_fn(|_| InputBlock::default()),
            name: String::new(),
            surfaces: Vec::new(),
            wall_construction_index: 0,
            assumed_indoor_temperature: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FoundationInput {
    pub(crate) settings: Settings,
    pub(crate) default_foundation: FoundationKiva,
    pub(crate) foundations: Vec<std::convert::Infallible>,
    pub(crate) timestep: f64,
    pub(crate) default_added: bool,
    pub(crate) default_index: i32,
}
impl Default for FoundationInput {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            default_foundation: FoundationKiva::default(),
            foundations: Vec::new(),
            timestep: 3600.0,
            default_added: false,
            default_index: 0,
        }
    }
}
impl FoundationInput {
    // Preserve Original defineDefaultFoundation's constructor then member-write order.
    #[allow(clippy::field_reassign_with_default)]
    pub(crate) fn get_zero_input(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        elevation: f64,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "Foundation:Kiva:Settings".into();
        if input.get_num_objects_found(&shortcut.current_module_object) != 0 {
            return Err(Error::Unavailable("positive Foundation:Kiva:Settings"));
        }
        // Actual constructor Settings values, then defineDefaultFoundation1203-1265.
        // No reached Kiva solver/mesh call; these are direct input member writes.
        let mut foundation = Foundation::default();
        foundation.soil = self.settings.soil;
        foundation.grade.absorptivity = self.settings.grade.absorptivity;
        foundation.grade.emissivity = self.settings.grade.emissivity;
        foundation.grade.roughness = self.settings.grade.roughness;
        foundation.far_field_width = self.settings.far_field_width;
        let water_table_depth = 0.1022 * elevation;
        if water_table_depth <= 40.0 {
            foundation.deep_ground_depth = water_table_depth;
            foundation.deep_ground_boundary = DeepGroundBoundary::FixedTemperature;
        } else {
            foundation.deep_ground_depth = 40.0;
            foundation.deep_ground_boundary = DeepGroundBoundary::ZeroFlux;
        }
        // Settings.autocalculateDeepGroundDepth is genuinely constructor true;
        // no user-settings branch is admitted or falsely treated as absent.
        foundation.wall.height_above_grade = Some(0.2);
        foundation.wall.layers.push(Layer {
            thickness: 0.3,
            material: Material {
                conductivity: 1.95,
                density: 2400.0,
                specific_heat: 900.0,
            },
        });
        foundation.wall.interior.emissivity = 0.9;
        foundation.wall.interior.absorptivity = 0.9;
        foundation.wall.exterior.emissivity = 0.9;
        foundation.wall.exterior.absorptivity = 0.9;
        foundation.wall.depth_below_slab = Some(0.0);
        foundation.mesh.minimum_cell_dimension = self.settings.minimum_cell_dimension;
        foundation.mesh.maximum_near_growth = self.settings.maximum_cell_growth;
        foundation.mesh.maximum_depth_growth = self.settings.maximum_cell_growth;
        foundation.mesh.maximum_interior_growth = self.settings.maximum_cell_growth;
        foundation.mesh.maximum_exterior_growth = self.settings.maximum_cell_growth;
        self.default_foundation.foundation = foundation;
        self.default_foundation.name = "<Default Foundation>".into();
        self.default_foundation.assumed_indoor_temperature = Some(-9999.0);
        shortcut.current_module_object = "Foundation:Kiva".into();
        if input.get_num_objects_found(&shortcut.current_module_object) != 0 {
            return Err(Error::Unavailable("positive Foundation:Kiva"));
        }
        Ok(())
    }
}
