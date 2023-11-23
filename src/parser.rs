use crate::api::Api;
use crate::entity::ParsedAddress;
use crate::parser::read_city::read_city;
use crate::parser::read_prefecture::read_prefecture;
use crate::parser::read_town::read_town;

mod read_city;
mod read_prefecture;
mod read_town;

pub async fn parse<T: Api>(api: T, input: &str) -> ParsedAddress {
    // 都道府県を特定
    let (rest, prefecture_name) = match read_prefecture(input) {
        None => return ParsedAddress::new("", "", "", input),
        Some(result) => result,
    };
    // その都道府県の市町村名リストを取得
    let prefecture = api.get_prefecture_master(prefecture_name).await.unwrap();
    // 市町村名を特定
    let (rest, city_name) = match read_city(rest, prefecture) {
        None => return ParsedAddress::new(prefecture_name, "", "", rest),
        Some(result) => result,
    };
    // その市町村の町名リストを取得
    let city = api
        .get_city_master(prefecture_name, city_name)
        .await
        .unwrap();
    // 町名を特定
    let (rest, town_name) = match read_town(rest, city) {
        None => return ParsedAddress::new(prefecture_name, city_name, "", rest),
        Some(result) => result,
    };

    ParsedAddress::new(prefecture_name, city_name, town_name, rest)
}

#[cfg(test)]
mod parser_tests {
    use crate::api::mock::ApiMock;
    use crate::api::wasm::ApiImplForWasm;
    use crate::parser::parse;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    #[tokio::test]
    async fn parse_mocked_success_神奈川県平塚市御殿二丁目() {
        let api = ApiMock { should_fail: false };
        let address = parse(api, "神奈川県平塚市御殿二丁目2-23").await;
        assert_eq!(address.prefecture, "神奈川県".to_string());
        assert_eq!(address.city, "平塚市".to_string());
        assert_eq!(address.town, "御殿二丁目".to_string());
        assert_eq!(address.rest, "2-23".to_string());
    }

    #[tokio::test]
    async fn parse_mocked_success_神奈川県平塚市桜ケ丘() {
        let api = ApiMock { should_fail: false };
        let address = parse(api, "神奈川県平塚市桜ケ丘100-1").await;
        assert_eq!(address.prefecture, "神奈川県".to_string());
        assert_eq!(address.city, "平塚市".to_string());
        assert_eq!(address.town, "桜ケ丘".to_string());
        assert_eq!(address.rest, "100-1".to_string());
    }

    #[tokio::test]
    async fn parse_mocked_fail_都道府県名が間違っている場合() {
        let api = ApiMock { should_fail: false };
        let address = parse(api, "神奈側県平塚市桜ケ丘100-1").await;
        assert_eq!(address.prefecture, "".to_string());
        assert_eq!(address.city, "".to_string());
        assert_eq!(address.town, "".to_string());
        assert_eq!(address.rest, "神奈側県平塚市桜ケ丘100-1".to_string());
    }

    #[tokio::test]
    async fn parse_mocked_fail_市町村名が間違っている場合() {
        let api = ApiMock { should_fail: false };
        let address = parse(api, "神奈川県平束市桜ケ丘100-1").await;
        assert_eq!(address.prefecture, "神奈川県".to_string());
        assert_eq!(address.city, "".to_string());
        assert_eq!(address.town, "".to_string());
        assert_eq!(address.rest, "平束市桜ケ丘100-1".to_string());
    }

    #[tokio::test]
    async fn parse_mocked_fail_町名が間違っている場合() {
        let api = ApiMock { should_fail: false };
        let address = parse(api, "神奈川県平塚市新百合ヶ丘100-1").await;
        assert_eq!(address.prefecture, "神奈川県".to_string());
        assert_eq!(address.city, "平塚市".to_string());
        assert_eq!(address.town, "".to_string());
        assert_eq!(address.rest, "新百合ヶ丘100-1".to_string());
    }

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn parse_wasm_success() {
        let api = ApiImplForWasm {};
        let address = parse(api, "兵庫県淡路市生穂新島8番地").await;
        assert_eq!(address.prefecture, "兵庫県".to_string());
        assert_eq!(address.city, "淡路市".to_string());
        assert_eq!(address.town, "生穂".to_string());
        assert_eq!(address.rest, "新島8番地".to_string());
    }
}
