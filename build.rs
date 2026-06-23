fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut builder = cc::Build::new();

    builder
        .include("bitshuffle/lz4")
        .include("bitshuffle")
        .file("bitshuffle/src/bitshuffle.c")
        .file("bitshuffle/src/bitshuffle_core.c")
        .file("bitshuffle/src/iochain.c")
        .file("bitshuffle/lz4/lz4.c")
        .define("BSHUF_VERSION_MAJOR", "0")
        .define("BSHUF_VERSION_MINOR", "5")
        .define("BSHUF_VERSION_POINT", "2")
        .define("NO_WARN_X86_INTRINSICS", None)
        .define("__SSE2__", None)
        .flag_if_supported("/arch:AVX2")
        .flag_if_supported("-O3")
        .flag_if_supported("-march=native")
        .flag_if_supported("-ffast-math")
        .flag_if_supported("-std=c99")
        .flag_if_supported("/std:c11")
        .flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-fPIC")
        .flag_if_supported("/Ox")
        .flag_if_supported("/fp:fast")
        .flag_if_supported("-w");

    builder.compile("bitshuffle");
}
