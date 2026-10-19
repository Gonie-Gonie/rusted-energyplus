//! Actual WriteComponentSizing14199, SurfaceShadowing14472,
//! CompCostTable10822 and AdaptiveComfortTable12356 on the same owners.
use super::{Error, SourceTabularBody, SourceTabularReports, SourceTabularWriterOwners};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::ordinary_utility_bills::SourceCostEstimateMonetary;
use crate::heat_balance::predefined_tables::{PredefinedRegistry, SourcePredefinedShadowRelation};
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::source_initialization::SourceSqliteState;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;
use std::collections::BTreeMap;

/// One report call borrows the existing geometry, shadow combinations and files.
pub(crate) struct SourceSurfaceShadowingReportOwners<'a, 'backend> {
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) radiation: &'a OrdinarySolarRadiation,
    pub(crate) registry: &'a mut PredefinedRegistry,
    pub(crate) sqlite: &'a SourceSqliteState,
    pub(crate) results: &'a SourceResultsFrameworkOptions,
    pub(crate) writer: SourceTabularWriterOwners<'a, 'backend>,
}

fn extent(value: i32) -> Result<usize, Error> {
    usize::try_from(value).map_err(|_| Error::Unavailable("actual report negative array extent"))
}
fn ordinal(value: i32) -> Result<usize, Error> {
    value
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("actual report positive surface ordinal"))
}

impl SourceTabularReports {
    pub(crate) fn write_source_component_sizing(
        &mut self,
        registry: &mut PredefinedRegistry,
    ) -> Result<(), Error> {
        if !self.display_component_sizing {
            return Ok(());
        }
        // Native local string/int arrays are constructor-unallocated here.
        self.write_source_report_headers(
            b"Component Sizing Summary",
            b"Entire Facility",
            SourceOutputStoreType::Average,
        )?;
        for _pass in 0..self.input_state.tabular_report_passes.len() {
            // Native14247 resets the entire actual allocation, including its tail.
            if let Some(entries) = &mut registry.component_sizing_entries {
                for entry in entries {
                    entry.written = false;
                }
            }
            let count = extent(registry.component_sizing_count)?;
            let _unique_desc = vec![String::new(); count];
            let _unique_obj = vec![String::new(); count];
            let mut loop_limit = 0;
            while loop_limit <= 100 {
                let mut found_entry = 0;
                loop_limit += 1;
                for number in 1..=registry.component_sizing_count {
                    let index = usize::try_from(number - 1)
                        .map_err(|_| Error::Unavailable("actual WriteComponentSizing entry ordinal14268"))?;
                    let entry = registry.component_sizing_entries.as_ref().and_then(|rows| rows.get(index))
                        .ok_or(Error::Unavailable("actual WriteComponentSizing entry allocation14268"))?;
                    if !entry.written {
                        found_entry = number;
                        break;
                    }
                }
                if found_entry == 0 {
                    break;
                }
                return Err(Error::Unavailable("actual WriteComponentSizing positive grouped-table child14276"));
            }
        }
        Ok(())
    }

