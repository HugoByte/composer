use serde_derive::{Deserialize, Serialize};
use starlark::environment::Module;
// use starlark::starlark_module;
use allocative::Allocative;
use starlark::any::ProvidesStaticType;
use starlark::environment::GlobalsBuilder;
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::none::NoneType;
use starlark::values::{NoSerialize, StarlarkValue, Value};
use starlark::{starlark_module, starlark_simple_value};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::ErrorKind;

pub mod composer;
pub mod task;
pub mod workflow;
pub mod test_module;

use composer::*;
use task::*;
use workflow::*;
use test_module::*;

fn main() {
    let content: String = std::fs::read_to_string("./config/multiple.star").unwrap();

    let ast = AstModule::parse("name", content.to_owned(), &Dialect::Extended).unwrap();
    // We build our globals adding some functions we wrote
    let globals = GlobalsBuilder::new().with(starlark_workflow).build();
    let module = Module::new();
    let store = Composer::default();
    {
        let mut eval = Evaluator::new(&module);
        // We add a reference to our store
        eval.extra = Some(&store);

        eval.eval_module(ast, &globals).unwrap();
    }

    // store.generate();

    println!("{:#?}", store);
}
