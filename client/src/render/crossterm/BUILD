load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "blitter",
    srcs = ["blitter.rs"],
    deps = [
      "@crates//:crossterm",
      "//client/src/render/texture:texture",
      ":sampler",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "input_manager",
    srcs = ["input_manager.rs"],
    deps = [
      "@crates//:crossterm",
      "@crates//:nalgebra",
      "//client/src/common:input",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "sampler",
    srcs = ["sampler.rs"],
    deps = [
      "@crates//:nalgebra",
      "//client/src/render/texture:texture", 
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "renderer",
    srcs = ["renderer.rs"],
    deps = [
      "@crates//:crossterm",
      "@crates//:nalgebra",
      "//client/src/render/texture:texture",
      "//client/src/containers:file_manager",
      "//client/src/render:sprite",
      ":sampler",
      ":blitter",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)


