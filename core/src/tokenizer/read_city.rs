use std::marker::PhantomData;

use crate::parser::adapter::orthographical_variant_adapter::{
    OrthographicalVariantAdapter, OrthographicalVariants, Variant,
};
use crate::parser::adapter::vague_expression_adapter::VagueExpressionAdapter;
use crate::tokenizer::{CityNameFound, End, PrefectureNameFound, Tokenizer};

impl Tokenizer<PrefectureNameFound> {
    pub(crate) fn read_city(
        &self,
        candidates: Vec<String>,
    ) -> Result<Tokenizer<CityNameFound>, Tokenizer<End>> {
        for candidate in &candidates {
            if self.rest.starts_with(candidate) {
                return Ok(Tokenizer {
                    input: self.input.clone(),
                    prefecture_name: self.prefecture_name.clone(),
                    city_name: Some(candidate.clone()),
                    town_name: None,
                    rest: self
                        .rest
                        .chars()
                        .skip(candidate.chars().count())
                        .collect::<String>(),
                    _state: PhantomData::<CityNameFound>,
                });
            }
            let mut variant_list = vec![Variant::ケ];
            match self.prefecture_name.clone().unwrap().as_str() {
                "青森県" => {
                    variant_list.push(Variant::舘);
                }
                "宮城県" => {
                    variant_list.push(Variant::竈);
                }
                "茨城県" => {
                    variant_list.push(Variant::龍);
                    variant_list.push(Variant::嶋);
                }
                "東京都" => {
                    variant_list.push(Variant::檜);
                    variant_list.push(Variant::葛);
                }
                "兵庫県" => {
                    variant_list.push(Variant::塚);
                }
                "奈良県" => {
                    variant_list.push(Variant::葛);
                }
                "高知県" => {
                    variant_list.push(Variant::梼);
                }
                "福岡県" => {
                    variant_list.push(Variant::恵);
                }
                _ => {}
            }
            let adapter = OrthographicalVariantAdapter { variant_list };
            if let Some(result) = adapter.apply(self.rest.as_str(), candidate) {
                return Ok(Tokenizer {
                    input: self.input.clone(),
                    prefecture_name: self.prefecture_name.clone(),
                    city_name: Some(result.1),
                    town_name: None,
                    rest: result.0,
                    _state: PhantomData::<CityNameFound>,
                });
            }
        }

        // ここまでで市町村名の特定ができない場合はVagueExpressionAdapterを使用して市町村名を推測する
        let vague_expression_adapter = VagueExpressionAdapter {};
        if let Some(result) = vague_expression_adapter.apply(self.rest.as_str(), &candidates) {
            return Ok(Tokenizer {
                input: self.input.clone(),
                prefecture_name: self.prefecture_name.clone(),
                city_name: Some(result.1),
                town_name: None,
                rest: result.0,
                _state: PhantomData::<CityNameFound>,
            });
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

#[cfg(test)]
mod tests {
    use crate::tokenizer::{PrefectureNameFound, Tokenizer};
    use std::marker::PhantomData;

    #[test]
    fn read_city_成功() {
        let tokenizer = Tokenizer {
            input: "神奈川県横浜市保土ケ谷区川辺町2番地9".to_string(),
            prefecture_name: Some("神奈川県".to_string()),
            city_name: None,
            town_name: None,
            rest: "横浜市保土ケ谷区川辺町2番地9".to_string(),
            _state: PhantomData::<PrefectureNameFound>,
        };
        let result = tokenizer.read_city(vec![
            "横浜市保土ケ谷区".to_string(),
            "横浜市鶴見区".to_string(),
            "横浜市西区".to_string(),
        ]);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.input, "神奈川県横浜市保土ケ谷区川辺町2番地9");
        assert_eq!(tokenizer.prefecture_name, Some("神奈川県".to_string()));
        assert_eq!(tokenizer.city_name, Some("横浜市保土ケ谷区".to_string()));
        assert_eq!(tokenizer.town_name, None);
        assert_eq!(tokenizer.rest, "川辺町2番地9");
    }

    #[test]
    fn read_city_orthographical_variant_adapterで成功() {
        let tokenizer = Tokenizer {
            input: "神奈川県横浜市保土ヶ谷区川辺町2番地9".to_string(), // 「ヶ」と「ケ」の表記ゆれ
            prefecture_name: Some("神奈川県".to_string()),
            city_name: None,
            town_name: None,
            rest: "横浜市保土ヶ谷区川辺町2番地9".to_string(),
            _state: PhantomData::<PrefectureNameFound>,
        };
        let result = tokenizer.read_city(vec![
            "横浜市保土ケ谷区".to_string(),
            "横浜市鶴見区".to_string(),
            "横浜市西区".to_string(),
        ]);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.input, "神奈川県横浜市保土ヶ谷区川辺町2番地9");
        assert_eq!(tokenizer.prefecture_name, Some("神奈川県".to_string()));
        assert_eq!(tokenizer.city_name, Some("横浜市保土ケ谷区".to_string()));
        assert_eq!(tokenizer.town_name, None);
        assert_eq!(tokenizer.rest, "川辺町2番地9");
    }

    #[test]
    fn read_city_vague_expression_adapterで成功() {
        let tokenizer = Tokenizer {
            input: "埼玉県東秩父村大字御堂634番地".to_string(), // 「秩父郡」が省略されている
            prefecture_name: Some("埼玉県".to_string()),
            city_name: None,
            town_name: None,
            rest: "東秩父村大字御堂634番地".to_string(),
            _state: PhantomData::<PrefectureNameFound>,
        };
        let result = tokenizer.read_city(vec![
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
    fn read_city_失敗() {
        let tokenizer = Tokenizer {
            input: "神奈川県京都市上京区川辺町2番地9".to_string(),
            prefecture_name: Some("神奈川県".to_string()),
            city_name: None,
            town_name: None,
            rest: "京都市上京区川辺町2番地9".to_string(),
            _state: PhantomData::<PrefectureNameFound>,
        };
        let result = tokenizer.read_city(vec![
            "横浜市保土ケ谷区".to_string(),
            "横浜市鶴見区".to_string(),
            "横浜市西区".to_string(),
        ]);
        assert!(result.is_err());
        let tokenizer = result.unwrap_err();
        assert_eq!(tokenizer.input, "神奈川県京都市上京区川辺町2番地9");
        assert_eq!(tokenizer.prefecture_name, Some("神奈川県".to_string()));
        assert_eq!(tokenizer.city_name, None);
        assert_eq!(tokenizer.town_name, None);
        assert_eq!(tokenizer.rest, "京都市上京区川辺町2番地9");
    }
}
