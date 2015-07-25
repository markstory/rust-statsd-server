use std::collections::HashMap;
use super::metric::{Metric, MetricKind};

/// Buckets are the primary internal storage type.
///
/// Each bucket contains a set of hashmaps containing
/// each set of metrics received by clients.
///
pub struct Buckets {
    counters: HashMap<String, f64>,
    bad_messages: usize,
}

impl Buckets {
    /// Create a new Buckets
    ///
    /// ```
    /// let bucket = Buckets::new();
    /// ```
    pub fn new() -> Buckets {
        Buckets {
            counters: HashMap::new(),
            bad_messages: 0,
        }
    }

    /// Adds a metric to the bucket storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use super::metric;
    /// use std::str::FromStr;
    ///
    /// let metric = metric::Metric::FromStr("foo:1|c");
    /// let mut bucket = Buckets::new();
    /// bucket.add(metric);
    /// ```
    pub fn add(&mut self, value: &Metric) {
        println!("{:?}", value);
        match value.kind {
            MetricKind::Counter(rate) => {
                // TODO handle sampling rate
                self.counters.insert(value.name.to_owned(), value.value);
            },
            _ => panic!("Not done yet!")
        }
    }

    /// Increment the bad message count by one.
    ///
    pub fn add_bad_message(&mut self) {
        self.bad_messages += 1
    }
}
