//! Material.cc2432-2564: the actual MaterialComplexWindowGap/MaterialGasMix owner.
use super::window_gas::SourceWindowGasMaterial;
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::{RuntimeError, WarmupOutputError};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use crate::heat_balance::selected_input_processor::{
    SourceInputFamily, SourceInputStatisticsFields, SourceInputStatisticsValue,
};
use ep_model::{MaterialKind, MaterialSurfaceRoughness, classify_source_json_number};

/// One Native derived row; its inherited gas/base fields are not typed physics.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceWindowGapMaterial {
    pub(super) base: SourceWindowGasMaterial,
    pub(super) pressure: f64,
    pub(super) pillar_spacing: f64,
    pub(super) pillar_radius: f64,
    pub(super) deflected_thickness: f64,
}

// Direct getRealFieldValue651-681, restricted to the three fields actually read.
// The original-schema validator rejects numeric Strings; their Native reader
// branch is preserved without widening Source admission.
fn direct_helper_real(
    fields: &SourceInputStatisticsFields,
    family: &str,
    field: &str,
) -> Result<f64, RuntimeError> {
    let default = match (family, field) {
        ("WindowGap:DeflectionState", "deflected_thickness") => 0.0,
        ("WindowGap:SupportPillar", "spacing") => 0.04,
        ("WindowGap:SupportPillar", "radius") => 0.0004,
        _ => return Err(unavailable("closed Gap direct helper field/default")),
    };
    match fields.get(field) {
        Some(SourceInputStatisticsValue::Number(token)) => classify_source_json_number(token)
            .map(ep_model::SourceJsonNumber::as_f64)
            .map_err(|_| unavailable("retained Gap helper JSON number token")),
        Some(SourceInputStatisticsValue::String(value)) if !value.is_empty() => Ok(-99999.0),
        None | Some(SourceInputStatisticsValue::String(_)) => Ok(default),
        _ => Err(unavailable("original-valid Gap helper numeric scalar")),
    }
}

