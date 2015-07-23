use std::net::{UdpSocket, SocketAddr};
use std::str::FromStr;


// Local module imports.
mod metric;
mod cli;
mod server;
mod backend {
    mod console;
}


static FLUSH_INTERVAL_MS: u64 = 10000;
static MAX_PACKET_SIZE: u8 = 256;


fn main() {
    println!("Hello, world!");
}
