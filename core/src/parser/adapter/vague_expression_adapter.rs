use crate::util::sequence_matcher::SequenceMatcher;

pub struct VagueExpressionAdapter;

impl VagueExpressionAdapter {
    pub fn apply(self, input: &str, region_name_list: &[String]) -> Option<(String, String)> {
        if let Ok(highest_match) =
            SequenceMatcher::get_most_similar_match(input, region_name_list, None)
        {
            if let Ok(complemented_address) = complement_county_name(input, &highest_match) {
                return Some((
                    highest_match.clone(),
                    complemented_address
                        .chars()
                        .skip(highest_match.chars().count())
                        .collect(),
                ));
            }
        }
        None
    }
}

/// 郡名が抜けている住所に郡名を補う関数
///
/// 欠けている郡名を補うだけで、それ以上のことはしない。
/// 市区町村名に表記揺れがあってもそれを上書きすることはしない。
fn complement_county_name(vague_address: &str, with: &str) -> Result<String, &'static str> {
    match with.chars().position(|c| c == '郡') {
        None => Err("郡名が見つかりませんでした"),
        Some(position) => Ok(with.chars().take(position + 1).collect::<String>() + vague_address),
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::geolonia::entity::Prefecture;
    use crate::parser::adapter::vague_expression_adapter::{
        complement_county_name, VagueExpressionAdapter,
    };

    #[test]
    fn complement_county_name_郡名が省略されている場合() {
        assert_eq!(
            complement_county_name("大町町大字福母297", "杵島郡大町町").unwrap(),
            "杵島郡大町町大字福母297"
        );
        assert_eq!(
            complement_county_name("村田町大字村田字迫6", "柴田郡村田町").unwrap(),
            "柴田郡村田町大字村田字迫6"
        );
        assert_eq!(
            complement_county_name("玉村町上新田1116", "佐波郡玉村町").unwrap(),
            "佐波郡玉村町上新田1116"
        );
        // 市区町村名に表記揺れも含む場合
        assert_eq!(
            complement_county_name("桧原村上元郷403", "西多摩郡檜原村").unwrap(),
            "西多摩郡桧原村上元郷403"
        )
    }

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
    fn 郡名が省略されている場合_杵島郡大町町() {
        let saga = Prefecture::saga();
        let (city_name, rest) = VagueExpressionAdapter {}
            .apply("大町町大字大町5017番地", &saga.cities)
            .unwrap();
        assert_eq!(city_name, "杵島郡大町町");
        assert_eq!(rest, "大字大町5017番地");
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
