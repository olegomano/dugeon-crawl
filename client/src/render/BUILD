load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "sprite",
    srcs = ["sprite.rs"],
    deps = [
        "//client/src/containers:file_manager",
        "//client/cargo:nalgebra",
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
        "//client/src/render/crossterm:blitter",
        "//client/src/render/texture:texture",
        "//client/src/render/crossterm:sampler",
        "//client/src/profiler:profiler",
        "//client/src/containers:file_manager",
    ],
    rustc_flags = ["-Z", "unstable-options","-Z","macro-backtrace"],
)



