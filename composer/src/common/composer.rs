use super::*;

const COMMON: &str = include_str!("../../../boilerplate/src/common.rs");
const LIB: &str = include_str!("../../../boilerplate/src/lib.rs");
const TRAIT: &str = include_str!("../../../boilerplate/src/traits.rs");
const CARGO: &str = include_str!("../../../boilerplate/Cargo.toml");

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

    /// Retrieves user-defined types and creates code to generate corresponding structs
    /// This method is invoked by the starlark_module
    ///
    /// # Arguments
    ///
    /// * `workflow_index` - The index of the workflow
    ///
    /// # Returns
    ///
    /// * A String containing code to create user-defined types as structs
    ///
    pub fn get_user_defined_types(&self, types: Vec<String>) -> String {
        let mut build_string = String::new();
        let custom_types = self.custom_types.borrow();

        for type_ in types.iter() {
            let typ = custom_types.get(type_).unwrap();
            build_string = format!("{build_string}{typ}\n");
        }

        build_string
    }

    pub fn build(&self, verbose: bool, pb: &mut ProgressBar, temp_dir: &PathBuf) {
        pb.inc(10 / self.config_files.len() as u64);

        if verbose {
            Command::new("rustup")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["target", "add", "wasm32-wasi"])
                .status()
                .expect("adding wasm32-wasi rust toolchain command failed to start");

            Command::new("cargo")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["build", "--release", "--target", "wasm32-wasi"])
                .status()
                .expect("building wasm32 command failed to start");
        } else {
            Command::new("cargo")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["build", "--release", "--target", "wasm32-wasi", "--quiet"])
                .status()
                .expect("building wasm32 command failed to start");
        }
    }

    fn copy_boilerplate(
        &self,
        types_rs: &str,
        workflow_name: String,
        pb: &mut ProgressBar,
    ) -> PathBuf {
        pb.inc(5 / self.config_files.len() as u64);
        let temp_dir = std::env::temp_dir().join(workflow_name);
        let curr = temp_dir.join("boilerplate");

        std::fs::create_dir_all(curr.clone().join("src")).unwrap();

        let src_curr = temp_dir.join("boilerplate/src");
        let temp_path = src_curr.as_path().join("common.rs");

        std::fs::write(temp_path, &COMMON[..]).unwrap();

        let temp_path = src_curr.as_path().join("lib.rs");
        std::fs::write(temp_path, &LIB[..]).unwrap();
        let temp_path = src_curr.as_path().join("types.rs");
        std::fs::write(temp_path, types_rs).unwrap();

        let temp_path = src_curr.as_path().join("traits.rs");
        std::fs::write(temp_path, &TRAIT[..]).unwrap();

        let cargo_path = curr.join("Cargo.toml");
        std::fs::write(cargo_path, &CARGO[..]).unwrap();

        pb.inc(10 / self.config_files.len() as u64);

        temp_dir
    }

    fn compile_starlark(&self, config: &str) -> Result<(), anyhow::Error> {
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
        let uint = module.heap().alloc(RustType::Uint);
        module.set("Uint", uint);
        let int = module.heap().alloc(RustType::Float);
        module.set("Float", int);
        let int = module.heap().alloc(RustType::String);
        module.set("String", int);
        let int = module.heap().alloc(RustType::Boolean);
        module.set("Bool", int);

        {
            let mut eval = Evaluator::new(&module);
            // We add a reference to our store
            eval.extra = Some(self);
            eval.eval_module(ast, &globals)?;
        }

        Ok(())
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
        pb.inc(10 / self.config_files.len() as u64);

        for config in self.config_files.iter() {
            self.compile_starlark(config)?;
            pb.inc(5 / self.config_files.len() as u64);
        }

        for (workflow_index, workflow) in self.workflows.borrow().iter().enumerate() {
            if workflow.tasks.is_empty() {
                continue;
            }

            let workflow_name = format!("{}_{}", workflow.name, workflow.version);
            pb.inc(10 / self.config_files.len() as u64);

            let temp_dir = self.copy_boilerplate(
                &generate_types_rs_file_code(&self, &self.workflows.borrow()[workflow_index]),
                workflow_name.clone(),
                pb,
            );

            self.build(verbose, pb, &temp_dir);

            let wasm_path = format!(
                "{}/boilerplate/target/wasm32-wasi/release/boilerplate.wasm",
                temp_dir.as_path().to_str().unwrap()
            );

            fs::copy(
                wasm_path,
                &std::env::current_dir()
                    .unwrap()
                    .join(format!("{workflow_name}.wasm")),
            )
            .unwrap();

            fs::remove_dir_all(temp_dir).unwrap();
        }

        Ok(())
    }
}
