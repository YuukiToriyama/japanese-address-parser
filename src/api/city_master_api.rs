use crate::entity::{City, Town};
use crate::err::{ApiErrorKind, Error};

pub struct CityMasterApi {
    server_url: &'static str,
}

impl CityMasterApi {
    async fn get(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
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
    fn get_blocking(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
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
