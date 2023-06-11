extern crate bindgen;
// use std::env;
use std::path::PathBuf;
// use bindgen::CargoCallbacks;

fn main() {
    println!("cargo:rustc-link-search=native=solclient-7.25.0.10/lib");
    println!("cargo:rustc-link-search=native=rsolace-sys/solclient-7.25.0.10/lib");
    // println!("cargo:rustc-link-search=solclient-7.25.0.10/include/solclient");
    println!("cargo:rustc-link-lib=static=solclient");
    // println!("cargo:rustc-link-lib=dylib=solclient");
    // println!("cargo:rerun-if-changed=solclient-7.25.0.10/include/solclient/solClient.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-Isolclient-7.25.0.10/include")
        .allowlist_function("^solClient_.*")
        .allowlist_var("^SOLCLIENT_.*")
        // .dynamic_library_name("solclient")
        // .dynamic_link_require_all(true)
        .size_t_is_usize(true)
        .generate_comments(false)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("solace_bindings.rs"))
        .expect("Couldn't write bindings!");

    // cc::Build::new().include("solclient-7.25.0.10/include");
    
}
