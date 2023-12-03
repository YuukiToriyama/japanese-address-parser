pub mod client;
pub mod mock;
#[cfg(not(target_arch = "wasm32"))]
pub mod blocking;

use crate::entity::{City, Prefecture};
use crate::err::Error;

pub trait Api {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error>;
    async fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error>;
}

pub trait BlockingApi {
    fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error>;
    fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error>;
}