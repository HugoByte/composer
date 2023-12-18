use super::*;

/// Formats the key-value pairs from the given HashMap into a specific string format
/// This string will be passed to the macros as arguments
/// # Arguments
///
/// * `map` - A reference to the HashMap containing key-value pairs
///
/// # Returns
///
/// * A String containing formatted key-value pairs enclosed in square brackets
///
pub fn parse_hashmap(map: &HashMap<String, RustType>) -> String {
    let mut attributes = "[".to_string();

    for (index, (k, v)) in map.iter().enumerate() {
        attributes = format!("{attributes}{}:{}", k, v);

        if index != map.len() - 1 {
            attributes = format!("{attributes},")
        } else {
            break;
        }
    }

    format!("{attributes}]")
}

/// Validates the kind name of the task and returns the formatted kind if valid
///
/// # Arguments
///
/// * `kind` - A reference to the kind name of the task
///
/// # Returns
///
/// * An Ok Result containing the formatted kind if the input is valid
/// * An Err Result with an ErrorKind::NotFound if the input is not valid
///
pub fn get_task_kind(kind: &str) -> Result<String, ErrorKind> {
    match kind.to_lowercase().as_str() {
        "openwhisk" => Ok("OpenWhisk".to_string()),
        "polkadot" => Ok("Polkadot".to_string()),
        _ => Err(ErrorKind::NotFound),
    }
}

