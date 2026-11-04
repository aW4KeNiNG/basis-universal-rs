# Changelog

## Unreleased
 * Updated basis_universal to 2.50 (`v2_50`). The ETC1S and UASTC LDR 4x4 encoders produce the
   same blocks as 1.16; KTX2 files are slightly different because Zstd went from 1.4.9 to 1.5.7
 * Requires C++17. The build compiles every codec of the upstream encoder library, Zstd as C,
   defines `NDEBUG` in release builds (the asserts of basisu aborted the process) and passes
   `-msse4.1` to GCC/Clang with `hint_sse4_1`
 * `basis_validate_output_buffer_size` ignores `total_slice_blocks` (dropped upstream)
 * `CompressorErrorCode` is converted with a match instead of a transmute, and has the codes it
   lacked (`cECFailedInitializing`, `cECFailedCreateKTX2File`, `cECFailedInvalidParameters`) and
   `cECUnknown`
 * Keeps the linear defaults of 1.16: basisu 2.x defaults `m_mip_srgb` and the sRGB transfer
   function of the output to true, which made every KTX2 declare sRGB (normal maps included)
 * New `CompressorParams::set_transfer_function_color_space`, for the transfer function the output
   declares. `tune_for_normal_maps` sets it to linear
 * `clear_source_image_list` clears only the source images (it cleared every parameter)
 * Dependencies: bitflags 2 (`DecodeFlags` derives `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` and
   `Hash`), image 0.25 for the tests and examples, and no more lazy_static

## 0.3.0
 * Updated to most recent up-stream (1.16.x)
 * `compressor_params_set_global_sel_pal`, `compressor_params_set_auto_global_sel_pal` and
   `etc1_global_selector_codebook` were removed, as they no longer exist upstream
 * Some of the bindings have been changed to i32 to match upstream C++ behavior

## 0.2.0
 * Add basis_block_format_is_uncompressed()
 * Add LowLevelUastcTranscoder, TranscoderBlockFormat
 * Fix typo HIGH_QULITY -> HIGH_QUALITY
 * Add files to be excluded when publishing

## 0.1.1
 * Add set_rdo_uastc()
 * Bump upstream code to more recent revision

## 0.1.0
 * Initial release
