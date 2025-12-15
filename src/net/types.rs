use std::io;

pub mod long;
pub mod string;
pub mod unsigned_short;
pub mod uuid;
pub mod varint;

#[macro_export]
macro_rules! try_read {
    ($stream:expr) => {
        match ($stream).read_byte().await {
            Ok(data) => data,
            Err(e) => return Err($crate::net::types::DecodeError::ReadError(e)),
        }
    };
}

#[derive(Debug)]
pub enum DecodeError {
    UnexpectedData,
    ReadError(io::Error),
}
