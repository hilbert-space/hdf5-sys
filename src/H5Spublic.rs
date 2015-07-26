use libc::c_int;

use H5Ipublic::hid_t;
use H5public::{herr_t, hsize_t};

pub const H5S_ALL: hid_t = 0;

extern "C" {
    pub fn H5Screate_simple(rank: c_int, current_dims: *const hsize_t,
                            maximum_dims: *const hsize_t) -> hid_t;

    pub fn H5Sclose(space_id: hid_t) -> herr_t;
}
