use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

use crate::entity::City;
use crate::parser::adapter::orthographical_variant_adapter::OrthographicalVariantAdapter;
use crate::parser::filter::Filter;
use crate::parser::filter::fullwidth_character::FullwidthCharacterFilter;
use crate::parser::filter::non_kanji_block_number::NonKanjiBlockNumberFilter;

pub fn read_town(input: &str, city: &City) -> Option<(String, String)> {
    let mut input: String = input.to_string();
    if input.contains("丁目") {
        input = FullwidthCharacterFilter {}.apply(input);
        input = NonKanjiBlockNumberFilter {}.apply(input);
    }
    for town in &city.towns {
        if let Ok((rest, town_name)) =
            tag::<&str, &str, VerboseError<&str>>(town.name.as_str()).parse(&input)
        {
            return Some((rest.to_string(), town_name.to_string()));
        }
        let adapter = OrthographicalVariantAdapter {
            variant_list: vec![
                vec!["の", "ノ"],
                vec!["ツ", "ッ"],
                vec!["ケ", "ヶ", "が", "ガ"],
                vec!["薮", "藪", "籔"],
                vec!["崎", "﨑"],
            ],
        };
        if let Some(result) = adapter.apply(&input, &town.name) {
            return Some(result);
        };
    }
    None
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod parser_tests {
    use crate::api::blocking::Client;
    use crate::api::BlockingApi;
    use crate::entity::{City, Town};
    use crate::parser::read_town::read_town;

    #[test]
    fn read_town_成功_静岡市清水区旭町() {
        let city = City {
            name: "静岡市清水区".to_string(),
            towns: vec![
                Town::new("旭町", "", 35.016292, 138.489362),
                Town::new("新丹谷", "", 35.072403, 138.474199),
            ],
        };
        let (rest, town) = read_town("旭町6-8", &city).unwrap();
        assert_eq!(rest, "6-8");
        assert_eq!(town, "旭町".to_string());
    }

    #[test]
    fn read_town_失敗_町名がない場合() {
        let city = City {
            name: "静岡市清水区".to_string(),
            towns: vec![],
        };
        assert_eq!(read_town("旭町6-8", &city), None);
    }

    #[test]
    fn read_town_表記ゆれ_東京都千代田区丸の内() {
        let city = generate_city_東京都千代田区();
        let (rest, town) = read_town("丸ノ内一丁目9", &city).unwrap();
        assert_eq!(rest, "9");
        assert_eq!(town, "丸の内一丁目");
    }

    #[test]
    fn read_town_表記ゆれ_東京都千代田区一ツ橋() {
        let city = generate_city_東京都千代田区();
        let (rest, town) = read_town("一ッ橋二丁目1番", &city).unwrap();
        assert_eq!(rest, "1番");
        assert_eq!(town, "一ツ橋二丁目");
    }

    fn generate_city_東京都千代田区() -> City {
        City {
            name: "千代田区".to_string(),
            towns: vec![
                Town::new("富士見一丁目", "", 35.697871, 139.746978),
                Town::new("富士見二丁目", "", 35.698126, 139.743057),
                Town::new("丸の内一丁目", "", 35.68156, 139.767201),
                Town::new("一ツ橋一丁目", "", 35.691189, 139.757119),
                Town::new("一ツ橋二丁目", "", 35.693171, 139.757346),
            ],
        }
    }

    #[test]
    fn read_town_表記ゆれ_京都府京都市左京区松ケ崎杉ケ海道町() {
        let city = generate_city_京都府京都市左京区();
        let (rest, town) = read_town("松ヶ崎杉ヶ海道町1", &city).unwrap();
        assert_eq!(rest, "1");
        assert_eq!(town, "松ケ崎杉ケ海道町");
    }

    fn generate_city_京都府京都市左京区() -> City {
        City {
            name: "京都市左京区".to_string(),
            towns: vec![
                Town::new("松ケ崎杉ケ海道町", "", 35.047438, 135.779877),
                Town::new("松ケ崎西池ノ内町", "", 35.054046, 135.773686),
                Town::new("松ケ崎井出ケ鼻町", "", 35.056292, 135.790852),
            ],
        }
    }

    #[test]
    fn read_town_異字体_岐阜県岐阜市薮田南二丁目() {
        let city = City {
            name: "岐阜県岐阜市".to_string(),
            towns: vec![
                Town::new("薮田南一丁目", "", 35.394373, 136.723208),
                Town::new("薮田南二丁目", "", 35.391964, 136.723151),
                Town::new("薮田南三丁目", "", 35.3896, 136.723086),
            ],
        };
        let (_, town) = read_town("薮田南二丁目", &city).unwrap();
        assert_eq!(town, "薮田南二丁目");
        let (_, town) = read_town("藪田南二丁目", &city).unwrap();
        assert_eq!(town, "薮田南二丁目");
        let (_, town) = read_town("籔田南二丁目", &city).unwrap();
        assert_eq!(town, "薮田南二丁目");
    }

    #[test]
    fn read_town_丁目が算用数字の場合_京都府京都市東山区n丁目() {
        let client = Client {};
        let city = client.get_city_master("京都府", "京都市東山区").unwrap();
        let test_cases = vec![
            ("本町1丁目45番", "本町一丁目"),
            ("本町2丁目64番", "本町二丁目"),
            ("本町10丁目169番", "本町十丁目"),
            ("本町12丁目224番", "本町十二丁目"),
            ("本町20丁目435番", "本町二十丁目"),
            ("本町22丁目489番", "本町二十二丁目"),
        ];
        for (input, town_name) in test_cases {
            let (_, town) = read_town(input, &city).unwrap();
            assert_eq!(town, town_name);
        }
    }
}
