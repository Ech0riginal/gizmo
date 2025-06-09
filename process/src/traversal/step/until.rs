use super::prelude::*;

pub struct UntilStep {
    params: Vec<GValue>,
}

impl UntilStep {
    fn new(params: Vec<GValue>) -> Self {
        UntilStep { params }
    }
}

impl From<UntilStep> for Vec<GValue> {
    fn from(step: UntilStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for UntilStep {
    fn from(param: TraversalBuilder) -> Self {
        UntilStep::new(vec![param.bytecode.into()])
    }
}
