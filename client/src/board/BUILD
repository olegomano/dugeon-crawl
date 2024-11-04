load("@rules_rust//rust:defs.bzl", "rust_library")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "board",
    srcs = ["board.rs"],
    deps = [
        "//client/cargo:crossterm",
        "//client/cargo:log",
        ":entity",
        ":entity_type",
        ":cell",
        "//client/src/board/entity_types:terrain",
        "//client/src/containers:slot_map",
        "//client/src/containers:entity_store",
    ],
)

rust_library(
    name = "entity",
    srcs = ["entity.rs"],
    deps = [
      ":entity_type",
      "//client/src/containers:slot_map",
    ]
)

rust_library(
    name = "entity_type",
    srcs = ["entity_type.rs"],
)

rust_library(
    name = "cell",
    srcs = ["cell.rs"],
    deps = [
      ":entity",
      ":entity_type"
    ]
)
