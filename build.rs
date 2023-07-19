fn main() {
    cc::Build::new()
        .file("interop/get_wp_hwnd.c")
        .compile("get_wp_hwnd");
    println!("cargo:rustc-link-lib=user32"); 
}
