use buckets::Buckets;
use time;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};
use std::fmt::Write as fmtWrite;


/// Handle the management commands
/// returning the response to send back.
pub fn exec(stream: TcpStream, buckets: &mut Buckets) {
    let mut reader = BufReader::new(stream);
    let mut done = false;

    while !done {
        let mut buffer = String::new();
        let _ = reader.read_line(&mut buffer);
        let command = buffer.split_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_lowercase();
        let writer = reader.get_mut();
        let mut out = String::new();

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
                out.push_str("clear    - clear stored metrics.\n");
                out.push_str("quit     - close this connection.\n");
            }
            "stats" => {
                let uptime = (time::get_time() - buckets.start_time()).num_seconds();
                write!(out, "uptime: {} seconds\n", uptime).unwrap();
                write!(out, "bad_messages: {}\n", buckets.bad_messages()).unwrap();
                write!(out, "total_messages: {}\n", buckets.total_messages()).unwrap();
                write!(out, "END\n\n").unwrap();
            }
            "counters" => {
                for (key, value) in buckets.counters().iter() {
                    write!(out, " {}: {}\n", key, value).unwrap();
                }
                write!(out, "END\n\n").unwrap();
            }
            "gauges" => {
                for (key, value) in buckets.gauges().iter() {
                    write!(out, " {}: {}\n", key, value).unwrap();
                }
                write!(out, "END\n\n").unwrap();
            }
            "timers" => {
                for (key, value) in buckets.timers().iter() {
                    write!(out, " {}: {:?}\n", key, value).unwrap();
                }
                write!(out, "END\n\n").unwrap();
            }
            "quit" => {
                write!(out, "Good bye!\n\n").unwrap();
                done = true
            }
            "clear" => {
                buckets.reset();
                write!(out, "Timers, counters and internal stats cleared.\n").unwrap();
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
