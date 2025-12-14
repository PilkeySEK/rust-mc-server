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

pub struct Client<'a> {
    pub state: i32,
    pub stream: DataStream<'a>,
}

impl<'a> Client<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        // let mut stream = DataStream::new(stream);
        // let length: VarInt = stream.read().unwrap(); // TODO error handling
        // let packet_id: VarInt = stream.read().unwrap(); // TODO
        // let protocol_ver: VarInt = stream.read().unwrap(); // TODO
        // let s: PacketString = stream.read().unwrap(); // TODO
        // let port: UnsignedShort = stream.read().unwrap(); // TODO
        // let intent: VarInt = stream.read().unwrap(); // TODO
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
                    println!("{:?}", e);
                    // todo!();
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
                    match self
                        .stream
                        .write(StatusS2CPacket { status_str }.encode())
                        .await
                    {
                        Ok(_) => (),
                        Err(_) => todo!(),
                    };
                }
                ClientBoundPacket::Ping(packet) => {
                    self.stream
                        .write(
                            PongS2CPacket {
                                timestamp: packet.timestamp,
                            }
                            .encode(),
                        )
                        .await
                        .unwrap(); // TODO error handling
                }
                ClientBoundPacket::Status(packet) => {}
            }
        }
        /*
        loop {
            println!("Reading length...");
            let length: VarInt = self.stream.read().unwrap(); // TODO error handling
            println!("Read length!");
            let packet_id: VarInt = self.stream.read().unwrap(); // TODO
            println!("packet id: {}", packet_id.0);
            if packet_id.0 == 1 {
                let timestamp: Long = self.stream.read().unwrap(); // TODO
                let timestamp_backup = timestamp;
                let mut response = Vec::<u8>::new();
                response.append(&mut timestamp.encode());
                self.stream.write(response).unwrap(); // TODO
                println!("Wrote pong response: {}", timestamp_backup.0);
            } else if packet_id.0 == 0 && self.state == 1 {
                let mut response = Vec::<u8>::new();
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
                let mut packet_status_str = PacketString(status_str).encode();
                response.append(&mut VarInt(0).encode());
                response.append(&mut packet_status_str);
                self.stream.write(response).unwrap(); // TODO
                println!("Wrote response");
            } else {
                println!("State is {}. Ending processing", self.state);
            }
        }
        */
    }
}
