use super::*;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Composer {
    config_files: Vec<String>,
    pub workflows: RefCell<Vec<Workflow>>,
    pub custom_types: RefCell<HashMap<String, String>>,
}

impl Composer {
    /// Adds config file to the composer
    /// This method is called by the user
    ///
    /// # Arguments
    ///
    /// * `config` - A string slice that holds the of the config file along with its name
    ///
    /// # Example
    ///
    /// ```
    /// use composer::Composer;
    /// let mut composer = Composer::default();
    /// composer.add_config("config/path/config_file_name_here");
    /// ```
    pub fn add_config(&mut self, config: &str) {
        self.config_files.push(config.to_string());
    }

    /// Runs the composer to build all workflows specified in the config files.
    /// This method is called by the user.
    ///
    /// # Example
    /// ```
    /// use your_module_name_here::Composer;
    /// let composer = Composer::default();
    /// composer.add_config("config_file_1");
    /// composer.add_config("config_file_2");
    /// composer.run();
    /// ```
    pub fn run(&self) {
        let current_path = env::current_dir().unwrap();

        for config in self.config_files.iter() {
            let content: String = std::fs::read_to_string(config).unwrap();
            let ast = AstModule::parse("config", content, &Dialect::Extended).unwrap();

            // We build our globals by adding some functions we wrote
            let globals = GlobalsBuilder::extended_by(&[
                StructType,
                RecordType,
                EnumType,
                Map,
                Filter,
                Partial,
                ExperimentalRegex,
                Debug,
                Print,
                Pprint,
                Breakpoint,
                Json,
                Typing,
                Internal,
                CallStack,
            ])
            .with(starlark_workflow_module)
            .with(starlark_datatype_module)
            .with_struct("Operation", starlark_operation_module)
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

    /// Adds a new workflow to the composer.
    /// This method is invoked by the workflows function inside the starlark_module.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the workflow to be added
    /// * `version` - Version of the workflow
    /// * `tasks` - HashMap of tasks associated with the workflow
    /// * `custom_types` - Optional vector of custom types names that are created within config
    ///   for the workflow.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Result indicating success if the workflow is added successfully,
    ///   or an error if the workflow name is empty or if there is a duplicate workflow name.
    ///
    pub fn add_workflow(
        &self,
        name: String,
        version: String,
        tasks: HashMap<String, Task>,
    ) -> Result<(), Error> {
        for workflow in self.workflows.borrow().iter() {
            if workflow.name == name {
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
            });
            Ok(())
        }
    }

    /// Adds a custom type that is created by the user inside the config.
    /// This method is called by the starlark_module.
    ///
    /// # Arguments
    ///
    /// * `type_name` - A string slice that holds the name of the struct for the custom type
    /// * `build_string` - A string that holds the Rust code, which uses macros to create a struct
    ///
    pub fn add_custom_type(&self, type_name: &str, build_string: String) {
        self.custom_types
            .borrow_mut()
            .insert(type_name.to_string(), build_string);
    }

    /// Finds the list of dependencies that the given task depends on.
    ///
    /// # Arguments
    ///
    /// * `task_name` - A string slice that holds the name of the task
    /// * `workflow_index` - A integer that holds the index of the workflow where the given
    ///   task is stored
    ///
    /// # Returns
    ///
    /// * `Option<Vec<String>>` - An option containing a vector of dependencies if the task is
    ///   found, or None if the task have no dependency
    ///
    pub fn get_dependencies(&self, task_name: &str, workflow_index: usize) -> Option<Vec<String>> {
        let mut dependencies = Vec::<String>::new();

        for task in self.workflows.borrow()[workflow_index]
            .tasks
            .get(task_name)
            .unwrap()
            .depend_on
            .iter()
        {
            dependencies.push(task.task_name.clone());
        }

        Some(dependencies)
    }

    /// Performs depth-first search (DFS) in the workflow subgraph.
    /// This method is invoked within the get_flow method to perform `Topological-Sorting`
    /// # Arguments
    ///
    /// * `task_name` - A string slice that holds the name of the task where the DFS should start
    /// * `visited` - A mutable reference to a HashMap that holds the list of task (node) names
    ///   and a boolean indicating whether it has been traversed
    /// * `flow` - A mutable reference to a vector of strings that stores the flow of the DFS
    ///   traversal
    /// * `workflow_index` - An integer that holds the index of the workflow where the given
    ///   task is located
    ///
    fn dfs(
        &self,
        task_name: &str,
        visited: &mut HashMap<String, bool>,
        flow: &mut Vec<String>,
        workflow_index: usize,
    ) {
        visited.insert(task_name.to_string(), true);

        for depend_task in self
            .get_dependencies(task_name, workflow_index)
            .unwrap()
            .iter()
        {
            if !visited[depend_task] {
                self.dfs(depend_task, visited, flow, workflow_index);
            }
        }

        flow.push(task_name.to_string());
    }

    /// Performs topological sort in the workflow graph.
    /// This method is invoked by the parse_module.
    ///
    /// # Arguments
    ///
    /// * `workflow_index` - An integer that holds the index of the workflow for which
    ///   topological sort is to be performed
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - A vector containing the list of task names in the order of the
    ///   topological sort
    ///
    pub fn get_flow(&self, workflow_index: usize) -> Vec<String> {
        let mut visited = HashMap::<String, bool>::new();
        let mut flow = Vec::<String>::new();

        for task in self.workflows.borrow()[workflow_index].tasks.iter() {
            visited.insert(task.0.to_string(), false);
        }

        for task in self.workflows.borrow()[workflow_index].tasks.iter() {
            if !visited[task.0] {
                self.dfs(task.0, &mut visited, &mut flow, workflow_index)
            }
        }

        flow
    }

    /// Copies the source directory and its files to the destination directory.
    ///
    /// # Arguments
    ///
    /// * `src` - A reference to the source directory path
    /// * `dest` - A reference to the destination directory path
    /// * `file` - An optional string slice representing the specific file to be copied
    ///
    /// This method copies the source directory and all its files to the destination directory
    /// . If the `file` argument is provided, only the specified file is copied.
    ///
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

    /// Builds and Fetches the WebAssembly (WASM) file
    /// This method is called by the `generate`
    ///
    /// # Arguments
    ///
    /// * `pwd` - A reference to the Path indicating the current working directory
    /// * `workflow_package_name` - A string slice representing the name of the
    ///   workflow package
    ///
    fn fetch_wasm(&self, pwd: &Path, workflow_package_name: &str) {
        Command::new("rustup")
            .current_dir(pwd.join(format!("temp-{}", workflow_package_name)))
            .args(["target", "add", "wasm32-wasi"])
            .status()
            .expect("adding wasm32-wasi rust toolchain command failed to start");

        Command::new("cargo")
            .current_dir(pwd.join(format!("temp-{}", workflow_package_name)))
            .args(["build", "--release", "--target", "wasm32-wasi"])
            .status()
            .expect("building wasm32 command failed to start");

        let src = pwd.join(format!(
            "temp-{}/target/wasm32-wasi/release",
            workflow_package_name
        ));
        fs::rename(
            src.join("workflow.wasm"),
            src.join(format!("{}.wasm", workflow_package_name)),
        )
        .unwrap();

        let dest = pwd.join("workflow_wasm");

        self.copy_dir(
            &src,
            &dest,
            Some(&format!("{}.wasm", workflow_package_name)),
        )
        .unwrap();
    }

    /// Generates workflow package and builds the WASM file for all of the workflows
    /// inside the composer
    ///
    /// # Arguments
    ///
    /// * `current_path` - A reference to the Path indicating the current working directory
    ///
    pub fn generate(&self, current_path: &Path) {
        // Getting the current working directory
        let src_path = current_path.join("boilerplate");

        for (workflow_index, workflow) in self.workflows.borrow().iter().enumerate() {
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
                self.generate_types_rs_file_code(workflow_index),
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
