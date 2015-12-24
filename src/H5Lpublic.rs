use libc::{c_char, c_int, c_uint, c_void, size_t, ssize_t};

use H5Ipublic::hid_t;
use H5Tpublic::H5T_cset_t;
use H5public::{herr_t, htri_t, hbool_t, hsize_t, haddr_t, H5_index_t, H5_iter_order_t};

pub type H5L_create_func_t = extern fn(*const c_char, hid_t, *const c_void, size_t, hid_t);
pub type H5L_move_func_t = extern fn(*const c_char, hid_t, *const c_void, size_t);
pub type H5L_copy_func_t = extern fn(*const c_char, hid_t, *const c_void, size_t);
pub type H5L_traverse_func_t = extern fn(*const c_char, hid_t, *const c_void, size_t, hid_t);
pub type H5L_delete_func_t = extern fn(*const c_char, hid_t, *const c_void, size_t, hid_t);
pub type H5L_query_func_t = extern fn(*const c_char, *const c_void, size_t, *mut c_void, size_t);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5L_type_t {
    H5L_TYPE_ERROR = (-1),
    H5L_TYPE_HARD = 0,
    H5L_TYPE_SOFT = 1,
    H5L_TYPE_EXTERNAL = 64,
    H5L_TYPE_MAX = 255,
}
pub use self::H5L_type_t::*;

#[repr(C)]
pub struct H5L_info_t {
    link_type: H5L_type_t,
    corder_valid: hbool_t,
    corder: i64,
    cset: H5T_cset_t,
    address: haddr_t,  // TODO: Should be a haddr_t/size_t union (address/val_size)
}

#[repr(C)]
pub struct H5L_class_t {
    version: c_int,
    id: H5L_type_t,
    comment: *const c_char,
    create_func: H5L_create_func_t,
    move_func: H5L_move_func_t,
    copy_func: H5L_copy_func_t,
    trav_func: H5L_traverse_func_t,
    del_func: H5L_delete_func_t,
    query_func: H5L_query_func_t,
}

pub type H5L_iterate_t = extern fn(hid_t, *const c_char, *const H5L_info_t, *mut c_void);
pub type H5L_elink_traverse_t = extern fn(*const c_char, *const c_char, *const c_char,
                                          *const c_char, *mut c_uint, hid_t, *mut c_void);

extern "C" {
    pub fn H5Lcreate_hard(obj_loc_id: hid_t, obj_name: *const c_char, link_loc_id: hid_t,
                          link_name: *const c_char, lcpl_id: hid_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Lcreate_soft(target_path: *const c_char, link_loc_id: hid_t, link_name: *const c_char,
                          lcpl_id: hid_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Lcreate_external(target_file_name: *const c_char, target_obj_name: *const c_char,
                              link_loc_id: hid_t, link_name: *const c_char, lcpl_id: hid_t,
                              lapl_id: hid_t) -> herr_t;
    pub fn H5Lexists(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> htri_t;
    pub fn H5Lmove(src_loc_id: hid_t, src_name: *const c_char, dest_loc_id: hid_t,
                   dest_name: *const c_char, lcpl_id: hid_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Lcopy(src_loc_id: hid_t, src_name: *const c_char, dest_loc_id: hid_t,
                   dest_name: *const c_char, lcpl_id: hid_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Ldelete(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> herr_t;
    pub fn H5Lget_info(link_loc_id: hid_t, link_name: *const c_char, link_buff: *mut H5L_info_t,
                       lapl_id: hid_t) -> herr_t;
    pub fn H5Lget_val(link_loc_id: hid_t, link_name: *const c_char, linkval_buff: *mut c_void,
                      size: size_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Lunpack_elink_val(ext_linkval: *const c_char, link_size: size_t, flags: *mut c_uint,
                               filename: *mut *const c_char, obj_path: *mut *const c_char) -> herr_t;
    pub fn H5Lcreate_ud(link_loc_id: hid_t, link_name: *const c_char, link_type: H5L_type_t,
                        udata: *const c_char, udata_size: size_t, lcpl_id: hid_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Lregister(link_class: *const H5L_class_t) -> herr_t;
    pub fn H5Lunregister(link_cls_id: H5L_type_t) -> herr_t;
    pub fn H5Lis_registered(link_cls_id: H5L_type_t) -> htri_t;
    pub fn H5Literate(group_id: hid_t, index_type: H5_index_t, order: H5_iter_order_t,
                      idx: *mut hsize_t, op: H5L_iterate_t, op_data: *const c_void) -> herr_t;
    pub fn H5Literate_by_name(loc_id: hid_t, group_name: *const c_char, index_type: H5_index_t,
                              order: H5_iter_order_t, idx: *mut hsize_t, op: H5L_iterate_t,
                              op_data: *const c_void, lapl_id: *const hid_t) -> herr_t;
    pub fn H5Lvisit(group_id: hid_t, index_type: H5_index_t, order: H5_iter_order_t,
                    op: H5L_iterate_t, op_data: *const c_void) -> herr_t;
    pub fn H5Lvisit_by_name(loc_id: hid_t, group_name: *const c_char, index_type: H5_index_t,
                            order: H5_iter_order_t, op: H5L_iterate_t, op_data: *const c_void,
                            lapl_id: hid_t) -> herr_t;
    pub fn H5Lget_info_by_idx(loc_id: hid_t, group_name: *const c_char, index_field: H5_index_t,
                              order: H5_iter_order_t, n: hsize_t, link_val: *mut H5L_info_t,
                              lapl_id: hid_t) -> herr_t;
    pub fn H5Lget_name_by_idx(loc_id: hid_t, group_name: *const c_char, index_field: H5_index_t,
                              order: H5_iter_order_t, n: hsize_t, name: *mut c_char, size: size_t,
                              lapl_id: hid_t) -> ssize_t;
    pub fn H5Lget_val_by_idx(loc_id: hid_t, group_name: *const c_char, index_type: H5_index_t,
                             order: H5_iter_order_t, n: hsize_t, link_val: *mut c_void,
                             size: size_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Ldelete_by_idx(loc_id: hid_t, group_name: *const c_char, index_field: H5_index_t,
                            order: H5_iter_order_t, n: hsize_t, lapl_id: hid_t) -> herr_t;
}
