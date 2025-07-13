use crate::domain::geolonia::entity::{City, Town};
use crate::domain::geolonia::error::Error;
use crate::http::client::ApiClient;

pub struct CityMasterRepository<C: ApiClient> {
    pub api_client: C,
}

impl<C: ApiClient> CityMasterRepository<C> {
    pub async fn get(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let server_url = "https://geolonia.github.io/japanese-addresses/api/ja";
        let endpoint = format!("{}/{}/{}.json", server_url, prefecture_name, city_name);
        match self.api_client.fetch::<Vec<Town>>(&endpoint).await {
            Ok(towns) => Ok(City {
                name: city_name.to_string(),
                towns,
            }),
            Err(error) => Err(error.into()),
        }
    }

    #[cfg(feature = "blocking")]
    pub fn get_blocking(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let server_url = "https://geolonia.github.io/japanese-addresses/api/ja";
        let endpoint = format!("{}/{}/{}.json", server_url, prefecture_name, city_name);
        match self.api_client.fetch_blocking::<Vec<Town>>(&endpoint) {
            Ok(towns) => Ok(City {
                name: city_name.to_string(),
                towns,
            }),
            Err(error) => Err(error.into()),
        }
    }
}

#[cfg(all(test, not(feature = "blocking")))]
mod async_tests {
    use crate::domain::geolonia::entity::Town;
    use crate::http::reqwest_client::ReqwestApiClient;
    use crate::repository::geolonia::city::CityMasterRepository;

    #[tokio::test]
    async fn 非同期_石川県羽咋郡志賀町_成功() {
        let repository = CityMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get("石川県", "羽咋郡志賀町").await;
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
        let repository = CityMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get("石川県", "敦賀市").await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            "https://geolonia.github.io/japanese-addresses/api/ja/石川県/敦賀市.jsonを取得できませんでした"
        );
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::domain::geolonia::entity::Town;
    use crate::http::reqwest_client::ReqwestApiClient;
    use crate::repository::geolonia::city::CityMasterRepository;

    #[test]
    fn 同期_石川県羽咋郡志賀町_成功() {
        let repository = CityMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get_blocking("石川県", "羽咋郡志賀町");
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
        let repository = CityMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get_blocking("石川県", "敦賀市");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            "https://geolonia.github.io/japanese-addresses/api/ja/石川県/敦賀市.jsonを取得できませんでした",
        );
    }
}
