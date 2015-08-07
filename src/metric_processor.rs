use super::buckets::Buckets;
use super::metric::{Metric, MetricKind};
use std::collections::HashMap;
use std::cmp::Ordering;
use time;

/// Creates derived values from metric data.
///
/// Creates:
///
/// - timer percentile data.
/// - internal processing metrics
pub fn process(buckets: &mut Buckets) {
    let start_time = time::get_time();

    let mut timer_data = HashMap::new();

    // Add the various derived values for timers.
    for (key, values) in buckets.timers().iter() {
        let mut v = values.clone();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());

        timer_data.insert(format!("{}.min", key), v[0]);
        timer_data.insert(format!("{}.max", key), v[v.len() - 1]);
    }
    buckets.set_timer_data(timer_data);

    let duration = time::get_time() - start_time;
    let process_duration = Metric::new(
        "statsd.processing_time",
        duration.num_milliseconds() as f64,
        MetricKind::Counter(1.0));
    buckets.add(&process_duration);
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::buckets::Buckets;
    use super::super::metric::{Metric, MetricKind};
    use time;

    fn make_buckets() -> Buckets {
        let mut buckets = Buckets::new();

        let metrics = [
            Metric::new("some.timer", 13.1, MetricKind::Timer),
            Metric::new("some.timer", 33.7, MetricKind::Timer),
            Metric::new("some.timer", 3.4, MetricKind::Timer),
            Metric::new("some.timer", 12.1, MetricKind::Timer),
        ];
        for m in metrics.iter() {
            buckets.add(&m);
        }
        buckets
    }

    #[test]
    fn test_process_timer_data() {
        let mut buckets = make_buckets();
        process(&mut buckets);

        assert_eq!(Some(&3.4), buckets.timer_data().get("some.timer.min"));
        assert_eq!(Some(&33.7), buckets.timer_data().get("some.timer.max"));
    }

    #[test]
    fn test_set_internal_metrics() {
    }
}
