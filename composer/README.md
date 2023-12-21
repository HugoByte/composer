# Echo-Library

## Overview

The Echo-Library is a library that enables you to define and create workflows and compile them into WebAssembly (WASM) files for execution.

## Key Features

1. **Workflow Management:**
   - Define workflows using echo Starlark language, a concise and readable domain-specific language.
   - Organize workflows into distinct tasks with clear dependencies.

2. **Code generation:**
   - Automatically generate Rust structs representing common inputs for workflow tasks.
   - Incorporate default values and custom data types as needed.

3. **WASM Compilation:**
   - Compile workflows into WASM files for efficient execution across various platforms.
   - Customize the build process with options for verbose or quiet output.
   
## Execution Flow

1. **Configuration:**
   - Load configuration 
   - Define workflows and their associated tasks
  
2. **Building Workflows:**
   - Initiate the build process with the `build` method.
   - Compile Starlark configuration files.
   - Generate WASM files for each workflow
     - This involves creating temporary directories, copying boilerplate code, writing workflow-specific code, and building the WASM file.

## Functionality

1. **Flexible command line interface:**
   - Leverages StructOpt for parsing and Execute for execution.
  
2. **Adoptable Configuration:**
   - The Context struct manages settings, parsing, and building, adaptable for different parsers.

3. **Error-handled compilation:**
   - Ensures smooth compilation of designated entry files with user-configurable build and output settings, including quiet mode.

4. **Polymorphic parser interface:**
   - Enables diverse Rust parsers through the parser trait, promoting code reuse.

5. **Key traits and types:**
   - Exception trait for exception handling, Execute trait for generic execution with Result type for error management.

## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)





