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
	[field2:String,field1:String,field3:i16],
	[Default, Clone, Debug]
);
make_input_struct!(
	Struct2,
	[field1:HashMap<i8, String>,field2:Vec<String>],
	[Default, Clone, Debug]
);
            

make_input_struct!(
    SalaryInput,
    [details:HashMap<i32,(i32,String)>],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Salary,
    SalaryInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [Insecure:"true",Namespace:"guest",ApiHost:"https://65.20.70.146:31001",AuthToken:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
);
impl_new!(
    Salary,
    SalaryInput,
    []
);
impl_setter!(Salary, [details:"result",details:"result"]);

make_input_struct!(
    GetaddressInput,
    [id:i32],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Getaddress,
    GetaddressInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [Namespace:"guest",Insecure:"true",AuthToken:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",ApiHost:"https://65.20.70.146:31001"]
);
impl_new!(
    Getaddress,
    GetaddressInput,
    []
);
impl_setter!(Getaddress, [id:"id"]);

make_input_struct!(
    GetsalariesInput,
    [id:Struct2],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    Getsalaries,
    GetsalariesInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [Namespace:"guest",AuthToken:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP",Insecure:"true",ApiHost:"https://65.20.70.146:31001"]
);
impl_new!(
    Getsalaries,
    GetsalariesInput,
    []
);
impl_setter!(Getsalaries, [id:"id"]);

make_input_struct!(
    EmployeeIdsInput,
    [input_field_1:Struct1,input_field_1:i32],
	[Debug, Clone, Default, Serialize, Deserialize]);
make_main_struct!(
    EmployeeIds,
    EmployeeIdsInput,
    [Debug, Clone, Default, Serialize, Deserialize, OpenWhisk],
    [ApiHost:"https://65.20.70.146:31001",Namespace:"guest",Insecure:"true",AuthToken:"23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP"]
);
impl_new!(
    EmployeeIds,
    EmployeeIdsInput,
    [input_field_1:Struct1,input_field_1:i32]
);
impl_setter!(EmployeeIds, []);


make_input_struct!(
	Input,
	[input_field_1:i32],
	[Debug, Clone, Default, Serialize, Deserialize]);
#[allow(dead_code, unused)]
pub fn main(args: Value) -> Result<Value, String> {
    const LIMIT: usize = 4;
    let mut workflow = WorkflowGraph::new(LIMIT);
    let input: Input = serde_json::from_value(args).map_err(|e| e.to_string())?;

	let salary = Salary::new("salary".to_string());
	let salary_index = workflow.add_node(Box::new(salary));
	let getaddress = Getaddress::new("getaddress".to_string());
	let getaddress_index = workflow.add_node(Box::new(getaddress));
	let getsalaries = Getsalaries::new("getsalaries".to_string());
	let getsalaries_index = workflow.add_node(Box::new(getsalaries));
	let employee_ids = EmployeeIds::new(input.input_field_1,input.input_field_1, "employee_ids".to_string());
	let employee_ids_index = workflow.add_node(Box::new(employee_ids));

	workflow.add_edges(&[
		(employee_ids_index, getsalaries_index),
		(getsalaries_index, getaddress_index),
		(getaddress_index, salary_index),
	]);

	let result = workflow
		.init()?
		.pipe(getsalaries_index)?
		.pipe(getaddress_index)?
		.pipe(salary_index)?
		.term(None)?;

    let result = serde_json::to_value(result).unwrap();
    Ok(result)
}
