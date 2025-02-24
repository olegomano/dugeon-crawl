load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "test_scene",
    srcs = ["test_scene.rs"],
    deps = [
      "@crates//:nalgebra",
      "//client/src/common:game_object",
      "//client/src/common:input",
      "//client/src/common:char_controller",
      "//client/src/containers:file_manager",
      "//client/src/containers:slot_map",
      "//client/src/render/texture:texture",
      "//client/src/render:sprite",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)
