use super::prelude::*;

pub struct LimitStep {
    limit: GValue,
    scope: Option<Scope>,
}

impl LimitStep {
    fn new(limit: GValue, scope: Option<Scope>) -> Self {
        LimitStep { limit, scope }
    }
}

impl From<LimitStep> for List<GValue> {
    fn from(step: LimitStep) -> Self {
        let mut params = step
            .scope
            .map(|m| match m {
                Scope::Global => list![String::from("Global").into()],
                Scope::Local => list![String::from("Local").into()],
            })
            .unwrap_or_else(List::new);

        params.push(step.limit);
        params
    }
}

impl From<i64> for LimitStep {
    fn from(param: i64) -> LimitStep {
        Self::from(Long(param))
    }
}

impl From<Long> for LimitStep {
    fn from(param: Long) -> LimitStep {
        LimitStep::new(param.into(), None)
    }
}
