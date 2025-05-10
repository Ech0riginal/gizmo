use crate::structure::{Double, Long};

#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Metrics {
    id: String,
    duration: Double,
    name: String,
    count: Long,
    traversers: Long,
    perc_duration: Double,
    nested: Vec<Metrics>,
}

impl Metrics {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn duration(&self) -> &f64 {
        &self.duration
    }

    pub fn perc_duration(&self) -> &f64 {
        &self.perc_duration
    }
    pub fn count(&self) -> &i64 {
        &self.count
    }
    pub fn traversers(&self) -> &i64 {
        &self.traversers
    }
}

impl Metrics {
    pub fn new<T, V>(
        id: T,
        name: V,
        duration: Double,
        count: Long,
        traversers: Long,
        perc_duration: Double,
        nested: Vec<Metrics>,
    ) -> Self
    where
        T: Into<String>,
        V: Into<String>,
    {
        Metrics {
            id: id.into(),
            name: name.into(),
            duration,
            count,
            traversers,
            perc_duration,
            nested,
        }
    }
}
