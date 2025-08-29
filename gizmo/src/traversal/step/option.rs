use gizmio::*;

use crate::traversal::TraversalBuilder;

pub struct OptionStep {
    params: List<GValue>,
}

impl OptionStep {
    fn new(params: List<GValue>) -> Self {
        OptionStep { params }
    }
}

impl From<OptionStep> for List<GValue> {
    fn from(step: OptionStep) -> Self {
        step.params
    }
}

impl From<(GValue, TraversalBuilder)> for OptionStep {
    fn from(value: (GValue, TraversalBuilder)) -> Self {
        OptionStep::new(list![value.0.into(), value.1.into()])
    }
}

impl From<(Merge, TraversalBuilder)> for OptionStep {
    fn from(value: (Merge, TraversalBuilder)) -> Self {
        OptionStep::new(list![value.0.into(), value.1.into()])
    }
}

impl From<(Merge, Map<GValue, GValue>)> for OptionStep {
    fn from(value: (Merge, Map<GValue, GValue>)) -> Self {
        OptionStep::new(list![value.0.into(), value.1.into()])
    }
}
