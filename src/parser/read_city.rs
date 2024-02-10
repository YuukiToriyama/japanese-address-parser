use crate::entity::Prefecture;
use crate::parser::adapter::orthographical_variant_adapter::{
    OrthographicalVariantAdapter, OrthographicalVariants, Variant,
};
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
        let mut variant_list = vec![Variant::ケ];
        match prefecture.name.as_str() {
            "宮城県" => {
                variant_list.push(Variant::竈);
            }
            "茨城県" => {
                variant_list.push(Variant::龍);
                variant_list.push(Variant::嶋);
            }
            "東京都" => {
                variant_list.push(Variant::檜);
            }
            _ => {}
        }
        let adapter = OrthographicalVariantAdapter { variant_list };
        if let Some(result) = adapter.apply(input, &city_name) {
            return Some(result);
        }
    }
    None
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::api::{BlockingApi, BlockingApiImpl};
    use crate::parser::read_city::read_city;
    use test_case::test_case;

    #[test_case("京都府", "京都市山科区椥辻池尻町14-2", "京都市山科区"; "success_京都市山科区")]
    #[test_case("神奈川県", "茅ヶ崎市香川5丁目1", "茅ヶ崎市"; "success_茅ヶ崎市")]
    #[test_case("神奈川県", "茅ケ崎市香川5丁目1", "茅ヶ崎市"; "success_茅ケ崎市_表記ゆれ")]
    #[test_case("神奈川県", "横浜市保土ケ谷区川辺町2番地9", "横浜市保土ケ谷区"; "success_横浜市保土ケ谷区")]
    #[test_case("神奈川県", "横浜市保土ヶ谷区川辺町2番地9", "横浜市保土ケ谷区"; "success_横浜市保土ヶ谷区_表記ゆれ")]
    #[test_case("岐阜県", "不破郡関ケ原町大字関ケ原894番地の58", "不破郡関ケ原町"; "success_不破郡関ケ原町")]
    #[test_case("岐阜県", "不破郡関が原町大字関ケ原894番地の58", "不破郡関ケ原町"; "success_不破郡関が原町_表記ゆれ")]
    #[test_case("茨城県", "龍ヶ崎市佐貫町647", "龍ヶ崎市"; "success_龍ヶ崎市")]
    #[test_case("茨城県", "龍ケ崎市佐貫町647", "龍ヶ崎市"; "success_龍ケ崎市_表記ゆれ")]
    #[test_case("茨城県", "竜ヶ崎市佐貫町647", "龍ヶ崎市"; "success_竜ヶ崎市_表記ゆれ")]
    #[test_case("茨城県", "竜ケ崎市佐貫町647", "龍ヶ崎市"; "success_竜ケ崎市_表記ゆれ")]
    fn test_read_city(prefecture_name: &str, input: &str, expected: &str) {
        let api = BlockingApiImpl::new();
        let prefecture = api.get_prefecture_master(prefecture_name).unwrap();
        let (_, city_name) = read_city(input, prefecture).unwrap();
        assert_eq!(city_name, expected);
    }
}
