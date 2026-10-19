//! Full original two opaque absorption consumers; declared primary inputs only.
use super::*;
use crate::heat_balance::radiation::ordinary::SolarRadiationEnclosure;
use crate::heat_balance::radiation::ordinary::radiant_exchange::RadiantExchangeSource;

fn diagnostics(state:&SourceErrorState,bytes:&[u8],e:&Value)->Result<()> {
    if e["warnings"].as_i64()!=Some(i64::from(state.tracking.total_warning_errors))
        || e["severe"].as_i64()!=Some(i64::from(state.tracking.total_severe_errors))
        || e["recurring"].as_i64()!=Some(0)
        || e["text"].as_str()!=Some(std::str::from_utf8(bytes)?)
        || state.output_error_header!=bytes.is_empty() {
        return Err("whole absorption diagnostic text/counters/header latch differ".into());
    }Ok(())
}
impl OrdinarySolarRadiation {
    fn assert_native_gap_absorption(&self,thermal:&[f64],shortwave:&[f64],errors:&SourceErrorState,bytes:&[u8],e:&Value)->Result<()> {
        let radiant=self.radiant.first().ok_or("actual radiant row")?;
        let solar=self.solar.as_ref().and_then(|rows|rows.first()).ok_or("actual solar row")?;
        if e["radiant_count"].as_i64()!=Some(i64::from(self.num_radiant_enclosures))
            || e["solar_count"].as_i64()!=Some(i64::from(self.num_solar_enclosures))
            || e["radReCalc"].as_bool()!=Some(radiant.rad_recalc)
            || e["solar_radReCalc"].as_bool()!=Some(solar.rad_recalc)
            || e["NumOfSurfaces"].as_i64()!=Some(i64::from(radiant.num_surfaces))
            || e["solAbsFirstCalc"].as_bool()!=Some(solar.sol_abs_first_calc) {
            return Err("actual absorption owner counts/flags differ".into());
        }
        let pointers=radiant.source_surface_ptr.as_ref().ok_or("actual SurfacePtr allocation")?;
        if serde_json::to_value(pointers)?!=e["SurfacePtr"]{return Err("actual absorption pointer order differs".into());}
        bits(radiant.rad_therm_abs_mult,&e["radThermAbsMult"],"thermal reciprocal")?;
        bits(solar.sol_vmult,&e["solVMULT"],"solar reciprocal")?;
        if e["inside_thermal_absorptance"]["values_defined"].as_bool()!=Some(true)||e["inside_solar_absorptance"]["values_defined"].as_bool()!=Some(true){return Err("Native absorption arrays must have genuine defined values".into());}
        vector(thermal,&e["inside_thermal_absorptance"],"actual inside thermal values")?;
        vector(shortwave,&e["inside_solar_absorptance"],"actual inside shortwave values")?;
        diagnostics(errors,bytes,&e["diagnostics"])
    }
    pub(crate) fn native_gap_absorption_cases(input:&Value,native:&Value)->Result<usize> {
        let cases=array(&input["absorption"])?;let expected=array(&native["absorption"])?;
        if cases.len()!=expected.len(){return Err("absorption case count differs".into());}let mut count=0;
        for(case,oracle)in cases.iter().zip(expected){
            if case!=&oracle["input"]||case["name"]!=oracle["name"]{return Err("whole absorption primary differs".into());}
            let areas=array(&case["areas"])?.iter().map(real).collect::<Result<Vec<_>>>()?;let n=areas.len();
            if n==0||n>8{return Err("declared bounded surface count".into());}
            let name=case["name"].as_str().ok_or("absorption name")?;
            let mut geometry=SurfaceGeometryInputState::native_gap_opaque_radiation_rows(n)?;
            for(i,surface)in geometry.final_surfaces.as_mut().ok_or("actual final rows")?.iter_mut().enumerate(){
                surface.row.name=format!("Declared opaque {}",i+1);surface.row.area=areas[i];
            }
            let active=&mut geometry.window_arrays.as_mut().ok_or("actual DataSurface arrays")?.surf_active_construction;
            for(i,v)in active.iter_mut().enumerate(){*v=i32::try_from(i+1)?;}
            let mut zones=SelectedZoneInputOwners::default();
            zones.zones=Some(vec![SelectedZoneData {name:name.into(),space_indexes:vec![1],..Default::default()}]);
            zones.spaces=Some(vec![Default::default()]);
            let space=zones.spaces.as_mut().and_then(|rows|rows.first_mut()).ok_or("actual Space row")?;
            space.name="Declared opaque unit space".into();space.zone_num=1;
            let mut control=SelectedHeatBalanceInputState::default();control.construction_usage=vec![SelectedConstructionUsage::default();n];
            let inventory=OrdinaryRadiationInput::from_admitted_inventory(std::iter::empty::<(&str,usize)>(),false)?;
            let mut owner=Self::for_surface_input(inventory);
            let pointers=(1..=n).map(i32::try_from).collect::<std::result::Result<Vec<_>,_>>()?;
            owner.num_radiant_enclosures=1;owner.num_solar_enclosures=1;
            owner.radiant.push(RadiantRadiationEnclosure {name:name.into(),space_nums:vec![1],num_surfaces:i32::try_from(n)?,source_surface_ptr:Some(pointers.clone()),..Default::default()});
            owner.solar=Some(vec![SolarRadiationEnclosure {name:name.into(),space_nums:vec![1],num_surfaces:i32::try_from(n)?,source_surface_ptr:Some(pointers),..Default::default()}]);
            let radiant_address=owner.radiant.as_ptr();let solar_address=owner.solar.as_ref().ok_or("solar allocation")?.as_ptr();
            let mut thermal=vec![f64::NAN;n];let mut shortwave=vec![f64::NAN;n];
            let mut errors=cold_errors();let mut bytes=Vec::new();let mut console=Vec::new();
            let steps=array(&case["steps"])?;let calls=array(&oracle["calls"])?;
            if steps.len()!=calls.len(){return Err("absorption repeated sequence differs".into());}
            let mut native_identity=None;
            for(step,call)in steps.iter().zip(calls){
                if step!=&call["primary_step"]{return Err("absorption whole step input differs".into());}
                let a=array(&step["thermal"])?;let b=array(&step["shortwave"])?;if a.len()!=n||b.len()!=n{return Err("primary absorption extents".into());}
                for(i,(a,b))in a.iter().zip(b).enumerate(){thermal[i]=real(a)?;shortwave[i]=real(b)?;}
                let recalc=step["recalculate"].as_bool().ok_or("primary recalc flag")?;
                owner.radiant[0].rad_recalc=recalc;owner.solar.as_mut().ok_or("solar rows")?[0].rad_recalc=recalc;
                for key in ["before","after_thermal","after_shortwave"] {
                    let state=&call[key];
                    for name in ["state_address","radiant_owner_address","solar_owner_address"]{if state[name].as_u64().filter(|v|*v>0).is_none(){return Err("Native absorption identity must be an actual nonnull address".into());}}
                    let addresses=(state["state_address"].clone(),state["radiant_owner_address"].clone(),state["solar_owner_address"].clone());
                    if let Some(identity)=&native_identity{if identity!=&addresses{return Err("Native repeated owner identity differs".into());}}
                    else{native_identity=Some(addresses);}
                }
                owner.assert_native_gap_absorption(&thermal,&shortwave,&errors,&bytes,&call["before"])?;
                let source=RadiantExchangeSource {geometry:&geometry,zones:&zones,constructions:&control,carroll_method:false,any_inside_shelf:false};
                owner.compute_source_thermal_absorption(&source,&mut thermal,false)?;
                owner.assert_native_gap_absorption(&thermal,&shortwave,&errors,&bytes,&call["after_thermal"])?;
                owner.compute_source_shortwave_absorption(&source,&shortwave,&mut backend(&mut errors,&mut bytes,&mut console))?;
                owner.assert_native_gap_absorption(&thermal,&shortwave,&errors,&bytes,&call["after_shortwave"])?;
                if owner.radiant.as_ptr()!=radiant_address||owner.solar.as_ref().ok_or("solar rows retained")?.as_ptr()!=solar_address{return Err("Source same-owner lifetime changed".into());}
                count+=1;
            }
        }Ok(count)
    }
}
