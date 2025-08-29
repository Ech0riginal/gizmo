use super::prelude::*;

pub struct RepeatStep {
    params: List<GValue>,
}

impl RepeatStep {
    fn new(params: List<GValue>) -> Self {
        RepeatStep { params }
    }
}

impl From<RepeatStep> for List<GValue> {
    fn from(step: RepeatStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for RepeatStep {
    fn from(param: TraversalBuilder) -> RepeatStep {
        RepeatStep::new(list![param.bytecode.into()])
    }
}
