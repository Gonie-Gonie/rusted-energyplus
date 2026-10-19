//! Selected actual GetShadowingInput, called by GetBuildingData before geometry.
use super::super::ctf_initialization_context::CtfInitializationContext;
use super::super::radiation::ordinary::{OrdinaryRadiationError, OrdinaryRadiationUnavailable};

/// Actual process-derived clipping operands, retained once at the host boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShadowingSystemSettings {
    pub(crate) sutherland_hodgman: bool,
    pub(crate) slater_barsky: bool,
}
impl Default for ShadowingSystemSettings {
    fn default() -> Self {
        Self {
            sutherland_hodgman: true,
            slater_barsky: false,
        }
    }
}

/// The sole selected SolarShading/default system-input owner. This is actual
/// mutable getter state, not a boolean claiming that another getter ran.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceShadowingInput {
    pub(crate) shading_method: SourceShadingMethod,
    pub(crate) calculation_frequency: i32,
    pub(crate) max_hcs: usize,
    pub(crate) detailed_solar_timestep_integration: bool,
    pub(crate) detailed_sky_diffuse_algorithm: bool,
    pub(crate) report_external_sunlit_fraction: bool,
    pub(crate) disable_within_group: bool,
    pub(crate) disable_between_groups: bool,
    pub(crate) disable_all_self_shading: bool,
    pub(crate) disable_group_self_shading: bool,
    pub(crate) shading_groups_num: i32,
    pub(crate) shading_group_zone_list_names: Option<Vec<String>>,
    pub(crate) output_control_shd: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceShadingMethod {
    PolygonClipping,
}
impl SourceShadowingInput {
    pub(crate) fn new() -> Self {
        Self {
            shading_method: SourceShadingMethod::PolygonClipping,
            calculation_frequency: 0,
            max_hcs: 15000,
            detailed_solar_timestep_integration: false,
            detailed_sky_diffuse_algorithm: false,
            report_external_sunlit_fraction: false,
            disable_within_group: false,
            disable_between_groups: false,
            disable_all_self_shading: false,
            disable_group_self_shading: false,
            shading_groups_num: 0,
            shading_group_zone_list_names: None,
            output_control_shd: true,
        }
    }

