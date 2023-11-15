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
use convert_case::{Case, Casing};

pub mod composer;
pub mod parse_module;
pub mod task;
pub mod workflow;
pub mod starlark_modules;

use composer::*;
use task::*;
use workflow::*;
use starlark_modules::*;

fn main() {

    let composer = Composer::new("./config/custom_types.star");
    composer.run();

}
