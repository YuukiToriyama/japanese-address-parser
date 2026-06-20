mod pure;

use std::sync::Arc;

use crate::domain::common::token::Token;
use crate::domain::geolonia::entity::Address;
use crate::domain::geolonia::error::Error;
#[cfg(feature = "enable-api-client-cache")]
use crate::http::cached_client::CachedApiClient;
use crate::http::reqwest_client::ReqwestApiClient;
use crate::interactor::geolonia::{GeoloniaInteractor, GeoloniaInteractorImpl};
use crate::parser::pure::{PureParser, PureParserAction};
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
        let mut pure_parser = PureParser::new(address);

        loop {
            match pure_parser.advance() {
                PureParserAction::RequestCityNameList(pref_name) => {
                    match interactor.get_prefecture_master(&pref_name).await {
                        Ok(result) => pure_parser.provide_input(result.cities),
                        Err(error) => return pure_parser.abort(error),
                    }
                }
                PureParserAction::RequestTownNameList(pref_name, city_name) => {
                    match interactor.get_city_master(&pref_name, &city_name).await {
                        Ok(result) => {
                            let town_names = result.towns.into_iter().map(|x| x.name).collect();
                            pure_parser.provide_input(town_names);
                        }
                        Err(error) => return pure_parser.abort(error),
                    }
                }
                PureParserAction::Done(result) => return result,
            }
        }
    }

    /// Parses the given `address` synchronously.
    #[cfg(feature = "blocking")]
    pub fn parse_blocking(&self, address: &str) -> ParseResult {
        let interactor = self.interactor.clone();
        let mut pure_parser = PureParser::new(address);

        loop {
            match pure_parser.advance() {
                PureParserAction::RequestCityNameList(pref_name) => {
                    match interactor.get_blocking_prefecture_master(&pref_name) {
                        Ok(result) => pure_parser.provide_input(result.cities),
                        Err(error) => return pure_parser.abort(error),
                    }
                }
                PureParserAction::RequestTownNameList(pref_name, city_name) => {
                    match interactor.get_blocking_city_master(&pref_name, &city_name) {
                        Ok(result) => {
                            let town_names = result.towns.into_iter().map(|x| x.name).collect();
                            pure_parser.provide_input(town_names);
                        }
                        Err(error) => return pure_parser.abort(error),
                    }
                }
                PureParserAction::Done(result) => return result,
            }
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

#[derive(Serialize, PartialEq, Debug)]
pub struct ParseResult {
    pub address: Address,
    pub error: Option<Error>,
}
