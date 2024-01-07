use crate::entity::Prefecture;
use crate::err::Error;

pub struct PrefectureMasterApi {}

impl PrefectureMasterApi {
    async fn get(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        todo!()
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_blocking(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        todo!()
    }
}
