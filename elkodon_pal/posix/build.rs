extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=acl");
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    println!("cargo:rustc-link-lib=pthread");

    println!("cargo:rerun-if-changed=src/posix.h");

    let bindings = bindgen::Builder::default()
        .header("src/c/posix.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("posix_generated.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/c/scandir.c")
        .compile("libscandir.a");

    cc::Build::new()
        .file("src/c/socket_macros.c")
        .compile("libsocket_macros.a");
}
