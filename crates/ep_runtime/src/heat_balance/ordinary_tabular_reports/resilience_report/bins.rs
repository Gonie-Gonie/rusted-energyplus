//! Native12918-12982,13065-13135,13216-13295 local table helpers.
use super::values::date;
use super::{
    Columns, Error, Field, SourceResilienceTableOwners, SourceTabularReports, count, value,
    zone_name,
};
impl SourceTabularReports {
    pub(super) fn resilience_bins(
        &mut self,
        pass: usize,
        title: &[u8],
        field: Field,
        multiplier: f64,
        rows: &mut [Vec<u8>],
        cols: &mut Columns,
        o: &mut SourceResilienceTableOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.resilience_subtitle(pass, title)?;
        let n = cols.headings.len();
        let mut max = vec![0.0; n];
        let mut min = vec![0.0; n];
        let mut sum = vec![0.0; n];
        // Preserve Original index order and same-row checked accesses across the retained work arrays.
        #[allow(clippy::needless_range_loop)]
        for j in 0..n {
            min[j] = value(o.zones, 1, field, j)? * multiplier;
        }
        for zone in 1..=o.globals.num_of_zones {
            let i = count(zone)?;
            rows[i - 1] = zone_name(o.zones, zone)?.as_bytes().to_vec();
            for j in 0..n {
                let current = value(o.zones, zone, field, j)? * multiplier;
                if current > max[j] {
                    max[j] = current;
                }
                if current < min[j] {
                    min[j] = current;
                }
                sum[j] += current;
                cols.body
                    .set(j + 1, i, self.resilience_real(pass, current)?)?;
            }
        }
        let zones = count(o.globals.num_of_zones)?;
        rows[zones] = b"Min".to_vec();
        rows[zones + 1] = b"Max".to_vec();
        rows[zones + 2] = b"Average".to_vec();
        rows[zones + 3] = b"Sum".to_vec();
        for j in 0..n {
            cols.body
                .set(j + 1, zones + 1, self.resilience_real(pass, min[j])?)?;
            cols.body
                .set(j + 1, zones + 2, self.resilience_real(pass, max[j])?)?;
            cols.body.set(
                j + 1,
                zones + 3,
                self.resilience_real(pass, sum[j] / f64::from(o.globals.num_of_zones))?,
            )?;
            cols.body
                .set(j + 1, zones + 4, self.resilience_real(pass, sum[j])?)?;
        }
        self.resilience_emit(pass, title, rows, cols, o)
    }
    pub(super) fn resilience_safety(
        &mut self,
        pass: usize,
        title: &[u8],
        field: Field,
        rows: &mut [Vec<u8>],
        cols: &mut Columns,
        o: &mut SourceResilienceTableOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.resilience_subtitle(pass, title)?;
        let n = cols.headings.len();
        for zone in 1..=o.globals.num_of_zones {
            let i = count(zone)?;
            rows[i - 1] = zone_name(o.zones, zone)?.as_bytes().to_vec();
            for j in 1..=n {
                cols.body.set(
                    j,
                    i,
                    self.resilience_real(pass, value(o.zones, zone, field, j - 1)?)?,
                )?;
            }
            cols.body
                .set(2, i, date(value(o.zones, zone, field, 1)?)?)?;
        }
        let mut max = vec![0.0; n];
        let mut min = vec![0.0; n];
        let mut sum = vec![0.0; n];
        // Preserve Original index order and same-row checked accesses across the retained work arrays.
        #[allow(clippy::needless_range_loop)]
        for j in 0..n {
            min[j] = value(o.zones, 1, field, j)?;
        }
        for zone in 1..=o.globals.num_of_zones {
            for j in 0..n {
                let current = value(o.zones, zone, field, j)?;
                if current > max[j] {
                    max[j] = current;
                }
                if current < min[j] {
                    min[j] = current;
                }
                sum[j] += current;
            }
        }
        let zones = count(o.globals.num_of_zones)?;
        rows[zones] = b"Min".to_vec();
        rows[zones + 1] = b"Max".to_vec();
        rows[zones + 2] = b"Average".to_vec();
        rows[zones + 3] = b"Sum".to_vec();
        for j in 0..n {
            cols.body
                .set(j + 1, zones + 1, self.resilience_real(pass, min[j])?)?;
            cols.body
                .set(j + 1, zones + 2, self.resilience_real(pass, max[j])?)?;
            cols.body.set(
                j + 1,
                zones + 3,
                self.resilience_real(pass, sum[j] / f64::from(o.globals.num_of_zones))?,
            )?;
            cols.body
                .set(j + 1, zones + 4, self.resilience_real(pass, sum[j])?)?;
        }
        for i in 1..5 {
            cols.body.set(2, zones + i, b"-".to_vec())?;
        }
        self.resilience_emit(pass, title, rows, cols, o)
    }
    pub(super) fn resilience_set(
        &mut self,
        pass: usize,
        title: &[u8],
        field: Field,
        unit_multiplier: f64,
        rows: &mut [Vec<u8>],
        cols: &mut Columns,
        o: &mut SourceResilienceTableOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.resilience_subtitle(pass, title)?;
        let n = cols.headings.len();
        let mut max = vec![0.0; n - 1];
        let mut min = vec![0.0; n - 1];
        let mut sum = vec![0.0; n - 1];
        let mut multiplier = vec![unit_multiplier; n - 1];
        multiplier[n - 2] = 1.0;
        for j in 0..n - 1 {
            min[j] = value(o.zones, 1, field, j)? * multiplier[j];
        }
        for zone in 1..=o.globals.num_of_zones {
            let i = count(zone)?;
            rows[i - 1] = zone_name(o.zones, zone)?.as_bytes().to_vec();
            for j in 0..n - 1 {
                let current = value(o.zones, zone, field, j)? * multiplier[j];
                if current > max[j] {
                    max[j] = current;
                }
                if current < min[j] {
                    min[j] = current;
                }
                sum[j] += current;
                cols.body
                    .set(j + 1, i, self.resilience_real(pass, current)?)?;
            }
            cols.body
                .set(n, i, date(value(o.zones, zone, field, n - 1)?)?)?;
        }
        let zones = count(o.globals.num_of_zones)?;
        rows[zones] = b"Min".to_vec();
        rows[zones + 1] = b"Max".to_vec();
        rows[zones + 2] = b"Average".to_vec();
        for j in 0..n - 1 {
            cols.body
                .set(j + 1, zones + 1, self.resilience_real(pass, min[j])?)?;
            cols.body
                .set(j + 1, zones + 2, self.resilience_real(pass, max[j])?)?;
            cols.body.set(
                j + 1,
                zones + 3,
                self.resilience_real(pass, sum[j] / f64::from(o.globals.num_of_zones))?,
            )?;
        }
        cols.body.set(n, zones + 1, b"-".to_vec())?;
        cols.body.set(n, zones + 2, b"-".to_vec())?;
        cols.body.set(n, zones + 3, b"-".to_vec())?;
        self.resilience_emit(pass, title, rows, cols, o)
    }
}
