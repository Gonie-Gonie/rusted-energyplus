//! Immutable, indexed schedule samples prepared for one time axis.

use super::constant::constant_cached_schedule_series;
use super::day_table::{
    precompile_day_schedule_table, year_schedule_series_for_environment_time_axis,
    year_schedule_series_for_time_axis,
};
use super::external_interface::external_interface_cached_schedule_series_iter;
use super::file_shading::{
    file_shading_series_for_environment_time_axis, file_shading_series_for_time_axis,
};
use super::{
    ScheduleSeriesKind, ScheduleTrace, compact_schedule_series_for_environment_time_axis,
    compact_schedule_series_for_hours, compact_schedule_series_for_time_axis,
    file_schedule_series_for_environment_time_axis, file_schedule_series_for_time_axis,
};
use crate::time_axis::{EnvironmentTimeAxis, TimeAxis};
use ep_model::{ScheduleId, TypedModel};
use std::iter::FusedIterator;
use std::slice;

/// Precomputes an immutable indexed cache for hour-only schedule consumers.
///
/// Calendar-varying compact, file-backed, and annual schedules require a
/// TimeAxis and are rejected rather than evaluated against an implicit
/// calendar state.
pub fn precompute_schedule_cache(
    model: &TypedModel,
    sample_count: usize,
) -> Result<ScheduleSeriesCache, String> {
    let hours = (0..sample_count).map(|index| u32::try_from(index % 24 + 1).unwrap_or(24));
    precompute_schedule_cache_for_hours(model, hours)
}

