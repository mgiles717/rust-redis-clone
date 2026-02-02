use std::net::{TcpListener, TcpStream};
use std::net::{Ipv4Addr};
use std::io::{Read, Write};

fn init_server() -> Ipv4Addr {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    return localhost;
}

fn main() {
    init_server();
}
