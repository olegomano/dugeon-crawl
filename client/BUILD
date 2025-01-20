load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    deps = [
        "//client/src/common:context",
        "//client/src/render/crossterm:renderer",
        "//client/src/render/crossterm:input_manager",
    ],
    rustc_flags = ["-Z", "unstable-options"],
    env = {
        "CARGO_PKG_AUTHORS": "Your Name <your.email@example.com>",
    },
)
