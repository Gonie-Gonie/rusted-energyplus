/// Gas species shared by ordinary and equivalent-layer window gaps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowGasType {
    /// User-supplied thermophysical coefficients.
    Custom,
    /// Built-in dry-air coefficients.
    Air,
    /// Built-in argon coefficients.
    Argon,
    /// Built-in krypton coefficients.
    Krypton,
    /// Built-in xenon coefficients.
    Xenon,
}

impl WindowGasType {
    /// Returns the canonical EnergyPlus 26.1 display and EIO token.
    #[must_use]
    pub const fn energyplus_name(self) -> &'static str {
        match self {
            Self::Custom => "Custom",
            Self::Air => "Air",
            Self::Argon => "Argon",
            Self::Krypton => "Krypton",
            Self::Xenon => "Xenon",
        }
    }

    /// Parses an EnergyPlus gas-type token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value {
            "Custom" => Some(Self::Custom),
            "Air" => Some(Self::Air),
            "Argon" => Some(Self::Argon),
            "Krypton" => Some(Self::Krypton),
            "Xenon" => Some(Self::Xenon),
            _ => None,
        }
    }

    /// Parses the uppercase gas token used by
    /// WindowMaterial:Gap:EquivalentLayer in EnergyPlus 26.1 epJSON.
    #[must_use]
    pub fn from_equivalent_layer_energyplus_name(value: &str) -> Option<Self> {
        match value {
            "CUSTOM" => Some(Self::Custom),
            "AIR" => Some(Self::Air),
            "ARGON" => Some(Self::Argon),
            "KRYPTON" => Some(Self::Krypton),
            "XENON" => Some(Self::Xenon),
            _ => None,
        }
    }

    /// Returns the EnergyPlus 26.1 built-in properties for a standard gas.
    ///
    /// Custom has no built-in properties and returns None.
    #[must_use]
    pub const fn standard_properties(self) -> Option<WindowGasProperties> {
        let properties = match self {
            Self::Custom => return None,
            Self::Air => WindowGasProperties {
                conductivity: WindowGasPolynomialCoefficients {
                    coefficient_a: 2.873e-3,
                    coefficient_b: 7.760e-5,
                    coefficient_c: 0.0,
                },
                viscosity: WindowGasPolynomialCoefficients {
                    coefficient_a: 3.723e-6,
                    coefficient_b: 4.940e-8,
                    coefficient_c: 0.0,
                },
                specific_heat: WindowGasPolynomialCoefficients {
                    coefficient_a: 1002.737,
                    coefficient_b: 1.2324e-2,
                    coefficient_c: 0.0,
                },
                molecular_weight_g_per_mol: 28.97,
                specific_heat_ratio: 1.4,
            },
            Self::Argon => WindowGasProperties {
                conductivity: WindowGasPolynomialCoefficients {
                    coefficient_a: 2.285e-3,
                    coefficient_b: 5.149e-5,
                    coefficient_c: 0.0,
                },
                viscosity: WindowGasPolynomialCoefficients {
                    coefficient_a: 3.379e-6,
                    coefficient_b: 6.451e-8,
                    coefficient_c: 0.0,
                },
                specific_heat: WindowGasPolynomialCoefficients {
                    coefficient_a: 521.929,
                    coefficient_b: 0.0,
                    coefficient_c: 0.0,
                },
                molecular_weight_g_per_mol: 39.948,
                specific_heat_ratio: 1.67,
            },
            Self::Krypton => WindowGasProperties {
                conductivity: WindowGasPolynomialCoefficients {
                    coefficient_a: 9.443e-4,
                    coefficient_b: 2.826e-5,
                    coefficient_c: 0.0,
                },
                viscosity: WindowGasPolynomialCoefficients {
                    coefficient_a: 2.213e-6,
                    coefficient_b: 7.777e-8,
                    coefficient_c: 0.0,
                },
                specific_heat: WindowGasPolynomialCoefficients {
                    coefficient_a: 248.091,
                    coefficient_b: 0.0,
                    coefficient_c: 0.0,
                },
                molecular_weight_g_per_mol: 83.8,
                specific_heat_ratio: 1.68,
            },
            Self::Xenon => WindowGasProperties {
                conductivity: WindowGasPolynomialCoefficients {
                    coefficient_a: 4.538e-4,
                    coefficient_b: 1.723e-5,
                    coefficient_c: 0.0,
                },
                viscosity: WindowGasPolynomialCoefficients {
                    coefficient_a: 1.069e-6,
                    coefficient_b: 7.414e-8,
                    coefficient_c: 0.0,
                },
                specific_heat: WindowGasPolynomialCoefficients {
                    coefficient_a: 158.340,
                    coefficient_b: 0.0,
                    coefficient_c: 0.0,
                },
                molecular_weight_g_per_mol: 131.3,
                specific_heat_ratio: 1.66,
            },
        };
        Some(properties)
    }
}