    pub(crate) fn write_source_surface_shadowing(
        &mut self,
        o: &mut SourceSurfaceShadowingReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !self.display_surface_shadowing {
            return Ok(());
        }
        let mut column_head = vec![Vec::<u8>::new(); 1];
        let mut column_width = [None::<i32>; 1];
        let mut receiving_fields = 0_i32;
        for surface in 1..=o.geometry.total_surfaces {
            let combination = o
                .radiation
                .solar_timestep
                .shadow_combinations
                .as_ref()
                .and_then(|v| v.get(ordinal(surface).ok()?))
                .ok_or(Error::Unavailable(
                    "actual WriteSurfaceShadowing14508 ShadowComb row",
                ))?;
            receiving_fields = receiving_fields
                .checked_add(combination.num_gen_surface)
                .ok_or(Error::Unavailable("actual NumGenSurf signed accumulation"))?;
            receiving_fields = receiving_fields
                .checked_add(combination.num_sub_surface)
                .ok_or(Error::Unavailable("actual NumSubSurf signed accumulation"))?;
        }
        o.registry.shadow_relations = Some(vec![
            SourcePredefinedShadowRelation::default();
            extent(receiving_fields)?
        ]);
        o.registry.shadow_relation_count = 0;
        for surface in 1..=o.geometry.total_surfaces {
            let combination = o
                .radiation
                .solar_timestep
                .shadow_combinations
                .as_ref()
                .and_then(|v| v.get(ordinal(surface).ok()?))
                .ok_or(Error::Unavailable(
                    "actual WriteSurfaceShadowing14515 ShadowComb row",
                ))?;
            for caster in 1..=combination.num_gen_surface {
                o.registry.shadow_relation_count = o
                    .registry
                    .shadow_relation_count
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "actual numShadowRelate signed increment",
                    ))?;
                let cast_surface = combination
                    .general
                    .as_ref()
                    .and_then(|v| v.get(extent(caster).ok()?))
                    .copied()
                    .ok_or(Error::Unavailable(
                        "actual WriteSurfaceShadowing14518 GenSurf read",
                    ))?;
                let relation = o
                    .registry
                    .shadow_relations
                    .as_mut()
                    .and_then(|v| v.get_mut(ordinal(o.registry.shadow_relation_count).ok()?))
                    .ok_or(Error::Unavailable("actual allocated ShadowRelate row"))?;
                relation.cast_surface = cast_surface;
                relation.receiving_surface = surface;
                relation.receiving_kind = 1;
            }
            for caster in 1..=combination.num_sub_surface {
                o.registry.shadow_relation_count = o
                    .registry
                    .shadow_relation_count
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "actual numShadowRelate signed increment",
                    ))?;
                let cast_surface = combination
                    .sub
                    .as_ref()
                    .and_then(|v| v.get(extent(caster).ok()?))
                    .copied()
                    .ok_or(Error::Unavailable(
                        "actual WriteSurfaceShadowing14525 SubSurf read",
                    ))?;
                let relation = o
                    .registry
                    .shadow_relations
                    .as_mut()
                    .and_then(|v| v.get_mut(ordinal(o.registry.shadow_relation_count).ok()?))
                    .ok_or(Error::Unavailable("actual allocated ShadowRelate row"))?;
                relation.cast_surface = cast_surface;
                relation.receiving_surface = surface;
                relation.receiving_kind = 2;
            }
        }
        if receiving_fields != o.registry.shadow_relation_count {
            return Err(Error::Unavailable(
                "actual WriteSurfaceShadowing14530 count assertion",
            ));
        }
        self.write_source_report_headers(
            b"Surface Shadowing Summary",
            b"Entire Facility",
            SourceOutputStoreType::Average,
        )?;
        let _unique = vec![None::<i32>; extent(o.registry.shadow_relation_count)?];
        for kind in 1..=2 {
            let mut shadow_map: BTreeMap<i32, (i32, Vec<&str>)> = BTreeMap::new();
            for number in 1..=o.registry.shadow_relation_count {
                let relation = o
                    .registry
                    .shadow_relations
                    .as_ref()
                    .and_then(|v| v.get(ordinal(number).ok()?))
                    .ok_or(Error::Unavailable("actual retained ShadowRelate row"))?;
                if relation.receiving_kind == kind {
                    let receiver = relation.receiving_surface;
                    let name = &o
                        .geometry
                        .final_surfaces
                        .as_ref()
                        .and_then(|v| v.get(ordinal(relation.cast_surface).ok()?))
                        .ok_or(Error::Unavailable(
                            "actual WriteSurfaceShadowing14545 casting Surface.Name",
                        ))?
                        .row
                        .name;
                    let element = shadow_map.entry(receiver).or_default();
                    let length = i32::try_from(name.len())
                        .map_err(|_| Error::Unavailable("actual casting name length int domain"))?;
                    element.0 = element.0.checked_add(length).ok_or(Error::Unavailable(
                        "actual casting name length signed accumulation",
                    ))?;
                    element.1.push(name);
                }
            }
            let unique_count = i32::try_from(shadow_map.len())
                .map_err(|_| Error::Unavailable("actual shadow_map size int domain"))?;
            column_head[0] = if unique_count == 0 {
                b"None".to_vec()
            } else {
                b"Possible Shadow Receivers".to_vec()
            };
            column_width.fill(Some(14));
            let mut row_head = vec![Vec::<u8>::new(); extent(unique_count)?];
            let mut body = SourceTabularBody::allocate(1, extent(unique_count)?)?;
            let mut unique_number = 0_usize;
            for (receiver, element) in &shadow_map {
                unique_number += 1;
                row_head[unique_number - 1] = o
                    .geometry
                    .final_surfaces
                    .as_ref()
                    .and_then(|v| v.get(ordinal(*receiver).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual WriteSurfaceShadowing14562 receiving Surface.Name",
                    ))?
                    .row
                    .name
                    .as_bytes()
                    .to_vec();
                let reserve = unique_count
                    .checked_mul(3)
                    .and_then(|v| element.0.checked_add(v))
                    .ok_or(Error::Unavailable(
                        "actual shadow name list reserve signed extent",
                    ))?;
                let mut names = Vec::with_capacity(extent(reserve)?);
                for name in &element.1 {
                    names.extend_from_slice(name.as_bytes());
                    names.extend_from_slice(b" | ");
                }
                body.set(1, unique_number, names)?;
            }
            let title: &[u8] = if kind == 1 {
                b"Surfaces (Walls, Roofs, etc) that may be Shadowed by Other Surfaces"
            } else {
                b"Subsurfaces (Windows and Doors) that may be Shadowed by Surfaces"
            };
            self.write_source_subtitle(title)?;
            // These actual unconditional sink guards precede WriteTable;
            // they do not use currentStyle.produceSQLite/produceJSON.
            if o.sqlite.present() {
                return Err(Error::Unavailable(
                    "actual WriteSurfaceShadowing14577/14597 SQLite table operation",
                ));
            }
            if o.results.time_series_and_tabular_enabled {
                return Err(Error::Unavailable(
                    "actual WriteSurfaceShadowing14586/14606 ResultsFramework table operation",
                ));
            }
            let mut widths =
                [column_width[0].ok_or(Error::Unavailable("actual columnWidth assignment"))?];
            self.write_source_table(
                &body,
                &row_head,
                &column_head,
                &mut widths,
                false,
                b"",
                &mut o.writer,
            )?;
        }
        Ok(())
    }

    pub(crate) fn write_source_component_cost_table(
        &mut self,
        monetary: &SourceCostEstimateMonetary,
    ) -> Result<(), Error> {
        if !*monetary.source_do_cost_estimate() {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual WriteCompCostTable10863 TableBodyData(3,10) positive cost body",
        ))
    }

    pub(crate) fn write_source_adaptive_comfort_table(
        &mut self,
        gains: &SelectedInternalHeatGains,
    ) -> Result<(), Error> {
        if !self.display_adaptive_comfort || gains.totals[0] <= 0 {
            return Ok(());
        }
        let _people_indices = vec![None::<i32>; extent(gains.totals[0])?];
        for _person in 1..=gains.totals[0] {
            return Err(Error::Unavailable(
                "actual WriteAdaptiveComfortTable12380 People AdaptiveASH55/AdaptiveCEN15251 read",
            ));
        }
        Ok(())
    }
}
