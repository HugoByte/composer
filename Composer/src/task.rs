use super::*;
use allocative::Allocative;
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, Allocative, Clone, Deserialize, Serialize)]
pub struct Task {
    pub kind: String,
    pub action_name: String,
    pub input_args: Vec<Input>,
    pub attributes: HashMap<String, String>,
    #[serde(default)]
    pub operation: String,
    pub depend_on: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Eq, Allocative, ProvidesStaticType,Clone, Deserialize, Serialize)]
pub struct Input {
    pub name : String,
    pub input_type : String,
    #[serde(default)]
    pub default_value : String,
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