    pub(crate) fn get_at_building_boundary(
        &mut self,
        context: &mut CtfInitializationContext<'_>,
    ) -> Result<(), OrdinaryRadiationError> {
        let shortcut = &mut context.reports.input_processor_shortcut;
        // These are the actual written prefixes of shared rNumericArgs and
        // cAlphaArgs. Other shortcut members/allocation are outside this slice.
        if shortcut.numeric_args.len() < 4 || shortcut.alpha_args.len() < 7 {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        }
        shortcut.numeric_args[..4].fill(0.0);
        shortcut.alpha_args[0].clear();
        shortcut.alpha_args[1].clear();
        shortcut.current_module_object = "ShadowCalculation".into();
        let num_items = context
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or(OrdinaryRadiationUnavailable::MissingInitializationContext)?
            .get_num_objects_found("ShadowCalculation");
        if num_items != 0 {
            return self.get_explicit_at_building_boundary(context, num_items);
        }
        let settings = &context
            .reports
            .selected_project_data
            .as_ref()
            .ok_or(OrdinaryRadiationUnavailable::MissingInitializationContext)?
            .system
            .shadowing_clipping;
        // Actual absence: NumItems=NumAlphas=NumNumbers=0. No getObjectItem.
        if self.calculation_frequency <= 0 {
            self.calculation_frequency = 20;
        }
        self.max_hcs = 15000;
        let alpha = &mut shortcut.alpha_args;
        alpha[0] = "PolygonClipping".into();
        self.shading_method = SourceShadingMethod::PolygonClipping;
        self.detailed_solar_timestep_integration = false;
        alpha[1] = "Periodic".into();
        alpha[2] = if !settings.sutherland_hodgman {
            "ConvexWeilerAtherton"
        } else if !settings.slater_barsky {
            "SutherlandHodgman"
        } else {
            "SlaterBarskyandSutherlandHodgman"
        }
        .into();
        alpha[3] = "SimpleSkyDiffuseModeling".into();
        self.detailed_sky_diffuse_algorithm = false;
        alpha[4] = "No".into();
        self.report_external_sunlit_fraction = false;
        alpha[5] = "No".into();
        alpha[6] = "No".into();
        self.disable_all_self_shading = self.disable_between_groups && self.disable_within_group;
        self.disable_group_self_shading = !self.disable_all_self_shading
            && (self.disable_between_groups || self.disable_within_group);
        // Literal NumAlphas-(aNum-1) with NumAlphas0 and aNum8, not inferred0.
        self.shading_groups_num = -7;
        self.shading_group_zone_list_names = Some(Vec::new());
        context.eio.write_all(b"! <Shadowing/Sun Position Calculations Annual Simulations>, Shading Calculation Method, Shading Calculation Update Frequency Method, Shading Calculation Update Frequency {days}, Maximum Figures in Shadow Overlap Calculations {}, Polygon Clipping Algorithm, Pixel Counting Resolution, Sky Diffuse Modeling Algorithm, Output External Shading Calculation Results, Disable Self-Shading Within Shading Zone Groups, Disable Self-Shading From Shading Zone Groups to Other Zones\n")?;
        writeln!(
            context.eio,
            "Shadowing/Sun Position Calculations Annual Simulations,{},{},{},{},{},512,{},{},{},{}",
            alpha[0],
            alpha[1],
            self.calculation_frequency,
            self.max_hcs,
            alpha[2],
            alpha[3],
            alpha[4],
            alpha[5],
            alpha[6]
        )?;
        Ok(())
    }
    /// Original SolarShading.cc446-743, selected real PolygonClipping input.
    fn get_explicit_at_building_boundary(
        &mut self,
        context: &mut CtfInitializationContext<'_>,
        num_items: usize,
    ) -> Result<(), OrdinaryRadiationError> {
        use super::super::selected_input_processor::SourceInputFamily;
        let reports = &mut context.reports;
        if num_items > 1 {
            context.backend.show_warning_error(
                "ShadowCalculation: More than 1 occurrence of this object found, only first will be used.",
            ).map_err(super::HeatBalanceInputError::from).map_err(input_error)?;
        }
        let input = reports
            .selected_input_processor
            .as_mut()
            .ok_or(OrdinaryRadiationUnavailable::MissingInitializationContext)?;
        // The scalar raw carrier has no extensible alpha operands. Stop at the
        // actual getObjectItem request rather than silently dropping a group.
        if let Some(SourceInputFamily::Objects(objects)) = input.family("ShadowCalculation") {
            if objects.first().is_some_and(|object| {
                object
                    .field_sizes
                    .get("shading_zone_groups")
                    .copied()
                    .unwrap_or(0)
                    != 0
            }) {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
        }
        let raw = reports
            .heat_balance_input
            .as_ref()
            .ok_or(OrdinaryRadiationUnavailable::MissingInitializationContext)?;
        let shortcut = &mut reports.input_processor_shortcut;
        let counts = raw
            .get_shared_source_object_item("ShadowCalculation", 0, input, shortcut)
            .map_err(input_error)?;
        self.calculation_frequency = source_int(shortcut.numeric_args[0])?;
        if self.calculation_frequency <= 0 {
            self.calculation_frequency = 20;
        }
        if self.calculation_frequency > 31 {
            context
                .backend
                .show_warning_error(&format!(
                    "ShadowCalculation: suspect {}",
                    shortcut.numeric_field_names[0],
                ))
                .map_err(super::HeatBalanceInputError::from)
                .map_err(input_error)?;
            context
                .backend
                .show_continue_error(&format!(
                    "Value entered=[{}], Shadowing Calculations will be inaccurate.",
                    super::rounded(shortcut.numeric_args[0], 0).map_err(input_error)?,
                ))
                .map_err(super::HeatBalanceInputError::from)
                .map_err(input_error)?;
        }
        self.max_hcs = if shortcut.numeric_args[1] > 199.0 {
            usize::try_from(source_int(shortcut.numeric_args[1])?)
                .map_err(|_| OrdinaryRadiationUnavailable::NumericDomain)?
        } else {
            15000
        };
        let alpha = &mut shortcut.alpha_args;
        if counts.num_alphas >= 1 {
            if alpha[0].eq_ignore_ascii_case("PolygonClipping") {
                self.shading_method = SourceShadingMethod::PolygonClipping;
                alpha[0] = "PolygonClipping".into();
            } else {
                // Scheduled first calls its real surface child; Imported and
                // PixelCounting require unrepresented schedule/GPU owners.
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
        } else {
            alpha[0] = "PolygonClipping".into();
            self.shading_method = SourceShadingMethod::PolygonClipping;
        }
        if counts.num_alphas >= 2 && alpha[1].eq_ignore_ascii_case("Timestep") {
            self.detailed_solar_timestep_integration = true;
            alpha[1] = "Timestep".into();
        } else if counts.num_alphas < 2 || alpha[1].eq_ignore_ascii_case("Periodic") {
            self.detailed_solar_timestep_integration = false;
            alpha[1] = "Periodic".into();
        } else {
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
        let settings = &mut reports
            .selected_project_data
            .as_mut()
            .ok_or(OrdinaryRadiationUnavailable::MissingInitializationContext)?
            .system
            .shadowing_clipping;
        if counts.num_alphas >= 3 {
            if alpha[2].eq_ignore_ascii_case("SutherlandHodgman") {
                settings.sutherland_hodgman = true;
                alpha[2] = "SutherlandHodgman".into();
            } else if alpha[2].eq_ignore_ascii_case("ConvexWeilerAtherton") {
                settings.sutherland_hodgman = false;
                alpha[2] = "ConvexWeilerAtherton".into();
            } else if alpha[2].eq_ignore_ascii_case("SlaterBarskyandSutherlandHodgman") {
                settings.sutherland_hodgman = true;
                settings.slater_barsky = true;
                alpha[2] = "SlaterBarskyandSutherlandHodgman".into();
            } else if shortcut.alpha_field_blanks[2] {
                alpha[2] = clipping_name(settings).into();
            } else {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
        } else {
            alpha[2] = clipping_name(settings).into();
        }
        if counts.num_alphas >= 4 {
            if alpha[3].eq_ignore_ascii_case("SimpleSkyDiffuseModeling") {
                self.detailed_sky_diffuse_algorithm = false;
                alpha[3] = "SimpleSkyDiffuseModeling".into();
            } else if alpha[3].eq_ignore_ascii_case("DetailedSkyDiffuseModeling") {
                self.detailed_sky_diffuse_algorithm = true;
                alpha[3] = "DetailedSkyDiffuseModeling".into();
            } else if shortcut.alpha_field_blanks[2] {
                // Original tests AlphaFieldBlanks(3), not the current alpha4.
                self.detailed_sky_diffuse_algorithm = false;
                alpha[3] = "SimpleSkyDiffuseModeling".into();
            } else {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
        } else {
            alpha[3] = "SimpleSkyDiffuseModeling".into();
            self.detailed_sky_diffuse_algorithm = false;
        }
        if counts.num_alphas >= 5 {
            if alpha[4].eq_ignore_ascii_case("Yes") {
                self.report_external_sunlit_fraction = true;
                alpha[4] = "Yes".into();
            } else if alpha[4].eq_ignore_ascii_case("No") {
                self.report_external_sunlit_fraction = false;
                alpha[4] = "No".into();
            } else {
                return Err(OrdinaryRadiationUnavailable::InputBranch.into());
            }
        } else {
            alpha[4] = "No".into();
            self.report_external_sunlit_fraction = false;
        }
        for (slot, flag) in [
            (5, &mut self.disable_within_group),
            (6, &mut self.disable_between_groups),
        ] {
            if counts.num_alphas > slot {
                if alpha[slot].eq_ignore_ascii_case("Yes") {
                    *flag = true;
                    alpha[slot] = "Yes".into();
                } else if alpha[slot].eq_ignore_ascii_case("No") {
                    // Original No canonicalizes the string but retains the flag.
                    alpha[slot] = "No".into();
                } else {
                    return Err(OrdinaryRadiationUnavailable::InputBranch.into());
                }
            } else {
                alpha[slot] = "No".into();
            }
        }
        if self.disable_between_groups && self.disable_within_group {
            self.disable_all_self_shading = true;
        } else if self.disable_between_groups || self.disable_within_group {
            self.disable_group_self_shading = true;
        }
        self.shading_groups_num = i32::try_from(counts.num_alphas)
            .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?
            - 7;
        // This selected scalar getObjectItem has no positive extension groups.
        self.shading_group_zone_list_names = Some(Vec::new());
        let transmittance_varies = reports.surface_geometry_input.shading_transmittance_varies;
        let minimal = reports.heat_balance_input_state.solar_distribution
            == super::SelectedSolarDistribution::Minimal;
        if !self.detailed_solar_timestep_integration && transmittance_varies && !minimal {
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
        if !self.detailed_sky_diffuse_algorithm && transmittance_varies && !minimal {
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        } else if self.detailed_sky_diffuse_algorithm && (!transmittance_varies || minimal) {
            context.backend.show_warning_error(
                "GetShadowingInput: DetailedSkyDiffuseModeling is chosen but not needed as either the shading transmittance for shading devices does not change throughout the year",
            ).map_err(super::HeatBalanceInputError::from).map_err(input_error)?;
            context
                .backend
                .show_continue_error(" or MinimalShadowing has been chosen.")
                .map_err(super::HeatBalanceInputError::from)
                .map_err(input_error)?;
            context.backend.show_continue_error(
                "Simulation should be set to use SimpleSkyDiffuseModeling, but is left at Detailed for simulation.",
            ).map_err(super::HeatBalanceInputError::from).map_err(input_error)?;
            context.backend.show_continue_error(
                "Choose SimpleSkyDiffuseModeling in the ShadowCalculation object to reduce computation time.",
            ).map_err(super::HeatBalanceInputError::from).map_err(input_error)?;
        }
        context.eio.write_all(b"! <Shadowing/Sun Position Calculations Annual Simulations>, Shading Calculation Method, Shading Calculation Update Frequency Method, Shading Calculation Update Frequency {days}, Maximum Figures in Shadow Overlap Calculations {}, Polygon Clipping Algorithm, Pixel Counting Resolution, Sky Diffuse Modeling Algorithm, Output External Shading Calculation Results, Disable Self-Shading Within Shading Zone Groups, Disable Self-Shading From Shading Zone Groups to Other Zones\n")?;
        writeln!(
            context.eio,
            "Shadowing/Sun Position Calculations Annual Simulations,{},{},{},{},{},512,{},{},{},{}",
            alpha[0],
            alpha[1],
            self.calculation_frequency,
            self.max_hcs,
            alpha[2],
            alpha[3],
            alpha[4],
            alpha[5],
            alpha[6],
        )?;
        Ok(())
    }
}

fn input_error(error: super::HeatBalanceInputError) -> OrdinaryRadiationError {
    OrdinaryRadiationError::Input(Box::new(error))
}
fn source_int(value: f64) -> Result<i32, OrdinaryRadiationError> {
    if !value.is_finite() || value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
        return Err(OrdinaryRadiationUnavailable::NumericDomain.into());
    }
    Ok(value as i32)
}
fn clipping_name(settings: &ShadowingSystemSettings) -> &'static str {
    if !settings.sutherland_hodgman {
        "ConvexWeilerAtherton"
    } else if !settings.slater_barsky {
        "SutherlandHodgman"
    } else {
        "SlaterBarskyandSutherlandHodgman"
    }
}
