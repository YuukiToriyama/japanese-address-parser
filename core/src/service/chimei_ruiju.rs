use crate::domain::chimei_ruiju::error::ApiError;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub struct ChimeiRuijuApiService {}

impl ChimeiRuijuApiService {
    pub async fn get<T>(&self, url: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::get(url).await.map_err(|_| ApiError::Network {
            url: url.to_string(),
        })?;
        if response.status() == StatusCode::NOT_FOUND {
            return Err(ApiError::NotFound {
                url: url.to_string(),
            });
        }
        response
            .json::<T>()
            .await
            .map_err(|_| ApiError::Deserialize {
                url: url.to_string(),
            })
    }
}

#[cfg(feature = "blocking")]
impl ChimeiRuijuApiService {
    pub fn get_blocking<T>(&self, url: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::blocking::get(url).map_err(|_| ApiError::Network {
            url: url.to_string(),
        })?;
        if response.status() == StatusCode::NOT_FOUND {
            return Err(ApiError::NotFound {
                url: url.to_string(),
            });
        }
        response.json::<T>().map_err(|_| ApiError::Deserialize {
            url: url.to_string(),
        })
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod async_tests {
    use crate::domain::chimei_ruiju::entity::PrefectureMaster;
    use crate::domain::chimei_ruiju::error::ApiError;
    use crate::service::chimei_ruiju::ChimeiRuijuApiService;

    #[tokio::test]
    async fn 失敗_ネットワークエラー() {
        let invalid_url = "htttp://chimei-ruiju.org";
        let api_service = ChimeiRuijuApiService {};

        let result = api_service.get::<PrefectureMaster>(invalid_url).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error,
            ApiError::Network {
                url: invalid_url.to_string(),
            }
        );
    }

    #[tokio::test]
    async fn 失敗_404エラー() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(404)
            .create_async()
            .await;

        let api_service = ChimeiRuijuApiService {};

        let result = api_service.get::<PrefectureMaster>(&url).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, ApiError::NotFound { url: url.clone() });

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn 失敗_デシリアライズエラー() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"hoge": true, "piyo": 100}"#)
            .create_async()
            .await;

        let api_service = ChimeiRuijuApiService {};

        let result = api_service.get::<PrefectureMaster>(&url).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, ApiError::Deserialize { url: url.clone() });

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn 成功() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "新浜県", "cities": ["新浜市"], "coordinate": {"latitude": 34.6570413, "longitude": 135.2741341}}"#)
            .create_async()
            .await;

        let api_service = ChimeiRuijuApiService {};

        let result = api_service.get::<PrefectureMaster>(&url).await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "新浜県");
        assert_eq!(entity.cities, vec!["新浜市"]);
        assert_eq!(entity.coordinate.latitude, 34.6570413);
        assert_eq!(entity.coordinate.longitude, 135.2741341);

        mock.assert_async().await;
    }
}

#[cfg(all(test, feature = "blocking", not(target_arch = "wasm32")))]
mod blocking_tests {
    use crate::domain::chimei_ruiju::entity::PrefectureMaster;
    use crate::domain::chimei_ruiju::error::ApiError;
    use crate::service::chimei_ruiju::ChimeiRuijuApiService;

    #[test]
    fn 失敗_ネットワークエラー() {
        let invalid_url = "htttp://chimei-ruiju.org";
        let api_service = ChimeiRuijuApiService {};

        let result = api_service.get_blocking::<PrefectureMaster>(invalid_url);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error,
            ApiError::Network {
                url: invalid_url.to_string(),
            }
        );
    }

    #[test]
    fn 失敗_404エラー() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server.mock("GET", "/master.json").with_status(404).create();

        let api_service = ChimeiRuijuApiService {};

        let result = api_service.get_blocking::<PrefectureMaster>(&url);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, ApiError::NotFound { url: url.clone() });

        mock.assert();
    }

    #[test]
    fn 失敗_デシリアライズエラー() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"hoge": true, "piyo": 100}"#)
            .create();

        let api_service = ChimeiRuijuApiService {};

        let result = api_service.get_blocking::<PrefectureMaster>(&url);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, ApiError::Deserialize { url: url.clone() });

        mock.assert();
    }

    #[test]
    fn 成功() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "新浜県", "cities": ["新浜市"], "coordinate": {"latitude": 34.6570413, "longitude": 135.2741341}}"#)
            .create();

        let api_service = ChimeiRuijuApiService {};

        let result = api_service.get_blocking::<PrefectureMaster>(&url);
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "新浜県");
        assert_eq!(entity.cities, vec!["新浜市"]);
        assert_eq!(entity.coordinate.latitude, 34.6570413);
        assert_eq!(entity.coordinate.longitude, 135.2741341);

        mock.assert();
    }
}