/// Returns a string containing Rust code to create structs using macros
///
/// # Returns
///
/// * A String containing Rust code for creating structs using macros
///
fn get_macros_code() -> String {
    "
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use super::*;
use openwhisk_macro::*;

macro_rules! make_input_struct {
    (
        $x:ident,
        // list of field and it's type
        [$(
            $(#[$default_derive:stmt])?
            $visibility:vis $element:ident : $ty:ty),*],
        // list of derive macros
        [$($der:ident),*]
) => {
        #[derive($($der),*)]
            pub struct $x { 
            $(
                $(#[serde(default=$default_derive)])?
                $visibility  $element: $ty
            ),*
        }
    }
}

macro_rules! make_main_struct {
    (
        $name:ident,
        $input:ty,
        [$($der:ident),*],
        // list of attributes
        [$($key:ident : $val:expr),*],
        $output_field: ident
) => {
        #[derive($($der),*)]
        $(
            #[$key = $val]
        )*
        pub struct $name {
            action_name: String,
            pub input: $input,
            pub output: Value,
            pub mapout: Value
        }
        impl $name{
            pub fn output(&self) -> Value {
                self.$output_field.clone()
            }
        }
    }
}

macro_rules! impl_new {
    (
        $name:ident,
        $input:ident,
        []
    ) => {
        impl $name{
            pub fn new(action_name:String) -> Self{
                Self{
                    action_name,
                    input: $input{
                        ..Default::default()
                    },
                    ..Default::default()
                }      
            }
        }
    };
    (
        $name:ident,
        $input:ident,
        [$($element:ident : $ty:ty),*]
    ) => {
        impl $name{
            pub fn new($( $element: $ty),*, action_name:String) -> Self{
                Self{
                    action_name,
                    input: $input{
                        $($element),*,
                        ..Default::default()
                    },
                    ..Default::default()
                }      
            }
        }
    }
}

macro_rules! impl_setter {
    (
        $name:ty,
        [$($element:ident : $key:expr),*]
    ) => {
        impl $name{
            pub fn setter(&mut self, value: Value) {
                $(
                    let val = value.get($key).unwrap();
                    self.input.$element = serde_json::from_value(val.clone()).unwrap();
                )*
            }
        }
    }
}

macro_rules! impl_map_setter {
    (
        $name:ty,
        $element:ident : $key:expr,  
        $typ_name : ty,
        $out:expr
    ) => {
        impl $name {
            pub fn setter(&mut self, val: Value) {
                
                    let value = val.get($key).unwrap();
                    let value = serde_json::from_value::<Vec<$typ_name>>(value.clone()).unwrap();
                    let mut map: HashMap<_, _> = value
                        .iter()
                        .map(|x| {
                            self.input.$element = x.to_owned() as $typ_name;
                            self.run();
                            (x.to_owned(), self.output.get($out).unwrap().to_owned())
                        })
                        .collect();
                    self.mapout = to_value(map).unwrap();
                
            }
        }
    }
    }

macro_rules! impl_concat_setter {
    (
        $name:ty,
        $input:ident
    ) => {
        impl $name{
            pub fn setter(&mut self, val: Value) {
                
                    let val: Vec<Value> = serde_json::from_value(val).unwrap();
                    let res = join_hashmap(
                        serde_json::from_value(val[0].to_owned()).unwrap(),
                        serde_json::from_value(val[1].to_owned()).unwrap(),
                    );
                    self.input.$input = res;
            }
        }
    }
}"
    .to_string()
}

fn get_main_method_code_template(tasks_length: usize) -> String {
    format!(
        "#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {{
    const LIMIT: usize = {tasks_length};
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;
"
    )
}

/// Formats the attributes from the given HashMap into a specific string format
/// This string will be passed to the macros as arguments
///
/// # Arguments
///
/// * `map` - A reference to the HashMap containing attribute key-value pairs
///
/// # Returns
///
/// * A String containing formatted attribute key-value pairs enclosed in square brackets
///
/// This formats the value of the attributes as enclosed by double quots
pub fn get_attributes(map: &HashMap<String, String>) -> String {
    let mut attributes = "[".to_string();

    for (index, (k, v)) in map.iter().enumerate() {
        let k = k.to_case(Case::Pascal);

        attributes = format!("{attributes}{}:\"{}\"", k, v);

        if index != map.len() - 1 {
            attributes = format!("{attributes},")
        } else {
            break;
        }
    }

    format!("{attributes}]")
}

/// Creates a Rust code to generate a struct with fields representing inputs not
/// depending on any task
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * A String containing Rust code to create a struct representing inputs not depending
///   on any task
///
fn get_task_common_input_type_constructor(
    composer_custom_types: &HashMap<String, String>,
    workflow: &Workflow,
) -> String {
    let mut common = Vec::<String>::new();
    let mut workflow_custom_types = Vec::<String>::new();

    let mut default_value_functions = String::new();

    for (_, task) in workflow.tasks.iter() {
        for input in task.input_arguments.iter() {
            if let RustType::Struct(name) = &input.input_type {
                workflow_custom_types.push(name.to_string());
            }

            if !input.is_depend {
                if let Some(val) = input.default_value.as_ref() {
                    common.push(format!(
                        "#[\"{}_fn\"] {}:{}",
                        input.name, input.name, input.input_type
                    ));

                    let content = match input.input_type {
                        RustType::String => format!("{val:?}.to_string()"),
                        _ => format!(
                            "let val = serde_json::from_str::<{}>({:?}).unwrap();\n\tval",
                            input.input_type, val
                        ),
                    };

                    let make_fn = format!(
                        "pub fn {}_fn() -> {}{{\n\t{}\n}}\n",
                        input.name, input.input_type, content
                    );
                    default_value_functions = format!("{default_value_functions}{make_fn}");
                } else {
                    common.push(format!("{}:{}", input.name, input.input_type));
                }
            };
        }
    }

    let workflow_custom_types = if !workflow_custom_types.is_empty() {
        let mut build_string = String::new();

        for custom_type in workflow_custom_types.iter() {
            let typ = composer_custom_types.get(custom_type).unwrap();
            build_string = format!("{build_string}{typ}\n");
        }

        build_string
    } else {
        "".to_string()
    };
    format!(
        "{workflow_custom_types}
{default_value_functions}
make_input_struct!(
    Input,
    [{}],
    [Debug, Clone, Default, Serialize, Deserialize]
);",
        common.join(",")
    )
}

fn get_task_type_constructors(workflow: &Workflow) -> String {
    let mut constructors = String::new();

    for task in workflow.tasks.values() {
        let mut parameters = String::new();

        for argument in task.input_arguments.iter() {
            if !argument.is_depend {
                parameters.push_str(&format!("input.{},", argument.name));
            }
        }

        let constructor = format!(
            "let {} = {}::new({}\"{}\".to_string());\n",
            task.action_name.to_case(Case::Snake),
            task.action_name.to_case(Case::Pascal),
            parameters,
            task.action_name.clone()
        );

        constructors.push_str(&constructor);
    }

    constructors
}

fn get_task_input_type_constructors(workflow: &Workflow) -> String {
    let mut input_type_build_string = String::new();

    for task in workflow.tasks.values() {
        let mut arguments = Vec::new();

        for field in task.input_arguments.iter() {
            arguments.push(format!("{}:{}", field.name, field.input_type));
        }

        input_type_build_string.push_str(&format!(
            "make_input_struct!(
                    {}Input,
                    [{}], [Debug, Clone, Default, Serialize, Deserialize]);",
            task.action_name.to_case(Case::Pascal),
            arguments.join(",")
        ));
    }

    input_type_build_string
}

fn get_independent_fields(task: &Task) -> Vec<String> {
    let mut independent_fields = Vec::<String>::new();

    for field in task.input_arguments.iter() {
        if !field.is_depend {
            independent_fields.push(format!("{}:{}", field.name, field.input_type));
        }
    }

    independent_fields
}

/// Generates Rust code to create structs for each task and its input, and creates object
/// for these types inside the main function
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * An array of Strings containing Rust code to create structs and objects for the
///   specified workflow
///
fn get_task_main_type_constructors(workflow: &Workflow) -> String {
    let mut input_structs = String::new();

    for (task_name, task) in workflow.tasks.iter() {
        let task_name = task_name.to_case(Case::Pascal);

        let independent_fields = get_independent_fields(task);

        let output_field = if task.operation.is_map() {
            "mapout"
        } else {
            "output"
        };

        input_structs = format!(
            "{input_structs}
make_main_struct!(
    {task_name},
    {task_name}Input,
    [Debug, Clone, Default, Serialize, Deserialize, {}],
    {},
    {}
);
impl_new!(
    {task_name},
    {task_name}Input,
    [{}]
);
",
            get_task_kind(&task.kind).unwrap(),
            get_attributes(&task.attributes),
            output_field,
            independent_fields.join(",")
        );
    }

    input_structs
}

fn get_impl_setters_code(workflow: &Workflow) -> String {
    let mut impl_setters_code = String::new();

    for (task_name, task) in workflow.tasks.iter() {
        let task_name = task_name.to_case(Case::Pascal);

        let mut setter_fields = Vec::<String>::new();

        for fields in task.depend_on.iter() {
            setter_fields.push(format!("{}:\"{}\"", fields.cur_field, fields.prev_field));
        }

        let setter_build_string = match &task.operation {
            Operation::Map(field) => format!(
                "impl_map_setter!({}, {}, {}, \"{}\");\n",
                task_name,
                setter_fields.join(","),
                task.input_arguments[0].input_type,
                field
            ),
            Operation::Concat => format!(
                "impl_concat_setter!({}, {});\n",
                task_name, task.input_arguments[0].name
            ),
            _ => format!(
                "impl_setter!({}, [{}]);\n",
                task_name,
                setter_fields.join(",")
            ),
        };

        impl_setters_code.push_str(&setter_build_string);
    }

    impl_setters_code
}

/// Generates Rust code to call the `impl_execute_trait!` macro with the arguments as all
/// of the task names
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * A String containing the Rust code to call the `impl_execute_trait!` macro
///
fn get_impl_execute_trait_code(workflow: &Workflow) -> String {
    let mut task_names = Vec::new();

    for task_name in workflow.tasks.keys() {
        task_names.push(task_name.to_case(Case::Pascal));
    }

    format!("impl_execute_trait!({});", task_names.join(","))
}

fn get_task_flow_type(
    workflow: &Workflow,
    flow: &Vec<String>,
    task_index: usize,
    execute_code: &str,
) -> String {
    if task_index + 1 == flow.len() - 1 {
        match workflow
            .tasks
            .get(&flow[task_index + 1])
            .unwrap()
            .depend_on
            .len()
        {
            0 | 1 => {
                format!(
                    "{execute_code}\n\t\t.term(Some({}_index))?;",
                    flow[task_index + 1].to_case(Case::Snake)
                )
            }

            _ => {
                format!(
                    "{execute_code}\n\t\t.pipe({}_index)?\n\t\t.term(None)?;",
                    flow[task_index + 1].to_case(Case::Snake)
                )
            }
        }
    } else {
        format!(
            "{execute_code}\n\t\t.pipe({}_index)?",
            flow[task_index + 1].to_case(Case::Snake)
        )
    }
}

fn get_add_nodes_code(flow: &Vec<String>) -> String {
    let mut add_nodes_code = String::new();

    for i in 0..flow.len() {
        add_nodes_code.push_str(&format!(
            "\tlet {}_index = workflow.add_node(Box::new({}));\n",
            flow[i].to_case(Case::Snake),
            flow[i].to_case(Case::Snake)
        ));
    }

    add_nodes_code
}

fn get_add_edges_code(workflow: &Workflow, flow: &Vec<String>) -> String {
    let mut add_edges_code = "\tworkflow.add_edges(&[\n".to_string();

    for index in 0..flow.len() - 1 {
        for dependent_task_name in workflow
            .tasks
            .get(&flow[index + 1])
            .unwrap()
            .depend_on
            .iter()
        {
            add_edges_code = format!(
                "{add_edges_code}\t\t({}_index, {}_index),\n",
                dependent_task_name.task_name.to_case(Case::Snake),
                flow[index + 1].to_case(Case::Snake)
            );
        }
    }

    format!("{add_edges_code}\t]);")
}

fn get_add_execute_worklow_code(workflow: &Workflow, flow: &Vec<String>) -> String {
    let mut execute_code = "\tlet result = workflow\n\t\t.init()?".to_string();

    for i in 0..flow.len() - 1 {
        execute_code = get_task_flow_type(&workflow, &flow, i, &execute_code);
    }

    execute_code
}

/// Generates Rust code to add workflow nodes and edges
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * An array containing the Rust code to add workflow nodes and edges
///
fn get_workflow_nodes_and_edges_code(workflow: &Workflow) -> String {
    let flow: Vec<String> = workflow.get_flow();

    if flow.is_empty() {
        return "".to_string();
    }

    if flow.len() == 1 {
        return format!(
            "\
let {}_index = workflow.add_node(Box::new({}));
\tlet result = workflow\n\t\t.init()?
\t\t.term(None)?;
Ok(result)
",
            flow[0].to_case(Case::Snake),
            flow[0].to_case(Case::Snake)
        );
    }

    format!(
        "{}\n{}\n{}let result = serde_json::to_value(result).unwrap();\nOk(result)",
        get_add_nodes_code(&flow),
        get_add_edges_code(workflow, &flow),
        get_add_execute_worklow_code(workflow, &flow),
    )
}

/// Generates the main Rust code for the workflow package and creates the `types.rs` file
///
/// # Arguments
///
/// * `workflow_index` - The index of the workflow
///
/// # Returns
///
/// * A String containing the Rust code to be written to `types.rs` file in the workflow package
///
pub fn generate_types_rs_file_code(
    workflow: &Workflow,
    custom_types: &HashMap<String, String>,
) -> String {
    let main_file = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}}}",
        add_polkadot_openwhisk(workflow),
        get_macros_code(),
        get_task_input_type_constructors(workflow),
        get_task_main_type_constructors(workflow),
        get_impl_setters_code(workflow),
        get_task_common_input_type_constructor(custom_types, workflow),
        get_impl_execute_trait_code(workflow),
        get_main_method_code_template(workflow.tasks.len()),
        get_task_type_constructors(workflow),
        get_workflow_nodes_and_edges_code(workflow)
    );
    main_file
}

