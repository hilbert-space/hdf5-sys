#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

mod H5Dpublic;
mod H5Fpublic;
mod H5Ipublic;
mod H5Lpublic;
mod H5Ppublic;
mod H5Spublic;
mod H5Tpublic;
mod H5public;

pub use H5Dpublic::*;
pub use H5Fpublic::*;
pub use H5Ipublic::*;
pub use H5Lpublic::*;
pub use H5Ppublic::*;
pub use H5Spublic::*;
pub use H5Tpublic::*;
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
