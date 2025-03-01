fn main() {
    println!("OUT_DIR = {:?}", std::env::var("OUT_DIR"));
    println!("OPT_LEVEL = {:?}", std::env::var("OPT_LEVEL"));
    println!("TARGET = {:?}", std::env::var("TARGET"));
    println!("HOST = {:?}", std::env::var("HOST"));
    
    // Build the C++ code
    cxx_build::bridge("src/ffi.rs")
        .include("src")
        .include("src/signalsmith-stretch")
        .flag_if_supported("-std=c++14")
        .compile("ssstretch");

    // Tell cargo to re-run this build script if source files change
    println!("cargo:rerun-if-changed=src/bridge.h");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/stretch.rs");
    println!("cargo:rerun-if-changed=src/dsp/filters.rs");
    println!("cargo:rerun-if-changed=src/signalsmith-stretch/signalsmith-stretch.h");
    
    // Additional include paths for DSP components
    println!("cargo:rerun-if-changed=src/signalsmith-stretch/dsp/filters.h");
    println!("cargo:rerun-if-changed=src/signalsmith-stretch/dsp/fft.h");
    println!("cargo:rerun-if-changed=src/signalsmith-stretch/dsp/windows.h");
    println!("cargo:rerun-if-changed=src/signalsmith-stretch/dsp/spectral.h");
    println!("cargo:rerun-if-changed=src/signalsmith-stretch/dsp/delay.h");
}