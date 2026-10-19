//! Native SystemReports2496-2736 writes on the actual retained EquipData rows.
use super::{OrdinaryEquipment, SourceEquipmentData};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_registration::{
    SourceMeterData, SourceMeterHeatOrCool, SourceMeteredVariable, SourceOutputRegistration,
};
use crate::heat_balance::startup_integrity::{SourceBranchNodeConnections, SourceNodeObjectType};

fn at(ordinal: i32) -> Result<usize, Error> {
    ordinal
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual CreateEnergyReportStructure one-based zone ordinal",
        ))
}
fn component_type(kind: &str) -> Result<SourceNodeObjectType, Error> {
    // Actual admitted EquipmentList producer accepts this one finite type.
    match kind {
        "ZONEHVAC:IDEALLOADSAIRSYSTEM" => Ok(SourceNodeObjectType::ZoneHvacIdealLoadsAirSystem),
        _ => Err(Error::Unavailable(
            "actual CreateEnergyReportStructure2505 getEnumValue component type outside represented domain",
        )),
    }
}

impl OrdinaryEquipment {
    fn energy_report_equipment_mut(
        &mut self,
        zone: i32,
        component: usize,
    ) -> Result<&mut SourceEquipmentData, Error> {
        self.equipment_lists
            .as_mut()
            .and_then(|rows| rows.get_mut(at(zone).ok()?))
            .and_then(|list| list.equipment_data.as_mut())
            .and_then(|rows| component.checked_sub(1).and_then(|i| rows.get_mut(i)))
            .ok_or(Error::Unavailable(
                "actual CreateEnergyReportStructure2520 retained EquipData",
            ))
    }

