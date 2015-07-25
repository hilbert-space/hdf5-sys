#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

mod H5Fpublic;
mod H5Ipublic;
mod H5Ppublic;
mod H5public;

pub use H5Fpublic::*;
pub use H5Ipublic::*;
pub use H5Ppublic::*;
pub use H5public::*;

#[cfg(test)]
mod tests {
    #[test]
    fn link() {
        let (mut majnum, mut minnum, mut relnum) = (0, 0, 0);
        assert!(unsafe { ::H5get_libversion(&mut majnum, &mut minnum, &mut relnum) } >= 0);
        assert_eq!((majnum, minnum, relnum), (1, 8, 15));
    }
}
