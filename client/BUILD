load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    deps = [
        "//client/cargo:crossterm",
        "//client/cargo:log",
        "//client/src/containers:entity_store",
        "//client/src/board:board",
        "//client/src/board:cell",
        "//client/src/board/entity_types:player",
        "//client/src/board:entity",
        "//client/src/board:entity_type",
        "//client/src/render:render",
        "//client/src/render:terminal_renderer",
    ],
)
