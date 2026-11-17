use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{
    MaterialDefinition, MaterialFamily, MaterialId, MaterialKind, WindowGapVentType,
    WindowGasProperties, WindowGasType,
};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "WindowMaterial:Gap:EquivalentLayer";

fn assert_close(actual: f64, expected: f64) {
    let tolerance = f64::EPSILON * expected.abs().max(1.0) * 16.0;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

fn assert_polynomial(
    properties: WindowGasProperties,
    conductivity: [f64; 3],
    viscosity: [f64; 3],
    specific_heat: [f64; 3],
) {
    assert_eq!(
        [
            properties.conductivity.coefficient_a,
            properties.conductivity.coefficient_b,
            properties.conductivity.coefficient_c,
        ],
        conductivity
    );
    assert_eq!(
        [
            properties.viscosity.coefficient_a,
            properties.viscosity.coefficient_b,
            properties.viscosity.coefficient_c,
        ],
        viscosity
    );
    assert_eq!(
        [
            properties.specific_heat.coefficient_a,
            properties.specific_heat.coefficient_b,
            properties.specific_heat.coefficient_c,
        ],
        specific_heat
    );
}

#[test]
fn equivalent_layer_standard_gaps_use_v261_constants_and_preserve_vent_types()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Air Gap": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                },
                "Argon Gap": {
                    "gas_type":"ARGON",
                    "thickness":0.013,
                    "gap_vent_type":"VentedIndoor",
                    "conductivity_coefficient_a":99.0,
                    "conductivity_coefficient_b":98.0,
                    "conductivity_coefficient_c":97.0,
                    "viscosity_coefficient_a":1.0,
                    "viscosity_coefficient_b":2.0,
                    "viscosity_coefficient_c":3.0,
                    "specific_heat_coefficient_a":100.0,
                    "specific_heat_coefficient_b":101.0,
                    "specific_heat_coefficient_c":102.0,
                    "molecular_weight":100.0,
                    "specific_heat_ratio":2.0
                },
                "Krypton Gap": {
                    "gas_type":"KRYPTON",
                    "thickness":0.014,
                    "gap_vent_type":"VentedOutdoor"
                },
                "Xenon Gap": {
                    "gas_type":"XENON",
                    "thickness":0.015,
                    "gap_vent_type":"Sealed"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected standard equivalent-layer gaps"))?;
    let expected = [
        (
            WindowGasType::Air,
            WindowGapVentType::Sealed,
            0.012,
            [2.873e-3, 7.760e-5, 0.0],
            [3.723e-6, 4.940e-8, 0.0],
            [1002.737, 1.2324e-2, 0.0],
            28.97,
            1.4,
        ),
        (
            WindowGasType::Argon,
            WindowGapVentType::VentedIndoor,
            0.013,
            [2.285e-3, 5.149e-5, 0.0],
            [3.379e-6, 6.451e-8, 0.0],
            [521.929, 0.0, 0.0],
            39.948,
            1.67,
        ),
        (
            WindowGasType::Krypton,
            WindowGapVentType::VentedOutdoor,
            0.014,
            [9.443e-4, 2.826e-5, 0.0],
            [2.213e-6, 7.777e-8, 0.0],
            [248.091, 0.0, 0.0],
            83.8,
            1.68,
        ),
        (
            WindowGasType::Xenon,
            WindowGapVentType::Sealed,
            0.015,
            [4.538e-4, 1.723e-5, 0.0],
            [1.069e-6, 7.414e-8, 0.0],
            [158.340, 0.0, 0.0],
            131.3,
            1.66,
        ),
    ];

    assert_eq!(model.materials.len(), expected.len());
    for (
        material,
        (gas_type, gap_vent_type, thickness, conductivity, viscosity, specific_heat, weight, ratio),
    ) in model.materials.iter().zip(expected)
    {
        assert_eq!(material.kind(), MaterialKind::WindowGapEquivalentLayer);
        assert_eq!(material.family(), MaterialFamily::EquivalentLayer);
        assert!(matches!(
            material.definition,
            MaterialDefinition::WindowGapEquivalentLayer(_)
        ));
        assert!(material.as_opaque().is_none());
        assert_eq!(material.thickness_m(), None);
        assert_eq!(material.conductivity_w_per_m_k(), None);
        assert_eq!(material.thermal_resistance(), None);
        assert_eq!(material.heat_capacity_per_area(), None);

        let gap = material
            .as_window_gap_equivalent_layer()
            .ok_or_else(|| std::io::Error::other("expected equivalent-layer gap payload"))?;
        assert_eq!(gap.gas_type, gas_type);
        assert_eq!(gap.gap_vent_type, gap_vent_type);
        assert_eq!(gap.thickness_m, thickness);
        assert_polynomial(gap.properties, conductivity, viscosity, specific_heat);
        assert_eq!(gap.properties.molecular_weight_g_per_mol, weight);
        assert_eq!(gap.properties.specific_heat_ratio, ratio);
        assert_close(
            gap.nominal_thermal_resistance_m2_k_per_w()
                .ok_or_else(|| std::io::Error::other("expected nominal gap resistance"))?,
            thickness / (conductivity[0] + 300.0 * conductivity[1]),
        );
    }

    // GapEQL Source qualifier and original-schema boundary; no runtime or gas-state seed.
    {
        use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue};
        const GAP_EQL: &str = "WindowMaterial:Gap:EquivalentLayer";
        let full_text = r#"{"WindowMaterial:Gap:EquivalentLayer":{" FULL GAP EQL ":{
            "gas_type":"CUSTOM","thickness":5662.75,"gap_vent_type":"VentedOutdoor",
            "conductivity_coefficient_a":0.25,"conductivity_coefficient_b":0.125,"conductivity_coefficient_c":0.0625,
            "viscosity_coefficient_a":0.125,"viscosity_coefficient_b":-0.25,"viscosity_coefficient_c":0.0625,
            "specific_heat_coefficient_a":2.0,"specific_heat_coefficient_b":-0.5,"specific_heat_coefficient_c":0.25,
            "molecular_weight":200.0,"specific_heat_ratio":2.0,
            "name":false,"idf_order":[],"idf_max_fields":null,"metadata":{"held":[null,true,1,"text",[],{}]}
        }}}"#;
        let full_raw = ep_raw_model::parse_epjson_str_for_source(full_text)?;
        let source = super::super::compile_raw_model_for_source(&full_raw);
        let source_model = source.model.as_ref().expect("full Source GapEQL carrier");
        let full = source_model.materials[0]
            .as_window_gap_equivalent_layer()
            .expect("actual GapEQL typed carrier");
        assert!(
            !source.has_errors()
                && source_model.material_names.is_source_exact_namespace()
                && source_model.materials.len() == 1
                && source_model.materials[0].id == MaterialId(0)
                && source_model.materials[0].name.0 == " FULL GAP EQL "
                && source_model.material_names.resolve(" full gap eql ") == Some(MaterialId(0))
                && source_model.ctf_report_names.materials
                    == [(MaterialId(0), " FULL GAP EQL ".into())]
                && full.gas_type == WindowGasType::Custom
                && full.gap_vent_type == WindowGapVentType::VentedOutdoor
                && [
                    full.thickness_m,
                    full.properties.conductivity.coefficient_a,
                    full.properties.conductivity.coefficient_b,
                    full.properties.conductivity.coefficient_c,
                    full.properties.viscosity.coefficient_a,
                    full.properties.viscosity.coefficient_b,
                    full.properties.viscosity.coefficient_c,
                    full.properties.specific_heat.coefficient_a,
                    full.properties.specific_heat.coefficient_b,
                    full.properties.specific_heat.coefficient_c,
                    full.properties.molecular_weight_g_per_mol,
                    full.properties.specific_heat_ratio
                ] == [
                    5662.75, 0.25, 0.125, 0.0625, 0.125, -0.25, 0.0625, 2.0, -0.5, 0.25, 200.0, 2.0
                ]
        );
        let standard = [
            (
                "AIR",
                WindowGasType::Air,
                [
                    2.873e-3, 7.760e-5, 0.0, 3.723e-6, 4.940e-8, 0.0, 1002.737, 1.2324e-2, 0.0,
                    28.97, 1.4,
                ],
            ),
            (
                "ARGON",
                WindowGasType::Argon,
                [
                    2.285e-3, 5.149e-5, 0.0, 3.379e-6, 6.451e-8, 0.0, 521.929, 0.0, 0.0, 39.948,
                    1.67,
                ],
            ),
            (
                "KRYPTON",
                WindowGasType::Krypton,
                [
                    9.443e-4, 2.826e-5, 0.0, 2.213e-6, 7.777e-8, 0.0, 248.091, 0.0, 0.0, 83.8, 1.68,
                ],
            ),
            (
                "XENON",
                WindowGasType::Xenon,
                [
                    4.538e-4, 1.723e-5, 0.0, 1.069e-6, 7.414e-8, 0.0, 158.340, 0.0, 0.0, 131.3,
                    1.66,
                ],
            ),
            (
                "CUSTOM",
                WindowGasType::Custom,
                [
                    0.25, 0.125, 0.0625, 0.125, -0.25, 0.0625, 2.0, -0.5, 0.25, 200.0, 2.0,
                ],
            ),
        ];
        assert!(standard.iter().all(|(gas, kind, expected)| {
            ["Sealed", "VentedIndoor", "VentedOutdoor"]
                .iter()
                .all(|vent| {
                    let mut raw = full_raw.clone();
                    let fields = &mut raw
                        .objects
                        .get_mut(&ObjectType(GAP_EQL.into()))
                        .expect("family")
                        .get_mut(&ObjectName(" FULL GAP EQL ".into()))
                        .expect("full GapEQL object")
                        .fields;
                    fields.insert(
                        FieldName("gas_type".into()),
                        RawValue::String((*gas).into()),
                    );
                    fields.insert(
                        FieldName("gap_vent_type".into()),
                        RawValue::String((*vent).into()),
                    );
                    let result = super::super::compile_raw_model_for_source(&raw);
                    !result.has_errors()
                        && result.model.as_ref().is_some_and(|model| {
                            model.material_names.is_source_exact_namespace()
                                && model.materials[0]
                                    .as_window_gap_equivalent_layer()
                                    .is_some_and(|row| {
                                        row.gas_type == *kind
                                            && row.gap_vent_type.energyplus_name() == *vent
                                            && row.thickness_m == 5662.75
                                            && [
                                                row.properties.conductivity.coefficient_a,
                                                row.properties.conductivity.coefficient_b,
                                                row.properties.conductivity.coefficient_c,
                                                row.properties.viscosity.coefficient_a,
                                                row.properties.viscosity.coefficient_b,
                                                row.properties.viscosity.coefficient_c,
                                                row.properties.specific_heat.coefficient_a,
                                                row.properties.specific_heat.coefficient_b,
                                                row.properties.specific_heat.coefficient_c,
                                                row.properties.molecular_weight_g_per_mol,
                                                row.properties.specific_heat_ratio,
                                            ] == *expected
                                    })
                        })
                })
        }));
        let minimum_text = r#"{"WindowMaterial:Gap:EquivalentLayer":{"MIN GAP":{
            "gas_type":"CUSTOM","thickness":0.125,"gap_vent_type":"Sealed","conductivity_coefficient_a":0.5,
            "viscosity_coefficient_a":0.125,"specific_heat_coefficient_a":2.0,"molecular_weight":20.0
        }}}"#;
        let minimum_raw = ep_raw_model::parse_epjson_str_for_source(minimum_text)?;
        let minimum_source = super::super::compile_raw_model_for_source(&minimum_raw);
        assert!(
            !minimum_source.has_errors()
                && minimum_source
                    .model
                    .as_ref()
                    .is_some_and(|model| model.materials[0]
                        .as_window_gap_equivalent_layer()
                        .is_some_and(|row| row.properties.specific_heat_ratio.to_bits() == 0
                            && row.properties.conductivity.coefficient_b.to_bits() == 0
                            && row.properties.viscosity.coefficient_c.to_bits() == 0))
                && !compile_raw_model(&minimum_raw).has_errors()
                && [
                    "viscosity_coefficient_a",
                    "specific_heat_coefficient_a",
                    "molecular_weight",
                    "ZERO_K",
                    "NEGATIVE_K"
                ]
                .iter()
                .all(|probe| {
                    let mut raw = minimum_raw.clone();
                    let fields = &mut raw
                        .objects
                        .get_mut(&ObjectType(GAP_EQL.into()))
                        .expect("family")
                        .get_mut(&ObjectName("MIN GAP".into()))
                        .expect("Custom object")
                        .fields;
                    if *probe == "ZERO_K" {
                        fields.insert(
                            FieldName("conductivity_coefficient_a".into()),
                            RawValue::Number("0.0".into()),
                        );
                    } else if *probe == "NEGATIVE_K" {
                        fields.insert(
                            FieldName("conductivity_coefficient_a".into()),
                            RawValue::Number("-0.5".into()),
                        );
                    } else {
                        fields.remove(&FieldName((*probe).into()));
                    }
                    let source = super::super::compile_raw_model_for_source(&raw);
                    let legacy = compile_raw_model(&raw);
                    !source.has_errors()
                        && legacy.has_errors()
                        && legacy.report.diagnostics.iter().any(|row| {
                            row.object_type == GAP_EQL
                                && row.code
                                    == if matches!(*probe, "ZERO_K" | "NEGATIVE_K") {
                                        "InvalidWindowGapEquivalentLayerConductivityAt300K"
                                    } else {
                                        "MissingCustomWindowGapEquivalentLayerProperty"
                                    }
                        })
                        && source.model.as_ref().is_some_and(|model| {
                            model.material_names.is_source_exact_namespace()
                                && model.ctf_report_names.materials
                                    == [(MaterialId(0), "MIN GAP".into())]
                        })
                })
        );
        let mut duplicate = full_raw.clone();
        let original = duplicate
            .objects
            .get(&ObjectType(GAP_EQL.into()))
            .expect("family")[&ObjectName(" FULL GAP EQL ".into())]
            .clone();
        duplicate
            .objects
            .get_mut(&ObjectType(GAP_EQL.into()))
            .expect("family")
            .insert(ObjectName(" full gap eql ".into()), original);
        let duplicate = super::super::compile_raw_model_for_source(&duplicate);
        assert!(
            duplicate.has_errors()
                && duplicate
                    .report
                    .diagnostics
                    .iter()
                    .any(|row| row.code == "DuplicateName" && row.object_type == GAP_EQL)
                && duplicate.model.as_ref().is_none_or(|model| model
                    .ctf_report_names
                    .materials
                    .len()
                    == 1)
                && ["", "\n", " \t\r\u{c}"].iter().all(|name| {
                    let mut raw = full_raw.clone();
                    let rows = raw
                        .objects
                        .get_mut(&ObjectType(GAP_EQL.into()))
                        .expect("family");
                    let row = rows
                        .remove(&ObjectName(" FULL GAP EQL ".into()))
                        .expect("row");
                    rows.insert(ObjectName((*name).into()), row);
                    let result = super::super::compile_raw_model_for_source(&raw);
                    result.has_errors()
                        && result.model.as_ref().is_none_or(|model| {
                            !model.material_names.is_source_exact_namespace()
                                && model.ctf_report_names.materials.is_empty()
                        })
                })
        );
        let idf_raw = ep_raw_model::parse_epjson_str_with_idf_order(
            r#"{"Zone":{"Zone":{}},"WindowMaterial:Gap:EquivalentLayer":{"IDF GAP EQL":{"gas_type":"AIR","thickness":0.125,"gap_vent_type":"Sealed"}}}"#,
            "Zone,Zone;WindowMaterial:Gap:EquivalentLayer,IDF GAP EQL,AIR,0.125,Sealed;",
        )?;
        let idf_source = super::super::compile_raw_model_for_source(&idf_raw);
        let idf_legacy = compile_raw_model(&idf_raw);
        let mut active = full_raw.clone();
        let construction = ep_raw_model::parse_epjson_str_for_source(
            r#"{"Construction":{"ACTIVE GAP":{"outside_layer":" full gap eql "}}}"#,
        )?;
        active.objects.extend(construction.objects);
        let active = super::super::compile_raw_model_for_source(&active);
        assert!(
            !idf_raw.is_epjson()
                && !idf_source.has_errors()
                && !idf_legacy.has_errors()
                && idf_source.model.as_ref().is_some_and(|model| !model
                    .material_names
                    .is_source_exact_namespace()
                    && model.ctf_report_names.materials.is_empty()
                    && model.materials[0].kind() == MaterialKind::WindowGapEquivalentLayer)
                && !compile_raw_model(&full_raw).has_errors()
                && active.has_errors()
                && active
                    .report
                    .diagnostics
                    .iter()
                    .any(|row| row.code == "InvalidEquivalentLayerConstruction")
        );
        for (field, kind, probe) in [
            ("thickness", "kind", 0_usize),
            ("thickness", "kind", 1_usize),
            ("thickness", "kind", 2_usize),
            ("thickness", "kind", 3_usize),
            ("thickness", "kind", 4_usize),
            ("conductivity_coefficient_a", "kind", 0_usize),
            ("conductivity_coefficient_a", "kind", 1_usize),
            ("conductivity_coefficient_a", "kind", 2_usize),
            ("conductivity_coefficient_a", "kind", 3_usize),
            ("conductivity_coefficient_a", "kind", 4_usize),
            ("conductivity_coefficient_b", "kind", 0_usize),
            ("conductivity_coefficient_b", "kind", 1_usize),
            ("conductivity_coefficient_b", "kind", 2_usize),
            ("conductivity_coefficient_b", "kind", 3_usize),
            ("conductivity_coefficient_b", "kind", 4_usize),
            ("conductivity_coefficient_c", "kind", 0_usize),
            ("conductivity_coefficient_c", "kind", 1_usize),
            ("conductivity_coefficient_c", "kind", 2_usize),
            ("conductivity_coefficient_c", "kind", 3_usize),
            ("conductivity_coefficient_c", "kind", 4_usize),
            ("viscosity_coefficient_a", "kind", 0_usize),
            ("viscosity_coefficient_a", "kind", 1_usize),
            ("viscosity_coefficient_a", "kind", 2_usize),
            ("viscosity_coefficient_a", "kind", 3_usize),
            ("viscosity_coefficient_a", "kind", 4_usize),
            ("viscosity_coefficient_b", "kind", 0_usize),
            ("viscosity_coefficient_b", "kind", 1_usize),
            ("viscosity_coefficient_b", "kind", 2_usize),
            ("viscosity_coefficient_b", "kind", 3_usize),
            ("viscosity_coefficient_b", "kind", 4_usize),
            ("viscosity_coefficient_c", "kind", 0_usize),
            ("viscosity_coefficient_c", "kind", 1_usize),
            ("viscosity_coefficient_c", "kind", 2_usize),
            ("viscosity_coefficient_c", "kind", 3_usize),
            ("viscosity_coefficient_c", "kind", 4_usize),
            ("specific_heat_coefficient_a", "kind", 0_usize),
            ("specific_heat_coefficient_a", "kind", 1_usize),
            ("specific_heat_coefficient_a", "kind", 2_usize),
            ("specific_heat_coefficient_a", "kind", 3_usize),
            ("specific_heat_coefficient_a", "kind", 4_usize),
            ("specific_heat_coefficient_b", "kind", 0_usize),
            ("specific_heat_coefficient_b", "kind", 1_usize),
            ("specific_heat_coefficient_b", "kind", 2_usize),
            ("specific_heat_coefficient_b", "kind", 3_usize),
            ("specific_heat_coefficient_b", "kind", 4_usize),
            ("specific_heat_coefficient_c", "kind", 0_usize),
            ("specific_heat_coefficient_c", "kind", 1_usize),
            ("specific_heat_coefficient_c", "kind", 2_usize),
            ("specific_heat_coefficient_c", "kind", 3_usize),
            ("specific_heat_coefficient_c", "kind", 4_usize),
            ("molecular_weight", "kind", 0_usize),
            ("molecular_weight", "kind", 1_usize),
            ("molecular_weight", "kind", 2_usize),
            ("molecular_weight", "kind", 3_usize),
            ("molecular_weight", "kind", 4_usize),
            ("specific_heat_ratio", "kind", 0_usize),
            ("specific_heat_ratio", "kind", 1_usize),
            ("specific_heat_ratio", "kind", 2_usize),
            ("specific_heat_ratio", "kind", 3_usize),
            ("specific_heat_ratio", "kind", 4_usize),
            ("gas_type", "alpha_kind", 0_usize),
            ("gas_type", "alpha_kind", 1_usize),
            ("gas_type", "alpha_kind", 2_usize),
            ("gas_type", "alpha_kind", 3_usize),
            ("gas_type", "alpha_kind", 4_usize),
            ("gap_vent_type", "alpha_kind", 0_usize),
            ("gap_vent_type", "alpha_kind", 1_usize),
            ("gap_vent_type", "alpha_kind", 2_usize),
            ("gap_vent_type", "alpha_kind", 3_usize),
            ("gap_vent_type", "alpha_kind", 4_usize),
            ("gas_type", "missing", 0_usize),
            ("thickness", "missing", 0_usize),
            ("gap_vent_type", "missing", 0_usize),
            ("thickness", "positive", 0_usize),
            ("thickness", "positive", 1_usize),
            ("thickness", "positive", 2_usize),
            ("viscosity_coefficient_a", "positive", 0_usize),
            ("viscosity_coefficient_a", "positive", 1_usize),
            ("viscosity_coefficient_a", "positive", 2_usize),
            ("specific_heat_coefficient_a", "positive", 0_usize),
            ("specific_heat_coefficient_a", "positive", 1_usize),
            ("specific_heat_coefficient_a", "positive", 2_usize),
            ("specific_heat_ratio", "ratio", 0_usize),
            ("specific_heat_ratio", "ratio", 1_usize),
            ("specific_heat_ratio", "ratio", 2_usize),
            ("molecular_weight", "weight", 0_usize),
            ("molecular_weight", "weight", 1_usize),
            ("thickness", "auto", 0_usize),
            ("thickness", "auto", 1_usize),
            ("thickness", "auto", 2_usize),
            ("conductivity_coefficient_a", "auto", 0_usize),
            ("conductivity_coefficient_a", "auto", 1_usize),
            ("conductivity_coefficient_a", "auto", 2_usize),
            ("conductivity_coefficient_b", "auto", 0_usize),
            ("conductivity_coefficient_b", "auto", 1_usize),
            ("conductivity_coefficient_b", "auto", 2_usize),
            ("conductivity_coefficient_c", "auto", 0_usize),
            ("conductivity_coefficient_c", "auto", 1_usize),
            ("conductivity_coefficient_c", "auto", 2_usize),
            ("viscosity_coefficient_a", "auto", 0_usize),
            ("viscosity_coefficient_a", "auto", 1_usize),
            ("viscosity_coefficient_a", "auto", 2_usize),
            ("viscosity_coefficient_b", "auto", 0_usize),
            ("viscosity_coefficient_b", "auto", 1_usize),
            ("viscosity_coefficient_b", "auto", 2_usize),
            ("viscosity_coefficient_c", "auto", 0_usize),
            ("viscosity_coefficient_c", "auto", 1_usize),
            ("viscosity_coefficient_c", "auto", 2_usize),
            ("specific_heat_coefficient_a", "auto", 0_usize),
            ("specific_heat_coefficient_a", "auto", 1_usize),
            ("specific_heat_coefficient_a", "auto", 2_usize),
            ("specific_heat_coefficient_b", "auto", 0_usize),
            ("specific_heat_coefficient_b", "auto", 1_usize),
            ("specific_heat_coefficient_b", "auto", 2_usize),
            ("specific_heat_coefficient_c", "auto", 0_usize),
            ("specific_heat_coefficient_c", "auto", 1_usize),
            ("specific_heat_coefficient_c", "auto", 2_usize),
            ("molecular_weight", "auto", 0_usize),
            ("molecular_weight", "auto", 1_usize),
            ("molecular_weight", "auto", 2_usize),
            ("specific_heat_ratio", "auto", 0_usize),
            ("specific_heat_ratio", "auto", 1_usize),
            ("specific_heat_ratio", "auto", 2_usize),
            ("gas_type", "enum", 0_usize),
            ("gas_type", "enum", 1_usize),
            ("gas_type", "enum", 2_usize),
            ("gas_type", "enum", 3_usize),
            ("gas_type", "enum", 4_usize),
            ("gap_vent_type", "enum", 0_usize),
            ("gap_vent_type", "enum", 1_usize),
            ("gap_vent_type", "enum", 2_usize),
            ("gap_vent_type", "enum", 3_usize),
            ("gap_vent_type", "enum", 4_usize),
            ("", "name", 0_usize),
            ("", "name", 1_usize),
            ("", "name", 2_usize),
            ("", "name", 3_usize),
            ("thickness", "wire", 0_usize),
            ("thickness", "wire", 1_usize),
            ("thickness", "wire", 2_usize),
            ("thickness", "wire", 3_usize),
        ] {
            let mut raw = full_raw.clone();
            let rows = raw
                .objects
                .get_mut(&ObjectType(GAP_EQL.into()))
                .expect("family");
            let name = if kind == "name" {
                match probe {
                    0 => "\u{b}",
                    1 => "\u{a0}",
                    2 => " odd Gap Eql ",
                    _ => "\t\u{b}\u{a0}",
                }
            } else {
                " FULL GAP EQL "
            };
            if kind == "name" {
                let row = rows
                    .remove(&ObjectName(" FULL GAP EQL ".into()))
                    .expect("row");
                rows.insert(ObjectName(name.into()), row);
            }
            let object = rows
                .get_mut(&ObjectName(name.into()))
                .expect("selected row");
            match kind {
                "kind" | "alpha_kind" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        match probe {
                            0 => RawValue::Null,
                            1 => RawValue::Bool(false),
                            2 => {
                                if kind == "kind" {
                                    RawValue::String("0.125".into())
                                } else {
                                    RawValue::Number("0.125".into())
                                }
                            }
                            3 => RawValue::Array(Vec::new()),
                            _ => RawValue::Object(Default::default()),
                        },
                    );
                }
                "missing" => {
                    object.fields.remove(&FieldName(field.into()));
                }
                "positive" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            match probe {
                                0 => "0",
                                1 => "-0.0",
                                _ => "-1",
                            }
                            .into(),
                        ),
                    );
                }
                "ratio" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            match probe {
                                0 => "1",
                                1 => "0",
                                _ => "-1",
                            }
                            .into(),
                        ),
                    );
                }
                "weight" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(if probe == 0 { "19.999" } else { "200.001" }.into()),
                    );
                }
                "auto" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::String(
                            match probe {
                                0 => "Autocalculate",
                                1 => "Autosize",
                                _ => "",
                            }
                            .into(),
                        ),
                    );
                }
                "enum" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::String(
                            if field == "gas_type" {
                                match probe {
                                    0 => "air",
                                    1 => "Air",
                                    2 => "",
                                    3 => " ",
                                    _ => "STEAM",
                                }
                            } else {
                                match probe {
                                    0 => "sealed",
                                    1 => "VENTEDINDOOR",
                                    2 => "",
                                    3 => " ",
                                    _ => "UnknownVent",
                                }
                            }
                            .into(),
                        ),
                    );
                }
                "wire" => {
                    object.fields.insert(
                        FieldName(field.into()),
                        RawValue::Number(
                            match probe {
                                0 => "18446744073709551615",
                                1 => "9223372036854775808",
                                2 => "18446744073709551616",
                                _ => "1.8446744073709552e19",
                            }
                            .into(),
                        ),
                    );
                }
                _ => {}
            }
            let source = super::super::compile_raw_model_for_source(&raw);
            if kind == "name" {
                let model = source.model.as_ref().expect("standalone SourceExact row");
                let expected = name.to_ascii_uppercase();
                assert!(
                    !source.has_errors()
                        && model.material_names.is_source_exact_namespace()
                        && model.materials.len() == 1
                        && model.materials[0].id == MaterialId(0)
                        && model.materials[0].name.0 == expected
                        && model.material_names.resolve(name) == Some(MaterialId(0))
                        && model.material_names.len() == 1
                        && model.material_names.names()
                            == [ep_model::NormalizedName(expected.clone())]
                        && model.ctf_report_names.materials == [(MaterialId(0), expected)],
                    "standalone GapEQL SourceExact {probe}"
                );
            } else if kind == "wire" {
                let model = source.model.as_ref().expect("original-positive thickness");
                let row = model.materials[0]
                    .as_window_gap_equivalent_layer()
                    .expect("advisory thickness");
                let original_expected = if probe == 1 {
                    9223372036854775808.0_f64
                } else {
                    18446744073709551616.0_f64
                };
                assert!(
                    !source.has_errors()
                        && model.material_names.is_source_exact_namespace()
                        && row.thickness_m.to_bits() == original_expected.to_bits()
                        && row.thickness_m > 0.0
                        && match (
                            probe,
                            ep_model::classify_source_json_number(match probe {
                                0 => "18446744073709551615",
                                1 => "9223372036854775808",
                                2 => "18446744073709551616",
                                _ => "1.8446744073709552e19",
                            })
                        ) {
                            (0, Ok(ep_model::SourceJsonNumber::Integer(-1)))
                            | (1, Ok(ep_model::SourceJsonNumber::Integer(i64::MIN))) => true,
                            (2 | 3, Ok(ep_model::SourceJsonNumber::Float(value))) =>
                                value.to_bits() == 18446744073709551616.0_f64.to_bits(),
                            _ => false,
                        },
                    "original/advisory/effective GapEQL thickness {probe}"
                );
            } else {
                assert!(
                    source
                        .model
                        .as_ref()
                        .is_none_or(|model| !model.material_names.is_source_exact_namespace()
                            && model.ctf_report_names.materials.is_empty())
                        && source.has_errors(),
                    "original GapEQL schema {kind}/{field}/{probe}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn equivalent_layer_custom_gap_preserves_blank_values_endpoints_and_k300()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Custom Gap": {
                    "gas_type":"CUSTOM",
                    "thickness":0.01,
                    "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":-0.001,
                    "conductivity_coefficient_b":0.0001,
                    "conductivity_coefficient_c":0.00000001,
                    "viscosity_coefficient_a":0.00001,
                    "viscosity_coefficient_b":0.00000002,
                    "viscosity_coefficient_c":-0.00000000001,
                    "specific_heat_coefficient_a":1000.0,
                    "specific_heat_coefficient_b":0.1,
                    "specific_heat_coefficient_c":-0.001,
                    "molecular_weight":20.0
                },
                "Weight 200": {
                    "gas_type":"CUSTOM",
                    "thickness":0.02,
                    "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00002,
                    "specific_heat_coefficient_a":900.0,
                    "molecular_weight":200.0,
                    "specific_heat_ratio":1.0001
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected custom equivalent-layer gaps"))?;
    let custom = model
        .materials
        .iter()
        .find(|material| material.name.0 == "CUSTOM GAP")
        .ok_or_else(|| std::io::Error::other("missing custom equivalent-layer gap"))?
        .as_window_gap_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("expected custom gap payload"))?;
    assert_eq!(custom.gas_type, WindowGasType::Custom);
    assert_eq!(custom.gap_vent_type, WindowGapVentType::Sealed);
    assert_eq!(custom.thickness_m, 0.01);
    assert_polynomial(
        custom.properties,
        [-0.001, 0.0001, 0.00000001],
        [0.00001, 0.00000002, -0.00000000001],
        [1000.0, 0.1, -0.001],
    );
    assert_eq!(custom.properties.molecular_weight_g_per_mol, 20.0);
    assert_eq!(
        custom.properties.specific_heat_ratio, 0.0,
        "EnergyPlus 26.1 retains numeric zero for a blank custom ratio"
    );
    assert_close(custom.conductivity_at_temperature_k(300.0), 0.0299);
    assert_close(
        custom
            .nominal_thermal_resistance_m2_k_per_w()
            .ok_or_else(|| std::io::Error::other("expected custom nominal resistance"))?,
        0.01 / 0.0299,
    );

    let upper_endpoint = model
        .materials
        .iter()
        .find(|material| material.name.0 == "WEIGHT 200")
        .ok_or_else(|| std::io::Error::other("missing upper-endpoint gap"))?
        .as_window_gap_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("expected upper-endpoint payload"))?;
    assert_eq!(upper_endpoint.properties.molecular_weight_g_per_mol, 200.0);
    assert_eq!(upper_endpoint.properties.specific_heat_ratio, 1.0001);

    Ok(())
}

