use super::prelude::*;

pub struct DedupStep {
    params: List<GValue>,
}

impl DedupStep {
    fn new(params: List<GValue>) -> Self {
        DedupStep { params }
    }
}

impl From<DedupStep> for List<GValue> {
    fn from(step: DedupStep) -> Self {
        step.params
    }
}

impl From<()> for DedupStep {
    fn from(_: ()) -> DedupStep {
        DedupStep::new(list![])
    }
}

impl From<&str> for DedupStep {
    fn from(param: &str) -> DedupStep {
        DedupStep::new(list![String::from(param).into()])
    }
}
