fn main() {
    // Build the C++ code
    cxx_build::bridge("src/lib.rs")
        .include("src")
        .include("src/signalsmith-stretch")
        .flag_if_supported("-std=c++14")
        .compile("ssstretch");

    // Tell cargo to re-run this build script if src/bridge.h changes
    println!("cargo:rerun-if-changed=src/bridge.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/signalsmith-stretch/signalsmith-stretch.h");
}