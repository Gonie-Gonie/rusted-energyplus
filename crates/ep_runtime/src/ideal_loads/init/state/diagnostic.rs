//! Manager-wide retained initialization diagnostics.

use ep_model::IdealLoadsAirSystemId;

/// Structured diagnostic emitted by the manager-wide equipment-list sweep.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitDiagnostic {
    /// Unit visited by the manager sweep.
    pub system: IdealLoadsAirSystemId,
    /// One-based declaration-order visit ordinal.
    pub scan_ordinal: usize,
    /// Source-shaped diagnostic category.
    pub kind: PurchasedAirInitDiagnosticKind,
}

/// Diagnostic categories retained by the bounded manager-wide sweep.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirInitDiagnosticKind {
    /// `CheckZoneEquipmentList` found no matching entry in any equipment list.
    EquipmentListMembershipMissing,
}
