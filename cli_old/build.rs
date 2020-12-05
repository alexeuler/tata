use std::env;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-link-search=target/{}", profile);
    println!("cargo:rerun-if-changed=target/{}/tata_core.a", profile);
    println!("cargo:rustc-link-lib=tata_core");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=framework=Security");
}
