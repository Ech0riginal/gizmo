use bytes::Bytes;
use crate::binary::GraphBinary;
use crate::{Format, GValue};
use crate::versions::V1;

impl Format for GraphBinary<V1> {
    const mime: &'static str = "application/vnd.graphbinary-v1.0";
    type Serial = Bytes;
    type Object = GValue;
}