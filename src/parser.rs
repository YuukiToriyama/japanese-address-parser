use crate::api::Api;
use crate::entity::{Address, ParseResult};
use crate::err::{Error, ParseErrorKind};
use crate::parser::read_city::read_city;
use crate::parser::read_prefecture::read_prefecture;
use crate::parser::read_town::read_town;

mod read_city;
mod read_prefecture;
mod read_town;

pub async fn parse<T: Api>(api: T, input: &str) -> ParseResult {
    // 都道府県を特定
    let (rest, prefecture_name) = match read_prefecture(input) {
        None => {
            return ParseResult {
                address: Address::new("", "", "", input),
                error: Some(Error::new_parse_error(ParseErrorKind::PREFECTURE)),
            }
        }
        Some(result) => result,
    };
    // その都道府県の市町村名リストを取得
    let prefecture = api.get_prefecture_master(prefecture_name).await.unwrap();
    // 市町村名を特定
    let (rest, city_name) = match read_city(rest, prefecture) {
        None => {
            return ParseResult {
                address: Address::new(prefecture_name, "", "", rest),
                error: Some(Error::new_parse_error(ParseErrorKind::CITY)),
            }
        }
        Some(result) => result,
    };
    // その市町村の町名リストを取得
    let city = api
        .get_city_master(prefecture_name, city_name)
        .await
        .unwrap();
    // 町名を特定
    let (rest, town_name) = match read_town(rest, city) {
        None => {
            return ParseResult {
                address: Address::new(prefecture_name, city_name, "", rest),
                error: Some(Error::new_parse_error(ParseErrorKind::TOWN)),
            }
        }
        Some(result) => result,
    };

    ParseResult {
        address: Address::new(prefecture_name, city_name, town_name, rest),
        error: None,
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::api::mock::ApiMock;
    use crate::api::wasm::ApiImplForWasm;
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
            ParseErrorKind::PREFECTURE.to_string()
        );
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
            ParseErrorKind::CITY.to_string()
        );
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
            ParseErrorKind::TOWN.to_string()
        );
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn parse_wasm_success() {
        let api = ApiImplForWasm {};
        let result = parse(api, "兵庫県淡路市生穂新島8番地").await;
        assert_eq!(result.address.prefecture, "兵庫県".to_string());
        assert_eq!(result.address.city, "淡路市".to_string());
        assert_eq!(result.address.town, "生穂".to_string());
        assert_eq!(result.address.rest, "新島8番地".to_string());
        assert_eq!(result.error, None);
    }
}
