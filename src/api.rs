pub mod mock;
pub mod wasm;

use crate::entity::{City, Prefecture};
use crate::err::Error;

pub trait Api {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, Error>;
    async fn get_city_master(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error>;
}
