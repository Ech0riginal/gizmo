use std::hash::{Hash, Hasher};

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Metrics {
    pub(crate) id: String,
    pub(crate) duration: Double,
    pub(crate) name: String,
    pub(crate) elements: Long,
    pub(crate) traversers: Long,
    pub(crate) annotations: Map<String, GValue>,
    pub(crate) nested: List<Metrics>,
}

obj!(Metrics);
tag!(Metrics);

impl Hash for Metrics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.duration.hash(state);
        self.elements.hash(state);
        self.traversers.hash(state);
    }
}

crate::getters!(
    Metrics,
    id -> String,
    name -> String,
    duration -> Double,
    annotations -> Map<String, GValue>,
    elements -> Long,
    traversers -> Long
);

impl Metrics {
    pub fn new<I, N, D, C, T, A>(
        id: I,
        name: N,
        duration: D,
        count: C,
        traversers: T,
        annotations: A,
        nested: List<Metrics>,
    ) -> Self
    where
        I: Into<String>,
        N: Into<String>,
        D: Into<Double>,
        C: Into<Long>,
        T: Into<Long>,
        A: Into<Map<String, GValue>>,
    {
        Metrics {
            id: id.into(),
            name: name.into(),
            duration: duration.into(),
            elements: count.into(),
            traversers: traversers.into(),
            annotations: annotations.into(),
            nested,
        }
    }
}
/*
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
*/
