use super::super::backend::Backend;
use super::super::buckets::Buckets;
use std::net::{ToSocketAddrs, SocketAddr, TcpStream};
use std::fmt::Write;
use std::io::Write as IoWrite;
use time;


#[derive(Debug)]
pub struct Graphite {
    addr: SocketAddr,
    last_flush_time: u64,
    last_flush_length: u64,
    flush_interval_seconds: i32,
    global_prefix: String,
    counter_prefix: String,
    gauge_prefix: String,
    timer_prefix: String,
}

fn default_prefix_str(input: &str) -> String {
    if input == "" {
        "".to_owned()
    } else {
        format!("{}.", input)
    }
}

impl Graphite {
    /// Create a Graphite formatter
    ///
    /// # Examples
    ///
    /// ```
    /// let graph = Graphite::new(host, port);
    /// ```
    pub fn new(host: &str, port: u16,
               global_prefix: &str,
               counter_prefix: &str,
               gauge_prefix: &str,
               timer_prefix: &str,
               flush_interval_seconds: i32,
    ) -> Graphite {
        let addr = format!("{}:{}", host, port)
            .to_socket_addrs().unwrap().last().unwrap();
        let glob_prefix = default_prefix_str(global_prefix);
        Graphite {
            addr: addr,
            last_flush_time: 0,
            last_flush_length: 0,
            flush_interval_seconds: flush_interval_seconds,
            global_prefix: glob_prefix.clone(),
            counter_prefix: format!("{}{}", glob_prefix, default_prefix_str(counter_prefix)),
            gauge_prefix: format!("{}{}", glob_prefix, default_prefix_str(gauge_prefix)),
            timer_prefix: format!("{}{}", glob_prefix, default_prefix_str(timer_prefix)),
        }
    }

    /// Convert the buckets into a String that
    /// can be sent to graphite's newline API
    pub fn format_stats(&self, buckets: &Buckets) -> String {
        let start = time::get_time().sec;
        let mut stats = String::new();

        write!(stats,
               "{}{} {} {}\n",
               self.global_prefix,
               "statsd.bad_messages",
               buckets.bad_messages(),
               start)
            .unwrap();
        write!(stats,
               "{}{} {} {}\n",
               self.global_prefix,
               "statsd.total_messages",
               buckets.total_messages(),
               start)
            .unwrap();

        for (key, value) in buckets.counters().iter() {
            write!(stats, "{}{} {} {}\n", self.counter_prefix, key,
                   value / self.flush_interval_seconds as f64, start).unwrap();
        }

        for (key, value) in buckets.gauges().iter() {
            write!(stats, "{}{} {} {}\n", self.gauge_prefix, key, value, start).unwrap();
        }

        // The raw timer data is not sent to graphite.
        for (key, value) in buckets.timer_data().iter() {
            write!(stats, "{}{} {} {}\n", self.timer_prefix, key, value, start).unwrap();
        }
        stats
    }
}


impl Backend for Graphite {
    fn flush_buckets(&mut self, buckets: &Buckets) {
        let stats = self.format_stats(&buckets);

        let start = time::get_time();
        let len = stats.as_bytes().len();

        match TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                match stream.write_all(stats.as_bytes()) {
                    Ok(_) => {
                        let end = time::get_time();
                        let taken = end - start;
                        println!("Successfully flushed {} bytes to graphite in {} milliseconds",
                                 len, taken.num_milliseconds())
                    },
                    Err(e) => eprintln!("Could not complete write to graphite: {:?}", e),
                }
            },
            Err(err) => eprintln!("Cannot connect to graphite server: {:?}", err),
        };
    }
}


#[cfg(test)]
mod test {
    use super::super::super::metric::{Metric, MetricKind};
    use super::super::super::buckets::Buckets;
    use super::super::super::metric_processor::process;
    use super::*;

    fn make_buckets() -> Buckets {
        let mut buckets = Buckets::new(0., true);
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
        let graphite = Graphite::new("127.0.0.1", 2003,
         "stats",
         "counters",
         "gauges",
         "timers",
         2,
        );
        let result = graphite.format_stats(&buckets);
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(4, lines.len());
        assert!(lines[0].contains("stats.statsd.bad_messages 0"));
        assert!(lines[1].contains("stats.statsd.total_messages 5"));
        assert!(lines[2].contains("stats.counters.test.counter 0.5"));
        assert!(lines[3].contains("stats.gauges.test.gauge 3.211"));
    }

    #[test]
    fn test_format_buckets_timers() {
        let mut buckets = make_buckets();
        process(&mut buckets);

        let graphite = Graphite::new("127.0.0.1", 2003,
            "stats",
            "counters",
            "gauges",
            "timers",
            2
        );
        let result = graphite.format_stats(&buckets);
        let lines: Vec<&str> = result.lines().collect();

        assert_eq!(15, lines.len());

        assert!(result.contains("stats.timers.test.timer.max 12.101"));
        assert!(result.contains("stats.timers.test.timer.min 1.101"));
        assert!(result.contains("stats.timers.test.timer.count 3"));
    }
}
