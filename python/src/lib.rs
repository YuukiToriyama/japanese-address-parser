use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::OnceLock;

use japanese_address_parser::parser::ParseResult;
use japanese_address_parser::parser::Parser;

#[pyclass(name = "ParseResult")]
struct PyParseResult {
    #[pyo3(get)]
    address: HashMap<String, String>,
    #[pyo3(get)]
    error: HashMap<String, String>,
}

impl From<ParseResult> for PyParseResult {
    fn from(value: ParseResult) -> Self {
        let address = HashMap::from([
            ("prefecture".to_string(), value.address.prefecture),
            ("city".to_string(), value.address.city),
            ("town".to_string(), value.address.town),
            ("rest".to_string(), value.address.rest),
        ]);
        let error = value.error.map_or_else(HashMap::new, |err| {
            HashMap::from([
                ("error_type".to_string(), err.error_type),
                ("error_message".to_string(), err.error_message),
            ])
        });
        Self { address, error }
    }
}

#[pyclass(name = "Parser")]
struct PyParser {
    parser: Parser,
}

#[pymethods]
impl PyParser {
    #[new]
    fn default() -> Self {
        PyParser {
            parser: Default::default(),
        }
    }

    fn parse(&self, py: Python<'_>, address: &str) -> PyParseResult {
        // parse_blocking はPythonオブジェクトに触れないためGILを解放する
        py.detach(|| self.parser.parse_blocking(address)).into()
    }
}

static GLOBAL_PARSER: OnceLock<Parser> = OnceLock::new();

fn get_parser() -> &'static Parser {
    GLOBAL_PARSER.get_or_init(Default::default)
}

#[pyfunction]
fn parse(py: Python<'_>, address: &str) -> PyParseResult {
    py.detach(|| get_parser().parse_blocking(address)).into()
}

#[pymodule]
#[pyo3(name = "japanese_address_parser_py")]
fn python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyParseResult>()?;
    m.add_class::<PyParser>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
