use crate::domain::chimei_ruiju::entity::PrefectureMaster;
use crate::domain::chimei_ruiju::error::ApiError;
use crate::service::chimei_ruiju::ChimeiRuijuApiService;
use jisx0401::Prefecture;

pub struct PrefectureMasterRepository {}

impl PrefectureMasterRepository {
    pub async fn get(
        api_service: &ChimeiRuijuApiService,
        prefecture: &Prefecture,
    ) -> Result<PrefectureMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/master.json",
            prefecture.name_en()
        );
        api_service.get::<PrefectureMaster>(&url).await
    }
}

#[cfg(test)]
mod async_tests {
    use crate::repository::chimei_ruiju::prefecture::PrefectureMasterRepository;
    use crate::service::chimei_ruiju::ChimeiRuijuApiService;
    use jisx0401::Prefecture;

    #[tokio::test]
    async fn 東京都() {
        let api_service = ChimeiRuijuApiService {};
        let result = PrefectureMasterRepository::get(&api_service, &Prefecture::TOKYO).await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "東京都");
        assert_eq!(
            entity.cities,
            vec![
                "千代田区",
                "中央区",
                "港区",
                "新宿区",
                "文京区",
                "台東区",
                "墨田区",
                "江東区",
                "品川区",
                "目黒区",
                "大田区",
                "世田谷区",
                "渋谷区",
                "中野区",
                "杉並区",
                "豊島区",
                "北区",
                "荒川区",
                "板橋区",
                "練馬区",
                "足立区",
                "葛飾区",
                "江戸川区",
                "八王子市",
                "立川市",
                "武蔵野市",
                "三鷹市",
                "青梅市",
                "府中市",
                "昭島市",
                "調布市",
                "町田市",
                "小金井市",
                "小平市",
                "日野市",
                "東村山市",
                "国分寺市",
                "国立市",
                "福生市",
                "狛江市",
                "東大和市",
                "清瀬市",
                "東久留米市",
                "武蔵村山市",
                "多摩市",
                "稲城市",
                "羽村市",
                "あきる野市",
                "西東京市",
                "西多摩郡瑞穂町",
                "西多摩郡日の出町",
                "西多摩郡檜原村",
                "西多摩郡奥多摩町",
                "大島町",
                "利島村",
                "新島村",
                "神津島村",
                "三宅村",
                "御蔵島村",
                "八丈町",
                "青ヶ島村",
                "小笠原村",
            ]
        )
    }

    #[tokio::test]
    async fn 富山県() {
        let api_service = ChimeiRuijuApiService {};
        let result = PrefectureMasterRepository::get(&api_service, &Prefecture::TOYAMA).await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "富山県");
        assert_eq!(
            entity.cities,
            vec![
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
            ]
        );
    }
}

#[cfg(feature = "blocking")]
impl PrefectureMasterRepository {
    #[allow(dead_code)]
    pub fn get_blocking(
        api_service: &ChimeiRuijuApiService,
        prefecture: Prefecture,
    ) -> Result<PrefectureMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/master.json",
            prefecture.name_en()
        );
        api_service.get_blocking::<PrefectureMaster>(&url)
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use crate::repository::chimei_ruiju::prefecture::PrefectureMasterRepository;
    use crate::service::chimei_ruiju::ChimeiRuijuApiService;
    use jisx0401::Prefecture;

    #[tokio::test]
    async fn 高知県() {
        let api_service = ChimeiRuijuApiService {};
        let result = PrefectureMasterRepository::get(&api_service, &Prefecture::KOCHI).await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "高知県");
        assert_eq!(
            entity.cities,
            vec![
                "高知市",
                "室戸市",
                "安芸市",
                "南国市",
                "土佐市",
                "須崎市",
                "宿毛市",
                "土佐清水市",
                "四万十市",
                "香南市",
                "香美市",
                "安芸郡東洋町",
                "安芸郡奈半利町",
                "安芸郡田野町",
                "安芸郡安田町",
                "安芸郡北川村",
                "安芸郡馬路村",
                "安芸郡芸西村",
                "長岡郡本山町",
                "長岡郡大豊町",
                "土佐郡土佐町",
                "土佐郡大川村",
                "吾川郡いの町",
                "吾川郡仁淀川町",
                "高岡郡中土佐町",
                "高岡郡佐川町",
                "高岡郡越知町",
                "高岡郡檮原町",
                "高岡郡日高村",
                "高岡郡津野町",
                "高岡郡四万十町",
                "幡多郡大月町",
                "幡多郡三原村",
                "幡多郡黒潮町"
            ]
        )
    }

    #[tokio::test]
    async fn 佐賀県() {
        let api_service = ChimeiRuijuApiService {};
        let result = PrefectureMasterRepository::get(&api_service, &Prefecture::SAGA).await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "佐賀県");
        assert_eq!(
            entity.cities,
            vec![
                "佐賀市",
                "唐津市",
                "鳥栖市",
                "多久市",
                "伊万里市",
                "武雄市",
                "鹿島市",
                "小城市",
                "嬉野市",
                "神埼市",
                "神埼郡吉野ヶ里町",
                "三養基郡基山町",
                "三養基郡上峰町",
                "三養基郡みやき町",
                "東松浦郡玄海町",
                "西松浦郡有田町",
                "杵島郡大町町",
                "杵島郡江北町",
                "杵島郡白石町",
                "藤津郡太良町"
            ]
        );
    }
}
