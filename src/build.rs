use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/data_app.proto"], &["src/"])?;
    Ok(())
}