#[test]
fn equivalent_layer_gap_enforces_schema_and_v261_source_validations() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Missing Required": {},
                "Bad Enum": {
                    "gas_type":"STEAM", "thickness":0.01, "gap_vent_type":"Sealed"
                },
                "Titlecase Enum": {
                    "gas_type":"Air", "thickness":0.01, "gap_vent_type":"Sealed"
                },
                "Lowercase Enum": {
                    "gas_type":"air", "thickness":0.01, "gap_vent_type":"Sealed"
                },
                "Blank Enum": {
                    "gas_type":"   ", "thickness":0.01, "gap_vent_type":"Sealed"
                },
                "Bad Vent": {
                    "gas_type":"AIR", "thickness":0.01, "gap_vent_type":"sealed"
                },
                "Blank Vent": {
                    "gas_type":"AIR", "thickness":0.01, "gap_vent_type":" "
                },
                "Bad Thickness": {
                    "gas_type":"AIR", "thickness":0.0, "gap_vent_type":"Sealed"
                },
                "Missing Custom": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.02
                },
                "Bad Custom Bounds": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.0,
                    "specific_heat_coefficient_a":0.0,
                    "molecular_weight":19.0,
                    "specific_heat_ratio":1.0
                },
                "Bad Conductivity": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":-0.01,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":44.0
                },
                "Bad Standard Extra": {
                    "gas_type":"AIR", "thickness":0.01, "gap_vent_type":"Sealed",
                    "viscosity_coefficient_a":0.0
                },
                "Bad Standard Type": {
                    "gas_type":"AIR", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_b":"not a number"
                },
                "Bad Upper Weight": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":201.0
                },
                "Zero Conductivity": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.0,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":44.0
                }
            }
        }"#,
    )
    .expect("invalid equivalent-layer gap cases should parse as raw epJSON");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in ["gas_type", "thickness", "gap_vent_type"] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "MissingRequiredField"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("Missing Required")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing required-field diagnostic for {field}"
        );
    }
    for object_name in ["Bad Enum", "Titlecase Enum", "Lowercase Enum"] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidEnumValue"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some(object_name)
                    && diagnostic.field.as_deref() == Some("gas_type")
            }),
            "missing strict uppercase gas-type diagnostic for {object_name}"
        );
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Blank Enum")
            && diagnostic.field.as_deref() == Some("gas_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Vent")
            && diagnostic.field.as_deref() == Some("gap_vent_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Blank Vent")
            && diagnostic.field.as_deref() == Some("gap_vent_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Thickness")
            && diagnostic.field.as_deref() == Some("thickness")
    }));
    for field in [
        "viscosity_coefficient_a",
        "specific_heat_coefficient_a",
        "molecular_weight",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "MissingCustomWindowGapEquivalentLayerProperty"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("Missing Custom")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing effective-required diagnostic for {field}"
        );
    }
    for field in [
        "viscosity_coefficient_a",
        "specific_heat_coefficient_a",
        "molecular_weight",
        "specific_heat_ratio",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("Bad Custom Bounds")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing custom-range diagnostic for {field}"
        );
    }
    for object_name in ["Bad Conductivity", "Zero Conductivity"] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidWindowGapEquivalentLayerConductivityAt300K"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some(object_name)
                    && diagnostic.field.as_deref() == Some("conductivity_coefficient_a")
            }),
            "missing non-positive k300 diagnostic for {object_name}"
        );
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Standard Extra")
            && diagnostic.field.as_deref() == Some("viscosity_coefficient_a")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidFieldType"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Standard Type")
            && diagnostic.field.as_deref() == Some("conductivity_coefficient_b")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Upper Weight")
            && diagnostic.field.as_deref() == Some("molecular_weight")
    }));

    let sub_runtime_minimum = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Thin Gap": {
                    "gas_type":"AIR",
                    "thickness":0.00001,
                    "gap_vent_type":"Sealed"
                }
            }
        }"#,
    )
    .expect("positive sub-runtime-minimum thickness should parse");
    let thin_result = compile_raw_model(&sub_runtime_minimum);
    assert!(
        !thin_result.has_errors(),
        "GetMaterialData accepts every positive thickness; BuildGap's later 0.0001 m replacement is outside this typed boundary: {:?}",
        thin_result.report.diagnostics
    );
}

