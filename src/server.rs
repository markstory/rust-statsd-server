
use std::sync::mpsc::SyncSender;
use std::net::{Ipv4Addr, TcpStream, TcpListener, SocketAddrV4};
use std::net::ToSocketAddrs;
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::net::SocketAddr;
use tokio_core::net::{UdpSocket, UdpCodec};
use tokio_core::reactor::Core;
use futures::{Stream};
use futures::future;
use metric::{Metric, ParseError};

/// Acceptable event types.
///
pub enum Event {
    UdpMessage(Vec<u8>),
    TcpMessage(TcpStream),
    TimerFlush,
    ParsedMetric(Result<Vec<Metric>, ParseError>)
}

pub struct LineCodec;

impl UdpCodec for LineCodec {
    type In = (SocketAddr, Vec<u8>);
    type Out = (SocketAddr, Vec<u8>);

    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        Ok((*addr, buf.to_vec()))
    }

    fn encode(&mut self, (addr, buf): Self::Out, into: &mut Vec<u8>) -> SocketAddr {
        into.extend(buf);
        addr
    }
}

/// Setup the UDP socket that listens for metrics and
/// publishes them into the bucket storage.
pub fn udp_server(chan: SyncSender<Event>, port: u16) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let addr = addr.to_socket_addrs().unwrap().last().unwrap();

    let socket = UdpSocket::bind(&addr, &handle).unwrap();
    let (_, stream) =
        socket.framed(LineCodec).split();

    let stream = stream.for_each(|(_addr, msg)| {
        let res = chan.send(Event::UdpMessage(Vec::from(msg)));
        res.expect("Cannot send udp message to channel");
        future::ok(())
    });
    let res = core.run(stream);
    res.expect("Udp metrics socket failed");
}

/// Setup the TCP socket that listens for management commands.
pub fn admin_server(chan: SyncSender<Event>, port: u16, host: &str) {
    let tcp = TcpListener::bind((host, port)).unwrap();
    for stream in tcp.incoming() {
        match stream {
            Ok(stream) => {
                chan.send(Event::TcpMessage(stream)).unwrap();
            }
            Err(e) => panic!("Unable to establish TCP socket: {}", e),
        }
    }
}


/// Publishes an event on the channel every interval
///
/// This message is used to push data from the buckets to the backends.
pub fn flush_timer_loop(chan: SyncSender<Event>, interval: u64) {
    let duration = Duration::new(interval, 0);
    loop {
        sleep(duration);
        chan.send(Event::TimerFlush).unwrap();
    }
}
