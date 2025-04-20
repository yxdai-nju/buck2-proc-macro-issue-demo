load("@prelude//rust:cargo_package.bzl", "cargo")

configured_alias(
    name = "thiserror_demo-stage0",
    actual = ":thiserror_demo",
    platform = "bootstrap//platforms:rust_bootstrap_stage0",
    visibility = ["PUBLIC"],
)

filegroup(
    name = "thiserror-demo-0.0.0.crate",
    srcs = [
        "src/bin/thiserror_demo.rs",
    ],
)

cargo.rust_binary(
    name = "thiserror_demo",
    srcs = [":thiserror-demo-0.0.0.crate"],
    crate_root = "thiserror-demo-0.0.0.crate/src/bin/thiserror_demo.rs",
    deps = [
        "//third-party:thiserror",
    ],
    edition = "2024",
)
