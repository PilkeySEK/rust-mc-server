use crate::{
    net::traits::{Decode, Encode},
    try_read,
};

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

#[derive(Debug)]
pub struct VarInt(pub i32);

impl Encode for VarInt {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut value = self.0;
        loop {
            if (value & !SEGMENT_BITS) == 0 {
                bytes.push(value as u8);
                break;
            }

            bytes.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

            value >>= 7;
        }
        bytes
    }
}

impl Decode for VarInt {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<VarInt, crate::net::types::DecodeError> {
        let mut value: i32 = 0;
        let mut position = 0;
        let mut current_byte: u8;
        loop {
            current_byte = try_read!(stream);
            value |= (current_byte as i32 & SEGMENT_BITS) << position;
            if current_byte as i32 & CONTINUE_BIT == 0 {
                break;
            }
            position += 7;
            if position >= 32 {
                return Err(crate::net::types::DecodeError::UnexpectedData);
            }
        }
        Ok(Self(value))
    }
}
