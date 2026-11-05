// Links the zstd library that basisu calls for KTX2 supercompression.
extern crate zstd_sys;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(deref_nullptr)]
#[rustfmt::skip]
pub mod transcoding_bindings;
pub use transcoding_bindings::*;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(deref_nullptr)]
#[rustfmt::skip]
pub mod encoding_bindings;
pub use encoding_bindings::*;
