use std::clone;
use std::{env, fs, path::PathBuf, process::Command};
// use anyhow::Ok;
use super::*;
use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use starlark::values::{Heap, NoSerialize, ProvidesStaticType, StarlarkValue, Value, ValueLike};
use starlark::{starlark_simple_value, values::starlark_value};
use std::fmt::{self, Display};
use std::io::ErrorKind;
use std::result::Result::Ok;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Composer {
    pub workflows: RefCell<Vec<Workflow>>,
}

impl Composer {
    fn add_workflow(
        &self,
        name: String,
        version: String,
        tasks: HashMap<String, Task>,
    ) -> Result<(), Error> {
       
        for i in self.workflows.borrow().iter() {
            if i.name == name {
                return Err(Error::msg("Workflows should not have same name"));
            }
        }
        if name.is_empty() {
            Err(Error::msg("Workflow name should not be empty"))
        } else {
            Ok(self.workflows.borrow_mut().push(Workflow {
                name,
                version,
                tasks,
            }))
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
        let ip_args: Vec<Input> = serde_json::from_str(&input_args.to_json()?).unwrap();
        let property: HashMap<String, String> =
            serde_json::from_str(&attributes.to_json()?).unwrap();
        let depnd: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&depend_on.to_json()?).unwrap();

        let operand = match operation {
            Some(a) => a,
            None => String::default(),
        };

        Ok(Task {
            kind: kind,
            action_name: action_name,
            input_args: ip_args,
            attributes: property,
            operation: operand,
            depend_on: depnd,
        })
    }

    fn workflows(
        name: String,
        version: String,
        tasks: Value,
        eval: &mut Evaluator,
    ) -> anyhow::Result<NoneType> {
        let tasks: Vec<Task> = serde_json::from_str(&tasks.to_json()?).unwrap();

        let task_hashmap = tasks
            .iter()
            .map(|te| (te.action_name.clone(), te.clone()))
            .collect();

        eval.extra
            .unwrap()
            .downcast_ref::<Composer>()
            .unwrap()
            .add_workflow(name.clone(), version.clone(), task_hashmap)
            .unwrap();

        Ok(NoneType)
    }

    fn ip_args(
        name: String,
        input_type: String,
        default_value: Option<String>,
        eval: &mut Evaluator,
    ) -> anyhow::Result<Input> {
        let default = match default_value {
            Some(b) => b,
            None => String::default(),
        };
        Ok(Input {
            name: name,
            input_type: input_type,
            default_value: default,
        })
    }
}

impl Composer {
    fn get_dependencies(&self, task_name: &str) -> Option<Vec<String>> {
        let mut deps = Vec::<String>::new();

        for d in self.workflows.borrow()[0]
            .tasks
            .get(task_name)
            .unwrap()
            .depend_on
            .iter()
        {
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
        self.workflows.borrow()[0]
            .tasks
            .get(task_name)
            .unwrap()
            .clone()
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

        for (_, task) in self.workflows.borrow()[0].tasks.iter() {
            let mut depend = Vec::<String>::new();

            for (_, fields) in task.depend_on.iter() {
                for k in fields.keys() {
                    depend.push(String::from(k));
                }
            }

            for input in task.input_args.iter() {
                if let Err(_) = depend.binary_search(&input.name) {
                    common.push((input.name.clone(), input.input_type.clone()));
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
        // Getting the current working directory
        let path = env::current_dir().unwrap().join("./generated_project");

        fs::create_dir_all(&path).unwrap();
        fs::write(&(path.join("./types.rs")), self.generate_main_file_code()).unwrap();
    }
}
