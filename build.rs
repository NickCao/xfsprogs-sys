fn main() {
    cc::Build::new().file("xfsprogs.c").compile("xfsprogs");

    println!("cargo:rustc-link-lib=xfsprogs");
    println!("cargo:rerun-if-changed=xfsprogs.h");

    let bindings = bindgen::Builder::default()
        .header("xfsprogs.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
