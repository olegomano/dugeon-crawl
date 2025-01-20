load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
package(default_visibility = ["//visibility:public"])


rust_library(
    name = "input",
    srcs = ["input.rs"],
    deps = [
        "@crates//:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)


rust_library(
    name = "context",
    srcs = ["context.rs"],
    deps = [
        "@crates//:nalgebra",
        "//client/src/containers:file_manager",
        "//client/src/containers:slot_map",
        "//client/src/render/texture:texture",
        "//client/src/render:sprite",
        ":input"
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

