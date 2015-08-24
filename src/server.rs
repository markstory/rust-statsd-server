use std::sync::mpsc::Sender;
use std::net::{Ipv4Addr, TcpStream, UdpSocket, SocketAddrV4};
use std::thread::sleep_ms;
use std::io::Read;


/// Acceptable event types.
///
pub enum Event {
    UdpMessage(Vec<u8>),
    TcpMessage(Vec<u8>),
    TimerFlush
}


/// Setup the UDP socket that listens for metrics and
/// publishes them into the bucket storage.
pub fn udp_server(chan: Sender<Event>, port: u16) {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let socket = UdpSocket::bind(addr).ok().unwrap();
    let mut buf = [0; 256];
    loop {
        let (len, _) = match socket.recv_from(&mut buf) {
            Ok(r) => r,
            Err(_) => panic!("Could not read UDP socket."),
        };
        let bytes = Vec::from(&buf[..len]);
        chan.send(Event::UdpMessage(bytes)).unwrap();
    }
}

/// Setup the TCP socket that listens for management commands.
pub fn admin_server(chan: Sender<Event>, port: u16, host: &str) {
    let mut stream = TcpStream::connect((host, port)).unwrap();
    let mut buf = [0; 256];
    loop {
        let len = match stream.read(&mut buf) {
            Ok(r) => r,
            Err(_) => panic!("Could not read TCP stream."),
        };
        let bytes = Vec::from(&buf[..len]);
        chan.send(Event::TcpMessage(bytes)).unwrap();
    }
}


/// Publishes an event on the channel every interval
///
/// This message is used to push data from the buckets to the backends.
pub fn flush_timer_loop(chan: Sender<Event>, interval: u32) {
    loop {
        sleep_ms(interval * 1000);
        chan.send(Event::TimerFlush).unwrap();
    }
}
