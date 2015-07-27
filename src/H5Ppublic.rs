use libc::c_int;

use H5Dpublic::H5D_layout_t;
use H5Ipublic::hid_t;
use H5public::{herr_t, hsize_t};

pub const H5P_DEFAULT: hid_t = 0;

extern "C" {
    pub fn H5Pcreate(cls_id: hid_t) -> hid_t;
    pub fn H5Pclose(plist_id: hid_t) -> herr_t;
    pub fn H5Pset_layout(plist_id: hid_t, layout: H5D_layout_t) -> herr_t;
    pub fn H5Pset_chunk(plist_id: hid_t, ndims: c_int, dim: *const hsize_t) -> herr_t;
}

extern "C" {
    pub static H5P_CLS_ROOT_ID_g: hid_t;
    pub static H5P_CLS_OBJECT_CREATE_ID_g: hid_t;
    pub static H5P_CLS_FILE_CREATE_ID_g: hid_t;
    pub static H5P_CLS_FILE_ACCESS_ID_g: hid_t;
    pub static H5P_CLS_DATASET_CREATE_ID_g: hid_t;
    pub static H5P_CLS_DATASET_ACCESS_ID_g: hid_t;
    pub static H5P_CLS_DATASET_XFER_ID_g: hid_t;
    pub static H5P_CLS_FILE_MOUNT_ID_g: hid_t;
    pub static H5P_CLS_GROUP_CREATE_ID_g: hid_t;
    pub static H5P_CLS_GROUP_ACCESS_ID_g: hid_t;
    pub static H5P_CLS_DATATYPE_CREATE_ID_g: hid_t;
    pub static H5P_CLS_DATATYPE_ACCESS_ID_g: hid_t;
    pub static H5P_CLS_STRING_CREATE_ID_g: hid_t;
    pub static H5P_CLS_ATTRIBUTE_CREATE_ID_g: hid_t;
    pub static H5P_CLS_OBJECT_COPY_ID_g: hid_t;
    pub static H5P_CLS_LINK_CREATE_ID_g: hid_t;
    pub static H5P_CLS_LINK_ACCESS_ID_g: hid_t;
}

pub use self::H5P_CLS_ROOT_ID_g as H5P_ROOT;
pub use self::H5P_CLS_OBJECT_CREATE_ID_g as H5P_OBJECT_CREATE;
pub use self::H5P_CLS_FILE_CREATE_ID_g as H5P_FILE_CREATE;
pub use self::H5P_CLS_FILE_ACCESS_ID_g as H5P_FILE_ACCESS;
pub use self::H5P_CLS_DATASET_CREATE_ID_g as H5P_DATASET_CREATE;
pub use self::H5P_CLS_DATASET_ACCESS_ID_g as H5P_DATASET_ACCESS;
pub use self::H5P_CLS_DATASET_XFER_ID_g as H5P_DATASET_XFER;
pub use self::H5P_CLS_FILE_MOUNT_ID_g as H5P_FILE_MOUNT;
pub use self::H5P_CLS_GROUP_CREATE_ID_g as H5P_GROUP_CREATE;
pub use self::H5P_CLS_GROUP_ACCESS_ID_g as H5P_GROUP_ACCESS;
pub use self::H5P_CLS_DATATYPE_CREATE_ID_g as H5P_DATATYPE_CREATE;
pub use self::H5P_CLS_DATATYPE_ACCESS_ID_g as H5P_DATATYPE_ACCESS;
pub use self::H5P_CLS_STRING_CREATE_ID_g as H5P_STRING_CREATE;
pub use self::H5P_CLS_ATTRIBUTE_CREATE_ID_g as H5P_ATTRIBUTE_CREATE;
pub use self::H5P_CLS_OBJECT_COPY_ID_g as H5P_OBJECT_COPY;
pub use self::H5P_CLS_LINK_CREATE_ID_g as H5P_LINK_CREATE;
pub use self::H5P_CLS_LINK_ACCESS_ID_g as H5P_LINK_ACCESS;
