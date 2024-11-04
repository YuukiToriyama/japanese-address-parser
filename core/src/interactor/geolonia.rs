use crate::domain::geolonia::entity::{City, Prefecture};
use crate::domain::geolonia::error::Error;
use crate::repository::geolonia::city::CityMasterRepository;
use crate::repository::geolonia::prefecture::PrefectureMasterRepository;
use crate::service::geolonia::GeoloniaApiService;

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

pub(crate) struct GeoloniaInteractorImpl {
    api_service: GeoloniaApiService,
}

impl Default for GeoloniaInteractorImpl {
    fn default() -> Self {
        Self {
            api_service: GeoloniaApiService {},
        }
    }
}

impl GeoloniaInteractor for GeoloniaInteractorImpl {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        PrefectureMasterRepository::get(&self.api_service, prefecture_name).await
    }

    #[cfg(feature = "blocking")]
    fn get_blocking_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        PrefectureMasterRepository::get_blocking(&self.api_service, prefecture_name)
    }

    async fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        CityMasterRepository::get(&self.api_service, prefecture_name, city_name).await
    }

    #[cfg(feature = "blocking")]
    fn get_blocking_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> Result<City, Error> {
        CityMasterRepository::get_blocking(&self.api_service, prefecture_name, city_name)
    }
}
