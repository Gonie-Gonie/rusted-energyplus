//! Actual SHD stream presence and DetermineShadowingCombinations report child.
//! The same local CastingSurface owner survives the numerical child and report.
use super::OrdinarySolarRadiation;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, SelectedSolarDistribution,
};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// A present nullptr-buffer iostream differs from an absent unique_ptr.
/// Physical fstream failures set stream state; Source does not enable exceptions
/// or test is_open at this call. The adapter retains those failures without
/// manufacturing an Original fatal at the unique_ptr allocation test.
pub(crate) enum SourceShadingOutput {
    NullBuffer,
    Physical {
        file: File,
        failure: Option<io::Error>,
    },
    #[allow(
        dead_code,
        reason = "Keep the actual failed-open IO error as owned diagnostic state; preserving Native allocation-versus-stream-failure behavior must not discard its failure payload."
    )]
    FailedOpen(io::Error),
}
#[derive(Default)]
pub(crate) struct SourceShadingStream {
    pub(crate) stream: Option<SourceShadingOutput>,
}
impl SourceShadingStream {
    /// SolarShading188-202. Path and output control belong to the real caller.
    pub(crate) fn open(&mut self, output_shd: bool, path: &Path) {
        self.stream = Some(if output_shd {
            match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
            {
                Ok(file) => SourceShadingOutput::Physical {
                    file,
                    failure: None,
                },
                Err(error) => SourceShadingOutput::FailedOpen(error),
            }
        } else {
            SourceShadingOutput::NullBuffer
        });
    }
    fn put(&mut self, text: &str) {
        if let Some(SourceShadingOutput::Physical { file, failure }) = self.stream.as_mut() {
            if failure.is_none() {
                // Original fstream is opened in text mode. Windows CRT translates
                // every LF, including those inside inserted string arguments.
                let translated;
                let bytes = if cfg!(windows) {
                    translated = text.replace('\n', "\r\n");
                    translated.as_bytes()
                } else {
                    text.as_bytes()
                };
                if let Err(error) = file.write_all(bytes) {
                    *failure = Some(error);
                }
            }
        }
    }
    /// Actual unique_ptr reset216, with no fabricated close-error return.
    pub(crate) fn reset(&mut self) {
        self.stream = None;
    }
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("actual positive shadow report ordinal"))
}
fn surface_name(geometry: &SurfaceGeometryInputState, number: i32) -> Result<&str, Error> {
    Ok(&geometry
        .final_surfaces
        .as_deref()
        .and_then(|rows| rows.get(index(number).ok()?))
        .ok_or(Error::Unavailable("same actual shadow report Surface Name"))?
        .row
        .name)
}
fn reference(values: &Option<Vec<i32>>, number: i32) -> Result<i32, Error> {
    values
        .as_deref()
        .and_then(|rows| rows.get(usize::try_from(number).ok()?))
        .copied()
        .ok_or(Error::Unavailable(
            "same actual lower-zero shadow reference array",
        ))
}
fn increment(value: &mut i32) -> Result<(), Error> {
    *value = value.checked_add(1).ok_or(Error::Unavailable(
        "undefined Source shadow counter increment",
    ))?;
    Ok(())
}
const DETAILS: &str = "...Shadowing values may be inaccurate. Check .shd report file for more surface shading details";

