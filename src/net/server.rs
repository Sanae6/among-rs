use std::io::{Cursor, Write};
use std::net::UdpSocket;

use protocol::{Parcel, Settings};
use protocol::wire::dgram::Pipeline;

use crate::net::common::NullPipeline;
use crate::proto::hazel::HazelPacket;
use crate::proto::hazel::HazelPacket::{Normal, Reliable};
use crate::proto::misc::close_reason::Reason;
use std::slice::Iter;
use std::cmp::max;
use crate::proto::AmongUsPacket;
use crate::util::varint::Varint;
use std::collections::HashMap;

pub enum ServerEvent {
    ClientConnected {
        id: u64
    },
    ClientDisconnected {
        reason: Reason
    },
    ClientMessage {
        packet: dyn AmongUsPacket
    }
}

struct ServerOwnedClient{
    write_buffer: [u8; 1024],
    read_buffer: [u8; 1024],
    source_addr: str
}
//impl Drop for ServerOwnedClient

pub struct AmongUsServer{
    sock: UdpSocket,
    latest_ack_id: u16,//latest id, used for reliable
    pipeline: Pipeline<HazelPacket, NullPipeline>,
    event_queue: Vec<ServerEvent>,
    client_list: std::collections::HashMap<Varint, Box<ServerOwnedClient>>
}

impl AmongUsServer{
    pub fn new(addr: &str) -> AmongUsServer{
        let mut line = Pipeline::<HazelPacket, NullPipeline>::new(NullPipeline{}, Settings::default());

        AmongUsServer{
            sock: UdpSocket::bind(addr).expect(&*format!("couldn't bind to {}", addr)),
            latest_ack_id: 0,
            pipeline: line,
            event_queue: Vec::new(),
            client_list: HashMap::new()
        }
    }

    pub fn iter(&self, max_packets: usize) -> Iter<'_, ServerEvent> {
        if max_packets > 0 {
            self.event_queue[..max_packets].iter()
        }else {
            self.event_queue[..].iter()
        }
    }

    pub fn send_reliable_packet<P: Parcel>(&mut self, to: &str, packet: P) where P : Parcel {
        self.pipeline.send_to(&mut Vec::from(self.write_buffer), &Reliable {
            id: self.latest_ack_id + 1,
            data: packet.raw_bytes(&Settings::default()).unwrap()
        });
        self.sock.send_to(&*self.write_buffer, to);
        Normal {
            data: Vec::from(self.write_buffer)
        };
    }
    pub fn poll(){

    }
}