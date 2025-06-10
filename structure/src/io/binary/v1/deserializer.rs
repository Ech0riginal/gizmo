use crate::*;
use crate::GValue;
use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};

pub struct BinaryV1;
pub struct BinaryV1Deserializer<'a> {
    input: &'a [u8],
}

impl BinaryV1 {
    pub fn deserialize<'a>(bytes: &'a [u8]) -> BinaryV1Deserializer<'a> {
        BinaryV1Deserializer { input: bytes }
    }
}

impl<'a> BinaryV1Deserializer<'a> {
    fn deserialize(&'a self) -> GremlinResult<GValue> {
        Ok(GValue::Null)
    }
}

impl FromResidual for BinaryV1Deserializer<'_> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        todo!("i am but a compiler flag")
    }
}

impl<'a> Try for BinaryV1Deserializer<'a> {
    type Output = GValue;
    type Residual = GremlinResult<Infallible>;

    fn from_output(output: Self::Output) -> Self {
        todo!("i am but a compiler flag")
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.deserialize() {
            Ok(gvalue) => ControlFlow::Continue(gvalue),
            Err(error) => ControlFlow::Break(Err(error)),
        }
    }
}

#[cfg(test)]
#[test]
fn what_do() -> GremlinResult<()> {
    let bytes = &[0, 0, 0, 0];
    let bits = BinaryV1::deserialize(bytes)?;
    Ok(())
}
