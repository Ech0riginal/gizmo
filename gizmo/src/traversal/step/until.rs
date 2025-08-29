use super::prelude::*;

pub struct UntilStep {
    params: List<GValue>,
}

impl UntilStep {
    fn new(params: List<GValue>) -> Self {
        UntilStep { params }
    }
}

impl From<UntilStep> for List<GValue> {
    fn from(step: UntilStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for UntilStep {
    fn from(param: TraversalBuilder) -> Self {
        UntilStep::new(list![param.bytecode.into()])
    }
}
