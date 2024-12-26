use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Debug)]
pub(crate) struct Prefecture {
    pub name: String,
    pub cities: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct City {
    #[allow(dead_code)]
    pub name: String,
    pub towns: Vec<Town>,
}

#[derive(PartialEq, Deserialize, Debug)]
pub(crate) struct Town {
    #[serde(alias = "town")]
    pub name: String,
    pub koaza: String,
    // TODO: https://github.com/geolonia/japanese-addresses/issues/148 が解消されたらOptionを外すことができる
    pub lat: Option<f32>,
    pub lng: Option<f32>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Address {
    pub prefecture: String,
    pub city: String,
    pub town: String,
    pub rest: String,
}

impl Address {
    pub(crate) fn new(
        prefecture_name: &str,
        city_name: &str,
        town_name: &str,
        rest_name: &str,
    ) -> Self {
        Address {
            prefecture: prefecture_name.to_string(),
            city: city_name.to_string(),
            town: town_name.to_string(),
            rest: rest_name.to_string(),
        }
    }
}

#[cfg(test)]
impl Prefecture {
    /// only for testing
    pub(crate) fn yamagata() -> Self {
        Prefecture {
            name: "山形県".to_string(),
            cities: vec![
                "山形市".to_string(),
                "米沢市".to_string(),
                "鶴岡市".to_string(),
                "酒田市".to_string(),
                "新庄市".to_string(),
                "寒河江市".to_string(),
                "上山市".to_string(),
                "村山市".to_string(),
                "長井市".to_string(),
                "天童市".to_string(),
                "東根市".to_string(),
                "尾花沢市".to_string(),
                "南陽市".to_string(),
                "東村山郡山辺町".to_string(),
                "東村山郡中山町".to_string(),
                "西村山郡河北町".to_string(),
                "西村山郡西川町".to_string(),
                "西村山郡朝日町".to_string(),
                "西村山郡大江町".to_string(),
                "北村山郡大石田町".to_string(),
                "最上郡金山町".to_string(),
                "最上郡最上町".to_string(),
                "最上郡舟形町".to_string(),
                "最上郡真室川町".to_string(),
                "最上郡大蔵村".to_string(),
                "最上郡鮭川村".to_string(),
                "最上郡戸沢村".to_string(),
                "東置賜郡高畠町".to_string(),
                "東置賜郡川西町".to_string(),
                "西置賜郡小国町".to_string(),
                "西置賜郡白鷹町".to_string(),
                "西置賜郡飯豊町".to_string(),
                "東田川郡三川町".to_string(),
                "東田川郡庄内町".to_string(),
                "飽海郡遊佐町".to_string(),
            ],
        }
    }

    /// only for testing
    pub(crate) fn fukushima() -> Self {
        Prefecture {
            name: "福島県".to_string(),
            cities: vec![
                "福島市".to_string(),
                "会津若松市".to_string(),
                "郡山市".to_string(),
                "いわき市".to_string(),
                "白河市".to_string(),
                "須賀川市".to_string(),
                "喜多方市".to_string(),
                "相馬市".to_string(),
                "二本松市".to_string(),
                "田村市".to_string(),
                "南相馬市".to_string(),
                "伊達市".to_string(),
                "本宮市".to_string(),
                "伊達郡桑折町".to_string(),
                "伊達郡国見町".to_string(),
                "伊達郡川俣町".to_string(),
                "安達郡大玉村".to_string(),
                "岩瀬郡鏡石町".to_string(),
                "岩瀬郡天栄村".to_string(),
                "南会津郡下郷町".to_string(),
                "南会津郡檜枝岐村".to_string(),
                "南会津郡只見町".to_string(),
                "南会津郡南会津町".to_string(),
                "耶麻郡北塩原村".to_string(),
                "耶麻郡西会津町".to_string(),
                "耶麻郡磐梯町".to_string(),
                "耶麻郡猪苗代町".to_string(),
                "河沼郡会津坂下町".to_string(),
                "河沼郡湯川村".to_string(),
                "河沼郡柳津町".to_string(),
                "大沼郡三島町".to_string(),
                "大沼郡金山町".to_string(),
                "大沼郡昭和村".to_string(),
                "大沼郡会津美里町".to_string(),
                "西白河郡西郷村".to_string(),
                "西白河郡泉崎村".to_string(),
                "西白河郡中島村".to_string(),
                "西白河郡矢吹町".to_string(),
                "東白川郡棚倉町".to_string(),
                "東白川郡矢祭町".to_string(),
                "東白川郡塙町".to_string(),
                "東白川郡鮫川村".to_string(),
                "石川郡石川町".to_string(),
                "石川郡玉川村".to_string(),
                "石川郡平田村".to_string(),
                "石川郡浅川町".to_string(),
                "石川郡古殿町".to_string(),
                "田村郡三春町".to_string(),
                "田村郡小野町".to_string(),
                "双葉郡広野町".to_string(),
                "双葉郡楢葉町".to_string(),
                "双葉郡富岡町".to_string(),
                "双葉郡川内村".to_string(),
                "双葉郡大熊町".to_string(),
                "双葉郡双葉町".to_string(),
                "双葉郡浪江町".to_string(),
                "双葉郡葛尾村".to_string(),
                "相馬郡新地町".to_string(),
                "相馬郡飯舘村".to_string(),
            ],
        }
    }

    /// only for testing
    pub(crate) fn fukui() -> Self {
        Prefecture {
            name: "福井県".to_string(),
            cities: vec![
                "福井市".to_string(),
                "敦賀市".to_string(),
                "小浜市".to_string(),
                "大野市".to_string(),
                "勝山市".to_string(),
                "鯖江市".to_string(),
                "あわら市".to_string(),
                "越前市".to_string(),
                "坂井市".to_string(),
                "吉田郡永平寺町".to_string(),
                "今立郡池田町".to_string(),
                "南条郡南越前町".to_string(),
                "丹生郡越前町".to_string(),
                "三方郡美浜町".to_string(),
                "大飯郡高浜町".to_string(),
                "大飯郡おおい町".to_string(),
                "三方上中郡若狭町".to_string(),
            ],
        }
    }

    /// only for testing
    pub(crate) fn saga() -> Self {
        Prefecture {
            name: "佐賀県".to_string(),
            cities: vec![
                "佐賀市".to_string(),
                "唐津市".to_string(),
                "鳥栖市".to_string(),
                "多久市".to_string(),
                "伊万里市".to_string(),
                "武雄市".to_string(),
                "鹿島市".to_string(),
                "小城市".to_string(),
                "嬉野市".to_string(),
                "神埼市".to_string(),
                "神埼郡吉野ヶ里町".to_string(),
                "三養基郡基山町".to_string(),
                "三養基郡上峰町".to_string(),
                "三養基郡みやき町".to_string(),
                "東松浦郡玄海町".to_string(),
                "西松浦郡有田町".to_string(),
                "杵島郡大町町".to_string(),
                "杵島郡江北町".to_string(),
                "杵島郡白石町".to_string(),
                "藤津郡太良町".to_string(),
            ],
        }
    }
}
