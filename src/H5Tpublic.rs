use libc::{c_char, c_int, c_uint, c_void, size_t};

use H5Ipublic::hid_t;
use H5public::{herr_t, hsize_t, htri_t, hbool_t};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_class_t {
    H5T_NO_CLASS = -1,
    H5T_INTEGER = 0,
    H5T_FLOAT = 1,
    H5T_TIME = 2,
    H5T_STRING = 3,
    H5T_BITFIELD = 4,
    H5T_OPAQUE = 5,
    H5T_COMPOUND = 6,
    H5T_REFERENCE = 7,
    H5T_ENUM = 8,
    H5T_VLEN = 9,
    H5T_ARRAY = 10,
    H5T_NCLASSES,
}
pub use self::H5T_class_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_order_t {
    H5T_ORDER_ERROR = -1,
    H5T_ORDER_LE = 0,
    H5T_ORDER_BE = 1,
    H5T_ORDER_VAX = 2,
    H5T_ORDER_MIXED = 3,
    H5T_ORDER_NONE = 4,
}
pub use self::H5T_order_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_sign_t {
    H5T_SGN_ERROR = -1,
    H5T_SGN_NONE = 0,
    H5T_SGN_2 = 1,
    H5T_NSGN = 2,
}
pub use self::H5T_sign_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_norm_t {
    H5T_NORM_ERROR = -1,
    H5T_NORM_IMPLIED = 0,
    H5T_NORM_MSBSET = 1,
    H5T_NORM_NONE = 2,
}
pub use self::H5T_norm_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_cset_t {
    H5T_CSET_ERROR = -1,
    H5T_CSET_ASCII = 0,
    H5T_CSET_UTF8 = 1,
    H5T_CSET_RESERVED_2 = 2,
    H5T_CSET_RESERVED_3 = 3,
    H5T_CSET_RESERVED_4 = 4,
    H5T_CSET_RESERVED_5 = 5,
    H5T_CSET_RESERVED_6 = 6,
    H5T_CSET_RESERVED_7 = 7,
    H5T_CSET_RESERVED_8 = 8,
    H5T_CSET_RESERVED_9 = 9,
    H5T_CSET_RESERVED_10 = 10,
    H5T_CSET_RESERVED_11 = 11,
    H5T_CSET_RESERVED_12 = 12,
    H5T_CSET_RESERVED_13 = 13,
    H5T_CSET_RESERVED_14 = 14,
    H5T_CSET_RESERVED_15 = 15,
}
pub use self::H5T_cset_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_str_t {
    H5T_STR_ERROR = -1,
    H5T_STR_NULLTERM = 0,
    H5T_STR_NULLPAD = 1,
    H5T_STR_SPACEPAD = 2,
    H5T_STR_RESERVED_3 = 3,
    H5T_STR_RESERVED_4 = 4,
    H5T_STR_RESERVED_5 = 5,
    H5T_STR_RESERVED_6 = 6,
    H5T_STR_RESERVED_7 = 7,
    H5T_STR_RESERVED_8 = 8,
    H5T_STR_RESERVED_9 = 9,
    H5T_STR_RESERVED_10 = 10,
    H5T_STR_RESERVED_11 = 11,
    H5T_STR_RESERVED_12 = 12,
    H5T_STR_RESERVED_13 = 13,
    H5T_STR_RESERVED_14 = 14,
    H5T_STR_RESERVED_15 = 15,
}
pub use self::H5T_str_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_pad_t {
    H5T_PAD_ERROR = -1,
    H5T_PAD_ZERO = 0,
    H5T_PAD_ONE = 1,
    H5T_PAD_BACKGROUND = 2,
    H5T_NPAD = 3,
}
pub use self::H5T_pad_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_cmd_t {
    H5T_CONV_INIT = 0,
    H5T_CONV_CONV = 1,
    H5T_CONV_FREE = 2,
}
pub use self::H5T_cmd_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_bkg_t {
    H5T_BKG_NO = 0,
    H5T_BKG_TEMP = 1,
    H5T_BKG_YES = 2,
}
pub use self::H5T_bkg_t::*;

