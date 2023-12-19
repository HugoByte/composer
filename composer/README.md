# Composer-V2

## Overview

The Composer is a Rust-based tool that enables you to define and manage workflows, compile them into WebAssembly (WASM) files for execution, and streamline input handling for workflow tasks.

## Key Features

1. **Workflow Management:**
   - Define workflows using echo Starlark language, a concise and readable domain-specific language.
   - Organize workflows into distinct tasks with clear dependencies.

2. **WASM Compilation:**
   - Compile workflows into WASM files for efficient execution across various platforms.
   - Customize the build process with options for verbose or quiet output.

3. **Input Handling Optimisation:**
   - Automatically generate Rust structs representing common inputs for workflow tasks.
   - Incorporate default values and custom data types as needed.

## Execution Flow

1. **Configuration:**
   - Load configuration 
   - Define workflows and their associated tasks
  
2. **Building Workflows:**
   - Initiate the build process with the `build` method.
   - Compile Starlark configuration files.
   - Generate WASM files for each workflow
     - This involves creating temporary directories, copying boilerplate code, writing workflow-specific code, and building the WASM file.

## Getting Started

1. Install [Rust](https://www.rust-lang.org/tools/install) and necessary dependencies.
2. Clone the composer repository
   
   ```
   git clone https://github.com/HugoByte/internal-research-and-sample-code.git
   ```
   *change the branch to composer-dev*

3. Define your workflows using Starlark [configuration files](../config).

4. Build the workflows using the [echo-cli](../echo-cli/README.md) command.

## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)





