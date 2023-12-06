use super::*;

impl Composer {
    /// Returns a string containing Rust code to create structs using macros
    ///
    /// # Returns
    ///
    /// * A String containing Rust code for creating structs using macros
    ///
    pub fn get_macros(&self) -> String {
        "use serde_json::Value;
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
    pub fn get_attributes(&self, map: &HashMap<String, String>) -> String {
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
    pub fn parse_hashmap(&self, map: &HashMap<String, RustType>) -> String {
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
    pub fn get_task_kind(&self, kind: &str) -> Result<String, ErrorKind> {
        match kind.to_lowercase().as_str() {
            "openwhisk" => Ok("OpenWhisk".to_string()),
            "polkadot" => Ok("Polkadot".to_string()),
            _ => Err(ErrorKind::NotFound),
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
    pub fn get_common_inputs_type(&self, workflow_index: usize) -> String {
        let mut common = Vec::<String>::new();
        let mut custom_types = Vec::<String>::new();

        let mut default_value_functions = String::new();

        for (_, task) in self.workflows.borrow()[workflow_index].tasks.iter() {
            let mut depend = Vec::<String>::new();

            for fields in task.depend_on.iter() {
                depend.push(fields.cur_field.to_string());
            }

            for input in task.input_arguments.iter() {
                if let RustType::Struct(name) = &input.input_type {
                    custom_types.push(name.to_string());
                }

                if !depend.contains(&input.name) {
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

        let custom_types = if !custom_types.is_empty() {
            self.get_user_defined_types(custom_types)
        } else {
            "".to_string()
        };

        format!(
            "{custom_types}
{default_value_functions}
make_input_struct!(
    Input,
    [{}],
    [Debug, Clone, Default, Serialize, Deserialize]
);",
            common.join(",")
        )
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
    pub fn get_types_and_constructors(&self, workflow_index: usize) -> [String; 2] {
        let mut constructors = String::new();
        let mut input_structs = String::new();

        for (task_name, task) in self.workflows.borrow()[workflow_index].tasks.iter() {
            let task_name = task_name.to_case(Case::Pascal);

            let mut depend = Vec::<String>::new();
            let mut setter = Vec::<String>::new();
            let mut map_setter = String::new();

            for fields in task.depend_on.iter() {
                depend.push(fields.cur_field.clone());
                setter.push(format!("{}:\"{}\"", fields.cur_field, fields.prev_field));
            }

            let field = match &task.operation {
                Operation::Map(_) => "map",
                _ => "",
            };
            map_setter.push_str(&field);

            let mut input = format!(
                "make_input_struct!(
    {task_name}Input,
    ["
            );

            let mut not_depend = Vec::<String>::new();

            for (index, field) in task.input_arguments.iter().enumerate() {
                input = format!("{input}{}:{}", field.name, field.input_type);

                if index != task.input_arguments.len() - 1 {
                    input = format!("{input},");
                } else {
                    input =
                        format!("{input}],\n\t[Debug, Clone, Default, Serialize, Deserialize]);");
                }

                if !depend.contains(&field.name) {
                    not_depend.push(format!("{}:{}", field.name, field.input_type));
                }
            }

            let setter_macro = match &task.operation {
                Operation::Map(field) => format!(
                    "impl_map_setter!({}, {}, {}, \"{}\");",
                    task_name,
                    setter.join(","),
                    task.input_arguments[0].input_type,
                    field
                ),
                Operation::Concat => format!(
                    "impl_concat_setter!({}, {});",
                    task_name, task.input_arguments[0].name
                ),
                _ => format!("impl_setter!({}, [{}]);", task_name, setter.join(",")),
            };

            input_structs = format!(
                "{input_structs}
{input}
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
{setter_macro}
",
                self.get_task_kind(&task.kind).unwrap(),
                self.get_attributes(&task.attributes),
                if let Operation::Map(_) = task.operation {
                    "mapout"
                } else {
                    "output"
                },
                not_depend.join(",")
            );

            constructors = {
                let commons: Vec<String> = not_depend
                    .iter()
                    .map(|x| format!("input.{}", x.split(':').collect::<Vec<&str>>()[0]))
                    .collect();

                let commons = commons.join(",");

                format!(
                    "{constructors}\tlet {} = {}::new({}{}\"{}\".to_string());\n",
                    task.action_name.to_case(Case::Snake),
                    task_name,
                    commons,
                    if !commons.is_empty() { ", " } else { "" },
                    task.action_name.clone()
                )
            };
        }

        [input_structs, constructors]
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
    pub fn get_impl_execute_trait_code(&self, workflow_index: usize) -> String {
        let mut build_string = String::from("\nimpl_execute_trait!(");
        let len = self.workflows.borrow()[workflow_index].tasks.len();

        for (index, task) in self.workflows.borrow()[workflow_index]
            .tasks
            .iter()
            .enumerate()
        {
            build_string = format!("{build_string}{}", task.1.action_name.to_case(Case::Pascal));

            build_string = if index != len - 1 {
                format!("{build_string},")
            } else {
                format!("{build_string});\n")
            }
        }

        build_string
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
    pub fn get_workflow_nodes_and_edges(&self, workflow_index: usize) -> [String; 2] {
        let mut execute_code = "\tlet result = workflow\n\t\t.init()?".to_string();

        let flow: Vec<String> = self.get_flow(workflow_index);

        if flow.is_empty() {
            return ["".to_string(), "".to_string()];
        }

        if flow.len() == 1 {
            return [
                format!(
                    "let {}_index = workflow.add_node(Box::new({}));\n",
                    flow[0].to_case(Case::Snake),
                    flow[0].to_case(Case::Snake)
                ),
                format!("{}\n\t\t.term(None)?;", execute_code),
            ];
        }

        let mut add_nodes_code = String::new();
        let mut add_edges_code = "\tworkflow.add_edges(&[\n".to_string();

        for i in 0..flow.len() - 1 {
            add_nodes_code = format!(
                "{add_nodes_code}\tlet {}_index = workflow.add_node(Box::new({}));\n",
                flow[i].to_case(Case::Snake),
                flow[i].to_case(Case::Snake)
            );

            for dependent_task_name in self.workflows.borrow()[workflow_index]
                .tasks
                .get(&flow[i + 1])
                .unwrap()
                .depend_on
                .iter()
            {
                add_edges_code = format!(
                    "{add_edges_code}\t\t({}_index, {}_index),\n",
                    dependent_task_name.task_name.to_case(Case::Snake),
                    flow[i + 1].to_case(Case::Snake)
                );
            }

            execute_code = if i + 1 == flow.len() - 1 {
                match self.workflows.borrow()[workflow_index]
                    .tasks
                    .get(&flow[i + 1])
                    .unwrap()
                    .depend_on
                    .len()
                {
                    0 | 1 => {
                        format!(
                            "{execute_code}\n\t\t.term(Some({}_index))?;",
                            flow[i + 1].to_case(Case::Snake)
                        )
                    }

                    _ => {
                        format!(
                            "{execute_code}\n\t\t.pipe({}_index)?\n\t\t.term(None)?;",
                            flow[i + 1].to_case(Case::Snake)
                        )
                    }
                }
            } else {
                format!(
                    "{execute_code}\n\t\t.pipe({}_index)?",
                    flow[i + 1].to_case(Case::Snake)
                )
            };
        }

        add_nodes_code = format!(
            "{add_nodes_code}\tlet {}_index = workflow.add_node(Box::new({}));\n",
            flow[flow.len() - 1].to_case(Case::Snake),
            flow[flow.len() - 1].to_case(Case::Snake)
        );

        add_edges_code = format!("{add_edges_code}\t]);\n\n{execute_code}");

        [add_nodes_code, add_edges_code]
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
    pub fn generate_types_rs_file_code(&self, workflow_index: usize) -> String {
        let structs = self.get_types_and_constructors(workflow_index);
        let workflow_nodes_and_edges = self.get_workflow_nodes_and_edges(workflow_index);

        let main_file = format!(
            "{}            
{}
{}
{}
#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {{
    const LIMIT: usize = {};
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

{}
{}
{}
    let result = serde_json::to_value(result).unwrap();
    Ok(result)
}}
",
            self.get_macros(),
            structs[0],
            self.get_common_inputs_type(workflow_index),
            self.get_impl_execute_trait_code(workflow_index),
            self.workflows.borrow()[workflow_index].tasks.len(),
            structs[1],
            workflow_nodes_and_edges[0],
            workflow_nodes_and_edges[1]
        );

        main_file
    }
}
