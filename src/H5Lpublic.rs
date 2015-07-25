use libc::c_char;

use H5Ipublic::hid_t;
use H5public::{herr_t, htri_t};

extern "C" {
    pub fn H5Ldelete(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> herr_t;
    pub fn H5Lexists(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> htri_t;
}
