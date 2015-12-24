use libc::{c_char, c_int, c_uint, c_void, size_t, ssize_t};

use H5public::{hbool_t, herr_t, htri_t, hsize_t};

pub type hid_t = c_int;

pub type H5I_search_func_t = extern fn (*const c_void, hid_t, *const c_void) -> c_int;
pub type H5I_free_t = extern fn(*mut c_void) -> herr_t;

#[repr(C)]
pub enum H5I_type_t {
    H5I_UNINIT = (-2),
    H5I_BADID = (-1),
    H5I_FILE = 1,
    H5I_GROUP,
    H5I_DATATYPE,
    H5I_DATASPACE,
    H5I_DATASET,
    H5I_ATTR,
    H5I_REFERENCE,
    H5I_VFL,
    H5I_GENPROP_CLS,
    H5I_GENPROP_LST,
    H5I_ERROR_CLASS,
    H5I_ERROR_MSG,
    H5I_ERROR_STACK,
    H5I_NTYPES,
}
pub use self::H5I_type_t::*;

extern "C" {
    pub fn H5Iget_file_id(obj_id: hid_t) -> hid_t;
    pub fn H5Iget_name(obj_id: hid_t, name: *mut c_char, size: size_t) -> ssize_t;
    pub fn H5Iget_type(obj_id: hid_t) -> H5I_type_t;
    pub fn H5Iobject_verify(id: hid_t, id_member_type: H5I_type_t) -> *mut c_void;
    pub fn H5Iremove_verify(id: hid_t, id_member_type: H5I_type_t) -> *mut c_void;
    pub fn H5Isearch(member_type: H5I_type_t, func: H5I_search_func_t, key: *const c_void)
                     -> *mut c_void;
    pub fn H5Iis_valid(obj_id: hid_t) -> htri_t;
    pub fn H5Iget_ref(obj_id: hid_t) -> c_int;
    pub fn H5Iinc_ref(obj_id: hid_t) -> c_int;
    pub fn H5Idec_ref(obj_id: hid_t) -> c_int;
    pub fn H5Iregister(member_type: H5I_type_t, object: *const c_void) -> hid_t;
    pub fn H5Iregister_type(hash_size: size_t, reserved: c_uint, free_func: H5I_free_t)
                            -> H5I_type_t;
    pub fn H5Idestroy_type(member_type: H5I_type_t) -> herr_t;
    pub fn H5Itype_exists(member_type: H5I_type_t) -> htri_t;
    pub fn H5Iget_type_ref(member_type: H5I_type_t) -> c_int;
    pub fn H5Idec_type_ref(member_type: H5I_type_t) -> c_int;
    pub fn H5Iinc_type_ref(member_type: H5I_type_t) -> c_int;
    pub fn H5Iclear_type(member_type: H5I_type_t, force: hbool_t) -> herr_t;
    pub fn H5Inmembers(member_type: H5I_type_t, num_members: *mut hsize_t) -> herr_t;
}
