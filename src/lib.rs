extern crate bitflags;
extern crate bytemuck;
extern crate thiserror;

//#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
extern crate liburing_sys as ffi;

//#[cfg(any(target_os = "android", target_os = "freebsd", target_os = "linux"))]
mod io_uring;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
