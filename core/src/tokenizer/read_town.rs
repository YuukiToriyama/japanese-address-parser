use crate::domain::common::token::{append_token, Token, Town};
use crate::formatter::chome_with_arabic_numerals::format_chome_with_arabic_numerals;
use crate::formatter::fullwidth_character::format_fullwidth_number;
use crate::formatter::house_number::format_house_number;
use crate::formatter::informal_town_name_notation::format_informal_town_name_notation;
use crate::parser::adapter::orthographical_variant_adapter::{
    OrthographicalVariantAdapter, OrthographicalVariants, Variant,
};
use crate::tokenizer::{CityNameFound, End, Tokenizer, TownNameFound};
use std::marker::PhantomData;

impl Tokenizer<CityNameFound> {
    pub(crate) fn read_town(
        &self,
        candidates: Vec<String>,
    ) -> Result<(String, Tokenizer<TownNameFound>), Tokenizer<End>> {
        let mut rest = format_fullwidth_number(&self.rest);
        if rest.contains("丁目") {
            rest = format_chome_with_arabic_numerals(&rest).unwrap_or(rest);
        }
        let (town_name, rest) = match find_town(&rest, &candidates) {
            Some(found) => found,
            None => {
                // 「〇〇町L丁目M番N」ではなく「〇〇町L-M-N」と表記されているような場合
                rest = format_informal_town_name_notation(&rest).unwrap_or(rest);
                match find_town(&rest, &candidates) {
                    Some(found) => found,
                    None => {
                        // ここまでで町名の検出に成功しない場合は、「大字」の省略の可能性を検討する
                        rest = format!("大字{}", rest);
                        match find_town(&rest, &candidates) {
                            Some(found) => found,
                            None => return Err(self.finish()),
                        }
                    }
                }
            }
        };
        Ok((
            town_name.clone(),
            Tokenizer {
                tokens: append_token(
                    &self.tokens,
                    Token::Town(Town {
                        town_name,
                        representative_point: None,
                    }),
                ),
                rest: if cfg!(feature = "format-house-number") && format_house_number(&rest).is_ok()
                {
                    format_house_number(&rest).unwrap()
                } else {
                    rest
                },
                _state: PhantomData::<TownNameFound>,
            },
        ))
    }
}

