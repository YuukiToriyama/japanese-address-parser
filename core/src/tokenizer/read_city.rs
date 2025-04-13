use crate::adapter::orthographical_variant_adapter::{
    OrthographicalVariant, OrthographicalVariantAdapter,
};
use crate::domain::common::token::{append_token, Token};
use crate::tokenizer::{CityNameFound, CityNameNotFound, PrefectureNameFound, Tokenizer};
use std::marker::PhantomData;

impl Tokenizer<PrefectureNameFound> {
    pub(crate) fn read_city(
        &self,
        candidates: &[String],
    ) -> Result<(String, Tokenizer<CityNameFound>), Tokenizer<CityNameNotFound>> {
        if let Some(found) = candidates
            .iter()
            .find(|&candidate| self.rest.starts_with(candidate))
        {
            return Ok((
                found.to_string(),
                Tokenizer {
                    tokens: append_token(&self.tokens, Token::City(found.to_string())),
                    rest: self.rest.chars().skip(found.chars().count()).collect(),
                    _state: PhantomData::<CityNameFound>,
                },
            ));
        }

        // ここまでで市区町村名が読み取れない場合は、表記ゆれを含む可能性を検討する
        use OrthographicalVariant::*;
        let mut variant_list = vec![ケ];
        if let Some(pref_name) = self.get_prefecture_name() {
            variant_list.extend(match pref_name {
                "青森県" => vec![舘, 鰺],
                "宮城県" => vec![竈],
                "茨城県" => vec![龍, 嶋],
                "東京都" => vec![檜],
                "岐阜県" => vec![驒],
                "三重県" => vec![婁],
                "大阪府" => vec![條],
                "兵庫県" => vec![塚],
                "奈良県" => vec![條],
                "和歌山県" => vec![婁],
                "高知県" => vec![梼],
                "福岡県" => vec![恵],
                "長崎県" => vec![諫],
                _ => vec![],
            });
        }
        for candidate in candidates {
            let adapter = OrthographicalVariantAdapter {
                variant_list: variant_list.clone(),
            };
            if let Some((city_name, rest)) = adapter.apply(&self.rest, candidate) {
                return Ok((
                    city_name.clone(),
                    Tokenizer {
                        tokens: append_token(&self.tokens, Token::City(city_name)),
                        rest,
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
    use crate::domain::common::token::Token;
    use crate::tokenizer::{PrefectureNameFound, Tokenizer};
    use std::marker::PhantomData;

    #[test]
    fn read_city_成功() {
        let tokenizer = Tokenizer {
            tokens: vec![Token::Prefecture("神奈川県".to_string())],
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
            tokens: vec![Token::Prefecture("神奈川県".to_string())],
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
            tokens: vec![Token::Prefecture("神奈川県".to_string())],
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
