use crate::entity::Prefecture;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

pub fn read_city(input: &str, prefecture: Prefecture) -> Option<(String, String)> {
    for city_name in prefecture.cities {
        match tag::<&str, &str, VerboseError<&str>>(&city_name).parse(input) {
            Ok((rest, city_name)) => return Some((rest.to_string(), city_name.to_string())),
            Err(_) => {}
        };
        if city_name.contains("ヶ") {
            let edited_city_name = city_name.replace("ヶ", "ケ");
            match tag::<&str, &str, VerboseError<&str>>(&edited_city_name).parse(input) {
                Ok((rest, _)) => return Some((rest.to_string(), city_name.to_string())),
                Err(_) => {}
            };
        };
    }
    None
}

#[cfg(test)]
mod parser_tests {
    use crate::entity::Prefecture;
    use crate::parser::read_city::read_city;

    #[test]
    fn read_city_成功_京都市山科区() {
        let prefecture = Prefecture {
            name: "京都府".to_string(),
            cities: vec![
                "京都市北区".to_string(),
                "京都市上京区".to_string(),
                "京都市山科区".to_string(),
                "京都市西京区".to_string(),
            ],
        };
        let (rest, city) = read_city("京都市山科区椥辻池尻町14-2", prefecture).unwrap();
        assert_eq!(rest, "椥辻池尻町14-2");
        assert_eq!(city, "京都市山科区");
    }

    #[test]
    fn read_city_失敗_市区町村名が誤っている() {
        let prefecture = Prefecture {
            name: "京都府".to_string(),
            cities: vec![
                "京都市北区".to_string(),
                "京都市上京区".to_string(),
                "京都市山科区".to_string(),
                "京都市西京区".to_string(),
            ],
        };
        assert_eq!(read_city("港区芝公園4丁目2-8", prefecture), None);
    }

    #[test]
    fn read_city_表記ゆれ_茅ヶ崎市() {
        let prefecture = Prefecture {
            name: "神奈川県".to_string(),
            cities: vec![
                "鎌倉市".to_string(),
                "藤沢市".to_string(),
                "小田原市".to_string(),
                "茅ヶ崎市".to_string(),
                "逗子市".to_string(),
            ],
        };
        let (rest, city) = read_city("茅ケ崎市香川5丁目1", prefecture).unwrap();
        assert_eq!(rest, "香川5丁目1");
        assert_eq!(city, "茅ヶ崎市");
    }
}
