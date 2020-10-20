use std::net::UdpSocket;

fn main() {
    let sock: UdpSocket = UdpSocket::bind("127.0.0.1:22023").expect("couldn't bind to 32394");
    //sock.connect().expect("couldn't connect to impostor");
    let mut buf = [0;1024];
    loop {
        let (amt,src) = sock.recv_from(&mut buf).expect("failed to read!");
        println!("{}", src);
    }
}