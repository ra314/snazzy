use std::io::Result;
fn main() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let descriptor_path = std::path::PathBuf::from(out_dir.clone()).join("descriptors.bin");
    prost_build::Config::new()
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(&[
        "src/items.proto"], &["src/"])?;
    Ok(())
}