use super::*;

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, Allocative, Clone, Deserialize, Serialize)]
pub struct Workflow {
    pub name: String,
    pub version: String,
    pub tasks: HashMap<String, Task>,
}

starlark_simple_value!(Workflow);

impl Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {:?}", self.name, self.version, self.tasks)
    }
}

#[starlark_value(type = "Workflow")]
impl<'v> StarlarkValue<'v> for Workflow {}
