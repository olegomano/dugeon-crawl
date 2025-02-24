load("@rules_rust//rust:defs.bzl", "rust_library")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "texture",
    srcs = ["texture.rs"],
    deps = [
      "@crates//:image",
      "@crates//:nalgebra",
      "//client/src/containers:file_manager"
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "sprite_sheet",
    srcs = ["sprite_sheet.rs"],
    deps = [
      "@crates//:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)
