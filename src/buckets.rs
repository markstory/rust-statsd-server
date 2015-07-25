use std::collections::HashMap;
use super::metric::{Metric, MetricKind};
use clock_ticks;

/// Buckets are the primary internal storage type.
///
/// Each bucket contains a set of hashmaps containing
/// each set of metrics received by clients.
///
pub struct Buckets {
    counters: HashMap<String, f64>,
    gauges: HashMap<String, f64>,
    timers: HashMap<String, f64>,

    server_start_time: u64,
    last_message: u64,
    bad_messages: usize,
    total_messages: usize,
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
            gauges: HashMap::new(),
            timers: HashMap::new(),
            bad_messages: 0,
            total_messages: 0,
            last_message: clock_ticks::precise_time_ms(),
            server_start_time: clock_ticks::precise_time_ms(),
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
            MetricKind::Gauge => {
                self.gauges.insert(value.name.to_owned(), value.value);
            },
            MetricKind::Timer => {
                self.timers.insert(value.name.to_owned(), value.value);
            },
        }
    }

    /// Increment the bad message count by one.
    ///
    pub fn add_bad_message(&mut self) {
        self.bad_messages += 1
    }

    /// Get the count of bad messages
    pub fn bad_messages(&self) -> usize {
        self.bad_messages
    }
}


//
// Tests
//
#[cfg(test)]
mod test {
    use super::*;
    use super::super::metric::{Metric, MetricKind};

    #[test]
    fn test_bad_messages() {
        let mut buckets = Buckets::new();
        buckets.add_bad_message();
        assert_eq!(1, buckets.bad_messages());

        buckets.add_bad_message();
        assert_eq!(2, buckets.bad_messages());
    }

    #[test]
    fn test_add_counter_metric() {
        assert!(false, "Not done");
    }

    #[test]
    fn test_add_counter_metric_sampled() {
        assert!(false, "Not done");
    }

    #[test]
    fn test_add_gauge_metric() {
        let mut buckets = Buckets::new();
        let metric = Metric::new("some.metric", 11.5, MetricKind::Gauge);
        buckets.add(&metric);
        assert!(buckets.gauges.contains_key("some.metric"),
                "Should contain the metric key");
        // TODO assert value in hashmap is a list.
        // TODO assert last_message time.
    }

    #[test]
    fn test_add_timer_metric() {
        let mut buckets = Buckets::new();
        let metric = Metric::new("some.metric", 11.5, MetricKind::Timer);
        buckets.add(&metric);
        assert!(buckets.timers.contains_key("some.metric"),
                "Should contain the metric key");
    }
}
