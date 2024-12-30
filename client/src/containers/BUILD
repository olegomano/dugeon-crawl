
load("@rules_rust//rust:defs.bzl", "rust_library")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "slot_map",
    srcs = ["slot_map.rs"],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "file_manager",
    srcs = ["file_manager.rs"],
    rustc_flags = ["-Z", "unstable-options"],
)

rust_library(
    name = "entity_store",
    srcs = ["entity_store.rs"],
    deps = [
      ":slot_map",
      "//client/src/board:entity",
      "//client/src/board:entity_type",
      "//client/src/board/entity_types:terrain",
      "//client/src/board/entity_types:player",
    ],
    rustc_flags = ["-Z", "unstable-options"],
)
