//! Full EnclDaylightCalc constructor, reached at positive Controls allocation.
use std::convert::Infallible;
// Full literal EnclDaylightCalc ctor; positive consumers stay unavailable.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceEnclosureDaylight {
    pub(super) average_visible_diffuse_reflectance: f64,
    pub(super) total_inside_surface_area: f64,
    pub(super) floor_visible_reflectance: f64,
    pub(super) total_exterior_windows: i32,
    pub(super) adjacent_interior_window_enclosure_numbers: Option<Vec<i32>>,
    pub(super) number_interior_window_adjacent_enclosures: i32,
    pub(super) number_interior_window_adjacent_enclosure_exterior_windows: i32,
    pub(super) interior_window_adjacent_enclosure_exterior_windows: Option<Vec<Infallible>>,
    pub(super) number_daylighting_exterior_windows: i32,
    pub(super) daylighting_exterior_window_surface_numbers: Option<Vec<i32>>,
    pub(super) adjacent_enclosure_has_daylighting_control: bool,
    pub(super) minimum_interior_window_solid_angle: f64,
    pub(super) inter_reflected_illuminance_from_interior_windows: f64,
    pub(super) has_split_flux_daylighting: bool,
    pub(super) daylight_control_indexes: Option<Vec<i32>>,
}
