use crate::domain::chimei_ruiju::entity::TownMaster;
use crate::domain::chimei_ruiju::error::ApiError;
use crate::service::chimei_ruiju::ChimeiRuijuApiService;
use jisx0401::Prefecture;

pub struct TownMasterRepository {}

impl TownMasterRepository {
    pub async fn get(
        api_service: &ChimeiRuijuApiService,
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
        api_service.get::<TownMaster>(&url).await
    }
}

#[cfg(test)]
mod async_tests {
    use crate::repository::chimei_ruiju::town::TownMasterRepository;
    use crate::service::chimei_ruiju::ChimeiRuijuApiService;
    use jisx0401::Prefecture;

    #[tokio::test]
    async fn 東京都千代田区千代田() {
        let api_service = ChimeiRuijuApiService {};
        let result =
            TownMasterRepository::get(&api_service, &Prefecture::TOKYO, "千代田区", "千代田").await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "千代田");
    }
}

#[cfg(feature = "blocking")]
impl TownMasterRepository {
    #[allow(dead_code)]
    pub fn get_blocking(
        api_service: &ChimeiRuijuApiService,
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
        api_service.get_blocking::<TownMaster>(&url)
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::repository::chimei_ruiju::town::TownMasterRepository;
    use crate::service::chimei_ruiju::ChimeiRuijuApiService;
    use jisx0401::Prefecture;

    #[test]
    fn 京都府京都市伏見区魚屋町() {
        let api_service = ChimeiRuijuApiService {};
        let result = TownMasterRepository::get_blocking(
            &api_service,
            &Prefecture::KYOTO,
            "京都市伏見区",
            "魚屋町",
        );
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "魚屋町");
    }
}
