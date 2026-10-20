//! Material.hh74-98 group identity, written by actual selected input getters.
use super::super::selected_heat_balance_input::HeatBalanceInputError;
use super::SelectedMaterialData;

// Preserve the native enum membership/numbers for the actual predicates. The
// selected getter writes Regular and AirGap; other producers remain unsupported.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceMaterialGroup {
    #[default]
    Invalid = -1,
    Regular,
    AirGap,
    Shade,
    Glass,
    Gas,
    Blind,
    GasMixture,
    Screen,
    EcoRoof,
    IrTransparent,
    GlassSimple,
    ComplexShade,
    ComplexWindowGap,
    GlassEql,
    ShadeEql,
    DrapeEql,
    BlindEql,
    ScreenEql,
    WindowGapEql,
    GlassTcParent,
}

impl SourceMaterialGroup {
    pub(crate) fn is_window(self) -> bool {
        matches!(
            self,
            Self::Glass
                | Self::Gas
                | Self::GasMixture
                | Self::Shade
                | Self::Blind
                | Self::Screen
                | Self::GlassSimple
                | Self::ComplexShade
                | Self::ComplexWindowGap
                | Self::GlassEql
                | Self::ShadeEql
                | Self::DrapeEql
                | Self::ScreenEql
                | Self::BlindEql
                | Self::WindowGapEql
        )
    }
    pub(crate) fn is_not_window(self) -> bool {
        matches!(
            self,
            Self::Invalid | Self::AirGap | Self::Regular | Self::EcoRoof | Self::IrTransparent
        )
    }
}

impl SelectedMaterialData {
    pub(crate) fn source_group_at(
        &self,
        number: i32,
    ) -> Result<SourceMaterialGroup, HeatBalanceInputError> {
        let index = number
            .checked_sub(1)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or(HeatBalanceInputError::Unavailable(
                "native MaterialNum bounds",
            ))?;
        let id = self
            .materials
            .get(index)
            .ok_or(HeatBalanceInputError::Unavailable(
                "native material-order bounds",
            ))?;
        self.material_groups
            .get(id)
            .copied()
            .ok_or(HeatBalanceInputError::Unavailable(
                "actual Material group writer",
            ))
    }
}
