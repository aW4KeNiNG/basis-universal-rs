/* automatically generated by rust-bindgen 0.66.1 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct basisu_color_rgba {
    pub __bindgen_anon_1: basisu_color_rgba__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union basisu_color_rgba__bindgen_ty_1 {
    pub m_comps: [u8; 4usize],
    pub __bindgen_anon_1: basisu_color_rgba__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basisu_color_rgba__bindgen_ty_1__bindgen_ty_1 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[test]
fn bindgen_test_layout_basisu_color_rgba__bindgen_ty_1__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<basisu_color_rgba__bindgen_ty_1__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<basisu_color_rgba__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(basisu_color_rgba__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<basisu_color_rgba__bindgen_ty_1__bindgen_ty_1>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(basisu_color_rgba__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_color_rgba__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).g) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_color_rgba__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_color_rgba__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_color_rgba__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(a)
        )
    );
}
#[test]
fn bindgen_test_layout_basisu_color_rgba__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<basisu_color_rgba__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<basisu_color_rgba__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(basisu_color_rgba__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<basisu_color_rgba__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(basisu_color_rgba__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m_comps) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basisu_color_rgba__bindgen_ty_1),
            "::",
            stringify!(m_comps)
        )
    );
}
#[test]
fn bindgen_test_layout_basisu_color_rgba() {
    assert_eq!(
        ::std::mem::size_of::<basisu_color_rgba>(),
        4usize,
        concat!("Size of: ", stringify!(basisu_color_rgba))
    );
    assert_eq!(
        ::std::mem::align_of::<basisu_color_rgba>(),
        1usize,
        concat!("Alignment of ", stringify!(basisu_color_rgba))
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct basisu_image {
    pub _bindgen_opaque_blob: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_basisu_image() {
    assert_eq!(
        ::std::mem::size_of::<basisu_image>(),
        32usize,
        concat!("Size of: ", stringify!(basisu_image))
    );
    assert_eq!(
        ::std::mem::align_of::<basisu_image>(),
        8usize,
        concat!("Alignment of ", stringify!(basisu_image))
    );
}
extern "C" {
    #[link_name = "\u{1}?debug_text@image@basisu@@QEAAXIIIIAEBVcolor_rgba@2@PEBV32@_NPEBDZZ"]
    pub fn basisu_image_debug_text(
        this: *mut basisu_image,
        x_ofs: u32,
        y_ofs: u32,
        x_scale: u32,
        y_scale: u32,
        fg: *const basisu_color_rgba,
        pBG: *const basisu_color_rgba,
        alpha_only: bool,
        p: *const ::std::os::raw::c_char,
        ...
    );
}
pub const basisu_TOTAL_PACK_UASTC_LEVELS: u32 = 5;
pub const basisu_BASISU_MAX_SUPPORTED_TEXTURE_DIMENSION: u32 = 16384;
pub const basisu_BASISU_DEFAULT_ENDPOINT_RDO_THRESH: f32 = 1.5;
pub const basisu_BASISU_DEFAULT_SELECTOR_RDO_THRESH: f32 = 1.25;
pub const basisu_BASISU_DEFAULT_QUALITY: ::std::os::raw::c_int = 128;
pub const basisu_BASISU_DEFAULT_HYBRID_SEL_CB_QUALITY_THRESH: f32 = 2.0;
pub const basisu_BASISU_MAX_IMAGE_DIMENSION: u32 = 16384;
pub const basisu_BASISU_QUALITY_MIN: u32 = 1;
pub const basisu_BASISU_QUALITY_MAX: u32 = 255;
pub const basisu_BASISU_MAX_ENDPOINT_CLUSTERS: u32 = 16128;
pub const basisu_BASISU_MAX_SELECTOR_CLUSTERS: u32 = 16128;
pub const basisu_BASISU_MAX_SLICES: u32 = 16777215;
pub const basisu_BASISU_RDO_UASTC_DICT_SIZE_DEFAULT: ::std::os::raw::c_int = 4096;
pub const basisu_BASISU_RDO_UASTC_DICT_SIZE_MIN: ::std::os::raw::c_int = 64;
pub const basisu_BASISU_RDO_UASTC_DICT_SIZE_MAX: ::std::os::raw::c_int = 65536;
pub const basisu_basis_compressor_error_code_cECSuccess: basisu_basis_compressor_error_code = 0;
pub const basisu_basis_compressor_error_code_cECFailedInitializing:
    basisu_basis_compressor_error_code = 1;
pub const basisu_basis_compressor_error_code_cECFailedReadingSourceImages:
    basisu_basis_compressor_error_code = 2;
pub const basisu_basis_compressor_error_code_cECFailedValidating:
    basisu_basis_compressor_error_code = 3;
pub const basisu_basis_compressor_error_code_cECFailedEncodeUASTC:
    basisu_basis_compressor_error_code = 4;
pub const basisu_basis_compressor_error_code_cECFailedFrontEnd: basisu_basis_compressor_error_code =
    5;
pub const basisu_basis_compressor_error_code_cECFailedFontendExtract:
    basisu_basis_compressor_error_code = 6;
pub const basisu_basis_compressor_error_code_cECFailedBackend: basisu_basis_compressor_error_code =
    7;
pub const basisu_basis_compressor_error_code_cECFailedCreateBasisFile:
    basisu_basis_compressor_error_code = 8;
pub const basisu_basis_compressor_error_code_cECFailedWritingOutput:
    basisu_basis_compressor_error_code = 9;
pub const basisu_basis_compressor_error_code_cECFailedUASTCRDOPostProcess:
    basisu_basis_compressor_error_code = 10;
pub const basisu_basis_compressor_error_code_cECFailedCreateKTX2File:
    basisu_basis_compressor_error_code = 11;
pub type basisu_basis_compressor_error_code = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ColorU8 {
    pub channels: ColorU8_Channels,
    pub components: [u8; 4usize],
    pub combined: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColorU8_Channels {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[test]
fn bindgen_test_layout_ColorU8_Channels() {
    const UNINIT: ::std::mem::MaybeUninit<ColorU8_Channels> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ColorU8_Channels>(),
        4usize,
        concat!("Size of: ", stringify!(ColorU8_Channels))
    );
    assert_eq!(
        ::std::mem::align_of::<ColorU8_Channels>(),
        1usize,
        concat!("Alignment of ", stringify!(ColorU8_Channels))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ColorU8_Channels),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).g) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(ColorU8_Channels),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ColorU8_Channels),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(ColorU8_Channels),
            "::",
            stringify!(a)
        )
    );
}
#[test]
fn bindgen_test_layout_ColorU8() {
    const UNINIT: ::std::mem::MaybeUninit<ColorU8> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ColorU8>(),
        4usize,
        concat!("Size of: ", stringify!(ColorU8))
    );
    assert_eq!(
        ::std::mem::align_of::<ColorU8>(),
        4usize,
        concat!("Alignment of ", stringify!(ColorU8))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).channels) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ColorU8),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).components) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ColorU8),
            "::",
            stringify!(components)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).combined) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ColorU8),
            "::",
            stringify!(combined)
        )
    );
}
pub const UastcPackFlags_PackUASTCLevelFastest: UastcPackFlags = 0;
pub const UastcPackFlags_PackUASTCLevelFaster: UastcPackFlags = 1;
pub const UastcPackFlags_PackUASTCLevelDefault: UastcPackFlags = 2;
pub const UastcPackFlags_PackUASTCLevelSlower: UastcPackFlags = 3;
pub const UastcPackFlags_PackUASTCLevelVerySlow: UastcPackFlags = 4;
pub const UastcPackFlags_PackUASTCLevelMask: UastcPackFlags = 15;
pub const UastcPackFlags_PackUASTCFavorUASTCError: UastcPackFlags = 8;
pub const UastcPackFlags_PackUASTCFavorBC7Error: UastcPackFlags = 16;
pub const UastcPackFlags_PackUASTCETC1FasterHints: UastcPackFlags = 64;
pub const UastcPackFlags_PackUASTCETC1FastestHints: UastcPackFlags = 128;
pub const UastcPackFlags_PackUASTCETC1DisableFlipAndIndividual: UastcPackFlags = 256;
pub type UastcPackFlags = ::std::os::raw::c_int;
extern "C" {
    pub fn image_clear(image: *mut basisu_image);
}
extern "C" {
    pub fn image_resize_with_pitch(
        image: *mut basisu_image,
        w: u32,
        h: u32,
        p: u32,
    );
}
extern "C" {
    pub fn image_resize(
        image: *mut basisu_image,
        w: u32,
        h: u32,
    );
}
extern "C" {
    pub fn image_init(
        image: *mut basisu_image,
        pData: *const u8,
        width: u32,
        height: u32,
        comps: u32,
    );
}
extern "C" {
    pub fn image_get_pixel_at_checked(
        image: *mut basisu_image,
        x: u32,
        y: u32,
        pOutColor: *mut ColorU8,
    ) -> bool;
}
extern "C" {
    pub fn image_get_pixel_at_unchecked(
        image: *mut basisu_image,
        x: u32,
        y: u32,
    ) -> ColorU8;
}
extern "C" {
    pub fn image_get_width(image: *mut basisu_image) -> u32;
}
extern "C" {
    pub fn image_get_height(image: *mut basisu_image) -> u32;
}
extern "C" {
    pub fn image_get_pitch(image: *mut basisu_image) -> u32;
}
extern "C" {
    pub fn image_get_total_pixels(image: *mut basisu_image) -> u32;
}
extern "C" {
    pub fn image_get_block_width(
        image: *mut basisu_image,
        w: u32,
    ) -> u32;
}
extern "C" {
    pub fn image_get_block_height(
        image: *mut basisu_image,
        h: u32,
    ) -> u32;
}
extern "C" {
    pub fn image_get_total_blocks(
        image: *mut basisu_image,
        w: u32,
        h: u32,
    ) -> u32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PixelData {
    pub pData: *mut ColorU8,
    pub length: usize,
}
#[test]
fn bindgen_test_layout_PixelData() {
    const UNINIT: ::std::mem::MaybeUninit<PixelData> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<PixelData>(),
        16usize,
        concat!("Size of: ", stringify!(PixelData))
    );
    assert_eq!(
        ::std::mem::align_of::<PixelData>(),
        8usize,
        concat!("Alignment of ", stringify!(PixelData))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pData) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PixelData),
            "::",
            stringify!(pData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PixelData),
            "::",
            stringify!(length)
        )
    );
}
extern "C" {
    pub fn image_get_pixel_data(image: *mut basisu_image) -> PixelData;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct CompressorParams {
    pub _bindgen_opaque_blob: u64,
}
#[test]
fn bindgen_test_layout_CompressorParams() {
    assert_eq!(
        ::std::mem::size_of::<CompressorParams>(),
        8usize,
        concat!("Size of: ", stringify!(CompressorParams))
    );
    assert_eq!(
        ::std::mem::align_of::<CompressorParams>(),
        8usize,
        concat!("Alignment of ", stringify!(CompressorParams))
    );
}
extern "C" {
    pub fn compressor_params_new() -> *mut CompressorParams;
}
extern "C" {
    pub fn compressor_params_delete(params: *mut CompressorParams);
}
extern "C" {
    pub fn compressor_params_clear(params: *mut CompressorParams);
}
extern "C" {
    pub fn compressor_params_get_or_create_source_image(
        params: *mut CompressorParams,
        index: u32,
    ) -> *mut basisu_image;
}
extern "C" {
    pub fn compressor_params_resize_source_image_list(
        params: *mut CompressorParams,
        size: usize,
    );
}
extern "C" {
    pub fn compressor_params_clear_source_image_list(params: *mut CompressorParams);
}
extern "C" {
    pub fn compressor_params_get_or_create_source_mipmap_image(
        params: *mut CompressorParams,
        index: u32,
        level: u32,
    ) -> *mut basisu_image;
}
extern "C" {
    pub fn compressor_params_resize_source_mipmap_image_list(
        params: *mut CompressorParams,
        size: usize,
    );
}
extern "C" {
    pub fn compressor_params_resize_source_mipmap_image_level_list(
        params: *mut CompressorParams,
        level: usize,
        size: usize,
    );
}
extern "C" {
    pub fn compressor_params_clear_source_mipmap_image_list(params: *mut CompressorParams);
}
extern "C" {
    pub fn compressor_params_set_status_output(
        params: *mut CompressorParams,
        status_output: bool,
    );
}
extern "C" {
    pub fn compressor_params_set_quality_level(
        params: *mut CompressorParams,
        quality_level: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn compressor_params_get_pack_uastc_flags(params: *mut CompressorParams) -> UastcPackFlags;
}
extern "C" {
    pub fn compressor_params_set_pack_uastc_flags(
        params: *mut CompressorParams,
        pack_uastc_flags: UastcPackFlags,
    );
}
extern "C" {
    pub fn compressor_params_set_uastc(
        params: *mut CompressorParams,
        is_uastc: bool,
    );
}
extern "C" {
    pub fn compressor_params_set_perceptual(
        params: *mut CompressorParams,
        perceptual: bool,
    );
}
extern "C" {
    pub fn compressor_params_set_mip_srgb(
        params: *mut CompressorParams,
        mip_srgb: bool,
    );
}
extern "C" {
    pub fn compressor_params_set_no_selector_rdo(
        params: *mut CompressorParams,
        no_selector_rdo: bool,
    );
}
extern "C" {
    pub fn compressor_params_set_no_endpoint_rdo(
        params: *mut CompressorParams,
        no_endpoint_rdo: bool,
    );
}
extern "C" {
    pub fn compressor_params_set_rdo_uastc(
        params: *mut CompressorParams,
        rdo_uastc: bool,
    );
}
extern "C" {
    pub fn compressor_params_set_rdo_uastc_quality_scalar(
        params: *mut CompressorParams,
        rdo_uastc_quality_scalar: f32,
    );
}
extern "C" {
    pub fn compressor_params_set_generate_mipmaps(
        params: *mut CompressorParams,
        generate_mipmaps: bool,
    );
}
extern "C" {
    pub fn compressor_params_set_mip_smallest_dimension(
        params: *mut CompressorParams,
        mip_smallest_dimension: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn compressor_params_set_userdata(
        params: *mut CompressorParams,
        userdata0: u32,
        userdata1: u32,
    );
}
extern "C" {
    pub fn compressor_params_set_create_ktx2_file(
        params: *mut CompressorParams,
        create_ktx2_file: bool,
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Compressor {
    pub _bindgen_opaque_blob: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_Compressor() {
    assert_eq!(
        ::std::mem::size_of::<Compressor>(),
        16usize,
        concat!("Size of: ", stringify!(Compressor))
    );
    assert_eq!(
        ::std::mem::align_of::<Compressor>(),
        8usize,
        concat!("Alignment of ", stringify!(Compressor))
    );
}
extern "C" {
    pub fn compressor_new(num_threads: ::std::os::raw::c_int) -> *mut Compressor;
}
extern "C" {
    pub fn compressor_delete(compressor: *mut Compressor);
}
extern "C" {
    pub fn compressor_init(
        compressor: *mut Compressor,
        params: *const CompressorParams,
    ) -> bool;
}
extern "C" {
    pub fn compressor_process(compressor: *mut Compressor) -> basisu_basis_compressor_error_code;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CompressorBasisFile {
    pub pData: *const u8,
    pub length: usize,
}
#[test]
fn bindgen_test_layout_CompressorBasisFile() {
    const UNINIT: ::std::mem::MaybeUninit<CompressorBasisFile> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<CompressorBasisFile>(),
        16usize,
        concat!("Size of: ", stringify!(CompressorBasisFile))
    );
    assert_eq!(
        ::std::mem::align_of::<CompressorBasisFile>(),
        8usize,
        concat!("Alignment of ", stringify!(CompressorBasisFile))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pData) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CompressorBasisFile),
            "::",
            stringify!(pData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CompressorBasisFile),
            "::",
            stringify!(length)
        )
    );
}
extern "C" {
    pub fn compressor_get_output_basis_file(compressor: *mut Compressor) -> CompressorBasisFile;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CompressorKtx2File {
    pub pData: *const u8,
    pub length: usize,
}
#[test]
fn bindgen_test_layout_CompressorKtx2File() {
    const UNINIT: ::std::mem::MaybeUninit<CompressorKtx2File> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<CompressorKtx2File>(),
        16usize,
        concat!("Size of: ", stringify!(CompressorKtx2File))
    );
    assert_eq!(
        ::std::mem::align_of::<CompressorKtx2File>(),
        8usize,
        concat!("Alignment of ", stringify!(CompressorKtx2File))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pData) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CompressorKtx2File),
            "::",
            stringify!(pData)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CompressorKtx2File),
            "::",
            stringify!(length)
        )
    );
}
extern "C" {
    pub fn compressor_get_output_ktx2_file(compressor: *mut Compressor) -> CompressorKtx2File;
}
extern "C" {
    pub fn compressor_get_basis_file_size(compressor: *const Compressor) -> u32;
}
extern "C" {
    pub fn compressor_get_basis_bits_per_texel(compressor: *const Compressor) -> f64;
}
extern "C" {
    pub fn compressor_get_any_source_image_has_alpha(compressor: *const Compressor) -> bool;
}
extern "C" {
    pub fn basisu_encoder_init();
}
