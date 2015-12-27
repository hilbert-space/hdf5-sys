use libc::{c_int, c_uint, c_void, size_t};

use H5public::{herr_t, htri_t};

pub type H5Z_filter_t = c_int;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5Z_SO_scale_type_t {
    H5Z_SO_FLOAT_DSCALE = 0,
    H5Z_SO_FLOAT_ESCALE = 1,
    H5Z_SO_INT = 2,
}
pub use self::H5Z_SO_scale_type_t::*;
enum_default!(H5Z_SO_scale_type_t, H5Z_SO_scale_type_t::H5Z_SO_FLOAT_DSCALE);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5Z_EDC_t {
    H5Z_ERROR_EDC = -1,
    H5Z_DISABLE_EDC = 0,
    H5Z_ENABLE_EDC = 1,
    H5Z_NO_EDC = 2,
}
pub use self::H5Z_EDC_t::*;
enum_default!(H5Z_EDC_t, H5Z_EDC_t::H5Z_ERROR_EDC);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5Z_cb_return_t {
    H5Z_CB_ERROR = -1,
    H5Z_CB_FAIL = 0,
    H5Z_CB_CONT = 1,
    H5Z_CB_NO = 2,
}
pub use self::H5Z_cb_return_t::*;
enum_default!(H5Z_cb_return_t, H5Z_cb_return_t::H5Z_CB_ERROR);

pub type H5Z_filter_func_t = extern "C" fn(H5Z_filter_t,*mut c_void, size_t, *mut c_void)
                                           -> H5Z_cb_return_t;

#[repr(C)]
pub struct H5Z_cb_t {
    func: H5Z_filter_func_t,
    op_data: *const c_void,
}

extern "C" {
    pub fn H5Zregister(cls: *const c_void) -> herr_t;
    pub fn H5Zunregister(id: H5Z_filter_t) -> herr_t;
    pub fn H5Zfilter_avail(id: H5Z_filter_t) -> htri_t;
    pub fn H5Zget_filter_info(filter: H5Z_filter_t, filter_config_flags: *mut c_uint) -> herr_t;
}
