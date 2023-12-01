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
    pub depend_on: Vec<Depend>,
}


#[derive(Debug, PartialEq, Eq, Allocative, ProvidesStaticType,Clone, Deserialize, Serialize)]
pub struct Depend {
    pub task_name: String,
    pub cur_field : String,
    pub prev_field : String,
}

impl Task {
    pub fn new(
        kind: &str,
        action_name: &str,
        input_args: Vec<Input>,
        attributes: HashMap<String, String>,
        depend_on: Vec<Depend>,
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

starlark_simple_value!(Depend);

impl Display for Depend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.task_name, self.cur_field, self.prev_field
        )
    }
}

impl Default for Depend {
    fn default() -> Self {
        Depend {
            task_name: String::default(),
            cur_field: String::default(),
            prev_field: String::default(),
        }
    }
}

#[starlark_value(type = "Depend")]
impl<'v> StarlarkValue<'v> for Depend {}

#[starlark_value(type = "Task")]
impl<'v> StarlarkValue<'v> for Task {}

