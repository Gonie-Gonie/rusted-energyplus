use super::{
    COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE, Compiler, RawObject, RawValue, field_value,
};
use ep_model::{
    ComplexFenestrationBasisSymmetry, ComplexFenestrationBasisType, ComplexFenestrationMatrix,
    ComplexFenestrationOpticalLayer, Construction, ConstructionComplexFenestrationState,
    ConstructionId, ConstructionKind, MaterialDefinition, MaterialId, NormalizedName, TypedModel,
    WindowThermalCalculationModel, WindowThermalCalculationStandard, WindowThermalDeflectionModel,
    WindowThermalModelParameters,
};
use ep_raw_model::FieldName;
use std::collections::BTreeMap;
use std::sync::Arc;

mod parsing;

use parsing::{
    ComplexLayerExpectation, ParsedDeflectionModel, format_complex_number, parse_basis_symmetry,
    parse_basis_type, parse_deflection_model, parse_thermal_calculation_model,
    parse_thermal_calculation_standard,
};

const WINDOW_THERMAL_MODEL_OBJECT_TYPE: &str = "WindowThermalModel:Params";
const MATRIX_TWO_DIMENSION_OBJECT_TYPE: &str = "Matrix:TwoDimension";

const OPTICAL_LAYER_FIELDS: [(&str, &str, &str); 5] = [
    (
        "outside_layer_name",
        "outside_layer_directional_front_absorptance_matrix_name",
        "outside_layer_directional_back_absorptance_matrix_name",
    ),
    (
        "layer_2_name",
        "layer_2_directional_front_absorptance_matrix_name",
        "layer_2_directional_back_absorptance_matrix_name",
    ),
    (
        "layer_3_name",
        "layer_3_directional_front_absorptance_matrix_name",
        "layer_3_directional_back_absorptance_matrix_name",
    ),
    (
        "layer_4_name",
        "layer_4_directional_front_absorptance_matrix_name",
        "layer_4_directional_back_absorptance_matrix_name",
    ),
    (
        "layer_5_name",
        "layer_5_directional_front_absorptance_matrix_name",
        "layer_5_directional_back_absorptance_matrix_name",
    ),
];

const GAP_LAYER_FIELDS: [(&str, &str, &str); 4] = [
    (
        "gap_1_name",
        "cfs_gap_1_directional_front_absorptance_matrix_name",
        "cfs_gap_1_directional_back_absorptance_matrix_name",
    ),
    (
        "gap_2_name",
        "gap_2_directional_front_absorptance_matrix_name",
        "gap_2_directional_back_absorptance_matrix_name",
    ),
    (
        "gap_3_name",
        "gap_3_directional_front_absorptance_matrix_name",
        "gap_3_directional_back_absorptance_matrix_name",
    ),
    (
        "gap_4_name",
        "gap_4_directional_front_absorptance_matrix_name",
        "gap_4_directional_back_absorptance_matrix_name",
    ),
];

type ThermalModelCatalog = BTreeMap<NormalizedName, WindowThermalModelParameters>;
type MatrixCatalog = BTreeMap<NormalizedName, ComplexFenestrationMatrix>;

