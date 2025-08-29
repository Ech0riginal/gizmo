use super::prelude::*;

pub struct ByStep {
    params: List<GValue>,
}

impl ByStep {
    fn new(params: List<GValue>) -> Self {
        ByStep { params }
    }
}

impl From<ByStep> for List<GValue> {
    fn from(step: ByStep) -> Self {
        step.params
    }
}

impl From<()> for ByStep {
    fn from(_: ()) -> Self {
        ByStep::new(list![])
    }
}

impl From<&str> for ByStep {
    fn from(param: &str) -> Self {
        ByStep::new(list![String::from(param).into()])
    }
}

impl From<Order> for ByStep {
    fn from(param: Order) -> Self {
        ByStep::new(list![param.into()])
    }
}

impl From<Column> for ByStep {
    fn from(value: Column) -> Self {
        ByStep::new(list![value.into()])
    }
}

impl From<T> for ByStep {
    fn from(param: T) -> Self {
        ByStep::new(list![param.into()])
    }
}

impl From<(&str, Order)> for ByStep {
    fn from(param: (&str, Order)) -> Self {
        ByStep::new(list![param.0.into(), param.1.into()])
    }
}

impl From<(String, Order)> for ByStep {
    fn from(param: (String, Order)) -> Self {
        ByStep::new(list![param.0.into(), param.1.into()])
    }
}

impl From<(TraversalBuilder, Order)> for ByStep {
    fn from(param: (TraversalBuilder, Order)) -> Self {
        ByStep::new(list![param.0.bytecode.into(), param.1.into()])
    }
}

impl From<TraversalBuilder> for ByStep {
    fn from(param: TraversalBuilder) -> Self {
        ByStep::new(list![param.bytecode.into()])
    }
}
