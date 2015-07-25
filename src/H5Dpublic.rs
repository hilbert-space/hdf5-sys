use libc::{c_char, c_void};

use H5Ipublic::hid_t;
use H5public::herr_t;

extern "C" {
    pub fn H5Dcreate2(loc_id: hid_t, name: *const c_char, type_id: hid_t, space_id: hid_t,
                      lcpl_id: hid_t, dcpl_id: hid_t, dapl_id: hid_t) -> hid_t;

    pub fn H5Dclose(dset_id: hid_t) -> herr_t;

    pub fn H5Dwrite(dset_id: hid_t, mem_type_id: hid_t, mem_space_id: hid_t, file_space_id: hid_t,
                    plist_id: hid_t, buf: *const c_void) -> herr_t;
}
