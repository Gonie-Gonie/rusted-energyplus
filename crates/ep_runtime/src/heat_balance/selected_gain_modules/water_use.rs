//! Same native WaterUseData owner, excluding its already retained numWaterEquipment field.
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use std::convert::Infallible;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceWaterUseModule {
    pub(crate) number_connections: i32,
    pub(crate) get_input_flag: bool,
    pub(crate) calculate_water_density: bool,
    pub(crate) environment_flag: bool,
    pub(crate) standard_water_density: f64,
    pub(crate) check_equipment_name: Option<Vec<bool>>,
    pub(crate) equipment: Option<Vec<Infallible>>,
    pub(crate) connections: Option<Vec<Infallible>>,
}
impl Default for SourceWaterUseModule {
    fn default() -> Self {
        Self {
            number_connections: 0,
            get_input_flag: true,
            calculate_water_density: true,
            environment_flag: true,
            standard_water_density: 1000.0,
            check_equipment_name: None,
            equipment: None,
            connections: None,
        }
    }
}
impl SourceWaterUseModule {
    pub(crate) fn simulate_source_water_use(
        &mut self,
        _first_hvac_iteration: bool,
        number_equipment: &mut i32,
        begin_environment: bool,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_input(number_equipment, input, shortcut)?;
            self.get_input_flag = false;
        }
        if begin_environment && self.environment_flag {
            if *number_equipment > 0 {
                if let Some(rows) = self.equipment.as_ref() {
                    // This real owner has uninhabited rows; preserve its literal traversal instead of inventing a populated row or count.
                    #[allow(clippy::never_loop)]
                    for row in rows {
                        match *row {}
                    }
                }
            }
            if self.number_connections > 0 {
                if let Some(rows) = self.connections.as_ref() {
                    // This real owner has uninhabited rows; preserve its literal traversal instead of inventing a populated row or count.
                    #[allow(clippy::never_loop)]
                    for row in rows {
                        match *row {}
                    }
                }
            }
            self.environment_flag = false;
        }
        if !begin_environment {
            self.environment_flag = true;
        }
        if let Some(rows) = self.equipment.as_ref() {
            // This real owner has uninhabited rows; preserve its literal traversal instead of inventing a populated row or count.
            #[allow(clippy::never_loop)]
            for row in rows {
                match *row {}
            }
        }
        // Actual ReportStandAloneWaterUse1570 traverses count, not a guessed tank inventory.
        if *number_equipment > 0 {
            return Err(Error::Unavailable(
                "actual standalone WaterEquipment record1583",
            ));
        }
        if let Some(rows) = self.connections.as_ref() {
            // This real owner has uninhabited rows; preserve its literal traversal instead of inventing a populated row or count.
            #[allow(clippy::never_loop)]
            for row in rows {
                match *row {}
            }
        }
        Ok(())
    }
    fn get_source_input(
        &mut self,
        number_equipment: &mut i32,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "WaterUse:Equipment".into();
        *number_equipment =
            i32::try_from(input.get_num_objects_found(&shortcut.current_module_object))
                .map_err(|_| Error::Unavailable("actual WaterUse Equipment count int domain303"))?;
        if *number_equipment > 0 {
            return Err(Error::Unavailable(
                "actual WaterEquipment positive allocation307",
            ));
        }
        shortcut.current_module_object = "WaterUse:Connections".into();
        self.number_connections = i32::try_from(
            input.get_num_objects_found(&shortcut.current_module_object),
        )
        .map_err(|_| Error::Unavailable("actual WaterUse Connections count int domain383"))?;
        if self.number_connections > 0 {
            return Err(Error::Unavailable(
                "actual WaterConnections positive allocation386",
            ));
        }
        // Genuine zero connections skip CheckEquipName, peak-flow and plant registration.
        // Final allowHotControl traversal uses the same default-unallocated equipment vector.
        if let Some(rows) = self.equipment.as_ref() {
            // This real owner has uninhabited rows; preserve its literal traversal instead of inventing a populated row or count.
            #[allow(clippy::never_loop)]
            for row in rows {
                match *row {}
            }
        }
        Ok(())
    }
}

impl SourceWaterUseModule {
    pub(crate) fn fill_source_predefined_water_equipment_tables(&self) -> Result<(), Error> {
        if self
            .equipment
            .as_deref()
            .and_then(|rows| rows.first())
            .is_some()
        {
            return Err(Error::Unavailable(
                "actual WaterEquipment first FillPredefinedTable child",
            ));
        }
        Ok(())
    }
}
