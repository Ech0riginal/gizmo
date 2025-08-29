use gizmio::types::*;
use gizmio::{Dialect, Format};

use crate::client::{GremlinClient, Supports};
use crate::traversal::remote::MockTerminator;
use crate::traversal::step::*;
use crate::traversal::{AsyncTerminator, GraphTraversal, Terminator, TraversalBuilder};

#[derive(Clone)]
pub struct GraphTraversalSource<A: Terminator<GValue>> {
    term: A,
}

impl<A: Terminator<GValue>> GraphTraversalSource<A> {
    pub fn new(terminator: A) -> GraphTraversalSource<A> {
        GraphTraversalSource { term: terminator }
    }

    pub fn empty() -> GraphTraversalSource<MockTerminator> {
        GraphTraversalSource::new(MockTerminator {})
    }

    pub fn with_remote<D: Dialect, F: Supports<D>>(
        &self,
        client: GremlinClient<D, F>,
    ) -> GraphTraversalSource<AsyncTerminator<D, F>>
    where
        <F as Format>::Serial: Send + Sync,
    {
        GraphTraversalSource {
            term: AsyncTerminator::new(client),
        }
    }

    pub fn v<T>(&self, ids: T) -> GraphTraversal<Vertex, Vertex, A>
    where
        T: Into<List<GID>>,
        A: Terminator<Vertex>,
    {
        let mut code = Bytecode::new();

        code.add_step(
            String::from("V"),
            ids.into().0.iter().map(|id| id.into()).collect(),
        );

        GraphTraversal::new(self.term.clone(), TraversalBuilder::new(code))
    }

