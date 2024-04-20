use crate::util::sequence_matcher::SequenceMatcher;
use nom::bytes::complete::{is_a, is_not};
use nom::combinator::rest;
use nom::error::Error;
use nom::sequence::tuple;

pub struct VagueExpressionAdapter;

impl VagueExpressionAdapter {
    pub fn apply(self, input: &str, region_name_list: &Vec<String>) -> Option<(String, String)> {
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
    use crate::parser::adapter::vague_expression_adapter::VagueExpressionAdapter;

    #[test]
    fn 郡名が省略されている場合_吉田郡永平寺町() {
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("永平寺町志比５－５", &provide_city_name_list())
            .unwrap();
        assert_eq!(rest, "志比５－５");
        assert_eq!(city_name, "吉田郡永平寺町");
    }

    #[test]
    fn 郡名が省略されている場合_今立郡池田町() {
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("池田町稲荷２８－７", &provide_city_name_list())
            .unwrap();
        assert_eq!(rest, "稲荷２８－７");
        assert_eq!(city_name, "今立郡池田町");
    }

    #[test]
    fn 郡名が省略されている場合_南条郡南越前町() {
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("南越前町今庄７４－７－１", &provide_city_name_list())
            .unwrap();
        assert_eq!(rest, "今庄７４－７－１");
        assert_eq!(city_name, "南条郡南越前町");
    }

    fn provide_city_name_list() -> Vec<String> {
        vec![
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
        ]
    }
}
