use super::prelude::*;
use crate::traversal::predicates::IntoPredicate;

pub struct WhereStep {
    params: List<GValue>,
}

impl WhereStep {
    fn new(params: List<GValue>) -> Self {
        WhereStep { params }
    }
}

impl From<WhereStep> for List<GValue> {
    fn from(step: WhereStep) -> Self {
        step.params
    }
}

// impl<A, B> From<(A, B)> for WhereStep
// where
//     A: Into<String>,
//     B: IntoPredicate,
// {
//     fn from(param: (A, B)) -> WhereStep {
//         WhereStep::new(list![
//             param.0.into().into(),
//             param.1.into_predicate().into()
//         ])
//     }
// }

impl<A> From<A> for WhereStep
where
    A: IntoPredicate,
{
    fn from(param: A) -> WhereStep {
        WhereStep::new(list![param.into_predicate().into()])
    }
}
