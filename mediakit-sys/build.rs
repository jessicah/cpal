extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
        // Tell cargo to tell rustc to link the system libmedia
        // shared libary.
        println!("cargo:rustc-link-lib=dylib=media");

        // Tell cargo to invalidate the built crate whenever the
        // wrapper changes.
        println!("cargo:rerun-if-changed=mediakit-link/wrapper.h");

        // The bindgen::Builder is the main entry point to
        // bindgen, and lets you build up options for the
        // resulting bindings.
        let bindings = bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header("mediakit-link/wrapper.h")
                // Tell cargo to invalidate the built create
                // whenever any of the included header files
                // changed.
                .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                // Adapted from asio binding build script...
                .clang_arg("-x")
                .clang_arg("c++")
                // C++ stdlib headers missing, so add
                .clang_arg("-I")
                .clang_arg("/system/develop/tools/lib/gcc/x86_64-unknown-haiku/8.3.0/include/c++")
                .clang_arg("-I")
                .clang_arg("/system/develop/tools/lib/gcc/x86_64-unknown-haiku/8.3.0/include/c++/x86_64-unknown-haiku")
                // Allow specific types, etc.
                .whitelist_type("BMediaRoster")
                .whitelist_type("BBuffer")
                .whitelist_type("BMediaNode")
                .whitelist_type("media_node")
                .whitelist_type("media_format")
                .whitelist_type("media_output")
                // Finish the builder and generate the bindings.
                .generate()
                // Unwrap the Result and panic on failure.
                .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
}
