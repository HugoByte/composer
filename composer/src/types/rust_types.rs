use super::*;

#[derive(Debug, PartialEq, Eq, Allocative, ProvidesStaticType, Clone, Deserialize, Serialize)]
pub enum RustType {
    Null,
    Int,
    Float,
    Boolean,
    String,
    List(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    Struct(String),
}

impl Default for RustType {
    fn default() -> RustType {
        Self::Null
    }
}

starlark_simple_value!(RustType);

#[starlark_value(type = "RustType")]
impl<'v> StarlarkValue<'v> for RustType {}

impl Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustType::Null => write!(f, "Null"),
            RustType::Int => write!(f, "i32"),
            RustType::Float => write!(f, "f32"),
            RustType::Boolean => write!(f, "bool"),
            RustType::String => write!(f, "String"),
            RustType::List(type_) => write!(f, "Vec<{type_}>"),
            RustType::HashMap(key_type, val_type) => write!(f, "HashMap<{key_type},{val_type}>"),
            RustType::Struct(name) => write!(f, "{name}"),
        }
    }
}
