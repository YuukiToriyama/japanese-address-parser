use crate::domain::geolonia::entity::Prefecture;
use crate::domain::geolonia::error::Error;
use crate::service::geolonia::GeoloniaApiService;

pub struct PrefectureMasterRepository {}

impl PrefectureMasterRepository {
    pub async fn get(
        api_service: &GeoloniaApiService,
        prefecture_name: &str,
    ) -> Result<Prefecture, Error> {
        let server_url = "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist";
        let endpoint = format!("{}/{}/master.json", server_url, prefecture_name);
        api_service.get::<Prefecture>(&endpoint).await
    }

    #[cfg(feature = "blocking")]
    pub fn get_blocking(
        api_service: &GeoloniaApiService,
        prefecture_name: &str,
    ) -> Result<Prefecture, Error> {
        let server_url = "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist";
        let endpoint = format!("{}/{}/master.json", server_url, prefecture_name);
        api_service.get_blocking::<Prefecture>(&endpoint)
    }
}

#[cfg(all(test, not(feature = "blocking")))]
mod tests {
    use crate::repository::geolonia::prefecture::PrefectureMasterRepository;
    use crate::service::geolonia::GeoloniaApiService;

    #[tokio::test]
    async fn 非同期_富山県_成功() {
        let api_service = GeoloniaApiService {};
        let result = PrefectureMasterRepository::get(&api_service, "富山県").await;
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
        let api_service = GeoloniaApiService {};
        let result = PrefectureMasterRepository::get(&api_service, "大阪都").await;
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/大阪都/master.jsonを取得できませんでした",
        );
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::repository::geolonia::prefecture::PrefectureMasterRepository;
    use crate::service::geolonia::GeoloniaApiService;

    #[test]
    fn 同期_富山県_成功() {
        let api_service = GeoloniaApiService {};
        let result = PrefectureMasterRepository::get_blocking(&api_service, "富山県");
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
        let api_service = GeoloniaApiService {};
        let result = PrefectureMasterRepository::get_blocking(&api_service, "大阪都");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().error_message,
            "https://yuukitoriyama.github.io/geolonia-japanese-addresses-accompanist/大阪都/master.jsonを取得できませんでした",
        );
    }
}
