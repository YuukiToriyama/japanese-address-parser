use std::sync::Arc;

use crate::domain::common::token::Token;
use crate::domain::geolonia::entity::Address;
use crate::domain::geolonia::error::{Error, ParseErrorKind};
#[cfg(feature = "enable-api-client-cache")]
use crate::http::cached_client::CachedApiClient;
use crate::http::reqwest_client::ReqwestApiClient;
use crate::interactor::geolonia::{GeoloniaInteractor, GeoloniaInteractorImpl};
use crate::tokenizer::{End, Tokenizer};
use serde::Serialize;

impl From<Tokenizer<End>> for Address {
    fn from(value: Tokenizer<End>) -> Self {
        let mut address = Address::new("", "", "", "");
        for token in value.tokens {
            match token {
                Token::Prefecture(prefecture_name) => address.prefecture = prefecture_name,
                Token::City(city_name) => address.city = city_name,
                Token::Town(town_name) => address.town = town_name,
                Token::Rest(rest) => address.rest = rest,
            }
        }
        address
    }
}

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
    #[cfg(not(feature = "enable-api-client-cache"))]
    interactor: Arc<GeoloniaInteractorImpl<ReqwestApiClient>>,
    #[cfg(feature = "enable-api-client-cache")]
    interactor: Arc<GeoloniaInteractorImpl<CachedApiClient<ReqwestApiClient>>>,
}

impl Default for Parser {
    /// Constructs a new `Parser`.
    fn default() -> Self {
        Self {
            interactor: Arc::new(Default::default()),
        }
    }
}

impl Parser {
    /// Parses the given `address` asynchronously.
    pub async fn parse(&self, address: &str) -> ParseResult {
        let interactor = self.interactor.clone();
        let tokenizer = Tokenizer::new(address);
        // 都道府県を特定
        let (prefecture, tokenizer) = match tokenizer.read_prefecture() {
            Ok(found) => found,
            Err(tokenizer) => {
                return ParseResult {
                    address: Address::from(tokenizer),
                    error: Some(Error::new_parse_error(ParseErrorKind::Prefecture)),
                }
            }
        };
        // その都道府県の市町村名リストを取得
        let prefecture_master = match interactor.get_prefecture_master(prefecture.name_ja()).await {
            Err(error) => {
                return ParseResult {
                    address: Address::from(tokenizer.finish()),
                    error: Some(error),
                };
            }
            Ok(result) => result,
        };
        // 市町村名を特定
        let (city_name, tokenizer) = match tokenizer.read_city(&prefecture_master.cities) {
            Ok(found) => found,
            Err(not_found) => {
                // 市区町村が特定できない場合かつフィーチャフラグが有効な場合、郡名が抜けている可能性を検討
                match not_found.read_city_with_county_name_completion(&prefecture_master.cities) {
                    Ok(found) if cfg!(feature = "city-name-correction") => found,
                    _ => {
                        // それでも見つからない場合は終了
                        return ParseResult {
                            address: Address::from(tokenizer.finish()),
                            error: Some(Error::new_parse_error(ParseErrorKind::City)),
                        };
                    }
                }
            }
        };
        // その市町村の町名リストを取得
        let city = match interactor
            .get_city_master(prefecture.name_ja(), &city_name)
            .await
        {
            Err(error) => {
                return ParseResult {
                    address: Address::from(tokenizer.finish()),
                    error: Some(error),
                };
            }
            Ok(result) => result,
        };
        // 町名を特定
        let Ok((_, tokenizer)) =
            tokenizer.read_town(city.towns.iter().map(|x| x.name.clone()).collect())
        else {
            return ParseResult {
                address: Address::from(tokenizer.finish()),
                error: Some(Error::new_parse_error(ParseErrorKind::Town)),
            };
        };

        ParseResult {
            address: Address::from(tokenizer.finish()),
            error: None,
        }
    }

