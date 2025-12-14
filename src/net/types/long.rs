use crate::{
    net::traits::{Decode, Encode},
    try_read,
};

#[derive(Clone, Copy)]
pub struct Long(pub i64);

impl Decode for Long {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<Self, crate::net::types::DecodeError> {
        let mut value = 0;
        for _ in 0..8 {
            let read_byte = try_read!(stream);
            value <<= 8;
            value |= read_byte as i64;
        }
        Ok(Self(value))
    }
}

impl Encode for Long {
    fn encode(&self) -> Vec<u8> {
        let mut parts = Vec::new();
        for i in 0..8 {
            parts.push(((self.0 >> (8 * i)) & (u8::MAX as i64)) as u8);
        }
        parts
    }
}
