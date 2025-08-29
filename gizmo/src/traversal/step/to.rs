use super::prelude::*;

pub struct ToStep {
    params: List<GValue>,
}

impl ToStep {
    fn new(params: List<GValue>) -> Self {
        ToStep { params }
    }
}

impl From<ToStep> for List<GValue> {
    fn from(step: ToStep) -> Self {
        step.params
    }
}

impl From<&str> for ToStep {
    fn from(param: &str) -> Self {
        ToStep::new(list![param.into()])
    }
}

impl From<&Vertex> for ToStep {
    fn from(param: &Vertex) -> Self {
        ToStep::new(list![param.into()])
    }
}

impl From<TraversalBuilder> for ToStep {
    fn from(param: TraversalBuilder) -> Self {
        ToStep::new(list![param.bytecode.into()])
    }
}
