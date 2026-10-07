//! Optional copies of actual geometry operands on the executing thread.
//!
//! The collector neither evaluates geometry nor supplies calculation inputs.
//! Its context belongs to the existing Rust execution scopes, not EnergyPlus.

use crate::psychrometrics::production_trace::{ExecutionContext, current_execution_scope};
use ep_model::{SurfaceId, ZoneId};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    panic::Location,
};

const EVENT_LIMIT: usize = 2_000_000;
const UNIQUE_LIMIT: usize = 100_000;

thread_local! {
    static ACTIVE_GEOMETRY: RefCell<Option<Buffer>> = RefCell::default();
}

/// Actual Rust consumer receiving the recorded operands.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GeometryConsumer {
    /// Surface outdoor dry/wet temperature height correction.
    OutdoorAirTemperature,
    /// Surface outside wind-height correction.
    OutsideWindSpeed,
    /// Outside convection orientation selection or coefficient calculation.
    OutsideConvection,
    /// Actual solar-incidence angle calculation.
    SolarIncidence,
    /// Actual heat-transfer area multiplication or normalization.
    SurfaceHeatTransfer,
}

impl GeometryConsumer {
    /// Stable label for the actual Rust consumer, without a source-stage claim.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::OutdoorAirTemperature => "outdoor_air_temperature_height",
            Self::OutsideWindSpeed => "outside_wind_speed_height",
            Self::OutsideConvection => "outside_convection_orientation",
            Self::SolarIncidence => "solar_incidence_orientation",
            Self::SurfaceHeatTransfer => "surface_heat_transfer_area",
        }
    }
}

/// Values copied from the production state and the actual consumer arguments.
#[derive(Clone, Copy, Debug)]
pub enum GeometryOperand {
    /// Stored centroid and the height actually passed to the height helper.
    CentroidHeight {
        /// Stored centroid in meters; no vertex averaging is performed here.
        centroid_m: [f64; 3],
        /// Actual height argument in meters.
        height_m: f64,
    },
    /// Stored degree values and radian operands actually used by the consumer.
    Orientation {
        /// Stored azimuth in degrees.
        azimuth_deg: f64,
        /// Stored tilt in degrees.
        tilt_deg: f64,
        /// Actual azimuth operand in radians.
        azimuth_rad: f64,
        /// Actual tilt operand in radians.
        tilt_rad: f64,
    },
    /// Degree state and the cosine actually passed to outside convection.
    ConvectionOrientation {
        /// Stored azimuth and actual degree argument.
        azimuth_deg: f64,
        /// Stored tilt in degrees.
        tilt_deg: f64,
        /// Actual cosine-tilt argument; no unused radian operand is invented.
        cos_tilt: f64,
    },
    /// Area actually used by the heat-transfer consumer.
    Area {
        /// Actual area operand in square meters.
        area_m2: f64,
    },
}

/// Exact IEEE encoding in the named operand's field order.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GeometryOperandBits {
    /// Centroid X/Y/Z followed by the actual height argument.
    CentroidHeight([u64; 4]),
    /// Azimuth/tilt degrees followed by azimuth/tilt radians.
    Orientation([u64; 4]),
    /// Azimuth/tilt degrees followed by the actual cosine-tilt argument.
    ConvectionOrientation([u64; 3]),
    /// Actual area argument.
    Area(u64),
}

impl GeometryOperand {
    fn bits(self) -> GeometryOperandBits {
        match self {
            Self::CentroidHeight {
                centroid_m,
                height_m,
            } => GeometryOperandBits::CentroidHeight(
                [centroid_m[0], centroid_m[1], centroid_m[2], height_m].map(f64::to_bits),
            ),
            Self::Orientation {
                azimuth_deg,
                tilt_deg,
                azimuth_rad,
                tilt_rad,
            } => GeometryOperandBits::Orientation(
                [azimuth_deg, tilt_deg, azimuth_rad, tilt_rad].map(f64::to_bits),
            ),
            Self::ConvectionOrientation {
                azimuth_deg,
                tilt_deg,
                cos_tilt,
            } => GeometryOperandBits::ConvectionOrientation(
                [azimuth_deg, tilt_deg, cos_tilt].map(f64::to_bits),
            ),
            Self::Area { area_m2 } => GeometryOperandBits::Area(area_m2.to_bits()),
        }
    }
}

/// Actual Rust call site captured with `track_caller`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GeometryCallSite {
    /// Rust source file.
    pub file: &'static str,
    /// One-based line.
    pub line: u32,
    /// One-based column.
    pub column: u32,
}

/// One distinct operand observation at its first actual occurrence.
#[derive(Clone, Debug)]
pub struct GeometryOperandCall {
    /// First occurrence; repeat order is retained in `ordered_ids`.
    pub sequence: u64,
    /// Actual Rust consumer.
    pub consumer: GeometryConsumer,
    /// Actual typed surface identity, never an EnergyPlus array index.
    pub surface_id: SurfaceId,
    /// Actual owning typed zone identity.
    pub zone_id: ZoneId,
    /// Actual active Rust phase, without an inferred native stage.
    pub phase: &'static str,
    /// Actual Rust source call site.
    pub caller: GeometryCallSite,
    /// Existing runtime scope or post-run validation cursor, if present.
    pub context: Option<ExecutionContext>,
    /// Already selected operands, copied without numerical evaluation.
    pub operand: GeometryOperand,
    /// Exact encoding of those same operands.
    pub operand_bits: GeometryOperandBits,
}

