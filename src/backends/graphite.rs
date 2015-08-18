use super::super::backend::Backend;
use super::super::buckets::Buckets;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;
use time;


#[derive(Debug)]
pub struct Graphite {
    addr: SocketAddrV4,
    last_flush_time: u64,
    last_flush_length: u64
}


impl Graphite {
    /// Create a Graphite formatter
    ///
    /// # Examples
    ///
    /// ```
    /// let graph = Graphite::new(host, port);
    /// ```
    pub fn new(host: &str, port: u16) -> Graphite {
        let ip = Ipv4Addr::from_str(&host).unwrap();
        let addr = SocketAddrV4::new(ip, port);
        Graphite {
            addr: addr,
            last_flush_time: 0,
            last_flush_length: 0,
        }
    }
}


impl Backend for Graphite {
    fn flush_buckets(&mut self, buckets: &Buckets) {
        println!("{:?}", self);
    }
}
