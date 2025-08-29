use super::prelude::*;

pub struct MergeEdgeStep {
    params: List<GValue>,
}

impl MergeEdgeStep {
    fn new(params: List<GValue>) -> Self {
        MergeEdgeStep { params }
    }
}

impl From<MergeEdgeStep> for List<GValue> {
    fn from(step: MergeEdgeStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for MergeEdgeStep {
    fn from(param: TraversalBuilder) -> Self {
        MergeEdgeStep::new(list![param.bytecode.into()])
    }
}

impl From<Map<GValue, GValue>> for MergeEdgeStep {
    fn from(value: Map<GValue, GValue>) -> Self {
        MergeEdgeStep::new(list![value.into()])
    }
}

impl<K, V> From<(K, V)> for MergeEdgeStep
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
