use domain::entity::TownMaster;
use domain::error::ApiError;
use jisx0401::Prefecture;
use service::service::ChimeiRuijuApiService;

pub struct TownMasterRepository {
    api_service: ChimeiRuijuApiService,
}

impl TownMasterRepository {
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
        self.api_service.get::<TownMaster>(&url).await
    }
}

#[cfg(test)]
mod async_tests {
    use crate::town::TownMasterRepository;
    use jisx0401::Prefecture;
    use service::service::ChimeiRuijuApiService;

    #[tokio::test]
    async fn 東京都千代田区千代田() {
        let repository = TownMasterRepository {
            api_service: ChimeiRuijuApiService {},
        };
        let result = repository
            .get(&Prefecture::TOKYO, "千代田区", "千代田")
            .await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "千代田");
    }
}

impl TownMasterRepository {
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
        self.api_service.get_blocking::<TownMaster>(&url)
    }
}

#[cfg(test)]
mod blocking_tests {
    use crate::town::TownMasterRepository;
    use jisx0401::Prefecture;
    use service::service::ChimeiRuijuApiService;

    #[test]
    fn 京都府京都市伏見区魚屋町() {
        let repository = TownMasterRepository {
            api_service: ChimeiRuijuApiService {},
        };
        let result = repository.get_blocking(&Prefecture::KYOTO, "京都市伏見区", "魚屋町");
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "魚屋町");
    }
}
