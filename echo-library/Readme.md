# Echo-Library

## Overview

The Echo-Library plays a pivotal role for facilitating the compilation of configuration files into WebAssembly (WASM) files. It not only handles the technical intricacies of this process but also establishes a streamlined workflow for the execution of the compiled files. This library is designed to empower developers by providing a cohesive and efficient solution for translating configuration settings into executable WebAssembly code, ensuring a smooth and well-organized development experience.

## Key Features

1. **Workflow Management:**
   - Define workflows using echo language, a concise and readable domain-specific language.
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


## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)