    pub(crate) fn create_source_zone_equipment_energy_report_structure(
        &mut self,
        number_zones: i32,
        branch: &SourceBranchNodeConnections,
        registration: &SourceOutputRegistration,
    ) -> Result<(), Error> {
        // These are Native GetComponentData output arguments, reused per call;
        // they are neither another registry nor a cached equipment projection.
        let mut is_parent = false;
        let mut number_inlets = 0;
        let mut number_outlets = 0;
        let mut inlet_names = None;
        let mut inlet_numbers = None;
        let mut inlet_streams = None;
        let mut outlet_names = None;
        let mut outlet_numbers = None;
        let mut outlet_streams = None;
        for zone in 1..=number_zones {
            if !*self.source_zone_equipment_config(zone)?.is_controlled() {
                continue;
            }
            let count = *self
                .source_zone_equipment_list(zone)?
                .number_equipment_types();
            for component in 1..=count {
                // Only the two actual string operands are copied to release
                // their borrow before mutating the SAME parallel EquipData row.
                let kind = self
                    .source_zone_equipment_list(zone)?
                    .equipment_type_name(component)
                    .ok_or(Error::Unavailable(
                        "actual CreateEnergyReportStructure2503 EquipTypeName",
                    ))?
                    .to_owned();
                let name = self
                    .source_zone_equipment_list(zone)?
                    .equipment_name(component)
                    .ok_or(Error::Unavailable(
                        "actual CreateEnergyReportStructure2504 EquipName",
                    ))?
                    .to_owned();
                let object_type = component_type(&kind)?;
                branch.get_source_component_data(
                    object_type,
                    &name,
                    &mut is_parent,
                    &mut number_inlets,
                    &mut inlet_names,
                    &mut inlet_numbers,
                    &mut inlet_streams,
                    &mut number_outlets,
                    &mut outlet_names,
                    &mut outlet_numbers,
                    &mut outlet_streams,
                )?;
                let row = self.energy_report_equipment_mut(zone, component)?;
                row.type_of = kind.clone();
                row.name = name.clone();
                let outlets = usize::try_from(number_outlets).map_err(|_| {
                    Error::Unavailable("actual CreateEnergyReportStructure2523 NumOutlets")
                })?;
                row.outlet_node_numbers = Some(vec![0; outlets]);
                row.number_outlets = number_outlets;
                // Native Array1D assignment copies the complete actual array.
                row.outlet_node_numbers = Some(
                    outlet_numbers
                        .as_ref()
                        .ok_or(Error::Unavailable(
                            "actual CreateEnergyReportStructure2525 OutletNodeNumbers",
                        ))?
                        .clone(),
                );
                let inlets = usize::try_from(number_inlets).map_err(|_| {
                    Error::Unavailable("actual CreateEnergyReportStructure2526 NumInlets")
                })?;
                row.inlet_node_numbers = Some(vec![0; inlets]);
                row.number_inlets = number_inlets;
                row.inlet_node_numbers = Some(
                    inlet_numbers
                        .as_ref()
                        .ok_or(Error::Unavailable(
                            "actual CreateEnergyReportStructure2528 InletNodeNumbers",
                        ))?
                        .clone(),
                );
                row.parent = is_parent;
                let number_variables =
                    registration.get_source_number_metered_variables(&kind, &name)?;
                row.number_metered_variables = number_variables;
                if number_variables > 0 {
                    inlet_names = None;
                    inlet_numbers = None;
                    inlet_streams = None;
                    outlet_names = None;
                    outlet_numbers = None;
                    outlet_streams = None;
                    let size = usize::try_from(number_variables).map_err(|_| {
                        Error::Unavailable("actual CreateEnergyReportStructure2540 NumVariables")
                    })?;
                    let mut variables = vec![SourceMeteredVariable::default(); size];
                    row.metered_variables = Some(vec![SourceMeterData::default(); size]);
                    let _number_copied =
                        registration.get_source_metered_variables(&name, &mut variables)?;
                    let mut mode_flag_on = true;
                    for index in 0..size {
                        // Native MeterData::operator=(MeteredVar) updates only
                        // its base; preserve both existing derived members.
                        let metered = row
                            .metered_variables
                            .as_mut()
                            .and_then(|rows| rows.get_mut(index))
                            .ok_or(Error::Unavailable(
                                "actual CreateEnergyReportStructure2548 MeteredVar",
                            ))?;
                        metered.variable = variables
                            .get(index)
                            .ok_or(Error::Unavailable(
                                "actual CreateEnergyReportStructure2548 local meteredVars",
                            ))?
                            .clone();
                        let category = metered.variable.end_use_category as i32;
                        // Actual OutputProcessor EndUseCat HeatingCoils33 and
                        // CoolingCoils22, distinct from Heating0/Cooling1.
                        if category == 33 && mode_flag_on {
                            for variable in
                                row.metered_variables.as_mut().ok_or(Error::Unavailable(
                                    "actual CreateEnergyReportStructure2552 MeteredVar allocation",
                                ))?
                            {
                                variable.heat_or_cool = SourceMeterHeatOrCool::HeatingOnly;
                            }
                            mode_flag_on = false;
                        } else if category == 22 && mode_flag_on {
                            for variable in
                                row.metered_variables.as_mut().ok_or(Error::Unavailable(
                                    "actual CreateEnergyReportStructure2557 MeteredVar allocation",
                                ))?
                            {
                                variable.heat_or_cool = SourceMeterHeatOrCool::CoolingOnly;
                            }
                            mode_flag_on = false;
                        } else if mode_flag_on {
                            row.metered_variables
                                .as_mut()
                                .and_then(|rows| rows.get_mut(index))
                                .ok_or(Error::Unavailable(
                                    "actual CreateEnergyReportStructure2561 MeteredVar",
                                ))?
                                .heat_or_cool = SourceMeterHeatOrCool::NoHeatNoCool;
                        }
                    }
                    drop(variables);
                }
                if branch.is_source_parent_object(object_type, &name)? {
                    return Err(Error::Unavailable(
                        "actual CreateEnergyReportStructure2570 GetNumChildren parent child",
                    ));
                }
                // The false branch changes only local NumChildren. It does
                // not reset this retained row's real NumSubEquip constructor.
                let number_children = 0_i32;
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _child in 1..=number_children {
                    return Err(Error::Unavailable(
                        "actual CreateEnergyReportStructure2611 SubEquipData.TypeOf",
                    ));
                }
            }
        }
        for zone in 1..=number_zones {
            if !*self.source_zone_equipment_config(zone)?.is_controlled() {
                continue;
            }
            let count = *self
                .source_zone_equipment_list(zone)?
                .number_equipment_types();
            for component in 1..=count {
                let number_children = self
                    .energy_report_equipment_mut(zone, component)?
                    .number_sub_equipment;
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _child in 1..=number_children {
                    return Err(Error::Unavailable(
                        "actual CreateEnergyReportStructure2665 SubEquipData retained row",
                    ));
                }
            }
        }
        Ok(())
    }
}
