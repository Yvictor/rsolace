extern crate bindgen;
// use std::env;
use std::{io::Write, path::PathBuf};
// use bindgen::CargoCallbacks;

#[cfg(target_os = "windows")]
const SOLCLIENT_GZ_PATH: &str = "solclient_Win_vs2015_7.25.0.10.tar.gz";

#[cfg(target_os = "macos")]
const SOLCLIENT_GZ_PATH: &str = "solclient_Darwin-universal2_opt_7.25.0.10.tar.gz";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const SOLCLIENT_GZ_PATH: &str = "solclient_Linux26-x86_64_opt_7.25.0.10.tar.gz";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const SOLCLIENT_GZ_PATH: &str = "solclient_Linux26-aarch64_opt_7.25.0.10.tar.gz";

#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "musl"))]
const SOLCLIENT_GZ_PATH: &str = "solclient_Linux26-musl-x86_64_opt_7.25.0.10.tar.gz";

fn main() {
    let solclient_folder_name = "../solclient-7.25.0.10";
    let solclient_folder_path = std::path::Path::new(solclient_folder_name);
    let solclient_gz_url =
        format!("https://github.com/Yvictor/rsolace/releases/download/0.0.0/{SOLCLIENT_GZ_PATH}");
    let resp = reqwest::blocking::get(solclient_gz_url).unwrap();
    let content = resp.bytes().unwrap();
    let file_gz_name = format!("{solclient_folder_name}.tar.gz");
    let file_gz_path = std::path::Path::new(&file_gz_name);
    if !file_gz_path.exists() {
        let mut file_gz = std::fs::File::create(file_gz_path).unwrap();
        file_gz.write_all(&content).unwrap();
        file_gz.sync_data().unwrap();
    }
    let file_gz = std::fs::File::open(file_gz_path).unwrap();
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file_gz));
    archive
        .entries()
        .unwrap()
        .filter_map(|r| r.ok())
        .map(|mut entry| -> std::io::Result<PathBuf> {
            let strip_path = entry.path()?.iter().skip(1).collect::<std::path::PathBuf>();
            let path = solclient_folder_path.join(strip_path);
            // println!("unpack: {:?}", path);
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    // let p = format!("../{}", solclient_folder_name);
    // let solclient_path = std::path::Path::new(&p);
    println!("cargo:rustc-link-search=native={solclient_folder_name}/lib");
    println!("cargo:rustc-link-search=native=rsolace-sys/{solclient_folder_name}/lib");
    // println!(
    //     "cargo:rustc-link-search=native={}/lib",
    //     package_solclient_folder.to_str().unwrap()
    // );
    // // println!("cargo:rustc-link-search=native=rsolace-sys/solclient-7.25.0.10/lib");
    // // println!("cargo:rustc-link-search=solclient-7.25.0.10/include/solclient");
    let os = std::env::consts::OS;
    if os == "macos" {
        println!("cargo:rustc-link-lib=dylib=gssapi_krb5");
    }
    if os == "windows" {
        println!("cargo:rustc-link-search=native={solclient_folder_name}/lib/third-party");
        println!(
            "cargo:rustc-link-search=native=rsolace-sys/{solclient_folder_name}/lib/third-party"
        );
    }
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=solclient");
    let include_path = solclient_folder_path.join("include");
    let include_arg = format!("-I{}", include_path.to_str().unwrap());
    println!("cargo:rerun-if-changed={solclient_folder_name}/include/solclient/solClient.h");
    println!("cargo:rerun-if-changed={solclient_folder_name}/include/solclient/solCache.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-v")
        .clang_arg("-Isolclient-7.25.0.10/include")
        .clang_arg(include_arg)
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
