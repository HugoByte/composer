use composer_library::{Composer, OutputDirectory, SourceFiles};

use crate::errors::IOError;

use super::{Parser, result};

pub static FILE_EXTENSION: &str = "star";
pub static ENTRY_FILE: &str = "main";

impl Parser for Composer {
    fn parse(&self, files: &SourceFiles) -> result::Result<()> {
        match self.compile(&format!("{}.{}", ENTRY_FILE, FILE_EXTENSION), files){
            Ok(_) =>  Ok(()),
            Err(err) => Err(Box::new(IOError::Anyhow(err))),
        }
    }

    fn build(
        &self,
        build_directory: &super::BuildDirectory,
        output_directory: &OutputDirectory,
        quiet: bool,
    ) {
        self.build_directory(&build_directory.path, &output_directory.base(), quiet);
    }
}