fn get_openwhisk_kind_dependencies() -> String {
    "\
openwhisk-rust = \"0.1.2\"

"
    .to_string()
}

fn get_polkadot_kind_dependencies() -> String {
    // some of the polkadot dependencies
    "substrate_macro = \"0.1.3\"
    openwhisk-rust = \"0.1.2\"
    pallet-staking = { git = \"https://github.com/paritytech/substrate.git\", package = \"pallet-staking\", rev = \"eb1a2a8\" }
    substrate-api-client = { git = \"https://github.com/HugoByte/substrate-api-client.git\", default-features = false, features = [\"staking-xt\"], branch =\"wasm-support\"}
sp-core = { version = \"6.0.0\", default-features = false, features = [\"full_crypto\"], git = \"https://github.com/paritytech/substrate.git\", rev = \"eb1a2a8\" }
sp-runtime = { version = \"6.0.0\", default-features = false, git = \"https://github.com/paritytech/substrate.git\", rev = \"eb1a2a8\" }
     "
        .to_string()
}

pub fn generate_cargo_toml_dependencies( workflow : &Workflow) -> String {
    // 0th index-openwhisk, 1st index-polkadot
    let  kinds = get_common_kind(workflow);
    
    let mut toml_dependencies = String::new();

    if kinds[0] {
        toml_dependencies = format!("{}", get_openwhisk_kind_dependencies());
    }

    if kinds[1] {
        toml_dependencies = format!("{}", get_polkadot_kind_dependencies());
    }

    if kinds[0] && kinds[1] {
        toml_dependencies = get_polkadot_kind_dependencies();
    }

    toml_dependencies
}

pub fn get_polkadot() -> String {
    "\
    use substrate_macro::Polkadot;
    use sp_core::H256;
    use openwhisk_rust::*;

    "
    .to_string()
}

pub fn get_openwhisk() -> String {
    "\
    use openwhisk_rust::*;
   
    
    "
    .to_string()
}

pub fn add_polkadot_openwhisk(workflow : &Workflow) -> String {
    let kinds = get_common_kind(workflow);
    
    let mut toml_dependencies = String::new();

    if kinds[0] {
        toml_dependencies = format!("{}", get_openwhisk());
    }

    if kinds[1] {
        toml_dependencies = format!("{}", get_polkadot());
    }

    if kinds[0] && kinds[1] {
        toml_dependencies = get_polkadot();
    }

    toml_dependencies
}

pub fn staking_ledger() -> String{
    "\
use sp_runtime::AccountId32;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug)]
 pub struct StakingLedger {
 pub stash: AccountId32,
 #[codec(compact)]
 pub total: u128,
 #[codec(compact)]
 pub active: u128,
 pub unlocking: Vec<u32>,
 pub claimed_rewards: Vec<u32>,
}
    "
    .to_string()
}

pub fn get_struct_stake_ledger( workflow: &Workflow) -> String{
    let kinds = get_common_kind(workflow);

    let mut toml_dependencies = String::new();

    if kinds[1] {
        toml_dependencies = format!("{}", staking_ledger());
    }

    toml_dependencies

}

pub fn get_common_kind(workflow: &Workflow) -> [bool; 2]{
    let mut kinds = [false, false];

    for task in workflow.tasks.values() {
        match task.kind.to_lowercase().as_str() {
            "openwhisk" => {
                if !kinds[0] {
                    kinds[0] = true
                }
            }
            "polkadot" => {
                if !kinds[1] {
                    kinds[1] = true
                }
            }
            _ => (),
        }

        if kinds[0] && kinds[1] {
            break;
        }
    }

    kinds
}

