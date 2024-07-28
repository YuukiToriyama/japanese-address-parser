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
    use crate::parser::adapter::vague_expression_adapter::VagueExpressionAdapter;
    use domain::geolonia::entity::Prefecture;

    #[test]
    fn 郡名が省略されている場合_吉田郡永平寺町() {
        let fukui = Prefecture::fukui();
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("永平寺町志比５－５", &fukui.cities)
            .unwrap();
        assert_eq!(rest, "志比５－５");
        assert_eq!(city_name, "吉田郡永平寺町");
    }

    #[test]
    fn 郡名が省略されている場合_今立郡池田町() {
        let fukui = Prefecture::fukui();
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("池田町稲荷２８－７", &fukui.cities)
            .unwrap();
        assert_eq!(rest, "稲荷２８－７");
        assert_eq!(city_name, "今立郡池田町");
    }

    #[test]
    fn 郡名が省略されている場合_南条郡南越前町() {
        let fukui = Prefecture::fukui();
        let (rest, city_name) = VagueExpressionAdapter {}
            .apply("南越前町今庄７４－７－１", &fukui.cities)
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
}
