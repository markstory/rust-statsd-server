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

        let len = v.len() as f64;
        let sum = v.iter().fold(0.0, |sum, x| sum + x);
        let mean = sum / len;

        let mid = (len / 2.0).floor() as usize;
        let median = if (v.len() % 2) == 0 {
            (v[mid - 1] + v[mid + 1]) / 2.0
        } else {
            v[mid]
        };

        timer_data.insert(format!("{}.min", key), v[0]);
        timer_data.insert(format!("{}.max", key), v[v.len() - 1]);
        timer_data.insert(format!("{}.count", key), len);
        timer_data.insert(format!("{}.mean", key), mean);
        timer_data.insert(format!("{}.median", key), median);
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
    use std::option::Option;
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

    fn assert_float(expected: &str, value: &f64) {
        assert_eq!(expected, format!("{:.*}", 3, value));
    }

    #[test]
    fn test_process_timer_data() {
        let mut buckets = make_buckets();
        process(&mut buckets);

        assert_eq!(Some(&3.4), buckets.timer_data().get("some.timer.min"));
        assert_eq!(Some(&33.7), buckets.timer_data().get("some.timer.max"));
        assert_eq!(Some(&4.0), buckets.timer_data().get("some.timer.count"));
        assert_float(
            "15.575",
            buckets.timer_data().get("some.timer.mean").unwrap());
        assert_float(
            "22.900",
            buckets.timer_data().get("some.timer.median").unwrap());
    }

    #[test]
    fn test_set_internal_metrics() {
    }
}