impl Compiler<'_> {
    pub(super) fn parse_complex_fenestration_states(&mut self, model: &mut TypedModel) {
        let definitions = self.objects(COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE);
        if definitions.is_empty() {
            return;
        }

        // EnergyPlus reads every thermal helper before inspecting CFS definitions.
        let thermal_models = self.parse_window_thermal_model_catalog();

        // A shared-name collision is rejected before the first MatrixIndex call. Preserve
        // that lazy edge: an input containing only colliding CFS definitions never loads
        // otherwise-unused Matrix:TwoDimension objects. Case-variant CFS duplicates are
        // left for the validate-before-reservation policy in parse_complex_fenestration_state.
        let mut candidates = Vec::new();
        for (name, object) in definitions {
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredName",
                    COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE,
                    None,
                    None,
                    format!(
                        "{COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE} requires a nonblank object name"
                    ),
                );
                continue;
            }
            if model.construction_names.resolve(&name).is_some() {
                self.duplicate_name(COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE, &name);
                continue;
            }
            candidates.push((name, object));
        }

        let matrices = if candidates.is_empty() {
            Some(MatrixCatalog::new())
        } else {
            // The first surviving MatrixIndex loads and validates every matrix definition,
            // including unused entries. Only referenced snapshots enter TypedModel.
            self.parse_complex_fenestration_matrix_catalog()
        };
        let (Some(thermal_models), Some(matrices)) = (thermal_models, matrices) else {
            return;
        };

        for (name, object) in candidates {
            self.parse_complex_fenestration_state(model, &thermal_models, &matrices, name, object);
        }
    }

    fn parse_window_thermal_model_catalog(&mut self) -> Option<ThermalModelCatalog> {
        let mut catalog = ThermalModelCatalog::new();
        let mut all_valid = true;

        for (name, object) in self.objects(WINDOW_THERMAL_MODEL_OBJECT_TYPE) {
            let name_valid = if name.trim().is_empty() {
                self.error(
                    "MissingRequiredName",
                    WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                    None,
                    None,
                    format!("{WINDOW_THERMAL_MODEL_OBJECT_TYPE} requires a nonblank object name"),
                );
                false
            } else {
                true
            };

            let standard = self.complex_default_enum(
                WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                &name,
                &object,
                "standard",
                WindowThermalCalculationStandard::Iso15099,
                "ISO15099",
                parse_thermal_calculation_standard,
            );
            let thermal_model = self.complex_default_enum(
                WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                &name,
                &object,
                "thermal_model",
                WindowThermalCalculationModel::Iso15099,
                "ISO15099",
                parse_thermal_calculation_model,
            );
            let shading_device_scalar = self.complex_number_default(
                WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                &name,
                &object,
                "sdscalar",
                1.0,
            );
            let deflection_selection = self.complex_default_enum(
                WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                &name,
                &object,
                "deflection_model",
                ParsedDeflectionModel::None,
                "NoDeflection",
                parse_deflection_model,
            );
            let vacuum_pressure_limit_pa = self.complex_number_default(
                WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                &name,
                &object,
                "vacuum_pressure_limit",
                13.238,
            );
            let initial_temperature_c = self.complex_number_default(
                WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                &name,
                &object,
                "initial_temperature",
                25.0,
            );
            let initial_pressure_pa = self.complex_number_default(
                WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                &name,
                &object,
                "initial_pressure",
                101_325.0,
            );

            let shading_device_scalar = shading_device_scalar.and_then(|value| {
                if (0.0..=1.0).contains(&value) {
                    Some(value)
                } else {
                    self.error(
                        "InvalidNumericRange",
                        WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                        Some(&name),
                        Some("sdscalar"),
                        format!(
                            "{WINDOW_THERMAL_MODEL_OBJECT_TYPE}/{name} field sdscalar must be between 0 and 1, got {value}"
                        ),
                    );
                    None
                }
            });

            let deflection_model = match deflection_selection {
                Some(ParsedDeflectionModel::None) => {
                    Some(WindowThermalDeflectionModel::NoDeflection)
                }
                Some(ParsedDeflectionModel::Measured) => {
                    Some(WindowThermalDeflectionModel::MeasuredDeflection)
                }
                Some(ParsedDeflectionModel::TemperatureAndPressure) => {
                    let vacuum_pressure_limit_pa = self.complex_positive_dependent_number(
                        WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                        &name,
                        "vacuum_pressure_limit",
                        vacuum_pressure_limit_pa,
                    );
                    let initial_temperature_c = self.complex_positive_dependent_number(
                        WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                        &name,
                        "initial_temperature",
                        initial_temperature_c,
                    );
                    let initial_pressure_pa = self.complex_positive_dependent_number(
                        WINDOW_THERMAL_MODEL_OBJECT_TYPE,
                        &name,
                        "initial_pressure",
                        initial_pressure_pa,
                    );
                    match (
                        vacuum_pressure_limit_pa,
                        initial_temperature_c,
                        initial_pressure_pa,
                    ) {
                        (Some(vacuum), Some(temperature), Some(pressure)) => {
                            Some(WindowThermalDeflectionModel::TemperatureAndPressureInput {
                                vacuum_pressure_limit_pa: vacuum,
                                initial_temperature_c: temperature,
                                initial_pressure_pa: pressure,
                            })
                        }
                        _ => None,
                    }
                }
                None => None,
            };

            let normalized_name = NormalizedName::new(&name);
            let duplicate = catalog.contains_key(&normalized_name);
            if duplicate {
                self.duplicate_name(WINDOW_THERMAL_MODEL_OBJECT_TYPE, &name);
            }

            match (
                name_valid,
                duplicate,
                standard,
                thermal_model,
                shading_device_scalar,
                deflection_model,
                vacuum_pressure_limit_pa,
                initial_temperature_c,
                initial_pressure_pa,
            ) {
                (
                    true,
                    false,
                    Some(standard),
                    Some(thermal_model),
                    Some(shading_device_scalar),
                    Some(deflection_model),
                    Some(_),
                    Some(_),
                    Some(_),
                ) => {
                    catalog.insert(
                        normalized_name.clone(),
                        WindowThermalModelParameters {
                            name: normalized_name,
                            standard,
                            thermal_model,
                            shading_device_scalar,
                            deflection_model,
                        },
                    );
                }
                _ => all_valid = false,
            }
        }

        all_valid.then_some(catalog)
    }

    fn parse_complex_fenestration_matrix_catalog(&mut self) -> Option<MatrixCatalog> {
        let mut catalog = MatrixCatalog::new();
        let mut all_valid = true;

        for (name, object) in self.objects(MATRIX_TWO_DIMENSION_OBJECT_TYPE) {
            let name_valid = if name.trim().is_empty() {
                self.error(
                    "MissingRequiredName",
                    MATRIX_TWO_DIMENSION_OBJECT_TYPE,
                    None,
                    None,
                    format!("{MATRIX_TWO_DIMENSION_OBJECT_TYPE} requires a nonblank object name"),
                );
                false
            } else {
                true
            };
            let rows = self.required_positive_u32(
                MATRIX_TWO_DIMENSION_OBJECT_TYPE,
                &name,
                &object,
                "number_of_rows",
            );
            let columns = self.required_positive_u32(
                MATRIX_TWO_DIMENSION_OBJECT_TYPE,
                &name,
                &object,
                "number_of_columns",
            );
            let values = self.complex_matrix_values(&name, &object);

            let effective_values = match (rows, columns, values) {
                (Some(rows), Some(columns), Some(values)) => {
                    let required_count = usize::try_from(rows).ok().and_then(|rows| {
                        usize::try_from(columns)
                            .ok()
                            .and_then(|columns| rows.checked_mul(columns))
                    });
                    match required_count {
                        Some(required_count) if values.len() >= required_count => {
                            Some(Arc::<[f64]>::from(&values[..required_count]))
                        }
                        Some(required_count) => {
                            self.error(
                                "IncompleteMatrixValues",
                                MATRIX_TWO_DIMENSION_OBJECT_TYPE,
                                Some(&name),
                                Some("values"),
                                format!(
                                    "{MATRIX_TWO_DIMENSION_OBJECT_TYPE}/{name} provides {} values but its {rows} by {columns} dimensions require {required_count}",
                                    values.len()
                                ),
                            );
                            None
                        }
                        None => {
                            self.error(
                                "MatrixSizeOverflow",
                                MATRIX_TWO_DIMENSION_OBJECT_TYPE,
                                Some(&name),
                                Some("number_of_rows"),
                                format!(
                                    "{MATRIX_TWO_DIMENSION_OBJECT_TYPE}/{name} dimensions {rows} by {columns} exceed the host matrix size"
                                ),
                            );
                            None
                        }
                    }
                }
                _ => None,
            };

            let normalized_name = NormalizedName::new(&name);
            let duplicate = catalog.contains_key(&normalized_name);
            if duplicate {
                self.duplicate_name(MATRIX_TWO_DIMENSION_OBJECT_TYPE, &name);
            }
            match (name_valid, duplicate, rows, columns, effective_values) {
                (true, false, Some(rows), Some(columns), Some(values)) => {
                    catalog.insert(
                        normalized_name,
                        ComplexFenestrationMatrix {
                            source_name: name,
                            rows,
                            columns,
                            values,
                        },
                    );
                }
                _ => all_valid = false,
            }
        }

        all_valid.then_some(catalog)
    }

    fn parse_complex_fenestration_state(
        &mut self,
        model: &mut TypedModel,
        thermal_models: &ThermalModelCatalog,
        matrices: &MatrixCatalog,
        name: String,
        object: RawObject,
    ) {
        let object_type = COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE;
        let name_valid = if name.trim().is_empty() {
            self.error(
                "MissingRequiredName",
                object_type,
                None,
                None,
                format!("{object_type} requires a nonblank object name"),
            );
            false
        } else {
            true
        };

        let basis_type = self.complex_default_enum(
            object_type,
            &name,
            &object,
            "basis_type",
            ComplexFenestrationBasisType::LbnlWindow,
            "LBNLWINDOW",
            parse_basis_type,
        );
        let basis_symmetry = self.complex_default_enum(
            object_type,
            &name,
            &object,
            "basis_symmetry_type",
            ComplexFenestrationBasisSymmetry::None,
            "None",
            parse_basis_symmetry,
        );
        let basis_type = basis_type.and_then(|basis_type| match basis_type {
            ComplexFenestrationBasisType::LbnlWindow => Some(basis_type),
            ComplexFenestrationBasisType::UserDefined => {
                self.error(
                    "UnsupportedComplexFenestrationBasis",
                    object_type,
                    Some(&name),
                    Some("basis_type"),
                    format!(
                        "{object_type}/{name} uses UserDefined basis input; the EnergyPlus 26.1 custom-basis path is not implemented in the bounded typed slice"
                    ),
                );
                None
            }
        });
        let basis_symmetry = basis_symmetry.and_then(|basis_symmetry| match basis_symmetry {
            ComplexFenestrationBasisSymmetry::None => Some(basis_symmetry),
            ComplexFenestrationBasisSymmetry::Axisymmetric => {
                self.error(
                    "UnsupportedComplexFenestrationBasis",
                    object_type,
                    Some(&name),
                    Some("basis_symmetry_type"),
                    format!(
                        "{object_type}/{name} uses Axisymmetric matrices; the EnergyPlus 26.1 branch is fail-closed by this bounded typed slice"
                    ),
                );
                None
            }
        });

        let thermal_model = self.required_thermal_model_reference(
            thermal_models,
            &name,
            &object,
            "window_thermal_model",
        );
        let basis_matrix =
            self.required_complex_matrix_reference(matrices, &name, &object, "basis_matrix_name");
        let solar_front_transmittance = self.required_complex_matrix_reference(
            matrices,
            &name,
            &object,
            "solar_optical_complex_front_transmittance_matrix_name",
        );
        let solar_back_reflectance = self.required_complex_matrix_reference(
            matrices,
            &name,
            &object,
            "solar_optical_complex_back_reflectance_matrix_name",
        );
        let visible_front_transmittance = self.required_complex_matrix_reference(
            matrices,
            &name,
            &object,
            "visible_optical_complex_front_transmittance_matrix_name",
        );
        let visible_back_reflectance = self.required_complex_matrix_reference(
            matrices,
            &name,
            &object,
            "visible_optical_complex_back_transmittance_matrix_name",
        );

        let basis_length = basis_matrix
            .as_ref()
            .and_then(|matrix| self.complex_basis_length(&name, matrix));
        if let Some(basis_length) = basis_length {
            for (field, matrix) in [
                (
                    "solar_optical_complex_front_transmittance_matrix_name",
                    solar_front_transmittance.as_ref(),
                ),
                (
                    "solar_optical_complex_back_reflectance_matrix_name",
                    solar_back_reflectance.as_ref(),
                ),
                (
                    "visible_optical_complex_front_transmittance_matrix_name",
                    visible_front_transmittance.as_ref(),
                ),
                (
                    "visible_optical_complex_back_transmittance_matrix_name",
                    visible_back_reflectance.as_ref(),
                ),
            ] {
                if let Some(matrix) = matrix {
                    self.validate_complex_matrix_dimensions(
                        &name,
                        field,
                        matrix,
                        basis_length,
                        basis_length,
                    );
                }
            }
        }

        let layer_state = basis_length.and_then(|basis_length| {
            self.complex_fenestration_layers(model, matrices, &name, &object, basis_length)
        });

        let Some(basis_type) = basis_type else {
            return;
        };
        let Some(basis_symmetry) = basis_symmetry else {
            return;
        };
        let Some(thermal_model) = thermal_model else {
            return;
        };
        let Some(basis_matrix) = basis_matrix else {
            return;
        };
        let Some(basis_length) = basis_length else {
            return;
        };
        let Some(solar_front_transmittance) = solar_front_transmittance else {
            return;
        };
        let Some(solar_back_reflectance) = solar_back_reflectance else {
            return;
        };
        let Some(visible_front_transmittance) = visible_front_transmittance else {
            return;
        };
        let Some(visible_back_reflectance) = visible_back_reflectance else {
            return;
        };
        let Some((layers, optical_layers)) = layer_state else {
            return;
        };
        if !name_valid
            || !self.complex_matrix_has_dimensions(
                &solar_front_transmittance,
                basis_length,
                basis_length,
            )
            || !self.complex_matrix_has_dimensions(
                &solar_back_reflectance,
                basis_length,
                basis_length,
            )
            || !self.complex_matrix_has_dimensions(
                &visible_front_transmittance,
                basis_length,
                basis_length,
            )
            || !self.complex_matrix_has_dimensions(
                &visible_back_reflectance,
                basis_length,
                basis_length,
            )
        {
            return;
        }

        let Some(id_value) = self.checked_id(object_type, &name, model.constructions.len()) else {
            return;
        };
        let id = ConstructionId(id_value);
        if model.construction_names.insert(&name, id).is_some() {
            self.duplicate_name(object_type, &name);
            return;
        }
        let outside_layer = layers.first().copied();
        model.constructions.push(Construction {
            id,
            name: NormalizedName::new(&name),
            kind: ConstructionKind::ComplexFenestration,
            outside_layer,
            layers,
            thermochromic_master: None,
            ground_factor: None,
            air_boundary: None,
            complex_fenestration: Some(ConstructionComplexFenestrationState {
                basis_type,
                basis_symmetry,
                thermal_model,
                basis_matrix,
                basis_length,
                solar_front_transmittance,
                solar_back_reflectance,
                visible_front_transmittance,
                visible_back_reflectance,
                optical_layers,
            }),
        });
    }

    fn complex_fenestration_layers(
        &mut self,
        model: &TypedModel,
        matrices: &MatrixCatalog,
        object_name: &str,
        object: &RawObject,
        basis_length: u32,
    ) -> Option<(Vec<MaterialId>, Vec<ComplexFenestrationOpticalLayer>)> {
        let object_type = COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE;
        let mut highest_position = 0_usize;
        for position in 0..9 {
            let fields = if position % 2 == 0 {
                let (name, front, back) = OPTICAL_LAYER_FIELDS[position / 2];
                [name, front, back]
            } else {
                let (name, front, back) = GAP_LAYER_FIELDS[position / 2];
                [name, front, back]
            };
            if fields
                .iter()
                .any(|field| field_value(object, field).is_some())
            {
                highest_position = position;
            }
        }

        let mut valid = true;
        if !highest_position.is_multiple_of(2) {
            self.error(
                "InvalidComplexFenestrationLayerTopology",
                object_type,
                Some(object_name),
                None,
                format!(
                    "{object_type}/{object_name} layer sequence must end with an optical layer"
                ),
            );
            valid = false;
        }

        let mut layers = Vec::new();
        let mut optical_layers = Vec::new();
        for position in 0..=highest_position {
            if position % 2 == 0 {
                let (material_field, front_field, back_field) = OPTICAL_LAYER_FIELDS[position / 2];
                let material = self.required_complex_layer_material(
                    model,
                    object_name,
                    object,
                    material_field,
                    ComplexLayerExpectation::Optical,
                );
                let front_absorptance = self.required_complex_matrix_reference(
                    matrices,
                    object_name,
                    object,
                    front_field,
                );
                let back_absorptance = self.required_complex_matrix_reference(
                    matrices,
                    object_name,
                    object,
                    back_field,
                );
                if let Some(matrix) = front_absorptance.as_ref()
                    && !self.validate_complex_matrix_dimensions(
                        object_name,
                        front_field,
                        matrix,
                        1,
                        basis_length,
                    )
                {
                    valid = false;
                }
                if let Some(matrix) = back_absorptance.as_ref()
                    && !self.validate_complex_matrix_dimensions(
                        object_name,
                        back_field,
                        matrix,
                        1,
                        basis_length,
                    )
                {
                    valid = false;
                }
                match (material, front_absorptance, back_absorptance) {
                    (Some(material), Some(front_absorptance), Some(back_absorptance)) => {
                        layers.push(material);
                        optical_layers.push(ComplexFenestrationOpticalLayer {
                            material,
                            front_absorptance,
                            back_absorptance,
                        });
                    }
                    _ => valid = false,
                }
            } else {
                let (material_field, reserved_front, reserved_back) =
                    GAP_LAYER_FIELDS[position / 2];
                let material = self.required_complex_layer_material(
                    model,
                    object_name,
                    object,
                    material_field,
                    ComplexLayerExpectation::Gap,
                );
                if !self.complex_reserved_gap_field_is_blank(object_name, object, reserved_front) {
                    valid = false;
                }
                if !self.complex_reserved_gap_field_is_blank(object_name, object, reserved_back) {
                    valid = false;
                }
                match material {
                    Some(material) => layers.push(material),
                    None => valid = false,
                }
            }
        }

        (valid && !layers.is_empty()).then_some((layers, optical_layers))
    }

    fn required_complex_layer_material(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &RawObject,
        field: &str,
        expectation: ComplexLayerExpectation,
    ) -> Option<MaterialId> {
        let object_type = COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE;
        let material_name = self.required_string(object_type, object_name, object, field)?;
        let material_id = self.resolve_name(
            &model.material_names,
            object_type,
            object_name,
            field,
            &material_name,
            expectation.target_label(),
        )?;
        let Some(material) = model.materials.get(material_id.0 as usize) else {
            self.error(
                "InvalidMaterialReference",
                object_type,
                Some(object_name),
                Some(field),
                format!(
                    "{object_type}/{object_name} field {field} resolved an invalid material ID"
                ),
            );
            return None;
        };
        let accepted = match expectation {
            ComplexLayerExpectation::Optical => matches!(
                material.definition,
                MaterialDefinition::WindowGlazingSpectralAverage(_)
                    | MaterialDefinition::WindowComplexShade(_)
            ),
            ComplexLayerExpectation::Gap => {
                matches!(material.definition, MaterialDefinition::WindowComplexGap(_))
            }
        };
        if accepted {
            return Some(material_id);
        }

        self.error(
            "InvalidComplexFenestrationLayerMaterial",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} references {material_name}; {}",
                expectation.supported_boundary()
            ),
        );
        None
    }

    fn required_thermal_model_reference(
        &mut self,
        catalog: &ThermalModelCatalog,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<WindowThermalModelParameters> {
        let object_type = COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE;
        let name = self.required_string(object_type, object_name, object, field)?;
        if let Some(model) = catalog.get(&NormalizedName::new(&name)) {
            return Some(model.clone());
        }
        self.error(
            "MissingReference",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} references missing {WINDOW_THERMAL_MODEL_OBJECT_TYPE} '{name}'"
            ),
        );
        None
    }

    fn required_complex_matrix_reference(
        &mut self,
        catalog: &MatrixCatalog,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<ComplexFenestrationMatrix> {
        let object_type = COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE;
        let name = self.required_string(object_type, object_name, object, field)?;
        if let Some(matrix) = catalog.get(&NormalizedName::new(&name)) {
            return Some(matrix.clone());
        }
        self.error(
            "MissingReference",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} references missing {MATRIX_TWO_DIMENSION_OBJECT_TYPE} '{name}'"
            ),
        );
        None
    }

    fn complex_basis_length(
        &mut self,
        object_name: &str,
        matrix: &ComplexFenestrationMatrix,
    ) -> Option<u32> {
        let object_type = COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE;
        if matrix.columns == 1 {
            return Some(matrix.rows);
        }
        if matrix.columns != 2 {
            self.error(
                "InvalidComplexFenestrationBasisMatrix",
                object_type,
                Some(object_name),
                Some("basis_matrix_name"),
                format!(
                    "{object_type}/{object_name} basis matrix {} must have one or two columns, got {}",
                    matrix.source_name, matrix.columns
                ),
            );
            return None;
        }

        let mut basis_length = 1.0_f64;
        for row in 1..matrix.rows as usize {
            let Some(value) = matrix.get(row, 1) else {
                self.error(
                    "InvalidComplexFenestrationBasisMatrix",
                    object_type,
                    Some(object_name),
                    Some("basis_matrix_name"),
                    format!(
                        "{object_type}/{object_name} basis matrix {} is missing row {row} column 1",
                        matrix.source_name
                    ),
                );
                return None;
            };
            basis_length += (value + 0.001).floor();
        }
        if basis_length.is_finite()
            && basis_length.fract() == 0.0
            && basis_length >= 1.0
            && basis_length <= f64::from(u32::MAX)
        {
            return Some(basis_length as u32);
        }

        self.error(
            "InvalidComplexFenestrationBasisMatrix",
            object_type,
            Some(object_name),
            Some("basis_matrix_name"),
            format!(
                "{object_type}/{object_name} basis matrix {} derives invalid basis length {basis_length}",
                matrix.source_name
            ),
        );
        None
    }

    fn validate_complex_matrix_dimensions(
        &mut self,
        object_name: &str,
        field: &str,
        matrix: &ComplexFenestrationMatrix,
        rows: u32,
        columns: u32,
    ) -> bool {
        if self.complex_matrix_has_dimensions(matrix, rows, columns) {
            return true;
        }
        let object_type = COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE;
        self.error(
            "InvalidComplexFenestrationMatrixDimensions",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} references {} with dimensions {} by {}; expected {rows} by {columns}",
                matrix.source_name, matrix.rows, matrix.columns
            ),
        );
        false
    }

    fn complex_matrix_has_dimensions(
        &self,
        matrix: &ComplexFenestrationMatrix,
        rows: u32,
        columns: u32,
    ) -> bool {
        matrix.rows == rows && matrix.columns == columns
    }

    fn complex_reserved_gap_field_is_blank(
        &mut self,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> bool {
        match field_value(object, field) {
            None => true,
            Some(RawValue::String(value)) if value.trim().is_empty() => true,
            Some(RawValue::String(value)) => {
                let object_type = COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE;
                self.error(
                    "UnsupportedComplexFenestrationReservedField",
                    object_type,
                    Some(object_name),
                    Some(field),
                    format!(
                        "{object_type}/{object_name} field {field} is reserved for future use and must be blank, got '{value}'"
                    ),
                );
                false
            }
            Some(_) => {
                self.invalid_field_type(
                    COMPLEX_FENESTRATION_CONSTRUCTION_OBJECT_TYPE,
                    object_name,
                    field,
                    "blank string",
                );
                false
            }
        }
    }

    fn complex_matrix_values(&mut self, object_name: &str, object: &RawObject) -> Option<Vec<f64>> {
        let Some(value) = field_value(object, "values") else {
            return Some(Vec::new());
        };
        let RawValue::Array(entries) = value else {
            self.invalid_field_type(
                MATRIX_TWO_DIMENSION_OBJECT_TYPE,
                object_name,
                "values",
                "array of value objects",
            );
            return None;
        };

        let mut values = Vec::with_capacity(entries.len());
        let mut valid = true;
        for (index, entry) in entries.iter().enumerate() {
            let RawValue::Object(fields) = entry else {
                self.error(
                    "InvalidFieldType",
                    MATRIX_TWO_DIMENSION_OBJECT_TYPE,
                    Some(object_name),
                    Some("values"),
                    format!(
                        "{MATRIX_TWO_DIMENSION_OBJECT_TYPE}/{object_name} values entry {index} must be an object"
                    ),
                );
                valid = false;
                continue;
            };
            let value = fields.get(&FieldName("value".to_string()));
            match value {
                None => values.push(0.0),
                Some(value) => match self.number_value(
                    MATRIX_TWO_DIMENSION_OBJECT_TYPE,
                    object_name,
                    &format!("values[{index}].value"),
                    value,
                ) {
                    Some(value) => values.push(value),
                    None => valid = false,
                },
            }
        }
        valid.then_some(values)
    }

    fn complex_number_default(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        default: f64,
    ) -> Option<f64> {
        match field_value(object, field) {
            None => {
                self.record_default(
                    object_type,
                    object_name,
                    field,
                    &format_complex_number(default),
                );
                Some(default)
            }
            Some(RawValue::String(value)) if value.trim().is_empty() => {
                self.record_default(
                    object_type,
                    object_name,
                    field,
                    &format_complex_number(default),
                );
                Some(default)
            }
            Some(value) => self.number_value(object_type, object_name, field, value),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn complex_default_enum<T: Copy>(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        default: T,
        default_label: &str,
        parser: fn(&str) -> Option<T>,
    ) -> Option<T> {
        match field_value(object, field) {
            None => {
                self.record_default(object_type, object_name, field, default_label);
                Some(default)
            }
            Some(RawValue::String(value)) if value.trim().is_empty() => {
                self.record_default(object_type, object_name, field, default_label);
                Some(default)
            }
            Some(RawValue::String(value)) => match parser(value) {
                Some(value) => Some(value),
                None => {
                    self.invalid_enum_value(object_type, object_name, field, value);
                    None
                }
            },
            Some(_) => {
                self.invalid_field_type(object_type, object_name, field, "string enum");
                None
            }
        }
    }

    fn complex_positive_dependent_number(
        &mut self,
        object_type: &str,
        object_name: &str,
        field: &str,
        value: Option<f64>,
    ) -> Option<f64> {
        let value = value?;
        if value > 0.0 {
            return Some(value);
        }
        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} must be greater than 0 for TemperatureAndPressureInput, got {value}"
            ),
        );
        None
    }
}
