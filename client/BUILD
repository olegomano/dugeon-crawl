load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    deps = [
        "//client/src/common:context",
        "//client/src/scene:test_scene",
        "//client/src/render/crossterm:crossterm_platform_impl",
    ],
    rustc_flags = ["-Z", "unstable-options"],
    env = {
        "CARGO_PKG_AUTHORS": "Your Name <your.email@example.com>",
    },
)
