use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = traopy_builder::stub_info()?;
    stub.generate()?;
    Ok(())
}
