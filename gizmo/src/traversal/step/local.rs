use super::prelude::*;

pub struct LocalStep {
    params: List<GValue>,
}

impl LocalStep {
    fn new(params: List<GValue>) -> Self {
        LocalStep { params }
    }
}

impl From<LocalStep> for List<GValue> {
    fn from(step: LocalStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for LocalStep {
    fn from(param: TraversalBuilder) -> LocalStep {
        LocalStep::new(list![param.bytecode.into()])
    }
}
