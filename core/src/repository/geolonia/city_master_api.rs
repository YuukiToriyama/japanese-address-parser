use crate::domain::geolonia::entity::{City, Town};
use crate::domain::geolonia::error::Error;
use crate::service::geolonia::GeoloniaApiService;

pub struct CityMasterApi {
    pub server_url: &'static str,
}

impl Default for CityMasterApi {
    fn default() -> Self {
        Self {
            server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
        }
    }
}

impl CityMasterApi {
    pub async fn get(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let endpoint = format!("{}/{}/{}.json", self.server_url, prefecture_name, city_name);
        let api_service = GeoloniaApiService {};
        let towns = api_service.get::<Vec<Town>>(&endpoint).await?;
        Ok(City {
            name: city_name.to_string(),
            towns,
        })
    }
    #[cfg(feature = "blocking")]
    pub fn get_blocking(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let endpoint = format!("{}/{}/{}.json", self.server_url, prefecture_name, city_name);
        let api_service = GeoloniaApiService {};
        let towns = api_service.get_blocking::<Vec<Town>>(&endpoint)?;
        Ok(City {
            name: city_name.to_string(),
            towns,
        })
    }
}

#[cfg(all(test, not(feature = "blocking")))]
mod tests {
    use crate::domain::geolonia::entity::Town;

    use crate::repository::geolonia::city_master_api::CityMasterApi;

    #[tokio::test]
    async fn 非同期_石川県羽咋郡志賀町_成功() {
        let city_master_api: CityMasterApi = Default::default();
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

    #[tokio::test]
    async fn 非同期_誤った市区町村名_失敗() {
        let city_master_api: CityMasterApi = Default::default();
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
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::domain::geolonia::entity::Town;
    use crate::repository::geolonia::city_master_api::CityMasterApi;

    #[test]
    fn 同期_石川県羽咋郡志賀町_成功() {
        let city_master_api: CityMasterApi = Default::default();
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

    #[test]
    fn 同期_誤った市区町村名_失敗() {
        let city_master_api: CityMasterApi = Default::default();
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
