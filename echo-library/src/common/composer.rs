use anyhow::{anyhow, Ok};
use composer_primitives::types::SourceFiles;
use rayon::prelude::*;
use starlark::environment::FrozenModule;
use starlark::eval::ReturnFileLoader;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::*;

const COMMON: &str = include_str!("../boilerplate/src/common.rs");
const LIB: &str = include_str!("../boilerplate/src/lib.rs");
const TRAIT: &str = include_str!("../boilerplate/src/traits.rs");
const MACROS: &str = include_str!("../boilerplate/src/macros.rs");
const CARGO: &str = include_str!("../boilerplate/Cargo.toml");

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Composer {
    pub config_files: Vec<String>,
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
    /// use echo_library::Composer;
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

    pub fn build(verbose: bool, temp_dir: &Path) -> Result<(), Error> {
        if verbose {
            Command::new("rustup")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["target", "add", "wasm32-wasi"])
                .status()?;

            Command::new("cargo")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["build", "--release", "--target", "wasm32-wasi"])
                .status()?;
        } else {
            Command::new("cargo")
                .current_dir(temp_dir.join("boilerplate"))
                .args(["build", "--release", "--target", "wasm32-wasi", "--quiet"])
                .status()?;
        }
        Ok(())
    }

    fn copy_boilerplate(
        temp_dir: &Path,
        types_rs: String,
        workflow_name: String,
        workflow: &Workflow,
    ) -> Result<PathBuf, Error> {
        let temp_dir = temp_dir.join(workflow_name);
        let curr = temp_dir.join("boilerplate");

        std::fs::create_dir_all(curr.clone().join("src"))?;

        let src_curr = temp_dir.join("boilerplate/src");
        let temp_path = src_curr.as_path().join("common.rs");

        std::fs::write(temp_path, COMMON)?;

        let temp_path = src_curr.as_path().join("lib.rs");
        std::fs::write(temp_path.clone(), LIB)?;

        let mut lib = OpenOptions::new()
            .write(true)
            .append(true)
            .open(temp_path)?;

        let library = get_struct_stake_ledger(workflow);
        writeln!(lib, "{library}").expect("could not able to add struct to lib");

        let temp_path = src_curr.as_path().join("types.rs");
        std::fs::write(temp_path, types_rs)?;

        let temp_path = src_curr.as_path().join("traits.rs");
        std::fs::write(temp_path, TRAIT)?;

        let temp_path = src_curr.as_path().join("macros.rs");
        std::fs::write(temp_path, MACROS)?;

        let cargo_path = curr.join("Cargo.toml");
        std::fs::write(cargo_path.clone(), CARGO)?;

        let mut cargo_toml = OpenOptions::new()
            .write(true)
            .append(true)
            .open(cargo_path)?;

        let dependencies = generate_cargo_toml_dependencies(workflow);
        writeln!(cargo_toml, "{dependencies}")
            .expect("could not able to add dependencies to the Cargo.toml");

        Ok(temp_dir)
    }
}

impl Composer {
    pub fn compile(&self, module: &str, files: &SourceFiles) -> Result<FrozenModule, Error> {
        let ast: AstModule = AstModule::parse_file(
            files
                .files()
                .get(&PathBuf::from(format!(
                    "{}/{}",
                    files.base().display(),
                    module
                )))
                .ok_or_else(|| {
                    Error::msg(format!(
                        "FileNotFound at {}/{}",
                        files.base().display(),
                        module
                    ))
                })?,
            &Dialect::Extended,
        )
        .map_err(|err| Error::msg(format!("Error parsing file: {}", err)))?;

        let mut loads = Vec::new();

        for load in ast.loads() {
            loads.push((
                load.module_id.to_owned(),
                Self::compile(self, load.module_id, files)?,
            ));
        }

        let modules = loads.iter().map(|(a, b)| (a.as_str(), b)).collect();
        let loader = ReturnFileLoader { modules: &modules };

        // We build our globals by adding some functions we wrote
        let globals = GlobalsBuilder::extended_by(&[
            StructType, RecordType, EnumType, Map, Filter, Partial, Debug, Print, Pprint,
            Breakpoint, Json, Typing, Internal, CallStack,
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
            let result = {
                let mut eval = Evaluator::new(&module);
                // We add a reference to our store
                eval.set_loader(&loader);
                eval.extra = Some(self);
                eval.eval_module(ast, &globals)
            };

            result.map_err(|err| Error::msg(format!("Evaluation error: {}", err)))?;
        }

        if self.workflows.borrow().is_empty(){
            return Err(Error::msg("Empty workflow detected!!!"));
        }
        Ok(module.freeze()?)
    }

    pub fn build_directory(
        &self,
        build_path: &Path,
        out_path: &Path,
        quiet: bool,
    ) -> anyhow::Result<(), Error> {
        let composer_custom_types = self.custom_types.take();

        let workflows = self.workflows.take();

        let results: Vec<Result<(), Error>> = workflows
            .par_iter()
            .enumerate()
            .map(|workflow: (usize, &Workflow)| {
                if workflow.1.tasks.is_empty() {
                    return Ok(());
                }

                let workflow_name = format!("{}_{}", workflow.1.name, workflow.1.version);

                let types_rs =
                    generate_types_rs_file_code(&workflows[workflow.0], &composer_custom_types)
                        .map_err(|err| {
                            anyhow!(
                                "{}: Failed to generate types.rs file: {}",
                                workflow.1.name,
                                err
                            )
                        })?;

                let temp_dir =
                    Self::copy_boilerplate(build_path, types_rs, workflow_name.clone(), workflow.1)
                        .map_err(|err| {
                            anyhow!("{}: Failed to copy boilerplate: {}", workflow.1.name, err)
                        })?;

                Self::build(quiet, &temp_dir)
                    .map_err(|err| anyhow!("{}: Failed to build: {}", workflow.1.name, err))?;

                let wasm_path = format!(
                    "{}/boilerplate/target/wasm32-wasi/release/boilerplate.wasm",
                    temp_dir.display()
                );

                fs::create_dir_all(out_path.join("output")).map_err(|err| {
                    anyhow!(
                        "{}: Failed to create output directory: {}",
                        workflow.1.name,
                        err
                    )
                })?;

                fs::copy(
                    wasm_path,
                    out_path.join(format!("output/{workflow_name}.wasm")),
                )
                .map_err(|err| anyhow!("{}: Failed to copy wasm: {}", workflow.1.name, err))?;

                fs::remove_dir_all(temp_dir).map_err(|err| {
                    anyhow!("{}: Failed to remove temp dir: {}", workflow.1.name, err)
                })?;

                Ok(())
            })
            .filter(|result| result.is_err())
            .collect::<Vec<_>>()
            .into_iter()
            .collect();

        if !results.is_empty() {
            return Err(Error::msg(format!(
                "Failed to build the following workflows: {:?}",
                results
            )));
        }

        Ok(())
    }
}
