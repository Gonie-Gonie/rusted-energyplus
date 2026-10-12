//! Real selected Fluid constant/input initialization before ScheduleManager.
//! User fluid definitions and later property lookup/interpolation remain unavailable.
mod glycol_data;
mod limits;
mod owners;
mod water_steam_data;
use super::selected_input_processor::{SelectedInputProcessor, SelectedInputUnavailable};
use glycol_data::*;
pub(crate) use owners::SelectedFluidProperties;
use owners::*;
use water_steam_data::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SelectedFluidError {
    Input(SelectedInputUnavailable),
    Unavailable(&'static str),
}
impl From<SelectedInputUnavailable> for SelectedFluidError {
    fn from(error: SelectedInputUnavailable) -> Self {
        Self::Input(error)
    }
}
impl std::fmt::Display for SelectedFluidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(error) => std::fmt::Display::fmt(error, f),
            Self::Unavailable(reason) => {
                write!(f, "selected Fluid initialization unavailable: {reason}")
            }
        }
    }
}
impl std::error::Error for SelectedFluidError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(error) => Some(error),
            Self::Unavailable(_) => None,
        }
    }
}

fn fill_raw(property: &mut RawGlycolProperty, concentrations: &[f64], values: &[f64]) {
    property.data_present = true;
    property.number_concentration_points = concentrations.len();
    property.number_temperature_points = DEFAULT_GLYCOL_TEMPS.len();
    property.temperatures = DEFAULT_GLYCOL_TEMPS.to_vec();
    property.concentrations = concentrations.to_vec();
    property.values = values.to_vec();
}
fn fill_mixture(property: &mut GlycolProperty, values: &[f64]) {
    property.data_present = true;
    property.number_temperature_points = DEFAULT_GLYCOL_TEMPS.len();
    property.temperatures = DEFAULT_GLYCOL_TEMPS.to_vec();
    property.values = values.to_vec();
}

impl SelectedFluidProperties {
    fn raw_index_or_add(&mut self, name: &str) -> usize {
        if let Some(index) = self.raw_glycols.iter().position(|g| g.name == name) {
            return index;
        }
        self.raw_glycols.push(RawGlycolProperties {
            name: name.into(),
            number: self.raw_glycols.len() + 1,
            ..Default::default()
        });
        self.raw_glycols.len() - 1
    }

