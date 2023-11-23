use crate::api::Api;
use crate::entity::{City, Prefecture, Town};
use crate::err::{ApiErrorKind, Error};
use gloo_net::http::Request;

pub struct ApiImplForWasm {}

impl Api for ApiImplForWasm {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let endpoint = format!(
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/{}/master.json",
            prefecture_name
        );
        let response = match Request::get(&endpoint).send().await {
            Ok(result) => result,
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::FETCH(endpoint))),
        };
        if response.ok() {
            match response.json::<Prefecture>().await {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::new_api_error(ApiErrorKind::DESERIALIZE(endpoint))),
            }
        } else {
            Err(Error::new_api_error(ApiErrorKind::FETCH(endpoint)))
        }
    }

    async fn get_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> Result<City, Error> {
        let endpoint = format!(
            "https://geolonia.github.io/japanese-addresses/api/ja/{}/{}.json",
            prefecture_name, city_name
        );
        let response = match Request::get(&endpoint).send().await {
            Ok(result) => result,
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::FETCH(endpoint))),
        };
        if response.ok() {
            match response.json::<Vec<Town>>().await {
                Ok(towns) => Ok(City {
                    name: city_name.to_string(),
                    towns,
                }),
                Err(_) => Err(Error::new_api_error(ApiErrorKind::DESERIALIZE(endpoint)))
            }
        } else {
            Err(Error::new_api_error(
                ApiErrorKind::FETCH(endpoint)
            ))
        }
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod api_tests {
    use crate::api::wasm::ApiImplForWasm;
    use crate::api::Api;
    use crate::entity::Town;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn get_prefecture_master_success() {
        let api = ApiImplForWasm {};
        let prefecture = api.get_prefecture_master("富山県").await.unwrap();
        assert_eq!(prefecture.name, "富山県".to_string());
        let cities = vec![
            "富山市".to_string(),
            "高岡市".to_string(),
            "魚津市".to_string(),
            "氷見市".to_string(),
            "滑川市".to_string(),
            "黒部市".to_string(),
            "砺波市".to_string(),
            "小矢部市".to_string(),
            "南砺市".to_string(),
            "射水市".to_string(),
            "中新川郡舟橋村".to_string(),
            "中新川郡上市町".to_string(),
            "中新川郡立山町".to_string(),
            "下新川郡入善町".to_string(),
            "下新川郡朝日町".to_string(),
        ];
        for city in cities {
            assert!(prefecture.cities.contains(&city));
        }
    }

    #[wasm_bindgen_test]
    async fn get_prefecture_master_fail() {
        let api = ApiImplForWasm {};
        let result = api.get_prefecture_master("大阪都").await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/大阪都/master.jsonを取得できませんでした".to_string()
        )
    }

    #[wasm_bindgen_test]
    async fn get_city_master_success() {
        let api = ApiImplForWasm {};
        let city = api.get_city_master("石川県", "羽咋郡志賀町").await.unwrap();
        assert_eq!(city.name, "羽咋郡志賀町".to_string());
        let town = Town {
            name: "末吉".to_string(),
            koaza: "千古".to_string(),
            lat: Some(37.006235),
            lng: Some(136.779155),
        };
        assert!(city.towns.contains(&town));
    }

    #[wasm_bindgen_test]
    async fn get_city_master_fail() {
        let api = ApiImplForWasm {};
        let result = api.get_city_master("石川県", "敦賀市").await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            "https://geolonia.github.io/japanese-addresses/api/ja/石川県/敦賀市.jsonを取得できませんでした".to_string()
        );
    }
}
