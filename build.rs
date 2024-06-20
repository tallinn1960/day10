#![allow(missing_docs)]
use cmake::Config;

fn main() {
    let dst = Config::new(".")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=day10cpp");
}