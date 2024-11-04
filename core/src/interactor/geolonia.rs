use crate::api::city_master_api::CityMasterApi;
use crate::api::prefecture_master_api::PrefectureMasterApi;
use crate::domain::geolonia::entity::{City, Prefecture};
use crate::domain::geolonia::error::Error;

#[allow(dead_code)]
pub(crate) trait GeoloniaInteractor {
    /// 都道府県マスタを取得(非同期)
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error>;

    /// 都道府県マスタを取得(同期)
    #[cfg(feature = "blocking")]
    fn get_blocking_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error>;

    /// 市区町村マスタを取得(非同期)
    async fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error>;

    /// 市区町村マスタを取得(同期)
    #[cfg(feature = "blocking")]
    fn get_blocking_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> Result<City, Error>;
}

#[allow(dead_code)]
pub(crate) struct GeoloniaInteractorImpl;

impl GeoloniaInteractor for GeoloniaInteractorImpl {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let prefecture_master_api = PrefectureMasterApi::default();
        prefecture_master_api.get(prefecture_name).await
    }

    #[cfg(feature = "blocking")]
    fn get_blocking_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let prefecture_master_api = PrefectureMasterApi::default();
        prefecture_master_api.get_blocking(prefecture_name)
    }

    async fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let city_master_api = CityMasterApi::default();
        city_master_api.get(prefecture_name, city_name).await
    }

    #[cfg(feature = "blocking")]
    fn get_blocking_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> Result<City, Error> {
        let city_master_api = CityMasterApi::default();
        city_master_api.get_blocking(prefecture_name, city_name)
    }
}
