#[cfg(not(target_arch = "wasm32"))]
pub mod blocking;
mod city_master_api;
pub mod client;
mod prefecture_master_api;

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

pub trait BlockingApi {
    fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error>;
    fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error>;
}
