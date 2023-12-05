use super::*;

#[derive(
    Debug, Default, PartialEq, Eq, Allocative, ProvidesStaticType, Clone, Deserialize, Serialize,
)]
pub struct Input {
    pub name: String,
    pub input_type: RustType,
    #[serde(default)]
    pub default_value: Option<String>,
}

starlark_simple_value!(Input);

impl Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {:?}",
            self.name, self.input_type, self.default_value
        )
    }
}

#[starlark_value(type = "Input")]
impl<'v> StarlarkValue<'v> for Input {}
