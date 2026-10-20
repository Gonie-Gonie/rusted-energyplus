//! Actual selected GetMaterialData, with borrowed sole physical properties.
use super::ctf_initialization_context::CtfInitializationContext;
use super::error_backend::SourceErrorBackend;
use super::report_scan::{
    ReportScanFailure, ReportScanState, SourceReportScanInput, try_scan_for_reports_with_input,
};
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
use super::selected_project_data::ProjectInputObject;
use crate::error::{RuntimeError, WarmupOutputError};
use ep_model::{
    MaterialDefinition, MaterialId, MaterialKind, MaterialSurfaceRoughness, TypedModel,
};
use std::collections::BTreeMap;
use std::io::Write;

mod input;
pub use input::SelectedMaterialInput;
#[cfg(test)]
mod air_gap_tests;
mod report;
mod type_group;
pub(crate) use type_group::SourceMaterialGroup;

pub(super) fn unavailable(message: &str) -> RuntimeError {
    RuntimeError::WarmupOutput(Box::new(WarmupOutputError::Unavailable(format!(
        "selected GetMaterialData: {message}"
    ))))
}

pub(super) struct MaterialDataOwners<'a, 'b> {
    model: &'a TypedModel,
    input: &'a mut SelectedInputProcessor,
    shortcut: &'a mut SourceInputProcessorShortCut,
    scan: &'a mut ReportScanState,
    globals: &'a SelectedHeatBalanceGlobals,
    backend: &'a mut SourceErrorBackend<'b>,
    eio: &'a mut dyn Write,
    errors_found: &'a mut bool,
}

// The selected FrameAndDivider domain has no elements. Keeping an empty
// element type prevents manufacturing unported positive property rows.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SelectedFrameDividerProperties {}

/// Reached MaterialData count/name/position owners; no second physical cache.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedMaterialData {
    pub(crate) materials: Vec<MaterialId>,
    pub(crate) material_map: BTreeMap<String, i32>,
    /// Same Material owner: group written by the actual input-family getter.
    pub(crate) material_groups: BTreeMap<MaterialId, SourceMaterialGroup>,
    /// MaterialBase ROnly ctorfalse and actual NoMass/AirGap getter writes.
    pub(crate) resistance_only: BTreeMap<MaterialId, bool>,
    /// MaterialBase NominalR ctor0 and getter writes, aligned with actual MaterialNum.
    pub(crate) nominal_r: Vec<f64>,
    pub(crate) num_no_masses: i32,
    pub(crate) num_irts: i32,
    pub(crate) num_air_gaps: i32,
    pub(crate) num_spectral_data: i32,
    pub(crate) num_w5_glazings: i32,
    pub(crate) num_w5_alt_glazings: i32,
    pub(crate) num_eql_glazings: i32,
    pub(crate) num_w5_gases: i32,
    pub(crate) num_eql_gaps: i32,
    pub(crate) num_w5_gas_mixtures: i32,
    pub(crate) num_shades: i32,
    pub(crate) num_eql_shades: i32,
    pub(crate) num_eql_drapes: i32,
    pub(crate) num_screens: i32,
    pub(crate) num_eql_screens: i32,
    pub(crate) num_blinds: i32,
    pub(crate) num_eql_blinds: i32,
    pub(crate) num_eco_roofs: i32,
    pub(crate) num_tc_glazings: i32,
    pub(crate) num_simple_windows: i32,
    pub(crate) num_w7_gaps: i32,
    pub(crate) any_variable_absorptance: bool,
    pub(crate) total_frame_divider: i32,
    /// Actual dataSurface FrameDivider allocation. None is the constructor;
    /// Some(empty box) is allocate(0): allocated, size0, native bounds1:0.
    /// This representation makes no Native address or allocator ABI claim.
    pub(crate) frame_divider: Option<Box<[SelectedFrameDividerProperties]>>,
}

