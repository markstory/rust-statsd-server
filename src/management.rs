use buckets::Buckets;
use time;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};
use std::fmt::Write as fmtWrite;
use std::sync::{Arc, Mutex};

/// Handle the management commands
/// returning the response to send back.
pub fn exec(stream: TcpStream, buckets_mutex: Arc<Mutex<Buckets>>) {
    let mut reader = BufReader::new(stream);
    let mut done = false;

    while !done {
        let mut buffer = String::new();
        let res = reader.read_line(&mut buffer);
        if res.is_err() {
            done = true;
        }

        let command = buffer.split_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_lowercase();
        let writer = reader.get_mut();
        let mut out = String::new();

        let latest_snapshot = || { (*buckets_mutex.lock().unwrap()).clone() };

        // Trigger Deref<Target = str>
        match &*command {
            "help" => {
                out.push_str("Statsd Admin Console:\n");
                out.push_str("\n");
                out.push_str("Available commands:\n");
                out.push_str("stats    - print server stats.\n");
                out.push_str("counters - print counter data.\n");
                out.push_str("gauges   - print gauge data.\n");
                out.push_str("timers   - print timer data.\n");
                out.push_str("quit     - close this connection.\n");
            }
            "stats" => {
                let buckets = latest_snapshot();
                let uptime = (time::get_time() - buckets.start_time()).num_seconds();
                write!(out, "uptime: {} seconds\n", uptime).unwrap();
                write!(out, "bad_messages: {}\n", buckets.bad_messages()).unwrap();
                write!(out, "total_messages: {}\n", buckets.total_messages()).unwrap();
                write!(out, "END\n\n").unwrap();
            }
            "counters" => {
                let buckets = latest_snapshot();
                for (key, value) in buckets.counters().iter() {
                    write!(out, " {}: {}\n", key, value).unwrap();
                }
                write!(out, "END\n\n").unwrap();
            }
            "gauges" => {
                for (key, value) in latest_snapshot().gauges().iter() {
                    write!(out, " {}: {}\n", key, value).unwrap();
                }
                write!(out, "END\n\n").unwrap();
            }
            "timers" => {
                for (key, value) in latest_snapshot().timers().iter() {
                    write!(out, " {}: {:?}\n", key, value).unwrap();
                }
                write!(out, "END\n\n").unwrap();
            }
            "quit" => {
                write!(out, "Good bye!\n\n").unwrap();
                done = true
            }
            "" => {
                // continue.
            }
            x => {
                write!(out, "ERROR - unknown command `{}`\n", x).unwrap();
            }
        }
        if out.len() > 0 {
            let _ = writer.write(&out.as_bytes());
            let _ = writer.flush();
        }
    }
}
