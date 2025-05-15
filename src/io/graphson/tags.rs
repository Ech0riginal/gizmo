#![allow(unused)]

crate::io::macros::types! {
    core,
    CLASS, "g:Class",
    DATE, "d:Date",
    DOUBLE, "g:Double",
    FLOAT, "g:Float",
    INT, "g:Int32",
    LIST, "g:List",
    LONG, "g:Int64",
    MAP, "g:Map",
    SET, "g:Set",
    TIMESTAMP, "g:Timestamp",
    UUID, "g:UUID"
}

crate::io::macros::types! {
    structure,
    EDGE, "g:Edge",
    PATH, "g:Path",
    PROPERTY, "g:Property",
    STAR_GRAPH, "g:StarGraph",
    TINKER_GRAPH, "tinker:graph",
    TREE, "g:Tree",
    VERTEX, "g:Vertex",
    VERTEX_PROPERTY, "g:VertexProperty"

}

crate::io::macros::types! {
    process,
    BARRIER, "g:Barrier",
    BINDING, "g:Binding",
    BULKSET, "g:BulkSet",
    BYTECODE, "g:Bytecode",
    CARDINALITY, "g:Cardinality",
    COLUMN, "g:Column",
    DIRECTION, "g:Direction",
    DT, "g:DT",
    LAMBDA, "g:Lambda",
    MERGE, "g:Merge",
    METRICS, "g:Metrics",
    OPERATOR, "g:Operator",
    ORDER, "g:Order",
    P, "g:P",
    PICK, "g:Pick",
    POP, "g:Pop",
    SCOPE, "g:Scope",
    T, "g:T",
    TEXT_P, "g:TextP",
    TRAVERSAL_EXPLANATION, "g:TraversalExplanation",
    TRAVERSAL_METRICS, "g:TraversalMetrics",
    TRAVERSER, "g:Traverser"
}

mod typed {
    use crate::io::macros::get_value;
    use crate::io::{Deserialize, Deserializer, Error, Serialize, Serializer};
    use serde_json::Value;

    const TYPE_TAG: &'static str = "@type";
    const VALUE_TAG: &'static str = "@value";

    pub struct Type<'a> {
        pub tag: Tag,
        pub value: &'a Value,
    }

    pub trait Typed {
        fn typed<'a>(&'a self) -> Result<Type<'a>, Error>;
    }

    impl Typed for Value {
        /// Validates a type against the expected { `@type`: ..., `@value`: ... } format
        fn typed<'a>(&'a self) -> Result<Type<'a>, Error> {
            let tagd = self
                .get(TYPE_TAG)
                .ok_or(Error::Missing(TYPE_TAG))
                .map(|v| v.as_str().unwrap())?;
            let tag = Tag::try_from(tagd)?;
            let value = self.get(VALUE_TAG).ok_or(Error::Missing(VALUE_TAG))?;

            Ok(Type { tag, value })
        }
    }

    macro_rules! enom {
        ($($variant:ident($repr:expr)),+) => {
            pub enum Tag {
                $($variant,)+
            }

            impl std::fmt::Debug for Tag {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", &self)
                }
            }

            impl std::fmt::Display for Tag {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(Self::$variant => write!(f, $repr)),+
                    }
                }
            }

            impl<'a> TryFrom<&'a str> for Tag {
                type Error = crate::io::Error;

                fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                    match value {
                        $($repr => Ok(Self::$variant),)+
                        _ => Err(Self::Error::Unexpected(value.to_string())),
                    }
                }
            }
        };
    }

    enom!(
        Class("g:Class"),
        Date("d:Date"),
        Double("g:Double"),
        Float("g:Float"),
        Integer("g:Int32"),
        List("g:List"),
        Long("g:Int64"),
        Map("g:Map"),
        Set("g:Set"),
        Timestamp("g:Timestamp"),
        Uuid("g:UUID"),
        Edge("g:Edge"),
        Path("g:Path"),
        Property("g:Property"),
        StarGraph("g:StarGraph"),
        TinkerGraph("tinker:graph"),
        Tree("g:Tree"),
        Vertex("g:Vertex"),
        VertexProperty("g:VertexProperty"),
        Barrier("g:Barrier"),
        Binding("g:Binding"),
        BulkSet("g:BulkSet"),
        Bytecode("g:Bytecode"),
        Cardinality("g:Cardinality"),
        Column("g:Column"),
        Direction("g:Direction"),
        DT("g:DT"),
        Lambda("g:Lambda"),
        Merge("g:Merge"),
        Metrics("g:Metrics"),
        Operator("g:Operator"),
        Order("g:Order"),
        P("g:P"),
        Pick("g:Pick"),
        Pop("g:Pop"),
        Scope("g:Scope"),
        T("g:T"),
        TextP("g:TextP"),
        TraversalExplanation("g:TraversalExplanation"),
        TraversalMetrics("g:TraversalMetrics"),
        Traverser("g:Traverser")
    );
}

pub use self::core::*;
pub use self::process::*;
pub use self::structure::*;
pub use self::typed::*;
use std::fmt::{Debug, Display, Formatter};
