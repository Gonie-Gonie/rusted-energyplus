//! Actual selected SurfaceGeometry2857 radiant enclosure setup.
//! Opaque one-Zone/implicit-Space only; AirBoundary producers remain excluded.
use super::{OrdinarySolarRadiation, RadiantRadiationEnclosure};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState, SurfaceGetterOwners,
};
use crate::heat_balance::selected_heat_balance_input::zone::ZoneOutputHost;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputFileHost, SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits,
    SourceRealOutputDeclaration, SourceReportVariableInput,
};
use std::io::Write;

/// Borrow the same real registry, pre-scan, process setting and output host.
pub(crate) struct EnclosureOutputOwners<'a> {
    pub(crate) output: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) host: &'a mut dyn SourceOutputFileHost,
    pub(crate) eio: &'a mut dyn Write,
}

fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or(Error::Unavailable("positive radiant Source array index"))
}

impl OrdinarySolarRadiation {
    pub(crate) fn setup_radiant_enclosures(
        &mut self,
        surface_state: &SurfaceGeometryInputState,
        owners: &mut SurfaceGetterOwners<'_>,
        output: &mut EnclosureOutputOwners<'_>,
    ) -> Result<(), Error> {
        // Original14784 is the first state effect. Preserve this actual array
        // allocation even if a later represented prerequisite is unavailable.
        let spaces_count = usize::try_from(owners.globals.num_spaces)
            .map_err(|_| Error::Unavailable("actual numSpaces radiant allocation"))?;
        self.radiant = vec![RadiantRadiationEnclosure::default(); spaces_count];
        if owners
            .control
            .construction_usage
            .iter()
            .any(|row| row.type_is_air_boundary)
        {
            return Err(Error::Unavailable(
                "positive Construction:AirBoundary grouping",
            ));
        }
        let zones = owners
            .zones
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("same Zone enclosure owner"))?;
        let spaces = owners
            .zones
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("same Space enclosure owner"))?;
        // This is the existing ordinary Solar binding domain, not a new raw
        // admission rule. No positive explicit Space/air-boundary is enabled.
        if spaces_count != 1
            || spaces.len() != spaces_count
            || zones.len() != 1
            || owners.globals.num_of_zones != 1
        {
            return Err(Error::Unavailable(
                "one-Zone implicit-Space enclosure domain",
            ));
        }
        let surfaces = surface_state
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("same final Surface enclosure traversal"))?;
        let mut any_grouped_spaces = false;
        let mut enclosure_num = 0_i32;
        // Literal14975: floor-only spaces are examined even without any
        // Construction:AirBoundary. An empty surface list is floor-only too.
        for zone in zones.iter() {
            let mut new_enclosure_num = 0;
            for &space_num in &zone.space_indexes {
                let space = spaces
                    .get_mut(index(space_num)?)
                    .ok_or(Error::Unavailable("actual Zone Space enclosure index"))?;
                let space_enclosure_num = space.radiant_enclosure_num;
                let mut space_has_only_floors = false;
                if space_enclosure_num == 0 {
                    space_has_only_floors = true;
                    for &surface_num in &space.surfaces {
                        let surface = surfaces
                            .get(index(surface_num)?)
                            .ok_or(Error::Unavailable("actual Space Surface enclosure index"))?;
                        // Positive InternalMass is rejected by the earlier
                        // actual getter. Its skip case is not fabricated here.
                        if surface.row.class != SurfaceClass::Floor {
                            space_has_only_floors = false;
                            break;
                        }
                    }
                }
                if space_enclosure_num == 0 && space_has_only_floors {
                    any_grouped_spaces = true;
                    if new_enclosure_num == 0 {
                        enclosure_num = enclosure_num
                            .checked_add(1)
                            .ok_or(Error::Unavailable("Source radiant enclosure count"))?;
                        new_enclosure_num = enclosure_num;
                    }
                    space.radiant_enclosure_num = enclosure_num;
                    let enclosure = self
                        .radiant
                        .get_mut(index(enclosure_num)?)
                        .ok_or(Error::Unavailable("allocated grouped radiant enclosure"))?;
                    enclosure.name.clone_from(&zone.name);
                    enclosure.space_names.push(space.name.clone());
                    enclosure.space_nums.push(space_num);
                    // Source ASSIGNS, not sums, these in the floor-only loop.
                    enclosure.floor_area = space.floor_area;
                    enclosure.exterior_window_area = space.exterior_window_area;
                    enclosure.total_surface_area = space.total_surface_area;
                }
            }
        }
        if any_grouped_spaces {
            // On the retained one-Space domain the sole floor-only Space is
            // already assigned. Preserve the real remaining-space traversal.
            for space in spaces.iter() {
                if space.radiant_enclosure_num == 0 {
                    return Err(Error::Unavailable(
                        "remaining grouped/remainder Space producer",
                    ));
                }
            }
            self.num_radiant_enclosures = enclosure_num;
        } else {
            for (offset, space) in spaces.iter_mut().enumerate() {
                let space_num = i32::try_from(offset)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .ok_or(Error::Unavailable("Source radiant Space number"))?;
                let enclosure = self
                    .radiant
                    .get_mut(offset)
                    .ok_or(Error::Unavailable("allocated radiant Space row"))?;
                enclosure.name.clone_from(&space.name);
                enclosure.space_names.push(space.name.clone());
                enclosure.space_nums.push(space_num);
                enclosure.floor_area = space.floor_area;
                space.radiant_enclosure_num = space_num;
            }
            self.num_radiant_enclosures = owners.globals.num_spaces;
        }
        let count = usize::try_from(self.num_radiant_enclosures)
            .map_err(|_| Error::Unavailable("Source radiant resize count"))?;
        if count > self.radiant.len() {
            return Err(Error::Unavailable("Source radiant size assertion"));
        }
        self.radiant.truncate(count); // Source resize after the same size assertion.

        // Real unmatched OutputProcessor branch still writes dictionary/setup
        // counts. Requested MRT stops at its existing unported OutVar boundary;
        // no constant value or fake pointer is substituted for enclosure.MRT.
        let mut request = SourceReportVariableInput::new(
            &owners.model.report_inputs,
            &mut *owners.input,
            output.minimum_report_frequency,
        );
        let mut host = ZoneOutputHost(&mut *output.host);
        for enclosure in &mut self.radiant {
            output
                .output
                .setup_unmetered_zone_real(
                    &mut request,
                    output.prescan,
                    SourceRealOutputDeclaration {
                        name: "Enclosure Mean Radiant Temperature",
                        key: &enclosure.name,
                        units: SourceOutputUnits::C,
                        store_type: SourceOutputStoreType::Average,
                    },
                    output.minutes_in_time_step,
                    &mut host,
                    &mut *output.eio,
                )
                .map_err(Error::Output)?;
        }
        // The source registers before writing Zone ranges. Failure above keeps
        // all allocated rows and Space pointers, with Zone defaults unchanged.
        for zone in zones.iter_mut() {
            for &space_num in &zone.space_indexes {
                let number = spaces
                    .get(index(space_num)?)
                    .ok_or(Error::Unavailable("actual Zone radiant range Space"))?
                    .radiant_enclosure_num;
                if zone.radiant_enclosure_first == -1 {
                    zone.radiant_enclosure_first = number;
                } else {
                    zone.radiant_enclosure_first = zone.radiant_enclosure_first.min(number);
                }
                zone.radiant_enclosure_last = zone.radiant_enclosure_last.max(number);
            }
            if zone.radiant_enclosure_first == -1
                || zone.radiant_enclosure_last == -1
                || zone.radiant_enclosure_first > zone.radiant_enclosure_last
            {
                return Err(Error::Unavailable("Source Zone radiant range assertions"));
            }
        }
        Ok(())
    }
}
