use libc::c_char;

use H5Ipublic::hid_t;
use H5public::{H5_index_t, H5_iter_order_t, hbool_t, herr_t, hsize_t};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5G_storage_type_t {
    H5G_STORAGE_TYPE_UNKNOWN = -1,
    H5G_STORAGE_TYPE_SYMBOL_TABLE,
    H5G_STORAGE_TYPE_COMPACT,
    H5G_STORAGE_TYPE_DENSE,
}
pub use self::H5G_storage_type_t::*;
enum_default!(H5G_storage_type_t, H5G_storage_type_t::H5G_STORAGE_TYPE_UNKNOWN);

#[derive(Default, Debug)]
#[repr(C)]
pub struct H5G_info_t {
    pub storage_type: H5G_storage_type_t,
    pub nlinks: hsize_t,
    pub max_corder: i64,
    pub mounted: hbool_t,
}
new_as_default!(H5G_info_t);

extern "C" {
    pub fn H5Gcreate2(loc_id: hid_t, name: *const c_char, lcpl_id: hid_t, gcpl_id: hid_t,
                      gapl_id: hid_t) -> hid_t;
    pub fn H5Gcreate_anon(loc_id: hid_t, gcpl_id: hid_t, gapl_id: hid_t) -> hid_t;
    pub fn H5Gopen2(loc_id: hid_t, name: *const c_char, gapl_id: hid_t) -> hid_t;
    pub fn H5Gclose(group_id: hid_t) -> herr_t;
    pub fn H5Gget_info(group_id: hid_t, group_info: *mut H5G_info_t) -> herr_t;
    pub fn H5Gget_info_by_name(loc_id: hid_t, group_name: *const c_char,
                               group_info: *mut H5G_info_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Gget_create_plist(group_id: hid_t) -> hid_t;
    pub fn H5Gget_info_by_idx(loc_id: hid_t, group_name: *const c_char, index_type: H5_index_t,
                              order: H5_iter_order_t, n: hsize_t, group_info: *mut H5G_info_t,
                              lapl_id: hid_t) -> herr_t;
}
