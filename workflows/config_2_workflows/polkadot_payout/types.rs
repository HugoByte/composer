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
make_input_struct!(
	Struct1,
	[field3:i16,field1:String,field2:String],
	[Default, Clone, Debug]
);
make_input_struct!(
	Struct2,
	[field1:HashMap<i8, String>,field2:Vec<String>],
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
impl_setter!(Stakingpayout, []);


make_input_struct!(
	Input,
	[url:String,address:String,era:String,owner_key:String],
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
