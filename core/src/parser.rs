use std::sync::Arc;

use crate::api::AsyncApi;
#[cfg(feature = "blocking")]
use crate::api::BlockingApi;
use crate::entity::{Address, ParseResult};
use crate::err::{Error, ParseErrorKind};
use crate::parser::read_city::read_city;
use crate::parser::read_prefecture::read_prefecture;
use crate::parser::read_town::read_town;

mod adapter;
mod filter;
mod read_city;
mod read_house_number;
mod read_prefecture;
mod read_town;

/// An asynchronous `Parser` to process addresses.
///
/// # Example
/// ```
/// use japanese_address_parser::parser::Parser;
///
/// async fn example() {
///     let parser : Parser = Default::default();
///     let result = parser.parse("東京都新宿区西新宿2-8-1").await;
///     println!("{:?}", result);
/// }
/// ```
pub struct Parser {
    async_api: Arc<AsyncApi>,
    #[cfg(feature = "blocking")]
    blocking_api: Arc<BlockingApi>,
}

impl Default for Parser {
    /// Constructs a new `Parser`.
    #[cfg(feature = "blocking")]
    fn default() -> Self {
        Self {
            async_api: Arc::new(Default::default()),
            blocking_api: Arc::new(Default::default()),
        }
    }
    /// Constructs a new `Parser`.
    #[cfg(not(feature = "blocking"))]
    fn default() -> Self {
        Self {
            async_api: Arc::new(Default::default()),
        }
    }
}

impl Parser {
    /// Parses the given `address` asynchronously.
    pub async fn parse(&self, address: &str) -> ParseResult {
        parse(self.async_api.clone(), address).await
    }

    /// Parses the given `address` synchronously.
    #[cfg(feature = "blocking")]
    pub fn parse_blocking(&self, address: &str) -> ParseResult {
        parse_blocking(self.blocking_api.clone(), address)
    }
}

/// A function to parse the given address asynchronously.
///
/// publicにしていますが、直接の使用は推奨されません。[Parser]の利用を検討してください。
pub async fn parse(api: Arc<AsyncApi>, input: &str) -> ParseResult {
    // 都道府県を特定
    let (rest, prefecture_name) = if let Some(result) = read_prefecture(input) {
        result
    } else {
        return ParseResult {
            address: Address::new("", "", "", input),
            error: Some(Error::new_parse_error(ParseErrorKind::Prefecture)),
        };
    };
    // その都道府県の市町村名リストを取得
    let prefecture = match api.get_prefecture_master(prefecture_name).await {
        Err(error) => {
            return ParseResult {
                address: Address::new(prefecture_name, "", "", rest),
                error: Some(error),
            };
        }
        Ok(result) => result,
    };
    // 市町村名を特定
    let (rest, city_name) = if let Some(result) = read_city(rest, prefecture) {
        result
    } else {
        return ParseResult {
            address: Address::new(prefecture_name, "", "", rest),
            error: Some(Error::new_parse_error(ParseErrorKind::City)),
        };
    };
    // その市町村の町名リストを取得
    let city = match api.get_city_master(prefecture_name, &city_name).await {
        Err(error) => {
            return ParseResult {
                address: Address::new(prefecture_name, &city_name, "", &rest),
                error: Some(error),
            };
        }
        Ok(result) => result,
    };
    // 町名を特定
    let (rest, town_name) = if let Some(result) = read_town(&rest, &city) {
        result
    } else {
        return ParseResult {
            address: Address::new(prefecture_name, &city_name, "", &rest),
            error: Some(Error::new_parse_error(ParseErrorKind::Town)),
        };
    };

    ParseResult {
        address: Address::new(prefecture_name, &city_name, &town_name, &rest),
        error: None,
    }
}

