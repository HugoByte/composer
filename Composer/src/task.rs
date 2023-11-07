use super::*;
use allocative::Allocative;
use serde_derive::Serialize;

// #[derive(Debug, Clone, Deserialize)]
#[derive(Debug, PartialEq, Eq, ProvidesStaticType, Allocative, Clone, Deserialize, Serialize)]
pub struct Task {
    pub kind: String,
    pub action_name: String,
    pub input_args: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
    #[serde(default)]
    pub operation: String,
    pub depend_on: HashMap<String, HashMap<String, String>>,
}

impl Task {
    pub fn new(
        kind: &str,
        action_name: &str,
        input_args: HashMap<String, String>,
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


