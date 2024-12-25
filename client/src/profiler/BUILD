load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "profiler",
    srcs = ["profiler.rs"],
    deps = [
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

