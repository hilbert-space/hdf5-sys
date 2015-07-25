#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_uint};

pub type herr_t = c_int;

extern "C" {
    pub fn H5get_libversion(majnum: *mut c_uint, minnum: *mut c_uint, relnum: *mut c_uint)
                            -> herr_t;
}

#[cfg(test)]
mod tests {
    #[test]
    fn linking() {
        let (mut majnum, mut minnum, mut relnum) = (0, 0, 0);
        assert!(unsafe { super::H5get_libversion(&mut majnum, &mut minnum, &mut relnum) } >= 0);
        assert_eq!((majnum, minnum, relnum), (1, 8, 15));
    }
}
