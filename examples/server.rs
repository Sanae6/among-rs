use std::net::UdpSocket;
use protocol::wire::dgram::Pipeline;
use protocol::Settings;
use among_rs::proto::hazel::HazelPacket;
use std::io::{Cursor, Read};
use among_rs::net::server::{AmongUsServer, ServerEvent};


fn main() {
    let ams = AmongUsServer::new("127.0.0.1:22023");
    loop {
        for x in ams.iter(0) {
            match x{
                ServerEvent::ClientConnected { .. } => {}
                ServerEvent::ClientDisconnected { .. } => {}
                ServerEvent::ClientMessage { packet:p } => match p {
                    HazelPacket::Hello { id: _id, data } => {
                        println!("{:?}", data);
                    }
                    _ => {}
                },
            }
        }
    }
}