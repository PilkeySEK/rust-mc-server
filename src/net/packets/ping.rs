use crate::net::{traits::Decode, types::long::Long};

pub struct PingC2SPacket {
    pub timestamp: Long,
}

impl Decode for PingC2SPacket {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<Self, crate::net::types::DecodeError> {
        let timestamp: Long = stream.read().await?;
        Ok(Self { timestamp })
    }
}