#[repr(C)]
pub struct H5T_cdata_t {
    command: H5T_cmd_t,
    need_bkg: H5T_bkg_t,
    recalc: hbool_t,
    priv_data: *const c_void,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_pers_t {
    H5T_PERS_DONTCARE = -1,
    H5T_PERS_HARD = 0,
    H5T_PERS_SOFT = 1,
}
pub use self::H5T_pers_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_direction_t {
    H5T_DIR_DEFAULT = 0,
    H5T_DIR_ASCEND = 1,
    H5T_DIR_DESCEND = 2,
}
pub use self::H5T_direction_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_conv_except_t {
    H5T_CONV_EXCEPT_RANGE_HI = 0,
    H5T_CONV_EXCEPT_RANGE_LOW = 1,
    H5T_CONV_EXCEPT_PRECISION = 2,
    H5T_CONV_EXCEPT_TRUNCATE = 3,
    H5T_CONV_EXCEPT_PINF = 4,
    H5T_CONV_EXCEPT_NINF = 5,
    H5T_CONV_EXCEPT_NAN = 6,
}
pub use self::H5T_conv_except_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5T_conv_ret_t {
    H5T_CONV_ABORT = -1,
    H5T_CONV_UNHANDLED = 0,
    H5T_CONV_HANDLED = 1,
}
pub use self::H5T_conv_ret_t::*;

pub type H5T_conv_t = extern fn(hid_t, hid_t, *mut H5T_cdata_t, size_t, size_t, size_t, *mut c_void, *mut c_void, hid_t) -> herr_t;
pub type H5T_conv_except_func_t = extern fn(H5T_conv_except_t, hid_t, hid_t, *mut c_void, *mut c_void, *mut c_void) -> H5T_conv_ret_t;

extern "C" {
    pub static H5T_C_S1_g: hid_t;
    pub static H5T_NATIVE_FLOAT_g: hid_t;
    pub static H5T_NATIVE_DOUBLE_g: hid_t;
    pub static H5T_NATIVE_INT8_g: hid_t;
    pub static H5T_NATIVE_UINT8_g: hid_t;
    pub static H5T_NATIVE_INT16_g: hid_t;
    pub static H5T_NATIVE_UINT16_g: hid_t;
    pub static H5T_NATIVE_INT32_g: hid_t;
    pub static H5T_NATIVE_UINT32_g: hid_t;
    pub static H5T_NATIVE_INT64_g: hid_t;
    pub static H5T_NATIVE_UINT64_g: hid_t;
}

pub use self::H5T_C_S1_g as H5T_C_S1;
pub use self::H5T_NATIVE_FLOAT_g as H5T_NATIVE_FLOAT;
pub use self::H5T_NATIVE_DOUBLE_g as H5T_NATIVE_DOUBLE;
pub use self::H5T_NATIVE_INT8_g as H5T_NATIVE_INT8;
pub use self::H5T_NATIVE_UINT8_g as H5T_NATIVE_UINT8;
pub use self::H5T_NATIVE_INT16_g as H5T_NATIVE_INT16;
pub use self::H5T_NATIVE_UINT16_g as H5T_NATIVE_UINT16;
pub use self::H5T_NATIVE_INT32_g as H5T_NATIVE_INT32;
pub use self::H5T_NATIVE_UINT32_g as H5T_NATIVE_UINT32;
pub use self::H5T_NATIVE_INT64_g as H5T_NATIVE_INT64;
pub use self::H5T_NATIVE_UINT64_g as H5T_NATIVE_UINT64;

extern "C" {
    pub fn H5Tcreate(dtype: H5T_class_t, size: size_t) -> hid_t;
    pub fn H5Tcopy(dtype_id: hid_t) -> hid_t;
    pub fn H5Tclose(dtype_id: hid_t) -> herr_t;
    pub fn H5Tequal(dtype1_id: hid_t, dtype2_id: hid_t) -> htri_t;
    pub fn H5Tlock(dtype_id: hid_t) -> herr_t;
    pub fn H5Tcommit2(loc_id: hid_t, name: *const c_char, dtype_id: hid_t, lcpl_id: hid_t,
                      tcpl_id: hid_t, tapl_id: hid_t) -> herr_t;
    pub fn H5Topen2(loc_id: hid_t, name: *const c_char, tapl_id: hid_t) -> hid_t;
    pub fn H5Tcommit_anon(loc_id: hid_t, dtype_id: hid_t, tcpl_id: hid_t, tapl_id: hid_t) -> herr_t;
    pub fn H5Tget_create_plist(dtype_id: hid_t) -> hid_t;
    pub fn H5Tcommitted(dtype_id: hid_t) -> htri_t;
    pub fn H5Tencode(obj_id: hid_t, buf: *mut c_void, nalloc: *mut size_t) -> herr_t;
    pub fn H5Tdecode(buf: *const c_void) -> hid_t;
    pub fn H5Tinsert(parent_id: hid_t, name: *const c_char, offset: size_t, member_id: hid_t) -> herr_t;
    pub fn H5Tpack(dtype_id: hid_t) -> herr_t;
    pub fn H5Tenum_create(base_id: hid_t) -> hid_t;
    pub fn H5Tenum_insert(dtype: hid_t, name: *const c_char, value: *const c_void) -> herr_t;
    pub fn H5Tenum_nameof(dtype: hid_t, value: *const c_void, name: *mut c_char/*out*/, size: size_t) -> herr_t;
    pub fn H5Tenum_valueof(dtype: hid_t, name: *const c_char, value: *mut c_void/*out*/) -> herr_t;
    pub fn H5Tvlen_create(base_id: hid_t) -> hid_t;
    pub fn H5Tarray_create2(base_id: hid_t, ndims: c_uint, dim: *const hsize_t) -> hid_t;
    pub fn H5Tget_array_ndims(dtype_id: hid_t) -> c_int;
    pub fn H5Tget_array_dims2(dtype_id: hid_t, dims: *mut hsize_t) -> c_int;
    pub fn H5Tset_tag(dtype: hid_t, tag: *const c_char) -> herr_t;
    pub fn H5Tget_tag(dtype: hid_t) -> *const c_char;
    pub fn H5Tget_super(dtype: hid_t) -> hid_t;
    pub fn H5Tget_class(dtype_id: hid_t) -> H5T_class_t;
    pub fn H5Tdetect_class(dtype_id: hid_t, cls: H5T_class_t) -> htri_t;
    pub fn H5Tget_size(dtype_id: hid_t) -> size_t;
    pub fn H5Tget_order(dtype_id: hid_t) -> H5T_order_t;
    pub fn H5Tget_precision(dtype_id: hid_t) -> size_t;
    pub fn H5Tget_offset(dtype_id: hid_t) -> c_int;
    pub fn H5Tget_pad(dtype_id: hid_t, lsb: *mut H5T_pad_t/*out*/, msb: *mut H5T_pad_t/*out*/) -> herr_t;
    pub fn H5Tget_sign(dtype_id: hid_t) -> H5T_sign_t;
    pub fn H5Tget_fields(dtype_id: hid_t, spos: *mut size_t/*out*/, epos: *mut size_t/*out*/,
                         esize: *mut size_t/*out*/, mpos: *mut size_t/*out*/,
                         msize: *mut size_t/*out*/) -> herr_t;
    pub fn H5Tget_ebias(dtype_id: hid_t) -> size_t;
    pub fn H5Tget_norm(dtype_id: hid_t) -> H5T_norm_t;
    pub fn H5Tget_inpad(dtype_id: hid_t) -> H5T_pad_t;
    pub fn H5Tget_strpad(dtype_id: hid_t) -> H5T_str_t;
    pub fn H5Tget_nmembers(dtype_id: hid_t) -> c_int;
    pub fn H5Tget_member_name(dtype_id: hid_t, membno: c_uint) -> *const c_char;
    pub fn H5Tget_member_index(dtype_id: hid_t, name: *const c_char) -> c_int;
    pub fn H5Tget_member_offset(dtype_id: hid_t, membno: c_uint) -> size_t;
    pub fn H5Tget_member_class(dtype_id: hid_t, membno: c_uint) -> H5T_class_t;
    pub fn H5Tget_member_dtype(dtype_id: hid_t, membno: c_uint) -> hid_t;
    pub fn H5Tget_member_value(dtype_id: hid_t, membno: c_uint, value: *mut c_void/*out*/) -> herr_t;
    pub fn H5Tget_cset(dtype_id: hid_t) -> H5T_cset_t;
    pub fn H5Tis_variable_str(dtype_id: hid_t) -> htri_t;
    pub fn H5Tget_native_dtype(dtype_id: hid_t, direction: H5T_direction_t) -> hid_t;
    pub fn H5Tset_size(dtype_id: hid_t, size: size_t) -> herr_t;
    pub fn H5Tset_order(dtype_id: hid_t, order: H5T_order_t) -> herr_t;
    pub fn H5Tset_precision(dtype_id: hid_t, prec: size_t) -> herr_t;
    pub fn H5Tset_offset(dtype_id: hid_t, offset: size_t) -> herr_t;
    pub fn H5Tset_pad(dtype_id: hid_t, lsb: H5T_pad_t, msb: H5T_pad_t) -> herr_t;
    pub fn H5Tset_sign(dtype_id: hid_t, sign: H5T_sign_t) -> herr_t;
    pub fn H5Tset_fields(dtype_id: hid_t, spos: size_t, epos: size_t, esize: size_t,
                         mpos: size_t, msize: size_t) -> herr_t;
    pub fn H5Tset_ebias(dtype_id: hid_t, ebias: size_t) -> herr_t;
    pub fn H5Tset_norm(dtype_id: hid_t, norm: H5T_norm_t) -> herr_t;
    pub fn H5Tset_inpad(dtype_id: hid_t, pad: H5T_pad_t) -> herr_t;
    pub fn H5Tset_cset(dtype_id: hid_t, cset: H5T_cset_t) -> herr_t;
    pub fn H5Tset_strpad(dtype_id: hid_t, strpad: H5T_str_t) -> herr_t;
    pub fn H5Tregister(pers: H5T_pers_t, name: *const c_char, src_id: hid_t,
                       dst_id: hid_t, func: H5T_conv_t) -> herr_t;
    pub fn H5Tunregister(pers: H5T_pers_t, name: *const c_char, src_id: hid_t,
                         dst_id: hid_t, func: H5T_conv_t) -> herr_t;
    pub fn H5Tfind(src_id: hid_t, dst_id: hid_t, pcdata: *mut *const H5T_cdata_t) -> H5T_conv_t;
    pub fn H5Tcompiler_conv(src_id: hid_t, dst_id: hid_t) -> htri_t;
    pub fn H5Tconvert(src_id: hid_t, dst_id: hid_t, nelmts: size_t, buf: *mut c_void,
                      background: *const c_void, plist_id: hid_t) -> herr_t;
}
