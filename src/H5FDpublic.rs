use libc::{c_void, size_t};

use H5Fpublic::H5F_mem_t;
use H5public::herr_t;

pub type H5FD_mem_t = H5F_mem_t;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5FD_file_image_op_t {
    H5FD_FILE_IMAGE_OP_NO_OP,
    H5FD_FILE_IMAGE_OP_PROPERTY_LIST_SET,
    H5FD_FILE_IMAGE_OP_PROPERTY_LIST_COPY,
    H5FD_FILE_IMAGE_OP_PROPERTY_LIST_GET,
    H5FD_FILE_IMAGE_OP_PROPERTY_LIST_CLOSE,
    H5FD_FILE_IMAGE_OP_FILE_OPEN,
    H5FD_FILE_IMAGE_OP_FILE_RESIZE,
    H5FD_FILE_IMAGE_OP_FILE_CLOSE,
}
pub use self::H5FD_file_image_op_t::*;
enum_default!(H5FD_file_image_op_t, H5FD_file_image_op_t::H5FD_FILE_IMAGE_OP_NO_OP);

type H5FD_file_image_callbacks_image_malloc_func = extern "C" fn(size_t, H5FD_file_image_op_t,
                                                                 *mut c_void) -> *const c_void;
type H5FD_file_image_callbacks_image_memcpy_func = extern "C" fn(*mut c_void, *const c_void,
                                                                 size_t, H5FD_file_image_op_t,
                                                                 udata: *mut c_void)
                                                                 -> *const c_void;
type H5FD_file_image_callbacks_image_realloc_func = extern "C" fn(*mut c_void, size_t,
                                                                  H5FD_file_image_op_t,
                                                                  *mut c_void) -> *const c_void;
type H5FD_file_image_callbacks_image_free_func = extern "C" fn(*mut c_void, H5FD_file_image_op_t,
                                                               *mut c_void) -> herr_t;
type H5FD_file_image_callbacks_udata_copy_func = extern "C" fn(*mut c_void) -> *const c_void;
type H5FD_file_image_callbacks_udata_free_func = extern "C" fn(*mut c_void) -> herr_t;

#[repr(C)]
pub struct H5FD_file_image_callbacks_t {
    image_malloc: H5FD_file_image_callbacks_image_malloc_func,
    image_memcpy: H5FD_file_image_callbacks_image_memcpy_func,
    image_realloc: H5FD_file_image_callbacks_image_realloc_func,
    image_free: H5FD_file_image_callbacks_image_free_func,
    udata_copy: H5FD_file_image_callbacks_udata_copy_func,
    udata_free: H5FD_file_image_callbacks_udata_free_func,
    udata: *const c_void,
}
