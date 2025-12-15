use std::io::ErrorKind;

use tokio::net::TcpStream;

use crate::net::{
    datastream::DataStream,
    packets::{
        ClientBoundPacket, PacketReadError, pong::PongS2CPacket, read_to_packet,
        status_response::StatusS2CPacket,
    },
    traits::Encode,
    types::DecodeError,
};

macro_rules! write_packet {
    ($packet:expr => $self:expr) => {
        ($self).stream.write(($packet).encode()).await
    };
}

#[derive(Debug, Copy, Clone)]
pub enum ConnectionState {
    None = 0,
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl ConnectionState {
    pub fn from(x: i32) -> Option<Self> {
        match x {
            1 => Some(Self::Status),
            2 => Some(Self::Login),
            3 => Some(Self::Transfer),
            _ => None,
        }
    }
}

pub struct Client<'a> {
    pub state: ConnectionState,
    pub stream: DataStream<'a>,
}

impl<'a> Client<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self {
            state: ConnectionState::None,
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
                    self.state = match ConnectionState::from(packet.intent.0) {
                        Some(state) => state,
                        None => {
                            if let Err(e) = write_packet!(StatusS2CPacket {
                                status_str: String::from(format!(r#"{{"text":"Disallowed intent sent by client: {}"}}"#, packet.intent.0)),
                            } => self)
                            {
                                println!("Failed to write disconnect packet: {}", e);
                            };
                            return;
                        }
                    };
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
