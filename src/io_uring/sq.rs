use std::sync::atomic::AtomicU32;

pub struct Sq {
    khead: *mut AtomicU32,
    ktail: *mut AtomicU32,
    kflags: *const u32,
    kdropped: *const u32,
    array: &'static mut [AtomicU32],
    sqes: &'static mut [ffi::io_uring_sqe],
    sqe_head: u32,
    sqe_tail: u32,
    ring_sz: usize,
    ring_ptr: *const libc::c_void,
    ring_mask: u32,
    ring_entries: u32,
}
