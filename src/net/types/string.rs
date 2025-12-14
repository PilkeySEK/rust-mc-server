use crate::{
    net::{
        traits::{Decode, Encode},
        types::varint::VarInt,
    },
    try_read,
};

pub struct PacketString(pub String);

impl Decode for PacketString {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<Self, crate::net::types::DecodeError> {
        let length: VarInt = stream.read().await.unwrap(); // TODO error handling
        let mut bytes = Vec::<u8>::new();
        for _ in 0..length.0 {
            bytes.push(try_read!(stream));
        }
        let str = match String::from_utf8(bytes) {
            Ok(data) => data,
            Err(_) => return Err(crate::net::types::DecodeError::UnexpectedData),
        };
        Ok(Self(str))
    }
}

impl Encode for PacketString {
    fn encode(&self) -> Vec<u8> {
        let bytes: &[u8] = self.0.as_bytes();
        let len = VarInt(bytes.len() as i32);
        let mut response = Vec::new();
        response.append(&mut len.encode());
        response.append(&mut Vec::from(bytes));
        response
    }
}
