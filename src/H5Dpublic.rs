use libc::{c_char, c_void};

use H5Ipublic::hid_t;
use H5public::{herr_t, hsize_t};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5D_layout_t {
    H5D_LAYOUT_ERROR = -1,
    H5D_COMPACT = 0,
    H5D_CONTIGUOUS = 1,
    H5D_CHUNKED = 2,
    H5D_NLAYOUTS = 3,
}
pub use self::H5D_layout_t::*;

extern "C" {
    pub fn H5Dcreate2(loc_id: hid_t, name: *const c_char, type_id: hid_t, space_id: hid_t,
                      lcpl_id: hid_t, dcpl_id: hid_t, dapl_id: hid_t) -> hid_t;

    pub fn H5Dclose(dset_id: hid_t) -> herr_t;
    pub fn H5Dget_space(dset_id: hid_t) -> hid_t;

    pub fn H5Dwrite(dset_id: hid_t, mem_type_id: hid_t, mem_space_id: hid_t, file_space_id: hid_t,
                    plist_id: hid_t, buf: *const c_void) -> herr_t;

    pub fn H5Dset_extent(dset_id: hid_t, size: *const hsize_t) -> herr_t;
}
