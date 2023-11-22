use serde_json::Value;
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
                            (x.to_owned(), self.output.get("$element").unwrap().to_owned())
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
}
make_input_struct!(
	Struct1,
	[field2:String,field1:String,field3:i16],
	[Default, Clone, Debug]
);
make_input_struct!(
	Struct2,
	[field2:Vec<String>,field1:HashMap<i8, String>],
	[Default, Clone, Debug]
);
            

make_input_struct!(
    StakingpayoutInput,
    [url:String,owner_key:String,address:String,era:String],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Stakingpayout,
    StakingpayoutInput,
    [Debug, Clone, Default, Serialize, Deserialize, Polkadot],
    [Chain:"westend",Operation:"stakingpayout"]
);
impl_new!(
    Stakingpayout,
    StakingpayoutInput,
    [url:String,owner_key:String,address:String,era:String]
);
impl_setter!(Stakingpayout, [])


make_input_struct!(
	Input,
	[owner_key:String,era:String,url:String,address:String],
	[Debug, Clone, Default, Serialize, Deserialize]);
#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {
    const LIMIT: usize = 4;
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

	let stakingpayout = Stakingpayout::new(input.url,input.owner_key,input.address,input.era, "stakingpayout".to_string());
	let stakingpayout_index = workflow.add_node(Box::new(stakingpayout));

	workflow.add_edges(&[
	]);

	let result = workflow
		.init()?

    let result = serde_json::to_value(result).unwrap();
    Ok(result)
}
