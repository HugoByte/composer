use super::*;

#[starlark_module]
pub fn starlark_workflow_module(builder: &mut GlobalsBuilder) {
    /// Creates a new task of the workflow and returns a task object of `Task` type
    /// This method will be invoked inside the config file.
    ///
    /// # Arguments
    ///
    /// * `kind` - A string that holds the kind of the task (i.e "polkadot", "openwhisk")
    /// * `action_name` - A string that holds the the name of the action associated with the task
    /// * `input_args` - The input arguments for the task
    /// * `attributes` - The attributes of the task
    /// * `depend_on` - The dependencies of the task
    /// * `operation` - An optional argument to mention type of the task operation
    ///   (i.e "map", "concat")
    ///
    /// # Returns
    ///
    /// * A Result containing the task object if the task is created successfully
    ///
    fn task(
        kind: String,
        action_name: String,
        input_args: Value,
        attributes: Value,
        depend_on: Value,
        operation: Option<String>,
    ) -> anyhow::Result<Task> {
        let input_args: Vec<Input> = serde_json::from_str(&input_args.to_json()?).unwrap();
        let attributes: HashMap<String, String> =
            serde_json::from_str(&attributes.to_json()?).unwrap();
        let depend_on: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&depend_on.to_json()?).unwrap();

        let operation = match operation {
            Some(a) => a,
            None => String::default(),
        };

        Ok(Task {
            kind,
            action_name,
            input_args,
            attributes,
            operation,
            depend_on,
        })
    }

    /// Creates and adds a new workflow to the composer
    /// This method will be invoked inside the config file.
    ///
    /// # Arguments
    ///
    /// * `name` - A string that holds the name of the workflow
    /// * `version` - A string that holds the version of the workflow
    /// * `tasks` - The tasks of the workflow
    /// * `custom_types` - Optional custom types for the workflow
    /// * `eval` - A mutable reference to the Evaluator (injected by the starlark rust package)
    ///
    /// # Returns
    ///
    /// * a workflow object of `Workflow` type
    ///
    fn workflows(
        name: String,
        version: String,
        tasks: Value,
        custom_types: Option<Value>,
        eval: &mut Evaluator,
    ) -> anyhow::Result<Workflow> {
        let tasks: Vec<Task> = serde_json::from_str(&tasks.to_json()?).unwrap();

        let custom_types: Option<Vec<String>> = match custom_types {
            Some(a) => serde_json::from_str(&a.to_json()?).unwrap(),
            None => None,
        };

        let mut task_hashmap = HashMap::new();

        for task in tasks {
            if task_hashmap.contains_key(&task.action_name) {
                return Err(Error::msg("Duplicate tasks, Task names must be unique"));
            } else {
                task_hashmap.insert(task.action_name.clone(), task);
            }
        }

        eval.extra
            .unwrap()
            .downcast_ref::<Composer>()
            .unwrap()
            .add_workflow(
                name.clone(),
                version.clone(),
                task_hashmap.clone(),
                custom_types.clone(),
            )
            .unwrap();

        Ok(Workflow {
            name,
            version,
            tasks: task_hashmap,
            custom_types,
        })
    }

    /// Creates a new field for the input argument of a task
    ///
    /// # Arguments
    ///
    /// * `name` - A string that holds the name of the input field
    /// * `input_type` - A string that holds the type of the input field
    /// * `default_value` - An optional JSON default value for the input field
    ///
    /// # Returns
    ///
    /// * A Result containing the input object of `Input` type
    ///
    fn input_args(
        name: String,
        input_type: String,
        default_value: Option<String>,
    ) -> anyhow::Result<Input> {
        let default_value = match default_value {
            Some(b) => b,
            None => String::default(),
        };
        Ok(Input {
            name,
            input_type,
            default_value,
        })
    }
}

#[starlark_module]
pub fn starlark_datatype_module(builder: &mut GlobalsBuilder) {
    /// Creates a user-defined type inside the `types.rs`.
    /// This method will be invoked inside the config file.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the user-defined type
    /// * `fields` - The fields of the user-defined type in JSON format
    /// * `eval` - A mutable reference to the Evaluator (injected by the starlark rust package)
    ///
    /// # Returns
    ///
    /// * A Result containing the name of the user-defined type
    ///
    fn typ(name: String, fields: Value, eval: &mut Evaluator) -> anyhow::Result<String> {
        let fields: HashMap<String, String> = serde_json::from_str(&fields.to_json()?).unwrap();

        let composer = eval.extra.unwrap().downcast_ref::<Composer>().unwrap();

        let name = name.to_case(Case::Pascal);

        composer.add_custom_type(
            &name,
            format!(
                "make_input_struct!(\n\t{},\n\t{},\n\t[Default, Clone, Debug]\n);",
                &name,
                composer.parse_hashmap(&fields)
            ),
        );

        Ok(name)
    }

    /// Returns the Rust type for a string
    /// This method will be invoked inside the config file.
    ///
    /// # Returns
    /// * A string representing the Rust type for a string
    ///
    fn string() -> anyhow::Result<String> {
        Ok("String".to_string())
    }

    /// Returns the Rust type for a bool
    ///
    /// # Returns
    /// * A string representing the Rust type for a bool
    ///
    fn bool() -> anyhow::Result<String> {
        Ok("bool".to_string())
    }

    /// Returns the Rust type for an integer with an optional size
    /// This method will be invoked inside the config file.
    ///
    /// # Arguments
    ///
    /// * `size` - An optional size for the integer
    ///
    /// # Returns
    /// * A Result containing the Rust type for an integer
    /// * an error message if the size is invalid
    ///
    fn int(size: Option<i32>) -> anyhow::Result<String> {
        match size {
            Some(x) => match x {
                8 | 16 | 32 | 64 | 128 => Ok(format!("i{}", x)),
                _ => Err(Error::msg("Size is invalid")),
            },
            None => Ok("i32".to_string()),
        }
    }

    /// Returns the Rust type for a map with specified types of the key and vale
    /// This method will be invoked inside the config file.
    ///
    /// # Arguments
    ///
    /// * `type_1` - The type of the key
    /// * `type_2` - The type of the value
    ///
    /// # Returns
    /// * A Result containing the Rust type for a map
    ///
    fn map(type_1: String, type_2: String) -> anyhow::Result<String> {
        Ok(format!("HashMap<{}, {}>", type_1, type_2))
    }

    /// Returns the Rust type for a list with specified element type
    /// This method will be invoked inside the config file.
    ///
    /// # Arguments
    ///
    /// * `type_` - The type of the element in the list
    ///
    /// # Returns
    ///
    ///  * A Result containing the Rust type for a list
    ///
    fn list(type_: String) -> anyhow::Result<String> {
        Ok(format!("Vec<{}>", type_))
    }

    /// Calls the user-defined type created by the user and validates if the type name exists
    /// This method will be invoked inside the config file.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The name of the user-defined type
    /// * `eval` - A mutable reference to the Evaluator (injected by the starlark rust package)
    ///
    /// # Returns
    ///
    /// * A Result containing the name of the user-defined type if it exists
    /// * an error message if it does not
    ///
    fn Struct(type_name: String, eval: &mut Evaluator) -> anyhow::Result<String> {
        let composer = eval.extra.unwrap().downcast_ref::<Composer>().unwrap();
        let type_name = type_name.to_case(Case::Pascal);

        if !type_name.is_empty() && composer.custom_types.borrow().contains_key(&type_name) {
            Ok(type_name)
        } else {
            Err(Error::msg("type {type_name} does not exist"))
        }
    }
}
