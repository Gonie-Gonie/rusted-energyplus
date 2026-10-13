//! Actual GetGeometryParameters local argument/default/diagnostic/EIO order.
use super::{
    Error, SelectedHeatBalanceInput, SourceErrorBackend, SurfaceGeometryInputState,
    SurfaceGetterOwners,
};
use std::io::Write;

impl SurfaceGeometryInputState {
    pub(super) fn get_geometry_parameters(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors: &mut bool,
    ) -> Result<(), Error> {
        owners.shortcut.current_module_object = "GlobalGeometryRules".into();
        let count = owners.input.get_num_objects_found("GlobalGeometryRules");
        let mut message = String::from(" Surface Geometry,");
        let mut values: [String; 5] = std::array::from_fn(|_| String::new());
        if count == 1 {
            (values, _) = raw.geometry_arguments(owners.input, owners.shortcut)?;
            self.corner = 1;
            self.world_coordinates = true;
            self.counterclockwise = true;
            let corners = [
                "UpperLeftCorner",
                "LowerLeftCorner",
                "LowerRightCorner",
                "UpperRightCorner",
            ];
            if let Some(index) = corners
                .iter()
                .position(|corner| corner.eq_ignore_ascii_case(&values[0]))
            {
                self.corner = i32::try_from(index + 1)
                    .map_err(|_| Error::Unavailable("geometry corner index"))?;
                message.push_str(corners[index]);
                message.push(',');
            } else {
                invalid(
                    backend,
                    &owners.shortcut.alpha_field_names[0],
                    &values[0],
                    true,
                )?;
                *errors = true;
            }
            match values[1].as_str() {
                "CCW" | "COUNTERCLOCKWISE" => {
                    self.counterclockwise = true;
                    message.push_str("Counterclockwise,");
                }
                "CW" | "CLOCKWISE" => {
                    self.counterclockwise = false;
                    message.push_str("Clockwise,");
                }
                _ => {
                    invalid(
                        backend,
                        &owners.shortcut.alpha_field_names[1],
                        &values[1],
                        true,
                    )?;
                    *errors = true;
                }
            }
            match values[2].as_str() {
                "WORLD" | "ABSOLUTE" => {
                    self.world_coordinates = true;
                    message.push_str("WorldCoordinateSystem,");
                }
                "RELATIVE" => {
                    self.world_coordinates = false;
                    message.push_str("RelativeCoordinateSystem,");
                }
                _ => {
                    invalid(
                        backend,
                        &owners.shortcut.alpha_field_names[2],
                        &values[2],
                        false,
                    )?;
                    backend.show_continue_error(&format!(
                        "{} defaults to \"WorldCoordinateSystem\"",
                        owners.shortcut.alpha_field_names[2]
                    ))?;
                    self.world_coordinates = true;
                    message.push_str("WorldCoordinateSystem,");
                }
            }
            match values[3].as_str() {
                "WORLD" | "ABSOLUTE" => {
                    self.daylight_world_coordinates = true;
                    message.push_str("WorldCoordinateSystem,");
                }
                "RELATIVE" | "" => {
                    self.daylight_world_coordinates = false;
                    message.push_str("RelativeCoordinateSystem,");
                }
                _ => {
                    invalid(
                        backend,
                        &owners.shortcut.alpha_field_names[3],
                        &values[3],
                        false,
                    )?;
                    backend.show_continue_error(&format!(
                        "{} defaults to \"RelativeToZoneOrigin\"",
                        owners.shortcut.alpha_field_names[3]
                    ))?;
                    self.daylight_world_coordinates = false;
                    message.push_str("RelativeToZoneOrigin,");
                }
            }
            match values[4].as_str() {
                "WORLD" | "ABSOLUTE" => {
                    self.rectangle_world_coordinates = true;
                    message.push_str("WorldCoordinateSystem");
                }
                "RELATIVE" | "" => {
                    self.rectangle_world_coordinates = false;
                    message.push_str("RelativeToZoneOrigin");
                }
                _ => {
                    invalid(
                        backend,
                        &owners.shortcut.alpha_field_names[4],
                        &values[4],
                        false,
                    )?;
                    backend.show_continue_error(&format!(
                        "{} defaults to \"RelativeToZoneOrigin\"",
                        owners.shortcut.alpha_field_names[4]
                    ))?;
                    self.rectangle_world_coordinates = false;
                    message.push_str("RelativeToZoneOrigin");
                }
            }
        } else if count == 0 {
            backend.show_severe_error("GlobalGeometryRules: Required object not found.")?;
            message.push_str("None found in input");
            *errors = true;
        } else {
            backend.show_severe_error(
                "GlobalGeometryRules: Too many objects entered.  Only one allowed.",
            )?;
            *errors = true;
        }
        let names = &owners.shortcut.alpha_field_names;
        if !self.world_coordinates {
            if self.daylight_world_coordinates {
                mismatch(backend, names, &values, 3)?;
            }
            if self.rectangle_world_coordinates {
                mismatch(backend, names, &values, 4)?;
            }
        } else {
            let zones = owners
                .zones
                .zones
                .as_ref()
                .ok_or(Error::Unavailable("actual Zone allocation before geometry"))?;
            if zones
                .iter()
                .any(|zone| zone.origin_m.iter().any(|value| *value != 0.0))
                && !self.rectangle_world_coordinates
            {
                backend.show_warning_error("GlobalGeometryRules: Potential mismatch of coordinate specifications. Note that the rectangular surfaces are relying on the default SurfaceGeometry for 'Relative to zone' coordinate.")?;
                backend.show_continue_error(&format!("{}=\"{}\"; while ", names[2], values[2]))?;
                if values[4] == "RELATIVE" {
                    backend.show_continue_error(&format!("{}=\"{}\".", names[4], values[4]))?;
                } else if values[4] != "ABSOLUTE" {
                    backend
                        .show_continue_error(&format!("{}=\"defaults to RELATIVE\".", names[4]))?;
                }
            }
        }
        writeln!(
            eio,
            "! <Surface Geometry>,Starting Corner,Vertex Input Direction,Coordinate System,Daylight Reference Point Coordinate System,Rectangular (Simple) Surface Coordinate System"
        )?;
        writeln!(eio, "{message}")?;
        Ok(())
    }
}
fn invalid(
    backend: &mut SourceErrorBackend<'_>,
    field: &str,
    value: &str,
    severe: bool,
) -> Result<(), Error> {
    let text = format!("GlobalGeometryRules: Invalid {field}={value}");
    if severe {
        backend.show_severe_error(&text)?;
    } else {
        backend.show_warning_error(&text)?;
    }
    Ok(())
}
fn mismatch(
    backend: &mut SourceErrorBackend<'_>,
    names: &[String],
    values: &[String; 5],
    other: usize,
) -> Result<(), Error> {
    backend.show_warning_error(
        "GlobalGeometryRules: Potential mismatch of coordinate specifications.",
    )?;
    backend.show_continue_error(&format!("{}=\"{}\"; while ", names[2], values[2]))?;
    backend.show_continue_error(&format!("{}=\"{}\".", names[other], values[other]))?;
    Ok(())
}
