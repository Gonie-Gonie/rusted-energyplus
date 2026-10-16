//! Actual GeneralRoutines843-1610 finite empty paths and orphan-input queries.
use super::{Error, extent, node_extent};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::ordinary_hvac_manager::SourceHvacAncillaryModules;
use crate::heat_balance::ordinary_hvac_manager::simulation::state::SourceAirLoopData;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use std::io::Write;

// Genuine function-local Array2D allocation and fill, not retained probe state.
struct IntMatrix {
    rows: usize,
    columns: usize,
    values: Vec<Option<i32>>,
}
impl IntMatrix {
    fn allocate(rows: usize, columns: i32) -> Result<Self, Error> {
        let rows = node_extent(rows)?;
        let columns = extent(columns)?;
        let count = rows
            .checked_mul(columns)
            .ok_or(Error::Unavailable("actual AirPath Array2D storage extent"))?;
        Ok(Self {
            rows,
            columns,
            values: vec![None; count],
        })
    }
    fn fill_zero(&mut self) {
        self.values.fill(Some(0));
    }
    fn read(&self, row: usize, column: i32) -> Result<i32, Error> {
        let column = extent(column)?;
        if row == 0 || row > self.rows || column == 0 || column > self.columns {
            return Err(Error::Unavailable(
                "actual AirPath Array2D subscript defined domain",
            ));
        }
        self.values
            .get((column - 1) * self.rows + (row - 1))
            .copied()
            .flatten()
            .ok_or(Error::Unavailable("actual AirPath written Array2D element"))
    }
}
fn flag(value: Option<bool>) -> Result<bool, Error> {
    value.ok_or(Error::Unavailable(
        "actual TestAirPathIntegrity local errFlag written by child",
    ))
}
pub(crate) fn test_source_air_path_integrity(
    equipment: &OrdinaryEquipment,
    hvac: &SelectedHvacGlobals,
    air_loop: &SourceAirLoopData,
    modules: &SourceHvacAncillaryModules,
    input: &SelectedInputProcessor,
    err_found: &mut bool,
    bnd: &mut dyn Write,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let mut err_flag = None;
    let mut number_supply_nodes = IntMatrix::allocate(
        *equipment.source_number_of_nodes(),
        hvac.number_of_primary_air_systems,
    )?;
    let mut number_return_nodes = IntMatrix::allocate(
        *equipment.source_number_of_nodes(),
        hvac.number_of_primary_air_systems,
    )?;
    let mut valid_return_paths = IntMatrix::allocate(
        *equipment.source_number_of_nodes(),
        hvac.number_of_primary_air_systems,
    )?;
    let mut valid_supply_paths = IntMatrix::allocate(
        *equipment.source_number_of_nodes(),
        hvac.number_of_primary_air_systems,
    )?;
    number_supply_nodes.fill_zero();
    number_return_nodes.fill_zero();
    valid_return_paths.fill_zero();
    valid_supply_paths.fill_zero();
    test_supply(
        equipment,
        hvac,
        air_loop,
        modules,
        input,
        &mut err_flag,
        bnd,
        backend,
    )?;
    if flag(err_flag)? {
        *err_found = true;
    }
    test_return(
        equipment,
        hvac,
        air_loop,
        modules,
        input,
        &mut err_flag,
        &mut valid_return_paths,
        bnd,
        backend,
    )?;
    if flag(err_flag)? {
        *err_found = true;
    }
    for air_system in 1..=hvac.number_of_primary_air_systems {
        if valid_return_paths.read(1, air_system)? != 0 {
            continue;
        }
        let _same_air_to_zone_node_info = &air_loop.air_to_zone_node_info;
        return Err(Error::Unavailable(
            "actual TestAirPathIntegrity883 AirToZoneNodeInfo.NumReturnNodes",
        ));
    }
    // The first actual positive air-system owner read above is unavailable.
    // At genuine NumPrimaryAirSys0 both native duplicate-node loops are empty.
    drop(number_supply_nodes);
    drop(number_return_nodes);
    drop(valid_return_paths);
    drop(valid_supply_paths);
    Ok(())
}
fn test_supply(
    equipment: &OrdinaryEquipment,
    hvac: &SelectedHvacGlobals,
    air_loop: &SourceAirLoopData,
    modules: &SourceHvacAncillaryModules,
    input: &SelectedInputProcessor,
    err_found: &mut Option<bool>,
    bnd: &mut dyn Write,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let _number_errors = 0;
    backend.show_message("Testing Individual Supply Air Path Integrity")?;
    *err_found = Some(false);
    writeln!(
        bnd,
        "! ==============================================================="
    )?;
    writeln!(bnd, "! <#Supply Air Paths>,<Number of Supply Air Paths>")?;
    writeln!(
        bnd,
        " #Supply Air Paths,{}",
        equipment.source_number_supply_paths()
    )?;
    writeln!(
        bnd,
        "! <Supply Air Path>,<Supply Air Path Count>,<Supply Air Path Name>,<AirLoopHVAC Name>"
    )?;
    writeln!(
        bnd,
        "! <#Components on Supply Air Path>,<Number of Components>"
    )?;
    writeln!(
        bnd,
        "! <Supply Air Path Component>,<Component Count>,<Component Type>,<Component Name>,<AirLoopHVAC Name>"
    )?;
    writeln!(
        bnd,
        "! <#Outlet Nodes on Supply Air Path Component>,<Number of Nodes>"
    )?;
    writeln!(
        bnd,
        "! <Supply Air Path Component Nodes>,<Node Count>,<Component Type>,<Component Name>,<Inlet Node Name>,<Outlet Node Name>,<AirLoopHVAC Name>"
    )?;
    if *equipment.source_number_supply_paths() > 0 {
        if hvac.number_of_primary_air_systems > 0 {
            let _same_air_to_zone_node_info = &air_loop.air_to_zone_node_info;
            return Err(Error::Unavailable(
                "actual TestSupplyAirPathIntegrity966 first AirToZoneNodeInfo.AirLoopName",
            ));
        }
        let _primary_air_loop_name = "**Unknown**";
        return Err(Error::Unavailable(
            "actual TestSupplyAirPathIntegrity982 SupplyAirPath.Name print argument",
        ));
    }
    if modules.splitter_component.number_splitters == 0 {
        if input.get_num_objects_found("AirLoopHVAC:ZoneSplitter") > 0 {
            return Err(Error::Unavailable(
                "actual TestSupplyAirPathIntegrity1112 SplitterComponent::GetSplitterInput child",
            ));
        }
    }
    if modules.zone_plenum.number_supply_plenums == 0
        && modules.zone_plenum.number_return_plenums == 0
    {
        if input.get_num_objects_found("AirLoopHVAC:SupplyPlenum") > 0 {
            return Err(Error::Unavailable(
                "actual TestSupplyAirPathIntegrity1117 ZonePlenum::GetZonePlenumInput child",
            ));
        }
    }
    let found_supply_plenum = vec![false; extent(modules.zone_plenum.number_supply_plenums)?];
    let found_zone_splitter = vec![false; extent(modules.splitter_component.number_splitters)?];
    let found_names = vec![String::new(); extent(modules.zone_plenum.number_supply_plenums)?];
    // Earlier positive SupplyPath first-access boundary dominates both nested
    // name/component traversals. With zero real paths they have no body access.
    drop(found_names);
    let found_names = vec![String::new(); extent(modules.splitter_component.number_splitters)?];
    drop(found_names);
    if !found_supply_plenum.iter().all(|found| *found) {
        return Err(Error::Unavailable(
            "actual TestSupplyAirPathIntegrity1179 ZoneSupPlenCond.ZonePlenumName before severe",
        ));
    }
    if !found_zone_splitter.iter().all(|found| *found) {
        return Err(Error::Unavailable(
            "actual TestSupplyAirPathIntegrity1190 SplitterCond.SplitterName before severe",
        ));
    }
    drop(found_supply_plenum);
    drop(found_zone_splitter);
    if flag(*err_found)? {
        backend.show_severe_error("Supply Air Path(s) did not pass integrity testing")?;
    } else {
        backend.show_message("All Supply Air Paths passed integrity testing")?;
    }
    Ok(())
}
fn test_return(
    equipment: &OrdinaryEquipment,
    hvac: &SelectedHvacGlobals,
    air_loop: &SourceAirLoopData,
    modules: &SourceHvacAncillaryModules,
    input: &SelectedInputProcessor,
    err_found: &mut Option<bool>,
    _valid_return_paths: &mut IntMatrix,
    bnd: &mut dyn Write,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    backend.show_message("Testing Individual Return Air Path Integrity")?;
    *err_found = Some(false);
    let _number_errors = 0;
    writeln!(
        bnd,
        "! ==============================================================="
    )?;
    writeln!(bnd, "! <#Return Air Paths>,<Number of Return Air Paths>")?;
    writeln!(
        bnd,
        " #Return Air Paths,{}",
        equipment.source_number_return_paths()
    )?;
    writeln!(
        bnd,
        "! <Return Air Path>,<Return Air Path Count>,<Return Air Path Name>,<AirLoopHVAC Name>"
    )?;
    writeln!(
        bnd,
        "! <#Components on Return Air Path>,<Number of Components>"
    )?;
    writeln!(
        bnd,
        "! <Return Air Path Component>,<Component Count>,<Component Type>,<Component Name>,<AirLoopHVAC Name>"
    )?;
    writeln!(
        bnd,
        "! <#Inlet Nodes on Return Air Path Component>,<Number of Nodes>"
    )?;
    writeln!(
        bnd,
        "! <Return Air Path Component Nodes>,<Node Count>,<Component Type>,<Component Name>,<Inlet Node Name>,<Outlet Node Name>,<AirLoopHVAC Name>"
    )?;
    let all_nodes = vec![None::<i32>; node_extent(*equipment.source_number_of_nodes())?];
    if *equipment.source_number_return_paths() > 0 {
        if hvac.number_of_primary_air_systems > 0 {
            let _same_air_to_zone_node_info = &air_loop.air_to_zone_node_info;
            return Err(Error::Unavailable(
                "actual TestReturnAirPathIntegrity1276 first AirToZoneNodeInfo.AirLoopName",
            ));
        }
        let _primary_air_loop_name = "**Unknown**";
        return Err(Error::Unavailable(
            "actual TestReturnAirPathIntegrity1292 ReturnAirPath.Name print argument",
        ));
    }
    drop(all_nodes);
    if modules.mixer_component.number_mixers == 0 {
        if input.get_num_objects_found("AirLoopHVAC:ZoneMixer") > 0 {
            return Err(Error::Unavailable(
                "actual TestReturnAirPathIntegrity1498 MixerComponent::GetMixerInput child",
            ));
        }
    }
    if modules.zone_plenum.number_supply_plenums == 0
        && modules.zone_plenum.number_return_plenums == 0
    {
        if input.get_num_objects_found("AirLoopHVAC:ReturnPlenum") > 0 {
            return Err(Error::Unavailable(
                "actual TestReturnAirPathIntegrity1503 ZonePlenum::GetZonePlenumInput child",
            ));
        }
    }
    let found_return_plenum = vec![false; extent(modules.zone_plenum.number_return_plenums)?];
    let found_zone_mixer = vec![false; extent(modules.mixer_component.number_mixers)?];
    let found_names = vec![String::new(); extent(modules.zone_plenum.number_return_plenums)?];
    if modules.zone_plenum.number_return_plenums > 0 {
        return Err(Error::Unavailable(
            "actual TestReturnAirPathIntegrity1531 CheckPurchasedAirForReturnPlenum child",
        ));
    }
    drop(found_names);
    let found_names = vec![String::new(); extent(modules.mixer_component.number_mixers)?];
    if modules.mixer_component.number_mixers > 0 {
        return Err(Error::Unavailable(
            "actual TestReturnAirPathIntegrity1559 MixerCond.MixerName before PIUnitHasMixer child",
        ));
    }
    drop(found_names);
    if !found_return_plenum.iter().all(|found| *found) {
        return Err(Error::Unavailable(
            "actual TestReturnAirPathIntegrity1585 ZoneRetPlenCond.ZonePlenumName severe argument",
        ));
    }
    if !found_zone_mixer.iter().all(|found| *found) {
        return Err(Error::Unavailable(
            "actual TestReturnAirPathIntegrity1597 MixerCond.MixerName severe argument",
        ));
    }
    drop(found_return_plenum);
    drop(found_zone_mixer);
    if flag(*err_found)? {
        backend.show_severe_error("Return Air Path(s) did not pass integrity testing")?;
    } else {
        backend.show_message("All Return Air Paths passed integrity testing")?;
    }
    Ok(())
}
