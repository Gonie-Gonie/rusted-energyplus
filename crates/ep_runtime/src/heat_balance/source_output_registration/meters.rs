//! Actual EnergyTransfer/Building Heating/Cooling AttachMeters/AddMeter path.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceMeterEndUse {
    Invalid = -1,
    Heating = 0,
    Cooling = 1,
    ExteriorLights = 3,
    InteriorEquipment = 4,
    Cogeneration = 13,
    CarbonEquivalentEmissions = 16,
}
impl SourceMeterEndUse {
    pub(super) fn name(self) -> Result<&'static str, OrdinaryRadiationError> {
        match self {
            Self::Heating => Ok("Heating"),
            Self::Cooling => Ok("Cooling"),
            Self::ExteriorLights => Ok("ExteriorLights"),
            Self::InteriorEquipment => Ok("InteriorEquipment"),
            Self::Cogeneration => Ok("Cogeneration"),
            Self::CarbonEquivalentEmissions => Ok("CarbonEquivalentEmissions"),
            Self::Invalid => Err(OrdinaryRadiationUnavailable::InputBranch.into()),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceMeterGroup {
    Invalid = -1,
    Building = 0,
    Hvac = 1,
    Plant = 2,
    Zone = 3,
    SpaceType = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceMeterResource {
    Invalid = -1,
    Electricity = 0,
    NaturalGas = 1,
    Gasoline = 2,
    Diesel = 3,
    Coal = 4,
    Propane = 5,
    FuelOilNo1 = 6,
    FuelOilNo2 = 7,
    OtherFuel1 = 8,
    OtherFuel2 = 9,
    DistrictCooling = 10,
    DistrictHeatingWater = 11,
    DistrictHeatingSteam = 12,
    EnergyTransfer = 15,
    ElectricityPurchased = 17,
    ElectricitySurplusSold = 18,
    ElectricityNet = 19,
    CarbonEquivalent = 22,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceMeterType {
    Invalid = -1,
    Normal = 0,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceMeterIpUnits {
    OtherJ = 0,
    Electricity = 1,
    Gas = 2,
    Cooling = 3,
    OtherKG = 5,
}

/// Literal meter arguments passed by the real demand setup call.
#[derive(Clone, Copy)]
pub(crate) struct SourceEnergyTransferMeterArguments<'a> {
    pub(crate) end_use: SourceMeterEndUse,
    pub(crate) end_use_subcategory: &'a str,
    pub(crate) zone_name: &'a str,
    pub(crate) zone_multiplier: i32,
    pub(crate) zone_list_multiplier: i32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SourceMeterPeriod {
    pub(crate) value: f64,
    pub(crate) maximum: f64,
    pub(crate) maximum_date: i32,
    pub(crate) minimum: f64,
    pub(crate) minimum_date: i32,
    pub(crate) report: bool,
    pub(crate) report_file_only: bool,
    pub(crate) report_number: i32,
    pub(crate) accumulated_report: bool,
    pub(crate) accumulated_file_only: bool,
    pub(crate) accumulated_report_number: i32,
}
impl Default for SourceMeterPeriod {
    fn default() -> Self {
        Self {
            value: 0.0,
            maximum: -99999999999999.0,
            maximum_date: -1,
            minimum: 99999999999999.0,
            minimum_date: -1,
            report: false,
            report_file_only: false,
            report_number: 0,
            accumulated_report: false,
            accumulated_file_only: false,
            accumulated_report_number: 0,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceMeter {
    pub(crate) name: String,
    pub(crate) meter_type: SourceMeterType,
    pub(crate) resource: SourceMeterResource,
    pub(crate) end_use: SourceMeterEndUse,
    pub(crate) end_use_subcategory: String,
    pub(crate) group: SourceMeterGroup,
    pub(crate) units: SourceOutputUnits,
    /// Native leaves this automatic member unwritten until GetResourceIPUnits.
    pub(crate) ip_units: Option<SourceMeterIpUnits>,
    pub(crate) current_time_step_value: f64,
    pub(crate) index_group: String,
    pub(crate) periods: [SourceMeterPeriod; 7],
    pub(crate) period_last_simulation: SourceMeterPeriod,
    pub(crate) period_final_year_simulation: SourceMeterPeriod,
    pub(crate) destination_meter_numbers: Vec<i32>,
    pub(crate) decrement_meter_number: i32,
    pub(crate) source_variable_numbers: Vec<usize>,
    pub(crate) source_meter_numbers: Vec<i32>,
}
impl SourceMeter {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            meter_type: SourceMeterType::Invalid,
            resource: SourceMeterResource::Invalid,
            end_use: SourceMeterEndUse::Invalid,
            end_use_subcategory: String::new(),
            group: SourceMeterGroup::Invalid,
            units: SourceOutputUnits::Invalid,
            ip_units: None,
            current_time_step_value: 0.0,
            index_group: String::new(),
            periods: [SourceMeterPeriod::default(); 7],
            period_last_simulation: SourceMeterPeriod::default(),
            period_final_year_simulation: SourceMeterPeriod::default(),
            destination_meter_numbers: Vec::new(),
            decrement_meter_number: -1,
            source_variable_numbers: Vec::new(),
            source_meter_numbers: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceMeters {
    pub(crate) meters: Vec<SourceMeter>,
    pub(crate) meter_map: std::collections::BTreeMap<String, usize>,
}
impl SourceOutputRegistration {
    fn add_source_end_use_subcategory(
        &mut self,
        end_use: SourceMeterEndUse,
        name: &str,
    ) -> Result<(), OrdinaryRadiationError> {
        let category_index = match end_use {
            SourceMeterEndUse::Heating => 0,
            SourceMeterEndUse::Cooling => 1,
            SourceMeterEndUse::Invalid
            | SourceMeterEndUse::ExteriorLights
            | SourceMeterEndUse::InteriorEquipment
            | SourceMeterEndUse::Cogeneration
            | SourceMeterEndUse::CarbonEquivalentEmissions => {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
        };
        let category = self
            .end_use_categories
            .as_mut()
            .and_then(|r| r.get_mut(category_index))
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        for number in 1..=category.num_subcategories {
            let i = usize::try_from(number - 1)
                .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?;
            let prior = category
                .subcategory_names
                .as_ref()
                .and_then(|r| r.get(i))
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            if prior.eq_ignore_ascii_case(name) {
                return Ok(());
            }
        }
        category.num_subcategories = category
            .num_subcategories
            .checked_add(1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        let count = usize::try_from(category.num_subcategories)
            .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?;
        let names = category.subcategory_names.get_or_insert_with(Vec::new);
        names.resize(count, String::new());
        *names
            .get_mut(count - 1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)? = name.into();
        if category.num_subcategories > self.max_num_subcategories {
            self.max_num_subcategories = category.num_subcategories;
        }
        Ok(())
    }
    fn add_source_energy_transfer_meter(
        &mut self,
        name: &str,
        units: SourceOutputUnits,
        end_use: SourceMeterEndUse,
        subcategory: &str,
        group: SourceMeterGroup,
        out_var_number: usize,
    ) -> Result<(), OrdinaryRadiationError> {
        let uppercase = name.to_ascii_uppercase();
        let number = if let Some(&number) = self.meters.meter_map.get(&uppercase) {
            number
        } else {
            let number = self.meters.meters.len();
            self.meters.meters.push(SourceMeter::new(name));
            self.meters.meter_map.insert(uppercase, number);
            let meter = &mut self.meters.meters[number];
            meter.meter_type = SourceMeterType::Normal;
            meter.resource = SourceMeterResource::EnergyTransfer;
            meter.end_use = end_use;
            meter.end_use_subcategory = subcategory.into();
            meter.group = group;
            meter.units = units;
            meter.current_time_step_value = 0.0;
            // Source explicitly orders Year before Simulation for counter writes,
            // although their actual enum indices are6 and5 respectively.
            let frequencies = [
                SourceReportFrequency::TimeStep,
                SourceReportFrequency::Hour,
                SourceReportFrequency::Day,
                SourceReportFrequency::Month,
                SourceReportFrequency::Year,
                SourceReportFrequency::Simulation,
            ];
            for frequency in frequencies {
                self.report_number_counter = self
                    .report_number_counter
                    .checked_add(1)
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                meter.periods[frequency as usize].report_number = self.report_number_counter;
            }
            for frequency in frequencies {
                self.report_number_counter = self
                    .report_number_counter
                    .checked_add(1)
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                meter.periods[frequency as usize].accumulated_report_number =
                    self.report_number_counter;
            }
            // Real GetResourceIPUnits default resource branch, J -> OtherJ;
            // Other units stop at the first unported GetResourceIPUnits domain.
            if units != SourceOutputUnits::J {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
            meter.ip_units = Some(SourceMeterIpUnits::OtherJ);
            number
        };
        self.out_vars
            .get_mut(out_var_number)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
            .meter_numbers
            .push(i32::try_from(number).map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?);
        self.meters
            .meters
            .get_mut(number)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
            .source_variable_numbers
            .push(out_var_number);
        Ok(())
    }
    pub(super) fn attach_source_energy_transfer_meters(
        &mut self,
        units: SourceOutputUnits,
        args: SourceEnergyTransferMeterArguments<'_>,
        out_var_number: usize,
    ) -> Result<(), OrdinaryRadiationError> {
        let end_use = args.end_use.name()?;
        // standardizeEndUseSub1523: genuine Heating/Cooling map to EndUse,
        // so the default empty argument becomes General, not an empty submeter.
        let subcategory = if args.end_use_subcategory.is_empty() {
            "General"
        } else {
            args.end_use_subcategory
        };
        self.add_source_end_use_subcategory(args.end_use, subcategory)?;
        self.add_source_energy_transfer_meter(
            "EnergyTransfer:Facility",
            units,
            SourceMeterEndUse::Invalid,
            "",
            SourceMeterGroup::Invalid,
            out_var_number,
        )?;
        self.add_source_energy_transfer_meter(
            "EnergyTransfer:Building",
            units,
            SourceMeterEndUse::Invalid,
            "",
            SourceMeterGroup::Building,
            out_var_number,
        )?;
        if !args.zone_name.is_empty() {
            self.add_source_energy_transfer_meter(
                &format!("EnergyTransfer:Zone:{}", args.zone_name),
                units,
                SourceMeterEndUse::Invalid,
                "",
                SourceMeterGroup::Zone,
                out_var_number,
            )?;
        }
        self.add_source_energy_transfer_meter(
            &format!("{end_use}:EnergyTransfer"),
            units,
            args.end_use,
            "",
            SourceMeterGroup::Invalid,
            out_var_number,
        )?;
        if !args.zone_name.is_empty() {
            self.add_source_energy_transfer_meter(
                &format!("{end_use}:EnergyTransfer:Zone:{}", args.zone_name),
                units,
                args.end_use,
                "",
                SourceMeterGroup::Zone,
                out_var_number,
            )?;
        }
        if !subcategory.is_empty() {
            self.add_source_energy_transfer_meter(
                &format!("{subcategory}:{end_use}:EnergyTransfer"),
                units,
                args.end_use,
                subcategory,
                SourceMeterGroup::Invalid,
                out_var_number,
            )?;
            if !args.zone_name.is_empty() {
                self.add_source_energy_transfer_meter(
                    &format!(
                        "{subcategory}:{end_use}:EnergyTransfer:Zone:{}",
                        args.zone_name
                    ),
                    units,
                    args.end_use,
                    subcategory,
                    SourceMeterGroup::Zone,
                    out_var_number,
                )?;
            }
        }
        Ok(())
    }
}

impl SourceOutputRegistration {
    fn add_source_general_end_use_subcategory(
        &mut self,
        end_use: SourceMeterEndUse,
        name: &str,
    ) -> Result<(), OrdinaryRadiationError> {
        let category_index = match end_use {
            SourceMeterEndUse::Heating => 0,
            SourceMeterEndUse::Cooling => 1,
            SourceMeterEndUse::ExteriorLights => 3,
            SourceMeterEndUse::InteriorEquipment => 4,
            SourceMeterEndUse::Cogeneration => 13,
            SourceMeterEndUse::Invalid | SourceMeterEndUse::CarbonEquivalentEmissions => {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
        };
        let category = self
            .end_use_categories
            .as_mut()
            .and_then(|r| r.get_mut(category_index))
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        for number in 1..=category.num_subcategories {
            let i = usize::try_from(number - 1)
                .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?;
            let prior = category
                .subcategory_names
                .as_ref()
                .and_then(|r| r.get(i))
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            if prior.eq_ignore_ascii_case(name) {
                return Ok(());
            }
        }
        category.num_subcategories = category
            .num_subcategories
            .checked_add(1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        let count = usize::try_from(category.num_subcategories)
            .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?;
        let names = category.subcategory_names.get_or_insert_with(Vec::new);
        names.resize(count, String::new());
        *names
            .get_mut(count - 1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)? = name.into();
        if category.num_subcategories > self.max_num_subcategories {
            self.max_num_subcategories = category.num_subcategories;
        }
        Ok(())
    }
    fn add_source_general_end_use_space_type(
        &mut self,
        end_use: SourceMeterEndUse,
        name: &str,
    ) -> Result<(), OrdinaryRadiationError> {
        let category_index = match end_use {
            SourceMeterEndUse::Heating => 0,
            SourceMeterEndUse::Cooling => 1,
            SourceMeterEndUse::ExteriorLights => 3,
            SourceMeterEndUse::InteriorEquipment => 4,
            SourceMeterEndUse::Cogeneration => 13,
            SourceMeterEndUse::Invalid | SourceMeterEndUse::CarbonEquivalentEmissions => {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
        };
        let category = self
            .end_use_categories
            .as_mut()
            .and_then(|r| r.get_mut(category_index))
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        for number in 1..=category.num_space_types {
            let i = usize::try_from(number - 1)
                .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?;
            let prior = category
                .space_type_names
                .as_ref()
                .and_then(|r| r.get(i))
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            if prior.eq_ignore_ascii_case(name) {
                return Ok(());
            }
        }
        category.num_space_types = category
            .num_space_types
            .checked_add(1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        let count = usize::try_from(category.num_space_types)
            .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?;
        let names = category.space_type_names.get_or_insert_with(Vec::new);
        names.resize(count, String::new());
        *names
            .get_mut(count - 1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)? = name.into();
        if category.num_space_types > self.max_num_end_use_space_types {
            self.max_num_end_use_space_types = category.num_space_types;
        }
        Ok(())
    }
    fn add_source_general_meter(
        &mut self,
        name: &str,
        units: SourceOutputUnits,
        resource: SourceMeterResource,
        end_use: SourceMeterEndUse,
        subcategory: &str,
        group: SourceMeterGroup,
        out_var_number: usize,
    ) -> Result<(), OrdinaryRadiationError> {
        let uppercase = name.to_ascii_uppercase();
        let number = if let Some(&number) = self.meters.meter_map.get(&uppercase) {
            number
        } else {
            let number = self.meters.meters.len();
            self.meters.meters.push(SourceMeter::new(name));
            self.meters.meter_map.insert(uppercase, number);
            let meter = &mut self.meters.meters[number];
            meter.meter_type = SourceMeterType::Normal;
            meter.resource = resource;
            meter.end_use = end_use;
            meter.end_use_subcategory = subcategory.into();
            meter.group = group;
            meter.units = units;
            meter.current_time_step_value = 0.0;
            // Source explicitly orders Year before Simulation for counter writes,
            // although their actual enum indices are6 and5 respectively.
            let frequencies = [
                SourceReportFrequency::TimeStep,
                SourceReportFrequency::Hour,
                SourceReportFrequency::Day,
                SourceReportFrequency::Month,
                SourceReportFrequency::Year,
                SourceReportFrequency::Simulation,
            ];
            for frequency in frequencies {
                self.report_number_counter = self
                    .report_number_counter
                    .checked_add(1)
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                meter.periods[frequency as usize].report_number = self.report_number_counter;
            }
            for frequency in frequencies {
                self.report_number_counter = self
                    .report_number_counter
                    .checked_add(1)
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                meter.periods[frequency as usize].accumulated_report_number =
                    self.report_number_counter;
            }
            // Actual GetResourceIPUnits Electricity or default J/kg branches;
            // other units stop before the unported warning call/IP assignment.
            if !matches!(units, SourceOutputUnits::J | SourceOutputUnits::Kg) {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
            meter.ip_units = Some(match resource {
                SourceMeterResource::Electricity
                | SourceMeterResource::ElectricityPurchased
                | SourceMeterResource::ElectricitySurplusSold
                | SourceMeterResource::ElectricityNet => SourceMeterIpUnits::Electricity,
                SourceMeterResource::NaturalGas => SourceMeterIpUnits::Gas,
                SourceMeterResource::DistrictCooling => SourceMeterIpUnits::Cooling,
                SourceMeterResource::DistrictHeatingWater
                | SourceMeterResource::DistrictHeatingSteam
                | SourceMeterResource::Gasoline
                | SourceMeterResource::Diesel
                | SourceMeterResource::Coal
                | SourceMeterResource::FuelOilNo1
                | SourceMeterResource::FuelOilNo2
                | SourceMeterResource::OtherFuel1
                | SourceMeterResource::OtherFuel2
                | SourceMeterResource::Propane
                | SourceMeterResource::EnergyTransfer
                | SourceMeterResource::CarbonEquivalent => {
                    if units == SourceOutputUnits::Kg {
                        SourceMeterIpUnits::OtherKG
                    } else {
                        SourceMeterIpUnits::OtherJ
                    }
                }
                SourceMeterResource::Invalid => {
                    return Err(OrdinaryRadiationUnavailable::InputBranch.into());
                }
            });
            number
        };
        self.out_vars
            .get_mut(out_var_number)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
            .meter_numbers
            .push(i32::try_from(number).map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?);
        self.meters
            .meters
            .get_mut(number)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
            .source_variable_numbers
            .push(out_var_number);
        Ok(())
    }
}

/// Actual optional meter arguments, independently selecting Resource/Group/EndUse.
#[derive(Clone, Copy)]
pub(crate) struct SourceGeneralMeterArguments<'a> {
    pub(crate) resource: SourceMeterResource,
    pub(crate) end_use: SourceMeterEndUse,
    pub(crate) end_use_subcategory: &'a str,
    pub(crate) group: SourceMeterGroup,
    pub(crate) zone_name: &'a str,
    pub(crate) space_type: &'a str,
    pub(crate) zone_multiplier: i32,
    pub(crate) zone_list_multiplier: i32,
}
impl SourceMeterResource {
    pub(super) fn source_name(self) -> Result<&'static str, OrdinaryRadiationError> {
        match self {
            Self::Electricity => Ok("Electricity"),
            Self::NaturalGas => Ok("NaturalGas"),
            Self::Gasoline => Ok("Gasoline"),
            Self::Diesel => Ok("Diesel"),
            Self::Coal => Ok("Coal"),
            Self::Propane => Ok("Propane"),
            Self::FuelOilNo1 => Ok("FuelOilNo1"),
            Self::FuelOilNo2 => Ok("FuelOilNo2"),
            Self::OtherFuel1 => Ok("OtherFuel1"),
            Self::OtherFuel2 => Ok("OtherFuel2"),
            Self::DistrictCooling => Ok("DistrictCooling"),
            Self::DistrictHeatingWater => Ok("DistrictHeatingWater"),
            Self::DistrictHeatingSteam => Ok("DistrictHeatingSteam"),
            Self::EnergyTransfer => Ok("EnergyTransfer"),
            Self::ElectricityPurchased => Ok("ElectricityPurchased"),
            Self::ElectricitySurplusSold => Ok("ElectricitySurplusSold"),
            Self::ElectricityNet => Ok("ElectricityNet"),
            Self::CarbonEquivalent => Ok("Carbon Equivalent"),
            Self::Invalid => Err(OrdinaryRadiationUnavailable::InputBranch.into()),
        }
    }
}
impl SourceMeterGroup {
    pub(super) fn source_name(self) -> Result<&'static str, OrdinaryRadiationError> {
        match self {
            Self::Building => Ok("Building"),
            Self::Hvac => Ok("HVAC"),
            Self::Plant => Ok("Plant"),
            Self::Zone => Ok("Zone"),
            Self::SpaceType => Ok("SpaceType"),
            Self::Invalid => Err(OrdinaryRadiationUnavailable::InputBranch.into()),
        }
    }
}
impl SourceOutputRegistration {
    pub(super) fn attach_source_general_meters(
        &mut self,
        units: SourceOutputUnits,
        args: SourceGeneralMeterArguments<'_>,
        out_var_number: usize,
    ) -> Result<(), OrdinaryRadiationError> {
        let resource = args.resource.source_name()?;
        let subcategory = if !args.end_use_subcategory.is_empty() {
            args.end_use_subcategory
        } else if matches!(
            args.end_use,
            SourceMeterEndUse::Heating
                | SourceMeterEndUse::Cooling
                | SourceMeterEndUse::ExteriorLights
                | SourceMeterEndUse::InteriorEquipment
                | SourceMeterEndUse::Cogeneration
        ) {
            "General"
        } else {
            ""
        };
        if !subcategory.is_empty() {
            self.add_source_general_end_use_subcategory(args.end_use, subcategory)?;
        }
        if !args.space_type.is_empty() {
            self.add_source_general_end_use_space_type(args.end_use, args.space_type)?;
        }
        self.add_source_general_meter(
            &format!("{resource}:Facility"),
            units,
            args.resource,
            SourceMeterEndUse::Invalid,
            "",
            SourceMeterGroup::Invalid,
            out_var_number,
        )?;
        if args.group != SourceMeterGroup::Invalid {
            self.add_source_general_meter(
                &format!("{resource}:{}", args.group.source_name()?),
                units,
                args.resource,
                SourceMeterEndUse::Invalid,
                "",
                args.group,
                out_var_number,
            )?;
            if args.group == SourceMeterGroup::Building {
                if !args.zone_name.is_empty() {
                    self.add_source_general_meter(
                        &format!("{resource}:Zone:{}", args.zone_name),
                        units,
                        args.resource,
                        SourceMeterEndUse::Invalid,
                        "",
                        SourceMeterGroup::Zone,
                        out_var_number,
                    )?;
                }
                if !args.space_type.is_empty() {
                    self.add_source_general_meter(
                        &format!("{resource}:SpaceType:{}", args.space_type),
                        units,
                        args.resource,
                        SourceMeterEndUse::Invalid,
                        "",
                        SourceMeterGroup::SpaceType,
                        out_var_number,
                    )?;
                }
            }
        }
        if args.end_use != SourceMeterEndUse::Invalid {
            let end_use = args.end_use.name()?;
            self.add_source_general_meter(
                &format!("{end_use}:{resource}"),
                units,
                args.resource,
                args.end_use,
                "",
                SourceMeterGroup::Invalid,
                out_var_number,
            )?;
            if args.group == SourceMeterGroup::Building {
                if !args.zone_name.is_empty() {
                    self.add_source_general_meter(
                        &format!("{end_use}:{resource}:Zone:{}", args.zone_name),
                        units,
                        args.resource,
                        args.end_use,
                        "",
                        SourceMeterGroup::Zone,
                        out_var_number,
                    )?;
                }
                if !args.space_type.is_empty() {
                    self.add_source_general_meter(
                        &format!("{end_use}:{resource}:SpaceType:{}", args.space_type),
                        units,
                        args.resource,
                        args.end_use,
                        "",
                        SourceMeterGroup::SpaceType,
                        out_var_number,
                    )?;
                }
            }
            if !subcategory.is_empty() {
                self.add_source_general_meter(
                    &format!("{subcategory}:{end_use}:{resource}"),
                    units,
                    args.resource,
                    args.end_use,
                    subcategory,
                    SourceMeterGroup::Invalid,
                    out_var_number,
                )?;
                if args.group == SourceMeterGroup::Building {
                    if !args.zone_name.is_empty() {
                        self.add_source_general_meter(
                            &format!("{subcategory}:{end_use}:{resource}:Zone:{}", args.zone_name),
                            units,
                            args.resource,
                            args.end_use,
                            subcategory,
                            SourceMeterGroup::Zone,
                            out_var_number,
                        )?;
                    }
                    if !args.space_type.is_empty() {
                        self.add_source_general_meter(
                            &format!(
                                "{subcategory}:{end_use}:{resource}:SpaceType:{}",
                                args.space_type
                            ),
                            units,
                            args.resource,
                            args.end_use,
                            subcategory,
                            SourceMeterGroup::SpaceType,
                            out_var_number,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}
