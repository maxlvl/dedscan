use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, SubDomain},
};

use std::net::{SocketAddrs, TcpStream, ToSocketAddrs};

pub fn scan_ports(mut subdomain: SubDomain) -> SubDomain {
    let socket_addrs: Vec<SocketAddrs> = format("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("Port Scanner: Establishing TCP Connection via socket address")
        .collect();

    if socket_addrs.len() == 0 {
        println!(
            "No socket addresses could be found for subdomain {}",
            subdomain.domain
        );
        return subdomain;
    }
}
