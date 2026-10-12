//! Sole DataHeatBalance space controls, before GetProjectControl is reached.
//! The later getter borrows these same fields; input switches never create copies.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SelectedHeatBalanceSpaceControls {
    pub(crate) sizing: bool,
    pub(crate) simulation: bool,
    pub(crate) current: bool,
}
