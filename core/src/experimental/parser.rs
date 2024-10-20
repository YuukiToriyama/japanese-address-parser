use crate::domain::common::token::Token;

pub enum DataSource {
    /// Geolonia 住所データ
    /// https://github.com/geolonia/japanese-addresses
    Geolonia,
}

pub struct ParserOptions {
    /// 使用する住所データ
    pub(crate) data_source: DataSource,
    /// 入力された住所が不正確で市区町村名を検出できない場合、あいまい検索で市区町村名を検出します
    pub(crate) correct_incomplete_city_names: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            data_source: DataSource::Geolonia,
            correct_incomplete_city_names: true,
        }
    }
}

#[derive(Default)]
pub(crate) struct Parser {
    pub(crate) options: ParserOptions,
}

impl Parser {
    pub async fn parse(&self, address: &str) -> Vec<Token> {
        match self.options.data_source {
            DataSource::Geolonia => self.parse_with_geolonia(address).await,
        }
    }
}
