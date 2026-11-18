//! Ordinary opaque CheckAndSet properties at the actual GetConstructData1795 site.
//! Window optics and positive AirGap/IRT diagnostics remain unavailable.
use super::super::selected_material_data::SourceMaterialGroup as Group;
use super::{
    ConstructionMaterialOwners, HeatBalanceInputError as Error, SelectedConstructionUsage,
};
use crate::heat_balance::error_backend::SourceErrorBackend;

fn material_at<'a>(
    owners: &ConstructionMaterialOwners<'a>,
    number: i32,
) -> Result<&'a ep_model::Material, Error> {
    let index = number
        .checked_sub(1)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Error::Unavailable("CheckAndSet MaterialNum bounds"))?;
    let id = owners.data.materials.get(index).ok_or(Error::Unavailable(
        "CheckAndSet actual material-order bounds",
    ))?;
    let mut matches = owners.model.materials.iter().filter(|row| row.id == *id);
    let material = matches
        .next()
        .ok_or(Error::Unavailable("CheckAndSet borrowed material identity"))?;
    if matches.next().is_some() {
        return Err(Error::Unavailable("CheckAndSet unique material identity"));
    }
    Ok(material)
}

impl SelectedConstructionUsage {
    pub(super) fn write_selected_opaque_properties(
        &mut self,
        materials: &ConstructionMaterialOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        // Original357/361 return before DayltPropPtr or any property write.
        if self.total_layers == 0 {
            return Ok(());
        }
        let count = usize::try_from(self.total_layers)
            .map_err(|_| Error::Unavailable("Construction TotLayers bounds"))?;
        let inside = *self
            .layer_points
            .get(count - 1)
            .ok_or(Error::Unavailable("Construction InsideLayer bounds"))?;
        if inside <= 0 {
            return Ok(());
        }
        self.daylt_prop_ptr = 0; // Original367, before the inside material read.
        let mat_inside = material_at(materials, inside)?;
        let roof_inside = if mat_inside.kind() == ep_model::MaterialKind::RoofVegetation {
            Some(
                materials
                    .data
                    .source_roof_vegetation_material(inside, mat_inside.id)?,
            )
        } else {
            None
        };
        self.inside_absorp_vis = if let Some(row) = roof_inside {
            row.visible_absorptance
        } else {
            mat_inside
                .visible_absorptance()
                .ok_or(Error::Unavailable("CheckAndSet inside visible property"))?
        };
        self.inside_absorp_solar = if let Some(row) = roof_inside {
            row.solar_absorptance
        } else {
            mat_inside
                .solar_absorptance()
                .ok_or(Error::Unavailable("CheckAndSet inside solar property"))?
        };
        self.reflect_vis_diff_back = 1.0 - self.inside_absorp_vis;
        let outside = self.layer_points[0];
        if outside != 0 {
            let mat = material_at(materials, outside)?;
            let roof = if mat.kind() == ep_model::MaterialKind::RoofVegetation {
                Some(
                    materials
                        .data
                        .source_roof_vegetation_material(outside, mat.id)?,
                )
            } else {
                None
            };
            self.outside_absorp_vis = if let Some(row) = roof {
                row.visible_absorptance
            } else {
                mat.visible_absorptance()
                    .ok_or(Error::Unavailable("CheckAndSet outside visible property"))?
            };
            self.outside_absorp_solar = if let Some(row) = roof {
                row.solar_absorptance
            } else {
                mat.solar_absorptance()
                    .ok_or(Error::Unavailable("CheckAndSet outside solar property"))?
            };
        }
        self.total_solid_layers = 0;
        self.total_glass_layers = 0;
        self.abs_diff_shade = 0.0;
        // Original392 resets Window only. Each nonzero layer ASSIGNS the
        // predicate at399-405; do not turn this into an accumulating OR.
        self.type_is_window = false;
        for point in &self.layer_points[..count] {
            if *point == 0 {
                continue;
            }
            let group = materials.data.source_group_at(*point)?;
            self.type_is_window = group.is_window();
            if !self.type_is_window && !group.is_not_window() {
                return Err(Error::Unavailable(
                    "native unclassified material group assertion",
                ));
            }
        }
        if inside == 0 {
            return Ok(());
        }
        if outside == 0 {
            return Ok(());
        }
        if self.type_is_window {
            return Err(Error::Unavailable(
                "positive CheckAndSet window optical branch",
            ));
        }
        let mat_outside = material_at(materials, outside)?;
        let roof_outside = if mat_outside.kind() == ep_model::MaterialKind::RoofVegetation {
            Some(
                materials
                    .data
                    .source_roof_vegetation_material(outside, mat_outside.id)?,
            )
        } else {
            None
        };
        self.inside_absorp_thermal = if let Some(row) = roof_inside {
            row.thermal_absorptance
        } else {
            mat_inside
                .thermal_absorptance()
                .ok_or(Error::Unavailable("CheckAndSet inside thermal property"))?
        };
        self.outside_absorp_thermal = if let Some(row) = roof_outside {
            row.thermal_absorptance
        } else {
            mat_outside
                .thermal_absorptance()
                .ok_or(Error::Unavailable("CheckAndSet outside thermal property"))?
        };
        self.outside_roughness = if let Some(row) = roof_outside {
            Some(
                row.roughness
                    .ok_or(Error::Unavailable("CheckAndSet outside roughness property"))?,
            )
        } else {
            match mat_outside.roughness() {
                Some(value) => Some(value),
                None if mat_outside.kind() == ep_model::MaterialKind::InfraredTransparent => None,
                None => return Err(Error::Unavailable("CheckAndSet outside roughness property")),
            }
        };
        // Original760+ follows all opaque scalar/roughness assignments.
        let outside_group = materials.data.source_group_at(outside)?;
        if outside_group == Group::AirGap
            || materials.data.source_group_at(inside)? == Group::AirGap
        {
            return Err(Error::Unavailable(
                "positive CheckAndSet AirGap diagnostics",
            ));
        }
        // EcoRoof/IRT are conditional true writes, not false resets.
        if outside_group == Group::EcoRoof {
            self.type_is_eco_roof = true;
            for &point in &self.layer_points[1..count] {
                if materials.data.source_group_at(point)? == Group::EcoRoof {
                    backend.show_severe_error(&format!(
                        "CheckAndSetConstructionProperties: Interior Layer is EcoRoof for construction {}",
                        self.name,
                    ))?;
                    let name = materials.data.source_material_name(point)?;
                    backend.show_continue_error(&format!("  Error in material {name}"))?;
                    *errors_found = true;
                }
            }
        }
        if outside_group == Group::IrTransparent {
            self.type_is_irt = true;
            if self.total_layers != 1 {
                return Err(Error::Unavailable(
                    "positive CheckAndSet IRT layer diagnostics",
                ));
            }
        }
        Ok(())
    }
}
