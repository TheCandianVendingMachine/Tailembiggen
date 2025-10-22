use bincode::{Decode, Encode};

#[derive(Debug, Encode)]
pub struct Ping {
    counter: u64,
}

#[derive(Debug, Decode)]
pub struct Pong {
    counter: u64,
}
