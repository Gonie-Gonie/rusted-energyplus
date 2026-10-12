//! Literal first/last valid table bounds; Source failure overwrites are retained.
use super::{SelectedFluidError, owners::*};

impl GlycolProperty {
    fn set_temperature_limits(&mut self) -> Result<(), SelectedFluidError> {
        if !self.data_present {
            return Ok(());
        }
        let n = self.number_temperature_points;
        if self.values.len() < n || self.temperatures.len() < n {
            return Err(SelectedFluidError::Unavailable("glycol property storage"));
        }
        for (i, value) in self.values.iter().enumerate().take(n) {
            if *value <= 0.0 {
                continue;
            }
            self.low_temperature_index = i + 1;
            self.low_temperature_value = self.temperatures[i];
            break;
        }
        for (i, value) in self.values.iter().enumerate().take(n).rev() {
            if *value <= 0.0 {
                continue;
            }
            self.high_temperature_index = i + 1;
            self.high_temperature_value = self.temperatures[i];
            break;
        }
        Ok(())
    }
    fn missing_bounds(&self) -> bool {
        self.low_temperature_index == 0 || self.high_temperature_index == 0
    }
}
impl GlycolProperties {
    pub(super) fn set_temperature_limits(&mut self) -> Result<(), SelectedFluidError> {
        self.cp.set_temperature_limits()?;
        self.rho.set_temperature_limits()?;
        self.conductivity.set_temperature_limits()?;
        self.viscosity.set_temperature_limits()?;
        let mut failure = false;
        if self.cp.data_present {
            failure = self.cp.missing_bounds();
        }
        if self.rho.data_present {
            failure = self.rho.missing_bounds();
        }
        if self.conductivity.data_present {
            failure = self.conductivity.missing_bounds();
        }
        if self.viscosity.data_present {
            failure = self.viscosity.missing_bounds();
        }
        if failure {
            return Err(SelectedFluidError::Unavailable(
                "glycol zero-table diagnostic outside builtin domain",
            ));
        }
        Ok(())
    }
}

impl RefrigerantValueSeries {
    fn set_temperature_limits(&mut self, n: usize) -> Result<(), SelectedFluidError> {
        if self.values.len() < n {
            return Err(SelectedFluidError::Unavailable(
                "refrigerant property storage",
            ));
        }
        for (i, value) in self.values.iter().enumerate().take(n) {
            if *value <= 0.0 {
                continue;
            }
            self.low_temperature_value = *value;
            self.low_temperature_index = i + 1;
            break;
        }
        for (i, value) in self.values.iter().enumerate().take(n).rev() {
            if *value <= 0.0 {
                continue;
            }
            self.high_temperature_value = *value;
            self.high_temperature_index = i + 1;
            break;
        }
        Ok(())
    }
    fn missing_bounds(&self) -> bool {
        self.low_temperature_index == 0 || self.high_temperature_index == 0
    }
}
impl RefrigerantProperties {
    pub(super) fn set_temperature_limits(&mut self) -> Result<(), SelectedFluidError> {
        let n = self.number_pressure_points;
        if self.pressures.len() < n || self.pressure_temperatures.len() < n {
            return Err(SelectedFluidError::Unavailable(
                "refrigerant pressure storage",
            ));
        }
        for (i, value) in self.pressures.iter().enumerate().take(n) {
            if *value <= 0.0 {
                continue;
            }
            self.low_pressure_index = i + 1;
            self.low_pressure_value = *value;
            self.pressure_low_temperature_value = self.pressure_temperatures[i];
            self.pressure_low_temperature_index = i + 1;
            break;
        }
        for (i, value) in self.pressures.iter().enumerate().take(n).rev() {
            if *value <= 0.0 {
                continue;
            }
            self.high_pressure_index = i + 1;
            self.high_pressure_value = *value;
            self.pressure_high_temperature_value = self.pressure_temperatures[i];
            self.pressure_high_temperature_index = i + 1;
            break;
        }
        self.enthalpy_fluid
            .set_temperature_limits(self.number_enthalpy_points)?;
        self.enthalpy_gas_fluid
            .set_temperature_limits(self.number_enthalpy_points)?;
        self.cp_fluid
            .set_temperature_limits(self.number_cp_points)?;
        self.cp_gas_fluid
            .set_temperature_limits(self.number_cp_points)?;
        self.density_fluid
            .set_temperature_limits(self.number_density_points)?;
        self.density_gas_fluid
            .set_temperature_limits(self.number_density_points)?;
        let mut failure = false;
        if self.number_pressure_points > 0 {
            failure = self.low_pressure_index == 0
                || self.pressure_low_temperature_index == 0
                || self.high_pressure_index == 0
                || self.pressure_high_temperature_index == 0;
        }
        if self.number_enthalpy_points > 0 {
            failure =
                self.enthalpy_fluid.missing_bounds() || self.enthalpy_gas_fluid.missing_bounds();
        }
        if self.number_cp_points > 0 {
            failure = self.cp_fluid.missing_bounds() || self.cp_gas_fluid.missing_bounds();
        }
        if self.number_density_points > 0 {
            failure =
                self.density_fluid.missing_bounds() || self.density_gas_fluid.missing_bounds();
        }
        if failure {
            return Err(SelectedFluidError::Unavailable(
                "refrigerant zero-table diagnostic outside builtin domain",
            ));
        }
        Ok(())
    }
}
