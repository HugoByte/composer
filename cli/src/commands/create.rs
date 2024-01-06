use clap::StructOpt;
use composer_primitives::result;
use std::fs;

use crate::errors::io_error;

#[derive(StructOpt, Debug)]
pub struct Create {
    pub package_name: String,
}

impl Create {
    pub fn execute(self) -> result::Result<()> {
        let current = std::env::current_dir().map_err(io_error)?;
        let package = current.join(&self.package_name);
        fs::create_dir(&package).map_err(io_error)?;

        let temp_path = package.join("main.echo");
        let content = format!(
            "\
hello_world = task(
    kind = \"hello_world\",
    action_name = \"hello_world\",
    input_arguments = [
        argument(
            name=\"name\",
            input_type = String,
            default_value = \"World\"
        ),
    ],
)

workflows(
    name = \"{}\",
    version = \"0.0.1\",
    tasks = [hello_world]
)",
            self.package_name
        );

        fs::write(temp_path, content.as_bytes()).map_err(io_error)?;
        println!(
            "\t\x1B[32m\x1b[1mCreated\x1b[0m: Workflow Package \x1B[34m\x1b[1m'{}' \x1b[0m",
            &self.package_name
        );
        Ok(())
    }
}
