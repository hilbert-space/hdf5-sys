use libc::{c_char, c_void, size_t, ssize_t};

use H5Ipublic::hid_t;
use H5Opublic::H5O_msg_crt_idx_t;
use H5Tpublic::H5T_cset_t;
use H5public::{H5_index_t, H5_iter_order_t, hbool_t, herr_t, hsize_t, htri_t};

#[repr(C)]
pub struct H5A_info_t {
    corder_valid: hbool_t,
    corder: H5O_msg_crt_idx_t,
    cset: H5T_cset_t,
    data_size: hsize_t,
}

pub type H5A_operator2_t = extern "C" fn(hid_t, *const c_char, *const H5A_info_t, *mut c_void)
                                         -> herr_t;

extern "C" {
    pub fn H5Acreate2(loc_id: hid_t, attr_name: *const c_char, type_id: hid_t, space_id: hid_t,
                      acpl_id: hid_t, aapl_id: hid_t) -> hid_t;
    pub fn H5Acreate_by_name(loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char,
                             type_id: hid_t, space_id: hid_t, acpl_id: hid_t, aapl_id: hid_t,
                             lapl_id: hid_t) -> hid_t;
    pub fn H5Aopen(obj_id: hid_t, attr_name: *const c_char, aapl_id: hid_t) -> hid_t;
    pub fn H5Aopen_by_name(loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char,
                           aapl_id: hid_t, lapl_id: hid_t) -> hid_t;
    pub fn H5Aopen_by_idx(loc_id: hid_t, obj_name: *const c_char, idx_type: H5_index_t,
                          order: H5_iter_order_t, n: hsize_t, aapl_id: hid_t, lapl_id: hid_t)
                          -> hid_t;
    pub fn H5Awrite(attr_id: hid_t, type_id: hid_t, buf: *const c_void) -> herr_t;
    pub fn H5Aread(attr_id: hid_t, type_id: hid_t, buf: *mut c_void) -> herr_t;
    pub fn H5Aclose(attr_id: hid_t) -> herr_t;
    pub fn H5Aget_space(attr_id: hid_t) -> hid_t;
    pub fn H5Aget_type(attr_id: hid_t) -> hid_t;
    pub fn H5Aget_create_plist(attr_id: hid_t) -> hid_t;
    pub fn H5Aget_name(attr_id: hid_t, buf_size: size_t, buf: *mut c_char) -> ssize_t;
    pub fn H5Aget_name_by_idx(loc_id: hid_t, obj_name: *const c_char, idx_type: H5_index_t,
                              order: H5_iter_order_t, n: hsize_t, name: *mut c_char, size: size_t,
                              lapl_id: hid_t) -> ssize_t;
    pub fn H5Aget_storage_size(attr_id: hid_t) -> hsize_t;
    pub fn H5Aget_info(attr_id: hid_t, ainfo: *mut H5A_info_t) -> herr_t;
    pub fn H5Aget_info_by_name(loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char,
                               ainfo: *mut H5A_info_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Aget_info_by_idx(loc_id: hid_t, obj_name: *const c_char, idx_type: H5_index_t,
                              order: H5_iter_order_t, n: hsize_t, ainfo: *mut H5A_info_t,
                              lapl_id: hid_t) -> herr_t;
    pub fn H5Arename(loc_id: hid_t, old_name: *const c_char, new_name: *const c_char) -> herr_t;
    pub fn H5Arename_by_name(loc_id: hid_t, obj_name: *const c_char, old_attr_name: *const c_char,
                             new_attr_name: *const c_char, lapl_id: hid_t) -> herr_t;
    pub fn H5Aiterate2(loc_id: hid_t, idx_type: H5_index_t, order: H5_iter_order_t,
                       idx: *mut hsize_t, op: H5A_operator2_t, op_data: *mut c_void) -> herr_t;
    pub fn H5Aiterate_by_name(loc_id: hid_t, obj_name: *const c_char, idx_type: H5_index_t,
                              order: H5_iter_order_t, idx: *mut hsize_t, op: H5A_operator2_t,
                              op_data: *mut c_void, lapd_id: hid_t) -> herr_t;
    pub fn H5Adelete(loc_id: hid_t, name: *const c_char) -> herr_t;
    pub fn H5Adelete_by_name(loc_id: hid_t, obj_name: *const c_char, attr_name: *const c_char,
                             lapl_id: hid_t) -> herr_t;
    pub fn H5Adelete_by_idx(loc_id: hid_t, obj_name: *const c_char, idx_type: H5_index_t,
                            order: H5_iter_order_t, n: hsize_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Aexists(obj_id: hid_t, attr_name: *const c_char) -> htri_t;
    pub fn H5Aexists_by_name(obj_id: hid_t, obj_name: *const c_char, attr_name: *const c_char,
                             lapl_id: hid_t) -> htri_t;
}