fn precompute_schedule_cache_for_hours(
    model: &TypedModel,
    hours: impl IntoIterator<Item = u32> + Clone,
) -> Result<ScheduleSeriesCache, String> {
    if model.file_shading_schedule.is_some() {
        return Err(
            "Schedule:File:Shading requires a calendar- and zone-timestep-aware TimeAxis; the hour-only API has no annual source index or zone timestep"
                .to_string(),
        );
    }
    if !model.file_schedules.is_empty() {
        return Err(
            "Schedule:File requires a calendar-aware TimeAxis; the hour-only API has no annual source index"
                .to_string(),
        );
    }
    if !model.year_schedules.is_empty() {
        return Err(
            "Schedule:Year requires a calendar-aware TimeAxis; the hour-only API has no annual day or day-type state"
                .to_string(),
        );
    }
    let sample_count = hours.clone().into_iter().count();
    let constants = model
        .schedules
        .iter()
        .map(|schedule| constant_cached_schedule_series(schedule, sample_count));
    let compact = model
        .compact_schedules
        .iter()
        .map(|schedule| compact_schedule_series_for_hours(schedule, hours.clone()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(CachedScheduleSeries::from_dense_trace);
    let external = external_interface_cached_schedule_series_iter(model, sample_count);
    Ok(ScheduleSeriesCache::from_entries(
        sample_count,
        constants.chain(compact).chain(external),
    ))
}

/// Precomputes every supported typed schedule into an immutable indexed cache.
///
/// The cache follows the run-period TimeAxis and preserves the existing family
/// and source order. Constant and inactive external-interface schedules use
/// scalar storage; varying schedules use compact boxed arrays.
#[must_use]
pub fn precompute_schedule_cache_for_time_axis(
    model: &TypedModel,
    time_axis: &TimeAxis,
) -> ScheduleSeriesCache {
    let sample_count = time_axis.sample_count();
    let day_schedule_table =
        precompile_day_schedule_table(model, time_axis.zone_timestep.timesteps_per_hour);
    let entries = model
        .file_shading_schedule
        .as_ref()
        .into_iter()
        .flat_map(|schedule| file_shading_series_for_time_axis(schedule, time_axis))
        .map(CachedScheduleSeries::from_dense_trace)
        .chain(
            model
                .schedules
                .iter()
                .map(|schedule| constant_cached_schedule_series(schedule, sample_count)),
        )
        .chain(
            model
                .compact_schedules
                .iter()
                .map(|schedule| compact_schedule_series_for_time_axis(schedule, time_axis))
                .map(CachedScheduleSeries::from_dense_trace),
        )
        .chain(
            model
                .file_schedules
                .iter()
                .map(|schedule| file_schedule_series_for_time_axis(schedule, time_axis))
                .map(CachedScheduleSeries::from_dense_trace),
        )
        .chain(
            model
                .year_schedules
                .iter()
                .map(|schedule| {
                    year_schedule_series_for_time_axis(
                        model,
                        schedule,
                        time_axis,
                        &day_schedule_table,
                    )
                })
                .map(CachedScheduleSeries::from_dense_trace),
        )
        .chain(external_interface_cached_schedule_series_iter(
            model,
            sample_count,
        ));
    ScheduleSeriesCache::from_entries(sample_count, entries)
}

/// Precomputes every supported schedule at zone-timestep resolution.
///
/// The cache follows one EnvironmentTimeAxis while preserving family/source
/// order and using scalar storage for immutable constant-valued families.
#[must_use]
pub fn precompute_schedule_cache_for_environment_time_axis(
    model: &TypedModel,
    time_axis: &EnvironmentTimeAxis,
) -> ScheduleSeriesCache {
    let sample_count = time_axis.sample_count();
    let day_schedule_table =
        precompile_day_schedule_table(model, time_axis.zone_timestep.timesteps_per_hour);
    let entries = model
        .file_shading_schedule
        .as_ref()
        .into_iter()
        .flat_map(|schedule| file_shading_series_for_environment_time_axis(schedule, time_axis))
        .map(CachedScheduleSeries::from_dense_trace)
        .chain(
            model
                .schedules
                .iter()
                .map(|schedule| constant_cached_schedule_series(schedule, sample_count)),
        )
        .chain(
            model
                .compact_schedules
                .iter()
                .map(|schedule| {
                    compact_schedule_series_for_environment_time_axis(schedule, time_axis)
                })
                .map(CachedScheduleSeries::from_dense_trace),
        )
        .chain(
            model
                .file_schedules
                .iter()
                .map(|schedule| file_schedule_series_for_environment_time_axis(schedule, time_axis))
                .map(CachedScheduleSeries::from_dense_trace),
        )
        .chain(
            model
                .year_schedules
                .iter()
                .map(|schedule| {
                    year_schedule_series_for_environment_time_axis(
                        model,
                        schedule,
                        time_axis,
                        &day_schedule_table,
                    )
                })
                .map(CachedScheduleSeries::from_dense_trace),
        )
        .chain(external_interface_cached_schedule_series_iter(
            model,
            sample_count,
        ));
    ScheduleSeriesCache::from_entries(sample_count, entries)
}

/// Immutable storage for the logical samples of one schedule.
#[derive(Clone, Debug, PartialEq)]
pub enum ScheduleSampleStorage {
    /// One value repeated for every logical sample without allocating an array.
    Scalar {
        /// Repeated schedule value.
        value: f64,
        /// Number of logical samples represented by `value`.
        len: usize,
    },
    /// Materialized samples for a schedule that varies over the time axis.
    Dense(Box<[f64]>),
}

impl ScheduleSampleStorage {
    /// Returns the number of logical samples.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Scalar { len, .. } => *len,
            Self::Dense(values) => values.len(),
        }
    }

    /// Returns true when this storage represents no logical samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the value at `sample_index`.
    #[must_use]
    pub fn get(&self, sample_index: usize) -> Option<f64> {
        match self {
            Self::Scalar { value, len } => (sample_index < *len).then_some(*value),
            Self::Dense(values) => values.get(sample_index).copied(),
        }
    }

    /// Iterates over every logical sample without materializing scalar storage.
    pub fn iter(&self) -> ScheduleSampleIter<'_> {
        match self {
            Self::Scalar { value, len } => ScheduleSampleIter {
                inner: ScheduleSampleIterInner::Scalar {
                    value: *value,
                    remaining: *len,
                },
            },
            Self::Dense(values) => ScheduleSampleIter {
                inner: ScheduleSampleIterInner::Dense(values.iter()),
            },
        }
    }

    pub(super) fn into_vec(self) -> Vec<f64> {
        match self {
            Self::Scalar { value, len } => vec![value; len],
            Self::Dense(values) => values.into_vec(),
        }
    }
}

/// Iterator over logical schedule samples.
#[derive(Clone, Debug)]
pub struct ScheduleSampleIter<'a> {
    inner: ScheduleSampleIterInner<'a>,
}