    /// Actual EnergyPlusData init_constant_state guard stays at the parent caller.
    pub(crate) fn init_constant_state(&mut self) -> Result<(), SelectedFluidError> {
        // These two Source locals have allocated but undefined scalar values.
        let default_superheated_enthalpy = UninitializedSteamMatrix::allocate(114, 114);
        let default_superheated_density = UninitializedSteamMatrix::allocate(114, 114);
        self.refrigerants.push(RefrigerantProperties {
            name: "STEAM".into(),
            number: self.refrigerants.len() + 1,
            ..Default::default()
        });
        let steam = self
            .refrigerants
            .last_mut()
            .ok_or(SelectedFluidError::Unavailable("Steam retained owner"))?;
        steam.number_pressure_points = DEFAULT_STEAM_TEMPS.len();
        steam.number_enthalpy_points = DEFAULT_STEAM_TEMPS.len();
        steam.number_cp_points = DEFAULT_STEAM_TEMPS.len();
        steam.number_density_points = DEFAULT_STEAM_TEMPS.len();
        steam.pressure_temperatures = DEFAULT_STEAM_TEMPS.to_vec();
        steam.pressures = DEFAULT_STEAM_PRESS_DATA.to_vec();
        steam.enthalpy_temperatures = DEFAULT_STEAM_TEMPS.to_vec();
        steam.enthalpy_fluid.values = DEFAULT_STEAM_ENTHALPY_FLUID_DATA.to_vec();
        steam.enthalpy_gas_fluid.values = DEFAULT_STEAM_ENTHALPY_GAS_FLUID_DATA.to_vec();
        steam.cp_temperatures = DEFAULT_STEAM_TEMPS.to_vec();
        steam.cp_fluid.values = DEFAULT_STEAM_CP_FLUID_DATA.to_vec();
        steam.cp_gas_fluid.values = DEFAULT_STEAM_CP_GAS_FLUID_DATA.to_vec();
        steam.density_temperatures = DEFAULT_STEAM_TEMPS.to_vec();
        steam.density_fluid.values = DEFAULT_STEAM_DENSITY_FLUID_DATA.to_vec();
        steam.density_gas_fluid.values = DEFAULT_STEAM_DENSITY_GAS_FLUID_DATA.to_vec();
        steam.number_superheated_temperature_points = DEFAULT_STEAM_SUPERHEATED_TEMPS.len();
        steam.number_superheated_pressure_points = DEFAULT_STEAM_SUPERHEATED_PRESS_DATA.len();
        // Preserve destination allocations without reading/copying unknown f64.
        steam.superheated_enthalpy = UninitializedSteamMatrix::allocate(114, 114);
        steam.superheated_density = UninitializedSteamMatrix::allocate(114, 114);
        steam.superheated_temperatures = DEFAULT_STEAM_SUPERHEATED_TEMPS.to_vec();
        steam.superheated_pressures = DEFAULT_STEAM_SUPERHEATED_PRESS_DATA.to_vec();
        steam.superheated_enthalpy = default_superheated_enthalpy;
        steam.superheated_density = default_superheated_density;
        steam.set_temperature_limits()?;

        let water_index = self.raw_index_or_add("WATER");
        let water_raw = &mut self.raw_glycols[water_index];
        fill_raw(&mut water_raw.cp, &[0.0], &DEFAULT_WATER_CP_DATA);
        fill_raw(&mut water_raw.rho, &[0.0], &DEFAULT_WATER_RHO_DATA);
        fill_raw(
            &mut water_raw.conductivity,
            &[0.0],
            &DEFAULT_WATER_COND_DATA,
        );
        fill_raw(&mut water_raw.viscosity, &[0.0], &DEFAULT_WATER_VISC_DATA);

        let water_index = if let Some(index) = self.glycols.iter().position(|g| g.name == "WATER") {
            self.glycols[index].used = true; // actual GetGlycolNum lookup side effect
            index
        } else {
            self.glycols.push(GlycolProperties {
                name: "WATER".into(),
                glycol_name: "WATER".into(),
                used: true,
                number: self.glycols.len() + 1,
                ..Default::default()
            });
            self.glycols.len() - 1
        };
        let water = &mut self.glycols[water_index];
        water.concentration = 1.0;
        fill_mixture(&mut water.cp, &DEFAULT_WATER_CP_DATA);
        fill_mixture(&mut water.rho, &DEFAULT_WATER_RHO_DATA);
        fill_mixture(&mut water.conductivity, &DEFAULT_WATER_COND_DATA);
        fill_mixture(&mut water.viscosity, &DEFAULT_WATER_VISC_DATA);
        water.set_temperature_limits()?;
        Ok(())
    }