#[cfg(all(test, not(feature = "blocking")))]
mod tests {
    use crate::api::city_master_api::CityMasterApi;
    use crate::api::prefecture_master_api::PrefectureMasterApi;
    use crate::api::AsyncApi;
    use crate::err::ParseErrorKind;
    use crate::parser::parse;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    #[tokio::test]
    async fn 都道府県名が誤っている場合() {
        let api: AsyncApi = Default::default();
        let result = parse(api.into(), "青盛県青森市長島１丁目１−１").await;
        assert_eq!(result.address.prefecture, "");
        assert_eq!(result.address.city, "");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "青盛県青森市長島１丁目１−１");
        assert_eq!(result.error.is_some(), true);
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::Prefecture.to_string()
        );
    }

    #[tokio::test]
    async fn 都道府県マスタが取得できない場合() {
        let mut api: AsyncApi = Default::default();
        api.prefecture_master_api = PrefectureMasterApi {
            server_url: "https://example.com/invalid_url/api/",
        };

        let result = parse(api.into(), "青森県青森市長島１丁目１−１").await;
        assert_eq!(result.error.is_some(), true);
        assert_eq!(result.address.prefecture, "青森県");
        assert_eq!(result.address.city, "");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "青森市長島１丁目１−１");
    }

    #[tokio::test]
    async fn 市区町村名が誤っている場合() {
        let api: AsyncApi = Default::default();
        let result = parse(api.into(), "青森県青盛市長島１丁目１−１").await;
        assert_eq!(result.address.prefecture, "青森県");
        assert_eq!(result.address.city, "");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "青盛市長島１丁目１−１");
        assert_eq!(result.error.is_some(), true);
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::City.to_string()
        );
    }

    #[tokio::test]
    async fn 市区町村マスタが取得できない場合() {
        let mut api: AsyncApi = Default::default();
        api.city_master_api = CityMasterApi {
            server_url: "https://example.com/invalid_url/api/",
        };

        let result = parse(api.into(), "青森県青森市長島１丁目１−１").await;
        assert_eq!(result.error.is_some(), true);
        assert_eq!(result.address.prefecture, "青森県");
        assert_eq!(result.address.city, "青森市");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "長島１丁目１−１");
    }

    #[tokio::test]
    async fn 町名が誤っている場合() {
        let api: AsyncApi = Default::default();
        let result = parse(api.into(), "青森県青森市永嶋１丁目１−１").await;
        assert_eq!(result.address.prefecture, "青森県");
        assert_eq!(result.address.city, "青森市");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "永嶋１丁目１−１");
        assert_eq!(result.error.is_some(), true);
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::Town.to_string()
        );
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn parse_wasm_success() {
        let api: AsyncApi = Default::default();
        let result = parse(api.into(), "兵庫県淡路市生穂新島8番地").await;
        assert_eq!(result.address.prefecture, "兵庫県".to_string());
        assert_eq!(result.address.city, "淡路市".to_string());
        assert_eq!(result.address.town, "生穂".to_string());
        assert_eq!(result.address.rest, "新島8番地".to_string());
        assert_eq!(result.error, None);
    }
}

/// A function to parse the given address synchronously.
///
/// publicにしていますが、直接の使用は推奨されません。[Parser]の利用を検討してください。
#[cfg(feature = "blocking")]
pub fn parse_blocking(api: Arc<BlockingApi>, input: &str) -> ParseResult {
    let (rest, prefecture_name) = match read_prefecture(input) {
        None => {
            return ParseResult {
                address: Address::new("", "", "", input),
                error: Some(Error::new_parse_error(ParseErrorKind::Prefecture)),
            };
        }
        Some(result) => result,
    };
    let prefecture = match api.get_prefecture_master(prefecture_name) {
        Err(error) => {
            return ParseResult {
                address: Address::new(prefecture_name, "", "", rest),
                error: Some(error),
            };
        }
        Ok(result) => result,
    };
    let (rest, city_name) = match read_city(rest, prefecture) {
        None => {
            return ParseResult {
                address: Address::new(prefecture_name, "", "", rest),
                error: Some(Error::new_parse_error(ParseErrorKind::City)),
            };
        }
        Some(result) => result,
    };
    let city = match api.get_city_master(prefecture_name, &city_name) {
        Err(error) => {
            return ParseResult {
                address: Address::new(prefecture_name, &city_name, "", &rest),
                error: Some(error),
            };
        }
        Ok(result) => result,
    };
    let (rest, town_name) = match read_town(&rest, &city) {
        None => {
            return ParseResult {
                address: Address::new(prefecture_name, &city_name, "", &rest),
                error: Some(Error::new_parse_error(ParseErrorKind::Town)),
            };
        }
        Some(result) => result,
    };

    ParseResult {
        address: Address::new(prefecture_name, &city_name, &town_name, &rest),
        error: None,
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::api::BlockingApi;
    use crate::err::ParseErrorKind;
    use crate::parser::parse_blocking;

    #[test]
    fn parse_blocking_success_埼玉県秩父市熊木町8番15号() {
        let client: BlockingApi = Default::default();
        let result = parse_blocking(client.into(), "埼玉県秩父市熊木町8番15号");
        assert_eq!(result.address.prefecture, "埼玉県");
        assert_eq!(result.address.city, "秩父市");
        assert_eq!(result.address.town, "熊木町");
        assert_eq!(result.address.rest, "8番15号");
        assert_eq!(result.error, None);
    }

    #[test]
    fn parse_blocking_fail_市町村名が間違っている場合() {
        let client: BlockingApi = Default::default();
        let result = parse_blocking(client.into(), "埼玉県秩父柿熊木町8番15号");
        assert_eq!(result.address.prefecture, "埼玉県");
        assert_eq!(result.address.city, "");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "秩父柿熊木町8番15号");
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::City.to_string()
        );
    }
}
