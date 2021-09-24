use std::{net::{TcpListener, UdpSocket}, thread, time::Duration};


#[derive(Clone, Debug)]
pub enum NetworkMessage {
    StartBroadcast(u16),
}

pub fn create_tcp_listener() ->  Option<u16> {
    if let Ok(stream_2) = TcpListener::bind("127.0.0.1:0") {
        println!("Local ip address and socket: {:?}", stream_2.local_addr());
        return match stream_2.local_addr() {
            Ok(xxx) => {Some(xxx.port())},
            Err(_) => {None},
        }
    } else {
        println!("Couldn't connect to server...");
        return None
    }
}

pub fn broadcast_port_and_username(username: String, tcp_port: String) {
    if let Ok(socket) = UdpSocket::bind("127.0.0.1:0") {
        match socket.set_broadcast(true) {
            Ok(_) => {},
            Err(e) => { println!("{}", e)},
        }
        match socket.connect("255.255.255.255:34066") {
            Ok(_) => {},
            Err(e) => { println!("{}", e)},
        }
        loop {
            thread::sleep(Duration::from_secs(2));
            let broadcast_s = format!("{}-{}", username, tcp_port);
            match socket.send(broadcast_s.as_bytes()) {
                Ok(_) => {println!("broadcast sended: {}", broadcast_s);},
                Err(e) => { println!("{}", e)},
            }
        }
    }
}