    /// Selected real getter: no noPlant gate, synthetic ready flag or private input map.
    /// Positive user data stops explicitly at the first unrepresented definition branch.
    pub(crate) fn get_fluid_properties_data(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), SelectedFluidError> {
        const FAMILIES: [&str; 6] = [
            "FluidProperties:Name",
            "FluidProperties:GlycolConcentration",
            "FluidProperties:Temperatures",
            "FluidProperties:Saturated",
            "FluidProperties:Superheated",
            "FluidProperties:Concentration",
        ];
        for family in FAMILIES {
            if input.get_num_objects_found(family) > 0 {
                return Err(SelectedFluidError::Unavailable(family));
            }
        }
        // Actual zero maxima: six local arrays, explicitly empty/+0/false-filled.
        let alphas: Vec<String> = Vec::new();
        let alpha_fields: Vec<String> = Vec::new();
        let alpha_blanks: Vec<bool> = Vec::new();
        let numbers: Vec<f64> = Vec::new();
        let numeric_fields: Vec<String> = Vec::new();
        let numeric_blanks: Vec<bool> = Vec::new();
        if input.get_num_objects_found("FluidProperties:Name") > 0 {
            return Err(SelectedFluidError::Unavailable("FluidProperties:Name"));
        }
        let fluid_temperatures: Vec<(String, usize, Vec<f64>)> = Vec::new();
        for refrigerant in &self.refrigerants {
            if refrigerant.pressures.is_empty()
                || refrigerant.enthalpy_fluid.values.is_empty()
                || refrigerant.enthalpy_gas_fluid.values.is_empty()
                || refrigerant.cp_fluid.values.is_empty()
                || refrigerant.cp_gas_fluid.values.is_empty()
                || refrigerant.density_fluid.values.is_empty()
                || refrigerant.density_gas_fluid.values.is_empty()
            {
                return Err(SelectedFluidError::Unavailable(
                    "required refrigerant-property diagnostic outside builtin domain",
                ));
            }
        }
        // The real superheated allocation pass skips Steam, not its unknown cells.
        for refrigerant in &self.refrigerants {
            if refrigerant.name != "STEAM" {
                return Err(SelectedFluidError::Unavailable(
                    "user refrigerant superheated allocation",
                ));
            }
        }
        for refrigerant in &mut self.refrigerants {
            refrigerant.set_temperature_limits()?;
        }
        let index = self.raw_index_or_add("ETHYLENEGLYCOL");
        let ethylene = &mut self.raw_glycols[index];
        fill_raw(
            &mut ethylene.cp,
            &DEFAULT_GLYCOL_CONCS,
            DEFAULT_ETH_GLY_CP_DATA.as_flattened(),
        );
        fill_raw(
            &mut ethylene.rho,
            &DEFAULT_GLYCOL_CONCS,
            DEFAULT_ETH_GLY_RHO_DATA.as_flattened(),
        );
        fill_raw(
            &mut ethylene.conductivity,
            &DEFAULT_GLYCOL_CONCS,
            DEFAULT_ETH_GLY_COND_DATA.as_flattened(),
        );
        fill_raw(
            &mut ethylene.viscosity,
            &DEFAULT_GLYCOL_CONCS,
            DEFAULT_ETH_GLY_VISC_DATA.as_flattened(),
        );
        let index = self.raw_index_or_add("PROPYLENEGLYCOL");
        let propylene = &mut self.raw_glycols[index];
        fill_raw(
            &mut propylene.cp,
            &DEFAULT_GLYCOL_CONCS,
            DEFAULT_PROP_GLY_CP_DATA.as_flattened(),
        );
        fill_raw(
            &mut propylene.rho,
            &DEFAULT_GLYCOL_CONCS,
            DEFAULT_PROP_GLY_RHO_DATA.as_flattened(),
        );
        fill_raw(
            &mut propylene.conductivity,
            &DEFAULT_GLYCOL_CONCS,
            DEFAULT_PROP_GLY_COND_DATA.as_flattened(),
        );
        fill_raw(
            &mut propylene.viscosity,
            &DEFAULT_GLYCOL_CONCS,
            DEFAULT_PROP_GLY_VISC_DATA.as_flattened(),
        );
        for glycol in &self.raw_glycols {
            for property in [
                &glycol.cp,
                &glycol.rho,
                &glycol.conductivity,
                &glycol.viscosity,
            ] {
                if !property.temperature_array_name.is_empty() {
                    return Err(SelectedFluidError::Unavailable(
                        "user raw glycol temperature arrays",
                    ));
                }
            }
        }
        if input.get_num_objects_found("FluidProperties:GlycolConcentration") > 0 {
            return Err(SelectedFluidError::Unavailable(
                "FluidProperties:GlycolConcentration",
            ));
        }
        drop(fluid_temperatures);
        drop(alphas);
        drop(alpha_fields);
        drop(alpha_blanks);
        drop(numbers);
        drop(numeric_fields);
        drop(numeric_blanks);
        if input.get_num_sections_found("REPORTGLYCOLS")? > 0 {
            self.debug_report_glycols = true;
        }
        if input.get_num_sections_found("REPORTREFRIGERANTS")? > 0 {
            self.debug_report_refrigerants = true;
        }
        if input.get_num_sections_found("INCREASEGLYCOLERRORLIMIT")? > 0 {
            self.glycol_error_limit_test = self
                .glycol_error_limit_test
                .checked_add(10)
                .ok_or(SelectedInputUnavailable::Size)?;
        }
        if input.get_num_sections_found("INCREASEREFRIGERANTERRORLIMIT")? > 0 {
            self.refrigerant_error_limit_test = self
                .refrigerant_error_limit_test
                .checked_add(10)
                .ok_or(SelectedInputUnavailable::Size)?;
        }
        if self.debug_report_glycols {
            return Err(SelectedFluidError::Unavailable("ReportAndTestGlycols"));
        }
        if self.debug_report_refrigerants {
            return Err(SelectedFluidError::Unavailable("ReportAndTestRefrigerants"));
        }
        Ok(())
    }
}
