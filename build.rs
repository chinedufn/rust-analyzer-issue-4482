use std::path::PathBuf;

fn main() {
    let generated = out_dir().join("generated.rs");
    std::fs::write(generated, r#"pub const GENERATED_CONSTANT: u16 = 0;"#).unwrap();
}

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
}
