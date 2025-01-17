use std::env;
use std::path::PathBuf;
use std::process::Command;

use bindgen::{EnumVariation, NonCopyUnionStyle};

fn main() {
    let cur = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let out = &env::var("OUT_DIR").unwrap();
    Command::new("cp")
        .arg("-R")
        .arg(concat(cur, "liburing"))
        .arg(concat(out, "liburing"))
        .status()
        .unwrap();
    Command::new("./configure")
        .current_dir(concat(out, "liburing"))
        .status()
        .unwrap();
    Command::new("make")
        .current_dir(concat(out, "liburing/src"))
        .arg("liburing-ffi.a")
        .status()
        .unwrap();
    bindgen::builder()
        .clang_arg(["-I", concat(out, "liburing/src/include")].concat())
        .header(concat(out, "liburing/src/ffi.c"))
        .allowlist_file(concat(out, "liburing/src/include/liburing.h"))
        .allowlist_file(concat(out, "liburing/src/include/liburing/io_uring.h"))
        .default_enum_style(EnumVariation::ModuleConsts)
        .default_non_copy_union_style(NonCopyUnionStyle::ManuallyDrop)
        .opaque_type("io_uring")
        .no_copy("io_uring")
        .blocklist_type("io_uring_sq")
        .blocklist_type("io_uring_cq")
        .generate()
        .unwrap()
        .write_to_file(concat(out, "bindings.rs"))
        .unwrap();
    println!("cargo::rustc-link-search={}", concat(out, "liburing/src"));
    println!("cargo::rustc-link-lib=static:+verbatim=liburing-ffi.a");
}

fn concat(a: &str, b: &str) -> &'static str {
    PathBuf::from(a).join(b).to_str().unwrap().to_owned().leak()
}
