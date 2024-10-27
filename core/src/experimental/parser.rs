use crate::domain::common::token::Token;

/// Data source for Parser
///
/// パーサーで使用するデータソースを指定します。
#[derive(Debug)]
pub enum DataSource {
    /// Geolonia 住所データ
    /// <https://github.com/geolonia/japanese-addresses>
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

/// Yet another address parser
///
/// 新型の住所パーサーです。オプションを指定しない場合は`Parser::default()`を使用できます。
#[derive(Debug, Default)]
pub struct Parser {
    /// パーサーのオプションを指定します
    pub options: ParserOptions,
}

impl Parser {
    /// Parse address into token sequence.
    ///
    /// 住所をパースしトークン列に変換します。
    ///
    /// # Example
    /// ```
    /// use japanese_address_parser::experimental::parser::Parser;
    ///
    /// async fn example() {
    ///     let parser = Parser::default();
    ///     let result = parser.parse("埼玉県所沢市上山口2135").await;
    ///     println!("{:?}", result);
    /// }
    /// ```
    pub async fn parse(&self, address: &str) -> Vec<Token> {
        match self.options.data_source {
            DataSource::Geolonia => self.parse_with_geolonia(address).await,
        }
    }
}

pub struct ParsedAddress {
    /// 都道府県名
    prefecture: String,
    /// 市区町村名
    city: String,
    /// 町名
    town: String,
    /// それ以降
    rest: String,
    /// メタデータ
    metadata: Metadata,
}

pub struct Metadata {
    /// 緯度
    ///
    /// 住所のパースに成功し、緯度経度の情報が取得できる場合、緯度を返します。
    /// 緯度経度の情報はあくまで検出できた地域の代表点を表すものであり、入力された住所の実際の位置とは必ずしも一致しないことに注意してください。
    latitude: Option<f64>,
    /// 軽度
    ///
    /// 住所のパースに成功し、緯度経度の情報が取得できる場合、軽度を返します。
    /// 緯度経度の情報はあくまで検出できた地域の代表点を表すものであり、入力された住所の実際の位置とは必ずしも一致しないことに注意してください。
    longitude: Option<f64>,
    /// パース処理の深度
    ///
    /// - `0`: 何も検出できなかった場合
    /// - `1`: 都道府県名までは検出できた場合
    /// - `2`: 市区町村名までは検出できた場合
    /// - `3`: 町名まで検出できた場合
    depth: u8,
}

impl From<Vec<Token>> for ParsedAddress {
    fn from(mut value: Vec<Token>) -> Self {
        // 現在の実装では`Tokenizer`からもたらされる`Vec<Token>`は要素が順序よく並んでいるため、本来以下の実装は不要である
        // ただし、ソート済みになっていることがコード上保証できないのが気になるため、ここでソートを行なっている。
        value.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut parsed_address = ParsedAddress {
            prefecture: "".to_string(),
            city: "".to_string(),
            town: "".to_string(),
            rest: "".to_string(),
            metadata: Metadata {
                latitude: None,
                longitude: None,
                depth: 0,
            },
        };

        for token in value {
            match token {
                Token::Prefecture(prefecture) => {
                    parsed_address.prefecture = prefecture.prefecture_name;
                    parsed_address.metadata.depth = 1;
                    if let Some(lat_lng) = prefecture.representative_point {
                        parsed_address.metadata.latitude = Some(lat_lng.latitude);
                        parsed_address.metadata.longitude = Some(lat_lng.longitude);
                    }
                }
                Token::City(city) => {
                    parsed_address.city = city.city_name;
                    parsed_address.metadata.depth = 2;
                    if let Some(lat_lng) = city.representative_point {
                        parsed_address.metadata.latitude = Some(lat_lng.latitude);
                        parsed_address.metadata.longitude = Some(lat_lng.longitude);
                    }
                }
                Token::Town(town) => {
                    parsed_address.town = town.town_name;
                    parsed_address.metadata.depth = 3;
                    if let Some(lat_lng) = town.representative_point {
                        parsed_address.metadata.latitude = Some(lat_lng.latitude);
                        parsed_address.metadata.longitude = Some(lat_lng.longitude);
                    }
                }
                Token::Rest(rest) => {
                    parsed_address.rest = rest;
                }
            }
        }

        parsed_address
    }
}
