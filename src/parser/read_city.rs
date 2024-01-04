use crate::entity::Prefecture;
use crate::parser::adapter::orthographical_variant_adapter::OrthographicalVariantAdapter;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

pub fn read_city(input: &str, prefecture: Prefecture) -> Option<(String, String)> {
    for city_name in prefecture.cities {
        if let Ok((rest, city_name)) =
            tag::<&str, &str, VerboseError<&str>>(&city_name).parse(input)
        {
            return Some((rest.to_string(), city_name.to_string()));
        }
        let adapter = OrthographicalVariantAdapter {
            variant_list: vec![vec!["ケ", "ヶ", "が"]],
        };
        if let Some(result) = adapter.apply(input, &city_name) {
            return Some(result);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::entity::Prefecture;
    use crate::api::blocking::Client;
    use crate::api::BlockingApi;
    use crate::parser::read_city::read_city;

    #[test]
    fn 京都市山科区_成功() {
        let api = Client {};
        let prefecture = api.get_prefecture_master("京都府").unwrap();
        let (rest, city) = read_city("京都市山科区椥辻池尻町14-2", prefecture).unwrap();
        assert_eq!(rest, "椥辻池尻町14-2");
        assert_eq!(city, "京都市山科区");
    }

    #[test]
    fn 市区町村名が誤っている_失敗() {
        let api = Client {};
        let prefecture = api.get_prefecture_master("京都府").unwrap();
        assert_eq!(read_city("港区芝公園4丁目2-8", prefecture), None);
    }

    #[test]
    fn 表記ゆれ_茅ヶ崎市() {
        let api = Client {};
        let prefecture = api.get_prefecture_master("神奈川県").unwrap();
        let (rest, city) = read_city("茅ケ崎市香川5丁目1", prefecture).unwrap();
        assert_eq!(rest, "香川5丁目1");
        assert_eq!(city, "茅ヶ崎市");
    }

    #[test]
    fn 表記ゆれ_横浜市保土ケ谷区() {
        let api = Client {};
        let prefecture = api.get_prefecture_master("神奈川県").unwrap();
        let (rest, city) = read_city("横浜市保土ヶ谷区川辺町2番地9", prefecture).unwrap();
        assert_eq!(rest, "川辺町2番地9");
        assert_eq!(city, "横浜市保土ケ谷区");
    }

    #[test]
    fn 表記ゆれ_不破郡関ケ原町() {
        let api = Client {};
        let prefecture = api.get_prefecture_master("岐阜県").unwrap();
        let (rest, city) = read_city("不破郡関が原町大字関ケ原894番地の58", prefecture).unwrap();
        assert_eq!(rest, "大字関ケ原894番地の58");
        assert_eq!(city, "不破郡関ケ原町");
    }
}
