#![allow(unused)]

use crate::graphson::Ensure;
use crate::graphson::error::*;
use crate::macros::get_value;
use serde_json::Value;
use snafu::prelude::*;
use snafu::{Location, location};
use std::fmt::{Debug, Display, Formatter};

const TYPE_TAG: &str = "@type";
const VALUE_TAG: &str = "@value";

pub trait Typed {
    fn typed<'a>(&'a self) -> Result<Type<'a>, Error>;
}

#[derive(Debug)]
pub struct Type<'a> {
    pub tag: Tag,
    pub value: &'a Value,
}

impl Typed for Value {
    /// Validates a type against the expected { `@type`: ..., `@value`: ... } format
    fn typed<'a>(&'a self) -> Result<Type<'a>, Error> {
        let tag = match self.ensure(TYPE_TAG).context(InvalidSnafu) {
            Ok(v) => {
                let tagd = v.as_str().unwrap().to_string();
                Tag::try_from(tagd.as_str()).context(InvalidSnafu)?
            }
            Err(e) => return Err(e),
        };

        match self.ensure(VALUE_TAG).context(InvalidSnafu) {
            Ok(value) => Ok(Type { tag, value }),
            Err(e) => Err(e),
        }
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
            type Error = Error;

            #[track_caller]
            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                match value {
                    $($repr => Ok(Self::$variant),)+
                    unsupported_tag => Err(Error::Unsupported {
                        tag: unsupported_tag.to_string(),
                        location: location!(),
                    }),
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
