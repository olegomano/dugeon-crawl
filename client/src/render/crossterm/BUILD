load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "blitter",
    srcs = ["blitter.rs"],
    deps = [
      "//client/cargo:crossterm",
      "//client/src/render/texture:texture",
      ":sampler",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "input_manager",
    srcs = ["input_manager.rs"],
    deps = [
      "//client/cargo:crossterm",
      "//client/src/common:input",
      "//client/cargo:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "sampler",
    srcs = ["sampler.rs"],
    deps = [
      "//client/src/render/texture:texture", 
      "//client/cargo:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "renderer",
    srcs = ["renderer.rs"],
    deps = [
      "//client/cargo:crossterm",
      "//client/src/render/texture:texture",
      "//client/src/containers:file_manager",
      "//client/cargo:nalgebra",
      "//client/src/render:sprite",
      ":sampler",
      ":blitter",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)


