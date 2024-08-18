use crate::tokenizer::{CityNameFound, CityNameNotFound, End, Tokenizer};
use crate::util::sequence_matcher::SequenceMatcher;
use std::marker::PhantomData;

impl Tokenizer<CityNameNotFound> {
    pub(crate) fn read_city_with_county_name_completion(
        &self,
        candidates: &[String],
    ) -> Result<Tokenizer<CityNameFound>, Tokenizer<End>> {
        if let Ok(highest_match) =
            SequenceMatcher::get_most_similar_match(&self.rest, candidates, None)
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
    use crate::domain::geolonia::entity::Prefecture;
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
        let result = tokenizer.read_city_with_county_name_completion(&vec![
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

    #[test]
    fn read_city_with_county_name_completion_吉田郡永平寺町() {
        let tokenizer = Tokenizer {
            input: "".to_string(),
            prefecture_name: None,
            city_name: None,
            town_name: None,
            rest: "永平寺町志比５－５".to_string(),
            _state: PhantomData::<CityNameNotFound>,
        };
        let result = tokenizer.read_city_with_county_name_completion(&Prefecture::fukui().cities);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.city_name, Some("吉田郡永平寺町".to_string()));
        assert_eq!(tokenizer.rest, "志比５－５");
    }

    #[test]
    fn read_city_with_county_name_completion_今立郡池田町() {
        let tokenizer = Tokenizer {
            input: "".to_string(),
            prefecture_name: None,
            city_name: None,
            town_name: None,
            rest: "池田町稲荷２８－７".to_string(),
            _state: PhantomData::<CityNameNotFound>,
        };
        let result = tokenizer.read_city_with_county_name_completion(&Prefecture::fukui().cities);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.city_name, Some("今立郡池田町".to_string()));
        assert_eq!(tokenizer.rest, "稲荷２８－７");
    }

    #[test]
    fn read_city_with_county_name_completion_南条郡南越前町() {
        let tokenizer = Tokenizer {
            input: "".to_string(),
            prefecture_name: None,
            city_name: None,
            town_name: None,
            rest: "南越前町今庄７４－７－１".to_string(),
            _state: PhantomData::<CityNameNotFound>,
        };
        let result = tokenizer.read_city_with_county_name_completion(&Prefecture::fukui().cities);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.city_name, Some("南条郡南越前町".to_string()));
        assert_eq!(tokenizer.rest, "今庄７４－７－１");
    }

    #[test]
    fn read_city_with_county_name_completion_西村山郡河北町() {
        let tokenizer = Tokenizer {
            input: "".to_string(),
            prefecture_name: None,
            city_name: None,
            town_name: None,
            rest: "河北町大字吉田字馬場261".to_string(),
            _state: PhantomData::<CityNameNotFound>,
        };
        let result =
            tokenizer.read_city_with_county_name_completion(&Prefecture::yamagata().cities);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.city_name, Some("西村山郡河北町".to_string()));
        assert_eq!(tokenizer.rest, "大字吉田字馬場261");
    }

    #[test]
    fn read_city_with_county_name_completion_杵島郡大町町() {
        let tokenizer = Tokenizer {
            input: "".to_string(),
            prefecture_name: None,
            city_name: None,
            town_name: None,
            rest: "大町町大字大町5017番地".to_string(),
            _state: PhantomData::<CityNameNotFound>,
        };
        let result = tokenizer.read_city_with_county_name_completion(&Prefecture::saga().cities);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.city_name, Some("杵島郡大町町".to_string()));
        assert_eq!(tokenizer.rest, "大字大町5017番地");
    }

    #[test]
    fn read_city_with_county_name_completion_最上郡最上町() {
        let tokenizer = Tokenizer {
            input: "".to_string(),
            prefecture_name: None,
            city_name: None,
            town_name: None,
            rest: "最上町法田2672-2".to_string(),
            _state: PhantomData::<CityNameNotFound>,
        };
        let result =
            tokenizer.read_city_with_county_name_completion(&Prefecture::yamagata().cities);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.city_name, Some("最上郡最上町".to_string()));
        assert_eq!(tokenizer.rest, "法田2672-2");
    }
}