/// Bounded ordered prefix of actual collecting-thread operand observations.
#[derive(Clone, Debug)]
pub struct GeometryProductionTrace {
    /// Maximum stored event IDs.
    pub event_limit: usize,
    /// Maximum distinct observations.
    pub unique_tuple_limit: usize,
    /// All actual calls, including any omitted suffix.
    pub total_call_count: u64,
    /// Stored occurrences, including dictionary repetitions.
    pub retained_call_count: u64,
    /// Calls omitted after the first exhausted bound.
    pub omitted_call_count: u64,
    /// First exhausted bound, or `None` when the capture is complete.
    pub truncation_reason: Option<&'static str>,
    /// All actual call totals per consumer, including omitted calls.
    pub consumer_counts: BTreeMap<GeometryConsumer, u64>,
    /// Dictionary in first-appearance order.
    pub dictionary: Vec<GeometryOperandCall>,
    /// Zero-based dictionary IDs in actual occurrence order.
    pub ordered_ids: Vec<u32>,
}

#[derive(Eq, Hash, PartialEq)]
struct Key {
    consumer: GeometryConsumer,
    surface_id: SurfaceId,
    zone_id: ZoneId,
    phase: &'static str,
    caller: GeometryCallSite,
    context: Option<ExecutionContext>,
    bits: GeometryOperandBits,
}
struct Buffer {
    event_limit: usize,
    unique_limit: usize,
    total: u64,
    truncation: Option<&'static str>,
    counts: BTreeMap<GeometryConsumer, u64>,
    ids: HashMap<Key, u32>,
    dictionary: Vec<GeometryOperandCall>,
    ordered: Vec<u32>,
}
struct CaptureGuard(Option<Buffer>);
impl Drop for CaptureGuard {
    fn drop(&mut self) {
        ACTIVE_GEOMETRY.with_borrow_mut(|active| *active = self.0.take());
    }
}

/// Executes unchanged. Disabled captures allocate no observation storage.
/// Nested captures and unwinding restore their predecessor. Bound exhaustion
/// retains a contiguous prefix and continues counting the omitted suffix.
pub fn capture<R>(
    enabled: bool,
    execute: impl FnOnce() -> R,
) -> (R, Option<GeometryProductionTrace>) {
    if !enabled {
        return (execute(), None);
    }
    capture_with_limits(EVENT_LIMIT, UNIQUE_LIMIT, execute)
}

fn capture_with_limits<R>(
    event_limit: usize,
    unique_limit: usize,
    execute: impl FnOnce() -> R,
) -> (R, Option<GeometryProductionTrace>) {
    let previous = ACTIVE_GEOMETRY.with_borrow_mut(|active| {
        active.replace(Buffer {
            event_limit,
            unique_limit,
            total: 0,
            truncation: None,
            counts: BTreeMap::new(),
            ids: HashMap::new(),
            dictionary: Vec::new(),
            ordered: Vec::new(),
        })
    });
    let _guard = CaptureGuard(previous);
    let result = execute();
    let trace = ACTIVE_GEOMETRY.with_borrow_mut(|active| {
        active.take().map(|buffer| {
            let retained = buffer.ordered.len() as u64;
            GeometryProductionTrace {
                event_limit: buffer.event_limit,
                unique_tuple_limit: buffer.unique_limit,
                total_call_count: buffer.total,
                retained_call_count: retained,
                omitted_call_count: buffer.total - retained,
                truncation_reason: buffer.truncation,
                consumer_counts: buffer.counts,
                dictionary: buffer.dictionary,
                ordered_ids: buffer.ordered,
            }
        })
    });
    (result, trace)
}

/// Copies the already-selected state/argument values only while enabled.
/// Callers must pass the actual consumer's operands rather than report-derived
/// recomputations. The collector observes one thread and supplies no inputs.
#[track_caller]
pub fn record(
    consumer: GeometryConsumer,
    surface_id: SurfaceId,
    zone_id: ZoneId,
    operand: GeometryOperand,
) {
    if !ACTIVE_GEOMETRY.with_borrow(Option::is_some) {
        return;
    }
    let location = Location::caller();
    let (phase, context) = current_execution_scope();
    ACTIVE_GEOMETRY.with_borrow_mut(|active| {
        let Some(buffer) = active.as_mut() else {
            return;
        };
        buffer.total += 1;
        *buffer.counts.entry(consumer).or_default() += 1;
        if buffer.truncation.is_some() {
            return;
        }
        if buffer.ordered.len() == buffer.event_limit {
            buffer.truncation = Some("event_limit");
            return;
        }
        let caller = GeometryCallSite {
            file: location.file(),
            line: location.line(),
            column: location.column(),
        };
        let bits = operand.bits();
        let key = Key {
            consumer,
            surface_id,
            zone_id,
            phase,
            caller: caller.clone(),
            context,
            bits,
        };
        let id = if let Some(&id) = buffer.ids.get(&key) {
            id
        } else {
            if buffer.dictionary.len() == buffer.unique_limit {
                buffer.truncation = Some("unique_tuple_limit");
                return;
            }
            let id = buffer.dictionary.len() as u32;
            buffer.dictionary.push(GeometryOperandCall {
                sequence: buffer.total,
                consumer,
                surface_id,
                zone_id,
                phase,
                caller,
                context,
                operand,
                operand_bits: bits,
            });
            buffer.ids.insert(key, id);
            id
        };
        buffer.ordered.push(id);
    });
}

#[cfg(test)]
mod tests;
