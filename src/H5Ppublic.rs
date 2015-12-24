use libc::{c_char, c_double, c_int, c_uint, c_void, off_t, size_t, ssize_t};

use H5ACpublic::H5AC_cache_config_t;
use H5Dpublic::{H5D_alloc_time_t, H5D_fill_time_t, H5D_fill_value_t, H5D_layout_t};
use H5FDpublic::{H5FD_file_image_callbacks_t, H5FD_mem_t};
use H5Fpublic::{H5F_close_degree_t, H5F_libver_t};
use H5Ipublic::hid_t;
use H5Lpublic::H5L_elink_traverse_t;
use H5MMpublic::{H5MM_allocate_t, H5MM_free_t};
use H5Opublic::H5O_mcdt_search_cb_t;
use H5Tpublic::{H5T_conv_except_func_t, H5T_cset_t};
use H5Zpublic::{H5Z_EDC_t, H5Z_SO_scale_type_t, H5Z_filter_func_t, H5Z_filter_t};
use H5public::{hbool_t, herr_t, htri_t, hsize_t};

pub const H5P_DEFAULT: hid_t = 0;

extern "C" {
    pub static H5P_CLS_ROOT_ID_g: hid_t;
    pub static H5P_CLS_OBJECT_CREATE_ID_g: hid_t;
    pub static H5P_CLS_FILE_CREATE_ID_g: hid_t;
    pub static H5P_CLS_FILE_ACCESS_ID_g: hid_t;
    pub static H5P_CLS_DATASET_CREATE_ID_g: hid_t;
    pub static H5P_CLS_DATASET_ACCESS_ID_g: hid_t;
    pub static H5P_CLS_DATASET_XFER_ID_g: hid_t;
    pub static H5P_CLS_FILE_MOUNT_ID_g: hid_t;
    pub static H5P_CLS_GROUP_CREATE_ID_g: hid_t;
    pub static H5P_CLS_GROUP_ACCESS_ID_g: hid_t;
    pub static H5P_CLS_DATATYPE_CREATE_ID_g: hid_t;
    pub static H5P_CLS_DATATYPE_ACCESS_ID_g: hid_t;
    pub static H5P_CLS_STRING_CREATE_ID_g: hid_t;
    pub static H5P_CLS_ATTRIBUTE_CREATE_ID_g: hid_t;
    pub static H5P_CLS_OBJECT_COPY_ID_g: hid_t;
    pub static H5P_CLS_LINK_CREATE_ID_g: hid_t;
    pub static H5P_CLS_LINK_ACCESS_ID_g: hid_t;
}

pub use self::H5P_CLS_ROOT_ID_g as H5P_ROOT;
pub use self::H5P_CLS_OBJECT_CREATE_ID_g as H5P_OBJECT_CREATE;
pub use self::H5P_CLS_FILE_CREATE_ID_g as H5P_FILE_CREATE;
pub use self::H5P_CLS_FILE_ACCESS_ID_g as H5P_FILE_ACCESS;
pub use self::H5P_CLS_DATASET_CREATE_ID_g as H5P_DATASET_CREATE;
pub use self::H5P_CLS_DATASET_ACCESS_ID_g as H5P_DATASET_ACCESS;
pub use self::H5P_CLS_DATASET_XFER_ID_g as H5P_DATASET_XFER;
pub use self::H5P_CLS_FILE_MOUNT_ID_g as H5P_FILE_MOUNT;
pub use self::H5P_CLS_GROUP_CREATE_ID_g as H5P_GROUP_CREATE;
pub use self::H5P_CLS_GROUP_ACCESS_ID_g as H5P_GROUP_ACCESS;
pub use self::H5P_CLS_DATATYPE_CREATE_ID_g as H5P_DATATYPE_CREATE;
pub use self::H5P_CLS_DATATYPE_ACCESS_ID_g as H5P_DATATYPE_ACCESS;
pub use self::H5P_CLS_STRING_CREATE_ID_g as H5P_STRING_CREATE;
pub use self::H5P_CLS_ATTRIBUTE_CREATE_ID_g as H5P_ATTRIBUTE_CREATE;
pub use self::H5P_CLS_OBJECT_COPY_ID_g as H5P_OBJECT_COPY;
pub use self::H5P_CLS_LINK_CREATE_ID_g as H5P_LINK_CREATE;
pub use self::H5P_CLS_LINK_ACCESS_ID_g as H5P_LINK_ACCESS;

