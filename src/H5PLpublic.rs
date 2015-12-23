use libc::c_int;
use H5public::herr_t;

extern "C" {
    pub fn H5PLset_loading_state(plugin_flags: c_int) -> herr_t;
    pub fn H5PLget_loading_state(plugin_flags: *mut c_int) -> herr_t;
}
