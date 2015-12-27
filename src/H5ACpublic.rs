use libc::{c_char, c_double, c_int, c_long, size_t};

use H5Cpublic::{H5C_cache_decr_mode, H5C_cache_flash_incr_mode, H5C_cache_incr_mode};
use H5public::hbool_t;

#[repr(C)]
pub struct H5AC_cache_config_t {
    pub version: c_int,
    pub rpt_fcn_enabled: hbool_t,
    pub open_trace_file: hbool_t,
    pub close_trace_file: hbool_t,
    pub trace_file_name: [c_char; 1025],
    pub evictions_enabled: hbool_t,
    pub set_initial_size: hbool_t,
    pub initial_size: size_t,
    pub min_clean_fraction: c_double,
    pub max_size: size_t,
    pub min_size: size_t,
    pub epoch_length: c_long,
    pub incr_mode: H5C_cache_incr_mode,
    pub lower_hr_threshold: c_double,
    pub increment: c_double,
    pub apply_max_increment: hbool_t,
    pub max_increment: size_t,
    pub flash_incr_mode: H5C_cache_flash_incr_mode,
    pub flash_multiple: c_double,
    pub flash_threshold: c_double,
    pub decr_mode: H5C_cache_decr_mode,
    pub upper_hr_threshold: c_double,
    pub decrement: c_double,
    pub apply_max_decrement: hbool_t,
    pub max_decrement: size_t,
    pub epochs_before_eviction: c_int,
    pub apply_empty_reserve: hbool_t,
    pub empty_reserve: c_double,
    pub dirty_bytes_threshold: c_int,
    pub metadata_write_strategy: c_int,
}

impl H5AC_cache_config_t {
    pub fn new() -> H5AC_cache_config_t {
        H5AC_cache_config_t {
            version: 0,
            rpt_fcn_enabled: 0,
            open_trace_file: 0,
            close_trace_file: 0,
            trace_file_name: [0 as c_char; 1025],
            evictions_enabled: 0,
            set_initial_size: 0,
            initial_size: 0,
            min_clean_fraction: 0.0,
            max_size: 0,
            min_size: 0,
            epoch_length: 0,
            incr_mode: H5C_cache_incr_mode::default(),
            lower_hr_threshold: 0.0,
            increment: 0.0,
            apply_max_increment: 0,
            max_increment: 0,
            flash_incr_mode: H5C_cache_flash_incr_mode::default(),
            flash_multiple: 0.0,
            flash_threshold: 0.0,
            decr_mode: H5C_cache_decr_mode::default(),
            upper_hr_threshold: 0.0,
            decrement: 0.0,
            apply_max_decrement: 0,
            max_decrement: 0,
            epochs_before_eviction: 0,
            apply_empty_reserve: 0,
            empty_reserve: 0.0,
            dirty_bytes_threshold: 0,
            metadata_write_strategy: 0,

        }
    }
}
