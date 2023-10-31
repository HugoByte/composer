use super::*;


#[derive(Debug, Clone, Deserialize)]
pub struct Workflow {
    pub name : String,
    pub version : String,
}