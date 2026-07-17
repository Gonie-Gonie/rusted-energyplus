//! User-declared window frame, divider, and reveal properties.

use crate::{NormalizedName, WindowFrameAndDividerId};

/// EnergyPlus window-divider construction type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowDividerType {
    /// `DividedLite`.
    DividedLite,
    /// `Suspended`.
    Suspended,
}

impl WindowDividerType {
    /// Parses an EnergyPlus divider-type token case-insensitively.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "DIVIDEDLITE" => Some(Self::DividedLite),
            "SUSPENDED" => Some(Self::Suspended),
            _ => None,
        }
    }
}

/// NFRC product category used for window-assembly reporting calculations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowNfrcProductType {
    /// `CasementDouble`.
    CasementDouble,
    /// `CasementSingle`.
    CasementSingle,
    /// `DualAction`.
    DualAction,
    /// `Fixed`.
    Fixed,
    /// `Garage`.
    Garage,
    /// `Greenhouse`.
    Greenhouse,
    /// `HingedEscape`.
    HingedEscape,
    /// `HorizontalSlider`.
    HorizontalSlider,
    /// `Jal`.
    Jal,
    /// `Pivoted`.
    Pivoted,
    /// `ProjectingSingle`.
    ProjectingSingle,
    /// `ProjectingDual`.
    ProjectingDual,
    /// `DoorSidelite`.
    DoorSidelite,
    /// `Skylight`.
    Skylight,
    /// `SlidingPatioDoor`.
    SlidingPatioDoor,
    /// `CurtainWall`.
    CurtainWall,
    /// `SpandrelPanel`.
    SpandrelPanel,
    /// `SideHingedDoor`.
    SideHingedDoor,
    /// `DoorTransom`.
    DoorTransom,
    /// `TropicalAwning`.
    TropicalAwning,
    /// `TubularDaylightingDevice`.
    TubularDaylightingDevice,
    /// `VerticalSlider`.
    VerticalSlider,
}

impl WindowNfrcProductType {
    /// Parses an EnergyPlus NFRC product-type token case-insensitively.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "CASEMENTDOUBLE" => Some(Self::CasementDouble),
            "CASEMENTSINGLE" => Some(Self::CasementSingle),
            "DUALACTION" => Some(Self::DualAction),
            "FIXED" => Some(Self::Fixed),
            "GARAGE" => Some(Self::Garage),
            "GREENHOUSE" => Some(Self::Greenhouse),
            "HINGEDESCAPE" => Some(Self::HingedEscape),
            "HORIZONTALSLIDER" => Some(Self::HorizontalSlider),
            "JAL" => Some(Self::Jal),
            "PIVOTED" => Some(Self::Pivoted),
            "PROJECTINGSINGLE" => Some(Self::ProjectingSingle),
            "PROJECTINGDUAL" => Some(Self::ProjectingDual),
            "DOORSIDELITE" => Some(Self::DoorSidelite),
            "SKYLIGHT" => Some(Self::Skylight),
            "SLIDINGPATIODOOR" => Some(Self::SlidingPatioDoor),
            "CURTAINWALL" => Some(Self::CurtainWall),
            "SPANDRELPANEL" => Some(Self::SpandrelPanel),
            "SIDEHINGEDDOOR" => Some(Self::SideHingedDoor),
            "DOORTRANSOM" => Some(Self::DoorTransom),
            "TROPICALAWNING" => Some(Self::TropicalAwning),
            "TUBULARDAYLIGHTINGDEVICE" => Some(Self::TubularDaylightingDevice),
            "VERTICALSLIDER" => Some(Self::VerticalSlider),
            _ => None,
        }
    }
}

/// Source-effective frame properties for one window frame/divider definition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowFrameProperties {
    /// Frame width in the plane of the window.
    pub width_m: f64,
    /// Projection from the outside face of the glazing.
    pub outside_projection_m: f64,
    /// Projection from the inside face of the glazing.
    pub inside_projection_m: f64,
    /// Effective frame conductance excluding air films.
    pub conductance_w_per_m2_k: f64,
    /// Frame-edge to center-of-glass conductance ratio.
    pub edge_to_center_glass_conductance_ratio: f64,
    /// Frame solar absorptance.
    pub solar_absorptance: f64,
    /// Frame visible absorptance.
    pub visible_absorptance: f64,
    /// Frame thermal hemispherical emissivity.
    pub thermal_hemispherical_emissivity: f64,
    /// Source-fixed frame edge width.
    pub edge_width_m: f64,
}

