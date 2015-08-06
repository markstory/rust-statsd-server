use super::buckets::Buckets;
use time;

/// Creates derived values from metric data.
///
/// Creates:
///
/// - timer percentile data.
/// - internal processing metrics
pub fn process(buckets: &mut Buckets) {
    let start_time = time::get_time();

    for (key, values) in buckets.timers().iter() {
    }

    let end_time = time::get_time();
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::metric::{Metric, MetricKind};
    use time;

    #[test]
    fn test_process_timer_data() {
    }

    #[test]
    fn test_set_internal_metrics() {
    }
}
