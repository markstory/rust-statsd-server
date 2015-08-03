use super::backend::Backend;
use super::super::buckets::Buckets;
use clock_ticks;

#[derive(Debug)]
pub struct Console {
    last_flush_time: u64,
    last_flush_length: u64
}


impl Console {
    /// Create a Console formatter bound to any implementation
    /// of `std::io::Write`. The writer will be wrapped in a
    /// BufWriter.
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
        println!("{}:", clock_ticks::precise_time_ms());

        println!("  counters:");
        for (key, value) in buckets.counters().iter() {
            fmt_line(&key, &value);
        }

        println!("  gauges:");
        for (key, value) in buckets.gauges().iter() {
            fmt_line(&key, &value);
        }

        /*
        println!("  timers:");
        for (key, values) in buckets.timers().iter() {
            let samples: &[f64] = values.as_slice();

            println!("    {key}:
       min: {min}
       max: {max}
       count: {count}
       mean: {mean}
       stddev: {std}
       upper_95: {max_threshold}",
                  key=*key,
                  min=samples.min(),
                  max=samples.max(),
                  count=samples.len(),
                  mean=samples.mean(),
                  std=samples.std_dev(),
                  max_threshold=samples.percentile(95.0));
        }
        */
    }
}
