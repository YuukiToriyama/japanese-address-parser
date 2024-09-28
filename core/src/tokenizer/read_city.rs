use crate::domain::common::token::{append_token, City, Token};
use crate::parser::adapter::orthographical_variant_adapter::{
    OrthographicalVariantAdapter, OrthographicalVariants, Variant,
};
use crate::tokenizer::{CityNameFound, CityNameNotFound, PrefectureNameFound, Tokenizer};
use std::marker::PhantomData;

impl Tokenizer<PrefectureNameFound> {
    pub(crate) fn read_city(
        &self,
        candidates: &Vec<String>,
    ) -> Result<(String, Tokenizer<CityNameFound>), Tokenizer<CityNameNotFound>> {
        for candidate in candidates {
            if self.rest.starts_with(candidate) {
                return Ok((
                    candidate.to_string(),
                    Tokenizer {
                        input: self.input.clone(),
                        tokens: append_token(
                            &self.tokens,
                            Token::City(City {
                                city_name: candidate.to_string(),
                                representative_point: None,
                            }),
                        ),
                        prefecture_name: self.prefecture_name.clone(),
                        city_name: Some(candidate.clone()),
                        rest: self
                            .rest
                            .chars()
                            .skip(candidate.chars().count())
                            .collect::<String>(),
                        _state: PhantomData::<CityNameFound>,
                    },
                ));
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
                }
                "兵庫県" => {
                    variant_list.push(Variant::塚);
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
                return Ok((
                    result.0.clone(),
                    Tokenizer {
                        input: self.input.clone(),
                        tokens: append_token(
                            &self.tokens,
                            Token::City(City {
                                city_name: result.0.clone(), // TODO: 以降に使用箇所があるためcloneしているが本来不要なので使用箇所なくなったら削除する
                                representative_point: None,
                            }),
                        ),
                        prefecture_name: self.prefecture_name.clone(),
                        city_name: Some(result.0),
                        rest: result.1,
                        _state: PhantomData::<CityNameFound>,
                    },
                ));
            }
        }

        Err(Tokenizer {
            input: self.input.clone(),
            tokens: self.tokens.clone(),
            prefecture_name: self.prefecture_name.clone(),
            city_name: None,
            rest: self.rest.clone(),
            _state: PhantomData::<CityNameNotFound>,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::common::token::{Prefecture, Token};
    use crate::tokenizer::{PrefectureNameFound, Tokenizer};
    use std::marker::PhantomData;

    #[test]
    fn read_city_成功() {
        let tokenizer = Tokenizer {
            input: "神奈川県横浜市保土ケ谷区川辺町2番地9".to_string(),
            tokens: vec![Token::Prefecture(Prefecture {
                prefecture_name: "神奈川県".to_string(),
                representative_point: None,
            })],
            prefecture_name: Some("神奈川県".to_string()),
            city_name: None,
            rest: "横浜市保土ケ谷区川辺町2番地9".to_string(),
            _state: PhantomData::<PrefectureNameFound>,
        };
        let result = tokenizer.read_city(&vec![
            "横浜市保土ケ谷区".to_string(),
            "横浜市鶴見区".to_string(),
            "横浜市西区".to_string(),
        ]);
        assert!(result.is_ok());
        let (city_name, tokenizer) = result.unwrap();
        assert_eq!(city_name, "横浜市保土ケ谷区");
        assert_eq!(tokenizer.input, "神奈川県横浜市保土ケ谷区川辺町2番地9");
        assert_eq!(tokenizer.tokens.len(), 2);
        assert_eq!(tokenizer.rest, "川辺町2番地9");
    }

    #[test]
    fn read_city_orthographical_variant_adapterで成功() {
        let tokenizer = Tokenizer {
            input: "神奈川県横浜市保土ヶ谷区川辺町2番地9".to_string(), // 「ヶ」と「ケ」の表記ゆれ
            tokens: vec![Token::Prefecture(Prefecture {
                prefecture_name: "神奈川県".to_string(),
                representative_point: None,
            })],
            prefecture_name: Some("神奈川県".to_string()),
            city_name: None,
            rest: "横浜市保土ヶ谷区川辺町2番地9".to_string(),
            _state: PhantomData::<PrefectureNameFound>,
        };
        let result = tokenizer.read_city(&vec![
            "横浜市保土ケ谷区".to_string(),
            "横浜市鶴見区".to_string(),
            "横浜市西区".to_string(),
        ]);
        assert!(result.is_ok());
        let (city_name, tokenizer) = result.unwrap();
        assert_eq!(city_name, "横浜市保土ケ谷区".to_string());
        assert_eq!(tokenizer.input, "神奈川県横浜市保土ヶ谷区川辺町2番地9");
        assert_eq!(tokenizer.tokens.len(), 2);
        assert_eq!(tokenizer.rest, "川辺町2番地9");
    }

    #[test]
    fn read_city_失敗() {
        let tokenizer = Tokenizer {
            input: "神奈川県京都市上京区川辺町2番地9".to_string(),
            tokens: vec![Token::Prefecture(Prefecture {
                prefecture_name: "神奈川県".to_string(),
                representative_point: None,
            })],
            prefecture_name: Some("神奈川県".to_string()),
            city_name: None,
            rest: "京都市上京区川辺町2番地9".to_string(),
            _state: PhantomData::<PrefectureNameFound>,
        };
        let result = tokenizer.read_city(&vec![
            "横浜市保土ケ谷区".to_string(),
            "横浜市鶴見区".to_string(),
            "横浜市西区".to_string(),
        ]);
        assert!(result.is_err());
        let tokenizer = result.unwrap_err();
        assert_eq!(tokenizer.input, "神奈川県京都市上京区川辺町2番地9");
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(tokenizer.rest, "京都市上京区川辺町2番地9");
    }
}
