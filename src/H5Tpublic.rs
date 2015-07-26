use libc::{c_char, c_uint, size_t};

use H5Ipublic::hid_t;
use H5public::{herr_t, hsize_t};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_class_t {
    H5T_NO_CLASS = -1,
    H5T_INTEGER = 0,
    H5T_FLOAT = 1,
    H5T_TIME = 2,
    H5T_STRING = 3,
    H5T_BITFIELD = 4,
    H5T_OPAQUE = 5,
    H5T_COMPOUND = 6,
    H5T_REFERENCE = 7,
    H5T_ENUM = 8,
    H5T_VLEN = 9,
    H5T_ARRAY = 10,

    H5T_NCLASSES,
}
pub use self::H5T_class_t::*;

extern "C" {
    pub fn H5Tcreate(typo: H5T_class_t, size: size_t) -> hid_t;
    pub fn H5Tvlen_create(base_id: hid_t) -> hid_t;

    pub fn H5Tinsert(parent_id: hid_t, name: *const c_char, offset: size_t, member_id: hid_t)
                     -> herr_t;

    pub fn H5Tarray_create2(base_id: hid_t, ndims: c_uint, dim: *const hsize_t) -> hid_t;
}

extern "C" {
    pub static H5T_NATIVE_FLOAT_g: hid_t;
    pub static H5T_NATIVE_DOUBLE_g: hid_t;
    pub static H5T_NATIVE_INT8_g: hid_t;
    pub static H5T_NATIVE_UINT8_g: hid_t;
    pub static H5T_NATIVE_INT16_g: hid_t;
    pub static H5T_NATIVE_UINT16_g: hid_t;
    pub static H5T_NATIVE_INT32_g: hid_t;
    pub static H5T_NATIVE_UINT32_g: hid_t;
    pub static H5T_NATIVE_INT64_g: hid_t;
    pub static H5T_NATIVE_UINT64_g: hid_t;
}

pub use self::H5T_NATIVE_FLOAT_g  as H5T_NATIVE_FLOAT;
pub use self::H5T_NATIVE_DOUBLE_g as H5T_NATIVE_DOUBLE;
pub use self::H5T_NATIVE_INT8_g   as H5T_NATIVE_INT8;
pub use self::H5T_NATIVE_UINT8_g  as H5T_NATIVE_UINT8;
pub use self::H5T_NATIVE_INT16_g  as H5T_NATIVE_INT16;
pub use self::H5T_NATIVE_UINT16_g as H5T_NATIVE_UINT16;
pub use self::H5T_NATIVE_INT32_g  as H5T_NATIVE_INT32;
pub use self::H5T_NATIVE_UINT32_g as H5T_NATIVE_UINT32;
pub use self::H5T_NATIVE_INT64_g  as H5T_NATIVE_INT64;
pub use self::H5T_NATIVE_UINT64_g as H5T_NATIVE_UINT64;
