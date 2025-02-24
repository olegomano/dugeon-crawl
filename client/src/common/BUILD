load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
package(default_visibility = ["//visibility:public"])


rust_library(
    name = "input",
    srcs = ["input.rs"],
    deps = [
        "@crates//:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "transform",
    srcs = ["transform.rs"],
    deps = [
        "@crates//:nalgebra",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "game_object",
    srcs = ["game_object.rs"],
    deps = [
        "@crates//:nalgebra",
        "//client/src/containers:file_manager",
        "//client/src/containers:slot_map",
        "//client/src/render/texture:texture",
        "//client/src/render:sprite",
        ":input",
        ":transform",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)


rust_library(
    name = "context",
    srcs = ["context.rs"],
    deps = [
        "@crates//:nalgebra",
        "//client/src/containers:file_manager",
        "//client/src/containers:slot_map",
        "//client/src/render/texture:texture",
        "//client/src/render:sprite",
        ":input",
        ":game_object",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)


rust_library(
    name = "char_controller",
    srcs = ["char_controller.rs"],
    deps = [
        "@crates//:nalgebra",
        "//client/src/containers:file_manager",
        "//client/src/containers:slot_map",
        "//client/src/render:sprite",
        "//client/src/render/texture:sprite_sheet",
        "//client/src/render/texture:texture",
        ":input",
        ":game_object",
        ":transform",
        ":context",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)

