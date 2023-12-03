use crate::api::BlockingApi;
use crate::entity::{City, Prefecture, Town};
use crate::err::{ApiErrorKind, Error};

pub struct Client {}

impl BlockingApi for Client {
    fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let endpoint = format!(
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/{}/master.json",
            prefecture_name
        );
        let response = match reqwest::blocking::get(&endpoint) {
            Ok(result) => result,
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::FETCH(endpoint))),
        };
        if response.status() == 200 {
            match response.json::<Prefecture>() {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::new_api_error(ApiErrorKind::DESERIALIZE(endpoint))),
            }
        } else {
            Err(Error::new_api_error(ApiErrorKind::FETCH(endpoint)))
        }
    }

    fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let endpoint = format!(
            "https://geolonia.github.io/japanese-addresses/api/ja/{}/{}.json",
            prefecture_name, city_name
        );
        let response = match reqwest::blocking::get(&endpoint) {
            Ok(result) => result,
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::FETCH(endpoint))),
        };
        if response.status() == 200 {
            match response.json::<Vec<Town>>() {
                Ok(result) => Ok(City {
                    name: city_name.to_string(),
                    towns: result,
                }),
                Err(_) => Err(Error::new_api_error(ApiErrorKind::DESERIALIZE(endpoint))),
            }
        } else {
            Err(Error::new_api_error(ApiErrorKind::FETCH(endpoint)))
        }
    }
}

#[cfg(test)]
mod blocking_client_tests {
    use crate::api::blocking::Client;
    use crate::api::BlockingApi;
    use crate::entity::{Prefecture, Town};

    #[test]
    fn get_prefecture_master_成功_香川県() {
        let client = Client {};
        let result = client.get_prefecture_master("香川県");
        assert!(result.is_ok());
        let prefecture = result.unwrap();
        assert_eq!(
            prefecture,
            Prefecture::new(
                "香川県",
                vec![
                    "高松市",
                    "丸亀市",
                    "坂出市",
                    "善通寺市",
                    "観音寺市",
                    "さぬき市",
                    "東かがわ市",
                    "三豊市",
                    "小豆郡土庄町",
                    "小豆郡小豆島町",
                    "木田郡三木町",
                    "香川郡直島町",
                    "綾歌郡宇多津町",
                    "綾歌郡綾川町",
                    "仲多度郡琴平町",
                    "仲多度郡多度津町",
                    "仲多度郡まんのう町",
                ],
            )
        );
    }

    #[test]
    fn get_prefecture_master_失敗_都道府県名が誤っている() {
        let client = Client {};
        let result = client.get_prefecture_master("東京県");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error_message, "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/東京県/master.jsonを取得できませんでした");
    }

    #[test]
    fn get_city_master_成功_香川県坂出市() {
        let client = Client {};
        let result = client.get_city_master("香川県", "坂出市");
        assert!(result.is_ok());
        let city = result.unwrap();
        assert_eq!(city.name, "坂出市");
        let town = Town::new("青葉町", "", 34.307609, 133.85252);
        assert!(city.towns.contains(&town));
    }

    #[test]
    fn get_city_master_失敗_市町村名が誤っている() {
        let client = Client {};
        let result = client.get_city_master("東京都", "東京市");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error_message, "https://geolonia.github.io/japanese-addresses/api/ja/東京都/東京市.jsonを取得できませんでした")
    }
}
