fn main() {
    println!("cargo:rustc-link-search=C/cmake-build-debug");
    println!("cargo:rustc-link-lib=foo");
}