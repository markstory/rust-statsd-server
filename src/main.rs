extern crate docopt;
extern crate rustc_serialize;


// Local module imports.
mod metric;
mod cli;
mod server;
mod backend {
    mod console;
}


// static FLUSH_INTERVAL_MS: u64 = 10000;
// static MAX_PACKET_SIZE: u16 = 256;


fn main() {
    let args = cli::parse_args();
    println!("{:?}", args);
}
