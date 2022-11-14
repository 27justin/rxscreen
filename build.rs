

fn main() {
    println!("cargo:rustc-link-lib=dylib=X11");

    #[cfg(feature = "shm")]
    println!("cargo:rustc-link-lib=dylib=Xext");

    #[cfg(feature = "xrandr")]
    println!("cargo:rustc-link-lib=dylib=Xrandr");
}
