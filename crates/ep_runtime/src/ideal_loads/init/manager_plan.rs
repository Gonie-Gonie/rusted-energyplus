//! Immutable manager-wide inputs for the bounded `InitPurchasedAir` sweep.

use std::collections::BTreeSet;

use ep_model::{IdealLoadsAirSystemId, TypedModel, ZoneEquipmentListId, ZoneEquipmentObjectType};

/// One PurchasedAir row visited by the manager-wide equipment-list sweep.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitManagerPlanRow {
    /// PurchasedAir system in source declaration order.
    pub system: IdealLoadsAirSystemId,
    /// First controlled-Zone-referenced list whose retained entries match.
    pub first_matching_equipment_list: Option<ZoneEquipmentListId>,
    /// Whether a system inlet reaches the still-excluded return-plenum path.
    pub return_plenum_active: bool,
}

/// Immutable declaration-order plan for one manager-wide initialization sweep.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitManagerPlan {
    rows: Vec<PurchasedAirInitManagerPlanRow>,
}

/// Invalid manager plan rejected before PurchasedAir runtime state changes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirInitManagerPlanError {
    /// A typed PurchasedAir system ID occurs more than once.
    DuplicateSystemId {
        /// Repeated typed system.
        system: IdealLoadsAirSystemId,
    },
    /// A system inlet reaches the return-plenum path excluded from this slice.
    ReturnPlenumUnsupported {
        /// System selecting the unsupported topology.
        system: IdealLoadsAirSystemId,
    },
}

impl PurchasedAirInitManagerPlan {
    /// Builds a declaration-order plan from the typed model.
    ///
    /// Equipment-list membership is pre-resolved in typed Zone declaration
    /// order through each Zone's referenced equipment list and that list's
    /// retained entry order. Unreferenced list objects are intentionally
    /// invisible, matching `CheckZoneEquipmentList`.
    pub fn from_model(model: &TypedModel) -> Result<Self, PurchasedAirInitManagerPlanError> {
        let rows = model
            .ideal_loads_air_systems
            .iter()
            .map(|system| PurchasedAirInitManagerPlanRow {
                system: system.id,
                first_matching_equipment_list: first_matching_equipment_list(model, system.id),
                return_plenum_active: system.system_inlet_air_node_name.is_some(),
            })
            .collect();
        Self::try_from_rows(rows)
    }

    /// Validates and retains already-resolved rows without reordering them.
    pub fn try_from_rows(
        rows: Vec<PurchasedAirInitManagerPlanRow>,
    ) -> Result<Self, PurchasedAirInitManagerPlanError> {
        let mut seen = BTreeSet::new();
        for row in &rows {
            if !seen.insert(row.system) {
                return Err(PurchasedAirInitManagerPlanError::DuplicateSystemId {
                    system: row.system,
                });
            }
            if row.return_plenum_active {
                return Err(PurchasedAirInitManagerPlanError::ReturnPlenumUnsupported {
                    system: row.system,
                });
            }
        }
        Ok(Self { rows })
    }

    /// Returns the validated rows in source declaration order.
    #[must_use]
    pub fn rows(&self) -> &[PurchasedAirInitManagerPlanRow] {
        &self.rows
    }

    /// Iterates typed system IDs in source declaration order.
    pub fn system_order(&self) -> impl ExactSizeIterator<Item = IdealLoadsAirSystemId> + '_ {
        self.rows.iter().map(|row| row.system)
    }
}

fn first_matching_equipment_list(
    model: &TypedModel,
    system: IdealLoadsAirSystemId,
) -> Option<ZoneEquipmentListId> {
    for zone in &model.zones {
        let Some(connection) = model
            .zone_equipment_connections
            .iter()
            .find(|connection| connection.zone == zone.id)
        else {
            continue;
        };
        let Some(list) = model
            .zone_equipment_lists
            .iter()
            .find(|list| list.id == connection.equipment_list)
        else {
            continue;
        };
        if list.equipment.iter().any(|entry| {
            entry.object_type == ZoneEquipmentObjectType::IdealLoadsAirSystem
                && entry.ideal_loads_air_system == system
        }) {
            return Some(list.id);
        }
    }
    None
}
