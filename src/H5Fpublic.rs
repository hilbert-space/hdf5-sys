use libc::{c_char, c_uint};

use H5Ipublic::hid_t;
use H5public::herr_t;

pub const H5F_ACC_RDONLY: c_uint = 0x0000;
pub const H5F_ACC_RDWR:   c_uint = 0x0001;
pub const H5F_ACC_TRUNC:  c_uint = 0x0002;
pub const H5F_ACC_EXCL:   c_uint = 0x0004;
pub const H5F_ACC_DEBUG:  c_uint = 0x0008;
pub const H5F_ACC_CREAT:  c_uint = 0x0010;

extern "C" {
    pub fn H5Fcreate(filename: *const c_char, flags: c_uint, create_plist: hid_t,
                     access_plist: hid_t) -> hid_t;

    pub fn H5Fopen(filename: *const c_char, flags: c_uint, access_plist: hid_t) -> hid_t;
    pub fn H5Fclose(file_id: hid_t) -> herr_t;
}
