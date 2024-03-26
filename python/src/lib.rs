use japanese_address_parser::api::{BlockingApi, BlockingApiImpl};
use japanese_address_parser::entity::ParseResult;
use japanese_address_parser::parser::parse_blocking;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass(name = "ParseResult")]
struct PyParseResult {
    #[pyo3(get)]
    address: HashMap<String, String>,
    #[pyo3(get)]
    error: HashMap<String, String>,
}

impl From<ParseResult> for PyParseResult {
    fn from(value: ParseResult) -> Self {
        let mut address = HashMap::new();
        address.insert(String::from("prefecture"), value.address.prefecture);
        address.insert(String::from("city"), value.address.city);
        address.insert(String::from("town"), value.address.town);
        address.insert(String::from("rest"), value.address.rest);
        let mut error = HashMap::new();
        if let Some(err) = value.error {
            error.insert(String::from("error_type"), err.error_type);
            error.insert(String::from("error_message"), err.error_message);
        }
        Self { address, error }
    }
}

#[pyfunction]
fn parse(address: &str) -> PyParseResult {
    let api = BlockingApiImpl::new();
    parse_blocking(api, address).into()
}

#[pymodule]
#[pyo3(name = "japanese_address_parser_py")]
fn python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyParseResult>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
