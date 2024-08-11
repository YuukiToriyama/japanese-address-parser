pub use crate::repository::geolonia::city_master_api;
pub use crate::repository::geolonia::prefecture_master_api;

use crate::domain::geolonia::entity::{City, Prefecture};
use crate::domain::geolonia::error::Error;
use crate::repository::geolonia::city_master_api::CityMasterApi;
use crate::repository::geolonia::prefecture_master_api::PrefectureMasterApi;

#[derive(Default)]
pub struct AsyncApi {
    pub prefecture_master_api: PrefectureMasterApi,
    pub city_master_api: CityMasterApi,
}

impl AsyncApi {
    pub async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        self.prefecture_master_api.get(prefecture_name).await
    }

    pub async fn get_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> Result<City, Error> {
        self.city_master_api.get(prefecture_name, city_name).await
    }
}

#[cfg(feature = "blocking")]
#[derive(Default)]
pub struct BlockingApi {
    prefecture_master_api: PrefectureMasterApi,
    city_master_api: CityMasterApi,
}

#[cfg(feature = "blocking")]
impl BlockingApi {
    pub fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        self.prefecture_master_api.get_blocking(prefecture_name)
    }

    pub fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        self.city_master_api
            .get_blocking(prefecture_name, city_name)
    }
}
