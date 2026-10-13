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
        let settings = &context
            .reports
            .selected_project_data
            .as_ref()
            .ok_or(OrdinaryRadiationUnavailable::MissingInitializationContext)?
            .system
            .shadowing_clipping;
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
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
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
}
