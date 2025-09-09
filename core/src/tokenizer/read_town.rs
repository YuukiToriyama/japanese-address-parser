use crate::adapter::orthographical_variant_adapter::{
    OrthographicalVariant, OrthographicalVariantAdapter,
};
use crate::domain::common::token::{append_token, Token};
use crate::formatter::chome_with_arabic_numerals::format_chome_with_arabic_numerals;
use crate::formatter::fullwidth_character::format_fullwidth_numerals;
use crate::formatter::house_number::format_house_number;
use crate::formatter::informal_town_name_notation::format_informal_town_name_notation;
use crate::tokenizer::{CityNameFound, End, Tokenizer, TownNameFound};
use std::marker::PhantomData;

impl Tokenizer<CityNameFound> {
    pub(crate) fn read_town(
        &self,
        candidates: Vec<String>,
    ) -> Result<(String, Tokenizer<TownNameFound>), Tokenizer<End>> {
        let mut rest = format_fullwidth_numerals(&self.rest);
        if rest.contains("丁目") {
            rest = format_chome_with_arabic_numerals(&rest).unwrap_or(rest);
        }
        let (town_name, rest) = find_town(&rest, &candidates)
            .or_else(|| {
                // 「〇〇町L丁目M番N」ではなく「〇〇町L-M-N」と表記されているような場合
                if let Some(it) = format_informal_town_name_notation(&rest) {
                    rest = it
                }
                find_town(&rest, &candidates)
            })
            .or_else(|| {
                // ここまでで町名の検出に成功しない場合は、「大字」の省略の可能性を検討する
                find_town(&format!("大字{}", rest), &candidates)
            })
            .or_else(|| {
                // ここまでで町名の検出に成功しない場合は、「字」の省略の可能性を検討する
                find_town(&format!("字{}", rest), &candidates)
            })
            .ok_or_else(|| self.finish())?;
        Ok((
            town_name.clone(),
            Tokenizer {
                tokens: append_token(&self.tokens, Token::Town(town_name)),
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

/// Find out one of the most likely matches from the given candidates
fn find_town(input: &str, candidates: &[String]) -> Option<(String, String)> {
    // 候補の中から「丁目」を含むものとそれ以外のものを分類する
    let (contains_chome, not_contains): (Vec<&String>, Vec<&String>) = candidates
        .iter()
        .partition(|candidate| candidate.contains("丁目"));
    // 住居表示施行済みの候補から試すようにする
    let candidates = [contains_chome, not_contains].concat();

    for candidate in candidates {
        if input.starts_with(candidate) {
            return Some((
                candidate.to_string(),
                input.chars().skip(candidate.chars().count()).collect(),
            ));
        }
        use OrthographicalVariant::*;
        let adapter = OrthographicalVariantAdapter {
            variant_list: vec![
                の, ツ, ケ, 薮, 崎, 檜, 竈, 舘, 鰺, 脊, 渕, 己, 槇, 治, 佛, 澤, 恵, 穂, 梼, 蛍, 與,
                瀧, 籠, 濱, 祗, 曾, 國, 鉋, 鷆, 斑, 櫻, 櫟, 冨, 鶯, 龍, 廣, 塚, 麴, 炮,
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
    use crate::domain::common::token::Token;
    use crate::tokenizer::read_town::find_town;
    use crate::tokenizer::{CityNameFound, Tokenizer};
    use std::marker::PhantomData;

    #[test]
    fn find_town_住居表示実施未実施が混在する場合実施済みの候補を優先的に処理する() {
        let candidates = vec![
            "下多良".to_string(),
            "下多良一丁目".to_string(),
            "下多良二丁目".to_string(),
            "下多良三丁目".to_string(),
        ];

        let result = find_town("下多良二丁目137", &candidates);
        assert_eq!(
            result.unwrap(),
            ("下多良二丁目".to_string(), "137".to_string())
        )
    }

    #[test]
    fn find_town_同一の部分を持つ候補が複数ある場合住居表示実施済みの候補を優先的に処理する() {
        let candidates = vec![
            "薮田".to_string(),
            "薮田中一丁目".to_string(),
            "薮田東二丁目".to_string(),
            "薮田南二丁目".to_string(),
        ];

        let result = find_town("藪田南二丁目1-1", &candidates);
        assert_eq!(
            result.unwrap(),
            ("薮田南二丁目".to_string(), "1-1".to_string())
        );
    }

    #[test]
    fn read_town_成功() {
        let tokenizer = Tokenizer {
            tokens: vec![
                Token::Prefecture("静岡県".to_string()),
                Token::City("静岡市清水区".to_string()),
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
                Token::Prefecture("東京都".to_string()),
                Token::City("千代田区".to_string()),
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
                Token::Prefecture("京都府".to_string()),
                Token::City("京都市東山区".to_string()),
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
                Token::Prefecture("東京都".to_string()),
                Token::City("西多摩郡日の出町".to_string()),
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
    fn read_town_字が省略されている場合() {
        let tokenizer = Tokenizer {
            tokens: vec![
                Token::Prefecture("埼玉県".to_string()),
                Token::City("南埼玉郡宮代町".to_string()),
            ],
            rest: "東粂原111".to_string(),
            _state: PhantomData::<CityNameFound>,
        };
        let result = tokenizer.read_town(vec![
            "東姫宮一丁目".to_string(),
            "字東".to_string(),
            "字宮東".to_string(),
            "大字東粂原".to_string(),
        ]);
        assert!(result.is_ok());
        let (town_name, tokenizer) = result.unwrap();
        assert_eq!(town_name, "大字東粂原");
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(tokenizer.rest, "111");
    }

    #[test]
    fn read_town_失敗() {
        let tokenizer = Tokenizer {
            tokens: vec![
                Token::Prefecture("静岡県".to_string()),
                Token::City("静岡市清水区".to_string()),
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
