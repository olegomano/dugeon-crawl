

load("@rules_rust//rust:defs.bzl", "rust_library")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "render",
    srcs = ["render.rs"],
    deps = [
      "//client/src/board:entity",
      "//client/src/board:board",
    ]
)

rust_library(
    name = "terminal_renderer",
    srcs = ["terminal_renderer.rs"],
    deps = [
      ":render",
      "//client/src/board:entity",
      "//client/src/board:entity_type",
      "//client/src/board:board",
      "//client/src/board:cell",
      "//client/cargo:crossterm",
      "//client/src/board/entity_types:terrain",
      "//client/src/board/entity_types:player",
    ]
)
