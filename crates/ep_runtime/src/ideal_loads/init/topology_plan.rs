//! Immutable per-unit topology inputs for the `InitPurchasedAir` one-time pass.

use ep_model::{
    IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit, NodeId, NormalizedName,
    OutdoorAirEconomizerType, TypedModel, ZoneEquipmentListId, ZoneId,
};

/// Node relation resolved while building the immutable topology plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirInitTopologyRelation {
    /// `ZoneHVAC:EquipmentConnections` selected by `ControlledZoneNum`.
    ZoneEquipmentConnection,
    /// `Zone Supply Air Node Name` from the IdealLoads object.
    ZoneSupplyAirNode,
    /// Optional `Zone Exhaust Air Node Name` from the IdealLoads object.
    ZoneExhaustAirNode,
}

/// Invalid typed topology rejected before the one-time runtime transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirInitTopologyPlanError {
    /// The selected IdealLoads system is absent from the typed arena.
    SystemMissing {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
    },
    /// A required relation did not resolve to exactly one retained object.
    Cardinality {
        /// Relation whose cardinality is invalid.
        relation: PurchasedAirInitTopologyRelation,
        /// Selected controlled Zone.
        controlled_zone: ZoneId,
        /// Required cardinality.
        expected: usize,
        /// Actual retained cardinality.
        actual: usize,
    },
}

/// Source-shaped recirculation selection after the one-time topology pass.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirRecirculationSource {
    /// A configured IdealLoads exhaust node belongs to the controlled Zone.
    ConfiguredZoneExhaust,
    /// Blank or invalid IdealLoads exhaust falls back to one Zone return node.
    SingleZoneReturn,
    /// EnergyPlus warns that it will use the first of multiple return nodes but
    /// leaves `ZoneRecircAirNodeNum` unassigned in this source branch.
    MultipleZoneReturnsUnassigned,
}

/// Successful result of the source-shaped one-time topology pass.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitTopologyOutcome {
    /// Recirculation node assigned by the source branch, when any.
    pub recirculation_node: Option<NodeId>,
    /// Branch that selected or intentionally left the node unassigned.
    pub recirculation_source: PurchasedAirRecirculationSource,
    /// Configured exhaust rejected because it is not a Zone exhaust node.
    pub rejected_exhaust_node: Option<NodeId>,
    /// First return node named by the multiple-return warning.
    pub reported_first_return_node: Option<NodeId>,
}

/// Source-shaped one-time topology failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirInitTopologyError {
    /// The IdealLoads supply node is not in the controlled Zone inlet arena.
    SupplyNodeNotZoneInlet {
        /// Selected system.
        system: IdealLoadsAirSystemId,
        /// Selected controlled Zone.
        controlled_zone: ZoneId,
        /// Invalid supply node.
        supply_node: NodeId,
    },
    /// Neither a valid exhaust node nor any Zone return node is available.
    NoRecirculationNode {
        /// Selected system.
        system: IdealLoadsAirSystemId,
        /// Selected controlled Zone.
        controlled_zone: ZoneId,
        /// Configured exhaust rejected before the return-node fatal branch.
        rejected_exhaust_node: Option<NodeId>,
    },
}

/// Severity retained for one source-shaped topology diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirInitTopologyDiagnosticSeverity {
    /// Nonfatal source `ShowSevereError` branch.
    Severe,
    /// Nonfatal source `ShowWarningError` branch.
    Warning,
    /// Fatal topology branch coalesced into one structured outcome.
    Fatal,
}

/// Diagnostic category emitted during the selected-unit one-time block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirInitTopologyDiagnosticKind {
    /// Supply node is absent from the selected controlled Zone inlet arena.
    SupplyNodeNotZoneInlet,
    /// Configured exhaust node is absent from the selected Zone exhaust arena.
    ExhaustNodeNotZoneExhaust,
    /// Multiple returns trigger a warning but leave recirculation unassigned.
    MultipleReturnNodesUnassigned,
    /// No valid exhaust or return node exists for recirculation.
    NoRecirculationNode,
    /// OA economizer is active without a cooling flow-rate limit.
    EconomizerWithoutCoolingFlowLimit,
}

