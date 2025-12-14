use crate::net::{datastream::DataStream, types::DecodeError};

pub trait Encode {
    fn encode(&self) -> Vec<u8>;
    // default implementation for write(), utilizing encode()
    async fn write(&self, stream: &mut DataStream<'_>) -> Result<(), std::io::Error> {
        stream.write(self.encode()).await
    }
}

pub trait Decode: Sized {
    async fn decode(stream: &mut DataStream<'_>) -> Result<Self, DecodeError>;
}
