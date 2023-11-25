use crate::entity::Prefecture;
use crate::parser::adapter::adapt_variety_of_spelling;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

pub fn read_city(input: &str, prefecture: Prefecture) -> Option<(String, String)> {
    for city_name in prefecture.cities {
        match tag::<&str, &str, VerboseError<&str>>(&city_name).parse(input) {
            Ok((rest, city_name)) => return Some((rest.to_string(), city_name.to_string())),
            Err(_) => {}
        };
        // 「ケ」「ヶ」「が」の表記ゆれに対応する
        if let Some(result) = adapt_variety_of_spelling(input, city_name, vec!["ケ", "ヶ", "が"]) {
            return Some(result);
        }
    }
    None
}

#[cfg(test)]
mod parser_tests {
    use crate::entity::Prefecture;
    use crate::parser::read_city::read_city;

    #[test]
    fn read_city_成功_京都市山科区() {
        let prefecture = Prefecture::new(
            "京都府",
            vec!["京都市北区", "京都市上京区", "京都市山科区", "京都市西京区"],
        );
        let (rest, city) = read_city("京都市山科区椥辻池尻町14-2", prefecture).unwrap();
        assert_eq!(rest, "椥辻池尻町14-2");
        assert_eq!(city, "京都市山科区");
    }

    #[test]
    fn read_city_失敗_市区町村名が誤っている() {
        let prefecture = Prefecture::new(
            "京都府",
            vec!["京都市北区", "京都市上京区", "京都市山科区", "京都市西京区"],
        );
        assert_eq!(read_city("港区芝公園4丁目2-8", prefecture), None);
    }

    #[test]
    fn read_city_表記ゆれ_茅ヶ崎市() {
        let prefecture = Prefecture::new(
            "神奈川県",
            vec!["鎌倉市", "藤沢市", "小田原市", "茅ヶ崎市", "逗子市"],
        );
        let (rest, city) = read_city("茅ケ崎市香川5丁目1", prefecture).unwrap();
        assert_eq!(rest, "香川5丁目1");
        assert_eq!(city, "茅ヶ崎市");
    }

    #[test]
    fn read_city_表記ゆれ_横浜市保土ケ谷区() {
        let prefecture = Prefecture::new(
            "神奈川県",
            vec![
                "横浜市中区",
                "横浜市南区",
                "横浜市保土ケ谷区",
                "横浜市磯子区",
            ],
        );
        let (rest, city) = read_city("横浜市保土ヶ谷区川辺町2番地9", prefecture).unwrap();
        assert_eq!(rest, "川辺町2番地9");
        assert_eq!(city, "横浜市保土ケ谷区");
    }

    #[test]
    fn read_city_表記ゆれ_不破郡関ケ原町() {
        let prefecture = Prefecture::new(
            "岐阜県",
            vec![
                "養老郡養老町",
                "不破郡垂井町",
                "不破郡関ケ原町",
                "安八郡神戸町",
                "安八郡輪之内町",
            ],
        );
        let (rest, city) = read_city("不破郡関が原町大字関ケ原894番地の58", prefecture).unwrap();
        assert_eq!(rest, "大字関ケ原894番地の58");
        assert_eq!(city, "不破郡関ケ原町");
    }
}
