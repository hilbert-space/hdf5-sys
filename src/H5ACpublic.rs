use libc::{c_char, c_int, c_long, c_double, size_t};
use H5public::hbool_t;
use H5Cpublic::{H5C_cache_incr_mode, H5C_cache_decr_mode, H5C_cache_flash_incr_mode};

#[repr(C)]
pub struct H5AC_cache_config_t {
    version: c_int,
    rpt_fcn_enabled: hbool_t,
    open_trace_file: hbool_t,
    close_trace_file: hbool_t,
    trace_file_name: [c_char; 1025],
    evictions_enabled: hbool_t,
    set_initial_size: hbool_t,
    initial_size: size_t,
    min_clean_fraction: c_double,
    max_size: size_t,
    min_size: size_t,
    epoch_length: c_long,
    incr_mode: H5C_cache_incr_mode,
    lower_hr_threshold: c_double,
    increment: c_double,
    apply_max_increment: hbool_t,
    max_increment: size_t,
    flash_incr_mode: H5C_cache_flash_incr_mode,
    flash_multiple: c_double,
    flash_threshold: c_double,
    decr_mode: H5C_cache_decr_mode,
    upper_hr_threshold: c_double,
    decrement: c_double,
    apply_max_decrement: hbool_t,
    max_decrement: size_t,
    epochs_before_eviction: c_int,
    apply_empty_reserve: hbool_t,
    empty_reserve: c_double,
    dirty_bytes_threshold: c_int,
    metadata_write_strategy: c_int,
}
