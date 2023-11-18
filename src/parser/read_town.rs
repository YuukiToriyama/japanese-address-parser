use crate::entity::City;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

pub fn read_town(input: &str, city: City) -> Option<(&str, &str)> {
    for town in city.towns {
        match tag::<&str, &str, VerboseError<&str>>(town.name.as_str()).parse(input) {
            Ok(result) => return Some(result),
            Err(_) => {}
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
                    lat: 35.016292,
                    lng: 138.489362,
                },
                Town {
                    name: "新丹谷".to_string(),
                    koaza: "".to_string(),
                    lat: 35.072403,
                    lng: 138.474199,
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
}