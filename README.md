# buck2-proc-macro-issue-demo

This is a demonstration repo related to the issue facebook/buck2#900

Created for verifying whether things with `rust_proc_macro_alias` still work while tinkering with prelude code

## Structure

- `@bootstrap//constraints/...` defines constraint values for bootstrap stages (stage0, stage1, stage2)
- `@bootstrap//platforms/...` defines platforms and execution platforms for each stage
- `@toolchains//...` defines a Rust toolchain with "exec_compatible_with" settings
- `src/bin/thiserror_demo.rs` is a single-file binary Rust demo project which depends on a crate with prodecural macros
- `third-party` uses Reindeer to manage crates.io dependencies
- `third-party/rule_macros.bzl` defines macros for the Reindeer-generated BUCK file, restricting libraries to build for stage0 platform
- `BUCK` Defines a binary target configured for the stage0 platform

## Usage

To verify:

```bash
buck2 build //:thiserror_demo-stage0
```
