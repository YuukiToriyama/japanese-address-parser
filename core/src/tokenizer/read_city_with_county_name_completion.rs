use crate::tokenizer::{CityNameFound, CityNameNotFound, End, Tokenizer};
use crate::util::sequence_matcher::SequenceMatcher;
use std::marker::PhantomData;

impl Tokenizer<CityNameNotFound> {
    pub(crate) fn read_city_with_county_name_completion(
        &self,
        candidates: Vec<String>,
    ) -> Result<Tokenizer<CityNameFound>, Tokenizer<End>> {
        if let Ok(highest_match) =
            SequenceMatcher::get_most_similar_match(&self.rest, &candidates, None)
        {
            if let Ok(complemented_address) = complement_county_name(&self.rest, &highest_match) {
                return Ok(Tokenizer {
                    input: self.input.clone(),
                    prefecture_name: self.prefecture_name.clone(),
                    city_name: Some(highest_match.clone()),
                    town_name: None,
                    rest: complemented_address
                        .chars()
                        .skip(highest_match.chars().count())
                        .collect(),
                    _state: PhantomData::<CityNameFound>,
                });
            }
        }
        Err(Tokenizer {
            input: self.input.clone(),
            prefecture_name: self.prefecture_name.clone(),
            city_name: None,
            town_name: None,
            rest: self.rest.clone(),
            _state: PhantomData::<End>,
        })
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
    use crate::tokenizer::read_city_with_county_name_completion::complement_county_name;
    use crate::tokenizer::{CityNameNotFound, Tokenizer};
    use std::marker::PhantomData;

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
    fn read_city_with_county_name_completion_秩父郡東秩父村() {
        let tokenizer = Tokenizer {
            input: "埼玉県東秩父村大字御堂634番地".to_string(), // 「秩父郡」が省略されている
            prefecture_name: Some("埼玉県".to_string()),
            city_name: None,
            town_name: None,
            rest: "東秩父村大字御堂634番地".to_string(),
            _state: PhantomData::<CityNameNotFound>,
        };
        let result = tokenizer.read_city_with_county_name_completion(vec![
            "秩父郡皆野町".to_string(),
            "秩父郡長瀞町".to_string(),
            "秩父郡小鹿野町".to_string(),
            "秩父郡東秩父村".to_string(),
        ]);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.input, "埼玉県東秩父村大字御堂634番地");
        assert_eq!(tokenizer.prefecture_name, Some("埼玉県".to_string()));
        assert_eq!(tokenizer.city_name, Some("秩父郡東秩父村".to_string()));
        assert_eq!(tokenizer.town_name, None);
        assert_eq!(tokenizer.rest, "大字御堂634番地");
    }
}
