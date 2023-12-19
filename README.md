# internal-r-d

[![License: Apache-2.0](https://img.shields.io/github/license/icon-project/IBC-Integration.svg?style=flat-square)](https://www.apache.org/licenses/LICENSE-2.0)

## Introduction

 The Composer and Echo-cli tandem empower developers with a comprehensive solution for defining, managing, and executing workflows with ease. By harnessing Rust's capabilities, these tools provide a solid foundation for creating efficient and optimized WebAssembly files, offering developers a versatile toolkit to streamline their development processes.

## Prerequisite

- Ensure [Rust](https://www.rust-lang.org/tools/install) is installed.
  
## Composer Feature

1. **Workflow Management:**
   - Define workflows using echo Starlark language, a concise and readable domain-specific language.
   - Organize workflows into distinct tasks with clear dependencies.

2. **WASM Compilation:**
   - Compile workflows into WASM files for efficient execution across various platforms.
   - Customize the build process with options for verbose or quiet output.

3. **Input Handling Optimisation:**
   - Automatically generate Rust structs representing common inputs for workflow tasks.
   - Incorporate default values and custom data types as needed.

## Echo-CLI

Echo-cli, a command-line interface created to streamline the creation of WebAssembly (Wasm) binary files through configuration files. Its primary function is the build command. This tool provides developers with an easy-to-use and customizable solution for generating Wasm binaries, making the process more intuitive and efficient.

![Allow Push Notification](../internal-research-and-sample-code/images/cli.png)

For a better understanding of echo-cli, [refer ](../internal-research-and-sample-code/echo-cli/README.md)

## Testing 

## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)