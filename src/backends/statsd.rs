use super::super::backend::Backend;
use super::super::buckets::Buckets;
use std::net::UdpSocket;

#[derive(Debug)]
struct StatsdConnection {
    socket: UdpSocket,
    remote_statsd: String,
}

#[derive(Debug)]
pub struct Statsd {
    packet_limit: usize,
    connections: Vec<StatsdConnection>,
}

fn open_new_udp_port() -> UdpSocket {
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

fn open_new_udp_connection(address: String) -> UdpSocket {
    let socket = open_new_udp_port();

    socket.set_nonblocking(true).expect("Cannot turn on the non blocking mode");
    socket.connect(address)
        .expect("Could not connect to remote statsd instance");
    socket
}

impl Statsd {
    /// Create a Statsd that sends aggregated metrics to other statsd instance
    ///
    /// # Examples
    ///
    /// ```
    /// let cons = Statsd::new("127.0.0.1", 8125, 1024);
    /// ```
    pub fn new(statsd_hosts: Vec<String>, packet_limit: usize) -> Statsd {
        let connections = statsd_hosts.iter().map(|host| {
            println!("Opening socket to another statsd server {}", host);
            StatsdConnection {
                socket: open_new_udp_connection(host.clone()),
                remote_statsd: host.clone(),
            }
        }).collect();
        Statsd {
            connections: connections,
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
            for i in 0..self.connections.len() {
                let connection = &mut self.connections[i];
                let result = connection.socket.send(packet.as_bytes());
                match result {
                    Err(e) => {
                        eprintln!("Failed to send udp packet, reopening connection: {:?}", e);
                        connection.socket = open_new_udp_connection(connection.remote_statsd.clone());
                    }
                    Ok(_) => {}
                }
            }
        }
    }
}
