fn main() {
    if std::env::var("CARGO_CFG_PROCMACRO2_SEMVER_EXEMPT").is_ok() {
        println!("cargo:rustc-cfg=feature=\"proc_macro_spans\"")
    }
}
