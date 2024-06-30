use crate::entity::PrefectureMaster;
use crate::error::ApiError;
use jisx0401::Prefecture;

pub struct PrefectureMasterRepository {}

impl PrefectureMasterRepository {
    pub async fn get(&self, prefecture: &Prefecture) -> Result<PrefectureMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/master.json",
            prefecture.name_en()
        );
        let response = reqwest::get(&url)
            .await
            .map_err(|error| ApiError::Network {
                url: url.clone(),
                status_code: error.status().unwrap(),
            })?;
        let json = response
            .json::<PrefectureMaster>()
            .await
            .map_err(|_| ApiError::Deserialize { url })?;
        Ok(json)
    }
}

#[cfg(test)]
mod async_tests {
    use crate::prefecture::PrefectureMasterRepository;
    use jisx0401::Prefecture;

    #[tokio::test]
    async fn tokyo() {
        let repository = PrefectureMasterRepository {};
        let result = repository.get(&Prefecture::TOKYO).await;
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
    async fn toyama() {
        let repository = PrefectureMasterRepository {};
        let result = repository.get(&Prefecture::TOYAMA).await;
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

impl PrefectureMasterRepository {
    pub fn get_blocking(&self, prefecture: Prefecture) -> Result<PrefectureMaster, ApiError> {
        let url = format!(
            "https://{}.chimei-ruiju.org/master.json",
            prefecture.name_en()
        );
        let response = reqwest::blocking::get(&url).map_err(|error| ApiError::Network {
            url: url.clone(),
            status_code: error.status().unwrap(),
        })?;
        let json = response
            .json::<PrefectureMaster>()
            .map_err(|_| ApiError::Deserialize { url })?;
        Ok(json)
    }
}
