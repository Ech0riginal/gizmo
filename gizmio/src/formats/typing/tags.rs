use std::fmt::Formatter;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum TypeTag {
    Class,
    Date,
    Double,
    Float,
    Integer,
    List,
    Long,
    Map,
    Set,
    Timestamp,
    Uuid,
    Edge,
    Path,
    Property,
    #[allow(dead_code)]
    StarGraph,
    TinkerGraph,
    Tree,
    Vertex,
    VertexProperty,
    Barrier,
    Binding,
    BulkSet,
    Bytecode,
    Cardinality,
    Column,
    Direction,
    Lambda,
    Merge,
    Metrics,
    Operator,
    Order,
    P,
    Pick,
    Pop,
    Scope,
    T,
    TextP,
    TraversalMetrics,
    Traverser,
    Geometry,
}

#[derive(Debug)]
pub(crate) struct Unexpected<T>(pub T);

impl<T: std::fmt::Debug> From<Unexpected<T>> for crate::Error {
    fn from(Unexpected(thing): Unexpected<T>) -> Self {
        Self::unexpected(&thing, "supported type tag")
    }
}

impl TypeTag {
    #[allow(private_bounds)]
    pub fn try_from<T>(value: T) -> Result<Self, Unexpected<T>>
    where
        T: TypeHandler<T>,
    {
        T::lookup(value)
    }
}

trait TypeHandler<T> {
    fn lookup(self) -> Result<TypeTag, Unexpected<T>>;
}

impl<'a, T: 'a> TypeHandler<T> for T
where
    T: AsRef<str>,
{
    fn lookup(self) -> Result<TypeTag, Unexpected<T>> {
        match self.as_ref() {
            "g:Class" => Ok(TypeTag::Class),
            "g:Date" => Ok(TypeTag::Date),
            "g:Double" => Ok(TypeTag::Double),
            "g:Float" => Ok(TypeTag::Float),
            "g:Int32" => Ok(TypeTag::Integer),
            "g:List" => Ok(TypeTag::List),
            "g:Int64" => Ok(TypeTag::Long),
            "g:Map" => Ok(TypeTag::Map),
            "g:Set" => Ok(TypeTag::Set),
            "g:Timestamp" => Ok(TypeTag::Timestamp),
            "g:UUID" => Ok(TypeTag::Uuid),
            "g:Edge" => Ok(TypeTag::Edge),
            "g:Path" => Ok(TypeTag::Path),
            "g:Property" => Ok(TypeTag::Property),
            "tinker:graph" => Ok(TypeTag::TinkerGraph),
            "g:Vertex" => Ok(TypeTag::Vertex),
            "g:VertexProperty" => Ok(TypeTag::VertexProperty),
            "g:Barrier" => Ok(TypeTag::Barrier),
            "g:Binding" => Ok(TypeTag::Binding),
            "g:BulkSet" => Ok(TypeTag::BulkSet),
            "g:Bytecode" => Ok(TypeTag::Bytecode),
            "g:Cardinality" => Ok(TypeTag::Cardinality),
            "g:Column" => Ok(TypeTag::Column),
            "g:Direction" => Ok(TypeTag::Direction),
            "g:Lambda" => Ok(TypeTag::Lambda),
            "g:Merge" => Ok(TypeTag::Merge),
            "g:Metrics" => Ok(TypeTag::Metrics),
            "g:Operator" => Ok(TypeTag::Operator),
            "g:Order" => Ok(TypeTag::Order),
            "g:P" => Ok(TypeTag::P),
            "g:Pick" => Ok(TypeTag::Pick),
            "g:Pop" => Ok(TypeTag::Pop),
            "g:Scope" => Ok(TypeTag::Scope),
            "g:T" => Ok(TypeTag::T),
            "g:TextP" => Ok(TypeTag::TextP),
            "g:Tree" => Ok(TypeTag::Tree),
            "g:TraversalMetrics" => Ok(TypeTag::TraversalMetrics),
            "g:Traverser" => Ok(TypeTag::Traverser),
            "g:Geometry" => Ok(TypeTag::Geometry),
            _invalid_or_unsupported => Err(Unexpected(self)),
        }
    }
}

impl std::fmt::Display for TypeTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class => write!(f, "Class"),
            Self::Date => write!(f, "Date"),
            Self::Double => write!(f, "Double"),
            Self::Float => write!(f, "Float"),
            Self::Integer => write!(f, "Integer"),
            Self::List => write!(f, "List"),
            Self::Long => write!(f, "Long"),
            Self::Map => write!(f, "Map"),
            Self::Set => write!(f, "Set"),
            Self::Timestamp => write!(f, "Timestamp"),
            Self::Uuid => write!(f, "Uuid"),
            Self::Edge => write!(f, "Edge"),
            Self::Path => write!(f, "Path"),
            Self::Property => write!(f, "Property"),
            Self::StarGraph => write!(f, "StarGraph"),
            Self::TinkerGraph => write!(f, "TinkerGraph"),
            Self::Tree => write!(f, "Tree"),
            Self::Vertex => write!(f, "Vertex"),
            Self::VertexProperty => write!(f, "VertexProperty"),
            Self::Barrier => write!(f, "Barrier"),
            Self::Binding => write!(f, "Binding"),
            Self::BulkSet => write!(f, "BulkSet"),
            Self::Bytecode => write!(f, "Bytecode"),
            Self::Cardinality => write!(f, "Cardinality"),
            Self::Column => write!(f, "Column"),
            Self::Direction => write!(f, "Direction"),
            Self::Lambda => write!(f, "Lambda"),
            Self::Merge => write!(f, "Merge"),
            Self::Metrics => write!(f, "Metrics"),
            Self::Operator => write!(f, "Operator"),
            Self::Order => write!(f, "Order"),
            Self::P => write!(f, "P"),
            Self::Pick => write!(f, "Pick"),
            Self::Pop => write!(f, "Pop"),
            Self::Scope => write!(f, "Scope"),
            Self::T => write!(f, "T"),
            Self::TextP => write!(f, "TextP"),
            Self::TraversalMetrics => write!(f, "TraversalMetrics"),
            Self::Traverser => write!(f, "Traverser"),
            Self::Geometry => write!(f, "Geometry"),
        }
    }
}
