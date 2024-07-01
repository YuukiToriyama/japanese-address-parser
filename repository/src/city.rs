use jisx0401::Prefecture;

use crate::entity::CityMaster;
use crate::error::ApiError;
use crate::service::ChimeiRuijuApiService;

pub struct CityMasterRepository {
    api_service: ChimeiRuijuApiService,
}

impl CityMasterRepository {
    pub async fn get(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/{}/master.json",
            prefecture.name_en(),
            city_name
        );
        self.api_service.get::<CityMaster>(&url).await
    }
}

impl CityMasterRepository {
    pub fn get_blocking(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/{}/master.json",
            prefecture.name_en(),
            city_name
        );
        self.api_service.get_blocking::<CityMaster>(&url)
    }
}
