use super::super::backend::Backend;
use super::super::buckets::Buckets;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::fmt::Write;
use std::io::Write as IoWrite;
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
        let start = time::get_time().sec;
        let mut stats = String::new();

        write!(
            stats,
            "{} {} {}\n",
            "statsd.bad_messages",
            buckets.bad_messages(),
            start).unwrap();
        write!(
            stats,
            "{} {} {}\n",
            "statsd.total_messages",
            buckets.total_messages(),
            start).unwrap();

        for (key, value) in buckets.counters().iter() {
            write!(
                stats,
                "{} {} {} \n",
                key,
                value,
                start).unwrap();
        }

        for (key, value) in buckets.gauges().iter() {
            write!(
                stats,
                "{} {} {} \n",
                key,
                value,
                start).unwrap();
        }

        // The raw timer data is not sent to graphite.
        for (key, value) in buckets.timer_data().iter() {
            write!(
                stats,
                "{} {} {} \n",
                key,
                value,
                start).unwrap();
        }

        let mut stream = TcpStream::connect(self.addr).unwrap();
        let _ = stream.write(stats.as_bytes());
    }
}
