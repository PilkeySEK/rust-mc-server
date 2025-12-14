use crate::net::traits::Decode;

pub struct StatusC2SPacket {}

impl Decode for StatusC2SPacket {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<Self, crate::net::types::DecodeError> {
        Ok(Self {})
    }
}
