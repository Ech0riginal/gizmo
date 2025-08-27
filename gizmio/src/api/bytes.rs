pub trait Bytable: std::fmt::Debug + Sized {
    fn into_bytes(self) -> bytes::Bytes;
    fn from_bytes(bytes: bytes::Bytes) -> Result<Self, crate::Error>;
}