#[test]
fn equivalent_layer_gap_follows_source_order_reports_coverage_and_shares_names()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Equivalent Gap": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            },
            "WindowMaterial:Gas": {
                "Ordinary Gas": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Glazing:EquivalentLayer": {
                "Equivalent Glass": {
                    "front_side_beam_beam_solar_transmittance":0.61,
                    "back_side_beam_beam_solar_transmittance":0.62,
                    "front_side_beam_beam_solar_reflectance":0.21,
                    "back_side_beam_beam_solar_reflectance":0.22
                }
            },
            "Material": {
                "Opaque": {
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":1000.0,
                    "specific_heat":1000.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected source-ordered materials"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "OPAQUE",
            "EQUIVALENT GLASS",
            "ORDINARY GAS",
            "EQUIVALENT GAP",
        ],
        "WindowMaterial:Gap:EquivalentLayer must be read after WindowMaterial:Gas"
    );
    let gap = &model.materials[3];
    assert_eq!(gap.id, MaterialId(3));
    assert_eq!(gap.kind(), MaterialKind::WindowGapEquivalentLayer);
    assert_eq!(gap.family(), MaterialFamily::EquivalentLayer);
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer gap coverage row"))?;
    assert_eq!(coverage.object_count, 1);
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

    let duplicate = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Shared": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Gap:EquivalentLayer": {
                "shared": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            }
        }"#,
    )?;
    let duplicate_result = compile_raw_model(&duplicate);
    assert!(
        duplicate_result
            .report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "DuplicateName"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("shared")
            })
    );

    let blank_name = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            }
        }"#,
    )?;
    let blank_result = compile_raw_model(&blank_name);
    assert!(blank_result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("")
            && diagnostic.field.as_deref() == Some("name")
    }));

    Ok(())
}

#[test]
fn equivalent_layer_gap_fails_closed_in_ordinary_construction() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Equivalent Gap": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            },
            "Construction": {
                "Wrong Window": {"outside_layer":"Equivalent Gap"}
            }
        }"#,
    )
    .expect("equivalent-layer gap construction boundary should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEquivalentLayerConstruction"
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some("Wrong Window")
            && diagnostic.field.as_deref() == Some("outside_layer")
    }));
}
