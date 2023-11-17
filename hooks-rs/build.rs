use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=c/");

    let bindings = bindgen::Builder::default()
        .blocklist_item(".*stdint.*")
        .use_core()
        .header("./c/hookapi.h")
        .clang_arg("-fvisibility=default")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