#[derive(Clone, Debug)]
enum ScheduleSampleIterInner<'a> {
    Scalar { value: f64, remaining: usize },
    Dense(slice::Iter<'a, f64>),
}

impl Iterator for ScheduleSampleIter<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            ScheduleSampleIterInner::Scalar { value, remaining } => {
                if *remaining == 0 {
                    None
                } else {
                    *remaining -= 1;
                    Some(*value)
                }
            }
            ScheduleSampleIterInner::Dense(values) => values.next().copied(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for ScheduleSampleIter<'_> {
    fn len(&self) -> usize {
        match &self.inner {
            ScheduleSampleIterInner::Scalar { remaining, .. } => *remaining,
            ScheduleSampleIterInner::Dense(values) => values.len(),
        }
    }
}

impl FusedIterator for ScheduleSampleIter<'_> {}

/// One immutable schedule entry in a precomputed series cache.
#[derive(Clone, Debug, PartialEq)]
pub struct CachedScheduleSeries {
    /// Typed schedule ID.
    pub schedule_id: ScheduleId,
    /// EnergyPlus-normalized schedule name.
    pub schedule_name: String,
    /// Compile-time representation used to generate this entry.
    pub kind: ScheduleSeriesKind,
    /// Scalar or dense logical samples.
    pub samples: ScheduleSampleStorage,
}

impl CachedScheduleSeries {
    /// Returns the number of logical samples.
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns true when this entry has no logical samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Returns the value at `sample_index`.
    #[must_use]
    pub fn value(&self, sample_index: usize) -> Option<f64> {
        self.samples.get(sample_index)
    }

    /// Iterates over every logical sample.
    pub fn values(&self) -> ScheduleSampleIter<'_> {
        self.samples.iter()
    }

    pub(super) fn from_dense_trace(trace: ScheduleTrace) -> Self {
        Self {
            schedule_id: trace.schedule_id,
            schedule_name: trace.schedule_name,
            kind: trace.kind,
            samples: ScheduleSampleStorage::Dense(trace.values.into_boxed_slice()),
        }
    }

    fn into_trace(self) -> ScheduleTrace {
        ScheduleTrace {
            schedule_id: self.schedule_id,
            schedule_name: self.schedule_name,
            kind: self.kind,
            values: self.samples.into_vec(),
        }
    }
}

/// Lookup representation selected for a schedule cache.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleSeriesIndexKind {
    /// Source-order entries whose slot equals `ScheduleId`.
    DenseIdentity,
    /// Sparse, out-of-order, high-ID, or duplicate input indexed by sorted pairs.
    Sparse,
}

/// Deterministic allocation and indexing counters for one schedule cache.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduleCacheProfile {
    /// Number of scalar-backed series.
    pub scalar_series_count: usize,
    /// Number of dense-array-backed series.
    pub dense_series_count: usize,
    /// Sum of logical sample counts across all series.
    pub logical_sample_count: usize,
    /// Number of `f64` samples allocated by dense series.
    pub allocated_dense_sample_count: usize,
    /// Lookup representation selected for this cache.
    pub index_kind: ScheduleSeriesIndexKind,
    /// Number of distinct duplicate IDs resolved to their first source-order entry.
    pub ambiguous_id_count: usize,
}

/// Immutable schedule series and typed-ID lookup for one time axis.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleSeriesCache {
    sample_count: usize,
    series: Box<[CachedScheduleSeries]>,
    index: ScheduleSeriesIndex,
    profile: ScheduleCacheProfile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ScheduleSeriesIndex {
    DenseIdentity,
    Sparse {
        entries: Box<[(ScheduleId, usize)]>,
        ambiguous_id_count: usize,
    },
}

