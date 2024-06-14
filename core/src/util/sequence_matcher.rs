use crate::util::trimmer::trim_city_name;
use rapidfuzz::distance::lcs_seq;

pub struct SequenceMatcher;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MoreThanOneCandidateExist(Vec<String>),
    NoCandidateExist,
}

impl SequenceMatcher {
    pub fn get_most_similar_match(
        input: &str,
        possibilities: &[String],
        threshold: Option<f64>,
    ) -> Result<String, Error> {
        let highest_matches: Vec<String> =
            Self::get_most_similar_matches(input, possibilities, threshold)
                .into_iter()
                .filter(|candidate| input.starts_with(&trim_city_name(candidate)))
                .collect();
        match &highest_matches.len() {
            0 => Err(Error::NoCandidateExist),
            1 => Ok(highest_matches.first().unwrap().clone()),
            _ => Err(Error::MoreThanOneCandidateExist(highest_matches)),
        }
    }

    fn get_most_similar_matches(
        input: &str,
        possibilities: &[String],
        threshold: Option<f64>,
    ) -> Vec<String> {
        let mut highest_similarity: f64 = 0.0;
        let mut highest_matches: Vec<String> = vec![];
        let length_of_longest_possibility = Self::get_length_of_longest_one(possibilities).unwrap();
        let input = Self::cut_text(input, length_of_longest_possibility);
        for possibility in possibilities {
            let similarity = Self::evaluate_match_ratio(possibility, &input);
            if similarity >= highest_similarity {
                if similarity > highest_similarity {
                    highest_matches.clear();
                }
                if threshold.is_none() || similarity > threshold.unwrap() {
                    highest_matches.push(possibility.clone());
                }
                highest_similarity = similarity;
            }
        }
        highest_matches
    }

    fn get_length_of_longest_one(text_list: &[String]) -> Option<usize> {
        text_list.iter().map(|x| x.chars().count()).max()
    }

    fn cut_text(input: &str, length: usize) -> String {
        if input.chars().count() > length {
            input.chars().take(length).collect::<String>()
        } else {
            input.to_string()
        }
    }

    fn evaluate_match_ratio(left: &str, right: &str) -> f64 {
        if left == right {
            return 1.0;
        }
        lcs_seq::normalized_similarity(left.chars(), right.chars())
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::Prefecture;
    use crate::util::sequence_matcher::Error::{MoreThanOneCandidateExist, NoCandidateExist};
    use crate::util::sequence_matcher::SequenceMatcher;

    #[test]
    fn get_length_of_longest_one() {
        let fukushima = Prefecture::fukushima();
        assert_eq!(SequenceMatcher::get_length_of_longest_one(&vec![]), None);
        assert_eq!(
            SequenceMatcher::get_length_of_longest_one(&fukushima.cities),
            Some(8)
        );
    }

    #[test]
    fn cut_text() {
        let city_name = "南会津郡檜枝岐村";
        assert_eq!(SequenceMatcher::cut_text(city_name, 0), "");
        assert_eq!(SequenceMatcher::cut_text(city_name, 1), "南");
        assert_eq!(SequenceMatcher::cut_text(city_name, 8), "南会津郡檜枝岐村");
        assert_eq!(SequenceMatcher::cut_text(city_name, 9), "南会津郡檜枝岐村");
    }

    #[test]
    fn evaluate_match_ratio_一致度100() {
        assert_eq!(
            SequenceMatcher::evaluate_match_ratio("相馬郡新地町", "相馬郡新地町"),
            1.0
        );
    }

    #[test]
    fn evaluate_match_ratio_一致度50() {
        assert_eq!(
            SequenceMatcher::evaluate_match_ratio("相馬郡新地町", "相馬郡飯舘村"),
            0.5
        );
    }

    #[test]
    fn evaluate_match_ratio_一致度0() {
        assert_eq!(
            SequenceMatcher::evaluate_match_ratio("相馬郡新地町", "福島市"),
            0.0
        );
    }

    #[test]
    fn get_most_similar_match() {
        let fukushima = Prefecture::fukushima();
        let possibilities = fukushima.cities;
        let result = SequenceMatcher::get_most_similar_match(
            "西郷村大字熊倉字折口原40番地",
            &possibilities,
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "西白河郡西郷村");
        let result = SequenceMatcher::get_most_similar_match(
            "小野町大字小野新町字舘廻",
            &possibilities,
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "田村郡小野町");
        let result = SequenceMatcher::get_most_similar_match(
            "桑折町大字谷地字道下22番地7",
            &possibilities,
            None,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "伊達郡桑折町");
    }

    #[test]
    fn get_most_similar_match_類似度が同じものが複数ある場合() {
        let possibilities = vec!["周智郡森町".to_string(), "茅部郡森町".to_string()];
        assert_eq!(
            SequenceMatcher::evaluate_match_ratio("森町", &possibilities[0]),
            SequenceMatcher::evaluate_match_ratio("森町", &possibilities[1])
        );
        let result = SequenceMatcher::get_most_similar_match("森町", &possibilities, None);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            MoreThanOneCandidateExist(vec!["周智郡森町".to_string(), "茅部郡森町".to_string()])
        );
    }

