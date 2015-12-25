use super::super::backend::Backend;
use super::super::buckets::Buckets;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::fmt::Write;
use std::io::Write as IoWrite;
use time;


#[derive(Debug)]
pub struct Graphite {
    addr: SocketAddrV4,
    last_flush_time: u64,
    last_flush_length: u64,
}


impl Graphite {
    /// Create a Graphite formatter
    ///
    /// # Examples
    ///
    /// ```
    /// let graph = Graphite::new(host, port);
    /// ```
    pub fn new(host: &str, port: u16) -> Graphite {
        let ip = Ipv4Addr::from_str(&host).unwrap();
        let addr = SocketAddrV4::new(ip, port);
        Graphite {
            addr: addr,
            last_flush_time: 0,
            last_flush_length: 0,
        }
    }

    /// Convert the buckets into a String that
    /// can be sent to graphite's newline API
    pub fn format_stats(&self, buckets: &Buckets) -> String {
        let start = time::get_time().sec;
        let mut stats = String::new();

        write!(stats,
               "{} {} {}\n",
               "statsd.bad_messages",
               buckets.bad_messages(),
               start)
            .unwrap();
        write!(stats,
               "{} {} {}\n",
               "statsd.total_messages",
               buckets.total_messages(),
               start)
            .unwrap();

        for (key, value) in buckets.counters().iter() {
            write!(stats, "{} {} {} \n", key, value, start).unwrap();
        }

        for (key, value) in buckets.gauges().iter() {
            write!(stats, "{} {} {} \n", key, value, start).unwrap();
        }

        // The raw timer data is not sent to graphite.
        for (key, value) in buckets.timer_data().iter() {
            write!(stats, "{} {} {} \n", key, value, start).unwrap();
        }
        stats
    }
}


impl Backend for Graphite {
    fn flush_buckets(&mut self, buckets: &Buckets) {
        let stats = self.format_stats(&buckets);

        let mut stream = TcpStream::connect(self.addr).unwrap();
        let _ = stream.write(stats.as_bytes());
    }
}


#[cfg(test)]
mod test {
    use super::super::super::metric::{Metric, MetricKind};
    use super::super::super::buckets::Buckets;
    use super::super::super::metric_processor::process;
    use super::*;

    fn make_buckets() -> Buckets {
        let mut buckets = Buckets::new();
        let m1 = Metric::new("test.counter", 1.0, MetricKind::Counter(1.0));
        let m2 = Metric::new("test.gauge", 3.211, MetricKind::Gauge);

        let m3 = Metric::new("test.timer", 12.101, MetricKind::Timer);
        let m4 = Metric::new("test.timer", 1.101, MetricKind::Timer);
        let m5 = Metric::new("test.timer", 3.101, MetricKind::Timer);
        buckets.add(&m1);
        buckets.add(&m2);
        buckets.add(&m3);
        buckets.add(&m4);
        buckets.add(&m5);
        buckets
    }

    #[test]
    fn test_format_buckets_no_timers() {
        let buckets = make_buckets();
        let graphite = Graphite::new("127.0.0.1", 2003);
        let result = graphite.format_stats(&buckets);
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(4, lines.len());
        assert!(lines[0].contains("statsd.bad_messages 0"));
        assert!(lines[1].contains("statsd.total_messages 5"));
        assert!(lines[2].contains("test.counter 1"));
        assert!(lines[3].contains("test.gauge 3.211"));
    }

    #[test]
    fn test_format_buckets_timers() {
        let mut buckets = make_buckets();
        process(&mut buckets);

        let graphite = Graphite::new("127.0.0.1", 2003);
        let result = graphite.format_stats(&buckets);
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(12, lines.len());

        assert!(result.contains("test.timer.max 12.101"));
        assert!(result.contains("test.timer.min 1.101"));
        assert!(result.contains("test.timer.count 3"));
    }
}