impl ScheduleSeriesCache {
    pub(super) fn from_entries(
        sample_count: usize,
        entries: impl IntoIterator<Item = CachedScheduleSeries>,
    ) -> Self {
        let series = entries.into_iter().collect::<Box<[_]>>();
        debug_assert!(series.iter().all(|entry| entry.len() == sample_count));
        let index = ScheduleSeriesIndex::new(&series);
        let index_kind = index.kind();
        let ambiguous_id_count = index.ambiguous_id_count();
        let mut scalar_series_count = 0;
        let mut dense_series_count = 0;
        let mut logical_sample_count: usize = 0;
        let mut allocated_dense_sample_count: usize = 0;
        for entry in &series {
            logical_sample_count = logical_sample_count.saturating_add(entry.len());
            match &entry.samples {
                ScheduleSampleStorage::Scalar { .. } => scalar_series_count += 1,
                ScheduleSampleStorage::Dense(values) => {
                    dense_series_count += 1;
                    allocated_dense_sample_count =
                        allocated_dense_sample_count.saturating_add(values.len());
                }
            }
        }
        let profile = ScheduleCacheProfile {
            scalar_series_count,
            dense_series_count,
            logical_sample_count,
            allocated_dense_sample_count,
            index_kind,
            ambiguous_id_count,
        };

        Self {
            sample_count,
            series,
            index,
            profile,
        }
    }

    /// Returns the logical sample count shared by all entries.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.sample_count
    }

    /// Returns the number of schedule entries in source order.
    #[must_use]
    pub fn len(&self) -> usize {
        self.series.len()
    }

    /// Returns true when the cache contains no schedule entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    /// Iterates over schedule entries in their original family/source order.
    pub fn iter(&self) -> slice::Iter<'_, CachedScheduleSeries> {
        self.series.iter()
    }

    /// Looks up a schedule entry by typed ID, resolving duplicates first-wins.
    #[must_use]
    pub fn get(&self, schedule_id: ScheduleId) -> Option<&CachedScheduleSeries> {
        let slot = self.index.slot(schedule_id)?;
        self.series
            .get(slot)
            .filter(|entry| entry.schedule_id == schedule_id)
    }

    /// Returns one logical schedule value by typed ID and sample index.
    #[must_use]
    pub fn value(&self, schedule_id: ScheduleId, sample_index: usize) -> Option<f64> {
        self.get(schedule_id)?.value(sample_index)
    }

    /// Returns deterministic storage and lookup counters.
    #[must_use]
    pub fn profile(&self) -> ScheduleCacheProfile {
        self.profile
    }

    /// Materializes legacy `Vec<f64>` traces in the original source order.
    #[must_use]
    pub fn into_traces(self) -> Vec<ScheduleTrace> {
        self.series
            .into_vec()
            .into_iter()
            .map(CachedScheduleSeries::into_trace)
            .collect()
    }
}

impl<'a> IntoIterator for &'a ScheduleSeriesCache {
    type Item = &'a CachedScheduleSeries;
    type IntoIter = slice::Iter<'a, CachedScheduleSeries>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl ScheduleSeriesIndex {
    fn new(series: &[CachedScheduleSeries]) -> Self {
        let dense_identity = series
            .iter()
            .enumerate()
            .all(|(slot, entry)| usize::try_from(entry.schedule_id.0).is_ok_and(|id| id == slot));
        if dense_identity {
            return Self::DenseIdentity;
        }

        let mut candidates = series
            .iter()
            .enumerate()
            .map(|(slot, entry)| (entry.schedule_id, slot))
            .collect::<Vec<_>>();
        candidates.sort_unstable_by_key(|(schedule_id, slot)| (*schedule_id, *slot));
        let mut entries = Vec::with_capacity(candidates.len());
        let mut ambiguous_id_count = 0;
        let mut cursor = 0;
        while cursor < candidates.len() {
            let schedule_id = candidates[cursor].0;
            let mut next = cursor + 1;
            while next < candidates.len() && candidates[next].0 == schedule_id {
                next += 1;
            }
            entries.push(candidates[cursor]);
            if next > cursor + 1 {
                ambiguous_id_count += 1;
            }
            cursor = next;
        }
        Self::Sparse {
            entries: entries.into_boxed_slice(),
            ambiguous_id_count,
        }
    }

    fn kind(&self) -> ScheduleSeriesIndexKind {
        match self {
            Self::DenseIdentity => ScheduleSeriesIndexKind::DenseIdentity,
            Self::Sparse { .. } => ScheduleSeriesIndexKind::Sparse,
        }
    }

    fn ambiguous_id_count(&self) -> usize {
        match self {
            Self::DenseIdentity => 0,
            Self::Sparse {
                ambiguous_id_count, ..
            } => *ambiguous_id_count,
        }
    }

