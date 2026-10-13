//! GetHTSurfaceData BuildingSurface arguments through the real GetVertices call.
//! Each caller must finish that row's actual vertex producer before the next get.
use super::super::super::ctf_initialization_context::original_loader_name;
use super::{
    Error, SelectedHeatBalanceInput, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState,
    SurfaceGetterOwners, SurfaceRowPrefix,
};

fn invalid(
    backend: &mut SourceErrorBackend<'_>,
    name: &str,
    field: &str,
    value: &str,
) -> Result<(), Error> {
    backend.show_severe_error(&format!(
        "BuildingSurface:Detailed=\"{name}\", invalid {field}=\"{value}\"."
    ))?;
    Ok(())
}
impl SurfaceGeometryInputState {
    pub(crate) fn read_opaque_row(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        index: usize,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<SurfaceRowPrefix, Error> {
        if !self.entered_once || self.pending_vertices.is_some() || index != self.next_object {
            return Err(Error::Unavailable(
                "actual sequential per-surface vertex-return boundary",
            ));
        }
        let (_, numeric_count) = raw.surface_arguments(index, owners.input, owners.shortcut)?;
        self.next_object += 1;
        let args = &mut owners.shortcut.alpha_args;
        let fields = &owners.shortcut.alpha_field_names;
        let name = args[0].clone();
        // GlobalNames91-113: actual error branch skips before SurfNum increment.
        if name.is_empty() {
            backend.show_severe_error(&format!(
                "E+ object type {name} cannot have blank {} field",
                fields[0]
            ))?;
            *errors = true;
            return Ok(SurfaceRowPrefix::Continue);
        }
        if let Some(previous) = self.unique_names.get(&name) {
            backend.show_severe_error(&format!("{name} with object type BuildingSurface:Detailed duplicates a name in object type {previous}"))?;
            *errors = true;
            return Ok(SurfaceRowPrefix::Continue);
        }
        self.unique_names
            .insert(name.clone(), "BuildingSurface:Detailed".into());
        let slot = self.surfaces_read;
        self.surfaces_read += 1;
        let row = self
            .surface_tmp
            .as_mut()
            .and_then(|rows| rows.get_mut(slot))
            .ok_or(Error::Unavailable("actual SurfaceTmp allocation"))?;
        row.name = name.clone();
        if args[1] == "CEILING" {
            args[1] = "ROOF".into();
        }
        row.class = match args[1].as_str() {
            "WALL" => SurfaceClass::Wall,
            "FLOOR" => SurfaceClass::Floor,
            "ROOF" => SurfaceClass::Roof,
            _ => {
                // Source3930 lacks a closing quote/period in this diagnostic.
                backend.show_severe_error(&format!(
                    "BuildingSurface:Detailed=\"{name}\", invalid {}=\"{}",
                    fields[1], args[1]
                ))?;
                *errors = true;
                SurfaceClass::None
            }
        };
        let mut found = None;
        for (ordinal, id) in owners.control.construction_order.iter().enumerate() {
            let construction = owners
                .model
                .constructions
                .iter()
                .find(|construction| construction.id == *id)
                .ok_or(Error::Unavailable("actual construction ordered identity"))?;
            let actual_name = original_loader_name(
                &owners.model.ctf_report_names.constructions,
                *id,
                &construction.name,
            )
            .map_err(|_| Error::Unavailable("actual construction loader name"))?;
            if actual_name.eq_ignore_ascii_case(&args[2]) {
                found = Some((ordinal, *id, construction));
                break;
            }
        }
        if let Some((ordinal, id, construction)) = found {
            row.construction = super::signed(ordinal + 1)?;
            if !construction.is_ordinary_opaque() {
                // Current admission excludes windows/other positive semantics.
                return Err(Error::Unavailable(
                    "positive non-opaque construction at surface lookup",
                ));
            }
            owners.control.mark_opaque_surface_construction_used(id)?;
            row.construction_stored_input = row.construction;
        } else {
            *errors = true;
            invalid(backend, &name, &fields[2], &args[2])?;
        }
        row.heat_transfer_surface = true;
        row.base_surface = super::signed(slot + 1)?;
        row.base_surface_name = name.clone();
        row.zone_name = args[3].clone();
        let zones = owners
            .zones
            .zones
            .as_ref()
            .ok_or(Error::Unavailable("actual allocated Zone owner"))?;
        if let Some(ordinal) = zones
            .iter()
            .position(|zone| zone.name.eq_ignore_ascii_case(&row.zone_name))
        {
            row.zone = super::signed(ordinal + 1)?;
        } else {
            invalid(backend, &name, &fields[3], &args[3])?;
            row.class = SurfaceClass::Invalid;
            row.zone_name = "Unknown Zone".into();
            *errors = true;
        }
        if !owners.shortcut.alpha_field_blanks[4] {
            let spaces = owners
                .zones
                .spaces
                .as_ref()
                .ok_or(Error::Unavailable("actual allocated Space owner"))?;
            if let Some(ordinal) = spaces
                .iter()
                .position(|space| space.name.eq_ignore_ascii_case(&args[4]))
            {
                row.space = super::signed(ordinal + 1)?;
                if row.zone != spaces[ordinal].zone_num {
                    backend.show_severe_error(&format!("BuildingSurface:Detailed=\"{name}\", invalid {}=\"{}\" is not in the same zone as the surface.", fields[4], args[4]))?;
                    row.class = SurfaceClass::Invalid;
                    *errors = true;
                }
            } else {
                backend.show_severe_error(&format!(
                    "BuildingSurface:Detailed=\"{name}\", invalid {}=\"{}\" not found.",
                    fields[4], args[4]
                ))?;
                row.class = SurfaceClass::Invalid;
                *errors = true;
            }
        }
        row.exterior_boundary_name = args[6].clone();
        match args[5].as_str() {
            "OUTDOORS" => row.exterior_boundary = 0,
            "ADIABATIC" => {
                row.exterior_boundary = -999;
                row.exterior_boundary_name = name.clone();
            }
            _ => {
                return Err(Error::Unavailable(
                    "positive unported opaque boundary condition",
                ));
            }
        }
        match args[7].as_str() {
            "SUNEXPOSED" => {
                if row.exterior_boundary != 0 {
                    backend.show_warning_error(&format!(
                        "BuildingSurface:Detailed=\"{name}\", {}=\"{}\".",
                        fields[7], args[7]
                    ))?;
                    backend.show_continue_error("..This surface is not exposed to External Environment.  Sun exposure has no effect.")?;
                } else {
                    row.exterior_solar = true;
                }
            }
            "NOSUN" => row.exterior_solar = false,
            _ => {
                invalid(backend, &name, &fields[7], &args[7])?;
                *errors = true;
            }
        }
        match args[8].as_str() {
            "WINDEXPOSED" => row.exterior_wind = true,
            "NOWIND" => row.exterior_wind = false,
            _ => {
                invalid(backend, &name, &fields[8], &args[8])?;
                *errors = true;
            }
        }
        row.view_factor_ground = owners.shortcut.numeric_args[0];
        if owners.shortcut.numeric_field_blanks[0] {
            row.view_factor_ground = -99999.0;
        }
        let coordinate_count = numeric_count
            .checked_sub(2)
            .ok_or(Error::Unavailable("actual surface numeric count below 2"))?;
        let entered_sides = coordinate_count / 3;
        let side_field = &owners.shortcut.numeric_field_names[1];
        if owners.shortcut.numeric_field_blanks[1] || owners.shortcut.numeric_args[1] == -99999.0 {
            row.sides = super::signed(entered_sides)?;
            if coordinate_count % 3 != 0 {
                backend.show_warning_error(&format!("BuildingSurface:Detailed=\"{name}\", {side_field} not even multiple of 3. Will read in {}", row.sides))?;
            }
            if row.sides < 3 {
                backend.show_severe_error(&format!("BuildingSurface:Detailed=\"{name}\", {side_field} (autocalculate) must be >= 3. Only {} provided.", row.sides))?;
                *errors = true;
                return Ok(SurfaceRowPrefix::Continue);
            }
        } else {
            let value = owners.shortcut.numeric_args[1];
            if !value.is_finite() || value < 0.0 || value.trunc() > f64::from(i32::MAX) {
                return Err(Error::Unavailable("Original signed allocation side count"));
            }
            row.sides = value as i32;
            if entered_sides
                > usize::try_from(row.sides)
                    .map_err(|_| Error::Unavailable("surface side allocation"))?
            {
                backend.show_warning_error(&format!(
                    "BuildingSurface:Detailed=\"{name}\", field {side_field}={}",
                    row.sides
                ))?;
                backend.show_continue_error(&format!("...but {entered_sides} were entered. Only the indicated {side_field} will be used."))?;
            }
        }
        let sides = usize::try_from(row.sides)
            .map_err(|_| Error::Unavailable("surface side allocation"))?;
        row.vertex = Some(vec![None; sides]);
        row.new_vertex = Some(vec![None; sides]);
        self.pending_vertices = Some(slot);
        Ok(SurfaceRowPrefix::BeforeVertices { slot, sides })
    }
    /// Called only by the genuine per-row GetVertices consumer after its return.
    /// This does not assert final geometry success, allocate Surface or copy it.
    pub(crate) fn returned_from_vertices(&mut self, slot: usize) -> Result<(), Error> {
        if self.pending_vertices != Some(slot) {
            return Err(Error::Unavailable("actual GetVertices return association"));
        }
        self.pending_vertices = None;
        Ok(())
    }
}
