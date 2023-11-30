use super::*;

#[derive(
    Debug, Default, PartialEq, Eq, ProvidesStaticType, Allocative, Clone, Deserialize, Serialize,
)]
pub struct Task {
    pub kind: String,
    pub action_name: String,
    pub input_args: Vec<Input>,
    pub attributes: HashMap<String, String>,
    #[serde(default)]
    pub operation: String,
    pub depend_on: HashMap<String, HashMap<String, String>>,
}

#[derive(
    Debug, Default, PartialEq, Eq, Allocative, ProvidesStaticType, Clone, Deserialize, Serialize,
)]
pub struct Input {
    pub name: String,
    pub input_type: String,
    #[serde(default)]
    pub default_value: String,
}

impl Task {
    pub fn new(
        kind: &str,
        action_name: &str,
        input_args: Vec<Input>,
        attributes: HashMap<String, String>,
        depend_on: HashMap<String, HashMap<String, String>>,
        operation: String,
    ) -> Self {
        Task {
            kind: kind.to_string(),
            action_name: action_name.to_string(),
            input_args,
            attributes,
            depend_on,
            operation,
        }
    }
}

starlark_simple_value!(Task);

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {:?} {:?} {} {:?}",
            self.kind,
            self.action_name,
            self.input_args,
            self.attributes,
            self.operation,
            self.depend_on
        )
    }
}

#[starlark_value(type = "task")]
impl<'v> StarlarkValue<'v> for Task {}

starlark_simple_value!(Input);

impl Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.name, self.input_type, self.default_value
        )
    }
}

#[starlark_value(type = "input")]
impl<'v> StarlarkValue<'v> for Input {}