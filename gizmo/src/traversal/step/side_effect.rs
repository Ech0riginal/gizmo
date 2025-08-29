use super::prelude::*;

pub struct SideEffectStep {
    params: List<GValue>,
}

impl SideEffectStep {
    fn new(params: List<GValue>) -> Self {
        SideEffectStep { params }
    }
}

impl From<SideEffectStep> for List<GValue> {
    fn from(step: SideEffectStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for SideEffectStep {
    fn from(param: TraversalBuilder) -> Self {
        SideEffectStep::new(list![param.bytecode.into()])
    }
}
