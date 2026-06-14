use std::os::raw::{c_int, c_longlong, c_void};

unsafe extern "C" {
    pub fn bshuf_using_AVX2() -> c_int;
    pub fn bshuf_using_SSE2() -> c_int;

    pub fn bshuf_compress_lz4(
        in_: *const c_void,
        out: *mut c_void,
        size: usize,
        elem_size: usize,
        block_size: usize,
    ) -> c_longlong;

    pub fn bshuf_decompress_lz4(
        in_: *const c_void,
        out: *mut c_void,
        size: usize,
        elem_size: usize,
        block_size: usize,
    ) -> c_longlong;

    pub fn bshuf_compress_lz4_bound(size: usize, elem_size: usize, block_size: usize) -> usize;
}
