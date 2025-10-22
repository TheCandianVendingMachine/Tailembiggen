use crate::peer::Peer;
use std::net;
use std::sync::{Arc, Mutex};

mod error;

#[derive(Debug, Clone)]
struct FirewallBypass {
    socket: Arc<Mutex<net::UdpSocket>>,
    counter: u64,
}

#[derive(Debug, Clone)]
struct Nat {
    socket: Arc<Mutex<net::UdpSocket>>,
}

#[derive(Debug, Clone)]
pub struct Connection {
    peer: Peer,
    socket: Arc<Mutex<net::UdpSocket>>,
    firewall_bypass: FirewallBypass,
    nat_traversal: Nat,
}

impl Connection {
    pub fn new(peer: Peer) -> Result<Connection, error::Error> {
        let socket = net::UdpSocket::bind(peer.address.to_socket_addr())?;
        let socket = Arc::new(Mutex::new(socket));
        Ok(Connection {
            peer,
            socket: socket.clone(),
            firewall_bypass: FirewallBypass {
                socket: socket.clone(),
                counter: 0,
            },
            nat_traversal: Nat {
                socket: socket.clone(),
            },
        })
    }
}
