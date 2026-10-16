//! Genuine GetHTSurfaceData post-GetVertices and CheckConvexity handoff.
use super::super::{
    SelectedSolarDistribution,
    shadowing::{ShadowingSystemSettings, SourceShadingMethod, SourceShadowingInput},
};
use super::{
    Error, SourceErrorBackend, SurfaceGeometryInputState, SurfaceGetterOwners, SurfaceTmpInput,
};
mod diagnostics;
mod state;
pub(super) use state::SourceConvexityState;

impl SurfaceGeometryInputState {
    /// Actual4339-4359. Final copy later retains this same row's IsConvex.
    pub(crate) fn finish_opaque_world_row_after_vertices(
        &mut self,
        slot: usize,
        owners: &SurfaceGetterOwners<'_>,
        system: Option<&ShadowingSystemSettings>,
        shadowing: Option<&SourceShadowingInput>,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        let row = self
            .surface_tmp
            .as_mut()
            .and_then(|rows| rows.get_mut(slot))
            .ok_or(Error::Unavailable("actual post-GetVertices SurfaceTmp"))?;
        let area = row.area;
        if area <= 0.0 {
            backend.show_severe_error(&format!(
                "{}=\"{}\", Surface Area <= 0.0; Entered Area={}",
                owners.shortcut.current_module_object,
                row.name,
                diagnostics::trim_2(area)?
            ))?;
            *errors = true;
        }
        let sides = usize::try_from(row.sides)
            .map_err(|_| Error::Unavailable("Source CheckConvexity NSides"))?;
        self.check_convexity.check(
            row,
            sides,
            self.coordinate.max_vertices_per_surface,
            owners,
            system,
            shadowing,
            backend,
        )?;
        // Literal cAlphaArgs(5), including the Source's space-name position.
        if owners
            .shortcut
            .alpha_args
            .get(4)
            .ok_or(Error::Unavailable("actual cAlphaArgs(5)"))?
            .eq_ignore_ascii_case("Surface")
        {
            let count = row
                .vertex
                .as_ref()
                .ok_or(Error::Unavailable(
                    "actual Vertex allocation after CheckConvexity",
                ))?
                .len();
            if i32::try_from(count).map_err(|_| Error::Unavailable("Source Vertex signed size"))?
                != row.sides
            {
                backend.show_severe_error(&format!(
                    "{}=\"{}\", After CheckConvexity, mismatch between Sides ({}) and size of Vertex ({}).",
                    owners.shortcut.current_module_object, row.name, row.sides, count
                ))?;
                backend.show_continue_error("CheckConvexity is used to verify the convexity of a surface and detect collinear points.")?;
                *errors = true;
            }
        }
        Ok(())
    }
}

fn increment(value: &mut i32) -> Result<(), Error> {
    *value = value.checked_add(1).ok_or(Error::Unavailable(
        "Source CheckConvexity tracking counter overflow",
    ))?;
    Ok(())
}
fn vertex(row: &SurfaceTmpInput, n: usize) -> Result<[f64; 3], Error> {
    row.vertex
        .as_ref()
        .and_then(|v| v.get(n - 1))
        .copied()
        .flatten()
        .ok_or(Error::Unavailable("actual CheckConvexity written Vertex"))
}

