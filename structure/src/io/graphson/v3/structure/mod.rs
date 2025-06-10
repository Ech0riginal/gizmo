pub mod edge;
mod path;
mod property;
mod tinkergraph;
mod vertex;
mod vertexproperty;

type VertexProperties = crate::Map<String, crate::List<crate::VertexProperty>>;
