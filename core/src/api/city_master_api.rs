use crate::entity::{City, Town};
use crate::err::{ApiErrorKind, Error};

pub struct CityMasterApi {
    pub server_url: &'static str,
}

impl CityMasterApi {
    pub async fn get(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let endpoint = format!("{}/{}/{}.json", self.server_url, prefecture_name, city_name);
        let response = match reqwest::get(&endpoint).await {
            Ok(result) => result,
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::Deserialize(endpoint))),
        };
        if response.status() == 200 {
            match response.json::<Vec<Town>>().await {
                Ok(result) => Ok(City {
                    name: city_name.to_string(),
                    towns: result,
                }),
                Err(_) => Err(Error::new_api_error(ApiErrorKind::Deserialize(endpoint))),
            }
        } else {
            Err(Error::new_api_error(ApiErrorKind::Fetch(endpoint)))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_blocking(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let endpoint = format!("{}/{}/{}.json", self.server_url, prefecture_name, city_name);
        let response = match reqwest::blocking::get(&endpoint) {
            Ok(result) => result,
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::Fetch(endpoint))),
        };
        if response.status() == 200 {
            match response.json::<Vec<Town>>() {
                Ok(result) => Ok(City {
                    name: city_name.to_string(),
                    towns: result,
                }),
                Err(_) => Err(Error::new_api_error(ApiErrorKind::Deserialize(endpoint))),
            }
        } else {
            Err(Error::new_api_error(ApiErrorKind::Fetch(endpoint)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::city_master_api::CityMasterApi;
    use crate::entity::Town;

    #[tokio::test]
    async fn 非同期_石川県羽咋郡志賀町_成功() {
        let city_master_api = CityMasterApi {
            server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
        };
        let result = city_master_api.get("石川県", "羽咋郡志賀町").await;
        let city = result.unwrap();
        assert_eq!(city.name, "羽咋郡志賀町");
        let town = Town {
            name: "末吉".to_string(),
            koaza: "千古".to_string(),
            lat: Some(37.006235),
            lng: Some(136.779155),
        };
        assert!(city.towns.contains(&town));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn 同期_石川県羽咋郡志賀町_成功() {
        let city_master_api = CityMasterApi {
            server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
        };
        let result = city_master_api.get_blocking("石川県", "羽咋郡志賀町");
        let city = result.unwrap();
        assert_eq!(city.name, "羽咋郡志賀町");
        let town = Town {
            name: "末吉".to_string(),
            koaza: "千古".to_string(),
            lat: Some(37.006235),
            lng: Some(136.779155),
        };
        assert!(city.towns.contains(&town));
    }

    #[tokio::test]
    async fn 非同期_誤った市区町村名_失敗() {
        let city_master_api = CityMasterApi {
            server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
        };
        let result = city_master_api.get("石川県", "敦賀市").await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            format!(
                "{}/石川県/敦賀市.jsonを取得できませんでした",
                city_master_api.server_url
            )
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn 同期_誤った市区町村名_失敗() {
        let city_master_api = CityMasterApi {
            server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
        };
        let result = city_master_api.get_blocking("石川県", "敦賀市");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            format!(
                "{}/石川県/敦賀市.jsonを取得できませんでした",
                city_master_api.server_url
            )
        );
    }
}
