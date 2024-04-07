load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_image_index")

def build_image(name, srcs, exposed_ports = [], visibility=None):

    # Build a Bazel Macro
    # https://belov.nz/posts/bazel-rules-macros/
    # https://codilime.com/blog/bazel-build-system-build-containerized-applications/
    entry_point = "bin"
    layer_name = "tar_layer"

    # Compress binary to layer using pkg_tar
    pkg_tar(
        name = layer_name,
        srcs = srcs,
    )

    # Build container image
    # https://github.com/bazel-contrib/rules_oci/blob/main/docs/image.md
    oci_image(
        name = name,
        base = "@distroless_cc",
        tars = [layer_name],
        entrypoint = ["/{}".format(entry_point)],
        exposed_ports = exposed_ports,
        visibility = visibility,
    )
