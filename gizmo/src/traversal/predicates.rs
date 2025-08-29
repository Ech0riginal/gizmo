use gizmio::types::{Either2, GValue, P, Predicate, TextP};

pub trait IntoPredicate {
    fn into_predicate(self) -> Either2<P, TextP>;
}

impl<T: Into<GValue>> IntoPredicate for T {
    fn into_predicate(self) -> Either2<P, TextP> {
        let val: GValue = self.into();
        match val {
            GValue::P(ref p) => Either2::A(p.clone()),
            GValue::TextP(ref p) => Either2::B(p.clone()),
            _ => Either2::A(P::new(Predicate::Equal, val)),
        }
    }
}
