use libc::{c_char, c_uint, c_ulong, c_void, size_t, ssize_t, time_t};

use H5Ipublic::hid_t;
use H5public::{H5_ih_info_t, H5_index_t, H5_iter_order_t, haddr_t, herr_t, hsize_t, htri_t};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5O_type_t {
    H5O_TYPE_UNKNOWN = -1,
    H5O_TYPE_GROUP,
    H5O_TYPE_DATASET,
    H5O_TYPE_NAMED_DATATYPE,
    H5O_TYPE_NTYPES,
}
pub use self::H5O_type_t::*;

#[derive(Debug)]
#[repr(C)]
pub struct H5O_hdr_info_space_t {
    pub total: hsize_t,
    pub meta: hsize_t,
    pub mesg: hsize_t,
    pub free: hsize_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct H5O_hdr_info_mesg_t {
    pub present: u64,
    pub shared: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct H5O_hdr_info_t {
    pub version: c_uint,
    pub nmesgs: c_uint,
    pub nchunks: c_uint,
    pub flags: c_uint,
    pub space: H5O_hdr_info_space_t,
    pub mesg: H5O_hdr_info_mesg_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct H5O_info_meta_size_t {
    pub obj: H5_ih_info_t,
    pub attr: H5_ih_info_t,
}

#[derive(Debug)]
#[repr(C)]
pub struct H5O_info_t {
    pub fileno: c_ulong,
    pub addr: haddr_t,
    pub info_type: H5O_type_t,
    pub rc: c_uint,
    pub atime: time_t,
    pub mtime: time_t,
    pub ctime: time_t,
    pub btime: time_t,
    pub num_attrs: hsize_t,
    pub hdr: H5O_hdr_info_t,
    pub meta_size: H5O_info_meta_size_t,
}

pub type H5O_msg_crt_idx_t = u32;

pub type H5O_iterate_t = extern "C" fn(hid_t, *const c_char, *const H5O_info_t, *mut c_void)
                                       -> herr_t;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5O_mcdt_search_ret_t {
    H5O_MCDT_SEARCH_ERROR = -1,
    H5O_MCDT_SEARCH_CONT,
    H5O_MCDT_SEARCH_STOP,
}
pub use self::H5O_mcdt_search_ret_t::*;

pub type H5O_mcdt_search_cb_t = extern "C" fn(*mut c_void) -> H5O_mcdt_search_ret_t;

extern "C" {
    pub fn H5Oopen(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> hid_t;
    pub fn H5Oopen_by_idx(loc_id: hid_t, group_name: *const c_char, index_type: H5_index_t,
                          order: H5_iter_order_t, n: hsize_t, lapl_id: hid_t) -> hid_t;
    pub fn H5Oopen_by_addr(loc_id: hid_t, addr: haddr_t) -> hid_t;
    pub fn H5Olink(object_id: hid_t, new_loc_id: hid_t, new_link_name: *const c_char, lcpl: hid_t,
                   lapl: hid_t) -> herr_t;
    pub fn H5Oclose(object_id: hid_t) -> herr_t;
    pub fn H5Ocopy(src_loc_id: hid_t, src_name: *const c_char, dst_loc_id: hid_t,
                   dst_name: *const c_char, ocpypl_id: hid_t, lcpl_id: hid_t) -> herr_t;
    pub fn H5Ovisit(object_id: hid_t, index_type: H5_index_t, order: H5_iter_order_t,
                    op: H5O_iterate_t, op_data: *const c_void) -> herr_t;
    pub fn H5Ovisit_by_name(loc_id: hid_t, object_name: *const c_char, index_type: H5_index_t,
                            order: H5_iter_order_t, op: H5O_iterate_t, op_data: *const c_void,
                            lapl_id: hid_t) -> herr_t;
    pub fn H5Oget_comment(object_id: hid_t, comment: *mut c_char, bufsize: size_t) -> ssize_t;
    pub fn H5Oget_comment_by_name(loc_id: hid_t, name: *const c_char, comment: *mut c_char,
                                  bufsize: size_t, lapl_id: hid_t) -> ssize_t;
    pub fn H5Oexists_by_name(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> htri_t;
    pub fn H5Oget_info(object_id: hid_t, object_info: *mut H5O_info_t) -> herr_t;
    pub fn H5Oget_info_by_name(loc_id: hid_t, object_name: *const c_char,
                               object_info: *mut H5O_info_t, lapl_id: hid_t) -> herr_t;
    pub fn H5Oget_info_by_idx(loc_id: hid_t, group_name: *const c_char, index_field: H5_index_t,
                              order: H5_iter_order_t, n: hsize_t, object_info: *mut H5O_info_t,
                              lapl_id: hid_t) -> herr_t;
    pub fn H5Oincr_refcount(object_id: hid_t) -> herr_t;
    pub fn H5Odecr_refcount(object_id: hid_t) -> herr_t;
}