fn count(input: &SelectedInputProcessor, family: &str) -> Result<i32, RuntimeError> {
    i32::try_from(input.get_num_objects_found(family))
        .map_err(|_| unavailable("Source count cannot fit int"))
}
fn require_zero(value: i32, family: &str) -> Result<(), RuntimeError> {
    if value != 0 {
        return Err(unavailable(family));
    }
    Ok(())
}
fn same(a: f64, b: f64) -> Result<(), RuntimeError> {
    if a.to_bits() != b.to_bits() {
        return Err(unavailable("raw getter/sole typed property association"));
    }
    Ok(())
}
fn roughness(object: &ProjectInputObject) -> Result<MaterialSurfaceRoughness, RuntimeError> {
    match input::alpha(object, "roughness")?
        .to_ascii_uppercase()
        .as_str()
    {
        "VERYROUGH" => Ok(MaterialSurfaceRoughness::VeryRough),
        "ROUGH" => Ok(MaterialSurfaceRoughness::Rough),
        "MEDIUMROUGH" => Ok(MaterialSurfaceRoughness::MediumRough),
        "MEDIUMSMOOTH" => Ok(MaterialSurfaceRoughness::MediumSmooth),
        "SMOOTH" => Ok(MaterialSurfaceRoughness::Smooth),
        "VERYSMOOTH" => Ok(MaterialSurfaceRoughness::VerySmooth),
        _ => Err(unavailable("literal non-trimmed Material roughness enum")),
    }
}
fn typed_id(
    model: &TypedModel,
    name: &str,
    kind: MaterialKind,
) -> Result<MaterialId, RuntimeError> {
    let mut matches = model
        .ctf_report_names
        .materials
        .iter()
        .filter(|(_, loader)| *loader == name);
    let (id, _) = matches
        .next()
        .ok_or_else(|| unavailable("actual material loader name"))?;
    if matches.next().is_some() {
        return Err(unavailable("duplicate typed loader name"));
    }
    let material = model
        .materials
        .iter()
        .find(|value| value.id == *id)
        .ok_or_else(|| unavailable("material typed ID"))?;
    if material.kind() != kind {
        return Err(unavailable("material family/typed ID"));
    }
    Ok(*id)
}

impl SelectedMaterialData {
    fn insert(
        &mut self,
        name: &str,
        id: MaterialId,
        group: SourceMaterialGroup,
    ) -> Result<usize, RuntimeError> {
        let key = name.to_ascii_uppercase();
        if self.material_map.contains_key(&key) {
            return Err(unavailable(
                "duplicate-name diagnostic outside admitted compiled domain",
            ));
        }
        // Material.cc199/275 writes group before push_back. The typed ID
        // only binds the existing property owner; it does not infer group.
        self.material_groups.insert(id, group);
        self.materials.push(id);
        self.nominal_r.push(0.0); // Material.hh239 constructor, before property reads.
        self.resistance_only.insert(id, false); // MaterialBase ctor, before getter assignments.
        let number = i32::try_from(self.materials.len())
            .map_err(|_| unavailable("Material.Num int capacity"))?;
        self.material_map.insert(key, number);
        Ok(self.materials.len() - 1)
    }

