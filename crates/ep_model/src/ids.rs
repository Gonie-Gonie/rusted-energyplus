//! Typed ID newtypes for compiled model entities.

macro_rules! typed_id {
    ($name:ident) => {
        #[doc = concat!("Typed ID for ", stringify!($name), ".")]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(pub u32);
    };
}

typed_id!(ZoneId);
typed_id!(SurfaceId);
typed_id!(ConstructionId);
typed_id!(MaterialId);
typed_id!(InternalGainId);
typed_id!(ScheduleTypeLimitId);
typed_id!(ScheduleId);
typed_id!(RunPeriodId);
typed_id!(ThermostatSetpointId);
typed_id!(ZoneThermostatId);
typed_id!(IdealLoadsAirSystemId);
typed_id!(ZoneEquipmentListId);
typed_id!(ZoneEquipmentConnectionId);
typed_id!(NodeId);
typed_id!(NodeListId);
typed_id!(ComponentId);
typed_id!(LoopId);
typed_id!(BranchId);
typed_id!(BranchListId);
typed_id!(ConnectorId);
typed_id!(ConnectorListId);
typed_id!(OutputHandle);
