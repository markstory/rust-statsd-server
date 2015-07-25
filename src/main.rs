extern crate docopt;
extern crate rustc_serialize;

use std::thread;
use std::sync::mpsc::channel;
use std::str;
use std::str::FromStr;


// Local module imports.
mod metric;
mod cli;
mod server;
mod buckets;
mod backend {
    mod backend;
    mod console;
}


// Max Packet Size is 256.
// static FLUSH_INTERVAL_MS: u64 = 10000;


fn main() {
    let args = cli::parse_args();
    let (event_send, event_recv) = channel();

    let mut buckets = buckets::Buckets::new();

    // Setup the UDP server which publishes events to the event channel
    thread::spawn(move || {
        server::udp_server(event_send, args.flag_port);
    });

    // Main event loop.
    loop {
        let result = match event_recv.recv() {
            Ok(res) => res,
            Err(e) => panic!(format!("Event channel has hung up: {:?}", e)),
        };

        match result {
            server::Event::UdpMessage(buf) => {
                // Create the metric and push it into the buckets.
                str::from_utf8(&buf).map(|val| {
                    metric::Metric::from_str(&val)
                    .and_then(|metric| {
                        buckets.add(&metric);
                        Ok(metric)
                    })
                    .or_else(|err| {
                        buckets.add_bad_message();
                        Err(err)
                    }).ok();
                }).ok();
            },
        }
    }
}
