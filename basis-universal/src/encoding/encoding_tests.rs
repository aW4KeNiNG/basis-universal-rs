use super::*;
use crate::BasisTextureFormat;

#[test]
fn test_new_compressor_params() {
    let compressor_params = CompressorParams::new();
    std::mem::drop(compressor_params);
}

#[test]
fn test_new_compressor() {
    let compressor = Compressor::default();
    std::mem::drop(compressor);
}

#[test]
fn test_compressor_init() {
    // basisu 2.x rejects (and asserts in debug builds) params without source images
    let mut compressor_params = CompressorParams::new();
    compressor_params
        .source_image_mut(0)
        .init(&[255u8; 4 * 4 * 4], 4, 4, 4);
    let mut compressor = Compressor::default();
    unsafe {
        assert!(compressor.init(&compressor_params));
    }
    std::mem::drop(compressor);
    std::mem::drop(compressor_params);
}

/// The transfer function in the DFD of a KTX2 file: 1 is linear, 2 is sRGB
fn ktx2_transfer_function(ktx2: &[u8]) -> u8 {
    use std::convert::TryInto;
    let dfd_offset = u32::from_le_bytes(ktx2[48..52].try_into().unwrap()) as usize;
    // dfdTotalSize, two words of block header, colorModel, colorPrimaries, transferFunction
    ktx2[dfd_offset + 14]
}

fn compress_ktx2(configure: impl FnOnce(&mut CompressorParams)) -> Vec<u8> {
    let mut compressor_params = CompressorParams::new();
    // First, so that a reset() doesn't undo the rest
    configure(&mut compressor_params);
    compressor_params.generate_ktx2_file();
    compressor_params.set_print_status_to_stdout(false);
    compressor_params.set_basis_format(BasisTextureFormat::UASTC4x4);
    compressor_params
        .source_image_mut(0)
        .init(&[128u8; 8 * 8 * 4], 8, 8, 4);
    let mut compressor = Compressor::default();
    unsafe {
        assert!(compressor.init(&compressor_params));
        compressor.process().unwrap();
    }
    compressor.ktx2_file().to_vec()
}

#[test]
fn test_transfer_function_is_linear_unless_set_to_srgb() {
    // basisu 2.x defaults to sRGB, the binding keeps the linear default of 1.16
    assert_eq!(ktx2_transfer_function(&compress_ktx2(|_| {})), 1);
    assert_eq!(
        ktx2_transfer_function(&compress_ktx2(|params| params.reset())),
        1
    );
    let srgb = compress_ktx2(|params| params.set_transfer_function_color_space(ColorSpace::Srgb));
    assert_eq!(ktx2_transfer_function(&srgb), 2);
    let normal_map = compress_ktx2(|params| {
        params.set_transfer_function_color_space(ColorSpace::Srgb);
        params.tune_for_normal_maps();
    });
    assert_eq!(ktx2_transfer_function(&normal_map), 1);
}

#[test]
fn test_uastc_quality_level_replaces_the_level() {
    let level = |params: &CompressorParams| unsafe {
        sys::compressor_params_get_pack_uastc_flags(params.0)
            & basis_universal_sys::UastcPackFlags_PackUASTCLevelMask
    };
    let mut compressor_params = CompressorParams::new();
    assert_eq!(
        level(&compressor_params),
        crate::UASTC_QUALITY_DEFAULT as i32
    );
    // An OR with the default level (2) turned 1 into 3
    compressor_params.set_uastc_quality_level(1);
    assert_eq!(level(&compressor_params), 1);
    compressor_params.set_uastc_quality_level(0);
    assert_eq!(level(&compressor_params), 0);
    compressor_params.set_uastc_quality_level(3);
    assert_eq!(level(&compressor_params), 3);
}

#[test]
fn test_compressor_params_smoketest_bindings() {
    let mut compressor_params = CompressorParams::new();

    // Call every parameter just to smoketest the bindings
    compressor_params.source_image_mut(5);
    compressor_params.resize_source_image_list(8);
    compressor_params.clear_source_image_list();
    compressor_params.set_print_status_to_stdout(false);
    compressor_params.set_etc1s_quality_level(crate::ETC1S_QUALITY_DEFAULT);
    compressor_params.set_uastc_quality_level(crate::UASTC_QUALITY_DEFAULT);
    compressor_params.set_basis_format(BasisTextureFormat::UASTC4x4);
    compressor_params.set_generate_mipmaps(true);
    compressor_params.set_etc1s_compression_level(crate::ETC1S_COMPRESSION_LEVEL_MAX);
    compressor_params.set_rdo_uastc_dict_size(crate::UASTC_RDO_DICT_SIZE_MIN);

    compressor_params.reset();
}

#[test]
fn test_image_smoketest_bindings() {
    let mut compressor_params = CompressorParams::new();

    let mut image = compressor_params.source_image_mut(0);
    let color = image.pixel_at(50, 50);
    assert!(color.is_none());
    image.resize(100, 100);
    let color = image.pixel_at(50, 50);
    assert!(color.is_some());
    let _color = unsafe { image.pixel_at_unchecked(50, 50) };
    image.invalidate();
}

#[test]
fn test_encode_image() {
    //
    // Read the PNG file from disk
    //
    let png_file = include_bytes!("../../test_assets/rust-logo.png");
    let image_data =
        image::load_from_memory_with_format(png_file, image::ImageFormat::Png).unwrap();

    use image::ColorType;
    let channel_count = match &image_data.color() {
        ColorType::L8 => 1,
        ColorType::La8 => 2,
        ColorType::Rgb8 => 3,
        ColorType::Rgba8 => 4,
        ColorType::L16 => 1,
        ColorType::La16 => 2,
        ColorType::Rgb16 => 3,
        ColorType::Rgba16 => 4,
        _ => unimplemented!(),
    };

    let mut compressor_params = CompressorParams::new();
    compressor_params.set_generate_mipmaps(true);

    //
    // Set up the source image in the params
    //
    let mut compressor_image = compressor_params.source_image_mut(0);
    compressor_image.init(
        image_data.as_bytes(),
        image_data.width(),
        image_data.height(),
        channel_count,
    );

    //
    // Create the compressor
    //
    let mut compressor = Compressor::default();
    unsafe {
        compressor.init(&compressor_params);
    }
    // Drop explicitly here to verify that borrowing rules allow this and that this doesn't cause a crash
    std::mem::drop(compressor_params);

    //
    // Do the compression
    //
    unsafe {
        compressor.process().unwrap();
    }

    // By default the test shouldn't write to disk, but this is a quick way to put it on disk to
    // check that it works with basisu
    let _basis_file = compressor.basis_file();
    //std::fs::write("test_assets/test_encode_image.basis", basis_file).unwrap();

    std::mem::drop(compressor);
}

#[test]
fn test_ktx2_zstd_supercompression() {
    use std::convert::TryInto;
    let ktx2 = compress_ktx2(|params| {
        params.set_ktx2_uastc_supercompression(crate::Ktx2UastcSuperCompression::ZStandard);
        params.set_ktx2_uastc_supercompression_level(crate::zstd_level_max());
    });
    // supercompressionScheme of the KTX2 header: 2 is Zstandard
    assert_eq!(u32::from_le_bytes(ktx2[44..48].try_into().unwrap()), 2);
}
