use crate::api::Api;
use crate::entity::{City, Prefecture, Town};

pub struct ApiMock {
    pub should_fail: bool,
}

impl Api for ApiMock {
    async fn get_prefecture_master(&self, _prefecture_name: &str) -> Result<Prefecture, String> {
        if self.should_fail {
            Err("Failed to fetch https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/神奈川県/master.json".to_string())
        } else {
            Ok(Prefecture {
                name: "神奈川県".to_string(),
                cities: vec![
                    "平塚市".to_string(),
                    "鎌倉市".to_string(),
                    "藤沢市".to_string(),
                    "小田原市".to_string(),
                ],
            })
        }
    }

    async fn get_city_master(
        &self,
        _prefecture_name: &str,
        _city_name: &str,
    ) -> Result<City, String> {
        if self.should_fail {
            Err(
                "https://geolonia.github.io/japanese-addresses/api/ja/神奈川県/平塚市.json"
                    .to_string(),
            )
        } else {
            Ok(City {
                name: "平塚市".to_string(),
                towns: vec![
                    Town {
                        name: "御殿一丁目".to_string(),
                        koaza: "".to_string(),
                        lat: Some(35.341184),
                        lng: Some(139.331577),
                    },
                    Town {
                        name: "御殿二丁目".to_string(),
                        koaza: "".to_string(),
                        lat: Some(35.344605),
                        lng: Some(139.329437),
                    },
                    Town {
                        name: "御殿三丁目".to_string(),
                        koaza: "".to_string(),
                        lat: Some(35.347622),
                        lng: Some(139.328261),
                    },
                    Town {
                        name: "御殿四丁目".to_string(),
                        koaza: "".to_string(),
                        lat: Some(35.34606),
                        lng: Some(139.323896),
                    },
                    Town {
                        name: "幸町".to_string(),
                        koaza: "".to_string(),
                        lat: Some(35.321227),
                        lng: Some(139.3602),
                    },
                    Town {
                        name: "桜ケ丘".to_string(),
                        koaza: "".to_string(),
                        lat: Some(35.32798),
                        lng: Some(139.326709),
                    },
                ],
            })
        }
    }
}
