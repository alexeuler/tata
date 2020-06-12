use std::env;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-link-search=target/{}", profile);
    println!("cargo:rustc-link-lib=tata_core");
}
