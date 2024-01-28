use build_target::{Arch, Os};
use std::env;

fn main() {
    let target_arch = build_target::target_arch().unwrap();
    let target_os = build_target::target_os().unwrap();

    // Determine the directory where build.rs and Cargo.toml reside
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Construct the path to static libraries
    let path = format!("{}/vendor", dir);

    // Specify the paths to the static libraries
    println!("cargo:rustc-link-search=native={}", path);

    let os_suffix = match target_os {
        Os::MacOs => {
            // Link with the C++ standard library
            println!("cargo:rustc-link-lib=c++");

            "darwin"
        }
        _ => {
            // Link with the C++ standard library
            println!("cargo:rustc-link-lib=stdc++");

            "linux"
        }
    };

    let arch_suffix = match target_arch {
        Arch::X86_64 => {
            // Link with OpenMP
            println!("cargo:rustc-link-lib=omp");

            "amd64"
        }
        Arch::AARCH64 => "arm64",
        _ => panic!("Unsupported architecture: {}", target_arch),
    };

    println!(
        "cargo:rustc-link-lib=static=rapidsnark-{}-{}",
        os_suffix, arch_suffix
    );
    println!(
        "cargo:rustc-link-lib=static=gmp-{}-{}",
        os_suffix, arch_suffix
    );
}
