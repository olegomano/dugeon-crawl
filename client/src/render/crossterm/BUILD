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
    name = "sampler",
    srcs = ["sampler.rs"],
    deps = [
      "@crates//:nalgebra",
      "//client/src/render/texture:texture", 
      "//client/src/common:transform",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "crossterm_platform_impl",
    srcs = ["crossterm_platform_impl.rs"],
    deps = [
      "@crates//:crossterm",
      "@crates//:nalgebra",
      "//client/src/render/texture:texture",
      "//client/src/common:game_object",
      "//client/src/common:input",
      "//client/src/containers:file_manager",
      "//client/src/render:sprite",
      ":sampler",
      ":blitter",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)


