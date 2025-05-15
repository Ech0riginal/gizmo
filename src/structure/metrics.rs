use crate::structure::{Double, Long};

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

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TraversalMetrics {
    duration: Double,
    metrics: Vec<Metrics>,
}

impl TraversalMetrics {
    pub fn duration(&self) -> &f64 {
        &self.duration.0
    }

    pub fn metrics(&self) -> &Vec<Metrics> {
        &self.metrics
    }
}

impl TraversalMetrics {
    pub fn new(duration: Double, metrics: Vec<Metrics>) -> Self {
        TraversalMetrics { duration, metrics }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Metrics {
    id: String,
    duration: Double,
    name: String,
    count: Long,
    traversers: Long,
    perc_duration: Double,
    nested: Vec<Metrics>,
}

getters!(
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
        nested: Vec<Metrics>,
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
