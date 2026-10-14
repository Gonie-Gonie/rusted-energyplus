//! Regular Construction argument/identity operations, reusing existing layer properties.
use super::super::ctf_initialization_context::{original_loader_name, original_material_name};
use super::{
    HeatBalanceInputError, SelectedHeatBalanceInput, SelectedHeatBalanceInputState,
    SelectedInputProcessor, SourceErrorBackend, SourceInputProcessorShortCut,
};
use ep_model::{MaterialKind, TypedModel};

/// Borrow the existing physical material properties and same Source identity/group owner.
pub(crate) struct ConstructionMaterialOwners<'a> {
    pub(crate) model: &'a TypedModel,
    pub(crate) data: &'a super::super::selected_material_data::SelectedMaterialData,
}

impl SelectedHeatBalanceInputState {
    pub(crate) fn read_regular_construction_arguments(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        materials: ConstructionMaterialOwners<'_>,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        current_module_object: &mut String,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: &mut bool,
    ) -> Result<(), HeatBalanceInputError> {
        let model = materials.model;
        let materials = materials.data;
        // Original1341 returns before ANY count or scratch operation.
        if !self.unique_construction_names.is_empty() {
            return Ok(());
        }
        let regular = input.get_num_objects_found("Construction");
        for family in [
            "Construction:AirBoundary",
            "Construction:FfactorGroundFloor",
            "Construction:CfactorUndergroundWall",
            "Construction:ComplexFenestrationState",
            "Construction:WindowDataFile",
            "Construction:WindowEquivalentLayer",
        ] {
            if input.get_num_objects_found(family) != 0 {
                return Err(HeatBalanceInputError::Unavailable(family));
            }
        }
        if i32::try_from(regular).is_err() {
            return Err(HeatBalanceInputError::Unavailable(
                "original construction count",
            ));
        }
        self.total_constructions = regular;
        self.nominal_r_for_u = Some(vec![0.0; regular]);
        self.nominal_u = Some(vec![0.0; regular]);
        self.nominal_u_before_adjusted = Some(vec![0.0; regular]);
        self.coefficient_adjustment_ratio = Some(vec![1.0; regular]);
        self.construction_usage = vec![super::SelectedConstructionUsage::default(); regular];
        shortcut.current_module_object = "Construction".into();
        for index in 0..regular {
            // Original local ConstructAlphas lower bound is zero. Our [0] is
            // that same Name slot; material layer1 follows it. DummyProps has5.
            let values = raw.get_object_item::<11, 5>("Construction", index, input, shortcut)?;
            if !self
                .unique_construction_names
                .insert(values.alphas[0].clone())
            {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted duplicate construction",
                ));
            }
            // Original1455 follows the uniqueness continue and precedes layers.
            // Keep actual uppercase/untrimmed alpha bytes on later failures.
            self.construction_usage[index]
                .name
                .clone_from(&values.alphas[0]);
            let mut matches = model.constructions.iter().filter(|c| {
                original_loader_name(&model.ctf_report_names.constructions, c.id, &c.name)
                    .is_ok_and(|name| name.to_ascii_uppercase() == values.alphas[0])
            });
            let construction = matches.next().ok_or(HeatBalanceInputError::Unavailable(
                "actual typed construction identity",
            ))?;
            if matches.next().is_some()
                || !construction.is_ordinary_opaque()
                || construction.has_internal_heat_source()
            {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted construction owner",
                ));
            }
            self.construction_order.push(construction.id);
            self.construction_usage[index].construction_id = Some(construction.id);
            let layers =
                values
                    .num_alphas
                    .checked_sub(1)
                    .ok_or(HeatBalanceInputError::Unavailable(
                        "Construction Name argument",
                    ))?;
            // Original1458 precedes layer lookup and its failure prefix.
            self.construction_usage[index].total_layers = i32::try_from(layers)
                .map_err(|_| HeatBalanceInputError::Unavailable("Construction TotLayers int"))?;
            if layers != construction.effective_layers().len() || layers == 0 || layers > 11 {
                return Err(HeatBalanceInputError::Unavailable(
                    "actual construction layer argument count",
                ));
            }
            for (layer, id) in construction.effective_layers().iter().enumerate() {
                if original_material_name(model, *id)
                    .map_err(|_| {
                        HeatBalanceInputError::Unavailable("actual material loader identity")
                    })?
                    .to_ascii_uppercase()
                    != values.alphas[layer + 1]
                {
                    return Err(HeatBalanceInputError::Unavailable(
                        "actual construction material association",
                    ));
                }
                // Same native MaterialMap and actual material-order owner:
                // LayerPoint is assigned before inspecting the layer group.
                let point = materials
                    .material_map
                    .get(&values.alphas[layer + 1])
                    .copied()
                    .unwrap_or(0);
                self.construction_usage[index].layer_points[layer] = point;
                let material_index = point
                    .checked_sub(1)
                    .and_then(|value| usize::try_from(value).ok())
                    .ok_or(HeatBalanceInputError::Unavailable(
                        "Construction native MaterialNum",
                    ))?;
                if materials.materials.get(material_index) != Some(id) {
                    return Err(HeatBalanceInputError::Unavailable(
                        "Construction native/typed MaterialNum association",
                    ));
                }
                let material = model.materials.iter().find(|m| m.id == *id).ok_or(
                    HeatBalanceInputError::Unavailable("actual construction material"),
                )?;
                match material.kind() {
                    MaterialKind::Mass | MaterialKind::NoMass => {}
                    _ => {
                        return Err(HeatBalanceInputError::Unavailable(
                            "unadmitted construction material family",
                        ));
                    }
                }
                let nominal_r = materials.nominal_r.get(material_index).copied().ok_or(
                    HeatBalanceInputError::Unavailable("same MaterialNum NominalR owner"),
                )?;
                let construction_r = self
                    .nominal_r_for_u
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index))
                    .ok_or(HeatBalanceInputError::Unavailable(
                        "actual Construction nominal R allocation",
                    ))?;
                *construction_r += nominal_r; // Actual1504, before NoRegularMaterialsUsed.
                if material.kind() == MaterialKind::Mass {
                    self.no_regular_materials_used = false;
                }
            }
        }
        *current_module_object = "ConstructionProperty:InternalHeatSource".into();
        if input
            .family("ConstructionProperty:InternalHeatSource")
            .is_some()
        {
            self.any_internal_heat_source_in_input = true;
            if input.get_num_objects_found("ConstructionProperty:InternalHeatSource") != 0 {
                return Err(HeatBalanceInputError::Unavailable(
                    "positive internal heat-source construction",
                ));
            }
        }
        if self.total_constructions > 0 && self.no_regular_materials_used {
            backend.show_warning_error(
                "This building has no thermal mass which can cause an unstable solution.",
            )?;
            backend.show_continue_error("Use Material object for all opaque material definitions except very light insulation layers.")?;
        }
        *current_module_object = "Construction:WindowEquivalentLayer".into();
        *current_module_object = "Construction:WindowDataFile".into();
        // Both actual window loops are empty; WConstructNames is gone at1776.
        self.finish_selected_construction_nominal_u(
            ConstructionMaterialOwners {
                model,
                data: materials,
            },
            backend,
            errors_found,
        )
    }

    /// Actual1779-1795 nominal reciprocal/error prefix and ordinary opaque properties.
    pub(crate) fn finish_selected_construction_nominal_u(
        &mut self,
        materials: ConstructionMaterialOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: &mut bool,
    ) -> Result<(), HeatBalanceInputError> {
        for index in 0..self.total_constructions {
            let row = self.construction_usage.get_mut(index).ok_or(
                HeatBalanceInputError::Unavailable("actual nominal Construction association"),
            )?;
            if row.type_is_air_boundary {
                continue;
            }
            let resistance = self
                .nominal_r_for_u
                .as_ref()
                .and_then(|rows| rows.get(index))
                .copied()
                .ok_or(HeatBalanceInputError::Unavailable("actual nominal R slot"))?;
            if resistance != 0.0 {
                *self
                    .nominal_u
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index))
                    .ok_or(HeatBalanceInputError::Unavailable("actual nominal U slot"))? =
                    1.0 / resistance;
            } else if !row.window_type_eql {
                backend.show_severe_error(&format!(
                    "Nominal U is zero, for construction={}",
                    row.name
                ))?;
                *errors_found = true;
            }
            // Same actual MaterialNum/property owners at1795; window optical
            // calculations and positive diagnostic branches stay unavailable.
            row.write_selected_opaque_properties(&materials)?;
        }
        Ok(())
    }
}
