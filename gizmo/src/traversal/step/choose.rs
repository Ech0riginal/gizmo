use super::prelude::*;

pub trait IntoChooseStep {
    fn into_step(self) -> List<GValue>;
}

impl IntoChooseStep for TraversalBuilder {
    fn into_step(self) -> List<GValue> {
        list![self.bytecode.into()]
    }
}

impl IntoChooseStep for (TraversalBuilder, TraversalBuilder) {
    fn into_step(self) -> List<GValue> {
        let mut out = list![];
        out.append(&mut list![self.0.bytecode.into()]);
        out.append(&mut list![self.1.bytecode.into()]);
        out
    }
}

impl IntoChooseStep for (TraversalBuilder, TraversalBuilder, TraversalBuilder) {
    fn into_step(self) -> List<GValue> {
        let mut out = list![];
        out.append(&mut list![self.0.bytecode.into()]);
        out.append(&mut list![self.1.bytecode.into()]);
        out.append(&mut list![self.2.bytecode.into()]);
        out
    }
}
