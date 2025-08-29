use super::prelude::*;

pub struct SelectStep {
    params: List<GValue>,
}

impl SelectStep {
    fn new(params: List<GValue>) -> Self {
        SelectStep { params }
    }
}

impl From<SelectStep> for List<GValue> {
    fn from(step: SelectStep) -> Self {
        step.params
    }
}

impl From<&str> for SelectStep {
    fn from(param: &str) -> SelectStep {
        SelectStep::new(list![String::from(param).into()])
    }
}

impl From<Pop> for SelectStep {
    fn from(param: Pop) -> SelectStep {
        SelectStep::new(list![GValue::Pop(param)])
    }
}

impl From<List<&str>> for SelectStep {
    fn from(param: List<&str>) -> SelectStep {
        SelectStep::new(param.into_iter().map(GValue::from).collect())
    }
}

impl From<TraversalBuilder> for SelectStep {
    fn from(param: TraversalBuilder) -> SelectStep {
        SelectStep::new(list![param.bytecode.into()])
    }
}

impl<B> From<(Pop, B)> for SelectStep
where
    B: Into<GValue>,
{
    fn from(param: (Pop, B)) -> SelectStep {
        SelectStep::new(list![GValue::Pop(param.0), param.1.into()])
    }
}

macro_rules! impl_into_select {
    ($n:expr) => {
        impl<T: Clone> From<[T; $n]> for SelectStep
        where
            T: Into<String>,
        {
            fn from(param: [T; $n]) -> SelectStep {
                SelectStep::new(param.iter().map(|e| e.clone().into().into()).collect())
            }
        }
    };
}

impl_into_select!(1);
impl_into_select!(2);
impl_into_select!(3);
impl_into_select!(4);
impl_into_select!(5);
impl_into_select!(6);
impl_into_select!(7);
impl_into_select!(8);
impl_into_select!(9);
impl_into_select!(10);
