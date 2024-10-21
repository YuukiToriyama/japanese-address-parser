use crate::domain::common::token::Token;

#[derive(Debug)]
pub enum DataSource {
    /// Geolonia 住所データ
    /// https://github.com/geolonia/japanese-addresses
    Geolonia,
}

/// Options for Parser
///
/// パーサーのオプションを指定します。
/// 何も変更しない場合は`ParserOptions::default()`を使用してください。
///
/// # Example
/// ```
/// use japanese_address_parser::experimental::parser::{DataSource, Parser, ParserOptions};
///
/// // Customize parser
/// let parser = Parser {
///     options: ParserOptions {
///         data_source: DataSource::Geolonia,
///         correct_incomplete_city_names: false,
///         verbose: false,
///     }
/// };
///
/// // Use default options
/// let parser = Parser {
///     options: ParserOptions::default()
/// };
/// ```
#[derive(Debug)]
pub struct ParserOptions {
    /// 使用する住所データ
    pub data_source: DataSource,
    /// 入力された住所が不正確で市区町村名を検出できない場合、あいまい検索で市区町村名を検出します
    pub correct_incomplete_city_names: bool,
    /// ログの出力の有無
    pub verbose: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            data_source: DataSource::Geolonia,
            correct_incomplete_city_names: true,
            verbose: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct Parser {
    pub options: ParserOptions,
}

impl Parser {
    pub async fn parse(&self, address: &str) -> Vec<Token> {
        match self.options.data_source {
            DataSource::Geolonia => self.parse_with_geolonia(address).await,
        }
    }
}
