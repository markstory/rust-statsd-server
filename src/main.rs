extern crate docopt;
extern crate rustc_serialize;

use std::thread;
use std::sync::mpsc::channel;
use std::str;


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
                let msg = str::from_utf8(&buf).unwrap().to_string();
                println!("{:?}", msg);
            },
        }
    }
}
