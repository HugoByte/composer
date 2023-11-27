use super::*;

impl Composer {
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
        [$($visibility:vis $element:ident : $ty:ty),*],
        // list of derive macros
        [$($der:ident),*]
) => {
        #[derive($($der),*)]
        pub struct $x { $($visibility  $element: $ty),*}
    }
}

macro_rules! make_main_struct {
    (
        $name:ident,
        $input:ty,
        [$($der:ident),*],
        // list of attributes
        [$($key:ident : $val:expr),*]
) => {
        #[derive($($der),*)]
        $(
            #[$key = $val]
        )*
        pub struct $name {
            action_name: String,
            pub input: $input,
            pub output: Value,
        }
        impl $name{
            pub fn output(&self) -> Value {
                self.output.clone()
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
            pub fn setter(&mut self, val: Value) {
                $(
                let value = val.get($key).unwrap();
                self.input.$element = serde_json::from_value(value.clone()).unwrap();
                )*
            }
        }
    }
}"
        .to_string()
    }

    pub fn get_attributes(&self, map: &HashMap<String, String>) -> String {
        let mut attributes = "[".to_string();

        for (i, (k, v)) in map.iter().enumerate() {
            let k = k.to_case(Case::Pascal);

            attributes = format!("{attributes}{}:\"{}\"", k, v);

            if i != map.len() - 1 {
                attributes = format!("{attributes},")
            } else {
                break;
            }
        }

        format!("{attributes}]")
    }

    pub fn parse_hashmap(&self, map: &HashMap<String, String>) -> String {
        let mut attributes = "[".to_string();

        for (i, (k, v)) in map.iter().enumerate() {
            attributes = format!("{attributes}{}:{}", k, v);

            if i != map.len() - 1 {
                attributes = format!("{attributes},")
            } else {
                break;
            }
        }

        format!("{attributes}]")
    }

    pub fn get_kind(&self, kind: &str) -> Result<String, ErrorKind> {
        match kind.to_lowercase().as_str() {
            "openwhisk" => Ok("OpenWhisk".to_string()),
            "polkadot" => Ok("Polkadot".to_string()),
            _ => Err(ErrorKind::NotFound),
        }
    }

    pub fn get_custom_types(&self, workflow_index: usize) -> String {
        let mut build_string = String::new();
        let custom_types = self.custom_types.borrow();

        if let Some(types) = self.workflows.borrow()[workflow_index]
            .custom_types
            .as_ref()
        {
            for t in types.iter() {
                let typ = custom_types.get(t).unwrap();
                build_string = format!("{build_string}{typ}\n");
            }
        };

        build_string
    }

    pub fn get_custom_structs(&self, workflow_index: usize) -> [String; 2] {
        let mut common_inputs = HashMap::<String, String>::new();

        let mut constructors = String::new();
        let mut input_structs = String::new();

        for (task_name, task) in self.workflows.borrow()[workflow_index].tasks.iter() {
            let task_name = task_name.to_case(Case::Pascal);

            let mut depend = Vec::<String>::new();
            let mut setter = Vec::<String>::new();

            for fields in task.depend_on.values() {
                let x = fields.iter().next().unwrap();
                depend.push(x.0.to_string());

                setter.push(format!("{}:\"{}\"", x.0, x.1));
            }

            let mut input = format!(
                "make_input_struct!(
    {task_name}Input,
    ["
            );

            let mut new = Vec::<String>::new();

            for (i, field) in task.input_args.iter().enumerate() {
                input = format!("{input}{}:{}", field.name, field.input_type);

                if i != task.input_args.len() - 1 {
                    input = format!("{input},");
                } else {
                    input =
                        format!("{input}],\n\t[Debug, Clone, Default, Serialize, Deserialize]);");
                }

                if depend.binary_search(&field.name).is_err() {
                    common_inputs.insert(field.name.clone(), field.input_type.clone());
                    new.push(format!("{}:{}", field.name, field.input_type));
                }
            }

            input_structs = format!(
                "{input_structs}
{input}
make_main_struct!(
    {task_name},
    {task_name}Input,
    [Debug, Clone, Default, Serialize, Deserialize, {}],
    {}
);
impl_new!(
    {task_name},
    {task_name}Input,
    [{}]
);
impl_setter!({task_name}, [{}]);
",
                self.get_kind(&task.kind).unwrap(),
                self.get_attributes(&task.attributes),
                new.join(","),
                setter.join(",")
            );

            constructors = if new.is_empty() {
                format!(
                    "{constructors}\tlet {} = {}::new(\"{}\".to_string());\n",
                    task.action_name.to_case(Case::Snake),
                    task_name,
                    task.action_name.clone()
                )
            } else {
                let commons: Vec<String> = new
                    .iter()
                    .map(|x| format!("input.{}", x.split(':').collect::<Vec<&str>>()[0]))
                    .collect();

                format!(
                    "{constructors}\tlet {} = {}::new({}, \"{}\".to_string());\n",
                    task.action_name.to_case(Case::Snake),
                    task_name,
                    commons.join(","),
                    task.action_name.clone()
                )
            };
        }

        let mut input = "\nmake_input_struct!(\n\tInput,\n\t[".to_string();

        for (i, field) in common_inputs.iter().enumerate() {
            input = format!("{input}{}:{}", field.0, field.1);

            if i != common_inputs.len() - 1 {
                input = format!("{input},");
            } else {
                input = format!("{input}],\n\t[Debug, Clone, Default, Serialize, Deserialize]);");
            }
        }

        input_structs = format!("{input_structs}\n{input}");

        [input_structs, constructors]
    }

    pub fn get_impl_execute_trait(&self, workflow_index: usize) -> String {
        let mut build_string = String::from("\nimpl_execute_trait!(");
        let len = self.workflows.borrow()[workflow_index].tasks.len();

        for (i, task) in self.workflows.borrow()[workflow_index]
            .tasks
            .iter()
            .enumerate()
        {
            build_string = format!("{build_string}{}", task.1.action_name.to_case(Case::Pascal));

            build_string = if i != len - 1 {
                format!("{build_string},")
            } else {
                format!("{build_string});\n")
            }
        }

        build_string
    }

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

            add_edges_code = format!(
                "{add_edges_code}\t\t({}_index, {}_index),\n",
                flow[i].to_case(Case::Snake),
                flow[i + 1].to_case(Case::Snake)
            );

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

    pub fn generate_main_file_code(&self, workflow_index: usize) -> String {
        let structs = self.get_custom_structs(workflow_index);
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
            self.get_custom_types(workflow_index),
            structs[0],
            self.get_impl_execute_trait(workflow_index),
            self.workflows.borrow()[workflow_index].tasks.len(),
            structs[1],
            workflow_nodes_and_edges[0],
            workflow_nodes_and_edges[1]
        );

        main_file
    }
}