pub type H5P_cls_create_func_t = extern fn(hid_t, *mut c_void) -> herr_t;
pub type H5P_cls_copy_func_t = extern fn(hid_t, hid_t, *mut c_void) -> herr_t;
pub type H5P_cls_close_func_t = extern fn(hid_t, *mut c_void) -> herr_t;

pub type H5P_prp_cb1_t = extern fn(*const c_char, size_t, *mut c_void) -> herr_t;
pub type H5P_prp_cb2_t = extern fn(hid_t, *const c_char, size_t, *mut c_void) -> herr_t;
pub type H5P_prp_create_func_t = H5P_prp_cb1_t;
pub type H5P_prp_set_func_t = H5P_prp_cb2_t;
pub type H5P_prp_get_func_t = H5P_prp_cb2_t;
pub type H5P_prp_delete_func_t = H5P_prp_cb2_t;
pub type H5P_prp_copy_func_t = H5P_prp_cb1_t;
pub type H5P_prp_compare_func_t = extern fn(*const c_void, *const c_void, size_t) -> c_int;
pub type H5P_prp_close_func_t = H5P_prp_cb1_t;

pub type H5P_iterate_t = extern fn(hid_t, *const c_char, *mut c_void) -> herr_t;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5D_mpio_actual_chunk_opt_mode_t {
    H5D_MPIO_NO_CHUNK_OPTIMIZATION = 0,
    H5D_MPIO_LINK_CHUNK,
    H5D_MPIO_MULTI_CHUNK,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum H5D_mpio_actual_io_mode_t {
    H5D_MPIO_NO_COLLECTIVE = 0x0,
    H5D_MPIO_CHUNK_INDEPENDENT = 0x1,
    H5D_MPIO_CHUNK_COLLECTIVE = 0x2,
    H5D_MPIO_CHUNK_MIXED = 0x1 | 0x2,
    H5D_MPIO_CONTIGUOUS_COLLECTIVE = 0x4,
}

extern "C" {
    pub fn H5Pcreate_class(parent: hid_t, name: *const c_char, cls_create: H5P_cls_create_func_t,
                           create_data: *const c_void, cls_copy: H5P_cls_copy_func_t,
                           copy_data: *const c_void, cls_close: H5P_cls_close_func_t,
                           close_data: *const c_void) -> hid_t;
    pub fn H5Pget_class_name(pclass_id: hid_t) -> *const c_char;
    pub fn H5Pcreate(cls_id: hid_t) -> hid_t;
    pub fn H5Pregister2(cls_id: hid_t, name: *const c_char, size: size_t, def_value: *const c_void,
                        prp_create: H5P_prp_create_func_t, prp_set: H5P_prp_set_func_t,
                        prp_get: H5P_prp_get_func_t, prp_del: H5P_prp_delete_func_t,
                        prp_copy: H5P_prp_copy_func_t, prp_cmp: H5P_prp_compare_func_t,
                        prp_close: H5P_prp_close_func_t) -> herr_t;
    pub fn H5Pinsert2(plist_id: hid_t, name: *const c_char, size: size_t, value: *const c_void,
                      prp_set: H5P_prp_set_func_t, prp_get: H5P_prp_get_func_t,
                      prp_delete: H5P_prp_delete_func_t, prp_copy: H5P_prp_copy_func_t,
                      prp_cmp: H5P_prp_compare_func_t, prp_close: H5P_prp_close_func_t) -> herr_t;
    pub fn H5Pset(plist_id: hid_t, name: *const c_char, value: *const c_void) -> herr_t;
    pub fn H5Pexist(plist_id: hid_t, name: *const c_char) -> htri_t;
    pub fn H5Pget_size(id: hid_t, name: *const c_char, size: *mut size_t) -> herr_t;
    pub fn H5Pget_nprops(id: hid_t, nprops: *mut size_t) -> herr_t;
    pub fn H5Pget_class(plist_id: hid_t) -> hid_t;
    pub fn H5Pget_class_parent(pclass_id: hid_t) -> hid_t;
    pub fn H5Pget(plist_id: hid_t, name: *const c_char, value: *mut c_void) -> herr_t;
    pub fn H5Pequal(id1: hid_t, id2: hid_t) -> htri_t;
    pub fn H5Pisa_class(plist_id: hid_t, pclass_id: hid_t) -> htri_t;
    pub fn H5Piterate(id: hid_t, idx: *mut c_int, iter_func: H5P_iterate_t, iter_data: *mut c_void)
                      -> c_int;
    pub fn H5Pcopy_prop(dst_id: hid_t, src_id: hid_t, name: *const c_char) -> herr_t;
    pub fn H5Premove(plist_id: hid_t, name: *const c_char) -> herr_t;
    pub fn H5Punregister(pclass_id: hid_t, name: *const c_char) -> herr_t;
    pub fn H5Pclose_class(plist_id: hid_t) -> herr_t;
    pub fn H5Pclose(plist_id: hid_t) -> herr_t;
    pub fn H5Pcopy(plist_id: hid_t) -> hid_t;
    pub fn H5Pset_attr_phase_change(plist_id: hid_t, max_compact: c_uint, min_dense: c_uint)
                                    -> herr_t;
    pub fn H5Pget_attr_phase_change(plist_id: hid_t, max_compact: *mut c_uint,
                                    min_dense: *mut c_uint) -> herr_t;
    pub fn H5Pset_attr_creation_order(plist_id: hid_t, crt_order_flags: c_uint) -> herr_t;
    pub fn H5Pget_attr_creation_order(plist_id: hid_t, crt_order_flags: *mut c_uint) -> herr_t;
    pub fn H5Pset_obj_track_times(plist_id: hid_t, track_times: hbool_t) -> herr_t;
    pub fn H5Pget_obj_track_times(plist_id: hid_t, track_times: *mut hbool_t) -> herr_t;
    pub fn H5Pmodify_filter(plist_id: hid_t, filter: H5Z_filter_t, flags: c_uint,
                            cd_nelmts: size_t, cd_values: *const c_uint) -> herr_t;
    pub fn H5Pset_filter(plist_id: hid_t, filter: H5Z_filter_t, flags: c_uint, cd_nelmts: size_t,
                         c_values: *const c_uint) -> herr_t;
    pub fn H5Pget_nfilters(plist_id: hid_t) -> c_int;
    pub fn H5Pget_filter2(plist_id: hid_t, filter: c_uint, flags: *mut c_uint,
                          cd_nelmts: *mut size_t, cd_values: *mut c_uint, namelen: size_t,
                          name: *mut c_char, filter_config: *mut c_uint) -> H5Z_filter_t;
    pub fn H5Pget_filter_by_id2(plist_id: hid_t, id: H5Z_filter_t, flags: *mut c_uint,
                                cd_nelmts: *mut size_t, cd_values: *mut c_uint, namelen: size_t,
                                name: *mut c_char, filter_config: *mut c_uint) -> herr_t;
    pub fn H5Pall_filters_avail(plist_id: hid_t) -> htri_t;
    pub fn H5Premove_filter(plist_id: hid_t, filter: H5Z_filter_t) -> herr_t;
    pub fn H5Pset_deflate(plist_id: hid_t, aggression: c_uint) -> herr_t;
    pub fn H5Pset_fletcher32(plist_id: hid_t) -> herr_t;
    pub fn H5Pget_version(plist_id: hid_t, boot: *mut c_uint, freelist: *mut c_uint,
                          stab: *mut c_uint, shhdr: *mut c_uint) -> herr_t;
    pub fn H5Pset_userblock(plist_id: hid_t, size: hsize_t) -> herr_t;
    pub fn H5Pget_userblock(plist_id: hid_t, size: *mut hsize_t) -> herr_t;
    pub fn H5Pset_sizes(plist_id: hid_t, sizeof_addr: size_t, sizeof_size: size_t) -> herr_t;
    pub fn H5Pget_sizes(plist_id: hid_t, sizeof_addr: *mut size_t, sizeof_size: *mut size_t)
                        -> herr_t;
    pub fn H5Pset_sym_k(plist_id: hid_t, ik: c_uint, lk: c_uint) -> herr_t;
    pub fn H5Pget_sym_k(plist_id: hid_t, ik: *mut c_uint, lk: *mut c_uint) -> herr_t;
    pub fn H5Pset_istore_k(plist_id: hid_t, ik: c_uint) -> herr_t;
    pub fn H5Pget_istore_k(plist_id: hid_t, ik: *mut c_uint) -> herr_t;
    pub fn H5Pset_shared_mesg_nindexes(plist_id: hid_t, nindexes: c_uint) -> herr_t;
    pub fn H5Pget_shared_mesg_nindexes(plist_id: hid_t, nindexes: *mut c_uint) -> herr_t;
    pub fn H5Pset_shared_mesg_index(plist_id: hid_t, index_num: c_uint, mesg_type_flags: c_uint,
                                    min_mesg_size: c_uint) -> herr_t;
    pub fn H5Pget_shared_mesg_index(plist_id: hid_t, index_num: c_uint,
                                    mesg_type_flags: *mut c_uint, min_mesg_size: *mut c_uint)
                                    -> herr_t;
    pub fn H5Pset_shared_mesg_phase_change(plist_id: hid_t, max_list: c_uint, min_btree: c_uint)
                                           -> herr_t;
    pub fn H5Pget_shared_mesg_phase_change(plist_id: hid_t, max_list: *mut c_uint,
                                           min_btree: *mut c_uint) -> herr_t;
    pub fn H5Pset_alignment(fapl_id: hid_t, threshold: hsize_t, alignment: hsize_t) -> herr_t;
    pub fn H5Pget_alignment(fapl_id: hid_t, threshold: *mut hsize_t,
                            alignment: *mut hsize_t) -> herr_t;
    pub fn H5Pset_driver(plist_id: hid_t, driver_id: hid_t, driver_info: *const c_void) -> herr_t;
    pub fn H5Pget_driver(plist_id: hid_t) -> hid_t;
    pub fn H5Pget_driver_info(plist_id: hid_t) -> *const c_void;
    pub fn H5Pset_family_offset(fapl_id: hid_t, offset: hsize_t) -> herr_t;
    pub fn H5Pget_family_offset(fapl_id: hid_t, offset: *mut hsize_t) -> herr_t;
    pub fn H5Pset_multi_type(fapl_id: hid_t, data_type: H5FD_mem_t) -> herr_t;
    pub fn H5Pget_multi_type(fapl_id: hid_t, data_type: *mut H5FD_mem_t) -> herr_t;
    pub fn H5Pset_cache(plist_id: hid_t, mdc_nelmts: c_int, rdcc_nslots: size_t,
                        rdcc_nbytes: size_t, rdcc_w0: c_double) -> herr_t;
    pub fn H5Pget_cache(plist_id: hid_t, mdc_nelmts: *mut c_int, rdcc_nslots: *mut size_t,
                        rdcc_nbytes: *mut size_t, rdcc_w0: *mut c_double) -> herr_t;
    pub fn H5Pset_mdc_config(plist_id: hid_t, config_ptr: *mut H5AC_cache_config_t) -> herr_t;
    pub fn H5Pget_mdc_config(plist_id: hid_t, config_ptr: *const H5AC_cache_config_t) -> herr_t;
    pub fn H5Pset_gc_references(fapl_id: hid_t, gc_ref: c_uint) -> herr_t;
    pub fn H5Pget_gc_references(fapl_id: hid_t, gc_ref: *mut c_uint) -> herr_t;
    pub fn H5Pset_fclose_degree(fapl_id: hid_t, degree: H5F_close_degree_t) -> herr_t;
    pub fn H5Pget_fclose_degree(fapl_id: hid_t, degree: *mut H5F_close_degree_t) -> herr_t;
    pub fn H5Pset_meta_block_size(fapl_id: hid_t, size: hsize_t) -> herr_t;
    pub fn H5Pget_meta_block_size(fapl_id: hid_t, size: *mut hsize_t) -> herr_t;
    pub fn H5Pset_sieve_buf_size(fapl_id: hid_t, size: size_t) -> herr_t;
    pub fn H5Pget_sieve_buf_size(fapl_id: hid_t, size: *mut size_t) -> herr_t;
    pub fn H5Pset_small_data_block_size(fapl_id: hid_t, size: hsize_t) -> herr_t;
    pub fn H5Pget_small_data_block_size(fapl_id: hid_t, size: *mut hsize_t) -> herr_t;
    pub fn H5Pset_libver_bounds(plist_id: hid_t, low: H5F_libver_t, high: H5F_libver_t) -> herr_t;
    pub fn H5Pget_libver_bounds(plist_id: hid_t, low: *mut H5F_libver_t, high: *mut H5F_libver_t)
                                -> herr_t;
    pub fn H5Pset_elink_file_cache_size(plist_id: hid_t, efc_size: c_uint) -> herr_t;
    pub fn H5Pget_elink_file_cache_size(plist_id: hid_t, efc_size: *mut c_uint) -> herr_t;
    pub fn H5Pset_file_image(fapl_id: hid_t, buf_ptr: *const c_void, buf_len: size_t) -> herr_t;
    pub fn H5Pget_file_image(fapl_id: hid_t, buf_ptr_ptr: *mut *const c_void,
                             buf_len_ptr: *mut size_t) -> herr_t;
    pub fn H5Pset_file_image_callbacks(fapl_id: hid_t,
                                       callbacks_ptr: *const H5FD_file_image_callbacks_t)
                                       -> herr_t;
    pub fn H5Pget_file_image_callbacks(fapl_id: hid_t,
                                       callbacks_ptr: *mut H5FD_file_image_callbacks_t) -> herr_t;
    pub fn H5Pset_core_write_tracking(fapl_id: hid_t, is_enabled: hbool_t, page_size: size_t)
                                      -> herr_t;
    pub fn H5Pget_core_write_tracking(fapl_id: hid_t, is_enabled: *mut hbool_t,
                                      page_size: *mut size_t) -> herr_t;
    pub fn H5Pset_layout(plist_id: hid_t, layout: H5D_layout_t) -> herr_t;
    pub fn H5Pget_layout(plist_id: hid_t) -> H5D_layout_t;
    pub fn H5Pset_chunk(plist_id: hid_t, ndims: c_int, dim: *const hsize_t) -> herr_t;
    pub fn H5Pget_chunk(plist_id: hid_t, max_ndims: c_int, dim: *mut hsize_t) -> c_int;
    pub fn H5Pset_external(plist_id: hid_t, name: *const c_char, offset: off_t, size: hsize_t)
                           -> herr_t;
    pub fn H5Pget_external_count(plist_id: hid_t) -> c_int;
    pub fn H5Pget_external(plist_id: hid_t, idx: c_uint, name_size: size_t, name: *mut c_char,
                           offset: *mut off_t, size: *mut hsize_t) -> herr_t;
    pub fn H5Pset_szip(plist_id: hid_t, options_mask: c_uint, pixels_per_block: c_uint) -> herr_t;
    pub fn H5Pset_shuffle(plist_id: hid_t) -> herr_t;
    pub fn H5Pset_nbit(plist_id: hid_t) -> herr_t;
    pub fn H5Pset_scaleoffset(plist_id: hid_t, scale_type: H5Z_SO_scale_type_t,
                              scale_factor: c_int) -> herr_t;
    pub fn H5Pset_fill_value(plist_id: hid_t, type_id: hid_t, value: *const c_void) -> herr_t;
    pub fn H5Pget_fill_value(plist_id: hid_t, type_id: hid_t, value: *mut c_void) -> herr_t;
    pub fn H5Pfill_value_defined(plist: hid_t, status: *mut H5D_fill_value_t) -> herr_t;
    pub fn H5Pset_alloc_time(plist_id: hid_t, alloc_time: H5D_alloc_time_t) -> herr_t;
    pub fn H5Pget_alloc_time(plist_id: hid_t, alloc_time: *mut H5D_alloc_time_t) -> herr_t;
    pub fn H5Pset_fill_time(plist_id: hid_t, fill_time: H5D_fill_time_t) -> herr_t;
    pub fn H5Pget_fill_time(plist_id: hid_t, fill_time: *mut H5D_fill_time_t) -> herr_t;
    pub fn H5Pset_chunk_cache(dapl_id: hid_t, rdcc_nslots: size_t, rdcc_nbytes: size_t,
                              rdcc_w0: c_double) -> herr_t;
    pub fn H5Pget_chunk_cache(dapl_id: hid_t, rdcc_nslots: *mut size_t, rdcc_nbytes: *mut size_t,
                              rdcc_w0: *mut c_double) -> herr_t;
    pub fn H5Pset_data_transform(plist_id: hid_t, expression: *const c_char) -> herr_t;
    pub fn H5Pget_data_transform(plist_id: hid_t, expression: *mut c_char, size: size_t)
                                 -> ssize_t;
    pub fn H5Pset_buffer(plist_id: hid_t, size: size_t, tconv: *const c_void, bkg: *const c_void)
                         -> herr_t;
    pub fn H5Pget_buffer(plist_id: hid_t, tconv: *mut *const c_void, bkg: *mut *const c_void)
                         -> size_t;
    pub fn H5Pset_preserve(plist_id: hid_t, status: hbool_t) -> herr_t;
    pub fn H5Pget_preserve(plist_id: hid_t) -> c_int;
    pub fn H5Pset_edc_check(plist_id: hid_t, check: H5Z_EDC_t) -> herr_t;
    pub fn H5Pget_edc_check(plist_id: hid_t) -> H5Z_EDC_t;
    pub fn H5Pset_filter_callback(plist_id: hid_t, func: H5Z_filter_func_t, op_data: *const c_void) -> herr_t;
    pub fn H5Pset_btree_ratios(plist_id: hid_t, left: c_double, middle: c_double, right: c_double)
                               -> herr_t;
    pub fn H5Pget_btree_ratios(plist_id: hid_t, left: *mut c_double, middle: *mut c_double,
                               right: *mut c_double) -> herr_t;
    pub fn H5Pset_vlen_mem_manager(plist_id: hid_t, alloc_func: H5MM_allocate_t,
                                   alloc_info: *const c_void, free_func: H5MM_free_t,
                                   free_info: *const c_void) -> herr_t;
    pub fn H5Pget_vlen_mem_manager(plist_id: hid_t,alloc_func: *mut H5MM_allocate_t,
                                   alloc_info: *mut *const c_void, free_func: *mut H5MM_free_t,
                                   free_info: *mut *const c_void) -> herr_t;
    pub fn H5Pset_hyper_vector_size(fapl_id: hid_t, size: size_t) -> herr_t;
    pub fn H5Pget_hyper_vector_size(fapl_id: hid_t, size: *mut size_t) -> herr_t;
    pub fn H5Pset_type_conv_cb(dxpl_id: hid_t, op: H5T_conv_except_func_t,
                               operate_data: *const c_void) -> herr_t;
    pub fn H5Pget_type_conv_cb(dxpl_id: hid_t, op: *mut H5T_conv_except_func_t,
                               operate_data: *mut *const c_void) -> herr_t;
    pub fn H5Pget_mpio_actual_chunk_opt_mode(plist_id: hid_t, actual_chunk_opt_mode: *mut H5D_mpio_actual_chunk_opt_mode_t) -> herr_t;
    pub fn H5Pget_mpio_actual_io_mode(plist_id: hid_t,
                                      actual_io_mode: *mut H5D_mpio_actual_io_mode_t) -> herr_t;
    pub fn H5Pget_mpio_no_collective_cause(plist_id: hid_t, local_no_collective_cause: *mut u32,
                                           global_no_collective_cause: *mut u32) -> herr_t;
    pub fn H5Pset_create_intermediate_group(plist_id: hid_t, crt_intmd: c_uint) -> herr_t;
    pub fn H5Pget_create_intermediate_group(plist_id: hid_t, crt_intmd: *mut c_uint) -> herr_t;
    pub fn H5Pset_local_heap_size_hint(plist_id: hid_t, size_hint: size_t) -> herr_t;
    pub fn H5Pget_local_heap_size_hint(plist_id: hid_t, size_hint: *mut size_t) -> herr_t;
    pub fn H5Pset_link_phase_change(plist_id: hid_t, max_compact: c_uint, min_dense: c_uint)
                                    -> herr_t;
    pub fn H5Pget_link_phase_change(plist_id: hid_t, max_compact: *mut c_uint,
                                    min_dense: *mut c_uint) -> herr_t;
    pub fn H5Pset_est_link_info(plist_id: hid_t, est_num_entries: c_uint, est_name_len: c_uint)
                                -> herr_t;
    pub fn H5Pget_est_link_info(plist_id: hid_t, est_num_entries: *mut c_uint,
                                est_name_len: *mut c_uint) -> herr_t;
    pub fn H5Pset_link_creation_order(plist_id: hid_t, crt_order_flags: c_uint) -> herr_t;
    pub fn H5Pget_link_creation_order(plist_id: hid_t, crt_order_flags: *mut c_uint) -> herr_t;
    pub fn H5Pset_char_encoding(plist_id: hid_t, encoding: H5T_cset_t) -> herr_t;
    pub fn H5Pget_char_encoding(plist_id: hid_t, encoding: *mut H5T_cset_t) -> herr_t;
    pub fn H5Pset_nlinks(plist_id: hid_t, nlinks: size_t) -> herr_t;
    pub fn H5Pget_nlinks(plist_id: hid_t, nlinks: *mut size_t) -> herr_t;
    pub fn H5Pset_elink_prefix(plist_id: hid_t, prefix: *const c_char) -> herr_t;
    pub fn H5Pget_elink_prefix(plist_id: hid_t, prefix: *mut c_char, size: size_t) -> ssize_t;
    pub fn H5Pget_elink_fapl(lapl_id: hid_t) -> hid_t;
    pub fn H5Pset_elink_fapl(lapl_id: hid_t, fapl_id: hid_t) -> herr_t;
    pub fn H5Pset_elink_acc_flags(lapl_id: hid_t, flags: c_uint) -> herr_t;
    pub fn H5Pget_elink_acc_flags(lapl_id: hid_t, flags: *mut c_uint) -> herr_t;
    pub fn H5Pset_elink_cb(lapl_id: hid_t, func: H5L_elink_traverse_t, op_data: *const c_void)
                           -> herr_t;
    pub fn H5Pget_elink_cb(lapl_id: hid_t, func: *mut H5L_elink_traverse_t,
                           op_data: *mut *const c_void) -> herr_t;
    pub fn H5Pset_copy_object(plist_id: hid_t, crt_intmd: c_uint) -> herr_t;
    pub fn H5Pget_copy_object(plist_id: hid_t, crt_intmd: *mut c_uint) -> herr_t;
    pub fn H5Padd_merge_committed_dtype_path(plist_id: hid_t, path: *const c_char) -> herr_t;
    pub fn H5Pfree_merge_committed_dtype_paths(plist_id: hid_t) -> herr_t;
    pub fn H5Pset_mcdt_search_cb(plist_id: hid_t, func: H5O_mcdt_search_cb_t,
                                 op_data: *const c_void) -> herr_t;
    pub fn H5Pget_mcdt_search_cb(plist_id: hid_t, func: *mut H5O_mcdt_search_cb_t,
                                 op_data: *mut *const c_void) -> herr_t;
}
