//! Actual AFN/RoomAir/Hybrid/DuctLoss selected constructors and reached children.
use super::*;
use std::io::Write;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceAirflowNetwork {
    pub(crate) get_input_flag: bool,
    pub(crate) num_airflow_network: usize,
    pub(crate) num_occupant_ventilation_controls: usize,
    pub(crate) control_type: i32,
    pub(crate) duct_loss: bool,
    pub(crate) fan_activated: bool,
    pub(crate) distribution_simulated: bool,
    pub(crate) multizone_always_simulated: bool,
    pub(crate) control_defaulted: bool,
    pub(crate) rollback_flag: bool,
}
impl Default for SourceAirflowNetwork {
    fn default() -> Self {
        Self {
            get_input_flag: true,
            num_airflow_network: 0,
            num_occupant_ventilation_controls: 0,
            control_type: 0,
            duct_loss: false,
            fan_activated: false,
            distribution_simulated: false,
            multizone_always_simulated: false,
            control_defaulted: false,
            rollback_flag: false,
        }
    }
}
impl SourceAirflowNetwork {
    pub(crate) fn manage_source_balance(
        &mut self,
        input: &mut SelectedInputProcessor,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_input(input, eio)?;
            self.get_input_flag = false; // Solver174 only after successful genuine input.
            return Ok(());
        }
        if self.control_type == 0 {
            return Ok(());
        } // Solver182
        Err(Error::Unavailable("positive AFN initialize190"))
    }
    fn get_source_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let mut max_numerics = 0;
        let mut max_alphas = 0;
        for family in [
            "AirflowNetwork:SimulationControl",
            "AirflowNetwork:MultiZone:Zone",
            "AirflowNetwork:MultiZone:Surface",
            "AirflowNetwork:MultiZone:Component:DetailedOpening",
            "AirflowNetwork:MultiZone:ExternalNode",
            "AirflowNetwork:MultiZone:WindPressureCoefficientArray",
            "AirflowNetwork:MultiZone:WindPressureCoefficientValues",
            "AirflowNetwork:Distribution:Node",
            "AirflowNetwork:Distribution:DuctViewFactors",
            "AirflowNetwork:Distribution:Linkage",
            "AirflowNetwork:OccupantVentilationControl",
            "AirflowNetwork:IntraZone:Node",
            "AirflowNetwork:IntraZone:Linkage",
            "AirflowNetwork:ZoneControl:PressureController",
            "AirflowNetwork:Distribution:DuctSizing",
        ] {
            let counts = input.get_object_def_max_args(family).map_err(|_| {
                Error::Unavailable("AFN actual definition and local scratch bounds1710-1767")
            })?;
            max_numerics = max_numerics.max(counts.numerics);
            max_alphas = max_alphas.max(counts.alphas);
        }
        // Genuine local arrays1769-1774; never overwrite shared IP shortcut arrays.
        let _alphas = vec![String::new(); max_alphas];
        let _alpha_fields = vec![String::new(); max_alphas];
        let _numeric_fields = vec![String::new(); max_numerics];
        let _numbers = vec![0.0; max_numerics];
        let _alpha_blanks = vec![true; max_alphas];
        let _numeric_blanks = vec![true; max_numerics];
        let conduction = input.get_num_objects_found("Duct:Loss:Conduction");
        let leakage = input.get_num_objects_found("Duct:Loss:Leakage");
        let makeup = input.get_num_objects_found("Duct:Loss:MakeupAir");
        if conduction
            .checked_add(leakage)
            .and_then(|v| v.checked_add(makeup))
            .ok_or(Error::Unavailable("AFN defined Source duct count sum"))?
            > 0
        {
            self.duct_loss = true;
        }
        self.num_occupant_ventilation_controls =
            input.get_num_objects_found("AirflowNetwork:OccupantVentilationControl");
        if self.num_occupant_ventilation_controls > 0 {
            return Err(Error::Unavailable(
                "positive AFN OccupantVentilationControl allocation1792",
            ));
        }
        self.num_airflow_network = input.get_num_objects_found("AirflowNetwork:SimulationControl");
        if self.num_airflow_network == 0 {
            if input.get_num_objects_found("AirflowNetwork:MultiZone:Zone") >= 1
                && input.get_num_objects_found("AirflowNetwork:MultiZone:Surface") >= 2
            {
                return Err(Error::Unavailable(
                    "positive AFN default control writes1960",
                ));
            } else if self.duct_loss {
                return Err(Error::Unavailable("positive AFN duct-loss EIO1982"));
            } else {
                self.control_type = 0; //1985; convenience flags2046/2049 are NOT reached.
                write!(
                    eio,
                    "! <AirflowNetwork Model:Control>, No Multizone or Distribution/Multizone with Distribution/Multizone without Distribution/Multizone with Distribution only during Fan Operation\n"
                )?;
                writeln!(
                    eio,
                    "AirflowNetwork Model:Control,NoMultizoneOrDistribution"
                )?;
                return Ok(());
            }
        }
        Err(Error::Unavailable("positive AFN SimulationControl1991"))
    }
    pub(super) fn replaces_simple_multizone(&self) -> bool {
        self.multizone_always_simulated || (self.control_type == 2 && self.fan_activated)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceRoomAirModel {
    pub(crate) name: String,
    pub(crate) zone_name: String,
    pub(crate) zone_ptr: i32,
    pub(crate) model: i32,
    pub(crate) coupling_scheme: i32,
    pub(crate) simulate_air_model: bool,
}
impl Default for SourceRoomAirModel {
    fn default() -> Self {
        Self {
            name: String::new(),
            zone_name: String::new(),
            zone_ptr: 0,
            model: 1,
            coupling_scheme: 0,
            simulate_air_model: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceRoomAir {
    pub(crate) get_air_model_data: bool,
    pub(crate) any_non_mixing: bool,
    pub(crate) disp_vent_1node_model_used: bool,
    pub(crate) ucsd_model_used: bool,
    pub(crate) air_models: Option<Vec<SourceRoomAirModel>>,
    pub(crate) num_room_afn_control: usize,
    pub(crate) num_temperature_distribution_controlled_zones: usize,
    pub(crate) num_constant_gradient: usize,
    pub(crate) num_two_gradient_interp: usize,
    pub(crate) num_nondimensional_height: usize,
    pub(crate) num_surface_mapping: usize,
    pub(crate) num_air_temperature_patterns: usize,
    pub(crate) total_ufad_int: i32,
    pub(crate) total_ufad_ext: i32,
}
impl Default for SourceRoomAir {
    fn default() -> Self {
        Self {
            get_air_model_data: true,
            any_non_mixing: false,
            disp_vent_1node_model_used: false,
            ucsd_model_used: false,
            air_models: None,
            num_room_afn_control: 0,
            num_temperature_distribution_controlled_zones: 0,
            num_constant_gradient: 0,
            num_two_gradient_interp: 0,
            num_nondimensional_height: 0,
            num_surface_mapping: 0,
            num_air_temperature_patterns: 0,
            total_ufad_int: 0,
            total_ufad_ext: 0,
        }
    }
}
impl SourceRoomAir {
    /// Actual GetAirHeatBalanceInput184 -> GetRoomAirModelParameters4269+.
    pub(crate) fn get_source_model_parameters(
        &mut self,
        zones: &SelectedZoneInputOwners,
        globals: &SelectedHeatBalanceGlobals,
        space_controls: &SelectedHeatBalanceSpaceControls,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let count = usize::try_from(globals.num_of_zones)
            .map_err(|_| Error::Unavailable("RoomAir actual NumOfZones4269"))?;
        if self.air_models.is_some() {
            return Err(Error::Unavailable(
                "repeated native AirModel allocate4269 state not yet represented",
            ));
        }
        self.air_models = Some(vec![SourceRoomAirModel::default(); count]);
        shortcut.current_module_object = "RoomAirModelType".into();
        let models = input.get_num_objects_found("RoomAirModelType");
        if models > 0 {
            return Err(Error::Unavailable("positive RoomAirModelType4276"));
        }
        for number in 1..=globals.num_of_zones {
            let zone = zones
                .zones
                .as_deref()
                .and_then(|rows| rows.get(index(number).ok()?))
                .ok_or(Error::Unavailable("RoomAir actual Zone4454"))?;
            let row = self
                .air_models
                .as_mut()
                .and_then(|rows| rows.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("RoomAir actual AirModel4454"))?;
            row.name = "MIXING AIR MODEL FOR ".to_owned() + &zone.name;
            row.zone_name.clone_from(&zone.name);
            if row.model != 1 {
                self.any_non_mixing = true;
            }
        }
        // For zero RoomAir objects every allocated row genuinely has Mixing1.
        if self.any_non_mixing && (space_controls.simulation || space_controls.sizing) {
            return Err(Error::Unavailable(
                "positive non-mixing RoomAir SpaceHB4462",
            ));
        }
        write!(
            eio,
            "! <RoomAir Model>, Zone Name, Mixing/Mundt/UCSDDV/UCSDCV/UCSDUFI/UCSDUFE/User Defined\n"
        )?;
        for number in 1..=globals.num_of_zones {
            let zone = zones
                .zones
                .as_deref()
                .and_then(|rows| rows.get(index(number).ok()?))
                .ok_or(Error::Unavailable("RoomAir report actual Zone4482"))?;
            let model = self.model(number)?;
            if model != 1 {
                return Err(Error::Unavailable("positive RoomAir EIO model4482"));
            }
            writeln!(eio, "RoomAir Model,{},Mixing/Well-Stirred", zone.name)?;
        }
        Ok(())
    }
    pub(crate) fn model(&self, zone: i32) -> Result<i32, Error> {
        Ok(self
            .air_models
            .as_deref()
            .and_then(|rows| rows.get(index(zone).ok()?))
            .ok_or(Error::Unavailable("actual RoomAir AirModel allocation"))?
            .model)
    }
    pub(crate) fn manage_source_air_model(
        &mut self,
        zone: i32,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.get_air_model_data {
            self.get_source_air_model_data(input, shortcut, backend)?;
            self.get_air_model_data = false;
        }
        if !self.any_non_mixing {
            return Ok(());
        }
        if self.ucsd_model_used {
            return Err(Error::Unavailable(
                "positive RoomAir SharedDVCVUFDataInit151",
            ));
        }
        if self.model(zone)? == 1 {
            return Ok(());
        }
        Err(Error::Unavailable("positive RoomAir model154"))
    }
    fn get_source_air_model_data(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        // GetAirNodeData217 -> genuine first predicate653.
        if self.disp_vent_1node_model_used {
            return Err(Error::Unavailable("positive RoomAir GetAirNodeData660"));
        }
        // GetMundtData220 -> genuine first predicate837.
        if self.disp_vent_1node_model_used {
            return Err(Error::Unavailable("positive RoomAir GetMundtData842"));
        }
        // GetRoomAirflowNetworkData223 always performs these shared module/count writes.
        shortcut.current_module_object = "RoomAirSettings:AirflowNetwork".into();
        self.num_room_afn_control = input.get_num_objects_found("RoomAirSettings:AirflowNetwork");
        if self.num_room_afn_control != 0 {
            return Err(Error::Unavailable("positive RoomAir AFN1298"));
        }
        // GetDisplacementVentData226 and GetCrossVentData229 each return before a query.
        if self.ucsd_model_used {
            return Err(Error::Unavailable(
                "positive RoomAir GetDisplacementVentData922",
            ));
        }
        if self.ucsd_model_used {
            return Err(Error::Unavailable("positive RoomAir GetCrossVentData998"));
        }
        // GetUserDefinedPatternData232 -> actual count writes273-283.
        self.num_temperature_distribution_controlled_zones =
            input.get_num_objects_found("RoomAir:TemperaturePattern:UserDefined");
        self.num_constant_gradient =
            input.get_num_objects_found("RoomAir:TemperaturePattern:ConstantGradient");
        self.num_two_gradient_interp =
            input.get_num_objects_found("RoomAir:TemperaturePattern:TwoGradient");
        self.num_nondimensional_height =
            input.get_num_objects_found("RoomAir:TemperaturePattern:NondimensionalHeight");
        self.num_surface_mapping =
            input.get_num_objects_found("RoomAir:TemperaturePattern:SurfaceMapping");
        self.num_air_temperature_patterns = self
            .num_constant_gradient
            .checked_add(self.num_two_gradient_interp)
            .and_then(|v| v.checked_add(self.num_nondimensional_height))
            .and_then(|v| v.checked_add(self.num_surface_mapping))
            .ok_or(Error::Unavailable(
                "defined Source RoomAir pattern count sum",
            ))?;
        shortcut.current_module_object = "RoomAir:TemperaturePattern:UserDefined".into();
        if self.num_temperature_distribution_controlled_zones == 0 {
            if self.num_air_temperature_patterns != 0 {
                backend.show_warning_error("Missing RoomAir:TemperaturePattern:UserDefined object needed to use roomair temperature patterns")?;
            }
        } else {
            return Err(Error::Unavailable("positive RoomAir AirPatternZoneInfo294"));
        }
        // GetUFADZoneData236 writes both counters before its genuine1125 return.
        if !self.ucsd_model_used {
            self.total_ufad_int = 0;
            self.total_ufad_ext = 0;
        } else {
            return Err(Error::Unavailable("positive RoomAir UFAD1130"));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceHybridModel {
    pub(crate) active: bool,
    pub(crate) thermal_mass: bool,
    pub(crate) air_infiltration: bool,
    pub(crate) num_zones: usize,
    pub(crate) current_module_object: String,
}
impl SourceHybridModel {
    /// ZTPC1450 real child; local CurrentModuleObject does not write module member.
    pub(crate) fn get_source_hybrid_model_zone(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), Error> {
        self.num_zones = input.get_num_objects_found("HybridModel:Zone");
        if self.num_zones > 0 {
            return Err(Error::Unavailable("positive HybridModelZone allocation99"));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceDuctLoss {
    pub(crate) simulated: bool,
    pub(crate) num_duct_losses: i32,
    pub(crate) get_input_flag: bool,
    pub(crate) air_loop_connection_flag: bool,
    pub(crate) air_loop_in_node_num: i32,
}
impl Default for SourceDuctLoss {
    fn default() -> Self {
        Self {
            simulated: false,
            num_duct_losses: 0,
            get_input_flag: true,
            air_loop_connection_flag: true,
            air_loop_in_node_num: 0,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneAirSubsystems {
    pub(crate) airflow_network: SourceAirflowNetwork,
    pub(crate) room_air: SourceRoomAir,
    pub(crate) hybrid_model: SourceHybridModel,
    pub(crate) duct_loss: SourceDuctLoss,
}
