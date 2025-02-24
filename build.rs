fn main() {
    println!("cargo:rustc-link-arg=--export=napi_register_wasm_v1");
    println!("cargo:rustc-link-arg=--export-if-defined=node_api_module_get_api_version_v1");
    println!("cargo:rustc-link-arg=--import-memory");
    println!("cargo:rustc-link-arg=--import-undefined");
    println!("cargo:rustc-link-arg=--max-memory=4294967296");
}
