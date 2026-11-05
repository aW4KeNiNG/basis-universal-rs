// args from the basis cmake file
fn build_with_common_settings() -> cc::Build {
    let mut build = cc::Build::new();
    // The cmake file only gives these to GCC and Clang. MSVC accepts -Wall, but there it enables
    // every warning, which floods the build output.
    if build.get_compiler().is_like_msvc() {
        return build;
    }
    build
        .flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-Wno-unused-local-typedefs")
        .flag_if_supported("-Wno-unused-value")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable")
        .flag_if_supported("-Wno-misleading-indentation");

    build
}

// The encoder library of the basis cmake file (ENCODER_LIB_SRC_LIST). basisu_comp.cpp references
// every codec, so all of them are needed even if only ETC1S and UASTC LDR 4x4 are exposed.
const ENCODER_FILES: &[&str] = &[
    "encoder/basisu_backend.cpp",
    "encoder/basisu_basis_file.cpp",
    "encoder/basisu_comp.cpp",
    "encoder/basisu_enc.cpp",
    "encoder/basisu_etc.cpp",
    "encoder/basisu_frontend.cpp",
    "encoder/basisu_gpu_texture.cpp",
    "encoder/basisu_pvrtc1_4.cpp",
    "encoder/basisu_resampler.cpp",
    "encoder/basisu_resample_filters.cpp",
    "encoder/basisu_ssim.cpp",
    "encoder/basisu_uastc_enc.cpp",
    "encoder/basisu_bc7e_scalar.cpp",
    "encoder/basisu_dds_export.cpp",
    "encoder/basisu_bc7enc.cpp",
    "encoder/jpgd.cpp",
    "encoder/basisu_kernels_sse.cpp",
    "encoder/basisu_bc15_spmd.cpp",
    "encoder/basisu_bc15_spmd_sse.cpp",
    "encoder/basisu_opencl.cpp",
    "encoder/pvpngreader.cpp",
    "encoder/basisu_uastc_hdr_4x4_enc.cpp",
    "encoder/basisu_astc_hdr_6x6_enc.cpp",
    "encoder/basisu_astc_hdr_common.cpp",
    "encoder/basisu_astc_ldr_common.cpp",
    "encoder/basisu_astc_ldr_encode.cpp",
    "encoder/basisu_astc_ldr_fencode.cpp",
    "encoder/basisu_xbc7_encode.cpp",
    "encoder/basisu_tinyexr.cpp",
    "encoder/3rdparty/android_astc_decomp.cpp",
    "transcoder/basisu_transcoder.cpp",
];

fn main() {
    // Only allow the SSE flag if we compile for x86/x86_64
    // See: https://github.com/aclysma/basis-universal-rs/pull/14#discussion_r1200978360
    let build_with_sse4_1 = cfg!(feature = "hint_sse4_1")
        && std::env::var("CARGO_CFG_TARGET_ARCH")
            .expect("Cargo didn't set the CARGO_CFG_TARGET_ARCH env var")
            .split(',')
            .any(|arch| arch == "x86_64" || arch == "x86");

    let mut build = build_with_common_settings();
    build
        .cpp(true)
        .std("c++17")
        .define("BASISD_SUPPORT_KTX2_ZSTD", "1")
        .define("BASISU_SUPPORT_OPENCL", "0")
        .define("BASISU_SUPPORT_ASTCENC", "0")
        .define("BASISU_DISABLE_ANDROID_ASTC_DECOMP", "0")
        .define(
            "BASISU_SUPPORT_SSE",
            if build_with_sse4_1 { "1" } else { "0" },
        );
    // Like a cmake Release build: without NDEBUG the asserts of basisu stay on, and an assert
    // aborts the process on parameters that the library would otherwise reject with an error.
    if std::env::var("PROFILE").as_deref() == Ok("release") {
        build.define("NDEBUG", None);
    }
    // MSVC compiles SSE 4.1 intrinsics without a flag, GCC and Clang need it
    if build_with_sse4_1 && !build.get_compiler().is_like_msvc() {
        build.flag("-msse4.1");
    }
    for file in ENCODER_FILES {
        build.file(format!("vendor/basis_universal/{file}"));
    }
    build
        .file("vendor/transcoding_wrapper.cpp")
        .file("vendor/encoding_wrapper.cpp")
        .compile("basisuniversal");

    // Zstd comes from the zstd-sys crate (see lib.rs) instead of the vendored zstd.c: a consumer
    // that also links zstd-sys would otherwise get every ZSTD_* symbol twice. basisu only calls
    // the stable API, so the vendored zstd.h still matches.

    // cc prints rerun-if-env-changed lines, which turn off the default rerun on any change of the
    // package: without these, an edit to the wrappers wouldn't rebuild them.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vendor");

    // We regenerate binding code and check it in. (See generate_bindings.sh)
}
