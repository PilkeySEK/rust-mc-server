use std::{
    io::{self, ErrorKind, Read, Write},
    thread,
    time::Duration,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::net::{
    traits::{Decode, Encode},
    types::varint::VarInt,
};

pub struct DataStream<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> DataStream<'a> {
    pub async fn read_byte(&mut self) -> Result<u8, io::Error> {
        let buf: &mut [u8; 1] = &mut [0];
        match self.stream.read_exact(buf).await {
            Ok(_) => return Ok(buf[0]),
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn write(&mut self, mut bytes: Vec<u8>) -> Result<(), std::io::Error> {
        let len = VarInt(bytes.len() as i32).encode();
        let mut response = len;
        response.append(&mut bytes);
        self.stream.write_all(&response).await
    }

    pub fn new(s: &'a mut TcpStream) -> Self {
        Self { stream: s }
    }

    pub async fn read<T: Decode>(&mut self) -> Result<T, super::types::DecodeError> {
        T::decode(self).await
    }
}
