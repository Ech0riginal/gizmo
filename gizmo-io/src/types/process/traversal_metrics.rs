use crate::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TraversalMetrics {
    pub(crate) duration: Double,
    pub(crate) metrics: List<Metrics>,
}

obj!(TraversalMetrics);
tag!(TraversalMetrics);

impl Hash for TraversalMetrics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for m in self.metrics.iter() {
            m.hash(state);
        }
    }
}

impl TraversalMetrics {
    pub fn duration(&self) -> &f64 {
        &self.duration.0
    }

    pub fn metrics(&self) -> &List<Metrics> {
        &self.metrics
    }
}

impl TraversalMetrics {
    pub fn new(duration: Double, metrics: List<Metrics>) -> Self {
        TraversalMetrics { duration, metrics }
    }
}
