use crate::domain::common::token::Token;
use crate::tokenizer::{End, Init, PrefectureNameFound, Tokenizer};
use crate::util::extension::StrExt;
use std::marker::PhantomData;

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
    ) -> Result<(jisx0401::Prefecture, Tokenizer<PrefectureNameFound>), Tokenizer<End>> {
        match find_prefecture(&self.rest) {
            Some(prefecture) => {
                let prefecture_name = prefecture.name_ja();
                Ok((
                    prefecture.clone(),
                    Tokenizer {
                        tokens: vec![Token::Prefecture(prefecture_name.to_string())],
                        rest: self
                            .rest
                            .chars()
                            .skip(prefecture_name.chars().count())
                            .collect::<String>(),
                        _state: PhantomData::<PrefectureNameFound>,
                    },
                ))
            }
            None => Err(self.finish()),
        }
    }
}

fn find_prefecture(input: &str) -> Option<&jisx0401::Prefecture> {
    jisx0401::Prefecture::values().find(|&prefecture| input.starts_with(prefecture.name_ja()))
}

#[cfg(test)]
mod tests {
    use crate::domain::common::token::Token;
    use crate::tokenizer::Tokenizer;
    use jisx0401::Prefecture;

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
        let (prefecture, tokenizer) = result.unwrap();
        assert_eq!(prefecture, Prefecture::TOKYO);
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
