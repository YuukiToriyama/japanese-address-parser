use jisx0401::Prefecture;

use domain::chimei_ruiju::entity::CityMaster;
use domain::chimei_ruiju::error::ApiError;
use service::chimei_ruiju::ChimeiRuijuApiService;

pub struct CityMasterRepository {
    api_service: ChimeiRuijuApiService,
}

impl CityMasterRepository {
    pub async fn get(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/{}/master.json",
            prefecture.name_en(),
            city_name
        );
        self.api_service.get::<CityMaster>(&url).await
    }
}

#[cfg(test)]
mod async_tests {
    use jisx0401::Prefecture;

    use crate::chimei_ruiju::city::CityMasterRepository;
    use service::chimei_ruiju::ChimeiRuijuApiService;

    #[tokio::test]
    async fn 神奈川県愛甲郡清川村() {
        let repository = CityMasterRepository {
            api_service: ChimeiRuijuApiService {},
        };
        let result = repository.get(&Prefecture::KANAGAWA, "愛甲郡清川村").await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "愛甲郡清川村");
        assert_eq!(entity.towns, vec!["煤ヶ谷", "宮ヶ瀬"]);
    }

    #[tokio::test]
    async fn 京都府乙訓郡大山崎町() {
        let repository = CityMasterRepository {
            api_service: ChimeiRuijuApiService {},
        };
        let result = repository.get(&Prefecture::KYOTO, "乙訓郡大山崎町").await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "乙訓郡大山崎町");
        assert_eq!(entity.towns, vec!["字円明寺", "字大山崎", "字下植野"]);
    }
}

#[cfg(feature = "blocking")]
impl CityMasterRepository {
    pub fn get_blocking(
        &self,
        prefecture: &Prefecture,
        city_name: &str,
    ) -> Result<CityMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/{}/master.json",
            prefecture.name_en(),
            city_name
        );
        self.api_service.get_blocking::<CityMaster>(&url)
    }
}

#[cfg(all(test, feature = "blocking"))]
mod blocking_tests {
    use jisx0401::Prefecture;

    use crate::chimei_ruiju::city::CityMasterRepository;
    use service::chimei_ruiju::ChimeiRuijuApiService;

    #[test]
    fn 埼玉県比企郡嵐山町() {
        let repository = CityMasterRepository {
            api_service: ChimeiRuijuApiService {},
        };
        let result = repository.get_blocking(&Prefecture::SAITAMA, "比企郡嵐山町");
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "比企郡嵐山町");
        assert_eq!(
            entity.towns,
            vec![
                "むさし台一丁目",
                "むさし台二丁目",
                "むさし台三丁目",
                "大字根岸",
                "大字勝田",
                "大字太郎丸",
                "大字川島",
                "花見台",
                "大字遠山",
                "大字大蔵",
                "大字菅谷",
                "大字千手堂",
                "大字廣野",
                "大字杉山",
                "大字平澤",
                "大字将軍澤",
                "大字志賀",
                "大字吉田",
                "大字古里",
                "大字越畑",
                "大字鎌形"
            ]
        );
    }

    #[test]
    fn 岐阜県不破郡関ケ原町() {
        let repository = CityMasterRepository {
            api_service: ChimeiRuijuApiService {},
        };
        let result = repository.get_blocking(&Prefecture::GIFU, "不破郡関ケ原町");
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "不破郡関ケ原町");
        assert_eq!(
            entity.towns,
            vec![
                "大字今須",
                "大字大高",
                "大字関ケ原",
                "大字玉",
                "大字藤下",
                "大字野上",
                "大字松尾",
                "大字山中"
            ]
        );
    }
}
