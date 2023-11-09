use allocative::Allocative;
use serde_derive::Serialize;

use super::*;

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, Allocative, Clone, Deserialize, Serialize)]
pub struct Workflow {
    pub name : String,
    pub version : String,
    pub tasks : HashMap<String,Task>,
    pub custom_types: Vec<String>,
}

