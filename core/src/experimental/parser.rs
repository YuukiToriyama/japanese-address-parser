use crate::domain::common::latlng::LatLng;
use crate::domain::common::token::Token;
use serde::Serialize;

/// Data source for Parser
///
/// パーサーで使用するデータソースを指定します。
#[derive(Debug, Default)]
pub enum DataSource {
    /// ChimeiRuiju 住所データ
    /// <https://github.com/Cocon/chimei-ruiju>
    ChimeiRuiju,
    /// Geolonia 住所データ
    /// <https://github.com/geolonia/japanese-addresses>
    #[default]
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
    /// Parse address into [ParsedAddress].
    ///
    /// 住所をパースし、[ParsedAddress]を返します。
    ///
    /// # Example
    /// ```
    /// use japanese_address_parser::experimental::parser::Parser;
    ///
    /// async fn example() {
    ///     let parser = Parser::default();
    ///     let result = parser.parse("埼玉県所沢市上山口2135").await;
    ///     assert_eq!(result.prefecture, "埼玉県");
    ///     assert_eq!(result.city, "所沢市");
    ///     assert_eq!(result.town, "上山口");
    ///     assert_eq!(result.rest, "2135");
    ///     assert_eq!(result.metadata.depth, 3);
    /// }
    /// ```
    pub async fn parse(&self, address: &str) -> ParsedAddress {
        match self.options.data_source {
            DataSource::ChimeiRuiju => {
                ParsedAddress::from(self.parse_with_chimeiruiju(address).await)
            }
            DataSource::Geolonia => ParsedAddress::from(self.parse_with_geolonia(address).await),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ParsedAddress {
    /// 都道府県名
    pub prefecture: String,
    /// 市区町村名
    pub city: String,
    /// 町名
    pub town: String,
    /// それ以降
    pub rest: String,
    /// メタデータ
    pub metadata: Metadata,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Metadata {
    /// 緯度
    ///
    /// 住所のパースに成功し、緯度経度の情報が取得できる場合、緯度を返します。
    /// 緯度経度の情報はあくまで検出できた地域の代表点を表すものであり、入力された住所の実際の位置とは必ずしも一致しないことに注意してください。
    pub latitude: Option<f64>,
    /// 経度
    ///
    /// 住所のパースに成功し、緯度経度の情報が取得できる場合、経度を返します。
    /// 緯度経度の情報はあくまで検出できた地域の代表点を表すものであり、入力された住所の実際の位置とは必ずしも一致しないことに注意してください。
    pub longitude: Option<f64>,
    /// パース処理の深度
    ///
    /// - `0`: 何も検出できなかった場合
    /// - `1`: 都道府県名までは検出できた場合
    /// - `2`: 市区町村名までは検出できた場合
    /// - `3`: 町名まで検出できた場合
    pub depth: u8,
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
                Token::Prefecture(prefecture_name) => {
                    parsed_address.prefecture = prefecture_name;
                    parsed_address.metadata.depth = 1;
                }
                Token::City(city_name) => {
                    parsed_address.city = city_name;
                    parsed_address.metadata.depth = 2;
                }
                Token::Town(town_name) => {
                    parsed_address.town = town_name;
                    parsed_address.metadata.depth = 3;
                }
                Token::Rest(rest) => {
                    parsed_address.rest = rest;
                }
            }
        }

        parsed_address
    }
}

impl From<(Vec<Token>, Option<LatLng>)> for ParsedAddress {
    fn from((tokens, lat_lng): (Vec<Token>, Option<LatLng>)) -> Self {
        let mut parsed_address = ParsedAddress::from(tokens);
        if let Some(lat_lng) = lat_lng {
            parsed_address.metadata.longitude = Some(lat_lng.longitude);
            parsed_address.metadata.latitude = Some(lat_lng.latitude);
        }
        parsed_address
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::common::latlng::LatLng;
    use crate::domain::common::token::Token;
    use crate::experimental::parser::{Metadata, ParsedAddress};

    #[test]
    fn conversion_depthが0() {
        let tokens = vec![Token::Rest(
            "新浜県新浜市ニューポートシティ1-1-1".to_string(),
        )];
        let parsed_address = ParsedAddress::from(tokens);
        assert_eq!(
            parsed_address,
            ParsedAddress {
                prefecture: "".to_string(),
                city: "".to_string(),
                town: "".to_string(),
                rest: "新浜県新浜市ニューポートシティ1-1-1".to_string(),
                metadata: Metadata {
                    latitude: None,
                    longitude: None,
                    depth: 0,
                },
            }
        )
    }

    #[test]
    fn conversion_depthが1() {
        let tokens = vec![
            Token::Prefecture("東京都".to_string()),
            Token::Rest("".to_string()),
        ];
        let lat_lng = Some(LatLng {
            latitude: 139.748264,
            longitude: 35.68532,
        });
        let parsed_address = ParsedAddress::from((tokens, lat_lng));
        assert_eq!(
            parsed_address,
            ParsedAddress {
                prefecture: "東京都".to_string(),
                city: "".to_string(),
                town: "".to_string(),
                rest: "".to_string(),
                metadata: Metadata {
                    latitude: Some(139.748264),
                    longitude: Some(35.68532),
                    depth: 1,
                },
            }
        )
    }

    #[test]
    fn conversion_depthが2() {
        let tokens = vec![
            Token::Prefecture("東京都".to_string()),
            Token::City("台東区".to_string()),
            Token::Rest("".to_string()),
        ];
        let lat_lng = Some(LatLng {
            latitude: 139.764379,
            longitude: 35.711162,
        });
        let parsed_address = ParsedAddress::from((tokens, lat_lng));
        assert_eq!(
            parsed_address,
            ParsedAddress {
                prefecture: "東京都".to_string(),
                city: "台東区".to_string(),
                town: "".to_string(),
                rest: "".to_string(),
                metadata: Metadata {
                    latitude: Some(139.764379),
                    longitude: Some(35.711162),
                    depth: 2,
                },
            }
        )
    }

    #[test]
    fn conversion_depthが3() {
        let tokens = vec![
            Token::Prefecture("東京都".to_string()),
            Token::City("文京区".to_string()),
            Token::Town("本駒込六丁目".to_string()),
            Token::Rest("16-3".to_string()),
        ];
        let lat_lng = Some(LatLng {
            latitude: 139.738043,
            longitude: 35.72791,
        });
        let parsed_address = ParsedAddress::from((tokens, lat_lng));
        assert_eq!(
            parsed_address,
            ParsedAddress {
                prefecture: "東京都".to_string(),
                city: "文京区".to_string(),
                town: "本駒込六丁目".to_string(),
                rest: "16-3".to_string(),
                metadata: Metadata {
                    latitude: Some(139.738043),
                    longitude: Some(35.72791),
                    depth: 3,
                },
            }
        )
    }
}
