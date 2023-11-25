use crate::entity::City;
use crate::parser::adapter::adapt_variety_of_spelling;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

pub fn read_town(input: &str, city: City) -> Option<(String, String)> {
    for town in city.towns {
        if let Ok((rest, town_name)) = tag::<&str, &str, VerboseError<&str>>(town.name.as_str()).parse(input) {
            return Some((rest.to_string(), town_name.to_string()));
        }
        // 「の」「ノ」の表記ゆれに対応する
        if let Some(result) = adapt_variety_of_spelling(input, &town.name, vec!["の", "ノ"]) {
            return Some(result);
        }
        // 「ツ」「ッ」の表記ゆれに対応する
        if let Some(result) = adapt_variety_of_spelling(input, &town.name, vec!["ツ", "ッ"]) {
            return Some(result);
        }
    }
    None
}

#[cfg(test)]
mod parser_tests {
    use crate::entity::{City, Town};
    use crate::parser::read_town::read_town;

    #[test]
    fn read_town_成功_静岡市清水区旭町() {
        let city = City {
            name: "静岡市清水区".to_string(),
            towns: vec![
                Town {
                    name: "旭町".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.016292),
                    lng: Some(138.489362),
                },
                Town {
                    name: "新丹谷".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.072403),
                    lng: Some(138.474199),
                },
            ],
        };
        let (rest, town) = read_town("旭町6-8", city).unwrap();
        assert_eq!(rest, "6-8");
        assert_eq!(town, "旭町".to_string());
    }

    #[test]
    fn read_town_失敗_町名がない場合() {
        let city = City {
            name: "静岡市清水区".to_string(),
            towns: vec![],
        };
        assert_eq!(read_town("旭町6-8", city), None);
    }

    #[test]
    fn read_town_表記ゆれ_東京都千代田区丸の内() {
        let city = generate_city_東京都千代田区();
        let (rest, town) = read_town("丸ノ内一丁目9", city).unwrap();
        assert_eq!(rest, "9");
        assert_eq!(town, "丸の内一丁目");
    }

    #[test]
    fn read_town_表記ゆれ_東京都千代田区一ツ橋() {
        let city = generate_city_東京都千代田区();
        let (rest, town) = read_town("一ッ橋二丁目1番", city).unwrap();
        assert_eq!(rest, "1番");
        assert_eq!(town, "一ツ橋二丁目");
    }

    fn generate_city_東京都千代田区() -> City {
        City {
            name: "千代田区".to_string(),
            towns: vec![
                Town {
                    name: "富士見一丁目".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.697871),
                    lng: Some(139.746978),
                },
                Town {
                    name: "富士見二丁目".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.698126),
                    lng: Some(139.743057),
                },
                Town {
                    name: "丸の内一丁目".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.68156),
                    lng: Some(139.767201),
                },
                Town {
                    name: "一ツ橋一丁目".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.691189),
                    lng: Some(139.757119),
                },
                Town {
                    name: "一ツ橋二丁目".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.693171),
                    lng: Some(139.757346),
                },
            ],
        }
    }

    #[test]
    fn read_town_表記ゆれ_京都府京都市左京区松ケ崎杉ケ海道町() {
        let city = generate_city_京都府京都市左京区();
        let (rest, town) = read_town("松ヶ崎杉ヶ海道町1", city).unwrap();
        assert_eq!(rest, "1");
        assert_eq!(town, "松ケ崎杉ケ海道町");
    }

    fn generate_city_京都府京都市左京区() -> City {
        City {
            name: "京都市左京区".to_string(),
            towns: vec![
                Town {
                    name: "松ケ崎杉ケ海道町".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.047438),
                    lng: Some(135.779877),
                },
                Town {
                    name: "松ケ崎西池ノ内町".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.054046),
                    lng: Some(135.773686),
                },
                Town {
                    name: "松ケ崎井出ケ鼻町".to_string(),
                    koaza: "".to_string(),
                    lat: Some(35.056292),
                    lng: Some(135.790852),
                },
            ],
        }
    }
}
