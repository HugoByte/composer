use serde_json::Value;
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
}
            

make_input_struct!(
    PurchaseInput,
    [model_price_list:HashMap<String,i32>,model_name:String,price:i32],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Purchase,
    PurchaseInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Insecure:"true",Namespace:"guest",ApiHost:"https://65.20.70.146:31001"]
);
impl_new!(
    Purchase,
    PurchaseInput,
    [model_name:String,price:i32]
);
impl_setter!(Purchase, [model_price_list:"model_price_list"]);

make_input_struct!(
    ModelavailInput,
    [car_company_list:HashMap<String,Vec<String>>,company_name:String],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Modelavail,
    ModelavailInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [ApiHost:"https://65.20.70.146:31001",Insecure:"true",AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Namespace:"guest"]
);
impl_new!(
    Modelavail,
    ModelavailInput,
    [company_name:String]
);
impl_setter!(Modelavail, [car_company_list:"car_company_list"]);

make_input_struct!(
    CartypeInput,
    [car_type:String],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Cartype,
    CartypeInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Insecure:"true",Namespace:"guest",ApiHost:"https://65.20.70.146:31001"]
);
impl_new!(
    Cartype,
    CartypeInput,
    [car_type:String]
);
impl_setter!(Cartype, []);

make_input_struct!(
    ModelspriceInput,
    [models:Vec<String>],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Modelsprice,
    ModelspriceInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [AuthKey:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Insecure:"true",Namespace:"guest",ApiHost:"https://65.20.70.146:31001"]
);
impl_new!(
    Modelsprice,
    ModelspriceInput,
    []
);
impl_setter!(Modelsprice, [models:"models"]);


make_input_struct!(
	Input,
	[price:i32,model_name:String,car_type:String,company_name:String],
	[Debug, Clone, Default, Serialize, Deserialize]);

impl_execute_trait!(Purchase,Modelavail,Cartype,Modelsprice);

#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {
    const LIMIT: usize = 4;
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

	let purchase = Purchase::new(input.model_name,input.price, "purchase".to_string());
	let purchase_index = workflow.add_node(Box::new(purchase));
	let modelavail = Modelavail::new(input.company_name, "modelavail".to_string());
	let modelavail_index = workflow.add_node(Box::new(modelavail));
	let cartype = Cartype::new(input.car_type, "cartype".to_string());
	let cartype_index = workflow.add_node(Box::new(cartype));
	let modelsprice = Modelsprice::new("modelsprice".to_string());
	let modelsprice_index = workflow.add_node(Box::new(modelsprice));

	workflow.add_edges(&[
		(cartype_index, modelavail_index),
		(modelavail_index, modelsprice_index),
		(modelsprice_index, purchase_index),
	]);

	let result = workflow
		.init()?
		.pipe(modelavail_index)?
		.pipe(modelsprice_index)?
		.term(Some(purchase_index))?;
    let result = serde_json::to_value(result).unwrap();
    Ok(result)
}
