fn main() {
    println!("cargo:rerun-if-changed=c/h/");

    let bindings = bindgen::Builder::default()
        .blocklist_item(".*stdint.*")
        .use_core()
        .header("./c/h/hookapi.h")
        .clang_arg("-fvisibility=default")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