    /// Parses the given `address` synchronously.
    #[cfg(feature = "blocking")]
    pub fn parse_blocking(&self, address: &str) -> ParseResult {
        let interactor = self.interactor.clone();
        let tokenizer = Tokenizer::new(address);
        let (prefecture, tokenizer) = match tokenizer.read_prefecture() {
            Ok(found) => found,
            Err(tokenizer) => {
                return ParseResult {
                    address: Address::from(tokenizer),
                    error: Some(Error::new_parse_error(ParseErrorKind::Prefecture)),
                }
            }
        };
        let prefecture_master =
            match interactor.get_blocking_prefecture_master(prefecture.name_ja()) {
                Err(error) => {
                    return ParseResult {
                        address: Address::from(tokenizer.finish()),
                        error: Some(error),
                    };
                }
                Ok(result) => result,
            };
        let (city_name, tokenizer) = match tokenizer.read_city(&prefecture_master.cities) {
            Ok(found) => found,
            Err(not_found) => {
                match not_found.read_city_with_county_name_completion(&prefecture_master.cities) {
                    Ok(found) if cfg!(feature = "city-name-correction") => found,
                    _ => {
                        return ParseResult {
                            address: Address::from(tokenizer.finish()),
                            error: Some(Error::new_parse_error(ParseErrorKind::City)),
                        };
                    }
                }
            }
        };
        let city = match interactor.get_blocking_city_master(prefecture.name_ja(), &city_name) {
            Err(error) => {
                return ParseResult {
                    address: Address::from(tokenizer.finish()),
                    error: Some(error),
                };
            }
            Ok(result) => result,
        };
        let Ok((_, tokenizer)) =
            tokenizer.read_town(city.towns.iter().map(|x| x.name.clone()).collect())
        else {
            return ParseResult {
                address: Address::from(tokenizer.finish()),
                error: Some(Error::new_parse_error(ParseErrorKind::Town)),
            };
        };

        ParseResult {
            address: Address::from(tokenizer.finish()),
            error: None,
        }
    }
}

#[cfg(all(test, not(feature = "blocking")))]
mod tests {
    use crate::domain::geolonia::error::ParseErrorKind;
    use crate::parser::Parser;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    #[tokio::test]
    async fn 都道府県名が誤っている場合() {
        let parser = Parser::default();
        let result = parser.parse("青盛県青森市長島１丁目１−１").await;
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
    async fn 市区町村名が誤っている場合() {
        let parser = Parser::default();
        let result = parser.parse("青森県青盛市長島１丁目１−１").await;
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
    async fn 町名が誤っている場合() {
        let parser = Parser::default();
        let result = parser.parse("青森県青森市永嶋１丁目１−１").await;
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
        let parser = Parser::default();
        let result = parser.parse("兵庫県淡路市生穂新島8番地").await;
        assert_eq!(result.address.prefecture, "兵庫県".to_string());
        assert_eq!(result.address.city, "淡路市".to_string());
        assert_eq!(result.address.town, "生穂".to_string());
        assert_eq!(result.address.rest, "新島8番地".to_string());
        assert_eq!(result.error, None);
    }
}

/// Tests specifically for the `enable-api-client-cache` feature.
///
/// These tests verify that `Parser` behaves correctly when it is backed by
/// `CachedApiClient<ReqwestApiClient>` instead of the plain `ReqwestApiClient`.
/// The caching layer must be transparent to callers: parse results must be
/// identical regardless of whether a response comes from the network or the
/// in-memory cache.
#[cfg(all(test, feature = "enable-api-client-cache", not(feature = "blocking")))]
mod cached_client_tests {
    use crate::domain::geolonia::error::ParseErrorKind;
    use crate::parser::Parser;

    // --- success path ---

    /// A successful parse must still succeed when the cache feature is active.
    /// Calling parse() a second time with the same address exercises the cache
    /// warm path; both results must be identical.
    #[tokio::test]
    async fn キャッシュ有効時に正常な住所をパースできること() {
        let parser = Parser::default();
        let address = "青森県青森市長島１丁目１−１";

        let first = parser.parse(address).await;
        let second = parser.parse(address).await;

        assert_eq!(first.address.prefecture, "青森県");
        assert_eq!(first.address.city, "青森市");
        assert!(first.error.is_none());

        // Second call should return the same data (served from cache).
        assert_eq!(first.address, second.address);
        assert_eq!(first.error, second.error);
    }

    /// Repeated calls with different addresses must not interfere with each
    /// other via the cache.
    #[tokio::test]
    async fn キャッシュ有効時に異なる住所を連続してパースできること() {
        let parser = Parser::default();

        let result_a = parser.parse("青森県青森市長島１丁目１−１").await;
        let result_b = parser.parse("兵庫県淡路市生穂新島8番地").await;

        assert_eq!(result_a.address.prefecture, "青森県");
        assert_eq!(result_a.address.city, "青森市");
        assert!(result_a.error.is_none());

        assert_eq!(result_b.address.prefecture, "兵庫県");
        assert_eq!(result_b.address.city, "淡路市");
        assert!(result_b.error.is_none());
    }

    // --- error paths ---

    /// An invalid prefecture must still return a Prefecture error when the
    /// cache feature is active.
    #[tokio::test]
    async fn キャッシュ有効時に都道府県名が誤っている場合はエラーになること() {
        let parser = Parser::default();
        let result = parser.parse("青盛県青森市長島１丁目１−１").await;

        assert_eq!(result.address.prefecture, "");
        assert_eq!(result.address.city, "");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "青盛県青森市長島１丁目１−１");
        assert!(result.error.is_some());
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::Prefecture.to_string()
        );
    }

