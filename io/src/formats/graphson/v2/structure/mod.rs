pub mod edge;
mod path;
mod property;
mod stargraph;
mod tinkergraph;
mod tree;
mod vertex;
mod vertexproperty;

type VertexProperties = crate::Map<String, crate::List<crate::VertexProperty>>;
