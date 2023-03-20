# libronin Rust Example

This repository contains a simple example for getting started with libronin using Rust.

## Getting Started

First, a custom build of cargo-gccrs must be used in order to compile since a custom binary (`sh-elf-gccrs`) is used
for the Dreamcast toolchain. To install the custom version, run the following command with your existing Rust
installation:

```cargo install --git https://github.com/spencerelliott/cargo-gccrs.git --branch main cargo-gccrs```

Once the tool is installed, it should be possible to build this example.

## Building

To build the example, run the following command:

```GCCRS_CUSTOM_BIN="sh-elf-gccrs" cargo gccrs build```

If the build is successful, an ELF file should be written to the `target/debug/` folder.