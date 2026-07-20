use super::{
    Compiler, MAX_WINDOW_EQUIVALENT_LAYER_CONSTRUCTION_LAYERS,
    WINDOW_EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE, construction_layer_field,
};
use ep_model::{
    Construction, ConstructionId, ConstructionKind, ConstructionWindowEquivalentLayer,
    MaterialFamily, NormalizedName, TypedModel,
};

impl Compiler<'_> {
    pub(super) fn parse_window_equivalent_layer_constructions(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = WINDOW_EQUIVALENT_LAYER_CONSTRUCTION_OBJECT_TYPE;

        for (source_index, (name, object)) in self.objects(OBJECT_TYPE).into_iter().enumerate() {
            let mut fields_valid = true;
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    format!("{OBJECT_TYPE} requires a nonblank object name"),
                );
                fields_valid = false;
            }

            let outside_layer_name =
                self.required_string(OBJECT_TYPE, &name, &object, "outside_layer");
            if outside_layer_name.is_none() {
                fields_valid = false;
            }
            let mut layer_names =
                Vec::with_capacity(MAX_WINDOW_EQUIVALENT_LAYER_CONSTRUCTION_LAYERS);
            layer_names.push(outside_layer_name);
            for layer_number in 2..=MAX_WINDOW_EQUIVALENT_LAYER_CONSTRUCTION_LAYERS {
                let field = format!("layer_{layer_number}");
                match self.optional_reference_name_checked(OBJECT_TYPE, &name, &object, &field) {
                    Some(layer_name) => layer_names.push(layer_name),
                    None => {
                        fields_valid = false;
                        layer_names.push(None);
                    }
                }
            }

            let mut first_missing_layer_index = None;
            let mut layers = Vec::with_capacity(MAX_WINDOW_EQUIVALENT_LAYER_CONSTRUCTION_LAYERS);
            for (layer_index, layer_name) in layer_names.into_iter().enumerate() {
                let field = construction_layer_field(layer_index);
                let Some(layer_name) = layer_name else {
                    first_missing_layer_index.get_or_insert(layer_index);
                    continue;
                };

                if let Some(missing_layer_index) = first_missing_layer_index {
                    let missing_field = construction_layer_field(missing_layer_index);
                    self.error(
                        "NonContiguousEquivalentLayerConstructionLayers",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(&missing_field),
                        format!(
                            "{OBJECT_TYPE}/{name} field {missing_field} is blank, missing, or malformed before populated field {field}; construction layers must be contiguous"
                        ),
                    );
                    fields_valid = false;
                }

                let Some(material_id) = self.resolve_name(
                    &model.material_names,
                    OBJECT_TYPE,
                    &name,
                    &field,
                    &layer_name,
                    "Material",
                ) else {
                    fields_valid = false;
                    continue;
                };
                let Ok(material_index) = usize::try_from(material_id.0) else {
                    self.error(
                        "InvalidReference",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(&field),
                        format!(
                            "{OBJECT_TYPE}/{name} resolved material '{layer_name}' outside the material arena"
                        ),
                    );
                    fields_valid = false;
                    continue;
                };
                let Some(material) = model.materials.get(material_index) else {
                    self.error(
                        "InvalidReference",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(&field),
                        format!(
                            "{OBJECT_TYPE}/{name} resolved material '{layer_name}' outside the material arena"
                        ),
                    );
                    fields_valid = false;
                    continue;
                };
                if material.family() != MaterialFamily::EquivalentLayer {
                    self.error(
                        "InvalidEquivalentLayerConstructionMaterial",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(&field),
                        format!(
                            "{OBJECT_TYPE}/{name} field {field} requires an equivalent-layer material, but '{layer_name}' belongs to the {} family",
                            material.family().id()
                        ),
                    );
                    fields_valid = false;
                }
                layers.push(material_id);
            }
            if !fields_valid || layers.is_empty() {
                continue;
            }

            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, model.constructions.len())
            else {
                continue;
            };
            let Some(source_index) = self.checked_id(OBJECT_TYPE, &name, source_index) else {
                continue;
            };
            let id = ConstructionId(id_value);
            if model.construction_names.insert(&name, id).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }

            model.constructions.push(Construction {
                id,
                name: NormalizedName::new(&name),
                kind: ConstructionKind::WindowEquivalentLayer,
                outside_layer: layers.first().copied(),
                layers,
                thermochromic_master: None,
                ground_factor: None,
                air_boundary: None,
                complex_fenestration: None,
                window_equivalent_layer: Some(ConstructionWindowEquivalentLayer { source_index }),
                internal_heat_source: None,
            });
        }
    }
}
