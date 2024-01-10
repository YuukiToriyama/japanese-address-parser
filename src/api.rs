#[cfg(not(target_arch = "wasm32"))]
pub mod blocking;
pub mod city_master_api;
pub mod client;
pub mod prefecture_master_api;

use crate::api::city_master_api::CityMasterApi;
use crate::api::prefecture_master_api::PrefectureMasterApi;
use crate::entity::{City, Prefecture};
use crate::err::Error;
use std::future::Future;

pub trait Api {
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

pub struct ApiImpl {}

impl Api for ApiImpl {
    fn get_prefecture_master(
        &self,
        prefecture_name: &str,
    ) -> impl Future<Output = Result<Prefecture, Error>> {
        let prefecture_master_api = PrefectureMasterApi {
            server_url: "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist",
        };
        prefecture_master_api.get(prefecture_name)
    }

    fn get_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> impl Future<Output = Result<City, Error>> {
        let city_master_api = CityMasterApi {
            server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
        };
        city_master_api.get(prefecture_name, city_name)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub trait BlockingApi {
    fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error>;
    fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct BlockingApiImpl {}

#[cfg(not(target_arch = "wasm32"))]
impl BlockingApi for BlockingApiImpl {
    fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let prefecture_master_api = PrefectureMasterApi {
            server_url: "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist",
        };
        prefecture_master_api.get_blocking(prefecture_name)
    }

    fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        let city_master_api = CityMasterApi {
            server_url: "https://geolonia.github.io/japanese-addresses/api/ja",
        };
        city_master_api.get_blocking(prefecture_name, city_name)
    }
}