    fn get_material_data(
        &mut self,
        raw: &SelectedMaterialInput,
        o: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        self.num_no_masses = count(o.input, "Material:NoMass")?;
        self.num_irts = count(o.input, "Material:InfraredTransparent")?;
        self.num_air_gaps = count(o.input, "Material:AirGap")?;
        let f_factor = count(o.input, "Construction:FfactorGroundFloor")?;
        let c_factor = count(o.input, "Construction:CfactorUndergroundWall")?;
        o.shortcut.current_module_object = "Material".into();
        for object in &raw.regular {
            if !o.input.has_object("Material", &object.object_name) {
                return Err(unavailable("regular raw/IP identity"));
            }
            // Direct JSON markObjectAsUsed; none of the six arrays is an argument.
            o.input.mark_object_as_used("Material", &object.object_name);
            let id = typed_id(o.model, &object.object_name, MaterialKind::Mass)?;
            let material_index =
                self.insert(&object.object_name, id, SourceMaterialGroup::Regular)?;
            let material = o
                .model
                .materials
                .iter()
                .find(|value| value.id == id)
                .ok_or_else(|| unavailable("regular ID"))?;
            let MaterialDefinition::Regular(value) = material.definition else {
                return Err(unavailable("regular definition"));
            };
            if roughness(object)? != value.roughness {
                return Err(unavailable("regular roughness/typed association"));
            }
            let thickness = input::numeric(object, "thickness", None)?;
            same(thickness, value.thickness_m)?;
            let conductivity = input::numeric(object, "conductivity", None)?;
            same(conductivity, value.conductivity_w_per_m_k)?;
            for (field, typed) in [
                ("density", value.density_kg_per_m3),
                ("specific_heat", value.specific_heat_j_per_kg_k),
            ] {
                same(input::numeric(object, field, None)?, typed)?;
            }
            for (field, default, typed) in [
                (
                    "thermal_absorptance",
                    0.9,
                    value.surface.thermal_absorptance,
                ),
                ("solar_absorptance", 0.7, value.surface.solar_absorptance),
                (
                    "visible_absorptance",
                    0.7,
                    value.surface.visible_absorptance,
                ),
            ] {
                same(input::numeric(object, field, Some(default))?, typed)?;
            }
            if conductivity > 0.0 {
                self.nominal_r[material_index] = thickness / conductivity;
            } else {
                o.backend
                    .show_severe_error(&format!(
                        "Positive thermal conductivity required for material {}",
                        object.object_name
                    ))
                    .map_err(|error| {
                        RuntimeError::WarmupOutput(Box::new(WarmupOutputError::Backend(error)))
                    })?;
                *o.errors_found = true;
            }
        }
        // Actual generated FC material branch is not admitted; do not synthesize it.
        require_zero(f_factor, "positive F-factor generated material")?;
        require_zero(c_factor, "positive C-factor generated material")?;
        o.shortcut.current_module_object = "Material:NoMass".into();
        if usize::try_from(self.num_no_masses).ok() != Some(raw.no_mass.len()) {
            return Err(unavailable("NoMass raw/IP count"));
        }
        for object in &raw.no_mass {
            let (_num_alphas, num_numbers) = raw.get_no_mass_item(object, o)?;
            let name = &o.shortcut.alpha_args[0];
            let id = typed_id(o.model, name, MaterialKind::NoMass)?;
            let material_index = self.insert(name, id, SourceMaterialGroup::Regular)?;
            let material = o
                .model
                .materials
                .iter()
                .find(|value| value.id == id)
                .ok_or_else(|| unavailable("NoMass ID"))?;
            let MaterialDefinition::NoMass(value) = material.definition else {
                return Err(unavailable("NoMass definition"));
            };
            if roughness(object)? != value.roughness {
                return Err(unavailable("NoMass roughness/typed association"));
            }
            same(
                o.shortcut.numeric_args[0],
                value.thermal_resistance_m2_k_per_w,
            )?;
            self.resistance_only.insert(id, true); // Actual285, before optional absorptance reads.
            for (index, default, typed) in [
                (1, 0.9, value.surface.thermal_absorptance),
                (2, 0.7, value.surface.solar_absorptance),
                (3, 0.7, value.surface.visible_absorptance),
            ] {
                same(
                    if num_numbers > index {
                        o.shortcut.numeric_args[index]
                    } else {
                        default
                    },
                    typed,
                )?;
            }
            self.nominal_r[material_index] = o.shortcut.numeric_args[0]; // Actual308.
        }
        o.shortcut.current_module_object = "Material:AirGap".into();
        if usize::try_from(self.num_air_gaps).ok() != Some(raw.air_gaps.len()) {
            return Err(unavailable("AirGap raw/IP count"));
        }
        for object in &raw.air_gaps {
            let (_num_alphas, num_numbers) = raw.get_air_gap_item(object, o)?;
            if num_numbers != 1 {
                return Err(unavailable("required AirGap numeric argument count"));
            }
            let name = &o.shortcut.alpha_args[0];
            let id = typed_id(o.model, name, MaterialKind::AirGap)?;
            let material_index = self.insert(name, id, SourceMaterialGroup::AirGap)?;
            let material = o
                .model
                .materials
                .iter()
                .find(|value| value.id == id)
                .ok_or_else(|| unavailable("AirGap ID"))?;
            let MaterialDefinition::AirGap(value) = material.definition else {
                return Err(unavailable("AirGap definition"));
            };
            // Material.cc366 writes MediumRough; the sole typed AirGap owner
            // supplies that fixed roughness and genuine ctorzero absorptances.
            same(
                o.shortcut.numeric_args[0],
                value.thermal_resistance_m2_k_per_w,
            )?;
            self.nominal_r[material_index] = o.shortcut.numeric_args[0]; // Actual367.
            self.resistance_only.insert(id, true); // Actual368; never a NoMass alias.
        }
        o.shortcut.current_module_object = "Material:InfraredTransparent".into();
        require_zero(self.num_irts, "positive Material:InfraredTransparent")?;
        macro_rules! empty {
            ($member:ident, $family:literal) => {{
                o.shortcut.current_module_object = $family.into();
                self.$member = count(o.input, $family)?;
                require_zero(self.$member, concat!("positive ", $family))?;
            }};
        }
        empty!(num_w5_glazings, "WindowMaterial:Glazing");
        empty!(
            num_w5_alt_glazings,
            "WindowMaterial:Glazing:RefractionExtinctionMethod"
        );
        empty!(num_eql_glazings, "WindowMaterial:Glazing:EquivalentLayer");
        empty!(num_w5_gases, "WindowMaterial:Gas");
        empty!(num_eql_gaps, "WindowMaterial:Gap:EquivalentLayer");
        empty!(num_w5_gas_mixtures, "WindowMaterial:GasMixture");
        empty!(num_shades, "WindowMaterial:Shade");
        empty!(num_eql_shades, "WindowMaterial:Shade:EquivalentLayer");
        empty!(num_eql_drapes, "WindowMaterial:Drape:EquivalentLayer");
        empty!(num_screens, "WindowMaterial:Screen");
        empty!(num_eql_screens, "WindowMaterial:Screen:EquivalentLayer");
        empty!(num_blinds, "WindowMaterial:Blind");
        empty!(num_eql_blinds, "WindowMaterial:Blind:EquivalentLayer");
        empty!(num_eco_roofs, "Material:RoofVegetation");
        empty!(num_tc_glazings, "WindowMaterial:GlazingGroup:Thermochromic");
        empty!(num_simple_windows, "WindowMaterial:SimpleGlazingSystem");
        empty!(num_w7_gaps, "WindowMaterial:Gap");
        o.shortcut.current_module_object = "WindowMaterial:ComplexShade".into();
        require_zero(
            count(o.input, "WindowMaterial:ComplexShade")?,
            "positive WindowMaterial:ComplexShade",
        )?;
        let mut source = SourceReportScanInput::new(o.input, o.shortcut, &o.model.report_inputs)
            .map_err(|_| unavailable("actual Materials Scan input"))?;
        let report = try_scan_for_reports_with_input(
            &o.model.report_inputs,
            o.scan,
            "Constructions",
            Some("Materials"),
            None,
            None,
            &mut source,
            &mut |diagnostic| match diagnostic {
                super::report_scan::ReportScanDiagnostic::Warning(message) => {
                    o.backend.show_warning_error(&message)
                }
                super::report_scan::ReportScanDiagnostic::Continue(message) => {
                    o.backend.show_continue_error(&message)
                }
            },
        )
        .map_err(|error| match error {
            ReportScanFailure::Unavailable(_) => unavailable("Materials Scan request"),
            ReportScanFailure::Diagnostic(error) => {
                RuntimeError::WarmupOutput(Box::new(WarmupOutputError::Backend(error)))
            }
        })?;
        if report {
            self.write_material_reports(o.model, o.eio)?;
        }
        if o.globals.any_energy_management_system_in_model {
            return Err(unavailable("actual positive EMS material actuators"));
        }
        o.shortcut.current_module_object = "MaterialProperty:VariableAbsorptance".into();
        let variable = count(o.input, "MaterialProperty:VariableAbsorptance")?;
        self.any_variable_absorptance = variable > 0;
        require_zero(variable, "positive MaterialProperty:VariableAbsorptance")
    }
}

