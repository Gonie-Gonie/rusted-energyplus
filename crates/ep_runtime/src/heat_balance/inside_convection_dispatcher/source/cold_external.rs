//! Actual cold InitExt first call: no Space traversal and no heat-transfer rows.
//! Physical Surface rows remain allocated in the same geometry owner. Only
//! the Native-skipped interior/roof/facade traversal views are empty here.
use super::*;

pub(super) fn layout(
    geometry: &SurfaceGeometryInputState,
    owners: &SelectedZoneInputOwners,
) -> Result<Option<InsideConvectionLayout>, Error> {
    let zones = owners.zones.as_deref()
        .ok_or(Error::Unavailable("actual cold external Zones"))?;
    let surfaces = geometry.final_surfaces.as_deref()
        .ok_or(Error::Unavailable("actual cold external physical Surface rows"))?;
    if owners.spaces.as_deref().is_some_and(|rows| !rows.is_empty())
        || zones.iter().any(|zone| !zone.space_indexes.is_empty())
        || surfaces.iter().any(|surface| surface.row.heat_transfer_surface)
    {
        return Ok(None);
    }
    let ids = zones.iter().map(|zone| zone.typed_zone_id
        .ok_or(Error::Unavailable("actual cold external Zone identity")))
        .collect::<Result<Vec<_>, _>>()?;
    // These are association/traversal views, not physical owner allocations.
    InsideConvectionLayout::new(ids, Vec::new(), Vec::new(), Vec::new())
        .map(Some)
        .map_err(|_| Error::Unavailable("actual cold external identity closure"))
}

impl SourceInsideConvection {
    pub(super) fn prepare_cold_external_convection(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        owners: &SelectedZoneInputOwners,
        input: &SelectedInputProcessor,
        reports: &ReportInputs,
        scan: &mut ReportScanState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<bool, Error> {
        let Some(layout) = layout(geometry, owners)? else { return Ok(false); };
        if self.state.layout != layout || !self.state.surfaces.is_empty() {
            return Err(Error::Unavailable("same actual cold external control owner"));
        }
        // Real six input families and the two repeated passes are queried;
        // each genuine zero-length array writer remains in its original order.
        let actual = self.get_source_user_input(input)?;
        self.actual_no_user_input = Some(actual);
        let zones = static_zone_views(owners)?;
        // Native still sums the actual Zone.Volume and multipliers. In the
        // constructor domain AutoCalculate remains -99999, never invented 0.
        // Actual empty spaceIndexes skip every interior metadata write; each
        // physical HeatTransSurf=false skips roof/facade reads of vertices.
        initialize_user_convection_metadata(
            &layout, &zones, &[], &mut self.state, geometry, reports, scan, backend,
        ).map_err(translate)?;
        Ok(true)
    }
}
