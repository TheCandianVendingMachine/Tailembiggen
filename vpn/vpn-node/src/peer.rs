use std::net;
mod payload;

#[derive(Debug, Copy, Clone)]
pub struct Address {
    ip: net::IpAddr,
    port: u16,
}

impl Address {
    pub fn to_socket_addr(&self) -> net::SocketAddr {
        net::SocketAddr::new(self.ip, self.port)
    }
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub address: Address,
    pub public_key: rsa::RsaPublicKey,
}
