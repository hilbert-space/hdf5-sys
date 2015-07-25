use libc::{c_int, c_uint};

pub type herr_t = c_int;

extern "C" {
    pub fn H5get_libversion(majnum: *mut c_uint, minnum: *mut c_uint, relnum: *mut c_uint)
                            -> herr_t;
}
