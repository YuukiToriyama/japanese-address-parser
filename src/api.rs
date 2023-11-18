mod wasm;

use crate::entity::Prefecture;

trait Api {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, String>;
}
