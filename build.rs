use std::path::PathBuf;

extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-search=./include/bin");
    println!("cargo:rustc-link-lib=nvapi64");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_default(true)
        .generate()
        .expect("Generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Write bindings");
}
