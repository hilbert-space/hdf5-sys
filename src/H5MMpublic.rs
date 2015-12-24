use libc::{c_void, size_t};

pub type H5MM_allocate_t = extern "C" fn(size_t, *mut c_void) -> *const c_void;
pub type H5MM_free_t = extern "C" fn(*mut c_void, *mut c_void);
