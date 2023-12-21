# Composer-v2

[![License: Apache-2.0](https://img.shields.io/github/license/icon-project/IBC-Integration.svg?style=flat-square)](https://www.apache.org/licenses/LICENSE-2.0)

## Introduction

 The Echo-Library and Echo-Cli tandem empower developers with a comprehensive solution for defining, managing, and executing workflows with ease. By harnessing Rust's capabilities, these tools provide a solid foundation for creating efficient and optimized WebAssembly files, offering developers a versatile toolkit to streamline their development processes.

## Prerequisite

- Ensure [Rust](https://www.rust-lang.org/tools/install) is installed and updated to the latest version.
  
## Getting started

- Clone the repository
  
  ```
  git clone https://github.com/HugoByte/internal-research-and-sample-code.git
  ```

- change the directory to `composer-dev`

- Installing the build-libraries
  
  ```
  brew install llvm@11  
  ```

  ```
  export CC=/opt/homebrew/Cellar/llvm@11/11.1.0_4/bin/clang-11 && export AR=/opt/homebrew/Cellar/llvm@11/11.1.0_4/bin/llvm-ar
  ```

- Installing the echo-library

  ```
  cargo install --path package
  ```

- Run
  
  ```
  composer
  ```

## Examples

- Building the current package
  
  ```
  composer build
  ```

- Validating the config file
  
  ```
  composer validate
  ```

- Creating the new Package
  
  ```
  composer create <package_name>
  ```

## Echo-CLI

Echo-cli, a command-line interface created to streamline the creation of WebAssembly (Wasm) binary files through configuration files. Its primary function is the build command. This tool provides developers with an easy-to-use and customizable solution for generating Wasm binaries, making the process more intuitive and efficient.

![Allow Push Notification](../internal-research-and-sample-code/images/cli.png)

For a better understanding of echo-cli, [refer ](../internal-research-and-sample-code/echo-cli/README.md)

## Testing 

## License

Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)