use std::env;
use std::path::PathBuf;

extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-search=/usr/include");

    println!("cargo:rustc-link-lib=uring");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_type(
            "(io_uring_(sqe|cqe|op)|io_(sq|cq)ring_offsets|io_uring_params|io_uring_rsrc_register)",
        )
        .allowlist_var("__NR_io_uring.*")
        .formatter(bindgen::Formatter::Rustfmt)
        .impl_debug(true)
        .derive_default(true)
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