impl SourceConvexityState {
    fn check(
        &mut self,
        row: &mut SurfaceTmpInput,
        nsides: usize,
        max_vertices: usize,
        owners: &SurfaceGetterOwners<'_>,
        system: Option<&ShadowingSystemSettings>,
        shadowing: Option<&SourceShadowingInput>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        const TURN_THRESHOLD: f64 = 0.000001;
        let mut _last_theta = 0.0;
        let mut previous_sign = false;
        let mut previous_initialized = false;
        let mut collinear = Vec::with_capacity(
            nsides
                .checked_add(2)
                .ok_or(Error::Unavailable("Source collinear reserve bounds"))?,
        );
        self.prepare(nsides, max_vertices)?;
        for n in 1..=nsides {
            self.write_xyz(n, vertex(row, n)?)?;
        }
        self.write_xyz(nsides + 1, vertex(row, 1)?)?;
        self.write_xyz(nsides + 2, vertex(row, 2)?)?;
        // Exact XY -> XZ -> YZ determinant traversal and whole-array assignment.
        let mut determinant = 0.0;
        for n in 1..=nsides {
            determinant += self.x(n)? * self.y(n + 1)? - self.x(n + 1)? * self.y(n)?;
        }
        if determinant.abs() > 1.0e-4 {
            self.plane(0)?;
        } else {
            determinant = 0.0;
            for n in 1..=nsides {
                determinant += self.x(n)? * self.z(n + 1)? - self.x(n + 1)? * self.z(n)?;
            }
            if determinant.abs() > 1.0e-4 {
                self.plane(1)?;
            } else {
                determinant = 0.0;
                for n in 1..=nsides {
                    determinant += self.y(n)? * self.z(n + 1)? - self.y(n + 1)? * self.z(n)?;
                }
                if determinant.abs() > 1.0e-4 {
                    self.plane(2)?;
                } else {
                    backend.show_severe_error(&format!(
                        "CheckConvexity: Surface=\"{}\" is non-planar.",
                        row.name
                    ))?;
                    backend
                        .show_continue_error("Coincident Vertices will be removed as possible.")?;
                    for n in 1..=usize::try_from(row.sides)
                        .map_err(|_| Error::Unavailable("Source CheckConvexity Sides"))?
                    {
                        let p = vertex(row, n)?;
                        backend.show_continue_error(&format!(
                            " ({},{},{})",
                            diagnostics::fixed_8_3(p[0])?,
                            diagnostics::fixed_8_3(p[1])?,
                            diagnostics::fixed_8_3(p[2])?
                        ))?;
                    }
                    // Source leaves prior A/B contents intact. Unwritten cells
                    // remain unavailable instead of being filled with a fake plane.
                }
            }
        }
        for n in 1..=nsides {
            let pt0 = self.ab(n)?;
            let pt1 = self.ab(n + 1)?;
            let pt2 = self.ab(n + 2)?;
            let v1 = [pt1[0] - pt0[0], pt1[1] - pt0[1]];
            let v2 = [pt2[0] - pt1[0], pt2[1] - pt1[1]];
            // ObjexxFCL Vector2 length and cross preserve operand order.
            let v1len = ((v1[0] * v1[0]) + (v1[1] * v1[1])).sqrt();
            let v2len = ((v2[0] * v2[0]) + (v2[1] * v2[1])).sqrt();
            if v1len <= 1.0e-8 || v2len <= 1.0e-8 {
                continue;
            }
            let cross = (v1[0] * v2[1]) - (v1[1] * v2[0]);
            let mut sinarg = cross / (v1len * v2len);
            #[expect(
                clippy::manual_clamp,
                reason = "Preserve EnergyPlus CheckConvexity's strict comparison and assignment order"
            )]
            if sinarg < -1.0 {
                sinarg = -1.0;
            } else if sinarg > 1.0 {
                sinarg = 1.0;
            }
            let theta = sinarg.asin();
            let sign = if theta > TURN_THRESHOLD {
                true
            } else if theta < -TURN_THRESHOLD {
                false
            } else {
                let mut index = n + 1;
                if index > nsides {
                    index -= nsides;
                }
                if owners.globals.display_extra_warnings {
                    backend.show_warning_error(&format!("CheckConvexity: Surface=\"{}\", vertex {} is colinear with previous and next.",row.name,index))?;
                }
                increment(&mut backend.state.tracking.total_coincident_vertices)?;
                collinear.push(index);
                continue;
            };
            if !previous_initialized {
                previous_sign = sign;
                _last_theta = theta;
                previous_initialized = true;
                continue;
            }
            if sign != previous_sign {
                if owners.globals.display_extra_warnings
                    && row.exterior_solar
                    && owners.control.solar_distribution != SelectedSolarDistribution::Minimal
                    && row.is_convex
                    && !system
                        .ok_or(Error::Unavailable(
                            "actual CheckConvexity SutherlandHodgman owner",
                        ))?
                        .sutherland_hodgman
                    && shadowing
                        .ok_or(Error::Unavailable(
                            "actual CheckConvexity shadingMethod owner",
                        ))?
                        .shading_method
                        == SourceShadingMethod::PolygonClipping
                {
                    diagnostics::nonconvex(self, row, n, nsides, owners, backend)?;
                }
                row.is_convex = false;
                // Do not break: Source10103 still removes every collinear point.
            }
            previous_sign = sign;
            _last_theta = theta;
        }
        let m = collinear.len();
        if m > 0 {
            if nsides - m >= 3 {
                row.sides = i32::try_from(nsides - m)
                    .map_err(|_| Error::Unavailable("Source convexity Sides signed count"))?;
                if owners.globals.display_extra_warnings {
                    backend.show_warning_error(&format!("CheckConvexity: Surface=\"{}\" has [{}] collinear points that have been removed.",row.name,m))?;
                }
            } else {
                if owners.globals.display_extra_warnings {
                    backend.show_warning_error(&format!(
                        "CheckConvexity: Surface=\"{}\" has [{}] collinear points.",
                        row.name, m
                    ))?;
                    backend.show_continue_error("...too many to remove all.  Will leave the surface with 3 sides. But this is now a degenerate surface")?;
                }
                increment(&mut backend.state.tracking.total_degenerate_surfaces)?;
                row.sides = 3;
                // Source resize(NSides-3) strictly shrinks in this branch.
                collinear.truncate(
                    nsides
                        .checked_sub(3)
                        .ok_or(Error::Unavailable("Source collinear resize bounds"))?,
                );
            }
            let mut new_vertices = vec![
                None;
                usize::try_from(row.sides).map_err(|_| {
                    Error::Unavailable("Source newVertices allocation bounds")
                })?
            ];
            let mut n = 0;
            for i in 1..=nsides {
                if !collinear.contains(&i) {
                    *new_vertices
                        .get_mut(n)
                        .ok_or(Error::Unavailable("Source newVertices indexed write"))? =
                        Some(vertex(row, i)?);
                    n += 1;
                }
            }
            row.vertex = Some(new_vertices);
            // No recomputation of NewVertex/Area/height/width occurs in Source.
            if owners.globals.display_extra_warnings {
                backend.show_warning_error(&format!("CheckConvexity: Surface=\"{}\": The vertex points has been reprocessed as Sides = {}",row.name,row.sides))?;
            }
        }
        Ok(())
    }
}
