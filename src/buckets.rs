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
    timers: HashMap<String, Vec<f64>>,

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
        let name = value.name.to_owned();
        match value.kind {
            MetricKind::Counter(rate) => {
                let counter = self.counters.entry(name).or_insert(0.0);
                *counter = *counter + value.value * (1.0 / rate);
            },
            MetricKind::Gauge => {
                self.gauges.insert(name, value.value);
            },
            MetricKind::Timer => {
                let slot = self.timers.entry(name).or_insert(Vec::new());
                slot.push(value.value);
            },
        }
        self.last_message = clock_ticks::precise_time_ms();
        self.total_messages += 1;
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

    /// Get the counters as a borrowed reference.
    pub fn counters(&self) -> &HashMap<String, f64> {
        &self.counters
    }

    /// Get the gauges as a borrowed reference.
    pub fn gauges(&self) -> &HashMap<String, f64> {
        &self.gauges
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
    fn test_add_increments_total_messages() {
        let mut buckets = Buckets::new();
        // duff value to ensure it changes.
        let original = 10;
        buckets.last_message = original;

        let metric = Metric::new("some.metric", 1.0, MetricKind::Counter(1.0));
        buckets.add(&metric);
        assert!(buckets.last_message > original);
    }

    #[test]
    fn test_add_increments_last_message_timer() {
        let mut buckets = Buckets::new();
        let metric = Metric::new("some.metric", 1.0, MetricKind::Counter(1.0));
        buckets.add(&metric);
        assert_eq!(1, buckets.total_messages);

        buckets.add(&metric);
        assert_eq!(2, buckets.total_messages);
    }

    #[test]
    fn test_add_counter_metric() {
        let mut buckets = Buckets::new();
        let metric = Metric::new("some.metric", 1.0, MetricKind::Counter(1.0));
        buckets.add(&metric);

        assert!(buckets.counters.contains_key("some.metric"),
                "Should contain the metric key");
        assert_eq!(Some(&1.0), buckets.counters.get("some.metric"));

        // Increment counter
        buckets.add(&metric);
        assert_eq!(Some(&2.0), buckets.counters.get("some.metric"));
        assert_eq!(1, buckets.counters().len());
        assert_eq!(0, buckets.gauges().len());
    }

    #[test]
    fn test_add_counter_metric_sampled() {
        let mut buckets = Buckets::new();
        let metric = Metric::new("some.metric", 1.0, MetricKind::Counter(0.1));

        buckets.add(&metric);
        assert_eq!(Some(&10.0), buckets.counters.get("some.metric"));

        let metric_two = Metric::new("some.metric", 1.0, MetricKind::Counter(0.5));
        buckets.add(&metric_two);
        assert_eq!(Some(&12.0), buckets.counters.get("some.metric"));
    }

    #[test]
    fn test_add_gauge_metric() {
        let mut buckets = Buckets::new();
        let metric = Metric::new("some.metric", 11.5, MetricKind::Gauge);
        buckets.add(&metric);
        assert!(buckets.gauges.contains_key("some.metric"),
                "Should contain the metric key");
        assert_eq!(Some(&11.5), buckets.gauges.get("some.metric"));
        assert_eq!(1, buckets.gauges().len());
        assert_eq!(0, buckets.counters().len());
    }

    #[test]
    fn test_add_timer_metric() {
        let mut buckets = Buckets::new();
        let metric = Metric::new("some.metric", 11.5, MetricKind::Timer);
        buckets.add(&metric);
        assert!(buckets.timers.contains_key("some.metric"),
                "Should contain the metric key");
        assert_eq!(Some(&vec![11.5]), buckets.timers.get("some.metric"));

        let metric_two = Metric::new("some.metric", 99.5, MetricKind::Timer);
        buckets.add(&metric_two);

        let metric_three = Metric::new("other.metric", 811.5, MetricKind::Timer);
        buckets.add(&metric_three);
        assert!(buckets.timers.contains_key("some.metric"),
                "Should contain the metric key");
        assert!(buckets.timers.contains_key("other.metric"),
                "Should contain the metric key");

        assert_eq!(Some(&vec![11.5, 99.5]), buckets.timers.get("some.metric"));
        assert_eq!(Some(&vec![811.5]), buckets.timers.get("other.metric"));
    }
}
