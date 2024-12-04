use std::arch::asm;
use std::convert::TryFrom;
use std::ffi::{c_long, c_uint, c_ulong, c_void};
use std::io;
use std::num::Wrapping;
use std::ops::Neg;

use bitflags::bitflags;

mod cq;
mod opcode;
mod sq;
mod uring;

const MAXERRNO: usize = 4095;

bitflags! {
    struct SetupFlags: u8 {

    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16],
}

impl Default for __sigset_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

// https://man7.org/linux/man-pages/man2/io_uring_enter.2.html
#[allow(dead_code)]
#[inline]
pub(crate) unsafe fn enter2(
    fd: c_uint,
    to_submit: c_uint,
    min_complete: c_uint,
    flags: c_uint,
    sig: *mut __sigset_t,
) -> io::Result<i32> {
    let ret: usize;
    let size = core::mem::size_of::<__sigset_t>();
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!(
            "syscall",
            in("rax") ffi::__NR_io_uring_enter,
            in("rdi") i64::from(fd),
            in("rsi") i64::from(to_submit),
            in("rdx") i64::from(min_complete),
            in("r10") i64::from(flags),
            in("r9") sig as c_long,
            in("r8") size,
            lateout("rax") ret,
            clobber_abi("C"),
        );

        #[cfg(all(any(target_arch = "aarch64", target_arch = "arm64")))]
        asm!(
            "svc #0",
            in("x8") ffi::__NR_io_uring_enter,
            in("x0") i64::from(fd),
            in("x1") i64::from(to_submit),
            in("x2") i64::from(min_complete),
            in("x3") i64::from(flags),
            in("x4") sig as c_long,
            in("x5") size,
            lateout("x0") ret,
            clobber_abi("C"),
        );
    }

    check_err(ret)
}

// https://man7.org/linux/man-pages/man2/io_uring_register.2.html
#[allow(dead_code)]
#[inline]
pub(crate) unsafe fn register(
    fd: c_uint,
    opcode: c_uint,
    mut arg: *mut ffi::io_uring_rsrc_register,
) -> io::Result<i32> {
    let ret: usize;
    let size = core::mem::size_of::<ffi::io_uring_rsrc_register>();
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!(
            "syscall",
            in("rax") ffi::__NR_io_uring_register,
            in("rdi") i64::from(fd),
            in("rsi") i64::from(opcode),
            in("rdx") &mut arg as *mut _ as *mut c_void,
            in("r10") size as c_long,
            lateout("rax") ret,
            clobber_abi("C"),
        );

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64"))]
        asm!(
            "svc #0",
            in("x8") ffi::__NR_io_uring_register,
            in("x0") i64::from(fd),
            in("x1") i64::from(opcode),
            in("x2") &mut arg as *mut _ as *mut c_void,
            in("x3") size as c_long,
            lateout("x0") ret,
            clobber_abi("C"),
        );
    }

    check_err(ret)
}

// https://man7.org/linux/man-pages/man2/io_uring_setup.2.html
#[allow(dead_code)]
#[inline]
pub(crate) unsafe fn setup(entries: u32, params: *mut ffi::io_uring_params) -> io::Result<i32> {
    let ret: usize;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!(
            "syscall",
            in("rax") ffi::__NR_io_uring_setup,
            in("rdi") entries,
            in("rsi") params as c_long,
            lateout("rax") ret,
            clobber_abi("C"),
        );

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64"))]
        asm!(
            "svc #0",
            in("x8") ffi::__NR_io_uring_setup,
            in("x0") entries,
            in("x1") params as c_long,
            lateout("x0") ret,
            clobber_abi("C"),
        );
    }

    check_err(ret)
}

#[allow(dead_code)]
#[inline]
fn check_err(ret: usize) -> io::Result<i32> {
    let wrapped = Wrapping::<usize>(MAXERRNO);
    assert!(ret >= (wrapped.neg().0));
    if ret > 0 {
        let err = io::Error::last_os_error();
        if let Some(12) = err.raw_os_error() {
            // TODO: return specific err on memlock rlimit
        }

        return Err(err);
    }

    Ok(i32::try_from(ret).unwrap())
}
