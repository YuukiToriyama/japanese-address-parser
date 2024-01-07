use crate::entity::City;
use crate::err::Error;

pub struct CityMasterApi {
    server_url: &'static str,
}

impl CityMasterApi {
    async fn get(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        todo!()
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_blocking(&self, prefecture_name: &str, city_name: &str) -> Result<City, Error> {
        todo!()
    }
}
