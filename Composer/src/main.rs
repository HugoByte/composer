use anyhow::Error;
use serde_derive::Deserialize;
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{none::NoneType, ProvidesStaticType, StarlarkValue, Value};
use starlark::{starlark_module, starlark_simple_value, values::starlark_value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::ErrorKind;
use std::result::Result::Ok;
use std::{env, fs, path::PathBuf, process::Command};

pub mod composer;
pub mod parse_module;
pub mod task;
pub mod workflow;

use composer::*;
use task::*;
use workflow::*;

fn main() {
    let content: String = std::fs::read_to_string("./config/custom_types.star").unwrap();

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
}
