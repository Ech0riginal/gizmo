use super::prelude::*;

pub struct NotStep {
    params: List<GValue>,
}

impl NotStep {
    fn new(params: List<GValue>) -> Self {
        NotStep { params }
    }
}

impl From<NotStep> for List<GValue> {
    fn from(step: NotStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for NotStep {
    fn from(param: TraversalBuilder) -> Self {
        NotStep::new(list![param.bytecode.into()])
    }
}
