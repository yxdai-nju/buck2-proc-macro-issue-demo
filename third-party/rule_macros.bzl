load("@prelude//rust:cargo_package.bzl", "cargo")

_EXTRA_RUSTC_FLAGS = [
    "-C",
    "opt-level=3",
]

_RUSTC_FLAGS = select({
    # Intentionally disabling the other two branches, so triggering an error if stage0 is not chosen
    "bootstrap//constraints:stage0": _EXTRA_RUSTC_FLAGS,
    # "bootstrap//constraints:stage1": _EXTRA_RUSTC_FLAGS,
    # "bootstrap//constraints:stage2": _EXTRA_RUSTC_FLAGS,
})

def demo_rust_binary(name, **kwargs):
    rustc_flags = kwargs.pop("rustc_flags", [])
    rustc_flags = rustc_flags + _RUSTC_FLAGS
    cargo.rust_binary(name = name, rustc_flags = rustc_flags, **kwargs)

def demo_rust_library(name, **kwargs):
    rustc_flags = kwargs.pop("rustc_flags", [])
    rustc_flags = rustc_flags + _RUSTC_FLAGS
    cargo.rust_library(name = name, rustc_flags = rustc_flags, **kwargs)
