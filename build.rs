fn main() {
    println!("cargo:rustc-link-search=cmake-build-debug");
    println!("cargo:rustc-link-lib=gameEngine");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
    println!("cargo:rustc-link-lib=SDL2_mixer");
    println!("cargo:rustc-link-lib=SDL2_ttf");
}
