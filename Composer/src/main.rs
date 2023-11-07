use serde_derive::{Deserialize};
use starlark::environment::Module;
use starlark::any::ProvidesStaticType;
use starlark::environment::GlobalsBuilder;
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::none::NoneType;
use starlark::values::Value;
use starlark::starlark_module;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::ErrorKind;

pub mod composer;
pub mod task;
pub mod workflow;
pub mod parse_module;

use composer::*;
use task::*;
use workflow::*;

fn main() {
    let content: String = std::fs::read_to_string("./config/multiple.star").unwrap();

    let ast = AstModule::parse("name", content.to_owned(), &Dialect::Extended).unwrap();
    // We build our globals adding some functions we wrote
    let globals = GlobalsBuilder::new().with(starlark_workflow).build();
    let module = Module::new();
    let composer = Composer::default();
    {
        let mut eval = Evaluator::new(&module);
        // We add a reference to our store
        eval.extra = Some(&composer);

        eval.eval_module(ast, &globals).unwrap();
    }

    composer.generate();
    // print!("{:?}", composer);
    // composer.generate_main_file_code();
}
