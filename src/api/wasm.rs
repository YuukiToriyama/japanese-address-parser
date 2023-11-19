use crate::api::Api;
use crate::entity::{City, Prefecture, Town};
use gloo_net::http::Request;

pub struct ApiImplForWasm {}

impl Api for ApiImplForWasm {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, String> {
        let endpoint = format!(
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/{}/master.json",
            prefecture_name
        );
        let response = Request::get(&endpoint).send().await.unwrap();
        if response.ok() {
            let prefecture = response.json::<Prefecture>().await.unwrap();
            Ok(prefecture)
        } else {
            Err(format!("Failed to fetch {}", &endpoint))
        }
    }

    async fn get_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> Result<City, String> {
        let endpoint = format!(
            "https://geolonia.github.io/japanese-addresses/api/ja/{}/{}.json",
            prefecture_name, city_name
        );
        let response = Request::get(&endpoint).send().await.unwrap();
        if response.ok() {
            let towns = response.json::<Vec<Town>>().await.unwrap();
            Ok(City {
                name: city_name.to_string(),
                towns,
            })
        } else {
            Err(format!("Failed to fetch {}", &endpoint))
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
    #[should_panic]
    async fn get_prefecture_master_fail() {
        let api = ApiImplForWasm {};
        api.get_prefecture_master("大阪都").await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn get_city_master_success() {
        let api = ApiImplForWasm {};
        let city = api.get_city_master("石川県", "羽咋郡志賀町").await.unwrap();
        assert_eq!(city.name, "羽咋郡志賀町".to_string());
        let town = Town {
            name: "末吉".to_string(),
            koaza: "千古".to_string(),
            lat: 37.006235,
            lng: 136.779155,
        };
        assert!(city.towns.contains(&town));
    }

    #[wasm_bindgen_test]
    #[should_panic]
    async fn get_city_master_fail() {
        let api = ApiImplForWasm {};
        api.get_city_master("石川県", "敦賀市").await.unwrap();
    }
}
