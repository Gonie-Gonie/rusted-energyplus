//! Construction::setArraysBasedOnMaxSolidWinLayers2071-2113, same Construction.
use super::HeatBalanceInputError as Error;

/// All four genuine BlindSolDfAbs constructor members are explicitly0.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SourceBlindSolarDiffuseAbsorptance {
    pub(crate) front: f64,
    pub(crate) front_ground: f64,
    pub(crate) front_sky: f64,
    pub(crate) back: f64,
}

/// None means constructor-unallocated, inner None means allocated-unwritten.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceConstructionWindowArrays {
    pub(crate) abs_diff: Option<Vec<Option<f64>>>,
    pub(crate) abs_diff_back: Option<Vec<Option<f64>>>,
    pub(crate) layer_slat_blind_df_abs: Option<Vec<[SourceBlindSolarDiffuseAbsorptance; 181]>>,
    pub(crate) abs_beam_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) abs_beam_back_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) t_bare_sol_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) t_bare_vis_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) rf_bare_sol_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) rf_bare_vis_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) rb_bare_sol_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) rb_bare_vis_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) af_bare_sol_coef: Option<Vec<[Option<f64>; 6]>>,
    pub(crate) ab_bare_sol_coef: Option<Vec<[Option<f64>; 6]>>,
}
impl SourceConstructionWindowArrays {
    pub(crate) fn allocate_source(&mut self, max_solid_window_layers: i32) -> Result<(), Error> {
        let count = usize::try_from(max_solid_window_layers)
            .map_err(|_| Error::Unavailable("actual preconstruction MaxSolidWinLayers"))?;
        self.abs_diff = Some(vec![None; count]);
        self.abs_diff_back = Some(vec![None; count]);
        self.layer_slat_blind_df_abs = Some(vec![
            [SourceBlindSolarDiffuseAbsorptance::default();
                181];
            count
        ]);
        self.abs_beam_coef = Some(vec![[None; 6]; count]);
        self.abs_beam_back_coef = Some(vec![[None; 6]; count]);
        self.t_bare_sol_coef = Some(vec![[None; 6]; count]);
        self.t_bare_vis_coef = Some(vec![[None; 6]; count]);
        self.rf_bare_sol_coef = Some(vec![[None; 6]; count]);
        self.rf_bare_vis_coef = Some(vec![[None; 6]; count]);
        self.rb_bare_sol_coef = Some(vec![[None; 6]; count]);
        self.rb_bare_vis_coef = Some(vec![[None; 6]; count]);
        self.af_bare_sol_coef = Some(vec![[None; 6]; count]);
        self.ab_bare_sol_coef = Some(vec![[None; 6]; count]);
        for layer in 0..count {
            self.abs_diff
                .as_mut()
                .ok_or(Error::Unavailable("AbsDiff allocation"))?[layer] = Some(0.0);
            self.abs_diff_back
                .as_mut()
                .ok_or(Error::Unavailable("AbsDiffBack allocation"))?[layer] = Some(0.0);
        }
        for layer in 0..count {
            for slat in 0..181 {
                let value = &mut self
                    .layer_slat_blind_df_abs
                    .as_mut()
                    .ok_or(Error::Unavailable("layerSlatBlindDfAbs allocation"))?[layer][slat];
                value.front = 0.0;
                value.front_ground = 0.0;
                value.front_ground = 0.0; // Authentic repeated AbsGnd write2097.
                value.back = 0.0;
            }
        }
        for layer in 0..count {
            self.abs_beam_coef
                .as_mut()
                .ok_or(Error::Unavailable("AbsBeamCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.abs_beam_back_coef
                .as_mut()
                .ok_or(Error::Unavailable("AbsBeamBackCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.t_bare_sol_coef
                .as_mut()
                .ok_or(Error::Unavailable("tBareSolCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.t_bare_vis_coef
                .as_mut()
                .ok_or(Error::Unavailable("tBareVisCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.rf_bare_sol_coef
                .as_mut()
                .ok_or(Error::Unavailable("rfBareSolCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.rf_bare_vis_coef
                .as_mut()
                .ok_or(Error::Unavailable("rfBareVisCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.rb_bare_sol_coef
                .as_mut()
                .ok_or(Error::Unavailable("rbBareSolCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.rb_bare_vis_coef
                .as_mut()
                .ok_or(Error::Unavailable("rbBareVisCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.af_bare_sol_coef
                .as_mut()
                .ok_or(Error::Unavailable("afBareSolCoef allocation"))?[layer]
                .fill(Some(0.0));
            self.ab_bare_sol_coef
                .as_mut()
                .ok_or(Error::Unavailable("abBareSolCoef allocation"))?[layer]
                .fill(Some(0.0));
        }
        Ok(())
    }
}
