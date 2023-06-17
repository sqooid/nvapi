use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=./include/bin");
    if std::env::var("TARGET")
        .expect("Get build target CPU architecture")
        .starts_with("x86_64")
    {
        println!("cargo:rustc-link-lib=static=nvapi64");
    } else {
        println!("cargo:rustc-link-lib=static=nvapi");
    }
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
    bindings
        .write_to_file("./src/bindings.rs")
        .expect("Unable to write bindings");
}
