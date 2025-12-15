use crate::net::{
    datastream::DataStream,
    packets::{PacketHandler, status_response::StatusS2CPacket},
    traits::{Decode, Encode},
    types::{string::PacketString, uuid::PacketUUID},
};

pub struct LoginStartC2SPacket {
    name: PacketString,
    uuid: PacketUUID,
}

impl Decode for LoginStartC2SPacket {
    async fn decode(
        stream: &mut crate::net::datastream::DataStream<'_>,
    ) -> Result<Self, crate::net::types::DecodeError> {
        let name: PacketString = stream.read().await?;
        let uuid: PacketUUID = stream.read().await?;
        Ok(Self { name, uuid })
    }
}

impl PacketHandler for LoginStartC2SPacket {
    async fn handle(&self, stream: &mut DataStream<'_>) {
        println!("name={}, uuid={}", self.name.0, self.uuid);
    }
}
