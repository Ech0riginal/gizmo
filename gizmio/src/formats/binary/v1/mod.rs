use crate::formats::binary::prelude::*;
use crate::{Format, V1};

impl Format for GraphBinary<V1> {
    const mime: &'static str = "application/vnd.graphbinary-v1.0";
    type Serial = Bytes;
}

impl<T, D: Dialect> GraphBinaryDeserializer<T, D> for GraphBinary<V1> {
    fn deserialize(_val: &Bytes) -> Result<T, Error> {
        todo!("GraphBinary impl")
    }
}

impl<T, D: Dialect> GraphBinarySerializer<T, D> for GraphBinary<V1> {
    fn serialize(_val: &T) -> Result<Bytes, Error> {
        todo!("GraphBinary impl")
    }
}

impl Bytable for Bytes {
    fn into_bytes(self) -> Bytes {
        self
    }

    fn from_bytes(bytes: Bytes) -> Result<Self, Error> {
        Ok(bytes)
    }
}
