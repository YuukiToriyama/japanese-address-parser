use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = japanese_address_parser_py::stub_info()?;
    stub.generate()?;
    Ok(())
}