/// Three-term EnergyPlus gas-property polynomial, A + B*T + C*T^2.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowGasPolynomialCoefficients {
    /// Constant coefficient A.
    pub coefficient_a: f64,
    /// Linear-temperature coefficient B.
    pub coefficient_b: f64,
    /// Squared-temperature coefficient C.
    pub coefficient_c: f64,
}

impl WindowGasPolynomialCoefficients {
    /// Evaluates the polynomial at an absolute temperature in kelvin.
    #[must_use]
    pub fn at_temperature_k(self, temperature_k: f64) -> f64 {
        self.coefficient_a
            + self.coefficient_b * temperature_k
            + self.coefficient_c * temperature_k * temperature_k
    }

    /// Evaluates the source-order 300 K expression with EnergyPlus 26.1's
    /// exact multiplication grouping.
    #[must_use]
    pub fn at_300_k(self) -> f64 {
        self.coefficient_a + self.coefficient_b * 300.0 + self.coefficient_c * 90000.0
    }
}

/// Resolved thermophysical properties for one window gas.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGasProperties {
    /// Thermal-conductivity coefficients.
    pub conductivity: WindowGasPolynomialCoefficients,
    /// Dynamic-viscosity coefficients.
    pub viscosity: WindowGasPolynomialCoefficients,
    /// Specific-heat coefficients.
    pub specific_heat: WindowGasPolynomialCoefficients,
    /// Molecular weight in g/mol.
    pub molecular_weight_g_per_mol: f64,
    /// Ratio of specific heats.
    pub specific_heat_ratio: f64,
}

/// Fully resolved WindowMaterial:Gas payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGasMaterial {
    /// Selected gas species.
    pub gas_type: WindowGasType,
    /// Gap thickness in meters.
    pub thickness_m: f64,
    /// User-supplied custom properties or source-fixed standard-gas properties.
    pub properties: WindowGasProperties,
}

impl WindowGasMaterial {
    /// Returns conductivity at the supplied absolute temperature.
    #[must_use]
    pub fn conductivity_at_temperature_k(self, temperature_k: f64) -> f64 {
        self.properties.conductivity.at_temperature_k(temperature_k)
    }

    /// Returns the source-order nominal resistance evaluated at 300 K.
    #[must_use]
    pub fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        let conductivity = self.properties.conductivity.at_300_k();
        (self.thickness_m > 0.0 && conductivity > 0.0).then_some(self.thickness_m / conductivity)
    }
}

/// Built-in gas species permitted by `WindowMaterial:GasMixture`.
///
/// EnergyPlus 26.1 does not permit `Custom` in this object, so this type keeps
/// that invalid state out of the typed model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowStandardGasType {
    /// Built-in dry-air coefficients.
    Air,
    /// Built-in argon coefficients.
    Argon,
    /// Built-in krypton coefficients.
    Krypton,
    /// Built-in xenon coefficients.
    Xenon,
}

impl WindowStandardGasType {
    /// Returns the canonical EnergyPlus 26.1 epJSON and display token.
    #[must_use]
    pub const fn energyplus_name(self) -> &'static str {
        match self {
            Self::Air => "Air",
            Self::Argon => "Argon",
            Self::Krypton => "Krypton",
            Self::Xenon => "Xenon",
        }
    }

    /// Parses the exact EnergyPlus 26.1 epJSON gas token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value {
            "Air" => Some(Self::Air),
            "Argon" => Some(Self::Argon),
            "Krypton" => Some(Self::Krypton),
            "Xenon" => Some(Self::Xenon),
            _ => None,
        }
    }

    /// Returns the corresponding shared window-gas species.
    #[must_use]
    pub const fn as_window_gas_type(self) -> WindowGasType {
        match self {
            Self::Air => WindowGasType::Air,
            Self::Argon => WindowGasType::Argon,
            Self::Krypton => WindowGasType::Krypton,
            Self::Xenon => WindowGasType::Xenon,
        }
    }

    /// Returns the EnergyPlus 26.1 built-in properties for this gas.
    #[must_use]
    pub fn properties(self) -> WindowGasProperties {
        match self.as_window_gas_type().standard_properties() {
            Some(properties) => properties,
            None => unreachable!("WindowStandardGasType cannot represent Custom"),
        }
    }
}

