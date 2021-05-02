fn main() -> anyhow::Result<()> {
    prost_build::compile_protos(&["../api.proto"], &[".."])?;
    Ok(())
}
