use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/soda_api.proto"], &["src/"])?;
    Ok(())
}
