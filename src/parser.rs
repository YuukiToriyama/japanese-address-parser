use crate::api::{Api, BlockingApi};
use crate::entity::{Address, ParseResult};
use crate::err::{Error, ParseErrorKind};
use crate::parser::read_city::read_city;
use crate::parser::read_prefecture::read_prefecture;
use crate::parser::read_town::read_town;

mod adapter;
mod filter;
mod read_city;
mod read_prefecture;
mod read_town;

pub async fn parse<T: Api>(api: T, input: &str) -> ParseResult {
    // 都道府県を特定
    let (rest, prefecture_name) = match read_prefecture(input) {
        None => {
            return ParseResult {
                address: Address::new("", "", "", input),
                error: Some(Error::new_parse_error(ParseErrorKind::Prefecture)),
            }
        }
        Some(result) => result,
    };
    // その都道府県の市町村名リストを取得
    let prefecture = match api.get_prefecture_master(prefecture_name).await {
        Err(error) => {
            return ParseResult {
                address: Address::new(prefecture_name, "", "", rest),
                error: Some(error),
            }
        }
        Ok(result) => result,
    };
    // 市町村名を特定
    let (rest, city_name) = match read_city(rest, prefecture) {
        None => {
            return ParseResult {
                address: Address::new(prefecture_name, "", "", rest),
                error: Some(Error::new_parse_error(ParseErrorKind::City)),
            }
        }
        Some(result) => result,
    };
    // その市町村の町名リストを取得
    let city = match api.get_city_master(prefecture_name, &city_name).await {
        Err(error) => {
            return ParseResult {
                address: Address::new(prefecture_name, &city_name, "", &rest),
                error: Some(error),
            }
        }
        Ok(result) => result,
    };
    // 町名を特定
    let (rest, town_name) = match read_town(&rest, &city) {
        None => {
            return ParseResult {
                address: Address::new(prefecture_name, &city_name, "", &rest),
                error: Some(Error::new_parse_error(ParseErrorKind::Town)),
            }
        }
        Some(result) => result,
    };

    ParseResult {
        address: Address::new(prefecture_name, &city_name, &town_name, &rest),
        error: None,
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::api::client::ApiImpl;
    use crate::api::mock::ApiMock;
    use crate::err::ParseErrorKind;
    use crate::parser::parse;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    #[tokio::test]
    async fn parse_mocked_success_神奈川県平塚市御殿二丁目() {
        let api = ApiMock { should_fail: false };
        let result = parse(api, "神奈川県平塚市御殿二丁目2-23").await;
        assert_eq!(result.address.prefecture, "神奈川県".to_string());
        assert_eq!(result.address.city, "平塚市".to_string());
        assert_eq!(result.address.town, "御殿二丁目".to_string());
        assert_eq!(result.address.rest, "2-23".to_string());
        assert_eq!(result.error, None);
    }

    #[tokio::test]
    async fn parse_mocked_success_神奈川県平塚市桜ケ丘() {
        let api = ApiMock { should_fail: false };
        let result = parse(api, "神奈川県平塚市桜ケ丘100-1").await;
        assert_eq!(result.address.prefecture, "神奈川県".to_string());
        assert_eq!(result.address.city, "平塚市".to_string());
        assert_eq!(result.address.town, "桜ケ丘".to_string());
        assert_eq!(result.address.rest, "100-1".to_string());
        assert_eq!(result.error, None);
    }

    #[tokio::test]
    async fn parse_mocked_fail_都道府県名が間違っている場合() {
        let api = ApiMock { should_fail: false };
        let result = parse(api, "神奈側県平塚市桜ケ丘100-1").await;
        assert_eq!(result.address.prefecture, "".to_string());
        assert_eq!(result.address.city, "".to_string());
        assert_eq!(result.address.town, "".to_string());
        assert_eq!(result.address.rest, "神奈側県平塚市桜ケ丘100-1".to_string());
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::Prefecture.to_string()
        );
    }

    #[tokio::test]
    async fn parse_mocked_fail_都道府県マスタの取得に失敗する() {
        let api = ApiMock { should_fail: true };
        let result = parse(api, "東京都新宿区西新宿二丁目8-1").await;
        assert_eq!(result.address.prefecture, "東京都".to_string());
        assert_eq!(result.address.city, "".to_string());
        assert_eq!(result.address.town, "".to_string());
        assert_eq!(result.address.rest, "新宿区西新宿二丁目8-1".to_string());
        assert_eq!(result.error.unwrap().error_type, "ApiError".to_string());
    }

    #[tokio::test]
    async fn parse_mocked_fail_市町村名が間違っている場合() {
        let api = ApiMock { should_fail: false };
        let result = parse(api, "神奈川県平束市桜ケ丘100-1").await;
        assert_eq!(result.address.prefecture, "神奈川県".to_string());
        assert_eq!(result.address.city, "".to_string());
        assert_eq!(result.address.town, "".to_string());
        assert_eq!(result.address.rest, "平束市桜ケ丘100-1".to_string());
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::City.to_string()
        );
    }

    #[tokio::test]
    async fn parse_mocked_fail_市区町村マスタの取得に失敗する() {
        // TODO: ApiMockの仕様を修正しないとこのテストコードは書けない
    }

    #[tokio::test]
    async fn parse_mocked_fail_町名が間違っている場合() {
        let api = ApiMock { should_fail: false };
        let result = parse(api, "神奈川県平塚市新百合ヶ丘100-1").await;
        assert_eq!(result.address.prefecture, "神奈川県".to_string());
        assert_eq!(result.address.city, "平塚市".to_string());
        assert_eq!(result.address.town, "".to_string());
        assert_eq!(result.address.rest, "新百合ヶ丘100-1".to_string());
        assert_eq!(
            result.error.unwrap().error_message,
            ParseErrorKind::Town.to_string()
        );
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn parse_wasm_success() {
        let api = ApiImpl {};
        let result = parse(api, "兵庫県淡路市生穂新島8番地").await;
        assert_eq!(result.address.prefecture, "兵庫県".to_string());
        assert_eq!(result.address.city, "淡路市".to_string());
        assert_eq!(result.address.town, "生穂".to_string());
        assert_eq!(result.address.rest, "新島8番地".to_string());
        assert_eq!(result.error, None);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_blocking<T: BlockingApi>(api: T, input: &str) -> ParseResult {
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod parse_blocking_tests {
    use crate::api;
    use crate::err::ParseErrorKind;
    use crate::parser::parse_blocking;

    #[test]
    fn parse_blocking_success_埼玉県秩父市熊木町8番15号() {
        let client = api::blocking::Client {};
        let result = parse_blocking(client, "埼玉県秩父市熊木町8番15号");
        assert_eq!(result.address.prefecture, "埼玉県");
        assert_eq!(result.address.city, "秩父市");
        assert_eq!(result.address.town, "熊木町");
        assert_eq!(result.address.rest, "8番15号");
        assert_eq!(result.error, None);
    }

    #[test]
    fn parse_blocking_fail_市町村名が間違っている場合() {
        let client = api::blocking::Client {};
        let result = parse_blocking(client, "埼玉県秩父柿熊木町8番15号");
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