    #[test]
    fn get_most_similar_match_マッチ候補が一つもない場合() {
        let result = SequenceMatcher::get_most_similar_match(
            "上町",
            &vec!["上村".to_string(), "下町".to_string()],
            Some(0.9),
        );
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), NoCandidateExist);
    }

    impl Prefecture {
        fn fukushima() -> Self {
            Prefecture {
                name: "福島県".to_string(),
                cities: vec![
                    "福島市".to_string(),
                    "会津若松市".to_string(),
                    "郡山市".to_string(),
                    "いわき市".to_string(),
                    "白河市".to_string(),
                    "須賀川市".to_string(),
                    "喜多方市".to_string(),
                    "相馬市".to_string(),
                    "二本松市".to_string(),
                    "田村市".to_string(),
                    "南相馬市".to_string(),
                    "伊達市".to_string(),
                    "本宮市".to_string(),
                    "伊達郡桑折町".to_string(),
                    "伊達郡国見町".to_string(),
                    "伊達郡川俣町".to_string(),
                    "安達郡大玉村".to_string(),
                    "岩瀬郡鏡石町".to_string(),
                    "岩瀬郡天栄村".to_string(),
                    "南会津郡下郷町".to_string(),
                    "南会津郡檜枝岐村".to_string(),
                    "南会津郡只見町".to_string(),
                    "南会津郡南会津町".to_string(),
                    "耶麻郡北塩原村".to_string(),
                    "耶麻郡西会津町".to_string(),
                    "耶麻郡磐梯町".to_string(),
                    "耶麻郡猪苗代町".to_string(),
                    "河沼郡会津坂下町".to_string(),
                    "河沼郡湯川村".to_string(),
                    "河沼郡柳津町".to_string(),
                    "大沼郡三島町".to_string(),
                    "大沼郡金山町".to_string(),
                    "大沼郡昭和村".to_string(),
                    "大沼郡会津美里町".to_string(),
                    "西白河郡西郷村".to_string(),
                    "西白河郡泉崎村".to_string(),
                    "西白河郡中島村".to_string(),
                    "西白河郡矢吹町".to_string(),
                    "東白川郡棚倉町".to_string(),
                    "東白川郡矢祭町".to_string(),
                    "東白川郡塙町".to_string(),
                    "東白川郡鮫川村".to_string(),
                    "石川郡石川町".to_string(),
                    "石川郡玉川村".to_string(),
                    "石川郡平田村".to_string(),
                    "石川郡浅川町".to_string(),
                    "石川郡古殿町".to_string(),
                    "田村郡三春町".to_string(),
                    "田村郡小野町".to_string(),
                    "双葉郡広野町".to_string(),
                    "双葉郡楢葉町".to_string(),
                    "双葉郡富岡町".to_string(),
                    "双葉郡川内村".to_string(),
                    "双葉郡大熊町".to_string(),
                    "双葉郡双葉町".to_string(),
                    "双葉郡浪江町".to_string(),
                    "双葉郡葛尾村".to_string(),
                    "相馬郡新地町".to_string(),
                    "相馬郡飯舘村".to_string(),
                ],
            }
        }
    }
}
