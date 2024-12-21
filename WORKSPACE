load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "50ec4b84a7ec5370f5882d52f4a1e6b8a75de2f8dcc0a4403747b69b2c4ef5b1",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.23.0/rules_rust-v0.23.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories","rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()
rust_register_toolchains(
  edition = "2024",
  versions = [
    "nightly/2024-11-22"
  ],
  extra_exec_rustc_flags = [
    "-Zunstable-options"
  ],
)
