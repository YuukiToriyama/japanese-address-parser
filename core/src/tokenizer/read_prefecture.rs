use crate::domain::common::token::{Prefecture, Token};
use crate::tokenizer::{End, Init, PrefectureNameFound, Tokenizer};
use crate::util::extension::StrExt;
use std::marker::PhantomData;

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
            tokens: vec![],
            rest: if cfg!(feature = "eliminate-whitespaces") {
                input.strip_variation_selectors().strip_whitespaces()
            } else {
                input.strip_variation_selectors()
            },
            _state: PhantomData,
        }
    }

    pub(crate) fn read_prefecture(
        &self,
    ) -> Result<(String, Tokenizer<PrefectureNameFound>), Tokenizer<End>> {
        for prefecture_name in PREFECTURE_NAME_LIST {
            if self.rest.starts_with(prefecture_name) {
                return Ok((
                    prefecture_name.to_string(),
                    Tokenizer {
                        tokens: vec![Token::Prefecture(Prefecture {
                            prefecture_name: prefecture_name.to_string(),
                            representative_point: None,
                        })],
                        rest: self
                            .rest
                            .chars()
                            .skip(prefecture_name.chars().count())
                            .collect::<String>(),
                        _state: PhantomData::<PrefectureNameFound>,
                    },
                ));
            }
        }
        Err(self.finish())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::common::token::Token;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn new() {
        let tokenizer = Tokenizer::new("東京都港区芝公園4丁目2-8");
        assert_eq!(tokenizer.tokens, vec![]);
        assert_eq!(tokenizer.rest, "東京都港区芝公園4丁目2-8");
    }

    #[test]
    fn new_異字体セレクタ除去() {
        let tokenizer = Tokenizer::new("東京都葛\u{E0100}飾区立石5-13-1");
        assert_eq!(tokenizer.tokens, vec![]);
        assert_eq!(tokenizer.rest, "東京都葛飾区立石5-13-1")
    }

    #[test]
    #[cfg(feature = "eliminate-whitespaces")]
    fn new_ホワイトスペース除却() {
        let tokenizer = Tokenizer::new("東京都 目黒区 下目黒 4‐1‐1");
        assert_eq!(tokenizer.tokens, vec![]);
        assert_eq!(tokenizer.rest, "東京都目黒区下目黒4‐1‐1")
    }

    #[test]
    fn read_prefecture_成功() {
        let tokenizer = Tokenizer::new("東京都港区芝公園4丁目2-8");
        let result = tokenizer.read_prefecture();
        assert!(result.is_ok());
        let (prefecture_name, tokenizer) = result.unwrap();
        assert_eq!(prefecture_name, "東京都");
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(tokenizer.rest, "港区芝公園4丁目2-8");
    }

    #[test]
    fn read_prefecture_失敗() {
        let tokenizer = Tokenizer::new("東今日都港区芝公園4丁目2-8");
        let result = tokenizer.read_prefecture();
        assert!(result.is_err());
        let tokenizer = result.unwrap_err();
        assert_eq!(
            tokenizer.tokens,
            vec![Token::Rest("東今日都港区芝公園4丁目2-8".to_string())]
        );
    }
}
