#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

macro_rules! enum_default {
    ($name:ty, $default:expr) => (
        #[inline]
        impl Default for $name {
            fn default() -> $name { $default }
        }
    )
}

macro_rules! new_as_default {
    ($name:ident) => (
        impl $name {
            #[inline]
            pub fn new() -> $name {
                Default::default()
            }
        }
    )
}

mod H5ACpublic;
mod H5Apublic;
mod H5Cpublic;
mod H5Dpublic;
mod H5Epublic;
mod H5FDpublic;
mod H5Fpublic;
mod H5Gpublic;
mod H5Ipublic;
mod H5Lpublic;
mod H5MMpublic;
mod H5Opublic;
mod H5PLpublic;
mod H5Ppublic;
mod H5Rpublic;
mod H5Spublic;
mod H5Tpublic;
mod H5Zpublic;
mod H5public;

pub use H5ACpublic::*;
pub use H5Apublic::*;
pub use H5Cpublic::*;
pub use H5Dpublic::*;
pub use H5Epublic::*;
pub use H5FDpublic::*;
pub use H5Fpublic::*;
pub use H5Gpublic::*;
pub use H5Ipublic::*;
pub use H5Lpublic::*;
pub use H5MMpublic::*;
pub use H5Opublic::*;
pub use H5PLpublic::*;
pub use H5Ppublic::*;
pub use H5Rpublic::*;
pub use H5Spublic::*;
pub use H5Tpublic::*;
pub use H5Zpublic::*;
pub use H5public::*;

#[cfg(test)]
mod tests {
    #[test]
    fn link() {
        let (mut majnum, mut minnum, mut relnum) = (0, 0, 0);
        assert!(unsafe { ::H5get_libversion(&mut majnum, &mut minnum, &mut relnum) } >= 0);
        assert_eq!((majnum, minnum), (1, 8));
    }
}
