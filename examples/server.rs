use std::io::{Cursor, ErrorKind, Read, Write};
use std::net::{UdpSocket, SocketAddr};

use among_rs::{read_data, write_disconnect, Reason, write_reliable, write_ack};
use among_rs::packets::Payload::Hello;
use among_rs::packets::{HelloPayload, HazelPacket, Payload};
use among_rs::reason::Reason::Custom;
use std::collections::HashMap;

fn sock_send(sock: &UdpSocket, addr: SocketAddr, x: fn(&mut dyn Write)->std::io::Result<()>) -> std::io::Result<usize>{
    let cur = &mut Cursor::new(Vec::new());
    x(cur)?;
    let mut buf = cur.get_mut();
    println!("{:02?}", buf);
    sock.send_to(buf, addr)
}

fn ack(sock: UdpSocket, addr: SocketAddr, mut nonce: &mut u16){
    let mut cur = Cursor::new(Vec::new());
    write_ack(&mut cur, *nonce);
    sock.send_to(cur.get_mut(), addr);
    nonce = nonce + 1;
}

fn main() {
    let sock = UdpSocket::bind("127.0.0.1:22023").expect("Failed to bind socket to 22023");
    let mut buf = [0; 1024];
    let mut noncies: HashMap<SocketAddr, u16> = HashMap::new();
    loop {
        let (size, addr) = sock.recv_from(&mut buf).expect("IO Error: failed to read!");
        let p = read_data(&mut Cursor::new(buf), true).expect("Failed to parse packet!");
        let m = noncies.get_mut(&addr).unwrap_or_else(|x| {
            &mut 0
        });
        match p{
            HazelPacket::Hello(hello) => {
                let d = match hello.data {
                    Hello(p) => {p}
                    _ => {unreachable!()}
                };
                println!("Hi {}!", d.name);

            }
            HazelPacket::Reliable(n) | HazelPacket::Normal(n) => {
            }
            _ => {}
        }
    }
}