/// Ordered structured diagnostic from the selected-unit one-time block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitTopologyDiagnostic {
    /// Selected system.
    pub system: IdealLoadsAirSystemId,
    /// One-based emission ordinal within this unit's one-time block.
    pub ordinal: usize,
    /// Source severity class.
    pub severity: PurchasedAirInitTopologyDiagnosticSeverity,
    /// Source branch represented by this diagnostic.
    pub kind: PurchasedAirInitTopologyDiagnosticKind,
}

/// Pure source-order evaluation performed after the runtime latch commits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitTopologyEvaluation {
    /// Successful topology outcome or fatal semantic branch.
    pub outcome: Result<PurchasedAirInitTopologyOutcome, PurchasedAirInitTopologyError>,
    /// Diagnostics in exact branch order up to the outcome.
    pub diagnostics: Vec<PurchasedAirInitTopologyDiagnostic>,
    /// Whether the nonfatal OA/economizer advisory was emitted.
    pub economizer_flow_limit_warning: bool,
}

/// Immutable resolved inputs for one per-unit topology pass.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitTopologyPlan {
    system: IdealLoadsAirSystemId,
    controlled_zone: ZoneId,
    equipment_list: ZoneEquipmentListId,
    supply_node: NodeId,
    zone_inlet_nodes: Vec<NodeId>,
    configured_exhaust_node: Option<NodeId>,
    zone_exhaust_nodes: Vec<NodeId>,
    zone_return_nodes: Vec<NodeId>,
    outdoor_air_resolved: bool,
}

impl PurchasedAirInitTopologyPlan {
    /// Resolves the per-unit one-time topology from the retained typed model.
    pub fn from_model(
        model: &TypedModel,
        system: IdealLoadsAirSystemId,
        controlled_zone: ZoneId,
    ) -> Result<Self, PurchasedAirInitTopologyPlanError> {
        let system_object = model
            .ideal_loads_air_systems
            .iter()
            .find(|candidate| candidate.id == system)
            .ok_or(PurchasedAirInitTopologyPlanError::SystemMissing { system })?;
        let connections: Vec<_> = model
            .zone_equipment_connections
            .iter()
            .filter(|connection| connection.zone == controlled_zone)
            .collect();
        if connections.len() != 1 {
            return Err(PurchasedAirInitTopologyPlanError::Cardinality {
                relation: PurchasedAirInitTopologyRelation::ZoneEquipmentConnection,
                controlled_zone,
                expected: 1,
                actual: connections.len(),
            });
        }
        let connection = connections[0];
        let supply_node = resolve_exactly_one(
            model,
            &system_object.zone_supply_air_node_name,
            PurchasedAirInitTopologyRelation::ZoneSupplyAirNode,
            controlled_zone,
        )?;
        let configured_exhaust_node = system_object
            .zone_exhaust_air_node_name
            .as_ref()
            .map(|name| {
                resolve_exactly_one(
                    model,
                    name,
                    PurchasedAirInitTopologyRelation::ZoneExhaustAirNode,
                    controlled_zone,
                )
            })
            .transpose()?;
        Ok(Self::from_resolved_nodes(
            system,
            controlled_zone,
            connection.equipment_list,
            supply_node,
            resolve_optional_nodes(
                model,
                connection.zone_air_inlet_node_or_nodelist_name.as_ref(),
            ),
            configured_exhaust_node,
            resolve_optional_nodes(
                model,
                connection.zone_air_exhaust_node_or_nodelist_name.as_ref(),
            ),
            resolve_optional_nodes(
                model,
                connection.zone_return_air_node_or_nodelist_name.as_ref(),
            ),
            system_object
                .design_specification_outdoor_air_object_name
                .as_ref()
                .and_then(|name| {
                    model
                        .design_specification_outdoor_air_names
                        .resolve(&name.0)
                })
                .is_some(),
        ))
    }

