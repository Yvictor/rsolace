use std::env;

fn main() {
    let solclient_lib_dir = "rsolace-sys/solclient-7.25.0.10/lib";
    println!("cargo:rustc-link-search=native={}", solclient_lib_dir);
    env::set_var("LD_LIBRARY_PATH", solclient_lib_dir);
}