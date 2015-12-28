use libc::{c_int, c_uchar, size_t};

use H5Ipublic::hid_t;
use H5public::{herr_t, hsize_t, hssize_t, htri_t};

pub const H5S_ALL: hid_t = 0;
pub const H5S_UNLIMITED: hsize_t = (-1 as hssize_t) as hsize_t;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5S_class_t {
    H5S_NO_CLASS = -1,
    H5S_SCALAR = 0,
    H5S_SIMPLE = 1,
    H5S_NULL = 2,
}
pub use self::H5S_class_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5S_seloper_t {
    H5S_SELECT_NOOP = -1,
    H5S_SELECT_SET = 0,
    H5S_SELECT_OR,
    H5S_SELECT_AND,
    H5S_SELECT_XOR,
    H5S_SELECT_NOTB,
    H5S_SELECT_NOTA,
    H5S_SELECT_APPEND,
    H5S_SELECT_PREPEND,
    H5S_SELECT_INVALID,
}
pub use self::H5S_seloper_t::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5S_sel_type {
    H5S_SEL_ERROR = -1,
    H5S_SEL_NONE = 0,
    H5S_SEL_POINTS = 1,
    H5S_SEL_HYPERSLABS = 2,
    H5S_SEL_ALL = 3,
    H5S_SEL_N,
}
pub use self::H5S_sel_type::*;

extern "C" {
    pub fn H5Screate(dataspace_type: H5S_class_t) -> hid_t;
    pub fn H5Scopy(space_id: hid_t) -> hid_t;
    pub fn H5Sclose(space_id: hid_t) -> herr_t;
    pub fn H5Sdecode(buf: *const c_uchar) -> hid_t;
    pub fn H5Sencode(obj_id: hid_t, buf: *mut c_uchar, nalloc: *mut size_t) -> herr_t;
    pub fn H5Screate_simple(rank: c_int, current_dims: *const hsize_t,
                            maximum_dims: *const hsize_t) -> hid_t;
    pub fn H5Sis_simple(space_id: hid_t) -> htri_t;
    pub fn H5Soffset_simple(space_id: hid_t, offset: *const hssize_t) -> herr_t;
    pub fn H5Sget_simple_extent_dims(space_id: hid_t, dims: *mut hsize_t, maxdims: *mut hsize_t)
                                     -> c_int;
    pub fn H5Sget_simple_extent_ndims(space_id: hid_t) -> c_int;
    pub fn H5Sget_simple_extent_npoints(space_id: hid_t) -> hssize_t;
    pub fn H5Sget_simple_extent_type(space_id: hid_t) -> H5S_class_t;
    pub fn H5Sextent_copy(dest_space_id: hid_t, source_space_id: hid_t) -> herr_t;
    pub fn H5Sextent_equal(space1_id: hid_t, space2_id: hid_t) -> htri_t;
    pub fn H5Sset_extent_simple(space_id: hid_t, rank: c_int, dims: *const hsize_t,
                                max: *const hsize_t) -> herr_t;
    pub fn H5Sset_extent_none(space_id: hid_t) -> herr_t;
    pub fn H5Sget_select_type(space_id: hid_t) -> H5S_sel_type;
    pub fn H5Sget_select_npoints(space_id: hid_t) -> hssize_t;
    pub fn H5Sget_select_hyper_nblocks(space_id: hid_t) -> hssize_t;
    pub fn H5Sget_select_hyper_blocklist(space_id: hid_t, startblock: hsize_t,
                                         numblocks: hsize_t, buf: *mut hsize_t) -> herr_t;
    pub fn H5Sget_select_elem_npoints(space_id: hid_t) -> hssize_t;
    pub fn H5Sget_select_elem_pointlist(space_id: hid_t, startpoint: hsize_t,
                                        numpoints: hsize_t, buf: *mut hsize_t) -> herr_t;
    pub fn H5Sget_select_bounds(space_id: hid_t, start: *mut hsize_t, end: *mut hsize_t) -> herr_t;
    pub fn H5Sselect_elements(space_id: hid_t, op: H5S_seloper_t, num_elements: size_t,
                              coord: *const hsize_t) -> herr_t;
    pub fn H5Sselect_all(dspace_id: hid_t) -> herr_t;
    pub fn H5Sselect_none(space_id: hid_t) -> herr_t;
    pub fn H5Sselect_valid(space_id: hid_t) -> htri_t;
    pub fn H5Sselect_hyperslab(space_id: hid_t, op: H5S_seloper_t, start: *const hsize_t,
                               _stride: *const hsize_t, count: *const hsize_t,
                               _block: *const hsize_t) -> herr_t;
}
