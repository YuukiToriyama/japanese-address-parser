pub mod city_master_api;
pub mod prefecture_master_api;

use crate::api::city_master_api::CityMasterApi;
use crate::api::prefecture_master_api::PrefectureMasterApi;
use crate::entity::{City, Prefecture};
use crate::err::Error;
use std::future::Future;

pub trait Api {
    fn new() -> Self;
    fn get_prefecture_master(
        &self,
        prefecture_name: &str,
    ) -> impl Future<Output = Result<Prefecture, Error>>;
    fn get_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> impl Future<Output = Result<City, Error>>;
}

pub struct ApiImpl {
    pub prefecture_master_api: PrefectureMasterApi,
    pub city_master_api: CityMasterApi,
}

impl Api for ApiImpl {
    fn new() -> Self {
        ApiImpl {
            prefecture_master_api: PrefectureMasterApi {
                server_url:
                    "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist",
            },
            city_master_api: CityMasterApi {
                server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
            },
        }
    }

    fn get_prefecture_master(
        &self,
        prefecture_name: &str,
    ) -> impl Future<Output = Result<Prefecture, Error>> {
        self.prefecture_master_api.get(prefecture_name)
    }

    fn get_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> impl Future<Output = Result<City, Error>> {
        self.city_master_api.get(prefecture_name, city_name)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub trait BlockingApi {
    fn new() -> Self;
    fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error>;
    fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct BlockingApiImpl {
    prefecture_master_api: PrefectureMasterApi,
    city_master_api: CityMasterApi,
}

#[cfg(not(target_arch = "wasm32"))]
impl BlockingApi for BlockingApiImpl {
    fn new() -> Self {
        BlockingApiImpl {
            prefecture_master_api: PrefectureMasterApi {
                server_url:
                    "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist",
            },
            city_master_api: CityMasterApi {
                server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
            },
        }
    }

    fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        self.prefecture_master_api.get_blocking(prefecture_name)
    }

    fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        self.city_master_api
            .get_blocking(prefecture_name, city_name)
    }
}
