use crate::domain::geolonia::entity::Prefecture;
use crate::domain::geolonia::error::Error;
use crate::service::geolonia::GeoloniaApiService;

pub struct PrefectureMasterApi {
    pub server_url: &'static str,
}

impl Default for PrefectureMasterApi {
    fn default() -> Self {
        Self {
            server_url: "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist",
        }
    }
}

impl PrefectureMasterApi {
    pub async fn get(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let endpoint = format!("{}/{}/master.json", self.server_url, prefecture_name);
        let api_service = GeoloniaApiService {};
        api_service.get::<Prefecture>(&endpoint).await
    }
    #[cfg(feature = "blocking")]
    pub fn get_blocking(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let endpoint = format!("{}/{}/master.json", self.server_url, prefecture_name);
        let api_service = GeoloniaApiService {};
        api_service.get_blocking::<Prefecture>(&endpoint)
    }
}

#[cfg(all(test, not(feature = "blocking")))]
mod tests {
    use crate::repository::geolonia::prefecture_master_api::PrefectureMasterApi;

    #[tokio::test]
    async fn 非同期_富山県_成功() {
        let prefecture_master_api: PrefectureMasterApi = Default::default();
        let result = prefecture_master_api.get("富山県").await;
        let prefecture = result.unwrap();
        assert_eq!(prefecture.name, "富山県");
        let cities = vec![
            "富山市",
            "高岡市",
            "魚津市",
            "氷見市",
            "滑川市",
            "黒部市",
            "砺波市",
            "小矢部市",
            "南砺市",
            "射水市",
            "中新川郡舟橋村",
            "中新川郡上市町",
            "中新川郡立山町",
            "下新川郡入善町",
            "下新川郡朝日町",
        ];
        for city in cities {
            assert!(prefecture.cities.contains(&city.to_string()));
        }
    }

    #[tokio::test]
    async fn 非同期_誤った都道府県名_失敗() {
        let prefecture_master_api: PrefectureMasterApi = Default::default();
        let result = prefecture_master_api.get("大阪都").await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            format!(
                "{}/大阪都/master.jsonを取得できませんでした",
                prefecture_master_api.server_url
            )
        );
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::repository::geolonia::prefecture_master_api::PrefectureMasterApi;

    #[test]
    fn 同期_富山県_成功() {
        let prefecture_master_api: PrefectureMasterApi = Default::default();
        let result = prefecture_master_api.get_blocking("富山県");
        let prefecture = result.unwrap();
        assert_eq!(prefecture.name, "富山県");
        let cities = vec![
            "富山市",
            "高岡市",
            "魚津市",
            "氷見市",
            "滑川市",
            "黒部市",
            "砺波市",
            "小矢部市",
            "南砺市",
            "射水市",
            "中新川郡舟橋村",
            "中新川郡上市町",
            "中新川郡立山町",
            "下新川郡入善町",
            "下新川郡朝日町",
        ];
        for city in cities {
            assert!(prefecture.cities.contains(&city.to_string()));
        }
    }

    #[test]
    fn 同期_誤った都道府県名_失敗() {
        let prefecture_master_api: PrefectureMasterApi = Default::default();
        let result = prefecture_master_api.get_blocking("大阪都");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            format!(
                "{}/大阪都/master.jsonを取得できませんでした",
                prefecture_master_api.server_url
            )
        );
    }
}