impl CtfInitializationContext<'_> {
    pub(crate) fn get_selected_material_data(
        &mut self,
        model: &TypedModel,
    ) -> Result<(), RuntimeError> {
        let reports = &mut *self.reports;
        let input = reports
            .selected_input_processor
            .as_mut()
            .ok_or_else(|| unavailable("missing sole IP owner"))?;
        let raw = reports
            .selected_material_input
            .as_ref()
            .ok_or_else(|| unavailable("missing retained raw materials"))?;
        let globals = reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or_else(|| unavailable("missing actual early globals"))?;
        let mut owners = MaterialDataOwners {
            model,
            input,
            shortcut: &mut reports.input_processor_shortcut,
            scan: &mut reports.scan,
            globals,
            backend: &mut self.backend,
            eio: self.eio,
            errors_found: &mut reports.heat_balance_input_errors_found,
        };
        // HBM266: real zero-selected spectral getter, before GetMaterialData.
        owners.shortcut.current_module_object = "MaterialProperty:GlazingSpectralData".into();
        reports.selected_material_data.num_spectral_data =
            count(owners.input, "MaterialProperty:GlazingSpectralData")?;
        require_zero(
            reports.selected_material_data.num_spectral_data,
            "positive spectral material data",
        )?;
        reports
            .selected_material_data
            .get_material_data(raw, &mut owners)?;
        // HBM270: actual zero-selected hysteresis getter.
        owners.shortcut.current_module_object = "MaterialProperty:PhaseChangeHysteresis".into();
        require_zero(
            count(owners.input, "MaterialProperty:PhaseChangeHysteresis")?,
            "positive material hysteresis",
        )?;
        // HBM272: distinct HBM String and actual zero-size FrameDivider.allocate.
        reports.heat_balance_current_module_object = "WindowProperty:FrameAndDivider".into();
        reports.selected_material_data.total_frame_divider =
            count(owners.input, "WindowProperty:FrameAndDivider")?;
        if reports.selected_material_data.total_frame_divider != 0 {
            // Positive element construction is outside the admitted domain;
            // retain the reached count/String without claiming that allocation.
            return Err(unavailable("positive FrameAndDivider"));
        }
        // HBM3474 executes even at count0. Objexx allocate_zero allocates a
        // non-null zero-sized owner; IndexRange(0) has lower1/upper0. Allocate
        // the sole selected storage before the actual zero-count return.
        reports.selected_material_data.frame_divider = Some(Vec::new().into_boxed_slice());
        Ok(())
    }
}
