use gloo_net::http::Request;

use crate::entity::Prefecture;

async fn get_prefecture_master(prefecture_name: &str) -> Result<Prefecture, String> {
    let endpoint = format!(
        "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/{}/master.json",
        prefecture_name
    );
    let response = Request::get(&endpoint).send().await.unwrap();
    if response.ok() {
        let prefecture = response.json::<Prefecture>().await.unwrap();
        Ok(prefecture)
    } else {
        Err(format!("Failed to fetch {}", &endpoint))
    }
}
