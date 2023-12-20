use composer_library::{Composer, SourceFiles, OutputDirectory};

use super::Parser;

pub static FILE_EXTENSION: &str = "star";
pub static ENTRY_FILE: &str = "main";

impl Parser for Composer {
    fn parse(&self, files: &SourceFiles) {
        let _main = self
            .compile(
                &format!(
                    "{}.{}",
                    ENTRY_FILE,
                    FILE_EXTENSION
                ),
                files,
            )
            .unwrap();
    }

    fn build(&self, build_directory: &super::BuildDirectory, output_directory: &OutputDirectory, quiet : bool) {
        self.build_directory(&build_directory.path, &output_directory.base(), quiet);
    }
}
