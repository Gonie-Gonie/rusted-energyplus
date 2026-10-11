//! Typed ID newtypes for compiled model entities.

macro_rules! typed_id {
    ($name:ident) => {
        #[doc = concat!("Typed ID for ", stringify!($name), ".")]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(pub u32);
    };
}

typed_id!(ZoneId);
typed_id!(ZoneListId);
typed_id!(ZoneGroupId);
typed_id!(ZoneLocalEnvironmentId);
typed_id!(SpaceId);
typed_id!(SpaceListId);
typed_id!(SpaceTypeId);
typed_id!(FenestrationSolarAbsorbedRequestId);
typed_id!(SurfaceId);
typed_id!(SurfaceIncidentSolarMultiplierRequestId);
typed_id!(SurfaceSolarIncidentId);
typed_id!(SurfaceVaporCoefficientsId);
typed_id!(WindowFrameAndDividerId);
typed_id!(ConstructionId);
typed_id!(ThermochromicConstructionChildId);
typed_id!(MaterialId);
typed_id!(GlazingSpectralDataId);
typed_id!(MaterialVariableAbsorptanceId);
typed_id!(MaterialPhaseChangeHysteresisId);
typed_id!(MaterialPhaseChangeId);
typed_id!(MaterialVariableThermalConductivityId);
typed_id!(MaterialMoisturePenetrationDepthSettingsId);
typed_id!(MaterialHeatAndMoistureTransferSettingsId);
typed_id!(MaterialHeatAndMoistureTransferSorptionIsothermId);
typed_id!(MaterialHeatAndMoistureTransferSuctionId);
typed_id!(MaterialHeatAndMoistureTransferRedistributionId);
typed_id!(MaterialHeatAndMoistureTransferDiffusionId);
typed_id!(MaterialHeatAndMoistureTransferThermalConductivityId);
typed_id!(InternalGainId);
typed_id!(ScheduleTypeLimitId);
typed_id!(DayScheduleId);
typed_id!(WeekScheduleId);
typed_id!(ScheduleId);
typed_id!(RunPeriodId);
typed_id!(RunPeriodSpecialDayId);
typed_id!(ThermostatSetpointId);
typed_id!(ZoneThermostatId);
typed_id!(ZoneHumidistatId);
typed_id!(IdealLoadsAirSystemId);
typed_id!(DesignSpecificationOutdoorAirId);
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