    /// An invalid city must still return a City error when the cache feature
    /// is active.
    #[tokio::test]
    async fn キャッシュ有効時に市区町村名が誤っている場合はエラーになること() {
        let parser = Parser::default();
        let result = parser.parse("青森県青盛市長島１丁目１−１").await;

        assert_eq!(result.address.prefecture, "青森県");
        assert_eq!(result.address.city, "");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "青盛市長島１丁目１−１");
        assert!(result.error.is_some());
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::City.to_string()
        );
    }

    /// An invalid town must still return a Town error when the cache feature
    /// is active.
    #[tokio::test]
    async fn キャッシュ有効時に町名が誤っている場合はエラーになること() {
        let parser = Parser::default();
        let result = parser.parse("青森県青森市永嶋１丁目１−１").await;

        assert_eq!(result.address.prefecture, "青森県");
        assert_eq!(result.address.city, "青森市");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "永嶋１丁目１−１");
        assert!(result.error.is_some());
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::Town.to_string()
        );
    }

    /// Calling parse() on the same invalid address twice must return
    /// identical (cached) errors – the cache must not swallow errors or
    /// transform them.
    #[tokio::test]
    async fn キャッシュ有効時にエラーは繰り返し呼び出しても一貫していること() {
        let parser = Parser::default();
        let address = "青盛県青森市長島１丁目１−１"; // bad prefecture

        let first = parser.parse(address).await;
        let second = parser.parse(address).await;

        // Both calls must report a prefecture error.
        assert!(first.error.is_some());
        assert!(second.error.is_some());
        assert_eq!(
            first.error.unwrap().error_message,
            second.error.unwrap().error_message
        );
        // rest field must be identical in both results.
        assert_eq!(first.address.rest, second.address.rest);
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::domain::geolonia::error::ParseErrorKind;
    use crate::parser::Parser;

    #[test]
    fn parse_blocking_success_埼玉県秩父市熊木町8番15号() {
        let parser = Parser::default();
        let result = parser.parse_blocking("埼玉県秩父市熊木町8番15号");
        assert_eq!(result.address.prefecture, "埼玉県");
        assert_eq!(result.address.city, "秩父市");
        assert_eq!(result.address.town, "熊木町");
        assert_eq!(result.address.rest, "8番15号");
        assert_eq!(result.error, None);
    }

    #[test]
    fn parse_blocking_fail_市町村名が間違っている場合() {
        let parser = Parser::default();
        let result = parser.parse_blocking("埼玉県秩父柿熊木町8番15号");
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

/// Blocking tests for the `enable-api-client-cache` + `blocking` feature combination.
///
/// When both features are active Parser uses `CachedApiClient<ReqwestApiClient>` AND
/// exposes a synchronous `parse_blocking` method. These tests verify that the
/// blocking surface is unaffected by the caching layer.
#[cfg(all(test, feature = "blocking", feature = "enable-api-client-cache"))]
mod blocking_cached_client_tests {
    use crate::domain::geolonia::error::ParseErrorKind;
    use crate::parser::Parser;

    /// A successful blocking parse must still succeed when the cache feature is
    /// active. Calling parse_blocking() twice with the same address exercises
    /// the cache warm path; both results must be identical.
    #[test]
    fn キャッシュ有効時に正常な住所をブロッキングでパースできること() {
        let parser = Parser::default();
        let address = "埼玉県秩父市熊木町8番15号";

        let first = parser.parse_blocking(address);
        let second = parser.parse_blocking(address);

        assert_eq!(first.address.prefecture, "埼玉県");
        assert_eq!(first.address.city, "秩父市");
        assert_eq!(first.address.town, "熊木町");
        assert_eq!(first.address.rest, "8番15号");
        assert!(first.error.is_none());

        // Second call must return identical data from cache.
        assert_eq!(first.address, second.address);
        assert_eq!(first.error, second.error);
    }

    /// A City error must still be returned when the cache feature is active
    /// and the blocking API is used.
    #[test]
    fn キャッシュ有効時に市区町村名が誤っている場合はブロッキングでエラーになること() {
        let parser = Parser::default();
        let result = parser.parse_blocking("埼玉県秩父柿熊木町8番15号");

        assert_eq!(result.address.prefecture, "埼玉県");
        assert_eq!(result.address.city, "");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "秩父柿熊木町8番15号");
        assert!(result.error.is_some());
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::City.to_string()
        );
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct ParseResult {
    pub address: Address,

}
