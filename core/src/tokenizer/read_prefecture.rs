use std::marker::PhantomData;

use crate::tokenizer::{End, Init, PrefectureNameFound, Tokenizer};

const PREFECTURE_NAME_LIST: [&str; 47] = [
    "北海道",
    "青森県",
    "岩手県",
    "宮城県",
    "秋田県",
    "山形県",
    "福島県",
    "茨城県",
    "栃木県",
    "群馬県",
    "埼玉県",
    "千葉県",
    "東京都",
    "神奈川県",
    "新潟県",
    "富山県",
    "石川県",
    "福井県",
    "山梨県",
    "長野県",
    "岐阜県",
    "静岡県",
    "愛知県",
    "三重県",
    "滋賀県",
    "京都府",
    "大阪府",
    "兵庫県",
    "奈良県",
    "和歌山県",
    "鳥取県",
    "島根県",
    "岡山県",
    "広島県",
    "山口県",
    "徳島県",
    "香川県",
    "愛媛県",
    "高知県",
    "福岡県",
    "佐賀県",
    "長崎県",
    "熊本県",
    "大分県",
    "宮崎県",
    "鹿児島県",
    "沖縄県",
];

impl Tokenizer<Init> {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            prefecture_name: None,
            city_name: None,
            town_name: None,
            rest: input.to_string(),
            _state: PhantomData,
        }
    }

    pub(crate) fn read_prefecture(&self) -> Result<Tokenizer<PrefectureNameFound>, Tokenizer<End>> {
        for prefecture_name in PREFECTURE_NAME_LIST {
            if self.input.starts_with(prefecture_name) {
                return Ok(Tokenizer {
                    input: self.input.clone(),
                    prefecture_name: Some(prefecture_name.to_string()),
                    city_name: None,
                    town_name: None,
                    rest: self
                        .input
                        .chars()
                        .skip(prefecture_name.chars().count())
                        .collect::<String>(),
                    _state: PhantomData::<PrefectureNameFound>,
                });
            }
        }
        Err(Tokenizer {
            input: self.input.clone(),
            prefecture_name: None,
            city_name: None,
            town_name: None,
            rest: self.rest.clone(),
            _state: PhantomData::<End>,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;

    #[test]
    fn new() {
        let tokenizer = Tokenizer::new("東京都港区芝公園4丁目2-8");
        assert_eq!(tokenizer.input, "東京都港区芝公園4丁目2-8");
        assert_eq!(tokenizer.prefecture_name, None);
        assert_eq!(tokenizer.city_name, None);
        assert_eq!(tokenizer.town_name, None);
        assert_eq!(tokenizer.rest, "東京都港区芝公園4丁目2-8");
    }

    #[test]
    fn read_prefecture_成功() {
        let tokenizer = Tokenizer::new("東京都港区芝公園4丁目2-8");
        let result = tokenizer.read_prefecture();
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        assert_eq!(tokenizer.input, "東京都港区芝公園4丁目2-8");
        assert_eq!(tokenizer.prefecture_name, Some("東京都".to_string()));
        assert_eq!(tokenizer.city_name, None);
        assert_eq!(tokenizer.town_name, None);
        assert_eq!(tokenizer.rest, "港区芝公園4丁目2-8");
    }

    #[test]
    fn read_prefecture_失敗() {
        let tokenizer = Tokenizer::new("東今日都港区芝公園4丁目2-8");
        let result = tokenizer.read_prefecture();
        assert!(result.is_err());
        let tokenizer = result.unwrap_err();
        assert_eq!(tokenizer.input, "東今日都港区芝公園4丁目2-8");
        assert_eq!(tokenizer.prefecture_name, None);
        assert_eq!(tokenizer.city_name, None);
        assert_eq!(tokenizer.town_name, None);
        assert_eq!(tokenizer.rest, "東今日都港区芝公園4丁目2-8".to_string());
    }
}
