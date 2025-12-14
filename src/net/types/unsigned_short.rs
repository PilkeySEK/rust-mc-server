use crate::{net::traits::Decode, try_read};

pub struct UnsignedShort(pub u16);

impl Decode for UnsignedShort {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<Self, crate::net::types::DecodeError> {
        let upper = try_read!(stream);
        let lower = try_read!(stream);
        let mut value = (upper as u16) << 8;
        value |= lower as u16;
        Ok(Self(value))
    }
}
