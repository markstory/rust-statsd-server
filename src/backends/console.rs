use super::super::backend::Backend;
use super::super::buckets::Buckets;
use time;

#[derive(Debug)]
pub struct Console {
    last_flush_time: u64,
    last_flush_length: u64
}


impl Console {
    /// Create a Console formatter that prints to stdout
    ///
    /// # Examples
    ///
    /// ```
    /// let cons = Console::new();
    /// ```
    pub fn new() -> Console {
        Console {
            last_flush_time: 0,
            last_flush_length: 0,
        }
    }

}

/// Print a single stats line.
fn fmt_line(key: &str, value: &f64) {
    println!("    {}: {}", key, value)
}


impl Backend for Console {
    fn flush_buckets(&mut self, buckets: &Buckets) {
        let now = time::get_time();
        println!("Flushing metrics: {}",
                 time::at(now).rfc822().to_string());

        println!("  bad_messages: {}", buckets.bad_messages());
        println!("  total_messages: {}", buckets.total_messages());

        println!("  counters:");
        for (key, value) in buckets.counters().iter() {
            fmt_line(&key, &value);
        }

        println!("  gauges:");
        for (key, value) in buckets.gauges().iter() {
            fmt_line(&key, &value);
        }

        println!("  timers:");
        for (key, values) in buckets.timers().iter() {
            println!("    {}: {:?}", key, values);
        }

        println!("  timer_data:");
        for (key, values) in buckets.timer_data().iter() {
            println!("    {}: {:?}", key, values);
        }
    }
}
