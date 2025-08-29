use super::prelude::*;

pub struct OrStep {
    params: List<GValue>,
}

impl OrStep {
    fn new(params: List<GValue>) -> Self {
        OrStep { params }
    }
}

impl From<OrStep> for List<GValue> {
    fn from(step: OrStep) -> Self {
        step.params
    }
}

impl From<()> for OrStep {
    fn from(_: ()) -> Self {
        OrStep::new(list![])
    }
}

impl From<TraversalBuilder> for OrStep {
    fn from(param: TraversalBuilder) -> Self {
        OrStep::new(list![param.bytecode.into()])
    }
}

impl From<Vec<TraversalBuilder>> for OrStep {
    fn from(param: Vec<TraversalBuilder>) -> Self {
        OrStep::new(param.into_iter().map(|s| s.bytecode.into()).collect())
    }
}

macro_rules! impl_into_or {
    ($n:expr) => {
        impl From<[TraversalBuilder; $n]> for OrStep {
            fn from(param: [TraversalBuilder; $n]) -> OrStep {
                OrStep::new(param.iter().map(|s| s.bytecode.clone().into()).collect())
            }
        }
    };
}

impl_into_or!(1);
impl_into_or!(2);
impl_into_or!(3);
impl_into_or!(4);
impl_into_or!(5);
impl_into_or!(6);
impl_into_or!(7);
impl_into_or!(8);
impl_into_or!(9);
impl_into_or!(10);
