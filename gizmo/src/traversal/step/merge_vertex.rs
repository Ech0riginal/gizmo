use super::prelude::*;

pub struct MergeVertexStep {
    params: List<GValue>,
}

impl MergeVertexStep {
    fn new(params: List<GValue>) -> Self {
        MergeVertexStep { params }
    }
}

impl From<MergeVertexStep> for List<GValue> {
    fn from(step: MergeVertexStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for MergeVertexStep {
    fn from(param: TraversalBuilder) -> Self {
        MergeVertexStep::new(list![param.bytecode.into()])
    }
}

impl From<Map<GValue, GValue>> for MergeVertexStep {
    fn from(value: Map<GValue, GValue>) -> Self {
        MergeVertexStep::new(list![value.into()])
    }
}

impl<K, V> From<(K, V)> for MergeVertexStep
where
    K: Into<GValue>,
    V: Into<GValue>,
{
    fn from(value: (K, V)) -> Self {
        let mut map = Map::<GValue, GValue>::new();
        map.insert(value.0.into(), value.1.into());
        Self::from(map)
    }
}
