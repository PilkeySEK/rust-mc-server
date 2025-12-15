use std::fmt::Display;

use crate::{net::traits::Decode, try_read};

pub struct PacketUUID(pub u128);

impl Decode for PacketUUID {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<Self, super::DecodeError> {
        let mut value: u128 = 0;
        for _ in 0..16 {
            let read_byte = try_read!(stream);
            value <<= 8;
            value |= read_byte as u128;
        }
        Ok(Self(value))
    }
}

impl Display for PacketUUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let splitted = self.split_4();
        f.write_fmt(format_args!(
            "{:08x}-{:08x}-{:08x}-{:08x}",
            splitted[0], splitted[1], splitted[2], splitted[3]
        ))
    }
}

impl PacketUUID {
    fn split_4(&self) -> [u32; 4] {
        [
            (self.0 >> (8 * 12) & (u32::MAX as u128)) as u32,
            (self.0 >> (8 * 8) & (u32::MAX as u128)) as u32,
            (self.0 >> (8 * 4) & (u32::MAX as u128)) as u32,
            (self.0 >> (8 * 0) & (u32::MAX as u128)) as u32,
        ]
    }
}