    fn slot(&self, schedule_id: ScheduleId) -> Option<usize> {
        match self {
            Self::DenseIdentity => usize::try_from(schedule_id.0).ok(),
            Self::Sparse { entries, .. } => entries
                .binary_search_by_key(&schedule_id, |(entry_id, _slot)| *entry_id)
                .ok()
                .map(|index| entries[index].1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time_axis::{build_environment_time_axes, build_hourly_time_axis};
    use ep_model::{
        ExternalInterfaceFmuExportSchedule, ExternalInterfaceFmuImportSchedule,
        ExternalInterfaceSchedule, NormalizedName, ScheduleCompact, ScheduleCompactDayProfile,
        ScheduleCompactPeriod, ScheduleCompactSegment, ScheduleConstant, ScheduleDayType,
        ScheduleInterpolation,
    };

    fn scalar_schedule_cache_model() -> TypedModel {
        TypedModel {
            schedules: vec![ScheduleConstant {
                id: ScheduleId(0),
                name: NormalizedName::new("Constant Scalar"),
                schedule_type_limits: None,
                hourly_value: 0.25,
            }],
            external_interface_schedules: vec![ExternalInterfaceSchedule {
                id: ScheduleId(1),
                name: NormalizedName::new("External Scalar"),
                schedule_type_limits: None,
                initial_value: 0.5,
            }],
            external_interface_fmu_import_schedules: vec![ExternalInterfaceFmuImportSchedule {
                id: ScheduleId(2),
                name: NormalizedName::new("FMU Import Scalar"),
                schedule_type_limits: None,
                fmu_file_name: "unused.fmu".to_string(),
                fmu_instance_name: "unused-instance".to_string(),
                fmu_variable_name: "unused-output".to_string(),
                initial_value: 0.75,
            }],
            external_interface_fmu_export_schedules: vec![ExternalInterfaceFmuExportSchedule {
                id: ScheduleId(3),
                name: NormalizedName::new("FMU Export Scalar"),
                schedule_type_limits: None,
                fmu_variable_name: "unused-input".to_string(),
                initial_value: 1.0,
            }],
            ..TypedModel::default()
        }
    }

    fn mixed_schedule_cache_model() -> TypedModel {
        TypedModel {
            schedules: vec![ScheduleConstant {
                id: ScheduleId(0),
                name: NormalizedName::new("Constant Scalar"),
                schedule_type_limits: None,
                hourly_value: 0.5,
            }],
            compact_schedules: vec![ScheduleCompact {
                id: ScheduleId(1),
                name: NormalizedName::new("Hourly Aggregation Pending"),
                schedule_type_limits: None,
                periods: vec![ScheduleCompactPeriod {
                    through_schedule_day_of_year: 366,
                    day_profiles: vec![ScheduleCompactDayProfile {
                        day_types: vec![ScheduleDayType::Monday],
                        interpolation: ScheduleInterpolation::Average,
                        segments: vec![ScheduleCompactSegment {
                            until_minute_of_day: 1440,
                            value: 2.0,
                        }],
                    }],
                }],
            }],
            ..TypedModel::default()
        }
    }

    fn assert_legacy_values_equal(left: &[f64], right: &[f64]) {
        assert_eq!(left.len(), right.len());
        for (left, right) in left.iter().zip(right) {
            assert!(
                (left.is_nan() && right.is_nan()) || left.to_bits() == right.to_bits(),
                "schedule values differ: {left:?} != {right:?}"
            );
        }
    }

    #[test]
    fn four_scalar_schedule_families_allocate_no_dense_samples() {
        let model = scalar_schedule_cache_model();
        let cache = precompute_schedule_cache(&model, 7).expect("scalar cache should precompute");

        assert_eq!(cache.sample_count(), 7);
        assert_eq!(cache.len(), 4);
        assert_eq!(
            cache.profile(),
            ScheduleCacheProfile {
                scalar_series_count: 4,
                dense_series_count: 0,
                logical_sample_count: 28,
                allocated_dense_sample_count: 0,
                index_kind: ScheduleSeriesIndexKind::DenseIdentity,
                ambiguous_id_count: 0,
            }
        );
        for (schedule_id, expected) in [
            (ScheduleId(0), 0.25),
            (ScheduleId(1), 0.5),
            (ScheduleId(2), 0.75),
            (ScheduleId(3), 1.0),
        ] {
            let entry = cache.get(schedule_id).expect("scalar entry should exist");
            assert_eq!(entry.len(), 7);
            assert_eq!(entry.value(0), Some(expected));
            assert_eq!(entry.value(6), Some(expected));
            assert_eq!(entry.value(7), None);
            assert_eq!(entry.values().collect::<Vec<_>>(), vec![expected; 7]);
            match &entry.samples {
                ScheduleSampleStorage::Scalar { value, len } => {
                    assert_eq!(*value, expected);
                    assert_eq!(*len, 7);
                }
                ScheduleSampleStorage::Dense(_) => {
                    unreachable!("immutable scalar family must not allocate dense samples");
                }
            }
        }
    }

    #[test]
    fn environment_axis_cache_preserves_scalar_legacy_values_and_order() {
        let model = scalar_schedule_cache_model();
        let axes = build_environment_time_axes(&model).expect("default environment should build");
        let axis = axes.first().expect("default environment should exist");
        let cache = precompute_schedule_cache_for_environment_time_axis(&model, axis);
        let legacy =
            super::super::precompute_schedule_value_series_for_environment_time_axis(&model, axis);

        assert_eq!(cache.sample_count(), axis.sample_count());
        assert_eq!(cache.profile().scalar_series_count, 4);
        assert_eq!(cache.profile().dense_series_count, 0);
        assert_eq!(cache.profile().allocated_dense_sample_count, 0);
        assert_eq!(
            cache.profile().logical_sample_count,
            axis.sample_count() * cache.len()
        );
        for (entry, trace) in cache.iter().zip(&legacy) {
            assert_eq!(trace.schedule_id, entry.schedule_id);
            assert_eq!(trace.schedule_name, entry.schedule_name);
            assert_eq!(trace.kind, entry.kind);
            assert_legacy_values_equal(&trace.values, &entry.values().collect::<Vec<_>>());
        }
    }

    #[test]
    fn schedule_sample_iter_exact_size_decreases_for_scalar_and_dense_storage() {
        for storage in [
            ScheduleSampleStorage::Scalar { value: 3.0, len: 3 },
            ScheduleSampleStorage::Dense(vec![1.0, 2.0, 3.0].into_boxed_slice()),
        ] {
            let mut values = storage.iter();
            assert_eq!(values.len(), 3);
            assert!(values.next().is_some());
            assert_eq!(values.len(), 2);
            assert!(values.next().is_some());
            assert_eq!(values.len(), 1);
            assert!(values.next().is_some());
            assert_eq!(values.len(), 0);
            assert_eq!(values.next(), None);
            assert_eq!(values.len(), 0);
        }
    }

    #[test]
    fn mixed_cache_uses_scalar_and_dense_storage_with_legacy_equivalence() {
        let model = mixed_schedule_cache_model();
        let axis = build_hourly_time_axis(&model).expect("default one-day axis should build");
        let cache = precompute_schedule_cache_for_time_axis(&model, &axis);

        assert_eq!(
            cache.profile(),
            ScheduleCacheProfile {
                scalar_series_count: 1,
                dense_series_count: 1,
                logical_sample_count: axis.sample_count() * 2,
                allocated_dense_sample_count: axis.sample_count(),
                index_kind: ScheduleSeriesIndexKind::DenseIdentity,
                ambiguous_id_count: 0,
            }
        );
        let scalar = cache.get(ScheduleId(0)).expect("constant should exist");
        assert!(matches!(
            &scalar.samples,
            ScheduleSampleStorage::Scalar { .. }
        ));
        let dense = cache.get(ScheduleId(1)).expect("compact should exist");
        match &dense.samples {
            ScheduleSampleStorage::Dense(values) => {
                assert_eq!(values.len(), axis.sample_count());
                assert!(values.iter().all(|value| value.is_nan()));
            }
            ScheduleSampleStorage::Scalar { .. } => {
                unreachable!("varying compact schedule must use dense storage");
            }
        }
        assert_eq!(
            cache
                .iter()
                .map(|entry| entry.schedule_id)
                .collect::<Vec<_>>(),
            vec![ScheduleId(0), ScheduleId(1)]
        );

        let legacy = super::super::precompute_schedule_value_series_for_time_axis(&model, &axis);
        assert_eq!(legacy.len(), cache.len());
        for (entry, trace) in cache.iter().zip(&legacy) {
            assert_eq!(trace.schedule_id, entry.schedule_id);
            assert_eq!(trace.schedule_name, entry.schedule_name);
            assert_eq!(trace.kind, entry.kind);
            assert_legacy_values_equal(&trace.values, &entry.values().collect::<Vec<_>>());
        }
    }

    #[test]
    fn sparse_out_of_order_high_ids_use_bounded_binary_search_index() {
        let model = TypedModel {
            schedules: vec![
                ScheduleConstant {
                    id: ScheduleId(u32::MAX),
                    name: NormalizedName::new("High ID First"),
                    schedule_type_limits: None,
                    hourly_value: 9.0,
                },
                ScheduleConstant {
                    id: ScheduleId(7),
                    name: NormalizedName::new("Low ID Second"),
                    schedule_type_limits: None,
                    hourly_value: 7.0,
                },
            ],
            ..TypedModel::default()
        };
        let cache = precompute_schedule_cache(&model, 2).expect("sparse cache should precompute");

        assert_eq!(cache.profile().index_kind, ScheduleSeriesIndexKind::Sparse);
        assert_eq!(cache.profile().ambiguous_id_count, 0);
        assert_eq!(cache.value(ScheduleId(u32::MAX), 1), Some(9.0));
        assert_eq!(cache.value(ScheduleId(7), 0), Some(7.0));
        assert!(cache.get(ScheduleId(0)).is_none());
        assert_eq!(
            cache
                .iter()
                .map(|entry| entry.schedule_id)
                .collect::<Vec<_>>(),
            vec![ScheduleId(u32::MAX), ScheduleId(7)]
        );
    }

    #[test]
    fn duplicate_ids_within_family_preserve_legacy_first_wins_lookup() {
        let model = TypedModel {
            schedules: vec![
                ScheduleConstant {
                    id: ScheduleId(5),
                    name: NormalizedName::new("First Duplicate"),
                    schedule_type_limits: None,
                    hourly_value: 1.0,
                },
                ScheduleConstant {
                    id: ScheduleId(5),
                    name: NormalizedName::new("Second Duplicate"),
                    schedule_type_limits: None,
                    hourly_value: 2.0,
                },
            ],
            ..TypedModel::default()
        };
        let cache =
            precompute_schedule_cache(&model, 2).expect("duplicate cache should precompute");

        assert_eq!(cache.profile().index_kind, ScheduleSeriesIndexKind::Sparse);
        assert_eq!(cache.profile().ambiguous_id_count, 1);
        assert_eq!(
            cache
                .get(ScheduleId(5))
                .map(|entry| entry.schedule_name.as_str()),
            Some("FIRST DUPLICATE")
        );
        assert_eq!(cache.value(ScheduleId(5), 1), Some(1.0));
        assert_eq!(cache.into_traces()[1].values, vec![2.0; 2]);
    }

    #[test]
    fn duplicate_ids_across_families_preserve_family_order_first_wins_lookup() {
        let model = TypedModel {
            schedules: vec![ScheduleConstant {
                id: ScheduleId(11),
                name: NormalizedName::new("Constant First"),
                schedule_type_limits: None,
                hourly_value: 3.0,
            }],
            external_interface_schedules: vec![ExternalInterfaceSchedule {
                id: ScheduleId(11),
                name: NormalizedName::new("External Second"),
                schedule_type_limits: None,
                initial_value: 4.0,
            }],
            ..TypedModel::default()
        };
        let cache =
            precompute_schedule_cache(&model, 2).expect("duplicate cache should precompute");

        assert_eq!(cache.profile().ambiguous_id_count, 1);
        let first = cache
            .get(ScheduleId(11))
            .expect("first duplicate should resolve");
        assert_eq!(first.schedule_name, "CONSTANT FIRST");
        assert_eq!(
            first.kind,
            ScheduleSeriesKind::ConstantScalar { value: 3.0 }
        );
        assert_eq!(cache.value(ScheduleId(11), 0), Some(3.0));
        assert_eq!(
            cache
                .iter()
                .map(|entry| entry.schedule_name.as_str())
                .collect::<Vec<_>>(),
            vec!["CONSTANT FIRST", "EXTERNAL SECOND"]
        );
    }
}
