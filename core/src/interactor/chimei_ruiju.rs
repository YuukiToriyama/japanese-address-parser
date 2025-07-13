use crate::domain::chimei_ruiju::entity::{CityMaster, PrefectureMaster, TownMaster};
use crate::domain::chimei_ruiju::error::ApiError;
use crate::http::reqwest_client::ReqwestApiClient;
use crate::repository::chimei_ruiju::city::CityMasterRepository;
use crate::repository::chimei_ruiju::prefecture::PrefectureMasterRepository;
use crate::repository::chimei_ruiju::town::TownMasterRepository;
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

#[derive(Default)]
pub(crate) struct ChimeiRuijuInteractorImpl {}

impl ChimeiRuijuInteractor for ChimeiRuijuInteractorImpl {
    async fn get_prefecture_master(
        &self,
        prefecture: &Prefecture,
    ) -> Result<PrefectureMaster, ApiError> {
        let repository = PrefectureMasterRepository {
            api_client: ReqwestApiClient {},
        };
        repository.get(prefecture).await
    }

    async fn get_city_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        let repository = CityMasterRepository {
            api_client: ReqwestApiClient {},
        };
        repository.get(prefecture, city_name).await
    }

    async fn get_town_master(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
        town_name: &str,
    ) -> Result<TownMaster, ApiError> {
        let repository = TownMasterRepository {
            api_client: ReqwestApiClient {},
        };
        repository.get(prefecture, city_name, town_name).await
    }
}
