use super::prelude::*;

pub struct LoopsStep {
    params: List<GValue>,
}

impl LoopsStep {
    fn new(params: List<GValue>) -> Self {
        LoopsStep { params }
    }
}

impl From<LoopsStep> for List<GValue> {
    fn from(step: LoopsStep) -> Self {
        step.params
    }
}

impl From<()> for LoopsStep {
    fn from(_: ()) -> LoopsStep {
        LoopsStep::new(list![])
    }
}

impl From<&str> for LoopsStep {
    fn from(param: &str) -> LoopsStep {
        LoopsStep::new(list![param.into()])
    }
}

impl From<String> for LoopsStep {
    fn from(param: String) -> LoopsStep {
        LoopsStep::new(list![param.into()])
    }
}
