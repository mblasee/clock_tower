use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::time::{Duration, Instant};

/// Global, thread-safe registry initialized once.
/// Stores the start time, end time, and duration for each measured key.
pub static REGISTRY: Lazy<DashMap<String, (Instant, Instant, Duration)>> = Lazy::new(|| DashMap::new());

/// Records a new measurement in the registry.
///
/// # Arguments
///
/// * `name` - The name of the measurement.
/// * `start` - The instant when the measurement started.
/// * `end` - The instant when the measurement ended.
pub fn record(name: &str, start: Instant, end: Instant) {
    let elapsed = end.duration_since(start);
    REGISTRY.insert(name.to_string(), (start, end, elapsed));
}

/// Retrieves a measurement by name.
///
/// Returns a tuple containing `(start_time, end_time, duration)`.
///
/// # Arguments
///
/// * `name` - The name of the measurement to retrieve.
pub fn get(name: &str) -> Option<(Instant, Instant, Duration)> {
    REGISTRY.get(name).map(|v| *v)
}

/// Retrieves all recorded measurements.
///
/// Returns a vector of tuples, where each tuple contains:
/// `(name, (start_time, end_time, duration))`
pub fn all() -> Vec<(String, (Instant, Instant, Duration))> {
    REGISTRY
        .iter()
        .map(|e| (e.key().clone(), *e.value()))
        .collect()
}

/// Clears all recorded measurements from the registry.
pub fn clear() {
    REGISTRY.clear();
}