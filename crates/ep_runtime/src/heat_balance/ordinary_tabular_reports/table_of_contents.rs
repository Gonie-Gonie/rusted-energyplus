//! Genuine AddTOCLoadComponentTableSummaries14828–14853 + AddTOCEntry18768–18780.
use super::*;
impl SourceTabularReports {
    pub(super) fn add_source_load_component_table_summaries(
        &mut self,
        o: &SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.globals.component_load_report_requested {
            if self.display_zone_component_load_summary {
                if o.space_controls.sizing {
                    for ordinal in 1..=o.globals.num_spaces {
                        let at = usize::try_from(ordinal - 1)
                            .map_err(|_| Error::Unavailable("actual TOC Space ordinal"))?;
                        let space = o
                            .zones
                            .spaces
                            .as_deref()
                            .and_then(|r| r.get(at))
                            .ok_or(Error::Unavailable("actual TOC Space owner"))?;
                        if !*o
                            .equipment
                            .source_zone_equipment_config(space.zone_num)?
                            .is_controlled()
                        {
                            continue;
                        }
                        self.add_source_table_of_contents_entry(
                            "Space Component Load Summary",
                            &space.name,
                        )?;
                    }
                }
                for ordinal in 1..=o.globals.num_of_zones {
                    if !*o
                        .equipment
                        .source_zone_equipment_config(ordinal)?
                        .is_controlled()
                    {
                        continue;
                    }
                    let at = usize::try_from(ordinal - 1)
                        .map_err(|_| Error::Unavailable("actual TOC Zone ordinal"))?;
                    let zone = o
                        .zones
                        .zones
                        .as_deref()
                        .and_then(|r| r.get(at))
                        .ok_or(Error::Unavailable("actual TOC Zone owner"))?;
                    self.add_source_table_of_contents_entry(
                        "Zone Component Load Summary",
                        &zone.name,
                    )?;
                }
            }
            if self.display_air_loop_component_load_summary
                && o.hvac.number_of_primary_air_systems > 0
            {
                return Err(Error::Unavailable(
                    "actual TOC FinalSysSizing(AirLoop).AirPriLoopName14847 first read",
                ));
            }
            if self.display_facility_component_load_summary {
                self.add_source_table_of_contents_entry(
                    "Facility Component Load Summary",
                    "Facility",
                )?;
            }
        }
        Ok(())
    }
    pub(super) fn add_source_table_of_contents_entry(
        &mut self,
        section: &str,
        report: &str,
    ) -> Result<(), Error> {
        if self.input_state.toc_entries.is_none() {
            self.input_state.toc_entries_size = 20;
            self.input_state.toc_entries =
                Some(vec![state::SourceTableOfContentsEntry::default(); 20]);
            self.input_state.toc_entries_count = 1;
        } else {
            self.input_state.toc_entries_count += 1;
            if self.input_state.toc_entries_count > self.input_state.toc_entries_size {
                self.input_state.toc_entries_size += 20;
                let n = usize::try_from(self.input_state.toc_entries_size)
                    .map_err(|_| Error::Unavailable("actual TOC redimension size"))?;
                self.input_state
                    .toc_entries
                    .as_mut()
                    .ok_or(Error::Unavailable("actual allocated TOC redimension"))?
                    .resize(n, state::SourceTableOfContentsEntry::default());
            }
        }
        let at = usize::try_from(self.input_state.toc_entries_count - 1)
            .map_err(|_| Error::Unavailable("actual TOC count"))?;
        let row = self
            .input_state
            .toc_entries
            .as_mut()
            .and_then(|r| r.get_mut(at))
            .ok_or(Error::Unavailable("actual TOC entry"))?;
        row.report_name = report.into();
        row.section_name = section.into();
        Ok(())
    }
}
