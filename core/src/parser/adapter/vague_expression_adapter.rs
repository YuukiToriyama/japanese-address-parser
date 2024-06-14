use crate::util::sequence_matcher::SequenceMatcher;
use nom::bytes::complete::{is_a, is_not};
use nom::combinator::rest;
use nom::error::Error;
use nom::sequence::tuple;

pub struct VagueExpressionAdapter;

impl VagueExpressionAdapter {
    pub fn apply(self, input: &str, region_name_list: &[String]) -> Option<(String, String)> {
        if let Ok(highest_match) =
            SequenceMatcher::get_most_similar_match(input, region_name_list, None)
        {
            let mut parser = tuple((
                is_not::<&str, &str, Error<&str>>("町村"),
                is_a::<&str, &str, Error<&str>>("町村"),
                rest,
            ));
            if let Ok((_, (_, _, rest))) = parser(input) {
                return Some((rest.to_string(), highest_match));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::Prefecture;
    use crate::parser::adapter::vague_expression_adapter::VagueExpressionAdapter;

    #[test]
    fn 郡名が省略されている場合_吉田郡永平寺町() {
        let hukui = Prefecture::hukui();
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("永平寺町志比５－５", &hukui.cities)
            .unwrap();
        assert_eq!(rest, "志比５－５");
        assert_eq!(city_name, "吉田郡永平寺町");
    }

    #[test]
    fn 郡名が省略されている場合_今立郡池田町() {
        let hukui = Prefecture::hukui();
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("池田町稲荷２８－７", &hukui.cities)
            .unwrap();
        assert_eq!(rest, "稲荷２８－７");
        assert_eq!(city_name, "今立郡池田町");
    }

    #[test]
    fn 郡名が省略されている場合_南条郡南越前町() {
        let hukui = Prefecture::hukui();
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("南越前町今庄７４－７－１", &hukui.cities)
            .unwrap();
        assert_eq!(rest, "今庄７４－７－１");
        assert_eq!(city_name, "南条郡南越前町");
    }

    #[test]
    fn 郡名が省略されている場合_西村山郡河北町() {
        let yamagata = Prefecture::yamagata();
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("河北町大字吉田字馬場261", &yamagata.cities)
            .unwrap();
        assert_eq!(rest, "大字吉田字馬場261");
        assert_eq!(city_name, "西村山郡河北町");
    }

    #[test]
    fn 郡名と町名が一致している場合_最上郡最上町() {
        let yamagata = Prefecture::yamagata();
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("最上町法田2672-2", &yamagata.cities)
            .unwrap();
        assert_eq!(rest, "法田2672-2");
        assert_eq!(city_name, "最上郡最上町");
    }

    impl Prefecture {
        fn yamagata() -> Self {
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

        fn hukui() -> Self {
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
    }
}
