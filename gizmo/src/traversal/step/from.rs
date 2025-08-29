use super::prelude::*;

pub struct FromStep {
    params: List<GValue>,
}

impl FromStep {
    fn new(params: List<GValue>) -> Self {
        FromStep { params }
    }
}

impl From<FromStep> for List<GValue> {
    fn from(step: FromStep) -> Self {
        step.params
    }
}

impl From<&str> for FromStep {
    fn from(param: &str) -> Self {
        FromStep::new(list![param.into()])
    }
}

impl From<&Vertex> for FromStep {
    fn from(param: &Vertex) -> Self {
        FromStep::new(list![param.into()])
    }
}

impl From<TraversalBuilder> for FromStep {
    fn from(param: TraversalBuilder) -> Self {
        FromStep::new(list![param.bytecode.into()])
    }
}
