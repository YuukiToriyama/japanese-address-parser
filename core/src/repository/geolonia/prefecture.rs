use crate::domain::geolonia::entity::Prefecture;
use crate::domain::geolonia::error::Error;
use crate::http::client::ApiClient;

pub struct PrefectureMasterRepository<C: ApiClient> {
    pub api_client: C,
}

impl<C: ApiClient> PrefectureMasterRepository<C> {
    pub async fn get(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let server_url = "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist";
        let endpoint = format!("{}/{}/master.json", server_url, prefecture_name);
        self.api_client
            .fetch::<Prefecture>(&endpoint)
            .await
            .map_err(|e| e.into())
    }

    #[cfg(feature = "blocking")]
    pub fn get_blocking(&self, prefecture_name: &str) -> Result<Prefecture, Error> {
        let server_url = "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist";
        let endpoint = format!("{}/{}/master.json", server_url, prefecture_name);
        self.api_client
            .fetch_blocking::<Prefecture>(&endpoint)
            .map_err(|e| e.into())
    }
}

#[cfg(all(test, not(feature = "blocking")))]
mod tests {
    use crate::http::reqwest_client::ReqwestApiClient;
    use crate::repository::geolonia::prefecture::PrefectureMasterRepository;

    #[tokio::test]
    async fn 非同期_富山県_成功() {
        let repository = PrefectureMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get("富山県").await;
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
        let repository = PrefectureMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get("大阪都").await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/大阪都/master.jsonを取得できませんでした",
        );
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::http::reqwest_client::ReqwestApiClient;
    use crate::repository::geolonia::prefecture::PrefectureMasterRepository;

    #[test]
    fn 同期_富山県_成功() {
        let repository = PrefectureMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get_blocking("富山県");
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
        let repository = PrefectureMasterRepository {
            api_client: ReqwestApiClient {},
        };
        let result = repository.get_blocking("大阪都");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/大阪都/master.jsonを取得できませんでした",
        );
    }
}
