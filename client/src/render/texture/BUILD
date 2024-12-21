load("@rules_rust//rust:defs.bzl", "rust_library")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "texture",
    srcs = ["texture.rs"],
    deps = [
      "//client/cargo:image",
      "//client/cargo:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "sampler",
    srcs = ["sampler.rs"],
    deps = [
      ":texture", 
      "//client/cargo:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)
