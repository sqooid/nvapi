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
}
