use std::io::{self, ErrorKind};

use tokio::net::TcpStream;

use crate::net::{
    datastream::DataStream,
    packets::{
        ClientBoundPacket, PacketReadError, pong::PongS2CPacket, read_to_packet,
        status_response::StatusS2CPacket,
    },
    traits::Encode,
    types::{
        DecodeError, long::Long, string::PacketString, unsigned_short::UnsignedShort,
        varint::VarInt,
    },
};

macro_rules! write_packet {
    ($packet:expr => $self:expr) => {
        ($self).stream.write(($packet).encode()).await
    };
}

pub struct Client<'a> {
    pub state: i32,
    pub stream: DataStream<'a>,
}

impl<'a> Client<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self {
            state: -1,
            stream: DataStream::new(stream),
        }
    }

    pub async fn process_client(&mut self) {
        loop {
            let packet = match read_to_packet(&mut self.stream, self.state).await {
                Ok(packet) => packet,
                Err(e) => {
                    match &e {
                        PacketReadError::DecodeError(DecodeError::ReadError(e)) => {
                            if e.kind() == ErrorKind::UnexpectedEof {
                                return;
                            }
                        }
                        _ => (),
                    }
                    println!("Error reading packet: {:?}", e);
                    continue;
                }
            };
            match packet {
                ClientBoundPacket::Handshake(packet) => {
                    self.state = 1;
                    let status_str = String::from(
                        r#"
        {
            "version": {
                "name": "1.21.8",
                "protocol": 773
            },
            "players": {
                "max": 20,
                "online": 0,
                "sample": []
            },
            "description": {"text":"Hello, World!"},
            "enforcesSecureChat": false
        }
            "#,
                    );
                    match write_packet!(StatusS2CPacket { status_str } => self) {
                        Ok(_) => (),
                        Err(_) => todo!(),
                    };
                }
                ClientBoundPacket::Ping(packet) => {
                    write_packet!(PongS2CPacket {timestamp: packet.timestamp} => self).unwrap(); // TODO error handling
                }
                ClientBoundPacket::Status(packet) => {}
            }
        }
    }
}