/// Source-effective divider properties for one window frame/divider definition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowDividerProperties {
    /// Divider construction type.
    pub divider_type: WindowDividerType,
    /// Divider width in the plane of the window.
    pub width_m: f64,
    /// Number of dividers parallel to the local window X-axis.
    pub horizontal_count: u32,
    /// Number of dividers parallel to the local window Y-axis.
    pub vertical_count: u32,
    /// Projection from the outside face of the glazing.
    pub outside_projection_m: f64,
    /// Projection from the inside face of the glazing.
    pub inside_projection_m: f64,
    /// Effective divider conductance excluding air films.
    pub conductance_w_per_m2_k: f64,
    /// Divider-edge to center-of-glass conductance ratio.
    pub edge_to_center_glass_conductance_ratio: f64,
    /// Divider solar absorptance.
    pub solar_absorptance: f64,
    /// Divider visible absorptance.
    pub visible_absorptance: f64,
    /// Divider thermal hemispherical emissivity.
    pub thermal_hemispherical_emissivity: f64,
    /// Source-fixed divider edge width.
    pub edge_width_m: f64,
}

/// Source-effective reveal and sill properties for one window definition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowRevealProperties {
    /// Outside reveal solar absorptance.
    pub outside_solar_absorptance: f64,
    /// Inside sill depth from the glazing plane.
    pub inside_sill_depth_m: f64,
    /// Inside sill solar absorptance.
    pub inside_sill_solar_absorptance: f64,
    /// Inside reveal depth from the glazing plane.
    pub inside_reveal_depth_m: f64,
    /// Inside reveal solar absorptance.
    pub inside_reveal_solar_absorptance: f64,
}

/// Typed source-effective `WindowProperty:FrameAndDivider` user object.
///
/// WINDOW 5 mullion orientation and synthesized frame/divider records are not
/// represented by this user-object type.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowFrameAndDivider {
    /// Typed ID in the independent frame-and-divider namespace.
    pub id: WindowFrameAndDividerId,
    /// Normalized object name.
    pub name: NormalizedName,
    /// Effective frame properties after source corrections.
    pub frame: WindowFrameProperties,
    /// Effective divider properties after source corrections.
    pub divider: WindowDividerProperties,
    /// Effective reveal and sill properties after source corrections.
    pub reveal: WindowRevealProperties,
    /// NFRC product category used for reporting calculations.
    pub nfrc_product_type: WindowNfrcProductType,
}

#[cfg(test)]
mod tests {
    use super::{WindowDividerType, WindowNfrcProductType};

    #[test]
    fn divider_type_parser_is_case_insensitive() {
        assert_eq!(
            WindowDividerType::from_energyplus_name(" dividedLITE "),
            Some(WindowDividerType::DividedLite)
        );
        assert_eq!(
            WindowDividerType::from_energyplus_name("SUSPENDED"),
            Some(WindowDividerType::Suspended)
        );
        assert_eq!(WindowDividerType::from_energyplus_name("unknown"), None);
    }

    #[test]
    fn nfrc_product_type_parser_covers_all_energyplus_tokens() {
        let tokens = [
            "CasementDouble",
            "CasementSingle",
            "DualAction",
            "Fixed",
            "Garage",
            "Greenhouse",
            "HingedEscape",
            "HorizontalSlider",
            "Jal",
            "Pivoted",
            "ProjectingSingle",
            "ProjectingDual",
            "DoorSidelite",
            "Skylight",
            "SlidingPatioDoor",
            "CurtainWall",
            "SpandrelPanel",
            "SideHingedDoor",
            "DoorTransom",
            "TropicalAwning",
            "TubularDaylightingDevice",
            "VerticalSlider",
        ];

        for token in tokens {
            assert!(WindowNfrcProductType::from_energyplus_name(token).is_some());
        }
        assert_eq!(
            WindowNfrcProductType::from_energyplus_name(" curtainwall "),
            Some(WindowNfrcProductType::CurtainWall)
        );
        assert_eq!(WindowNfrcProductType::from_energyplus_name("unknown"), None);
    }
}
