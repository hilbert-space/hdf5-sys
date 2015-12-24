use libc::{c_char, c_uint, c_void, size_t};

use H5Ipublic::hid_t;
use H5public::{herr_t, hsize_t, haddr_t};

pub type H5D_operator_t = extern fn(*mut c_void, hid_t, c_uint, *const hsize_t, *mut c_void);
pub type H5D_gather_func_t = extern fn(*const c_void, size_t, *mut c_void);
pub type H5D_scatter_func_t = extern fn(*mut *const c_void, *mut size_t, *mut c_void);

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

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5D_alloc_time_t {
    H5D_ALLOC_TIME_ERROR = -1,
    H5D_ALLOC_TIME_DEFAULT = 0,
    H5D_ALLOC_TIME_EARLY = 1,
    H5D_ALLOC_TIME_LATE = 2,
    H5D_ALLOC_TIME_INCR = 3,
}
pub use self::H5D_alloc_time_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5D_space_status_t {
    H5D_SPACE_STATUS_ERROR = -1,
    H5D_SPACE_STATUS_NOT_ALLOCATED = 0,
    H5D_SPACE_STATUS_PART_ALLOCATED = 1,
    H5D_SPACE_STATUS_ALLOCATED = 2,
}
pub use self::H5D_space_status_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5D_fill_time_t {
    H5D_FILL_TIME_ERROR = -1,
    H5D_FILL_TIME_ALLOC = 0,
    H5D_FILL_TIME_NEVER = 1,
    H5D_FILL_TIME_IFSET = 2,
}
pub use self::H5D_fill_time_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5D_fill_value_t {
    H5D_FILL_VALUE_ERROR = -1,
    H5D_FILL_VALUE_UNDEFINED = 0,
    H5D_FILL_VALUE_DEFAULT = 1,
    H5D_FILL_VALUE_USER_DEFINED = 2,
}
pub use self::H5D_fill_value_t::*;

extern "C" {
    pub fn H5Dcreate2(loc_id: hid_t, name: *const c_char, type_id: hid_t, space_id: hid_t,
                      lcpl_id: hid_t, dcpl_id: hid_t, dapl_id: hid_t) -> hid_t;
    pub fn H5Dcreate_anon(loc_id: hid_t, type_id: hid_t, space_id: hid_t, dcpl_id: hid_t,
                          dapl_id: hid_t) -> hid_t;
    pub fn H5Dopen2(loc_id: hid_t, name: *const c_char, dapl_id: hid_t) -> hid_t;
    pub fn H5Dclose(dset_id: hid_t) -> herr_t;
    pub fn H5Dget_space(dset_id: hid_t) -> hid_t;
    pub fn H5Dget_space_status(dset_id: hid_t, status: *mut H5D_space_status_t) -> herr_t;
    pub fn H5Dget_type(dataset_id: hid_t) -> hid_t;
    pub fn H5Dget_create_plist(dataset_id: hid_t) -> hid_t;
    pub fn H5Dget_access_plist(dataset_id: hid_t) -> hid_t;
    pub fn H5Dget_offset(dset_id: hid_t) -> haddr_t;
    pub fn H5Dget_storage_size(dataset_id: hid_t) -> hsize_t;
    pub fn H5Dvlen_get_buf_size(dataset_id: hid_t, type_id: hid_t, space_id: hid_t,
                                size: *mut hsize_t) -> herr_t;
    pub fn H5Dvlen_reclaim(type_id: hid_t, space_id: hid_t, plist_id: hid_t, buf: *const c_void)
                           -> herr_t;
    pub fn H5Dread(dataset_id: hid_t, mem_type_id: hid_t, mem_space_id: hid_t,
                   file_space_id: hid_t, xfer_plist_id: hid_t, buf: *mut c_void) -> herr_t;
    pub fn H5Dwrite(dset_id: hid_t, mem_type_id: hid_t, mem_space_id: hid_t, file_space_id: hid_t,
                    plist_id: hid_t, buf: *const c_void) -> herr_t;
    pub fn H5Dgather(src_space_id: hid_t, src_buf: *const c_void, type_id: hid_t,
                     dst_buf_size: size_t, dst_buf: *mut c_void, op: H5D_gather_func_t,
                     op_data: *const c_void) -> herr_t;
    pub fn H5Dscatter(op: H5D_scatter_func_t, op_data: *const c_void, type_id: hid_t,
                      dst_space_id: hid_t, dst_buf: *mut c_void) -> herr_t;
    pub fn H5Diterate(buf: *mut c_void, type_id: hid_t, space_id: hid_t, operator: H5D_operator_t,
                      operator_data: *mut c_void);
    pub fn H5Dset_extent(dset_id: hid_t, size: *const hsize_t) -> herr_t;
    pub fn H5Dfill(fill: *const c_void, fill_type_id: hid_t, buf: *mut c_void, buf_type_id: hid_t,
                   space_id: hid_t) -> herr_t;
}
