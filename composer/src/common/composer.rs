
use super::*;

const COMMON: &str = include_str!("../../boilerplate/src/common.rs");
const LIB: &str = include_str!("../../boilerplate/src/lib.rs");
const TRAIT: &str = include_str!("../../boilerplate/src/traits.rs");
const CARGO: &str = include_str!("../../boilerplate/Cargo.toml");

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

    pub fn copy_boilerplate(&self, types_rs: &str, workflow_name: String){

        let temp_dir = std::env::temp_dir().join(&workflow_name);
        let curr = temp_dir.join("boilerplate");

        std::fs::create_dir_all(curr.clone()).unwrap();

        std::fs::create_dir_all(curr.clone().join("src")).unwrap();
 
        let src_curr = temp_dir.join("boilerplate/src");
        let temp_path = src_curr.as_path().join("common.rs");
        // println!("{:?}", temp_path);
        std::fs::write(&temp_path, &COMMON[..]).unwrap();
     
        let temp_path = src_curr.as_path().join("lib.rs");
        std::fs::write(&temp_path, &LIB[..]).unwrap();
        let temp_path = src_curr.as_path().join("types.rs");
        std::fs::write(&temp_path, types_rs).unwrap();
     
        let temp_path = src_curr.as_path().join("traits.rs");
        std::fs::write(&temp_path, &TRAIT[..]).unwrap();
     
        let cargo_path = curr.join("Cargo.toml");
        std::fs::write(&cargo_path, &CARGO[..]).unwrap();

        Command::new("cargo")
            .current_dir(temp_dir.join("boilerplate"))
            .args(["build", "--release", "--target", "wasm32-wasi"])
            .status()
            .expect("building wasm32 command failed to start");
        
        let wasm_path = format!("{}/target/wasm32-wasi/release/workflow.wasm",curr.as_path().to_str().unwrap() );
  
        fs::copy(wasm_path, &std::env::current_dir().unwrap().join(format!("{workflow_name}.wasm"))).unwrap();

 
    }

    fn compile_starlark(&self, config: &str) -> Composer {
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

        let int = module.heap().alloc(RustType::Int);
        module.set("Int", int);
        let int = module.heap().alloc(RustType::Float);
        module.set("Float", int);
        let int = module.heap().alloc(RustType::String);
        module.set("String", int);
        let int = module.heap().alloc(RustType::Boolean);
        module.set("Bool", int);

        let composer = Composer::default();
        {
            let mut eval = Evaluator::new(&module);
            // We add a reference to our store
            eval.extra = Some(&composer);
            eval.eval_module(ast, &globals).unwrap();
        }

        composer
    }

    /// Generates workflow package and builds the WASM file for all of the workflows
    /// inside the composer
    ///
    /// # Arguments
    ///
    /// * `current_path` - A reference to the Path indicating the current working directory
    ///
    pub fn generate(&self, verbose: bool, pb: &mut ProgressBar) -> Result<(), Error> {
        // Getting the current working directory

        for config in self.config_files.iter() {
            let composer = self.compile_starlark(config);

            for (workflow_index, workflow) in composer.workflows.borrow().iter().enumerate() {
               
                if workflow.tasks.is_empty() {
                    continue;
                }

                let workflow_name = format!("{}_{}", workflow.name, workflow.version);
                self.copy_boilerplate(&self.generate_types_rs_file_code(workflow_index), workflow_name);

            }
        }

        Ok(())
    }
}
