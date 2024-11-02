use crate::domain::chimei_ruiju::entity::{CityMaster, PrefectureMaster, TownMaster};
use crate::domain::chimei_ruiju::error::ApiError;
use crate::repository::chimei_ruiju::city::CityMasterRepository;
use crate::repository::chimei_ruiju::prefecture::PrefectureMasterRepository;
use crate::repository::chimei_ruiju::town::TownMasterRepository;
use crate::service::chimei_ruiju::ChimeiRuijuApiService;
use jisx0401::Prefecture;

pub(crate) trait ChimeiRuijuInteractor {
    /// 都道府県マスタを取得
    async fn get_prefecture_master(
        &self,
        prefecture: &Prefecture,
    ) -> Result<PrefectureMaster, ApiError>;
    /// 市区町村マスタを取得
    async fn get_city_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError>;
    /// 町名マスタを取得
    #[allow(dead_code)]
    async fn get_town_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
        town_name: &str,
    ) -> Result<TownMaster, ApiError>;
}

pub(crate) struct ChimeiRuijuInteractorImpl {
    api_service: ChimeiRuijuApiService,
}

impl Default for ChimeiRuijuInteractorImpl {
    fn default() -> Self {
        Self {
            api_service: ChimeiRuijuApiService {},
        }
    }
}

impl ChimeiRuijuInteractor for ChimeiRuijuInteractorImpl {
    async fn get_prefecture_master(
        &self,
        prefecture: &Prefecture,
    ) -> Result<PrefectureMaster, ApiError> {
        PrefectureMasterRepository::get(&self.api_service, prefecture).await
    }

    async fn get_city_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        CityMasterRepository::get(&self.api_service, prefecture, city_name).await
    }

    async fn get_town_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
        town_name: &str,
    ) -> Result<TownMaster, ApiError> {
        TownMasterRepository::get(&self.api_service, prefecture, city_name, town_name).await
    }
}
