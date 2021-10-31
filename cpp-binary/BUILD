
cc_binary(
	name = "hello-bazel",
	srcs = glob(["*.cc"]),
	deps = ["//cpp-library:utils"]
)

alias(
	name = "cpp-binary",
	actual = ":hello-bazel"
)
