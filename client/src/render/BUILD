load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "blit_renderer",
    srcs = ["blit_renderer.rs"],
    deps = [
      "//client/cargo:crossterm",
      "//client/src/render/texture:texture",
      "//client/src/render/texture:sampler",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_binary(
    name = "term_png",
    srcs = ["executables/term_png.rs"],
    deps = [
        "//client/cargo:crossterm",
        "//client/cargo:image",
        "//client/cargo:nalgebra",
        "//client/src/render:blit_renderer",
        "//client/src/render/texture:texture",
        "//client/src/render/texture:sampler",
        "//client/src/profiler:profiler",
    ],
    rustc_flags = ["-Z", "unstable-options","-Z","macro-backtrace"],
)