/// Actual SolarShading5621-5725 PolygonClipping report branch. Positive general
/// lists remain unavailable in the preceding Minimal numerical producer; this
/// child nevertheless preserves every Source list and diagnostic operation.
pub(crate) fn write_source_shadow_combinations(
    stream: &mut SourceShadingStream,
    solar: &mut OrdinarySolarRadiation,
    geometry: &SurfaceGeometryInputState,
    control: &SelectedHeatBalanceInputState,
    globals: &SelectedHeatBalanceGlobals,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let count = usize::try_from(geometry.total_surfaces)
        .map_err(|_| Error::Unavailable("actual shadow report TotSurfaces"))?;
    let state = &mut solar.solar_timestep;
    let combinations = state
        .shadow_combinations
        .as_deref()
        .ok_or(Error::Unavailable("same actual ShadowComb allocation"))?;
    let casting = state.casting_surface.as_mut().ok_or(Error::Unavailable(
        "same actual DetermineShadowingCombinations local CastingSurface",
    ))?;
    if casting.len() != count {
        return Err(Error::Unavailable(
            "actual CastingSurface allocation extent",
        ));
    }
    let surfaces = geometry
        .final_surfaces
        .as_deref()
        .ok_or(Error::Unavailable(
            "same actual SHD final Surface allocation",
        ))?;
    if stream.stream.is_some() {
        stream.put("Shadowing Combinations\n");
        match control.solar_distribution {
            SelectedSolarDistribution::Minimal => stream.put("..Solar Distribution=Minimal Shadowing, Detached Shading will not be used in shadowing calculations\n"),
            SelectedSolarDistribution::FullExterior => {
                if control.calculate_solar_reflections {
                    stream.put("..Solar Distribution=FullExteriorWithReflectionsFromExteriorSurfaces\n");
                } else { stream.put("..Solar Distribution=FullExterior\n"); }
            }
            SelectedSolarDistribution::FullInteriorExterior => {
                if control.calculate_solar_reflections {
                    stream.put("..Solar Distribution=FullInteriorAndExteriorWithReflectionsFromExteriorSurfaces\n");
                } else { stream.put("..Solar Distribution=FullInteriorAndExterior\n"); }
            }
        }
        stream.put("..In the following, only the first 10 reference surfaces will be shown.\n");
        stream.put("..But all surfaces are used in the calculations.\n");
        for &number in &geometry.all_surface_list_report_order {
            let i = index(number)?;
            let combination = combinations
                .get(i)
                .ok_or(Error::Unavailable("actual ShadowComb report ordinal"))?;
            let surface = &surfaces
                .get(i)
                .ok_or(Error::Unavailable("actual SHD Surface ordinal"))?
                .row;
            stream.put("==================================\n");
            if combination.use_this_surface {
                stream.put("Surface=");
                stream.put(&surface.name);
                if surface.is_convex {
                    stream.put(" is used as Receiving Surface in calculations and is convex.\n");
                } else {
                    stream
                        .put(" is used as Receiving Surface in calculations and is non-convex.\n");
                    if combination.num_gen_surface > 0 {
                        if globals.display_extra_warnings {
                            backend.show_warning_error(&format!("DetermineShadowingCombinations: Surface=\"{}\" is a receiving surface and is non-convex.",surface.name))?;
                            backend.show_continue_error(DETAILS)?;
                        } else {
                            increment(
                                &mut backend.state.tracking.total_receiving_non_convex_surfaces,
                            )?;
                        }
                    }
                }
            } else {
                stream.put("Surface=");
                stream.put(&surface.name);
                stream.put(" is not used as Receiving Surface in calculations.\n");
            }
            stream.put("Number of general casting surfaces=");
            stream.put(&combination.num_gen_surface.to_string());
            stream.put("\n");
            for number in 1..=combination.num_gen_surface {
                if number <= 10 {
                    stream.put("..Surface=");
                    stream.put(surface_name(
                        geometry,
                        reference(&combination.general, number)?,
                    )?);
                    stream.put("\n");
                }
                *casting
                    .get_mut(index(reference(&combination.general, number)?)?)
                    .ok_or(Error::Unavailable(
                        "actual CastingSurface reference ordinal",
                    ))? = true;
            }
            stream.put("Number of back surfaces=");
            stream.put(&combination.num_back_surface.to_string());
            stream.put("\n");
            for number in 1..=10.min(combination.num_back_surface) {
                stream.put("...Surface=");
                stream.put(surface_name(
                    geometry,
                    reference(&combination.back, number)?,
                )?);
                stream.put("\n");
            }
            stream.put("Number of receiving sub surfaces=");
            stream.put(&combination.num_sub_surface.to_string());
            stream.put("\n");
            for number in 1..=10.min(combination.num_sub_surface) {
                stream.put("....Surface=");
                stream.put(surface_name(
                    geometry,
                    reference(&combination.sub, number)?,
                )?);
                stream.put("\n");
            }
        }
    }
    for i in 0..count {
        if casting[i]
            && !surfaces
                .get(i)
                .ok_or(Error::Unavailable("actual casting Surface ordinal"))?
                .row
                .is_convex
        {
            if globals.display_extra_warnings {
                backend.show_severe_error(&format!("DetermineShadowingCombinations: Surface=\"{}\" is a casting surface and is non-convex.",surfaces[i].row.name))?;
                backend.show_continue_error(DETAILS)?;
            } else {
                increment(&mut backend.state.tracking.total_casting_non_convex_surfaces)?;
            }
        }
    }
    let receiving = backend.state.tracking.total_receiving_non_convex_surfaces;
    if receiving > 0 {
        backend.show_warning_message(&format!("DetermineShadowingCombinations: There are {receiving} surfaces which are receiving surfaces and are non-convex."))?;
        backend.show_continue_error(DETAILS)?;
        backend.show_continue_error("...Add Output:Diagnostics,DisplayExtraWarnings; to see individual warnings for each surface.")?;
        backend.state.tracking.total_warning_errors = backend
            .state
            .tracking
            .total_warning_errors
            .checked_add(receiving)
            .ok_or(Error::Unavailable(
                "undefined Source warning count addition",
            ))?;
    }
    let casting_count = backend.state.tracking.total_casting_non_convex_surfaces;
    if casting_count > 0 {
        backend.show_severe_message(&format!("DetermineShadowingCombinations: There are {casting_count} surfaces which are casting surfaces and are non-convex."))?;
        backend.show_continue_error(DETAILS)?;
        backend.show_continue_error("...Add Output:Diagnostics,DisplayExtraWarnings; to see individual severes for each surface.")?;
        backend.state.tracking.total_severe_errors = backend
            .state
            .tracking
            .total_severe_errors
            .checked_add(casting_count)
            .ok_or(Error::Unavailable("undefined Source severe count addition"))?;
    }
    // The same caller drops actual CastingSurface at5727 after this return.
    Ok(())
}
