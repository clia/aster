fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/pb/aster.cache.proto"], &["src/pb"])?;
    Ok(())
}