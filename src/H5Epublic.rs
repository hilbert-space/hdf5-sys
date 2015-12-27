use libc::{FILE, c_char, c_void, c_uint, size_t, ssize_t};
use std::ptr;

use H5Ipublic::hid_t;
use H5public::herr_t;

pub type H5E_major_t = hid_t;
pub type H5E_minor_t = hid_t;

pub type H5E_walk2_t = extern "C" fn(c_uint, *const H5E_error2_t, *const c_void);
pub type H5E_auto2_t = extern "C" fn(hid_t, *const c_void);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5E_type_t {
    H5E_MAJOR,
    H5E_MINOR,
}
pub use self::H5E_type_t::*;
enum_default!(H5E_type_t, H5E_type_t::H5E_MAJOR);

#[repr(C)]
pub struct H5E_error2_t {
    pub cls_id: hid_t,
    pub maj_num: hid_t,
    pub min_num: hid_t,
    pub line: c_uint,
    pub func_name: *const c_char,
    pub file_name: *const c_char,
    pub desc: *const c_char,
}

impl Default for H5E_error2_t {
    #[inline]
    fn default() -> H5E_error2_t {
        H5E_error2_t {
            cls_id: 0,
            maj_num: 0,
            min_num: 0,
            line: 0,
            func_name: ptr::null(),
            file_name: ptr::null(),
            desc: ptr::null(),
        }
    }
}
new_as_default!(H5E_error2_t);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5E_direction_t {
    H5E_WALK_UPWARD = 0,
    H5E_WALK_DOWNWARD = 1,
}
pub use self::H5E_direction_t::*;
enum_default!(H5E_direction_t, H5E_direction_t::H5E_WALK_UPWARD);

extern "C" {
    pub fn H5Eauto_is_v2(estack_id: hid_t, is_tack: *mut c_uint) -> herr_t;
    pub fn H5Eclear2(estack: hid_t) -> herr_t;
    pub fn H5Eclose_msg(mesg_id: hid_t) -> herr_t;
    pub fn H5Eclose_stack(estack_id: hid_t) -> herr_t;
    pub fn H5Ecreate_msg(class: hid_t, msg_type: H5E_type_t, mesg: *const c_char) -> hid_t;
    pub fn H5Ecreate_stack() -> hid_t;
    pub fn H5Eget_auto2(estack_id: hid_t, func: *mut H5E_auto2_t, client_data: *mut c_void)
                        -> herr_t;
    pub fn H5Eget_class_name(class_id: hid_t, name: *const c_char, size: size_t) -> ssize_t;
    pub fn H5Eget_current_stack() -> hid_t;
    pub fn H5Eget_msg(mesg_id: hid_t, mesg_type: *mut H5E_type_t, mesg: *mut c_char, size: size_t)
                      -> ssize_t;
    pub fn H5Eget_num(estack_id: hid_t) -> ssize_t;
    pub fn H5Epop(estack_id: hid_t, count: size_t) -> herr_t;
    pub fn H5Eprint2(estack_id: hid_t, stream: *const FILE) -> herr_t;
    pub fn H5Epush2(estack_id: hid_t, file: *const c_char, func: *const c_char, line: c_uint,
                    maj_num: H5E_major_t, min_num: H5E_minor_t, msg: *const c_char, ...)
                    -> herr_t;
    pub fn H5Eregister_class(cls_name: *const c_char, lib_name: *const c_char,
                             version: *const c_char) -> hid_t;
    pub fn H5Eset_auto2(error_stack: hid_t, func: H5E_auto2_t, client_data: *const c_void)
                        -> herr_t;
    pub fn H5Eset_current_stack(estack_id: hid_t) -> herr_t;
    pub fn H5Eunregister_class(class_id: hid_t) -> herr_t;
    pub fn H5Ewalk2(estack_id: hid_t, direction: H5E_direction_t, func: H5E_walk2_t,
                    client_data: *const c_void) -> herr_t;
}
