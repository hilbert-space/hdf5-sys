use libc::{c_char, c_void, size_t, ssize_t};

use H5Ipublic::hid_t;
use H5Opublic::H5O_type_t;
use H5public::herr_t;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5R_type_t {
    H5R_BADTYPE = -1,
    H5R_OBJECT,
    H5R_DATASET_REGION,
    H5R_MAXTYPE,
}
pub use self::H5R_type_t::*;

extern "C" {
    pub fn H5Rcreate(reference: *mut c_void, loc_id: hid_t, name: *const c_char,
                     ref_type: H5R_type_t, space_id: hid_t) -> herr_t;
    pub fn H5Rdereference(obj_id: hid_t, ref_type: H5R_type_t, reference: *const c_void) -> hid_t;
    pub fn H5Rget_obj_type2(loc_id: hid_t, ref_type: H5R_type_t, reference: *const c_void,
                            obj_type: *mut H5O_type_t) -> herr_t;
    pub fn H5Rget_region(loc_id: hid_t, ref_type: H5R_type_t, reference: *const c_void) -> hid_t;
    pub fn H5Rget_name(loc_id: hid_t, ref_type: H5R_type_t, reference: *const c_void,
                       name: *mut c_char, size: size_t) -> ssize_t;
}

