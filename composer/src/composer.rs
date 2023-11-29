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
        let current_path = env::current_dir().unwrap();

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

            composer.generate(current_path.as_path());
        }
    }

    pub fn add_workflow(
        &self,
        name: String,
        version: String,
        tasks: HashMap<String, Task>,
        custom_types: Option<Vec<String>>,
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

    pub fn get_dependencies(&self, task_name: &str, workflow_index: usize) -> Option<Vec<String>> {
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

            // for fields in task.depend_on.iter() {
            //     for k in fields.keys() {
            //         depend.push(k.to_string());
            //     }
            // }

            for input in task.input_args.iter() {
                if depend.binary_search(&input.name).is_err() {
                    common.push((input.name.clone(), input.input_type.clone()));
                };
            }
        }

        common
    }

    fn copy_dir(&self, src: &Path, dest: &Path, file: Option<&str>) -> io::Result<()> {
        // Create the destination directory if it doesn't exist
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dest.join(&file_name);
            if file.is_some() && entry_path.is_dir() {
                continue;
            }

            if entry_path.is_dir() {
                // Recursively copy subdirectories
                self.copy_dir(&entry_path, &dest_path, file)?;
            } else {
                // Copy files
                if file.is_none() || (file_name.to_str() == file) {
                    fs::copy(&entry_path, &dest_path)?;
                }
            }
        }

        Ok(())
    }

    fn fetch_wasm(&self, pwd: &Path, workflow_name: &str) {
        Command::new("rustup")
            .current_dir(pwd.join(format!("temp-{}", workflow_name)))
            .args(["target", "add", "wasm32-wasi"])
            .status()
            .expect("adding wasm32-wasi rust toolchain command failed to start");

        Command::new("cargo")
            .current_dir(pwd.join(format!("temp-{}", workflow_name)))
            .args(["build", "--release", "--target", "wasm32-wasi"])
            .status()
            .expect("building wasm32 command failed to start");

        let src = pwd.join(format!("temp-{}/target/wasm32-wasi/release", workflow_name));
        fs::rename(
            src.join("workflow.wasm"),
            src.join(format!("{}.wasm", workflow_name)),
        )
        .unwrap();

        let dest = pwd.join("workflow_wasm");

        self.copy_dir(&src, &dest, Some(&format!("{}.wasm", workflow_name)))
            .unwrap();
    }

    pub fn generate(&self, current_path: &Path) {
        // Getting the current working directory
        let src_path = current_path.join("boilerplate");

        for (i, workflow) in self.workflows.borrow().iter().enumerate() {
            if workflow.tasks.is_empty() {
                continue;
            }

            let dest_path = current_path.join(format!(
                "temp-{}-{}",
                workflow.name.to_case(Case::Snake),
                workflow.version
            ));

            self.copy_dir(&src_path, &dest_path, None).unwrap();

            fs::write(
                dest_path.join("src/types.rs"),
                self.generate_main_file_code(i),
            )
            .unwrap();

            self.fetch_wasm(
                current_path,
                &format!(
                    "{}-{}",
                    workflow.name.to_case(Case::Snake),
                    workflow.version
                ),
            );
            fs::remove_dir_all(current_path.join(&format!(
                "temp-{}-{}",
                workflow.name.to_case(Case::Snake),
                workflow.version
            )))
            .unwrap();
        }
    }
}