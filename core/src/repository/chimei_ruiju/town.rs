use crate::domain::chimei_ruiju::entity::TownMaster;
use crate::domain::chimei_ruiju::error::ApiError;
use crate::http::client::ApiClient;
use jisx0401::Prefecture;

pub struct TownMasterRepository<C: ApiClient> {
    pub api_client: C,
}

impl<C: ApiClient> TownMasterRepository<C> {
    pub async fn get(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
        town_name: &str,
    ) -> Result<TownMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/{}/{}/master.json",
            prefecture.name_en(),
            city_name,
            town_name
        );
        self.api_client
            .fetch::<TownMaster>(&url)
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod async_tests {
    use crate::http::reqwest_client::ReqwestApiClient;
    use crate::repository::chimei_ruiju::town::TownMasterRepository;
    use jisx0401::Prefecture;

    #[tokio::test]
    async fn 東京都千代田区千代田() {
        let repository = TownMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository
            .get(&Prefecture::TOKYO, "千代田区", "千代田")
            .await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "千代田");
    }
}

#[cfg(feature = "blocking")]
impl<C: ApiClient> TownMasterRepository<C> {
    #[allow(dead_code)]
    pub fn get_blocking(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
        town_name: &str,
    ) -> Result<TownMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/{}/{}/master.json",
            prefecture.name_en(),
            city_name,
            town_name
        );
        self.api_client.fetch_blocking(&url).map_err(|e| e.into())
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::http::reqwest_client::ReqwestApiClient;
    use crate::repository::chimei_ruiju::town::TownMasterRepository;
    use jisx0401::Prefecture;

    #[test]
    fn 京都府京都市伏見区魚屋町() {
        let repository = TownMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get_blocking(&Prefecture::KYOTO, "京都市伏見区", "魚屋町");
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "魚屋町");
    }
}
