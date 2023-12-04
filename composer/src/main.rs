use allocative::Allocative;
use anyhow::Error;
use convert_case::{Case, Casing};
use serde_derive::{Deserialize, Serialize};
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::{ProvidesStaticType, StarlarkValue, Value};
use starlark::{starlark_module, starlark_simple_value, values::starlark_value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::ErrorKind;
use std::process::Command;
use std::result::Result::Ok;
use std::{env, fs};
use std::{io, path::Path};

pub mod composer;
pub mod input;
pub mod parse_module;
pub mod starlark_modules;
pub mod task;
pub mod tests;
pub mod workflow;

use composer::*;
use input::*;
use starlark_modules::*;
use task::*;
use workflow::*;

fn main() {
    let mut composer = Composer::default();

    composer.add_config("./config/map_concat_test(emp_salary).star");

    composer.run();
}
