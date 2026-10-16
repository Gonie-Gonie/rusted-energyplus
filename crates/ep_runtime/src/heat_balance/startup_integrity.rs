//! Genuine finite BranchInputManager and GeneralRoutines startup integrity children.
//! The same caller ErrFound, native module counts, input, backend and BND are borrowed.
mod air_path;
mod branch;
mod mixed_air;
mod nodes;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
pub(crate) use air_path::test_source_air_path_integrity;
pub(crate) use mixed_air::SourceMixedAir;
pub(crate) use nodes::SourceBranchNodeConnections;

// These are independent real native module slices, not Branch connector counts.
// Their positive input writers remain unavailable at the actual child call.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSplitterComponentCounts {
    pub(crate) number_splitters: i32,
}
impl Default for SourceSplitterComponentCounts {
    fn default() -> Self {
        Self {
            number_splitters: 0,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceMixerComponentCounts {
    pub(crate) number_mixers: i32,
}
impl Default for SourceMixerComponentCounts {
    fn default() -> Self {
        Self { number_mixers: 0 }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceZonePlenumCounts {
    pub(crate) number_return_plenums: i32,
    pub(crate) number_supply_plenums: i32,
}
impl Default for SourceZonePlenumCounts {
    fn default() -> Self {
        Self {
            number_return_plenums: 0,
            number_supply_plenums: 0,
        }
    }
}
fn extent(number: i32) -> Result<usize, Error> {
    usize::try_from(number).map_err(|_| {
        Error::Unavailable("actual startup integrity nonnegative native allocation extent")
    })
}
fn node_extent(number: usize) -> Result<usize, Error> {
    i32::try_from(number)
        .map_err(|_| Error::Unavailable("actual startup integrity NumOfNodes signed i32 domain"))?;
    Ok(number)
}