impl SelectedMaterialData {
    pub(super) fn get_window_gap_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        const FAMILY: &str = "WindowMaterial:Gap";
        owners.shortcut.current_module_object = FAMILY.into();
        self.num_w7_gaps = super::count(owners.input, FAMILY)?;
        if usize::try_from(self.num_w7_gaps).ok() != Some(raw.window_gap.len()) {
            return Err(unavailable("WindowMaterial:Gap raw/IP count"));
        }
        for object in &raw.window_gap {
            let (alphas, numbers) = raw.get_window_gap_item(object, owners)?;
            if !(2..=4).contains(&alphas) || !(1..=2).contains(&numbers) {
                return Err(unavailable("Gap4A2N/min0 returned primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual Gap duplicate diagnostic phase owners"))?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!("GetMaterialData: WindowMaterial:Gap = {name}, duplicate name."),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowComplexGap)?;
            self.insert(&name, id, SourceMaterialGroup::ComplexWindowGap)?;
            self.window_gap
                .insert(id, SourceWindowGapMaterial::default());
            self.window_gap
                .get_mut(&id)
                .ok_or_else(|| unavailable("allocated Gap runtime row"))?
                .base
                .roughness = Some(MaterialSurfaceRoughness::Rough);
            self.resistance_only.insert(id, true);

            let thickness = owners.shortcut.numeric_args[0];
            self.window_gap
                .get_mut(&id)
                .ok_or_else(|| unavailable("allocated Gap Thickness owner"))?
                .base
                .thickness = thickness;
            if thickness <= 0.0 {
                *owners.errors_found = true;
                let entered =
                    super::super::source_round_format::round(thickness, 2).map_err(|error| {
                        RuntimeError::WarmupOutput(Box::new(WarmupOutputError::Format(format!(
                            "Gap Thickness .2R: {error:?}"
                        ))))
                    })?;
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual Gap Thickness diagnostic phase owners"))?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!("GetMaterialData: WindowMaterial:Gap = {name}"),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                owners
                    .backend
                    .show_continue_error(&format!(
                        "{} must be > 0, entered {entered}",
                        owners.shortcut.numeric_field_names[0],
                    ))
                    .map_err(|error| {
                        RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Backend(
                            error,
                        )))
                    })?;
            }
            let pressure = owners.shortcut.numeric_args[1];
            self.window_gap
                .get_mut(&id)
                .ok_or_else(|| unavailable("allocated Gap Pressure owner"))?
                .pressure = pressure;
            if pressure <= 0.0 {
                *owners.errors_found = true;
                let entered =
                    super::super::source_round_format::round(pressure, 2).map_err(|error| {
                        RuntimeError::WarmupOutput(Box::new(WarmupOutputError::Format(format!(
                            "Gap Pressure .2R: {error:?}"
                        ))))
                    })?;
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual Gap Pressure diagnostic phase owners"))?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!("GetMaterialData: WindowMaterial:Gap = {name}"),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                owners
                    .backend
                    .show_continue_error(&format!(
                        "{} must be > 0, entered {entered}",
                        owners.shortcut.numeric_field_names[1],
                    ))
                    .map_err(|error| {
                        RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Backend(
                            error,
                        )))
                    })?;
            }
            if !owners.shortcut.alpha_field_blanks[1] {
                let reference = &owners.shortcut.alpha_args[1];
                let Some(number) = self.material_map.get(reference).copied() else {
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual Gap gas-reference diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!("GetMaterialData: WindowMaterial:Gap = {name}"),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    owners
                        .backend
                        .show_continue_error(&format!(
                            "{} = {}, item not found.",
                            owners.shortcut.alpha_field_names[1], reference,
                        ))
                        .map_err(|error| {
                            RuntimeError::HeatBalanceInput(Box::new(
                                HeatBalanceInputError::Backend(error),
                            ))
                        })?;
                    *owners.errors_found = true;
                    continue;
                };
                let source_index = number
                    .checked_sub(1)
                    .and_then(|value| usize::try_from(value).ok())
                    .ok_or_else(|| unavailable("Gap actual gas MaterialNum"))?;
                let source_id = self
                    .materials
                    .get(source_index)
                    .ok_or_else(|| unavailable("Gap actual gas material arena"))?;
                // Snapshot just the four copied members before borrowing the
                // destination. In particular, self refers to its allocated row.
                let source = match self.material_groups.get(source_id) {
                    Some(
                        SourceMaterialGroup::Gas
                        | SourceMaterialGroup::GasMixture
                        | SourceMaterialGroup::WindowGapEql,
                    ) => self
                        .window_gas
                        .get(source_id)
                        .ok_or_else(|| unavailable("Gap earlier GasMix owner"))?,
                    Some(SourceMaterialGroup::ComplexWindowGap) => {
                        &self
                            .window_gap
                            .get(source_id)
                            .ok_or_else(|| unavailable("Gap allocated-self/earlier Gap owner"))?
                            .base
                    }
                    _ => return Err(unavailable("undefined Native Gap non-GasMix reference")),
                };
                let copied = (
                    source.num_gases,
                    source.gas_fractions,
                    source.gases,
                    source.gap_vent_type,
                );
                let destination = &mut self
                    .window_gap
                    .get_mut(&id)
                    .ok_or_else(|| unavailable("Gap gas-copy destination"))?
                    .base;
                destination.num_gases = copied.0;
                destination.gas_fractions = copied.1;
                destination.gases = copied.2;
                destination.gap_vent_type = copied.3;
            }
            if !owners.shortcut.alpha_field_blanks[2] {
                const HELPER: &str = "WindowGap:DeflectionState";
                let reference = &owners.shortcut.alpha_args[2];
                let selected = match owners.input.find_exact_family(HELPER) {
                    None | Some(SourceInputFamily::Null) => None,
                    Some(SourceInputFamily::Objects(_)) => owners
                        .input
                        .source_raw_object_names(HELPER)
                        .map_err(|_| unavailable("Gap DeflectionState original name order"))?
                        .iter()
                        .find(|key| key.to_ascii_uppercase() == *reference)
                        .cloned(),
                };
                let Some(key) = selected else {
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual Gap deflection-reference diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!("GetMaterialData: WindowMaterial:Gap = {name}"),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    owners
                        .backend
                        .show_continue_error(&format!(
                            "{} = {}, item not found.",
                            owners.shortcut.alpha_field_names[2], reference,
                        ))
                        .map_err(|error| {
                            RuntimeError::HeatBalanceInput(Box::new(
                                HeatBalanceInputError::Backend(error),
                            ))
                        })?;
                    *owners.errors_found = true;
                    continue;
                };
                owners.input.mark_object_as_used(HELPER, &key);
                let fields = owners
                    .input
                    .source_raw_object_fields(HELPER, &key)
                    .map_err(|_| unavailable("Gap selected DeflectionState raw fields"))?;
                let value = direct_helper_real(fields, HELPER, "deflected_thickness")?;
                self.window_gap
                    .get_mut(&id)
                    .ok_or_else(|| unavailable("Gap deflection write owner"))?
                    .deflected_thickness = value;
            }
            if !owners.shortcut.alpha_field_blanks[3] {
                const HELPER: &str = "WindowGap:SupportPillar";
                let reference = &owners.shortcut.alpha_args[3];
                let selected = match owners.input.find_exact_family(HELPER) {
                    None | Some(SourceInputFamily::Null) => None,
                    Some(SourceInputFamily::Objects(_)) => owners
                        .input
                        .source_raw_object_names(HELPER)
                        .map_err(|_| unavailable("Gap SupportPillar original name order"))?
                        .iter()
                        .find(|key| key.to_ascii_uppercase() == *reference)
                        .cloned(),
                };
                let Some(key) = selected else {
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual Gap pillar-reference diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!("GetMaterialData: WindowMaterial:Gap = {name}"),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    owners
                        .backend
                        .show_continue_error(&format!(
                            "{} = {}, item not found.",
                            owners.shortcut.alpha_field_names[3], reference,
                        ))
                        .map_err(|error| {
                            RuntimeError::HeatBalanceInput(Box::new(
                                HeatBalanceInputError::Backend(error),
                            ))
                        })?;
                    *owners.errors_found = true;
                    continue;
                };
                owners.input.mark_object_as_used(HELPER, &key);
                let fields = owners
                    .input
                    .source_raw_object_fields(HELPER, &key)
                    .map_err(|_| unavailable("Gap selected SupportPillar raw fields"))?;
                let value = direct_helper_real(fields, HELPER, "spacing")?;
                self.window_gap
                    .get_mut(&id)
                    .ok_or_else(|| unavailable("Gap pillar spacing owner"))?
                    .pillar_spacing = value;
                let value = direct_helper_real(fields, HELPER, "radius")?;
                self.window_gap
                    .get_mut(&id)
                    .ok_or_else(|| unavailable("Gap pillar radius owner"))?
                    .pillar_radius = value;
            }
        }
        Ok(())
    }
}
