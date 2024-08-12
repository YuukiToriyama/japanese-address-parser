use crate::util::sequence_matcher::SequenceMatcher;

pub struct VagueExpressionAdapter;

impl VagueExpressionAdapter {
    pub fn apply(self, input: &str, region_name_list: &[String]) -> Option<(String, String)> {
        if let Ok(highest_match) =
            SequenceMatcher::get_most_similar_match(input, region_name_list, None)
        {
            if let Some(position) = input.chars().position(|c| c == '町' || c == '村') {
                return Some((highest_match, input.chars().skip(position + 1).collect()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::geolonia::entity::Prefecture;
    use crate::parser::adapter::vague_expression_adapter::VagueExpressionAdapter;

    #[test]
    fn 郡名が省略されている場合_吉田郡永平寺町() {
        let fukui = Prefecture::fukui();
        let (city_name, rest) = VagueExpressionAdapter {}
            .apply("永平寺町志比５－５", &fukui.cities)
            .unwrap();
        assert_eq!(city_name, "吉田郡永平寺町");
        assert_eq!(rest, "志比５－５");
    }

    #[test]
    fn 郡名が省略されている場合_今立郡池田町() {
        let fukui = Prefecture::fukui();
        let (city_name, rest) = VagueExpressionAdapter {}
            .apply("池田町稲荷２８－７", &fukui.cities)
            .unwrap();
        assert_eq!(city_name, "今立郡池田町");
        assert_eq!(rest, "稲荷２８－７");
    }

    #[test]
    fn 郡名が省略されている場合_南条郡南越前町() {
        let fukui = Prefecture::fukui();
        let (city_name, rest) = VagueExpressionAdapter {}
            .apply("南越前町今庄７４－７－１", &fukui.cities)
            .unwrap();
        assert_eq!(city_name, "南条郡南越前町");
        assert_eq!(rest, "今庄７４－７－１");
    }

    #[test]
    fn 郡名が省略されている場合_西村山郡河北町() {
        let yamagata = Prefecture::yamagata();
        let (city_name, rest) = VagueExpressionAdapter {}
            .apply("河北町大字吉田字馬場261", &yamagata.cities)
            .unwrap();
        assert_eq!(city_name, "西村山郡河北町");
        assert_eq!(rest, "大字吉田字馬場261");
    }

    #[test]
    fn 郡名と町名が一致している場合_最上郡最上町() {
        let yamagata = Prefecture::yamagata();
        let (city_name, rest) = VagueExpressionAdapter {}
            .apply("最上町法田2672-2", &yamagata.cities)
            .unwrap();
        assert_eq!(city_name, "最上郡最上町");
        assert_eq!(rest, "法田2672-2");
    }
}
