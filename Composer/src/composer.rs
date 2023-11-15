use super::*;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Composer {
    config_file: String,
    pub workflows: RefCell<Vec<Workflow>>,
    pub custom_types: RefCell<HashMap<String, String>>,
}

impl Composer {
    
    pub fn new(config_file: &str) -> Self{
        Composer{
            config_file: config_file.to_string(),
            ..Default::default()
        }
    }

    pub fn run(&self) {

        let content: String = std::fs::read_to_string(&self.config_file).unwrap();
        let ast = AstModule::parse("config", content.to_owned(), &Dialect::Extended).unwrap();
        
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
            Ok(self.workflows.borrow_mut().push(Workflow {
                name,
                version,
                tasks,
                custom_types,
            }))
        }
    }

    pub fn add_custom_type(&self, type_name: &str, build_string: String) {
        self.custom_types
            .borrow_mut()
            .insert(String::from(type_name), String::from(build_string));
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

    fn dfs(&self, task_name: &str, visited: &mut HashMap<String, bool>, flow: &mut Vec<String>, workflow_index: usize) {
        visited.insert(String::from(task_name), true);

        for d in self.get_dependencies(task_name, workflow_index).unwrap().iter() {
            if !visited[d] {
                self.dfs(d, visited, flow, workflow_index);
            }
        }

        flow.push(String::from(task_name));
    }

    pub fn get_flow(&self, workflow_index: usize) -> Vec<String> {
        let mut visited = HashMap::<String, bool>::new();
        let mut flow = Vec::<String>::new();

        for t in self.workflows.borrow()[workflow_index].tasks.iter() {
            visited.insert(String::from(t.0), false);
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

    // Function to generate a new Cargo package and write the main.rs and Cargo.toml files
    #[allow(dead_code)]
    fn generate_cargo(
        &self,
        project_name: &str,
        path: &PathBuf,
        main_file_content: &str,
        cargo_toml_content: &str,
    ) {
        // Generating a new Cargo package
        Command::new("cargo")
            .args(&["new", &project_name, "--lib"])
            .status()
            .unwrap();

        // Creating and writing into the files
        fs::write(&(path.join("src/lib.rs")), main_file_content).unwrap();
        fs::write(&(path.join("Cargo.toml")), cargo_toml_content).unwrap();
    }

    pub fn generate(&self) {
        // Getting the current working directory
        let path = env::current_dir().unwrap().join("./workflows");
        fs::create_dir_all(&path).unwrap();

        for (i, workflow) in self.workflows.borrow().iter().enumerate(){
            fs::create_dir_all(path.join(format!("./{}", workflow.name))).unwrap();
            fs::write(path.join(format!("./{}/types.rs", workflow.name)), self.generate_main_file_code(i)).unwrap();
        }
    }
}