    /// Retains already-resolved typed nodes without changing their order.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_resolved_nodes(
        system: IdealLoadsAirSystemId,
        controlled_zone: ZoneId,
        equipment_list: ZoneEquipmentListId,
        // The typed Rust subset requires a resolved supply node. C++'s
        // nonpositive integer sentinel bypass remains outside this plan.
        supply_node: NodeId,
        zone_inlet_nodes: Vec<NodeId>,
        configured_exhaust_node: Option<NodeId>,
        zone_exhaust_nodes: Vec<NodeId>,
        zone_return_nodes: Vec<NodeId>,
        outdoor_air_resolved: bool,
    ) -> Self {
        Self {
            system,
            controlled_zone,
            equipment_list,
            supply_node,
            zone_inlet_nodes,
            configured_exhaust_node,
            zone_exhaust_nodes,
            zone_return_nodes,
            outdoor_air_resolved,
        }
    }

    /// Applies the source one-time supply and recirculation selection order.
    pub fn resolve(
        &self,
    ) -> Result<PurchasedAirInitTopologyOutcome, PurchasedAirInitTopologyError> {
        if !self.zone_inlet_nodes.contains(&self.supply_node) {
            return Err(PurchasedAirInitTopologyError::SupplyNodeNotZoneInlet {
                system: self.system,
                controlled_zone: self.controlled_zone,
                supply_node: self.supply_node,
            });
        }
        if let Some(exhaust_node) = self.configured_exhaust_node
            && self.zone_exhaust_nodes.contains(&exhaust_node)
        {
            return Ok(PurchasedAirInitTopologyOutcome {
                recirculation_node: Some(exhaust_node),
                recirculation_source: PurchasedAirRecirculationSource::ConfiguredZoneExhaust,
                rejected_exhaust_node: None,
                reported_first_return_node: None,
            });
        }
        let rejected_exhaust_node = self.configured_exhaust_node;
        match self.zone_return_nodes.as_slice() {
            [return_node] => Ok(PurchasedAirInitTopologyOutcome {
                recirculation_node: Some(*return_node),
                recirculation_source: PurchasedAirRecirculationSource::SingleZoneReturn,
                rejected_exhaust_node,
                reported_first_return_node: None,
            }),
            [first_return_node, ..] => Ok(PurchasedAirInitTopologyOutcome {
                recirculation_node: None,
                recirculation_source:
                    PurchasedAirRecirculationSource::MultipleZoneReturnsUnassigned,
                rejected_exhaust_node,
                reported_first_return_node: Some(*first_return_node),
            }),
            [] => Err(PurchasedAirInitTopologyError::NoRecirculationNode {
                system: self.system,
                controlled_zone: self.controlled_zone,
                rejected_exhaust_node,
            }),
        }
    }

    /// Evaluates topology diagnostics and the following economizer advisory.
    #[must_use]
    pub(crate) fn evaluate(
        &self,
        system: &IdealLoadsAirSystem,
    ) -> PurchasedAirInitTopologyEvaluation {
        let outcome = self.resolve();
        let mut diagnostics = Vec::new();
        match outcome {
            Ok(outcome) => {
                if outcome.rejected_exhaust_node.is_some() {
                    push_diagnostic(
                        &mut diagnostics,
                        self.system,
                        PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                        PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust,
                    );
                }
                if outcome.recirculation_source
                    == PurchasedAirRecirculationSource::MultipleZoneReturnsUnassigned
                {
                    push_diagnostic(
                        &mut diagnostics,
                        self.system,
                        PurchasedAirInitTopologyDiagnosticSeverity::Warning,
                        PurchasedAirInitTopologyDiagnosticKind::MultipleReturnNodesUnassigned,
                    );
                }
            }
            Err(PurchasedAirInitTopologyError::SupplyNodeNotZoneInlet { .. }) => push_diagnostic(
                &mut diagnostics,
                self.system,
                PurchasedAirInitTopologyDiagnosticSeverity::Fatal,
                PurchasedAirInitTopologyDiagnosticKind::SupplyNodeNotZoneInlet,
            ),
            Err(PurchasedAirInitTopologyError::NoRecirculationNode {
                rejected_exhaust_node,
                ..
            }) => {
                if rejected_exhaust_node.is_some() {
                    push_diagnostic(
                        &mut diagnostics,
                        self.system,
                        PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                        PurchasedAirInitTopologyDiagnosticKind::ExhaustNodeNotZoneExhaust,
                    );
                }
                push_diagnostic(
                    &mut diagnostics,
                    self.system,
                    PurchasedAirInitTopologyDiagnosticSeverity::Fatal,
                    PurchasedAirInitTopologyDiagnosticKind::NoRecirculationNode,
                );
            }
        }
        let economizer_flow_limit_warning = outcome.is_ok()
            && self.outdoor_air_resolved
            && system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
            && matches!(
                system.cooling_limit,
                IdealLoadsLimit::NoLimit | IdealLoadsLimit::LimitCapacity
            );
        if economizer_flow_limit_warning {
            push_diagnostic(
                &mut diagnostics,
                self.system,
                PurchasedAirInitTopologyDiagnosticSeverity::Severe,
                PurchasedAirInitTopologyDiagnosticKind::EconomizerWithoutCoolingFlowLimit,
            );
        }
        PurchasedAirInitTopologyEvaluation {
            outcome,
            diagnostics,
            economizer_flow_limit_warning,
        }
    }

    /// Selected system identity.
    #[must_use]
    pub const fn system(&self) -> IdealLoadsAirSystemId {
        self.system
    }

    /// Selected controlled Zone.
    #[must_use]
    pub const fn controlled_zone(&self) -> ZoneId {
        self.controlled_zone
    }

    /// Controlled Zone equipment list.
    #[must_use]
    pub const fn equipment_list(&self) -> ZoneEquipmentListId {
        self.equipment_list
    }

    /// IdealLoads supply node.
    #[must_use]
    pub const fn supply_node(&self) -> NodeId {
        self.supply_node
    }

    /// Whether `GetPurchasedAir` resolved a DesignSpecification:OutdoorAir edge.
    #[must_use]
    pub const fn outdoor_air_resolved(&self) -> bool {
        self.outdoor_air_resolved
    }
}

