use std::env;

fn main() {
    // Determine the directory where build.rs and Cargo.toml reside
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Construct the path to static libraries
    let path = format!("{}/vendor", dir);

    // Specify the paths to the static libraries
    println!("cargo:rustc-link-search=native={}", path);

    // Tell cargo to link with these static libraries
    println!("cargo:rustc-link-lib=static=rapidsnark-darwin-arm64");
    println!("cargo:rustc-link-lib=static=gmp-darwin-arm64");

    // Link with the C++ standard library
    println!("cargo:rustc-link-lib=c++");
}
