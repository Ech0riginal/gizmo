pub mod edge;
mod path;
mod property;
mod stargraph;
mod tinkergraph;
mod tree;
mod vertex;
mod vertexproperty;

type VertexProperties = std::collections::HashMap<String, Vec<crate::VertexProperty>>;