fn resolve_exactly_one(
    model: &TypedModel,
    name: &NormalizedName,
    relation: PurchasedAirInitTopologyRelation,
    controlled_zone: ZoneId,
) -> Result<NodeId, PurchasedAirInitTopologyPlanError> {
    let nodes = resolve_nodes(model, name);
    if nodes.len() != 1 {
        return Err(PurchasedAirInitTopologyPlanError::Cardinality {
            relation,
            controlled_zone,
            expected: 1,
            actual: nodes.len(),
        });
    }
    Ok(nodes[0])
}

fn resolve_optional_nodes(model: &TypedModel, name: Option<&NormalizedName>) -> Vec<NodeId> {
    name.map_or_else(Vec::new, |name| resolve_nodes(model, name))
}

fn resolve_nodes(model: &TypedModel, name: &NormalizedName) -> Vec<NodeId> {
    if let Some(node) = model.node_names.resolve(&name.0) {
        return vec![node];
    }
    model
        .node_list_names
        .resolve(&name.0)
        .and_then(|node_list| model.node_lists.iter().find(|list| list.id == node_list))
        .map_or_else(Vec::new, |list| list.nodes.clone())
}

fn push_diagnostic(
    diagnostics: &mut Vec<PurchasedAirInitTopologyDiagnostic>,
    system: IdealLoadsAirSystemId,
    severity: PurchasedAirInitTopologyDiagnosticSeverity,
    kind: PurchasedAirInitTopologyDiagnosticKind,
) {
    diagnostics.push(PurchasedAirInitTopologyDiagnostic {
        system,
        ordinal: diagnostics.len() + 1,
        severity,
        kind,
    });
}
