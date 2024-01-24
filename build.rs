use ethers::contract::Abigen;
use std::env;
use std::path::Path;

/// Path to Depositor contract's JSON ABI. If it is absent use `npm run install && npm run compile`
/// in `repository/contracts` to compile it.
const DEPOSITOR_ABI_ARTIFACTS_PATH: &str =
    "contracts/artifacts/contracts/Depositor.sol/Depositor.json";
const DEPOSITOR_RUST_BINDINGS_PATH: &str = "src/depositor.rs";

fn main() {
    let cargo_manifest_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rerun-if-changed={}/{}",
        cargo_manifest_path, DEPOSITOR_ABI_ARTIFACTS_PATH
    );

    compile_depositor_bindings(cargo_manifest_path)
}

fn compile_depositor_bindings(cargo_manifest_path: String) {
    if env::var("UPDATE_DEPOSITOR_BINDINGS").is_err() {
        return;
    }

    let abi_src_path = format!("{}/{}", cargo_manifest_path, DEPOSITOR_ABI_ARTIFACTS_PATH);

    // Check if JSON ABI file of the Depositor contract is exists.
    let if_json_abi_exists = Path::new(&abi_src_path).exists();

    if !if_json_abi_exists {
        panic!(
            "\n=> JSON ABI file of the Depositor contract is absent.\n=> Use `npm install && npm run compile` in `<repository_root>/contracts` to compile it.\n"
        );
    }

    let depositor_bindings_path =
        format!("{}/{}", cargo_manifest_path, DEPOSITOR_RUST_BINDINGS_PATH);

    Abigen::new("Depositor", abi_src_path)
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(depositor_bindings_path)
        .unwrap();
}
