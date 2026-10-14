// Optional Native BEFORE kernel replay inside the existing test body.
// Native identities below validate captured input context only. No independently
// produced Rust World row/association/endpoint join is performed: that is UNKNOWN.
// Captured BEFORE operands enter test calls only, never production fixture state.
fn check_setup_process_native_reference_if_requested() -> Result<(), Box<dyn std::error::Error>> {
    let Some(path) = std::env::var_os("SRC06_SETUP_PROCESS_REFERENCE") else {
        return Ok(());
    };
    use crate::geometry::{
        source_process_coplanarity, source_process_distance, source_process_is_rectangle,
        source_process_shift, source_process_sky_rad_split, source_process_translation,
    };
    use ep_model::Point3;
    use serde_json::Value;
    fn bits(value: &Value) -> Result<u64, Box<dyn std::error::Error>> {
        let text = value.as_str().ok_or("Native binary64 hex string")?;
        if text.len() != 16 || !text.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err("Native exact16 hex bits".into());
        }
        let result = u64::from_str_radix(text, 16)?;
        if !f64::from_bits(result).is_finite() {
            return Err("Native nonfinite operand is not a numerical PASS".into());
        }
        Ok(result)
    }
    fn number(value: &Value) -> Result<f64, Box<dyn std::error::Error>> {
        Ok(f64::from_bits(bits(&value["value_bits"])?))
    }
    fn vector(value: &Value) -> Result<[f64; 3], Box<dyn std::error::Error>> {
        let row = value.as_array().ok_or("Native scalar vector")?;
        if row.len() != 3 {
            return Err("Native three-vector length".into());
        }
        Ok([number(&row[0])?, number(&row[1])?, number(&row[2])?])
    }
    fn same(
        actual: f64,
        expected: &Value,
        count: &mut usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !actual.is_finite() {
            return Err("Rust nonfinite point result is not a numerical PASS".into());
        }
        assert_eq!(actual.to_bits(), bits(&expected["value_bits"])?);
        *count += 1;
        Ok(())
    }
    let metadata = std::fs::metadata(&path)?;
    if metadata.len() > 2 * 1024 * 1024 {
        return Err("Native finite point fixture exceeds2MiB".into());
    }
    let native: Value = serde_json::from_slice(&std::fs::read(path)?)?;
    assert_eq!(
        native["schema"],
        "ordinary-setup-process-same-input-results.v1"
    );
    assert_eq!(native["actual_helper_exit_code"], 0);
    assert_eq!(native["actual_loader_ready"], true);
    assert_eq!(native["read_only_observer_integrity_failed"], false);
    let reference = &native["setup_process_reference"];
    assert_eq!(
        reference["schema"],
        "src06-setup-process-same-input-Native-points.v1"
    );
    assert_eq!(reference["complete"], true);
    assert_eq!(reference["production_Rust_or_Native_AFTER_seeded"], false);
    let cases = reference["cases"]
        .as_array()
        .ok_or("Native completed actual cases")?;
    assert_eq!(cases.len(), 6);
    let mut names = std::collections::BTreeSet::new();
    let mut source_ids = std::collections::BTreeSet::new();
    let mut scalar_comparisons = 0;
    let mut last_points = Vec::new();
    let mut exterior_ids = Vec::new();
    for (native_index, row) in cases.iter().enumerate() {
        assert_eq!(row["complete"], true);
        assert_eq!(row["actual_inputs_unchanged_after_calls"], true);
        let before = &row["before"];
        let name = before["name"]
            .as_str()
            .ok_or("actual Native surface name")?;
        assert!(!name.is_empty() && names.insert(name));
        let source_id = before["surface_id"]
            .as_i64()
            .ok_or("actual Native Source ID")?;
        assert!(source_ids.insert(source_id));
        assert_eq!(source_id, i64::try_from(native_index + 1)?);
        // Actual Native SurfaceClass Wall1/Floor2/Roof3, not Rust enum indexes.
        assert!(matches!(before["class"].as_i64(), Some(1..=3)));
        assert_eq!(before["zone_id"], 1);
        assert_eq!(before["space_id"], 1);
        assert!(before["construction_id"].as_i64().is_some_and(|id| id > 0));
        assert!(before["ext_solar"].as_bool().is_some());
        number(&before["gross_area_m2"])?;
        number(&before["cos_tilt"])?;
        vector(&before["lcsx"])?;
        vector(&before["lcsy"])?;
        vector(&before["lcsz"])?;
        assert_eq!(before["base_surface"].as_i64(), Some(source_id));
        assert_eq!(before["heat_transfer_surface"], true);
        assert_eq!(before["mirrored_surface"], false);
        assert_eq!(before["is_shadowing"], false);
        let vertices = before["world_vertex_bits"]
            .as_array()
            .ok_or("actual Native BEFORE vertices")?;
        assert_eq!(
            before["sides"].as_u64(),
            Some(u64::try_from(vertices.len())?)
        );
        let points = vertices
            .iter()
            .map(|value| {
                let v = value.as_array().ok_or("actual Native vertex triple")?;
                if v.len() != 3 {
                    return Err("actual Native vertex triple length".into());
                }
                Ok(Point3 {
                    x_m: f64::from_bits(bits(&v[0])?),
                    y_m: f64::from_bits(bits(&v[1])?),
                    z_m: f64::from_bits(bits(&v[2])?),
                })
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
        // Frozen A policy contains six actual rectangular four-sided rows;
        // triangle/polygon/nonplanar/fatal variants are not promoted to paired.
        assert_eq!(points.len(), 4);
        let direct = &row["direct_Original"];
        let planar = source_process_coplanarity(&points);
        assert_eq!(
            Some(planar.plane_error),
            direct["plane_equation"]["error"].as_bool()
        );
        let coefficients = direct["plane_equation"]["coefficients"]
            .as_array()
            .ok_or("actual Original plane coefficients")?;
        assert_eq!(coefficients.len(), 4);
        for (value, expected) in planar.plane.into_iter().zip(coefficients) {
            same(value, expected, &mut scalar_comparisons)?;
        }
        assert_eq!(
            Some(planar.is_coplanar),
            direct["coplanarity"]["is_coplanar"].as_bool()
        );
        assert_eq!(
            Some(u64::try_from(planar.error_vertex)?),
            direct["coplanarity"]["error_vertex"].as_u64()
        );
        same(
            planar.max_distance,
            &direct["coplanarity"]["max_distance"],
            &mut scalar_comparisons,
        )?;
        let rectangle = source_process_is_rectangle(&[points[0], points[1], points[2], points[3]]);
        assert_eq!(Some(rectangle), direct["is_rectangle"].as_bool());
        assert!(rectangle);
        let translation = source_process_translation(&points)
            .ok_or("actual valid Native translation must be available")?;
        let original = direct["coordinate_translation"]
            .as_array()
            .ok_or("actual Original translation")?;
        assert_eq!(original.len(), 3);
        for (value, expected) in translation.into_iter().zip(original) {
            same(value, expected, &mut scalar_comparisons)?;
        }
        let stored = &row["stored_Process_return"];
        assert_eq!(stored["vertices_processed"], true);
        assert_eq!(
            stored["shape"],
            crate::geometry::SourceProcessShape::Rectangle as i32
        );
        same(
            source_process_distance(points[2], points[1]),
            &stored["width_m"],
            &mut scalar_comparisons,
        )?;
        same(
            source_process_distance(points[1], points[0]),
            &stored["height_m"],
            &mut scalar_comparisons,
        )?;
        same(
            source_process_shift(vector(&before["lcsx"])?, points[1], translation),
            &stored["x_shift_m"],
            &mut scalar_comparisons,
        )?;
        same(
            source_process_shift(vector(&before["lcsy"])?, points[1], translation),
            &stored["y_shift_m"],
            &mut scalar_comparisons,
        )?;
        let stored_translation = stored["coordinate_translation"]
            .as_array()
            .ok_or("actual stored X0/Y0/Z0")?;
        assert_eq!(stored_translation.len(), 3);
        for (value, expected) in translation.into_iter().zip(stored_translation) {
            same(value, expected, &mut scalar_comparisons)?;
        }
        // This literal scalar operation is the actual caller326 writer, not a
        // fabricated Native point routine or expected value.
        same(
            source_process_sky_rad_split(number(&before["cos_tilt"])?),
            &stored["surf_air_sky_rad_split"],
            &mut scalar_comparisons,
        )?;
        assert_eq!(
            stored["shade_vertices"]["n_vert"].as_u64(),
            Some(u64::try_from(points.len())?)
        );
        for (component, key) in [(0, "xv"), (1, "yv"), (2, "zv")] {
            let array = &stored["shade_vertices"][key];
            assert_eq!(array["allocated"], true);
            assert_eq!(array["lower"], 1);
            assert_eq!(array["size"].as_u64(), Some(u64::try_from(points.len())?));
            assert_eq!(array["upper"].as_u64(), Some(u64::try_from(points.len())?));
            let values = array["values"]
                .as_array()
                .ok_or("actual Native ShadeV values")?;
            assert_eq!(values.len(), points.len());
            for (point, expected) in points.iter().zip(values) {
                same(
                    [point.x_m, point.y_m, point.z_m][component],
                    expected,
                    &mut scalar_comparisons,
                )?;
            }
        }
        if before["ext_solar"] == true {
            exterior_ids.push(source_id);
        }
        last_points = points;
    }
    let global = &reference["global_Process_return"];
    assert_eq!(global["process_one_time_flag"], false);
    assert_eq!(global["max_vertices_per_surface"], 4);
    assert_eq!(
        global["all_ext_solar_and_shading"],
        serde_json::to_value(exterior_ids)?
    );
    assert_eq!(global["calc_window_reveal_reflection"], false);
    for key in [
        "building_shading_count",
        "fixed_shading_count",
        "attached_shading_count",
        "shading_surface_first",
    ] {
        assert_eq!(global[key], 0);
    }
    assert_eq!(global["shading_surface_last"], -1);
    for (component, key) in [(0, "xpsv"), (1, "ypsv"), (2, "zpsv")] {
        let array = &global[key];
        assert_eq!(array["allocated"], true);
        assert_eq!(array["lower"], 1);
        assert_eq!(array["upper"], 4);
        assert_eq!(array["size"], 4);
        let values = array["values"]
            .as_array()
            .ok_or("actual persistent final Process scratch")?;
        assert_eq!(values.len(), last_points.len());
        for (point, expected) in last_points.iter().zip(values) {
            same(
                [point.x_m, point.y_m, point.z_m][component],
                expected,
                &mut scalar_comparisons,
            )?;
        }
    }
    eprintln!(
        "SRC06_SETUP_PROCESS_REFERENCE: {} Native BEFORE kernel replay cases, {scalar_comparisons} exact finite scalar comparisons; Rust World producer join UNKNOWN",
        cases.len()
    );
    Ok(())
}
