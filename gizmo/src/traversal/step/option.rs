use crate::process::traversal::TraversalBuilder;
use crate::*;
use std::collections::HashMap;

pub struct OptionStep {
    params: Vec<GValue>,
}

impl OptionStep {
    fn new(params: Vec<GValue>) -> Self {
        OptionStep { params }
    }
}

impl From<OptionStep> for Vec<GValue> {
    fn from(step: OptionStep) -> Self {
        step.params
    }
}

impl From<(GValue, TraversalBuilder)> for OptionStep {
    fn from(value: (GValue, TraversalBuilder)) -> Self {
        OptionStep::new(vec![value.0.into(), value.1.into()])
    }
}

impl From<(Merge, TraversalBuilder)> for OptionStep {
    fn from(value: (Merge, TraversalBuilder)) -> Self {
        OptionStep::new(vec![value.0.into(), value.1.into()])
    }
}

impl From<(Merge, HashMap<GKey, GValue>)> for OptionStep {
    fn from(value: (Merge, HashMap<GKey, GValue>)) -> Self {
        OptionStep::new(vec![value.0.into(), value.1.into()])
    }
}
