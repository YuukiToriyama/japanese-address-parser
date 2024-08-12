use std::marker::PhantomData;

use crate::parser::adapter::orthographical_variant_adapter::{
    OrthographicalVariantAdapter, OrthographicalVariants, Variant,
};
use crate::parser::filter::fullwidth_character::FullwidthCharacterFilter;
use crate::parser::filter::invalid_town_name_format::InvalidTownNameFormatFilter;
use crate::parser::filter::non_kanji_block_number::NonKanjiBlockNumberFilter;
use crate::parser::filter::Filter;
use crate::tokenizer::{CityNameFound, End, Tokenizer, TownNameFound};

impl Tokenizer<CityNameFound> {
    pub(crate) fn read_town(
        &self,
        candidates: Vec<String>,
    ) -> Result<Tokenizer<TownNameFound>, Tokenizer<End>> {
        let mut rest = FullwidthCharacterFilter {}.apply(self.rest.clone());
        if rest.contains("丁目") {
            rest = NonKanjiBlockNumberFilter {}.apply(rest);
        }
        if let Some((town_name, rest)) = find_town(&rest, &candidates) {
            return Ok(Tokenizer {
                input: self.input.clone(),
                prefecture_name: self.prefecture_name.clone(),
                city_name: self.city_name.clone(),
                town_name: Some(town_name),
                rest,
                _state: PhantomData::<TownNameFound>,
            });
        }
        // 「〇〇町L丁目M番N」ではなく「〇〇町L-M-N」と表記されているような場合
        rest = InvalidTownNameFormatFilter {}.apply(rest);
        if let Some((town_name, rest)) = find_town(&rest, &candidates) {
            return Ok(Tokenizer {
                input: self.input.clone(),
                prefecture_name: self.prefecture_name.clone(),
                city_name: self.city_name.clone(),
                town_name: Some(town_name),
                rest,
                _state: PhantomData::<TownNameFound>,
            });
        }
        // ここまでで町名の検出に成功しない場合は、「大字」の省略の可能性を検討する
        if let Some((town_name, rest)) = find_town(&format!("大字{}", rest), &candidates) {
            return Ok(Tokenizer {
                input: self.input.clone(),
                prefecture_name: self.prefecture_name.clone(),
                city_name: self.city_name.clone(),
                town_name: Some(town_name),
                rest,
                _state: PhantomData::<TownNameFound>,
            });
        }
        Err(Tokenizer {
            input: self.input.clone(),
            prefecture_name: self.prefecture_name.clone(),
            city_name: self.city_name.clone(),
            town_name: None,
            rest: self.rest.clone(),
            _state: PhantomData::<End>,
        })
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
                Variant::瀧,
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
    use crate::tokenizer::{CityNameFound, Tokenizer};
    use std::marker::PhantomData;

    #[test]
    fn read_town_成功() {
        let tokenizer = Tokenizer {
            input: "静岡県静岡市清水区旭町6番8号".to_string(),
            prefecture_name: Some("静岡県".to_string()),
            city_name: Some("静岡市清水区".to_string()),
            town_name: None,
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
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.input, "静岡県静岡市清水区旭町6番8号");
        assert_eq!(tokenizer.prefecture_name.unwrap(), "静岡県");
        assert_eq!(tokenizer.city_name.unwrap(), "静岡市清水区");
        assert_eq!(tokenizer.town_name.unwrap(), "旭町");
        assert_eq!(tokenizer.rest, "6番8号");
    }

    #[test]
    fn read_town_orthographical_variant_adapterで成功() {
        let tokenizer = Tokenizer {
            input: "東京都千代田区一ッ橋二丁目1番".to_string(), // 「ッ」と「ツ」の表記ゆれ
            prefecture_name: Some("東京都".to_string()),
            city_name: Some("千代田区".to_string()),
            town_name: None,
            rest: "一ッ橋二丁目1番".to_string(),
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
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.input, "東京都千代田区一ッ橋二丁目1番");
        assert_eq!(tokenizer.prefecture_name.unwrap(), "東京都");
        assert_eq!(tokenizer.city_name.unwrap(), "千代田区");
        assert_eq!(tokenizer.town_name.unwrap(), "一ツ橋二丁目");
        assert_eq!(tokenizer.rest, "1番");
    }

    #[test]
    fn read_town_invalid_town_name_format_filterで成功() {
        let tokenizer = Tokenizer {
            input: "京都府京都市東山区本町22丁目489番".to_string(),
            prefecture_name: Some("京都府".to_string()),
            city_name: Some("京都市東山区".to_string()),
            town_name: None,
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
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.input, "京都府京都市東山区本町22丁目489番");
        assert_eq!(tokenizer.prefecture_name.unwrap(), "京都府");
        assert_eq!(tokenizer.city_name.unwrap(), "京都市東山区");
        assert_eq!(tokenizer.town_name.unwrap(), "本町二十二丁目");
        assert_eq!(tokenizer.rest, "489番");
    }

    #[test]
    fn read_town_大字が省略されている場合_成功() {
        let tokenizer = Tokenizer {
            input: "東京都西多摩郡日の出町平井2780番地".to_string(), // 「大字」が省略されている
            prefecture_name: Some("東京都".to_string()),
            city_name: Some("西多摩郡日の出町".to_string()),
            town_name: None,
            rest: "平井2780番地".to_string(),
            _state: PhantomData::<CityNameFound>,
        };
        let result = tokenizer.read_town(vec!["大字大久野".to_string(), "大字平井".to_string()]);
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.input, "東京都西多摩郡日の出町平井2780番地");
        assert_eq!(tokenizer.prefecture_name.unwrap(), "東京都");
        assert_eq!(tokenizer.city_name.unwrap(), "西多摩郡日の出町");
        assert_eq!(tokenizer.town_name.unwrap(), "大字平井");
        assert_eq!(tokenizer.rest, "2780番地");
    }

    #[test]
    fn read_town_失敗() {
        let tokenizer = Tokenizer {
            input: "静岡県静岡市清水区".to_string(),
            prefecture_name: Some("静岡県".to_string()),
            city_name: Some("静岡市清水区".to_string()),
            town_name: None,
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
        assert_eq!(tokenizer.input, "静岡県静岡市清水区");
        assert_eq!(tokenizer.prefecture_name.unwrap(), "静岡県");
        assert_eq!(tokenizer.city_name.unwrap(), "静岡市清水区");
        assert_eq!(tokenizer.town_name, None);
        assert_eq!(tokenizer.rest, "");
    }
}
