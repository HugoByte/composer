use composer_library::{SourceFiles, OutputDirectory};

use super::BuildDirectory;

pub trait Parser {
    fn parse(&self, source: &SourceFiles);
    fn build(&self, build_directory: &BuildDirectory, output_directory: &OutputDirectory, quiet: bool);
}
