pub mod mock;
pub mod wasm;

use crate::entity::{City, Prefecture};

pub trait Api {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, String>;
    async fn get_city_master(&self, prefecture_name: &str, city_name: &str)
        -> Result<City, String>;
}
