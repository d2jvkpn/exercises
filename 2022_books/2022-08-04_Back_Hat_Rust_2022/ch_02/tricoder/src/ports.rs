use rayon::prelude::*;

use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::{error, time::Duration};

use crate::common_ports::MOST_COMMON_PORTS_100;
use crate::model::{Port, Subdomain};

pub fn scan_ports(mut subdomain: Subdomain) -> Result<Subdomain, Box<dyn error::Error>> {
    let addrs: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .map_err(|e| format!("creating socket address: {}", e))?
        .collect();

    if addrs.is_empty() {
        return Ok(subdomain);
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port(addrs[0], *port))
        .filter(|port| port.is_open) // filter closed ports
        .collect();

    Ok(subdomain)
}

fn scan_port(mut addr: SocketAddr, port: u16) -> Port {
    addr.set_port(port);

    let is_open = TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok();

    Port { port, is_open }
}
