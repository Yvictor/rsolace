extern crate bindgen;
// use std::env;
use std::process::Command;
use std::{io::Write, path::PathBuf};
// use bindgen::CargoCallbacks;

#[cfg(target_os = "windows")]
const SOLCLIENT_GZ_PATH: &str = "solclient_Win_vs2015_7.25.0.10.tar.gz";

#[cfg(target_os = "macos")]
const SOLCLIENT_GZ_PATH: &str = "solclient_Darwin-universal2_opt_7.25.0.10.tar.gz";

#[cfg(target_os = "ios")]
const SOLCLIENT_GZ_PATH: &str = "solclient_ios_opt_7.25.0.10.tar.gz";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const SOLCLIENT_GZ_PATH: &str = "solclient_Linux26-x86_64_opt_7.25.0.10.tar.gz";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const SOLCLIENT_GZ_PATH: &str = "solclient_Linux-aarch64_opt_7.25.0.10_withssl.tar.gz";

#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "musl"))]
const SOLCLIENT_GZ_PATH: &str = "solclient_Linux_musl-x86_64_opt_7.25.0.10.tar.gz";

fn lipo_arch_lib(solclient_folder_name: &str, arch: &str, lib_name: &str) {
    let lib_path = format!("{}/lib/lib{}.a", solclient_folder_name, lib_name);
    let lib_path_exist = std::path::Path::new(&lib_path).exists();
    let source_lib_path = if lib_path_exist {
        lib_path
    } else {
        format!(
            "rsolace-sys/{}/lib/lib{}.a",
            solclient_folder_name, lib_name
        )
    };
    let arch_lib_path = if lib_path_exist {
        format!("{}/lib/lib{}-{}.a", solclient_folder_name, lib_name, arch)
    } else {
        format!(
            "rsolace-sys/{}/lib/lib{}-{}.a",
            solclient_folder_name, lib_name, arch
        )
    };

    Command::new("lipo")
        .arg(source_lib_path)
        .arg("-thin")
        .arg(arch)
        .arg("-output")
        .arg(arch_lib_path)
        .spawn()
        .expect(&format!("lipo {} error", lib_name));
}

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("target_os: {} | target_arch: {}", target_os, target_arch);
    let solclient_gz_path = if target_os == "ios" {
        "solclient_ios_opt_7.25.0.10.tar.gz"
    } else {
        SOLCLIENT_GZ_PATH
    };
    let solclient_folder_name = "../solclient-7.25.0.10";
    let solclient_folder_path = std::path::Path::new(solclient_folder_name);
    let solclient_gz_url =
        format!("https://github.com/Yvictor/rsolace/releases/download/0.0.0/{solclient_gz_path}");
    println!("{}", solclient_gz_url);
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build()
        .unwrap();
    let resp = client.get(solclient_gz_url).send().unwrap();
    // let resp = reqwest::blocking::get(solclient_gz_url).unwrap();
    let content = resp.bytes().unwrap();
    println!("content size: {}", content.len());
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
    // let os = std::env::consts::OS;
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=dylib=gssapi_krb5");
    }
    if target_os == "windows" {
        let arch_folder = if std::env::var("Platform").unwrap_or("x64".into()) == "x64" {
            "Win64"
        } else {
            "Win32"
        };
        println!("cargo:rustc-link-search=native={solclient_folder_name}/lib/{arch_folder}");
        println!(
            "cargo:rustc-link-search=native={solclient_folder_name}/lib/{arch_folder}/third-party"
        );
        println!(
            "cargo:rustc-link-search=native=rsolace-sys/{solclient_folder_name}/lib/{arch_folder}"
        );
        println!(
            "cargo:rustc-link-search=native=rsolace-sys/{solclient_folder_name}/lib/{arch_folder}/third-party"
        );
        println!("cargo:rustc-link-lib=static=libssl_s");
        println!("cargo:rustc-link-lib=static=libcrypto_s");
        println!("cargo:rustc-link-lib=static=libsolclient_s");
        // println!("cargo:rustc-link-lib=static=libsolclient");
    } else {
        if target_os == "ios" {
            let arch = if target_arch == "aarch64" {
                "arm64"
            } else if target_arch == "x86_64" {
                "x86_64"
            } else {
                "armv7s"
            };
            // let arch = "arm64";
            lipo_arch_lib(solclient_folder_name, arch, "ssl-universal");
            lipo_arch_lib(solclient_folder_name, arch, "crypto-universal");
            lipo_arch_lib(solclient_folder_name, arch, "solclient");
            println!("cargo:rustc-link-lib=static=ssl-universal-{arch}");
            println!("cargo:rustc-link-lib=static=crypto-universal-{arch}");
            println!("cargo:rustc-link-lib=static=solclient-{arch}");
        } else {
            println!("cargo:rustc-link-lib=static=ssl");
            println!("cargo:rustc-link-lib=static=crypto");
            println!("cargo:rustc-link-lib=static=solclient");
            println!("cargo:rustc-link-lib=static=solclientssl");
        }
    }
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