/// One active gas and its input fraction in a window gas mixture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGasMixtureComponent {
    /// Built-in gas species. Custom gases are unrepresentable here.
    pub gas_type: WindowStandardGasType,
    /// Input fraction, preserved without normalization.
    pub fraction: f64,
}

impl WindowGasMixtureComponent {
    /// Returns the source-fixed properties for this component's gas species.
    #[must_use]
    pub fn properties(self) -> WindowGasProperties {
        self.gas_type.properties()
    }
}

/// Active-prefix representation for the one-to-four gases accepted by
/// `WindowMaterial:GasMixture`.
///
/// The variant encodes the active count, so invalid counts and holes in the
/// active prefix cannot be represented.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowGasMixture {
    /// One active gas.
    One([WindowGasMixtureComponent; 1]),
    /// Two active gases.
    Two([WindowGasMixtureComponent; 2]),
    /// Three active gases.
    Three([WindowGasMixtureComponent; 3]),
    /// Four active gases.
    Four([WindowGasMixtureComponent; 4]),
}

impl WindowGasMixture {
    /// Returns the active components in source order.
    #[must_use]
    pub const fn components(&self) -> &[WindowGasMixtureComponent] {
        match self {
            Self::One(components) => components,
            Self::Two(components) => components,
            Self::Three(components) => components,
            Self::Four(components) => components,
        }
    }

    /// Returns the encoded active gas count.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.components().len()
    }

    /// Returns false because every variant contains at least one component.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

/// Fully resolved `WindowMaterial:GasMixture` payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGasMixtureMaterial {
    /// Gap thickness in meters.
    pub thickness_m: f64,
    /// One-to-four active gas components in source order.
    pub gases: WindowGasMixture,
}

impl WindowGasMixtureMaterial {
    /// Returns the source-order nominal resistance evaluated from only the
    /// first gas's conductivity at 300 K.
    #[must_use]
    pub fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        let first = self.gases.components().first()?;
        let conductivity = first.properties().conductivity.at_300_k();
        (self.thickness_m > 0.0 && conductivity > 0.0).then_some(self.thickness_m / conductivity)
    }
}

/// Venting mode for an equivalent-layer window gap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowGapVentType {
    /// Gas-tight gap with no indoor or outdoor venting.
    Sealed,
    /// Gap vented to the indoor environment.
    VentedIndoor,
    /// Gap vented to the outdoor environment.
    VentedOutdoor,
}

impl WindowGapVentType {
    /// Returns the canonical EnergyPlus 26.1 display and EIO token.
    #[must_use]
    pub const fn energyplus_name(self) -> &'static str {
        match self {
            Self::Sealed => "Sealed",
            Self::VentedIndoor => "VentedIndoor",
            Self::VentedOutdoor => "VentedOutdoor",
        }
    }

    /// Parses the exact EnergyPlus epJSON vent-type token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value {
            "Sealed" => Some(Self::Sealed),
            "VentedIndoor" => Some(Self::VentedIndoor),
            "VentedOutdoor" => Some(Self::VentedOutdoor),
            _ => None,
        }
    }
}

/// Fully resolved WindowMaterial:Gap:EquivalentLayer payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowGapEquivalentLayerMaterial {
    /// Selected gas species.
    pub gas_type: WindowGasType,
    /// Gap thickness in meters.
    pub thickness_m: f64,
    /// Indoor/outdoor venting relationship.
    pub gap_vent_type: WindowGapVentType,
    /// User-supplied custom properties or source-fixed standard-gas properties.
    pub properties: WindowGasProperties,
}

impl WindowGapEquivalentLayerMaterial {
    /// Returns conductivity at the supplied absolute temperature.
    #[must_use]
    pub fn conductivity_at_temperature_k(self, temperature_k: f64) -> f64 {
        self.properties.conductivity.at_temperature_k(temperature_k)
    }

    /// Returns the source-order nominal resistance evaluated at 300 K.
    #[must_use]
    pub fn nominal_thermal_resistance_m2_k_per_w(self) -> Option<f64> {
        let conductivity = self.properties.conductivity.at_300_k();
        (self.thickness_m > 0.0 && conductivity > 0.0).then_some(self.thickness_m / conductivity)
    }
}
