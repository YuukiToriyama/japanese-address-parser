use std::collections::HashMap;

use japanese_address_parser::parser::ParseResult;
use japanese_address_parser::parser::Parser;
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyfunction, gen_stub_pymethods};

/// A class represent parse result.
///
/// パース処理の結果を表すクラスです。
///
/// - `address`: 都道府県名、市区町村名、町名、それ以降の文字列をそれぞれ格納する辞書型を返します。
///     - `address.prefecture`: 都道府県名
///     - `address.city`: 市区町村名
///     - `address.town`: 町名
///     - `address.rest`: それ以降
/// - `error`: パース処理中にエラーが発生した場合、エラーのタイプとエラーメッセージを格納する辞書型を返します。
///     - `error_type`: エラータイプ
///     - `error_message`: エラーメッセージ
#[gen_stub_pyclass]
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

#[gen_stub_pyclass]
#[pyclass(name = "Parser")]
struct PyParser {
    parser: Parser,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyParser {
    /// Construct a parser.
    ///
    /// パーサーを生成します。
    ///
    /// :return: パーサー
    /// :rtype: Parser
    #[new]
    fn default() -> Self {
        PyParser {
            parser: Default::default(),
        }
    }

    /// Format informal address into formal style
    ///
    /// 入力された住所を正式な表記に整形します。
    ///
    /// :param str address: 住所の文字列
    /// :return: 整形、分割済みの住所の構造体
    /// :rtype: ParseResult
    fn parse(&self, address: &str) -> PyParseResult {
        self.parser.parse_blocking(address).into()
    }
}

/// Format informal address into formal style
///
/// 入力された住所を正式な表記に整形します。
///
/// :param str address: 住所の文字列
/// :return: 整形、分割済みの住所の構造体
/// :rtype: ParseResult
#[gen_stub_pyfunction]
#[pyfunction]
fn parse(address: &str) -> PyParseResult {
    let parser: Parser = Default::default();
    parser.parse_blocking(address).into()
}

#[pymodule]
#[pyo3(name = "japanese_address_parser_py")]
fn python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyParseResult>()?;
    m.add_class::<PyParser>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
