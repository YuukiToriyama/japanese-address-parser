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
                        tokens: append_token(
                            &self.tokens,
                            Token::City(City {
                                city_name: candidate.to_string(),
                                representative_point: None,
                            }),
                        ),
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
            match self.get_prefecture_name() {
                Some("青森県") => {
                    variant_list.push(Variant::舘);
                }
                Some("宮城県") => {
                    variant_list.push(Variant::竈);
                }
                Some("茨城県") => {
                    variant_list.push(Variant::龍);
                    variant_list.push(Variant::嶋);
                }
                Some("東京都") => {
                    variant_list.push(Variant::檜);
                }
                Some("兵庫県") => {
                    variant_list.push(Variant::塚);
                }
                Some("高知県") => {
                    variant_list.push(Variant::梼);
                }
                Some("福岡県") => {
                    variant_list.push(Variant::恵);
                }
                _ => {}
            }
            let adapter = OrthographicalVariantAdapter { variant_list };
            if let Some(result) = adapter.apply(self.rest.as_str(), candidate) {
                return Ok((
                    result.0.clone(),
                    Tokenizer {
                        tokens: append_token(
                            &self.tokens,
                            Token::City(City {
                                city_name: result.0,
                                representative_point: None,
                            }),
                        ),
                        rest: result.1,
                        _state: PhantomData::<CityNameFound>,
                    },
                ));
            }
        }

        Err(Tokenizer {
            tokens: self.tokens.clone(),
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
            tokens: vec![Token::Prefecture(Prefecture {
                prefecture_name: "神奈川県".to_string(),
                representative_point: None,
            })],
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
        assert_eq!(tokenizer.tokens.len(), 2);
        assert_eq!(tokenizer.rest, "川辺町2番地9");
    }

    #[test]
    fn read_city_orthographical_variant_adapterで成功() {
        let tokenizer = Tokenizer {
            tokens: vec![Token::Prefecture(Prefecture {
                prefecture_name: "神奈川県".to_string(),
                representative_point: None,
            })],
            rest: "横浜市保土ヶ谷区川辺町2番地9".to_string(), // 「ヶ」と「ケ」の表記ゆれ
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
        assert_eq!(tokenizer.tokens.len(), 2);
        assert_eq!(tokenizer.rest, "川辺町2番地9");
    }

    #[test]
    fn read_city_失敗() {
        let tokenizer = Tokenizer {
            tokens: vec![Token::Prefecture(Prefecture {
                prefecture_name: "神奈川県".to_string(),
                representative_point: None,
            })],
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
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(tokenizer.rest, "京都市上京区川辺町2番地9");
    }
}
