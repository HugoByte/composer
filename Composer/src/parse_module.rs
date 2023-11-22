use std::fmt::{format, Debug};

use starlark::values::AllocValue;

use super::*;

impl Composer {
    pub fn get_macros(&self) -> String {
        "use serde_json::Value;
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

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
            pub mapout: Value,
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
 
}

macro_rules! impl_map_setter {
    (
        $name:ty,
        $element:ident,
        $key:expr ,  
        $typ_name : ty  
    ) => {
        impl $name
            pub fn setter(&mut self, val: Value) {
                
                    let value = val.get($key).unwrap();
                    let value = serde_json::from_value::<Vec<$typ_name>>(value.clone()).unwrap();
                    let mut map: HashMap<_, _> = value
                        .iter()
                        .map(|x| {
                            self.input.$element = x.to_owned() as $typ_name;
                            self.run();
                            (x.to_owned(), self.output.get(\"$element\").unwrap().to_owned())
                        })
                        .collect();
                    self.mapout = to_value(map).unwrap();
                
            }
        }
    }

macro_rules! impl_concat_setter {
    (
        $name:ty,
        $element:ident,
    ) => {
        impl $name{
            pub fn setter(&mut self, val: Value) {
                $(
                    let val: Vec<Value> = serde_json::from_value(val).unwrap();
                    let res = join_hashmap(
                        serde_json::from_value(val[0].to_owned()).unwrap(),
                        serde_json::from_value(val[1].to_owned()).unwrap(),
                    );
                    self.input.$element = res;

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

            attributes = if i != map.len() - 1 {
                format!("{attributes},")
            } else {
                format!("{attributes}]")
            }
        }

        attributes
    }


    pub fn parse_hashmap(&self, map: &HashMap<String, String>) -> String {
        let mut attributes = "[".to_string();

        for (i, (k, v)) in map.iter().enumerate() {
            attributes = format!("{attributes}{}:{}", k, v);

            attributes = if i != map.len() - 1 {
                format!("{attributes},")
            } else {
                format!("{attributes}]")
            }
        }

        attributes
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

        for t in self.workflows.borrow()[workflow_index].custom_types.iter() {
            let typ = custom_types.get(t).unwrap();
            build_string = format!("{build_string}{typ}\n");
        }

        build_string
    }

    pub fn get_custom_structs(&self, workflow_index: usize) -> Vec<String> {
        let mut common_inputs = HashMap::<String, String>::new();

        let mut constructors = String::new();
        let mut input_structs = String::new();

        for (task_name, task) in self.workflows.borrow()[workflow_index].tasks.iter() {
            let task_name = task_name.to_case(Case::Pascal);

            let mut depend = Vec::<String>::new();
            let mut setter = Vec::<String>::new();
            let mut map_setter = String::new();

            for fields in task.depend_on.values() {
                let x = fields.iter().next().unwrap();
                depend.push(x.0.to_string());

                let mut input ="".to_string();
                for i in task.input_args.iter(){
                    if i.name.as_str() == x.0{
                        input = i.input_type.to_string(); 
                    }
                }

                setter.push(format!("{},\"{}\", {}", x.0, x.1, input ));

            }

            let field = match &task.operation{
                Operation::Map(_) => "map",
                _ => "",
            };

            // for gg in task.input_args.iter(){
            //     if gg.name.as_str().is_empty(){
            //         gg
            //     }
                   
            //     }
            //     setter.push(format!({},))
            // }
            
            // setter.push(asd.clone());

            // let concat_field : &str = match &task.kind{
            //     Operation::Concat => "concat",
            //     _ => "",
            // };
           
            map_setter.push_str(&field);

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

        let setter_macro = match &task.operation{
            Operation::Map(field) => 
                format!("impl_map_setter!({}, [{}], {})", task_name, setter.join(","), field),
            Operation::Concat => 
                format!("impl_concat_setter!({}, {})", task_name, task.input_args[0].name),
            _ =>  format!("impl_setter!({}, [{}])", task_name, setter.join(","))
        };

        
       
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
{setter_macro}
",
    

                self.get_kind(&task.kind).unwrap(),
                self.get_attributes(&task.attributes),
                new.join(","),
            
                
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

            constructors = format!(
                "{constructors}\tlet {}_index = workflow.add_node(Box::new({}));\n",
                task.action_name.to_case(Case::Snake),
                task.action_name.to_case(Case::Snake)
            );
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
        vec![input_structs, constructors]
    }

    pub fn get_workflow_execute_code(&self, workflow_index: usize) -> String {
        let mut execute_code = "\tlet result = workflow\n\t\t.init()?\n".to_string();

        let mut add_edges_code = "\tworkflow.add_edges(&[\n".to_string();
        let flow: Vec<String> = self.get_flow(workflow_index);

        for i in 0..flow.len() - 1 {
            add_edges_code = format!(
                "{add_edges_code}\t\t({}_index, {}_index),\n",
                flow[i].to_lowercase(),
                flow[i + 1].to_lowercase()
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
                            "{execute_code}\t\t.term(Some({}_index))?;\n",
                            flow[i + 1].to_lowercase()
                        )
                    }

                    _ => {
                        format!(
                            "{execute_code}\t\t.pipe({}_index)?\n\t\t.term(None)?;\n",
                            flow[i + 1].to_lowercase()
                        )
                    }
                }
            } else {
                format!(
                    "{execute_code}\t\t.pipe({}_index)?\n",
                    flow[i + 1].to_lowercase()
                )
            };
        }

        add_edges_code = format!("{add_edges_code}\t]);\n\n{execute_code}");

        add_edges_code
    }

    pub fn generate_main_file_code(&self, workflow_index: usize) -> String {
        let structs = self.get_custom_structs(workflow_index);

        let main_file = format!(
            "{}
{}            
{}
#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {{
    const LIMIT: usize = {};
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

{}
{}
    let result = serde_json::to_value(result).unwrap();
    Ok(result)
}}
",
            self.get_macros(),
            self.get_custom_types(0),
            structs[0],
            self.workflows.borrow()[0].tasks.len(),
            structs[1],
            self.get_workflow_execute_code(workflow_index)
        );

        main_file
    }
}
