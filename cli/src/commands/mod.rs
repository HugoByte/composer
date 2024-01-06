mod build;
mod create;
mod validate;

use self::{create::Create, validate::Validate};
use crate::errors::io_error;
use crate::types::Context;
use build::Build;
use clap::StructOpt;
use composer_primitives::result;
use composer_primitives::{Execute, Result};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(StructOpt, Debug)]
pub enum Commands {
    #[structopt(about = "Build the current package as a workflow")]
    Build {
        #[structopt(flatten)]
        command: Build,
    },

    #[structopt(about = "Create a new package for echo")]
    Create {
        #[structopt(flatten)]
        command: Create,
    },

    #[structopt(about = "Validate the configuration file")]
    Validate {
        #[structopt(flatten)]
        command: Validate,
    },
}
