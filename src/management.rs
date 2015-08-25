use buckets::Buckets;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};


/// Handle the management commands
/// returning the response to send back.
pub fn exec(stream: TcpStream, buckets: &Buckets) {
    let mut reader = BufReader::new(stream);
    let mut done = false;

    while !done {
        let mut buffer = String::new();
        let _ = reader.read_line(&mut buffer);
        let command = buffer.split_whitespace()
            .next()
            .unwrap_or("")
            .to_lowercase();
        let mut writer = reader.get_mut();

        // Trigger Deref<Target = str>
        match &*command {
            "help" => {
                let mut help = String::new();
                help.push_str("Statsd Admin Console:\n");
                help.push_str("\n");
                help.push_str("Available commands:\n");
                help.push_str("stats    - print server stats.\n");
                help.push_str("counters - print counter data.\n");
                help.push_str("gauges   - print gauge data.\n");
                help.push_str("timers   - print timer data.\n");
                help.push_str("clear    - clear stored metrics.\n");
                help.push_str("quit     - close this connection.\n");
                let _ = writer.write(&help.as_bytes());
            },
            /*
            "stats" => {
            },
            "counters" => {
            }
            "gauges" => {
            },
            "timers" => {
            },
            */
            "quit" => {
                let bye = "Good bye!\n";
                let _ = writer.write(&bye.as_bytes());
                done = true
            }
            x => {
                let out = format!("ERROR - unknown command `{}`", x);
                let _ = writer.write(&out.as_bytes());
            }
        }
        let _ = writer.flush();
    }
}
