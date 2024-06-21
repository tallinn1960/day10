#![allow(missing_docs)]
use cmake::Config;

fn main() {
    #[cfg(target_os = "macos")]
    {
        use swift_rs::SwiftLinker;
        SwiftLinker::new("10.13")
            // Only if you are also targetting iOS
            // Ensure the same minimum supported iOS version is specified as in your `Package.swift` file
            .with_ios("11")
            .with_package("Day10Swift", "Day10Swift")
            .link();
    }

    let dst = Config::new(".")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=day10cpp");
}
