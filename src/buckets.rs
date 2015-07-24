use std::collections::HashMap;

/// Buckets are the primary internal storage type.
///
/// Each bucket contains a set of hashmaps containing
/// each set of metrics received by clients.
///
pub struct Buckets {
    counters: HashMap<String, f64>,
}