    pub fn add_v<T>(&self, label: T) -> GraphTraversal<Vertex, Vertex, A>
    where
        T: Into<Labels>,
        A: Terminator<Vertex>,
    {
        let mut code = Bytecode::new();

        code.add_step(
            String::from("addV"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.term.clone(), TraversalBuilder::new(code))
    }

    pub fn add_e<T>(&self, label: T) -> GraphTraversal<Edge, Edge, A>
    where
        T: Into<Labels>,
        A: Terminator<Edge>,
    {
        let mut code = Bytecode::new();

        code.add_step(
            String::from("addE"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.term.clone(), TraversalBuilder::new(code))
    }

    pub fn e<T>(&self, ids: T) -> GraphTraversal<Edge, Edge, A>
    where
        T: Into<List<GID>>,
        A: Terminator<Edge>,
    {
        let mut code = Bytecode::new();

        code.add_step(
            String::from("E"),
            ids.into().0.iter().map(|id| id.into()).collect(),
        );

        GraphTraversal::new(self.term.clone(), TraversalBuilder::new(code))
    }

    pub fn with_side_effect<T>(&self, step: (&'static str, T)) -> GraphTraversal<GValue, GValue, A>
    where
        T: Into<GValue> + From<GValue>,
        A: Terminator<T>,
    {
        let mut code = Bytecode::new();

        code.add_source(
            String::from("withSideEffect"),
            list![step.0.into(), step.1.into()],
        );
        GraphTraversal::new(self.term.clone(), TraversalBuilder::new(code))
    }

    pub fn inject<T>(&self, injection: T) -> GraphTraversal<GValue, GValue, A>
    where
        T: Into<GValue>,
        A: Terminator<T>,
    {
        let mut code = Bytecode::new();

        code.add_step(String::from("inject"), list![injection.into()]);
        GraphTraversal::new(self.term.clone(), TraversalBuilder::new(code))
    }

    pub fn merge_v<V>(&self, merge_v: V) -> GraphTraversal<Vertex, Vertex, A>
    where
        V: Into<MergeVertexStep>,
        A: Terminator<Vertex>,
    {
        let mut code = Bytecode::new();

        code.add_step(String::from("mergeV"), merge_v.into().into());

        GraphTraversal::new(self.term.clone(), TraversalBuilder::new(code))
    }

    pub fn merge_e<V>(&self, merge_e: V) -> GraphTraversal<Edge, Edge, A>
    where
        V: Into<MergeEdgeStep>,
        A: Terminator<Edge>,
    {
        let mut code = Bytecode::new();

        code.add_step(String::from("mergeE"), merge_e.into().into());

        GraphTraversal::new(self.term.clone(), TraversalBuilder::new(code))
    }
}

// TESTS
#[cfg(test)]
mod tests {

    use gizmio::Predicate;
    use gizmio::{GValue, Order, P, Scope, T, list};

    use super::GraphTraversalSource;
    use crate::traversal::remote::MockTerminator;
    use crate::traversal::{__, Bytecode};

    fn empty() -> GraphTraversalSource<MockTerminator> {
        GraphTraversalSource::new(MockTerminator {})
    }

    #[test]
    fn v_traversal() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);

        assert_eq!(&code, g.v(1).bytecode());
    }

    #[test]
    fn e_traversal() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("E"), list![1.into()]);

        assert_eq!(&code, g.e(1).bytecode());
    }
    #[test]
    fn v_has_label_traversal() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);
        code.add_step(
            String::from("hasLabel"),
            list![String::from("person").into()],
        );

        assert_eq!(&code, g.v(1).has_label("person").bytecode());
    }

    #[test]
    fn v_has_traversal() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);
        code.add_step(
            String::from("has"),
            list![
                String::from("name").into(),
                P::new(Predicate::Equal, String::from("marko").into()).into(),
            ],
        );
        code.add_step(
            String::from("has"),
            list![
                String::from("age").into(),
                P::new(Predicate::Equal, 23.into()).into()
            ],
        );

        assert_eq!(
            &code,
            g.v(1).has(("name", "marko")).has(("age", 23)).bytecode()
        );

        // has with 3 params

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("has"),
            list![
                String::from("person").into(),
                String::from("name").into(),
                P::new(Predicate::Equal, String::from("marko").into()).into(),
            ],
        );

        assert_eq!(&code, g.v(()).has(("person", "name", "marko")).bytecode());

        // has with 1 param

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("has"), list![String::from("name").into()]);

        assert_eq!(&code, g.v(()).has("name").bytecode());

        // hasNot

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("hasNot"), list![String::from("name").into()]);

        assert_eq!(&code, g.v(()).has_not("name").bytecode());
    }

    #[test]
    fn v_has_traversal_with_p() {
        let g = empty();

        // EQ
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);

        code.add_step(
            String::from("has"),
            list![String::from("age").into(), P::eq(23).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::eq(23))).bytecode());

        // NEQ
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);

        code.add_step(
            String::from("has"),
            list![String::from("age").into(), P::neq(23).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::neq(23))).bytecode());

        // GTE
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);

        code.add_step(
            String::from("has"),
            list![String::from("age").into(), P::gte(23).into()],
        );

        assert_eq!(&code, g.v(1).has(("age", P::gte(23))).bytecode());

        // GT
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);

        code.add_step(
            String::from("has"),
            list![String::from("age").into(), P::gt(23).into()],
        );

        assert_eq!(&code, g.v(1).has(("age", P::gt(23))).bytecode());

        // LTE
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);

        code.add_step(
            String::from("has"),
            list![String::from("age").into(), P::lte(23).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::lte(23))).bytecode());

        // LT
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);

        code.add_step(
            String::from("has"),
            list![String::from("age").into(), P::lt(23).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::lt(23))).bytecode());

        // Within
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);

        code.add_step(
            String::from("has"),
            list![String::from("age").into(), P::within((23, 26)).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::within((23, 26)))).bytecode());

        // IS
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![1.into()]);
        code.add_step(String::from("values"), list!["age".into()]);
        code.add_step(String::from("is"), list![P::eq(23).into()]);

        assert_eq!(&code, g.v(1).values("age").is(23).bytecode());
    }
    #[test]
    fn add_v_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("addV"), list![String::from("person").into()]);

        assert_eq!(&code, g.add_v("person").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("addV"), list![]);

        assert_eq!(&code, g.add_v(()).bytecode());
    }

    #[test]
    fn add_v_with_property_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("addV"), list![String::from("person").into()]);
        code.add_step(
            String::from("property"),
            list![String::from("name").into(), String::from("marko").into()],
        );

        assert_eq!(
            &code,
            g.add_v("person").property("name", "marko").bytecode()
        );
    }

    #[test]
    fn add_e_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("addE"), list![String::from("knows").into()]);

        assert_eq!(&code, g.add_e("knows").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("addE"), list![String::from("knows").into()]);
        code.add_step(String::from("from"), list![String::from("a").into()]);
        code.add_step(String::from("to"), list![String::from("b").into()]);

        assert_eq!(&code, g.add_e("knows").from("a").to("b").bytecode());
    }

    #[test]
    fn add_e_test_with_traversal() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("addE"), list![String::from("knows").into()]);
        code.add_step(
            String::from("from"),
            list![__.v(1).bytecode().clone().into()],
        );
        code.add_step(String::from("to"), list![__.v(2).bytecode().clone().into()]);

        assert_eq!(&code, g.add_e("knows").from(__.v(1)).to(__.v(2)).bytecode());
    }

    #[test]
    fn as_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("as"), list![String::from("a").into()]);

        assert_eq!(&code, g.v(()).as_("a").bytecode());
    }

    #[test]
    fn label_step_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("label"), list![]);

        assert_eq!(&code, g.v(()).label().bytecode());
    }

    #[test]
    fn properties_step_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("properties"), list![]);

        assert_eq!(&code, g.v(()).properties(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("properties"),
            list![String::from("name").into()],
        );

        assert_eq!(&code, g.v(()).properties("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("properties"),
            list![String::from("name").into(), String::from("surname").into()],
        );

        // with vec
        assert_eq!(
            &code,
            g.v(()).properties(list!["name", "surname"]).bytecode()
        );

        // without vec
        assert_eq!(&code, g.v(()).properties(["name", "surname"]).bytecode());
    }

    #[test]
    fn property_map_step_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("propertyMap"), list![]);

        assert_eq!(&code, g.v(()).property_map(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("propertyMap"),
            list![String::from("name").into()],
        );

        assert_eq!(&code, g.v(()).property_map("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("propertyMap"),
            list![String::from("name").into(), String::from("surname").into()],
        );

        // with vec
        assert_eq!(
            &code,
            g.v(()).property_map(list!["name", "surname"]).bytecode()
        );

        // without vec
        assert_eq!(&code, g.v(()).property_map(["name", "surname"]).bytecode());
    }

    #[test]
    fn values_step_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list![]);

        assert_eq!(&code, g.v(()).values(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list![String::from("name").into()]);

        assert_eq!(&code, g.v(()).values("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("values"),
            list![String::from("name").into(), String::from("surname").into()],
        );

        // with vec
        assert_eq!(&code, g.v(()).values(list!["name", "surname"]).bytecode());

        // without vec
        assert_eq!(&code, g.v(()).values(["name", "surname"]).bytecode());
    }

    #[test]
    fn value_map_step_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("valueMap"), list![]);

        assert_eq!(&code, g.v(()).value_map(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("valueMap"), list![String::from("name").into()]);

        assert_eq!(&code, g.v(()).value_map("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("valueMap"),
            list![String::from("name").into(), String::from("surname").into()],
        );

        assert_eq!(
            &code,
            g.v(()).value_map(list!["name", "surname"]).bytecode()
        );

        assert_eq!(&code, g.v(()).value_map(["name", "surname"]).bytecode());
    }

    #[test]
    fn element_map_step_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("elementMap"), list![]);

        assert_eq!(&code, g.v(()).element_map(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("elementMap"),
            list![String::from("name").into()],
        );

        assert_eq!(&code, g.v(()).element_map("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("elementMap"),
            list![String::from("name").into(), String::from("surname").into()],
        );

        assert_eq!(
            &code,
            g.v(()).element_map(list!["name", "surname"]).bytecode()
        );

        assert_eq!(&code, g.v(()).element_map(["name", "surname"]).bytecode());
    }

    #[test]
    fn count_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("count"), list![]);

        assert_eq!(&code, g.v(()).count().bytecode());
    }

    #[test]
    fn group_count_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("groupCount"), list![]);

        assert_eq!(&code, g.v(()).group_count().bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("groupCount"), list!["m".into()]);

        assert_eq!(&code, g.v(()).group_count_as("m").bytecode());
    }

    #[test]
    fn group_count_by_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("groupCount"), list![]);
        code.add_step(String::from("by"), list![]);

        assert_eq!(&code, g.v(()).group_count().by(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("groupCount"), list![]);
        code.add_step(String::from("by"), list!["name".into()]);

        assert_eq!(&code, g.v(()).group_count().by("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("groupCount"), list![]);
        code.add_step(String::from("by"), list![T::Label.into()]);

        assert_eq!(&code, g.v(()).group_count().by(T::Label).bytecode());
    }

    #[test]
    fn group_by_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("group"), list![]);
        code.add_step(String::from("by"), list![]);

        assert_eq!(&code, g.v(()).group().by(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("group"), list![]);
        code.add_step(String::from("by"), list!["name".into()]);

        assert_eq!(&code, g.v(()).group().by("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("group"), list![]);
        code.add_step(String::from("by"), list![T::Label.into()]);

        assert_eq!(&code, g.v(()).group().by(T::Label).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("group"), list![]);
        code.add_step(String::from("by"), list![T::Label.into()]);
        code.add_step(
            String::from("by"),
            list![__.count().bytecode().clone().into()],
        );

        assert_eq!(
            &code,
            g.v(()).group().by(T::Label).by(__.count()).bytecode()
        );

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("group"), list!["m".into()]);
        code.add_step(String::from("by"), list![T::Label.into()]);

        assert_eq!(&code, g.v(()).group_as("m").by(T::Label).bytecode());
    }

    #[test]
    fn select_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("select"), list!["name".into()]);

        assert_eq!(&code, g.v(()).select("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("select"),
            list!["name".into(), "surname".into()],
        );

        assert_eq!(&code, g.v(()).select(list!["name", "surname"]).bytecode());

        assert_eq!(&code, g.v(()).select(["name", "surname"]).bytecode());
    }

    #[test]
    fn fold_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list!["name".into()]);
        code.add_step(String::from("fold"), list![]);

        assert_eq!(&code, g.v(()).values("name").fold().bytecode());
    }

    #[test]
    fn unfold_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("propertyMap"), list![]);
        code.add_step(String::from("unfold"), list![]);

        assert_eq!(&code, g.v(()).property_map(()).unfold().bytecode());
    }

    #[test]
    fn path_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("path"), list![]);

        assert_eq!(&code, g.v(()).path().bytecode());
    }

    #[test]
    fn limit_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("limit"), list![GValue::Long(1.into())]);

        assert_eq!(&code, g.v(()).limit(1).bytecode());
    }

    #[test]
    fn dedup_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list![]);
        code.add_step(String::from("dedup"), list![]);

        assert_eq!(&code, g.v(()).values(()).dedup(()).bytecode());
    }

    #[test]
    fn numerical_test() {
        let g = empty();

        // sum
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list!["test".into()]);
        code.add_step(String::from("sum"), list![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("test").sum(()).bytecode());

        // max
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list!["test".into()]);
        code.add_step(String::from("max"), list![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("test").max(()).bytecode());

        // mean

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list!["test".into()]);
        code.add_step(String::from("mean"), list![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("test").mean(()).bytecode());

        // min

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list!["test".into()]);
        code.add_step(String::from("min"), list![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("test").min(()).bytecode());
    }

    #[test]
    fn where_test() {
        let g = empty();

        // sum
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list!["age".into()]);
        code.add_step(String::from("where"), list![P::eq(23).into()]);

        assert_eq!(&code, g.v(()).values("age").where_(P::eq(23)).bytecode());
    }

    #[test]
    fn not_test() {
        let g = empty();

        // sum
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("not"),
            list![__.has_label("person").bytecode().clone().into()],
        );

        assert_eq!(&code, g.v(()).not(__.has_label("person")).bytecode());
    }

    #[test]
    fn order_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list!["name".into()]);
        code.add_step(String::from("order"), list![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("name").order(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("values"), list!["name".into()]);
        code.add_step(String::from("order"), list![Scope::Global.into()]);
        code.add_step(String::from("by"), list![Order::Desc.into()]);

        assert_eq!(
            &code,
            g.v(()).values("name").order(()).by(Order::Desc).bytecode()
        );
    }

    #[test]
    fn match_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(
            String::from("match"),
            list![
                __.as_("a").out(()).as_("b").bytecode().clone().into(),
                __.as_("b").out(()).as_("c").bytecode().clone().into(),
            ],
        );
        code.add_step(String::from("select"), list!["a".into(), "c".into()]);

        // with vec
        assert_eq!(
            &code,
            g.v(())
                .match_(list![
                    __.as_("a").out(()).as_("b"),
                    __.as_("b").out(()).as_("c")
                ])
                .select(list!["a", "c"])
                .bytecode()
        );

        // without vec
        assert_eq!(
            &code,
            g.v(())
                .match_([__.as_("a").out(()).as_("b"), __.as_("b").out(()).as_("c")])
                .select(["a", "c"])
                .bytecode()
        );
    }

    #[test]
    fn drop_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("drop"), list![]);

        assert_eq!(&code, g.v(()).drop().bytecode());
    }

    #[test]
    fn or_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("or"), list![]);

        assert_eq!(&code, g.v(()).or(()).bytecode());
    }

    #[test]
    fn coalesce_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), list![]);
        code.add_step(String::from("hasLabel"), list!["Person".into()]);
        code.add_step(
            String::from("coalesce"),
            list![
                __.values("nickname").bytecode().clone().into(),
                __.values("name").bytecode().clone().into(),
            ],
        );

        assert_eq!(
            &code,
            g.v(())
                .has_label("Person")
                .coalesce::<GValue, _>([__.values("nickname"), __.values("name")])
                .bytecode()
        );
    }

    #[test]
    fn inject_test() {
        let g = empty();

        let mut code = Bytecode::new();

        code.add_step(
            String::from("inject"),
            list![GValue::List(list!["foo".into(), "bar".into()].into())],
        );
        code.add_step(String::from("unfold"), list![]);

        assert_eq!(
            &code,
            g.inject(list!["foo".into(), "bar".into()])
                .unfold()
                .bytecode()
        );
    }

    //TODO add tests for mergeV, etc
}
