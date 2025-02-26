fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    #[cfg(feature = "winpty")]
    println!("cargo:rustc-link-lib=static=winpty");
}
