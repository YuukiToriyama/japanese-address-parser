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

#[cfg(all(test, target_arch = "wasm32"))]
mod api_tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::parser::api::get_prefecture_master;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn get_prefecture_master_success() {
        let prefecture = get_prefecture_master("富山県").await.unwrap();
        assert_eq!(prefecture.name, "富山県".to_string());
        let cities = vec![
            "富山市".to_string(),
            "高岡市".to_string(),
            "魚津市".to_string(),
            "氷見市".to_string(),
            "滑川市".to_string(),
            "黒部市".to_string(),
            "砺波市".to_string(),
            "小矢部市".to_string(),
            "南砺市".to_string(),
            "射水市".to_string(),
            "中新川郡舟橋村".to_string(),
            "中新川郡上市町".to_string(),
            "中新川郡立山町".to_string(),
            "下新川郡入善町".to_string(),
            "下新川郡朝日町".to_string(),
        ];
        for city in cities {
            assert!(prefecture.cities.contains(&city));
        }
    }

    #[wasm_bindgen_test]
    #[should_panic]
    async fn get_prefecture_master_fail() {
        get_prefecture_master("大阪都").await.unwrap();
    }
}