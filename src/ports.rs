use crate::{
    common_ports::{MOST_COMMON_PORTS, MOST_COMMON_PORTS_100},
    model::{Port, SubDomain},
};

use rayon::prelude::*;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn scan_ports(mut subdomain: SubDomain) -> SubDomain {
    println!("Scanning ports for subdomain");
    let socket_addrs: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
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

    subdomain.open_ports = MOST_COMMON_PORTS
        .into_par_iter()
        .map(|port| scan_port(socket_addrs[0], *port))
        .filter(|port| port.is_open) // filter out closed ports
        .collect();
    subdomain
}

pub fn scan_port(mut socket_addr: SocketAddr, port: u16) -> Port {
    println!("Checking socket_addr {} on port {}", socket_addr, port);
    let timeout = Duration::from_secs(3);
    socket_addr.set_port(port);

    let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_addr, timeout) {
        true
    } else {
        false
    };

    Port {
        port: port,
        is_open: is_open,
    }
}
