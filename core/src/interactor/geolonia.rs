use crate::domain::geolonia::entity::{City, Prefecture};
use crate::domain::geolonia::error::Error;
use crate::http::reqwest_client::ReqwestApiClient;
use crate::repository::geolonia::city::CityMasterRepository;
use crate::repository::geolonia::prefecture::PrefectureMasterRepository;

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
    prefecture_repository: PrefectureMasterRepository<ReqwestApiClient>,
    city_repository: CityMasterRepository<ReqwestApiClient>,
}

impl Default for GeoloniaInteractorImpl {
    fn default() -> Self {
        Self {
            prefecture_repository: PrefectureMasterRepository {
                api_client: ReqwestApiClient {},
            },
            city_repository: CityMasterRepository {
                api_client: ReqwestApiClient {},
            },
        }
    }
}

impl GeoloniaInteractor for GeoloniaInteractorImpl {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        self.prefecture_repository.get(prefecture_name).await
    }

    #[cfg(feature = "blocking")]
    fn get_blocking_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        self.prefecture_repository.get_blocking(prefecture_name)
    }

    async fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        self.city_repository.get(prefecture_name, city_name).await
    }

    #[cfg(feature = "blocking")]
    fn get_blocking_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> Result<City, Error> {
        self.city_repository
            .get_blocking(prefecture_name, city_name)
    }
}
