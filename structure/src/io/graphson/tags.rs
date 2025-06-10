#![allow(unused)]

use crate::io::error::Missing;
use crate::io::macros::get_value;
use crate::io::utils::Ensure;
use crate::io::{Deserialize, Deserializer, Error, Serialize, Serializer};
use serde_json::Value;
use std::fmt::{Debug, Display, Formatter};

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
            .ok_or(TYPE_TAG.missing())
            .map(|v| v.as_str().unwrap())?;
        let tag = Tag::try_from(tagd)?;
        let value = self.ensure(VALUE_TAG)?;

        Ok(Type { tag, value })
    }
}

macro_rules! enom {
    ($($variant:ident($repr:expr)),+) => {
        pub enum Tag {
            $($variant,)+
        }

        impl serde::Serialize for Tag {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                match self {
                    $(Self::$variant => serializer.serialize_str($repr)),+
                }
            }
        }

        impl Into<crate::GValue> for Tag {
            fn into(self) -> crate::GValue {
                match self {
                    $(Self::$variant => crate::GValue::String($repr.into())),+
                }
            }
        }

        impl Into<String> for Tag {
            fn into(self) -> String {
                match self {
                    $(Self::$variant => $repr.into()),+
                }
            }
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
    // Core
    Class("g:Class"),
    Date("g:Date"),
    Double("g:Double"),
    Float("g:Float"),
    Integer("g:Int32"),
    List("g:List"),
    Long("g:Int64"),
    Map("g:Map"),
    Set("g:Set"),
    Timestamp("g:Timestamp"),
    Uuid("g:UUID"),
    // Structure
    Edge("g:Edge"),
    Path("g:Path"),
    Property("g:Property"),
    StarGraph("g:StarGraph"),
    TinkerGraph("tinker:graph"),
    Tree("g:Tree"),
    Vertex("g:Vertex"),
    VertexProperty("g:VertexProperty"),
    // Process
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
    Traverser("g:Traverser"),
    // Extended

    // Custom
    // The geometric type value emitted by SQLg
    Geometry("g:Geometry"),
    // The geometric type value emitted by Janusgraph
    Geoshape("g:Geoshape")
);
