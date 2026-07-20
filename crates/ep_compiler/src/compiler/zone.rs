use ep_model::{
    AutoOrNumber, InsideSurfaceConvectionAlgorithm, NormalizedName,
    OutsideSurfaceConvectionAlgorithm, Point3, TypedModel, Zone, ZoneConvectionAlgorithm, ZoneId,
};

use super::{
    Compiler, parse_inside_surface_convection_algorithm,
    parse_outside_surface_convection_algorithm, parse_yes_no,
};

const OBJECT_TYPE: &str = "Zone";

impl Compiler<'_> {
    pub(super) fn parse_zones(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects(OBJECT_TYPE) {
            let diagnostics_before_fields = self.diagnostics.len();
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    "Zone requires a non-blank name".to_string(),
                );
            }

            let direction_of_relative_north_deg = self.number_default(
                OBJECT_TYPE,
                &name,
                &object,
                "direction_of_relative_north",
                0.0,
            );
            let origin = Point3 {
                x_m: self.number_default(OBJECT_TYPE, &name, &object, "x_origin", 0.0),
                y_m: self.number_default(OBJECT_TYPE, &name, &object, "y_origin", 0.0),
                z_m: self.number_default(OBJECT_TYPE, &name, &object, "z_origin", 0.0),
            };

            let zone_type = self.u32_default(OBJECT_TYPE, &name, &object, "type", 1);
            if zone_type != 1 {
                self.error(
                    "InvalidNumericRange",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("type"),
                    format!("Zone/{name} field type must equal 1, got {zone_type}"),
                );
            }

            let multiplier = self.u32_default(OBJECT_TYPE, &name, &object, "multiplier", 1);
            if multiplier == 0 || multiplier > i32::MAX as u32 {
                self.error(
                    "InvalidNumericRange",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("multiplier"),
                    format!(
                        "Zone/{name} field multiplier must be between 1 and {}, got {multiplier}",
                        i32::MAX
                    ),
                );
            }

            let ceiling_height = self.auto_default(
                OBJECT_TYPE,
                &name,
                &object,
                "ceiling_height",
                AutoOrNumber::AutoCalculate,
                "Autocalculate",
            );
            let volume = self.auto_default(
                OBJECT_TYPE,
                &name,
                &object,
                "volume",
                AutoOrNumber::AutoCalculate,
                "Autocalculate",
            );
            let floor_area = self.auto_default(
                OBJECT_TYPE,
                &name,
                &object,
                "floor_area",
                AutoOrNumber::AutoCalculate,
                "Autocalculate",
            );

            let inside_override = self.optional_enum(
                OBJECT_TYPE,
                &name,
                &object,
                "zone_inside_convection_algorithm",
                parse_zone_inside_surface_convection_algorithm,
            );
            let outside_override = self.optional_enum(
                OBJECT_TYPE,
                &name,
                &object,
                "zone_outside_convection_algorithm",
                parse_outside_surface_convection_algorithm,
            );
            let is_part_of_total_floor_area = self.enum_default(
                OBJECT_TYPE,
                &name,
                (&object, "part_of_total_floor_area"),
                true,
                "Yes",
                parse_yes_no,
            );

            if self.diagnostics.len() != diagnostics_before_fields {
                continue;
            }
            if model.zone_names.resolve(&name).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, model.zones.len()) else {
                continue;
            };
            let id = ZoneId(id_value);
            let inside_convection_algorithm = match inside_override {
                Some(algorithm) => ZoneConvectionAlgorithm::Override(algorithm),
                None => ZoneConvectionAlgorithm::Inherited(
                    model
                        .surface_convection_algorithms
                        .inside
                        .unwrap_or(InsideSurfaceConvectionAlgorithm::Tarp),
                ),
            };
            let outside_convection_algorithm = match outside_override {
                Some(algorithm) => ZoneConvectionAlgorithm::Override(algorithm),
                None => ZoneConvectionAlgorithm::Inherited(
                    model
                        .surface_convection_algorithms
                        .outside
                        .unwrap_or(OutsideSurfaceConvectionAlgorithm::Doe2),
                ),
            };

            let existing = model.zone_names.insert(&name, id);
            debug_assert!(existing.is_none());
            model.zones.push(Zone {
                id,
                name: NormalizedName::new(&name),
                direction_of_relative_north_deg,
                origin,
                zone_type: 1,
                multiplier,
                ceiling_height,
                volume,
                floor_area,
                inside_convection_algorithm,
                outside_convection_algorithm,
                is_part_of_total_floor_area,
            });
        }
    }
}

fn parse_zone_inside_surface_convection_algorithm(
    value: &str,
) -> Option<InsideSurfaceConvectionAlgorithm> {
    if value.eq_ignore_ascii_case("TrombeWall") {
        Some(InsideSurfaceConvectionAlgorithm::TrombeWall)
    } else {
        parse_inside_surface_convection_algorithm(value)
    }
}
