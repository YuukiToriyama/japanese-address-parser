use crate::util::trimmer::trim_city_name;
use rapidfuzz::distance::lcs_seq;

pub struct SequenceMatcher;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MoreThanOneCandidateExist(Vec<String>),
    NoCandidateExist,
}

struct Candidate {
    similarity: f64,
    text: String,
}

impl SequenceMatcher {
    pub fn get_most_similar_match(
        input: &str,
        possibilities: &[String],
        threshold: Option<f64>,
    ) -> Result<String, Error> {
        let mut candidates: Vec<Candidate> =
            Self::get_most_similar_matches(input, possibilities, threshold)
                .into_iter()
                .map(|candidate| {
                    // 郡名を取り除いた部分がinputの先頭に一致する場合はそのまま、一致しない場合は類似度に0.9を掛ける
                    if input.starts_with(&trim_city_name(&candidate.text)) {
                        candidate
                    } else {
                        Candidate {
                            similarity: candidate.similarity * 0.9,
                            text: candidate.text,
                        }
                    }
                })
                .collect();
        if candidates.is_empty() {
            return Err(Error::NoCandidateExist);
        }
        // 類似度で並び替える
        candidates.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        let highest_similarity = candidates[0].similarity;
        // 類似度が一位のものだけを抽出する
        let highest_matches: Vec<String> = candidates
            .iter()
            .filter(|candidate| candidate.similarity == highest_similarity)
            .map(|candidate| candidate.text.clone())
            .collect();
        match &highest_matches.len() {
            1 => Ok(highest_matches[0].clone()),
            _ => Err(Error::MoreThanOneCandidateExist(highest_matches)),
        }
    }

    fn get_most_similar_matches(
        input: &str,
        possibilities: &[String],
        threshold: Option<f64>,
    ) -> Vec<Candidate> {
        let mut highest_similarity = 0.0;
        let mut highest_matches = Vec::with_capacity(possibilities.len());
        let length_of_longest_possibility = Self::get_length_of_longest_one(possibilities).unwrap();
        let input = Self::cut_text(input, length_of_longest_possibility);
        for possibility in possibilities {
            let similarity = Self::evaluate_match_ratio(possibility, &input);
            if similarity >= highest_similarity {
                if similarity > highest_similarity {
                    highest_matches.clear();
                }
                if similarity > threshold.unwrap_or(0.0) {
                    highest_matches.push(Candidate {
                        similarity,
                        text: possibility.clone(),
                    });
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
        input.chars().take(length).collect()
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
    use crate::domain::geolonia::entity::Prefecture;
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
}
