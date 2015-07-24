use std::sync::mpsc::Sender;
use std::net::{Ipv4Addr, UdpSocket, SocketAddrV4};


/// Acceptable event types.
///
pub enum Event {
    UdpMessage(Vec<u8>)
}


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
