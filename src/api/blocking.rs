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
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::FETCH(endpoint)))
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
