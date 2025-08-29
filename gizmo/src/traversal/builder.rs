use gizmio::*;

use crate::traversal::predicates::IntoPredicate;
use crate::traversal::step::*;

#[derive(Clone)]
pub struct TraversalBuilder {
    pub(crate) bytecode: Bytecode,
}

impl Default for TraversalBuilder {
    fn default() -> Self {
        TraversalBuilder {
            bytecode: Bytecode::default(),
        }
    }
}

impl TraversalBuilder {
    pub fn new(bytecode: Bytecode) -> Self {
        TraversalBuilder { bytecode }
    }
    pub fn bytecode(&self) -> &Bytecode {
        &self.bytecode
    }

    pub fn v<T>(mut self, ids: T) -> TraversalBuilder
    where
        T: Into<List<GID>>,
    {
        self.bytecode.add_step(
            String::from("V"),
            ids.into().0.iter().map(|id| id.into()).collect(),
        );
        self
    }

    pub fn e<T>(mut self, ids: T) -> TraversalBuilder
    where
        T: Into<List<GID>>,
    {
        self.bytecode.add_step(
            String::from("E"),
            ids.into().0.iter().map(|id| id.into()).collect(),
        );
        self
    }

    pub fn has_label<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("hasLabel"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn add_v<A>(mut self, label: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("addV"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn property<K, A>(mut self, key: K, value: A) -> Self
    where
        K: Into<GValue>,
        A: Into<GValue>,
    {
        let args = list![key.into(), value.into()];
        self.bytecode.add_step(String::from("property"), args);
        self
    }

    pub fn property_many<K, A>(mut self, values: Vec<(K, A)>) -> Self
    where
        K: Into<GValue>,
        A: Into<GValue>,
    {
        for property in values {
            self.bytecode.add_step(
                String::from("property"),
                list![property.0.into(), property.1.into()],
            )
        }

        self
    }

    pub fn property_with_cardinality<K, A>(
        mut self,
        cardinality: Cardinality,
        key: K,
        value: A,
    ) -> Self
    where
        K: Into<GValue>,
        A: Into<GValue>,
    {
        self.bytecode.add_step(
            String::from("property"),
            list![cardinality.into(), key.into(), value.into()],
        );
        self
    }

    pub fn has<A>(mut self, step: A) -> Self
    where
        A: Into<HasStep>,
    {
        self.bytecode
            .add_step(String::from("has"), step.into().into());
        self
    }

    pub fn has_id(mut self, id: &GID) -> Self {
        self.bytecode
            .add_step(String::from("hasId"), list![id.into()]);
        self
    }

    pub fn side_effect<A>(mut self, step: A) -> Self
    where
        A: Into<SideEffectStep>,
    {
        self.bytecode
            .add_step(String::from("sideEffect"), step.into().into());
        self
    }

    pub fn with_side_effect<A>(mut self, step: (&'static str, A)) -> Self
    where
        A: Into<GValue> + From<GValue>,
    {
        self.bytecode.add_source(
            String::from("withSideEffect"),
            list![step.0.into(), step.1.into()],
        );
        self
    }

    pub fn has_many<A>(mut self, steps: Vec<A>) -> Self
    where
        A: Into<HasStep>,
    {
        for step in steps {
            self.bytecode
                .add_step(String::from("has"), step.into().into());
        }
        self
    }

    pub fn has_not<A>(mut self, key: A) -> Self
    where
        A: Into<String>,
    {
        self.bytecode
            .add_step(String::from("hasNot"), list![key.into().into()]);
        self
    }
    pub fn as_<A>(mut self, alias: A) -> Self
    where
        A: Into<String>,
    {
        self.bytecode
            .add_step(String::from("as"), list![alias.into().into()]);

        self
    }

    pub fn add_e<A>(mut self, label: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("addE"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn out<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("out"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn out_e<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("outE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn out_v(mut self) -> Self {
        self.bytecode.add_step(String::from("outV"), list![]);

        self
    }
    pub fn in_<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("in"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn in_e<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("inE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn in_v(mut self) -> Self {
        self.bytecode.add_step(String::from("inV"), list![]);

        self
    }

    pub fn both<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("both"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn both_e<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("bothE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn other(mut self) -> Self {
        self.bytecode.add_step(String::from("other"), list![]);

        self
    }

    pub fn other_v(mut self) -> Self {
        self.bytecode.add_step(String::from("otherV"), list![]);

        self
    }

    pub fn none(mut self) -> Self {
        self.bytecode.add_step(String::from("none"), list![]);

        self
    }

    pub fn label(mut self) -> Self {
        self.bytecode.add_step(String::from("label"), list![]);

        self
    }

    pub fn from<A>(mut self, step: A) -> Self
    where
        A: Into<FromStep>,
    {
        self.bytecode
            .add_step(String::from("from"), step.into().into());

        self
    }

    pub fn to<A>(mut self, step: A) -> Self
    where
        A: Into<ToStep>,
    {
        self.bytecode
            .add_step(String::from("to"), step.into().into());

        self
    }

    pub fn properties<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("properties"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn property_map<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("propertyMap"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn values<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("values"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn value_map<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("valueMap"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn element_map<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("elementMap"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn count(mut self) -> Self {
        self.bytecode.add_step(String::from("count"), list![]);
        self
    }

    pub fn group_count(mut self, key: Option<String>) -> Self {
        let mut params = list![];

        if let Some(k) = key {
            params.push(k.into());
        }
        self.bytecode.add_step(String::from("groupCount"), params);
        self
    }

    pub fn group(mut self, key: Option<String>) -> Self {
        let mut params = list![];

        if let Some(k) = key {
            params.push(k.into());
        }
        self.bytecode.add_step(String::from("group"), params);
        self
    }

    pub fn by<A>(mut self, step: A) -> Self
    where
        A: Into<ByStep>,
    {
        self.bytecode
            .add_step(String::from("by"), step.into().into());
        self
    }

    pub fn select<A>(mut self, step: A) -> Self
    where
        A: Into<SelectStep>,
    {
        self.bytecode
            .add_step(String::from("select"), step.into().into());
        self
    }

    pub fn fold(mut self) -> Self {
        self.bytecode.add_step(String::from("fold"), list![]);
        self
    }
    pub fn unfold(mut self) -> Self {
        self.bytecode.add_step(String::from("unfold"), list![]);
        self
    }

    pub fn path(mut self) -> Self {
        self.bytecode.add_step(String::from("path"), list![]);
        self
    }

    pub fn limit<A>(mut self, limit: A) -> Self
    where
        A: Into<LimitStep>,
    {
        self.bytecode
            .add_step(String::from("limit"), limit.into().into());

        self
    }

    pub fn dedup<A>(mut self, limit: A) -> Self
    where
        A: Into<DedupStep>,
    {
        self.bytecode
            .add_step(String::from("dedup"), limit.into().into());

        self
    }

    pub fn sum<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("sum"), list![scope.into().into()]);

        self
    }

    pub fn max<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("max"), list![scope.into().into()]);

        self
    }

    pub fn mean<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("mean"), list![scope.into().into()]);

        self
    }

    pub fn min<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("min"), list![scope.into().into()]);

        self
    }

    pub fn is<A>(mut self, val: A) -> Self
    where
        A: IntoPredicate,
    {
        self.bytecode
            .add_step(String::from("is"), list![val.into_predicate().into()]);

        self
    }

    pub fn where_<A>(mut self, step: A) -> Self
    where
        A: Into<WhereStep>,
    {
        self.bytecode
            .add_step(String::from("where"), step.into().into());
        self
    }

    pub fn not<A>(mut self, step: A) -> Self
    where
        A: Into<NotStep>,
    {
        self.bytecode
            .add_step(String::from("not"), step.into().into());
        self
    }

    pub fn order<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("order"), list![scope.into().into()]);

        self
    }

    pub fn match_<A>(mut self, step: A) -> Self
    where
        A: Into<MatchStep>,
    {
        self.bytecode
            .add_step(String::from("match"), step.into().into());
        self
    }

    pub fn drop(mut self) -> Self {
        self.bytecode.add_step(String::from("drop"), list![]);
        self
    }

    pub fn or<A>(mut self, step: A) -> Self
    where
        A: Into<OrStep>,
    {
        self.bytecode
            .add_step(String::from("or"), step.into().into());
        self
    }

    pub fn project<A>(mut self, step: A) -> Self
    where
        A: Into<SelectStep>,
    {
        self.bytecode
            .add_step(String::from("project"), step.into().into());
        self
    }

    pub fn map<A>(mut self, step: A) -> Self
    where
        A: Into<ByStep>,
    {
        self.bytecode
            .add_step(String::from("map"), step.into().into());
        self
    }

    pub fn repeat<A>(mut self, step: A) -> Self
    where
        A: Into<RepeatStep>,
    {
        self.bytecode
            .add_step(String::from("repeat"), step.into().into());

        self
    }

    pub fn until<A>(mut self, step: A) -> Self
    where
        A: Into<UntilStep>,
    {
        self.bytecode
            .add_step(String::from("until"), step.into().into());

        self
    }

    pub fn simple_path(mut self) -> Self {
        self.bytecode.add_step(String::from("simplePath"), list![]);

        self
    }

    pub fn sample(mut self, step: i32) -> Self {
        let repr = Integer(step);
        self.bytecode
            .add_step(String::from("sample"), list![repr.into()]);
        self
    }

    pub fn loops<A>(mut self, step: A) -> Self
    where
        A: Into<LoopsStep>,
    {
        self.bytecode
            .add_step(String::from("loops"), step.into().into());
        self
    }

    pub fn local<A>(mut self, step: A) -> Self
    where
        A: Into<LocalStep>,
    {
        self.bytecode
            .add_step(String::from("local"), step.into().into());
        self
    }

    pub fn aggregate<A>(mut self, alias: A) -> Self
    where
        A: Into<String>,
    {
        self.bytecode
            .add_step(String::from("aggregate"), list![alias.into().into()]);
        self
    }

    pub fn value(mut self) -> Self {
        self.bytecode.add_step(String::from("value"), list![]);
        self
    }

    pub fn choose<A>(mut self, step: A) -> Self
    where
        A: IntoChooseStep,
    {
        self.bytecode
            .add_step(String::from("choose"), step.into_step());
        self
    }

    pub fn coalesce<A>(mut self, coalesce: A) -> Self
    where
        A: Into<CoalesceStep>,
    {
        self.bytecode
            .add_step(String::from("coalesce"), coalesce.into().into());

        self
    }

    pub fn merge_v<A>(mut self, merge_v: A) -> Self
    where
        A: Into<MergeVertexStep>,
    {
        self.bytecode
            .add_step(String::from("mergeV"), merge_v.into().into());

        self
    }

    pub fn merge_e<A>(mut self, merge_e: A) -> Self
    where
        A: Into<MergeEdgeStep>,
    {
        self.bytecode
            .add_step(String::from("mergeE"), merge_e.into().into());

        self
    }

    pub fn option<A>(mut self, option: A) -> Self
    where
        A: Into<OptionStep>,
    {
        self.bytecode
            .add_step(String::from("option"), option.into().into());

        self
    }

    pub fn identity(mut self) -> Self {
        self.bytecode.add_step(String::from("identity"), list![]);
        self
    }

    pub fn range(mut self, step: i64, step2: i64) -> Self {
        let step1_repr = Long(step);
        let step2_repr = Long(step2);
        self.bytecode.add_step(
            String::from("range"),
            list![step1_repr.into(), step2_repr.into()],
        );
        self
    }

    pub fn cap(mut self, step: &'static str) -> Self {
        self.bytecode
            .add_step(String::from("cap"), list![step.into()]);
        self
    }

    pub fn barrier(mut self) -> Self {
        self.bytecode.add_step(String::from("barrier"), list![]);
        self
    }

    pub fn optional(mut self, step: TraversalBuilder) -> Self {
        self.bytecode
            .add_step(String::from("optional"), list![step.bytecode.into()]);
        self
    }

    pub fn constant<A>(mut self, value: A) -> Self
    where
        A: Into<GValue>,
    {
        self.bytecode
            .add_step(String::from("constant"), list![value.into()]);
        self
    }

    pub fn emit(mut self) -> Self {
        self.bytecode.add_step(String::from("emit"), list![]);
        self
    }

    pub fn id(mut self) -> Self {
        self.bytecode.add_step(String::from("id"), list![]);
        self
    }
}

impl From<TraversalBuilder> for GValue {
    fn from(value: TraversalBuilder) -> Self {
        value.bytecode.into()
    }
}
