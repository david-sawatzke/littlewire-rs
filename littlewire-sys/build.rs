extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // First create the bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rerun-if-changed=wrapper.h");

    // Then add the library
    cc::Build::new()
        .define(
            if cfg!(unix) {
                "LINUX"
            } else if cfg!(windows) {
                "WIN"
            } else {
                panic!("Unsupported os");
            },
            None,
        )
        .include("library")
        .warnings(false)
        .file("library/littleWire.c")
        .file("library/littleWire_servo.c")
        .file("library/littleWire_util.c")
        .file("library/opendevice.c")
        .compile("littlewire-c");

    if cfg!(unix) {
        println!("cargo:rustc-link-lib=dylib=usb");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
