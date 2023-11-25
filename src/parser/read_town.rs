use crate::entity::City;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

pub fn read_town(input: &str, city: City) -> Option<(String, String)> {
    for town in city.towns {
        if let Ok((rest, town_name)) = tag::<&str, &str, VerboseError<&str>>(town.name.as_str()).parse(input) {
            return Some((rest.to_string(), town_name.to_string()));
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
        let city = City {
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
            ],
        };
        let (rest, town) = read_town("丸ノ内一丁目9", city).unwrap();
        assert_eq!(rest, "9");
        assert_eq!(town, "丸の内一丁目");
    }
}
