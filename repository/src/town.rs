use crate::entity::TownMaster;
use crate::error::ApiError;
use crate::service::ChimeiRuijuApiService;
use jisx0401::Prefecture;

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
