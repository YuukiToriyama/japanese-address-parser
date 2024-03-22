use crate::api::Api;
#[cfg(not(target_arch = "wasm32"))]
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

pub async fn parse<T: Api>(api: T, input: &str) -> ParseResult {
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

#[cfg(test)]
mod non_blocking_tests {
    use crate::api::city_master_api::CityMasterApi;
    use crate::api::prefecture_master_api::PrefectureMasterApi;
    use crate::api::{Api, ApiImpl};
    use crate::err::ParseErrorKind;
    use crate::parser::parse;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    #[tokio::test]
    async fn 都道府県名が誤っている場合() {
        let api = ApiImpl::new();
        let result = parse(api, "青盛県青森市長島１丁目１−１").await;
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
        let mut api = ApiImpl::new();
        api.prefecture_master_api = PrefectureMasterApi {
            server_url: "https://example.com/invalid_url/api/",
        };

        let result = parse(api, "青森県青森市長島１丁目１−１").await;
        assert_eq!(result.error.is_some(), true);
        assert_eq!(result.address.prefecture, "青森県");
        assert_eq!(result.address.city, "");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "青森市長島１丁目１−１");
    }

    #[tokio::test]
    async fn 市区町村名が誤っている場合() {
        let api = ApiImpl::new();
        let result = parse(api, "青森県青盛市長島１丁目１−１").await;
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
        let mut api = ApiImpl::new();
        api.city_master_api = CityMasterApi {
            server_url: "https://example.com/invalid_url/api/",
        };

        let result = parse(api, "青森県青森市長島１丁目１−１").await;
        assert_eq!(result.error.is_some(), true);
        assert_eq!(result.address.prefecture, "青森県");
        assert_eq!(result.address.city, "青森市");
        assert_eq!(result.address.town, "");
        assert_eq!(result.address.rest, "長島１丁目１−１");
    }

    #[tokio::test]
    async fn 町名が誤っている場合() {
        let api = ApiImpl::new();
        let result = parse(api, "青森県青森市永嶋１丁目１−１").await;
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
        let api = ApiImpl::new();
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
mod blocking_tests {
    use crate::api::{BlockingApi, BlockingApiImpl};
    use crate::err::ParseErrorKind;
    use crate::parser::parse_blocking;

    #[test]
    fn parse_blocking_success_埼玉県秩父市熊木町8番15号() {
        let client = BlockingApiImpl::new();
        let result = parse_blocking(client, "埼玉県秩父市熊木町8番15号");
        assert_eq!(result.address.prefecture, "埼玉県");
        assert_eq!(result.address.city, "秩父市");
        assert_eq!(result.address.town, "熊木町");
        assert_eq!(result.address.rest, "8番15号");
        assert_eq!(result.error, None);
    }

    #[test]
    fn parse_blocking_fail_市町村名が間違っている場合() {
        let client = BlockingApiImpl::new();
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
