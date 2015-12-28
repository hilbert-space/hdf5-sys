use libc::{c_int, c_longlong, c_uint, c_ulonglong, c_void, size_t};

pub type herr_t = c_int;
pub type htri_t = c_int;
pub type hsize_t = c_ulonglong;
pub type hssize_t = c_longlong;
pub type hbool_t = c_uint;

#[cfg(target_pointer_width = "32")]
pub type haddr_t = u32;
#[cfg(target_pointer_width = "64")]
pub type haddr_t = u64;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5_iter_order_t {
    H5_ITER_UNKNOWN = -1,
    H5_ITER_INC,
    H5_ITER_DEC,
    H5_ITER_NATIVE,
    H5_ITER_N,
}
pub use self::H5_iter_order_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5_index_t {
    H5_INDEX_UNKNOWN = -1,
    H5_INDEX_NAME,
    H5_INDEX_CRT_ORDER,
    H5_INDEX_N,
}
pub use self::H5_index_t::*;

#[derive(Debug)]
#[repr(C)]
pub struct H5_ih_info_t {
    pub index_size: hsize_t,
    pub heap_size: hsize_t,
}

extern "C" {
    pub fn H5open() -> herr_t;
    pub fn H5dont_atexit() -> herr_t;
    pub fn H5set_free_list_limits(reg_global_lim: c_int, reg_list_lim: c_int,
                                  arr_global_lim: c_int, arr_list_lim: c_int,
                                  blk_global_lim: c_int, blk_list_lim: c_int) -> herr_t;
    pub fn H5garbage_collect() -> herr_t;
    pub fn H5allocate_memory(size: size_t, clear: hbool_t) -> *mut c_void;
    pub fn H5resize_memory(mem: *mut c_void, size: size_t ) -> *mut c_void;
    pub fn H5free_memory(buf: *mut c_void) -> herr_t;
    pub fn H5get_libversion(majnum: *mut c_uint, minnum: *mut c_uint, relnum: *mut c_uint)
                            -> herr_t;
    pub fn H5check_version(majnum: c_uint, minnum: c_uint, relnum: c_uint) -> herr_t;
    pub fn H5is_library_threadsafe(is_ts: *mut hbool_t) -> herr_t;
}
