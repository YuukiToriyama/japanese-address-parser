use crate::domain::common::token::Token;

pub enum DataSource {
    /// Geolonia 住所データ
    /// https://github.com/geolonia/japanese-addresses
    Geolonia,
}

pub struct ParserOptions {
    /// 使用する住所データ
    pub(crate) data_source: DataSource,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            data_source: DataSource::Geolonia,
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

    #[inline]
    async fn parse_with_geolonia(&self, address: &str) -> Vec<Token> {
        unimplemented!()
    }
}
