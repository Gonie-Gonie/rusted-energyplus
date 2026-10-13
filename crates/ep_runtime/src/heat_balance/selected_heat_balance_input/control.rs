//! Literal selected remaining GetProjectControlData call/query/EIO order.
use super::{
    HeatBalanceInputError, HeatBalanceInputOwners, SelectedHeatBalanceInput,
    SelectedHeatBalanceInputState, SelectedHeatTransferModel, SelectedWindowOpticalModel,
    SelectedWindowThermalModel, SelectedZoneAirSolution, SourceErrorBackend, absent,
};
use ep_model::{InsideSurfaceConvectionAlgorithm, OutsideSurfaceConvectionAlgorithm};
use std::io::Write;

impl SelectedHeatBalanceInputState {
    pub(crate) fn get_project_control_data(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        owners: &mut HeatBalanceInputOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors_found: &mut bool,
    ) -> Result<(), HeatBalanceInputError> {
        self.get_building(raw, owners, backend, eio, errors_found)?;
        *owners.current_module_object = "SurfaceConvectionAlgorithm:Inside".into();
        if owners
            .input
            .get_num_objects_found("SurfaceConvectionAlgorithm:Inside")
            > 0
        {
            let values = raw.get_object_item::<4, 5>(
                "SurfaceConvectionAlgorithm:Inside",
                0,
                owners.input,
                owners.shortcut,
            )?;
            if values.alphas[0] != "TARP" {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted inside convection branch",
                ));
            }
        }
        self.inside_convection_algorithm = InsideSurfaceConvectionAlgorithm::Tarp;
        write!(
            eio,
            "! <Inside Convection Algorithm>, Algorithm {{Simple | TARP | CeilingDiffuser | AdaptiveConvectionAlgorithm}}\nInside Convection Algorithm,TARP\n"
        )?;
        *owners.current_module_object = "SurfaceConvectionAlgorithm:Outside".into();
        if owners
            .input
            .get_num_objects_found("SurfaceConvectionAlgorithm:Outside")
            > 0
        {
            let values = raw.get_object_item::<4, 5>(
                "SurfaceConvectionAlgorithm:Outside",
                0,
                owners.input,
                owners.shortcut,
            )?;
            if values.alphas[0] != "DOE-2" {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted outside convection branch",
                ));
            }
        }
        self.outside_convection_algorithm = OutsideSurfaceConvectionAlgorithm::Doe2;
        write!(
            eio,
            "! <Outside Convection Algorithm>, Algorithm {{SimpleCombined | TARP | MoWitt | DOE-2 | AdaptiveConvectionAlgorithm}}\nOutside Convection Algorithm,DOE-2\n"
        )?;
        *owners.current_module_object = "HeatBalanceAlgorithm".into();
        if owners.input.get_num_objects_found("HeatBalanceAlgorithm") > 0 {
            let values = raw.get_object_item::<4, 5>(
                "HeatBalanceAlgorithm",
                0,
                owners.input,
                owners.shortcut,
            )?;
            if values.alphas[0] != "CONDUCTIONTRANSFERFUNCTION" {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted heat-transfer algorithm",
                ));
            }
            self.overall_heat_transfer_solution = SelectedHeatTransferModel::Ctf;
            self.any_ctf = true;
            if values.num_numbers > 0 {
                self.max_surface_temperature_limit = values.numbers[0];
                self.max_surface_temperature_before_fatal =
                    self.max_surface_temperature_limit * 2.5;
                if self.max_surface_temperature_limit >= -100.0
                    && self.max_surface_temperature_limit < 0.0
                {
                    self.max_surface_temperature_limit = 200.0;
                    self.max_surface_temperature_before_fatal =
                        self.max_surface_temperature_limit * 2.5;
                }
            }
            if !owners.shortcut.numeric_field_blanks[1] {
                self.low_hconv_limit = values.numbers[1];
            }
            if !owners.shortcut.numeric_field_blanks[2] {
                self.high_hconv_limit = values.numbers[2];
            }
        } else {
            self.overall_heat_transfer_solution = SelectedHeatTransferModel::Ctf;
            self.any_ctf = true;
            self.max_surface_temperature_limit = 200.0;
            self.max_surface_temperature_before_fatal = self.max_surface_temperature_limit * 2.5;
        }
        write!(
            eio,
            "! <Sky Radiance Distribution>, Value {{Anisotropic}}\nSky Radiance Distribution,Anisotropic\n"
        )?;
        absent(owners, "Compliance:Building")?;
        *owners.current_module_object = "ZoneAirHeatBalanceAlgorithm".into();
        let mut zone_label = "ThirdOrderBackwardDifference".to_string();
        if owners
            .input
            .get_num_objects_found("ZoneAirHeatBalanceAlgorithm")
            > 0
        {
            let values = raw.get_object_item::<4, 5>(
                "ZoneAirHeatBalanceAlgorithm",
                0,
                owners.input,
                owners.shortcut,
            )?;
            if values.num_alphas > 0 {
                if values.alphas[0] != "THIRDORDERBACKWARDDIFFERENCE" {
                    return Err(HeatBalanceInputError::Unavailable(
                        "unadmitted zone algorithm",
                    ));
                }
                self.zone_air_solution = SelectedZoneAirSolution::ThirdOrder;
            } else {
                zone_label = values.alphas[0].clone();
            }
            if !owners.shortcut.alpha_field_blanks[1] {
                if values.alphas[1] != "NO" {
                    return Err(HeatBalanceInputError::Unavailable(
                        "unadmitted Space sizing branch",
                    ));
                }
                owners.space_controls.sizing = false;
            }
            if !owners.shortcut.alpha_field_blanks[2] {
                if values.alphas[2] != "NO" {
                    return Err(HeatBalanceInputError::Unavailable(
                        "unadmitted Space simulation branch",
                    ));
                }
                owners.space_controls.simulation = false;
            }
        } else {
            self.zone_air_solution = SelectedZoneAirSolution::ThirdOrder;
        }
        if owners.project.override_zone_air_solution_algorithm {
            self.zone_air_solution = SelectedZoneAirSolution::Euler;
            zone_label = "EulerMethod".into();
        }
        writeln!(
            eio,
            "! <Zone Air Solution Algorithm>, Algorithm {{ThirdOrderBackwardDifference | AnalyticalSolution | EulerMethod}}, Space Heat Balance Sizing, Space Heat Balance Simulation"
        )?;
        writeln!(
            eio,
            " Zone Air Solution Algorithm, {}, {}, {}",
            zone_label,
            if owners.space_controls.sizing {
                "Yes"
            } else {
                "No"
            },
            if owners.space_controls.simulation {
                "Yes"
            } else {
                "No"
            }
        )?;
        absent(owners, "ZoneAirContaminantBalance")?;
        self.simulate_contaminants = false;
        self.carbon_dioxide_simulation = false;
        self.generic_contaminant_simulation = false;
        // Each factory returns its actual selected allocation before the
        // window-manager assignment. Unsupported factory locals are not stored.
        if owners
            .input
            .get_num_objects_found("WindowsCalculationEngine")
            != 0
        {
            return Err(HeatBalanceInputError::Unavailable(
                "positive WindowsCalculationEngine",
            ));
        }
        self.window_thermal_model = Some(SelectedWindowThermalModel::BuiltIn);
        if owners
            .input
            .get_num_objects_found("Construction:ComplexFenestrationState")
            != 0
        {
            return Err(HeatBalanceInputError::Unavailable(
                "positive complex optical model",
            ));
        }
        self.window_optical_model = Some(SelectedWindowOpticalModel::Simplified);
        write!(
            eio,
            "! <Zone Air Carbon Dioxide Balance Simulation>, Simulation {{Yes/No}}, Carbon Dioxide Concentration\n Zone Air Carbon Dioxide Balance Simulation, No,N/A\n"
        )?;
        write!(
            eio,
            "! <Zone Air Generic Contaminant Balance Simulation>, Simulation {{Yes/No}}, Generic Contaminant Concentration\n Zone Air Generic Contaminant Balance Simulation, No,N/A\n"
        )?;
        absent(owners, "ZoneAirMassFlowConservation")?;
        self.enforce_zone_mass_balance = false;
        write!(
            eio,
            "! <Zone Air Mass Flow Balance Simulation>, Enforce Mass Balance, Adjust Zone Mixing and Return {{AdjustMixingOnly | AdjustReturnOnly | AdjustMixingThenReturn | AdjustReturnThenMixing | None}}, Adjust Zone Infiltration {{AddInfiltration | AdjustInfiltration | None}}, Infiltration Zones {{MixingSourceZonesOnly | AllZones}}\n Zone Air Mass Flow Balance Simulation, No,N/A,N/A,N/A\n"
        )?;
        absent(owners, "HVACSystemRootFindingAlgorithm")?;
        self.root_algorithm = "RegulaFalsi".into();
        writeln!(
            eio,
            "! <HVACSystemRootFindingAlgorithm>, Value {{RegulaFalsi | Bisection | BisectionThenRegulaFalsi | RegulaFalsiThenBisection}}"
        )?;
        writeln!(
            eio,
            " HVACSystemRootFindingAlgorithm, {}",
            self.root_algorithm
        )?;
        Ok(())
    }
}
