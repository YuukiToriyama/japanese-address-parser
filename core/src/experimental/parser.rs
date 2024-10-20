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
