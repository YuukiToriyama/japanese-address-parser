use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::domain::geolonia::error::{ApiErrorKind, Error};

pub struct GeoloniaApiService {}

impl GeoloniaApiService {
    pub async fn get<T>(&self, url: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::get(url)
            .await
            .map_err(|_| Error::new_api_error(ApiErrorKind::Fetch(url.to_string())))?;
        if response.status() == StatusCode::OK {
            let json = response
                .json::<T>()
                .await
                .map_err(|_| Error::new_api_error(ApiErrorKind::Deserialize(url.to_string())))?;
            return Ok(json);
        }
        Err(Error::new_api_error(ApiErrorKind::Fetch(url.to_string())))
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod async_tests {
    use crate::domain::geolonia::entity::{Prefecture, Town};
    use crate::domain::geolonia::error::{ApiErrorKind, Error};

    use crate::service::geolonia::GeoloniaApiService;

    #[tokio::test]
    async fn 失敗_ネットワークエラー() {
        let invalid_url =
            "htttps://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/";
        let api_service = GeoloniaApiService {};

        let result = api_service.get::<Prefecture>(invalid_url).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error,
            Error::new_api_error(ApiErrorKind::Fetch(invalid_url.to_string()))
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

        let api_service = GeoloniaApiService {};

        let result = api_service.get::<Prefecture>(&url).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, Error::new_api_error(ApiErrorKind::Fetch(url)));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn 失敗_パースエラー() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"hoge": true, "piyo": 100}"#)
            .create_async()
            .await;

        let api_service = GeoloniaApiService {};

        let result = api_service.get::<Prefecture>(&url).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, Error::new_api_error(ApiErrorKind::Deserialize(url)));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn 成功_都道府県マスタ() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"name": "京都府", "cities": ["京都市北区", "京都市上京区", "京都市左京区"]}"#,
            )
            .create_async()
            .await;

        let api_service = GeoloniaApiService {};

        let result = api_service.get::<Prefecture>(&url).await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "京都府");
        assert_eq!(
            entity.cities,
            vec!["京都市北区", "京都市上京区", "京都市左京区"]
        );

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn 成功_市区町村マスタ() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[{"town":"上賀茂茨谷町","koaza":"","lat":35.075936,"lng":135.747963},{"town":"中川奥山","koaza":"","lat":35.090298,"lng":135.675356},{"town":"中川水谷","koaza":"","lat":35.097157,"lng":135.668432}]"#,
            )
            .create_async()
            .await;

        let api_service = GeoloniaApiService {};

        let result = api_service.get::<Vec<Town>>(&url).await;
        assert!(result.is_ok());
        let entity: Vec<Town> = result.unwrap();
        assert_eq!(
            entity[0],
            Town {
                name: "上賀茂茨谷町".to_string(),
                koaza: "".to_string(),
                lat: Some(35.075936),
                lng: Some(135.747963),
            }
        );
        assert_eq!(
            entity[1],
            Town {
                name: "中川奥山".to_string(),
                koaza: "".to_string(),
                lat: Some(35.090298),
                lng: Some(135.675356),
            }
        );
        assert_eq!(
            entity[2],
            Town {
                name: "中川水谷".to_string(),
                koaza: "".to_string(),
                lat: Some(35.097157),
                lng: Some(135.668432),
            }
        );

        mock.assert_async().await;
    }
}

#[cfg(feature = "blocking")]
impl GeoloniaApiService {
    pub fn get_blocking<T>(&self, url: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::blocking::get(url)
            .map_err(|_| Error::new_api_error(ApiErrorKind::Fetch(url.to_string())))?;
        if response.status() == StatusCode::OK {
            let json = response
                .json::<T>()
                .map_err(|_| Error::new_api_error(ApiErrorKind::Deserialize(url.to_string())))?;
            return Ok(json);
        }
        Err(Error::new_api_error(ApiErrorKind::Fetch(url.to_string())))
    }
}

#[cfg(all(test, feature = "blocking", not(target_arch = "wasm32")))]
mod blocking_tests {
    use crate::domain::geolonia::entity::{Prefecture, Town};
    use crate::domain::geolonia::error::{ApiErrorKind, Error};

    use crate::service::geolonia::GeoloniaApiService;

    #[test]
    fn 失敗_ネットワークエラー() {
        let invalid_url =
            "htttps://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/";
        let api_service = GeoloniaApiService {};

        let result = api_service.get_blocking::<Prefecture>(invalid_url);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error,
            Error::new_api_error(ApiErrorKind::Fetch(invalid_url.to_string()))
        );
    }

    #[test]
    fn 失敗_404エラー() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server.mock("GET", "/master.json").with_status(404).create();

        let api_service = GeoloniaApiService {};

        let result = api_service.get_blocking::<Prefecture>(&url);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, Error::new_api_error(ApiErrorKind::Fetch(url)));

        mock.assert();
    }

    #[test]
    fn 失敗_パースエラー() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"hoge": true, "piyo": 100}"#)
            .create();

        let api_service = GeoloniaApiService {};

        let result = api_service.get_blocking::<Prefecture>(&url);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, Error::new_api_error(ApiErrorKind::Deserialize(url)));

        mock.assert();
    }

    #[test]
    fn 成功_都道府県マスタ() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"name": "京都府", "cities": ["京都市北区", "京都市上京区", "京都市左京区"]}"#,
            )
            .create();

        let api_service = GeoloniaApiService {};

        let result = api_service.get_blocking::<Prefecture>(&url);
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "京都府");
        assert_eq!(
            entity.cities,
            vec!["京都市北区", "京都市上京区", "京都市左京区"]
        );

        mock.assert();
    }

    #[test]
    fn 成功_市区町村マスタ() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[{"town":"上賀茂茨谷町","koaza":"","lat":35.075936,"lng":135.747963},{"town":"中川奥山","koaza":"","lat":35.090298,"lng":135.675356},{"town":"中川水谷","koaza":"","lat":35.097157,"lng":135.668432}]"#,
            )
            .create();

        let api_service = GeoloniaApiService {};

        let result = api_service.get_blocking::<Vec<Town>>(&url);
        assert!(result.is_ok());
        let entity: Vec<Town> = result.unwrap();
        assert_eq!(
            entity[0],
            Town {
                name: "上賀茂茨谷町".to_string(),
                koaza: "".to_string(),
                lat: Some(35.075936),
                lng: Some(135.747963),
            }
        );
        assert_eq!(
            entity[1],
            Town {
                name: "中川奥山".to_string(),
                koaza: "".to_string(),
                lat: Some(35.090298),
                lng: Some(135.675356),
            }
        );
        assert_eq!(
            entity[2],
            Town {
                name: "中川水谷".to_string(),
                koaza: "".to_string(),
                lat: Some(35.097157),
                lng: Some(135.668432),
            }
        );

        mock.assert();
    }
}
