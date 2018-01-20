extern crate cc;

fn main() {
    use std::env;
    use std::path::Path;

    let profile = env::var("PROFILE").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let trace_ffi_path = manifest_path.parent().unwrap().join("trace-ffi").join("target").join(profile.clone());

    eprintln!("Building {} profile in {}", profile, manifest_path.to_str().unwrap());

    cc::Build::new()
        .file("src/example.c")
        .compile("libc_example.a");
    //println!("cargo:rustc-link-search=native={}", trace_ffi_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=libxi_trace_ffi.a");
}
