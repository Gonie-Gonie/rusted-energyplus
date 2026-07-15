//! Deterministic operation counts for heat-balance internal-gain schedule access.

/// Exact scope attached to the deterministic operation-count profile.
pub const HEAT_BALANCE_INTERNAL_GAIN_SCHEDULE_PROFILE_SCOPE: &str = "ep_runtime heat-balance simulation referenced-only OtherEquipment cache; excludes ep_cli full-axis precompute and public one-step fallback";

/// Schedule operations observed during one heat-balance execution phase.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InternalGainSchedulePhaseOperations {
    /// `ScheduleSeriesCache::value` calls for scheduled `OtherEquipment` objects.
    pub cached_value_lookup_count: usize,
    /// Live-model schedule lookups used by the explicit fallback/reference path.
    pub live_fallback_lookup_count: usize,
    /// Entries into the source-ordered live schedule-object-family chain.
    pub live_schedule_family_chain_scan_count: usize,
    /// Live Compact daily-profile resolutions after cache construction.
    pub compact_profile_resolution_count: usize,
    /// Live Compact segment-value evaluations after cache construction.
    pub compact_value_evaluation_count: usize,
}

impl InternalGainSchedulePhaseOperations {
    pub(crate) fn record_cached_value_lookup(&mut self) {
        self.cached_value_lookup_count = self.cached_value_lookup_count.saturating_add(1);
    }

    pub(crate) fn record_live_fallback_lookup(&mut self) {
        self.live_fallback_lookup_count = self.live_fallback_lookup_count.saturating_add(1);
        self.live_schedule_family_chain_scan_count =
            self.live_schedule_family_chain_scan_count.saturating_add(1);
    }

    pub(crate) fn record_compact_profile_resolution(&mut self) {
        self.compact_profile_resolution_count =
            self.compact_profile_resolution_count.saturating_add(1);
    }

    pub(crate) fn record_compact_value_evaluation(&mut self) {
        self.compact_value_evaluation_count = self.compact_value_evaluation_count.saturating_add(1);
    }
}

/// Simulation-owned profile for one referenced-only internal-gain schedule cache.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HeatBalanceInternalGainScheduleOperationProfile {
    /// Successful specialized cache constructions during this simulation.
    pub referenced_only_cache_build_count: usize,
    /// Number of distinct referenced schedule entries in the specialized cache.
    pub cache_entry_count: usize,
    /// Total logical values represented by all specialized cache entries.
    pub cache_logical_sample_count: usize,
    /// Compact value evaluations used only to materialize the specialized cache.
    pub cache_build_compact_value_evaluation_count: usize,
    /// Operations performed while initializing heat-balance state.
    pub initialization: InternalGainSchedulePhaseOperations,
    /// Operations performed during repeated warmup timesteps.
    pub warmup: InternalGainSchedulePhaseOperations,
    /// Operations performed during reported run-period timesteps.
    pub run_period: InternalGainSchedulePhaseOperations,
}

impl HeatBalanceInternalGainScheduleOperationProfile {
    pub(crate) const fn for_single_build(
        entry_count: usize,
        logical_sample_count: usize,
        compact_value_evaluation_count: usize,
    ) -> Self {
        Self {
            referenced_only_cache_build_count: 1,
            cache_entry_count: entry_count,
            cache_logical_sample_count: logical_sample_count,
            cache_build_compact_value_evaluation_count: compact_value_evaluation_count,
            initialization: InternalGainSchedulePhaseOperations {
                cached_value_lookup_count: 0,
                live_fallback_lookup_count: 0,
                live_schedule_family_chain_scan_count: 0,
                compact_profile_resolution_count: 0,
                compact_value_evaluation_count: 0,
            },
            warmup: InternalGainSchedulePhaseOperations {
                cached_value_lookup_count: 0,
                live_fallback_lookup_count: 0,
                live_schedule_family_chain_scan_count: 0,
                compact_profile_resolution_count: 0,
                compact_value_evaluation_count: 0,
            },
            run_period: InternalGainSchedulePhaseOperations {
                cached_value_lookup_count: 0,
                live_fallback_lookup_count: 0,
                live_schedule_family_chain_scan_count: 0,
                compact_profile_resolution_count: 0,
                compact_value_evaluation_count: 0,
            },
        }
    }

    /// Total typed cache lookups across initialization, warmup, and run period.
    #[must_use]
    pub fn total_cached_value_lookup_count(&self) -> usize {
        self.initialization
            .cached_value_lookup_count
            .saturating_add(self.warmup.cached_value_lookup_count)
            .saturating_add(self.run_period.cached_value_lookup_count)
    }

    /// Total live fallback lookups across the three simulation phases.
    #[must_use]
    pub fn total_live_fallback_lookup_count(&self) -> usize {
        self.initialization
            .live_fallback_lookup_count
            .saturating_add(self.warmup.live_fallback_lookup_count)
            .saturating_add(self.run_period.live_fallback_lookup_count)
    }

    /// Total live schedule-family-chain scans across the three simulation phases.
    #[must_use]
    pub fn total_live_schedule_family_chain_scan_count(&self) -> usize {
        self.initialization
            .live_schedule_family_chain_scan_count
            .saturating_add(self.warmup.live_schedule_family_chain_scan_count)
            .saturating_add(self.run_period.live_schedule_family_chain_scan_count)
    }

    /// Total Compact profile resolutions across the three simulation phases.
    #[must_use]
    pub fn total_compact_profile_resolution_count(&self) -> usize {
        self.initialization
            .compact_profile_resolution_count
            .saturating_add(self.warmup.compact_profile_resolution_count)
            .saturating_add(self.run_period.compact_profile_resolution_count)
    }

    /// Total Compact value evaluations across the three simulation phases.
    #[must_use]
    pub fn total_compact_value_evaluation_count(&self) -> usize {
        self.initialization
            .compact_value_evaluation_count
            .saturating_add(self.warmup.compact_value_evaluation_count)
            .saturating_add(self.run_period.compact_value_evaluation_count)
    }
}
