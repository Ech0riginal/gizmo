use std::collections::HashMap;

use crate::prelude::{GKey, GValue, traversal::TraversalBuilder};

pub struct MergeEdgeStep {
    params: Vec<GValue>,
}

impl MergeEdgeStep {
    fn new(params: Vec<GValue>) -> Self {
        MergeEdgeStep { params }
    }
}

impl From<MergeEdgeStep> for Vec<GValue> {
    fn from(step: MergeEdgeStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for MergeEdgeStep {
    fn from(param: TraversalBuilder) -> Self {
        MergeEdgeStep::new(vec![param.bytecode.into()])
    }
}

impl From<HashMap<GKey, GValue>> for MergeEdgeStep {
    fn from(value: HashMap<GKey, GValue>) -> Self {
        MergeEdgeStep::new(vec![value.into()])
    }
}

impl<K, V> From<(K, V)> for MergeEdgeStep
where
    K: Into<GKey>,
    V: Into<GValue>,
{
    fn from(value: (K, V)) -> Self {
        let mut map = HashMap::<GKey, GValue>::new();
        map.insert(value.0.into(), value.1.into());
        Self::from(map)
    }
}
