use std::clone;
use std::{env, fs, path::PathBuf, process::Command};
// use anyhow::Ok;
use starlark::values::{Heap, StarlarkValue, Value, ProvidesStaticType, NoSerialize, ValueLike};
use starlark::{starlark_simple_value, values::starlark_value};
use std::fmt::{ self, Display};
use serde_derive::{Deserialize, Serialize};
use std::result::Result::Ok;
use super::*;
use std::io::ErrorKind;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Composer {
    pub workflows: RefCell<Vec<Workflow>>,
}

impl Composer {
    fn adds(&self, name: String, version: String, tasks: HashMap<String, Task>)  -> Result<(), ErrorKind>{
       
       if name.is_empty(){
            Err(ErrorKind::NotFound)
       }else{

       Ok( self.workflows.borrow_mut().push(Workflow { name, version, tasks }))
       }
    }
   
    }

impl Workflow {
    fn validate(&self) -> Result<(), String>{
         if self.name.is_empty(){
            return Err("name cannot be empty".to_string());
        }
    
        Ok(())
    }
}


starlark_simple_value!(Task);

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {:?} {:?} {} {:?}", self.kind, self.action_name, self.input_args, self.attributes, self.operation, self.depend_on)
    }
}

#[starlark_value(type = "task")]
impl<'v> StarlarkValue<'v> for Task {}

#[starlark_module]
pub fn starlark_workflow(builder: &mut GlobalsBuilder) {
    fn task(
        kind: String,
        action_name: String,
        input_args: Value,
        attributes: Value,
        depend_on: Value,
        operation: Option<String>,
        eval: &mut Evaluator,
    ) -> anyhow::Result<Task> {
        let ip_args : HashMap<String, String> = serde_json::from_str(&input_args.to_json()?).unwrap();
        let property : HashMap<String, String> = serde_json::from_str(&attributes.to_json()?).unwrap();
        let depnd : HashMap<String, HashMap<String, String>> = serde_json::from_str(&depend_on.to_json()?).unwrap();

        let operand = match operation {
            Some(a) => a,
            None => String::default(),
        };

        Ok(Task { kind: kind, action_name: action_name, input_args: ip_args, attributes: property, operation: operand, depend_on: depnd })
        
    }

    fn workflows(name: String, version: String, tasks: Value, eval: &mut Evaluator) -> anyhow::Result<NoneType> {
        let tasks : Vec<Task> = serde_json::from_str(&tasks.to_json()?).unwrap();
        let delimiter = " , ";

        let task_hashmap = tasks.iter().map(|te|  (te.action_name.clone(), te.clone() )).collect();
    
            eval.extra
                .unwrap()
                .downcast_ref::<Composer>()
                .unwrap()
                .adds(name.clone(), version.clone(), task_hashmap).unwrap();

        Ok(NoneType)
    }
}

impl Composer {
    fn get_dependencies(&self, task_name: &str) -> Option<Vec<String>> {
        let mut deps = Vec::<String>::new();

        for d in self.workflows.borrow()[0].tasks.get(task_name).unwrap().depend_on.iter() {
            deps.push(d.0.clone());
        }

        Some(deps)
    }

    fn dfs(&self, task_name: &str, visited: &mut HashMap<String, bool>, flow: &mut Vec<String>) {
        visited.insert(String::from(task_name), true);

        for d in self.get_dependencies(task_name).unwrap().iter() {
            if !visited[d] {
                self.dfs(d, visited, flow);
            }
        }

        flow.push(String::from(task_name));
    }

    pub fn get_flow(&self) -> Vec<String> {
        let mut visited = HashMap::<String, bool>::new();
        let mut flow = Vec::<String>::new();

        for t in self.workflows.borrow()[0].tasks.iter() {
            visited.insert(String::from(t.0), false);
        }

        for t in self.workflows.borrow()[0].tasks.iter() {
            if !visited[t.0] {
                self.dfs(t.0, &mut visited, &mut flow)
            }
        }

        flow
    }

    pub fn get_task(&self, task_name: &str) -> Task {
        self.workflows.borrow()[0].tasks.get(task_name).unwrap().clone()
    }

    pub fn get_task_input_data(&self, task_name: &str, task: &HashMap<String, String>) -> String {
        let mut input = format!("{task_name}Input, [");

        for (i, field) in task.iter().enumerate() {
            input = format!("{input}{}:{}", field.0, field.1);

            if i != task.len() - 1 {
                input = format!("{input},");
            } else {
                input = format!("{input}]");
            }
        }

        input
    }

    pub fn get_common_inputs(&self) -> Vec<(String, String)> {
        let mut common = Vec::<(String, String)>::new();

        // HashMap<String, Task
        for (_, task) in self.workflows.borrow()[0].tasks.iter() {
            let mut depend = Vec::<String>::new();

            for (_, fields) in task.depend_on.iter() {
                for k in fields.keys() {
                    depend.push(String::from(k));
                }
            }

            for (field, ty) in task.input_args.iter() {
                if let Err(_) = depend.binary_search(field) {
                    common.push((String::from(field), String::from(ty)));
                };
            }
        }

        common
    }

    // Function to generate the code for the main.rs file
    // returns main file and dependencies file
    fn get_code(&self) -> Vec<String> {
        let dependencies = "\
[package]
name = \"generated-project\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
serde_json = \"1.0.81\"
";

        vec![self.generate_main_file_code(), dependencies.to_string()]
    }

    // Function to generate a new Cargo package and write the main.rs and Cargo.toml files
    fn generate_cargo(
        &self,
        project_name: &str,
        path: &PathBuf,
        main_file_content: &str,
        cargo_toml_content: &str,
    ) {
        // Generating a new Cargo package
        Command::new("cargo")
            .args(&["new", &project_name])
            .status()
            .unwrap();

        // Creating and writing into the files
        fs::write(&(path.join("src/main.rs")), main_file_content).unwrap();
        fs::write(&(path.join("Cargo.toml")), cargo_toml_content).unwrap();
    }

    pub fn generate(&self) {
        let project_name = String::from("generated-project");
        // Getting the current working directory
        let pwd = env::current_dir().unwrap();
        let proj_path = pwd.join(&project_name);

        let content = self.get_code();

        // Generating the Cargo package and writing the main.rs and Cargo.toml files
        self.generate_cargo(&project_name, &proj_path, &content[0], &content[1]);
    }
}
