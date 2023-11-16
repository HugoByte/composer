use std::{io, path::Path};

use super::*;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Composer {
    config_files: Vec<String>,
    pub workflows: RefCell<Vec<Workflow>>,
    pub custom_types: RefCell<HashMap<String, String>>,
}

impl Composer {
    pub fn add_config(&mut self, config: &str) {
        self.config_files.push(config.to_string());
    }

    pub fn run(&self) {
        for config in self.config_files.iter() {
            let content: String = std::fs::read_to_string(config).unwrap();
            let ast = AstModule::parse("config", content, &Dialect::Extended).unwrap();

            // We build our globals by adding some functions we wrote
            let globals = GlobalsBuilder::new()
                .with(starlark_workflow_module)
                .with(starlark_datatype_module)
                .build();

            let module = Module::new();
            let composer = Composer::default();
            {
                let mut eval = Evaluator::new(&module);
                // We add a reference to our store
                eval.extra = Some(&composer);
                eval.eval_module(ast, &globals).unwrap();
            }

            composer.generate();
        }
    }

    pub fn add_workflow(
        &self,
        name: String,
        version: String,
        tasks: HashMap<String, Task>,
        custom_types: Vec<String>,
    ) -> Result<(), Error> {
        for i in self.workflows.borrow().iter() {
            if i.name == name {
                return Err(Error::msg("Workflows should not have same name"));
            }
        }
        if name.is_empty() {
            Err(Error::msg("Workflow name should not be empty"))
        } else {
            self.workflows.borrow_mut().push(Workflow {
                name,
                version,
                tasks,
                custom_types,
            });
            Ok(())
        }
    }

    pub fn add_custom_type(&self, type_name: &str, build_string: String) {
        self.custom_types
            .borrow_mut()
            .insert(type_name.to_string(), build_string);
    }

    fn get_dependencies(&self, task_name: &str, workflow_index: usize) -> Option<Vec<String>> {
        let mut deps = Vec::<String>::new();

        for d in self.workflows.borrow()[workflow_index]
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

    fn dfs(
        &self,
        task_name: &str,
        visited: &mut HashMap<String, bool>,
        flow: &mut Vec<String>,
        workflow_index: usize,
    ) {
        visited.insert(task_name.to_string(), true);

        for d in self
            .get_dependencies(task_name, workflow_index)
            .unwrap()
            .iter()
        {
            if !visited[d] {
                self.dfs(d, visited, flow, workflow_index);
            }
        }

        flow.push(task_name.to_string());
    }

    pub fn get_flow(&self, workflow_index: usize) -> Vec<String> {
        let mut visited = HashMap::<String, bool>::new();
        let mut flow = Vec::<String>::new();

        for t in self.workflows.borrow()[workflow_index].tasks.iter() {
            visited.insert(t.0.to_string(), false);
        }

        for t in self.workflows.borrow()[workflow_index].tasks.iter() {
            if !visited[t.0] {
                self.dfs(t.0, &mut visited, &mut flow, workflow_index)
            }
        }

        flow
    }

    pub fn get_task(&self, task_name: &str, workflow_index: usize) -> Task {
        self.workflows.borrow()[workflow_index]
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

    pub fn get_common_inputs(&self, workflow_index: usize) -> Vec<(String, String)> {
        let mut common = Vec::<(String, String)>::new();

        for (_, task) in self.workflows.borrow()[workflow_index].tasks.iter() {
            let mut depend = Vec::<String>::new();

            for (_, fields) in task.depend_on.iter() {
                for k in fields.keys() {
                    depend.push(k.to_string());
                }
            }

            for input in task.input_args.iter() {
                if depend.binary_search(&input.name).is_err() {
                    common.push((input.name.clone(), input.input_type.clone()));
                };
            }
        }

        common
    }

    fn copy_dir(&self, src: &Path, dest: &Path) -> io::Result<()> {
        // Create the destination directory if it doesn't exist
        if !dest.exists() {
            fs::create_dir(dest)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry.file_name();

            let dest_path = dest.join(file_name);

            if entry_path.is_dir() {
                // Recursively copy subdirectories
                self.copy_dir(&entry_path, &dest_path)?;
            } else {
                // Copy files
                fs::copy(&entry_path, &dest_path)?;
            }
        }

        Ok(())
    }

    pub fn generate(&self) {
        // Getting the current working directory
        let current_path = env::current_dir().unwrap();

        let temp_path = current_path.join("temp");
        let workflow_wasm = current_path.join("workflow_wasm");

        let src_path = current_path.join("boilerplate");
        // creates temp directory to build the workflow
        // fs::create_dir_all(&temp_path).expect("not able to create workflow directory");
        // creates the wasm_workflow directory were the workflow wasm binaries are stored
        // fs::create_dir_all(workflow_wasm.clone()).expect("not able to create workflow directory");

        self.copy_dir(&src_path, &temp_path).unwrap();

        println!("{:?}", current_path.join("temp/src"));

        for (i, _workflow) in self.workflows.borrow().iter().enumerate() {
            fs::write(
                current_path.join("temp/src/types.rs"),
                self.generate_main_file_code(i),
            )
            .unwrap();

            //  code to build&copy the wasm, and store it within workflow_wasm directory

            // Command::new("cd /Users/shanithkk/Hugobyte/work/macos/workspace-aurras/internal-research-and-sample-code/temp/ && CC=/opt/homebrew/opt/llvm/bin/clang AR=/opt/homebrew/opt/llvm/bin/llvm-ar cargo build --release --target wasm32-wasi").status().unwrap();

            // match fs::copy(temp_path.join("boilerplate/<wasm>"), workflow_wasm.clone()) {
            //     Ok(_) => println!("File copied successfully!"),
            //     Err(err) => eprintln!("Error copying file: {}", err),
            // }

            // std::fs::remove_file(temp_path.join("boilerplate/type.rs"))
            //     .expect("not able to delete temp folder");
            // std::fs::remove_file(temp_path.join("boilerplate/<wasm>"))
            //     .expect("not able to delete temp folder");
        }

        // std::fs::remove_dir(temp_path).expect("not able to delete temp folder");
    }
}
