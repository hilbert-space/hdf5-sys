use libc::c_int;

use H5Ipublic::hid_t;
use H5public::{herr_t, hsize_t, hssize_t};

pub const H5S_ALL: hid_t = 0;
pub const H5S_UNLIMITED: hsize_t = (-1 as hssize_t) as hsize_t;

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

extern "C" {
    pub fn H5Screate_simple(rank: c_int, current_dims: *const hsize_t,
                            maximum_dims: *const hsize_t) -> hid_t;

    pub fn H5Sset_extent_simple(space_id: hid_t, rank: c_int, dims: *const hsize_t,
                                max: *const hsize_t) -> herr_t;

    pub fn H5Sclose(space_id: hid_t) -> herr_t;

    pub fn H5Sselect_hyperslab(space_id: hid_t, op: H5S_seloper_t, start: *const hsize_t,
                               _stride: *const hsize_t, count: *const hsize_t,
                               _block: *const hsize_t) -> herr_t;
}
