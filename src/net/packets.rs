use crate::net::{
    datastream::DataStream,
    packets::{
        handshake::HandshakeC2SPacket, ping::PingC2SPacket, status_request::StatusC2SPacket,
    },
    traits::Decode,
    types::{DecodeError, varint::VarInt},
};

pub mod handshake;
pub mod ping;
pub mod pong;
pub mod status_request;
pub mod status_response;

#[derive(Debug)]
pub enum PacketReadError {
    DecodeError(DecodeError),
    UnknownPacketId(VarInt),
}

pub enum ClientBoundPacket {
    Handshake(HandshakeC2SPacket),
    Ping(PingC2SPacket),
    Status(StatusC2SPacket),
}

macro_rules! try_packet_read {
    ($packet:ty, $packet_enum_name:ident, $stream:expr) => {
        match <$packet>::decode($stream).await {
            Ok(packet) => Ok(ClientBoundPacket::$packet_enum_name(packet)),
            Err(e) => Err(PacketReadError::DecodeError(e)),
        }
    };
}

/// Read the next packet from the stream and try to match the packet id to a packet, then read the packet.
#[must_use]
pub async fn read_to_packet(
    stream: &mut DataStream<'_>,
    state: i32,
) -> Result<ClientBoundPacket, PacketReadError> {
    let _length: VarInt = match stream.read().await {
        Ok(data) => data,
        Err(e) => return Err(PacketReadError::DecodeError(e)),
    };
    let packet_id: VarInt = match stream.read().await {
        Ok(data) => data,
        Err(e) => return Err(PacketReadError::DecodeError(e)),
    };

    match packet_id.0 {
        0x0 => match state {
            -1 => try_packet_read!(HandshakeC2SPacket, Handshake, stream),
            1 => try_packet_read!(StatusC2SPacket, Status, stream),
            _ => panic!("invalid state: {}", state),
        },
        0x1 => try_packet_read!(PingC2SPacket, Ping, stream),
        _ => Err(PacketReadError::UnknownPacketId(packet_id)),
    }
}
