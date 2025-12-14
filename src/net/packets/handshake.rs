use crate::net::{
    traits::Decode,
    types::{string::PacketString, unsigned_short::UnsignedShort, varint::VarInt},
};

pub struct HandshakeC2SPacket {
    pub protocol_version: VarInt,
    pub server_address: PacketString,
    pub server_port: UnsignedShort,
    /// 1 = Status;
    /// 2 = Login;
    /// 3 = Transfer;
    pub intent: VarInt,
}

impl Decode for HandshakeC2SPacket {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<Self, crate::net::types::DecodeError> {
        let protocol_version: VarInt = stream.read().await?;
        let server_address: PacketString = stream.read().await?;
        let server_port: UnsignedShort = stream.read().await?;
        let intent: VarInt = stream.read().await?;

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            intent,
        })
    }
}
