#![allow(unused_imports)]
use super::*;
use alloc::task;
use paste::paste;
use workflow_macro::Flow;

#[derive(Debug, Flow)]
pub struct WorkflowGraph {
    edges: Vec<(usize, usize)>,
    nodes: Vec<Box<dyn Execute>>,
    pub state_manager: StateManager,
}

impl WorkflowGraph {
    pub fn new(size: usize) -> Self {
        WorkflowGraph {
            nodes: Vec::with_capacity(size),
            edges: Vec::new(),
            state_manager: StateManager::init(),
        }
    }
}

#[macro_export]
macro_rules! impl_execute_trait {
    ($ ($struct : ty), *) => {

    paste!{$(
    impl Execute for $struct {
            fn execute(&mut self) -> Result<(),String>{
                self.run()
            }

            fn get_task_output(&self) -> Value {
                self.output().clone().into()
            }

            fn set_output_to_task(&mut self, input: Value) {
                self.setter(input)
            }

            fn get_action_name(&self) -> String{
                self.action_name.clone()
            }

            fn get_json_string(&self) -> String{
                serde_json::to_string(&self).unwrap()
            }

            fn set_result_output(&mut self, inp: Value) {
                self.set_result_output(inp)
            }

        }
    )*}
    };
}

#[allow(dead_code, unused)]
pub fn join_hashmap<T: PartialEq + std::hash::Hash + Eq + Clone, U: Clone, V: Clone>(
    first: HashMap<T, U>,
    second: HashMap<T, V>,
) -> HashMap<T, (U, V)> {
    let mut data: HashMap<T, (U, V)> = HashMap::new();
    for (key, value) in first {
        for (s_key, s_value) in &second {
            if key.clone() == *s_key {
                data.insert(key.clone(), (value.clone(), s_value.clone()));
            }
        }
    }
    data
}

#[no_mangle]
pub unsafe extern "C" fn free_memory(ptr: *mut u8, size: u32, alignment: u32) {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::dealloc(ptr, layout);
}

#[link(wasm_import_module = "host")]
extern "C" {
    pub fn set_output(ptr: i32, size: i32);
}

#[link(wasm_import_module = "host")]
extern "C" {
    pub fn set_state(ptr: i32, size: i32);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}

#[no_mangle]
pub unsafe extern "C" fn memory_alloc(size: u32, alignment: u32) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::alloc(layout)
}
