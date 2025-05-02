use std::collections::HashMap;

use crate::prelude::{GKey, GValue, traversal::TraversalBuilder};

pub struct MergeVertexStep {
    params: Vec<GValue>,
}

impl MergeVertexStep {
    fn new(params: Vec<GValue>) -> Self {
        MergeVertexStep { params }
    }
}

impl From<MergeVertexStep> for Vec<GValue> {
    fn from(step: MergeVertexStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for MergeVertexStep {
    fn from(param: TraversalBuilder) -> Self {
        MergeVertexStep::new(vec![param.bytecode.into()])
    }
}

impl From<HashMap<GKey, GValue>> for MergeVertexStep {
    fn from(value: HashMap<GKey, GValue>) -> Self {
        MergeVertexStep::new(vec![value.into()])
    }
}

impl<K, V> From<(K, V)> for MergeVertexStep
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