fn find_town(input: &str, candidates: &Vec<String>) -> Option<(String, String)> {
    for candidate in candidates {
        if input.starts_with(candidate) {
            return Some((
                candidate.to_string(),
                input
                    .chars()
                    .skip(candidate.chars().count())
                    .collect::<String>(),
            ));
        }
        let adapter = OrthographicalVariantAdapter {
            variant_list: vec![
                Variant::の,
                Variant::ツ,
                Variant::ケ,
                Variant::薮,
                Variant::崎,
                Variant::檜,
                Variant::竈,
                Variant::舘,
                Variant::脊,
                Variant::渕,
                Variant::己,
                Variant::槇,
                Variant::治,
                Variant::佛,
                Variant::澤,
                Variant::恵,
                Variant::穂,
                Variant::梼,
                Variant::蛍,
                Variant::與,
                Variant::瀧,
                Variant::籠,
                Variant::濱,
                Variant::祗,
                Variant::曾,
            ],
        };
        if let Some(result) = adapter.apply(input, candidate) {
            return Some(result);
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::domain::common::token::{City, Prefecture, Token};
    use crate::tokenizer::{CityNameFound, Tokenizer};
    use std::marker::PhantomData;

    #[test]
    fn read_town_成功() {
        let tokenizer = Tokenizer {
            tokens: vec![
                Token::Prefecture(Prefecture {
                    prefecture_name: "静岡県".to_string(),
                    representative_point: None,
                }),
                Token::City(City {
                    city_name: "静岡市清水区".to_string(),
                    representative_point: None,
                }),
            ],
            rest: "旭町6番8号".to_string(),
            _state: PhantomData::<CityNameFound>,
        };
        let result = tokenizer.read_town(vec![
            "下野緑町".to_string(),
            "承元寺町".to_string(),
            "旭町".to_string(),
            "新丹谷".to_string(),
            "三保松原町".to_string(),
        ]);
        assert!(result.is_ok());
        let (town_name, tokenizer) = result.unwrap();
        assert_eq!(town_name, "旭町");
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(tokenizer.rest, "6番8号");
    }

    #[test]
    fn read_town_orthographical_variant_adapterで成功() {
        let tokenizer = Tokenizer {
            tokens: vec![
                Token::Prefecture(Prefecture {
                    prefecture_name: "東京都".to_string(),
                    representative_point: None,
                }),
                Token::City(City {
                    city_name: "千代田区".to_string(),
                    representative_point: None,
                }),
            ],
            rest: "一ッ橋二丁目1番".to_string(), // 「ッ」と「ツ」の表記ゆれ
            _state: PhantomData::<CityNameFound>,
        };
        let result = tokenizer.read_town(vec![
            "神田錦町一丁目".to_string(),
            "神田錦町二丁目".to_string(),
            "神田錦町三丁目".to_string(),
            "一ツ橋一丁目".to_string(),
            "一ツ橋二丁目".to_string(),
        ]);
        assert!(result.is_ok());
        let (town_name, tokenizer) = result.unwrap();
        assert_eq!(town_name, "一ツ橋二丁目");
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(tokenizer.rest, "1番");
    }

    #[test]
    fn read_town_invalid_town_name_format_filterで成功() {
        let tokenizer = Tokenizer {
            tokens: vec![
                Token::Prefecture(Prefecture {
                    prefecture_name: "京都府".to_string(),
                    representative_point: None,
                }),
                Token::City(City {
                    city_name: "京都市東山区".to_string(),
                    representative_point: None,
                }),
            ],
            rest: "本町22丁目489番".to_string(),
            _state: PhantomData::<CityNameFound>,
        };
        let result = tokenizer.read_town(vec![
            "本町十九丁目".to_string(),
            "本町二十丁目".to_string(),
            "本町二十一丁目".to_string(),
            "本町二十二丁目".to_string(),
            "本町新五丁目".to_string(),
            "本町新六丁目".to_string(),
        ]);
        assert!(result.is_ok());
        let (town_name, tokenizer) = result.unwrap();
        assert_eq!(town_name, "本町二十二丁目");
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(tokenizer.rest, "489番");
    }

    #[test]
    fn read_town_大字が省略されている場合_成功() {
        let tokenizer = Tokenizer {
            tokens: vec![
                Token::Prefecture(Prefecture {
                    prefecture_name: "東京都".to_string(),
                    representative_point: None,
                }),
                Token::City(City {
                    city_name: "西多摩郡日の出町".to_string(),
                    representative_point: None,
                }),
            ],
            rest: "平井2780番地".to_string(), // 「大字」が省略されている
            _state: PhantomData::<CityNameFound>,
        };
        let result = tokenizer.read_town(vec!["大字大久野".to_string(), "大字平井".to_string()]);
        assert!(result.is_ok());
        let (town_name, tokenizer) = result.unwrap();
        assert_eq!(town_name, "大字平井");
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(tokenizer.rest, "2780番地");
    }

    #[test]
    fn read_town_失敗() {
        let tokenizer = Tokenizer {
            tokens: vec![
                Token::Prefecture(Prefecture {
                    prefecture_name: "静岡県".to_string(),
                    representative_point: None,
                }),
                Token::City(City {
                    city_name: "静岡市清水区".to_string(),
                    representative_point: None,
                }),
            ],
            rest: "".to_string(),
            _state: PhantomData::<CityNameFound>,
        };
        let result = tokenizer.read_town(vec![
            "下野緑町".to_string(),
            "承元寺町".to_string(),
            "旭町".to_string(),
            "新丹谷".to_string(),
            "三保松原町".to_string(),
        ]);
        assert!(result.is_err());
        let tokenizer = result.unwrap_err();
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(tokenizer.tokens[2], Token::Rest("".to_string()));
    }
}
