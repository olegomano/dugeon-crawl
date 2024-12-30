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
    name = "sampler",
    srcs = ["sampler.rs"],
    deps = [
      "//client/src/render/texture:texture", 
      "//client/cargo:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)
