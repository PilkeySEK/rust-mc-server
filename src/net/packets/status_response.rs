use crate::net::{
    traits::Encode,
    types::{string::PacketString, varint::VarInt},
};

pub struct StatusS2CPacket {
    pub status_str: String,
}

impl Encode for StatusS2CPacket {
    fn encode(&self) -> Vec<u8> {
        let mut response = Vec::new();

        response.append(&mut VarInt(0x0).encode());
        response.append(&mut PacketString(self.status_str.clone()).encode());
        response
    }
}
