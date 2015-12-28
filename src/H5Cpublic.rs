#[derive(Debug)]
#[repr(C)]
pub enum H5C_cache_incr_mode {
    H5C_incr__off,
    H5C_incr__threshold,
}
pub use self::H5C_cache_incr_mode::*;

#[derive(Debug)]
#[repr(C)]
pub enum H5C_cache_flash_incr_mode {
    H5C_flash_incr__off,
    H5C_flash_incr__add_space,
}
pub use self::H5C_cache_flash_incr_mode::*;

#[derive(Debug)]
#[repr(C)]
pub enum H5C_cache_decr_mode {
    H5C_decr__off,
    H5C_decr__threshold,
    H5C_decr__age_out,
    H5C_decr__age_out_with_threshold,
}
pub use self::H5C_cache_decr_mode::*;
