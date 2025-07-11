use crate::GValue;
use crate::process::traversal::step::*;
use crate::process::traversal::{Terminator, TraversalBuilder, WRITE_OPERATORS};
use crate::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct GraphTraversal<S, E, T> {
    start: PhantomData<S>,
    end: PhantomData<E>,
    pub(crate) builder: TraversalBuilder,
    terminator: T,
}

impl<S, E, T> GraphTraversal<S, E, T>
where
    E: From<GValue>,
    T: Terminator<E>,
{
    pub fn new(terminator: T, builder: TraversalBuilder) -> GraphTraversal<S, E, T> {
        GraphTraversal {
            start: PhantomData,
            end: PhantomData,
            builder,
            terminator,
        }
    }

    pub fn does_write(&self) -> bool {
        self.bytecode()
            .steps()
            .iter()
            .any(|instruction| WRITE_OPERATORS.contains(&&*instruction.operator.as_ref()))
    }

    pub fn bytecode(&self) -> &Bytecode {
        &self.builder.bytecode
    }

    pub fn has_label<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.builder = self.builder.has_label(labels);
        self
    }

    pub fn add_v<A>(mut self, label: A) -> GraphTraversal<Vertex, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.add_v(label);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn property<K, A>(mut self, key: K, value: A) -> Self
    where
        K: Into<GValue>,
        A: Into<GValue>,
    {
        self.builder = self.builder.property(key.into(), value);
        self
    }

    pub fn property_opt<K, A>(mut self, key: K, value: &Option<A>) -> Self
    where
        K: Into<GValue>,
        A: Into<GValue>,
        GValue: for<'a> From<&'a A>,
    {
        if let Some(a) = value {
            self.builder = self.builder.property(key, a);
        }

        self
    }

    pub fn property_with_cardinality<K, V>(
        mut self,
        cardinality: Cardinality,
        key: K,
        value: V,
    ) -> Self
    where
        K: Into<GValue>,
        V: Into<GValue>,
    {
        self.builder = self
            .builder
            .property_with_cardinality(cardinality, key, value);
        self
    }

    pub fn property_many<K, V>(mut self, values: Vec<(K, V)>) -> Self
    where
        K: Into<GValue>,
        V: Into<GValue>,
    {
        for property in values {
            self.builder = self.builder.property(property.0, property.1)
        }

        self
    }

    pub fn property_many_with_cardinality<K, V>(mut self, values: Vec<(Cardinality, K, V)>) -> Self
    where
        K: Into<GValue>,
        V: Into<GValue>,
    {
        for property in values {
            self.builder = self
                .builder
                .property_with_cardinality(property.0, property.1, property.2);
        }

        self
    }

    pub fn has<A>(mut self, step: A) -> Self
    where
        A: Into<HasStep>,
    {
        self.builder = self.builder.has(step);

        self
    }

    pub fn has_many<A>(mut self, steps: Vec<A>) -> Self
    where
        A: Into<HasStep>,
    {
        self.builder = self.builder.has_many(steps);

        self
    }

    pub fn has_not<A>(mut self, key: A) -> Self
    where
        A: Into<String>,
    {
        self.builder = self.builder.has_not(key);
        self
    }
    pub fn as_<A>(mut self, alias: A) -> Self
    where
        A: Into<String>,
    {
        self.builder = self.builder.as_(alias);

        self
    }

    pub fn with_side_effect<A>(mut self, step: (&'static str, A)) -> Self
    where
        A: Into<GValue> + From<GValue>,
    {
        self.builder = self.builder.with_side_effect(step);

        self
    }

    pub fn side_effect<A>(mut self, step: A) -> Self
    where
        A: Into<SideEffectStep>,
    {
        self.builder = self.builder.side_effect(step);

        self
    }

    pub fn add_e<A>(mut self, label: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<Labels>,
        T: Terminator<Edge>,
    {
        self.builder = self.builder.add_e(label);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn out<A>(mut self, labels: A) -> GraphTraversal<S, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.out(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn out_e<A>(mut self, labels: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<Labels>,
        T: Terminator<Edge>,
    {
        self.builder = self.builder.out_e(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn out_v(mut self) -> GraphTraversal<S, Vertex, T>
    where
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.out_v();

        GraphTraversal::new(self.terminator, self.builder)
    }
    pub fn in_<A>(mut self, labels: A) -> GraphTraversal<S, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.in_(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn in_e<A>(mut self, labels: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<Labels>,
        T: Terminator<Edge>,
    {
        self.builder = self.builder.in_e(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn in_v(mut self) -> GraphTraversal<S, Vertex, T>
    where
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.in_v();

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn both<A>(mut self, labels: A) -> GraphTraversal<S, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.both(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn both_e<A>(mut self, labels: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<Labels>,
        T: Terminator<Edge>,
    {
        self.builder = self.builder.both_e(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn other(mut self) -> GraphTraversal<S, Vertex, T>
    where
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.other();

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn other_v(mut self) -> GraphTraversal<S, Vertex, T>
    where
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.other_v();

        GraphTraversal::new(self.terminator, self.builder)
    }

    ///Filters all objects from the traversal stream. Generally only useful for applying traversal sideffects and avoiding unwanted response I/O
    pub fn none(mut self) -> GraphTraversal<S, Null, T>
    where
        T: Terminator<Null>,
    {
        self.builder = self.builder.none();

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn label(mut self) -> GraphTraversal<S, String, T>
    where
        T: Terminator<String>,
    {
        self.builder = self.builder.label();

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn to_list(&self) -> T::List {
        self.terminator.to_list(self)
    }

    pub fn next(&self) -> T::Next {
        self.terminator.next(self)
    }
    pub fn has_next(&self) -> T::HasNext {
        self.terminator.has_next(self)
    }

    pub fn iter(&self) -> T::Iter {
        self.terminator.iter(self)
    }

    pub fn from<A>(mut self, target: A) -> Self
    where
        A: Into<FromStep>,
    {
        self.builder = self.builder.from(target);

        self
    }

    pub fn to<A>(mut self, target: A) -> Self
    where
        A: Into<ToStep>,
    {
        self.builder = self.builder.to(target);

        self
    }

    pub fn properties<L>(mut self, labels: L) -> GraphTraversal<S, GProperty, T>
    where
        L: Into<Labels>,
        T: Terminator<GProperty>,
    {
        self.builder = self.builder.properties(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn property_map<L>(mut self, labels: L) -> GraphTraversal<S, Map, T>
    where
        L: Into<Labels>,
        T: Terminator<Map>,
    {
        self.builder = self.builder.property_map(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn values<L>(mut self, labels: L) -> GraphTraversal<S, GValue, T>
    where
        L: Into<Labels>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.values(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn value_map<L>(mut self, labels: L) -> GraphTraversal<S, Map, T>
    where
        L: Into<Labels>,
        T: Terminator<Map>,
    {
        self.builder = self.builder.value_map(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn element_map<L>(mut self, labels: L) -> GraphTraversal<S, Map, T>
    where
        L: Into<Labels>,
        T: Terminator<Map>,
    {
        self.builder = self.builder.element_map(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn count(mut self) -> GraphTraversal<S, Long, T>
    where
        T: Terminator<Long>,
    {
        self.builder = self.builder.count();
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn group_count(mut self) -> GraphTraversal<S, Map, T>
    where
        T: Terminator<Map>,
    {
        self.builder = self.builder.group_count(None);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn group_count_as<A>(mut self, key: A) -> GraphTraversal<S, E, T>
    where
        T: Terminator<Map>,
        A: Into<String>,
    {
        self.builder = self.builder.group_count(Some(key.into()));
        self
    }

    pub fn group(mut self) -> GraphTraversal<S, Map, T>
    where
        T: Terminator<Map>,
    {
        self.builder = self.builder.group(None);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn group_as<A>(mut self, key: A) -> GraphTraversal<S, E, T>
    where
        T: Terminator<Map>,
        A: Into<String>,
    {
        self.builder = self.builder.group(Some(key.into()));
        self
    }

    pub fn by<A>(mut self, step: A) -> Self
    where
        A: Into<ByStep>,
    {
        self.builder = self.builder.by(step);
        self
    }

    pub fn select<A>(mut self, step: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<SelectStep>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.select(step);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn fold(mut self) -> GraphTraversal<S, List, T>
    where
        T: Terminator<List>,
    {
        self.builder = self.builder.fold();
        GraphTraversal::new(self.terminator, self.builder)
    }
    pub fn unfold(mut self) -> Self {
        self.builder = self.builder.unfold();
        self
    }

    pub fn path(mut self) -> GraphTraversal<S, Path, T>
    where
        T: Terminator<Path>,
    {
        self.builder = self.builder.path();
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn limit<A>(mut self, limit: A) -> Self
    where
        A: Into<LimitStep>,
    {
        self.builder = self.builder.limit(limit);

        self
    }

    pub fn dedup<A>(mut self, dedup: A) -> Self
    where
        A: Into<DedupStep>,
    {
        self.builder = self.builder.dedup(dedup);
        self
    }

    pub fn sum<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.sum(scope);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn max<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.max(scope);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn mean<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.mean(scope);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn min<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.min(scope);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn is<A>(mut self, val: A) -> Self
    where
        A: IntoPredicate,
    {
        self.builder = self.builder.is(val);

        self
    }

    pub fn where_<A>(mut self, step: A) -> Self
    where
        A: Into<WhereStep>,
    {
        self.builder = self.builder.where_(step);

        self
    }

    pub fn not<A>(mut self, step: A) -> Self
    where
        A: Into<NotStep>,
    {
        self.builder = self.builder.not(step);
        self
    }

    pub fn order<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.builder = self.builder.order(scope);

        self
    }

    pub fn match_<A>(mut self, step: A) -> GraphTraversal<S, Map, T>
    where
        A: Into<MatchStep>,
        T: Terminator<Map>,
    {
        self.builder = self.builder.match_(step);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn drop(mut self) -> Self {
        self.builder = self.builder.drop();
        self
    }

    pub fn or<A>(mut self, step: A) -> Self
    where
        A: Into<OrStep>,
    {
        self.builder = self.builder.or(step);
        self
    }

    pub fn map<A>(mut self, step: A) -> Self
    where
        A: Into<ByStep>,
    {
        self.builder = self.builder.map(step);
        self
    }

    pub fn project<A>(mut self, step: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<SelectStep>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.project(step);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn v<VT>(mut self, ids: VT) -> Self
    where
        VT: Into<GIDs>,
    {
        self.builder = self.builder.v(ids);
        self
    }

    pub fn repeat<A>(mut self, step: A) -> Self
    where
        A: Into<RepeatStep>,
    {
        self.builder = self.builder.repeat(step);
        self
    }

    pub fn until<A>(mut self, step: A) -> Self
    where
        A: Into<UntilStep>,
    {
        self.builder = self.builder.until(step);
        self
    }

    pub fn simple_path(mut self) -> Self {
        self.builder = self.builder.simple_path();
        self
    }

    pub fn sample(mut self, step: i32) -> Self {
        self.builder = self.builder.sample(step);
        self
    }

    pub fn loops<A>(mut self, step: A) -> Self
    where
        A: Into<LoopsStep>,
    {
        self.builder = self.builder.loops(step);
        self
    }

    pub fn local<A>(mut self, step: A) -> Self
    where
        A: Into<LocalStep>,
    {
        self.builder = self.builder.local(step);
        self
    }

    pub fn aggregate<A>(mut self, alias: A) -> Self
    where
        A: Into<String>,
    {
        self.builder = self.builder.aggregate(alias);
        self
    }

    pub fn value(mut self) -> Self {
        self.builder = self.builder.value();

        self
    }

    pub fn choose<A>(mut self, step: A) -> Self
    where
        A: IntoChooseStep,
    {
        self.builder = self.builder.choose(step);
        self
    }

    pub fn coalesce<B, A>(mut self, colaesce: A) -> GraphTraversal<S, B, T>
    where
        A: Into<CoalesceStep>,
        B: From<GValue>,
        T: Terminator<B>,
    {
        self.builder = self.builder.coalesce(colaesce);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn merge_v<A>(mut self, merge_v: A) -> GraphTraversal<Vertex, Vertex, T>
    where
        A: Into<MergeVertexStep>,
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.merge_v(merge_v);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn merge_e<A>(mut self, merge_e: A) -> GraphTraversal<Edge, Edge, T>
    where
        A: Into<MergeEdgeStep>,
        T: Terminator<Edge>,
    {
        self.builder = self.builder.merge_e(merge_e);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn identity(mut self) -> Self {
        self.builder = self.builder.identity();
        self
    }

    pub fn range(mut self, step: i64, step2: i64) -> Self {
        self.builder = self.builder.range(step, step2);
        self
    }

    pub fn cap(mut self, step: &'static str) -> Self {
        self.builder = self.builder.cap(step);
        self
    }

    pub fn barrier(mut self) -> Self {
        self.builder = self.builder.barrier();
        self
    }

    pub fn option<A>(mut self, step: A) -> Self
    where
        A: Into<OptionStep>,
    {
        self.builder = self.builder.option(step);
        self
    }

    pub fn optional(mut self, step: TraversalBuilder) -> Self {
        self.builder = self.builder.optional(step);
        self
    }

    pub fn constant<A>(mut self, value: A) -> Self
    where
        A: Into<GValue>,
    {
        self.builder = self.builder.constant(value);
        self
    }

    pub fn emit(mut self) -> Self {
        self.builder = self.builder.emit();
        self
    }
}
