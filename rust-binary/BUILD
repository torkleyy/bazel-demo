#load("@rules_rust//rust:defs.bzl", "rust_binary")

# rust_binary(
#     name = "rust-binary",
#     srcs = ["src/main.rs"],
#     deps = ["//rust-binary/cargo:rand"]
# )

filegroup(
    name = "cargo-project-files",
    srcs = glob([
        "src/**/*.rs"
    ]) + [
        "Cargo.toml",
        "Cargo.lock",
    ]
)
