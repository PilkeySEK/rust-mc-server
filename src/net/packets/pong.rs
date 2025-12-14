use crate::net::{
    traits::Encode,
    types::{long::Long, varint::VarInt},
};

pub struct PongS2CPacket {
    pub timestamp: Long,
}

impl Encode for PongS2CPacket {
    fn encode(&self) -> Vec<u8> {
        let mut response = Vec::new();
        response.append(&mut VarInt(0x1).encode());
        response.append(&mut self.timestamp.encode());
        response
    }
}
