use crate::api::Api;
use crate::entity::{City, Prefecture, Town};

pub struct ApiImplForNative {}

impl Api for ApiImplForNative {
    async fn get_prefecture_master(&self, prefecture_name: &str) -> Result<Prefecture, String> {
        let endpoint = format!(
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/{}/master.json",
            prefecture_name
        );
        let response = reqwest::get(&endpoint).await.unwrap();
        if response.status() == 200 {
            let prefecture = response.json::<Prefecture>().await.unwrap();
            Ok(prefecture)
        } else {
            Err(format!("Failed to fetch {}", &endpoint))
        }
    }

    async fn get_city_master(
        &self,
        prefecture_name: &str,
        city_name: &str,
    ) -> Result<City, String> {
        let endpoint = format!(
            "https://geolonia.github.io/japanese-addresses/api/ja/{}/{}.json",
            prefecture_name, city_name
        );
        let response = reqwest::get(&endpoint).await.unwrap();
        if response.status() == 200 {
            let towns = response.json::<Vec<Town>>().await.unwrap();
            Ok(City {
                name: city_name.to_string(),
                towns,
            })
        } else {
            Err(format!("Failed to fetch {}", &endpoint))
        }
    }
}

#[cfg(test)]
mod api_tests {
    use crate::api::native::ApiImplForNative;
    use crate::api::Api;
    use crate::entity::Town;

    #[tokio::test]
    async fn get_prefecture_master_success() {
        let api = ApiImplForNative {};
        let prefecture = api.get_prefecture_master("富山県").await.unwrap();
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

    #[tokio::test]
    #[should_panic]
    async fn get_prefecture_master_fail() {
        let api = ApiImplForNative {};
        api.get_prefecture_master("大阪都").await.unwrap();
    }

    #[tokio::test]
    async fn get_city_master_success() {
        let api = ApiImplForNative {};
        let city = api.get_city_master("石川県", "羽咋郡志賀町").await.unwrap();
        assert_eq!(city.name, "羽咋郡志賀町".to_string());
        let town = Town {
            name: "末吉".to_string(),
            koaza: "千古".to_string(),
            lat: 37.006235,
            lng: 136.779155,
        };
        assert!(city.towns.contains(&town));
    }

    #[tokio::test]
    #[should_panic]
    async fn get_city_master_fail() {
        let api = ApiImplForNative {};
        api.get_city_master("石川県", "敦賀市").await.unwrap();
    }
}
