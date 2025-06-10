use crate::{Double, List, Long};

use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TraversalExplanation {
    final_t: Vec<String>,
    original: Vec<String>,
    intermediate: Vec<IntermediateRepr>,
}

impl TraversalExplanation {
    pub fn final_t(&self) -> &Vec<String> {
        &self.final_t
    }
    pub fn original(&self) -> &Vec<String> {
        &self.original
    }

    pub fn intermediate(&self) -> &Vec<IntermediateRepr> {
        &self.intermediate
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct IntermediateRepr {
    traversal: Vec<String>,
    strategy: String,
    category: String,
}

impl IntermediateRepr {
    pub fn new(traversal: Vec<String>, strategy: String, category: String) -> IntermediateRepr {
        IntermediateRepr {
            traversal,
            strategy,
            category,
        }
    }
}
impl TraversalExplanation {
    pub fn new(
        original: Vec<String>,
        final_t: Vec<String>,
        intermediate: Vec<IntermediateRepr>,
    ) -> TraversalExplanation {
        TraversalExplanation {
            final_t,
            original,
            intermediate,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TraversalMetrics {
    duration: Double,
    metrics: List<Metrics>,
}

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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Metrics {
    pub(crate) id: String,
    pub(crate) duration: Double,
    pub(crate) name: String,
    pub(crate) count: Long,
    pub(crate) traversers: Long,
    pub(crate) perc_duration: Double,
    pub(crate) nested: List<Metrics>,
}

impl Hash for Metrics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.duration.hash(state);
        self.count.hash(state);
    }
}

crate::getters!(
    Metrics,
    id -> String,
    name -> String,
    duration -> Double,
    perc_duration -> Double,
    count -> Long,
    traversers -> Long
);

impl Metrics {
    pub fn new<I, N, D, C, T, P>(
        id: I,
        name: N,
        duration: D,
        count: C,
        traversers: T,
        perc_duration: P,
        nested: List<Metrics>,
    ) -> Self
    where
        I: Into<String>,
        N: Into<String>,
        D: Into<Double>,
        C: Into<Long>,
        T: Into<Long>,
        P: Into<Double>,
    {
        Metrics {
            id: id.into(),
            name: name.into(),
            duration: duration.into(),
            count: count.into(),
            traversers: traversers.into(),
            perc_duration: perc_duration.into(),
            nested,
        }
    }
}
