use super::super::backend::Backend;
use super::super::buckets::Buckets;
use std::net::UdpSocket;

#[derive(Debug)]
pub struct Statsd {
    socket: UdpSocket,
    packet_limit: usize,
}

fn get_new_socket() -> UdpSocket {
    let mut port = 12000;
    let mut result = None;
    while result.is_none() {
        port += 1;
        match UdpSocket::bind(format!("0.0.0.0:{}", port)) {
            Ok(socket) => result = Some(socket),
            _ => ()
        }
    }

    result.unwrap()
}

impl Statsd {
    /// Create a Statsd that sends aggregated metrics to other statsd instance
    ///
    /// # Examples
    ///
    /// ```
    /// let cons = Statsd::new("127.0.0.1", 8125, 1024);
    /// ```
    pub fn new(statsd_host: &str, statsd_port: u16, packet_limit: usize) -> Statsd {
        let socket = get_new_socket();

        let remote_statsd = format!("{}:{}", statsd_host, statsd_port);

        socket.set_nonblocking(true).expect("Cannot turn on the non blocking mode");
        socket.connect(remote_statsd)
            .expect("Could not connect to remote statsd instance");
        Statsd {
            socket: socket,
            packet_limit: packet_limit,
        }
    }

    pub fn format_stats(&self, buckets: &Buckets) -> Vec<String> {
        let mut stats = vec![String::new()];

        {
            let mut push_str = |new_str: String| {
                let last_idx = stats.len() - 1;
                if stats[last_idx].len() + new_str.len() <= self.packet_limit {
                    stats[last_idx].push_str(&new_str);
                } else {
                    stats.push(new_str);
                }
            };

            push_str(
                format!("{}:{}|c\n",
                    "statsd.bad_messages",
                    buckets.bad_messages())
            );

            push_str(
                format!("{}:{}|c\n",
                    "statsd.total_messages",
                    buckets.total_messages())
            );

            for (key, value) in buckets.counters().iter() {
                push_str(format!("{}:{}|c\n", key, value));
            }

            for (key, value) in buckets.gauges().iter() {
                push_str(format!("{}:{}|g\n", key, value));
            }

            for (key, values) in buckets.timers().iter() {
                for value in values {
                    push_str(format!("{}:{}|ms\n", key, value));
                }
            }
        }

        stats
    }
}

impl Backend for Statsd {
    fn flush_buckets(&mut self, buckets: &Buckets) {
        for packet in self.format_stats(buckets) {
            let result = self.socket.send(packet.as_bytes());
            result.expect("Flushing to other statsd backend failed");
        }
    }
}
