//! Minimal owner adapters; all SUR-02 arithmetic comes from the unchanged real module.
#![allow(dead_code)]
use std::{fmt::Write, fs};
mod heat_balance {
    pub mod selected_heat_balance_globals {
        #[derive(Default)]
        pub struct SelectedHeatBalanceGlobals { pub num_of_zones: i32 }
    }
    pub mod selected_heat_balance_input {
        #[derive(Debug)]
        pub enum HeatBalanceInputError { Unavailable(&'static str) }
        #[derive(Clone, Copy, PartialEq)]
        pub enum SelectedHeatTransferModel { Ctf = 0, Other = 2 }
        pub struct ConstructionUsage { pub construction_id: Option<i32> }
        pub struct SelectedHeatBalanceInputState { pub construction_usage: Vec<ConstructionUsage> }
        pub mod surface {
            use super::SelectedHeatTransferModel;
            pub struct Row { pub heat_transfer_algorithm: SelectedHeatTransferModel, pub construction: i32 }
            pub struct FinalSurface { pub row: Row, pub typed_id: Option<i32> }
            pub struct SurfaceGeometryInputState { pub final_surfaces: Option<Vec<FinalSurface>> }
        }
        pub mod zone {
            pub struct Zone { pub space_indexes: Vec<i32> }
            pub struct Space { pub opaque_or_intmass_first: i32, pub opaque_or_intmass_last: i32, pub ht_surface_first: i32, pub ht_surface_last: i32 }
            pub struct SelectedZoneInputOwners { pub zones: Option<Vec<Zone>>, pub spaces: Option<Vec<Space>> }
        }
    }
    pub mod ctf_public_initialization {
        pub struct Storage { pub num_ctf_terms: i32, pub outside: [f64;19], pub cross: [f64;19], pub inside: [f64;19], pub flux: [f64;19] }
        pub struct Returned { pub storage: Storage }
        pub struct Construction { pub construction_id: i32, pub result: Result<Returned, ()> }
        pub struct OrderedCtfPublicInitialization { pub constructions: Vec<Construction> }
    }
    pub mod ctf_thermal_history {
        pub struct Normal { pub outside_temperature: [Vec<f64>;19], pub inside_temperature: [Vec<f64>;19], pub outside_flux: [Vec<f64>;19], pub inside_flux: [Vec<f64>;19] }
        pub struct CtfThermalHistoryState { pub surface_ids: Vec<i32>, pub normal: Normal }
    }
    pub mod ordinary_surface_initialization {
        #[derive(Default)]
        pub struct HvacFlux { pub ht_rad_sys: Option<f64>, pub hw_baseboard: Option<f64>, pub steam_baseboard: Option<f64>, pub elec_baseboard: Option<f64>, pub cooling_panel: Option<f64> }
        #[derive(Default)]
        pub struct SourceSurfaceHeatBalanceArrays {
            pub surf_ctf_const_in_part: Option<Vec<f64>>, pub surf_ctf_const_out_part: Option<Vec<f64>>,
            pub rad_sys_ti_hb_const_coef: Option<Vec<f64>>, pub rad_sys_ti_hb_tout_coef: Option<Vec<f64>>, pub rad_sys_ti_hb_qsrc_coef: Option<Vec<f64>>,
            pub rad_sys_to_hb_const_coef: Option<Vec<f64>>, pub rad_sys_to_hb_tin_coef: Option<Vec<f64>>, pub rad_sys_to_hb_qsrc_coef: Option<Vec<f64>>,
            pub q_rad_sys_source: Option<Vec<f64>>, pub qpv_sys_source: Option<Vec<f64>>, pub q_pool_surf_numerator: Option<Vec<f64>>, pub pool_heat_trans_coefs: Option<Vec<f64>>,
            pub surf_q_rad_from_hvac: Option<Vec<HvacFlux>>,
        }
        mod prefix {
            use crate::heat_balance::selected_heat_balance_input::{HeatBalanceInputError as Error,zone::SelectedZoneInputOwners};
/*__PRODUCTION_PREFIX__*/
        }
        #[path = "__ACTUAL_SUFFIX_PATH__"]
        mod actual_suffix;
    }
}
use heat_balance::{ctf_public_initialization as c,ctf_thermal_history as h,selected_heat_balance_input as i};
use heat_balance::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
struct Tokens<'a>(std::str::SplitWhitespace<'a>);
impl<'a> Tokens<'a> {
    fn word(&mut self) -> &'a str { self.0.next().expect("input token") }
    fn token(&mut self, value: &str) { assert_eq!(self.word(),value,"input token"); }
    fn integer(&mut self) -> i32 { self.word().parse().expect("signed integer") }
    fn count(&mut self) -> usize { usize::try_from(self.integer()).expect("nonnegative count") }
    fn real(&mut self) -> f64 {
        let s=self.word(); assert!(s.len()==16 && s.bytes().all(|c|c.is_ascii_digit() || (b'a'..=b'f').contains(&c)));
        let v=f64::from_bits(u64::from_str_radix(s,16).expect("raw16")); assert!(v.is_finite()); v
    }
}
struct Owners {
    globals: SelectedHeatBalanceGlobals,
    zones: i::zone::SelectedZoneInputOwners,
    geometry: i::surface::SurfaceGeometryInputState,
    control: i::SelectedHeatBalanceInputState,
    public: c::OrderedCtfPublicInitialization,
    history: h::CtfThermalHistoryState,
    arrays: SourceSurfaceHeatBalanceArrays,
}
fn outputs(a:&SourceSurfaceHeatBalanceArrays)->String {
    let mut s=String::new();
    for (x,y) in a.surf_ctf_const_in_part.as_ref().unwrap().iter().zip(a.surf_ctf_const_out_part.as_ref().unwrap()) { write!(s," {:016x} {:016x}",x.to_bits(),y.to_bits()).unwrap(); }
    s
}
fn readset(o:&Owners)->String {
    let mut s=o.globals.num_of_zones.to_string();
    for z in o.zones.zones.as_ref().unwrap() { write!(s," {}",z.space_indexes.len()).unwrap(); for n in &z.space_indexes {write!(s," {n}").unwrap();} }
    let spaces=o.zones.spaces.as_ref().unwrap();write!(s," {}",spaces.len()).unwrap();
    for p in spaces { write!(s," {} {}",p.opaque_or_intmass_first,p.opaque_or_intmass_last).unwrap(); }
    let surfaces=o.geometry.final_surfaces.as_ref().unwrap();write!(s," {}",surfaces.len()).unwrap();
    for sf in surfaces {write!(s," {} {} {}",sf.typed_id.unwrap(),sf.row.heat_transfer_algorithm as i32,sf.row.construction).unwrap();}
    write!(s," {}",o.public.constructions.len()).unwrap();
    for row in &o.public.constructions {let c=&row.result.as_ref().unwrap().storage;write!(s," {}",c.num_ctf_terms).unwrap();for a in [&c.outside,&c.cross,&c.inside,&c.flux] {for x in a {write!(s," {:016x}",x.to_bits()).unwrap();}}}
    write!(s," {}",o.history.surface_ids.len()).unwrap();for n in &o.history.surface_ids {write!(s," {n}").unwrap();}
    for a in [&o.history.normal.outside_temperature,&o.history.normal.inside_temperature,&o.history.normal.outside_flux,&o.history.normal.inside_flux] {
        for row in a {for sf in surfaces {let pos=o.history.surface_ids.iter().position(|x|Some(*x)==sf.typed_id).unwrap();write!(s," {:016x}",row[pos].to_bits()).unwrap();}}
    }
    // The actual method additionally selects these typed construction identities.
    assert!(o.control.construction_usage.iter().enumerate().all(|(j,x)|x.construction_id==Some((j+1) as i32)));
    assert!(o.public.constructions.iter().enumerate().all(|(j,x)|x.construction_id==(j+1) as i32));
    s
}
fn addresses(o:&Owners)->Vec<usize> {
    let mut p=vec![o as *const _ as usize,o.zones.zones.as_ref().unwrap().as_ptr() as usize,o.zones.spaces.as_ref().unwrap().as_ptr() as usize,
        o.geometry.final_surfaces.as_ref().unwrap().as_ptr() as usize,o.public.constructions.as_ptr() as usize,
        o.arrays.surf_ctf_const_in_part.as_ref().unwrap().as_ptr() as usize,o.arrays.surf_ctf_const_out_part.as_ref().unwrap().as_ptr() as usize];
    for a in [&o.history.normal.outside_temperature,&o.history.normal.inside_temperature,&o.history.normal.outside_flux,&o.history.normal.inside_flux] {for r in a {p.push(r.as_ptr() as usize);}}
    p
}
fn main() {
    let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),2);
    let raw=fs::read_to_string(&args[1]).expect("shared input");let mut t=Tokens(raw.split_whitespace());
    t.token("SUR02_INPUT_V1");t.token("LIFETIMES");let lives=t.count();assert!(lives>0&&lives<100);
    for _ in 0..lives {
        t.token("LIFE");let name=t.word();let nz=t.count();let nsp=t.count();let ns=t.count();let nc=t.count();let nh=t.count();let calls=t.count();
        assert!(nz>0&&nsp>0&&ns>0&&nc>0&&ns==nh&&ns<=8&&calls>0&&calls<100);
        let mut zones=Vec::new();for _ in 0..nz {t.token("ZONE");let n=t.count();assert!(n>0&&n<=nsp);zones.push(i::zone::Zone{space_indexes:(0..n).map(|_|t.integer()).collect()});}
        let mut spaces=Vec::new();for _ in 0..nsp {t.token("SPACE");let first=t.integer();let last=t.integer();spaces.push(i::zone::Space{opaque_or_intmass_first:first,opaque_or_intmass_last:last,ht_surface_first:first,ht_surface_last:last});}
        let mut surfaces=Vec::new();for _ in 0..ns {t.token("SURFACE");let id=t.integer();let alg=match t.integer(){0=>i::SelectedHeatTransferModel::Ctf,2=>i::SelectedHeatTransferModel::Other,_=>panic!("bounded selector")};let construction=t.integer();surfaces.push(i::surface::FinalSurface{typed_id:Some(id),row:i::surface::Row{heat_transfer_algorithm:alg,construction}});}
        let mut constructions=Vec::new();for n in 0..nc {t.token("CONSTRUCTION");let terms=t.integer();assert!((0..=18).contains(&terms));let mut arrays=[[0.0;19];4];for a in &mut arrays {for x in a {*x=t.real();}}constructions.push(c::Construction{construction_id:(n+1) as i32,result:Ok(c::Returned{storage:c::Storage{num_ctf_terms:terms,outside:arrays[0],cross:arrays[1],inside:arrays[2],flux:arrays[3]}})});}
        t.token("HISTORY_IDS");let ids:Vec<_>=(0..nh).map(|_|t.integer()).collect();
        assert!(surfaces.iter().all(|sf|ids.iter().filter(|x|Some(**x)==sf.typed_id).count()==1));
        t.token("INITIAL");let mut inside=Vec::new();let mut outside=Vec::new();for _ in 0..ns {inside.push(t.real());outside.push(t.real());}
        let empty=||std::array::from_fn(|_|vec![0.0;nh]);
        let mut o=Box::new(Owners{globals:SelectedHeatBalanceGlobals{num_of_zones:nz as i32},zones:i::zone::SelectedZoneInputOwners{zones:Some(zones),spaces:Some(spaces)},
            geometry:i::surface::SurfaceGeometryInputState{final_surfaces:Some(surfaces)},control:i::SelectedHeatBalanceInputState{construction_usage:(0..nc).map(|n|i::ConstructionUsage{construction_id:Some((n+1) as i32)}).collect()},
            public:c::OrderedCtfPublicInitialization{constructions},history:h::CtfThermalHistoryState{surface_ids:ids,normal:h::Normal{outside_temperature:empty(),inside_temperature:empty(),outside_flux:empty(),inside_flux:empty()}},
            arrays:SourceSurfaceHeatBalanceArrays{surf_ctf_const_in_part:Some(inside),surf_ctf_const_out_part:Some(outside),..Default::default()}});
        let initial=addresses(&o);
        for call in 0..calls {
            t.token("CALL");let call_name=t.word();
            for a in [&mut o.history.normal.outside_temperature,&mut o.history.normal.inside_temperature,&mut o.history.normal.outside_flux,&mut o.history.normal.inside_flux] {
                for row in a {for sf in o.geometry.final_surfaces.as_ref().unwrap() {let pos=o.history.surface_ids.iter().position(|x|Some(*x)==sf.typed_id).unwrap();row[pos]=t.real();}}
            }
            println!("CASE {name} CALL {call_name} INDEX {call}");println!("BEFORE{}",outputs(&o.arrays));let before=readset(&o);println!("READSET_BEFORE {before}");
            o.arrays.compute_source_ctf_constants(&o.geometry,&o.zones,&o.control,&o.public,&o.history,&o.globals).expect("actual current Source method");
            println!("AFTER{}",outputs(&o.arrays));let after=readset(&o);println!("READSET_AFTER {after}");assert_eq!(before,after,"Source primary inputs unchanged");assert_eq!(initial,addresses(&o),"Source adapter owner/storage stable");println!("OWNER_STORAGE_STABLE 1");
        }
    }
    t.token("END");assert!(t.0.next().is_none(),"no trailing input");
}
