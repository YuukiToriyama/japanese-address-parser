use crate::entity::Prefecture;
use crate::err::{ApiErrorKind, Error};

pub struct PrefectureMasterApi {
    server_url: &'static str,
}

impl PrefectureMasterApi {
    async fn get(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let endpoint = format!("{}/{}/master.json", self.server_url, prefecture_name);
        let response = match reqwest::get(&endpoint).await {
            Ok(result) => result,
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::Fetch(endpoint))),
        };
        if response.status() == 200 {
            match response.json::<Prefecture>().await {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::new_api_error(ApiErrorKind::Deserialize(endpoint))),
            }
        } else {
            Err(Error::new_api_error(ApiErrorKind::Fetch(endpoint)))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_blocking(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let endpoint = format!("{}/{}/master.json", self.server_url, prefecture_name);
        let response = match reqwest::blocking::get(&endpoint) {
            Ok(result) => result,
            Err(_) => return Err(Error::new_api_error(ApiErrorKind::Fetch(endpoint))),
        };
        if response.status() == 200 {
            match response.json::<Prefecture>() {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::new_api_error(ApiErrorKind::Deserialize(endpoint))),
            }
        } else {
            Err(Error::new_api_error(ApiErrorKind::Fetch(endpoint)))
        }
    }
}
