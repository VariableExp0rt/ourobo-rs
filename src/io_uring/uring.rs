use super::{cq::Cq, sq::Sq};
use std::ffi::{c_int, c_uint};
use std::sync::Mutex;

pub struct Uring {
    sq: Mutex<Sq>,
    cq: Cq,
    flags: c_uint,
    ring_fd: c_int,
    features: c_uint,
    enter_ring_fd: c_int,
}
